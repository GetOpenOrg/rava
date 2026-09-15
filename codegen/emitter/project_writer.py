"""
Cargo workspace 写出：_write、_update_user_lib_rs、_append_cargo_bin、write_cargo_project。
"""

import os
import re
from ..types import ClassInfo
from ..type_map import short_cls
from ..constants import RUST_KEYWORDS as _RUST_KEYWORDS
from .attrs import to_snake, pkg_from_java
from .method_gen import _scan_native_impls
from .class_writer import _gen_class_rs


def _write(path: str, content: str) -> None:
    """创建目录并写文件。"""
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
    生成 Cargo workspace，包含三个子 crate：
      java_runtime/  — 手写 VM 基础设施（git 管理，不由转译器写入）
      jdk_classes/   — JDK 字节码翻译
      user/          — 用户 Java 代码翻译
    """
    import shutil

    rt_dir   = os.path.join(out_dir, 'java_runtime')
    jdk_dir  = os.path.join(out_dir, 'jdk_classes')
    user_dir = os.path.join(out_dir, 'user')

    # 3. jdk_classes crate（JDK 字节码翻译）
    jdk_src = os.path.join(jdk_dir, 'src')
    # 若 jdk_classes/Cargo.toml 不存在（新工作区），生成它
    jdk_cargo = os.path.join(jdk_dir, 'Cargo.toml')
    if not os.path.exists(jdk_cargo):
        _write(jdk_cargo, '\n'.join([
            '[package]',
            'name = "jdk_classes"',
            'version = "0.1.0"',
            'edition = "2021"',
            '',
            '[lib]',
            'name = "jdk_classes"',
            'path = "src/lib.rs"',
            '',
            '[dependencies]',
            'java_runtime    = { path = "../java_runtime" }',
            'java_rta_macros = { path = "../java_rta_macros" }',
            'parking_lot     = "0.12"',
            '',
        ]))
    # 清理旧版生成文件（batch 模式由调用方在批次开始前统一清理）
    # K-4: 只跳过 *_impl.rs 手写共置文件，其余生成 stub 正常删除重建
    if not batch_bin and os.path.isdir(jdk_src):
        for root, _dirs, files in os.walk(jdk_src):
            for fname in files:
                if not fname.endswith('.rs'):
                    continue
                if fname.endswith('_impl.rs'):
                    continue  # 手写共置文件，保留
                try:
                    os.remove(os.path.join(root, fname))
                except FileNotFoundError:
                    pass

    # 构建 registry（用户类 + JDK 类）
    registry: dict = {ci.name: ci for ci in class_infos}
    if jdk_class_infos:
        for jci in jdk_class_infos:
            registry.setdefault(jci.name, jci)

    # 扫描 jdk_classes/src/**/*_impl.rs，构建 new_format_map（已手写方法 → codegen 跳过 stub）
    new_format_map, full_impl_classes = _scan_native_impls(out_dir)  # full_impl_classes 恒为空集

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
            # 调用链上的非 native 方法翻译字节码，调用链外的方法生成 panic! 存根
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

    def _mod_decl(name: str) -> str:
        """生成 pub mod 声明，对 Rust 关键字用 r# 转义。"""
        safe = f'r#{name}' if name in _RUST_KEYWORDS else name
        return f'pub mod {safe};'

    def _use_decl(name: str) -> str:
        """生成 pub use *::* 声明，对 Rust 关键字用 r# 转义。"""
        safe = f'r#{name}' if name in _RUST_KEYWORDS else name
        return f'pub use {safe}::*;'

    # batch 模式：从磁盘全量重建 jdk_mod_tree，合并所有历次批量转译积累的 stub 文件。
    # 原因：每次 write_cargo_project(batch_bin=True) 的 jdk_mod_tree 只包含当前测试
    # 的 JDK 类；若直接用它写 lib.rs/mod.rs，会抹掉之前测试的 stub 声明（E0432）。
    # 解决：写完当前测试的 .rs 文件后，扫描磁盘收集全部 .rs，自底向上传播目录，
    # 只声明有文件的目录（避免 E0583），再写 lib.rs/mod.rs。
    if batch_bin and os.path.isdir(jdk_src):
        jdk_mod_tree = {}
        for root, _dirs, files in os.walk(jdk_src):
            for fname in files:
                if fname.endswith('.rs') and fname not in ('lib.rs', 'mod.rs'):
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

    # jdk_classes/src/lib.rs
    top_jdk = sorted(jdk_mod_tree.get(jdk_src, set()))
    jdk_lib_lines = [
        '#![allow(unused_variables, unused_mut, dead_code, non_snake_case, unused_imports, non_camel_case_types)]',
        *[_mod_decl(m) for m in top_jdk],
        '',
    ]
    _write(os.path.join(jdk_src, 'lib.rs'), '\n'.join(jdk_lib_lines))

    # 中间 mod.rs（jdk 子包）：pub mod + pub use *（使 glob import 能拿到类型）
    for dir_path, children in jdk_mod_tree.items():
        if dir_path == jdk_src:
            continue
        mod_lines = ['#![allow(ambiguous_glob_reexports)]']
        for c in sorted(children):
            mod_lines.append(_mod_decl(c))
            mod_lines.append(_use_decl(c))
        # K-2: 扫描目录中的共置 _impl.rs 文件，加入私有 mod 声明（不 pub use，只是 impl 扩展块）
        if os.path.isdir(dir_path):
            for _f in sorted(os.listdir(dir_path)):
                if _f.endswith('_impl.rs'):
                    _impl_mod = _f[:-3]  # 去掉 .rs 后缀
                    mod_lines.append(f'mod {_impl_mod};')
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
                                        user_crate_prefix='jdk_classes',
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
            'jdk_classes     = { path = "../jdk_classes" }',
            '',
        ]
        _write(os.path.join(user_dir, 'Cargo.toml'), '\n'.join(cargo_toml_lines))

    if jdk_class_infos:
        print(f'[codegen] JDK 翻译 → {len(jdk_class_infos)} 个类')
    print(f'[codegen] Cargo workspace → {out_dir}/')
