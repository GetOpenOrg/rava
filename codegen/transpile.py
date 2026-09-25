"""
转译流水线：javac → parse → jdk_scan → codegen → Cargo 项目。

jar 输入模式（--lib）：javac（-cp 全部 jar）→ 用户类解析 + jar 条目枚举进
registry → 方法级 BFS（种子 = 库公开 API 面：全 public 类成员）→ lib crate
发射（crate-type=["lib"]）+ user bin 消费。
"""

import subprocess
import sys
import os
import re
import zipfile
from dataclasses import dataclass, field
from collections import deque
from .classfile import parse_class, parse_class_bytes
from .emitter import write_cargo_project
from .constants import CLASS_CLASS as _CLASS_CLASS
from .callchain import (_JDK_PREFIXES, _JDK_STUB_ONLY_PREFIXES,
                        _read_manifest, _is_boundary_class,
                        _JAVA_RUNTIME_CLASSES, _desc_class_refs,
                        _discover_jdk_classes_method_level)

_ACC_PUBLIC = 0x0001


@dataclass
class LibSpec:
    """一个 lib crate 的 jar 输入规格（main.py --lib 解析产物）。

    seed_classes 为 None = 整包模式（M1 hamcrest）：jar 全部类发射进 crate，
    BFS 种子 = 全部类的 public 成员（库的公开 API 面，access_flags 驱动）。
    seed_classes 非空 = 子集模式（M2 junit Assert 子集）：只有种子类可达闭包
    内的 jar 类进 crate，Runner/annotation 族等不被触达的类显式不入。
    """
    crate_name: str
    jar_path: str
    seed_classes: list[str] | None = None


def _load_jar_registry(jar_path: str) -> dict:
    """枚举 jar 条目 → {binary name → ClassInfo}（条目名排序，确定性）。

    module-info.class 跳过（非类成员）。与 dep_scan._iter_classfiles 同一
    遍历模式（jar 遍历已趟过的路径）。
    """
    registry: dict = {}
    with zipfile.ZipFile(jar_path) as zf:
        for entry in sorted(zf.namelist()):
            if not entry.endswith('.class') or entry.endswith('module-info.class'):
                continue
            name = entry[:-len('.class')]
            try:
                registry[name] = parse_class_bytes(zf.read(entry), name)
            except (ValueError, OSError) as e:
                print(f"[jar] 跳过无法解析的条目 {entry}: {e}")
    return registry


def _jar_package_prefixes(registry: dict) -> tuple[str, ...]:
    """jar 全部类的包前缀（精确到包，排序确定）：BFS 各通道的 lib 前缀闸。"""
    pkgs = {'/'.join(name.split('/')[:-1]) for name in registry if '/' in name}
    return tuple(sorted(pkgs))


