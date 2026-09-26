# 从 codegen/instr/sim.py 中拆出

from ...rs_ir import LetStmt, Lit, RawExpr, RawStmt, RsNamed, Var
from ... import equiv_audit
from ...render import render_expr, render_type
from ...sig_types import method_sig_types
from ...stack import I32
from ...type_map import (jvm_to_rust, parse_descriptor_params, parse_descriptor_return, short_cls,
                         effective_class_type_params)
from ..invoke import _gen_string_concat, _static_call_turbofish
from ...classfile import _PRIM_CLASS_TO_WRAPPER
from ...constants import (PRIMITIVE_RUST_TYPES as _PRIMITIVE_RUST_TYPES,
                          safe_ident as _safe_field)
from ..coerce import _coerce_to_object, _render_cast, _same_generic_family
from ..member_naming import (lambda_impl_rust_name, LAMBDA_NAME_LEDGER,
                             _mangle_if_overloaded)


def _monitor_operand(obj_expr) -> str:
    """monitorenter / monitorexit 的操作数 → 保持对象身份的 Object 表达式。

    - `this`（&Self / owned Self）：`Object::from(Clone::clone(this))`——
      vtable 上下文的归类（classify_vtable_body）按 `Clone::clone(this)`
      子串路由到 wrapper 重建；owned-this（构造器）由 _normalize_this_clone 归一。
    - 其余表达式：`Into::<Object>::into(Clone::clone(&expr))`（Object 自反 /
      wrapper / String / Class / JArray 均满足 blanket From）。
    同一对象的多次装箱共享存储身份（__identity 单元），监视器挂接点唯一。"""
    if isinstance(obj_expr, Var) and obj_expr.name == 'this':
        return 'Object::from(Clone::clone(this))'
    return f'Into::<Object>::into(Clone::clone(&({render_expr(obj_expr)})))'


def _unbox_object_arg(val_expr: str, prim_desc: str, registry) -> str | None:
    """Object 形态的 SAM 实参 → 实现方法基本类型形参（LambdaMetafactory 装箱适配
    的拆箱侧，S-3.1 残留）：`x.try_cast::<Integer>("java/lang/Integer")?.intValue()?`。

    SAM 实参经擦除边界以 Object 流动，其中是装箱对象（真实 wrapper 实例）；
    实现方法形参是基本类型（`Integer::sum` 的 (II)I 适配 BinaryOperator 的
    (Object,Object)）。wrapper 二进制名由 primitive 描述符映射
    （classfile._PRIM_CLASS_TO_WRAPPER），拆箱方法与 Rust 类型名从 registry 的
    wrapper 类动态解析（实例方法、零实参、名以 Value 结尾、返回该基本类型，
    P-1 不引入 JDK 类名字面量）。解析失败返回 None，调用方退化为直接传参
    （编译期 E0308 暴露，而非运行期空类名 CCE）。"""
    if registry is None or prim_desc not in _PRIM_CLASS_TO_WRAPPER:
        return None
    wrapper_bin = _PRIM_CLASS_TO_WRAPPER[prim_desc]
    wci = registry.get(wrapper_bin)
    if wci is None:
        return None
    unbox_m = next((m for m in wci.methods
                    if not m.is_static and m.name.endswith('Value')
                    and parse_descriptor_params(m.descriptor) == []
                    and parse_descriptor_return(m.descriptor) == prim_desc), None)
    if unbox_m is None:
        return None
    wrapper_rust = jvm_to_rust(f'L{wci.name};', registry)
    if wrapper_rust == 'Object':
        return None
    mname_r = _safe_field(_mangle_if_overloaded(
        wci.name, unbox_m.name, f'{unbox_m.name}:{unbox_m.descriptor}', registry))
    return f'{val_expr}.try_cast::<{wrapper_rust}>("{wci.name}")?.{mname_r}()?'


