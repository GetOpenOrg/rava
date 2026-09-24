# 从 codegen/instr/sim.py 中拆出

from ...type_map import short_cls as _short_cls_g
import re as _re_g

from ...stack import BOOL, _clone_moved_var, erased_base, erased_class_of
from ...rs_ir import CastExpr, Lit, RawExpr, RawStmt, NewPendingExpr, StaticFieldRef, RsNamed
from ...render import render_expr, render_type, upcast_expr
from ...sig_parse import parse_field_type as _parse_field_type
from ...sig_types import instance_field_rust_name as _instance_field_rust_name
from ...type_args import (
    ancestor_type_args as _ancestor_type_args,
    class_type_param_bounds as _class_type_param_bounds,
    outer_ref_field_type as _outer_ref_field_type,
    split_rust_type_args as _split_rust_type_args,
    substitute_type_params as _substitute_type_params,
)
from ...type_map import (
    jvm_to_rust,
    parse_class_type_params as _parse_class_type_params,
    effective_class_type_params as _effective_class_type_params,
)
from ...constants import safe_ident as _safe_ident, PRIMITIVE_RUST_TYPES as _PRIMITIVE_RUST_TYPES
from ...constants import PRIMITIVE_RUST_TYPES as _PRIMITIVE_RUST_TYPES
from ..coerce import _coerce_to_object, _coerce_from_null, _coerce_value, _render_cast, _same_generic_family
from ..hierarchy import _is_subtype, _rust_type_to_binary
from ..member_owner import _get_field_generic_signature, _resolve_static_field_owner
from ..member_naming import _parse_field_ref
from ..invoke import _gen_invokespecial
from ..invoke_sig import type_var_receiver_bound_view
from ... import equiv_audit

# Rust 内建容器与已知类型短名（用于泛型类型可见性校验）
# 基本类型名也必须视为可见：装箱类型实参映射为 Rust 基本类型（X<Boolean> → X<bool>），
# 否则此类字段的声明类型恢复被整体拒绝，读取侧退化为擦除形态
_BUILTIN_G: frozenset[str] = frozenset({'Object', 'String', 'Rc', 'Vec', 'RefCell', 'JArray'}) | _PRIMITIVE_RUST_TYPES


def _static_field_decl_class(cls: str, comment: str, registry: dict | None) -> str:
    """getstatic/putstatic 常量池类 → static 字段的真实声明类（解析失败保持原类）。"""
    if not cls or not comment:
        return cls
    _ref = comment.strip().split(' ', 1)[-1].split(':', 1)[0]
    _raw_name = _ref.rsplit('.', 1)[-1]
    return _resolve_static_field_owner(cls, _raw_name, registry) or cls


