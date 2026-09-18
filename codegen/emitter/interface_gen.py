"""接口实现声明生成：`impl Iface for Class { 擦除签名 }`（类的 java_class! 块内）。

JVM 的运行时类型标识只有擦除后的类 / 接口：`ArrayList$Itr` 的对象无论元素类型是什么，
`invokeinterface Iterator.hasNext` 都经同一张 itable 命中。Rust 侧的等价物：

  - 接口块（class_writer）为每个实例方法输出成员声明，宏据此生成**无类型参数**的
    `Iface__VTable`（擦除签名：提及接口类型变量的位置一律 `Object`）
  - 本模块为每个具体类、对它实现的每个接口（含经超类 / 超接口传递得到的）输出

        impl<E> Iterator for ArrayList_Itr<E> {
            fn hasNext(&self) -> Result<bool>;
            fn next(&self) -> Result<Object>;
        }

    宏据此为 `Class__inner` 实现 `Iface__VTable`（重建 wrapper 后调用同名成员，等价 javac 的
    桥接方法），并让对象的 `ObjectVTable::__interface` 应答该接口

实现方法的定位完全来自定义侧记录（ClassEmission）：本类声明 → 超类链上最近的声明
（同时登记继承成员需求，使 wrapper 拥有该成员）→ 桥接方法指向的真实方法 →
共置 `_impl.rs` 提供的成员。定位不到的方法不输出，`Iface__VTable` 的缺省体
`panic!("stub: ...")` 精确报出缺口。
"""

import re

from .. import inherited_calls
from ..constants import OBJECT_CLASS as _OBJECT_CLASS, RUST_KEYWORDS as _RUST_KEYWORDS
import sys

from ..type_map import (effective_class_type_params, short_cls, substitute_type_params,
                        superinterface_type_args)
from .inherited_gen import (ClassEmission, EmittedMethod, IMPORTS_SLOT, MEMBERS_SLOT,
                            _imports_for, _USE_RE, type_arg_uses)

# 类文本中的插入位（整行，位于 java_class! 块内、impl 块之后）
IMPLS_SLOT = '//@@java_rta:interface-impls@@'

_ACC_ABSTRACT = 0x0400
_ACC_BRIDGE = 0x0040
_IDENT_RE = re.compile(r'\b[A-Za-z_][A-Za-z0-9_]*\b')
_SIG_RE = re.compile(r'^pub fn\s+([A-Za-z_][A-Za-z0-9_]*)\s*\((.*)\)\s*->\s*Result<(.*)>\s*$')


def _split_top_level(text: str) -> list[str]:
    parts: list[str] = []
    depth = 0
    cur: list[str] = []
    for ch in text:
        if ch in '<([':
            depth += 1
        elif ch in '>)]':
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


def _erase(rust_ty: str, type_params: set[str]) -> str:
    """提及接口类型变量的类型位置在运行时是 Object 引用（与宏的 erase_type 同一规则）。"""
    return 'Object' if type_params & set(_IDENT_RE.findall(rust_ty)) else rust_ty


def erased_declaration(method: EmittedMethod, type_params: set[str]) -> 'str | None':
    """接口方法声明 → 擦除签名的 trait 方法声明（`fn m(&self, a: Object) -> Result<Object>`）。"""
    sm = _SIG_RE.match(method.signature)
    if sm is None:
        return None
    params = _split_top_level(sm.group(2))
    if not params or params[0] != '&self':
        return None
    out = ['&self']
    for p in params[1:]:
        pname, _, pty = p.partition(':')
        out.append(f"{pname.strip()}: {_erase(pty.strip(), type_params)}")
    return f"fn {sm.group(1)}({', '.join(out)}) -> Result<{_erase(sm.group(3).strip(), type_params)}>"