def _box_prim_via_valueof(val_expr: str, prim_desc: str, registry) -> str | None:
    """基本类型表达式 → Object 形态的 SAM 返回值（LambdaMetafactory 装箱适配的
    装箱侧，S-3.1）：`Object::from(Integer::valueOf_i(x)?)`。

    实现方法返回基本类型、SAM 返回擦除引用（`Integer::sum` 的 (II)I 对
    BinaryOperator 的 (Object,Object)Object）。必须经 wrapper 的 valueOf 工厂
    装成真实包装对象（含缓存池语义，与 javac 行为一致）：`.into()` 产生的是
    原生盒（Rc<i32>，类名报告 java/lang/Integer 但非翻译 Integer），折叠状态
    再入 SAM 时 try_cast::<Integer> 无法命中。valueOf 方法与 Rust 类型名从
    registry 的 wrapper 类动态解析（static、单实参为该基本类型、返回该
    wrapper，P-1 不引入 JDK 类名字面量）。解析失败返回 None，调用方退回
    `_coerce_to_object` 的 `.into()` 原生盒路径。"""
    if registry is None or prim_desc not in _PRIM_CLASS_TO_WRAPPER:
        return None
    wrapper_bin = _PRIM_CLASS_TO_WRAPPER[prim_desc]
    wci = registry.get(wrapper_bin)
    if wci is None:
        return None
    valueof_m = next((m for m in wci.methods
                      if m.is_static and m.name == 'valueOf'
                      and parse_descriptor_params(m.descriptor) == [prim_desc]
                      and parse_descriptor_return(m.descriptor) == f'L{wci.name};'), None)
    if valueof_m is None:
        return None
    wrapper_rust = jvm_to_rust(f'L{wci.name};', registry)
    if wrapper_rust == 'Object':
        return None
    mname_r = _safe_field(_mangle_if_overloaded(
        wci.name, valueof_m.name, f'{valueof_m.name}:{valueof_m.descriptor}', registry))
    return f'Object::from({wrapper_rust}::{mname_r}({val_expr})?)'


def _decode_tslabels(comment: str) -> list[tuple[str, str]] | None:
    """comment 中提取 typeSwitch 的 case 标签序列（S-17）。

    格式 `tslabels:c:bin0,c:bin1,...`（值内 % / , / 空白百分号编码，见
    classfile._decode_bytecode）。无 tslabels 令牌返回 None。"""
    for tok in comment.split(' '):
        if tok.startswith('tslabels:'):
            labels: list[tuple[str, str]] = []
            for item in tok[len('tslabels:'):].split(','):
                if not item:
                    continue
                kind, _, val = item.partition(':')
                val = val.replace('%20', ' ').replace('%2C', ',').replace('%25', '%')
                labels.append((kind, val))
            return labels
    return None


def _gen_type_switch(sim, comment: str, registry) -> None:
    """S-17: `SwitchBootstraps.typeSwitch` 调用点 → 运行时标签判定链。

    调用点描述符 `(Ljava/lang/Object;I)I`：栈上 (selector, restart) → 命中下标。
    JVM 语义（SwitchBootstraps javadoc / createMethodHandleSwitch）：
      - selector 为 null → -1（`case null` 由 javac 在 switch 前用 ifnull 单独编译）
      - 自 restart 起首个匹配标签的下标；未命中 → len(labels)（tableswitch default 消化）
      - Class 标签 = 运行时 instanceof；连续相同标签由「首个下标先判」等价去重
    guarded pattern（`case X when g`）无需感知：guard 失败路径将 restart 置为下一
    case 下标后跳回本调用点（CFG 回边 → 循环），`restart <= i` 守卫跳过已否决标签。
    case 绑定（`case X var`）也无需感知：分支内 javac 显式 checkcast + astore。
    """
    labels = _decode_tslabels(comment) or []
    # 调用点固定弹 (restart: i32, selector: 引用)
    restart_e, _restart_t = sim.pop()
    sel_e, sel_t = sim.pop()
    restart_s = render_expr(restart_e)
    # selector 求值一次（JVM 语义），统一到 Object 后由 ObjectVTable::is_instance_of
    # 按运行时类判定（G-9）；具体类型经 Object 边界装箱保持对象身份
    sel_ty = render_type(sel_t)
    if sel_ty == 'Object':
        obj_expr = sel_e if isinstance(sel_e, (Var, Lit)) else sim.fresh_let('__ts_sel', sel_e, sel_t)
    else:
        coerced = _coerce_to_object(render_expr(sel_e), sel_ty, registry, sim.class_type_params)
        obj_expr = sim.fresh_let('__ts_sel', RawExpr(coerced), RsNamed('Object'))
    obj_s = render_expr(obj_expr)
    if any(kind not in ('c', 's', 'i') for kind, _ in labels):
        # EnumDesc 等（CONSTANT_Dynamic 形态）暂未落地 —— 保持可见的占位失败，
        # 归类报告，不静默给错值
        sim.emit(RawStmt(
            '/* TODO S-17: typeSwitch 含未支持的标签形态（'
            + ','.join(kind for kind, _ in labels if kind not in ('c', 's', 'i'))
            + '），退化为占位 */'))
        sim.push(RawExpr('Object::default()'), RsNamed('Object'))
        return
    # null → -1；否则按标签序判定（`restart <= i` 守卫实现 restart 语义）；未命中 → len。
    # 标签谓词（SwitchBootstraps.typeSwitch 语义）：
    #   'c' Class 标签 = 运行时 instanceof；'s' String 常量 = label.equals(selector)；
    #   'i' Integer 常量 = selector instanceof Integer 且值相等（判定桥见 java_runtime
    #   lib.rs 的 _ts_str_label_eq / _ts_int_label_eq，经 prelude 导出）
    import json as _json_ts
    chain = f'if _is_jnull(&{obj_s}) {{ -1 }} else '
    for i, (_kind, _lval) in enumerate(labels):
        if _kind == 'c':
            _pred = f'{obj_s}.is_instance_of("{_lval}")'
        elif _kind == 's':
            _pred = f'_ts_str_label_eq({_json_ts.dumps(_lval)}, &{obj_s})'
        else:  # 'i'
            _pred = f'_ts_int_label_eq({_lval}, &{obj_s})'
        chain += f'if {restart_s} <= {i} && {_pred} {{ {i} }} else '
    chain += f'{{ {len(labels)} }}'
    idx = sim.fresh_let('__ts_idx', RawExpr(chain), I32)
    sim.push(idx, I32)


