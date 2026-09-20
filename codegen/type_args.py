"""
层次类型实参解析：父类 / 祖先链 / 接口闭包的类型实参、外围作用域实参与接口实现视图。

自 type_map.py 拆出。

依赖方向（禁止回环）：本模块 → sig_parse → type_map；sig_types → 本模块。
"""

from __future__ import annotations
import re

from .constants import OBJECT_CLASS as _OBJECT_CLASS
from .sig_parse import (_CLASSNAME_MAP, _extract_method_tparam_bounds, _parse_one_type,
                        _parse_type_args, substitute_signature_type_vars)
from .type_map import (_PRIMITIVE_MAP, _registry_short_index, _skip_field_type_sig,
                       effective_class_type_params, enclosing_method_info,
                       outer_instance_class, parse_class_type_params, short_cls)


def enclosing_scope_type_args(ci, registry, scope_type_params) -> list[str]:
    """在构造点（其外围作用域内）实例化 ci 的有效类型参数。

    构造点可见的同名类型变量（scope_type_params：当前 impl 的形参）原样传递；
    其余是外围方法的方法级类型变量 —— 实例方法内它们在 Rust 侧按上界 / Object
    擦除（见 parse_method_param_types），实例化取同一形态。
    """
    params = effective_class_type_params(ci, registry)
    em = enclosing_method_info(ci, registry)
    bounds = (_extract_method_tparam_bounds(em.generic_signature, registry)
              if em is not None and em.generic_signature else {})
    scope = set(scope_type_params or ())
    return [p if p in scope else bounds.get(p, 'Object') for p in params]


def class_type_param_bounds(ci, registry=None) -> 'dict[str, tuple[str, str]]':
    """类级类型变量 → (上界 Rust 类型, 上界类 binary name)。

    Java `class C<E extends B<E>>` 的类上界在 Rust 侧等价于 `E: Into<B<E>>`
    （宏生成的 From<Child> for Ancestor / From<Object> for B 使该约束对子类实参与
    擦除实参 Object 均成立）。只保留 registry 中存在的具体类上界：
    接口上界是 Object 别名（无需转换），根类上界无信息量。
    形参来源与 effective_class_type_params 一致（内部类继承外部类形参及其上界）。
    """
    if not registry:
        return {}
    own = parse_class_type_params(ci.generic_signature) if ci.generic_signature else []
    result: dict[str, tuple[str, str]] = {}
    outer_ci = registry.get(outer_instance_class(ci) or '')
    if outer_ci is not None and outer_ci is not ci:
        result = {name: b for name, b in class_type_param_bounds(outer_ci, registry).items()
                  if name not in own}
    if own:
        binaries: dict[str, str] = {}
        bounds = _extract_method_tparam_bounds(ci.generic_signature, registry,
                                               type_params=own, bound_binaries=binaries)
        for name, rust_t in bounds.items():
            b_ci = registry.get(binaries.get(name, ''))
            if name in own and b_ci is not None and not b_ci.is_interface:
                result[name] = (rust_t, b_ci.name)
    return result


def _type_arg_is_resolvable(rust_ty: str, class_type_params: list[str], registry) -> bool:
    """类型实参中出现的每个类型名都必须可解析（类型参数 / 内建 / registry 中的类且元数正确）。"""
    short_names = _registry_short_index(registry)
    mapped = set(_CLASSNAME_MAP.values()) | set(_PRIMITIVE_MAP.values()) | {'JArray'}
    for m in re.finditer(r'([A-Za-z_]\w*)\s*(<)?', rust_ty):
        name, has_args = m.group(1), bool(m.group(2))
        if name in class_type_params or name in mapped:
            continue
        ref_ci = short_names.get(name)
        if ref_ci is None:
            return False
        # 裸用泛型类（raw type）在 Rust 中缺实参 → 不可用
        if bool(effective_class_type_params(ref_ci, registry)) != has_args:
            return False
    return True


