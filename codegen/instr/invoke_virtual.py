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
from .invoke_sig import (_lookup_method_sig_params, _lookup_method_sig_ret, _erased_ret_is_type_var,
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
                # 泛型类（如 ArrayList_Itr<E>）：jvm_to_rust 返回裸名，downcast_ref
                # 需要完整泛型实参，用 `_` 通配符让 Rust 自动推断（E0107 防护）。
                # 内部类（ArrayList_Itr）通过 this$0 继承外部类类型参数，
                # 不在自身 generic_signature 中声明，需额外检测。
                if '<' not in sub_rust and registry:
                    _sub_ci_g = registry.get(sub_bin)
                    if _sub_ci_g:
                        import re as _re_icg
                        _tp_g = _parse_class_type_params(_sub_ci_g.generic_signature) if _sub_ci_g.generic_signature else []
                        if not _tp_g:
                            # 内部类：从 this$0 外部类继承类型参数
                            for _fg in _sub_ci_g.fields:
                                if _re_icg.match(r'^this\$\d+$', _fg.name):
                                    _om_g = _re_icg.match(r'L([^;]+);', _fg.descriptor)
                                    if _om_g:
                                        _outer_g = registry.get(_om_g.group(1))
                                        if _outer_g and _outer_g.generic_signature:
                                            _tp_g = _parse_class_type_params(_outer_g.generic_signature)
                                    break
                        if _tp_g:
                            # 使用 Object 作为类型实参（Java 类型擦除语义）：
                            # - 内部类（ArrayList_Itr）的 TypeId 与外部类类型参数绑定
                            # - downcast_ref::<ArrayList_Itr<_>>() 无法推断 `_`（E0283）
                            # - 使用 Object 使代码可编译；iterator() 存储 ArrayList_Itr<E>
                            #   时已做 Object::from_any 擦除，运行时 TypeId 匹配 Object 参数形式
                            sub_rust = sub_rust + '<' + ', '.join(['Object'] * len(_tp_g)) + '>'
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
                _bm17 = None
                if registry:
                    _own_ci17 = registry.get(_owner_bin or sub_bin)
                    if _own_ci17 is not None:
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
                                    if (_bt17 not in ('Object', '()')
                                            and _shared17 == 'Object'
                                            and not _is_type_var17):
                                        _bparts17.append(
                                            f"({args[_i17]}).downcast::<{_bt17}>()")
                                    else:
                                        _bparts17.append(args[_i17])
                                _barg_str = ', '.join(_bparts17)
                # T76：方法定义在父类（如 getKey 定义在 HashMap_Node，TreeNode
                # 经继承获得）时，dispatch 分支需路由到父类方法。
                # 新架构（java_class! 宏）wrapper 不生成 __super() 方法，
                # 改为通过 vtable supertrait UFCS 调用：OwnerVTable::method(&*_d.vtable, args)
                _sub_pfx = _find_method_super_prefix_for_type(
                    sub_rust.split('<')[0], mname, registry,
                    descriptor=f"({''.join(params)}){ret}",
                )
                if _sub_pfx and _owner_bin:
                    # 方法在祖先类 _owner_bin 中声明，使用 <dyn OwnerVTable>::method(&*_d.vtable, args)
                    # 形式消歧义（避免 E0034）且满足 Rust 2021 trait object UFCS 语法。
                    # 不含 `dyn` 的裸 UFCS（TypeName__VTable::method(...)）是 E0782。
                    _owner_short_v = _owner_bin.rsplit('/', 1)[-1].replace('$', '_')
                    _sep_v = ', ' if _barg_str else ''
                    # 提取 sub_rust 的类型实参（如 LinkedHashMap<Object, Object> → 'Object, Object'）
                    import re as _re_ta
                    _ta_m = _re_ta.search(r'<(.+)>$', sub_rust)
                    _sub_type_args = _ta_m.group(1) if _ta_m else ''
                    if '_' in _sub_type_args:
                        # 含通配符 _ 时无法用于 dyn Trait 位置，直接调用 vtable
                        _call_expr = f"_d.vtable.{sub_mname_r}({_barg_str})"
                    else:
                        # 检查 owner 是否泛型，并从 sub_rust 提取匹配的类型实参
                        _owner_type_args_str = ''
                        if registry:
                            _owner_ci_v = registry.get(_owner_bin)
                            if _owner_ci_v:
                                _owner_tps = (
                                    _parse_class_type_params(_owner_ci_v.generic_signature)
                                    if _owner_ci_v.generic_signature else []
                                )
                                if not _owner_tps:
                                    # 内部类：generic_signature 为 None，从 this$0 外部类继承
                                    import re as _re_inner_tp
                                    for _fi_tp in _owner_ci_v.fields:
                                        if _re_inner_tp.match(r'^this\$\d+$', _fi_tp.name):
                                            _om_tp = _re_inner_tp.match(r'L([^;]+);', _fi_tp.descriptor)
                                            if _om_tp:
                                                _outer_ci_v = registry.get(_om_tp.group(1))
                                                if _outer_ci_v and _outer_ci_v.generic_signature:
                                                    _owner_tps = _parse_class_type_params(
                                                        _outer_ci_v.generic_signature)
                                            break
                                if _owner_tps:
                                    if _sub_type_args:
                                        # 从 sub_rust 的类型实参取前 N 个
                                        _sub_args_list = [a.strip() for a in _sub_type_args.split(',')]
                                        if len(_sub_args_list) >= len(_owner_tps):
                                            _owner_type_args_str = '<' + ', '.join(_sub_args_list[:len(_owner_tps)]) + '>'
                                    else:
                                        # sub 无类型实参：从 sub_bin.generic_signature 解析 owner 的类型实参
                                        # e.g., Pattern_Qtype generic_sig = Ljava/lang/Enum<Ljava/util/regex/Pattern$Qtype;>;
                                        _sub_ci_gs = registry.get(sub_bin)
                                        if _sub_ci_gs and _sub_ci_gs.generic_signature:
                                            import re as _re_gsp
                                            _esc_owner = re.escape('L' + _owner_bin)
                                            _gsp_m = _re_gsp.search(_esc_owner + r'<([^>]+)>;?', _sub_ci_gs.generic_signature)
                                            if _gsp_m:
                                                _raw_args = _gsp_m.group(1)
                                                _targs_gs: list[str] = []
                                                _gpos = 0
                                                while _gpos < len(_raw_args):
                                                    _gc = _raw_args[_gpos]
                                                    if _gc == 'L':
                                                        _ge = _raw_args.index(';', _gpos)
                                                        _targs_gs.append(
                                                            _raw_args[_gpos+1:_ge].rsplit('/', 1)[-1].replace('$', '_'))
                                                        _gpos = _ge + 1
                                                    elif _gc == 'T':
                                                        _ge = _raw_args.index(';', _gpos)
                                                        _targs_gs.append('Object')
                                                        _gpos = _ge + 1
                                                    else:
                                                        _gpos += 1
                                                if _targs_gs and len(_targs_gs) >= len(_owner_tps):
                                                    _owner_type_args_str = '<' + ', '.join(_targs_gs[:len(_owner_tps)]) + '>'
                        # 完全限定 UFCS：<dyn SubVTable<STA> as VtableOwnerVTable<OTA>>::method(...)
                        # VtableOwner 是方法的真实 VTable 声明类（virtual_in 值），
                        # 可能与 Java owner 不同（如 containsKey virtual_in=AbstractMap 但 Java owner=HashMap）。
                        # 使用 "as VtableOwner" 消除多 supertrait 路径的 E0034 歧义。
                        _vtable_owner_v = _owner_short_v  # 默认与 Java owner 相同
                        if registry:
                            from codegen.emitter.vtable_util import _find_virtual_in as _fvi
                            _owner_ci_vt = registry.get(_owner_bin)
                            if _owner_ci_vt:
                                _param_part_vt = jvm_desc.split(')')[0] + ')' if jvm_desc else ''
                                for _m_vt in _owner_ci_vt.methods:
                                    if (_m_vt.name == mname and not _m_vt.is_synthetic
                                            and (_m_vt.descriptor.startswith(_param_part_vt) if _param_part_vt else True)):
                                        _vi = _fvi(_m_vt, _owner_ci_vt, registry)
                                        if _vi:
                                            _vtable_owner_v = _vi
                                        break
                        _sub_vtable_short = sub_rust.split('<')[0] + '__VTable'
                        _sub_vtable_ta = ('<' + _sub_type_args + '>') if _sub_type_args else ''
                        _call_expr = (f"<dyn {_sub_vtable_short}{_sub_vtable_ta} as {_vtable_owner_v}__VTable{_owner_type_args_str}>::{sub_mname_r}"
                                      f"(&*_d.vtable{_sep_v}{_barg_str})")
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
    _ufcs_vtable_prefix = None  # 若非 None，改写为 UFCS：<dyn _ufcs_vtable_prefix>::rust_mname(&*obj.vtable, args)
    if sim.in_vtable_body and obj_base == _cur_class_short:
        _recv = obj_e
    else:
        _obj_jvm = _rust_type_to_binary(obj_base, registry) if registry else None
        _ci_recv = registry.get(_obj_jvm) if _obj_jvm else None
        if _ci_recv is not None:
            _param_desc = '(' + ''.join(params) + ')' if params is not None else None
            _declared_here = any(
                not m.is_synthetic and m.name == mname and (_param_desc is None or m.descriptor.startswith(_param_desc))
                for m in _ci_recv.methods
            )
            if _declared_here:
                _recv = obj_e
            else:
                # 方法继承自父类：解析 owner 以生成 UFCS，消除同名方法多 VTable 来源的 E0034
                _jvm_desc_v = '(' + ''.join(params) + ')' + ret
                _owner_bin_v, _ = _resolve_method_owner(_obj_jvm, mname, registry, descriptor=_jvm_desc_v)
                if _owner_bin_v and _owner_bin_v != _obj_jvm:
                    _owner_short_v = _owner_bin_v.rsplit('/', 1)[-1].replace('$', '_')
                    _ufcs_vtable_prefix = f"<dyn {_owner_short_v}__VTable>"
                    _recv = f"&*{obj_e}.vtable"
                else:
                    _recv = f"{obj_e}.vtable"
        else:
            _recv = obj_e  # 手写类 / 不在 registry → 直接调用
    # 生成方法调用表达式（普通形式 or UFCS 形式）
    def _build_call(mname_r, recv, args):
        if _ufcs_vtable_prefix:
            sep = ', ' if args else ''
            return f"{_ufcs_vtable_prefix}::{mname_r}({recv}{sep}{args})"
        return f"{recv}.{mname_r}({args})"

    if rust_ret == '()':
        if not obj_is_bare:
            sim.emit(RawStmt(f"{_build_call(rust_mname, _recv, arg_str)}?;"))
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