def sim_dynamic(ins, sim, class_name, registry) -> bool:
    op      = ins.opcode
    operand = ins.operand or ''
    comment = ins.comment or ''

    # ── invokedynamic ──
    if op == 'invokedynamic':
        if comment and 'makeConcatWithConstants' in comment:
            _gen_string_concat(sim, comment, registry)
        elif comment and 'tslabels:' in comment:
            _gen_type_switch(sim, comment, registry)
        else:
            # 解析 comment 格式：
            # "InvokeDynamic samName:dynDesc [impl:Cls.method:implDesc] [samtype:samDesc]"
            _dyn_desc = ''
            _impl_method_ref = ''  # "Cls.lambda$main$0:desc"
            _sam_type_desc = ''
            if comment:
                _ctoks = comment.split(' ')
                # 第 2 个词是 "samName:dynDesc"
                if len(_ctoks) >= 2:
                    _nd = _ctoks[1]
                    _ci = _nd.find(':')
                    if _ci >= 0:
                        _dyn_desc = _nd[_ci + 1:]
                for _tok in _ctoks[2:]:
                    if _tok.startswith('impl:'):
                        _impl_method_ref = _tok[5:]
                    elif _tok.startswith('samtype:'):
                        _sam_type_desc = _tok[8:]

            # Arch-3: 若有 impl: 和 samtype:，生成真实 Rust 闭包
            _lam_idx = operand or '0'
            _cap_exprs: list[tuple] = []
            if _dyn_desc.startswith('('):
                _cap_descs = parse_descriptor_params(_dyn_desc)
                for _ci_idx, _cd in enumerate(_cap_descs):
                    if sim.stack:
                        _ce, _cty = sim.pop()
                        _cap_exprs.insert(0, (_ce, _cty, _ci_idx))
                    else:
                        _cap_exprs.insert(0, (RawExpr('Object::default()'), RsNamed('Object'), _ci_idx))

            if _impl_method_ref and _sam_type_desc:
                # 解析实现方法：Cls.method:desc 或 pkg/Cls.method:desc
                _dot = _impl_method_ref.rfind('.')
                _impl_colon = _impl_method_ref.find(':', _dot) if _dot >= 0 else -1
                if _dot >= 0 and _impl_colon > _dot:
                    _impl_cls_bin = _impl_method_ref[:_dot]           # "TestLambda" or "pkg/Cls"
                    _impl_mname   = _impl_method_ref[_dot+1:_impl_colon]  # "lambda$main$0"
                    _impl_desc    = _impl_method_ref[_impl_colon+1:]  # "(I)I"
                    # 转换为 Rust 标识符
                    _impl_cls_rust  = short_cls(_impl_cls_bin)      # 内部类 `$` → `_`，与定义侧一致
                    # G-10：实现方法名取 lambda_impl_rust_name 单一来源（定义侧 class_writer
                    # 同源取名），调用点引用名在此登记，生成收尾由账本断言两侧恒等
                    _impl_mname_r = lambda_impl_rust_name(
                        _impl_cls_bin, _impl_mname, _impl_desc, registry)
                    LAMBDA_NAME_LEDGER.record_reference(_impl_cls_bin, _impl_mname, _impl_mname_r)
                    # SAM 方法参数/返回类型 → Rust 类型
                    _sam_params = parse_descriptor_params(_sam_type_desc)
                    _sam_ret    = parse_descriptor_return(_sam_type_desc)
                    _sam_ptypes = [jvm_to_rust(p, registry) for p in _sam_params]
                    _sam_rtype  = jvm_to_rust(_sam_ret, registry) if _sam_ret != 'V' else '()'
                    # 捕获变量声明
                    _cap_var_stmts: list[str] = []
                    _cap_var_names: list[str] = []
                    for _cv_idx, (_cexpr, _cty, _) in enumerate(_cap_exprs):  # _cap_exprs 已按声明顺序排列（pop 时 insert(0)）
                        _cv_name = f'__lam_cap{_lam_idx}_{_cv_idx}'
                        # Java 捕获的是引用副本：被捕获的局部变量在闭包创建后仍可使用，
                        # 因此按 Clone 捕获而非 move（否则 E0382 use after move）
                        _cap_var_stmts.append(f'let {_cv_name} = Clone::clone(&{render_expr(_cexpr)});')
                        _cap_var_names.append(_cv_name)
                    # SAM 参数名
                    _sam_anames = [f'_la{i}' for i in range(len(_sam_ptypes))]
                    # Fn 类型签名（Result 用裸名：user crate 里 crate::error 是 E0433，两边均经 prelude 引入）
                    _fn_params_sig = ', '.join(f'{_a}: {_t}' for _a, _t in zip(_sam_anames, _sam_ptypes))
                    _fn_type = f'std::rc::Rc<dyn Fn({", ".join(_sam_ptypes)}) -> Result<{_sam_rtype}>>'
                    # 调用实现方法的参数列表（捕获变量 + SAM 参数）
                    # Clone::clone 而非 .clone()：捕获值可能是带 Java clone() 的类
                    # 实例实现方法（捕获 this 的 lambda / 绑定接收者的方法引用）：
                    # 第一个捕获值是接收者，以 &self 形式传入
                    # 构造器引用（<init>）非 static 但无接收者：全部实参进参数表，返回新实例
                    _impl_is_ctor = _impl_mname == '<init>'
                    _impl_is_instance = False
                    _impl_has_generic_sig = False
                    _impl_tparams: list[str] = []
                    _impl_sig_types: list = []
                    _impl_ci = registry.get(_impl_cls_bin) if registry else None
                    if _impl_ci is not None:
                        for _im in _impl_ci.methods:
                            if _im.name == _impl_mname and _im.descriptor == _impl_desc:
                                _impl_is_instance = not _im.is_static and not _impl_is_ctor
                                _impl_has_generic_sig = bool(_im.generic_signature)
                                _impl_tparams = effective_class_type_params(_impl_ci, registry)
                                _impl_sig_types = (method_sig_types(_impl_ci, _im, _impl_tparams, registry)[0]
                                                   or [])
                                break

                    def _is_erased_ref(_d: str) -> bool:
                        """描述符类型在 Rust 侧是否表现为擦除的 Object（接口别名 / 未翻译类）。"""
                        if not (_d.startswith('L') and _d.endswith(';')):
                            return False
                        _dci = registry.get(_d[1:-1]) if registry else None
                        return _dci is None or _dci.is_interface or jvm_to_rust(_d, registry) == 'Object'

                    # 函数式接口的擦除签名（samtype）与实现方法签名之间的适配：
                    # SAM 实参是擦除的 Object、实现方法形参是具体类 → 拆箱（目标类型由形参推断）
                    _impl_params = parse_descriptor_params(_impl_desc)
                    if (_impl_ci is None and not _impl_is_ctor):
                        # impl 类不在 registry（java/lang/Object 等运行时手写层类）：其实例
                        # 方法是 &self 形态（描述符不含接收者）。接收者来自两处之一：
                        #   - SAM 首参（未绑定 X::m，A-3）：SAM 实参数恰比描述符形参多一个
                        #   - 捕获首值（绑定 recv::m / this::m）：捕获数恰比描述符形参多一个
                        if len(_impl_params) + 1 == len(_sam_params):
                            _impl_is_instance = True
                        elif _cap_var_names and len(_cap_var_names) == len(_impl_params) + 1:
                            _impl_is_instance = True
                    _call_cap_list = [f'Clone::clone(&{v})' for v in _cap_var_names]
                    _call_sam_list = list(_sam_anames)
                    from ...jvm_type import carrier_type_for_ident as _carrier_of
                    # 实现方法形参表对应 (捕获值 + SAM 实参) 去掉接收者之后的部分
                    _recv_from_sam = _impl_is_instance and not _call_cap_list
                    _sam_param_offset = len(_call_cap_list) - (1 if _impl_is_instance and _call_cap_list else 0)
                    for _si, _sd in enumerate(_sam_params):
                        if _recv_from_sam and _si == 0:
                            continue
                        _pi = _sam_param_offset + _si - (1 if _recv_from_sam else 0)
                        if 0 <= _pi < len(_impl_params):
                            _pd = _impl_params[_pi]
                            _pd_rust = jvm_to_rust(_pd, registry)
                            _pd_carrier = _carrier_of(_pd_rust, registry)
                            if (_pd_carrier is not None and _pd_carrier == _pd_rust
                                    and _pd != _sd and _is_erased_ref(_sd)):
                                # A-4 批次 3+：实现方法形参是已铺设接口载体——SAM 擦除
                                # 实参经载体的 From<Object> 非受检包装（UFCS 不被接口
                                # 自带静态 from 工厂遮蔽，与 _coerce_arg 载体分支同源）
                                _call_sam_list[_si] = (
                                    f'<{_pd_carrier} as ::std::convert::From<Object>>'
                                    f'::from({_sam_anames[_si]})')
                            elif _pd != _sd and _is_erased_ref(_sd) and not _is_erased_ref(_pd):
                                if _pd in _PRIM_CLASS_TO_WRAPPER:
                                    # SAM 擦除实参是装箱对象、实现方法形参是基本类型
                                    # （`Integer::sum` 的 (II)I 适配 BinaryOperator 的
                                    # (Object,Object)）：cast 到 wrapper 后调拆箱方法
                                    # （此前 _pd[1:-1] 切出空类名，运行期必 CCE，S-3.1）
                                    _call_sam_list[_si] = (
                                        _unbox_object_arg(_sam_anames[_si], _pd, registry)
                                        or _sam_anames[_si])
                                else:
                                    # SAM 擦除实参 → 实现方法具体形参：checkcast 语义
                                    # （A-3：try_cast 失败返回 Err 可被 java_try 捕获，S-1；
                                    # 目标类型由形参推断，turbofish 不写）
                                    _sam_bin = _pd if _pd.startswith('[') else _pd[1:-1]
                                    _call_sam_list[_si] = f'{_sam_anames[_si]}.try_cast("{_sam_bin}")?'
                            elif (_is_erased_ref(_sd) and not _impl_ci.is_interface
                                  and len(_impl_sig_types) == len(_impl_params)
                                  and _impl_sig_types[_pi] in _impl_tparams):
                                # 实现方法形参是声明类的类型变量（`this::addLast`，addLast(E)）：
                                # SAM 的擦除实参经宏补的 From<Object> bound 取回类型变量视图
                                _call_sam_list[_si] = f'From::from({_sam_anames[_si]})'
                            elif len(_sd) == 1 and _pd_rust == 'Object':
                                # SAM 实参是基本类型、实现方法形参是引用（metafactory 的装箱适配）
                                _call_sam_list[_si] = f'{_sam_anames[_si]}.into()'
                    # 捕获值 → 实现方法形参：形参是擦除引用（接口 / Object）而捕获值是具体类实例
                    # （List<X> 形参捕获 ArrayList 局部）→ Java 的隐式上转，保持对象标识
                    _cap_recv = 1 if (_impl_is_instance and _call_cap_list) else 0
                    for _ci_idx in range(_cap_recv, len(_call_cap_list)):
                        _cp_idx = _ci_idx - _cap_recv
                        _cap_ty = render_type(_cap_exprs[_ci_idx][1])
                        _cp_rust = (jvm_to_rust(_impl_params[_cp_idx], registry)
                                    if _cp_idx < len(_impl_params) else '')
                        _cp_carrier = _carrier_of(_cp_rust, registry)
                        if (_cp_idx < len(_impl_params)
                                and _cp_carrier is not None and _cp_carrier == _cp_rust):
                            # A-4 批次 3+：实现方法形参是已铺设接口载体——本分支完全
                            # 消费，不再落入下方擦除装箱分支。捕获值已是同载体 → 保持
                            # 列表初始 Clone::clone(&v)；是 Object / 具体类 → 经 Object
                            # 边界的非受检查体包装（与 _coerce_arg 同源）
                            if _cap_ty == 'Object':
                                # Fn 闭包可多次调用：Clone 取值，不 move 捕获变量（E0507）
                                _call_cap_list[_ci_idx] = (
                                    f'<{_cp_carrier} as ::std::convert::From<_>>'
                                    f'::from(Clone::clone(&{_cap_var_names[_ci_idx]}))')
                            elif _cap_ty not in ('()', '_') and _cap_ty != _cp_carrier:
                                _call_cap_list[_ci_idx] = (
                                    f'<{_cp_carrier} as ::std::convert::From<_>>::from('
                                    f'{_coerce_to_object(_cap_var_names[_ci_idx], _cap_ty, registry, sim.class_type_params)})')
                            continue
                        elif (_cp_idx < len(_impl_params) and _is_erased_ref(_impl_params[_cp_idx])
                                and _cap_ty not in ('Object', '()', '_')
                                and _cap_ty not in _PRIMITIVE_RUST_TYPES
                                and not _cap_ty.startswith(('JArray<', 'Vec<', '&'))):
                            _call_cap_list[_ci_idx] = _coerce_to_object(
                                _cap_var_names[_ci_idx], _cap_ty, registry, sim.class_type_params)
                        elif _cp_idx < len(_impl_params):
                            # 合成 lambda 方法的形参是擦除实例化（X<Object, Object>），捕获值是精确
                            # 实例化（X<T, bool>）：经 Object 边界重新实例化。
                            # 形参类型与方法定义侧同源：有泛型签名取签名，否则取描述符擦除形态
                            # （A-3：CastExpr 的擦除路径，替代已删除的 _reinstantiate_generic）
                            _cap_expected = (_impl_sig_types[_cp_idx]
                                             if len(_impl_sig_types) == len(_impl_params)
                                             else jvm_to_rust(_impl_params[_cp_idx], registry))
                            if _same_generic_family(_cap_ty, _cap_expected or ''):
                                _call_cap_list[_ci_idx] = _render_cast(
                                    _cap_var_names[_ci_idx], _cap_expected, box_first=True)
                    if _impl_is_instance and _call_cap_list:
                        # 捕获 this 的 lambda / 绑定接收者的方法引用：第一个捕获值是接收者
                        if _impl_ci is None and not _impl_is_ctor:
                            # 实现类在运行时手写层（java/lang/Object 等）：UFCS 第一实参是
                            # &self（实现类形态）——捕获接收者是调用点子类实例，先经
                            # Object 边界上转（保持对象身份，虚分派在 vtable 上进行）
                            _call_cap_list[0] = (f'&Object::from(Clone::clone(&{_cap_var_names[0]}))')
                        else:
                            _call_cap_list[0] = f'&{_cap_var_names[0]}'
                    elif _recv_from_sam and _call_sam_list:
                        # 未绑定接收者的方法引用（X::method）：第一个 SAM 实参是接收者
                        _recv_desc = f'L{_impl_cls_bin};'
                        if _is_erased_ref(_sam_params[0]) and not _is_erased_ref(_recv_desc):
                            # 擦除的 SAM 接收者 → 实现类具体接收者：checkcast 语义
                            # （A-3：try_cast 失败返回 Err，可被 java_try 捕获，S-1）
                            _recv_ty = jvm_to_rust(_recv_desc, registry)
                            _call_sam_list[0] = f'&{_sam_anames[0]}.try_cast::<{_recv_ty}>("{_impl_cls_bin}")?'
                        else:
                            _call_sam_list[0] = f'&{_sam_anames[0]}'
                    _call_cap_args  = ', '.join(_call_cap_list)
                    _call_sam_args  = ', '.join(_call_sam_list)
                    _all_call_args  = ', '.join(filter(None, [_call_cap_args, _call_sam_args]))
                    # 生成闭包
                    for _s in _cap_var_stmts:
                        sim.emit(RawStmt(_s))
                    _cap_move = ' '.join(f'Clone::clone(&{v}),' for v in _cap_var_names)
                    # 泛型类上的实现方法：静态方法不使用类的类型参数，闭包内无上下文可推断（E0283），
                    # 与 invokestatic 同规则显式给出 turbofish；实例实现方法由接收者类型推断，无需给出
                    _impl_turbofish = ('' if _impl_is_instance else
                                       _static_call_turbofish(_impl_cls_bin, class_name, sim, registry))
                    _closure_body = f'{_impl_cls_rust}{_impl_turbofish}::{_impl_mname_r}({_all_call_args})'
                    if _impl_is_instance and _impl_ci is not None and _impl_ci.is_interface:
                        # 实现方法是接口实例方法（`action::accept`）：接收者是擦除的接口引用，
                        # 经与接口同名的载体分派（与 invokeinterface 同形态）
                        _all_args = _call_cap_list + _call_sam_list
                        _iface_tps = effective_class_type_params(_impl_ci, registry)
                        _iface_targs = f"<{', '.join(['Object'] * len(_iface_tps))}>" if _iface_tps else ''
                        _iface_recv = _all_args[0].lstrip('&')
                        _iface_recv_src = f"Clone::clone(&{_iface_recv})"
                        if _call_cap_list:
                            # 绑定接收者是捕获值：静态类型为具体类（`list::add`，list 是 ArrayList<E>）
                            # 时先按对象标识上转为擦除的接口引用
                            _recv_cap_ty = render_type(_cap_exprs[0][1])
                            if (_recv_cap_ty not in ('Object', '()', '_')
                                    and _recv_cap_ty not in _PRIMITIVE_RUST_TYPES):
                                _iface_recv_src = _coerce_to_object(
                                    _iface_recv, _recv_cap_ty, registry, sim.class_type_params)
                        _closure_body = (f"Into::<{_impl_cls_rust}{_iface_targs}>::into({_iface_recv_src})"
                                         f".{_impl_mname_r}({', '.join(_all_args[1:])})")
                                        # 返回值适配：SAM 返回 void → 丢弃实现方法返回值；
                    # SAM 返回擦除的 Object 而实现方法返回具体类型 → 装箱
                    _impl_ret = f'L{_impl_cls_bin};' if _impl_is_ctor else parse_descriptor_return(_impl_desc)
                    if _sam_ret == 'V':
                        if _impl_ret != 'V':
                            _closure_body = f'{_closure_body}?; Ok(())'
                    elif _is_erased_ref(_sam_ret) and _impl_ret in _PRIM_CLASS_TO_WRAPPER:
                        # 实现方法返回基本类型、SAM 返回擦除引用（`Integer::sum` 的 I 对
                        # BinaryOperator 的 Ljava/lang/Object;）：经 wrapper::valueOf
                        # 装成真实包装对象（S-3.1）——`.into()` 的原生盒（Rc<i32>）类名
                        # 虽是 java/lang/Integer，但 downcast 不命中翻译 Integer，
                        # 折叠状态再入 SAM 时 try_cast::<Integer> 必失败
                        _boxed = _box_prim_via_valueof(f'{_closure_body}?', _impl_ret, registry)
                        if _boxed is not None:
                            _closure_body = f'Ok({_boxed})'
                        else:
                            _impl_ret_rust = jvm_to_rust(_impl_ret, registry)
                            _closure_body = (f'Ok({_coerce_to_object(f"{_closure_body}?", _impl_ret_rust, registry, sim.class_type_params)})')
                    elif _is_erased_ref(_sam_ret) and _impl_ret != 'V' and (
                            not _is_erased_ref(_impl_ret) or _impl_has_generic_sig
                            or _carrier_of(jvm_to_rust(_impl_ret, registry), registry)
                            == jvm_to_rust(_impl_ret, registry)):
                        # 按实现方法的返回类型装箱（S-3.1）：registry 类（Integer 等
                        # 翻译类）走 Object::from —— 对象身份、运行时类与接口 vtable
                        # 全部可达；仅未知形态（闭包等）才 from_any 不透明包装。
                        # A-4 批次 3+：返回是已铺设载体（`List<Object>` 等）也走本支——
                        # _coerce_to_object 对载体发射 Object::from（解包 __ref 装箱）
                        _impl_ret_rust = jvm_to_rust(_impl_ret, registry)
                        _closure_body = (f'Ok({_coerce_to_object(f"{_closure_body}?", _impl_ret_rust, registry, sim.class_type_params)})')
                    _lam_varname = f'__lam_{_lam_idx}'
                    # A-5 lambda 对象化：samtype 是函数式接口（且可合成，预扫描定案）时，
                    # 闭包经合成对象装箱——`Object::from(I__Lambda::new(Rc::new(closure)))`。
                    # 合成对象实现接口闭包的 __VTable（SAM 直调闭包 / default 经载体
                    # __default_<m> 体）并应答 __interface 查询；不可合成（非函数式 /
                    # 接口未发射 / 手写覆盖）回落闭包装箱（from_any + downcast 回退域）。
                    # 函数对象以 Object（函数式接口的擦除形态）绑定为 LetStmt：
                    # 在 try / 分支体内创建、体外消费时由变量提升 pass 管理作用域
                    from ...emitter.sam_objects import site_ctor_path, record_site
                    _sam_iface_bin = ''
                    if _sam_type_desc.startswith('('):
                        _iface_ret = parse_descriptor_return(_dyn_desc)
                        if _iface_ret.startswith('L') and _iface_ret.endswith(';'):
                            _sam_iface_bin = _iface_ret[1:-1]
                    _sam_ctor = (site_ctor_path(_sam_iface_bin, class_name)
                                 if _sam_iface_bin else None)
                    if _sam_ctor is not None:
                        record_site(_sam_iface_bin, _sam_type_desc, class_name)
                        _lam_box_expr = (f'Object::from({_sam_ctor}(std::rc::Rc::new('
                                         f'move |{_fn_params_sig}| -> Result<{_sam_rtype}> '
                                         f'{{ {_closure_body} }})))')
                    else:
                        _lam_box_expr = (f'Object::from_any(std::rc::Rc::new('
                                         f'move |{_fn_params_sig}| -> Result<{_sam_rtype}> '
                                         f'{{ {_closure_body} }}) as {_fn_type})')
                    sim.emit(LetStmt(_lam_varname, RsNamed('Object'), False, RawExpr(
                        _lam_box_expr)))
                    sim.push(Var(_lam_varname), RsNamed('Object'))
                else:
                    sim.emit(RawStmt(f"/* TODO: {op} {operand} (impl parse failed) */"))
                    if _sam_type_desc:
                        _r2 = parse_descriptor_return(_sam_type_desc)
                        if _r2 != 'V':
                            sim.push(RawExpr('Object::default()'), RsNamed('Object'))
            else:
                # 短期占位（无 impl 信息，如方法引用 REF_invokeVirtual 等）
                sim.emit(RawStmt(f"/* TODO: {op} {operand} */"))
                _ret_desc = parse_descriptor_return(_dyn_desc) if _dyn_desc else 'V'
                if _ret_desc != 'V':
                    sim.push(RawExpr('Object::default()'), RsNamed('Object'))

    # ── 同步（S-20 真实化：可重入监视器，单线程语义不变）──
    elif op == 'monitorenter':
        # JVMS §6.5：弹出 objectref，进入其监视器（null → NPE 由运行时承载）。
        # 操作数装箱为 Object（保持对象身份——wrapper 克隆共享存储的 __identity
        # 单元），重复 acquire/release 经身份命中同一监视器。
        equiv_audit.record('monitor-mt')   # 条件等价：多线程互斥语义（S-11）
        obj_expr, _ty = sim.pop()
        sim.emit(RawStmt(f'{_monitor_operand(obj_expr)}.monitor_enter()?;'))
    elif op == 'monitorexit':
        obj_expr, _ty = sim.pop()
        sim.emit(RawStmt(f'{_monitor_operand(obj_expr)}.monitor_exit()?;'))

    # ── 杂项 ──
    elif op in ('nop', 'wide'): pass
    elif op == 'athrow':
        # 被抛出的就是栈顶对象本身：JvmError 携带该对象，异常表匹配 / getMessage /
        # 未捕获报告都基于它的运行时类（参考文档 §8.3）
        e_expr, _ = sim.pop()
        _thrown = render_expr(e_expr)
        # 局部变量被抛出：按值克隆（Java 引用无 move 语义）。java_try! 的 catch 体改写
        # return 的控制流后，借用检查不再视 `return Err(from(t))` 为路径终点——同一 catch
        # 体中之后对 t 的使用会报 E0382（MethodHandleImpl.guardWithCatch 实证）
        if isinstance(e_expr, Var):
            _thrown = 'Clone::clone(this)' if _thrown == 'this' else f'Clone::clone(&{_thrown})'
        sim.emit(RawStmt(f'return Err(JvmError::from({_thrown}));'))
    else:
        return False
    return True