def superclass_type_args(ci, registry) -> list[str]:
    """直接父类的 Rust 类型实参，以 ci 的有效类型参数表达。

    实参取自 ci 类级 Signature 的 SuperclassSignature（而非按位置把子类形参套给父类）：
      `<P_IN,P_OUT> extends AbstractPipeline<P_IN,P_OUT,Stream<P_OUT>>` → ['P_IN','P_OUT','Object']
      `extends RecursiveTask<BigInteger>`                               → ['BigInteger']
      `extends AbstractList<E>.Itr`（父类形参继承自其外部类）           → ['E']
    无法解析的实参（raw type、通配符、未翻译的类）取 Object（类型擦除的真实多态边界）。
    """
    if not registry or not ci.super_class or ci.super_class not in registry:
        return []
    parent_ci = registry[ci.super_class]
    parent_params = effective_class_type_params(parent_ci, registry)
    if not parent_params:
        return []
    own_params = effective_class_type_params(ci, registry)
    args: list[str] = []
    if ci.generic_signature:
        # SuperclassSignature 按一般类型解析（`Outer<A>.Inner<B>` 的外层实参并入内部类的有效实参）
        sig = ci.generic_signature
        i = 0
        if sig.startswith('<'):
            depth = 0
            while i < len(sig):
                if sig[i] == '<':
                    depth += 1
                elif sig[i] == '>':
                    depth -= 1
                    if depth == 0:
                        i += 1
                        break
                i += 1
        if i < len(sig) and sig[i] == 'L':
            parent_ty, _ = _parse_one_type(sig, i, own_params, registry)
            args = split_rust_type_args(parent_ty)
    if len(args) != len(parent_params):
        args = ['Object'] * len(parent_params)
    return [a if _type_arg_is_resolvable(a, own_params, registry) else 'Object' for a in args]


def superinterface_type_args(ci, registry) -> 'dict[str, list[str]]':
    """直接超接口的 Rust 类型实参（以 ci 的有效类型参数表达）：{接口 binary name → [实参]}。

    取自类级 Signature 的 SuperinterfaceSignature；raw 继承 / 无 Signature / 不可解析的实参
    按擦除语义取 Object。非泛型接口 → []。
    """
    out: dict[str, list[str]] = {}
    if not registry:
        return out
    own_params = effective_class_type_params(ci, registry)
    parsed: dict[str, list[str]] = {}
    sig = ci.generic_signature or ''
    if sig:
        i = 0
        if sig.startswith('<'):
            depth = 0
            while i < len(sig):
                if sig[i] == '<':
                    depth += 1
                elif sig[i] == '>':
                    depth -= 1
                    if depth == 0:
                        i += 1
                        break
                i += 1
        i = _skip_field_type_sig(sig, i)  # SuperclassSignature
        while i < len(sig) and sig[i] == 'L':
            end = _skip_field_type_sig(sig, i)
            j = i + 1
            while j < end and sig[j] not in ('<', ';', '.'):
                j += 1
            name = sig[i + 1:j]
            args: list[str] = []
            if j < end and sig[j] == '<':
                args, _ = _parse_type_args(sig, j, own_params, registry)
            parsed[name] = args
            i = end
    for iface in (ci.interfaces or []):
        iface_ci = registry.get(iface)
        if iface_ci is None:
            continue
        params = effective_class_type_params(iface_ci, registry)
        args = parsed.get(iface, [])
        if len(args) != len(params):
            args = ['Object'] * len(params)
        out[iface] = [a if _type_arg_is_resolvable(a, own_params, registry) else 'Object' for a in args]
    return out


def outer_instance_rust_type(outer_bin: str, decl_params: list, registry) -> str:
    """外部实例在内部类视角下的 Rust 类型：外部类 + 其有效形参中被内部类继承的同名
    类型变量（如 ArrayList$Itr → ArrayList<E>）；未继承的形参取 Object。
    外部类非泛型 / 不在 registry → ''。"""
    outer_ci = registry.get(outer_bin) if registry else None
    if outer_ci is None:
        return ''
    outer_tp = effective_class_type_params(outer_ci, registry)
    if not outer_tp:
        return ''
    return rust_type_with_args(short_cls(outer_bin),
                               [p if p in decl_params else 'Object' for p in outer_tp])


