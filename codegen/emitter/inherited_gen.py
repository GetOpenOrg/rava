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
from ..sig_types import interface_member_local_name
from ..type_args import (ancestor_type_args, implemented_interface_views,
                         substitute_type_params, superinterface_type_args)
from ..type_map import effective_class_type_params, short_cls
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
    if start < 0:
        return [], ''
    # 形参列表的右括号：从 '(' 起做深度计数回到 0 的位置（返回类型的括号不计入）
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


def _bridge_override_member(name: str, param_desc: str, recv: ClassEmission,
                            recv_ci, registry: dict, emissions: dict) -> 'tuple[str, str] | None':
    """接收者类（或其超类链）的 synthetic bridge → bridge 转发成员声明。

    javac 为泛型形参 / 协变返回生成的 ACC_BRIDGE 方法（如
    `EnumMap.put(Object,Object)` → `put(Enum,Object)`）不生成 Rust 实现，但它是对
    该签名（name, param_desc）的**本类声明**：vtable / itable 分派经它转发到真实方法。
    不识别它会把分派落到超类链的普通声明（`Map.put` → `AbstractMap.put` 的
    UnsupportedOperationException 体）。成员体在 self 上调用 bridge 的真实方法
    （虚分派，子类覆盖生效），并以 `virtual_in` 填声明类的 vtable 槽位（桥接签名
    即擦除签名，与 vtable 的 Object 化参数天然一致）。

    返回 (声明文本, 导入扫描用签名文本, 真实方法的补充需求)：真实方法声明在祖先时，
    bridge 体调用的 `self.<real>` 需要 wrapper 一并补上其继承成员，补充需求以
    (方法名, 参数描述符) 给出（本类声明则为 None）。无 bridge / 真实方法不可解析
    → None（回落到普通继承路径）。
    """
    from ..instr.member_owner import _resolve_bridge_target
    from ..type_map import jvm_to_rust, parse_descriptor_params, parse_descriptor_return

    # 超类链（含自身）中找 (name, param_desc) 精确命中的 synthetic bridge（JVM 方法解析顺序）
    bridge = None
    cur = recv_ci
    seen: set[str] = set()
    while cur is not None and cur.name not in seen:
        seen.add(cur.name)
        bridge = next((m for m in cur.methods
                       if m.is_synthetic and m.name == name
                       and m.descriptor.startswith(param_desc)), None)
        if bridge is not None:
            break
        cur = registry.get(cur.super_class) if cur.super_class else None
    if bridge is None:
        return None
    target = _resolve_bridge_target(recv_ci, name, bridge.descriptor, registry)
    if target is None:
        return None
    real_owner_ci, real_desc = target
    real_owner_bin = real_owner_ci.name
    real_param = real_desc.split(')', 1)[0] + ')'
    # 只处理形参擦除桥接（形参描述符不同：`(Object,Object)` → `(Enum,Object)`），且
    # 返回类型与真实方法一致。协变返回桥接（形参相同、仅返回收窄）由普通继承路径
    # 处理；返回类型不同 + 形参擦除的组合（ClassSpecializer.newSpeciesData）其桥接
    # 返回是声明类的擦除类型变量位置，槽位签名需按 owner vtable 擦除渲染，本路径
    # 的描述符渲染不覆盖，回落普通继承。
    if real_param == param_desc or real_desc.split(')', 1)[1] != bridge.descriptor.split(')', 1)[1]:
        return None

    # 真实方法的签名与 Rust 名（接收者视角）：本类声明直接取；祖先声明代入接收者实参
    real_m = recv.find(name, real_param)
    real_want = None
    if real_m is not None:
        real_sig = real_m.signature
        real_rust = real_m.rust_name
    else:
        owner_em = emissions.get(real_owner_bin)
        if owner_em is None or owner_em.handwritten:
            return None
        found = owner_em.find(name, real_param)
        if found is None:
            return None
        anc_args = dict(ancestor_type_args(recv_ci, registry))
        owner_params = effective_class_type_params(registry[real_owner_bin], registry)
        owner_args = anc_args.get(real_owner_bin, [])
        mapping = {p: (owner_args[i] if i < len(owner_args) else 'Object')
                   for i, p in enumerate(owner_params)}
        real_sig = substitute_type_params(found.signature, mapping)
        real_rust = found.rust_name
        real_want = (name, real_param)

    # 声明类的 vtable 槽位：超类链上有该签名的声明 → VirtualOverride 填槽；成员名取
    # 声明类的 Rust 方法名（与该类 vtable trait 的槽位名同源），virtual_in 为声明类短名。
    # 纯接口桥接（超类链无声明）无槽位：接口经载体分派直接落到本 wrapper 方法，
    # 名字按接收者视角的重载判定。
    vt_short = ''
    member_name = ''
    cur = recv_ci.super_class
    seen = set()
    while cur and cur != _OBJECT_CLASS and cur in registry and cur not in seen:
        seen.add(cur)
        anc_em = emissions.get(cur)
        if anc_em is not None and not anc_em.handwritten:
            found = anc_em.find(name, param_desc)
            if found is not None:
                vt_short = short_cls(cur)
                member_name = found.rust_name
                break
        cur = registry[cur].super_class
    if not member_name:
        member_name = interface_member_local_name(recv_ci, name, bridge.descriptor, registry)
    if member_name == real_rust:
        return None  # 转发到自身（同名）：交回普通继承路径

    # bridge 成员签名：按 bridge 描述符渲染（桥接位置即擦除静态类型）
    param_tys = [jvm_to_rust(p, registry) for p in parse_descriptor_params(bridge.descriptor)]
    ret_desc = parse_descriptor_return(bridge.descriptor)
    ret_ty = '()' if ret_desc == 'V' else jvm_to_rust(ret_desc, registry)
    params = ', '.join(f'arg{i}: {t}' for i, t in enumerate(param_tys))
    signature = f"pub fn {member_name}(&self, {params}) -> Result<{ret_ty}>"

    # 转发体（翻译体词汇：`this` 接收者 + 对真实方法的调用，宏按 NeedsWrapper 路径
    # 落到 wrapper 的 `__impl_<m>`，`this.real(..)` 被重写为 vtable 分派——等价 Java
    # 桥接体的 invokevirtual）：实参从 bridge 静态类型经 From 还原到真实方法形参类型
    # （等价桥接体的 checkcast），返回值经 Into 装箱回 bridge 静态类型（等价擦除返回）
    real_param_tys, real_ret = _sig_param_types(real_sig)
    call_args = []
    for i, (bridge_ty, real_ty) in enumerate(zip(param_tys, real_param_tys)):
        arg = f'arg{i}'
        call_args.append(arg if bridge_ty == real_ty
                         else f'<{real_ty} as ::std::convert::From<{bridge_ty}>>::from({arg})')
    call = f"this.{real_rust}({', '.join(call_args)})"
    real_inner = _result_inner(real_ret)
    if ret_desc == 'V':
        body = f"let this = self; {call}?; Ok(())"
    elif real_inner == ret_ty:
        body = f"let this = self; Ok({call}?)"
    else:
        body = (f"let this = self; "
                f"Ok(::std::convert::Into::<{ret_ty}>::into({call}?))")

    # 声明类的 vtable 槽位已在上方与成员名一并解析（同一声明的名字与槽位必须同源）

    parts = [f'name = "{name}"', f'descriptor = "{bridge.descriptor}"']
    if bridge.access_flags & 0x0001:
        parts.append('access = "public"')
    elif bridge.access_flags & 0x0004:
        parts.append('access = "protected"')
    if vt_short:
        parts.append(f'virtual_in = "{vt_short}"')
    decl = f"#[java_method({', '.join(parts)})]\n{signature} {{ {body} }}"
    return decl, decl, real_want


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


