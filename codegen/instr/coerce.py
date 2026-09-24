"""
值强转：栈值 → 目标类型的表达式构造（基本类型互转 / Object 装箱 / null 还原 /
泛型实例化重建 / 字面量转义）。无 StackSim 状态，可被 invoke* 与 sim* 安全导入。

层次查询（hierarchy.py）/ 成员归属解析（member_owner.py）/ 命名与 ref 解析
（member_naming.py）已按 2026-09-20 拆分方案 §3.4 拆出；本模块只保留值强转。
"""


# Python float → Rust 字面量（处理 nan/inf/-inf 等特殊值）
_FLOAT_SPECIAL = {
    'nan': '{ty}::NAN', 'inf': '{ty}::INFINITY', '-inf': '{ty}::NEG_INFINITY',
    'infinity': '{ty}::INFINITY', '-infinity': '{ty}::NEG_INFINITY',
}

def _float_lit(val_str: str, ty: str) -> str:
    """将 Python float repr 转为合法 Rust 字面量，处理 nan/inf/-inf。"""
    key = val_str.lower()
    if key in _FLOAT_SPECIAL:
        return _FLOAT_SPECIAL[key].format(ty=ty)
    return val_str + ty


def _escape_str(s: str) -> str:
    """将字符串常量的解码值转义为 Rust 字符串字面量内容（不含两端的 "）。

    输入契约：常量池 Utf8 的解码值（classfile._ldc_str 产出），即字符序列本身。
    值里的反斜杠是字面文本而非转义序列，一律双写——例如 Properties 的
    "Malformed \\uxxxx encoding."，值含 `\\`+`u`，必须落成 Rust 的 `\\\\u`，
    若原样保留 `\\u` 会产出非法的 unicode 转义（rustc E0709 类编译错误）。"""
    result = []
    for c in s:
        if c == '\\':
            result.append('\\\\')
        elif c == '"':
            result.append('\\"')
        elif c == '\n':
            result.append('\\n')
        elif c == '\r':
            result.append('\\r')
        elif c == '\t':
            result.append('\\t')
        else:
            cp = ord(c)
            if cp < 0x20 or (0x7f <= cp <= 0x9f):
                result.append(f'\\u{{{cp:04x}}}')
            else:
                result.append(c)
    return ''.join(result)


def _to_i32(expr_str: str, ty: 'RsType') -> str:
    """窄类型（i8/i16/u16/bool）向上转为 i32，避免 JVM int 运算中的类型不匹配。"""
    ty_name = getattr(ty, 'name', '')
    if ty_name in ('i8', 'i16', 'u16', 'bool'):
        # `as` 优先级高于比较/算术运算符：`a > b as i32` 会被解析为 `a > (b as i32)`，
        # 非原子表达式必须先整体加括号再转换。
        if not _is_atomic_expr(expr_str):
            expr_str = f"({expr_str})"
        return f"({expr_str} as i32)"
    return expr_str


def _is_atomic_expr(expr_str: str) -> bool:
    """表达式是否可直接作为 `as` 的左操作数：标识符/字面量/路径/调用链（括号内内容不计）。"""
    depth = 0
    for ch in expr_str.strip():
        if ch in '([{':
            depth += 1
        elif ch in ')]}':
            depth -= 1
        elif depth == 0 and not (ch.isalnum() or ch in '_.:?'):
            return False
    return True


def _coerce_to_object(val_str: str, ty: str, registry: dict | None = None,
                      class_type_params=(), clone: bool = True) -> str:
    """值 → Object 引用（Java 的隐式向上转型）。对象身份必须保持：

    - 类实例：`Object::from(x)` —— Object 直接持有该 wrapper，运行时类（is_instance_of）、
      虚方法覆盖（hashCode/equals/toString）与接口 vtable（`__interface`）全部可达
    - 接口载体：同样 `Object::from(x)` —— 载体的 From 解包 `__ref`，保持底层接收者的
      对象身份（`from_any` 会把载体自身装成新对象，双重包装后接口查询与 SAM 直调失联）
    - 类型变量：`Into::<Object>::into(x)`（类型实参恒为引用类型，宏为类型形参补 Into<Object>）
    - 基本类型：`.into()`
    - 其余（闭包等无运行时类的值）：`Object::from_any(..)` 不透明装箱

    注意 Clone::clone 而非 .clone()：值可能是带 Java clone() 的类（Enum_/HashMap 等）。"""
    if ty in ('i32', 'i64', 'f32', 'f64', 'bool', 'i8', 'i16', 'u16'):
        # G-12：负数字面量补外层括号——方法调用优先级高于一元负号，
        # `-1i32.into()` 解析为 `-(1i32.into())`，目标类型推断失败（E0282）
        if val_str.lstrip().startswith('-'):
            return f"({val_str}).into()"
        return f"{val_str}.into()"
    src = f"Clone::clone(&{val_str})" if clone else val_str
    if ty in (class_type_params or ()):
        return f"Into::<Object>::into({src})"
    from ..stack import erased_class_of, is_jvm_array
    if is_jvm_array(ty):
        # 数组是对象：Object 直接持有数组引用（元素类型具体化，可按 `T[]` 精确取回）
        return f"Object::from({src})"
    if erased_class_of(ty, registry) is not None:
        # registry 类 / 接口载体（擦除身份经 TypeIR 边界查询，清单第 5 项 S2）
        return f"Object::from({src})"
    return f"Object::from_any({src})"