def outer_ref_field_type(f, decl_params: list, registry) -> str:
    """内部类外部引用字段（this$N）的 Rust 类型（见 outer_instance_rust_type）。
    非 this$N 字段或外部类非泛型 → ''。
    struct 字段定义（class_writer）与 getfield/putfield 的字段类型恢复共用。"""
    if not (re.match(r'^this\$\d+$', f.name) and decl_params and registry):
        return ''
    fm = re.match(r'L([^;]+);', f.descriptor)
    if not fm:
        return ''
    return outer_instance_rust_type(fm.group(1), decl_params, registry)


def rust_type_with_args(short_name: str, args: list[str]) -> str:
    """`Name` + 实参表 → `Name<A, B>`；无实参时为裸名。"""
    return f"{short_name}<{', '.join(args)}>" if args else short_name


def substitute_type_params(rust_ty: str, mapping: dict) -> str:
    """把 Rust 类型字符串中的类型参数名按 mapping 同步替换。"""
    if not mapping:
        return rust_ty
    return re.sub(r'\b[A-Za-z_]\w*\b', lambda m: mapping.get(m.group(0), m.group(0)), rust_ty)


def ancestor_type_args(ci, registry, self_args: 'list[str] | None' = None) -> 'list[tuple[str, list[str]]]':
    """沿超类链解析每个祖先的 Rust 类型实参：[(ancestor_binary, [args]), ...]，直接父类在前。

    self_args 给出 ci 自身形参的实参（调用点静态类型，如 `X<Object>` → ['Object']）；
    缺省时以 ci 的形参自身表达（类定义内部视角）。不含 java.lang.Object，链在 registry 之外截断。
    """
    chain: list[tuple[str, list[str]]] = []
    if not registry:
        return chain
    own_params = effective_class_type_params(ci, registry)
    mapping: dict = {}
    if self_args is not None and len(self_args) == len(own_params):
        mapping = dict(zip(own_params, self_args))
    elif self_args is not None:
        mapping = {p: 'Object' for p in own_params}
    cur = ci
    seen: set[str] = set()
    while (cur.super_class and cur.super_class != _OBJECT_CLASS
           and cur.super_class in registry and cur.super_class not in seen):
        seen.add(cur.super_class)
        args = [substitute_type_params(a, mapping) for a in superclass_type_args(cur, registry)]
        parent_ci = registry[cur.super_class]
        chain.append((cur.super_class, args))
        mapping = dict(zip(effective_class_type_params(parent_ci, registry), args))
        cur = parent_ci
    return chain


def split_rust_type_args(rust_ty: str) -> list[str]:
    """`Name<A, B<C, D>>` → ['A', 'B<C, D>']（仅拆顶层逗号）；无实参 → []。"""
    lt = rust_ty.find('<')
    if lt < 0 or not rust_ty.rstrip().endswith('>'):
        return []
    inner = rust_ty[lt + 1:rust_ty.rstrip().rfind('>')]
    parts: list[str] = []
    depth = 0
    cur: list[str] = []
    for ch in inner:
        if ch == '<':
            depth += 1
        elif ch == '>':
            depth -= 1
        if ch == ',' and depth == 0:
            parts.append(''.join(cur).strip())
            cur = []
        else:
            cur.append(ch)
    tail = ''.join(cur).strip()
    if tail:
        parts.append(tail)
    return parts


def ancestor_vtable_args_by_short(ci, rust_ty: str, registry) -> 'dict[str, str]':
    """静态类型为 rust_ty（ci 的实例化，如 `X<Object>`）的接收者，其各祖先 VTable 的
    类型实参串：{祖先 Rust 短名 → '<A, B>' 或 ''}。供 UFCS `<dyn Anc__VTable<..>>::m(..)` 使用。"""
    return {short_cls(anc_bin): (f"<{', '.join(args)}>" if args else '')
            for anc_bin, args in ancestor_type_args(ci, registry, split_rust_type_args(rust_ty))}


