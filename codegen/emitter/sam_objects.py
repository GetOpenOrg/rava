"""A-5 函数式接口合成对象（lambda 对象化）。

lambda 不再以 `Rc<dyn Fn>` 闭包装箱（`Object::from_any` + 调用点
`downcast_ref` 回落），而是与 JVM `LambdaMetafactory` 产物同构的**合成对象**：
每个被 invokedynamic 站点用作 samtype 的函数式接口 `I` 在其翻译文件内生成

    #[derive(Clone)]
    pub struct I__Lambda(pub Rc<dyn Fn(擦除形参) -> Result<擦除返回>>);

- 实现 `ObjectVTable`：`is_instance_of`（I + 超接口闭包 + 根类 / Serializable，
  与 LambdaMetafactory 的实现集一致）、`__interface`（对 I 及每个有契约方法的
  超接口应答——default 方法由此可在 lambda 上调用、接口查询有效）、
  `__class_name`（报 I 的 binary name）；
- 实现 `I__VTable`（及各超接口 `J__VTable`）：SAM 条目直调闭包 `(self.0)(..)`，
  default 条目经载体固有方法 `J::__default_<m>` 执行默认体（不重入分派，
  JVM lambda 类继承接口 default 的语义）；
- `TryFrom<Object>`：解箱保留 ClassCastException 语义（装箱侧由 blanket
  `From<T: ObjectVTable> for Object` 承担）。

证据驱动的接口集（不铺全集）：预扫描本轮全部字节码的 invokedynamic 站点，
只有实际作为 samtype 出现、且可合成的接口才生成（规则一/二：清单来自
字节码，Python 侧零 JDK 类名）。

共置伴生形态：合成对象追加在接口自身的翻译文件尾部。理由：合成对象的形状
（vtable 签名 / default 体有无）与接口发射同源，同文件保证原子生成、同 mod
树零挂载成本；引用本文件类型免限定，引用超接口用全限定路径，不动 import_gen。
"""

import os
import re

from ..constants import OBJECT_CLASS as _OBJECT_CLASS, SERIALIZABLE_CLASS as _SERIALIZABLE
from ..type_map import (jvm_to_rust, short_cls, effective_class_type_params,
                        parse_descriptor_params, parse_descriptor_return)
from .attrs import to_snake

_ACC_PRIVATE = 0x0002
_ACC_ABSTRACT = 0x0400

_IDENT_RE = re.compile(r'\b[A-Za-z_][A-Za-z0-9_]*\b')


class SamSpec:
    """一个可合成函数式接口的预扫描结论。

    sam_name/sam_desc：闭包上唯一未被 default 覆盖的抽象方法（可在任一闭包
    接口上声明）；erased_params/erased_ret：SAM 擦除 Rust 签名（站点闭包与
    合成对象字段的公共类型，描述符驱动、两侧恒等）；closure：[I] + 传递
    超接口（registry 内、广度优先发现序）。
    """

    def __init__(self, iface_bin, sam_name, sam_desc, erased_params, erased_ret, closure):
        self.iface_bin = iface_bin
        self.sam_name = sam_name
        self.sam_desc = sam_desc
        self.erased_params = erased_params
        self.erased_ret = erased_ret
        self.closure = closure


class _PathCtx:
    """class_use_path 所需的最小 emission 视图（crate_prefix）。"""

    def __init__(self, crate_prefix: str):
        self.crate_prefix = crate_prefix


# 本轮全部可合成接口（{iface_bin: SamSpec}）；站点（sim/dynamic.py）与合成
# 收尾（project_writer）共用。与 LAMBDA_NAME_LEDGER 同生命周期（每轮 reset）。
SAM_LEDGER: dict[str, SamSpec] = {}

# 站点 crate 前缀判定所需的用户类名集合 + 路径解析用的 emission 视图
_user_class_names: frozenset[str] = frozenset()
_path_ctx: dict[str, _PathCtx] = {}


def reset() -> None:
    SAM_LEDGER.clear()
    global _user_class_names, _path_ctx
    _user_class_names = frozenset()
    _path_ctx = {}


def _param_part(descriptor: str) -> str:
    idx = descriptor.find(')')
    return descriptor[:idx + 1] if idx >= 0 else descriptor