def resolve_inherited_members(emissions: 'dict[str, ClassEmission]', registry: dict,
                              impl_methods: 'dict[str, set[str]] | None' = None) -> None:
    """按登记的需求为各接收者类生成继承成员声明，并填充所有类文本的插入位。

    impl_methods：共置 `_impl.rs` / `_ext.rs` 已提供的方法名（{binary name → 名字集}）。
    手写 companion 的方法是 wrapper 的固有成员（不在 emissions 记录里），登记的
    需求命中它时不再生成继承成员（E0592 重复定义）。
    """
    members: dict[str, list[str]] = {}
    imports: dict[str, list[str]] = {}
    impl_methods = impl_methods or {}

    for recv_bin, wanted in inherited_calls.requests().items():
        recv = emissions.get(recv_bin)
        recv_ci = registry.get(recv_bin)
        if recv is None or recv_ci is None or recv.handwritten or MEMBERS_SLOT not in recv.text:
            continue
        provided = impl_methods.get(recv_bin, set())
        taken = {m.rust_name for m in recv.methods} | provided
        imported = {short_cls(recv_bin)}
        for ln in recv.text.split('\n'):
            um = _USE_RE.match(ln)
            if um:
                imported.add(um.group(2))
        arg_uses = type_arg_uses(recv_ci, registry, emissions, recv.crate_prefix)

        def _super_decl(mname: str, pdesc: str):
            """超类链上 (mname, pdesc) 的最近声明者；返回 (owner_bin, EmittedMethod)。"""
            cur = recv_ci.super_class
            seen_sup: set[str] = set()
            while cur and cur != _OBJECT_CLASS and cur in registry and cur not in seen_sup:
                seen_sup.add(cur)
                anc_em = emissions.get(cur)
                if anc_em is not None and not anc_em.handwritten:
                    found = anc_em.find(mname, pdesc)
                    if found is not None:
                        return cur, found
                cur = registry[cur].super_class
            return '', None

        for name, param_desc in sorted(wanted):
            if recv.find(name, param_desc) is not None:
                continue  # 本类已有（自身声明 / 注入的接口 default 方法）

            # 本类（或超类链）的 synthetic bridge：bridge 是对该签名的本类声明，其转发
            # 语义优先于超类链的普通声明（否则 Map.put 会落到 AbstractMap.put 的 UOE 体）
            bridge_hit = _bridge_override_member(name, param_desc, recv, recv_ci, registry, emissions)
            if bridge_hit is not None:
                decl, sig_text, real_want = bridge_hit
                fn_match = _FN_NAME_RE.match(sig_text.split('\n', 1)[1].strip())
                if fn_match is None or fn_match.group(1) in taken:
                    print(f"[codegen] bridge 成员 {recv_bin}.{name}{param_desc} 名字缺失或与本类"
                          f"方法重名，退回普通继承", file=sys.stderr)
                fn_match = _FN_NAME_RE.match(sig_text.split('\n', 1)[1].strip())
                if fn_match is None or fn_match.group(1) in taken:
                    print(f"[codegen] bridge 成员 {recv_bin}.{name}{param_desc} 名字缺失或与本类"
                          f"方法重名，退回普通继承", file=sys.stderr)
                else:
                    taken.add(fn_match.group(1))
                    members.setdefault(recv_bin, []).append(decl)
                    imports.setdefault(recv_bin, []).extend(
                        _imports_for(sig_text, recv, recv, imported, arg_uses))
                    # 真实方法声明在祖先 → 一并补其继承成员（bridge 体调用 self.<real>）
                    if real_want is not None and recv.find(*real_want) is None:
                        real_owner_bin, real_method = _super_decl(*real_want)
                        if real_method is not None and real_method.rust_name not in taken:
                            taken.add(real_method.rust_name)
                            real_decl = _member_declaration(real_method, real_owner_bin, recv_ci, registry)
                            members.setdefault(recv_bin, []).append(real_decl)
                            imports.setdefault(recv_bin, []).extend(
                                _imports_for(real_decl.split('\n', 1)[1], emissions[real_owner_bin],
                                             recv, imported, arg_uses))
                            if not real_method.handwritten:
                                base_short = f"{short_cls(real_owner_bin)}__{real_method.rust_name}_base"
                                if base_short not in imported:
                                    imported.add(base_short)
                                    base_path = (class_use_path(real_owner_bin, recv.crate_prefix, emissions)
                                                 + f"__{real_method.rust_name}_base")
                                    imports.setdefault(recv_bin, []).append(f"use {base_path};")
                    continue

            owner_bin, method = _super_decl(name, param_desc)
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
