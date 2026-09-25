# 从 codegen/instr/invoke.py 中拆出；接收者/泛型实参解析段与 _declaring_interface/
# _close_open_type_args 已按 §3.7 上移 member_owner.py
from ..type_map import short_cls as _short_cls_g
from ..stack import StackSim
from .. import inherited_calls as _inherited_calls
from ..rs_ir import Lit, Var, RawExpr, RawStmt, RsNamed
from ..render import render_expr, render_type
from ..type_map import jvm_to_rust, short_cls
from ..constants import safe_ident as _safe_field
from .. import equiv_audit
from ..constants import (
    PRIMITIVE_RUST_TYPES as _PRIMITIVE_RUST_TYPES,
    JAVA_RUNTIME_SHORT_NAMES as _JAVA_RUNTIME_SHORT_NAMES,
    OBJECT_CLASS as _OBJECT_CLASS,
)
from .coerce import _coerce_to_object, _render_cast
from .hierarchy import _rust_type_to_binary
from .member_owner import (
    parse_method_ref,
    _resolve_method_owner, _root_virtual_methods, _root_protected_void_methods,
    _declaring_interface, _close_open_type_args,
    _resolve_virtual_sig_params,
    private_interface_method_target as _private_iface_target,
)
from .member_naming import (
    _mangle_if_overloaded, _resolve_bridge_target, _handwritten_root_api,
)
from ..type_args import ancestor_vtable_args_by_short as _ancestor_vtable_args_by_short
from ..type_args import (ancestor_type_args as _ancestor_type_args,
                         substitute_type_params as _substitute_type_params)
from ..type_map import effective_class_type_params as _effective_class_type_params
from .invoke_sig import (_lookup_method_sig_ret, _erased_ret_is_type_var,
                         _coerce_arg)


def _pop_receiver_and_args(sim, params, sig_params_v, registry):
    """弹出并 coerce 全部实参与接收者；构造结果直接作接收者时，待推断的 `_`
    类型实参按擦除语义闭合（接收者位置不提供推断上下文，E0283）。"""
    args = []
    for _idx_v, param_jvm in enumerate(reversed(params)):
        e_expr, e_ty_node = sim.pop()
        e_str = render_expr(e_expr)
        _pi_v = len(params) - 1 - _idx_v
        _sig_t_v = sig_params_v[_pi_v] if sig_params_v and _pi_v < len(sig_params_v) else None
        expected_rust = _sig_t_v if _sig_t_v is not None else jvm_to_rust(param_jvm, registry)
        actual_rust = render_type(e_ty_node)
        e_str = _coerce_arg(e_str, e_ty_node, expected_rust, actual_rust, sim, registry)
        args.insert(0, e_str)
    obj_expr, obj_ty_node = sim.pop()
    obj_e = render_expr(obj_expr)
    obj_ty = render_type(obj_ty_node)

    # `new Foo<>(..).m()`：构造结果直接作接收者。接收者位置不提供任何类型推断上下文，
    # 构造器 turbofish 里待推断的 `_` 永远无解（E0283）→ 按擦除语义落为 Object。
    # 已绑定到局部变量的构造结果不在此列（后续赋值 / 传参仍可提供推断）。
    import re as _re_infer
    _infer_m = _re_infer.match(r'^(\w+)<(_(?:, _)*)>$', obj_ty)
    if _infer_m and obj_e.startswith(f"{_infer_m.group(1)}::<{_infer_m.group(2)}>::"):
        _erased_args = ', '.join('Object' for _ in _infer_m.group(2).split(', '))
        obj_e = (f"{_infer_m.group(1)}::<{_erased_args}>::"
                 + obj_e[len(f"{_infer_m.group(1)}::<{_infer_m.group(2)}>::"):])
        obj_ty = f"{_infer_m.group(1)}<{_erased_args}>"
        obj_expr = RawExpr(obj_e)
        obj_ty_node = RsNamed(obj_ty)
    return args, obj_e, obj_ty


def _try_early_receiver_paths(sim, _obj_is_typevar, mname, args, obj_e, obj_ty,
                              params, ret, registry) -> bool:
    """四类特殊接收者路径（装箱类型变量的 Object 手写直调 / 基本类型 equals 的
    == 比较 / 基本类型接收者的根类方法 / 数组 getClass），命中即发射并返回 True。"""
    if _obj_is_typevar and mname in ('equals', 'hashCode', 'toString'):
        # [equiv-audit] identity-hash（S-6）：类型变量接收者装箱后直调 Object
        # 手写实现——hashCode 未被运行时类覆盖时落到默认（非 identity）实现
        if mname == 'hashCode':
            equiv_audit.record('identity-hash')
        _rust_ret_eq = jvm_to_rust(ret, registry)
        _arg_str_eq = ', '.join(args)
        if _rust_ret_eq == '()':
            sim.emit(RawStmt(f"{obj_e}.{mname}({_arg_str_eq})?;"))
        else:
            _v_eq = sim.fresh()
            sim.emit(RawStmt(f"let {_v_eq}: {_rust_ret_eq} = {obj_e}.{mname}({_arg_str_eq})?;"))
            sim.push(Var(_v_eq), RsNamed(_rust_ret_eq))
        return True

    # 基本类型 .equals(x) → 生成 == 比较（基本类型无 equals 方法）
    if mname == 'equals' and len(args) == 1 and obj_ty in _PRIMITIVE_RUST_TYPES:
        raw_arg = args[0].removesuffix('.into()')
        v = sim.fresh()
        sim.emit(RawStmt(f"let {v}: bool = ({obj_e} == {raw_arg});"))
        sim.push(Var(v), RsNamed('bool'))
        return True

    # 基本类型接收者（包装类映射为 Rust 基本类型）调用根类声明的方法：
    # 基本类型上没有 Java 方法 → 装箱为 Object 后走根 vtable
    if (obj_ty in _PRIMITIVE_RUST_TYPES
            and (mname, f"({''.join(params)})") in _root_virtual_methods()):
        _rust_ret_p = jvm_to_rust(ret, registry)
        _call_p = f"Object::from_any({obj_e}).{_safe_field(mname)}({', '.join(args)})?"
        if _rust_ret_p == '()':
            sim.emit(RawStmt(f"{_call_p};"))
        else:
            _v_p = sim.fresh()
            sim.emit(RawStmt(f"let {_v_p}: {_rust_ret_p} = {_call_p};"))
            sim.push(Var(_v_p), RsNamed(_rust_ret_p))
        return True

    # JVM 数组.getClass() → Object::from(数组).0.getClass()（动态分派）。
    # 数组类由创建时的元素类型决定（JLS §10.8），接收者的静态元素类型
    # （协变上转后是祖先）拿不到运行时数组名——JArray 的 ObjectVTable::
    # getClass 对协变视图委托源数组，`String[]` 以 `Object[]` 形态流转时
    # 仍取 `[Ljava.lang.String;`。Class 一律经 for_class 的同一缓存条目，
    # `== Object[].class`（ldc 类字面量）身份比较成立——Arrays.copyOf /
    # copyOfRange 的同型判定不受影响（异型数组按 JVM 语义不再误判为
    # Object[]）。
    if mname == 'getClass':
        # TypeIR 批次 3（V1）：数组接收者判定走类型对象（Array 变体），
        # 替代 obj_ty 的数组前缀文本形态探测
        from ..jvm_type import from_rust_type, Array as _ArrayT
        if isinstance(from_rust_type(obj_ty, registry), _ArrayT):
            # [equiv-audit] class-literal（S-5）：数组 getClass 的发射早路径
            #（动态分派后数组侧同一性已精确；计数维持发射点口径）
            equiv_audit.record('class-literal')
            _recv = obj_e[1:] if obj_e.startswith('&') else obj_e
            v = sim.fresh()
            sim.emit(RawStmt(
                f"let {v}: Class = Object::from(Clone::clone(&{_recv})).0.getClass()?;"))
            sim.push(Var(v), RsNamed('Class'))
            return True

    return False



