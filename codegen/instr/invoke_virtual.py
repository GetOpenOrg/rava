# 从 codegen/instr/invoke.py 中拆出

from ..type_map import short_cls as _short_cls_g
from ..stack import StackSim
from .. import inherited_calls as _inherited_calls
from ..rs_ir import Lit, Var, RawExpr, RawStmt, RsNamed
from ..render import render_expr, render_type
from ..type_map import jvm_to_rust, short_cls, parse_descriptor_params, is_jdk
from ..constants import safe_ident as _safe_field
from .coerce import (
    parse_method_ref,
    _mangle_if_overloaded, _resolve_bridge_target,
    UNBOX_VIRTUAL, _PRIMITIVE_RUST_TYPES,
    _JAVA_RUNTIME_SHORT_NAMES,
    _rust_type_to_binary, _get_all_subtypes_ordered,
    _find_method_super_prefix_for_type, _super_prefix_to_expr,
    _resolve_method_owner, _root_virtual_methods,
)
from ..type_map import parse_class_type_params as _parse_class_type_params
from ..type_map import ancestor_vtable_args_by_short as _ancestor_vtable_args_by_short
from ..type_map import (ancestor_type_args as _ancestor_type_args,
                        effective_class_type_params as _effective_class_type_params,
                        split_rust_type_args as _split_rust_type_args)
from .invoke_sig import (receiver_type_arg_map, type_var_receiver_bound_view, _lookup_method_sig_params, _lookup_method_sig_ret, _erased_ret_is_type_var,
                         _coerce_arg, _split_type_args)


def _declaring_interface(iface_ci, mname: str, descriptor: str, registry: dict) -> str:
    """接口 iface_ci（或其超接口，广度优先）中声明实例方法 mname:descriptor 的接口 binary name。
    根类方法的重声明（经 Object vtable 分派）、私有 / 合成方法不算接口成员 → ''。"""
    if (mname, descriptor[:descriptor.index(')') + 1]) in _root_virtual_methods():
        return ''
    queue = [iface_ci]
    seen: set[str] = set()
    while queue:
        cur = queue.pop(0)
        if cur.name in seen:
            continue
        seen.add(cur.name)
        for m in cur.methods:
            if (m.name == mname and m.descriptor == descriptor and not m.is_static
                    and not m.is_synthetic and not (m.access_flags & 0x0002)):
                return cur.name
        queue.extend(registry[i] for i in (cur.interfaces or []) if i in registry)
    return ''


def _close_open_type_args(sim, stack_idx: int) -> None:
    """菱形构造结果（类型实参待推断的 `X<_, A>`）直接作方法调用的接收者
    （`new Task<>(helper, spliterator).invoke()`）：值不流经任何带类型的位置（局部声明、形参、
    字段、返回值），待推断的类型实参没有约束来源 → 取其擦除（根类），写回构造处的 turbofish。"""
    import re as _re_open
    from ..rs_ir import LetStmt as _LetStmt, RawExpr as _RawExpr, RsNamed as _RsNamed, Var as _Var
    expr, ty = sim.stack[stack_idx]
    ty_s = render_type(ty)
    _open = r'(?<![A-Za-z0-9_])_(?![A-Za-z0-9_])'
    if '<' not in ty_s or not _re_open.search(_open, ty_s):
        return
    if not isinstance(expr, _Var):
        # 构造表达式本身在栈上（`new X<>(..).m()` 未绑定临时变量）
        _base, _args = ty_s.split('<', 1)
        _open_tf = f"{_base}::<{_args[:-1]}>::"
        _code = render_expr(expr)
        if _code.startswith(_open_tf):
            _closed = _re_open.sub(_open, 'Object', _args[:-1])
            sim.stack[stack_idx] = (_RawExpr(f"{_base}::<{_closed}>::" + _code[len(_open_tf):]),
                                    _RsNamed(f"{_base}<{_closed}>"))
        return
    if any(_loc[0] == expr.name for _loc in sim.locals.values()):
        return      # Java 局部变量：类型实参由声明 / 后续用法确定
    base, args = ty_s.split('<', 1)
    open_tf = f"{base}::<{args[:-1]}>::"
    closed_args = _re_open.sub(_open, 'Object', args[:-1])
    for stmt in reversed(sim.stmts):
        if isinstance(stmt, _LetStmt) and stmt.name == expr.name:
            if not (isinstance(stmt.value, _RawExpr) and stmt.value.code.startswith(open_tf)):
                return
            stmt.value = _RawExpr(f"{base}::<{closed_args}>::" + stmt.value.code[len(open_tf):])
            closed_ty = _RsNamed(f"{base}<{closed_args}>")
            if stmt.ty is not None:
                stmt.ty = closed_ty
            if stmt.value_ty is not None:
                stmt.value_ty = closed_ty
            sim.stack[stack_idx] = (expr, closed_ty)
            return


