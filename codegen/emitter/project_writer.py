"""
Cargo workspace 写出：_write、_update_user_lib_rs、_append_cargo_bin、write_cargo_project。
"""

from ..type_map import short_cls as _short_cls_g, configure_short_names as _configure_short_names
import os
import re
from ..types import ClassInfo
from ..type_map import short_cls
from ..constants import RUST_KEYWORDS as _RUST_KEYWORDS
from ..constants import RUNTIME_MACROS_CRATE as _MACROS_CRATE
from ..constants import RUNTIME_JAVA_RUNTIME as _RUNTIME_JAVA_RUNTIME
from ..constants import scratch_pkg_version as _scratch_pkg_version
from .attrs import to_snake, pkg_from_java
from .method_gen import _scan_impl_files
from .import_gen import collect_referenced
from .class_writer import _gen_class_rs
from .inherited_gen import ClassEmission, resolve_inherited_members
from .interface_gen import resolve_interface_impls, resolve_interface_inherited_members
from .. import inherited_calls as _inherited_calls
from ..instr.member_naming import LAMBDA_NAME_LEDGER


_RUNTIME_JRT_SRC = os.path.join(_RUNTIME_JAVA_RUNTIME, 'src')
_JRT_SRC_SEP = 'java_runtime' + os.sep + 'src' + os.sep


def _is_handwritten(path: str) -> bool:
    """java_runtime/src/ 下的 .rs 是否为手写文件。

    判定依据（CLAUDE.md：「手写代码唯一真源在 runtime/」）：overlay 把
    `runtime/java_runtime/src/**` 复制进 scratch，**同相对路径是否存在于
    runtime/java_runtime/src/** 才是手写的充要条件**。

    旧实现按「文件内容是否含 `java_rta_macros::java_class` 标记」判定，有致命缺陷：
    历史版本生成的存根文件（如 PhantomData struct 形态的 `Function`）不含该标记，
    会被永久误判为手写文件 → codegen 永不刷新它们。这些文件停留在旧形态（缺
    `From<Object>` / `Into<Object>` 等转换 impl），编译期表现为大面积
    `X: From<Object> is not satisfied`，与真实 codegen 缺口难以区分。
    """
    _basename = os.path.basename(path)
    if not (path.endswith('.rs') and _JRT_SRC_SEP in path):
        return False
    if _basename in ('mod.rs', 'lib.rs'):
        return False
    # scratch 中 java_runtime/src/ 之后的相对路径 → 在 runtime/ 真源中查找
    _idx = path.rfind(_JRT_SRC_SEP)
    if _idx < 0:
        return False
    _rel = path[_idx + len(_JRT_SRC_SEP):]
    return os.path.exists(os.path.join(_RUNTIME_JRT_SRC, _rel))


# 本轮 write_cargo_project 实际落盘（或内容相同跳过落盘）的生成文件路径。
# 用于区分「本轮生成」与「同 scratch 上次运行的幸存文件」（见 _write_jdk_mod_tree 的清除段）。
_WRITTEN_THIS_RUN: set[str] = set()

# 共置手写 impl（K-3a/K-4）编译期硬引用的语料条件生成类：impl 文件名 → 同目录
# 依赖文件元组。companion 声明以此为准——依赖不齐时 impl 整体不参与编译（其
# 服务的原生方法回落 panic 存根），而不是产生无法解析的 mod 声明拖垮 scratch。
# MemberName/MethodType 经 owner 的 Java 签名闭包通常恒在场，防御性列入。
# 登记须与 impl 的实际 import 同步：var_handle_impl.rs 曾硬引用三 flavor 类
# （FieldInstanceReadOnly 三族 try_cast 目标），85e051d 改为按运行时类名路由后
# 只 import VarHandle / Unsafe——过期登记在 JDK25（闭包不含 flavor 类）把整个
# impl 排除，签名多态方法全部缺席（E0599 compareAndSet/getVolatile/…），已删除。
_IMPL_FILE_DEPS: dict[str, tuple[str, ...]] = {
    'method_handle_natives_impl.rs': (
        'member_name.rs',
        'method_type.rs',
    ),
}


def _write(path: str, content: str) -> None:
    """创建目录并写文件。
    对 java_runtime/src/ 下的 .rs，若同相对路径存在于 runtime/java_runtime/src/
    （= 手写真源），则保留不覆盖（见 `_is_handwritten`）。
    mod.rs / lib.rs / user/ 下文件始终正常写入。
    """
    if _is_handwritten(path):
        return  # 手写文件，不覆盖
    os.makedirs(os.path.dirname(path) or '.', exist_ok=True)
    if os.path.exists(path):
        try:
            with open(path, encoding='utf-8') as _ef:
                if _ef.read() == content:
                    _WRITTEN_THIS_RUN.add(path)  # 内容相同视为本轮产物
                    return  # 内容相同，跳过写入保留 mtime
        except Exception:
            pass
    _WRITTEN_THIS_RUN.add(path)
    # 孤立代理项（常量池解码保留）只允许经 String::from_utf16_lit 码元形态进入源码；
    # 其余位置残留的一律替换为 U+FFFD 落盘（Rust 源文件必须是合法 UTF-8，防转译崩溃）
    if any(0xD800 <= ord(ch) <= 0xDFFF for ch in content):
        content = ''.join('\ufffd' if 0xD800 <= ord(ch) <= 0xDFFF else ch for ch in content)
    with open(path, 'w', encoding='utf-8') as f:
        f.write(content)


def _update_user_lib_rs(user_src: str, new_mods: list[str]) -> None:
    """追加新的 pub mod 声明到 user/src/lib.rs（重复则跳过）。"""
    lib_path = os.path.join(user_src, 'lib.rs')
    header = '#![allow(unused_variables, unused_mut, dead_code, non_snake_case, unused_imports, non_camel_case_types)]'
    existing: set[str] = set()
    if os.path.exists(lib_path):
        with open(lib_path) as f:
            for line in f:
                m = re.match(r'pub mod (\w+);', line)
                if m:
                    existing.add(m.group(1))
    to_add = [m for m in new_mods if m not in existing]
    if not to_add:
        return
    all_mods = sorted(existing | set(new_mods))
    lines = [header] + [f'pub mod {m};' for m in all_mods] + ['']
    _write(lib_path, '\n'.join(lines))


