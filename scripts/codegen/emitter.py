"""
Rust 文件生成器：将 ClassInfo 列表写出为 Cargo workspace，
包含三个子 crate：java_runtime / jdk_classes / user。

结构原则：
- java_runtime：手写运行时（Field<T>、String、Result 等），独立 lib crate
- jdk_classes：JDK 字节码翻译，lib crate，依赖 java_runtime
- user：用户 Java 翻译，bin crate，依赖 java_runtime + jdk_classes
- 生成代码用 `use java_runtime::prelude::*;`（外部 crate，无 crate:: 前缀）
- 每个类/字段/方法携带 #[java_class] / #[java_field] / #[java_method] 属性，
  供 build.rs 自动解析继承链与 native 状态
"""

import os
import re
from .types import ClassInfo, FieldInfo, ParsedMethod
from .type_map import jvm_to_rust, rust_default, short_cls
from .method import gen_method_body, _indent
from .runtime import RUNTIME_FILES
from .sig_parser import parse_class_type_params

# Access flags
_ACC_PUBLIC    = 0x0001
_ACC_PRIVATE   = 0x0002
_ACC_PROTECTED = 0x0004
_ACC_STATIC    = 0x0008
_ACC_FINAL     = 0x0010
_ACC_NATIVE    = 0x0100
_ACC_INTERFACE = 0x0200
_ACC_ABSTRACT  = 0x0400
_ACC_ENUM      = 0x4000

# ── Cargo.toml 模板 ──────────────────────────────────────────────

WORKSPACE_CARGO_TOML = """\
[workspace]
members = ["java_runtime", "jdk_classes", "user"]
resolver = "2"

[profile.release]
opt-level = 3
lto       = true
codegen-units = 1
strip     = "symbols"
"""

JAVA_RUNTIME_CARGO_TOML = """\
[package]
name = "java_runtime"
version = "0.1.0"
edition = "2021"

[lib]
name = "java_runtime"
path = "src/lib.rs"
"""

JDK_CLASSES_CARGO_TOML = """\
[package]
name = "jdk_classes"
version = "0.1.0"
edition = "2021"

[lib]
name = "jdk_classes"
path = "src/lib.rs"

[dependencies]
java_runtime = { path = "../java_runtime" }
"""

USER_CARGO_TOML = """\
[package]
name = "user"
version = "0.1.0"
edition = "2021"

[[bin]]
name = "user"
path = "src/main.rs"

[dependencies]
java_runtime = { path = "../java_runtime" }
# jdk_classes will be added when JDK translations compile successfully
# jdk_classes = { path = "../jdk_classes" }
"""


# ── 辅助 ─────────────────────────────────────────────────────────

_RUST_KEYWORDS = frozenset({
    'as', 'async', 'await', 'break', 'const', 'continue', 'crate', 'dyn',
    'else', 'enum', 'extern', 'false', 'fn', 'for', 'if', 'impl', 'in',
    'let', 'loop', 'match', 'mod', 'move', 'mut', 'pub', 'ref', 'return',
    'self', 'static', 'struct', 'super', 'trait', 'true', 'type',
    'union', 'unsafe', 'use', 'where', 'while',
})


def _safe_field_name(name: str) -> str:
    """字段名安全化：$ → _，Rust 关键字加 _。"""
    name = name.replace('$', '_')
    if name in _RUST_KEYWORDS:
        return name + '_'
    return name


def to_snake(name: str) -> str:
    """PascalCase / camelCase → snake_case（Rust 模块文件名）。
    $ (内部类分隔符) → _ ，Rust 关键字加 _ 后缀。"""
    name = name.replace('$', '_')
    s = re.sub(r'([A-Z]+)([A-Z][a-z])', r'\1_\2', name)
    s = re.sub(r'([a-z\d])([A-Z])', r'\1_\2', s)
    s = s.lower()
    if s in _RUST_KEYWORDS:
        s = s + '_'
    return s


def pkg_from_java(java_file: str) -> str:
    """从 .java 源文件读取 package 声明，无则返回空串"""
    try:
        with open(java_file, encoding='utf-8') as f:
            for line in f:
                line = line.strip()
                if line.startswith('package '):
                    return line[8:].rstrip(';').strip()
                if line and not line.startswith('//') and not line.startswith('/*'):
                    if any(line.startswith(k) for k in ('import ', 'public ', 'class ', '@')):
                        break
    except OSError:
        pass
    return ''