def _emit_object_direct_call(sim, obj_e, args, rust_mname, rust_ret) -> bool:
    """bare Object 接收者 + Object 类方法：仅 void / 基本类型返回值走包装器直调。"""
    # Object 类方法（hashCode/equals/compareTo 等）通过 Object 包装器直接调用。
    # 仅对基本类型返回值或 void 方法走直接调用路径；
    # 非基本类型返回（如 getClass→Class）保持原来的 closure-only dispatch，
    # 因为 object_impl.rs 的实现返回 Object 而非具体类型，不能直接赋值。
    if rust_ret == '()' or rust_ret in _PRIMITIVE_RUST_TYPES:
        # [equiv-audit] identity-hash（S-6）：bare Object 接收者的 hashCode 直调
        # ——经 ObjectVTable 分派，运行时类未覆盖时落到默认（非 identity）实现
        if rust_mname == 'hashCode':
            equiv_audit.record('identity-hash')
        arg_str = ', '.join(args)
        v = sim.fresh()
        if rust_ret == '()':
            sim.emit(RawStmt(f"{obj_e}.{rust_mname}({arg_str})?;"))
        else:
            # 基本类型返回值（hashCode→i32, equals→bool, compareTo→i32 等）
            # 通过 ObjectVTable dyn dispatch 正确路由到实际类型的覆盖实现
            sim.emit(RawStmt(f"let {v}: {rust_ret} = {obj_e}.{rust_mname}({arg_str})?;"))
            sim.push(Var(v), RsNamed(rust_ret))
        return True
    return False


