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
    with open(path, 'w') as f:
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
                         visited_methods: set | None = None):
    """
    生成 Cargo workspace，包含两个子 crate：
      java_runtime/  — VM 基础设施 + JDK 字节码翻译（合并，git 管理基础设施部分）
      user/          — 用户 Java 代码翻译
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
    _WRITTEN_THIS_RUN.clear()
    emissions: dict[str, ClassEmission] = {}

    # 构建 registry（用户类 + JDK 类）
    registry: dict = {ci.name: ci for ci in class_infos}
    if jdk_class_infos:
        for jci in jdk_class_infos:
            registry.setdefault(jci.name, jci)
    _configure_short_names(registry)

    # 扫描 jdk_classes/src/**/*_impl.rs，构建 new_format_map（已手写方法 → codegen 跳过 stub）
    # 传入 registry 使 _scan_impl_files 能通过 registry 解析嵌套类的真实 binary_name（如 HashMap$TreeNode）
    new_format_map, full_impl_classes = _scan_impl_files(out_dir, registry=registry)

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
                                handwritten=_is_handwritten(file_path))
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

    def _mod_decl(name: str) -> str:
        """生成 pub mod 声明，对 Rust 关键字用 r# 转义。"""
        safe = f'r#{name}' if name in _RUST_KEYWORDS else name
        return f'pub mod {safe};'

    def _use_decl(name: str) -> str:
        """生成 pub use *::* 声明，对 Rust 关键字用 r# 转义。"""
        safe = f'r#{name}' if name in _RUST_KEYWORDS else name
        return f'pub use {safe}::*;'

    def _write_jdk_mod_tree() -> None:
        """JDK 类文件全部落盘后调用：mod.rs 如实声明磁盘上的全部模块。"""
        # 陈旧生成文件清除：带生成标记（java_rta_macros::java_class）、但本轮未写入的
        # .rs 是同 scratch 上次运行的幸存者。若不清除，下方的磁盘扫描会把它们的模块
        # 声明重新挂进 mod.rs，与手写 companion（E0592，如 unsafe_.rs + unsafe__impl.rs）
        # 或本轮闭包冲突。手写文件无生成标记，不受影响。
        for _root_sweep, _dirs_sweep, _files_sweep in os.walk(jdk_src):
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
        # 从磁盘全量重建 jdk_mod_tree：mod.rs 如实声明磁盘上的全部模块。
        # 磁盘内容 = 手写 overlay（runtime/ 复制进来的 object.rs、function 存根、
        # companion）+ 本次生成的类文件 + 同 scratch 上次运行的幸存文件。
        # 用作用域内的 jdk_mod_tree 直接写 mod.rs 会抹掉其余文件的声明（E0432/E0433）。
        # 扫描收集全部 .rs，自底向上传播目录，只声明有文件的目录（避免 E0583）。
        jdk_mod_tree: dict[str, set[str]] = {}
        if os.path.isdir(jdk_src):
            for root, _dirs, files in os.walk(jdk_src):
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
                    if _dp == jdk_src:
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
            if dir_path == jdk_src:
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
                    # X.rs 不存在时跳过：_impl.rs 静默，不产生无法解析的 mod 声明
                    if not os.path.exists(os.path.join(dir_path, base + '.rs')):
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

    # 提取包名
    packages: dict[str, str] = {}
    if java_files:
        for jf, ci in zip(java_files, class_infos):
            packages[ci.name] = pkg_from_java(jf)

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

    # 构建每个用户类所需的兄弟内部类导入（crate::mod::Type）
    # 原则：一个外部类文件引用其内部类时，需要 use crate::{inner_mod}::{InnerType}
    _user_layout_names: set[str] = set(layout.keys())
    _sibling_imports: dict[str, list[str]] = {}
    for ci in class_infos:
        seen_imports: set[str] = set()
        imports: list[str] = []

        def _add_import(class_name: str) -> None:
            if class_name not in _user_layout_names or class_name == ci.name:
                return
            mod_n = to_snake(class_name)
            type_n = short_cls(class_name)
            line = f"use crate::{mod_n}::{type_n};"
            if line not in seen_imports:
                seen_imports.add(line)
                imports.append(line)

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
    for ci in class_infos:
        file_path, _, _ = layout[ci.name]
        _em = ClassEmission(binary_name=ci.name, crate_prefix='java_runtime', path=file_path)
        _em.text = _gen_class_rs(ci, registry=registry,
                                 jdk_crate_pkg_paths=user_pkg_paths,
                                 user_crate_prefix='java_runtime',
                                 new_format_map=new_format_map,
                                 workspace_root=out_dir,
                                 full_impl_classes=full_impl_classes,
                                 conflict_map=conflict_map if jdk_class_infos else None,
                                 skipped_classes=skipped_classes if jdk_class_infos else None,
                                 user_sibling_imports=_sibling_imports.get(ci.name),
                                 generated_classes=_user_gen_jdk,
                                 emission=_em)
        emissions[ci.name] = _em

    # G-10 生成期断言：invokedynamic 实现方法的「调用点引用名 ↔ 定义名」恒等，
    # 且被引用方法在生成类中必须存在定义（不一致直接抛错，防止静默生成坏代码）
    LAMBDA_NAME_LEDGER.check()

    # 全部方法体已生成 → 继承成员需求已齐：补声明后统一落盘
    resolve_interface_impls(emissions, registry,
                            {k: set(v.get('methods', set())) for k, v in (new_format_map or {}).items()})
    resolve_interface_inherited_members(emissions, registry)
    resolve_inherited_members(emissions, registry)
    for _em in emissions.values():
        _write(_em.path, _em.text)
    _write_jdk_mod_tree()

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
            f'    {main_class}::main().unwrap_or_else(|e| e.report_uncaught());',
            '}',
            '',
        ]
        _write(os.path.join(user_src, 'main.rs'), '\n'.join(main_lines))
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
            'java_runtime    = { path = "../java_runtime" }',
            f'java_rta_macros = {{ path = "{_MACROS_CRATE}" }}',
            '',
        ]
        _write(os.path.join(user_dir, 'Cargo.toml'), '\n'.join(cargo_toml_lines))

    # 5. scratch workspace 根 Cargo.toml（幂等，每次覆写相同内容）
    #    java_rta_macros 不复制进 scratch，作为 runtime/ 的 path 依赖参与编译
    #    （绝对路径稳定 → 共享 CARGO_TARGET_DIR 下指纹不变，宏与 syn/quote 缓存命中）
    _write(os.path.join(out_dir, 'Cargo.toml'), '\n'.join([
        '[workspace]',
        'members = ["java_runtime", "user"]',
        'resolver = "2"',
        '',
        '[profile.release]',
        'opt-level = 3',
        'lto       = true',
        'codegen-units = 1',
        'strip     = "symbols"',
        '',
    ]))

    if jdk_class_infos:
        print(f'[codegen] JDK 翻译 → {len(jdk_class_infos)} 个类')
    print(f'[codegen] Cargo workspace → {out_dir}/')
