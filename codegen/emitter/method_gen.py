"""
方法生成相关：_parse_synthetic_fn、_scan_native_impls、_gen_native_stub。
"""

import os
import re
from ..types import ClassInfo, ParsedMethod
from ..constants import safe_ident


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


def _scan_native_impls(workspace_root: str) -> tuple[dict, dict]:
    """扫描 native_impls/ 目录，识别新格式文件（impl super::TypeName { pub fn ... }）。
    返回:
      new_format_map: {class_binary -> {'methods': set[str], 'main': rel_path, 'ergonomic': rel_path|None}}
      extra_fields:   {class_binary -> [(field_name, rust_type), ...]}
    rel_path 相对于 workspace_root。
    extra_fields 通过 /// @field name: RustType 注释声明，注入到生成的 struct 中。
    """
    import re as _re
    impls_dir = os.path.join(workspace_root, 'native_impls')
    if not os.path.isdir(impls_dir):
        return {}, {}

    new_format_map: dict = {}
    extra_fields: dict = {}

    def _snake_to_class(s: str) -> str:
        return ''.join(w.capitalize() for w in s.split('_'))

    for root_dir, dirs, files in os.walk(impls_dir):
        dirs.sort()
        for fname in sorted(files):
            if not fname.endswith('.rs'):
                continue
            fpath = os.path.join(root_dir, fname)
            rel = os.path.relpath(fpath, workspace_root).replace('\\', '/')
            rel_from_impls = os.path.relpath(fpath, impls_dir).replace('\\', '/')
            parts = rel_from_impls.replace('.rs', '').split('/')
            *pkg, cls_snake = parts
            class_binary = '/'.join(pkg + [_snake_to_class(cls_snake)])

            try:
                content = open(fpath, encoding='utf-8').read()
            except Exception:
                continue

            # 扫描 @field 注解
            for line in content.splitlines():
                stripped = line.strip()
                if stripped.startswith('/// @field ') and ':' in stripped:
                    field_decl = stripped[len('/// @field '):].strip()
                    colon = field_decl.index(':')
                    fn_name = field_decl[:colon].strip()
                    ft = field_decl[colon + 1:].strip()
                    extra_fields.setdefault(class_binary, []).append((fn_name, ft))

            # 扫描 pub fn 名字（确定覆盖了哪些方法）
            method_names = {m.group(1) for m in _re.finditer(r'^\s*pub fn\s+(\w+)', content, _re.MULTILINE)}

            entry = new_format_map.setdefault(class_binary, {'methods': set(), 'main': None})
            entry['methods'].update(method_names)
            entry['main'] = rel

    return new_format_map, extra_fields


def _gen_native_stub(m: ParsedMethod, ci: ClassInfo, rust_name: str | None = None,
                     registry: dict | None = None) -> str:
    """为 native / abstract / stub 方法生成 panic! 存根。"""
    from ..type_map import jvm_to_rust, sig_type, parse_descriptor_params, parse_descriptor_return
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
    label = 'native' if m.is_native else 'stub'
    body = f'panic!("{label}: {ci.name}.{m.name}:{m.descriptor}")'

    return (
        f'pub fn {fn_name}({sig_self}{args_str}) -> {ret_type} {{\n'
        f'    {body}\n'
        f'}}'
    )
