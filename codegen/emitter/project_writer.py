"""
Cargo workspace 写出：_write、_update_user_lib_rs、_append_cargo_bin、write_cargo_project。
"""

import os
import re
from ..types import ClassInfo
from ..type_map import short_cls
from ..constants import RUST_KEYWORDS as _RUST_KEYWORDS
from .attrs import to_snake, pkg_from_java
from .method_gen import _scan_impl_files
from .class_writer import _gen_class_rs


def _write(path: str, content: str) -> None:
    """创建目录并写文件。
    对 java_runtime/src/ 下的 .rs 文件，若已存在且不含自动生成标记，则视为手写文件保留不覆盖。
    mod.rs / lib.rs / user/ 下文件始终正常写入。
    """
    _basename = os.path.basename(path)
    _is_jrt_rs = (
        path.endswith('.rs')
        and 'java_runtime' + os.sep + 'src' in path
        and _basename not in ('mod.rs', 'lib.rs')
    )
    if _is_jrt_rs and os.path.exists(path):
        try:
            with open(path, encoding='utf-8') as _f:
                if 'java_rta_macros::java_class' not in _f.read(4096):
                    return  # 手写文件，不覆盖
        except Exception:
            pass
    os.makedirs(os.path.dirname(path) or '.', exist_ok=True)
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
        '[package]', 'name = "user"', 'version = "0.1.0"', 'edition = "2021"', '',
        '[dependencies]',
        'java_runtime    = { path = "../java_runtime" }',
        'java_rta_macros = { path = "../java_rta_macros" }',
        'jdk_classes     = { path = "../jdk_classes" }', '',
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

    # 清理旧版生成文件（batch 模式由调用方在批次开始前统一清理）
    # 只清理 java/ jdk/ 等子包目录，保留根目录的手写基础设施文件
    # 保留规则：
    #   1. *_impl.rs / *_ext.rs — 手写共置文件（永不删除）
    #   2. _PERMANENT 集合中的文件 — 永久手写文件（如 object.rs）
    #   其余 *.rs 均为 codegen 生成文件，清理后由本次 codegen 重新生成
    _PERMANENT = {
        os.path.join(jdk_src, 'java', 'lang', 'object.rs'),
        # 注：java/util/iterator.rs 虽在旧名单中，但其内容含 java_class 生成标记，
        # 实为 codegen 产物（Arch-1 接口存根）。留在名单里会导致它永远停留在旧宏
        # 格式、无法随 java_class! 块宏迁移，故移出，交由 codegen 重新生成。
        os.path.join(jdk_src, 'java', 'util', 'function', 'bi_consumer.rs'),
        os.path.join(jdk_src, 'java', 'util', 'function', 'binary_operator.rs'),
        os.path.join(jdk_src, 'java', 'util', 'function', 'supplier.rs'),
        os.path.join(jdk_src, 'java', 'util', 'function', 'function.rs'),
    }
    # 将 git 追踪的 .rs 文件纳入 _PERMANENT：仅「无生成标记」的文件视为手写，
    # codegen 不覆盖。判别方式与 _impl 扫描一致 —— 看文件里有没有自动生成标记
    # `java_rta_macros::java_class`（属性宏形态与块宏形态都含该子串）：
    #   含标记 → codegen 生成物，允许覆盖（历史提交里混入了生成文件）
    #   不含标记 → 真正手写（object.rs、*_impl.rs、*_ext.rs 等），禁止覆盖
    # 早期版本把「git 追踪」直接等同于「手写」并塞进 _PERMANENT，导致已提交的
    # 生成文件被永久冻结：内容停留在旧宏格式，无法随 java_class! 块宏迁移。
    #
    # 注意：git 追踪的生成文件**不**因此获得删除保护——非 batch 清理仍然会删除
    # 作用域外的生成文件（单测试 = 窄作用域语料，这是 dev loop 的既有语义）；
    # 被删文件可随时通过 batch 重生成或 git checkout 恢复。
    try:
        import subprocess as _subprocess
        _git_root = os.path.dirname(out_dir)
        _git_files = _subprocess.check_output(
            ['git', 'ls-files', '--', jdk_src],
            cwd=_git_root,
            stderr=_subprocess.DEVNULL,
            text=True,
        ).splitlines()
        for _gf in _git_files:
            if not _gf.endswith('.rs'):
                continue
            _abs = os.path.normpath(os.path.join(_git_root, _gf))
            try:
                with open(_abs, encoding='utf-8') as _fh:
                    _is_generated = 'java_rta_macros::java_class' in _fh.read(4096)
            except Exception:
                _is_generated = False  # 读不到时保守视为手写
            if not _is_generated:
                _PERMANENT.add(_abs)
    except Exception:
        pass
    if not batch_bin and os.path.isdir(jdk_src):
        for root, _dirs, files in os.walk(jdk_src):
            if root == jdk_src:
                continue  # 跳过根目录（lib.rs, error.rs 永久保留）
            for fname in files:
                if not fname.endswith('.rs'):
                    continue
                if fname.endswith('_impl.rs') or fname.endswith('_ext.rs'):
                    # 检测是否为自动生成文件（如 Collectors$CollectorImpl 碰巧生成 *_impl.rs）。
                    # 自动生成的类文件含有 java_rta_macros::java_class 标注；手写文件则无。
                    _fpath_check = os.path.join(root, fname)
                    try:
                        with open(_fpath_check, encoding='utf-8') as _fc:
                            if 'java_rta_macros::java_class' not in _fc.read():
                                continue  # 真正手写共置文件，保留
                    except Exception:
                        continue  # 读取失败时保守保留
                fpath = os.path.join(root, fname)
                if fpath in _PERMANENT:
                    continue
                try:
                    os.remove(fpath)
                except FileNotFoundError:
                    pass

    # 构建 registry（用户类 + JDK 类）
    registry: dict = {ci.name: ci for ci in class_infos}
    if jdk_class_infos:
        for jci in jdk_class_infos:
            registry.setdefault(jci.name, jci)

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
                    _simple_cls = _parts[-1].replace('$', '_')
                    skipped_classes.add(f"{_rust_pkg}::{_simple_cls}")
        _jdk_pkg_path_set = set(jdk_crate_pkg_paths)
        conflict_map: dict[str, list[str]] = {}
        for _sn, _pkgs in _sn_to_pkgs.items():
            _in_scope = list({p for p in _pkgs
                              if '::'.join(_safe_pkg_part(x) for x in p.split('/')) in _jdk_pkg_path_set})
            if len(_in_scope) >= 2:
                conflict_map[_sn] = _in_scope

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
            # _PERMANENT 中的文件（如 object.rs）由手写提供，跳过 codegen 覆盖
            if file_path in _PERMANENT:
                parent = jdk_src
                for part in pkg_parts:
                    jdk_mod_tree.setdefault(parent, set()).add(part)
                    parent = os.path.join(parent, part)
                jdk_mod_tree.setdefault(parent, set()).add(mod_name)
                continue
            # 调用链上的非 native 方法翻译字节码，调用链外的方法生成 panic! 存根
            # new_format_map 中已有 _impl.rs 实现的方法，codegen 跳过那些方法的 stub 生成
            # new_format_map 中已有 _impl.rs 实现的方法，codegen 跳过那些方法的 stub 生成
            _write(file_path, _gen_class_rs(jdk_ci, registry=registry,
                                            jdk_crate_pkg_paths=jdk_crate_pkg_paths,
                                            call_chain=visited_methods,
                                            new_format_map=new_format_map,
                                            workspace_root=out_dir,
                                            full_impl_classes=full_impl_classes,
                                            conflict_map=conflict_map,
                                            skipped_classes=skipped_classes))
            # 更新 mod 树
            parent = jdk_src
            for part in pkg_parts:
                jdk_mod_tree.setdefault(parent, set()).add(part)
                parent = os.path.join(parent, part)
            jdk_mod_tree.setdefault(parent, set()).add(mod_name)

    # 非 batch 模式：_PERMANENT 手写文件不在调用链中，不会被 jdk_class_infos 覆盖，
    # 但它们存在于磁盘，需要手动添加到 mod_tree，否则 mod.rs 不会声明对应模块。
    if not batch_bin:
        for _perm_path in _PERMANENT:
            if not os.path.exists(_perm_path):
                continue
            # 计算相对于 jdk_src 的路径
            try:
                _rel = os.path.relpath(_perm_path, jdk_src)
            except ValueError:
                continue
            _parts = _rel.replace('\\', '/').split('/')
            if not _parts or not _parts[-1].endswith('.rs'):
                continue
            _mod_name = _parts[-1][:-3]  # 去掉 .rs 后缀
            # mod.rs / lib.rs 自身不作为模块名声明
            if _mod_name in ('mod', 'lib'):
                continue
            # 手写共置 _impl.rs / _ext.rs（不含 java_class 注解）不作为 pub mod 声明
            if _parts[-1].endswith('_impl.rs') or _parts[-1].endswith('_ext.rs'):
                try:
                    with open(_perm_path, encoding='utf-8') as _fc:
                        if 'java_rta_macros::java_class' not in _fc.read(4096):
                            continue  # 手写共置文件，跳过
                except Exception:
                    continue
            _parent = jdk_src
            for _part in _parts[:-1]:
                jdk_mod_tree.setdefault(_parent, set()).add(_part)
                _parent = os.path.join(_parent, _part)
            jdk_mod_tree.setdefault(_parent, set()).add(_mod_name)

    def _mod_decl(name: str) -> str:
        """生成 pub mod 声明，对 Rust 关键字用 r# 转义。"""
        safe = f'r#{name}' if name in _RUST_KEYWORDS else name
        return f'pub mod {safe};'

    def _use_decl(name: str) -> str:
        """生成 pub use *::* 声明，对 Rust 关键字用 r# 转义。"""
        safe = f'r#{name}' if name in _RUST_KEYWORDS else name
        return f'pub use {safe}::*;'

    # 从磁盘全量重建 jdk_mod_tree，合并所有历次转译积累的生成文件。
    # 原因：每次 write_cargo_project() 的 jdk_mod_tree 只包含当前测试的 JDK 类；
    # 若直接用它写 lib.rs/mod.rs，会抹掉其他已存在文件的模块声明（E0432/E0433）。
    # 解决：写完当前测试的 .rs 文件后，扫描磁盘收集全部 .rs，自底向上传播目录，
    # 只声明有文件的目录（避免 E0583），再写 lib.rs/mod.rs。
    # batch 与非 batch 均启用：清理后磁盘上仍可能有 _PERMANENT 手写文件与本次
    # 作用域外的幸存文件，mod.rs 必须如实声明磁盘上的全部模块。
    if os.path.isdir(jdk_src):
        jdk_mod_tree = {}
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
            mod_lines.append(_use_decl(c))
        # K-2: 扫描目录中的 _impl.rs / _ext.rs 共置文件，加入私有 mod 声明。
        # 规则：只有当 X.rs 存在（即 X 在调用链中已生成，或是 _PERMANENT 手写文件）时，
        # 才声明 mod X_impl; / mod X_ext;。否则 _impl.rs 静默等待，避免 E0583 / 未定义类型。
        # 若 X.rs 存在但不在 children（_PERMANENT 手写文件），还需补充 pub mod X; 声明。
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
                # X.rs 不存在时跳过：_impl.rs 静默，不产生无法解析的 mod 声明
                if not os.path.exists(os.path.join(dir_path, base + '.rs')):
                    continue
                companion_mods.append(f'mod {_f[:-3]};')
                # X.rs 在磁盘但不在 children（如 _PERMANENT object.rs）→ 补充 pub mod 声明
                if base not in children:
                    extra_pub.append(base)
            for base in sorted(set(extra_pub)):
                mod_lines.append(_mod_decl(base))
                mod_lines.append(_use_decl(base))
            mod_lines.extend(companion_mods)
        _write(os.path.join(dir_path, 'mod.rs'), '\n'.join(mod_lines) + '\n')

    # 4. user crate（用户 Java 翻译）
    # Cargo.toml 由 git 直接管理，emitter 不再写出
    user_src = os.path.join(user_dir, 'src')
    # 清理旧版生成文件（保证当前运行不被历史文件污染）
    if not batch_bin and os.path.isdir(user_src):
        for root, _dirs, files in os.walk(user_src):
            for fname in files:
                if fname.endswith('.rs'):
                    try:
                        os.remove(os.path.join(root, fname))
                    except FileNotFoundError:
                        pass

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
    for ci in class_infos:
        file_path, _, _ = layout[ci.name]
        _write(file_path, _gen_class_rs(ci, registry=registry,
                                        jdk_crate_pkg_paths=user_pkg_paths,
                                        user_crate_prefix='java_runtime',
                                        new_format_map=new_format_map,
                                        workspace_root=out_dir,
                                        full_impl_classes=full_impl_classes,
                                        conflict_map=conflict_map if jdk_class_infos else None,
                                        skipped_classes=skipped_classes if jdk_class_infos else None,
                                        user_sibling_imports=_sibling_imports.get(ci.name)))

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
            f'    {main_class}::main().unwrap_or_else(|e| eprintln!("JVM Error: {{:?}}", e));',
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
            f'    {main_class}::main().unwrap_or_else(|e| eprintln!("JVM Error: {{:?}}", e));',
            '}',
            '',
        ]
        _write(os.path.join(user_src, 'main.rs'), '\n'.join(main_lines))
        cargo_toml_lines = [
            '[package]',
            'name = "user"',
            'version = "0.1.0"',
            'edition = "2021"',
            '',
            '[[bin]]',
            f'name = "{bin_name}"',
            'path = "src/main.rs"',
            '',
            '[dependencies]',
            'java_runtime    = { path = "../java_runtime" }',
            'java_rta_macros = { path = "../java_rta_macros" }',
            '',
        ]
        _write(os.path.join(user_dir, 'Cargo.toml'), '\n'.join(cargo_toml_lines))

    if jdk_class_infos:
        print(f'[codegen] JDK 翻译 → {len(jdk_class_infos)} 个类')
    print(f'[codegen] Cargo workspace → {out_dir}/')
