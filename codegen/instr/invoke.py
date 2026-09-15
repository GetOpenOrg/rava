"""
invoke 指令生成器：invokespecial / invokestatic / invokevirtual / invokedynamic(string concat)。
"""

import re
from ..stack import StackSim
from ..rs_ir import (
    Lit, Var, RawExpr, RawStmt, NewPendingExpr, RsNamed,
)
from ..render import render_expr, render_type
from ..type_map import jvm_to_rust, short_cls, parse_descriptor_params
from ..sig_parser import parse_class_type_params as _parse_class_type_params
from ..constants import safe_ident as _safe_field
from .coerce import (
    parse_method_ref, _coerce_from_null, _coerce_to_object,
    _coerce_to_interface, _coerce_value, _find_super_chain_to_class,
    _find_method_super_prefix, _find_method_super_prefix_for_type,
    _mangle_if_overloaded, _class_known, _is_subtype, _rust_type_to_binary,
    BOXING_SKIP_STATIC, UNBOX_VIRTUAL, _PRIMITIVE_RUST_TYPES,
    _JAVA_RUNTIME_SHORT_NAMES,
)


def _is_generic_type_param(ty: str) -> bool:
    """检测 ty 是否是泛型类型参数（单个大写字母，如 T/E/K/V）——这种情况下不能用 Object::from_any 包装。"""
    return bool(ty) and len(ty) <= 2 and ty[0].isupper() and ty.rstrip('0123456789').isalpha()


def _gen_string_concat(sim: StackSim, comment: str):
    """处理 invokedynamic makeConcatWithConstants 字符串拼接。
    结果为 java.lang.String（通过 String::from(format!(...)) 转换）。
    """
    desc_m = re.search(r'makeConcatWithConstants:(\([^)]*\))', comment)
    desc = desc_m.group(1) + 'Ljava/lang/String;' if desc_m else '(Ljava/lang/String;)Ljava/lang/String;'
    params = parse_descriptor_params(desc)

    args = []
    for p in reversed(params):
        e_expr, _ = sim.pop()
        raw = render_expr(e_expr)
        # Java 浮点数格式化：整数值需显示 .0（如 5.0 而非 5）
        if p in ('D',):
            raw = f'java_fmt_f64({raw})'
        elif p in ('F',):
            raw = f'java_fmt_f32({raw})'
        elif p in ('C',):
            # Java char (u16) 必须转为 Rust char 才能以字符形式格式化
            raw = f"char::from_u32({raw} as u32).unwrap_or('?')"
        args.insert(0, raw)

    tmpl_m = re.search(r' template:(.+)$', comment)
    if tmpl_m:
        template = tmpl_m.group(1)
        parts = template.split('\x01')
        if len(parts) == len(args) + 1:
            fmt_str = ''.join(
                (p.replace('{', '{{').replace('}', '}}') + '{}' if i < len(args)
                 else p.replace('{', '{{').replace('}', '}}'))
                for i, p in enumerate(parts)
            )
            fmt_args = ', '.join(args)
            if fmt_args:
                # 用 from_owned 避免 From<&str> vs From<std::string::String> 歧义
                sim.push(Lit(f'String::from_owned(format!("{fmt_str}", {fmt_args}))'), RsNamed('String'))
            else:
                sim.push(Lit(f'String::from("{fmt_str}")'), RsNamed('String'))
            return

    # fallback
    if not args:
        sim.push(Lit('String::new()'), RsNamed('String'))
    elif len(args) == 1:
        sim.push(Lit(f'String::from_owned(format!("{{}}", {args[0]}))'), RsNamed('String'))
    else:
        fmt = '{}'.join([''] * (len(args) + 1))  # "{}{}{}" for 3 args
        fmt_args = ', '.join(args)
        sim.push(Lit(f'String::from_owned(format!("{fmt}", {fmt_args}))'), RsNamed('String'))


