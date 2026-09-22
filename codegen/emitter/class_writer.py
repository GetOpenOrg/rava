"""
单个 Java 类 → Rust 文件内容生成：_gen_class_rs 主函数。
"""

import os
import re as _re
from ..types import ClassInfo, FieldInfo, ParsedMethod
from ..method import gen_method_body, _indent
from ..cfg import CfgAuditError, STATS as _CFG_STATS
from .. import equiv_audit
from ..type_map import (short_cls, short_cls as _short_cls_g, jvm_to_rust, mangle_name,
                        rust_default, parse_class_type_params, effective_class_type_params,
                        _PRIMITIVE_MAP as _JVM_PRIMITIVE_MAP)
from ..sig_parse import parse_field_type
from ..type_args import (ancestor_type_args, class_type_param_bounds, outer_ref_field_type,
                         interface_signature_views as _interface_signature_views,
                         rust_type_with_args as _rust_type_with_args)
from ..sig_types import emitted_method_sig_types
from ..sig_types import (hierarchy_overloaded_names, instance_field_rust_name,
                         method_name_is_mangled)
from ..constants import (safe_ident, RUST_KEYWORDS as _RUST_KEYWORDS, OBJECT_CLASS as _OBJECT_CLASS,
                         CLASS_CLASS as _CLASS_CLASS,
                         PRIMITIVE_RUST_TYPES as _PRIMITIVE_RUST_TYPES, STRING_CLASS)

from .attrs import _java_class_block_head, _java_field_attr, _java_method_attr
from .method_gen import _gen_native_stub
from .vtable_util import _bin_to_rust, resolve_virtual_slot, slot_member_rust_name
from .clinit_extract import _gen_static_field_blocks, _gen_clinit_block
from .import_gen import (collect_referenced, gen_cross_imports,
                         scan_used_vtable_imports)
from .field_gen import _resolve_field_rust, _resolve_anc_field_rust
from .inherited_gen import (ClassEmission, IMPORTS_SLOT as _INHERITED_IMPORTS_SLOT,
                            MEMBERS_SLOT as _INHERITED_MEMBERS_SLOT)
from .interface_gen import IMPLS_SLOT as _INTERFACE_IMPLS_SLOT, UPCASTS_SLOT as _INTERFACE_UPCASTS_SLOT
from ..instr.member_naming import _parse_field_ref
from ..instr.member_naming import lambda_impl_rust_name, LAMBDA_NAME_LEDGER

_safe_field_name = safe_ident

_ACC_FINAL   = 0x0010
_ACC_STATIC  = 0x0008


def _adapt_interface_method(method, ci, iface_bin: str, views: dict):
    """接口方法体展开到实现类 ci：所属类换成 ci，泛型签名（方法 / 局部变量）里的接口类型变量
    换成 ci 视角下的类型实参。"""
    import copy as _copy_adapt
    from ..sig_parse import substitute_signature_type_vars as _subst
    adapted = _copy_adapt.copy(method)
    adapted.class_name = ci.name
    view = views.get(iface_bin)
    if view:
        adapted.generic_signature = _subst(method.generic_signature, view)
        if getattr(method, 'local_types', None):
            adapted.local_types = {slot: (_subst(sig, view), start)
                                   for slot, (sig, start) in method.local_types.items()}
        if getattr(method, 'local_vars', None):
            adapted.local_vars = [tuple(entry[:5]) + (_subst(entry[5], view),)
                                  for entry in method.local_vars]
    return adapted


def _owner_slot_declaration(owner_ci, m, registry):
    """槽位声明者在 owner（或其接口闭包的 default 声明者）上的方法声明。

    按槽位精确签名匹配：同名 + 同参数描述符（K-6a：协变返回覆盖的子类描述符
    与声明者不同，返回类型可收窄）。精确描述符优先，其次协变（非私有、非
    static）。owner 类链未声明时按接口 default 解析（E0053：owner 解析失败
    返回 None → 覆盖条目的 owner 形参位未擦除，trait impl 签名不匹配）。
    返回 (声明者 ClassInfo, method) 或 (None, None)。真实声明优先（synthetic /
    bridge 是编译器产物，其描述符复述更早声明者的擦除形态，不作槽位签名源）。"""
    hit = next((x for x in (owner_ci.methods or [])
                if x.name == m.name and x.descriptor == m.descriptor
                and not x.is_static and not x.is_synthetic), None)
    if hit is None:
        from .vtable_util import _ACC_PRIVATE, _same_vtable_slot
        hit = next((x for x in (owner_ci.methods or [])
                    if _same_vtable_slot(x, m)
                    and not x.is_static and not x.is_synthetic
                    and not (x.access_flags & _ACC_PRIVATE)), None)
    if hit is not None:
        return owner_ci, hit
    from .vtable_util import _descriptor_param_part
    from collections import deque
    queue = deque(owner_ci.interfaces or [])
    seen: set[str] = set()
    while queue:
        name = queue.popleft()
        if not name or name in seen:
            continue
        seen.add(name)
        ici = (registry or {}).get(name)
        if ici is None:
            continue
        hit = next((x for x in (ici.methods or [])
                    if x.name == m.name and not x.is_synthetic
                    and _descriptor_param_part(x.descriptor)
                    == _descriptor_param_part(m.descriptor)
                    and not x.is_static and not x.is_abstract), None)
        if hit is not None:
            return ici, hit
        queue.extend(ici.interfaces or [])
    return None, None


def _override_vtable_erasure(m, ci, registry) -> list[str]:
    """覆盖方法在「声明祖先 vtable」上的擦除名单 —— 槽位精确签名模型（K-6）。

    槽位（声明祖先 A 的 A__VTable）签名 = A 的发射签名（emitted_method_sig_types，
    与 A 的文件逐字同源）经 A 的类型形参 Object 化（A-1 按声明类判定）：提及 A
    自身形参、或字面 Object（接口过滤 / 原生 Object 声明，如
    MethodHandle.internalProperties → Object）的形参 / 返回位置，槽位上是 Object。
    覆盖者（本类）的发射签名在这些位置可能保持具体形态——本类形参（宏按
    type_param 擦除自行消解）、祖先形参代入后的具体实参（K-6b）、协变返回的
    具体类（K-6a）——后两类由本名单按 token 全等交给宏整体 Object 化。"""
    if not registry:
        return []
    # 声明祖先：沿超类链找 virtual_in 对应的类
    owner_bin = None
    cur = ci.super_class
    seen: set[str] = set()
    while cur and cur not in seen and cur in registry:
        seen.add(cur)
        if short_cls(cur) == m.virtual_in:
            owner_bin = cur
            break
        cur = registry[cur].super_class
    if owner_bin is None:
        return []
    owner_ci = registry[owner_bin]
    # 槽位声明：owner（含接口 default 分支）按槽位签名（同名 + 同参数描述符）解析
    decl_ci, owner_m = _owner_slot_declaration(owner_ci, m, registry)
    if owner_m is None:
        return []
    owner_params = effective_class_type_params(decl_ci, registry)
    # 槽位侧：声明者发射签名；Object 位 = 字面 Object / 空，或提及声明者自身形参
    #（与 vtable_util._slot_return_is_object 同一判定，槽位归属的跳转校验同源）
    anc_types, anc_ret = emitted_method_sig_types(
        decl_ci, owner_m, owner_params, registry)
    anc_types, anc_ret = list(anc_types or []), anc_ret
    # 本类侧：发射签名（K-6b：祖先形参已代入接收者视角实参；K-6a：无 Signature
    # 的协变覆盖回退描述符形态）—— 与 gen_method_body 逐字同源
    own_types, own_ret = emitted_method_sig_types(
        ci, m, effective_class_type_params(ci, registry), registry)
    own_types, own_ret = list(own_types or []), own_ret

    def _slot_position_is_object(ty: str) -> bool:
        if not ty or ty in ('Object', '()'):
            return True
        return any(_re.search(r'\b' + _re.escape(_p) + r'\b', ty) for _p in owner_params)

    out: list[str] = []
    for i, anc_ty in enumerate(anc_types):
        if _slot_position_is_object(anc_ty):
            if i < len(own_types) and own_types[i] and own_types[i] != 'Object':
                out.append(own_types[i])
    def _result_inner(ty: str):
        mm = _re.match(r'^Result<(.*)>$', ty or '')
        return mm.group(1) if mm else None
    anc_inner = _result_inner(anc_ret)
    if anc_inner is not None:
        if _slot_position_is_object(anc_inner):
            own_inner = _result_inner(own_ret) or own_ret
            if own_inner and own_inner not in ('Object', '()'):
                out.append(own_inner)
    elif _slot_position_is_object(anc_ret):
        own_inner = _result_inner(own_ret) or own_ret
        if own_inner and own_inner not in ('Object', '()'):
            out.append(own_inner)
    return list(dict.fromkeys(out))


