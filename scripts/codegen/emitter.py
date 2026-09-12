"""
Rust 文件生成器：将 ClassInfo 列表写出为 Cargo 项目，
按 Java 包路径组织模块（Java 命名空间同构）。

结构原则：
- 生成代码使用 java_runtime prelude（String, ArrayList, Field 等）
- struct 直接包含 Field<T> 字段，不生成 raw:: 子模块
- 所有方法返回 Result<T>
- 每个类/字段/方法携带 #[java_class] / #[java_field] / #[java_method] 属性，
  供 build.rs 自动解析继承链与 native 状态
"""

import os
import re
from .types import ClassInfo, FieldInfo, ParsedMethod
from .type_map import jvm_to_rust, rust_default, short_cls
from .method import gen_method_body, _indent
from .runtime import RUNTIME_FILES

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

CARGO_TOML = """\
[package]
name = "java_transpiled"
version = "0.1.0"
edition = "2021"

[profile.release]
opt-level = 3
lto       = true
codegen-units = 1
strip     = "symbols"
"""


# ── 辅助 ─────────────────────────────────────────────────────────

def to_snake(name: str) -> str:
    """PascalCase / camelCase → snake_case（Rust 模块文件名）"""
    s = re.sub(r'([A-Z]+)([A-Z][a-z])', r'\1_\2', name)
    s = re.sub(r'([a-z\d])([A-Z])', r'\1_\2', s)
    return s.lower()


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
    """生成 #[java_method(...)] 或 #[java_native(...)] 属性行。

    compiled=True：包裹在 cfg_attr(any(), ...) 内，防止编译错误。
    compiled=False：原生属性格式，用于不参与编译的 JDK 元数据存根文件。
    """
    tag = 'java_native' if (m.is_native or m.is_abstract) else 'java_method'
    parts = [f'name = "{m.name}"', f'descriptor = "{m.descriptor}"']
    if m.access_flags:
        parts.append(f'access = "{_access_str(m.access_flags)}"')
    inner = f'{tag}(' + ', '.join(parts) + ')'
    if compiled:
        return f'#[cfg_attr(any(), {inner})]'
    else:
        return f'#[{inner}]'


