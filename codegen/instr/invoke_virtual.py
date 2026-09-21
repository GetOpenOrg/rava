# 从 codegen/instr/invoke.py 中拆出；接收者/泛型实参解析段与 _declaring_interface/
# _close_open_type_args 已按 §3.7 上移 member_owner.py
from ..type_map import short_cls as _short_cls_g
from ..stack import StackSim
from .. import inherited_calls as _inherited_calls
from ..rs_ir import Lit, Var, RawExpr, RawStmt, RsNamed
from ..render import render_expr, render_type
from ..type_map import jvm_to_rust, short_cls, parse_descriptor_params, is_jdk
from ..constants import safe_ident as _safe_field
from ..constants import (
    PRIMITIVE_RUST_TYPES as _PRIMITIVE_RUST_TYPES,
    JAVA_RUNTIME_SHORT_NAMES as _JAVA_RUNTIME_SHORT_NAMES,
    OBJECT_CLASS as _OBJECT_CLASS,
)
from .coerce import _coerce_to_object, _render_cast
from .hierarchy import (
    _rust_type_to_binary, _get_all_subtypes_ordered,
    _super_prefix_to_expr,
)
from .member_owner import (
    parse_method_ref,
    _resolve_method_owner, _root_virtual_methods,
    _find_method_super_prefix_for_type,
    _declaring_interface, _close_open_type_args,
    _resolve_virtual_sig_params,
)
from .member_naming import (
    _mangle_if_overloaded, _resolve_bridge_target,
)
from ..type_map import parse_class_type_params as _parse_class_type_params
from ..type_args import ancestor_vtable_args_by_short as _ancestor_vtable_args_by_short
from ..type_args import (ancestor_type_args as _ancestor_type_args,
                         split_rust_type_args as _split_rust_type_args)
from ..type_map import effective_class_type_params as _effective_class_type_params
from .invoke_sig import (_lookup_method_sig_ret, _erased_ret_is_type_var,
                         _coerce_arg, _split_type_args)


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

    # JVM 数组.getClass() → Class::for_class("<数组描述符>")（元素类型静态可知）。
    # 调用方（Arrays.copyOf / copyOfRange）用结果与 Object[].class 比较：
    # `== Object[].class` 为真 → 走 new Object[n] 分支（与 JVM 一致）。
    # 此前发射 Object::default()（null），null 与任何 Class 不等 → 恒走
    # Array.newInstance(newType.getComponentType()) 分支 → null Class 解引用 NPE。
    # 元素类型含无法解析的形态（类型变量等）时保持 null 兜底（调用方按 Object[] 语义
    # 使用结果的场景已由描述符路径覆盖）。
    if mname == 'getClass' and obj_ty.startswith('JArray<'):
        _desc = _jarray_type_desc(obj_ty, registry)
        if _desc is not None:
            v = sim.fresh()
            sim.emit(RawStmt(
                f"let {v}: Class = Class::for_class(String::from(\"{_desc}\"));"))
            sim.push(Var(v), RsNamed('Class'))
            return True
        v = sim.fresh()
        sim.emit(RawStmt(f"let {v}: Object = Object::default();"))
        sim.push(Var(v), RsNamed('Object'))
        return True

    return False


_PRIM_DESC: dict[str, str] = {
    'i8': 'B', 'i16': 'S', 'i32': 'I', 'i64': 'J',
    'f32': 'F', 'f64': 'D', 'bool': 'Z', 'u16': 'C',
}


def _jarray_type_desc(arr_rust: str, registry: dict | None) -> str | None:
    """Rust 数组类型串 → JVM 数组描述符（`JArray<JArray<String>>` → `[[Ljava/lang/String;`）。
    元素类型无法解析（类型变量 / 未知容器）→ None。"""
    elem = arr_rust[len('JArray<'):-1]
    # 嵌套数组递归（JArray<...> 内层可能带空格）
    if elem.startswith('JArray<') and elem.endswith('>'):
        inner = _jarray_type_desc(elem, registry)
        return None if inner is None else '[' + inner
    if elem in _PRIM_DESC:
        return '[' + _PRIM_DESC[elem]
    elem_base = elem.split('<', 1)[0].strip()
    if elem_base == 'Object':
        # java/lang/Object 不经 registry 翻译（运行时根类），描述符恒可知；
        # binary 名引用 constants.OBJECT_CLASS（Python 侧不散置 JDK 类名字面量）
        from ..constants import OBJECT_CLASS as _OBJECT_CLASS
        return '[L' + _OBJECT_CLASS + ';'
    from .hierarchy import _rust_type_to_binary
    bin_name = _rust_type_to_binary(elem_base, registry)
    if not bin_name:
        return None
    return '[L' + bin_name + ';'


def _emit_object_direct_call(sim, obj_e, args, rust_mname, rust_ret) -> bool:
    """bare Object 接收者 + Object 类方法：仅 void / 基本类型返回值走包装器直调。"""
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
        return True
    return False