def _restore_field_declared_type(f_owner: str, fname: str, ftype: str,
                                 class_name: str, registry: dict | None,
                                 sim, recv_ty: str = '') -> str:
    """按字段 generic_signature 恢复声明类型（getfield / putfield 共用）。

    ftype 是 jvm_to_rust(fdesc) 的擦除形态；沿继承链查字段声明，用声明类
    tparams 上下文解析为精确泛型形态（HashMap_Node<Object,Object> →
    HashMap_Node<K,V>、Rc<RefCell<Vec<Object>>> → Rc<RefCell<Vec<E>>>）。
    解析结果中的类型名在调用方不可见（跨类参数名不同）时保持擦除形态。
    """
    if not registry:
        return ftype
    _g_owner = f_owner if f_owner else class_name
    if not _g_owner:
        return ftype
    _ref_ci = registry.get(_g_owner)
    # 字段解析（JVMS §5.4.3.2）：字段引用的限定类是接收者静态类型，声明类可能是其祖先；
    # generic_signature 里的类型变量属于声明类，须在声明类的形参上下文中解析
    _g_ci = _ref_ci
    while _g_ci is not None and not any(
            not _of.is_static and _safe_ident(_of.name) == fname for _of in _g_ci.fields):
        _g_ci = registry.get(_g_ci.super_class) if _g_ci.super_class else None
    if _g_ci is None:
        _g_ci = _ref_ci
    # 内部类外部引用字段（this$N，无 generic_signature）：与 struct 字段定义同规则
    _parsed = ''
    if _g_ci is not None:
        for _of in _g_ci.fields:
            if not _of.is_static and _safe_ident(_of.name) == fname:
                _parsed = _outer_ref_field_type(
                    _of, _effective_class_type_params(_g_ci, registry), registry)
                break
    if not _parsed:
        _gsig = _get_field_generic_signature(_g_owner, fname, registry)
        if not (_gsig and _g_ci is not None):
            return ftype
        # 内部类自身无 generic_signature 时，类型参数继承自外部类（与 struct 定义同规则）
        _decl_tparams = list(_effective_class_type_params(_g_ci, registry) or [])
        _parsed = _parse_field_type(_gsig, _decl_tparams, registry)
    if _parsed and _g_ci is not None and recv_ty:
        # 字段声明在接收者静态类型的祖先上（`this.curChunk`，声明为祖先的 T_ARR）：
        # 祖先类型变量按接收者视角的超类实参代入——与接收者类的转发访问器
        # （superclass_fields 展平）同一规则，读取侧记录的类型与访问器返回类型一致
        _recv_ref = erased_class_of(recv_ty, registry)
        _recv_bin = _recv_ref.binary if _recv_ref is not None else ''
        _recv_ci = registry.get(_recv_bin) if _recv_bin else None
        if _recv_ci is None and _ref_ci is not None and _ref_ci.name != _g_ci.name:
            _recv_ci = _ref_ci
        if _recv_ci is not None and _recv_ci.name != _g_ci.name:
            _anc_view = dict(_ancestor_type_args(_recv_ci, registry, _split_rust_type_args(recv_ty) or None))
            if _g_ci.name in _anc_view:
                _owner_params = list(_effective_class_type_params(_g_ci, registry) or [])
                _owner_args = list(_anc_view[_g_ci.name])
                _parsed = _substitute_type_params(_parsed, {
                    _p: (_owner_args[_i] if _i < len(_owner_args) else 'Object')
                    for _i, _p in enumerate(_owner_params)})
        elif _recv_ci is not None and _recv_ci.name == _g_ci.name:
            # 字段就声明在接收者自己的类上（`Pair<A, B>.second`）：宏访问器
            # `__get_second()` 按接收者实例化返回 B 的实参类型（Pair<.., String>
            # 上返回 String）。声明类类型变量名在调用方不可见时此前直接退回擦除
            # Object，读取侧记录与访问器实际返回类型脱节 —— 紧随的 checkcast
            # 源侧被判为 Object，在已是 String 的 getter 结果上再发
            # `.downcast::<String>()`（E0599，A-3 形态 2）。此处按接收者记录的
            # 类型实参代入，与访问器返回类型对齐；checkcast 同型走 control.py
            # 的 no-op 路径。实参含推断占位 `_` 或为空（裸类名）时不可代入，
            # 保持既有擦除回退。
            _owner_params_s = list(_effective_class_type_params(_g_ci, registry) or [])
            _owner_args_s = list(_split_rust_type_args(recv_ty) or [])
            if (_owner_params_s and len(_owner_args_s) == len(_owner_params_s)
                    and _owner_args_s != _owner_params_s
                    and not any(_re_g.search(r'(?<![\w])_(?![\w])', _a) for _a in _owner_args_s)):
                _parsed = _substitute_type_params(_parsed, dict(zip(_owner_params_s, _owner_args_s)))
    if _parsed and _parsed != 'Object' and _parsed != ftype:
        # 校验：解析结果中的类型名须在调用方可见
        # （当前 impl 类型参数 / registry 短名 / 内建容器），
        # 跨类不可见（声明类参数名与调用方不同）时降级回擦除形态
        _caller_tparams = set(sim.class_type_params) if sim.class_type_params else set()
        _reg_shorts = {_short_cls_g(_k) for _k in registry}
        if all(_n in _caller_tparams or _n in _reg_shorts or _n in _BUILTIN_G
               for _n in _re_g.findall(r'[A-Za-z_][A-Za-z0-9_]*', _parsed)):
            return _parsed
    return ftype


