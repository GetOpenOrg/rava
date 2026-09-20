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
    """将原始字符串内容转义为 Rust 字符串字面量内容（不含两端的 "）。
    处理：\\ → \\\\，" → \\"，控制字符，以及无效的 \\% 等 Java 格式化符号。"""
    result = []
    i = 0
    while i < len(s):
        c = s[i]
        if c == '\\':
            # 已有反斜杠：检查下一个字符是否构成合法 Rust 转义序列
            if i + 1 < len(s):
                nc = s[i + 1]
                if nc in ('"', "'", '\\', 'n', 'r', 't', '0', 'x', 'u'):
                    result.append('\\')
                    result.append(nc)
                    i += 2
                    continue
                else:
                    # 非法转义（如 \%、\u 后跟非十六进制）→ 转义为 \\
                    result.append('\\\\')
                    i += 1
                    continue
            else:
                result.append('\\\\')
                i += 1
        elif c == '"':
            result.append('\\"')
            i += 1
        elif c == '\n':
            result.append('\\n')
            i += 1
        elif c == '\r':
            result.append('\\r')
            i += 1
        elif c == '\t':
            result.append('\\t')
            i += 1
        else:
            cp = ord(c)
            if cp < 0x20 or (0x7f <= cp <= 0x9f):
                result.append(f'\\u{{{cp:04x}}}')
            else:
                result.append(c)
            i += 1
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
    if ty.startswith('JArray<'):
        # 数组是对象：Object 直接持有数组引用（元素类型具体化，可按 `T[]` 精确取回）
        return f"Object::from({src})"
    if registry:
        from ..type_map import _registry_short_index
        _ci = _registry_short_index(registry).get(ty.split('<')[0].strip())
        if _ci is not None:
            return f"Object::from({src})"
    return f"Object::from_any({src})"


_NULL_OBJECT_EXPRS = frozenset({'Object::default()', 'Object::default().clone()'})


def _checkcast_runtime_expr(val_str: str, target: str) -> str:
    """checkcast / 按声明类型还原的运行时入口（S-4）。

    - 数组目标（`JArray<...>`）：`From<Object> for JArray<T>`——downcast 的泛型路径
      无法分解出元素类型，数组 checkcast 的判定（null 还原 / 同元素类型还原 /
      任意祖先元素类型协变视图 / 泛型数组的擦除还原）由 From 侧的元素类型 T 驱动。
    - 类目标：`<T as From<Object>>::from(..)`——wrapper 的 From 自带三层路径
      （null 还原 / `__view_into` 视图 / `is_instance_of` + 擦除部件重建），覆盖
      「运行时类是目标类或其子类 + 目标实例化非精确实参」的泛型擦除场景
      （`(Enum) key` 于 `Enum<K extends Enum<K>>`）；`Object::downcast` 的 slot 按精确
      TypeId 判定，跨实例化会误抛 ClassCastException。
    From 按值收 Object（downcast 借用接收者）→ 统一先 Clone::clone，值可能在兄弟分支
    继续使用（`Object o; if(..) f((int[]) o); else g((long[]) o);`，E0382）。
    """
    return f"<{target} as ::std::convert::From<Object>>::from(Clone::clone(&{val_str}))"


def _coerce_from_null(val_str: str, expected: str) -> str | None:
    """若 val_str 是 aconst_null 的结果（Object::default()），
    且 expected 是具体的引用类型，返回 Default::default() 作为替代。
    否则返回 None 表示无需特殊处理。"""
    if val_str not in _NULL_OBJECT_EXPRS:
        return None
    if expected in ('Object', '()') or expected in ('i32', 'i64', 'f32', 'f64', 'bool', 'i8', 'i16', 'u16'):
        return None
    # null 作为参数：用 Default::default() 提供类型安全的零值
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


def _reinstantiate_generic(e: str, actual: str, expected: str) -> str | None:
    """同一泛型类的不同实例化之间的转换（Java 的 raw type / 通配符 / unchecked cast）。

    Java 侧 `AbstractPipeline` 原始类型字段可接收任意实例化的 `this`，
    `(Optional<T>) EMPTY` 是无检查转换；Rust 侧 `X<A>` 与 `X<B>` 是不同类型，
    经 Object 边界构造目标实例化的视图。A-1 存储层擦除落地后
    `From<Object> for X<A>` 对任意 A 成立（__inner 非泛型，共享存储与对象
    标识，运行时按擦除类判定）——本转换由宏的擦除路径支撑，不再依赖
    （已删除的）#[immutable_state] 逐字段重建。
    actual / expected 基名相同且类型实参不同 → 返回转换表达式，否则 None。"""
    if '<' not in actual or '<' not in expected or actual == expected:
        return None
    if actual.split('<', 1)[0] != expected.split('<', 1)[0]:
        return None
    import re as _re_infer
    if _re_infer.search(r'(?<![\w])_(?![\w])', actual):
        # 实参含推断占位符 `_`（new X<>() 菱形）：由 Rust 类型推断对齐，无需转换
        return None
    src = 'Clone::clone(this)' if e == 'this' else f'Clone::clone(&{e})'
    return f"<{expected} as ::std::convert::From<Object>>::from(Object::from({src}))"


# Java 中任何对象都可以传递给 Object 参数（引用协变），Rust 需要显式 Into<Object> 转换
# _PRIMITIVE_RUST_TYPES 已统一到 codegen/constants.py 的 PRIMITIVE_RUST_TYPES