def _dispatch_bare_object(sim, obj_e, cls, mname, comment, params, ret,
                          args, arg_str, rust_ret, registry, rust_mname,
                          sig_params_v):
    """bare Object 接收者的多态分派（§6 步骤 4）：接口经载体 / 根类方法经根
    vtable 单次直调 / 类虚方法经类 vtable 查询（`__virtual_view` 擦除视图重建，
    共享存储与对象标识）后在 wrapper 边界调用目标方法。lambda 是 A-5 合成对象
    （接口方法的载体分支经 `__interface` + vtable 分派，default 同覆盖），
    SAM 闭包回退已移除。所有路径发射后返回。"""
    # 多态 dispatch：cls 是 Rust 短类名（$ 已替换为 _），需转回 binary name
    cls_binary = _rust_type_to_binary(cls, registry) or cls
    # 接口方法：经与接口同名的载体分派（`Into::<I<Object>>::into(obj).m()`）。
    # 载体按擦除后的接口（itable 语义）向对象查询接口 vtable，不依赖对象的类型实参；
    # lambda 对象由函数式接口的唯一抽象方法直接调用。不枚举实现类。
    _iface_ci = registry.get(cls_binary)
    if _iface_ci is not None and _iface_ci.is_interface:
        _iface_desc = f"({''.join(params)}){ret}"
        _decl_bin = _declaring_interface(_iface_ci, mname, _iface_desc, registry)
        if _decl_bin:
            if _decl_bin != cls_binary:
                _inherited_calls.request(cls_binary, mname, f"({''.join(params)})")
            # 名字视角 = 声明接口（接口载体的继承成员按声明接口重载态命名，
            # interface_gen 机制——如 Sink 载体上来自 Consumer 的 accept 裸名）
            _iface_mname = _safe_field(_mangle_if_overloaded(_decl_bin, mname, comment, registry))
            _iface_tps = _effective_class_type_params(_iface_ci, registry)
            _iface_targs = f"<{', '.join(['Object'] * len(_iface_tps))}>" if _iface_tps else ''
            # Into 全限定：接口自身可能声明名为 from 的 Java 静态方法，`Iface::from(..)` 会被其遮蔽
            _iface_call = (f"Into::<{short_cls(cls_binary)}{_iface_targs}>::into(Clone::clone(&{obj_e}))"
                           f".{_iface_mname}({arg_str})?")
            if rust_ret == '()':
                sim.emit(RawStmt(f"{_iface_call};"))
            else:
                v = sim.fresh()
                # 擦除描述符返回 Object 而接口签名给出具体引用类型（Node_OfDouble
                # .asPrimitiveArray → JArray<f64>）：装箱为 Object 记录（返回位
                # Object↔JArray<T>，与 _emit_call_result 的对齐规则同源）。方法可在
                # 接口闭包的祖先接口上声明（OfPrimitive 的 T_ARR），按声明者解析
                _sig_ret_v = _lookup_method_sig_ret(
                    short_cls(_decl_bin), mname, params, ret, registry,
                    caller_class=getattr(sim, 'class_name', ''),
                    caller_tparams=sim.class_type_params,
                    receiver_type=f"{short_cls(cls_binary)}{_iface_targs}",
                )
                if rust_ret == 'Object' and _sig_ret_v is not None and _sig_ret_v != 'Object':
                    sim.emit(RawStmt(
                        f"let {v}: {rust_ret} = {_coerce_to_object(_iface_call, _sig_ret_v, registry, sim.class_type_params)};"))
                else:
                    sim.emit(RawStmt(f"let {v}: {rust_ret} = {_iface_call};"))
                sim.push(Var(v), RsNamed(rust_ret))
            return
    # 根类（Object）声明的方法：单次根 vtable 分派。声明 / 覆盖该方法的类都在
    # 自身的 ObjectVTable impl 上把入口桥接到所属 vtable（struct_layout 的
    # hashCode/equals 桥接 + wrapper 的 hash_code_fwd / to_string_fwd），
    # 按子类枚举的 downcast 链与根调用到达同一实现（JVM 的 vtable 继承条目
    # 语义），链是纯冗余——链尾即全部语义。
    if (mname, f"({''.join(params)})") in _root_virtual_methods():
        _root_call = f"{obj_e}.{rust_mname}({arg_str})?"
        if rust_ret == '()':
            sim.emit(RawStmt(f"{_root_call};"))
        else:
            v = sim.fresh()
            sim.emit(RawStmt(f"let {v}: {rust_ret} = {_root_call};"))
            sim.push(Var(v), RsNamed(rust_ret))
        return
    # 类虚方法：经类 vtable 分派（依赖 A-1 的类 vtable 去形参与 __erased_vtable /
    # __erased_inner 部件导出）。`Cls::<Object, ..>::__virtual_view(&obj)` 按运行时
    # 类查询本类擦除 vtable（子类经 supertrait 上转填充），命中即以原对象的
    # (vtable, 存储) 部件重建本类擦除实例化视图——共享存储与对象标识，与
    # From<Object> 擦除路径同源；未命中（闭包、无运行时类值）回落闭包 SAM 分支。
    _cls_ci = registry.get(cls_binary)
    _cls_rust = jvm_to_rust(f'L{cls_binary};', registry) if _cls_ci is not None else 'Object'
    if _cls_ci is not None and not _cls_ci.is_interface and _cls_rust != 'Object':
        _emit_class_vtable_dispatch(
            sim, obj_e, _cls_ci, cls_binary, _cls_rust, mname, comment, params,
            ret, args, arg_str, rust_ret, registry, sig_params_v)
        return
    # 未翻译类 / 接口残余形：无 wrapper 可建 —— 记默认值占位（既有兜底的终值形态）。
    # SAM 闭包回退已随 A-5 移除：lambda 一律是合成对象（实现接口闭包 __VTable、
    # 经 `__interface` 应答），接口方法的分派在上面的载体分支完成；此处到达的
    # 接收者既无接口载体也无类 vtable，不再有可调用的实现。
    if rust_ret == '()':
        sim.emit(RawStmt(
            f'/* A-5: 未翻译接收者残余 {cls}.{mname} —— 无可分派实现 */'))
    else:
        v = sim.fresh('_vdispatch')
        sim.emit(RawStmt(f"let {v}: {rust_ret} = Default::default();"))
        sim.push(Var(v), RsNamed(rust_ret))
    return


def _erased_view_sig(cls_ci, mname, params, ret, registry):
    """调用目标在接收者擦除实例化（`Cls<Object, ..>` wrapper 视角）下的
    (形参类型列表, 返回类型)。

    沿 cls 超类链找 (mname, 参数描述符) 的最近非 synthetic 声明，取其发射签名
    （emitted_method_sig_types，与定义侧文件逐字同源），把声明者类型变量按
    「接收者全 Object 实参」的祖先实参代入（裸变量 → Object，参数化形态的
    实参位同步 Object 化）。调用描述符只命中 synthetic 桥接时按被桥接的真实
    方法解析（Fix 17 同源）。链上无声明 → (None, None)。"""
    full_desc = '(' + ''.join(params) + ')' + ret
    pdesc = '(' + ''.join(params) + ')'
    from ..sig_types import emitted_method_sig_types

    def _subst_owner(view_ci, m):
        owner_tps = _effective_class_type_params(view_ci, registry)
        sig_ps, sig_r = emitted_method_sig_types(view_ci, m, owner_tps, registry)
        if view_ci.name == cls_ci.name:
            mapping = {p: 'Object' for p in owner_tps}
        else:
            recv_tps = _effective_class_type_params(cls_ci, registry)
            anc_args = dict(_ancestor_type_args(
                cls_ci, registry, ['Object'] * len(recv_tps))).get(view_ci.name, [])
            mapping = {p: (anc_args[i] if i < len(anc_args) else 'Object')
                       for i, p in enumerate(owner_tps)}
        return ([_substitute_type_params(p, mapping) for p in sig_ps],
                _substitute_type_params(sig_r, mapping))

    cur, seen = cls_ci, set()
    while cur is not None and cur.name not in seen and not cur.is_interface:
        seen.add(cur.name)
        m = next((x for x in cur.methods
                  if not x.is_synthetic and x.name == mname
                  and x.descriptor.startswith(pdesc)), None)
        if m is not None:
            return _subst_owner(cur, m)
        cur = registry.get(cur.super_class) if cur.super_class else None
    # 桥接（形参擦除：`compareTo(Object)` → `compareTo(Enum)`）：真实方法的
    # 发射签名按其声明者解析
    _bt = _resolve_bridge_target(cls_ci, mname, full_desc, registry)
    if _bt is not None and not _bt[0].is_interface:
        real_ci, real_desc = _bt[0], _bt[1]
        m = next((x for x in real_ci.methods
                  if not x.is_synthetic and x.name == mname
                  and x.descriptor == real_desc), None)
        if m is not None:
            return _subst_owner(real_ci, m)
    return None, None


