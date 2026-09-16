"""
方法生成相关：_parse_synthetic_fn、_scan_impl_files、_gen_native_stub。
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


def _scan_impl_files(workspace_root: str, registry: dict | None = None) -> tuple[dict, set]:
    """扫描 java_runtime/src/**/*_impl.rs 共置手写文件，提取已实现的方法名。
    codegen 根据返回的 new_format_map 跳过对应方法的 stub 生成。
    返回:
      new_format_map: {class_binary -> {'methods': set[str]}}
      (空集占位，保持调用签名兼容)
    """
    import re as _re
    jdk_src = os.path.join(workspace_root, 'java_runtime', 'src')
    if not os.path.isdir(jdk_src):
        return {}, set()

    new_format_map: dict = {}

    # 预先构建：snake_case 路径 → class binary name（用于 $ 嵌套类名，无需依赖已生成的 .rs 文件）
    # 例：java/util/hash_map_tree_node → java/util/HashMap$TreeNode
    def _camel_to_snake(name: str) -> str:
        s = _re.sub(r'(?<=[a-z0-9])(?=[A-Z])', '_', name)
        s = _re.sub(r'(?<=[A-Z])(?=[A-Z][a-z])', '_', s)
        return s.lower()

    _snake_to_binary: dict[str, str] = {}
    if registry:
        for binary_name in registry:
            pkg_parts = binary_name.split('/')
            *pkg, cls = pkg_parts
            # 嵌套类按 $ 拆分后各段分别转 snake_case，再用 _ 连接
            # HashMap$TreeNode → hash_map + tree_node → hash_map_tree_node
            snake_cls = '_'.join(_camel_to_snake(p) for p in cls.split('$'))
            for sv in {snake_cls, snake_cls.replace('__', '_')}:
                key = '/'.join(pkg + [sv])
                _snake_to_binary.setdefault(key, binary_name)

    def _snake_to_class(s: str) -> str:
        return ''.join(w.capitalize() for w in s.split('_'))

    for root_dir, dirs, files in os.walk(jdk_src):
        dirs.sort()
        for fname in sorted(files):
            # K-4: 处理 *_impl.rs 和 *_ext.rs 手写共置文件
            if not (fname.endswith('_impl.rs') or fname.endswith('_ext.rs')):
                continue

            fpath = os.path.join(root_dir, fname)
            rel_from_src = os.path.relpath(fpath, jdk_src).replace('\\', '/')
            stem = rel_from_src.replace('.rs', '')  # e.g. java/lang/system_impl

            # 去掉 _impl / _ext 后缀还原为对应类的 binary name
            if fname.endswith('_impl.rs'):
                base_stem = stem[:-5]   # 去掉 _impl (5 chars)
                gen_fname = fname[:-8] + '.rs'  # hash_map_impl.rs -> hash_map.rs
            else:
                base_stem = stem[:-4]   # 去掉 _ext  (4 chars)
                gen_fname = fname[:-7] + '.rs'  # hash_map_ext.rs -> hash_map.rs
            parts = base_stem.split('/')
            *pkg, cls_snake = parts
            class_binary = '/'.join(pkg + [_snake_to_class(cls_snake)])

            # 优先从 registry 中查找真实 binary_name（处理 $ 嵌套类，不依赖已生成的 .rs 文件）
            if base_stem in _snake_to_binary:
                class_binary = _snake_to_binary[base_stem]
            else:
                # 其次从同目录生成的 .rs 文件中读取真实 binary_name
                gen_rs_path = os.path.join(root_dir, gen_fname)
                if os.path.exists(gen_rs_path):
                    try:
                        gen_head = open(gen_rs_path, encoding='utf-8').read(2000)
                        bm = _re.search(r'binary_name\s*=\s*"([^"]+)"', gen_head)
                        if bm:
                            class_binary = bm.group(1)
                    except Exception:
                        pass

            try:
                content = open(fpath, encoding='utf-8').read()
            except Exception:
                continue

            # 自动生成的类文件碰巧以 _impl.rs 结尾时（如 Collectors$CollectorImpl），跳过。
            # 真正的手写共置文件不包含 java_rta_macros::java_class 宏标注。
            if '#[java_rta_macros::java_class(' in content:
                continue

            # 扫描 pub fn 名字（确定已手写哪些方法，codegen 跳过对应 stub）
            # 兼容 #[attr] pub fn name(...) 同行写法
            method_names = {m.group(1) for m in _re.finditer(r'\bpub fn\s+(\w+)\s*[(<]', content)}
            if method_names:
                entry = new_format_map.setdefault(class_binary, {'methods': set()})
                entry['methods'].update(method_names)

    return new_format_map, set()


def _gen_native_stub(m: ParsedMethod, ci: ClassInfo, rust_name: str | None = None,
                     registry: dict | None = None,
                     class_type_params: list | None = None) -> str:
    """为 native / abstract / stub 方法生成 panic! 存根。"""
    from ..type_map import (
        jvm_to_rust, sig_type, parse_descriptor_params, parse_descriptor_return,
        parse_class_type_params, parse_method_param_types,
    )
    params = parse_descriptor_params(m.descriptor)
    ret    = parse_descriptor_return(m.descriptor)
    rust_ret = jvm_to_rust(ret, registry)

    _ctparams: list[str] = class_type_params or (
        parse_class_type_params(ci.generic_signature) if ci.generic_signature else []
    )

    # 从 generic_signature 提取更具体的参数类型（与 gen_method_body 对齐）
    sig_param_types: list[str] = []
    if m.generic_signature and _ctparams:
        sig_param_types, _ = parse_method_param_types(m.generic_signature, _ctparams)
        if len(sig_param_types) != len(params):
            sig_param_types = []

    def _sig_param_valid(sp: str) -> bool:
        if sp in _ctparams:
            return True
        _builtin = frozenset({
            'Object', 'String', 'i32', 'i64', 'f32', 'f64', 'bool', 'u16',
            'i8', 'i16', 'u32', 'u64', '()', 'Rc', 'Vec', 'RefCell', 'usize', 'u8',
        })
        _reg_shorts = (
            {k.rsplit('/', 1)[-1].replace('$', '_') for k in registry}
            if registry else set()
        )
        # PERMANENT 手写 stub 的短名（glob import 引入，不在 registry 中）
        _permanent_shorts = frozenset({
            'Iterator', 'BiConsumer', 'BinaryOperator', 'Supplier', 'Function',
        })
        import re as _re
        for name in _re.findall(r'[A-Za-z_][A-Za-z0-9_]*', sp):
            if name in _builtin or name in _ctparams or name in _reg_shorts or name in _permanent_shorts:
                continue
            return False
        return True

    # PERMANENT functional interface 类型在参数位置强制使用 Object（Java 类型擦除语义）
    _perm_iface_names = frozenset({'Supplier', 'BiConsumer', 'BinaryOperator', 'Function', 'Iterator'})
    import re as _re2
    def _is_perm_iface_param(sp: str) -> bool:
        m = _re2.match(r'^(\w+)(?:<|$)', sp)
        return bool(m and m.group(1) in _perm_iface_names)

    def _param_rust_type(i: int, desc_p: str) -> str:
        if sig_param_types and i < len(sig_param_types):
            sp = sig_param_types[i]
            if _sig_param_valid(sp) and not _is_perm_iface_param(sp):
                return sp
        return jvm_to_rust(desc_p, registry)

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
    def param_type_fn(i: int, p: str) -> str:
        pt = _param_rust_type(i, p)
        return sig_type(pt) if m.is_static else pt

    args_str = ', '.join(
        f'{name}: {param_type_fn(i, p)}' for i, (name, p) in enumerate(zip(arg_names, params))
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
    # toString/hashCode 非 native 存根：生成 ObjectVTable 可安全调用的默认值
    # （java_class 宏会在 vtable_impl 中调用 Self::toString / Self::hashCode，
    #  不能 panic，否则打印任何该类对象时都会崩溃）
    if not m.is_native and not m.is_abstract:
        if m.name == 'toString' and m.descriptor == '()Ljava/lang/String;':
            body = 'Ok(String::from(Self::BINARY_NAME))'
        elif m.name == 'hashCode' and m.descriptor == '()I':
            body = 'Ok(0)'
        else:
            body = f'panic!("stub: {ci.name}.{m.name}:{m.descriptor}")'
    else:
        label = 'native' if m.is_native else 'stub'
        body = f'panic!("{label}: {ci.name}.{m.name}:{m.descriptor}")'

    # main(String[] args) 与 gen_method_body 保持一致：不生成参数
    if m.is_static and m.name == 'main' and m.descriptor == '([Ljava/lang/String;)V':
        return (
            f'pub fn main() -> Result<()> {{\n'
            f'    {body}\n'
            f'}}'
        )

    return (
        f'pub fn {fn_name}({sig_self}{args_str}) -> {ret_type} {{\n'
        f'    {body}\n'
        f'}}'
    )