def _dispatch_bare_object(sim, obj_e, cls, mname, comment, params, ret,
                          args, arg_str, rust_ret, registry):
    """bare Object 接收者的多态分派：接口经载体 / 类经 downcast 链（叶→根）/
    闭包回退（Arch-3）；所有路径发射后返回。"""
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
            _branch = _subtype_branch(registry, sub_bin, mname, comment, params, ret,
                                      args, arg_str, jvm_desc, _root_declared,
                                      rust_ret, obj_e)
            if _branch is not None:
                branches.append(_branch)
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


def _subtype_branch(registry, sub_bin, mname, comment, params, ret,
                    args, arg_str, jvm_desc, _root_declared, rust_ret, obj_e):
    """单个 downcast 分支：owner 解析（Fix 12b）→ bridge 参数 checkcast（Fix 17）→
    _super 路由与协变返回包装；不可编译 / 不可达分支返回 None（跳过）。"""
    _sub_ci_abs = registry.get(sub_bin)
    if _sub_ci_abs is not None and _sub_ci_abs.is_abstract and not _sub_ci_abs.is_interface:
        # 抽象类不可能是对象的运行期类型：downcast 分支恒不命中，不生成
        return None
    sub_rust = jvm_to_rust(f'L{sub_bin};', registry)
    if sub_rust == 'Object':
        # 目标类是接口（jvm_to_rust 对接口返回 Object）：
        # downcast_ref::<Object>() 恒为 Some，会遮蔽闭包回退分支，
        # 且 Object 上没有业务方法（E0599），直接丢弃该分支
        return None
    # Fix 12a：包装类在 JVM_RUST 中映射为基本类型（Boolean→bool、
    # Double→f64 等），downcast_ref::<bool>() 不满足 Any 约束是硬
    # 错误，该分支不可编译 → 丢弃
    if sub_rust in _PRIMITIVE_RUST_TYPES:
        return None
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
        return None
    if not _owner_bin and registry and registry.get(sub_bin) is not None:
        _sub_ci_own = registry[sub_bin]
        if _sub_ci_own.is_abstract and not _sub_ci_own.is_interface:
            # 抽象类自身与祖先链都未声明该方法：对象的运行时类必为其具体子类
            # （各有独立分支），本分支在 Java 语义下不可达，不生成。
            return None
        # 具体类：实现来自祖先注入的接口 default 方法 → 登记继承成员声明
        _inherited_calls.request(sub_bin, mname, '(' + ''.join(params) + ')')
    # 名字视角 = 接收者（_d 的类型 sub_rust：本类覆盖与继承成员都在其 wrapper 上，
    # 名字按接收者重载态）——声明者 _owner_bin 只用于 base 函数 / 继承成员登记
    _mangle_cls = sub_bin or sub_rust
    sub_mname_r = _mangle_if_overloaded(_mangle_cls, mname, comment, registry)
    sub_mname_r = _safe_field(sub_mname_r)
    sub_mname_r, _barg_str, _bm17 = _bridge_downcast_args(
        registry, sub_bin, sub_rust, mname, comment, params, args, arg_str,
        jvm_desc, _bridge_desc, _owner_bin, _mangle_cls, sub_mname_r)
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
        return f"if let Some(_d) = {obj_e}.0.as_any().downcast_ref::<{sub_rust}>() {{ Object::from_any({_call_expr}?) }}"
    elif rust_ret == '()':
        return f"if let Some(_d) = {obj_e}.0.as_any().downcast_ref::<{sub_rust}>() {{ {_call_expr}?; }}"
    else:
        return f"if let Some(_d) = {obj_e}.0.as_any().downcast_ref::<{sub_rust}>() {{ {_call_expr}? }}"


def _bridge_downcast_args(registry, sub_bin, sub_rust, mname, comment, params,
                          args, arg_str, jvm_desc, _bridge_desc, _owner_bin,
                          _mangle_cls, sub_mname_r):
    """Fix 17：bridge 分派的参数 checkcast —— dispatch 共享 args 按擦除描述符
    coercion，子类分支的真实方法参数是具体类型 → per-branch 经 CastExpr 的
    try_cast 还原（等价 bridge 方法内的 checkcast，A-3）。返回
    (可能按真实描述符改写的分支方法名, 分支实参串, 解析出的真实方法 _bm17)。"""
    # Fix 17：bridge 分派的参数 checkcast —— dispatch 的共享 args 按
    # 擦除描述符 coercion（如 Comparable.compareTo(Object) 的参数为
    # Object），但子类分支的真实方法（bridge 的目标，如
    # Byte.compareTo(Byte)）参数是具体类型 → per-branch 经 CastExpr 的
    # try_cast 还原（等价 bridge 方法内的 checkcast，A-3）。
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
                            # （Enum<E> 经子类 SuperclassSignature 得 E=子类自身）→
                            # 具体类型时按 checkcast 语义还原（A-3：try_cast，失败返回
                            # Err 可被 java_try 捕获；binary 无法解析时退 From 视图路径）
                            _tv_inst17 = _tv_map17.get(_gen_params17[_i17][1:-1], 'Object')
                            if _tv_inst17 not in ('Object', '()', '_'):
                                _tv_bin17 = _rust_type_to_binary(
                                    _tv_inst17.split('<')[0], registry)
                                if _tv_bin17:
                                    _bparts17.append(_render_cast(
                                        args[_i17], _tv_inst17,
                                        binary_name=_tv_bin17, checked=True))
                                else:
                                    _bparts17.append(_render_cast(args[_i17], _tv_inst17))
                                continue
                        if (_bt17 not in ('Object', '()')
                                and _shared17 == 'Object'
                                and not _is_type_var17):
                            # 具体形参的隐式 checkcast（bridge 方法内 javac 补的
                            # checkcast）：binary name 直接取自真实描述符（L..; / [..）
                            _bt_bin17 = (_bd17 if _bd17.startswith('[')
                                         else _bd17[1:-1])
                            _bparts17.append(_render_cast(
                                args[_i17], _bt17,
                                binary_name=_bt_bin17, checked=True))
                        else:
                            _bparts17.append(args[_i17])
                    _barg_str = ', '.join(_bparts17)
    return sub_mname_r, _barg_str, _bm17