# ── Java 元数据注释生成（供 build.rs 扫描）──────────────────────

def _access_str(flags: int) -> str:
    """将 access_flags 整数转为可读字符串，如 'public static final'。"""
    parts = []
    if flags & _ACC_PUBLIC:    parts.append('public')
    if flags & _ACC_PRIVATE:   parts.append('private')
    if flags & _ACC_PROTECTED: parts.append('protected')
    if flags & _ACC_STATIC:    parts.append('static')
    if flags & _ACC_FINAL:     parts.append('final')
    if flags & _ACC_ABSTRACT:  parts.append('abstract')
    if flags & _ACC_NATIVE:    parts.append('native')
    return ' '.join(parts)


def _java_class_attr(ci: ClassInfo, compiled: bool = False) -> str:
    """生成 #[java_class(...)] 属性块，供 build.rs 解析类层次。

    compiled=True：包裹在 cfg_attr(any(), ...) 内，用于参与 Rust 编译的用户类文件，
    避免未注册属性导致的编译错误（any() 永远为 false，inner attr 不被校验）。
    compiled=False：原生属性格式，用于不参与编译的 JDK 元数据存根文件。
    """
    binary_name = ci.name
    super_class = ci.super_class or ""
    interfaces  = ','.join(ci.interfaces) if ci.interfaces else ""
    access      = _access_str(ci.access_flags) if ci.access_flags else ""
    source      = ci.source_file or ""
    inner_lines = [
        f'    binary_name = "{binary_name}",',
        f'    super_class = "{super_class}",',
        f'    interfaces  = "{interfaces}",',
        f'    access      = "{access}",',
        f'    source      = "{source}",',
    ]
    inner = '\n'.join(inner_lines)
    if compiled:
        return f'#[cfg_attr(any(), java_class(\n{inner}\n))]'
    else:
        return f'#[java_class(\n{inner}\n)]'


def _java_field_attr(f: FieldInfo) -> str:
    """生成 #[cfg_attr(any(), java_field(...))] 属性行（编译安全）。"""
    parts = [f'name = "{f.name}"', f'descriptor = "{f.descriptor}"']
    if f.access_flags:
        parts.append(f'access = "{_access_str(f.access_flags)}"')
    return '#[cfg_attr(any(), java_field(' + ', '.join(parts) + '))]'


def _java_method_attr(m: ParsedMethod, compiled: bool = False) -> str:
    """生成方法元数据标注行。

    compiled=True（JDK 类生成）：输出为行注释 `// java: ...`，
    避免 cfg_attr 内字符串包含关键字（如 static）导致词法解析失败。
    compiled=False（元数据存根）：原生属性格式 #[java_method(...)]。
    """
    tag = 'java_native' if (m.is_native or m.is_abstract) else 'java_method'
    parts = [f'name = "{m.name}"', f'descriptor = "{m.descriptor}"']
    if m.access_flags:
        parts.append(f'access = "{_access_str(m.access_flags)}"')
    inner = f'{tag}(' + ', '.join(parts) + ')'
    if compiled:
        # 注释形式，避免 cfg_attr 内 "public static" 等含关键字字符串触发词法错误
        return f'// java: {m.name}{m.descriptor}'
    else:
        return f'#[{inner}]'


def _gen_native_stub(m: ParsedMethod, ci: ClassInfo, rust_name: str | None = None) -> str:
    """为 native / abstract 方法生成 todo! 存根，供手工实现替换。"""
    from .type_map import jvm_to_rust, parse_descriptor_params, parse_descriptor_return
    params = parse_descriptor_params(m.descriptor)
    ret    = parse_descriptor_return(m.descriptor)
    rust_ret = jvm_to_rust(ret)

    # 构建参数列表
    arg_names = [m.local_names.get(i + (0 if m.is_static else 1), f'arg{i}')
                 for i in range(len(params))]
    args_str = ', '.join(
        f'{name}: {jvm_to_rust(p)}' for name, p in zip(arg_names, params)
    )

    if m.is_static:
        sig_self = ''
    else:
        sig_self = '&self'
        if args_str:
            sig_self += ', '

    ret_type = f'Result<{rust_ret}>' if rust_ret != '()' else 'Result<()>'
    label = 'native' if m.is_native else 'abstract'
    fn_name = rust_name or m.name
    body = f'todo!("{label} {ci.name}.{m.name}")'

    return (
        f'pub fn {fn_name}({sig_self}{args_str}) -> {ret_type} {{\n'
        f'    {body}\n'
        f'}}'
    )


