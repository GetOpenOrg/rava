# 从 codegen/instr/sim.py 中拆出

import re as _re_g

from ...stack import BOOL, _clone_moved_var
from ...rs_ir import Lit, RawExpr, RawStmt, NewPendingExpr, StaticFieldRef, RsNamed
from ...render import render_expr, render_type
from ...type_map import (
    jvm_to_rust,
    parse_class_type_params as _parse_class_type_params,
    parse_field_type as _parse_field_type,
    effective_class_type_params as _effective_class_type_params,
    outer_ref_field_type as _outer_ref_field_type,
    class_type_param_bounds as _class_type_param_bounds,
)
from ...constants import safe_ident as _safe_ident, PRIMITIVE_RUST_TYPES as _PRIMITIVE_RUST_TYPES
from ..coerce import (
    _parse_field_ref, _coerce_to_object, _coerce_from_null, _coerce_value,
    _is_subtype, _rust_type_to_binary, _get_field_generic_signature,
    _PRIMITIVE_RUST_TYPES, _into_super_chain, _resolve_static_field_owner,
    _reinstantiate_generic,
)
from ..invoke import _gen_invokespecial

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
                                 sim) -> str:
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
    _g_ci = registry.get(_g_owner)
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
        _decl_tparams = (_parse_class_type_params(_g_ci.generic_signature)
                         if _g_ci.generic_signature else [])
        _parsed = _parse_field_type(_gsig, _decl_tparams, registry)
    if _parsed and _parsed != 'Object' and _parsed != ftype:
        # 校验：解析结果中的类型名须在调用方可见
        # （当前 impl 类型参数 / registry 短名 / 内建容器），
        # 跨类不可见（声明类参数名与调用方不同）时降级回擦除形态
        _caller_tparams = set(sim.class_type_params) if sim.class_type_params else set()
        _reg_shorts = {_k.rsplit('/', 1)[-1].replace('$', '_') for _k in registry}
        if all(_n in _caller_tparams or _n in _reg_shorts or _n in _BUILTIN_G
               for _n in _re_g.findall(r'[A-Za-z_][A-Za-z0-9_]*', _parsed)):
            return _parsed
    return ftype