def implemented_interface_views(recv_ci, registry: dict) -> 'list[tuple[str, list[str]]]':
    """类 recv 实现的全部接口（自身 + 祖先类 + 超接口闭包）及其在 recv 视角下的类型实参，
    广度优先、近者在前。"""
    anc_args = dict(ancestor_type_args(recv_ci, registry))
    queue: list[tuple[object, dict]] = [(recv_ci, {})]
    cur = recv_ci.super_class
    seen_cls: set[str] = {recv_ci.name}
    while cur and cur in registry and cur not in seen_cls:
        seen_cls.add(cur)
        cur_ci = registry[cur]
        cur_params = effective_class_type_params(cur_ci, registry)
        cur_args = anc_args.get(cur, [])
        queue.append((cur_ci, {p: (cur_args[i] if i < len(cur_args) else 'Object')
                               for i, p in enumerate(cur_params)}))
        cur = cur_ci.super_class
    out: list[tuple[str, list[str]]] = []
    seen: set[str] = set()
    while queue:
        cur_ci, mapping = queue.pop(0)
        for sup_bin, sup_args in superinterface_type_args(cur_ci, registry).items():
            if sup_bin in seen:
                continue
            seen.add(sup_bin)
            args = [substitute_type_params(a, mapping) for a in sup_args] if mapping else list(sup_args)
            out.append((sup_bin, args))
            sup_ci = registry[sup_bin]
            queue.append((sup_ci, dict(zip(effective_class_type_params(sup_ci, registry), args))))
    return out


def _supertype_signature_args(ci, registry) -> 'list[tuple[str, list[str]]]':
    """类签名里的直接超类型（超类 + 超接口）→ (binary name, 各段类型实参的签名，外层在前)。"""
    sig = ci.generic_signature or ''
    i = 0
    if sig.startswith('<'):
        depth = 0
        while i < len(sig):
            if sig[i] == '<':
                depth += 1
            elif sig[i] == '>':
                depth -= 1
                if depth == 0:
                    i += 1
                    break
            i += 1
    out: list[tuple[str, list[str]]] = []
    while i < len(sig) and sig[i] == 'L':
        end = _skip_field_type_sig(sig, i)
        name: list[str] = []
        args: list[str] = []
        j = i + 1
        while j < end - 1:
            ch = sig[j]
            if ch == '<':
                j += 1
                while sig[j] != '>':
                    k = _skip_field_type_sig(sig, j)
                    arg = sig[j:k]
                    # 通配符实参：上界通配取其上界，其余按根类
                    args.append(arg[1:] if arg[0] == '+' else
                                f'L{_OBJECT_CLASS};' if arg[0] in '-*' else arg)
                    j = k
                j += 1
            elif ch == '.':
                name.append('$')
                j += 1
            else:
                name.append(ch)
                j += 1
        out.append((''.join(name), args))
        i = end
    return out


def interface_signature_views(ci, registry) -> 'dict[str, dict[str, str]]':
    """类 ci 实现的全部接口（自身 + 祖先类 + 超接口闭包）→ 接口类型形参在 ci 视角下的类型签名。

    接口 default 方法体展开到实现类时，其泛型签名里的接口类型变量据此换成实现类作用域内的类型
    （`Node<T>.asArray: (IntFunction<T[]>)T[]` 在 `ConcNode<E> implements Node<E>` 里是 `E[]`）。
    原始类型（签名未给实参）的接口不在结果里，其类型变量按擦除处理。"""
    views: dict[str, dict[str, str]] = {}
    queue: list[tuple[object, dict]] = [(ci, {})]
    seen: set[str] = {ci.name}
    while queue:
        cur_ci, mapping = queue.pop(0)
        declared = dict(_supertype_signature_args(cur_ci, registry))
        supers = ([cur_ci.super_class] if cur_ci.super_class else []) + list(cur_ci.interfaces or [])
        for sup_bin in supers:
            sup_ci = registry.get(sup_bin)
            if sup_ci is None or sup_bin in seen:
                continue
            seen.add(sup_bin)
            params = effective_class_type_params(sup_ci, registry)
            args = declared.get(sup_bin, [])
            sup_map: dict[str, str] = {}
            if params and len(args) == len(params) and mapping is not None:
                sup_map = {p_: substitute_signature_type_vars(a, mapping)
                           for p_, a in zip(params, args)}
            if sup_ci.is_interface and sup_map:
                views[sup_bin] = sup_map
            # 祖先以原始类型出现：其类型变量不可代入（None 标记沿链传播）
            queue.append((sup_ci, sup_map if (sup_map or not params) else None))
    return views