def _jdk_class_file_path(src_dir: str, binary_name: str) -> str:
    """将 JDK 类 binary name 转为 src/ 下的元数据文件路径。"""
    parts = binary_name.split('/')
    *pkg_parts, class_name = parts
    mod_name = to_snake(class_name)
    return os.path.join(src_dir, *pkg_parts, mod_name + '.rs')


def _gen_jdk_class_rs(ci: ClassInfo) -> str:
    """为 JDK 类生成元数据属性文件。

    文件使用 #[java_class] / #[java_native] 属性格式，供 build.rs 扫描维护
    native_status.toml，不参与 Rust 模块编译（无 mod 声明引用此路径）。
    """
    lines = [
        "// 此文件由 java_rta 自动生成，仅供 build.rs 扫描。不参与 Rust 模块编译。",
        "",
        _java_class_attr(ci),
        "struct _JavaClassMarker;",
    ]
    for m in ci.methods:
        if m.is_native:  # 只记录真正的 native 方法，abstract 接口方法不需要 native 实现
            lines.append("")
            lines.append(_java_method_attr(m))
            sanitized = m.name.replace('<', '_').replace('>', '_')
            lines.append(f"fn _{sanitized}() {{}}")
    return '\n'.join(lines) + '\n'


def _gen_class_rs(ci: ClassInfo, registry: dict | None = None,
                  jdk_crate_pkg_paths: list[str] | None = None) -> str:
    """生成单个 Java 类对应的完整 .rs 文件内容。

    生成规则：
    - 实例字段用 Field<T> 包装（提供 Java 字段语义的内部可变性）
    - 方法直接在 impl 块中，无 raw:: 子模块
    - 所有方法返回 Result<T>
    - 每个 struct / field / method 前加 // @java_* 注释供 build.rs 扫描
    """
    from collections import Counter
    from .type_map import mangle_name

    # 跨包 glob import：让生成代码能直接用 Objects、Integer 等翻译过的 JDK 类型
    cross_imports: list[str] = []
    if jdk_crate_pkg_paths:
        for pkg_path in jdk_crate_pkg_paths:
            cross_imports.append(f"use crate::{pkg_path}::*;")

    parts: list[str] = [
        "#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]",
        "use java_runtime::prelude::*;",
        *cross_imports,
        "",
        _java_class_attr(ci, compiled=True),
    ]

    inst_fields = [f for f in ci.fields if not f.is_static]

    # 用短名作为 Rust 标识符（JDK 类的 ci.name 含 /，不是合法 Rust 名）
    struct_name = short_cls(ci.name) if '/' in ci.name else ci.name

    # 解析类级泛型参数（如 ArrayList<E>、HashMap<K,V>）
    class_type_params = parse_class_type_params(ci.generic_signature) if ci.generic_signature else []

    # 构建泛型参数字符串（用于 struct 和 impl 头）
    if class_type_params:
        type_params_str = ', '.join(class_type_params)
        bounds_str = ', '.join(f"{p}: Clone + 'static" for p in class_type_params)
        struct_generic = f"<{type_params_str}>"
        impl_header   = f"impl<{bounds_str}> {struct_name}<{type_params_str}>"
    else:
        struct_generic = ''
        impl_header   = f"impl {struct_name}"

    if inst_fields:
        field_lines = []
        for f in inst_fields:
            safe_fname = _safe_field_name(f.name)
            field_lines.append("    " + _java_field_attr(f))
            field_lines.append(f"    pub {safe_fname}: Field<{jvm_to_rust(f.descriptor)}>,")
        # 若有泛型参数但字段中未用到，加 PhantomData 防止 E0392
        if class_type_params:
            phantom_ty = ', '.join(f'std::marker::PhantomData<{p}>' for p in class_type_params)
            if len(class_type_params) > 1:
                phantom_ty = f'std::marker::PhantomData<({", ".join(class_type_params)},)>'
            field_lines.append(f"    pub _phantom: {phantom_ty},")
        decls = '\n'.join(field_lines)
        parts.append(f"pub struct {struct_name}{struct_generic} {{\n{decls}\n}}\n")
    else:
        if class_type_params:
            # 无字段但有泛型参数：改用 tuple struct 包含 PhantomData
            if len(class_type_params) == 1:
                phantom_ty = f'std::marker::PhantomData<{class_type_params[0]}>'
            else:
                phantom_ty = f'std::marker::PhantomData<({", ".join(class_type_params)},)>'
            parts.append(f"pub struct {struct_name}{struct_generic}({phantom_ty});\n")
        else:
            parts.append(f"pub struct {struct_name}{struct_generic};\n")

    # 过滤 synthetic 方法（编译器合成桥接方法），再统计重载
    visible_methods = [m for m in ci.methods if not m.is_synthetic]
    name_counts = Counter(m.name for m in visible_methods if m.name != '<clinit>')
    overloaded_names: set[str] = {name for name, count in name_counts.items() if count > 1}

    method_blocks: list[str] = []
    for m in visible_methods:
        if m.name == '<clinit>':
            continue
        # 确定最终 Rust 方法名（有重载则加描述符后缀）
        rust_name = mangle_name(m.name, m.descriptor) if m.name in overloaded_names else m.name
        # 构造器统一用 new / new__suffix
        if m.is_constructor:
            if '<init>' in overloaded_names:
                rust_name = mangle_name('new', m.descriptor)
            else:
                rust_name = 'new'

        attr_line = _java_method_attr(m, compiled=True)
        if m.is_native or m.is_abstract:
            stub = _gen_native_stub(m, ci, rust_name=rust_name)
            method_blocks.append(attr_line + '\n' + stub)
        else:
            try:
                body = gen_method_body(
                    m, ci, registry=registry,
                    class_type_params=class_type_params,
                    overloaded_names=overloaded_names,
                )
                method_blocks.append(attr_line + '\n' + body)
            except Exception as e:
                method_blocks.append(f"/* codegen error {m.name}: {e} */")

    impl_body = '\n\n'.join(_indent(b) for b in method_blocks)
    parts.append(f"{impl_header} {{\n{impl_body}\n}}\n")
    return '\n'.join(parts)