def _iface_closure(iface_bin: str, registry: dict) -> list[str]:
    """{I} + 传递超接口（registry 内，广度优先发现序去重）。"""
    out: list[str] = []
    seen: set[str] = set()
    queue = [iface_bin]
    while queue:
        name = queue.pop(0)
        if name in seen:
            continue
        seen.add(name)
        out.append(name)
        ci = registry.get(name)
        if ci is not None:
            queue.extend(ci.interfaces or [])
    return out


def _contract_methods(jci, registry: dict) -> list:
    """接口自身声明的契约实例方法（进 `J__VTable` 的方法集）。

    与 class_writer._emit_method_blocks 的接口过滤完全同源：跳过 static /
    synthetic（含桥接与 G-10 私有 lambda 体）/ private / 根类方法重声明
    （`Comparator.equals`——经根 vtable 分派，不是接口契约）。
    """
    from ..instr.member_owner import _root_virtual_methods
    root_keys = _root_virtual_methods()
    out = []
    for m in jci.methods:
        if m.is_static or m.is_synthetic or m.name.startswith('<'):
            continue
        if m.access_flags & _ACC_PRIVATE:
            continue
        if (m.name, _param_part(m.descriptor)) in root_keys:
            continue
        out.append(m)
    return out


def _effective_view_methods(jci, registry: dict) -> list:
    """SAM 判定视角的接口实例方法：非 static / 非 private / 非 <init> / 非根类
    方法重声明（`Comparator.equals`——JLS §9.8：与 Object 公开方法 override-equivalent
    的声明不计入函数式判定）。

    与 `_contract_methods`（vtable 视角）的差别：**含 synthetic 桥接**——桥接带
    转发体（`Sink.OfInt` 的 accept(Object)→accept(Integer)），在 JVM 方法解析里
    是对该签名的有效覆盖；排除它会把桥接覆盖的抽象方法误判为未覆盖（JLS §9.4.4）。"""
    from ..instr.member_owner import _root_virtual_methods
    root_keys = _root_virtual_methods()
    out = []
    for m in jci.methods:
        if m.is_static or m.name.startswith('<'):
            continue
        if m.access_flags & _ACC_PRIVATE:
            continue
        if (m.name, _param_part(m.descriptor)) in root_keys:
            continue
        out.append(m)
    return out


def _functional_sam(iface_bin: str, registry: dict) -> 'SamSpec | None':
    """接口闭包上的 SAM（JVM 函数式接口判定，JLS §9.8 / §9.4.4）。

    每个 (name, 参数描述符) 的**有效声明** = 声明者中的极大元（无更派生接口
    再次声明）；极大声明者任一为抽象 → 有效抽象（子接口抽象重声明会覆盖祖先
    default，`Sink.OfInt` 的 accept(I)V 即此形态）。有效抽象恰 1 个 → SAM。
    多于 1 个、或 SAM 描述符无法映射 Rust 类型 → None（站点回落闭包装箱）。"""
    ci = registry.get(iface_bin)
    if ci is None or not ci.is_interface:
        return None
    closure = _iface_closure(iface_bin, registry)
    # key → {声明接口: 是否有体}；同接口同 key 多声明（协变/桥接并立）有体优先
    decls: dict[tuple[str, str], dict[str, bool]] = {}
    rep: dict[tuple[str, str], dict[str, object]] = {}
    for jbin in closure:
        jci = registry.get(jbin)
        if jci is None or not jci.is_interface:
            continue
        for m in _effective_view_methods(jci, registry):
            key = (m.name, _param_part(m.descriptor))
            concrete = not (m.access_flags & _ACC_ABSTRACT)
            per = decls.setdefault(key, {})
            if per.get(jbin, False) is False and not concrete:
                rep.setdefault(key, {})[jbin] = m
            per[jbin] = per.get(jbin, False) or concrete
    anc: dict[str, set[str]] = {j: set(_iface_closure(j, registry))
                                for j in closure if registry.get(j)}
    uncovered: dict[tuple[str, str], object] = {}
    for key, per in decls.items():
        maximal = [j for j in per
                   if not any(k != j and j in anc.get(k, set()) for k in per)]
        for j in maximal:
            if not per[j]:
                m = rep.get(key, {}).get(j)
                if m is not None:
                    uncovered[key] = m
                break
    if len(uncovered) != 1:
        return None
    (name, _pdesc), m = next(iter(uncovered.items()))
    desc = m.descriptor
    try:
        params = [jvm_to_rust(p, registry) for p in parse_descriptor_params(desc)]
        rd = parse_descriptor_return(desc)
        ret = '()' if rd == 'V' else jvm_to_rust(rd, registry)
    except Exception:
        return None
    return SamSpec(iface_bin, name, desc, params, ret, closure)


