"""继承成员声明生成：把调用侧登记的「接收者类需要某继承方法」落到定义侧。

Java 子类天然拥有祖先的非私有实例方法；Rust wrapper 之间没有继承关系。
终态做法：接收者类的 java_class! 块里出现一条**无方法体的继承成员声明**

    #[java_method(name = "speak", descriptor = "()I", access = "public",
                  inherited_from = "Animal", vtable_owner = "Animal")]
    pub fn speak(&self) -> Result<i32>;

宏据此在 wrapper 上展开同名转发方法（虚方法经声明它的祖先 VTable 分派，保持多态；
非虚方法向上转型后调用）。调用点因此与 Java 完全一致：`dog.speak()`。

数据流：
  1. 方法体生成期间，调用点向 codegen.inherited_calls 登记需求（按需，不全量）
  2. 每个类生成时把**实际输出的方法声明**记入 ClassEmission（定义侧真源：
     签名、Rust 方法名、virtual_in 都取自已生成文本，转发声明与祖先定义逐字同源，
     重载改名 / 泛型签名 / 接口 default 注入等规则无需在此重复实现）
  3. 全部类生成完后 resolve_inherited_members 沿接收者的超类链找到最近的声明者，
     把祖先签名中的类型变量代入为接收者视角下的实参，填入接收者文本的插入位
"""

import re
import sys
from dataclasses import dataclass, field

from .. import inherited_calls
from ..constants import OBJECT_CLASS as _OBJECT_CLASS, RUST_KEYWORDS as _RUST_KEYWORDS
from ..type_map import (ancestor_type_args, effective_class_type_params,
                        interface_member_local_name,
                        short_cls, substitute_type_params, superinterface_type_args)

# 类文本中的两个插入位（整行），由 resolve_inherited_members 统一替换
IMPORTS_SLOT = '//@@java_rta:inherited-imports@@'
MEMBERS_SLOT = '//@@java_rta:inherited-members@@'

_ATTR_RE = re.compile(r'#\[java_(?:method|native)\(name = "((?:[^"\\]|\\.)*)", descriptor = "((?:[^"\\]|\\.)*)"([^\n]*)')
_ACCESS_RE = re.compile(r'\baccess = "([^"]*)"')
_VIRTUAL_IN_RE = re.compile(r'\bvirtual_in = "([^"]*)"')
_FN_NAME_RE = re.compile(r'^pub fn\s+([A-Za-z_][A-Za-z0-9_]*)')
_USE_RE = re.compile(r'^use\s+(.+)::([A-Za-z_][A-Za-z0-9_]*);\s*$')
_IDENT_RE = re.compile(r'\b[A-Za-z_][A-Za-z0-9_]*\b')


@dataclass
class EmittedMethod:
    """某个类实际输出到 java_class! 块里的一条实例方法声明。"""
    name: str            # Java 方法名
    descriptor: str      # JVM 描述符
    rust_name: str       # 生成的 Rust 方法名（含重载改名）
    signature: str       # `pub fn name(&self, ..) -> Result<..>`（不含方法体、参数不带 mut）
    access: str          # public / protected / private / ''（package）
    virtual_in: str      # 声明该虚方法的 VTable 所属类（Rust 短名）；非虚方法为空


@dataclass
class ClassEmission:
    """一个类的生成结果：文本 + 定义侧方法声明记录。"""
    binary_name: str
    crate_prefix: str            # 本类文件中引用 JDK 类型所用的 crate 前缀（crate / java_runtime）
    path: str = ''
    text: str = ''
    handwritten: bool = False    # 目标文件是手写文件：生成文本不落盘，记录不可信
    methods: list[EmittedMethod] = field(default_factory=list)

    def record_methods(self, method_blocks: list[str]) -> None:
        for block in method_blocks:
            attr = _ATTR_RE.search(block)
            if attr is None:
                continue
            name, descriptor, rest = attr.group(1), attr.group(2), attr.group(3)
            if name.startswith('<') or re.search(r'\bis_static\s*=\s*true', rest):
                continue
            sig_line = next((ln.strip() for ln in block.split('\n')
                             if ln.lstrip().startswith('pub fn ')), None)
            if sig_line is None:
                continue
            fn_name = _FN_NAME_RE.match(sig_line)
            if fn_name is None or '&self' not in sig_line:
                continue
            signature = sig_line.rstrip()
            if signature.endswith(('{', ';')):
                signature = signature[:-1].rstrip()
            signature = re.sub(r'\bmut\s+(?=[A-Za-z_][A-Za-z0-9_]*\s*:)', '', signature)
            access = _ACCESS_RE.search(rest)
            virtual_in = _VIRTUAL_IN_RE.search(rest)
            self.methods.append(EmittedMethod(
                name=name, descriptor=descriptor, rust_name=fn_name.group(1),
                signature=signature,
                access=access.group(1) if access else '',
                virtual_in=virtual_in.group(1) if virtual_in else '',
            ))

    def find(self, name: str, param_descriptor: str) -> 'EmittedMethod | None':
        for m in self.methods:
            if m.name == name and m.descriptor.startswith(param_descriptor):
                return m
        return None