_NULL_OBJECT_EXPRS = frozenset({'Object::default()', 'Object::default().clone()'})


def _render_cast(e, target: str, binary_name: str = '', checked: bool = False,
                 box_first: bool = False) -> str:
    """checkcast / 跨实例化转换的唯一发射入口（A-3 IR 化）。

    字符串管线（invoke 实参、putfield 存值、lambda 捕获）经本入口构造 CastExpr
    节点并立即渲染；IR 管线（stack 模拟、CFG）直接构造 CastExpr 节点入栈——
    两条管线同一渲染源（render.render_cast），消费方按节点分派而非匹配字符串。

    - checked=True：checkcast 语义，`try_cast::<target>("binary_name")?`，
      失败返回 Err(JvmError::class_cast)（S-1，可被 java_try 捕获）；
    - checked=False：`<target as From<Object>>::from(..)` 静态合法视图转换
      （跨实例化擦除路径，A-1：From<Object> for X<A> 对任意 A 成立）；
    - box_first=True：e 是具体 wrapper（非 Object）时先装箱（保持对象标识）。
    """
    from ..rs_ir import CastExpr, RawExpr
    from ..render import render_cast
    child = e if isinstance(e, RawExpr) else RawExpr(e)
    return render_cast(CastExpr(child, target, binary_name=binary_name,
                                checked=checked, box_first=box_first))


def _same_generic_family(actual: str, expected: str) -> bool:
    """同一泛型类的不同实例化（raw type / 通配符 / unchecked cast）：
    基名相同且类型实参不同 → 经 Object 边界重建目标实例化视图（CastExpr 的
    擦除路径）。实参含推断占位 `_`（new X<>() 菱形）时由 Rust 类型推断对齐，
    不算跨实例化。"""
    if '<' not in actual or '<' not in expected or actual == expected:
        return False
    if actual.split('<', 1)[0] != expected.split('<', 1)[0]:
        return False
    import re as _re_infer
    return not _re_infer.search(r'(?<![\w])_(?![\w])', actual)


def _coerce_from_null(val_str: str, expected: str) -> str | None:
    """若 val_str 是 aconst_null 的结果（Object::default()），
    且 expected 是具体的引用类型，返回 Default::default() 作为替代。
    否则返回 None 表示无需特殊处理。"""
    if val_str not in _NULL_OBJECT_EXPRS:
        return None
    if expected in ('Object', '()') or expected in ('i32', 'i64', 'f32', 'f64', 'bool', 'i8', 'i16', 'u16'):
        return None
    # null 作为参数：用 Default::default() 提供类型安全的零值
    # [equiv-audit] boxed-null（S-3）：装箱 null 路径——null 字面量流入具体
    # 引用类型槽位被替换为零值（参数经 _coerce_arg、字段存储经 fields.py 两条
    # 消费路径共用本入口），只计数不改发射
    from .. import equiv_audit
    equiv_audit.record('boxed-null')
    return 'Default::default()'


def _coerce_value(val_str: str, val_ty: 'RsType', target: str) -> str:
    """将 val 强制转换为 target 字段/参数类型，避免窄类型与 i32 不匹配。
    只在必要时插入 cast，若类型已匹配则原样返回。"""
    src = getattr(val_ty, 'name', '')
    if target == src:
        return val_str
    if target == 'i32':
        if src in ('bool', 'u16', 'i8', 'i16'):
            return f"({val_str}) as i32"
        return val_str
    if target == 'bool':
        if src == 'bool':
            return val_str
        return f"({val_str} != 0i32)"
    if target in ('i8', 'i16', 'u16'):
        if src != 'i32':
            val_str = f"({val_str} as i32)"
        return f"(({val_str}) as {target})"
    return val_str


# Java 中任何对象都可以传递给 Object 参数（引用协变），Rust 需要显式 Into<Object> 转换
# _PRIMITIVE_RUST_TYPES 已统一到 codegen/constants.py 的 PRIMITIVE_RUST_TYPES