def _append_cargo_bin(user_dir: str, bin_name: str, bin_src: str) -> None:
    """向 user/Cargo.toml 追加一个 [[bin]] 条目（已存在则跳过）。"""
    cargo_path = os.path.join(user_dir, 'Cargo.toml')
    base = '\n'.join([
        '[package]', 'name = "user"',
        f'version = "{_scratch_pkg_version(user_dir)}"', 'edition = "2021"', '',
        '[dependencies]',
        'java_runtime    = { path = "../java_runtime" }',
        f'java_rta_macros = {{ path = "{_MACROS_CRATE}" }}',
        '',
    ])
    new_bin = f'\n[[bin]]\nname = "{bin_name}"\npath = "{bin_src}"\n'
    if not os.path.exists(cargo_path):
        dep_idx = base.index('[dependencies]')
        _write(cargo_path, base[:dep_idx] + new_bin + '\n' + base[dep_idx:])
        return
    with open(cargo_path) as f:
        content = f.read()
    if f'name = "{bin_name}"' in content:
        return
    dep_idx = content.find('[dependencies]')
    if dep_idx >= 0:
        content = content[:dep_idx] + new_bin + '\n' + content[dep_idx:]
    else:
        content += new_bin
    with open(cargo_path, 'w') as f:
        f.write(content)