def _emit_method_blocks(ci, registry, call_chain, stub_bodies, new_format_map,
                        _nf_entry, class_type_params, overloaded_names,
                        visible_methods, _type_only, _is_iface,
                        method_blocks, _iface_lambda_blocks) -> None:
    """方法翻译主循环：visible + 非桥接 synthetic 方法逐个发射。

    <clinit> → __clinit（clinit_extract）；G-10 接口私有 lambda body 落
    java_class! 块外的擦除固有 impl 块；接口实例方法发声明 / 载体 default 体；
    普通方法按调用链翻译字节码或生成 stub。结果追加进 method_blocks 与
    _iface_lambda_blocks。原 _gen_class_rs 内联段逐字搬移，闭包变量改为本
    函数参数。"""
    used_rust_names: dict[str, int] = {}  # 追踪已用名，防止 mangle 碰撞后重名
    # 非桥接的 synthetic 方法（lambda$xxx$N、access$NNN 等）是 invokedynamic 闭包 /
    # 内部类访问器的真实调用目标，必须生成定义；桥接方法（ACC_BRIDGE）与真实方法同名，继续过滤。
    # 注意：它们不参与 name_counts / overloaded_names 统计（编译器保证其名字唯一）。
    _ACC_BRIDGE = 0x0040
    emitted_methods = visible_methods + [
        m for m in ci.methods
        if m.is_synthetic and not (m.access_flags & _ACC_BRIDGE)
        and m.name not in ('<init>', '<clinit>')
    ]
    # 接口重声明的 Object 公开方法（如 Comparator.equals）经 Object vtable 分派，不进接口 vtable
    from ..instr.member_owner import _root_virtual_methods
    _root_method_keys = _root_virtual_methods() if _is_iface else set()
    for m in emitted_methods:
        if m.name == '<clinit>':
            # <clinit> → `fn __clinit()`：宏生成的 __class_init() 状态机在首次主动使用时调用它。
            # 类型存根不会被初始化；不在调用链上的 <clinit> 与其它方法同规则生成 panic 存根。
            _clinit_block = _gen_clinit_block(
                m, ci, registry=registry, class_type_params=class_type_params,
                overloaded_names=overloaded_names, call_chain=call_chain,
                _type_only=_type_only)
            if _clinit_block is not None:
                method_blocks.append(_clinit_block)
            continue
        if _is_iface and not m.is_static and m.is_synthetic and m.name.startswith('lambda$'):
            # G-10：接口的私有实例 lambda body（如 Comparator.lambda$thenComparing$...）。
            # 不是接口契约成员：不进 Iface__VTable、不被实现类继承展开，而是落在接口
            # 载体擦除实例化（Iface<Object>）的固有 impl 块 —— 调用点（sim/dynamic.py
            # 的接口实例分支）生成 Into::<Iface<Object>>::into(recv).<name>(args)，
            # 接收者即擦除载体，body 内对 this 的虚调用经载体 vtable 分派（JVM 语义：
            # 私有 lambda 体在运行时接收者上执行）。方法名取 lambda_impl_rust_name
            # 单一来源，与调用点引用名一起受 G-10 生成期断言保护。
            _lam_rust = lambda_impl_rust_name(ci.name, m.name, m.descriptor, registry)
            # this 按擦除实例化定型（Comparator<T> → Comparator<Object>），与调用点
            # 接收者的 Into::<Iface<Object>> 一致；body 内 Object 实参直接命中擦除签名
            _lam_ctparams = ['Object'] * len(class_type_params) if class_type_params else []
            _lam_in_chain = (call_chain is None or (ci.name, m.name, m.descriptor) in call_chain)
            try:
                _lam_block = gen_method_body(
                    m, ci, registry=registry,
                    class_type_params=_lam_ctparams,
                    overloaded_names=overloaded_names,
                    rust_name=_lam_rust,
                ) if (_lam_in_chain and not stub_bodies) else _gen_native_stub(
                    m, ci, rust_name=_lam_rust, registry=registry,
                    class_type_params=_lam_ctparams)
            except CfgAuditError:
                raise
            except Exception as e:
                _CFG_STATS.record_stub_fallback(f"{ci.name}.{m.name}:{m.descriptor}", repr(e))
                _lam_block = _gen_native_stub(m, ci, rust_name=_lam_rust, registry=registry,
                                              class_type_params=_lam_ctparams)
            _iface_lambda_blocks.append(_lam_block)
            LAMBDA_NAME_LEDGER.record_definition(ci.name, m.name, _lam_rust)
            continue
        if _is_iface and not m.is_static and not m.is_synthetic \
                and (m.access_flags & 0x0002) \
                and (m.name, m.descriptor[:m.descriptor.index(')') + 1]) not in _root_method_keys:
            # Java 9+ 接口私有实例方法：非契约成员（不进 Iface__VTable、不被实现类
            # 继承——JVM 对 invokeinterface 私有目标的解析是直接执行接口自身的实现，
            # 实现类同名方法不构成覆盖）。与 G-10 lambda 体同形：落接口载体擦除
            # 实例化的固有 impl 块（java_class! 块之外，不产生 vtable / 分派成员），
            # 调用侧（invoke_virtual 的私有接口方法分支）经载体路由。
            _pv_ctparams = ['Object'] * len(class_type_params) if class_type_params else []
            _pv_in_chain = (call_chain is None or (ci.name, m.name, m.descriptor) in call_chain)
            _pv_rust = (mangle_name(m.name, m.descriptor)
                        if method_name_is_mangled(ci, m, registry) else m.name)
            if _pv_rust in used_rust_names:
                used_rust_names[_pv_rust] += 1
                _pv_rust = f'{_pv_rust}_{used_rust_names[_pv_rust]}'
            else:
                used_rust_names[_pv_rust] = 0
            try:
                _pv_block = gen_method_body(
                    m, ci, registry=registry,
                    class_type_params=_pv_ctparams,
                    overloaded_names=overloaded_names,
                    rust_name=_pv_rust,
                ) if (_pv_in_chain and not stub_bodies) else _gen_native_stub(
                    m, ci, rust_name=_pv_rust, registry=registry,
                    class_type_params=_pv_ctparams)
            except CfgAuditError:
                raise
            except Exception as e:
                _CFG_STATS.record_stub_fallback(f"{ci.name}.{m.name}:{m.descriptor}(iface-private)", repr(e))
                _pv_block = _gen_native_stub(m, ci, rust_name=_pv_rust, registry=registry,
                                             class_type_params=_pv_ctparams)
            _iface_lambda_blocks.append(_pv_block)
            LAMBDA_NAME_LEDGER.record_definition(ci.name, m.name, safe_ident(_pv_rust))
            continue
        if _is_iface and not m.is_static and (
                m.is_synthetic or (m.access_flags & 0x0002) or (m.name, m.descriptor[:m.descriptor.index(')') + 1]) in _root_method_keys):
            continue  # 其余私有 / 合成实例方法不是接口契约的一部分
        # 确定最终 Rust 方法名（有重载则加描述符后缀）
        rust_name = (mangle_name(m.name, m.descriptor)
                     if method_name_is_mangled(ci, m, registry) else m.name)
        # 构造器统一用 new / new_suffix
        if m.is_constructor:
            if '<init>' in overloaded_names:
                rust_name = mangle_name('new', m.descriptor)
            else:
                rust_name = 'new'
        # 碰撞去重：若 mangle 后仍重名，追加数字后缀
        if rust_name in used_rust_names:
            used_rust_names[rust_name] += 1
            rust_name = f'{rust_name}_{used_rust_names[rust_name]}'
        else:
            used_rust_names[rust_name] = 0

        # 若 new_format_map 覆盖了此方法，跳过（_impl 模块已提供实现）
        _nf_covered = (_nf_entry or {}).get('methods', set())
        fn_name_check = safe_ident(rust_name or m.name)
        if fn_name_check in _nf_covered:
            # 手写共置文件按同一 mangle 规则提供实现 → 定义名仍记为计算名（G-10 账本）
            LAMBDA_NAME_LEDGER.record_definition(ci.name, m.name, fn_name_check)
            continue
        # G-10 账本：定义侧登记最终 Rust 名（stub / 翻译体 / 接口声明各路径统一在此登记）
        LAMBDA_NAME_LEDGER.record_definition(ci.name, m.name, fn_name_check)

        if _is_iface and not m.is_static:
            # 接口实例方法（abstract / default）：宏据此生成擦除签名的接口 vtable
            # （`Iface__VTable`）与载体上的同名分派方法。abstract 保持无体声明；
            # default 方法体由实现类继承展开（类覆盖优先），**同时**以载体为接收者
            # 翻译一份落到接口自身块内 —— 载体分派在 vtable 查询未命中（lambda /
            # 闭包接收者不实现 `Iface__VTable`）时执行它，对应 JVM 对函数式接口
            # 实例调用 default 方法的语义（A-5 的闭包接收者路径）。
            _dm_iface_body = None
            # 含 `Iface.super.m()`（invokespecial 常量池类为接口）的 default 体不落到
            # 接口载体：其展开成员（`Owner_super_m`）建模在实现类，载体上下文不存在。
            # 判定与调用侧（invoke.py invokespecial）同源：_resolve_interface_special_target。
            from ..instr.member_owner import (
                _resolve_interface_special_target as _rist_dm,
            )
            from ..instr.member_naming import (
                _method_ref_binary_class as _mrbc_dm,
                _method_ref_descriptor as _mrd_dm,
            )

            def _ref_mname(c: str) -> str:
                _rest = c.split(' ', 1)[1] if ' ' in c else c
                _dot = _rest.find('.')
                _colon = _rest.find(':', _dot)
                return _rest[_dot + 1:_colon] if 0 < _dot < _colon else ''

            _has_iface_super = False
            for _ins in (m.instrs or []):
                if _ins.opcode != 'invokespecial' or not _ins.comment:
                    continue
                if _rist_dm(_mrbc_dm(_ins.comment), _ref_mname(_ins.comment),
                            _mrd_dm(_ins.comment), registry):
                    _has_iface_super = True
                    break
            # 载体只声明本接口的方法：default 体调用本接口未声明的方法（典型：
            # `this.hasNext()` 声明在父接口 Iterator）时不落接口 —— 实现类侧的
            # 继承成员展开（inherited_from）在载体上下文不存在，仍走类 vtable 分派。
            _own_sigs = {(_mm.name, _mm.descriptor) for _mm in ci.methods}
            _calls_nonself = False
            for _ins in (m.instrs or []):
                if _ins.opcode not in ('invokevirtual', 'invokeinterface') or not _ins.comment:
                    continue
                if (_ref_mname(_ins.comment), _mrd_dm(_ins.comment)) not in _own_sigs:
                    _calls_nonself = True
                    break
            _dm_in_cc = (
                not m.is_abstract and not m.is_synthetic and not _has_iface_super
                and not _calls_nonself
                and (call_chain is None or (ci.name, m.name, m.descriptor) in call_chain)
            )
            if _dm_in_cc and not stub_bodies:
                try:
                    _dm_iface_body = gen_method_body(
                        m, ci, registry=registry,
                        class_type_params=class_type_params,
                        overloaded_names=overloaded_names,
                        rust_name=rust_name,
                        in_vtable_body=False,
                    )
                except CfgAuditError:
                    raise
                except Exception as e:
                    _CFG_STATS.record_stub_fallback(
                        f"{ci.name}.{m.name}:{m.descriptor}(iface-default)", repr(e))
                    _dm_iface_body = None
            if _dm_iface_body is not None:
                method_blocks.append(_java_method_attr(m) + '\n' + _dm_iface_body)
                continue
            _decl = _gen_native_stub(m, ci, rust_name=rust_name, registry=registry,
                                     class_type_params=class_type_params)
            _decl_sig = next(ln.strip() for ln in _decl.split('\n') if ln.lstrip().startswith('pub fn '))
            _decl_sig = _decl_sig[:-1].rstrip() if _decl_sig.endswith('{') else _decl_sig
            method_blocks.append(_java_method_attr(m) + '\n' + _decl_sig + ';')
            continue

        # 计算虚方法归属（vtable 架构）：精确描述符路径（_find_virtual_in）→ 判为本类新槽位时
        # 按协变模型归属父槽位（K-6a，resolve_virtual_slot；两路不一致取更远声明者）
        m.virtual_in = resolve_virtual_slot(m, ci, registry, new_format_map)
        # 槽位名解耦（K-6 机制推广到覆盖条目）：wrapper 名按本类重载态（rust_name），
        # trait 槽位 impl 名按槽位声明者态——两者不同时经 vtable_name 属性传给宏
        if m.virtual_in and m.virtual_in != short_cls(ci.name):
            _slot_name = slot_member_rust_name(m, ci, registry, new_format_map)
            if _slot_name and _slot_name != rust_name:
                m.vtable_name = _slot_name
            m.vtable_erasure = _override_vtable_erasure(m, ci, registry)

        # 虚方法的方法体由共置 `_impl.rs` 手写为 `__impl_<method>`：声明留在宏块内（进 vtable、
        # 参与覆盖与根类方法桥接），宏经 wrapper 钩子执行手写体
        if m.virtual_in and ('__impl_' + fn_name_check) in _nf_covered:
            m.handwritten_body = True
            _decl = _gen_native_stub(m, ci, rust_name=rust_name, registry=registry,
                                     class_type_params=class_type_params)
            _decl_sig = next(ln.strip() for ln in _decl.split('\n') if ln.lstrip().startswith('pub fn '))
            _decl_sig = _decl_sig[:-1].rstrip() if _decl_sig.endswith('{') else _decl_sig
            method_blocks.append(_java_method_attr(m) + '\n' + _decl_sig + ';')
            continue

        attr_line = _java_method_attr(m)
        # 判断该方法是否需要翻译字节码：
        #   1. native / abstract → 永远生成 stub（panic!）
        #   2. call_chain 不为空 且 此方法不在调用链上 → panic!("stub: ...")
        #   3. stub_bodies=True（兜底/fallback）→ stub
        #   4. 其他 → 翻译字节码
        in_call_chain = (
            call_chain is None or
            (ci.name, m.name, m.descriptor) in call_chain
        )
        if m.is_native or m.is_abstract:
            stub = _gen_native_stub(m, ci, rust_name=rust_name, registry=registry, class_type_params=class_type_params)
            method_blocks.append(attr_line + '\n' + stub)
        elif not in_call_chain or stub_bodies:
            stub = _gen_native_stub(m, ci, rust_name=rust_name, registry=registry, class_type_params=class_type_params)
            method_blocks.append(attr_line + '\n' + stub)
        else:
            try:
                body = gen_method_body(
                    m, ci, registry=registry,
                    class_type_params=class_type_params,
                    overloaded_names=overloaded_names,
                    rust_name=rust_name,
                    in_vtable_body=bool(m.virtual_in),
                )
                method_blocks.append(attr_line + '\n' + body)
            except CfgAuditError:
                raise
            except Exception as e:
                # 翻译失败：退化为 stub，避免生成无效 Rust
                _CFG_STATS.record_stub_fallback(f"{ci.name}.{m.name}:{m.descriptor}", repr(e))
                import os as _os
                if _os.environ.get('JAVA_RTA_DEBUG'):
                    import traceback as _tb
                    print(f"[DEBUG] stub fallback for {ci.name}.{m.name}{m.descriptor}: {e}", file=__import__('sys').stderr)
                    _tb.print_exc()
                stub = _gen_native_stub(m, ci, rust_name=rust_name, registry=registry, class_type_params=class_type_params)
                method_blocks.append(attr_line + '\n' + stub)