def _result_reinstantiated(erased: str, member: 'EmittedMethod | None') -> bool:
    """成员的返回类型与接口擦除声明的返回类型是同一泛型类的不同实例化。"""
    if member is None:
        return False
    em, mm = _SIG_RE.match('pub ' + erased), _SIG_RE.match(member.signature)
    if em is None or mm is None:
        return False
    want, got = em.group(3).strip(), mm.group(3).strip()
    return ('<' in want and '<' in got and want != got
            and want.split('<', 1)[0] == got.split('<', 1)[0])


def _all_interfaces(ci, registry: dict) -> list[str]:
    """类实现的全部接口（自身 + 超类链 + 超接口传递闭包），按发现顺序去重。"""
    found: dict[str, None] = {}
    pending: list[str] = []
    cur = ci
    seen_cls: set[str] = set()
    while cur is not None and cur.name not in seen_cls:
        seen_cls.add(cur.name)
        pending.extend(cur.interfaces or [])
        cur = registry.get(cur.super_class) if cur.super_class else None
    while pending:
        iface = pending.pop(0)
        if iface in found:
            continue
        found[iface] = None
        iface_ci = registry.get(iface)
        if iface_ci is not None:
            pending.extend(iface_ci.interfaces or [])
    return list(found)


def _param_part(descriptor: str) -> str:
    return descriptor[:descriptor.index(')') + 1]


def _bridge_real_descriptor(ci, name: str, descriptor: str, registry: dict) -> 'tuple[str, str] | None':
    """沿超类链找 (name, descriptor 形参) 命中的桥接方法 → (声明类, 真实方法描述符)。"""
    from ..instr.coerce import _resolve_bridge_target
    hit = _resolve_bridge_target(ci, name, descriptor, registry)
    if hit is None:
        # 桥接方法的返回类型可能与接口声明不同（协变）：按形参匹配
        want = _param_part(descriptor)
        cur = ci
        seen: set[str] = set()
        while cur is not None and cur.name not in seen:
            seen.add(cur.name)
            for m in cur.methods:
                if (m.access_flags & _ACC_BRIDGE) and m.name == name and _param_part(m.descriptor) == want:
                    hit = _resolve_bridge_target(cur, name, m.descriptor, registry)
                    break
            if hit is not None:
                break
            cur = registry.get(cur.super_class) if cur.super_class else None
    if hit is None:
        return None
    return hit[0].name, hit[1]


def _locate(recv_bin: str, name: str, param_desc: str, emissions: 'dict[str, ClassEmission]',
            registry: dict) -> 'tuple[str, EmittedMethod] | None':
    """沿 recv 的超类链找最近的方法声明 → (声明类, 方法)。"""
    cur = recv_bin
    seen: set[str] = set()
    while cur and cur != _OBJECT_CLASS and cur in registry and cur not in seen:
        seen.add(cur)
        em = emissions.get(cur)
        if em is not None and not em.handwritten:
            method = em.find(name, param_desc)
            if method is not None:
                return cur, method
        cur = registry[cur].super_class
    return None


def _vtable_use(iface_bin: str, crate_prefix: str) -> str:
    pkg = '::'.join(f'r#{p}' if p in _RUST_KEYWORDS else p for p in iface_bin.split('/')[:-1])
    return f"use {crate_prefix}::{pkg}::{short_cls(iface_bin)}__VTable;"