def sim_fields(ins, sim, class_name, registry) -> bool:
    op      = ins.opcode
    operand = ins.operand or ''
    comment = ins.comment or ''

    # ── 对象创建 ──
    if op == 'new':
        raw = (comment or operand).strip()
        if raw.startswith('class '): raw = raw[6:]
        sim.push(NewPendingExpr(raw), RsNamed(raw.split('/')[-1]))

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
            # 静态解析 → Rust 侧把值转换为上界类型（宏生成的 From<Child> for Ancestor，
            # vtable upcast 保留运行时类型）后再读字段；所需约束 `E: Into<B<E>>`
            # 记入 sim，由方法签名声明为 where 子句。
            if registry and _recv_ty in (sim.class_type_params or ()):
                _cur_ci = registry.get(class_name) if class_name else None
                _tv_bound = (_class_type_param_bounds(_cur_ci, registry).get(_recv_ty)
                             if _cur_ci is not None else None)
                if _tv_bound is not None:
                    _src = _clone_moved_var(obj_expr, obj_ty)
                    obj_expr = RawExpr(f"Into::<{_tv_bound[0]}>::into({render_expr(_src)})")
                    obj_ty = RsNamed(_tv_bound[0])
                    sim.type_var_bound_uses[_recv_ty] = _tv_bound[0]
            # 字段声明类型恢复：struct 字段生成（class_writer._resolve_field_rust）
            # 优先字段级 generic_signature（如 interfaces: Vec<Class<Object>>、
            # parent: HashMap_TreeNode<K, V>），宏访问器 __get_xxx() 按声明类型返回。
            # 读取侧必须记录同一类型：否则 sim 记录擦除形态（Object /
            # X<Object,Object> / Rc<RefCell<Vec<Object>>>）而表达式实际是
            # 精确泛型形态，局部变量标注 E0308（expected 擦除, found 精确）。
            if registry:
                _g_owner = f_owner if f_owner else class_name
                _g_ci = registry.get(_g_owner) if _g_owner else None
                _gsig = _get_field_generic_signature(_g_owner, fname, registry) if _g_owner else None
                if _gsig and _g_ci is not None:
                    # 用声明类的类型参数解析（签名中的类型变量属于声明类上下文）
                    _decl_tparams = (_parse_class_type_params(_g_ci.generic_signature)
                                     if _g_ci.generic_signature else [])
                    _parsed = _parse_field_type(_gsig, _decl_tparams, registry)
                    if _parsed and _parsed != 'Object' and _parsed != ftype:
                        # 校验：解析结果中的类型名须在调用方可见
                        # （当前 impl 类型参数 / registry 短名 / 内建容器），
                        # 跨类不可见（声明类参数名与调用方不同）时降级回擦除形态
                        _caller_tparams = set(sim.class_type_params) if sim.class_type_params else set()
                        _reg_shorts = {_k.rsplit('/', 1)[-1].replace('$', '_') for _k in registry}
                        if all(_n in _caller_tparams or _n in _reg_shorts or _n in _BUILTIN_G
                               for _n in _re_g.findall(r'[A-Za-z_][A-Za-z0-9_]*', _parsed)):
                            ftype = _parsed
            # 字段读取 → 宏生成的访问器（方案 §7）。
            # 继承字段由子类的转发访问器统一暴露（父类字段在前展平，§6），
            # 所以不再需要按接收者静态类型拼 `_super._super.` 路径——
            # 那层复杂度已收拢进宏（见方案 §16：_super 语义边界）。
            sim.push(RawExpr(f"{render_expr(obj_expr)}.__get_{fname}()"), RsNamed(ftype))
        else:
            sim.push(RawExpr(f"{render_expr(obj_expr)}.field"), RsNamed('i32'))

    elif op == 'putfield':
        val_expr, val_ty = sim.pop()
        obj_expr, obj_ty = sim.pop()
        if comment:
            cls_owner, fname, fdesc = _parse_field_ref(comment)
            ftype = jvm_to_rust(fdesc, registry) if fdesc else 'i32'
            # Fix 15：字段声明类型恢复（与 getfield 对齐，原数组特例泛化）——
            # 擦除形态（HashMap_Node<Object,Object> / Vec<Object>）恢复为声明
            # 类 tparams 上下文的精确泛型形态（HashMap_Node<K,V> / Vec<E>），
            # 与宏访问器 __set_xxx 的参数类型（class_writer 按字段 generic_signature
            # 生成）一致，否则 E0308/E0277（Into 目标是擦除形态、From 不存在）。
            ftype = _restore_field_declared_type(cls_owner, fname, ftype, class_name, registry, sim)
            val_str_raw = render_expr(val_expr)
            val_ty_name = render_type(val_ty)
            # Vec<Object>(擦除) ↔ Vec<E>(泛型)：当 ftype 是参数化 Vec 而 val 是擦除 Vec 时，
            # 将 val 的 downcast 目标类型替换为泛型版本，使字段赋值类型一致
            if (ftype != val_ty_name
                    and 'Vec<' in ftype and 'Vec<Object>' in val_ty_name
                    and isinstance(val_expr, RawExpr)
                    and 'downcast::<JArray<Object>>' in val_str_raw):
                val_str_raw = val_str_raw.replace(
                    'downcast::<JArray<Object>>',
                    f'downcast::<{ftype}>'
                )
                val_ty_name = ftype
            # null 值（aconst_null → Object::default()）赋给具体类型字段时用 Default::default()
            null_coerce = _coerce_from_null(val_str_raw, ftype)
            if null_coerce is not None:
                val_str = null_coerce
            elif ftype == 'Object' and val_ty_name not in ('Object', '()') and val_str_raw != 'this':
                # 若值的类型是泛型参数（单大写字母如 T/E/K/V），字段槽位类型就是该参数本身，
                # 直接赋值（不再有 JField<T> 包装）
                if len(val_ty_name) <= 2 and val_ty_name[0].isupper() and val_ty_name.rstrip('0123456789').isalpha():
                    val_str = val_str_raw
                else:
                    val_str = _coerce_to_object(val_str_raw, val_ty_name, registry, sim.class_type_params)
            elif _reinstantiate_generic(val_str_raw, val_ty_name, ftype) is not None:
                # raw type / 通配符字段接收精确实例化的值（如自引用的 this）
                val_str = _reinstantiate_generic(val_str_raw, val_ty_name, ftype)
            elif (ftype not in _PRIMITIVE_RUST_TYPES and val_ty_name not in _PRIMITIVE_RUST_TYPES
                  and ftype not in ('Object', '()', val_ty_name)
                  and _is_subtype(val_ty_name.split('<')[0], ftype.split('<')[0], registry)):
                # vtable 架构：子类型赋给祖先类型字段，用 From trait（.into()）
                # 先 Clone::clone(&val) 再 .into()，避免 into() 转移所有权后变量失效（E0382）
                chain = _into_super_chain(val_ty_name.split('<')[0], ftype.split('<')[0], registry)
                val_str = f"Clone::clone(&{val_str_raw}){chain}"
            elif (val_ty_name == 'Object' and ftype not in _PRIMITIVE_RUST_TYPES
                  and ftype not in ('Object', '()') and not ftype.startswith('Rc<')):
                # 值经擦除边界（泛型静态方法 <T> T f(T) 等）退化为 Object，
                # 字段声明为具体类/类型参数：downcast 还原（Java 侧此处是隐式 checkcast）
                val_str = f"({val_str_raw}).downcast::<{ftype}>()"
            else:
                val_str = _coerce_value(val_str_raw, val_ty, ftype)
            # 引用类型赋值时加 Clone::clone()，避免 E0382（move after use）
            _obj_str = render_expr(obj_expr)
            if (val_ty_name not in _PRIMITIVE_RUST_TYPES
                    and not val_str.startswith('Default::')
                    and '.clone()' not in val_str
                    and '.downcast::<' not in val_str
                    and 'Clone::clone(' not in val_str):
                if val_str == 'this' and 'this' in _obj_str:
                    val_str = 'Clone::clone(&this)'
                elif val_str != 'this':
                    val_str = f'Clone::clone(&{val_str})'
            # 字段写入 → 宏生成的 `__set_xxx` 访问器（方案 §7）。
            # 继承字段同样由转发访问器承接，无需 `_super` 前缀路径（§16）。
            sim.emit(RawStmt(f"{render_expr(obj_expr)}.__set_{fname}({val_str});"))
        else:
            sim.emit(RawStmt(f"/* putfield {render_expr(val_expr)} */"))

    elif op == 'getstatic':
        cls, field_name, descriptor = _parse_field_ref(comment) if comment else ('', '', '')
        if field_name == '$assertionsDisabled':
            # 合成字段：断言控制标志，始终视为已禁用（= true），等价于以 -da 运行 JVM
            sim.push(Lit('true'), BOOL)
        elif field_name:
            ty_str = jvm_to_rust(descriptor, registry) if descriptor else 'Object'
            # JVM 字段解析：常量池类可以是子类，static 字段实际声明在祖先类/父接口
            # → 访问器生成在声明类上，读取侧必须解析到声明类（否则 E0599）
            cls = _static_field_decl_class(cls, comment, registry)
            # 泛型类静态字段访问需要 turbofish，避免 E0283 类型推断歧义
            _getstatic_turbofish = ''
            _getstatic_cls_ci = None
            if registry and cls:
                _cls_bin = cls if cls in registry else _rust_type_to_binary(cls.rsplit('/', 1)[-1].replace('$', '_'), registry) if '/' in cls else _rust_type_to_binary(cls.replace('$', '_'), registry)
                if not _cls_bin and cls in registry:
                    _cls_bin = cls
                if _cls_bin:
                    _getstatic_cls_ci = registry.get(_cls_bin)
                    if _getstatic_cls_ci and _getstatic_cls_ci.generic_signature:
                        _tparams = _parse_class_type_params(_getstatic_cls_ci.generic_signature)
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
                        _s_reg_shorts = {_k.rsplit('/', 1)[-1].replace('$', '_') for _k in registry}
                        if all(_n in _s_reg_shorts or _n in _BUILTIN_G
                               for _n in _re_g.findall(r'[A-Za-z_][A-Za-z0-9_]*', _s_parsed)):
                            ty_str = _s_parsed
            # 若字段名与方法名冲突，emitter 生成了 fieldname_field 后缀，调用方也须一致
            _actual_field_name = field_name
            if _getstatic_cls_ci is not None:
                _method_names = {m.name for m in _getstatic_cls_ci.methods}
                if field_name in _method_names:
                    _actual_field_name = field_name + '_field'
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
            cls = _static_field_decl_class(cls, comment, registry)
            raw_cls = cls.rsplit('/', 1)[-1].replace('$', '_')
            rust_fname = _safe_ident(field_name)
            val_str = render_expr(val_expr)
            # 基本类型静态字段与 putfield 同规则收窄：JVM 操作数栈上 boolean/byte/char/short
            # 都是 int，写入字段时按字段描述符还原（bool ← `x != 0`，i8/i16/u16 ← as）
            _sf_ty = jvm_to_rust(descriptor, registry) if descriptor else ''
            if _sf_ty in _PRIMITIVE_RUST_TYPES:
                val_str = _coerce_value(val_str, val_ty, _sf_ty)
            sim.emit(RawStmt(f"{raw_cls}::set_{rust_fname}({val_str});"))
        else:
            sim.emit(RawStmt(f"/* putstatic {cls}.{field_name} = {render_expr(val_expr)} */"))

    else:
        return False
    return True
