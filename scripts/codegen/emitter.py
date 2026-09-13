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
from .sig_parser import parse_class_type_params, parse_field_type
from .constants import safe_ident, RUST_KEYWORDS as _RUST_KEYWORDS

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



# ── 辅助 ─────────────────────────────────────────────────────────

_safe_field_name = safe_ident


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

    compiled=True（JDK 类生成）：
      - 真正的 native 方法：输出 #[cfg_attr(any(), java_native(...))]，供 build.rs 扫描
      - 其他方法：输出行注释 `// java: ...`（避免关键字字符串引发词法错误）
    compiled=False（元数据存根）：原生属性格式 #[java_method(...)] / #[java_native(...)]。
    """
    tag = 'java_native' if m.is_native else 'java_method'
    parts = [f'name = "{m.name}"', f'descriptor = "{m.descriptor}"']
    if m.access_flags:
        parts.append(f'access = "{_access_str(m.access_flags)}"')
    inner = f'{tag}(' + ', '.join(parts) + ')'
    if compiled:
        if m.is_native:
            # native 方法用 cfg_attr 包裹，让 build.rs 能扫描到
            return f'#[cfg_attr(any(), java_native(' + ', '.join(parts) + '))]'
        # 注释形式，避免 cfg_attr 内 "public static" 等含关键字字符串触发词法错误
        return f'// java: {m.name}{m.descriptor}'
    else:
        return f'#[{inner}]'


_safe_param_name = safe_ident


def _parse_synthetic_fn(line: str) -> dict | None:
    """解析 'pub fn name(params) -> ret' 行，返回 synthetic 方法信息。"""
    m = re.match(r'\s*pub fn\s+(\w+)\s*\(([^)]*)\)\s*(?:->\s*(.+?))?\s*\{?\s*$', line)
    if not m:
        return None
    fn_name = m.group(1)
    raw_params = m.group(2).strip()
    ret_type = (m.group(3) or '()').strip().rstrip('{').strip()

    # 解析参数列表，确定 self 类型和其余参数
    param_parts = [p.strip() for p in raw_params.split(',') if p.strip()]
    is_static = True
    is_mut_self = False
    rest_params = param_parts

    if param_parts and param_parts[0].startswith('_this:'):
        is_static = False
        self_decl = param_parts[0]
        is_mut_self = '&mut' in self_decl
        rest_params = param_parts[1:]

    # 提取参数名和类型（用于 wrapper 声明和调用）
    params_decl_parts = []
    call_arg_names = []
    for p in rest_params:
        colon_idx = p.find(':')
        if colon_idx > 0:
            pname = p[:colon_idx].strip()
            ptype = p[colon_idx+1:].strip()
            params_decl_parts.append(f'{pname}: {ptype}')
            call_arg_names.append(pname)
        else:
            params_decl_parts.append(p)
            call_arg_names.append(p)

    return {
        'fn_name': fn_name,
        'is_static': is_static,
        'is_mut_self': is_mut_self,
        'params_decl': ', '.join(params_decl_parts),
        'call_args': ', '.join(call_arg_names),
        'ret_type': ret_type,
    }


def _scan_native_impls(workspace_root: str) -> tuple[dict, dict, dict]:
    """扫描 native_impls/ 目录，解析 /// java/Class.method:descriptor 注释。
    返回:
      native_map:   {(class_binary_name, member_name, descriptor): (rust_fn_name, rel_file)}
      synthetics:   {class_binary_name: [synthetic_info_dict, ...]}
      extra_fields: {class_binary_name: [(field_name, rust_type), ...]}
    rel_file 相对于 workspace_root。
    synthetic 函数用 /// @synthetic 标注（不对应任何 Java 方法）。
    extra_fields 通过 /// @field name: RustType 注释声明，注入到生成的 struct 中。
    """
    impls_dir = os.path.join(workspace_root, 'native_impls')
    result: dict = {}
    synthetics: dict = {}
    extra_fields: dict = {}
    if not os.path.isdir(impls_dir):
        return result, synthetics
    for dirpath, _, files in os.walk(impls_dir):
        for fname in sorted(files):
            if not fname.endswith('.rs'):
                continue
            fpath = os.path.join(dirpath, fname)
            rel = os.path.relpath(fpath, workspace_root).replace('\\', '/')
            # 从文件路径推断 class binary name（去掉 native_impls/ 前缀和 .rs 后缀）
            rel_no_ext = rel[:-3] if rel.endswith('.rs') else rel
            # native_impls/java/lang/string.rs → java/lang/String
            parts_from_rel = rel_no_ext.replace('\\', '/').split('/')
            if parts_from_rel and parts_from_rel[0] == 'native_impls':
                parts_from_rel = parts_from_rel[1:]
            # snake_to_class: string → String, print_stream → PrintStream
            def _snake_to_class(s: str) -> str:
                return ''.join(w.capitalize() for w in s.split('_'))
            if parts_from_rel:
                *pkg, cls_snake = parts_from_rel
                class_binary = '/'.join(pkg + [_snake_to_class(cls_snake)])
            else:
                class_binary = ''
            try:
                content = open(fpath, encoding='utf-8').read()
            except Exception:
                continue
            lines = content.splitlines()
            i = 0
            while i < len(lines):
                stripped = lines[i].strip()
                if stripped.startswith('/// @field ') and ':' in stripped and class_binary:
                    # /// @field field_name: RustType → 注入额外的 struct 字段
                    field_decl = stripped[len('/// @field '):].strip()
                    colon = field_decl.index(':')
                    fname = field_decl[:colon].strip()
                    ftype = field_decl[colon + 1:].strip()
                    extra_fields.setdefault(class_binary, []).append((fname, ftype))
                    i += 1
                elif stripped == '/// @synthetic':
                    # 找下一个 pub fn
                    j = i + 1
                    while j < len(lines) and not lines[j].strip().startswith('pub fn'):
                        j += 1
                    if j < len(lines) and class_binary:
                        info = _parse_synthetic_fn(lines[j])
                        if info:
                            synthetics.setdefault(class_binary, []).append(info)
                    i = j + 1
                elif stripped.startswith('/// java/') and '.' in stripped:
                    java_ref = stripped[4:].strip()
                    dot_idx = java_ref.rfind('.')
                    if dot_idx < 0:
                        i += 1
                        continue
                    class_part = java_ref[:dot_idx]
                    rest = java_ref[dot_idx + 1:]
                    colon_idx = rest.find(':')
                    if colon_idx >= 0:
                        member_name = rest[:colon_idx]
                        descriptor = rest[colon_idx + 1:]
                    else:
                        member_name = rest
                        descriptor = ''
                    # 收集后续 /// 注释行，检查 not-needed 标记
                    j = i + 1
                    not_needed = False
                    while j < len(lines) and lines[j].strip().startswith('///'):
                        if 'not-needed' in lines[j]:
                            not_needed = True
                        j += 1
                    if not_needed:
                        i = j
                        continue
                    # 找下一个 pub fn
                    while j < len(lines) and not lines[j].strip().startswith('pub fn'):
                        j += 1
                    if j < len(lines):
                        m2 = re.match(r'\s*pub fn\s+(\w+)', lines[j])
                        if m2:
                            rust_fn = m2.group(1)
                            key = (class_part, member_name, descriptor)
                            result[key] = (rust_fn, rel)
                    i = j + 1
                else:
                    i += 1
    return result, synthetics, extra_fields


def _gen_native_stub(m: ParsedMethod, ci: ClassInfo, rust_name: str | None = None,
                     native_fn: str | None = None) -> str:
    """为 native / abstract / stub 方法生成存根，若有 native_fn 则调用 _native 模块。"""
    from .type_map import jvm_to_rust, sig_type, parse_descriptor_params, parse_descriptor_return
    params = parse_descriptor_params(m.descriptor)
    ret    = parse_descriptor_return(m.descriptor)
    rust_ret = jvm_to_rust(ret)

    # 构建参数列表（参数名需转义 $ 和 Rust 关键字）
    raw_names = [m.local_names.get(i + (0 if m.is_static else 1), f'arg{i}')
                 for i in range(len(params))]
    arg_names = [_safe_param_name(n) for n in raw_names]
    # 防止去重后重名：加序号后缀
    seen: dict[str, int] = {}
    deduped = []
    for n in arg_names:
        if n in seen:
            seen[n] += 1
            deduped.append(f'{n}{seen[n]}')
        else:
            seen[n] = 0
            deduped.append(n)
    arg_names = deduped
    # 静态方法用 sig_type（Vec<T> → &[T]），与 gen_method_body 保持一致
    param_type_fn = (lambda p: sig_type(jvm_to_rust(p))) if m.is_static else jvm_to_rust
    args_str = ', '.join(
        f'{name}: {param_type_fn(p)}' for name, p in zip(arg_names, params)
    )

    if m.is_static:
        sig_self = ''
    else:
        sig_self = '&self'
        if args_str:
            sig_self += ', '

    ret_type = f'Result<{rust_ret}>' if rust_ret != '()' else 'Result<()>'
    fn_name = safe_ident(rust_name or m.name)
    if native_fn:
        # 调用 _native 模块中的手写实现
        if m.is_static:
            call_args = ', '.join(arg_names)
            body = f'_native::{native_fn}({call_args})'
        else:
            call_args = ', '.join(['self'] + arg_names)
            body = f'_native::{native_fn}({call_args})'
    else:
        label = 'native' if m.is_native else 'stub'
        body = f'panic!("{label}: {ci.name}.{m.name}:{m.descriptor}")'

    return (
        f'pub fn {fn_name}({sig_self}{args_str}) -> {ret_type} {{\n'
        f'    {body}\n'
        f'}}'
    )


def _gen_class_rs(ci: ClassInfo, registry: dict | None = None,
                  jdk_crate_pkg_paths: list[str] | None = None,
                  stub_bodies: bool = False,
                  native_impls_map: dict | None = None,
                  workspace_root: str | None = None,
                  user_crate_prefix: str | None = None,
                  synthetics: dict | None = None,
                  extra_fields: dict | None = None) -> str:
    """生成单个 Java 类对应的完整 .rs 文件内容。

    生成规则：
    - 实例字段用 Field<T> 包装（提供 Java 字段语义的内部可变性）
    - 方法直接在 impl 块中，无 raw:: 子模块
    - 所有方法返回 Result<T>
    - 每个 struct / field / method 前加 // @java_* 注释供 build.rs 扫描
    - native_impls_map: 若提供，则为有实现的方法调用 _native::fn()，并插入 mod _native
    - user_crate_prefix: 若提供（如 'jdk_classes'），cross_imports 用该 crate 前缀
    """
    from collections import Counter
    from .type_map import mangle_name

    # 跨包 glob import：让生成代码能直接用 Objects、Integer 等翻译过的 JDK 类型
    cross_imports: list[str] = []
    if jdk_crate_pkg_paths:
        prefix = user_crate_prefix or 'crate'
        for pkg_path in jdk_crate_pkg_paths:
            cross_imports.append(f"use {prefix}::{pkg_path}::*;")

    parts: list[str] = [
        "#![allow(unused_variables, unused_mut, dead_code, non_snake_case, unused_imports, non_camel_case_types)]",
        "use java_runtime::prelude::*;",
        *cross_imports,
        "",
        _java_class_attr(ci, compiled=True),
    ]
    field_type_prefix = "JField"

    inst_fields = [f for f in ci.fields if not f.is_static]

    # 用短名作为 Rust 标识符（JDK 类的 ci.name 含 /，不是合法 Rust 名）
    struct_name = short_cls(ci.name) if '/' in ci.name else ci.name

    # 解析类级泛型参数（如 ArrayList<E>、HashMap<K,V>）
    class_type_params = parse_class_type_params(ci.generic_signature) if ci.generic_signature else []

    # 构建泛型参数字符串（用于 struct 和 impl 头）
    if class_type_params:
        type_params_str = ', '.join(class_type_params)
        # 结构体和 impl 只需 Clone + 'static；#[derive(Default)] 宏会自动在 impl Default
        # 的 where 子句中添加 E: Default，不影响普通 use 场景（不需要 E: Default）
        bounds_str = ', '.join(f"{p}: Clone + 'static" for p in class_type_params)
        struct_generic = f"<{bounds_str}>"
        impl_header   = f"impl<{bounds_str}> {struct_name}<{type_params_str}>"
    else:
        struct_generic = ''
        impl_header   = f"impl {struct_name}"

    cls_extra_fields = (extra_fields or {}).get(ci.name, [])
    if inst_fields or cls_extra_fields:
        field_lines = []
        for f in inst_fields:
            safe_fname = _safe_field_name(f.name)
            field_lines.append("    " + _java_field_attr(f))
            # 优先用字段级 generic_signature（如 TE; → E），回退到裸描述符
            field_rust = (parse_field_type(f.generic_signature, class_type_params)
                          if f.generic_signature else '') or jvm_to_rust(f.descriptor)
            field_lines.append(f"    pub {safe_fname}: {field_type_prefix}<{field_rust}>,")
        for ef_name, ef_type in cls_extra_fields:
            field_lines.append(f"    pub {ef_name}: {ef_type},")
        # 若有泛型参数但字段中未用到，加 PhantomData 防止 E0392
        if class_type_params:
            phantom_ty = ', '.join(f'std::marker::PhantomData<{p}>' for p in class_type_params)
            if len(class_type_params) > 1:
                phantom_ty = f'std::marker::PhantomData<({", ".join(class_type_params)},)>'
            field_lines.append(f"    pub _phantom: {phantom_ty},")
        decls = '\n'.join(field_lines)
        parts.append(f"#[derive(Clone, Default)]\npub struct {struct_name}{struct_generic} {{\n{decls}\n}}\n")
    else:
        if class_type_params:
            # 无字段但有泛型参数：改用 tuple struct 包含 PhantomData
            if len(class_type_params) == 1:
                phantom_ty = f'std::marker::PhantomData<{class_type_params[0]}>'
            else:
                phantom_ty = f'std::marker::PhantomData<({", ".join(class_type_params)},)>'
            parts.append(f"#[derive(Clone, Default)]\npub struct {struct_name}{struct_generic}({phantom_ty});\n")
        else:
            parts.append(f"#[derive(Clone, Default)]\npub struct {struct_name}{struct_generic};\n")

    # 若有 native_impls 或 synthetics，插入 mod _native { include!("..."); } 块
    has_synthetics_here = synthetics and ci.name in synthetics
    _native_rel_file: str | None = None
    if native_impls_map:
        class_native_impls = {k: v for k, v in native_impls_map.items() if k[0] == ci.name}
        if class_native_impls:
            _native_rel_file = next(iter(class_native_impls.values()))[1]
    # 若没有 /// java/ 条目但有 synthetics，也需要找 native_impls 文件路径
    if _native_rel_file is None and has_synthetics_here and workspace_root:
        # 推断路径：native_impls/java/pkg/class_snake.rs
        *pkg_parts_ci, cls_ci = ci.name.split('/')
        cls_snake = to_snake(cls_ci)
        candidate = os.path.join('native_impls', *pkg_parts_ci, cls_snake + '.rs')
        if os.path.isfile(os.path.join(workspace_root, candidate)):
            _native_rel_file = candidate.replace('\\', '/')
    if _native_rel_file and workspace_root:
        pkg_depth = len(ci.name.split('/')) - 1
        ups = '../' * (pkg_depth + 2)
        include_path = ups + _native_rel_file
        native_mod_lines = [
            'mod _native {',
            '    #![allow(unused_imports, dead_code, unused_variables, non_snake_case, non_camel_case_types)]',
            '    use java_runtime::prelude::*;',
            '    use super::*;',
        ]
        for imp in cross_imports:
            native_mod_lines.append(f'    {imp}')
        native_mod_lines.append(f'    include!("{include_path}");')
        native_mod_lines.append('}')
        parts.append('\n'.join(native_mod_lines) + '\n')

    # 过滤 synthetic 方法（编译器合成桥接方法），再统计重载
    visible_methods = [m for m in ci.methods if not m.is_synthetic]
    name_counts = Counter(m.name for m in visible_methods if m.name != '<clinit>')
    overloaded_names: set[str] = {name for name, count in name_counts.items() if count > 1}

    method_blocks: list[str] = []

    # public static 字段的 getter 方法（用于 getstatic 访问，如 System::out()）
    # 只生成 native_impls 中有明确实现的字段 getter（避免与同名方法冲突）
    static_fields = [f for f in ci.fields if f.is_static]
    existing_method_names: set[str] = {m.name for m in visible_methods}
    for sf in static_fields:
        native_fn_key = (ci.name, sf.name, sf.descriptor)
        if not (native_impls_map and native_fn_key in native_impls_map):
            continue  # 只为有 native_impls 实现的字段生成 getter
        safe_fname = _safe_field_name(sf.name)
        if safe_fname in existing_method_names:
            continue  # 有同名方法，跳过（方法已覆盖此访问路径）
        rust_ret = jvm_to_rust(sf.descriptor, registry=registry)
        native_fn = native_impls_map[native_fn_key][0]
        body = f'_native::{native_fn}()'
        method_blocks.append(f'// static field: {sf.name}:{sf.descriptor}\npub fn {safe_fname}() -> {rust_ret} {{\n    {body}\n}}')

    used_rust_names: dict[str, int] = {}  # 追踪已用名，防止 mangle 碰撞后重名
    for m in visible_methods:
        if m.name == '<clinit>':
            continue
        # 确定最终 Rust 方法名（有重载则加描述符后缀）
        rust_name = mangle_name(m.name, m.descriptor) if m.name in overloaded_names else m.name
        # 构造器统一用 new / new_suffix
        if m.is_constructor:
            if '<init>' in overloaded_names:
                rust_name = mangle_name('new', m.descriptor)
            else:
                rust_name = 'new'
            # 若 @synthetic 已占用 'new'（工厂函数），将 JDK <init> 存根改名为 new_init*
            # 避免：pub fn new(&self) 与 pub fn new() 同名冲突
            if rust_name == 'new' and synthetics and ci.name in synthetics:
                syn_names = {s['fn_name'] for s in synthetics[ci.name]}
                if 'new' in syn_names:
                    rust_name = 'new_init'
        # 碰撞去重：若 mangle 后仍重名，追加数字后缀
        if rust_name in used_rust_names:
            used_rust_names[rust_name] += 1
            rust_name = f'{rust_name}_{used_rust_names[rust_name]}'
        else:
            used_rust_names[rust_name] = 0

        attr_line = _java_method_attr(m, compiled=True)
        if m.is_native or m.is_abstract or stub_bodies:
            # native/abstract 方法，或 JDK 类以 stub_bodies=True 模式生成：
            # 只生成存根（或调用 _native 实现），确保 jdk_classes 可编译
            native_fn = None
            if native_impls_map:
                nkey = (ci.name, m.name, m.descriptor)
                if nkey in native_impls_map:
                    native_fn = native_impls_map[nkey][0]
            stub = _gen_native_stub(m, ci, rust_name=rust_name, native_fn=native_fn)
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

    # 合成方法 wrapper（来自 native_impls 中 /// @synthetic 标注的函数）
    if synthetics and ci.name in synthetics:
        for syn in synthetics[ci.name]:
            if syn['is_static']:
                wrapper = (
                    f"pub fn {syn['fn_name']}({syn['params_decl']}) -> {syn['ret_type']} {{\n"
                    f"    _native::{syn['fn_name']}({syn['call_args']})\n"
                    f"}}"
                )
            elif syn['is_mut_self']:
                sep = ', ' if syn['params_decl'] else ''
                wrapper = (
                    f"pub fn {syn['fn_name']}(&mut self{sep}{syn['params_decl']}) -> {syn['ret_type']} {{\n"
                    f"    _native::{syn['fn_name']}(self{', ' if syn['call_args'] else ''}{syn['call_args']})\n"
                    f"}}"
                )
            else:
                sep = ', ' if syn['params_decl'] else ''
                wrapper = (
                    f"pub fn {syn['fn_name']}(&self{sep}{syn['params_decl']}) -> {syn['ret_type']} {{\n"
                    f"    _native::{syn['fn_name']}(self{', ' if syn['call_args'] else ''}{syn['call_args']})\n"
                    f"}}"
                )
            method_blocks.append(wrapper)

    impl_body = '\n\n'.join(_indent(b) for b in method_blocks)
    parts.append(f"{impl_header} {{\n{impl_body}\n}}\n")

    # 自动生成 Into<Object> / From<Object> trait impl（所有非 Object 类均需要）
    # Into<Object>：将该类型装入 Object（JVM upcasting）
    # From<Object>：从 Object 中取出该类型（JVM checkcast / downcasting）
    if struct_name != 'Object':
        if class_type_params:
            tp_str  = ', '.join(class_type_params)
            bd_str  = ', '.join(f"{p}: Clone + 'static" for p in class_type_params)
            full_ty = f"{struct_name}<{tp_str}>"
            parts.append(
                f"impl<{bd_str}> Into<Object> for {full_ty} {{\n"
                f"    fn into(self) -> Object {{ Object::from_any(self) }}\n"
                f"}}\n"
            )
            parts.append(
                f"impl<{bd_str}> From<Object> for {full_ty} {{\n"
                f"    fn from(obj: Object) -> {full_ty} {{ obj.downcast::<{full_ty}>() }}\n"
                f"}}\n"
            )
        else:
            parts.append(
                f"impl Into<Object> for {struct_name} {{\n"
                f"    fn into(self) -> Object {{ Object::from_any(self) }}\n"
                f"}}\n"
            )
            parts.append(
                f"impl From<Object> for {struct_name} {{\n"
                f"    fn from(obj: Object) -> {struct_name} {{ obj.downcast::<{struct_name}>() }}\n"
                f"}}\n"
            )

    # 若存在同名 _ergonomic.rs，则直接 include! 到生成文件顶层（不在 mod _native 内）
    # 用于 ergonomic 泛型方法：impl<E: Into<Object> + From<Object>> Collection<E> { add/get_item/... }
    if _native_rel_file and workspace_root:
        ergonomic_rel = _native_rel_file.replace('.rs', '_ergonomic.rs')
        if os.path.isfile(os.path.join(workspace_root, ergonomic_rel)):
            pkg_depth = len(ci.name.split('/')) - 1
            ups = '../' * (pkg_depth + 2)
            erg_path = ups + ergonomic_rel
            parts.append(
                f"// ergonomic API: typed add/get using E directly (T39)\n"
                f"include!(\"{erg_path}\");\n"
            )

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
      java_runtime/  — 手写 VM 基础设施（git 管理，不由转译器写入）
      jdk_classes/   — JDK 字节码翻译
      user/          — 用户 Java 代码翻译
    """
    import shutil

    rt_dir   = os.path.join(out_dir, 'java_runtime')
    jdk_dir  = os.path.join(out_dir, 'jdk_classes')
    user_dir = os.path.join(out_dir, 'user')

    # 3. jdk_classes crate（JDK 字节码翻译）
    # Cargo.toml / build.rs 由 git 直接管理，emitter 不再写出
    jdk_src = os.path.join(jdk_dir, 'src')
    # 清理旧版生成文件：删除 src/ 下所有 .rs 文件，防止 stale 文件影响 native_status.toml
    if os.path.isdir(jdk_src):
        for root, _dirs, files in os.walk(jdk_src):
            for fname in files:
                if fname.endswith('.rs'):
                    os.remove(os.path.join(root, fname))

    # 构建 registry（用户类 + JDK 类）
    registry: dict = {ci.name: ci for ci in class_infos}
    if jdk_class_infos:
        for jci in jdk_class_infos:
            registry.setdefault(jci.name, jci)

    # 扫描 native_impls/ 目录，构建 native 映射和 synthetic 方法映射
    native_impls_map, synthetics_map, extra_fields_map = _scan_native_impls(out_dir)

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

        jdk_pkg_set: set[str] = set()
        for jdk_ci in jdk_class_infos:
            pkg_parts = jdk_ci.name.split('/')[:-1]
            if pkg_parts:
                jdk_pkg_set.add('::'.join(_safe_pkg_part(p) for p in pkg_parts))
        jdk_crate_pkg_paths = sorted(jdk_pkg_set)

        # 按调用链选择性开启字节码翻译；未列出的类保持 stub_bodies=True
        # 选择性开启字节码翻译的类集合（当前留空，按需添加）
        _TRANSLATE_BODIES: set[str] = set()  # 按需添加要翻译方法体的类

        for jdk_ci in jdk_class_infos:
            parts = jdk_ci.name.split('/')          # e.g. ['java','util','ArrayList']
            *pkg_parts, class_name = parts
            mod_name  = to_snake(class_name)
            parent_dir = os.path.join(jdk_src, *pkg_parts)
            # 跳过与子包目录同名的类文件（E0761：module.rs 和 module/mod.rs 不能共存）
            if mod_name in pkg_dir_names.get(parent_dir, set()):
                continue
            file_path = os.path.join(parent_dir, mod_name + '.rs')
            use_stubs = jdk_ci.name not in _TRANSLATE_BODIES
            _write(file_path, _gen_class_rs(jdk_ci, registry=registry,
                                            jdk_crate_pkg_paths=jdk_crate_pkg_paths,
                                            stub_bodies=use_stubs,
                                            native_impls_map=native_impls_map,
                                            workspace_root=out_dir,
                                            synthetics=synthetics_map,
                                            extra_fields=extra_fields_map))
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
        _write(os.path.join(dir_path, 'mod.rs'), '\n'.join(mod_lines) + '\n')

    # 4. user crate（用户 Java 翻译）
    # Cargo.toml 由 git 直接管理，emitter 不再写出
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
    user_pkg_paths = jdk_crate_pkg_paths if jdk_class_infos else None
    for ci in class_infos:
        file_path, _, _ = layout[ci.name]
        _write(file_path, _gen_class_rs(ci, registry=registry,
                                        jdk_crate_pkg_paths=user_pkg_paths,
                                        user_crate_prefix='jdk_classes',
                                        native_impls_map=native_impls_map,
                                        workspace_root=out_dir,
                                        synthetics=synthetics_map,
                                        extra_fields=extra_fields_map))

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

    if jdk_class_infos:
        print(f'[codegen] JDK 翻译 → {len(jdk_class_infos)} 个类')
    print(f'[codegen] Cargo workspace → {out_dir}/')