def _gen_invokespecial(sim: StackSim, comment: str, class_name: str, registry: dict | None = None):
    if '<init>' not in comment and '"<init>"' not in comment:
        # super.method() 调用（invokespecial 非构造器）：
        # 不能用 invokevirtual 语义——否则 this.method() 会对被覆盖方法产生无限递归。
        # 必须精确路由到目标父类的 ._super 链，直接调用父类实现，跳过虚拟派发。
        cls_short, mname, params, ret = parse_method_ref(comment)
        args: list[str] = []
        for param_jvm in reversed(params):
            e_expr, e_ty_node = sim.pop()
            e_str = render_expr(e_expr)
            expected_rust = jvm_to_rust(param_jvm, registry)
            actual_rust = render_type(e_ty_node)
            null_coerce = _coerce_from_null(e_str, expected_rust)
            if null_coerce is not None:
                e_str = null_coerce
            elif _coerce_to_interface(actual_rust, expected_rust):
                e_str = 'Default::default()'
            elif expected_rust == 'Object' and actual_rust not in ('Object', '()') and not _is_generic_type_param(actual_rust):
                e_str = _coerce_to_object(e_str, actual_rust)
            elif expected_rust in ('bool', 'i8', 'i16', 'u16') and actual_rust != expected_rust:
                e_str = _coerce_value(e_str, e_ty_node, expected_rust)
            elif actual_rust not in _PRIMITIVE_RUST_TYPES:
                e_str = f"{e_str}.clone()"
            args.insert(0, e_str)
        obj_expr, _ = sim.pop()
        obj_e = render_expr(obj_expr)
        # 找到 ._super 链：class_name（当前类 binary）→ cls_short（目标父类 Rust 短名）
        super_pfx = _find_super_chain_to_class(class_name, cls_short or '', registry) if registry else '_super.'
        recv_e = f"{obj_e}.{super_pfx.rstrip('.')}" if super_pfx else obj_e
        rust_mname = _safe_field(_mangle_if_overloaded(cls_short or '', mname, comment, registry))
        arg_str = ', '.join(args)
        rust_ret = jvm_to_rust(ret, registry)
        if rust_ret == '()':
            sim.emit(RawStmt(f"{recv_e}.{rust_mname}({arg_str})?;"))
        else:
            v = sim.fresh()
            sim.emit(RawStmt(f"let {v} = {recv_e}.{rust_mname}({arg_str})?;"))
            sim.push(Var(v), RsNamed(rust_ret))
        return

    cls, _, params, _ = parse_method_ref(comment)
    args = []
    for param_jvm in reversed(params):
        e_expr, e_ty_node = sim.pop()
        e = render_expr(e_expr)
        expected = jvm_to_rust(param_jvm, registry)
        ty = render_type(e_ty_node)
        null_coerce = _coerce_from_null(e, expected)
        if null_coerce is not None:
            e = null_coerce
        elif _coerce_to_interface(ty, expected):
            e = 'Default::default()'
        elif expected == 'Object' and ty not in ('Object', '()') and e != 'this' and not _is_generic_type_param(ty):
            e = _coerce_to_object(e, ty)
        elif expected == 'Object' and ty not in ('Object', '()') and e == 'this':
            e = f"Object::from_any(self.clone())"
        elif expected in ('bool', 'i8', 'i16', 'u16') and ty != expected:
            e = _coerce_value(e, e_ty_node, expected)
        elif expected == 'i32' and ty in ('i8', 'i16', 'u16', 'bool'):
            e = f"({e} as i32)"
        elif ty not in _PRIMITIVE_RUST_TYPES:
            e = f"{e}.clone()"
        args.insert(0, e)
    obj_expr, obj_ty_node = sim.pop()

    if isinstance(obj_expr, NewPendingExpr):
        full_cls = obj_expr.class_name          # e.g. 'java/util/ArrayList'
        raw_cls = full_cls.rsplit('/', 1)[-1]
        raw_cls = short_cls(raw_cls) or raw_cls

        if '/' in full_cls:
            # JDK class（含包路径）→ 用 new() 工厂（@synthetic）
            rust_ty_str = jvm_to_rust(f'L{full_cls};', registry)
            # 自动装箱优化：原始包装类型（Integer→i32等）直接用值，跳过构造器调用
            _PRIM_TYPES = frozenset({'i32', 'i64', 'f32', 'f64', 'bool', 'i8', 'i16', 'u16'})
            if rust_ty_str in _PRIM_TYPES:
                init_expr = args[0] if args else '0'
                rust_ty = rust_ty_str
                rust_ty_node = RsNamed(rust_ty_str)
            elif rust_ty_str != 'Object' and '<' in rust_ty_str:
                type_params_str = rust_ty_str[len(raw_cls):]   # '<Object>' / '<Object, Object>'
                rust_ty = raw_cls + type_params_str
                rust_ty_node = RsNamed(rust_ty)
                # 重载构造器：用 '<init>' 查重载再替换为 'new'，以匹配 method.py 生成的定义
                _init_mangled = _mangle_if_overloaded(full_cls, '<init>', comment, registry)
                ctor_name = _safe_field(_init_mangled.replace('<init>', 'new'))
                turbofish = '::' + type_params_str
                init_expr = f"{raw_cls}{turbofish}::{ctor_name}({', '.join(args)})?"
            else:
                type_params_str = ''
                rust_ty = raw_cls
                rust_ty_node = RsNamed(rust_ty)
                # 重载构造器：用 '<init>' 查重载再替换为 'new'，以匹配 method.py 生成的定义
                _init_mangled = _mangle_if_overloaded(full_cls, '<init>', comment, registry)
                ctor_name = _safe_field(_init_mangled.replace('<init>', 'new'))
                if args:
                    init_expr = f"{raw_cls}::{ctor_name}({', '.join(args)})?"
                else:
                    init_expr = f"{raw_cls}::{ctor_name}()?"
        elif raw_cls and '/' not in raw_cls:
            # 用户类：new()? 返回 Result<Self>，同样 mangle 重载构造器
            _init_mangled2 = _mangle_if_overloaded(raw_cls, '<init>', comment, registry)
            ctor_name    = _safe_field(_init_mangled2.replace('<init>', 'new'))
            init_expr    = f"{raw_cls}::{ctor_name}({', '.join(args)})?"
            rust_ty      = raw_cls
            rust_ty_node = RsNamed(rust_ty)
        else:
            init_expr    = f"/* {raw_cls}::new() */"
            rust_ty      = raw_cls
            rust_ty_node = RsNamed(rust_ty)

        if sim.stack and isinstance(sim.stack[-1][0], NewPendingExpr):
            sim.stack[-1] = (RawExpr(init_expr), rust_ty_node)
        else:
            v = sim.fresh('_obj')
            sim.emit(RawStmt(f"let mut {v}: {rust_ty} = {init_expr};"))
            sim.push(Var(v), rust_ty_node)
    else:
        obj_e = render_expr(obj_expr)
        obj_e = render_expr(obj_expr)
        if obj_e in ('this', 'self') and cls:
            # super(args) 调用：在子类构造器中初始化 _super 字段
            # java/lang/Object 的 super() 是 no-op（Rust 不需要 Object 初始化）
            raw_cls = cls.rsplit('/', 1)[-1]
            raw_cls_rust = short_cls(raw_cls.replace('$', '_')) or raw_cls.replace('$', '_')
            if raw_cls_rust in ('Object',) or cls in ('java/lang/Object',):
                sim.emit(RawStmt(f"/* invokespecial {comment} (Object no-op) */"))
            else:
                _init_mangled = _mangle_if_overloaded(cls, '<init>', comment, registry)
                ctor_name = _safe_field(_init_mangled.replace('<init>', 'new'))
                arg_str = ', '.join(args)
                ctor_call = f"{raw_cls_rust}::{ctor_name}({arg_str})"
                super_pfx = _find_super_chain_to_class(class_name, raw_cls_rust, registry) if registry else '_super.'
                if super_pfx:
                    field_path = super_pfx.rstrip('.')
                    sim.emit(RawStmt(f"this.{field_path} = {ctor_call}?;"))
                else:
                    sim.emit(RawStmt(f"/* invokespecial {comment} (same class) */"))
        else:
            sim.emit(RawStmt(f"/* invokespecial {comment} */"))