def _rust_type(binary_name: str, args: list[str]) -> str:
    base = short_cls(binary_name)
    return f"{base}<{', '.join(args)}>" if args else base


def _member_declaration(method: EmittedMethod, owner_bin: str, recv_ci, registry: dict) -> str:
    """祖先方法声明 → 接收者类视角下的继承成员声明（两行文本）。"""
    anc_args = dict(ancestor_type_args(recv_ci, registry))
    owner_ci = registry[owner_bin]
    owner_params = effective_class_type_params(owner_ci, registry)
    owner_args = anc_args.get(owner_bin, [])
    # 祖先类型变量 → 接收者视角实参；raw 继承（无实参）按擦除语义取 Object
    mapping = {p: (owner_args[i] if i < len(owner_args) else 'Object')
               for i, p in enumerate(owner_params)}
    signature = substitute_type_params(method.signature, mapping)

    parts = [f'name = "{method.name}"', f'descriptor = "{method.descriptor}"']
    if method.access:
        parts.append(f'access = "{method.access}"')
    parts.append(f'inherited_from = "{_rust_type(owner_bin, owner_args)}"')
    if method.virtual_in:
        vt_bin = next((b for b in anc_args if short_cls(b) == method.virtual_in), owner_bin)
        parts.append(f'vtable_owner = "{_rust_type(vt_bin, anc_args.get(vt_bin, []))}"')
    return f"#[java_method({', '.join(parts)})]\n{signature};"


def _implemented_interface_views(recv_ci, registry: dict) -> 'list[tuple[str, list[str]]]':
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


def _interface_member_declaration(method: EmittedMethod, owner_bin: str, owner_args: list[str],
                                  recv_ci, registry: dict) -> 'tuple[str, str]':
    """接口方法声明 → 类接收者视角下的继承成员声明；返回 (声明文本, 本类视角的 Rust 方法名)。

    抽象类未声明的接口抽象方法（`this.getLong(f)`）：实现位于运行时具体子类，宏把该成员
    展开为经接口载体分派的转发方法。方法名按接收者类层次的重载判定（与调用点同源），
    与接口侧名字不同时用 `target` 指明载体上的方法。"""
    owner_params = effective_class_type_params(registry[owner_bin], registry)
    mapping = {p: (owner_args[i] if i < len(owner_args) else 'Object')
               for i, p in enumerate(owner_params)}
    signature = substitute_type_params(method.signature, mapping)
    local_name = interface_member_local_name(recv_ci, method.name, method.descriptor, registry)
    parts = [f'name = "{method.name}"', f'descriptor = "{method.descriptor}"']
    if method.access:
        parts.append(f'access = "{method.access}"')
    parts.append(f'inherited_from = "{_rust_type(owner_bin, owner_args)}"')
    parts.append('owner_kind = "interface"')
    if local_name != method.rust_name:
        parts.append(f'target = "{method.rust_name}"')
        signature = re.sub(r'^pub fn\s+' + re.escape(method.rust_name) + r'\b',
                           f'pub fn {local_name}', signature)
    return f"#[java_method({', '.join(parts)})]\n{signature};", local_name


def _imports_for(signature: str, owner: ClassEmission, recv: ClassEmission,
                 already: set[str]) -> list[str]:
    """继承成员签名引用的类型：沿用祖先文件里的精确 use（换成接收者所在 crate 的前缀）。"""
    owner_uses = {}
    for ln in owner.text.split('\n'):
        m = _USE_RE.match(ln)
        if m:
            owner_uses.setdefault(m.group(2), ln.strip())
        elif ln.startswith('java_rta_macros::java_class!'):
            break
    out: list[str] = []
    for ident in dict.fromkeys(_IDENT_RE.findall(signature)):
        use_line = owner_uses.get(ident)
        if use_line is None or ident in already:
            continue
        if recv.crate_prefix != owner.crate_prefix and use_line.startswith('use crate::'):
            use_line = f"use {recv.crate_prefix}::" + use_line[len('use crate::'):]
        already.add(ident)
        out.append(use_line)
    return out


