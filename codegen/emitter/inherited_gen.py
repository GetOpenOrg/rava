"""继承成员声明生成：把调用侧登记的「接收者类需要某继承方法」落到定义侧。

Java 子类天然拥有祖先的非私有实例方法；Rust wrapper 之间没有继承关系。
终态做法：接收者类的 java_class! 块里出现一条**带转发体的继承成员声明**

    #[java_method(name = "speak", descriptor = "()I", access = "public",
                  inherited_from = "Animal", vtable_owner = "Animal")]
    pub fn speak(&self) -> Result<i32> { Animal__speak_base::<Self>(self) }

宏据此做两件事（见 block/mod.rs「继承成员填槽」）：
  - wrapper 上展开同名转发方法（虚方法经声明它的祖先 VTable 分派，保持多态）
  - 本类对 vtable_owner 的 vtable impl 用该体填槽——否则 vtable_owner 声明为
    abstract（实现位于中间祖先）时，槽位落到声明类的 trait default（stub panic），
    虚分派命中空洞（S-16）。转发体与 super 调用同源：精确命中 owner 的实现，
    this 以本类 __inner 视图传入（supertrait 链满足 base 函数的 `__BT` 约束）。

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
                        implemented_interface_views,
                        interface_member_local_name,
                        short_cls, substitute_type_params, superinterface_type_args)
from .attrs import to_snake

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
    handwritten: bool = False  # body = "handwritten"：无块，体在共置 _impl.rs 的 __impl_<m>


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
            if signature.endswith((';', '{')):
                signature = signature[:-1].rstrip()
            signature = re.sub(r'\bmut\s+(?=[A-Za-z_][A-Za-z0-9_]*\s*:)', '', signature)
            access = _ACCESS_RE.search(rest)
            virtual_in = _VIRTUAL_IN_RE.search(rest)
            self.methods.append(EmittedMethod(
                name=name, descriptor=descriptor, rust_name=fn_name.group(1),
                signature=signature,
                access=access.group(1) if access else '',
                virtual_in=virtual_in.group(1) if virtual_in else '',
                handwritten=bool(re.search(r'\bbody\s*=\s*"handwritten"', rest)),
            ))

    def find(self, name: str, param_descriptor: str) -> 'EmittedMethod | None':
        for m in self.methods:
            if m.name == name and m.descriptor.startswith(param_descriptor):
                return m
        return None


def _rust_type(binary_name: str, args: list[str]) -> str:
    base = short_cls(binary_name)
    return f"{base}<{', '.join(args)}>" if args else base


def _param_idents(signature: str) -> list[str]:
    """`pub fn name(&self, a: T, b: U) -> R` → ['a', 'b']。

    顶层逗号分割（忽略泛型 / 括号内的逗号）；跳过 self 接收者；去掉 mut 绑定。"""
    start = signature.find('(')
    if start < 0:
        return []
    depth, end = 0, -1
    for i in range(start, len(signature)):
        if signature[i] in '(<[':
            depth += 1
        elif signature[i] in ')>]':
            depth -= 1
            if depth == 0:
                end = i
                break
    if end < 0:
        end = len(signature)
    parts: list[str] = []
    depth, cur = 0, ''
    for ch in signature[start + 1:end]:
        if ch in '(<[':
            depth += 1
        elif ch in ')>]':
            depth -= 1
        if ch == ',' and depth == 0:
            parts.append(cur)
            cur = ''
        else:
            cur += ch
    if cur.strip():
        parts.append(cur)
    out: list[str] = []
    for p in parts:
        p = p.strip()
        if not p or p == 'self' or p == 'mut self' or p.startswith('&'):
            continue
        name = p.split(':', 1)[0].strip()
        if name.startswith('mut '):
            name = name[4:].strip()
        if name and _IDENT_RE.fullmatch(name):
            out.append(name)
    return out


def _forward_body(method: EmittedMethod, owner_bin: str, owner_args: list[str]) -> str:
    """继承成员的转发体：精确执行 owner 祖先的实现（super 调用同源，不经 vtable 分派）。

    运行上下文是「本类对 vtable_owner 的 vtable impl」（Self = 本类 __inner）：
      - 常规：调用 owner 宏展开生成的 `Owner__m_base` 自由函数；this 以 __inner 视图
        传入，经 supertrait 链满足 base 函数的 `__BT: Owner__VTable` 约束。turbofish
        传 owner 视角实参 + Self。
      - 手写（body = "handwritten"，宏不生成 base 函数）：经 `__as_Owner` 钩子（owner
        自身 vtable trait 的 self-hook）重建 owner wrapper 视图后执行其共置 `__impl_<m>`。"""
    args = _param_idents(method.signature)
    owner_short = short_cls(owner_bin)
    if not method.handwritten:
        turbo = ', '.join([*owner_args, 'Self'])
        call = f"self, {', '.join(args)}" if args else 'self'
        return f"{owner_short}__{method.rust_name}_base::<{turbo}>({call})"
    hook_trait = f"{owner_short}__VTable" + (f"<{', '.join(owner_args)}>" if owner_args else '')
    return (f"<Self as {hook_trait}>::__as_{owner_short}(self)"
            f".__impl_{method.rust_name}({', '.join(args)})")


def _sig_param_types(signature: str) -> 'tuple[list[str], str]':
    """`pub fn name(&self, a: T, b: U) -> R` → (['T', 'U'], 'R')。
    顶层逗号 / 箭头按括号与尖括号深度切分（与 _param_idents 同一深度规则）。"""
    start = signature.find('(')
    end = signature.rfind(')')
    if start < 0 or end < 0:
        return [], ''
    params_str = signature[start + 1:end]
    parts, depth, cur = [], 0, ''
    for ch in params_str:
        if ch in '(<[':
            depth += 1
        elif ch in ')>]':
            depth -= 1
        if ch == ',' and depth == 0:
            parts.append(cur)
            cur = ''
        else:
            cur += ch
    if cur.strip():
        parts.append(cur)
    types = []
    for p in parts:
        p = p.strip()
        if not p or p.startswith('&') or p in ('self', 'mut self'):
            continue
        types.append(p.split(':', 1)[1].strip() if ':' in p else p)
    ret = ''
    arrow = signature.rfind('->')
    if arrow >= 0:
        ret = signature[arrow + 2:].strip()
    return types, ret


def _result_inner(ty: str) -> 'str | None':
    """`Result<T>` → 'T'；其余 None。"""
    m = re.match(r'^Result<(.*)>$', ty)
    return m.group(1) if m else None


def _owner_erasure_entries(owner_sig: str, substituted_sig: str, owner_params: list[str]) -> list[str]:
    """owner（声明类）签名中提及自身类型形参的位置 → 接收者视角下代入后的类型串。

    声明方的 vtable 方法签名在这些位置已是 Object（A-1 擦除按声明类判定）；接收者的
    槽位条目 / wrapper 转发需按名单同步擦除（宏按 token 全等匹配，含嵌套形态整体）。"""
    if not owner_params:
        return []
    owner_types, owner_ret = _sig_param_types(owner_sig)
    subst_types, subst_ret = _sig_param_types(substituted_sig)
    def mentions(ty: str) -> bool:
        return any(re.search(r'\b' + re.escape(p) + r'\b', ty) for p in owner_params)
    out: list[str] = []
    for i, ty in enumerate(owner_types):
        if mentions(ty) and i < len(subst_types):
            out.append(subst_types[i])
    if owner_ret:
        inner = _result_inner(owner_ret)
        if inner is not None:
            if mentions(inner):
                s_inner = _result_inner(subst_ret) if subst_ret else None
                if s_inner is not None:
                    out.append(s_inner)
        elif mentions(owner_ret):
            out.append(subst_ret)
    return [t for t in dict.fromkeys(out) if t and t != 'Object']


def _member_declaration(method: EmittedMethod, owner_bin: str, recv_ci, registry: dict) -> str:
    """祖先方法声明 → 接收者类视角下的继承成员声明（声明 + 转发体）。"""
    anc_args = dict(ancestor_type_args(recv_ci, registry))
    owner_ci = registry[owner_bin]
    owner_params = effective_class_type_params(owner_ci, registry)
    owner_args = anc_args.get(owner_bin, [])
    # 祖先类型变量 → 接收者视角实参；raw 继承（无实参）按擦除语义取 Object
    mapping = {p: (owner_args[i] if i < len(owner_args) else 'Object')
               for i, p in enumerate(owner_params)}
    signature = substitute_type_params(method.signature, mapping)

    vt_bin, vt_args = owner_bin, owner_args
    if method.virtual_in:
        vt_bin = next((b for b in anc_args if short_cls(b) == method.virtual_in), owner_bin)
        vt_args = anc_args.get(vt_bin, [])
    parts = [f'name = "{method.name}"', f'descriptor = "{method.descriptor}"']
    if method.access:
        parts.append(f'access = "{method.access}"')
    parts.append(f'inherited_from = "{_rust_type(owner_bin, owner_args)}"')
    if method.virtual_in:
        parts.append(f'vtable_owner = "{_rust_type(vt_bin, vt_args)}"')
    erasure = _owner_erasure_entries(method.signature, signature, owner_params)
    if erasure:
        parts.append(f'vtable_erasure = "{";".join(erasure)}"')
    body = _forward_body(method, owner_bin, owner_args)
    return f"#[java_method({', '.join(parts)})]\n{signature} {{ {body} }}"


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


_SIG_CLASS_RE = re.compile(r'L([A-Za-z_$][\w$]*(?:/[A-Za-z_$][\w$]*)+)[<;]')


def class_use_path(binary_name: str, crate_prefix: str,
                   emissions: 'dict[str, ClassEmission] | None' = None) -> str:
    """类在 Rust 中的完整引用路径（不含 `use` 关键字与末尾 `;`）。

    两种 crate 布局决定了路径写到哪一层：

      - JDK 类（java/lang/String，落在 java_runtime crate）：包目录的 mod.rs 有
        `pub use <mod>::*;` 再导出，故为 `<crate 前缀>::java::lang::String`
      - 用户类（默认包，落在 user crate）：main.rs 只做 `mod` 声明、无再导出，
        必须写到模块层 `crate::test_interfaces_drawable::TestInterfaces_Drawable`

    crate_prefix 是「接收者文件引用 JDK 类型所用的前缀」；用户类一律用 `crate`
    （用户类只被同 crate 的用户类引用）。判断归属看目标类的 emission：
    JDK 类的 crate_prefix 为 'crate'（它自身就在 java_runtime 里），用户类为 'java_runtime'。
    """
    short = short_cls(binary_name)
    em = (emissions or {}).get(binary_name)
    pkg = '::'.join(f'r#{p}' if p in _RUST_KEYWORDS else p
                    for p in binary_name.split('/')[:-1])
    if not pkg or (em is not None and em.crate_prefix != 'crate'):
        # 无包名（用户类）：文件即模块，路径必须写到模块层
        mod = to_snake(binary_name)
        return f"crate::{mod}::{short}"
    return f"{crate_prefix}::{pkg}::{short}"


def type_arg_uses(recv_ci, registry: dict, emissions: 'dict[str, ClassEmission]',
                  crate_prefix: str) -> dict[str, str]:
    """接收者视角下类型实参引用的类：Rust 短名 → use 行。

    祖先签名中的类型变量被代入为接收者超类型签名里的实参（`CountedCompleter<Void>` 的 Void），
    这些类型不出现在祖先文件的 use 里；从接收者及其全部超类型的类级泛型签名中解析
    binary name，只收已生成的类。"""
    uses: dict[str, str] = {}
    queue = [recv_ci]
    seen: set[str] = set()
    while queue:
        cur = queue.pop(0)
        if cur is None or cur.name in seen:
            continue
        seen.add(cur.name)
        for bin_name in _SIG_CLASS_RE.findall(getattr(cur, 'generic_signature', '') or ''):
            if bin_name not in emissions:
                continue
            short = short_cls(bin_name)
            uses.setdefault(short, f"use {class_use_path(bin_name, crate_prefix, emissions)};")
        for sup in [cur.super_class] + list(cur.interfaces or []):
            if sup and sup in registry:
                queue.append(registry[sup])
    return uses


def _imports_for(signature: str, owner: ClassEmission, recv: ClassEmission,
                 already: set[str], arg_uses: 'dict[str, str] | None' = None) -> list[str]:
    """继承成员签名引用的类型：沿用祖先文件里的精确 use（换成接收者所在 crate 的前缀）；
    代入的类型实参不在祖先文件中，按 arg_uses（见 type_arg_uses）解析。"""
    owner_uses = {}
    for ln in owner.text.split('\n'):
        m = _USE_RE.match(ln)
        if m:
            owner_uses.setdefault(m.group(2), ln.strip())
        elif ln.startswith('java_rta_macros::java_class!'):
            break
    # 祖先文件不 use 自身：签名引用声明类自身（`fork() -> ForkJoinTask<V>`）时按其包路径导入
    if '/' in owner.binary_name:
        owner_short = short_cls(owner.binary_name)
        owner_pkg = '::'.join(f'r#{p}' if p in _RUST_KEYWORDS else p
                              for p in owner.binary_name.split('/')[:-1])
        owner_uses.setdefault(owner_short, f"use {owner.crate_prefix}::{owner_pkg}::{owner_short};")
    out: list[str] = []
    for ident in dict.fromkeys(_IDENT_RE.findall(signature)):
        if ident in already:
            continue
        use_line = owner_uses.get(ident)
        if use_line is None:
            if not arg_uses or ident not in arg_uses:
                continue
            already.add(ident)
            out.append(arg_uses[ident])
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
        arg_uses = type_arg_uses(recv_ci, registry, emissions, recv.crate_prefix)
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
                for iface_bin, iface_args in implemented_interface_views(recv_ci, registry):
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
                        uses.append(f"use {class_use_path(iface_bin, recv.crate_prefix, emissions)};")
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
                _imports_for(decl.split('\n', 1)[1], emissions[owner_bin], recv, imported, arg_uses))
            # 转发体调用的 owner __base 自由函数（宏在 owner 模块展开，pub 可达）：
            # 经与类型相同的再导出路径导入（与祖先 __VTable trait 导入同构）
            if not method.handwritten:
                base_short = f"{short_cls(owner_bin)}__{method.rust_name}_base"
                if base_short not in imported:
                    imported.add(base_short)
                    base_path = (class_use_path(owner_bin, recv.crate_prefix, emissions)
                                 + f"__{method.rust_name}_base")
                    imports.setdefault(recv_bin, []).append(f"use {base_path};")

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