# ── 主函数 ────────────────────────────────────────────────────────

def _write(path: str, content: str) -> None:
    """创建目录并写文件。"""
    os.makedirs(os.path.dirname(path) or '.', exist_ok=True)
    with open(path, 'w') as f:
        f.write(content)


def write_cargo_project(out_dir: str, class_infos: list[ClassInfo],
                         jdk_class_infos: list[ClassInfo] | None = None,
                         java_files: list[str] | None = None):
    """
    生成 Cargo workspace，包含三个子 crate：
      java_runtime/  — 手写运行时（来自 RUNTIME_FILES）
      jdk_classes/   — JDK 字节码翻译
      user/          — 用户 Java 代码翻译
    """
    import shutil

    rt_dir   = os.path.join(out_dir, 'java_runtime')
    jdk_dir  = os.path.join(out_dir, 'jdk_classes')
    user_dir = os.path.join(out_dir, 'user')

    # 1. workspace 根 Cargo.toml
    _write(os.path.join(out_dir, 'Cargo.toml'), WORKSPACE_CARGO_TOML)

    # 2. java_runtime crate（来自 RUNTIME_FILES）
    _write(os.path.join(rt_dir, 'Cargo.toml'), JAVA_RUNTIME_CARGO_TOML)
    rt_src = os.path.join(rt_dir, 'src')
    for rel_path, content in RUNTIME_FILES.items():
        # mod.rs 是 crate 根，对应 lib crate 的 src/lib.rs
        dest = 'lib.rs' if rel_path == 'mod.rs' else rel_path
        _write(os.path.join(rt_src, dest), content)

    # 3. jdk_classes crate（JDK 字节码翻译）
    _write(os.path.join(jdk_dir, 'Cargo.toml'), JDK_CLASSES_CARGO_TOML)
    jdk_src = os.path.join(jdk_dir, 'src')

    # 构建 registry（用户类 + JDK 类）
    registry: dict = {ci.name: ci for ci in class_infos}
    if jdk_class_infos:
        for jci in jdk_class_infos:
            registry.setdefault(jci.name, jci)

    # 写 JDK 翻译文件，构建 jdk mod 树
    jdk_mod_tree: dict[str, set[str]] = {}
    if jdk_class_infos:
        # 先收集所有翻译包的 crate 路径（用于 cross-module glob import）
        jdk_pkg_set: set[str] = set()
        for jdk_ci in jdk_class_infos:
            pkg_parts = jdk_ci.name.split('/')[:-1]
            if pkg_parts:
                jdk_pkg_set.add('::'.join(pkg_parts))
        jdk_crate_pkg_paths = sorted(jdk_pkg_set)

        for jdk_ci in jdk_class_infos:
            parts = jdk_ci.name.split('/')          # e.g. ['java','util','ArrayList']
            *pkg_parts, class_name = parts
            mod_name  = to_snake(class_name)
            file_path = os.path.join(jdk_src, *pkg_parts, mod_name + '.rs')
            _write(file_path, _gen_class_rs(jdk_ci, registry=registry,
                                            jdk_crate_pkg_paths=jdk_crate_pkg_paths))
            # 更新 mod 树
            parent = jdk_src
            for part in pkg_parts:
                jdk_mod_tree.setdefault(parent, set()).add(part)
                parent = os.path.join(parent, part)
            jdk_mod_tree.setdefault(parent, set()).add(mod_name)

    # jdk_classes/src/lib.rs
    top_jdk = sorted(jdk_mod_tree.get(jdk_src, set()))
    jdk_lib_lines = [
        '#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]',
        *[f'pub mod {m};' for m in top_jdk],
        '',
    ]
    _write(os.path.join(jdk_src, 'lib.rs'), '\n'.join(jdk_lib_lines))

    # 中间 mod.rs（jdk 子包）：pub mod + pub use *（使 glob import 能拿到类型）
    for dir_path, children in jdk_mod_tree.items():
        if dir_path == jdk_src:
            continue
        mod_lines = []
        for c in sorted(children):
            mod_lines.append(f'pub mod {c};')
            mod_lines.append(f'pub use {c}::*;')
        _write(os.path.join(dir_path, 'mod.rs'), '\n'.join(mod_lines) + '\n')

    # build.rs：优先使用 out_dir 已有的 build.rs，迁移到 jdk_classes/
    old_build_rs = os.path.join(out_dir, 'build.rs')
    new_build_rs = os.path.join(jdk_dir, 'build.rs')
    if os.path.exists(old_build_rs) and not os.path.exists(new_build_rs):
        shutil.copy2(old_build_rs, new_build_rs)

    # native_impls/：迁移到 jdk_classes/native_impls/
    old_native = os.path.join(out_dir, 'native_impls')
    new_native  = os.path.join(jdk_dir, 'native_impls')
    os.makedirs(new_native, exist_ok=True)
    if os.path.isdir(old_native):
        for item in os.listdir(old_native):
            src_item = os.path.join(old_native, item)
            dst_item = os.path.join(new_native, item)
            if not os.path.exists(dst_item):
                if os.path.isdir(src_item):
                    shutil.copytree(src_item, dst_item)
                else:
                    shutil.copy2(src_item, dst_item)

    # 4. user crate（用户 Java 翻译）
    _write(os.path.join(user_dir, 'Cargo.toml'), USER_CARGO_TOML)
    user_src = os.path.join(user_dir, 'src')

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

    # 写用户类文件
    for ci in class_infos:
        file_path, _, _ = layout[ci.name]
        _write(file_path, _gen_class_rs(ci, registry=registry))

    # 中间 mod.rs（用户子包）
    for dir_path, children in user_mod_tree.items():
        if dir_path == user_src:
            continue
        mod_lines = [f'pub mod {c};' for c in sorted(children)]
        for mod_name, cls_name in sorted(user_reexport.get(dir_path, set())):
            mod_lines.append(f'pub use {mod_name}::{cls_name};')
        _write(os.path.join(dir_path, 'mod.rs'), '\n'.join(mod_lines) + '\n')

    # user/src/main.rs
    main_class = class_infos[0].name if class_infos else 'Main'
    _, pkg_parts, mod_name = layout[main_class]
    top_user_mods = sorted(user_mod_tree.get(user_src, set()))
    use_path = '::'.join(pkg_parts + [main_class]) if pkg_parts else f'{mod_name}::{main_class}'

    main_lines = [
        '#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]',
        *[f'mod {m};' for m in top_user_mods],
        f'use {use_path};',
        '',
        'fn main() {',
        f'    {main_class}::main().unwrap_or_else(|e| eprintln!("JVM Error: {{:?}}", e));',
        '}',
        '',
    ]
    _write(os.path.join(user_src, 'main.rs'), '\n'.join(main_lines))

    if jdk_class_infos:
        print(f'[codegen] JDK 翻译 → {len(jdk_class_infos)} 个类')
    print(f'[codegen] Cargo workspace → {out_dir}/')