def _resolve_static_field(cls: str, field_name: str, descriptor: str, comment: str,
                          registry) -> tuple[str, str, str, str]:
    """getstatic / putstatic 共用：解析 static 字段的 (声明类, 访问器名, 声明类型, turbofish)。"""
    ty_str = jvm_to_rust(descriptor, registry) if descriptor else 'Object'
    # JVM 字段解析：常量池类可以是子类，static 字段实际声明在祖先类/父接口
    # → 访问器生成在声明类上，读取侧必须解析到声明类（否则 E0599）
    cls = _static_field_decl_class(cls, comment, registry)
    # 泛型类静态字段访问需要 turbofish，避免 E0283 类型推断歧义
    _getstatic_turbofish = ''
    _getstatic_cls_ci = None
    if registry and cls:
        _cls_bin = cls if cls in registry else _rust_type_to_binary(_short_cls_g(cls), registry) if '/' in cls else _rust_type_to_binary(cls.replace('$', '_'), registry)
        if not _cls_bin and cls in registry:
            _cls_bin = cls
        if _cls_bin:
            _getstatic_cls_ci = registry.get(_cls_bin)
            if _getstatic_cls_ci:
                # 有效形参：含内部 / 局部类从外围作用域继承的类型变量（struct 的泛型形参同源）
                _tparams = _effective_class_type_params(_getstatic_cls_ci, registry)
                if _tparams:
                    _getstatic_turbofish = '::<' + ', '.join('Object' for _ in _tparams) + '>'
    # 静态字段声明类型恢复（与 getfield 同规则）：getter 按字段级 generic_signature
    # 生成返回类型（如 HashMap<String, X>），读取侧必须记录同一精确类型，
    # 否则后续 invoke 的返回类型替换拿不到接收者实参，checkcast 误判源类型为 Object
    # 而在 wrapper 上生成 .downcast（E0599）。
    if registry and _getstatic_cls_ci is not None:
        _sgsig = next((_sf.generic_signature for _sf in _getstatic_cls_ci.fields
                       if _sf.is_static and _sf.name == field_name), '')
        if _sgsig:
            _s_tparams = (_parse_class_type_params(_getstatic_cls_ci.generic_signature)
                          if _getstatic_cls_ci.generic_signature else [])
            _s_parsed = _parse_field_type(_sgsig, _s_tparams, registry)
            if _s_parsed and _s_parsed != 'Object' and _s_parsed != ty_str:
                _s_reg_shorts = {_short_cls_g(_k) for _k in registry}
                if all(_n in _s_reg_shorts or _n in _BUILTIN_G
                       for _n in _re_g.findall(r'[A-Za-z_][A-Za-z0-9_]*', _s_parsed)):
                    ty_str = _s_parsed
    # 若字段名与方法名冲突，emitter 生成了 fieldname_field 后缀，调用方也须一致
    _actual_field_name = field_name
    if _getstatic_cls_ci is not None:
        _method_names = {m.name for m in _getstatic_cls_ci.methods}
        if field_name in _method_names:
            _actual_field_name = field_name + '_field'
    return cls, _safe_ident(_actual_field_name), ty_str, _getstatic_turbofish


