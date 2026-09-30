"""闭包来源：调用 `rava closure`（Rust 精确闭包分析器）并把 closure.json 转成发射层消费的形状。

C4（docs/plans/2026-09-30-c4-closure-integration.md）：生成范围只有这一个来源。返回值与
发射层约定：
  jdk_class_infos  非用户域的全部闭包类（ClassInfo；库类取 jar 注册表同一对象）
  visited_methods  {(类, 名, 描述符)}：已解析方法 ∪ 活代码调用点符号键（槽位需求）∪ <clinit>
  field_stubs      只按类型层级入闭包的类（报告用）
另填充 callchain 模块的发射期全局（反射面 / 种子 / 模块资源 / 预检链事实）。
"""

import json
import os
import shutil
import subprocess
import sys

from . import callchain as _cc
from . import options as _options
from .classfile import parse_class_bytes
from .jdk_resolver import JdkResolver

_REPO = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
_GENERATOR = os.path.join(_REPO, 'generator', 'Cargo.toml')
# 分析器构建产物按工作树隔离：共享 target 下多个工作树的同名 crate 源路径不同，会互相作废指纹、
# 交替重编（开发中的工作树与跑批快照并存时尤甚）
_ANALYZER_TARGET = os.path.join(_REPO, 'build', 'analyzer-target')


def _split_id(mid: str) -> tuple[str, str, str]:
    """`owner.name:desc` → (owner, name, desc)（owner 含 `/`、`$`，名字不含 `.`）"""
    head, desc = mid.split(':', 1)
    owner, name = head.rsplit('.', 1)
    return owner, name, desc


def _user_class_dir(class_files, out_dir: str) -> str:
    """只含本编译单元类的目录（javac 输出目录按 feature 共享，混有其他单元的类）"""
    d = os.path.join(out_dir, 'closure_input', 'classes')
    shutil.rmtree(d, ignore_errors=True)
    for name, src in class_files:
        p = os.path.join(d, name + '.class')
        os.makedirs(os.path.dirname(p), exist_ok=True)
        shutil.copyfile(src, p)
    return d


def _run_analyzer(args: list[str]) -> None:
    cmd = ['cargo', 'run', '--release', '-q', '--manifest-path', _GENERATOR,
           '--target-dir', _ANALYZER_TARGET, '--', 'closure'] + args
    r = subprocess.run(cmd, capture_output=True, text=True)
    for line in r.stderr.splitlines():
        if line.startswith('[closure]'):
            print(f"      {line}")
    if r.returncode != 0:
        sys.exit(f"rava closure 失败（{r.returncode}）：\n{r.stderr[-4000:]}")
    if _options.TRACE_CLASS:
        # --why 的 provenance 链在 stdout 摘要 JSON 之前
        for line in r.stdout.splitlines():
            if line.startswith('{'):
                break
            print(f"      [why] {line}" if line else '')


def discover(class_infos, class_files, out_dir: str, *, lib_jars=(), lib_registries=(),
             lib_seed_classes=None, locales=(), jdk_seed_methods=None):
    """class_files：[(binary name, .class 路径)]，与 class_infos 同为本编译单元的用户类"""
    _max_major = max((getattr(ci, 'major_version', 0) for ci in class_infos), default=0)
    _prefer = _max_major - 44 if _max_major >= 45 else None
    if _prefer:
        print(f"      语料选择：用户类 class 版本 {_max_major} → JDK {_prefer} jmods"
              f"（--jdk 显式指定优先）", flush=True)
    resolver = JdkResolver(prefer_major=_prefer)

    # 手写层真源（scratch 的 java_runtime/src 混有上轮生成物，不能作分析输入）
    runtime = os.path.join(_REPO, 'runtime', 'java_runtime')
    json_path = os.path.join(out_dir, 'closure_input', 'closure.json')
    args = [_user_class_dir(class_files, out_dir), '--java-home', str(resolver._home),
            '--runtime', runtime, '--main', class_infos[0].name, '-o', json_path]
    for loc in locales:
        args += ['--locale', loc]
    for c, m, d in (jdk_seed_methods or ()):
        args += ['--root', f'{c}.{m}:{d}']
    for jar in lib_jars:
        args += ['--lib', jar]
    for c in (lib_seed_classes or ()):
        args += ['--seed-class', c]
    for d in getattr(resolver, 'image_class_dirs', lambda: ())():
        args += ['--image', d]
    if _options.TRACE_CLASS:
        args += ['--why', _options.TRACE_CLASS]
    _run_analyzer(args)
    with open(json_path, encoding='utf-8') as f:
        cj = json.load(f)
    # 折叠点（剪除的不可达代码 / 常量读取点）在类解码时单点应用：先载入再解析闭包类；
    # 用户类在分析前已解析，按折叠重解析（就地替换，调用方持有同一列表）
    from . import closure_folds
    print(f"[folds] {closure_folds.load(json_path)} 个方法带折叠")
    from .classfile import parse_class
    paths = dict(class_files)
    class_infos[:] = [parse_class(paths[ci.name]) for ci in class_infos]
    return _consume(cj, class_infos, resolver, lib_registries)