def write_cargo_project(out_dir: str, class_infos: list[ClassInfo],
                         jdk_class_infos: list[ClassInfo] | None = None,
                         java_files: list[str] | None = None,
                         batch_bin: bool = False,
                         visited_methods: set | None = None,
                         lib_crate_classes: dict[str, list[ClassInfo]] | None = None):
    """
    生成 Cargo workspace，包含两个子 crate：
      java_runtime/  — VM 基础设施 + JDK 字节码翻译（合并，git 管理基础设施部分）
      user/          — 用户 Java 代码翻译

    lib_crate_classes（jar 输入模式的 --lib 发射）：{crate 名 → 类列表}。
    每个 lib crate 发射为 crate-type=["lib"] 的独立 crate（hamcrest / junit4）：
    lib.rs 汇出模块树、Java 可见性映射（public/protected→pub、其余→pub(crate) 近似，
    access_flags 驱动）、引用按目标 crate 定向（junit4→hamcrest→java_runtime）。
    依赖方向 = 字典插入序（后面的 crate path 依赖前面的）。
    """
    import shutil

    rt_dir   = os.path.join(out_dir, 'java_runtime')
    jdk_dir  = os.path.join(out_dir, 'java_runtime')  # JDK 翻译直接写入 java_runtime/
    user_dir = os.path.join(out_dir, 'user')

    # JDK 字节码翻译输出到 java_runtime/src/
    jdk_src = os.path.join(jdk_dir, 'src')

    # scratch 语义（per-test scratch workspace）：out_dir 是一次性工作区，
    # 手写文件已由脚本层（main.py）从 runtime/ overlay 进来，生成文件全部
    # 可再生。因此这里**不做任何清理与保护**：
    #   - 旧的 _PERMANENT 硬编码集合、git ls-files 探测、生成标记判别、
    #     非 batch 清理分支已全部删除
    #   - 手写文件不被生成内容覆盖的保证由 _write() 的标记检查提供
    #     （scratch 里手写文件无 java_rta_macros::java_class 标记 → 跳过写入）
    #   - scratch 的清空/复用策略由脚本层决定（main.py --clean）

    # 两阶段生成：先生成全部类文本（期间调用点登记继承成员需求），
    # 再统一补上继承成员声明后落盘（见 inherited_gen.py）
    _inherited_calls.reset()
    LAMBDA_NAME_LEDGER.reset()
    from . import sam_objects as _sam_objects
    _sam_objects.reset()
    from . import annotation_objects as _anno_objects
    _anno_objects.reset()
    from . import dispatch_gen as _dispatch_gen
    _dispatch_gen.reset()
    _WRITTEN_THIS_RUN.clear()
    emissions: dict[str, ClassEmission] = {}

    # 构建 registry（用户类 + lib crate 类 + JDK 类）
    registry: dict = {ci.name: ci for ci in class_infos}
    for _lc_classes in (lib_crate_classes or {}).values():
        for _lci in _lc_classes:
            registry.setdefault(_lci.name, _lci)
    if jdk_class_infos:
        for jci in jdk_class_infos:
            registry.setdefault(jci.name, jci)
    _configure_short_names(registry)

    # 扫描 jdk_classes/src/**/*_impl.rs，构建 new_format_map（已手写方法 → codegen 跳过 stub）
    # 传入 registry 使 _scan_impl_files 能通过 registry 解析嵌套类的真实 binary_name（如 HashMap$TreeNode）
    new_format_map, full_impl_classes = _scan_impl_files(out_dir, registry=registry)

    # A-5 预扫描：本轮全部 invokedynamic 站点的 samtype → 可合成函数式接口集。
    # 必须先于类文本生成：站点（sim/dynamic.py）发射时即需判定合成对象装箱
    # 还是回落闭包装箱，合成可行性（接口发射/手写覆盖/函数式）此刻定案。
    _sam_objects.prescan(registry, jdk_class_infos or [], class_infos,
                         full_impl_classes,
                         lib_crate_classes=lib_crate_classes)

    # 写 JDK 翻译文件，构建 jdk mod 树
    jdk_mod_tree: dict[str, set[str]] = {}
    if jdk_class_infos:
        # 先确定哪些 snake_cased 包名会成为目录（用于检测类/包名冲突）
        pkg_dir_names: dict[str, set[str]] = {}  # parent_dir → set of pkg subdir names
        for jdk_ci in jdk_class_infos:
            pkg_parts = jdk_ci.name.split('/')[:-1]
            parent = jdk_src
            for part in pkg_parts:
                pkg_dir_names.setdefault(parent, set()).add(part)
                parent = os.path.join(parent, part)

        # 先收集所有翻译包的 crate 路径（用于 cross-module glob import）
        # 关键字包名用 r# 转义（如 java::lang::r#ref）
        def _safe_pkg_part(p: str) -> str:
            return f'r#{p}' if p in _RUST_KEYWORDS else p

        # jdk/ 内部实现类不加入全局跨包 glob 导入，避免大量命名冲突
        _SKIP_GLOBAL_IMPORT_PREFIXES = ('jdk/',)
        jdk_pkg_set: set[str] = set()
        for jdk_ci in jdk_class_infos:
            if jdk_ci.name.startswith(_SKIP_GLOBAL_IMPORT_PREFIXES):
                continue
            pkg_parts = jdk_ci.name.split('/')[:-1]
            if pkg_parts:
                jdk_pkg_set.add('::'.join(_safe_pkg_part(p) for p in pkg_parts))
        jdk_crate_pkg_paths = sorted(jdk_pkg_set)

        # 构建冲突消歧表：同一简单类名出现在多个包中时，需要在每个文件里用显式 use 覆盖
        from collections import defaultdict as _defaultdict
        _sn_to_pkgs: dict[str, list[str]] = _defaultdict(list)
        # skipped_classes：被跳过全局导入、但实际已生成 stub 的类的完整 rust 路径集合
        # 格式：{"jdk::internal::util::StaticProperty", ...}
        # 只包含确实生成了 stub 文件的类（防止 E0432：引用了不存在的 use 路径）
        skipped_classes: set[str] = set()
        for jdk_ci in jdk_class_infos:
            _parts = jdk_ci.name.split('/')
            if len(_parts) >= 2:
                _sn_to_pkgs[_parts[-1]].append('/'.join(_parts[:-1]))
                if jdk_ci.name.startswith(_SKIP_GLOBAL_IMPORT_PREFIXES):
                    _rust_pkg = '::'.join(_safe_pkg_part(x) for x in _parts[:-1])
                    # Java 内部类 $ → Rust struct 名用 _
                    _simple_cls = _short_cls_g(jdk_ci.name)
                    skipped_classes.add(f"{_rust_pkg}::{_simple_cls}")
        _jdk_pkg_path_set = set(jdk_crate_pkg_paths)
        conflict_map: dict[str, list[str]] = {}
        for _sn, _pkgs in _sn_to_pkgs.items():
            _in_scope = list({p for p in _pkgs
                              if '::'.join(_safe_pkg_part(x) for x in p.split('/')) in _jdk_pkg_path_set})
            if len(_in_scope) >= 2:
                conflict_map[_sn] = _in_scope

        # 预计算实际生成的类集合，用于过滤 cross_imports（避免为不存在的类型生成 use）
        _generated_jdk_names: set[str] = {ci.name for ci in jdk_class_infos}

        for jdk_ci in jdk_class_infos:
            parts = jdk_ci.name.split('/')          # e.g. ['java','util','ArrayList']
            *pkg_parts, class_name = parts
            mod_name  = to_snake(class_name)
            parent_dir = os.path.join(jdk_src, *pkg_parts)
            # 类名与子包目录同名时（E0761：module.rs 和 module/mod.rs 不能共存），
            # 改用 module_t.rs 文件名（类型名仍是 Module）
            if mod_name in pkg_dir_names.get(parent_dir, set()):
                mod_name = mod_name + '_t'
            file_path = os.path.join(parent_dir, mod_name + '.rs')
            # 手写文件（如 object.rs，由脚本层从 runtime/ overlay 进 scratch）不含
            # 生成标记，_write 会跳过写入；mod 树照常登记，保证 mod.rs 声明该模块
            # 调用链上的非 native 方法翻译字节码，调用链外的方法生成 panic! 存根
            # new_format_map 中已有 _impl.rs 实现的方法，codegen 跳过那些方法的 stub 生成
            _em = ClassEmission(binary_name=jdk_ci.name, crate_prefix='crate', path=file_path,
                                handwritten=_is_handwritten(file_path), crate_name='java_runtime')
            _em.text = _gen_class_rs(jdk_ci, registry=registry,
                                     jdk_crate_pkg_paths=jdk_crate_pkg_paths,
                                     call_chain=visited_methods,
                                     new_format_map=new_format_map,
                                     workspace_root=out_dir,
                                     full_impl_classes=full_impl_classes,
                                     conflict_map=conflict_map,
                                     skipped_classes=skipped_classes,
                                     generated_classes=_generated_jdk_names,
                                     emission=_em)
            emissions[jdk_ci.name] = _em
            # 更新 mod 树
            parent = jdk_src
            for part in pkg_parts:
                jdk_mod_tree.setdefault(parent, set()).add(part)
                parent = os.path.join(parent, part)
                jdk_mod_tree.setdefault(parent, set()).add(mod_name)

    # ── lib crate 发射（jar 输入模式：--lib）───────────────────────────────
    # 每个 lib crate 一个 crate-type=["lib"] 的独立 crate：类文件 + lib.rs 模块树
    # + 可见性映射 + 按目标 crate 定向的引用。依赖方向 = 字典插入序。
    _lib_sets: dict[str, set[str]] = {}
    if lib_crate_classes:
        _jdk_generated_now = _generated_jdk_names if jdk_class_infos else set()
        for _lc_name, _lc_classes in lib_crate_classes.items():
            _lib_sets[_lc_name] = {ci.name for ci in _lc_classes}

        _lib_order = list(lib_crate_classes or {})

        def _make_lib_resolver(current_crate: str):
            def _resolve(bin_name: str) -> str:
                for _cname, _names in _lib_sets.items():
                    if bin_name in _names:
                        return 'crate' if _cname == current_crate else _cname
                return 'java_runtime'
            # 声明序元数据（gen_cross_imports 的可达性过滤）：发射 crate 只能
            # 引用自身 + java_runtime + 声明序在前的 lib crate（依赖方向按
            # 声明序）。反向跨 crate 引用（如 hamcrest 文件的子类型收集捡到
            # junit4 类——非 JDK 接口实现者不在 _JDK_NS 过滤域内）是 M5 跨
            # crate 分派边界，导入即 E0433，在此显式过滤。
            _resolve._rfl_current_crate = current_crate
            _resolve._rfl_lib_order = _lib_order
            return _resolve

        _lib_generated_all = set(_jdk_generated_now)
        for _names in _lib_sets.values():
            _lib_generated_all |= _names

        for _lc_name, _lc_classes in lib_crate_classes.items():
            _lc_dir = os.path.join(out_dir, _lc_name)
            _lc_src = os.path.join(_lc_dir, 'src')
            _lc_resolver = _make_lib_resolver(_lc_name)
            # E0761 预检：snake 类名与同目录子包目录同名时改 _t 后缀（与 JDK 循环同规则）
            _lc_pkg_dirs: dict[str, set[str]] = {}
            for _lc_ci in _lc_classes:
                _parent = _lc_src
                for _part in _lc_ci.name.split('/')[:-1]:
                    _lc_pkg_dirs.setdefault(_parent, set()).add(_part)
                    _parent = os.path.join(_parent, _part)
            for _lc_ci in sorted(_lc_classes, key=lambda c: c.name):
                *_lc_pkg_parts, _lc_cls_name = _lc_ci.name.split('/')
                _lc_mod = to_snake(_lc_cls_name)
                _lc_parent_dir = os.path.join(_lc_src, *_lc_pkg_parts)
                if _lc_mod in _lc_pkg_dirs.get(_lc_parent_dir, set()):
                    _lc_mod = _lc_mod + '_t'
                _lc_path = os.path.join(_lc_parent_dir, _lc_mod + '.rs')
                _em = ClassEmission(binary_name=_lc_ci.name, crate_prefix='java_runtime',
                                    path=_lc_path, crate_name=_lc_name)
                _em.text = _gen_class_rs(
                    _lc_ci, registry=registry,
                    user_crate_prefix='java_runtime',
                    call_chain=visited_methods,
                    new_format_map=new_format_map,
                    workspace_root=out_dir,
                    full_impl_classes=full_impl_classes,
                    generated_classes=_lib_generated_all,
                    emission=_em,
                    crate_prefix_resolver=_lc_resolver,
                    java_visibility=True)
                emissions[_lc_ci.name] = _em
            # lib.rs：顶层包模块树（sorted 确定性；_mod_decl 定义在函数后段，此处内联同规则）
            _lc_top_pkgs = sorted({ci.name.split('/')[0]
                                   for ci in _lc_classes if '/' in ci.name})
            _lib_rs = ['#![allow(unused_variables, unused_mut, dead_code, '
                       'non_snake_case, unused_imports, non_camel_case_types, '
                       'non_upper_case_globals, static_mut_refs, ambiguous_glob_reexports)]']
            _lib_rs += [f'pub mod {"r#" + p if p in _RUST_KEYWORDS else p};'
                        for p in _lc_top_pkgs]
            _write(os.path.join(_lc_src, 'lib.rs'), '\n'.join(_lib_rs) + '\n')
            # Cargo.toml：crate-type=["lib"]，依赖前面的 lib crate（插入序）
            _lc_deps = ['java_runtime    = { path = "../java_runtime" }',
                        f'java_rta_macros = {{ path = "{_MACROS_CRATE}" }}']
            for _prev in lib_crate_classes:
                if _prev == _lc_name:
                    break
                _lc_deps.append(f'{_prev:<15} = {{ path = "../{_prev}" }}')
            _write(os.path.join(_lc_dir, 'Cargo.toml'), '\n'.join([
                '[package]',
                f'name = "{_lc_name}"',
                f'version = "{_scratch_pkg_version(_lc_dir)}"',
                'edition = "2021"',
                '',
                '[lib]',
                f'name = "{_lc_name}"',
                'path = "src/lib.rs"',
                'crate-type = ["lib"]',
                '',
                '[dependencies]',
                *_lc_deps,
                '',
                '[lints.rust]',
                'unused_parens = "allow"',
                'unused_braces = "allow"',
                'dead_code = "allow"',
                'unused_assignments = "allow"',
                'unused_variables = "allow"',
                'unused_mut = "allow"',
                'unused_imports = "allow"',
                'non_snake_case = "allow"',
                'non_camel_case_types = "allow"',
                'non_upper_case_globals = "allow"',
                'unreachable_code = "allow"',
                'unreachable_patterns = "allow"',
                '',
            ]))

    def _mod_decl(name: str) -> str:
        """生成 pub mod 声明，对 Rust 关键字用 r# 转义。"""
        safe = f'r#{name}' if name in _RUST_KEYWORDS else name
        return f'pub mod {safe};'

    def _use_decl(name: str) -> str:
        """生成 pub use *::* 声明，对 Rust 关键字用 r# 转义。"""
        safe = f'r#{name}' if name in _RUST_KEYWORDS else name
        return f'pub use {safe}::*;'

    def _write_jdk_mod_tree(src_root: str | None = None) -> None:
        """类文件全部落盘后调用：mod.rs 如实声明磁盘上的全部模块。

        src_root 参数化（lib crate 的 src 复用同一逻辑：陈旧清扫 / 磁盘重建 /
        companion 声明 / glob 再导出），默认 java_runtime/src（既有行为不变）。"""
        _src_root = jdk_src if src_root is None else src_root
        # 陈旧生成文件清除：带生成标记（java_rta_macros::java_class）、但本轮未写入的
        # .rs 是同 scratch 上次运行的幸存者。若不清除，下方的磁盘扫描会把它们的模块
        # 声明重新挂进 mod.rs，与手写 companion（E0592，如 unsafe_.rs + unsafe__impl.rs）
        # 或本轮闭包冲突。手写文件无生成标记，不受影响。
        for _root_sweep, _dirs_sweep, _files_sweep in os.walk(_src_root):
            for _fname_sweep in _files_sweep:
                if not _fname_sweep.endswith('.rs') or _fname_sweep in ('lib.rs', 'mod.rs'):
                    continue
                _fpath_sweep = os.path.join(_root_sweep, _fname_sweep)
                if _fpath_sweep in _WRITTEN_THIS_RUN:
                    continue
                try:
                    with open(_fpath_sweep, encoding='utf-8') as _fs_sweep:
                        if 'java_rta_macros::java_class' in _fs_sweep.read():
                            os.remove(_fpath_sweep)
                except Exception:
                    pass  # 读取失败时保守保留，交由 mod 树扫描处理
        # 从磁盘全量重建 mod 树：mod.rs 如实声明磁盘上的全部模块。
        # 磁盘内容 = 手写 overlay（runtime/ 复制进来的 object.rs、function 存根、
        # companion）+ 本次生成的类文件 + 同 scratch 上次运行的幸存文件。
        # 用作用域内的 mod 树直接写 mod.rs 会抹掉其余文件的声明（E0432/E0433）。
        # 扫描收集全部 .rs，自底向上传播目录，只声明有文件的目录（避免 E0583）。
        jdk_mod_tree: dict[str, set[str]] = {}
        if os.path.isdir(_src_root):
            for root, _dirs, files in os.walk(_src_root):
                for fname in files:
                    if not fname.endswith('.rs') or fname in ('lib.rs', 'mod.rs'):
                        continue
                    # _impl.rs / _ext.rs 是共置手写文件，由下方 companion_mods 以私有
                    # `mod X_impl;` 声明。若此处也计入 children，会再生成一条
                    # `pub mod X_impl; pub use X_impl::*;`，导致 E0428（重复定义）
                    # 与 E0592/E0034（glob 重导出歧义）。
                    # 例外：含生成标记的 *_impl.rs 是碰巧命名的生成类（如
                    # Collectors$CollectorImpl → collectors_collector_impl.rs），
                    # 必须计入 children 才有 pub mod 声明（否则 E0425）。
                    if fname.endswith('_impl.rs') or fname.endswith('_ext.rs'):
                        _fpath_scan = os.path.join(root, fname)
                        try:
                            with open(_fpath_scan, encoding='utf-8') as _fs:
                                if 'java_rta_macros::java_class' not in _fs.read():
                                    continue  # 真正手写共置文件，由 companion_mods 声明
                        except Exception:
                            continue  # 读取失败时保守视为手写
                    jdk_mod_tree.setdefault(root, set()).add(fname[:-3])
            _changed = True
            while _changed:
                _changed = False
                for _dp in list(jdk_mod_tree.keys()):
                    if _dp == _src_root:
                        continue
                    _par = os.path.dirname(_dp)
                    _dn  = os.path.basename(_dp)
                    if _dn not in jdk_mod_tree.get(_par, set()):
                        jdk_mod_tree.setdefault(_par, set()).add(_dn)
                        _changed = True

        # java_runtime/src/lib.rs 是手写文件，不覆写。
        # 顶层 pub mod 声明（java/、jdk/ 等）已在 lib.rs 中手动维护。

        # 中间 mod.rs（jdk 子包）：pub mod + pub use *（使 glob import 能拿到类型）
        for dir_path, children in jdk_mod_tree.items():
            if dir_path == _src_root:
                continue
            mod_lines = ['#![allow(ambiguous_glob_reexports)]']
            for c in sorted(children):
                mod_lines.append(_mod_decl(c))
                # 子包（目录）只声明 pub mod，不 glob 重导出：Java 的包之间没有
                # 嵌套可见性，java.util 不包含 java.util.stream 的类。若重导出，
                # 父包与子包的同名类（同一简单名）会在父包命名空间里产生歧义（E0659）。
                if os.path.join(dir_path, c) in jdk_mod_tree:
                    continue
                mod_lines.append(_use_decl(c))
            # K-2: 扫描目录中的 _impl.rs / _ext.rs 共置文件，加入私有 mod 声明。
            # 规则：只有当 X.rs 存在（调用链生成，或手写 overlay 提供）时，
            # 才声明 mod X_impl; / mod X_ext;。否则 _impl.rs 静默等待，避免 E0583 / 未定义类型。
            # 若 X.rs 存在但不在 children（手写 overlay 文件），还需补充 pub mod X; 声明。
            if os.path.isdir(dir_path):
                extra_pub: list[str] = []
                companion_mods: list[str] = []
                for _f in sorted(os.listdir(dir_path)):
                    if _f.endswith('_impl.rs'):
                        base = _f[:-len('_impl.rs')]
                    elif _f.endswith('_ext.rs'):
                        base = _f[:-len('_ext.rs')]
                    else:
                        continue
                    # 含生成标记的 *_impl.rs / *_ext.rs 是类名恰以 Impl/Ext 结尾的生成类
                    # （已在 children 中以 pub mod 声明），不是共置手写文件
                    if _f[:-3] in children:
                        continue
                    # X.rs 不存在时跳过：_impl.rs 静默，不产生无法解析的 mod 声明。
                    # 碰撞后缀类（类名与子包目录同名 → 生成文件 X_t.rs，如
                    # java/lang/Module → module_t.rs）的伴生文件仍按无后缀命名
                    # （module_impl.rs——_scan_impl_files 经 registry 以类名映射），
                    # 存在性判定同时接受 X.rs / X_t.rs 两种宿主形态
                    if not (os.path.exists(os.path.join(dir_path, base + '.rs'))
                            or os.path.exists(os.path.join(dir_path, base + '_t.rs'))):
                        continue
                    # 依赖闭包不齐时跳过：impl 编译期硬引用的语料条件生成类（同目录
                    # sibling）缺席则该 impl 整体不参与编译——等价于该 impl 尚不存在，
                    # 其服务的原生方法回落 panic 存根——而不是让 mod 声明拖着无法
                    # 解析的 import 拖垮整个 scratch（登记表见 _IMPL_FILE_DEPS）。
                    if not all(os.path.exists(os.path.join(dir_path, _dep))
                               for _dep in _IMPL_FILE_DEPS.get(_f, ())):
                        continue
                    companion_mods.append(f'mod {_f[:-3]};')
                    # X.rs 在磁盘但不在 children（如手写 object.rs）→ 补充 pub mod 声明
                    if base not in children:
                        extra_pub.append(base)
                for base in sorted(set(extra_pub)):
                    mod_lines.append(_mod_decl(base))
                    mod_lines.append(_use_decl(base))
                mod_lines.extend(companion_mods)
            _write(os.path.join(dir_path, 'mod.rs'), '\n'.join(mod_lines) + '\n')

    # 4. user crate（用户 Java 翻译）
    user_src = os.path.join(user_dir, 'src')
    # scratch 语义：不做清理。同 scratch 复跑 = 同测试作用域，文件全部被覆写；
    # 换测试 = 换 scratch 目录（脚本层保证）。残留文件不被 lib.rs 声明，无害。

    # 提取包名：按字节码 SourceFile 属性对应源文件。同一编译单元的全部类
    # （入口、嵌套、兄弟顶层类）共享该文件的 package 声明；位置 zip 配对
    # 无法覆盖类发现阶段加入的附加类
    packages: dict[str, str] = {}
    if java_files:
        _pkg_by_source = {os.path.basename(jf): pkg_from_java(jf) for jf in java_files}
        for ci in class_infos:
            _pkg = _pkg_by_source.get(ci.source_file)
            if _pkg is not None:
                packages[ci.name] = _pkg

    # 计算文件路径
    layout: dict[str, tuple] = {}
    for ci in class_infos:
        pkg       = packages.get(ci.name, '')
        pkg_parts = pkg.split('.') if pkg else []
        mod_name  = to_snake(ci.name)
        file_path = os.path.join(user_src, *pkg_parts, mod_name + '.rs')
        layout[ci.name] = (file_path, pkg_parts, mod_name)

    # 构建用户 mod 树
    user_mod_tree: dict[str, set[str]]            = {}
    user_reexport: dict[str, set[tuple[str,str]]] = {}
    for ci in class_infos:
        _, pkg_parts, mod_name = layout[ci.name]
        parent = user_src
        for part in pkg_parts:
            user_mod_tree.setdefault(parent, set()).add(part)
            parent = os.path.join(parent, part)
        user_mod_tree.setdefault(parent, set()).add(mod_name)
        user_reexport.setdefault(parent, set()).add((mod_name, ci.name))

    # 构建每个用户类所需的同 crate 兄弟类导入（use crate::[pkg::]mod::Type）
    # 原则：用户类引用同编译单元的其他用户类（嵌套类、同文件兄弟顶层类——各自
    # 独立成文件）时，类型名须经 use 引入作用域。引用集来自字节码
    # （collect_referenced：超类/接口/描述符/指令注释/局部变量表/分派子类型）；
    # 内部类关系额外经 InnerClasses 属性补充
    _user_layout_names: set[str] = set(layout.keys())
    _sibling_imports: dict[str, list[str]] = {}
    for ci in class_infos:
        seen_imports: set[str] = set()
        imports: list[str] = []

        def _add_import(class_name: str) -> None:
            if class_name not in _user_layout_names or class_name == ci.name:
                return
            _, _pkg_parts, _mod_n = layout[class_name]
            _mod_path = '::'.join(_pkg_parts + [_mod_n])
            type_n = short_cls(class_name)
            line = f"use crate::{_mod_path}::{type_n};"
            if line not in seen_imports:
                seen_imports.add(line)
                imports.append(line)

        # 字节码引用集里的其他用户类（generated_classes=None 不做生成集过滤，
        # JDK 名由 _add_import 的布局成员检查排除）
        for _ref in sorted(collect_referenced(ci, registry, None)):
            _add_import(_ref)

        # 通过 InnerClasses 属性发现该类定义的内部类
        for ic in (ci.inner_classes or []):
            _add_import(ic.inner_class)

        # 同时：内部类本身需要导入其外部类及同级内部类（通过 outer_class 字段）
        outer_class = next(
            (ic.outer_class for ic in (ci.inner_classes or []) if ic.inner_class == ci.name),
            None
        )
        if outer_class and outer_class in _user_layout_names:
            _add_import(outer_class)
            # 同级内部类（同一外部类下的其他内部类）
            outer_ci = next((c for c in class_infos if c.name == outer_class), None)
            if outer_ci:
                for sib_ic in (outer_ci.inner_classes or []):
                    _add_import(sib_ic.inner_class)

        _sibling_imports[ci.name] = imports

    # 写用户类文件
    user_pkg_paths = jdk_crate_pkg_paths if jdk_class_infos else None
    _user_gen_jdk = {ci.name for ci in jdk_class_infos} if jdk_class_infos else None
    # lib 模式：用户类引用 lib crate 类 + JDK 闭包，按目标 crate 定向
    _user_resolver = None
    if lib_crate_classes:
        def _user_resolve(bin_name: str) -> str:
            for _cname, _names in _lib_sets.items():
                if bin_name in _names:
                    return _cname
            return 'java_runtime'
        _user_resolver = _user_resolve
        _user_gen_jdk = (_user_gen_jdk or set()) | _lib_generated_all
        user_pkg_paths = None
    for ci in class_infos:
        file_path, _, _ = layout[ci.name]
        _em = ClassEmission(binary_name=ci.name, crate_prefix='java_runtime',
                            path=file_path, crate_name='user')
        _em.text = _gen_class_rs(ci, registry=registry,
                                 jdk_crate_pkg_paths=user_pkg_paths,
                                 user_crate_prefix='java_runtime',
                                 new_format_map=new_format_map,
                                 workspace_root=out_dir,
                                 full_impl_classes=full_impl_classes,
                                 conflict_map=None if _user_resolver else (conflict_map if jdk_class_infos else None),
                                 skipped_classes=None if _user_resolver else (skipped_classes if jdk_class_infos else None),
                                 user_sibling_imports=_sibling_imports.get(ci.name),
                                 generated_classes=_user_gen_jdk,
                                 emission=_em,
                                 crate_prefix_resolver=_user_resolver)
        emissions[ci.name] = _em

    # G-10 生成期断言：invokedynamic 实现方法的「调用点引用名 ↔ 定义名」恒等，
    # 且被引用方法在生成类中必须存在定义（不一致直接抛错，防止静默生成坏代码）
    LAMBDA_NAME_LEDGER.check()

    # 全部方法体已生成 → 继承成员需求已齐：补声明后统一落盘
    resolve_interface_impls(emissions, registry,
                            {k: set(v.get('methods', set())) for k, v in (new_format_map or {}).items()})
    resolve_interface_inherited_members(emissions, registry)
    resolve_inherited_members(emissions, registry,
                              impl_methods={k: set(v.get('methods', set()))
                                            for k, v in (new_format_map or {}).items()})
    # A-5 收尾：函数式接口合成对象（接口文件尾部的伴生段；条目签名 / default
    # 体有无取此刻的发射记录，与落盘内容同源）
    _sam_objects.synthesize(emissions, registry)
    # 反射 L3 段 1 收尾：注解代理合成（getAnnotation 的实例形态——JDK 动态
    # 代理的翻译期同构物；条目签名同取发射记录）。必须在落盘前；工厂登记行
    # 由 main 生成段经 _anno_objects.registration_lines() 消费。
    _anno_objects.synthesize(emissions, registry)
    # 反射 L3 段 2 收尾：用户类分派闭包（Method.invoke / Constructor.
    # newInstance 的按名协议，java_runtime::reflect_dispatch 头注定稿）。
    # 登记行由 main 生成段经 _dispatch_gen.registration_lines() 消费。
    from ..callchain import REFLECT_CONSTS as _reflect_consts
    _dispatch_gen.synthesize(emissions, registry,
                             user_bins={ci.name for ci in class_infos},
                             reflect_members=_reflect_consts)
    for _em in emissions.values():
        _write(_em.path, _em.text)
    _write_jdk_mod_tree()
    # lib crate 的包 mod 树（同一逻辑：陈旧清扫 / 磁盘重建 / glob 再导出）
    for _lc_name in (lib_crate_classes or {}):
        _write_jdk_mod_tree(os.path.join(out_dir, _lc_name, 'src'))
    _complete_jrt_lib_rs(out_dir)




    # 中间 mod.rs（用户子包）
    for dir_path, children in user_mod_tree.items():
        if dir_path == user_src:
            continue
        mod_lines = [f'pub mod {c};' for c in sorted(children)]
        for mod_name, cls_name in sorted(user_reexport.get(dir_path, set())):
            mod_lines.append(f'pub use {mod_name}::{cls_name};')
        _write(os.path.join(dir_path, 'mod.rs'), '\n'.join(mod_lines) + '\n')

    # user/src/main.rs（单测试模式）或 src/bin/<class>.rs（批量模式）
    main_class = class_infos[0].name if class_infos else 'Main'
    _, pkg_parts, mod_name = layout[main_class]
    top_user_mods = sorted(user_mod_tree.get(user_src, set()))
    use_path = '::'.join(pkg_parts + [main_class]) if pkg_parts else f'{mod_name}::{main_class}'
    bin_name = to_snake(main_class.split('/')[-1])   # snake_case，如 TestArrayList → test_array_list

    # 枚举形态类（自身类型 static 字段——与宏侧常量目录登记同一结构谓词）的
    # class-init 钩子登记：main 启动时登记，运行时反射按名消费方
    # （getEnumConstantsShared / Enum.valueOf）经 ensure_class_initialized 强制
    # 目标类初始化（JVM 反射路径语义；ldc 类字面量保持 init-passive）。
    # 仅用户类：JDK 生成枚举冷反射语料不存在，手写边界类无 __class_init。
    hook_lines: list[str] = []
    for ci in class_infos:
        if not any(f.is_static and f.descriptor == f'L{ci.name};' for f in ci.fields):
            continue
        _, _pkg, _mod = layout[ci.name]
        hook_path = '::'.join(['crate', *_pkg, _mod, short_cls(ci.name)])
        hook_lines.append(
            f'    ("{ci.name}", std::rc::Rc::new(|| {hook_path}::__class_init())),')
    hook_block = ''
    if hook_lines:
        hook_block = ('    java_runtime::register_class_init_hooks(&[\n'
                      + '\n'.join(hook_lines) + '\n    ]);\n')
    # 注解工厂登记（反射 L3 段 1）：全部合成注解代理的 from_values 工厂
    #（用户树 + lib crate + java_runtime 三域——user bin 是唯一能看到全部
    # crate 的发射点；与类初始化钩子同一登记模式）
    _anno_reg_lines = _anno_objects.registration_lines()
    if _anno_reg_lines:
        hook_block += ('    java_runtime::annotation_meta::register_annotation_factories(&[\n'
                       + '\n'.join(_anno_reg_lines) + '\n    ]);\n')
    # L3 分派闭包登记（反射段 2）：用户类 __reflect_dispatch 注册表
    _disp_reg_lines = _dispatch_gen.registration_lines()
    if _disp_reg_lines:
        hook_block += ('    java_runtime::reflect_dispatch::register_method_dispatch(&[\n'
                       + '\n'.join(_disp_reg_lines) + '\n    ]);\n')
    _field_reg_lines = _dispatch_gen.field_registration_lines()
    if _field_reg_lines:
        hook_block += ('    java_runtime::reflect_dispatch::register_field_dispatch(&[\n'
                       + '\n'.join(_field_reg_lines) + '\n    ]);\n')

    # L-1 资源束登记：BFS 按 locale 种子入选的 CLDR 束类（翻译字节码）的构造闭包，
    # 供手写边界 LocaleResources 按候选链装载（替代 ResourceBundle.getBundle 的类名反射）
    from ..callchain import DATA_BUNDLE_SEEDS as _bundle_seeds
    if _bundle_seeds:
        _bundle_lines = []
        for _b in _bundle_seeds:
            _bpath = '::'.join(['java_runtime',
                                *(f'r#{p}' if p in _RUST_KEYWORDS else p for p in _b.split('/')[:-1]),
                                short_cls(_b)])
            _bundle_lines.append(
                f'        ("{_b}", (|| Ok(java_runtime::java::lang::Object::from({_bpath}::new()?)))'
                f' as java_runtime::data_bundles::BundleCtor),')
        hook_block += ('    java_runtime::data_bundles::register_data_bundles(&[\n'
                       + '\n'.join(_bundle_lines) + '\n    ]);\n')

    # K-JCA 服务登记：BFS 按「engine 类在链上 × 用户算法名」入选的服务实现类（翻译字节码）
    # 的构造闭包，供手写边界 sun/security/jca 的服务查找构造（替代 Provider$Service.newInstance
    # 的类名反射）。元组：(类型, 算法, 实现类 binary name, provider 名, 构造闭包)。
    from ..callchain import JCA_SEEDS as _jca_seeds
    if _jca_seeds:
        _jca_lines = []
        for _sv in _jca_seeds:
            _jpath = '::'.join(['java_runtime',
                                *(f'r#{p}' if p in _RUST_KEYWORDS else p for p in _sv.impl.split('/')[:-1]),
                                short_cls(_sv.impl)])
            _jca_lines.append(
                f'        ("{_sv.type}", "{_sv.algorithm}", "{_sv.impl}", "{_sv.provider}", '
                f'(|| Ok(java_runtime::java::lang::Object::from({_jpath}::new()?)))'
                f' as java_runtime::jca::ServiceCtor),')
        hook_block += ('    java_runtime::jca::register_services(&[\n'
                       + '\n'.join(_jca_lines) + '\n    ]);\n')

    if batch_bin:
        # 批量模式：每个 bin 用 #[path] 独立包含自己的类文件，不共享 lib.rs。
        # 这样某个测试编译失败不会影响其他测试。
        path_decls: list[str] = []
        for m in top_user_mods:
            path_decls += [f'#[path = "../{m}.rs"]', f'mod {m};']
        bin_lines = [
            '#![allow(unused_variables, unused_mut, dead_code, non_snake_case, unused_imports, non_camel_case_types)]',
            *path_decls,
            f'use {use_path};',
            '',
            'fn main() {',
            *([hook_block] if hook_block else []),
            f'    {main_class}::main().unwrap_or_else(|e| e.report_uncaught());',
            '}',
            '',
        ]
        _write(os.path.join(user_src, 'bin', bin_name + '.rs'), '\n'.join(bin_lines))
        _append_cargo_bin(user_dir, bin_name, f'src/bin/{bin_name}.rs')
    else:
        # 单测试模式（默认）：写 src/main.rs + 覆写 Cargo.toml
        main_lines = [
            '#![allow(unused_variables, unused_mut, dead_code, non_snake_case, unused_imports, non_camel_case_types)]',
            *[f'mod {m};' for m in top_user_mods],
            f'use {use_path};',
            '',
            'fn main() {',
            *([hook_block] if hook_block else []),
            f'    {main_class}::main().unwrap_or_else(|e| e.report_uncaught());',
            '}',
            '',
        ]
        _write(os.path.join(user_src, 'main.rs'), '\n'.join(main_lines))
        _user_dep_lines = [
            'java_runtime    = { path = "../java_runtime" }',
            f'java_rta_macros = {{ path = "{_MACROS_CRATE}" }}',
        ]
        # lib 模式：用户 bin crate 消费全部 lib crate（jar 输入的交付形态）
        for _lc_dep in (lib_crate_classes or {}):
            _user_dep_lines.append(f'{_lc_dep:<15} = {{ path = "../{_lc_dep}" }}')
        cargo_toml_lines = [
            '[package]',
            'name = "user"',
            f'version = "{_scratch_pkg_version(user_dir)}"',
            'edition = "2021"',
            '',
            '[[bin]]',
            f'name = "{bin_name}"',
            'path = "src/main.rs"',
            '',
            '[dependencies]',
            *_user_dep_lines,
            '',
            '[lints.rust]',
            'unused_parens = "allow"',
            'unused_braces = "allow"',
            'dead_code = "allow"',
            'unused_assignments = "allow"',
            'unused_variables = "allow"',
            'unused_mut = "allow"',
            'unused_imports = "allow"',
            'non_snake_case = "allow"',
            'non_camel_case_types = "allow"',
            'non_upper_case_globals = "allow"',
            'unreachable_code = "allow"',
            'unreachable_patterns = "allow"',
            '',
        ]
        _write(os.path.join(user_dir, 'Cargo.toml'), '\n'.join(cargo_toml_lines))

    # 5. scratch workspace 根 Cargo.toml（幂等，每次覆写相同内容）
    #    java_rta_macros 不复制进 scratch，作为 runtime/ 的 path 依赖参与编译
    #    （绝对路径稳定 → 共享 CARGO_TARGET_DIR 下指纹不变，宏与 syn/quote 缓存命中）
    _members = ['java_runtime', *(lib_crate_classes or {}), 'user']
    _write(os.path.join(out_dir, 'Cargo.toml'), '\n'.join([
        '[workspace]',
        'members = ' + repr(_members).replace("'", '"'),
        'resolver = "2"',
        '',
        '[profile.release]',
        'opt-level = 3',
        'lto       = true',
        'codegen-units = 1',
        'strip     = "symbols"',
        '',
    ]))

    # 语料 JDK 特性版本 → java_runtime/jdk_feature.txt（build.rs 转为编译期环境变量
    # JAVA_RTA_JDK_FEATURE，手写层经 crate::jdk_feature() 读取）：手写边界类中
    # 随 JDK 版本变化的数据按此选择。写入幂等（同版本内容不变，不触发重编译）。
    from ..jdk_resolver import corpus_jdk_major
    _jdk_major = corpus_jdk_major()
    if _jdk_major:
        _write(os.path.join(rt_dir, 'jdk_feature.txt'), f'{_jdk_major}\n')

    if jdk_class_infos:
        print(f'[codegen] JDK 翻译 → {len(jdk_class_infos)} 个类')
    for _lc_name, _lc_classes in (lib_crate_classes or {}).items():
        print(f'[codegen] lib crate {_lc_name} → {len(_lc_classes)} 个类')
    print(f'[codegen] Cargo workspace → {out_dir}/')