def _coerce_stored_value(val_expr, val_ty, ftype: str, registry, _obj_str: str = '',
                         slot_is_type_var: bool = False, class_type_params=()) -> str:
    """putfield / putstatic 共用：把栈顶值转换为字段声明类型 ftype 的存储表达式。"""
    val_str_raw = render_expr(val_expr)
    val_ty_name = render_type(val_ty)
    from ...jvm_type import carrier_type_for_ident
    _carrier_stored = carrier_type_for_ident(ftype, registry)
    # Vec<Object>(擦除) ↔ Vec<E>(泛型)：当 ftype 是参数化 Vec 而值按擦除数组还原
    # （CastExpr 目标 JArray<Object>，A-3 节点分派）时，重定向转换目标为泛型版本，
    # 使字段赋值类型一致（旧字符串改写形态在当前语料零触发，见报告）
    if (ftype != val_ty_name
            and 'Vec<' in ftype and 'Vec<Object>' in val_ty_name
            and isinstance(val_expr, CastExpr) and val_expr.target == 'JArray<Object>'):
        val_expr = CastExpr(val_expr.expr, ftype)
        val_str_raw = render_expr(val_expr)
        val_ty_name = ftype
    # null 值（aconst_null → Object::default()）赋给具体类型字段时用 Default::default()
    null_coerce = _coerce_from_null(val_str_raw, ftype)
    if null_coerce is not None:
        val_str = null_coerce
    elif _carrier_stored is not None and _carrier_stored == ftype and val_ty_name != ftype:
        # A-4 批次 3+：字段声明类型是已铺设的接口载体。值 → 载体的边界转换：
        # Object 直接 From<_> 包装（非受检，接口视图按运行时类成立）；具体类 /
        # 其余静态类型先经 _coerce_to_object 保持对象身份。UFCS 形态不可被接口
        # 自带的静态 from 工厂遮蔽（与 _coerce_arg 的载体分支同源）。
        if val_ty_name == 'Object':
            val_str = f"<{ftype} as ::std::convert::From<_>>::from(Clone::clone(&{val_str_raw}))"
        else:
            val_str = f"<{ftype} as ::std::convert::From<_>>::from(" \
                      f"{_coerce_to_object(val_str_raw, val_ty_name, registry, class_type_params)})"
    elif (ftype in (class_type_params or ())
            and val_ty_name != 'Object' and val_ty_name != '()'
            and carrier_type_for_ident(val_ty_name, registry) == val_ty_name):
        # A-4 批次 5：字段声明是类型变量（`S extends Spliterator<T>` 的
        # lastNodeSpliterator: S），值是接口载体（javac 按擦除上界补的 checkcast
        # 在载体化后落在载体上）。经 Object 边界按类型变量的 From<Object> bound
        # 取回（宏为类型形参补的 bound）——载体解包 __ref，S 视图按运行时类成立
        val_str = f"<{ftype} as ::std::convert::From<Object>>::from(" \
                  f"Object::from({val_str_raw}))"
    elif ftype == 'Object' and slot_is_type_var and val_ty_name not in ('Object', '()'):
        # 字段声明为类型变量，但声明类的参数名在调用方不可见（ftype 保持擦除形态）：
        # 槽位类型是接收者的类型实参，值按原类型直接存入。
        val_str = val_str_raw
    elif ftype == 'Object' and val_ty_name not in ('Object', '()'):
        # 字段在 Java 中就声明为 Object（真实多态边界）：任何值（含类型变量值、this）装箱存入。
        # 身份保持的向上转型（Object::from / Into::<Object>），运行时类与接口 vtable 保持可达
        val_str = _coerce_to_object(val_str_raw, val_ty_name, registry, class_type_params)
    elif _same_generic_family(val_ty_name, ftype):
        # raw type / 通配符字段接收精确实例化的值（如自引用的 this）：
        # CastExpr 的擦除路径（A-3，替代已删除的 _reinstantiate_generic 字符串发射）
        val_str = _render_cast(val_str_raw, ftype, box_first=True)
    elif (ftype not in _PRIMITIVE_RUST_TYPES and val_ty_name not in _PRIMITIVE_RUST_TYPES
          and ftype not in ('Object', '()', val_ty_name)
          and _is_subtype(erased_base(val_ty_name), erased_base(ftype), registry)):
        # vtable 架构：子类型赋给祖先类型字段，用 From trait（.into()）
        # 先 Clone::clone(&val) 再 .into()，避免 into() 转移所有权后变量失效（E0382）
        val_str = upcast_expr(val_str_raw, 'clone')
    elif (val_ty_name == 'Object' and ftype not in _PRIMITIVE_RUST_TYPES
          and ftype not in ('Object', '()') and not ftype.startswith('Rc<')):
        # 值经擦除边界（泛型静态方法 <T> T f(T) 等）退化为 Object，
        # 字段声明为具体类/类型参数：checkcast 还原（Java 侧此处是隐式 checkcast）。
        # 统一经 From<Object>（A-1 存储层擦除后对任意类型实参成立，共享存储与
        # 对象标识）；Object::downcast 按精确 TypeId 判定，跨实例化会误抛 CCE。
        val_str = f"From::from(Clone::clone(&{val_str_raw}))"
    else:
        val_str = _coerce_value(val_str_raw, val_ty, ftype)
    # 引用类型赋值时加 Clone::clone()，避免 E0382（move after use）。
    # CastExpr 的两条渲染形态（From 视图 / try_cast）自带 Clone::clone。
    if (val_ty_name not in _PRIMITIVE_RUST_TYPES
            and not val_str.startswith('Default::')
            and '.clone()' not in val_str
            and 'Clone::clone(' not in val_str
            and not isinstance(val_expr, CastExpr)):
        if val_str == 'this' and 'this' in _obj_str:
            val_str = 'Clone::clone(&this)'
        elif val_str != 'this':
            val_str = f'Clone::clone(&{val_str})'
    return val_str