def _jdk_iface_handwritten(iface_bin: str) -> bool:
    """接口的目标文件是否被 runtime/ 真源手写覆盖（与 project_writer._is_handwritten
    同一判据：runtime/java_runtime/src 下同相对路径存在即手写——scratch 里可能
    残留上轮生成文件，不能作判据）。类名与子包目录同名时的 `_t` 后缀两种布局
    都探测。"""
    from ..constants import RUNTIME_JAVA_RUNTIME
    parts = iface_bin.split('/')
    *pkg_parts, class_name = parts
    mod_name = to_snake(class_name)
    parent = os.path.join(RUNTIME_JAVA_RUNTIME, 'src', *pkg_parts)
    for cand in (mod_name, mod_name + '_t'):
        if os.path.isfile(os.path.join(parent, cand + '.rs')):
            return True
    return False


def prescan(registry: dict, jdk_class_infos: list, user_class_infos: list,
            full_impl_classes: set) -> None:
    """预扫描全部字节码的 invokedynamic 站点，得出可合成接口集。

    必须在类文本生成（站点发射）之前调用：站点需要即时判定走合成对象装箱
    还是回落闭包装箱，而合成可行性（接口是否发射 / 是否手写全覆盖 / 是否
    函数式）此刻即须定案。"""
    global _user_class_names, _path_ctx

    user_names = {ci.name for ci in user_class_infos}
    _user_class_names = frozenset(user_names)
    _path_ctx = {name: _PathCtx('java_runtime' if name in user_names else 'crate')
                 for name in registry}

    candidates: set[str] = set()
    for ci in list(user_class_infos) + list(jdk_class_infos):
        for m in ci.methods:
            for ins in (m.instrs or []):
                if ins.opcode != 'invokedynamic' or not ins.comment:
                    continue
                # comment 格式：`InvokeDynamic <sam>:<dynDesc> [impl:...] [samtype:<SAM 方法描述符>]`
                # 函数式接口 = dynDesc 的返回描述符（`()Lx/Y;` → `x/Y`）
                toks = ins.comment.split(' ')
                if len(toks) < 2 or ':' not in toks[1]:
                    continue
                try:
                    ret_desc = parse_descriptor_return(toks[1].split(':', 1)[1])
                except Exception:
                    continue
                if ret_desc.startswith('L') and ret_desc.endswith(';'):
                    candidates.add(ret_desc[1:-1])

    for iface_bin in sorted(candidates):
        if iface_bin in full_impl_classes:
            continue
        if iface_bin not in user_names and _jdk_iface_handwritten(iface_bin):
            continue
        spec = _functional_sam(iface_bin, registry)
        if spec is not None:
            SAM_LEDGER[iface_bin] = spec


def site_ctor_path(iface_bin: str, current_class: str) -> 'str | None':
    """invokedynamic 站点的合成对象构造路径（全限定，免 import）。

    当前文件在 user crate → JDK 接口用 `java_runtime::` 前缀，在 java_runtime
    crate → `crate::`；用户接口（emission 视角非本 crate）一律模块层路径
    `crate::<mod>::<Short>`。返回 None 表示该接口不可合成（调用方回落闭包装箱）。"""
    spec = SAM_LEDGER.get(iface_bin)
    if spec is None:
        return None
    from .inherited_gen import class_use_path
    crate_prefix = 'java_runtime' if current_class in _user_class_names else 'crate'
    return class_use_path(iface_bin, crate_prefix, _path_ctx) + '__Lambda::new'


def record_site(iface_bin: str, sam_desc: str, current_class: str) -> None:
    """站点侧一致性断言（G-10 同款 fail-loud）：站点 samtype 描述符与预扫描
    SAM 描述符恒等——LambdaMetafactory 的 samtype 即 SAM 方法描述符，发散
    意味着合成对象字段类型与闭包签名错位（E0308 静默面），生成期直接抛错。"""
    spec = SAM_LEDGER.get(iface_bin)
    if spec is not None and spec.sam_desc != sam_desc:
        raise RuntimeError(
            f'[sam-objects] samtype 描述符发散: {iface_bin} 站点 {sam_desc} '
            f'vs 预扫描 SAM {spec.sam_desc}（{current_class}）')