def _gen_invokestatic(sim: StackSim, comment: str, class_name: str, registry: dict | None = None):
    for skip in BOXING_SKIP_STATIC:
        if skip in comment:
            return  # 自动装箱：栈顶值保留

    if 'String.valueOf' in comment:
        a_expr, _ = sim.pop()
        a = render_expr(a_expr)
        sim.push(Lit(f'String::from_owned(format!("{{}}", {a}))'), RsNamed('String'))
        return

    cls, mname, params, ret = parse_method_ref(comment)
    args = []
    for param_jvm in reversed(params):
        e_expr, ty_node = sim.pop()
        e = render_expr(e_expr)
        ty = render_type(ty_node)
        expected = jvm_to_rust(param_jvm, registry)
        null_coerce = _coerce_from_null(e, expected)
        if null_coerce is not None:
            e = null_coerce
        elif _coerce_to_interface(ty, expected):
            e = 'Default::default()'
        elif expected == 'Object' and ty not in ('Object', '()') and not _is_generic_type_param(ty):
            e = _coerce_to_object(e, ty)
        elif expected in ('bool', 'i8', 'i16', 'u16') and ty != expected:
            e = _coerce_value(e, ty_node, expected)
        elif expected == 'i32' and ty in ('i8', 'i16', 'u16', 'bool'):
            e = f"({e} as i32)"
        elif (expected not in _PRIMITIVE_RUST_TYPES and ty not in _PRIMITIVE_RUST_TYPES
              and expected not in ('Object', '()', ty)
              and _is_subtype(ty.split('<')[0], expected.split('<')[0], registry)):
            # T55：子类型传给父类型参数位置，插入 .into() 类型提升
            e = f"{e}.into()"
        elif ty not in _PRIMITIVE_RUST_TYPES:
            e = f"{e}.clone()"
        args.insert(0, e)

    needs_q = False  # 是否加 ?（用户类方法返回 Result）

    # 目标类不在 registry（被截断的内部类如 jdk.internal.*）→ 生成 panic 存根
    if cls and not _class_known(cls, registry):
        rust_ret = jvm_to_rust(ret, registry)
        stub_msg = f"stub: {cls}.{mname}"
        if rust_ret == '()':
            sim.emit(RawStmt(f'panic!("{stub_msg}");'))
        else:
            v = sim.fresh()
            sim.emit(RawStmt(f'let {v}: {rust_ret} = panic!("{stub_msg}");'))
            sim.push(Var(v), RsNamed(rust_ret))
        return

    if cls is None or cls == class_name:
        rust_mname = _safe_field(_mangle_if_overloaded(class_name, mname, comment, registry))
        call = f"Self::{rust_mname}({', '.join(args)})"
        needs_q = True
    elif cls and '/' in cls:
        call = f"/* {cls}.{mname}({', '.join(args)}) */"
    else:
        rust_mname = _safe_field(_mangle_if_overloaded(cls, mname, comment, registry))
        # 泛型类静态方法需要 turbofish，避免 E0283 类型推断歧义
        turbofish = ''
        if registry:
            _cls_bin = _rust_type_to_binary(cls, registry)
            if _cls_bin:
                _cls_ci = registry.get(_cls_bin)
                if _cls_ci and _cls_ci.generic_signature:
                    _tparams = _parse_class_type_params(_cls_ci.generic_signature)
                    if _tparams:
                        turbofish = '::<' + ', '.join('Object' for _ in _tparams) + '>'
        call = f"{cls}{turbofish}::{rust_mname}({', '.join(args)})"
        needs_q = True

    q = '?' if needs_q else ''
    rust_ret = jvm_to_rust(ret, registry)
    if rust_ret == '()':
        sim.emit(RawStmt(f"{call}{q};"))
    else:
        v = sim.fresh()
        sim.emit(RawStmt(f"let {v}: {rust_ret} = {call}{q};"))
        sim.push(Var(v), RsNamed(rust_ret))