def _gen_invokevirtual(sim: StackSim, comment: str, class_name: str, registry: dict | None = None):
    cls, mname, params, ret = parse_method_ref(comment)
    # 在弹出参数前先 peek 接收者类型（在栈顶之下 len(params) 个位置），
    # 解析泛型实参以建立 callee 类型参数 → 接收者实参的映射（如 HashMap<E,Object> → K=E）
    _recv_targ_map: dict | None = None
    _recv_is_this = False
    _recv_ty = ''
    _recv_stack_idx = len(params)
    if len(sim.stack) > _recv_stack_idx:
        import re as _re_recv
        # 接收者是 this（继承到类里的接口 default 方法体）：接口类型形参即本类类型形参
        _recv_is_this = render_expr(sim.stack[-(_recv_stack_idx + 1)][0]) == 'this'
        # 类型变量接收者（task.makeChild(..)，task: K，K extends B<..,K>）：方法定义在
        # 上界类的 wrapper 上 → 接收者换成上界类型视图（与 getfield/putfield 同规则），
        # 之后的签名查找、实参映射、分派均按上界类型进行。无类上界时保持原样。
        _rv_e, _rv_t = sim.stack[-(_recv_stack_idx + 1)]
        _bv_e, _bv_t = type_var_receiver_bound_view(sim, _rv_e, _rv_t)
        if _bv_t is not _rv_t:
            sim.stack[-(_recv_stack_idx + 1)] = (_bv_e, _bv_t)
        _close_open_type_args(sim, len(sim.stack) - (_recv_stack_idx + 1))
        _recv_ty = render_type(sim.stack[-(_recv_stack_idx + 1)][1])
        _recv_targ_map = receiver_type_arg_map(_recv_ty, cls, registry)
    sig_params_v = None
    _recv_base_v = _recv_ty.split('<')[0].strip()
    if registry and _recv_base_v and _recv_base_v != cls and not _recv_is_this:
        # 接口方法经具体类接收者调用（`Map<Long,String> m = new HashMap<>(); m.put(k, v)`，
        # 局部变量的 Rust 类型是构造出的类实例化）：Rust 侧解析到类自身的方法，
        # 形参类型按类的声明签名 + 接收者实参确定，而非接口的擦除载体形态
        _recv_cls_ci = registry.get(_rust_type_to_binary(_recv_base_v, registry) or '')
        _call_cls_ci = registry.get(_rust_type_to_binary(cls, registry) or '')
        if (_recv_cls_ci is not None and not _recv_cls_ci.is_interface
                and _call_cls_ci is not None and _call_cls_ci.is_interface):
            sig_params_v = _lookup_method_sig_params(
                _recv_base_v, mname, params, ret, registry, sim.class_type_params,
                receiver_targ_map=receiver_type_arg_map(_recv_ty, _recv_base_v, registry),
                receiver_is_this=False,
                receiver_type=_recv_ty,
            )
    if registry and sig_params_v is None:
        # 调用描述符在接收者类上只命中 synthetic bridge（`copyInto(Object[],int)` →
        # `copyInto(Integer[],int)`）：Rust 侧只生成被桥接的真实方法，形参类型按真实方法确定
        _br_recv_bin = (class_name if _recv_is_this
                        else _rust_type_to_binary(_recv_base_v, registry)) if (_recv_is_this or _recv_base_v) else ''
        _br_recv_ci = registry.get(_br_recv_bin or '')
        if _br_recv_ci is not None and not _br_recv_ci.is_interface:
            _br_desc = '(' + ''.join(params) + ')' + ret
            _br_target = _resolve_bridge_target(_br_recv_ci, mname, _br_desc, registry)
            if _br_target is not None and _br_target[1] != _br_desc:
                _, _, _br_params, _br_ret = parse_method_ref(f"{mname}:{_br_target[1]}")
                if len(_br_params) == len(params):
                    sig_params_v = _lookup_method_sig_params(
                        _short_cls_g(_br_target[0].name), mname, _br_params, _br_ret, registry,
                        sim.class_type_params,
                        receiver_targ_map=receiver_type_arg_map(
                            _recv_ty, _short_cls_g(_br_target[0].name), registry),
                        receiver_is_this=_recv_is_this,
                        receiver_type=_recv_ty,
                    ) or [jvm_to_rust(_p, registry) for _p in _br_params]
    if sig_params_v is None:
        sig_params_v = _lookup_method_sig_params(
            cls, mname, params, ret, registry, sim.class_type_params,
            receiver_targ_map=_recv_targ_map,
            receiver_is_this=_recv_is_this,
            receiver_type=_recv_ty,
        )
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

    # Fix 16：泛型参数接收者（如 k.equals(pk) 中 k: K）——inherent 方法不在
    # 类型参数上可见（E0599）。装箱为 Object 后：Object 自身的方法
    # （equals/hashCode/toString）直接调用 java_runtime 手写实现，避免
    # dispatch 链枚举 Object 的全部子类；其余方法走 obj_is_bare 的多态
    # dispatch。参数已在上方 pop 循环中完成 Object::from_any 装箱。
    if obj_ty in (sim.class_type_params or ()):
        obj_e = f"Into::<Object>::into(Clone::clone(&{obj_e}))"
        obj_ty_node = RsNamed('Object')
        obj_ty = 'Object'
        if mname in ('equals', 'hashCode', 'toString'):
            _rust_ret_eq = jvm_to_rust(ret, registry)
            _arg_str_eq = ', '.join(args)
            if _rust_ret_eq == '()':
                sim.emit(RawStmt(f"{obj_e}.{mname}({_arg_str_eq})?;"))
            else:
                _v_eq = sim.fresh()
                sim.emit(RawStmt(f"let {_v_eq}: {_rust_ret_eq} = {obj_e}.{mname}({_arg_str_eq})?;"))
                sim.push(Var(_v_eq), RsNamed(_rust_ret_eq))
            return
    if mname in UNBOX_VIRTUAL:
        _UNBOX_TARGET = {
            'intValue': 'i32', 'longValue': 'i64', 'doubleValue': 'f64',
            'floatValue': 'f32', 'booleanValue': 'bool', 'byteValue': 'i8', 'shortValue': 'i16',
            'charValue': 'u16',
        }
        target_ty = _UNBOX_TARGET.get(mname)
        if target_ty is None or obj_ty == target_ty:
            sim.push(obj_expr, obj_ty_node)
            return
        # 接收者已是基本类型但与目标不同（Boolean.valueOf 被 BOXING_SKIP_STATIC 跳过
        # 后栈上留 i32，再调 booleanValue 期望 bool）：生成类型转换而非方法调用
        if obj_ty in _PRIMITIVE_RUST_TYPES:
            v = sim.fresh()
            if target_ty == 'bool':
                sim.emit(RawStmt(f"let {v}: bool = ({obj_e} != 0);"))
            else:
                sim.emit(RawStmt(f"let {v}: {target_ty} = {obj_e} as {target_ty};"))
            sim.push(Var(v), RsNamed(target_ty))
            return
        # 接收者是装箱对象（Integer/Double/Number 等），生成实际方法调用完成解箱
        v = sim.fresh()
        sim.emit(RawStmt(f"let {v}: {target_ty} = {obj_e}.{mname}()?;"))
        sim.push(Var(v), RsNamed(target_ty))
        return

    # 基本类型 .equals(x) → 生成 == 比较（基本类型无 equals 方法）
    if mname == 'equals' and len(args) == 1 and obj_ty in _PRIMITIVE_RUST_TYPES:
        raw_arg = args[0].removesuffix('.into()')
        v = sim.fresh()
        sim.emit(RawStmt(f"let {v}: bool = ({obj_e} == {raw_arg});"))
        sim.push(Var(v), RsNamed('bool'))
        return

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
        return

    # JVM 数组.getClass() → Object::default()（代表 Class<T[]>）
    # Rust 侧 Vec/数组类型没有 getClass()，但调用方（如 Arrays.copyOf）只用
    # 其结果判断是否为 Object[] 类型；Object::default() 使判断走 Object[] 分支
    if mname == 'getClass' and obj_ty.startswith('JArray<'):
        v = sim.fresh()
        sim.emit(RawStmt(f"let {v}: Object = Object::default();"))
        sim.push(Var(v), RsNamed('Object'))
        return

    # 若接收方 Rust 类型是 java_runtime 手写类，不做 mangle
    obj_base = obj_ty.split('<')[0].strip()  # 去泛型后缀（ArrayList<T> → ArrayList）
    if obj_base in _JAVA_RUNTIME_SHORT_NAMES:
        rust_mname = _safe_field(mname)
    else:
        # 优先用接收者实际类型 mangle（invokeinterface 通过接口调用时 cls 是接口，
        # 接口只有一个方法→漏判重载；用实际 receiver 类型能正确找到重载）
        mangle_cls = obj_base if (obj_base and obj_base not in ('Object', '()')) else (cls or '')
        rust_mname = _safe_field(_mangle_if_overloaded(mangle_cls, mname, comment, registry))

    # 所有方法统一处理：obj.method(args)?（用户类 + JDK 类均走此路径）
    arg_str = ', '.join(args)
    rust_ret = jvm_to_rust(ret, registry)
    # E0599 防护：接收者是 Object 类型时，Object 结构体不定义具体子类方法，
    # 直接调用会产生 E0599。若目标类已知且有子类，生成 downcast dispatch 链（多态虚分发）。
    obj_is_bare = (obj_ty == 'Object')
    if obj_is_bare and cls == 'Object' and registry:
        # Object 类方法（hashCode/equals/compareTo 等）通过 Object 包装器直接调用。
        # 仅对基本类型返回值或 void 方法走直接调用路径；
        # 非基本类型返回（如 getClass→Class）保持原来的 closure-only dispatch，
        # 因为 object_impl.rs 的实现返回 Object 而非具体类型，不能直接赋值。
        if rust_ret == '()' or rust_ret in _PRIMITIVE_RUST_TYPES:
            arg_str = ', '.join(args)
            v = sim.fresh()
            if rust_ret == '()':
                sim.emit(RawStmt(f"{obj_e}.{rust_mname}({arg_str})?;"))
            else:
                # 基本类型返回值（hashCode→i32, equals→bool, compareTo→i32 等）
                # 通过 ObjectVTable dyn dispatch 正确路由到实际类型的覆盖实现
                sim.emit(RawStmt(f"let {v}: {rust_ret} = {obj_e}.{rust_mname}({arg_str})?;"))
                sim.push(Var(v), RsNamed(rust_ret))
            return
    if obj_is_bare and cls and registry:
        # 多态 dispatch：按继承链（叶→根）依次 downcast，找到实际类型后调用方法
        # cls 是 Rust 短类名（$ 已替换为 _），需转回 binary name 查继承链
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
                    sim.emit(RawStmt(f"let {v}: {rust_ret} = {_iface_call};"))
                    sim.push(Var(v), RsNamed(rust_ret))
                return
        subtypes = _get_all_subtypes_ordered(cls_binary, registry)
        # 跨 crate 防泄漏：JDK 类文件落在 java_runtime crate，不能引用 user crate
        # 的类型。batch 并集 registry 会把用户测试类也列为 Comparable 等接口的
        # 子类，烘焙进 java_runtime 后产生 E0425（类型不在本 crate 作用域）。
        # 因此 JDK 类的分派链只枚举 JDK 子类；user 类文件可引用两者（依赖方向合法）。
        if is_jdk(cls_binary):
            subtypes = [s for s in subtypes if is_jdk(s)]
        cls_rust = jvm_to_rust(f'L{cls_binary};', registry)
        # Arch-3: 闭包回退 —— Rc<dyn Fn(...)> downcast（lambda / 方法引用）
        # SAM 参数列表对应 invokevirtual/invokeinterface 的实际参数类型
        # Result 用裸名：两个 crate 的生成文件均经 prelude 引入
        # （java_runtime: crate::prelude / user: crate::error::Result），
        # 写 crate::error::Result 在 user crate 里是 E0433。
        _sam_ptypes = [jvm_to_rust(p, registry) for p in params]
        _fn_type = (f'std::rc::Rc<dyn Fn({", ".join(_sam_ptypes)})'
                    f' -> Result<{rust_ret}>>')
        if rust_ret == '()':
            _closure_branch = (f'if let Some(__f) = {obj_e}.0.as_any()'
                               f'.downcast_ref::<{_fn_type}>() {{ (__f)({arg_str})?; }}')
        else:
            _closure_branch = (f'if let Some(__f) = {obj_e}.0.as_any()'
                               f'.downcast_ref::<{_fn_type}>() {{ (__f)({arg_str})? }}')

        # 当前调用的完整 JVM 描述符（供下面 _resolve_method_owner 精确匹配）
        jvm_desc = f"({''.join(params)}){ret}"
        # 根类（Object）声明的方法：未在自身/祖先链声明该方法的类不生成分支，
        # 统一落到链尾的根 vtable 调用（Java 语义：继承根类实现），
        # 而不是在 wrapper 上找不存在的 inherent 方法（E0599）。
        _root_declared = (mname, f"({''.join(params)})") in _root_virtual_methods()
        if _root_declared:
            _root_call = f"{obj_e}.{_safe_field(mname)}({arg_str})?"
            _chain_tail = f" else {{ {_root_call}; }}" if rust_ret == '()' else f" else {{ {_root_call} }}"
        else:
            _chain_tail = '' if rust_ret == '()' else " else { Default::default() }"
        # 只有当目标类有已知子类时，才生成 dispatch 链（否则退化为简单 downcast）
        if subtypes:
            all_types = subtypes + [cls_binary]  # 叶→根
            v = sim.fresh('_vdispatch')
            branches = []
            for sub_bin in all_types:
                _sub_ci_abs = registry.get(sub_bin)
                if _sub_ci_abs is not None and _sub_ci_abs.is_abstract and not _sub_ci_abs.is_interface:
                    # 抽象类不可能是对象的运行期类型：downcast 分支恒不命中，不生成
                    continue
                sub_rust = jvm_to_rust(f'L{sub_bin};', registry)
                if sub_rust == 'Object':
                    # 目标类是接口（jvm_to_rust 对接口返回 Object）：
                    # downcast_ref::<Object>() 恒为 Some，会遮蔽闭包回退分支，
                    # 且 Object 上没有业务方法（E0599），直接丢弃该分支
                    continue
                # Fix 12a：包装类在 JVM_RUST 中映射为基本类型（Boolean→bool、
                # Double→f64 等），downcast_ref::<bool>() 不满足 Any 约束是硬
                # 错误，该分支不可编译 → 丢弃
                if sub_rust in _PRIMITIVE_RUST_TYPES:
                    continue
                # 泛型类（如 ArrayList_Itr<E>）：jvm_to_rust 返回裸名，downcast_ref
                # 需要完整泛型实参，用 `_` 通配符让 Rust 自动推断（E0107 防护）。
                # 内部类（ArrayList_Itr）通过 this$0 继承外部类类型参数，
                # 不在自身 generic_signature 中声明，需额外检测。
                if '<' not in sub_rust and registry:
                    _sub_ci_g = registry.get(sub_bin)
                    if _sub_ci_g:
                        from ..type_map import effective_class_type_params as _ectp_g
                        _tp_g = _ectp_g(_sub_ci_g, registry)
                        if _tp_g:
                            # 使用 Object 作为类型实参（Java 类型擦除语义）：
                            # - 内部类（ArrayList_Itr）的 TypeId 与外部类类型参数绑定
                            # - downcast_ref::<ArrayList_Itr<_>>() 无法推断 `_`（E0283）
                            # - 使用 Object 使代码可编译；iterator() 存储 ArrayList_Itr<E>
                            #   时已做 Object::from_any 擦除，运行时 TypeId 匹配 Object 参数形式
                            sub_rust = sub_rust + '<' + ', '.join(['Object'] * len(_tp_g)) + '>'
                # Fix 12b：重载 mangle 按声明类（owner）查 —— 子类继承的重载
                # 方法在子类方法表中查不到同名重载，按子类表 mangle 会得到
                # 错误的方法名（如 collect 声明处 mangle 为 collect_collector，
                # 子类分支按子类表查成 collect）。owner 沿父类链解析不到时
                # （接口 default 方法由 class_writer 注入实现类 impl 块，
                # 父类链上查不到）保留子类名。
                _owner_bin, _ = _resolve_method_owner(
                    sub_bin, mname, registry, descriptor=jvm_desc)
                # 调用描述符只命中 synthetic bridge（Comparable.compareTo(Object) → 实现类的
                # compareTo(Self)）：owner 取 bridge 的声明类，后续按被桥接的真实方法处理
                # （继承自祖先的真实方法经 vtable supertrait UFCS 调用）。
                _bridge_desc = ''
                if not _owner_bin and registry and registry.get(sub_bin) is not None:
                    _bridged = _resolve_bridge_target(registry[sub_bin], mname, jvm_desc, registry)
                    if _bridged is not None and not _bridged[0].is_interface:
                        _owner_bin, _bridge_desc = _bridged[0].name, _bridged[1]
                if _root_declared and not _owner_bin:
                    continue
                if not _owner_bin and registry and registry.get(sub_bin) is not None:
                    _sub_ci_own = registry[sub_bin]
                    if _sub_ci_own.is_abstract and not _sub_ci_own.is_interface:
                        # 抽象类自身与祖先链都未声明该方法：对象的运行时类必为其具体子类
                        # （各有独立分支），本分支在 Java 语义下不可达，不生成。
                        continue
                    # 具体类：实现来自祖先注入的接口 default 方法 → 登记继承成员声明
                    _inherited_calls.request(sub_bin, mname, '(' + ''.join(params) + ')')
                _mangle_cls = _owner_bin or sub_rust
                sub_mname_r = _mangle_if_overloaded(_mangle_cls, mname, comment, registry)
                sub_mname_r = _safe_field(sub_mname_r)
                # Fix 17：bridge 分派的参数 downcast —— dispatch 的共享 args 按
                # 擦除描述符 coercion（如 Comparable.compareTo(Object) 的参数为
                # Object），但子类分支的真实方法（bridge 的目标，如
                # Byte.compareTo(Byte)）参数是具体类型 → per-branch 包装
                # .downcast::<T>()（等价 bridge 方法内的 checkcast）。
                _barg_str = arg_str
                _bm17 = None
                if registry:
                    _own_ci17 = registry.get(_owner_bin or sub_bin)
                    if _own_ci17 is not None:
                        for _m in _own_ci17.methods:
                            if (_m.name == mname and not _m.is_synthetic
                                    and _m.descriptor == (_bridge_desc or jvm_desc)):
                                _bm17 = _m
                                break
                        if _bm17 is None:
                            # 精确 descriptor 失败（bridge 擦除场景）：
                            # 按名字 + 参数个数唯一匹配真实方法
                            _cands17 = [
                                _m for _m in _own_ci17.methods
                                if _m.name == mname and not _m.is_synthetic
                                and len(parse_descriptor_params(_m.descriptor)) == len(params)
                            ]
                            if len(_cands17) > 1:
                                # 多个同名同参数个数的重载：bridge 目标的每个参数
                                # 与擦除描述符的 primitive/引用 类别必须逐位一致
                                # （bridge 只擦除引用类型，不改变 primitive 参数）
                                def _is_ref17(_d):
                                    return _d.startswith(('L', '['))
                                _cands17 = [
                                    _m for _m in _cands17
                                    if all(
                                        _is_ref17(_a) == _is_ref17(_b) and (_is_ref17(_a) or _a == _b)
                                        for _a, _b in zip(parse_descriptor_params(_m.descriptor), params))
                                ]
                            if len(_cands17) == 1:
                                _bm17 = _cands17[0]
                        if _bm17 is not None:
                            if _bm17.descriptor != jvm_desc:
                                # bridge 目标的真实描述符与调用点擦除描述符不同：
                                # 重载 mangle 后缀必须按真实方法的描述符生成，与定义侧一致
                                sub_mname_r = _safe_field(_mangle_if_overloaded(
                                    _mangle_cls, mname,
                                    f"Method {_own_ci17.name}.{mname}:{_bm17.descriptor}", registry))
                            _bp17 = parse_descriptor_params(_bm17.descriptor)
                            if len(_bp17) == len(args) and _bp17 != list(params):
                                # 从 generic_signature 提取参数类型（若有）
                                # 格式：(TE;Ljava/lang/String;...)RetType
                                _gen_params17: list[str] = []
                                if _bm17.generic_signature:
                                    import re as _re_gs
                                    _gs_inner = _re_gs.match(r'\(([^)]*)\)', _bm17.generic_signature)
                                    if _gs_inner:
                                        _gp_str = _gs_inner.group(1)
                                        _gp_pos = 0
                                        while _gp_pos < len(_gp_str):
                                            _c = _gp_str[_gp_pos]
                                            if _c == 'T':
                                                _te = _gp_str.index(';', _gp_pos)
                                                _gen_params17.append(_gp_str[_gp_pos:_te+1])
                                                _gp_pos = _te + 1
                                            elif _c == 'L':
                                                _te = _gp_str.index(';', _gp_pos)
                                                _gen_params17.append(_gp_str[_gp_pos:_te+1])
                                                _gp_pos = _te + 1
                                            elif _c in 'BCDFIJSZ':
                                                _gen_params17.append(_c)
                                                _gp_pos += 1
                                            elif _c == '[':
                                                _gp_pos += 1
                                                # 跳过数组维度
                                            else:
                                                _gp_pos += 1
                                _bparts17 = []
                                _tv_map17: dict[str, str] = {}
                                _sub_ci17 = registry.get(sub_bin)
                                if _sub_ci17 is not None:
                                    _own_tps17 = _effective_class_type_params(_own_ci17, registry)
                                    if _own_ci17.name == sub_bin:
                                        _own_args17 = _split_rust_type_args(sub_rust)
                                    else:
                                        _own_args17 = dict(_ancestor_type_args(
                                            _sub_ci17, registry, _split_rust_type_args(sub_rust))).get(_own_ci17.name, [])
                                    _tv_map17 = dict(zip(_own_tps17, _own_args17))
                                for _i17, _bd17 in enumerate(_bp17):
                                    _bt17 = jvm_to_rust(_bd17, registry)
                                    _shared17 = jvm_to_rust(params[_i17], registry)
                                    # 若 generic_signature 参数是类型变量（TE; 格式），
                                    # 说明是类型擦除产物（如 Enum.compareTo(E) → (Enum)），
                                    # 不做 downcast：实际参数类型是类型变量对应的运行时类型
                                    _is_type_var17 = (
                                        _i17 < len(_gen_params17)
                                        and _gen_params17[_i17].startswith('T')
                                        and _gen_params17[_i17].endswith(';')
                                    )
                                    if _is_type_var17 and _shared17 == 'Object':
                                        # 类型变量按 owner 在该子类视角下的实参实例化
                                        # （Enum<E> 经子类 SuperclassSignature 得 E=子类自身）→ 具体类型时 downcast
                                        _tv_inst17 = _tv_map17.get(_gen_params17[_i17][1:-1], 'Object')
                                        if _tv_inst17 not in ('Object', '()', '_'):
                                            _bparts17.append(
                                                f"({args[_i17]}).downcast::<{_tv_inst17}>()")
                                            continue
                                    if (_bt17 not in ('Object', '()')
                                            and _shared17 == 'Object'
                                            and not _is_type_var17):
                                        _bparts17.append(
                                            f"({args[_i17]}).downcast::<{_bt17}>()")
                                    else:
                                        _bparts17.append(args[_i17])
                                _barg_str = ', '.join(_bparts17)
                # 方法由祖先 _owner_bin 声明、sub 自身未覆盖：分支内同样写 `_d.method(args)`，
                # 并登记 sub 需要该继承成员 —— 由 sub 的 java_class! 块声明、宏展开为
                # wrapper 转发方法（内部经声明该方法的祖先 VTable 分派，消除 E0034 歧义）。
                _sub_pfx = _find_method_super_prefix_for_type(
                    sub_rust.split('<')[0], mname, registry,
                    descriptor=_bridge_desc or f"({''.join(params)}){ret}",
                )
                if _sub_pfx and _owner_bin:
                    _inherited_calls.request(
                        sub_bin, mname, (_bridge_desc or jvm_desc).split(')')[0] + ')')
                    _call_expr = f"_d.{sub_mname_r}({_barg_str})"
                elif _sub_pfx:
                    # fallback（owner 未知）：保留旧的 __super() 路由
                    _d_recv = _super_prefix_to_expr('_d', _sub_pfx)
                    _call_expr = f"{_d_recv}.{sub_mname_r}({_barg_str})"
                else:
                    _call_expr = f"_d.{sub_mname_r}({_barg_str})"
                # 协变返回：dispatch 结果按擦除描述符为 Object，分支真实方法返回具体类型
                # → 分支内向上转型（等价 Java bridge 方法的隐式 upcast）。
                _branch_wrap_obj = False
                _bm_ret = None
                if rust_ret == 'Object' and registry:
                    _own_ci_ret = registry.get(_owner_bin or sub_bin)
                    _param_part_ret = '(' + ''.join(params) + ')'
                    if _own_ci_ret is not None:
                        for _m in _own_ci_ret.methods:
                            if (_m.name == mname and not _m.is_synthetic
                                    and _m.descriptor.startswith(_param_part_ret)):
                                _bm_ret = _m
                                break
                if _bm_ret is None and rust_ret == 'Object':
                    # 擦除描述符未命中（泛型接口的具体化实现，如 apply(String)→具体类）：
                    # 取上面按 bridge 规则解析出的真实方法
                    _bm_ret = _bm17
                if _bm_ret is not None:
                    _bret_desc = _bm_ret.descriptor.split(')', 1)[1]
                    _bret_gen = (_bm_ret.generic_signature.split(')', 1)[1]
                                 if _bm_ret.generic_signature and ')' in _bm_ret.generic_signature else '')
                    if (not _bret_gen.startswith('T')
                            and jvm_to_rust(_bret_desc, registry) not in ('Object', '()')):
                        _branch_wrap_obj = True
                if _branch_wrap_obj:
                    branches.append(f"if let Some(_d) = {obj_e}.0.as_any().downcast_ref::<{sub_rust}>() {{ Object::from_any({_call_expr}?) }}")
                elif rust_ret == '()':
                    branches.append(f"if let Some(_d) = {obj_e}.0.as_any().downcast_ref::<{sub_rust}>() {{ {_call_expr}?; }}")
                else:
                    branches.append(f"if let Some(_d) = {obj_e}.0.as_any().downcast_ref::<{sub_rust}>() {{ {_call_expr}? }}")
            branches.append(_closure_branch)
            if rust_ret == '()':
                dispatch_code = ' else '.join(branches) + _chain_tail
                sim.emit(RawStmt(f"{dispatch_code}"))
            else:
                dispatch_expr = ' else '.join(branches) + _chain_tail
                sim.emit(RawStmt(f"let {v}: {rust_ret} = {dispatch_expr};"))
                sim.push(Var(v), RsNamed(rust_ret))
            return
        # 无 subtypes 时（接口无已知实现类）：单独生成闭包 dispatch + 占位
        v = sim.fresh('_vdispatch')
        if rust_ret == '()':
            sim.emit(RawStmt(_closure_branch + _chain_tail))
        else:
            dispatch_expr = _closure_branch + _chain_tail
            sim.emit(RawStmt(f"let {v}: {rust_ret} = {dispatch_expr};"))
            sim.push(Var(v), RsNamed(rust_ret))
        return
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
                        and (mname, _param_desc) in _root_virtual_methods()):
                    # 整条祖先链未声明、由根类声明 → 装箱后走根 vtable
                    _recv = f"Object::from_any(Clone::clone(&{obj_e}))"
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

    def _build_call(mname_r, recv, args):
        return f"{recv}.{mname_r}({args})"

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
        elif rust_mname == 'clone' and obj_ty not in ('Object', '()'):
            # invokevirtual Object.clone 调用在具体类型上（如数组）：
            # Rust 的 clone() 不返回 Result，用 Object::from_any 包装匹配 Java 返回类型
            # Clone::clone 而非 .clone()：接收者可能是带 Java clone() 的类
            sim.emit(RawStmt(f"let {v}: Object = Object::from_any(Clone::clone(&{obj_e}));"))
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
                sim.emit(RawStmt(f"let {v} = Object::from_any({_call_str}?);"))
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