def _emit_class_vtable_dispatch(sim, obj_e, cls_ci, cls_binary, cls_rust,
                                mname, comment, params, ret, args, arg_str,
                                rust_ret, registry, sig_params_v):
    """类 vtable 分派的发射体：`__virtual_view` 视图重建 + wrapper 方法调用。
    SAM 闭包回退已随 A-5 移除（lambda 经接口载体分派，不可能是类实例）。
    含继承成员登记（本类 + 闭包子类的槽位填充，S-16）与
    形参 / 返回值的擦除边界对齐。"""
    from ..rs_ir import RsNamed as _RsNamed
    pdesc = '(' + ''.join(params) + ')'
    # 名字视角 = 接收者（Cls 的 wrapper：本类覆盖与继承成员都在其 wrapper 上，
    # 名字按本类重载态）——与 typed 接收者路径（_resolve_direct_call_sig 后的
    # _emit_call_result）同一 mangle 源
    mname_r = _safe_field(_mangle_if_overloaded(cls_binary, mname, comment, registry))
    # 继承成员登记：本类未声明时由 inherited_gen 在本类 wrapper 上补转发成员；
    # 闭包内全部子类逐一登记（未自行声明的子类按链上最近声明者填 vtable 槽，
    # 等价 JVM 子类 vtable 继承条目；与 this 虚调用的登记同一机制）
    _inherited_calls.request(cls_binary, mname, pdesc)
    if _virtually_dispatched(cls_ci, mname, pdesc, registry):
        for _sub_bin in _closure_subclasses(registry).get(cls_binary, ()):
            # K-6b：类型变量签名的方法逐子类判定（有 bridge 才登记）
            if not _virtually_dispatched(cls_ci, mname, pdesc, registry, sub_bin=_sub_bin):
                continue
            _inherited_calls.request(_sub_bin, mname, pdesc)
    # 形参边界：wrapper 方法（擦除实例化）签名 vs 调用点实参（擦除描述符 /
    # 调用方泛型视图）逐位对齐——类型变量位实参装箱为 Object、桥接的具体
    # 形参按 checkcast 语义还原（A-3 try_cast）
    sig_ps_w, sig_r_w = _erased_view_sig(cls_ci, mname, params, ret, registry)
    wargs = list(args)
    if sig_ps_w is not None and len(sig_ps_w) == len(args):
        for _i, _a in enumerate(args):
            _expected = sig_ps_w[_i]
            _actual = (sig_params_v[_i] if sig_params_v and _i < len(sig_params_v)
                       and sig_params_v[_i] is not None
                       else jvm_to_rust(params[_i], registry))
            if _expected == _actual or _expected in (None, '()'):
                continue
            if _expected == 'Object':
                wargs[_i] = _coerce_arg(_a, _RsNamed(_actual), 'Object', _actual,
                                        sim, registry)
            elif _actual == 'Object':
                # 桥接的真实形参是具体类型：等价 bridge 方法内的 checkcast
                #（binary 无法解析的形态退 From 视图路径，与 Fix 17 同源）。
                # TypeIR 批次 3（V2）：形参头 → binary 经类型对象（ClassRef.binary，
                # 域内解析），替代形参头文本解剖 + 短名反查
                from ..jvm_type import from_rust_type, ClassRef as _ClassRefT
                _exp17 = from_rust_type(_expected, registry)
                _bin17 = (_exp17.binary if isinstance(_exp17, _ClassRefT) and registry
                          and _exp17.binary in registry else '')
                if _bin17:
                    wargs[_i] = _render_cast(_a, _expected,
                                             binary_name=_bin17, checked=True)
                else:
                    wargs[_i] = _render_cast(_a, _expected)
            else:
                wargs[_i] = _coerce_arg(_a, _RsNamed(_actual), _expected, _actual,
                                        sim, registry)
    barg_str = ', '.join(wargs)
    _cls_tps = _effective_class_type_params(cls_ci, registry)
    _erased_targs = f"<{', '.join(['Object'] * len(_cls_tps))}>" if _cls_tps else ''
    # 接收者静态类型串可能已带泛型实参（K-6b 擦除发射后为 Cls<Object>）：turbofish
    # 统一在裸基名上追加擦除实参，避免 Cls<Object><Object> 双后缀
    _cls_base_rust = cls_rust.split('<', 1)[0].strip()
    _view_recv = f"{_cls_base_rust}{_erased_targs}::__virtual_view(&{obj_e})"
    # 类虚方法分派（§6 步骤 4）：`__virtual_view` 命中即调用；未命中（闭包、
    # 无运行时类值）记默认值。SAM 闭包回退已随 A-5 移除——lambda 只实现接口
    # （合成对象经接口载体 + `__interface` 分派），不可能是本类实例，类虚方法
    # 的接收者在合法 Java 中恒命中 `__virtual_view`，未命中是无实现可调的残余。
    _call_expr = f"_d.{mname_r}({barg_str})?"
    if rust_ret == '()':
        sim.emit(RawStmt(
            f"if let Some(_d) = {_view_recv} {{ {_call_expr}; }}"))
        return
    v = sim.fresh('_vdispatch')
    # 返回对齐（与 _emit_call_result 同规则）：擦除描述符返回 Object 而发射签名
    # 给出具体类型（协变 / 泛型签名）→ 装箱保持 Object 记录（S-3.1，registry 类
    # 走 Object::from，vtable 桥接与后续分派全部可达）；raw 形态（X）对精确
    # 实例化（X<Object,..>）→ 记录精确形态；其余直接按 rust_ret 记录。
    if rust_ret == 'Object' and sig_r_w is not None and sig_r_w != 'Object':
        _boxed = _coerce_to_object(_call_expr, sig_r_w, registry, sim.class_type_params)
        sim.emit(RawStmt(
            f"let {v}: {rust_ret} = if let Some(_d) = {_view_recv} {{ {_boxed} }} "
            f"else {{ Default::default() }};"))
        sim.push(Var(v), RsNamed(rust_ret))
    elif (sig_r_w is not None and sig_r_w != rust_ret
            and rust_ret not in _PRIMITIVE_RUST_TYPES):
        sim.emit(RawStmt(
            f"let {v} = if let Some(_d) = {_view_recv} {{ {_call_expr} }} "
            f"else {{ Default::default() }};"))
        sim.push(Var(v), RsNamed(sig_r_w))
    else:
        sim.emit(RawStmt(
            f"let {v}: {rust_ret} = if let Some(_d) = {_view_recv} {{ {_call_expr} }} "
            f"else {{ Default::default() }};"))
        sim.push(Var(v), RsNamed(rust_ret))