def transpile(java_files: list[str], out_dir: str, batch_bin: bool = False,
              lib_specs: list[LibSpec] | None = None, locales: tuple[str, ...] = ()):
    for jf in java_files:
        if not os.path.exists(jf):
            sys.exit(f"File not found: {jf}")

    lib_specs = lib_specs or []

    # .class 统一输出到 <源码目录>/classes/，与 .java 隔离
    src_dir   = os.path.dirname(os.path.abspath(java_files[0]))
    class_dir = os.path.join(src_dir, 'classes')
    os.makedirs(class_dir, exist_ok=True)

    # 1. javac（JAVA_HOME 同源解析，与 jdk_resolver 的语料保持同一 JDK）；
    #    jar 模式下 jar 全部上 classpath（用户 main 消费库 API 的编译期真源）
    _javac = 'javac'
    _home = os.environ.get('JAVA_HOME', '')
    if _home and os.path.exists(os.path.join(_home, 'bin', 'javac')):
        _javac = os.path.join(_home, 'bin', 'javac')
    # 预览语法（如 JDK21 未命名变量 `_`，JEP 443/456）需 --enable-preview，且该
    # 标志必须与 --release/-source 同用；major 取当前 javac 自身版本号，与非预览
    # 编译行为一致（--release N == 默认源/目标平台），仅放开预览语法面。
    _ver = subprocess.run([_javac, '-version'], capture_output=True, text=True)
    m = re.search(r'(\d+)', _ver.stdout or _ver.stderr or '')
    _preview_args: list[str] = []
    if m and int(m.group(1)) >= 14:
        _preview_args = ['--enable-preview', '--release', m.group(1)]
    _cp_args: list[str] = []
    if lib_specs:
        _cp_args = ['-cp', os.pathsep.join(spec.jar_path for spec in lib_specs)]
    print(f"[1/4] {_javac} {' '.join(_cp_args + java_files)}")
    r = subprocess.run([_javac, '-g'] + _preview_args + _cp_args + ['-d', class_dir] + java_files,
                       capture_output=True, text=True)
    if r.returncode != 0:
        sys.exit(f"javac failed:\n{r.stderr}")

    # 2. 解析用户 .class 二进制：编译单元闭包（入口类 + 同源文件的全部类）
    #    javac 把同一 .java 的全部类输出到 classes/：入口类、其嵌套类（Outer$Inner、
    #    匿名/局部类 Outer$1）、以及同文件的兄弟顶层类（含非 public，如 ShapeBase）
    #    及其嵌套类。判据是字节码 SourceFile 属性 == 源文件名（规则一：字节码是
    #    唯一数据源）。classes/ 目录按 feature 目录共享，历史运行会混入其他编译
    #    单元的 .class，不能整目录收编，必须按 SourceFile 过滤。
    class_infos = []
    _seen_class_files: set[str] = set()
    for jf in java_files:
        source_name = os.path.basename(jf)
        class_name = os.path.splitext(source_name)[0]
        class_file = os.path.join(class_dir, class_name + '.class')

        print(f"[2/4] 解析 {class_name}.class")
        ci = parse_class(class_file)
        print(f"      字段: {[f.name for f in ci.fields]}")
        print(f"      方法: {[m.name for m in ci.methods]}")
        class_infos.append(ci)
        _seen_class_files.add(class_file)

        # 同编译单元类发现：SourceFile 属性与源文件一致的全部 .class
        for fname in sorted(os.listdir(class_dir)):
            if not fname.endswith('.class'):
                continue
            unit_file = os.path.join(class_dir, fname)
            if unit_file in _seen_class_files:
                continue
            try:
                unit_ci = parse_class(unit_file)
            except (ValueError, OSError):
                continue   # 共享目录里的历史产物/损坏文件，不属于本编译单元
            if unit_ci.source_file != source_name:
                continue
            _seen_class_files.add(unit_file)
            class_infos.append(unit_ci)
            print(f"      同文件类: {unit_ci.name} "
                  f"(字段: {[f.name for f in unit_ci.fields]}, "
                  f"方法: {[m.name for m in unit_ci.methods]})")

    # 2b. jar 输入模式：枚举 lib crate 的 jar 条目 → registry。
    #     种子统一走可达性通道（lib 类不进用户类通道——用户通道全量翻译、
    #     不参与虚分派传播，库语义不符）：整包模式种子 = jar 全部类，
    #     子集模式种子 = seed_classes；均收敛到 public 成员（callchain 侧）。
    _lib_registries: list[dict] = []
    _lib_prefixes: tuple[str, ...] = ()
    _lib_seed_classes: list[str] = []
    _crate_of: dict[str, str] = {}   # jar 类 binary name → crate 名（闭包拆分用）
    for spec in lib_specs:
        reg = _load_jar_registry(spec.jar_path)
        if not reg:
            sys.exit(f"jar 中未找到类条目: {spec.jar_path}")
        print(f"[jar] {spec.crate_name} ← {os.path.basename(spec.jar_path)}"
              f"（{len(reg)} 类）")
        for name in reg:
            _crate_of[name] = spec.crate_name
        _lib_registries.append(reg)
        _lib_prefixes += _jar_package_prefixes(reg)
        if spec.seed_classes is None:
            _lib_seed_classes.extend(sorted(reg))
        else:
            for fqn in spec.seed_classes:
                if fqn not in reg:
                    sys.exit(f"--lib seed 类 {fqn} 不在 {spec.jar_path} 中")
            _lib_seed_classes.extend(spec.seed_classes)
    _lib_prefixes = tuple(sorted(set(_lib_prefixes)))

    # 3. 方法级调用链 BFS 发现 JDK 类（jar 类注册表优先于 jmods 解析）
    print(f"[3/4] 扫描 JDK 类引用...", end=' ', flush=True)
    jdk_class_infos, visited_methods, field_stubs = _discover_jdk_classes_method_level(
        class_infos,
        runtime_src=os.path.join(out_dir, 'java_runtime', 'src'),
        lib_registries=_lib_registries,
        lib_prefixes=_lib_prefixes,
        extra_seed_classes=_lib_seed_classes or None,
        locales=tuple(locales))

    # jar 模式闭包拆分：BFS 发现集里归属 jar 的类拆到对应 lib crate（整包模式
    # 再并入 jar 全集——wholesale 语义），其余（java/ javax/ …）留在 java_runtime。
    # dict 按 lib_specs 声明序预置——crate 依赖方向（后面的 path 依赖前面的）
    # 是用户声明语义，不能由 BFS 发现序（哪只 jar 的类先被触达）决定。
    _lib_crate_classes: dict[str, list] = {}
    if lib_specs:
        _lib_crate_classes = {spec.crate_name: [] for spec in lib_specs}
        _jdk_only: list = []
        for ci in jdk_class_infos:
            _crate = _crate_of.get(ci.name)
            if _crate is not None:
                _lib_crate_classes.setdefault(_crate, []).append(ci)
            else:
                _jdk_only.append(ci)
        jdk_class_infos = _jdk_only
        for spec in lib_specs:
            if spec.seed_classes is None:
                _discovered = {ci.name for ci in _lib_crate_classes.get(spec.crate_name, [])}
                _whole = _lib_registries[lib_specs.index(spec)]
                assert _discovered <= set(_whole), 'wholesale 可达类必须是 jar 全集子集'
                _lib_crate_classes[spec.crate_name] = [
                    _whole[n] for n in sorted(_whole)]
            else:
                _lib_crate_classes[spec.crate_name] = sorted(
                    _lib_crate_classes.get(spec.crate_name, []), key=lambda c: c.name)
        for _crate, _classes in _lib_crate_classes.items():
            _api_n = sum(1 for ci in _classes
                         if any(m.access_flags & _ACC_PUBLIC for m in ci.methods))
            print(f"      lib {_crate}: {len(_classes)} 类（public API 类 {_api_n}）")
    field_stub_count = sum(1 for ci in jdk_class_infos
                           if any(ci.name == cls for cls in field_stubs))
    bfs_count = len(jdk_class_infos) - field_stub_count
    print(f"完成，{bfs_count} 个调用链类 + {field_stub_count} 个 field stub = {len(jdk_class_infos)} 个")

    # 写 JDK 扫描报告
    project_root = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
    reports_dir = os.path.join(project_root, 'docs', 'reports')
    os.makedirs(reports_dir, exist_ok=True)
    stem0 = os.path.splitext(os.path.basename(java_files[0]))[0]
    report_path = os.path.join(reports_dir, f"jdk-scan-{stem0}.md")
    _write_jdk_scan_report(report_path, jdk_class_infos, field_stubs)
    print(f"      JDK 扫描报告 → {report_path}")

    # 4. 生成 Rust（jar 模式附 lib_crate_classes：lib crate 发射）
    print(f"[4/4] 生成 Rust → {out_dir}/")
    write_cargo_project(out_dir, class_infos, jdk_class_infos, java_files,
                        batch_bin=batch_bin, visited_methods=visited_methods,
                        lib_crate_classes=_lib_crate_classes or None)
    print(f"\n✓ 完成。运行方式：\n  cd {out_dir} && cargo run --release")