def _consume(cj, class_infos, resolver, lib_registries):
    user_names = {ci.name for ci in class_infos}
    lib_lookup = {}
    for reg in lib_registries:
        lib_lookup.update(reg)
    cache: dict[str, object] = {}

    def load(name: str):
        if name in lib_lookup:
            return lib_lookup[name]
        if name not in cache:
            data = resolver.resolve(name)
            cache[name] = parse_class_bytes(data, name) if data is not None else None
        return cache[name]

    jdk_infos = []
    field_stubs: set[str] = set()
    for c in cj['classes']:
        name = c['name']
        # 根域类（手写整体承载，如 Object）不进发射 registry：其方法表不参与后代的重载判定，
        # 与手写层的方法命名一致
        if c['domain'] == 'root' or (c['domain'] == 'user' and name in user_names):
            continue
        ci = load(name)
        if ci is None:
            print(f"[closure] 警告：闭包类无法装载：{name}")
            continue
        jdk_infos.append(ci)
        if c['level'] == 'type':
            field_stubs.add(name)

    # 边界成员（`handwritten:boundary`）同样入链：发射层对手写提供的出 [meta]，其余翻译方法体
    # 而不展开其被调方（规模截断的过渡语义，分析器按类型级触达体内引用的类；终态随截断清零消失）。
    # 边界类 `<clinit>` 手写承载，不入链
    boundary = {c['name'] for c in cj['classes'] if c['domain'] == 'boundary'}
    visited = {k for k, kind in ((_split_id(m['id']), m['kind']) for m in cj['methods'])
               if not (k[1] == '<clinit>' and kind == 'handwritten:boundary')}
    visited |= {k for k in ((c, '<clinit>', '()V') for c in cj['clinit'])
                if k[0] not in boundary or k in visited}
    # 调用点符号键只表达继承槽位需求（常量池类未声明、经超类 / 接口解析的成员）：常量池类自身声明
    # 而分析器未解析到的成员没有可达接收者，发射为存根
    for k in map(_split_id, cj.get('refs', ())):
        if k in visited or k[0] in boundary:
            continue
        ci = load(k[0])
        if ci is not None and any(x.name == k[1] and x.descriptor == k[2] for x in ci.methods):
            continue
        visited.add(k)

    seeds = cj.get('seeds', {})
    _cc.REFLECT_CONSTS.clear()
    for r in cj['reflect']['members']:
        owner, name, _ = _split_id(r['member'])
        _cc.REFLECT_CONSTS.setdefault(owner, set()).add(name)
    for owner, names in seeds.get('reflect_names', {}).items():
        _cc.REFLECT_CONSTS.setdefault(owner, set()).update(names)
    _cc.REFLECT_ALL_MEMBERS.clear()
    _cc.REFLECT_ALL_MEMBERS.update(seeds.get('reflect_all', ()))
    _cc.REFLECT_FIELD_NAMES.clear()
    by_name = {ci.name: ci for ci in jdk_infos}
    for c, m, d in visited:
        ci = by_name.get(c)
        for x in (ci.methods if ci else ()):
            if x.name == m and x.descriptor == d:
                for ins in x.instrs or ():
                    cm = ins.comment or ''
                    if ins.opcode.startswith('ldc') and cm.startswith('String '):
                        s = cm[len('String '):]
                        if s.isidentifier():
                            _cc.REFLECT_FIELD_NAMES.add(s)
    _cc.ANNOTATION_ENUM_SEEDS[:] = seeds.get('annotation_enums', [])
    _cc.MODULE_RESOURCES.clear()
    from .runtime_manifest import module_resource_paths
    for p in module_resource_paths():
        b = resolver.resolve_resource(p)
        if b is not None:
            _cc.MODULE_RESOURCES[p] = b

    # 预检链事实：分析器方法节点即按实例化可达的可执行方法（含边界成员，按精确描述符）
    _cc.PRECHECK_CHAIN['visited'] = {m['id'] for m in cj['methods']}
    _cc.PRECHECK_CHAIN['touched'] = set()

    s = cj['summary']
    print(f"[closure] 类 {s['classes']}（{s['classes_by_level']}）方法 {s['methods']}"
          f"（{s['methods_by_kind']}）耗时 {s['elapsed_ms']} ms")
    for m in cj.get('missing', ()):
        print(f"[closure] 警告：闭包引用的类不存在：{m['name']}")
    if _options.DEBUG:
        for u in cj.get('unresolved', ()):
            print(f"[closure] unresolved: {u}")
    for g in cj['reflect'].get('gaps', ()):
        print(f"[closure] 反射缺口：{g}")
    return jdk_infos, visited, field_stubs