def resolve_interface_impls(emissions: 'dict[str, ClassEmission]', registry: dict,
                            impl_members: 'dict[str, set[str]] | None' = None) -> None:
    """为每个具体类生成接口实现声明，填充 IMPLS_SLOT；需要的继承成员登记到 inherited_calls。

    必须先于 resolve_inherited_members 调用（后者消费登记并填充 IMPORTS_SLOT，
    本函数的 use 行插在 IMPORTS_SLOT 之前）。
    """
    impl_members = impl_members or {}
    for recv_bin, recv in emissions.items():
        recv_ci = registry.get(recv_bin)
        blocks: list[str] = []
        uses: list[str] = []
        if (recv_ci is not None and not recv.handwritten and IMPLS_SLOT in recv.text
                and not recv_ci.is_interface and not (recv_ci.access_flags & _ACC_ABSTRACT)):
            imported = {short_cls(recv_bin)}
            for ln in recv.text.split('\n'):
                um = _USE_RE.match(ln)
                if um:
                    imported.add(um.group(2))
            own_names = {m.rust_name for m in recv.methods}
            provided = impl_members.get(recv_bin, set())
            recv_params = effective_class_type_params(recv_ci, registry)
            recv_generics = f"<{', '.join(recv_params)}>" if recv_params else ''
            recv_ty = short_cls(recv_bin) + (f"<{', '.join(recv_params)}>" if recv_params else '')
            for iface_bin in _all_interfaces(recv_ci, registry):
                iface = emissions.get(iface_bin)
                iface_ci = registry.get(iface_bin)
                if iface is None or iface_ci is None or iface.handwritten or not iface.methods:
                    continue
                type_params = set(effective_class_type_params(iface_ci, registry))
                decls: list[str] = []
                for im in iface.methods:
                    erased = erased_declaration(im, type_params)
                    if erased is None:
                        continue
                    param_desc = _param_part(im.descriptor)
                    # 本类共置 _impl.rs 提供的成员先于超类声明（它就是本类对该方法的覆盖）
                    own = recv.find(im.name, param_desc)
                    located = None
                    if own is not None:
                        target = own.rust_name
                    elif im.rust_name in provided:
                        target = im.rust_name
                    elif im.name in provided:
                        target = im.name
                    else:
                        hit = _locate(recv_bin, im.name, param_desc, emissions, registry)
                        if hit is None:
                            real = _bridge_real_descriptor(recv_ci, im.name, im.descriptor, registry)
                            if real is not None:
                                param_desc = _param_part(real[1])
                                hit = _locate(recv_bin, im.name, param_desc, emissions, registry)
                        if hit is None:
                            continue
                        owner_bin, method = hit
                        if owner_bin != recv_bin:
                            if method.rust_name in own_names or method.rust_name in provided:
                                continue  # 与本类成员重名：继承成员无法声明
                            inherited_calls.request(recv_bin, im.name, param_desc)
                        target = method.rust_name
                        located = method
                    attr_parts = [f'target = "{target}"'] if target != im.rust_name else []
                    if _result_reinstantiated(erased, own if own is not None else located):
                        attr_parts.append('result = "checkcast"')
                    attr = f"#[java_method({', '.join(attr_parts)})]\n" if attr_parts else ''
                    decls.append(attr + erased + ';')
                    uses.extend(_imports_for(erased, iface, recv, imported))
                if not decls:
                    continue
                vt_name = short_cls(iface_bin) + '__VTable'
                if vt_name not in imported:
                    imported.add(vt_name)
                    uses.append(_vtable_use(iface_bin, recv.crate_prefix))
                body = '\n'.join('    ' + ln for d in decls for ln in d.split('\n'))
                blocks.append(f"impl{recv_generics} {short_cls(iface_bin)} for {recv_ty} {{\n{body}\n}}")

        if blocks:
            text = '\n\n'.join(blocks)
            impl_text = '\n' + '\n'.join((' ' * 4 + ln) if ln else '' for ln in text.split('\n')) + '\n'
        else:
            impl_text = ''
        recv.text = re.sub(r'^[ \t]*' + re.escape(IMPLS_SLOT) + r'\n', lambda _m: impl_text,
                           recv.text, flags=re.M)
        if uses:
            use_text = ''.join(ln + '\n' for ln in uses)
            recv.text = re.sub(r'^([ \t]*' + re.escape(IMPORTS_SLOT) + r'\n)',
                               lambda m: use_text + m.group(1), recv.text, count=1, flags=re.M)