def _gen_native_stub(m: ParsedMethod, ci: ClassInfo) -> str:
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
    body = f'todo!("{label} {ci.name}.{m.name}")'

    return (
        f'pub fn {m.name}({sig_self}{args_str}) -> {ret_type} {{\n'
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


def _gen_class_rs(ci: ClassInfo, registry: dict | None = None) -> str:
    """生成单个 Java 类对应的完整 .rs 文件内容。

    生成规则：
    - 实例字段用 Field<T> 包装（提供 Java 字段语义的内部可变性）
    - 方法直接在 impl 块中，无 raw:: 子模块
    - 所有方法返回 Result<T>
    - 每个 struct / field / method 前加 // @java_* 注释供 build.rs 扫描
    """
    parts: list[str] = [
        "#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]",
        "use crate::java_runtime::prelude::*;",
        "",
        _java_class_attr(ci, compiled=True),
    ]

    inst_fields = [f for f in ci.fields if not f.is_static]
    has_instance_methods = any(not m.is_static and not m.is_constructor for m in ci.methods)

    # 用短名作为 Rust 标识符（JDK 类的 ci.name 含 /，不是合法 Rust 名）
    struct_name = short_cls(ci.name) if '/' in ci.name else ci.name

    if inst_fields:
        field_lines = []
        for f in inst_fields:
            field_lines.append("    " + _java_field_attr(f))
            field_lines.append(f"    pub {f.name}: Field<{jvm_to_rust(f.descriptor)}>,")
        decls = '\n'.join(field_lines)
        parts.append(f"pub struct {struct_name} {{\n{decls}\n}}\n")
    else:
        parts.append(f"pub struct {struct_name};\n")

    method_blocks: list[str] = []
    for m in ci.methods:
        if m.name == '<clinit>':
            continue
        attr_line = _java_method_attr(m, compiled=True)
        if m.is_native or m.is_abstract:
            stub = _gen_native_stub(m, ci)
            method_blocks.append(attr_line + '\n' + stub)
        else:
            try:
                body = gen_method_body(m, ci, registry=registry)
                method_blocks.append(attr_line + '\n' + body)
            except Exception as e:
                method_blocks.append(f"/* codegen error {m.name}: {e} */")

    impl_body = '\n\n'.join(_indent(b) for b in method_blocks)
    parts.append(f"impl {struct_name} {{\n{impl_body}\n}}\n")
    return '\n'.join(parts)


# ── 主函数 ────────────────────────────────────────────────────────

def write_cargo_project(out_dir: str, class_infos: list[ClassInfo],
                         jdk_class_infos: list[ClassInfo] | None = None,
                         java_files: list[str] | None = None):
    src_dir = os.path.join(out_dir, 'src')
    rt_dir  = os.path.join(src_dir, 'java_runtime')

    # 1. 写 Cargo.toml
    with open(os.path.join(out_dir, 'Cargo.toml'), 'w') as f:
        f.write(CARGO_TOML)

    # 2. 写 java_runtime（按 JDK 包路径）
    for rel_path, content in RUNTIME_FILES.items():
        abs_path = os.path.join(rt_dir, rel_path)
        os.makedirs(os.path.dirname(abs_path), exist_ok=True)
        with open(abs_path, 'w') as f:
            f.write(content)

    # 3. 提取每个类的 Java 包名
    packages: dict[str, str] = {}
    if java_files:
        for jf, ci in zip(java_files, class_infos):
            packages[ci.name] = pkg_from_java(jf)

    # 4. 计算每个类的文件路径
    #    layout[class_name] = (abs_file_path, pkg_parts, mod_name)
    layout: dict[str, tuple] = {}
    for ci in class_infos:
        pkg       = packages.get(ci.name, '')
        pkg_parts = pkg.split('.') if pkg else []
        mod_name  = to_snake(ci.name)
        file_path = os.path.join(src_dir, *pkg_parts, mod_name + '.rs')
        layout[ci.name] = (file_path, pkg_parts, mod_name)

    # 5. 构建模块树：dir → {子模块名}
    #    同时构建 reexport 表：dir → {(mod_name, ClassName)} 用于生成 pub use
    mod_tree:    dict[str, set[str]]            = {}
    reexport:    dict[str, set[tuple[str,str]]] = {}   # dir → {(mod, ClassName)}
    for ci in class_infos:
        _, pkg_parts, mod_name = layout[ci.name]
        parent = src_dir
        for part in pkg_parts:
            mod_tree.setdefault(parent, set()).add(part)
            parent = os.path.join(parent, part)
        mod_tree.setdefault(parent, set()).add(mod_name)
        reexport.setdefault(parent, set()).add((mod_name, ci.name))

    # 6. 写各类的 .rs 文件
    # 构建 registry：所有已知类（用户类 + JDK 类）的 binary_name → ClassInfo 映射
    registry: dict = {ci.name: ci for ci in class_infos}
    if jdk_class_infos:
        for jci in jdk_class_infos:
            registry.setdefault(jci.name, jci)

    for ci in class_infos:
        file_path, _, _ = layout[ci.name]
        os.makedirs(os.path.dirname(file_path), exist_ok=True)
        with open(file_path, 'w') as f:
            f.write(_gen_class_rs(ci, registry=registry))

    # 7. 写中间包目录的 mod.rs（含 pub mod 和 pub use 再导出）
    for dir_path, children in mod_tree.items():
        if dir_path == src_dir:
            continue
        os.makedirs(dir_path, exist_ok=True)
        mod_lines = [f"pub mod {c};" for c in sorted(children)]
        # pub use ClassX; 让上层可以通过短路径引用
        for mod_name, cls_name in sorted(reexport.get(dir_path, set())):
            mod_lines.append(f"pub use {mod_name}::{cls_name};")
        with open(os.path.join(dir_path, 'mod.rs'), 'w') as f:
            f.write('\n'.join(mod_lines) + '\n')

    # 8. 写 main.rs
    main_class = class_infos[0].name if class_infos else 'Main'
    top_mods   = sorted(mod_tree.get(src_dir, set()))

    _, pkg_parts, mod_name = layout[main_class]
    if pkg_parts:
        use_path = '::'.join(pkg_parts + [main_class])  # com::example::HelloWorld
    else:
        use_path = f"{mod_name}::{main_class}"

    main_lines = [
        "#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]",
        "mod java_runtime;",
        *[f"mod {m};" for m in top_mods],
        f"use {use_path};",
        "",
        f"fn main() {{",
        f"    {main_class}::main().unwrap_or_else(|e| eprintln!(\"JVM Error: {{:?}}\", e));",
        f"}}",
        "",
    ]
    with open(os.path.join(src_dir, 'main.rs'), 'w') as f:
        f.write('\n'.join(main_lines))

    # 9. 写 JDK 类完整翻译文件（不在 mod 树中，不参与 Rust 模块编译）
    if jdk_class_infos:
        for jdk_ci in jdk_class_infos:
            file_path = _jdk_class_file_path(src_dir, jdk_ci.name)
            os.makedirs(os.path.dirname(file_path), exist_ok=True)
            with open(file_path, 'w') as f:
                f.write(_gen_class_rs(jdk_ci, registry=registry))
        print(f"[codegen] JDK 翻译 → {len(jdk_class_infos)} 个类")

    print(f"[codegen] Cargo project → {out_dir}/")