_SUBCLASS_INDEX: dict[int, dict[str, list[str]]] = {}

# 泛型签名里的类型变量 token（`TP_IN;` / `TV;`；前面的负向后顾排除 `Lfoo/Type;` 形态）
_TYPE_VAR_TOKEN = __import__('re').compile(r'(?<![A-Za-z0-9_$/])T[A-Za-z0-9_$]+;')


def _virtually_dispatched(recv_ci, mname: str, param_desc: str, registry: dict) -> bool:
    """(mname, param_desc) 沿接收者超类链的最近声明是否可安全按子类槽位登记。

    跳过三类：private（invokespecial 静态解析，JLS §8.4.8）、final（不可覆盖，
    运行时行为恒为声明类实现）、声明签名提及声明类类型变量的方法（`TP_IN;` 形态——
    祖先形参代入具体实参的位置，如 `PipelineHelper<P_OUT>` → `PipelineHelper<Integer>`，
    继承成员的 vtable_erasure 按名匹配覆盖不到，槽位签名会与 trait 声明不一致；
    归类继承成员擦除缺口）。
    """
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
                return False
            return True
        cur = registry.get(cur.super_class) if cur.super_class else None
    return True


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
                        and (mname, _param_desc) in _root_virtual_methods()):
                    # 整条祖先链未声明、由根类声明 → 装箱后走根 vtable。
                    # Object::from（非 from_any）：保持接收者的 vtable（运行时类名、
                    # is_instance_of、覆盖的 hashCode/equals/toString），JvmRef 装箱会丢这些
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
            # `this.m(args)` 虚调用（wrapper 体由宏重写为 vtable UFCS 分派、vtable 体直呼
            # trait 方法——均经 this.vtable 的运行时类槽位）：中间祖先覆盖了方法而叶类
            # 未再声明时（AbstractPipeline.opIsStateful 抽象 → StatelessOp 覆盖 → 过滤器
            # 阶段类静默继承），叶类槽位为空会分派到声明类的 trait default（抽象 stub /
            # 声明体），丢失中间覆盖。为每个未自行声明该方法的闭包子类登记继承成员需求
            # （成员体转发到链上最近声明者，等价 JVM 子类 vtable 继承条目）。外部接收者
            # 的调用按静态类型成员分派（sig-poly 站点已按子类分支登记），不在本登记范围。
            # private 方法 invokespecial 静态解析、final 方法不可覆盖，均跳过。
            if (obj_e in ('this', 'self') and not _ci_recv.is_interface
                    and _virtually_dispatched(_ci_recv, mname, _param_desc, registry)):
                for _sub_bin in _closure_subclasses(registry).get(_obj_jvm, ()):
                    _inherited_calls.request(_sub_bin, mname, _param_desc)
    return params, ret, rust_ret, _sig_owner, _sig_recv_ty, _recv, _root_routed


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
    # 若接收方 Rust 类型是 java_runtime 手写类：API 名面固定，仅根类 Object 的
    # 同名重载（wait(J)/wait(JI) → wait_l/wait_l_i，S-20）按描述符后缀取名
    obj_base = obj_ty.split('<')[0].strip()  # 去泛型后缀（ArrayList<T> → ArrayList）
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
    # 直接调用会产生 E0599。若目标类已知且有子类，生成 downcast dispatch 链（多态虚分发）。
    obj_is_bare = (obj_ty == 'Object')
    if obj_is_bare and cls == 'Object' and registry:
        if _emit_object_direct_call(sim, obj_e, args, rust_mname, rust_ret):
            return
    if obj_is_bare and cls and registry:
        _dispatch_bare_object(sim, obj_e, cls, mname, comment, params, ret,
                              args, arg_str, rust_ret, registry)
        return
    params, ret, rust_ret, _sig_owner, _sig_recv_ty, _recv, _root_routed = _resolve_direct_call_sig(
        sim, class_name, cls, mname, params, ret, rust_ret, obj_base, obj_e, obj_ty, registry)
    _emit_call_result(sim, class_name, cls, mname, params, ret, rust_ret, rust_mname,
                      arg_str, obj_e, obj_ty, obj_is_bare, _sig_owner, _sig_recv_ty,
                      _recv, _root_routed, registry)