def _emit_interface_default_inheritance(ci, registry, call_chain, stub_bodies,
                                        class_type_params, overloaded_names,
                                        visible_methods, method_blocks,
                                        _translated_defaults) -> None:
    """接口 default 方法继承：当类实现接口但未覆盖其 default 方法时，自动生成继承实现。
    已翻译体记入 _translated_defaults（Iface.super 展开段的种子）。原
    _gen_class_rs 内联段逐字搬移（含守卫与祖先接口去重 / 预扫描计数），闭包
    变量改为本函数参数。"""
    # 接口 default 方法继承：当类实现接口但未覆盖其 default 方法时，自动生成继承实现
    if ci.interfaces and registry and not ci.is_interface:
        import copy as _copy
        existing_sigs: set[tuple] = {(m.name, m.descriptor) for m in visible_methods}
        # 参数签名集合（忽略返回类型），用于检测协变返回覆盖（如 LinkedList.reversed() 覆盖 Deque.reversed()）
        def _param_part(desc: str) -> str:
            idx = desc.find(')')
            return desc[:idx + 1] if idx >= 0 else desc
        existing_param_sigs: set[tuple] = {(m.name, _param_part(m.descriptor)) for m in visible_methods}
        # 覆盖判定沿父类链（JVM 方法解析沿超类链查找）：祖先类自身的实例方法
        # （非 static / 非 private / 非 synthetic / 非构造器）同样构成覆盖——
        # HashMap$KeyIterator 未声明 remove，但父类 HashMap$HashIterator 的
        # public remove 已覆盖 Iterator.remove；只查本类会把 default 体（UOE）
        # 注入本类，遮蔽继承的正确实现。槽位填补由 resolve_interface_impls 的
        # _locate 沿链定位后经 inherited_calls 登记转发成员（与 hasNext 同机制）。
        # 祖先**类的方法**在此累积；祖先实现的**接口**仍由下方 _anc_ifaces 去重
        # 机制处理（防 E0034 的作用不变）。
        _sup_cur = ci.super_class
        _sup_seen: set[str] = set()
        while _sup_cur and _sup_cur in registry and _sup_cur not in _sup_seen:
            _sup_seen.add(_sup_cur)
            _sup_ci = registry[_sup_cur]
            for _sm in _sup_ci.methods:
                if (_sm.is_static or _sm.is_synthetic
                        or _sm.name in ('<init>', '<clinit>')
                        or (_sm.access_flags & 0x0002)):
                    continue
                existing_sigs.add((_sm.name, _sm.descriptor))
                existing_param_sigs.add((_sm.name, _param_part(_sm.descriptor)))
            _sup_cur = _sup_ci.super_class
        # 已用的 Rust 方法名（用于检测 default 方法与类自身方法重名）
        used_rust_names: set[str] = {
            (mangle_name(m.name, m.descriptor) if method_name_is_mangled(ci, m, registry) else m.name)
            for m in visible_methods if m.name not in ('<init>', '<clinit>')
        }
        # 祖先类已实现的接口：其 default 方法已注入到该祖先（VirtualDefine），
        # 本类经 VTable supertrait 链继承；重复注入会在 vtable 上产生同名二义（E0034）。
        _anc_ifaces: set[str] = set()
        _anc_cur = ci.super_class
        _anc_seen: set[str] = set()
        while _anc_cur and _anc_cur in registry and _anc_cur not in _anc_seen:
            _anc_seen.add(_anc_cur)
            _anc_ci = registry[_anc_cur]
            _aq = list(_anc_ci.interfaces or [])
            while _aq:
                _an = _aq.pop(0)
                if _an in _anc_ifaces:
                    continue
                _anc_ifaces.add(_an)
                _a_ci = registry.get(_an)
                if _a_ci is not None:
                    _aq.extend(_a_ci.interfaces or [])
            _anc_cur = _anc_ci.super_class
        # 预扫描：统计所有待继承 default 方法的名字（用于 default 方法之间互相冲突判断）
        default_name_counts: dict[str, int] = {}
        _pre_counted_sigs: set[tuple] = set()
        _pre_iface_queue = list(ci.interfaces)
        _pre_visited: set[str] = set(_anc_ifaces)
        while _pre_iface_queue:
            _iname = _pre_iface_queue.pop(0)
            if _iname in _pre_visited:
                continue
            _pre_visited.add(_iname)
            _ici = registry.get(_iname)
            if _ici is None:
                continue
            if _ici.interfaces:
                _pre_iface_queue.extend(_ici.interfaces)
            for _dm in _ici.methods:
                if (not _dm.is_abstract and not _dm.is_static and not _dm.is_synthetic
                        and not (_dm.access_flags & 0x0002)
                        and _dm.name not in ('<init>', '<clinit>')):
                    _dm_pp = _param_part(_dm.descriptor)
                    # 子接口覆盖父接口的同签名 default（如子接口重新声明 and(P)）只注入一次，
                    # 计数也必须按 (name, 参数签名) 去重，否则单一方法被误判为重载而 mangle
                    if ((_dm.name, _dm.descriptor) not in existing_sigs
                            and (_dm.name, _dm_pp) not in existing_param_sigs
                            and (_dm.name, _dm_pp) not in _pre_counted_sigs):
                        _pre_counted_sigs.add((_dm.name, _dm_pp))
                        default_name_counts[_dm.name] = default_name_counts.get(_dm.name, 0) + 1
        iface_queue: list[str] = list(ci.interfaces)
        visited_ifaces: set[str] = set(_anc_ifaces)
        _iface_sig_views = _interface_signature_views(ci, registry)
        while iface_queue:
            iface_name = iface_queue.pop(0)
            if iface_name in visited_ifaces:
                continue
            visited_ifaces.add(iface_name)
            iface_ci = registry.get(iface_name)
            if iface_ci is None:
                continue
            if iface_ci.interfaces:
                iface_queue.extend(iface_ci.interfaces)
            for dm in iface_ci.methods:
                if (dm.is_abstract or dm.is_static or dm.is_synthetic
                        or (dm.access_flags & 0x0002)
                        or dm.name in ('<init>', '<clinit>')):
                    continue
                if (dm.name, dm.descriptor) in existing_sigs:
                    continue
                # 协变返回覆盖：如果类已有同名同参方法（返回类型不同），也跳过注入
                _dm_pp = _param_part(dm.descriptor)
                if (dm.name, _dm_pp) in existing_param_sigs:
                    continue
                existing_sigs.add((dm.name, dm.descriptor))
                existing_param_sigs.add((dm.name, _dm_pp))
                # 若名字与类自身方法冲突，或 default 方法之间有重名，则 mangle
                needs_mangle = (dm.name in overloaded_names or
                                dm.name in used_rust_names or
                                default_name_counts.get(dm.name, 0) > 1)
                dm_rust = mangle_name(dm.name, dm.descriptor) if needs_mangle else dm.name
                used_rust_names.add(dm_rust)
                # 将 class_name 替换为实现类，使 gen_method_body 生成正确的 this 类型
                dm_adapted = _adapt_interface_method(dm, ci, iface_name, _iface_sig_views)
                # 接口 default 方法注入实现类时：virtual_in 改为实现类名（VirtualDefine）
                # 原始 virtual_in 是接口名（如 Drawable），在宏中会生成不存在的 Drawable__VTable impl
                dm_adapted.virtual_in = _bin_to_rust(ci.name)
                dm_attr = _java_method_attr(dm_adapted)
                # 仅在调用链上时翻译字节码，否则生成 stub（避免复杂 JDK default 方法引入编译错误）
                dm_in_cc = (
                    call_chain is None or
                    (ci.name, dm.name, dm.descriptor) in call_chain or
                    (iface_name, dm.name, dm.descriptor) in call_chain
                )
                if dm_in_cc and not stub_bodies:
                    try:
                        dm_body = gen_method_body(
                            dm_adapted, ci, registry=registry,
                            class_type_params=class_type_params,
                            overloaded_names=overloaded_names,
                            rust_name=dm_rust,
                            in_vtable_body=True,
                        )
                        method_blocks.append(dm_attr + '\n' + dm_body)
                        _translated_defaults.append(dm)
                    except CfgAuditError:
                        raise
                    except Exception as e:
                        _CFG_STATS.record_stub_fallback(f"{ci.name}.{dm.name}:{dm.descriptor}", repr(e))
                        dm_stub = _gen_native_stub(dm_adapted, ci, rust_name=dm_rust, registry=registry, class_type_params=class_type_params)
                        method_blocks.append(dm_attr + '\n' + dm_stub)
                else:
                    dm_stub = _gen_native_stub(dm_adapted, ci, rust_name=dm_rust, registry=registry, class_type_params=class_type_params)
                    method_blocks.append(dm_attr + '\n' + dm_stub)