def _superinterface_views(recv_ci, registry: dict) -> 'list[tuple[str, list[str]]]':
    """接口 recv 的全部超接口及其在 recv 视角下的类型实参（广度优先，近者在前）。"""
    out: list[tuple[str, list[str]]] = []
    seen: set[str] = {recv_ci.name}
    queue: list[tuple[object, dict]] = [(recv_ci, {})]
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


def resolve_interface_inherited_members(emissions: 'dict[str, ClassEmission]', registry: dict) -> None:
    """接口接收者的继承成员：`list.forEach(..)`（List 未重声明 Iterable.forEach）→ 在 List 的
    接口块里声明该成员（`inherited_from = "Iterable<E>"`），宏展开为载体上的同名方法
    （向上转型为声明接口的载体后调用）。必须先于 resolve_inherited_members 调用。"""
    for recv_bin, wanted in inherited_calls.requests().items():
        recv = emissions.get(recv_bin)
        recv_ci = registry.get(recv_bin)
        if (recv is None or recv_ci is None or not recv_ci.is_interface or recv.handwritten
                or MEMBERS_SLOT not in recv.text):
            continue
        taken = {m.rust_name for m in recv.methods}
        imported = {short_cls(recv_bin)}
        for ln in recv.text.split('\n'):
            um = _USE_RE.match(ln)
            if um:
                imported.add(um.group(2))
        views = _superinterface_views(recv_ci, registry)
        arg_uses = type_arg_uses(recv_ci, registry, emissions, recv.crate_prefix)
        decls: list[str] = []
        uses: list[str] = []
        for name, param_desc in sorted(wanted):
            if recv.find(name, param_desc) is not None:
                continue
            for owner_bin, owner_args in views:
                owner = emissions.get(owner_bin)
                if owner is None or owner.handwritten:
                    continue
                method = owner.find(name, param_desc)
                if method is None:
                    continue
                if method.rust_name in taken:
                    print(f"[codegen] 继承成员 {recv_bin}.{method.rust_name} 与本接口方法重名，未声明"
                          f"（来自 {owner_bin}.{name}{method.descriptor}）", file=sys.stderr)
                    break
                taken.add(method.rust_name)
                owner_params = effective_class_type_params(registry[owner_bin], registry)
                mapping = {p: (owner_args[i] if i < len(owner_args) else 'Object')
                           for i, p in enumerate(owner_params)}
                signature = substitute_type_params(method.signature, mapping)
                owner_ty = short_cls(owner_bin) + (f"<{', '.join(owner_args)}>" if owner_args else '')
                attr = [f'name = "{method.name}"', f'descriptor = "{method.descriptor}"']
                if method.access:
                    attr.append(f'access = "{method.access}"')
                attr.append(f'inherited_from = "{owner_ty}"')
                decls.append(f"#[java_method({', '.join(attr)})]\n{signature};")
                uses.extend(_imports_for(signature + ' ' + owner_ty, owner, recv, imported, arg_uses))
                owner_short = short_cls(owner_bin)
                if owner_short not in imported:
                    imported.add(owner_short)
                    uses.append(_vtable_use(owner_bin, recv.crate_prefix).replace('__VTable;', ';'))
                break
        if decls:
            body = '\n\n'.join(decls)
            indented = '\n'.join((' ' * 8 + ln) if ln else '' for ln in body.split('\n'))
            member_text = ('\n' + ' ' * 8 + '// ── 继承成员（超接口声明）──────────────────\n' + indented + '\n')
            recv.text = re.sub(r'^[ \t]*' + re.escape(MEMBERS_SLOT) + r'\n', lambda _m: member_text,
                               recv.text, flags=re.M)
        if uses:
            use_text = ''.join(ln + '\n' for ln in uses)
            recv.text = re.sub(r'^([ \t]*' + re.escape(IMPORTS_SLOT) + r'\n)',
                               lambda m: use_text + m.group(1), recv.text, count=1, flags=re.M)