def resolve_inherited_members(emissions: 'dict[str, ClassEmission]', registry: dict) -> None:
    """按登记的需求为各接收者类生成继承成员声明，并填充所有类文本的插入位。"""
    members: dict[str, list[str]] = {}
    imports: dict[str, list[str]] = {}

    for recv_bin, wanted in inherited_calls.requests().items():
        recv = emissions.get(recv_bin)
        recv_ci = registry.get(recv_bin)
        if recv is None or recv_ci is None or recv.handwritten or MEMBERS_SLOT not in recv.text:
            continue
        taken = {m.rust_name for m in recv.methods}
        imported = {short_cls(recv_bin)}
        for ln in recv.text.split('\n'):
            um = _USE_RE.match(ln)
            if um:
                imported.add(um.group(2))
        for name, param_desc in sorted(wanted):
            if recv.find(name, param_desc) is not None:
                continue  # 本类已有（自身声明 / 注入的接口 default 方法）
            owner_bin, method = '', None
            cur = recv_ci.super_class
            seen: set[str] = set()
            while cur and cur != _OBJECT_CLASS and cur in registry and cur not in seen:
                seen.add(cur)
                anc = emissions.get(cur)
                if anc is not None and not anc.handwritten:
                    method = anc.find(name, param_desc)
                    if method is not None:
                        owner_bin = cur
                        break
                cur = registry[cur].super_class
            if method is None:
                # 超类链无声明 → 接口方法（抽象类未实现的接口抽象方法 / 未注入本类的 default）
                for iface_bin, iface_args in _implemented_interface_views(recv_ci, registry):
                    iface = emissions.get(iface_bin)
                    if iface is None or iface.handwritten:
                        continue
                    iface_method = iface.find(name, param_desc)
                    if iface_method is None:
                        continue
                    decl, local_name = _interface_member_declaration(
                        iface_method, iface_bin, iface_args, recv_ci, registry)
                    if local_name in taken:
                        break
                    taken.add(local_name)
                    members.setdefault(recv_bin, []).append(decl)
                    owner_ty = _rust_type(iface_bin, iface_args)
                    uses = _imports_for(decl.split('\n', 1)[1] + ' ' + owner_ty, iface, recv, imported)
                    iface_short = short_cls(iface_bin)
                    if iface_short not in imported:
                        imported.add(iface_short)
                        pkg = '::'.join(f'r#{seg}' if seg in _RUST_KEYWORDS else seg
                                        for seg in iface_bin.split('/')[:-1])
                        uses.append(f"use {recv.crate_prefix}::{pkg}::{iface_short};")
                    imports.setdefault(recv_bin, []).extend(uses)
                    break
                continue
            if method.rust_name in taken:
                print(f"[codegen] 继承成员 {recv_bin}.{method.rust_name} 与本类方法重名，未声明"
                      f"（来自 {owner_bin}.{name}{method.descriptor}）", file=sys.stderr)
                continue
            taken.add(method.rust_name)
            decl = _member_declaration(method, owner_bin, recv_ci, registry)
            members.setdefault(recv_bin, []).append(decl)
            imports.setdefault(recv_bin, []).extend(
                _imports_for(decl.split('\n', 1)[1], emissions[owner_bin], recv, imported))

    for bin_name, em in emissions.items():
        decls = members.get(bin_name, [])
        if decls:
            body = '\n\n'.join(decls)
            indented = '\n'.join((' ' * 8 + ln) if ln else '' for ln in body.split('\n'))
            member_text = ('\n' + ' ' * 8 + '// ── 继承成员（祖先声明，本类未覆盖）──────────────────\n'
                           + indented + '\n')
        else:
            member_text = ''
        use_text = ''.join(ln + '\n' for ln in imports.get(bin_name, []))
        em.text = re.sub(r'^[ \t]*' + re.escape(MEMBERS_SLOT) + r'\n', lambda _m: member_text,
                         em.text, flags=re.M)
        em.text = re.sub(r'^[ \t]*' + re.escape(IMPORTS_SLOT) + r'\n', lambda _m: use_text,
                         em.text, flags=re.M)