def _emit_interface_special_members(ci, registry, call_chain, stub_bodies,
                                    class_type_params, overloaded_names,
                                    visible_methods, method_blocks,
                                    _translated_defaults) -> None:
    """`Iface.super.m()` / 接口私有方法（invokespecial InterfaceMethod）的落点：
    接口方法体按「展开到实现类」建模，以非虚成员 Iface_super_m 展开到本类，
    展开出的方法体自身的同类调用递归处理。原 _gen_class_rs 内联段逐字搬移，
    闭包变量改为本函数参数。"""
    # `Iface.super.m()` / 接口私有方法（invokespecial InterfaceMethod）的落点：
    # 接口方法体按「展开到实现类」建模，被本类覆盖的 default 方法体（及接口私有方法体）
    # 以非虚成员 `Iface_super_m` 展开到本类；展开出的方法体自身的同类调用递归处理。
    if registry and not ci.is_interface and not stub_bodies:
        import copy as _copy_sp
        from ..instr.member_owner import (
            _resolve_interface_special_target as _rist,
            interface_special_member_name as _ismn,
            parse_method_ref as _pmr_sp,
        )
        from ..instr.member_naming import (
            _method_ref_binary_class as _mrbc,
            _method_ref_descriptor as _mrd_sp,
        )
        _sp_sources: list = [
            _m for _m in visible_methods
            if call_chain is None or (ci.name, _m.name, _m.descriptor) in call_chain
        ] + _translated_defaults
        _sp_done: set[tuple] = set()
        while _sp_sources:
            _src = _sp_sources.pop(0)
            for _ins in getattr(_src, 'instrs', []) or []:
                if getattr(_ins, 'opcode', '') != 'invokespecial':
                    continue
                _c = (getattr(_ins, 'comment', '') or '').strip()
                if not _c or '<init>' in _c:
                    continue
                _, _sp_mname, _, _ = _pmr_sp(_c)
                _sp_desc = _mrd_sp(_c)
                _sp_owner = _rist(_mrbc(_c), _sp_mname, _sp_desc, registry)
                if not _sp_owner or (_sp_owner, _sp_mname, _sp_desc) in _sp_done:
                    continue
                _sp_done.add((_sp_owner, _sp_mname, _sp_desc))
                _sp_m = next(_im for _im in registry[_sp_owner].methods
                             if _im.name == _sp_mname and _im.descriptor == _sp_desc
                             and not _im.is_static and not _im.is_abstract)
                _sp_rust = safe_ident(_ismn(_sp_owner, _sp_mname, _sp_desc, registry))
                _sp_adapted = _adapt_interface_method(
                    _sp_m, ci, _sp_owner, _interface_signature_views(ci, registry))
                _sp_adapted.virtual_in = None  # 非虚：只经 `Iface.super.m()` 静态绑定到达
                _sp_attr = _java_method_attr(_sp_adapted)
                _sp_in_cc = (call_chain is None
                             or (_sp_owner, _sp_mname, _sp_desc) in call_chain)
                _sp_block = None
                if _sp_in_cc:
                    try:
                        _sp_block = gen_method_body(
                            _sp_adapted, ci, registry=registry,
                            class_type_params=class_type_params,
                            overloaded_names=overloaded_names,
                            rust_name=_sp_rust,
                            in_vtable_body=False,
                        )
                        _sp_sources.append(_sp_m)
                    except CfgAuditError:
                        raise
                    except Exception as e:
                        _CFG_STATS.record_stub_fallback(
                            f"{ci.name}.{_sp_mname}:{_sp_desc}", repr(e))
                if _sp_block is None:
                    _sp_block = _gen_native_stub(_sp_adapted, ci, rust_name=_sp_rust,
                                                 registry=registry,
                                                 class_type_params=class_type_params)
                method_blocks.append(_sp_attr + '\n' + _sp_block)