_SUBCLASS_INDEX: dict[int, dict[str, list[str]]] = {}

# 泛型签名里的类型变量 token（`TP_IN;` / `TV;`；前面的负向后顾排除 `Lfoo/Type;` 形态）
_TYPE_VAR_TOKEN = __import__('re').compile(r'(?<![A-Za-z0-9_$/])T[A-Za-z0-9_$]+;')


def _virtually_dispatched(recv_ci, mname: str, param_desc: str, registry: dict,
                          sub_bin: 'str | None' = None) -> bool:
    """(mname, param_desc) 沿接收者超类链的最近声明是否可安全按子类槽位登记。

    跳过三类：private（invokespecial 静态解析，JLS §8.4.8）、final（不可覆盖，
    运行时行为恒为声明类实现）、声明签名提及声明类类型变量的方法（`TP_IN;` 形态——
    祖先形参代入具体实参的位置，如 `PipelineHelper<P_OUT>` → `PipelineHelper<Integer>`，
    继承成员的 vtable_erasure 按名匹配覆盖不到，槽位签名会与 trait 声明不一致；
    归类继承成员擦除缺口）。

    K-6b 放行：sub_bin 指定（逐子类登记判定）且该子类自身或其超类链上有对应
    synthetic bridge（参数描述符 = 槽位擦除形态）时放行——inherited_gen 的桥成员
    路径（_bridge_override_member 按 bridge 描述符渲染签名）与槽位擦除形态天然
    一致，正是 JVM 为参数位擦除覆盖（arrayLength(T_ARR) 被 arrayLength(int[])
    覆盖）记录的槽位同一性；无 bridge 的子类维持跳过（普通继承路径对不上，
    归类继承成员擦除缺口）。sub_bin 为 None（族级判定）时交由逐子类判定，
    不再整族跳过。"""
    _ACC_PRIVATE = 0x0002
    _ACC_FINAL = 0x0010
    cur, seen = recv_ci, set()
    while cur is not None and cur.name not in seen:
        seen.add(cur.name)
        m = next((x for x in cur.methods
                  if not x.is_synthetic and x.name == mname
                  and x.descriptor.startswith(param_desc)), None)
        if m is not None:
            if m.access_flags & (_ACC_PRIVATE | _ACC_FINAL):
                return False
            gs = getattr(m, 'generic_signature', '') or ''
            if _TYPE_VAR_TOKEN.search(gs):
                if sub_bin is None:
                    # K-6b：类型变量签名（`TP_IN;` / `TT_ARR;` 形态）——是否登记由
                    # 逐子类判定（有 bridge 见证才放行），族级不再整体跳过
                    return True
                return _chain_has_bridge(sub_bin, mname, param_desc, registry)
            return True
        cur = registry.get(cur.super_class) if cur.super_class else None
    return True


def _chain_has_bridge(sub_bin: str, mname: str, param_desc: str, registry: dict) -> bool:
    """sub_bin 自身及其超类链上的 (mname, 参数描述符) synthetic bridge 存在性（K-6b）。

    bridge 的参数描述符即槽位的擦除形态（javac 为参数位擦除 / 协变返回覆盖合成，
    体是对真实方法的 checkcast 转发）。"""
    cur = registry.get(sub_bin)
    seen: set[str] = set()
    while cur is not None and cur.name not in seen:
        seen.add(cur.name)
        if any(b.is_synthetic and (b.access_flags & 0x0040) and not b.is_static
               and b.name == mname
               and b.descriptor.split(')', 1)[0] + ')' == param_desc
               for b in cur.methods):
            return True
        cur = registry.get(cur.super_class) if cur.super_class else None
    return False


def _closure_subclasses(registry: dict) -> dict[str, list[str]]:
    """registry 的类子类索引：{祖先 binary name → [闭包内全部子类 binary name]}。

    按 id(registry) 记忆化（一次转译内共享；换 registry 时重建）。虚调用的运行时
    槽位分派需要为闭包子类登记继承成员（见 _resolve_direct_call_sig 的登记点）。
    """
    key = id(registry)
    cached = _SUBCLASS_INDEX.get(key)
    if cached is not None:
        return cached
    subs: dict[str, list[str]] = {}
    for bin_name, ci in registry.items():
        if getattr(ci, 'is_interface', False) or not getattr(ci, 'super_class', ''):
            continue
        cur, seen = ci.super_class, set()
        while cur and cur not in seen and cur in registry:
            seen.add(cur)
            subs.setdefault(cur, []).append(bin_name)
            cur = registry[cur].super_class
    _SUBCLASS_INDEX.clear()  # 只保留当前 registry 的索引
    _SUBCLASS_INDEX[key] = subs
    return subs