def _gen_invokevirtual(sim: StackSim, comment: str, class_name: str, registry: dict | None = None):
    cls, mname, params, ret = parse_method_ref(comment)
    args = []
    for param_jvm in reversed(params):
        e_expr, e_ty_node = sim.pop()
        e_str = render_expr(e_expr)
        expected_rust = jvm_to_rust(param_jvm, registry)
        actual_rust = render_type(e_ty_node)
        null_coerce = _coerce_from_null(e_str, expected_rust)
        if null_coerce is not None:
            e_str = null_coerce
        elif _coerce_to_interface(actual_rust, expected_rust):
            e_str = 'Default::default()'
        elif expected_rust == 'Object' and actual_rust not in ('Object', '()') and not _is_generic_type_param(actual_rust):
            e_str = _coerce_to_object(e_str, actual_rust)
        elif expected_rust in ('bool', 'i8', 'i16', 'u16') and actual_rust != expected_rust:
            e_str = _coerce_value(e_str, e_ty_node, expected_rust)
        elif expected_rust == 'i32' and actual_rust in ('i8', 'i16', 'u16'):
            e_str = f"({e_str} as i32)"
        elif (expected_rust not in _PRIMITIVE_RUST_TYPES and actual_rust not in _PRIMITIVE_RUST_TYPES
              and expected_rust not in ('Object', '()', actual_rust)
              and _is_subtype(actual_rust.split('<')[0], expected_rust.split('<')[0], registry)):
            # T55：子类型传给父类型参数位置，插入 .into() 类型提升
            e_str = f"{e_str}.into()"
        elif actual_rust not in _PRIMITIVE_RUST_TYPES:
            e_str = f"{e_str}.clone()"
        args.insert(0, e_str)
    obj_expr, obj_ty_node = sim.pop()
    obj_e = render_expr(obj_expr)
    obj_ty = render_type(obj_ty_node)

    # 拆箱：若接收者已是目标基本类型（autoboxing 被跳过），恒等；否则生成实际方法调用
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
        # 接收者是装箱对象（Integer/Double/Number 等），生成实际方法调用完成解箱
        v = sim.fresh()
        sim.emit(RawStmt(f"let {v}: {target_ty} = {obj_e}.{mname}()?;"))
        sim.push(Var(v), RsNamed(target_ty))
        return

    # T38：PrintStream.println 有参版本 → 统一生成 println_v(x)（Printable trait 派发）
    # 注：无参 println() 保持原名；println_v<T: Printable> 处理所有有参版本
    if mname == 'println' and cls and cls.endswith('PrintStream') and len(args) == 1:
        sim.emit(RawStmt(f"{obj_e}.println_v({args[0]})?;"))
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

    # T76：若方法定义在父类（继承方法），通过 _super 链路由调用
    # 基于接收者实际 Rust 类型查找方法是否需要通过 _super 路由
    # 用 JVM 描述符精确匹配重载，避免同名但不同参数的方法干扰路由判断
    super_method_pfx = ''
    if registry:
        recv_base = obj_ty.split('<')[0].strip()
        cls_short = class_name.rsplit('/', 1)[-1] if class_name and '/' in class_name else (class_name or '')
        jvm_desc = f"({''.join(params)}){ret}"
        if recv_base == cls_short:
            super_method_pfx = _find_method_super_prefix(class_name, mname, registry, descriptor=jvm_desc)
        elif recv_base and recv_base not in ('Object', '()'):
            super_method_pfx = _find_method_super_prefix_for_type(recv_base, mname, registry, descriptor=jvm_desc)
    if super_method_pfx:
        obj_e = f"{obj_e}.{super_method_pfx.rstrip('.')}"

    # 所有方法统一处理：obj.method(args)?（用户类 + JDK 类均走此路径）
    arg_str = ', '.join(args)
    rust_ret = jvm_to_rust(ret, registry)
    # E0599 防护：接收者是 Object 类型时，Object 结构体不定义具体子类方法，
    # 直接调用会产生 E0599。对 void 返回跳过调用，对非 void 用 Default::default()。
    obj_is_bare = (obj_ty == 'Object')
    if rust_ret == '()':
        if not obj_is_bare:
            sim.emit(RawStmt(f"{obj_e}.{rust_mname}({arg_str})?;"))
    else:
        v = sim.fresh()
        if obj_is_bare and rust_ret not in ('Object', '()') and rust_ret not in _PRIMITIVE_RUST_TYPES:
            sim.emit(RawStmt(f"let {v}: {rust_ret} = Default::default();"))
        else:
            sim.emit(RawStmt(f"let {v} = {obj_e}.{rust_mname}({arg_str})?;"))
        sim.push(Var(v), RsNamed(rust_ret))