# ── 合成对象文本生成（全部类文本生成后、落盘前）─────────────────────────

def _quote_path(jbin: str, em, emissions: dict) -> str:
    """接口 J 在合成对象所在文件（接口 I 的文件）中的全限定类型路径。"""
    from .inherited_gen import class_use_path
    return class_use_path(jbin, em.crate_prefix, emissions)


def _entry_sig_parts(em_method, jci, registry: dict) -> 'tuple[str, list[str]] | None':
    """发射记录的方法 → (vtable 擦除条目签名, 形参名列表)。

    擦除规则与宏 vtable trait（interface_gen.erased_declaration）同源：提及
    接口类型变量的类型位置整体 Object 化。签名形如
    `m(&self, a: Object) -> Result<Object>`（不含 fn 前缀，impl 条目书写用）。"""
    from .interface_gen import erased_declaration, _split_top_level
    type_params = set(effective_class_type_params(jci, registry))
    erased = erased_declaration(em_method, type_params)
    if erased is None:
        return None
    head = erased[len('fn '):]
    rest = erased[erased.index('(') + 1:erased.rindex(')')]
    params = [p.split(':')[0].strip() for p in _split_top_level(rest)]
    return head, params[1:]  # 去 &self


def _declared_sig_parts(em_method) -> 'tuple[list[str], str] | None':
    """发射记录的载体声明签名 → (形参类型列表（去 self）, 返回类型文本)。"""
    sig = em_method.signature
    if not sig.startswith('pub fn ') or '(' not in sig:
        return None
    open_p = sig.index('(')
    close_p = sig.rindex(')')
    inner = sig[open_p + 1:close_p]
    parts: list[str] = []
    depth = 0
    cur: list[str] = []
    for ch in inner:
        if ch in '<([':
            depth += 1
        elif ch in '>)]':
            depth -= 1
        if ch == ',' and depth == 0:
            parts.append(''.join(cur).strip())
            cur = []
        else:
            cur.append(ch)
    if ''.join(cur).strip():
        parts.append(''.join(cur).strip())
    param_tys = [p.partition(':')[2].strip() for p in parts[1:]]
    ret = sig[close_p + 1:].strip().removeprefix('->').strip()
    return param_tys, ret


def _subst_type_vars(ty: str, tparams: set[str]) -> str:
    """声明类型文本中的接口类型变量代入 Object（`Comparator<T>` → `Comparator<Object>`）。"""
    return re.sub(r'\b(' + '|'.join(sorted(tparams)) + r')\b', 'Object', ty)


def _mentions(ty: str, tparams: set[str]) -> bool:
    return bool(tparams & set(_IDENT_RE.findall(ty)))


def _is_carrier_type(rust_ty: str, registry: dict) -> bool:
    """类型串是否为已铺设载体化的接口载体形态（判定单一来源 jvm_type）。"""
    if not registry or not rust_ty:
        return False
    from ..jvm_type import carrier_type_for_ident
    return carrier_type_for_ident(rust_ty, registry) == rust_ty


def _default_entry_body(em_m, jci, registry: dict, j_erased_ty: str, args: list[str]) -> 'str | None':
    """default 条目体：`<J<Object,..> as From<Object>>::from(..).__default_m(..)`。

    擦除条目形参恒为 Object（提及类型变量的位置整体 Object 化），载体默认体
    在擦除实例化上的形参是「类型变量代入 Object 后」的形态——嵌套提及
    （`Comparator<T>`）需经 `From<Object>` 还原；返回值提及类型变量时经
    `Into<Object>` 装箱（与宏 expand_interface_impl 的边界转换同规则）。
    A-4 批次 3+：形参/返回是已铺设载体（`List<Object>` 等）时同桥接——形参经
    载体 From<Object> 非受检包装（接口视图按运行时类成立），返回解包 __ref。"""
    parts = _declared_sig_parts(em_m)
    if parts is None:
        return None
    param_tys, ret_ty = parts
    tparams = set(effective_class_type_params(jci, registry))
    if len(param_tys) != len(args):
        return None
    call_args: list[str] = []
    for a, ty in zip(args, param_tys):
        if _mentions(ty, tparams):
            call_args.append(f'<{_subst_type_vars(ty, tparams)} as From<Object>>::from({a})')
        elif _is_carrier_type(ty, registry):
            call_args.append(f'<{ty} as From<Object>>::from({a})')
        else:
            call_args.append(a)
    # UFCS 取 From：接口载体可能自带 Java static from 工厂（ChronoLocalDate.from），
    # 路径解析会被固有方法遮蔽（与 interface_gen 的 upcast 同款防护）
    call = (f'<{j_erased_ty} as From<Object>>::from(Object::from(Clone::clone(self)))'
            f'.__default_{em_m.rust_name}({", ".join(call_args)})')
    if _mentions(ret_ty, tparams) or _is_carrier_type(ret_ty, registry):
        # 声明返回提及类型变量 / 是载体 → 条目（擦除）返回 Object：解包后装箱
        return f'Ok(Into::<Object>::into({call}?))'
    # 声明返回不含类型变量 → 擦除签名与声明一致，Result 直接透传
    return call