def _emit_superclass_virtual_inheritance(ci, registry, call_chain, stub_bodies,
                                         new_format_map, class_type_params,
                                         overloaded_names, visible_methods,
                                         method_blocks) -> None:
    """超类虚方法继承：若子类未覆盖祖先虚方法，生成继承实现使 vtable impl
    包含实际函数体（避免子类 Ancestor__VTable impl 退化为 panic 存根）。

    用户类超类链（无 '/'）保持全量处理（体内发射）；JDK 超类链（K-6 后放开）
    的槽位填补经 inherited_calls 登记需求，由 inherited_gen 在两阶段生成的
    第二阶段统一生成**转发成员**——与调用点触发的继承成员同一机制：owner 的
    rust_name（与声明者 trait 名一致）、槽位擦除名单、base 函数导入全部单一
    来源。全量体重发射在 JDK 链不可行：体引用的类型不在本文件导入集（E0433）、
    方法名按接收者重载态 mangle 与声明者 trait 名不一致（E0407）、手写伴生调用
    （如 Throwable 的 fillInStackTrace_i）在子类上下文不可解析（E0599）、
    synthetic 桥接被整体复制（E0201）。转发体（Owner__m_base / __as_Owner 钩子）
    在 owner 模块上下文执行真实体，与 super 调用同源，不重发射字节码。

    JDK 链门控：(祖先类, 方法) 已在 BFS 调用链上——虚分派经声明类的 wrapper
    静态类型落在具体子类的 impl 上（如 PipelineHelper.wrapAndCopyInto 声明抽象、
    AbstractPipeline 实现 final、ReferencePipeline_Head 运行期接收），BFS 只保证
    实现体的收录，不保证槽位在每个翻译子类上有条目——缺条目则落回声明类的
    trait default（抽象声明 = stub panic）。抽象声明不登记（无体可转发，trait
    default 即存根）；native 声明仅在其体由共置 _impl.rs 手写时登记（__impl_
    钩子路径），未手写的 native 转发到声明类存根与 trait default 等价。

    链模式按**逐祖先**判定（非仅直接超类）：用户超类链可经用户祖先进入 JDK
    祖先（枚举常量体 Operation$1 → Operation → java/lang/Enum），JDK 祖先段
    必须切换为转发——全量体重发射在 JDK 链不可行的四条理由见上（E0433 /
    E0407 / E0599 / E0201）。

    覆盖判定按 vtable 槽位语义（同名 + 参数描述符，返回可协变，键与
    _same_vtable_slot 同构），与接口 default 继承的 existing_param_sigs 一致：
    协变覆盖（get()Integer 覆盖 get()Object）经 vtable_erasure 已填父槽位，
    精确描述符匹配漏判 → 父方法重发射与真实覆盖重名（E0201）。"""
    _user_chain = bool(ci.super_class) and '/' not in ci.super_class
    if not ci.is_interface and registry and ci.super_class and (_user_chain or call_chain is not None):
        import copy as _copy3
        from .. import inherited_calls as _inherited_calls
        from .vtable_util import _descriptor_param_part as _vinh_param_part
        # 覆盖判定键：vtable 槽位（name + 参数描述符部分，返回类型协变不计）
        _vinh_existing: set[tuple] = {
            (m.name, _vinh_param_part(m.descriptor)) for m in visible_methods}
        # 本轮已发射的槽位（name + 参数部分）：链上多个祖先重复声明同签名（含
        # 逐级协变重声明）时只落一次，桥路径不受 _vinh_existing 初始集抑制
        _vinh_done: set[tuple] = set()
        _vinh_super = ci.super_class
        while _vinh_super and _vinh_super != _OBJECT_CLASS and _vinh_super in registry:
            _vinh_is_user = '/' not in _vinh_super   # 链模式逐祖先判定
            _vinh_sci = registry.get(_vinh_super)
            if _vinh_sci is None:
                break
            for _vm in _vinh_sci.methods:
                # 桥方法（ACC_BRIDGE）是槽位覆盖的语义载体：javac 为两类覆盖合成
                # 「与祖先槽位精确同签名」的桥——
                #   - 泛型父类/接口的参数位擦除覆盖（put(Object) → put(String)）
                #   - 协变返回覆盖（value()Number → value()Integer）
                # 桥体（checkcast / 转发 + 返回位上转）翻译后落槽，替代按本类视角
                # 代入的存根（签名与擦除槽位不符，E0053）或描述符不匹配的漏判。
                _vm_bridge = None
                if (not _vm.is_static and not _vm.is_constructor
                        and _vm.name not in ('<init>', '<clinit>')):
                    _has_exact = any(m.name == _vm.name and m.descriptor == _vm.descriptor
                                     for m in visible_methods)
                    if not _has_exact:
                        _vm_bridge = next((b for b in ci.methods
                                           if (b.access_flags & 0x0040) and not b.is_static
                                           and b.name == _vm.name
                                           and b.descriptor == _vm.descriptor), None)
                if (_vm.name, _vinh_param_part(_vm.descriptor)) in _vinh_existing \
                        and (_vm_bridge is None
                             or (_vm.name, _vinh_param_part(_vm.descriptor)) in _vinh_done):
                    continue
                if _vm.is_static or _vm.is_constructor or _vm.name in ('<init>', '<clinit>'):
                    continue
                _vm_in_cc = (
                    call_chain is None or
                    (ci.name, _vm.name, _vm.descriptor) in call_chain or
                    (_vinh_super, _vm.name, _vm.descriptor) in call_chain
                )
                if not _vinh_is_user and not _vm_in_cc:
                    continue  # JDK 链：调用链未收录的祖先虚方法不登记（无体可转发，
                    # 留空槽位 = 声明类 trait default，与既有行为一致）
                _vinh_existing.add((_vm.name, _vinh_param_part(_vm.descriptor)))
                if (_vm.name, _vinh_param_part(_vm.descriptor)) in _vinh_done:
                    continue
                _vinh_done.add((_vm.name, _vinh_param_part(_vm.descriptor)))
                if not _vinh_is_user:
                    # JDK 链：槽位填补经继承成员转发（inherited_gen 第二阶段生成，
                    # 桥成员槽位由其 _bridge_override_member 机制处理——virtual_in 归
                    # 真实声明链）。登记键 = (接收者, Java 方法名, 参数描述符部分)，
                    # 与调用点触发的需求同一账本（去重 / bridge 优先级 / 导入全部复用）
                    if not _vm.is_abstract:
                        _anc_hand = ((new_format_map or {}).get(_vinh_super) or {}).get('methods', ())
                        _hand_hit = (safe_ident(_vm.name) in _anc_hand
                                     or safe_ident(mangle_name(_vm.name, _vm.descriptor)) in _anc_hand)
                        if not _vm.is_native or _hand_hit:
                            _inherited_calls.request(
                                ci.name, _vm.name, _vm.descriptor.split(')', 1)[0] + ')')
                    continue
                # 祖先虚方法自身的槽位归属同样经协变模型解析（K-6a：祖先的协变覆盖
                # 在祖先文件里归父槽位，本类继承展开须填同一个槽）
                _vm_virt_in = resolve_virtual_slot(_vm, _vinh_sci, registry, new_format_map)
                if not _vm_virt_in:
                    continue  # 非虚方法，不继承
                if _vm_bridge is not None:
                    # 槽位是否已由本类可见覆盖经协变落位填充（返回位 Object 化可表达的
                    # 协变覆盖，如 ClassCache$1.computeValue → ClassValue 槽）：已填则
                    # 桥再发射会与真实覆盖在 vtable impl 重复（E0201），跳过
                    _slot_filled = any(
                        (not m.is_static and not m.is_constructor and m.name == _vm.name
                         and resolve_virtual_slot(m, ci, registry, new_format_map) == _vm_virt_in)
                        for m in visible_methods)
                    if _slot_filled:
                        continue
                _vm2 = _copy3.copy(_vm)
                _vm2.class_name = ci.name
                _vm2.virtual_in = _vm_virt_in
                _vm_attr = _java_method_attr(_vm2)
                if _vm_bridge is not None and _vm_in_cc and not stub_bodies:
                    # 桥 wrapper 名：与可见方法同名即 mangle（可见方法与桥常仅返回位不同，
                    # 重载判定可能漏计桥自身）；同名同参（仅返回不同，参数位 mangle 无法
                    # 区分）→ 沿 Iface_super_m 命名约定取唯一名，槽位名经 vtable_name 传递
                    _vb_base = _vm_bridge.name
                    if any(m.name == _vm_bridge.name for m in visible_methods):
                        _same_params = any(
                            m.name == _vm_bridge.name
                            and _vinh_param_part(m.descriptor) == _vinh_param_part(_vm_bridge.descriptor)
                            for m in visible_methods)
                        if _same_params:
                            from ..instr.member_owner import (
                                interface_special_member_name as _ismn_vb,
                            )
                            from ..instr.hierarchy import _rust_type_to_binary as _rtb_vb
                            _vb_owner_bin = (_rtb_vb(_vm_virt_in, registry) or _vinh_super)
                            _vb_base = safe_ident(_ismn_vb(
                                _vb_owner_bin, _vm_bridge.name, _vm_bridge.descriptor, registry))
                        else:
                            _vb_base = mangle_name(_vm_bridge.name, _vm_bridge.descriptor)
                    _vb2 = _copy3.copy(_vm_bridge)
                    _vb2.class_name = ci.name
                    _vb2.virtual_in = _vm_virt_in
                    if _vm_virt_in and _vm_virt_in != short_cls(ci.name):
                        _slot_name = slot_member_rust_name(_vm2, ci, registry, new_format_map)
                        if _slot_name and _slot_name != _vb_base:
                            _vb2.vtable_name = _slot_name
                        _vb2.vtable_erasure = _override_vtable_erasure(_vm2, ci, registry)
                    try:
                        _vb_body = gen_method_body(
                            _vm_bridge, ci, registry=registry,
                            class_type_params=class_type_params,
                            overloaded_names=overloaded_names,
                            rust_name=_vb_base,
                            in_vtable_body=True,
                        )
                        method_blocks.append(_java_method_attr(_vb2) + '\n' + _vb_body)
                        continue
                    except CfgAuditError:
                        raise
                    except Exception as e:
                        # 桥体翻译失败：本类已有同参可见覆盖（协变/参数位）时回退旧
                        # 行为（跳过——槽位由既有机制处理），避免与真实覆盖重复定义；
                        # 无可见覆盖（抽象祖先参数位）时保持存根
                        _CFG_STATS.record_stub_fallback(f"{ci.name}.{_vm.name}:{_vm.descriptor}(bridge)", repr(e))
                        if any(m.name == _vm.name
                               and _vinh_param_part(m.descriptor) == _vinh_param_part(_vm.descriptor)
                               for m in visible_methods):
                            continue
                if _vm.is_native or _vm.is_abstract or not _vm_in_cc or stub_bodies:
                    _vm_stub = _gen_native_stub(_vm2, ci, registry=registry, class_type_params=class_type_params)
                    method_blocks.append(_vm_attr + '\n' + _vm_stub)
                else:
                    try:
                        _vm_body = gen_method_body(
                            _vm2, ci, registry=registry,
                            class_type_params=class_type_params,
                            overloaded_names=overloaded_names,
                            in_vtable_body=True,
                        )
                        method_blocks.append(_vm_attr + '\n' + _vm_body)
                    except CfgAuditError:
                        raise
                    except Exception as e:
                        _CFG_STATS.record_stub_fallback(f"{ci.name}.{_vm.name}:{_vm.descriptor}", repr(e))
                        _vm_stub = _gen_native_stub(_vm2, ci, registry=registry, class_type_params=class_type_params)
                        method_blocks.append(_vm_attr + '\n' + _vm_stub)
            _vinh_super = _vinh_sci.super_class if _vinh_sci.super_class else None