def _complete_jrt_lib_rs(out_dir: str) -> None:
    """scratch java_runtime/src/lib.rs 的顶层模块补全。

    手写 lib.rs（runtime/ 真源的 scratch 副本）只声明既有闭包出现过的顶层包
    （java/jdk/sun + 基础设施）。jar 输入模式的库闭包会首次拉入新顶层根
    （hamcrest beans→com/（sun.beans）、xml→javax/）——占位目录由 codegen
    落盘，但 mod 声明缺失使整 crate E0583/E0433。按磁盘实际存在的顶层包目录
    补声明到 scratch 副本（不碰 runtime/ 真源；非 jar 路径的三根已声明，零追加）。
    """
    _lib_rs = os.path.join(out_dir, 'java_runtime', 'src', 'lib.rs')
    _src_root = os.path.join(out_dir, 'java_runtime', 'src')
    if not os.path.isfile(_lib_rs) or not os.path.isdir(_src_root):
        return
    with open(_lib_rs, encoding='utf-8') as _f:
        _text = _f.read()
    _declared: set[str] = set()
    for _m in re.finditer(r'^\s*(?:pub\s+)?mod\s+(r#\s*)?(\w+)\s*;', _text, re.M):
        _declared.add(_m.group(2))
    _missing: list[str] = []
    for _name in sorted(os.listdir(_src_root)):
        if (_name in _declared or not _name.isidentifier()
                or not os.path.isdir(os.path.join(_src_root, _name))):
            continue
        if os.path.isfile(os.path.join(_src_root, _name, 'mod.rs')):
            _safe = f'r#{_name}' if _name in _RUST_KEYWORDS else _name
            _missing.append(f'pub mod {_safe};')
    if _missing:
        with open(_lib_rs, 'a', encoding='utf-8') as _f:
            _f.write('\n// jar 输入模式：新顶层包根（按磁盘实际目录补全，非手写清单成员）\n'
                     + '\n'.join(_missing) + '\n')
