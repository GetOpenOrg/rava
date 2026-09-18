# 从 codegen/instr/invoke.py 中拆出

from ..stack import StackSim
from ..rs_ir import Lit, Var, RawExpr, RawStmt, RsNamed
from ..render import render_expr, render_type
from ..type_map import jvm_to_rust, short_cls, parse_descriptor_params, is_jdk
from ..constants import safe_ident as _safe_field
from .coerce import (
    parse_method_ref,
    _mangle_if_overloaded,
    UNBOX_VIRTUAL, _PRIMITIVE_RUST_TYPES,
    _JAVA_RUNTIME_SHORT_NAMES,
    _rust_type_to_binary, _get_all_subtypes_ordered,
    _find_method_super_prefix_for_type, _super_prefix_to_expr,
    _resolve_method_owner,
)
from ..type_map import parse_class_type_params as _parse_class_type_params
from .invoke_sig import (_lookup_method_sig_params, _lookup_method_sig_ret,
                         _coerce_arg, _split_type_args)


def _gen_invokevirtual(sim: StackSim, comment: str, class_name: str, registry: dict | None = None):
    cls, mname, params, ret = parse_method_ref(comment)
    # 在弹出参数前先 peek 接收者类型（在栈顶之下 len(params) 个位置），
    # 解析泛型实参以建立 callee 类型参数 → 接收者实参的映射（如 HashMap<E,Object> → K=E）
    _recv_targ_map: dict | None = None
    _recv_stack_idx = len(params)
    if len(sim.stack) > _recv_stack_idx:
        import re as _re_recv
        _recv_ty = render_type(sim.stack[-(_recv_stack_idx + 1)][1])
        _rm = _re_recv.match(r'^(\w+)<(.+)>$', _recv_ty)
        if _rm and registry:
            _r_cls_short = _rm.group(1)
            _r_args = _split_type_args(_rm.group(2))
            _r_bin = _rust_type_to_binary(_r_cls_short, registry)
            if _r_bin:
                _r_ci = registry.get(_r_bin)
                if _r_ci and getattr(_r_ci, 'generic_signature', None):
                    _r_tparams = _parse_class_type_params(_r_ci.generic_signature)
                    if len(_r_tparams) == len(_r_args):
                        _recv_targ_map = dict(zip(_r_tparams, _r_args))
    sig_params_v = _lookup_method_sig_params(
        cls, mname, params, ret, registry, sim.class_type_params,
        receiver_targ_map=_recv_targ_map,
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

    # Fix 16：泛型参数接收者（如 k.equals(pk) 中 k: K）——inherent 方法不在
    # 类型参数上可见（E0599）。装箱为 Object 后：Object 自身的方法
    # （equals/hashCode/toString）直接调用 java_runtime 手写实现，避免
    # dispatch 链枚举 Object 的全部子类；其余方法走 obj_is_bare 的多态
    # dispatch。参数已在上方 pop 循环中完成 Object::from_any 装箱。
    if obj_ty in (sim.class_type_params or ()):
        obj_e = f"Object::from_any(Clone::clone(&{obj_e}))"
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

    # JVM 数组.getClass() → Object::default()（代表 Class<T[]>）
    # Rust 侧 Vec/数组类型没有 getClass()，但调用方（如 Arrays.copyOf）只用
    # 其结果判断是否为 Object[] 类型；Object::default() 使判断走 Object[] 分支
    if mname == 'getClass' and obj_ty.startswith('JArray<'):
        v = sim.fresh()
        sim.emit(RawStmt(f"let {v}: Object = Object::default();"))
        sim.push(Var(v), RsNamed('Object'))
        return

    # T38：PrintStream.println 有参版本 → 统一生成 println_v(x)（Display 派发）
    # 注：无参 println() 保持原名；println_v<T: Display> 处理所有有参版本
    # 浮点参数先经 java_fmt_*（Java 语义：3.0 不打成 3；NaN/Infinity 拼写一致）
    if mname == 'println' and cls and cls.endswith('PrintStream') and len(args) == 1:
        if params == ['D']:
            sim.emit(RawStmt(f"{obj_e}.println_v(java_fmt_f64({args[0]}))?;"))
        elif params == ['F']:
            sim.emit(RawStmt(f"{obj_e}.println_v(java_fmt_f32({args[0]}))?;"))
        else:
            # 基本类型传给 println(Object) 时 .into() 产生类型推断歧义：
            # println_v<T: Display> 直接接受 i32/bool 等，无需装箱
            arg = args[0].removesuffix('.into()')
            sim.emit(RawStmt(f"{obj_e}.println_v({arg})?;"))
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
    if obj_is_bare and cls and registry:
        # 多态 dispatch：按继承链（叶→根）依次 downcast，找到实际类型后调用方法
        # cls 是 Rust 短类名（$ 已替换为 _），需转回 binary name 查继承链
        cls_binary = _rust_type_to_binary(cls, registry) or cls
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
        # 只有当目标类有已知子类时，才生成 dispatch 链（否则退化为简单 downcast）
        if subtypes:
            all_types = subtypes + [cls_binary]  # 叶→根
            v = sim.fresh('_vdispatch')
            branches = []
            for sub_bin in all_types:
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
                # Fix 12b：重载 mangle 按声明类（owner）查 —— 子类继承的重载
                # 方法在子类方法表中查不到同名重载，按子类表 mangle 会得到
                # 错误的方法名（如 collect 声明处 mangle 为 collect_collec，
                # 子类分支按子类表查成 collect）。owner 沿父类链解析不到时
                # （接口 default 方法由 class_writer 注入实现类 impl 块，
                # 父类链上查不到）保留子类名。
                _owner_bin, _ = _resolve_method_owner(
                    sub_bin, mname, registry, descriptor=jvm_desc)
                _mangle_cls = _owner_bin or sub_rust
                sub_mname_r = _mangle_if_overloaded(_mangle_cls, mname, comment, registry)
                sub_mname_r = _safe_field(sub_mname_r)
                # Fix 17：bridge 分派的参数 downcast —— dispatch 的共享 args 按
                # 擦除描述符 coercion（如 Comparable.compareTo(Object) 的参数为
                # Object），但子类分支的真实方法（bridge 的目标，如
                # Byte.compareTo(Byte)）参数是具体类型 → per-branch 包装
                # .downcast::<T>()（等价 bridge 方法内的 checkcast）。
                _barg_str = arg_str
                if registry:
                    _own_ci17 = registry.get(_owner_bin or sub_bin)
                    if _own_ci17 is not None:
                        _bm17 = None
                        for _m in _own_ci17.methods:
                            if (_m.name == mname and not _m.is_synthetic
                                    and _m.descriptor == jvm_desc):
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
                            if len(_cands17) == 1:
                                _bm17 = _cands17[0]
                        if _bm17 is not None:
                            _bp17 = parse_descriptor_params(_bm17.descriptor)
                            if len(_bp17) == len(args) and _bp17 != list(params):
                                _bparts17 = []
                                for _i17, _bd17 in enumerate(_bp17):
                                    _bt17 = jvm_to_rust(_bd17, registry)
                                    _shared17 = jvm_to_rust(params[_i17], registry)
                                    if (_bt17 not in ('Object', '()')
                                            and _shared17 == 'Object'):
                                        _bparts17.append(
                                            f"({args[_i17]}).downcast::<{_bt17}>()")
                                    else:
                                        _bparts17.append(args[_i17])
                                _barg_str = ', '.join(_bparts17)
                # T76：方法定义在父类（如 getKey 定义在 HashMap_Node，TreeNode
                # 经继承获得）时，dispatch 分支同样需要 _super 链路由，
                # 否则 _d.getKey() E0599（no method in &HashMap_TreeNode）
                _d_recv = '_d'
                _sub_pfx = _find_method_super_prefix_for_type(
                    sub_rust.split('<')[0], mname, registry,
                    descriptor=f"({''.join(params)}){ret}",
                )
                if _sub_pfx:
                    _d_recv = _super_prefix_to_expr('_d', _sub_pfx)
                if rust_ret == '()':
                    branches.append(f"if let Some(_d) = {obj_e}.0.as_any().downcast_ref::<{sub_rust}>() {{ {_d_recv}.{sub_mname_r}({_barg_str})?; }}")
                else:
                    branches.append(f"if let Some(_d) = {obj_e}.0.as_any().downcast_ref::<{sub_rust}>() {{ {_d_recv}.{sub_mname_r}({_barg_str})? }}")
            branches.append(_closure_branch)
            if rust_ret == '()':
                dispatch_code = ' else '.join(branches)
                sim.emit(RawStmt(f"{dispatch_code}"))
            else:
                dispatch_expr = ' else '.join(branches) + f" else {{ Default::default() }}"
                sim.emit(RawStmt(f"let {v}: {rust_ret} = {dispatch_expr};"))
                sim.push(Var(v), RsNamed(rust_ret))
            return
        # 无 subtypes 时（接口无已知实现类）：单独生成闭包 dispatch + 占位
        v = sim.fresh('_vdispatch')
        if rust_ret == '()':
            sim.emit(RawStmt(_closure_branch))
        else:
            dispatch_expr = _closure_branch + f" else {{ Default::default() }}"
            sim.emit(RawStmt(f"let {v}: {rust_ret} = {dispatch_expr};"))
            sim.push(Var(v), RsNamed(rust_ret))
        return
    # vtable body 上下文（this: &ClassName__inner，无 .vtable 字段）→ 直接调用。
    # 仅对当前类自身的 receiver（obj_base 匹配当前类短名）生效；
    # 其他 wrapper 类型的 receiver 仍走常规 vtable 分派检查。
    _cur_class_short = short_cls(class_name) if class_name else ''
    if sim.in_vtable_body and obj_base == _cur_class_short:
        _recv = obj_e
    else:
        _obj_jvm = _rust_type_to_binary(obj_base, registry) if registry else None
        _ci_recv = registry.get(_obj_jvm) if _obj_jvm else None
        if _ci_recv is not None:
            _param_desc = '(' + ''.join(params) + ')' if params is not None else None
            _declared_here = any(
                m.name == mname and (_param_desc is None or m.descriptor.startswith(_param_desc))
                for m in _ci_recv.methods
            )
            _recv = obj_e if _declared_here else f"{obj_e}.vtable"
        else:
            _recv = obj_e  # 手写类 / 不在 registry → 直接调用
    if rust_ret == '()':
        if not obj_is_bare:
            sim.emit(RawStmt(f"{_recv}.{rust_mname}({arg_str})?;"))
    else:
        v = sim.fresh()
        if obj_is_bare and rust_ret not in ('Object', '()') and rust_ret not in _PRIMITIVE_RUST_TYPES:
            sim.emit(RawStmt(f"let {v}: {rust_ret} = Default::default();"))
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
                cls, mname, params, ret, registry,
                caller_class=class_name, caller_tparams=sim.class_type_params,
                receiver_type=obj_ty,
            )
            if (rust_ret == 'Object' and _sig_ret_v is not None
                    and _sig_ret_v != 'Object'):
                sim.emit(RawStmt(f"let {v} = Object::from_any({_recv}.{rust_mname}({arg_str})?);"))
                sim.push(Var(v), RsNamed(rust_ret))
            elif (_sig_ret_v is not None and _sig_ret_v != rust_ret
                    and rust_ret not in _PRIMITIVE_RUST_TYPES):
                sim.emit(RawStmt(f"let {v} = {_recv}.{rust_mname}({arg_str})?;"))
                sim.push(Var(v), RsNamed(_sig_ret_v))
            else:
                sim.emit(RawStmt(f"let {v} = {_recv}.{rust_mname}({arg_str})?;"))
                sim.push(Var(v), RsNamed(rust_ret))