def _patch_record_method_blocks(ci, registry, struct_name, struct_generic,
                                method_blocks) -> list[str]:
    """Record 类（super_class == java/lang/Record）：覆盖 invokedynamic 无法翻译的
    toString / hashCode / equals 为字段级实现。原 _gen_class_rs 内联段逐字
    搬移；原 method_blocks 重绑定改为返回新列表。"""
    # Record 类（super_class == java/lang/Record）：覆盖 invokedynamic 无法翻译的方法
    if ci.super_class == 'java/lang/Record' and not ci.is_interface:
        record_fields = [f for f in ci.fields if not f.is_static]
        simple_name = ci.name.split('$')[-1].split('/')[-1]
        fmt_parts = [f'{f.name}={{}}' for f in record_fields]
        fmt_str = f'{simple_name}[{", ".join(fmt_parts)}]'
        # 字段一律经宏生成的访问器读取（struct 字段的存储形态是宏的实现细节）
        # 基本类型 / 字符串分量直接格式化；其余引用分量按 Java 语义经 Object.toString() 取文本
        def _record_component_text(_rf) -> str:
            _rf_ty = jvm_to_rust(_rf.descriptor, registry)
            _rf_get = f'this.__get_{safe_ident(_rf.name)}()'
            if _rf_ty in _PRIMITIVE_RUST_TYPES or _rf_ty == jvm_to_rust(f'L{STRING_CLASS};', registry):
                return _rf_get
            return f'Into::<Object>::into({_rf_get}).toString()?'
        # 方法体与字节码翻译的方法同一约定：`let this = self;` + 字段访问器
        # （宏把虚方法体搬进 `this: &__BT` 的自由函数，其中不存在 `self` / `Self`）
        fmt_args = ', '.join(_record_component_text(f) for f in record_fields)
        _record_ty = f'{struct_name}{struct_generic}'
        new_blocks = []
        for block in method_blocks:
            if '/* TODO: invokedynamic' in block and any(f'pub fn {n}(' in block for n in ('toString', 'hashCode', 'equals')):
                if 'pub fn toString(' in block:
                    attr = block[:block.index('pub fn toString(')]
                    fmtcall = (f'String::from(format!("{fmt_str}", {fmt_args}).as_str())' if fmt_args
                               else f'String::from("{fmt_str}")')
                    new_blocks.append(attr +
                        f'pub fn toString(&self) -> Result<String> {{\n    let this = self;\n    Ok({fmtcall})\n}}')
                elif 'pub fn hashCode(' in block:
                    # [equiv-audit] record-hash（S-7）：record hashCode 发射
                    # Ok(0)（ObjectMethods 的 31 多项式未实现）——每个 record
                    # 类计 1，只计数不改发射
                    equiv_audit.record('record-hash')
                    attr = block[:block.index('pub fn hashCode(')]
                    new_blocks.append(attr +
                        f'pub fn hashCode(&self) -> Result<i32> {{\n    Ok(0)\n}}')
                elif 'pub fn equals(' in block:
                    attr = block[:block.index('pub fn equals(')]
                    from ..jvm_type import Primitive as _JvmPrimitive, ClassRef as _JvmClassRef
                    from ..jvm_type import from_descriptor as _jvm_from_desc
                    field_cmps = []
                    for f in record_fields:
                        fname = safe_ident(f.name)
                        # 分量比较形态经 jvm_type 代数判定（规则六：类型决策走类型对象）：
                        # 基本分量 == ；String 分量值等（to_string() ==，与既有路径一致）；
                        # 其余引用分量（含泛型 T，擦除描述符 Ljava/lang/Object;）按 javac
                        # 字节码语义走擦除 Object.equals 虚分派（运行时类覆盖优先，
                        # Box<Integer> 的 Integer.equals 即值等）
                        _f_ty = _jvm_from_desc(f.descriptor)
                        if isinstance(_f_ty, _JvmPrimitive):
                            field_cmps.append(f'this.__get_{fname}() == other.__get_{fname}()')
                        elif isinstance(_f_ty, _JvmClassRef) and _f_ty.binary == STRING_CLASS:
                            field_cmps.append(
                                f'this.__get_{fname}().to_string() == other.__get_{fname}().to_string()'
                            )
                        else:
                            field_cmps.append(
                                f'Into::<Object>::into(this.__get_{fname}())'
                                f'.equals(Into::<Object>::into(other.__get_{fname}()))?'
                            )
                    cmp_expr = ' && '.join(field_cmps) if field_cmps else 'true'
                    new_blocks.append(attr +
                        f'pub fn equals(&self, mut o: Object) -> Result<bool> {{\n'
                        f'    let this = self;\n'
                        f'    if !o.is_instance_of("{ci.name}") {{\n'
                        f'        return Ok(false);\n'
                        f'    }}\n'
                        f'    let other = Into::<{_record_ty}>::into(Clone::clone(&o));\n'
                        f'    Ok({cmp_expr})\n'
                        f'}}')
                else:
                    new_blocks.append(block)
            else:
                new_blocks.append(block)
        method_blocks = new_blocks
    return method_blocks