def _resolve_direct_call_sig(sim, class_name, cls, mname, params, ret, rust_ret,
                             obj_base, obj_e, obj_ty, registry):
    """非 bare 接收者的调用形态解析：vtable body 内 this 直调 / 继承成员登记 /
    根 vtable 路由 / bridge 真实方法重解析，产出最终（声明类, 接收者形态, 返回类型）。
    返回 (params, ret, rust_ret, _sig_owner, _sig_recv_ty, _recv, _root_routed)。"""
    # vtable body 上下文（this: &ClassName__inner，无 .vtable 字段）→ 直接调用。
    # 仅对当前类自身的 receiver（obj_base 匹配当前类短名）生效；
    # 其他 wrapper 类型的 receiver 仍走常规 vtable 分派检查。
    _cur_class_short = short_cls(class_name) if class_name else ''
    # 返回类型解析所用的（声明类, 接收者形态）：方法继承自祖先时换成祖先及其实参化形态，
    # 使祖先的类型变量（ForkJoinTask<V>.join → V）按接收者的超类实参代入
    _sig_owner, _sig_recv_ty = cls, obj_ty
    _root_routed = False  # True：调用路由到根 vtable（根类方法返回类型由调用点标注决定）
    # 仅当接收者就是 this 本身时才直接调用；同类的其他实例（如 compareTo(that) 的 that）
    # 是 wrapper。wrapper 接收者一律生成 `obj.method(args)`（与 Java 一致）：
    # 方法继承自祖先时登记继承成员需求，由接收者类的 java_class! 块声明该成员、
    # 宏展开为 wrapper 转发方法，vtable 分派不出现在方法体里。
    _recv = obj_e
    # this 接收者同样登记：调用祖先声明的方法、或抽象类调用自身未声明的接口抽象方法
    # （`this.getLong(f)`）时，由本类 java_class! 块声明转发成员（后者经接口载体分派），
    # 调用点保持 `this.m(args)`。this 的类取当前类 binary name（短名可能跨包重名）。
    _this_recv_bin = (class_name if (sim.in_vtable_body and obj_base == _cur_class_short
                                     and obj_e in ('this', 'self')
                                     and registry and class_name in registry) else None)
    if registry:
        _obj_jvm = _this_recv_bin or _rust_type_to_binary(obj_base, registry)
        _ci_recv = registry.get(_obj_jvm) if _obj_jvm else None
        if _ci_recv is not None:
            _param_desc = '(' + ''.join(params) + ')'
            _declared_here = any(
                not m.is_synthetic and m.name == mname and m.descriptor.startswith(_param_desc)
                for m in _ci_recv.methods
            )
            if _declared_here:
                # 协变返回：调用描述符（接口 / 祖先视角的返回类型）命中的是 synthetic bridge，
                # 生成的 Rust 方法是接收者类声明的真实方法 → 返回类型按真实方法记录
                _real_m = next((m for m in _ci_recv.methods
                                if not m.is_synthetic and m.name == mname
                                and m.descriptor.startswith(_param_desc)), None)
                if _real_m is not None and _real_m.descriptor != _param_desc + ret:
                    ret = _real_m.descriptor[len(_param_desc):]
                    rust_ret = jvm_to_rust(ret, registry)
                    _sig_owner, _sig_recv_ty = _obj_jvm, obj_ty
            if not _declared_here:
                _jvm_desc_v = _param_desc + ret
                _owner_bin_v, _ = _resolve_method_owner(_obj_jvm, mname, registry, descriptor=_jvm_desc_v)
                if _owner_bin_v and _owner_bin_v != _obj_jvm:
                    # 返回类型按 owner 在接收者静态类型下的实参化形态解析
                    _owner_short_v = _short_cls_g(_owner_bin_v)
                    _owner_args_v = _ancestor_vtable_args_by_short(
                        _ci_recv, obj_ty, registry).get(_owner_short_v, '')
                    _sig_owner, _sig_recv_ty = _owner_bin_v, _owner_short_v + _owner_args_v
                    _inherited_calls.request(_obj_jvm, mname, _param_desc)
                elif (not _owner_bin_v
                        and ((mname, _param_desc) in _root_virtual_methods()
                             or ((mname, _param_desc) in _root_protected_void_methods()
                                 and mname in _handwritten_root_api()))):
                    # protected void 根方法（静态链未覆盖的 this.finalize()）同样落根类
                    # API；限制：运行时子类的覆盖不经此路由生效（该方法不在 ObjectVTable）
                    # 整条祖先链未声明、由根类声明 → 装箱后走根 vtable。
                    # Object::from（非 from_any）：保持接收者的 vtable（运行时类名、
                    # is_instance_of、覆盖的 hashCode/equals/toString），JvmRef 装箱会丢这些
                    # [equiv-audit] identity-hash（S-6）：接收者链上无人声明
                    # hashCode → 恒走根 vtable 默认实现（当前非 identity hash）
                    if mname == 'hashCode':
                        equiv_audit.record('identity-hash')
                    _recv = f"Object::from(Clone::clone(&{obj_e}))"
                    _root_routed = True
                else:
                    # 超类链上无字节码声明：
                    #   · 调用描述符只命中 synthetic bridge → 按被桥接真实方法的参数登记；
                    #   · 否则为接口方法（default 注入到实现类 / 抽象方法由接口载体分派）。
                    _bridged_v = _resolve_bridge_target(_ci_recv, mname, _jvm_desc_v, registry)
                    if _bridged_v is not None:
                        _param_desc = _bridged_v[1].split(')')[0] + ')'
                        if not _bridged_v[0].is_interface:
                            # 返回类型 / 签名查找按被桥接的真实方法（与生成的 Rust 方法同源）
                            _, _, params, ret = parse_method_ref(f"{mname}:{_bridged_v[1]}")
                            rust_ret = jvm_to_rust(ret, registry)
                            _sig_owner = _bridged_v[0].name
                        _bridge_owner_short = _short_cls_g(_bridged_v[0].name)
                        if _bridged_v[0].name != _obj_jvm and not _bridged_v[0].is_interface:
                            _owner_args_v = _ancestor_vtable_args_by_short(
                                _ci_recv, obj_ty, registry).get(_bridge_owner_short, '')
                            _sig_owner = _bridged_v[0].name
                            _sig_recv_ty = _bridge_owner_short + _owner_args_v
                    if _bridged_v is None or _bridged_v[0].name != _obj_jvm:
                        _inherited_calls.request(_obj_jvm, mname, _param_desc)
    return params, ret, rust_ret, _sig_owner, _sig_recv_ty, _recv, _root_routed