def _write_jdk_scan_report(path: str, jdk_class_infos: list, field_stubs: set):
    """将 JDK 扫描结果写成 Markdown 报告。"""
    from datetime import date
    field_stub_names = {ci.name for ci in jdk_class_infos if ci.name in field_stubs}
    callchain_infos = [ci for ci in jdk_class_infos if ci.name not in field_stubs]
    with open(path, 'w', encoding='utf-8') as f:
        f.write(f"# JDK 扫描报告\n\n生成时间：{date.today()}\n\n")
        f.write("## 摘要\n\n")
        f.write(f"| 来源 | 类数 |\n|------|-----:|\n")
        f.write(f"| 调用链 BFS | {len(callchain_infos)} |\n")
        f.write(f"| Field-only stub | {len(field_stub_names)} |\n")
        f.write(f"| 合计 | {len(jdk_class_infos)} |\n\n")
        f.write("## 调用链 BFS 发现的类\n\n")
        f.write("| 类名 | 方法数 | native 数 |\n|------|-------:|----------:|\n")
        for ci in sorted(callchain_infos, key=lambda c: c.name):
            native = sum(1 for m in ci.methods if m.is_native)
            f.write(f"| `{ci.name}` | {len(ci.methods)} | {native} |\n")
        f.write("\n## Field-only Stub 类\n\n")
        f.write("| 类名 | 方法数 | native 数 |\n|------|-------:|----------:|\n")
        for ci in sorted((ci for ci in jdk_class_infos if ci.name in field_stubs),
                         key=lambda c: c.name):
            native = sum(1 for m in ci.methods if m.is_native)
            f.write(f"| `{ci.name}` | {len(ci.methods)} | {native} |\n")