def _gen_class_rs(ci: ClassInfo, registry: dict | None = None,
                  jdk_crate_pkg_paths: list[str] | None = None,
                  stub_bodies: bool = False,
                  call_chain: set | None = None,
                  new_format_map: dict | None = None,
                  workspace_root: str | None = None,
                  user_crate_prefix: str | None = None,
                  full_impl_classes: set | None = None,
                  conflict_map: dict | None = None,
                  skipped_classes: set | None = None,
                  user_sibling_imports: list[str] | None = None,
                  generated_classes: set | None = None,
                  emission: 'ClassEmission | None' = None) -> str:
    """生成单个 Java 类对应的完整 .rs 文件内容。

    生成规则：
    - 实例字段用 Field<T> 包装（提供 Java 字段语义的内部可变性）
    - 方法直接在 impl 块中，无 raw:: 子模块
    - 所有方法返回 Result<T>
    - 每个 struct / field / method 前加 // @java_* 注释供 build.rs 扫描
    - new_format_map: 若提供，为覆盖的类插入 #[path] mod _impl; 并跳过被覆盖方法
    - user_crate_prefix: 若提供（如 'jdk_classes'），cross_imports 用该 crate 前缀
    - emission: 若提供，记录本类实际生成的方法声明，并在文本中留出继承成员声明的
      两个插入位（use 区 / impl 块尾），由 inherited_gen.resolve_inherited_members 统一填充
    """

    # ── Step 1+2: 引用收集与精确 cross_imports（import_gen）─────────────────
    _referenced = collect_referenced(ci, registry, generated_classes)
    cross_imports = gen_cross_imports(
        ci, registry, jdk_crate_pkg_paths, call_chain, generated_classes,
        conflict_map, skipped_classes, user_sibling_imports,
        user_crate_prefix, _referenced)

    # 全量手写类（native_impl 文件含 pub struct）：codegen 跳过 struct 生成，改输出 pub use _impl::*
    _full_impl = ci.name in (full_impl_classes or set())
    if not _full_impl:
        # G-10 账本：本类方法由本轮生成（手写全量类的方法不走 class_writer，不参与断言）
        LAMBDA_NAME_LEDGER.generated_classes.add(ci.name)

    # 接口在 Rust 层是与 Java 同名的载体类型（由 java_class 宏展开）：
    #   - 值：持有 Object 的接口引用，实例方法分派走 Object vtable（impl 块不含实例方法；
    #     default 方法由实现类继承展开，见下方「接口 default 方法继承」）
    #   - 命名空间：static 字段访问器 / static 方法（含 private static 与 static 合成方法）
    #     落在载体的 impl 块，调用点与 Java 同构（`Iface::staticMethod(args)`）
    _is_iface = bool(ci.is_interface)

    parts: list[str] = [
        "#![allow(unused_variables, unused_mut, dead_code, non_snake_case, unused_imports, non_camel_case_types, static_mut_refs)]",
        f"use {user_crate_prefix or 'crate'}::prelude::*;",
        *cross_imports,
        *([_INHERITED_IMPORTS_SLOT] if emission is not None else []),
        "",
    ]
    inst_fields = [f for f in ci.fields if not f.is_static]

    # Option C（方案 §16「_super 语义边界」）：`_super` 是父类状态的**唯一所有者**，
    # 子类通过宏生成的转发访问器获得「展平字段视图」。
    # 不把父类字段复制进子类 Inner——那会让同一字段存在两份状态并立即分叉。
    _has_super = bool(
        ci.super_class and ci.super_class != _OBJECT_CLASS
    )

    # 用短名作为 Rust 标识符（JDK 类含 /，内部类含 $，均需转换为合法 Rust 名）
    struct_name = short_cls(ci.name) if ('/' in ci.name or '$' in ci.name) else ci.name

    # 类的有效类型参数：自身 Signature 形参；内部类无自身形参时继承外部类形参
    # （Java 内部类经 this$N 隐式可见外部类类型变量，如 ArrayList$Itr → ArrayList_Itr<E>）。
    class_type_params = effective_class_type_params(ci, registry)

    # 构建泛型参数字符串（用于 struct 和 impl 头）
    if class_type_params:
        type_params_str = ', '.join(class_type_params)
        # 泛型参数需要 Clone + Default + 'static。
        # 这组 bound 是纯粹的 Rust 能力声明——故意不引入 `JavaType` 这类
        # 游离于 Java 命名空间之外的 trait 名（CLAUDE.md 命名原则，方案 §4）。
        # Default 是必须的：`_super: Default::default()` 与字段默认值都要求它。
        bounds_str = ', '.join(f"{p}: Clone + Default + 'static" for p in class_type_params)
        struct_generic = f"<{type_params_str}>"   # bounds 由 java_class! 宏展开时注入，不出现在生成代码
        ty_params_only = f"<{type_params_str}>"
        # impl 头只写裸参数：块级宏用 struct 上的 generics（含补齐的 bound 与 where 子句）
        # 重新生成 impl 头，这里的 impl generics 仅作读者提示。
        impl_header   = f"impl<{type_params_str}> {struct_name}<{type_params_str}>"
    else:
        struct_generic = ''
        ty_params_only = ''
        impl_header   = f"impl {struct_name}"

    # 构建注册表短名集合，用于校验字段类型中引用的类是否存在
    _registry_short_names: set[str] = (
        {_short_cls_g(_k) for _k in registry}
        if registry else set()
    )

    # ── 父类 Rust 类型（含泛型实参）─────────────────────────────────────
    # 实参取自本类 Signature 的 SuperclassSignature，整条祖先链逐级代入
    # （见 type_map.ancestor_type_args），不做「子类形参按位置套给父类」的猜测。
    parent_rust = ''
    _ancestor_args: dict[str, list[str]] = (
        dict(ancestor_type_args(ci, registry)) if _has_super and registry else {})
    if _has_super:
        parent_rust = _rust_type_with_args(short_cls(ci.super_class),
                                           _ancestor_args.get(ci.super_class, []))


    # ── 继承链字段展平（方案 §6）────────────────────────────────────────
    # codegen 侧展平整条继承链，父类字段在前；宏侧零 registry 依赖。
    # 注意：展平结果只用于生成「转发访问器」，父类字段的实际存储在 `_super` 里
    # （见 §16：复制字段会造成同一字段两份状态）。
    superclass_fields: list[tuple[str, str]] = []
    superclass_reference_fields: list[str] = []
    superclass_erased_fields: list[str] = []
    if _has_super and registry and not _full_impl:
        _chain: list = []
        _seen_chain: set[str] = set()
        _cursor = ci.super_class
        while (_cursor and _cursor != _OBJECT_CLASS
               and _cursor in registry and _cursor not in _seen_chain):
            _seen_chain.add(_cursor)
            _p_ci = registry[_cursor]
            _chain.append(_p_ci)
            _cursor = _p_ci.super_class
        _declared: set[str] = set()
        for _ancestor in reversed(_chain):
            # 祖先形参 → 本类视角实参（沿 SuperclassSignature 逐级代入的结果）
            _anc_params = effective_class_type_params(_ancestor, registry)
            _sub_args = list(_ancestor_args.get(_ancestor.name, []))
            while len(_sub_args) < len(_anc_params):
                _sub_args.append('Object')
            _anc_map = dict(zip(_anc_params, _sub_args))
            for _f in _ancestor.fields:
                if _f.is_static:
                    continue
                # 隐藏更上层祖先同名字段的声明取独立槽位名（Java 字段按声明类静态解析）
                _sf_name = instance_field_rust_name(
                    _ancestor.name, _safe_field_name(_f.name), registry)
                if _sf_name in _declared:
                    continue
                _declared.add(_sf_name)
                _sf_view_ty = _resolve_anc_field_rust(
                    _f, _anc_params, _anc_map, registry, _registry_short_names)
                superclass_fields.append((_sf_name, _sf_view_ty))
                # 祖先按类型变量声明（引用存储 + __borrow_mut 访问器）而本类视角代入成基本类型
                # （Box<U>.state 在 `extends Box<Long>` 下是 i64）：存储形态由声明方决定，
                # 宏须按引用字段实现祖先 VTable 的访问器
                if (_sf_view_ty in _PRIMITIVE_RUST_TYPES and _resolve_anc_field_rust(
                        _f, _anc_params, {_p: _p for _p in _anc_params},
                        registry, _registry_short_names) not in _PRIMITIVE_RUST_TYPES):
                    superclass_reference_fields.append(_sf_name)
                # 声明方（祖先）按自身类型形参声明的字段 → 存储与访问器已被声明方的宏
                # Object 化（A-1 擦除按声明类判定）——继承者的宏按名单同步擦除
                _declared_ty = _resolve_anc_field_rust(
                    _f, _anc_params, {_p: _p for _p in _anc_params},
                    registry, _registry_short_names)
                if any(_re.search(r'\b' + _re.escape(_p) + r'\b', _declared_ty)
                       for _p in _anc_params):
                    superclass_erased_fields.append(_sf_name)

    # ── struct 声明（裸类型，封装细节由宏收拢）──────────────────────────
    struct_lines: list[str] = []
    if not _full_impl:
        # 父类已有的字段名（继承展平），避免子类重复声明（如内部类 this$0 与父类同名）
        _super_field_names: set[str] = {name for name, _ in superclass_fields}
        for f in inst_fields:
            safe_fname = instance_field_rust_name(ci.name, _safe_field_name(f.name), registry)
            if safe_fname in _super_field_names:
                continue  # 父类已展平，不重复声明
            struct_lines.append("    " + _java_field_attr(f))
            struct_lines.append(
                f"    pub {safe_fname}: "
                f'{_resolve_field_rust(f, class_type_params, registry, _registry_short_names)},')
        # 未被字段引用的类型参数由宏补 PhantomData（block.rs），codegen 不再输出 _phantom


    # T76 的 as_xxx / into_xxx upcast 方法已由 java_class! 宏统一承接
    # （宏生成 `__super()` / `__into_super()`，upcast 与字段存储解耦，见方案 §16）。

    # T55 已删除（R-2）：
    # From<Child> for Parent 链由 invoke.py / sim.py 的显式 __into_super() 链替代，
    # 宏为有父类的类生成 Deref<Target=Parent> 覆盖引用层面的向上转型。
    # upcast 调用点：Clone::clone(&child).__into_super().__into_super()...（见 coerce._into_super_chain）

    # T55b 已删除（Arch-5）：
    # Arch-1 后接口 = Object 类型别名，From<ConcreteClass> for Interface 语义上等于
    # From<ConcreteClass> for Object，与 java_class 宏生成的 Into<Object> 冲突且
    # 会用 Default::default() 丢弃具体数据。
    # 正确路径：ConcreteClass.into() → Object，通过 java_class 宏生成的 Into<Object> 完成。
    # K-2: 共置 _impl.rs 文件由 project_writer 在生成阶段复制；class_writer 不再生成 #[path] 块。
    _nf_entry = (new_format_map or {}).get(ci.name)

    # 过滤 synthetic 方法（编译器合成桥接方法），再统计重载
    visible_methods = [m for m in ci.methods if not m.is_synthetic]
    # 重载判定在整条父类链上进行（与调用侧 _mangle_if_overloaded 共用同一函数），
    # 保证子类方法名不会按名字遮蔽父类的同名异参方法。
    overloaded_names: set[str] = hierarchy_overloaded_names(ci, registry)

    method_blocks: list[str] = []
    # G-10：接口私有实例 lambda body 落在 java_class! 块之外的擦除 impl 块
    # （不进接口 vtable / 不被实现类继承），见方法循环内的专门分支
    _iface_lambda_blocks: list[str] = []

    # 是否是用户类（call_chain is None 表示用户类，所有方法都翻译）
    _is_user_class = call_chain is None

    # ── static 字段声明（JVMS §5.5 类初始化的事实层）→ clinit_extract ──────
    _type_only = stub_bodies or (
        call_chain is not None
        and not any((ci.name, _m.name, _m.descriptor) in call_chain for _m in ci.methods))
    method_blocks.extend(_gen_static_field_blocks(
        ci, registry, class_type_params, _type_only, _nf_entry,
        _registry_short_names))
    # 方法翻译主循环（可见方法 + 非桥接 synthetic；<clinit> / 接口 lambda /
    # 接口声明 / 虚方法体发射，见 _emit_method_blocks）
    _emit_method_blocks(
        ci, registry, call_chain=call_chain, stub_bodies=stub_bodies,
        new_format_map=new_format_map, _nf_entry=_nf_entry,
        class_type_params=class_type_params, overloaded_names=overloaded_names,
        visible_methods=visible_methods, _type_only=_type_only,
        _is_iface=_is_iface, method_blocks=method_blocks,
        _iface_lambda_blocks=_iface_lambda_blocks)

    # 已翻译方法体的接口 default 方法（展开到本类）：其字节码同样可能含 `Iface.super.m()`
    _translated_defaults: list = []
    _emit_interface_default_inheritance(
        ci, registry, call_chain=call_chain, stub_bodies=stub_bodies,
        class_type_params=class_type_params, overloaded_names=overloaded_names,
        visible_methods=visible_methods, method_blocks=method_blocks,
        _translated_defaults=_translated_defaults)
    _emit_interface_special_members(
        ci, registry, call_chain=call_chain, stub_bodies=stub_bodies,
        class_type_params=class_type_params, overloaded_names=overloaded_names,
        visible_methods=visible_methods, method_blocks=method_blocks,
        _translated_defaults=_translated_defaults)
    _emit_superclass_virtual_inheritance(
        ci, registry, call_chain=call_chain, stub_bodies=stub_bodies,
        new_format_map=new_format_map,
        class_type_params=class_type_params, overloaded_names=overloaded_names,
        visible_methods=visible_methods, method_blocks=method_blocks)

    # Record 类：覆盖 invokedynamic 无法翻译的方法（_patch_record_method_blocks）
    method_blocks = _patch_record_method_blocks(
        ci, registry, struct_name, struct_generic, method_blocks)

    # ── 组装 java_class! { ... } 块（方案 §3 核心设计）────────────────────
    if not _full_impl:
        block: list[str] = []
        block.extend(_java_class_block_head(
            ci, registry=registry,
            superclass_rust=parent_rust,
            superclass_fields=superclass_fields,
            superclass_reference_fields=superclass_reference_fields,
            superclass_erased_fields=superclass_erased_fields,
            impl_methods=set((_nf_entry or {}).get('methods', set())),
            handwritten_methods=new_format_map,
        ))
        block.append('')
        # struct 声明：裸类型（封装细节收拢进宏），无 derive / 无 _super / 无 _phantom
        if struct_lines:
            block.append(f"pub struct {struct_name}{struct_generic} {{")
            block.extend(struct_lines)
            block.append("}")
        else:
            block.append(f"pub struct {struct_name}{struct_generic};")

        # 接口无成员且不可能补继承成员（无 emission）时不写空 impl 块
        if method_blocks or not _is_iface or emission is not None:
            block.append('')
            impl_body = '\n\n'.join(_indent(b) for b in method_blocks)
            block.append(f"{impl_header} {{")
            if impl_body:
                block.append(impl_body)
            if emission is not None:
                block.append(_INHERITED_MEMBERS_SLOT)
            block.append("}")
        if emission is not None and not _is_iface:
            block.append(_INTERFACE_IMPLS_SLOT)

        if emission is not None:
            emission.record_methods(method_blocks)
        parts.append("java_rta_macros::java_class! {")
        for line in block:
            parts.append(_indent(line) if line else '')
        parts.append("}")
        if emission is not None:
            # A-4 协变 upcast 占位（interface_gen 填充：impl From<Class> for Iface<Object>，
            # 载体构造依赖其私有字段，必须经载体的 From<Object>，不属于宏输入）
            parts.append(_INTERFACE_UPCASTS_SLOT)
        parts.append('')

        # G-10：接口私有实例 lambda body 的擦除固有 impl 块。置于 java_class! 块之外，
        # 避免被宏归入接口 vtable / 载体分派（lambda 体不是接口契约，只被 invokedynamic
        # 调用点按名引用）。this 与调用点接收者同为 Iface<Object> 擦除实例化。
        if _iface_lambda_blocks:
            _lam_recv = (f"{struct_name}<{', '.join(['Object'] * len(class_type_params))}>"
                         if class_type_params else struct_name)
            parts.append('// G-10: 接口私有实例 lambda body —— 载体擦除实例化上的固有方法')
            parts.append(f'impl {_lam_recv} {{')
            for _lb in _iface_lambda_blocks:
                parts.append(_indent(_lb))
            parts.append('}')
            parts.append('')

    # 扫描方法体中使用的 VTable trait（UFCS 调用 XxxVTable::method(...)），
    # 为未导入的 VTable 类型补充 use 语句（避免 E0433）。不盲目为所有类添加
    # __VTable（手写类如 Object/String 不一定有），而是按实际生成代码中
    # 出现的名称按需导入（import_gen.scan_used_vtable_imports）。
    _prefix = user_crate_prefix or 'crate'
    parts.extend(scan_used_vtable_imports(
        method_blocks, _iface_lambda_blocks, cross_imports,
        struct_name, registry, _prefix))

    # BINARY_NAME / ObjectVTable / Into<Object> / From<Object> / Debug 全部由
    # java_class! 宏在编译期展开（方案 §11 职责边界总表）。
    # （Object 类走手写路径 java_runtime/，不经过此函数）

    return '\n'.join(parts)