def sim_fields(ins, sim, class_name, registry) -> bool:
    op      = ins.opcode
    operand = ins.operand or ''
    comment = ins.comment or ''

    # ── 对象创建 ──
    if op == 'new':
        raw = (comment or operand).strip()
        if raw.startswith('class '): raw = raw[6:]
        sim.push(NewPendingExpr(raw), RsNamed(_short_cls_g(raw)))

    # ── invokespecial（含构造器）──
    elif op == 'invokespecial':
        _gen_invokespecial(sim, comment, class_name, registry=registry)

    # ── 字段访问 ──
    elif op == 'getfield':
        obj_expr, obj_ty = sim.pop()
        if comment:
            f_owner, fname, fdesc = _parse_field_ref(comment)
            ftype = jvm_to_rust(fdesc, registry) if fdesc else 'Object'
            # 装箱类擦除特例：Integer/Long/Double/Boolean 的签名类型被 type_map
            # 擦除为基本类型（i32/i64/f64/bool）。当 getfield 的接收者已经是
            # 基本类型时（如 compareTo 的 anotherInteger: i32），字段访问就是
            # 值本身 —— 不能生成 __get_value()（基本类型上无此方法，E0599）。
            # 接收者是装箱 struct（如 this: &Integer）时仍走正常访问器。
            _recv_ty = render_type(obj_ty)
            if _recv_ty in _PRIMITIVE_RUST_TYPES:
                sim.push(obj_expr, RsNamed(_recv_ty))
                return True
            # 类型变量接收者（`o.ordinal`，o: E，E extends B<E>）：访问器定义在上界类
            # 的 wrapper 上，类型变量本身没有方法（E0599）。Java 侧该访问经上界类型
            # 静态解析（类型变量擦除为上界）→ Rust 侧经 Object 的 checkcast 视图转换为上界类型
            # （按运行时类重建上界类视图，保留运行时类型）后再读字段。不用 `E: Into<B<E>>` 约束：
            # 覆盖方法不能比 vtable 声明多带约束，约束放 struct 头又会使擦除实例化的证明循环。
            obj_expr, obj_ty = type_var_receiver_bound_view(sim, obj_expr, obj_ty)
            # 字段声明类型恢复：struct 字段生成（class_writer._resolve_field_rust）
            # 优先字段级 generic_signature（如 interfaces: Vec<Class<Object>>、
            # parent: HashMap_TreeNode<K, V>），宏访问器 __get_xxx() 按声明类型返回。
            # 读取侧必须记录同一类型：否则 sim 记录擦除形态（Object /
            # X<Object,Object> / Rc<RefCell<Vec<Object>>>）而表达式实际是
            # 精确泛型形态，局部变量标注 E0308（expected 擦除, found 精确）。
            # 与 putfield 共用同一恢复规则（含内部类外部引用字段 this$N）。
            ftype = _restore_field_declared_type(f_owner, fname, ftype, class_name, registry, sim,
                                                 recv_ty=render_type(obj_ty))
            # 字段读取 → 宏生成的访问器（方案 §7）。
            # 继承字段由子类的转发访问器统一暴露（父类字段在前展平，§6），
            # 所以不再需要按接收者静态类型拼 `_super._super.` 路径——
            # 那层复杂度已收拢进宏（见方案 §16：_super 语义边界）。
            # [equiv-audit] field-npe（S-9）：Object 接收者的 getfield——null
            # 接收者不抛 NPE 的路径，只计数不改发射
            if render_type(obj_ty) == 'Object':
                equiv_audit.record('field-npe')
            _slot = _instance_field_rust_name(f_owner or class_name, fname, registry)
            sim.push(RawExpr(f"{render_expr(obj_expr)}.__get_{_slot}()"), RsNamed(ftype))
        else:
            sim.push(RawExpr(f"{render_expr(obj_expr)}.field"), RsNamed('i32'))

    elif op == 'putfield':
        val_expr, val_ty = sim.pop()
        obj_expr, obj_ty = sim.pop()
        if comment:
            cls_owner, fname, fdesc = _parse_field_ref(comment)
            ftype = jvm_to_rust(fdesc, registry) if fdesc else 'i32'
            # 类型变量接收者（`task.leftChild = x`，task: K）：与 getfield 同规则，经上界类型写字段
            obj_expr, obj_ty = type_var_receiver_bound_view(sim, obj_expr, obj_ty)
            # Fix 15：字段声明类型恢复（与 getfield 对齐，原数组特例泛化）——
            # 擦除形态（HashMap_Node<Object,Object> / Vec<Object>）恢复为声明
            # 类 tparams 上下文的精确泛型形态（HashMap_Node<K,V> / Vec<E>），
            # 与宏访问器 __set_xxx 的参数类型（class_writer 按字段 generic_signature
            # 生成）一致，否则 E0308/E0277（Into 目标是擦除形态、From 不存在）。
            ftype = _restore_field_declared_type(cls_owner, fname, ftype, class_name, registry, sim,
                                                 recv_ty=render_type(obj_ty))
            _slot_gsig = _get_field_generic_signature(cls_owner or class_name, fname, registry) if registry else ''
            val_str = _coerce_stored_value(val_expr, val_ty, ftype, registry, render_expr(obj_expr),
                                           slot_is_type_var=bool(_slot_gsig) and _slot_gsig.startswith('T'),
                                           class_type_params=sim.class_type_params)
            # 字段写入 → 宏生成的 `__set_xxx` 访问器（方案 §7）。
            # 继承字段同样由转发访问器承接，无需 `_super` 前缀路径（§16）。
            # [equiv-audit] field-npe（S-9）：Object 接收者的 putfield，同 getfield
            if render_type(obj_ty) == 'Object':
                equiv_audit.record('field-npe')
            _slot = _instance_field_rust_name(cls_owner or class_name, fname, registry)
            sim.emit(RawStmt(f"{render_expr(obj_expr)}.__set_{_slot}({val_str});"))
        else:
            sim.emit(RawStmt(f"/* putfield {render_expr(val_expr)} */"))

    elif op == 'getstatic':
        cls, field_name, descriptor = _parse_field_ref(comment) if comment else ('', '', '')
        if field_name == '$assertionsDisabled':
            # 合成字段：断言控制标志，始终视为已禁用（= true），等价于以 -da 运行 JVM
            sim.push(Lit('true'), BOOL)
        elif field_name:
            cls, _actual_field_name, ty_str, _getstatic_turbofish = _resolve_static_field(
                cls, field_name, descriptor, comment, registry)
            sim.push(StaticFieldRef(cls, _actual_field_name, RsNamed(ty_str), turbofish=_getstatic_turbofish), RsNamed(ty_str))
        else:
            sim.push(RawExpr(f"/* getstatic {comment} */"), RsNamed('Object'))

    elif op == 'putstatic':
        val_expr, val_ty = sim.pop()
        # Java 引用赋值无 move 语义：putstatic 之后源局部变量仍可被使用
        # （dup; putstatic; areturn 模式）→ setter 实参包 Clone::clone 保活（E0382）
        val_expr = _clone_moved_var(val_expr, val_ty)
        cls, field_name, descriptor = _parse_field_ref(comment) if comment else ('', '', '')
        if cls and field_name:
            cls, rust_fname, _sf_ty, _turbofish = _resolve_static_field(
                cls, field_name, descriptor, comment, registry)
            raw_cls = _short_cls_g(cls)
            if _sf_ty in _PRIMITIVE_RUST_TYPES:
                # JVM 操作数栈上 boolean/byte/char/short 都是 int，写入字段时按字段描述符还原
                val_str = _coerce_value(render_expr(val_expr), val_ty, _sf_ty)
            else:
                val_str = _coerce_stored_value(val_expr, val_ty, _sf_ty, registry,
                                               class_type_params=sim.class_type_params)
            # static 写入 → 宏生成的 set_xxx 访问器（入口触发类初始化，JVMS §5.5）
            sim.emit(RawStmt(f"{raw_cls}{_turbofish}::set_{rust_fname}({val_str})?;"))
        else:
            sim.emit(RawStmt(f"/* putstatic {cls}.{field_name} = {render_expr(val_expr)} */"))

    else:
        return False
    return True