def synthesize(emissions: dict, registry: dict) -> None:
    """对账本内每个可合成接口，把合成对象文本追加到接口发射尾部。

    在全部 resolve_*（接口实现 / 继承成员）之后、落盘之前调用：条目签名与
    default 体有无取自发射记录（与落盘内容同源，不二次推导）。"""
    for iface_bin in sorted(SAM_LEDGER):
        spec = SAM_LEDGER[iface_bin]
        em = emissions.get(iface_bin)
        if em is None:
            raise RuntimeError(f'[sam-objects] 接口未发射: {iface_bin}')
        if em.handwritten:
            continue  # 预扫描已排除；防御性跳过（合成文本无处落盘）
        short = short_cls(iface_bin)
        lam = f'{short}__Lambda'
        param_names = [f'__a{i}' for i in range(len(spec.erased_params))]
        fn_ty = f'Rc<dyn Fn({", ".join(spec.erased_params)}) -> Result<{spec.erased_ret}>>'

        lines: list[str] = []
        lines.append('// ── A-5 函数式接口合成对象（LambdaMetafactory 产物的同构物）──')
        lines.append(f'// {iface_bin} 的 lambda 实例：SAM 闭包为存储，实现本接口及超接口的')
        lines.append('// __VTable（SAM 条目直调闭包、default 条目经载体 __default_<m> 体执行）；')
        lines.append('// __interface 查询对本接口及超接口闭包应答。')
        lines.append('#[derive(Clone)]')
        lines.append(f'pub struct {lam}(pub {fn_ty});')
        lines.append('')
        lines.append(f'impl {lam} {{')
        lines.append(f'    pub fn new(f: {fn_ty}) -> Self {{ Self(f) }}')
        lines.append('}')
        lines.append('')

        # instanceof 静态名单：I + 超接口闭包 + 根类 + LambdaMetafactory 恒加的
        # Serializable（实现集与 JVM 一致；与类侧 all_supertypes 名单同构生成）
        patterns = sorted({iface_bin, *spec.closure, _OBJECT_CLASS, _SERIALIZABLE})
        lines.append(f'impl ObjectVTable for {lam} {{')
        lines.append('    fn as_any(&self) -> &dyn std::any::Any { self }')
        lines.append(f'    fn __obj_str(&self) -> std::string::String {{ std::format!("{iface_bin}::Lambda") }}')
        lines.append(f'    fn __class_name(&self) -> &\'static str {{ "{iface_bin}" }}')
        lines.append('    fn is_instance_of(&self, type_id: &str) -> bool {')
        lines.append('        matches!(type_id, ' + ' | '.join(f'"{p}"' for p in patterns) + ')')
        lines.append('    }')
        lines.append('    fn __interface(self: Rc<Self>, slot: &mut dyn std::any::Any) {')
        impl_targets: list[tuple[str, str]] = []   # (J binary, J 全限定路径)
        for jbin in spec.closure:
            jci = registry.get(jbin)
            if jci is None or not _contract_methods(jci, registry):
                continue  # 标记接口（无契约方法）无槽可填，只进 instanceof 名单
            jpath = _quote_path(jbin, em, emissions)
            lines.append('        if let Some(s) = slot.downcast_mut::'
                         f'<Option<Rc<dyn {jpath}__VTable>>>() {{ *s = Some(self); return; }}')
            impl_targets.append((jbin, jpath))
        lines.append('    }')
        lines.append('}')
        lines.append('')

        # 各闭包接口的 vtable 实现。条目分三类：
        #   1. SAM（未被 default 覆盖的唯一抽象方法）——闭包直调（条目擦除签名
        #      与站点闭包同源于 SAM 描述符，恒等）；
        #   2. 声明处即 default——经声明接口载体的 __default_<m> 执行默认体；
        #   3. 本接口抽象、被闭包内其它接口的 default 覆盖（JLS 继承 default，
        #      `Sink.OfInt` 的 accept(I)V 由 Sink 的 default 覆盖）——经覆盖接口
        #      载体的 __default_<m> 执行（JVM 的 lambda 类继承同一默认体）。
        # 无翻译体的 default 维持 trait 缺省 panic（与类侧 stub 行为一致）。
        sam_key = (spec.sam_name, _param_part(spec.sam_desc))
        default_bodies: dict[tuple[str, str], tuple[str, object, dict]] = {}
        for jbin_d, _ in impl_targets:
            jci_d = registry.get(jbin_d)
            jem_d = emissions.get(jbin_d)
            for jm_d in _contract_methods(jci_d, registry):
                if jm_d.access_flags & _ACC_ABSTRACT:
                    continue
                em_d = jem_d.find(jm_d.name, _param_part(jm_d.descriptor))
                if em_d is not None and getattr(em_d, 'has_body', False):
                    default_bodies.setdefault((jm_d.name, _param_part(jm_d.descriptor)),
                                              (jbin_d, em_d, jci_d))
        for jbin, jpath in impl_targets:
            jci = registry.get(jbin)
            jem = emissions.get(jbin)
            j_params = effective_class_type_params(jci, registry)
            j_erased_targs = f"<{', '.join(['Object'] * len(j_params))}>" if j_params else ''
            j_erased_ty = f'{jpath}{j_erased_targs}'
            entries: list[str] = []
            for jm in _contract_methods(jci, registry):
                key = (jm.name, _param_part(jm.descriptor))
                em_m = jem.find(jm.name, _param_part(jm.descriptor))
                if em_m is None:
                    continue
                sig_parts = _entry_sig_parts(em_m, jci, registry)
                if sig_parts is None:
                    continue
                head, args = sig_parts
                arg_str = ', '.join(args)
                if key == sam_key:
                    body = f'(self.0)({arg_str})'
                else:
                    # default 的执行载体：声明处带体优先，其次覆盖本抽象的闭包 default
                    if not (jm.access_flags & _ACC_ABSTRACT):
                        target = (jbin, em_m, jci) if getattr(em_m, 'has_body', False) else None
                    else:
                        target = default_bodies.get(key)
                    if target is None:
                        continue
                    kbin, em_k, kci = target
                    kpath = _quote_path(kbin, em, emissions)
                    k_params = effective_class_type_params(kci, registry)
                    k_targs = f"<{', '.join(['Object'] * len(k_params))}>" if k_params else ''
                    body = _default_entry_body(em_k, kci, registry, f'{kpath}{k_targs}', args)
                    if body is None:
                        continue
                entries.append(f'    fn {head} {{ {body} }}')
            # __interface 已为本接口应答（填槽），impl 必须恒在——条目可为空：
            # 空档（无翻译体的 default / SAM 之外的桥接覆盖位）落到 trait 缺省
            # panic，与旧闭包回退未命中时的行为一致
            lines.append(f'impl {jpath}__VTable for {lam} {{')
            lines.extend(entries)
            lines.append('}')
            lines.append('')

        # 解箱（checkcast 语义保留：类型不符 → JvmError::class_cast，沿 ? 可捕获）
        dotted = iface_bin.replace('/', '.')
        lines.append(f'impl TryFrom<Object> for {lam} {{')
        lines.append('    type Error = JvmError;')
        lines.append('    fn try_from(obj: Object) -> Result<Self> {')
        lines.append('        obj.try_checkcast::<Self>().ok_or_else(|| JvmError::class_cast(')
        lines.append(f'            std::format!("class {{}} cannot be cast to {dotted}",')
        lines.append('                ObjectVTable::__class_name(&*obj.0).replace(\'/\', "."))))')
        lines.append('    }')
        lines.append('}')

        em.text = em.text.rstrip('\n') + '\n\n' + '\n'.join(lines) + '\n'
