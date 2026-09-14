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
from .type_map import jvm_to_rust, rust_default, short_cls, get_ergonomic_jvm_rename
from .method import gen_method_body, _indent
from .sig_parser import parse_class_type_params, parse_field_type
from .constants import safe_ident, RUST_KEYWORDS as _RUST_KEYWORDS

# Access flags
_ACC_PUBLIC       = 0x0001
_ACC_PRIVATE      = 0x0002
_ACC_PROTECTED    = 0x0004
_ACC_STATIC       = 0x0008
_ACC_FINAL        = 0x0010
_ACC_SYNCHRONIZED = 0x0020
_ACC_VOLATILE     = 0x0040  # 字段：volatile；方法：bridge
_ACC_TRANSIENT    = 0x0080  # 字段：transient；方法：varargs
_ACC_NATIVE       = 0x0100
_ACC_INTERFACE    = 0x0200
_ACC_ABSTRACT     = 0x0400
_ACC_SYNTHETIC    = 0x1000
_ACC_ANNOTATION   = 0x2000
_ACC_ENUM         = 0x4000

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
    """将 access_flags 整数转为访问权限字符串（public/protected/private/package）。"""
    if flags & _ACC_PUBLIC:    return 'public'
    if flags & _ACC_PRIVATE:   return 'private'
    if flags & _ACC_PROTECTED: return 'protected'
    return 'package'


def _class_modifiers_str(flags: int) -> str:
    """从类 access_flags 提取修饰符（排除访问权限）。"""
    parts = []
    if flags & _ACC_FINAL:      parts.append('final')
    if flags & _ACC_ABSTRACT:   parts.append('abstract')
    if flags & _ACC_INTERFACE:  parts.append('interface')
    if flags & _ACC_ENUM:       parts.append('enum')
    if flags & _ACC_ANNOTATION: parts.append('annotation')
    if flags & _ACC_SYNTHETIC:  parts.append('synthetic')
    return ' '.join(parts)


def _field_modifiers_str(flags: int) -> str:
    """从字段 access_flags 提取修饰符（static/final/volatile/transient/synthetic）。"""
    parts = []
    if flags & _ACC_STATIC:    parts.append('static')
    if flags & _ACC_FINAL:     parts.append('final')
    if flags & _ACC_VOLATILE:  parts.append('volatile')
    if flags & _ACC_TRANSIENT: parts.append('transient')
    if flags & _ACC_SYNTHETIC: parts.append('synthetic')
    return ' '.join(parts)


def _method_modifiers_str(flags: int) -> str:
    """从方法 access_flags 提取修饰符（static/final/synchronized/native/abstract/bridge/varargs）。"""
    parts = []
    if flags & _ACC_STATIC:       parts.append('static')
    if flags & _ACC_FINAL:        parts.append('final')
    if flags & _ACC_SYNCHRONIZED: parts.append('synchronized')
    if flags & _ACC_NATIVE:       parts.append('native')
    if flags & _ACC_ABSTRACT:     parts.append('abstract')
    if flags & _ACC_VOLATILE:     parts.append('bridge')    # ACC_BRIDGE 与 ACC_VOLATILE 同值
    if flags & _ACC_TRANSIENT:    parts.append('varargs')   # ACC_VARARGS 与 ACC_TRANSIENT 同值
    if flags & _ACC_SYNTHETIC:    parts.append('synthetic')
    return ' '.join(parts)


def _java_class_attr(ci: ClassInfo, compiled: bool = False) -> str:
    """生成 #[java_class(...)] 属性块。

    compiled=True：生成真实的 #[java_rta_macros::java_class(...)]，proc-macro 会自动派生
                   Into<Object>、From<Object>、Debug（Object 类除外）。
    compiled=False：原生属性格式，用于不参与编译的 JDK 元数据存根文件。
    """
    binary_name = ci.name
    super_class = ci.super_class or ""
    interfaces  = ','.join(ci.interfaces) if ci.interfaces else ""
    access      = _access_str(ci.access_flags) if ci.access_flags else ""
    modifiers   = _class_modifiers_str(ci.access_flags) if ci.access_flags else ""
    source      = ci.source_file or ""
    inner_lines = [
        f'    binary_name = "{binary_name}",',
        f'    super_class = "{super_class}",',
        f'    interfaces  = "{interfaces}",',
        f'    access      = "{access}",',
        f'    modifiers   = "{modifiers}",',
        f'    source      = "{source}",',
    ]
    inner = '\n'.join(inner_lines)
    if compiled:
        # Object 类自身不使用宏（from_any/downcast 定义在 Object 上，循环依赖）
        struct_name = ci.name.split('/')[-1]
        if struct_name == 'Object':
            return f'#[cfg_attr(any(), java_class(\n{inner}\n))]'
        return f'#[java_rta_macros::java_class(\n{inner}\n)]'
    else:
        return f'#[java_class(\n{inner}\n)]'