def _chain_declares(obj_ty: str, mname: str, registry) -> bool:
    """接收者静态类型的类链（本类及超类，registry 内）是否声明了实例方法 mname。
    数组 / 手写类 / 未知类型 → False（调用解析到根类）。"""
    if not registry:
        return False
    from ..jvm_type import from_rust_type, rust_head_name
    head = rust_head_name(from_rust_type(obj_ty, registry).erasure())
    cur = _rust_type_to_binary(head, registry) if head else None
    seen: set = set()
    while cur and cur in registry and cur not in seen:
        seen.add(cur)
        ci = registry[cur]
        if any(m.name == mname and not m.is_static for m in ci.methods):
            return True
        cur = ci.super_class
    return False


def _build_call(mname_r, recv, args):
    return f"{recv}.{mname_r}({args})"


def _emit_call_result(sim, class_name, cls, mname, params, ret, rust_ret, rust_mname,
                      arg_str, obj_e, obj_ty, obj_is_bare, _sig_owner,
                      _sig_recv_ty, _recv, _root_routed, registry):
    """调用发射与结果记录：void 直发 / 根路由 / clone 特例 / 签名真实返回类型
    与擦除类型的对齐（S-3.1 装箱、精确形态记录、幂等 from_any）。"""
    if rust_ret == '()':
        if not obj_is_bare:
            sim.emit(RawStmt(f"{_build_call(rust_mname, _recv, arg_str)}?;"))
    else:
        v = sim.fresh()
        if obj_is_bare and rust_ret not in ('Object', '()') and rust_ret not in _PRIMITIVE_RUST_TYPES:
            sim.emit(RawStmt(f"let {v}: {rust_ret} = Default::default();"))
        elif _root_routed:
            sim.emit(RawStmt(f"let {v}: {rust_ret} = {_build_call(rust_mname, _recv, arg_str)}?;"))
            sim.push(Var(v), RsNamed(rust_ret))
        elif (rust_mname == 'clone' and obj_ty not in ('Object', '()')
              and not _chain_declares(obj_ty, mname, registry)):
            # 仅当调用解析到根类 Object.clone（数组 / 类链无人声明 clone）：类链上有
            # 声明（用户 / JDK 的 clone 覆盖、协变 clone + 桥）时走常规虚分派——
            # 否则覆盖体（深拷贝逻辑）被绕过、经基类静态类型的虚调用丢失（#19 探针）。
            # invokevirtual Object.clone 调用在具体类型上（如数组）：Java 的 clone 是
            # 浅拷贝（新对象、字段 / 元素共享引用），不是 Rust 的引用克隆——
            # Object__clone_base 经接收者 vtable 的 __shallow_copy 派发（数组 →
            # 新数组；未实现 Cloneable → CloneNotSupportedException，与 Java 一致），
            # 结果以 Object 返回（与 Object.clone 的返回类型一致）。
            # `this` 在实例方法中已是 &Self（再取 && 会脱离 ObjectVTable 约束）
            _recv_clone = 'this' if obj_e == 'this' else f'&{obj_e}'
            sim.emit(RawStmt(f"let {v}: Object = Object__clone_base({_recv_clone})?;"))
            sim.push(Var(v), RsNamed('Object'))
        else:
            # 签名真实返回类型与擦除类型不一致时的对齐（与 invokestatic 同规则）：
            # - 擦除映射 Object：包 from_any 保持 Object 记录
            # - 擦除实例化 X<Object,Object> → 精确形态 X<K,V>：记录精确类型，
            #   与被调方法声明（gen_method_body 按 generic_signature 生成）一致
            _sig_ret_v = _lookup_method_sig_ret(
                _sig_owner, mname, params, ret, registry,
                caller_class=class_name, caller_tparams=sim.class_type_params,
                receiver_type=_sig_recv_ty,
            )
            _call_str = _build_call(rust_mname, _recv, arg_str)
            if (rust_ret == 'Object' and _sig_ret_v is not None
                    and _sig_ret_v != 'Object'):
                # 签名真实返回类型装箱（S-3.1）：registry 类走 Object::from —— vtable
                # 桥接（toString/equals/is_instance_of）与 downcast 还原全部可达；
                # 类型变量走 Into；仅未知形态才 from_any 不透明包装
                sim.emit(RawStmt(f"let {v} = {_coerce_to_object(f'{_call_str}?', _sig_ret_v, registry, sim.class_type_params)};"))
                sim.push(Var(v), RsNamed(rust_ret))
            elif (_sig_ret_v is not None and _sig_ret_v != rust_ret
                    and rust_ret not in _PRIMITIVE_RUST_TYPES):
                sim.emit(RawStmt(f"let {v} = {_call_str}?;"))
                sim.push(Var(v), RsNamed(_sig_ret_v))
            elif (rust_ret == 'Object' and _sig_ret_v is None
                    and _erased_ret_is_type_var(cls, mname, params, ret, registry)):
                # 返回裸类型变量且无法按接收者实例化（接口 default 方法内联、跨类擦除接收者）：
                # 真实类型可能是 V 也可能是 Object，幂等装箱保证与 sim 记录的 Object 一致
                sim.emit(RawStmt(f"let {v} = Object::from_any({_call_str}?);"))
                sim.push(Var(v), RsNamed(rust_ret))
            else:
                sim.emit(RawStmt(f"let {v} = {_call_str}?;"))
                sim.push(Var(v), RsNamed(rust_ret))


