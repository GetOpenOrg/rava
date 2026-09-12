"""
Rust 文件生成器：将 ClassInfo 列表写出为 Cargo 项目，
按 Java 包路径组织模块，java_runtime 按 JDK 包路径分层。
"""

import os
import re
from .types import ClassInfo
from .type_map import jvm_to_rust, sig_type, rust_default
from .method import gen_method_body, _indent
from .runtime import RUNTIME_FILES

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


def _gen_class_rs(ci: ClassInfo) -> str:
    """生成单个 Java 类对应的完整 .rs 文件内容"""
    parts: list[str] = [
        "#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]",
        "use std::rc::Rc;",
        "use std::cell::RefCell;",
        "use std::collections::HashMap;",
        "use std::collections::HashSet;",
        "",
    ]

    inst_fields           = [f for f in ci.fields if not f.is_static]
    has_instance_methods  = any(not m.is_static and not m.is_constructor for m in ci.methods)

    if inst_fields:
        decls = '\n'.join(f"    pub {f.name}: {jvm_to_rust(f.descriptor)}," for f in inst_fields)
        parts.append(f"#[derive(Debug, Clone, Default)]\npub struct {ci.name} {{\n{decls}\n}}\n")
    elif has_instance_methods:
        parts.append(f"#[derive(Debug, Clone, Default)]\npub struct {ci.name};\n")
    else:
        parts.append(f"pub struct {ci.name};\n")

    method_blocks: list[str] = []
    for m in ci.methods:
        if m.name == '<clinit>':
            continue
        try:
            method_blocks.append(gen_method_body(m, ci))
        except Exception as e:
            method_blocks.append(f"/* codegen error {m.name}: {e} */")

    impl_body = '\n\n'.join(_indent(b) for b in method_blocks)
    parts.append(f"impl {ci.name} {{\n{impl_body}\n}}\n")
    return '\n'.join(parts)


# ── 主函数 ────────────────────────────────────────────────────────

def write_cargo_project(out_dir: str, class_infos: list[ClassInfo], java_files: list[str] | None = None):
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
    mod_tree: dict[str, set[str]] = {}
    for ci in class_infos:
        _, pkg_parts, mod_name = layout[ci.name]
        parent = src_dir
        for part in pkg_parts:
            mod_tree.setdefault(parent, set()).add(part)
            parent = os.path.join(parent, part)
        mod_tree.setdefault(parent, set()).add(mod_name)

    # 6. 写各类的 .rs 文件
    for ci in class_infos:
        file_path, _, _ = layout[ci.name]
        os.makedirs(os.path.dirname(file_path), exist_ok=True)
        with open(file_path, 'w') as f:
            f.write(_gen_class_rs(ci))

    # 7. 写中间包目录的 mod.rs
    for dir_path, children in mod_tree.items():
        if dir_path == src_dir:
            continue
        os.makedirs(dir_path, exist_ok=True)
        with open(os.path.join(dir_path, 'mod.rs'), 'w') as f:
            f.write('\n'.join(f"pub mod {c};" for c in sorted(children)) + '\n')

    # 8. 写 main.rs
    main_class = class_infos[0].name if class_infos else 'Main'
    top_mods   = sorted(mod_tree.get(src_dir, set()))

    _, pkg_parts, mod_name = layout[main_class]
    use_path = '::'.join(pkg_parts + [mod_name, main_class]) if pkg_parts else f"{mod_name}::{main_class}"

    main_lines = [
        "#![allow(unused_variables, unused_mut, dead_code, non_snake_case)]",
        "mod java_runtime;",
        *[f"mod {m};" for m in top_mods],
        f"use {use_path};",
        "",
        f"fn main() {{ {main_class}::main(); }}",
        "",
    ]
    with open(os.path.join(src_dir, 'main.rs'), 'w') as f:
        f.write('\n'.join(main_lines))

    print(f"[codegen] Cargo project → {out_dir}/")