def _java_field_attr(f: FieldInfo) -> str:
    """生成 #[cfg_attr(any(), java_field(...))] 属性行（编译安全）。"""
    parts = [f'name = "{f.name}"', f'descriptor = "{f.descriptor}"']
    if f.access_flags:
        parts.append(f'access = "{_access_str(f.access_flags)}"')
        mods = _field_modifiers_str(f.access_flags)
        parts.append(f'modifiers = "{mods}"')
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
        mods = _method_modifiers_str(m.access_flags)
        parts.append(f'modifiers = "{mods}"')
    inner = f'{tag}(' + ', '.join(parts) + ')'
    if compiled:
        if m.is_native:
            # native 方法用 cfg_attr 包裹，让 build.rs 能扫描到
            return f'#[cfg_attr(any(), java_native(' + ', '.join(parts) + '))]'
        # 注释形式，避免 cfg_attr 内关键字字符串触发词法错误
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
                     native_fn: str | None = None,
                     registry: dict | None = None) -> str:
    """为 native / abstract / stub 方法生成存根，若有 native_fn 则调用 _native 模块。"""
    from .type_map import jvm_to_rust, sig_type, parse_descriptor_params, parse_descriptor_return
    params = parse_descriptor_params(m.descriptor)
    ret    = parse_descriptor_return(m.descriptor)
    rust_ret = jvm_to_rust(ret, registry)

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
    param_type_fn = (lambda p: sig_type(jvm_to_rust(p, registry))) if m.is_static else (lambda p: jvm_to_rust(p, registry))
    args_str = ', '.join(
        f'{name}: {param_type_fn(p)}' for name, p in zip(arg_names, params)
    )

    if m.is_static or m.is_constructor:
        sig_self = ''
    else:
        sig_self = '&self'
        if args_str:
            sig_self += ', '

    # 构造器返回 Result<Self>，其他方法按描述符决定
    if m.is_constructor:
        ret_type = 'Result<Self>'
    else:
        ret_type = f'Result<{rust_ret}>' if rust_ret != '()' else 'Result<()>'
    fn_name = safe_ident(rust_name or m.name)
    # Java clone() 与 Rust Clone trait 同名冲突：重命名为 jvm_clone
    if fn_name == 'clone':
        fn_name = 'jvm_clone'
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
                  call_chain: set | None = None,
                  native_impls_map: dict | None = None,
                  workspace_root: str | None = None,
                  user_crate_prefix: str | None = None,
                  synthetics: dict | None = None,
                  extra_fields: dict | None = None,
                  conflict_map: dict | None = None,
                  skipped_classes: set | None = None) -> str:
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

    # 若文件所在包未包含在全局 glob 导入中（如 jdk/ 前缀），则补充自身包的 glob 导入
    # 以确保同包兄弟类型（如内部接口）可直接引用
    if ci.name and '/' in ci.name:
        _own_pkg_parts = ci.name.split('/')[:-1]
        _own_pkg_path = '::'.join(
            f'r#{p}' if p in _RUST_KEYWORDS else p for p in _own_pkg_parts
        )
        _existing_pkgs = set(jdk_crate_pkg_paths) if jdk_crate_pkg_paths else set()
        if _own_pkg_path not in _existing_pkgs:
            _pfx = user_crate_prefix or 'crate'
            cross_imports.append(f"use {_pfx}::{_own_pkg_path}::*;")

    # 以下两个机制（冲突消歧 + jdk/ 显式导入）共享同一次指令扫描
    _need_refs = bool(conflict_map or skipped_classes)
    _referenced: set[str] = set()
    if _need_refs:
        # 扫描方法指令中的类型引用
        for _m in ci.methods:
            for _instr in (_m.instrs or []):
                _c = _instr.comment
                if not _c:
                    continue
                if _c.startswith(('Method ', 'InterfaceMethod ')):
                    _rest = _c.split(' ', 1)[1]
                    _dot = _rest.find('.')
                    if _dot > 0:
                        _referenced.add(_rest[:_dot])
                elif _c.startswith('Field '):
                    _rest = _c[6:]
                    _dot = _rest.find('.')
                    if _dot > 0:
                        _referenced.add(_rest[:_dot])
        # 扫描字段描述符（含超类链继承字段）中引用的类型（含 jdk/ 等跨包引用）
        import re as _re
        _all_fields_to_scan = list(ci.fields)
        if registry:
            _sc_scan = ci.super_class
            _seen_scan: set[str] = {f.name for f in ci.fields}
            while _sc_scan and _sc_scan != 'java/lang/Object' and _sc_scan in registry:
                _sci_scan = registry[_sc_scan]
                for _f2 in _sci_scan.fields:
                    if not _f2.is_static and _f2.name not in _seen_scan:
                        _all_fields_to_scan.append(_f2)
                        _seen_scan.add(_f2.name)
                _sc_scan = _sci_scan.super_class
        for _f in _all_fields_to_scan:
            for _m in _re.finditer(r'L([^;]+);', _f.descriptor or ''):
                _referenced.add(_m.group(1))
            for _m in _re.finditer(r'L([^;]+);', _f.generic_signature or ''):
                _referenced.add(_m.group(1))
        # 扫描方法描述符（参数和返回值）中引用的类型
        for _method in ci.methods:
            for _m in _re.finditer(r'L([^;]+);', _method.descriptor or ''):
                _referenced.add(_m.group(1))
            for _m in _re.finditer(r'L([^;]+);', getattr(_method, 'generic_signature', '') or ''):
                _referenced.add(_m.group(1))

    # 消歧：当同一简单名存在于多个包中时，追加显式 use 覆盖 glob 歧义
    if conflict_map:
        _own_pkg = '/'.join(ci.name.split('/')[:-1]) if '/' in ci.name else ''
        _prefix = user_crate_prefix or 'crate'

        def _pkg_to_use(pkg_slash: str) -> str:
            return _prefix + '::' + '::'.join(
                f'r#{p}' if p in _RUST_KEYWORDS else p
                for p in pkg_slash.split('/')
            )

        # 本文件自身定义的类型简名（跳过：不能 use 自己定义的名字）
        # Java 内部类用 $ 分隔，Rust struct 名用 _ 替代
        _self_simple = (ci.name.split('/')[-1] if '/' in ci.name else ci.name).replace('$', '_')

        for _sn, _pkgs in conflict_map.items():
            # 跳过：本文件就是该类型的定义文件（struct 名会与 use 重复导致 E0255）
            if _sn.replace('$', '_') == _self_simple:
                continue
            # 优先：指令中有直接引用的包
            _matches = [p for p in _pkgs if f'{p}/{_sn}' in _referenced]
            if len(_matches) == 1:
                _chosen = _matches[0]
            elif _own_pkg in _pkgs:
                # 次优：文件自身所在包（引用了与本包同名的类型，用本包版本）
                _chosen = _own_pkg
            elif _matches:
                _chosen = _matches[0]
            else:
                # 回退：优先 java/ 包，否则取第一个
                _java = [p for p in _pkgs if p.startswith('java/')]
                _chosen = _java[0] if _java else _pkgs[0]
            # Java 内部类 $ → Rust struct 名用 _
            cross_imports.append(f"use {_pkg_to_use(_chosen)}::{_sn.replace('$', '_')};")

    # 为被跳过全局导入的包（如 jdk/）中的类型，按需添加逐文件显式导入
    if skipped_classes and _referenced:
        _prefix2 = user_crate_prefix or 'crate'
        _self_simple2 = (ci.name.split('/')[-1] if '/' in ci.name else ci.name).replace('$', '_')
        _added_skipped: set[str] = set()
        for _full_cls in sorted(_referenced):
            _cls_parts = _full_cls.split('/')
            if len(_cls_parts) < 2:
                continue
            _rust_pkg = '::'.join(
                f'r#{p}' if p in _RUST_KEYWORDS else p for p in _cls_parts[:-1]
            )
            # Java 内部类 $ → Rust struct 名用 _
            _simple = _cls_parts[-1].replace('$', '_')
            if f"{_rust_pkg}::{_simple}" in skipped_classes and _simple != _self_simple2 and _simple not in _added_skipped:
                cross_imports.append(f"use {_prefix2}::{_rust_pkg}::{_simple};")
                _added_skipped.add(_simple)

    parts: list[str] = [
        "#![allow(unused_variables, unused_mut, dead_code, non_snake_case, unused_imports, non_camel_case_types)]",
        "use java_runtime::prelude::*;",
        *cross_imports,
        "",
        _java_class_attr(ci, compiled=True),
    ]
    field_type_prefix = "JField"

    inst_fields = [f for f in ci.fields if not f.is_static]

    # T76：_super 嵌套字段替代字段展平
    # 若有父类（且不是 Object），在实例字段前插入 _super: ParentType
    # proc-macro 读 super_class 注解，自动生成 From<Self> for Parent
    # emitter 在 struct 定义后额外生成显式 upcast 方法（as_xxx / into_xxx）
    _has_super = bool(
        ci.super_class and ci.super_class != 'java/lang/Object'
    )

    # 用短名作为 Rust 标识符（JDK 类的 ci.name 含 /，不是合法 Rust 名）
    struct_name = short_cls(ci.name) if '/' in ci.name else ci.name

    # 解析类级泛型参数（如 ArrayList<E>、HashMap<K,V>）
    class_type_params = parse_class_type_params(ci.generic_signature) if ci.generic_signature else []

    # 构建泛型参数字符串（用于 struct 和 impl 头）
    if class_type_params:
        type_params_str = ', '.join(class_type_params)
        # 泛型参数需要 Clone + Default + 'static；Default 是必须的，因为 _super: Default::default() 要求父链所有类型参数实现 Default
        bounds_str = ', '.join(f"{p}: Clone + Default + 'static" for p in class_type_params)
        struct_generic = f"<{bounds_str}>"
        impl_header   = f"impl<{bounds_str}> {struct_name}<{type_params_str}>"
    else:
        struct_generic = ''
        impl_header   = f"impl {struct_name}"

    cls_extra_fields = (extra_fields or {}).get(ci.name, [])
    if inst_fields or cls_extra_fields or _has_super:
        field_lines = []
        # _super 字段：嵌入直接父类（T76，替代字段展平）
        if _has_super:
            parent_rust = short_cls(ci.super_class)
            if registry and ci.super_class in registry:
                parent_ci = registry[ci.super_class]
                parent_params = parse_class_type_params(parent_ci.generic_signature) if parent_ci.generic_signature else []
                if parent_params:
                    if class_type_params:
                        # 子类有泛型参数：传播给父类，不足的用 Object 填充
                        args = class_type_params[:len(parent_params)]
                        while len(args) < len(parent_params):
                            args.append('Object')
                        parent_rust += '<' + ', '.join(args) + '>'
                    else:
                        # 子类无泛型参数但父类需要（如 CharacterUnicodeScript extends Enum<E>）：用 Object 后备
                        parent_rust += '<' + ', '.join('Object' for _ in parent_params) + '>'
            field_lines.append(f"    pub _super: {parent_rust},")
        for f in inst_fields:
            safe_fname = _safe_field_name(f.name)
            field_lines.append("    " + _java_field_attr(f))
            # 优先用字段级 generic_signature（如 TE; → E），回退到裸描述符
            # 若 generic_signature 解析结果是 Object（简化），用描述符推断更精确的类型
            gen_rust = (parse_field_type(f.generic_signature, class_type_params)
                        if f.generic_signature else '')
            desc_rust = jvm_to_rust(f.descriptor, registry)
            field_rust = gen_rust if (gen_rust and gen_rust != 'Object') else desc_rust
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

    # T76：为有父类的类生成显式 upcast 方法（as_xxx / into_xxx）
    # 不使用 Deref（Rust 反模式），改用显式方法，语义清晰
    if _has_super and registry:
        upcast_lines = [f'{impl_header} {{']
        access_path = '_super'
        cur_super = ci.super_class
        # upcast 方法只使用子类自身的类型参数（child_tparams 在 impl block 中始终可用）
        # 不传播祖先的参数名（祖先参数名在子类 impl block 中不可见）
        child_tparams: list[str] = list(class_type_params)
        while cur_super and cur_super != 'java/lang/Object':
            parent_rust_name = short_cls(cur_super)
            snake = to_snake(parent_rust_name.replace('$', '_'))
            # 计算该祖先级的完整类型表达式（含泛型参数），只用子类自身参数名
            parent_full_type = parent_rust_name
            ancestor_ci = registry.get(cur_super)
            if ancestor_ci:
                ancestor_params = parse_class_type_params(ancestor_ci.generic_signature) if ancestor_ci.generic_signature else []
                if ancestor_params:
                    if child_tparams:
                        args = child_tparams[:len(ancestor_params)]
                        while len(args) < len(ancestor_params):
                            args.append('Object')
                    else:
                        args = ['Object'] * len(ancestor_params)
                    parent_full_type += '<' + ', '.join(args) + '>'
            upcast_lines.append(f'    pub fn as_{snake}(&self) -> &{parent_full_type} {{ &self.{access_path} }}')
            upcast_lines.append(f'    pub fn into_{snake}(self) -> {parent_full_type} {{ self.{access_path} }}')
            cur_super = ancestor_ci.super_class if ancestor_ci else None
            access_path += '._super'
        upcast_lines.append('}')
        parts.append('\n'.join(upcast_lines) + '\n')

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
    # 生成静态字段 getter：有 native_impls 的用真实实现，其余生成 panic stub
    static_fields = [f for f in ci.fields if f.is_static]
    existing_method_names: set[str] = {m.name for m in visible_methods}
    for sf in static_fields:
        safe_fname = _safe_field_name(sf.name)
        if safe_fname in existing_method_names:
            continue  # 有同名方法，跳过（方法已覆盖此访问路径）
        rust_ret = jvm_to_rust(sf.descriptor, registry=registry)
        native_fn_key = (ci.name, sf.name, sf.descriptor)
        if native_impls_map and native_fn_key in native_impls_map:
            native_fn = native_impls_map[native_fn_key][0]
            body = f'_native::{native_fn}()'
        else:
            # 生成 panic stub，确保 getstatic 对应的 ClassName::fieldName() 能编译
            body = f'panic!("stub: {ci.name}.{sf.name}:{sf.descriptor}")'
        method_blocks.append(f'// static field: {sf.name}:{sf.descriptor}\npub fn {safe_fname}() -> {rust_ret} {{\n    {body}\n}}')

    used_rust_names: dict[str, int] = {}  # 追踪已用名，防止 mangle 碰撞后重名
    for m in visible_methods:
        if m.name == '<clinit>':
            continue
        # 确定最终 Rust 方法名（有重载则加描述符后缀）
        rust_name = mangle_name(m.name, m.descriptor) if m.name in overloaded_names else m.name
        # T39：若 _ergonomic.rs 有 @jvm_rename 指令，将非重载方法改名（腾出干净名称给 ergonomic 层）
        if not m.is_constructor and m.name not in overloaded_names:
            erg_rename = get_ergonomic_jvm_rename(ci.name, m.name)
            if erg_rename is not None:
                rust_name = erg_rename
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
        # 判断该方法是否需要翻译字节码：
        #   1. native / abstract → 永远生成 stub（调用 _native 或 panic!）
        #   2. call_chain 不为空 且 此方法不在调用链上 → panic!("stub: ...")
        #   3. stub_bodies=True（兜底/fallback）→ stub
        #   4. 其他 → 翻译字节码
        in_call_chain = (
            call_chain is None or
            (ci.name, m.name, m.descriptor) in call_chain
        )
        if m.is_native or m.is_abstract:
            native_fn = None
            if native_impls_map:
                nkey = (ci.name, m.name, m.descriptor)
                if nkey in native_impls_map:
                    native_fn = native_impls_map[nkey][0]
            stub = _gen_native_stub(m, ci, rust_name=rust_name, native_fn=native_fn, registry=registry)
            method_blocks.append(attr_line + '\n' + stub)
        elif not in_call_chain or stub_bodies:
            # 不在调用链上，或兜底 stub 模式：生成 panic! 存根
            stub = _gen_native_stub(m, ci, rust_name=rust_name, native_fn=None, registry=registry)
            method_blocks.append(attr_line + '\n' + stub)
        else:
            try:
                body = gen_method_body(
                    m, ci, registry=registry,
                    class_type_params=class_type_params,
                    overloaded_names=overloaded_names,
                    rust_name=rust_name,
                )
                method_blocks.append(attr_line + '\n' + body)
            except Exception as e:
                # 翻译失败：退化为 stub，避免生成无效 Rust
                stub = _gen_native_stub(m, ci, rust_name=rust_name, native_fn=None, registry=registry)
                method_blocks.append(attr_line + '\n' + stub)

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

    # Into<Object> / From<Object> / Debug 由 #[java_rta_macros::java_class] proc-macro 自动生成
    # （Object 类走手写路径 java_runtime/，不经过此函数）

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
    # Cargo.toml / build.rs 由 git 直接管理，emitter 不再写出
    jdk_src = os.path.join(jdk_dir, 'src')
    # 清理旧版生成文件（batch 模式由调用方在批次开始前统一清理）
    if not batch_bin and os.path.isdir(jdk_src):
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
            _write(file_path, _gen_class_rs(jdk_ci, registry=registry,
                                            jdk_crate_pkg_paths=jdk_crate_pkg_paths,
                                            call_chain=visited_methods,
                                            native_impls_map=native_impls_map,
                                            workspace_root=out_dir,
                                            synthetics=synthetics_map,
                                            extra_fields=extra_fields_map,
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
                                        extra_fields=extra_fields_map,
                                        conflict_map=conflict_map if jdk_class_infos else None,
                                        skipped_classes=skipped_classes if jdk_class_infos else None))

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