def _gen_invokevirtual(sim: StackSim, comment: str, class_name: str, registry: dict | None = None):
    cls, mname, params, ret = parse_method_ref(comment)
    # [equiv-audit] 按字节码常量池方法名匹配的近似等价形态（invokevirtual /
    # invokeinterface 共用本入口），只计数不改发射：
    # - intern-identity（S-6）：String.intern 调用点——intern 后 == 的同一性
    #   是近似等价（该名由 String 独占声明，名字即字节码事实）
    # - class-literal（S-5）：getClass 调用点——返回 Class 对象的同一性近似
    #   （数组接收者的早路径在 _try_early_receiver_paths 里另行计数）
    if mname == 'intern':
        equiv_audit.record('intern-identity')
    elif mname == 'getClass':
        equiv_audit.record('class-literal')
    sig_params_v = _resolve_virtual_sig_params(sim, cls, mname, params, ret,
                                              class_name, registry)
    args, obj_e, obj_ty = _pop_receiver_and_args(sim, params, sig_params_v, registry)
    # Fix 16：泛型参数接收者（如 k.equals(pk) 中 k: K）——inherent 方法不在
    # 类型参数上可见（E0599）。装箱为 Object 后：Object 自身的方法
    # （equals/hashCode/toString）直接调用 java_runtime 手写实现，避免
    # dispatch 链枚举 Object 的全部子类；其余方法走 obj_is_bare 的多态
    # dispatch。参数已在上方 pop 循环中完成 Object::from_any 装箱。
    _obj_is_typevar = obj_ty in (sim.class_type_params or ())
    if _obj_is_typevar:
        obj_e = f"Into::<Object>::into(Clone::clone(&{obj_e}))"
        obj_ty = 'Object'
    if _try_early_receiver_paths(sim, _obj_is_typevar, mname, args, obj_e, obj_ty,
                                 params, ret, registry):
        return
    # 接口私有实例方法（Java 9+，javac 自 default 体发射 invokeinterface）：JVM 解析后
    # 直接执行接口自身的实现（非契约成员：不进 vtable、不被实现类继承，实现类同名
    # 方法不构成覆盖）。定义侧在接口载体擦除实例化的固有 impl 块（class_writer 的
    # iface-private 分支）——接收者静态类型即接口载体时直调，否则经 A-4 载体视图
    # 转换（From<Impl> / From<Object> 均经 Object 边界保持对象身份）。
    if registry and cls:
        _pv_bin = _private_iface_target(cls, mname, f"({''.join(params)}){ret}", registry)
        if _pv_bin is not None:
            _pv_short = short_cls(_pv_bin)
            _pv_mname = _safe_field(_mangle_if_overloaded(_pv_bin, mname, comment, registry))
            _pv_tps = _effective_class_type_params(registry[_pv_bin], registry)
            _pv_targs = f"<{', '.join(['Object'] * len(_pv_tps))}>" if _pv_tps else ''
            _pv_arg_str = ', '.join(args)
            rust_ret_pv = jvm_to_rust(ret, registry)
            # TypeIR 批次 3（V3）：接收者静态类型是否即私有接口所属载体——
            # erasure 基名经 binary 比较（短名单射下与旧短名比较等价），
            # 替代接收者头部文本解剖
            from ..jvm_type import from_rust_type, JvmType as _JvmTypeT
            if from_rust_type(obj_ty, registry).erasure() \
                    == _JvmTypeT.class_of(_pv_bin, registry):
                _pv_recv_e = obj_e
            elif obj_e == 'this':
                _pv_recv_e = (f"Into::<{_pv_short}{_pv_targs}>::into(Clone::clone(this))")
            elif obj_e.startswith('&'):
                _pv_recv_e = (f"Into::<{_pv_short}{_pv_targs}>::into(Clone::clone({obj_e[1:]}))")
            else:
                _pv_recv_e = (f"Into::<{_pv_short}{_pv_targs}>::into(Clone::clone(&{obj_e}))")
            _pv_call = f"{_pv_recv_e}.{_pv_mname}({_pv_arg_str})?"
            if rust_ret_pv == '()':
                sim.emit(RawStmt(f"{_pv_call};"))
            else:
                v = sim.fresh()
                sim.emit(RawStmt(f"let {v}: {rust_ret_pv} = {_pv_call};"))
                sim.push(Var(v), RsNamed(rust_ret_pv))
            return
    # 若接收方 Rust 类型是 java_runtime 手写类：API 名面固定，仅根类 Object 的
    # 同名重载（wait(J)/wait(JI) → wait_l/wait_l_i，S-20）按描述符后缀取名。
    # TypeIR 批次 3（V4）：接收者基名经类型对象 erasure 头标识符取
    # （rust_head_name：ClassRef → short_cls(binary)、Array → 'JArray'、
    # Primitive → Rust 拼写、占位 → 短名本身），替代接收者头部文本解剖
    from ..jvm_type import from_rust_type, rust_head_name
    obj_base = rust_head_name(from_rust_type(obj_ty, registry).erasure())  # ArrayList<T> → ArrayList
    if obj_base in _JAVA_RUNTIME_SHORT_NAMES:
        rust_mname = _safe_field(
            _mangle_if_overloaded(_OBJECT_CLASS, mname, comment, registry))
    else:
        # 优先用接收者实际类型 mangle（invokeinterface 通过接口调用时 cls 是接口，
        # 接口只有一个方法→漏判重载；用实际 receiver 类型能正确找到重载）
        mangle_cls = obj_base if (obj_base and obj_base not in ('Object', '()')) else (cls or '')
        rust_mname = _safe_field(_mangle_if_overloaded(mangle_cls, mname, comment, registry))

    # 所有方法统一处理：obj.method(args)?（用户类 + JDK 类均走此路径）
    arg_str = ', '.join(args)
    rust_ret = jvm_to_rust(ret, registry)
    # E0599 防护：接收者是 Object 类型时，Object 结构体不定义具体子类方法，
    # 直接调用会产生 E0599 → 走 bare 接收者的多态分派（§6 步骤 4：根 vtable
    # 直调 / 类 vtable 视图重建 / 接口载体 / 闭包回退）。
    obj_is_bare = (obj_ty == 'Object')
    if obj_is_bare and cls == 'Object' and registry:
        if _emit_object_direct_call(sim, obj_e, args, rust_mname, rust_ret):
            return
    if obj_is_bare and cls and registry:
        _dispatch_bare_object(sim, obj_e, cls, mname, comment, params, ret,
                              args, arg_str, rust_ret, registry,
                              rust_mname=rust_mname, sig_params_v=sig_params_v)
        return
    params, ret, rust_ret, _sig_owner, _sig_recv_ty, _recv, _root_routed = _resolve_direct_call_sig(
        sim, class_name, cls, mname, params, ret, rust_ret, obj_base, obj_e, obj_ty, registry)
    _emit_call_result(sim, class_name, cls, mname, params, ret, rust_ret, rust_mname,
                      arg_str, obj_e, obj_ty, obj_is_bare, _sig_owner, _sig_recv_ty,
                      _recv, _root_routed, registry)
