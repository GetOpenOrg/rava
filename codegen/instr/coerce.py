"""
纯辅助函数：类型强制转换、字段/方法查找、字面量处理。
无 StackSim 状态，可被 invoke.py 和 sim.py 安全导入。
"""

from ..type_map import short_cls as _short_cls_g
import re
from ..constants import safe_ident as _safe_field, PRIMITIVE_RUST_TYPES as _PRIMITIVE_RUST_TYPES, OBJECT_CLASS as _OBJECT_CLASS
from ..type_map import (
    parse_descriptor_params, parse_descriptor_return,
    mangle_name, hierarchy_overloaded_names, method_name_is_mangled,
    BOXING_SKIP_STATIC, UNBOX_VIRTUAL,
)

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


# ── 辅助：解析 javap 注释中的方法引用 ────────────────────────────

def parse_method_ref(comment: str) -> tuple[str | None, str, list, str]:
    """
    解析 'Method Foo.bar:(II)I' 或 'InterfaceMethod ...' 格式。
    返回 (class_short_name, method_name, param_jvm_types, return_jvm_type)
    """
    comment = comment.strip()
    for prefix in ('Method ', 'InterfaceMethod '):
        if comment.startswith(prefix):
            comment = comment[len(prefix):]

    comment = (comment
               .replace('"<init>"',   '__init__')
               .replace('"<clinit>"', '__clinit__')
               .replace('<init>',     '__init__')
               .replace('<clinit>',   '__clinit__'))

    # 方法名按 JVMS §4.2.2：除 . ; [ / < > 之外的任意字符（含 `$`：枚举的 $values、access$NNN、lambda$..）
    m = re.match(r'(?:([^.]+)\.)?([^.;\[/<>:()]+(?:<\w+>)?):(\([^)]*\).+)', comment)
    if not m:
        return (None, comment, [], 'V')

    raw_cls = m.group(1)
    mname   = m.group(2).replace('__init__', '<init>').replace('__clinit__', '<clinit>')
    desc    = m.group(3)

    if raw_cls:
        raw_cls = _short_cls_g(raw_cls)

    return (raw_cls, mname, parse_descriptor_params(desc), parse_descriptor_return(desc))


def _parse_slot(op: str, operand: str) -> int:
    if '_' in op:
        return int(op.split('_')[-1])
    return int(operand.strip()) if operand else 0


# java_runtime 手写实现的短类名：这些类的方法名不经过 mangle（hand-written API 已定好名称）
# System/PrintStream/String/Math/ArrayList/HashMap/HashSet/StringBuilder 由 jdk_classes 字节码翻译提供
_JAVA_RUNTIME_SHORT_NAMES: frozenset[str] = frozenset({
    'Object',
})


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


def _super_path_to_class(from_cls: str, to_cls: str, registry: dict | None) -> str:
    """计算从 from_cls 到 to_cls 的 _super 访问路径。
    返回如 '_super._super.' 形式的前缀，若 from_cls == to_cls 或未找到则返回 ''。
    用于 T76：父类字段/方法的访问需要通过 _super 链路由。"""
    if not registry or not from_cls or not to_cls or from_cls == to_cls:
        return ''
    path_parts: list[str] = []
    ci = registry.get(from_cls)
    while ci:
        sc = ci.super_class
        if not sc or sc == _OBJECT_CLASS:
            break
        path_parts.append('_super')
        if sc == to_cls:
            return '.'.join(path_parts) + '.'
        ci = registry.get(sc)
    return ''


# T76 的 `_find_field_super_prefix` / `_find_field_super_prefix_for_type` 已删除。
# 它们为「按接收者静态类型拼 `_super._super.` 字段路径」而存在；现在继承字段由
# java_class! 宏生成的转发访问器统一暴露（方案 §6 展平 + §16 _super 语义边界），
# getfield/putfield 直接发 `__get_xxx()` / `__set_xxx(v)`，路径计算不再需要。


def _find_super_chain_to_class(current_binary: str, target_cls_short: str, registry: dict | None) -> str:
    """从 current_binary 到 target_cls_short（Rust 短名）的 _super 链前缀。
    用于 invokespecial super.method() 的精确路由（跳过虚拟派发，直接访问目标父类实例）。
    返回类似 '_super.' 或 '_super._super.' 的前缀：
      - 若目标即为当前类本身（私有方法调用）返回 ''
      - 若找到父类路径返回 '_super.' 链
      - 若无注册信息 fallback 到 '_super.'"""
    if not registry or not current_binary or not target_cls_short:
        return '_super.'
    # 当前类短名（私有方法 invokespecial 时 target == current）
    cur_short = _short_cls_g(current_binary) if '/' in current_binary else current_binary.replace('$', '_')
    if target_cls_short == cur_short or target_cls_short == current_binary:
        return ''  # 同类调用（私有方法）：不需要 _super 路由
    ci = registry.get(current_binary)
    if ci is None:
        return '_super.'  # fallback
    path_parts: list[str] = []
    sc = ci.super_class
    while sc and sc != _OBJECT_CLASS:
        path_parts.append('_super')
        sc_short = _short_cls_g(sc) if '/' in sc else sc.replace('$', '_')
        if sc_short == target_cls_short or sc == target_cls_short:
            return '.'.join(path_parts) + '.'
        sc_ci = registry.get(sc)
        if sc_ci is None:
            break
        sc = sc_ci.super_class
    return '_super.'  # fallback: 至少一级 _super（目标类在继承链上但未在 registry 中）


def _super_prefix_to_expr(recv: str, pfx: str) -> str:
    """把 `_super.` / `_super._super.` 前缀转成 `__super()` 调用链。

    java_class! 宏把 `_super` 收成实现细节（方案 §16）：宏外只能通过 `__super()`
    取父类引用，不允许拼字段路径。于是

        this._super.m()           → this.__super().m()
        this._super._super.m()    → this.__super().__super().m()

    层数由前缀里 `_super` 出现的次数决定，与旧实现一一对应。
    """
    return recv + '.__super()' * pfx.count('_super')


def _find_method_super_prefix(class_name: str, mname: str, registry: dict | None,
                              descriptor: str = '') -> str:
    """T76: 找到方法 mname 在继承链中的位置，返回 _super 访问前缀。
    若当前类有该方法名/重载，返回 ''（不需要路由）。
    若只在父类/祖先类有，返回 '_super.' 等前缀。
    descriptor: JVM 描述符（如 '(Ljava/lang/String;)V'），用于精确重载匹配。
    提供 descriptor 时仅匹配该特定重载；否则匹配任意同名方法。"""
    if not registry or not class_name:
        return ''
    ci = registry.get(class_name)
    if ci is None:
        return ''
    # 当前类直接方法中是否有该方法名（排除 synthetic/bridge 桥接方法，它们不会生成 Rust 实现）
    real_methods = [m for m in ci.methods if not m.is_synthetic]
    # 描述符匹配：用参数部分前缀匹配（忽略返回类型）以处理协变返回的接口/实现描述符差异
    # 例如：Appendable.append(CharSequence)Appendable vs Writer.append(CharSequence)Writer
    _param_part = (descriptor.split(')')[0] + ')') if descriptor else ''
    if _param_part:
        if any(m.name == mname and m.descriptor.startswith(_param_part) for m in real_methods):
            return ''
    else:
        if any(m.name == mname for m in real_methods):
            return ''
    # 向上遍历继承链查找（同样排除 synthetic/bridge）
    path_parts: list[str] = []
    sc = ci.super_class
    while sc and sc != _OBJECT_CLASS and sc in registry:
        path_parts.append('_super')
        parent_ci = registry[sc]
        parent_real_methods = [m for m in parent_ci.methods if not m.is_synthetic]
        if _param_part:
            found = any(m.name == mname and m.descriptor.startswith(_param_part) for m in parent_real_methods)
        else:
            found = any(m.name == mname for m in parent_real_methods)
        if found:
            return '.'.join(path_parts) + '.'
        sc = parent_ci.super_class
    return ''


def _rust_type_to_binary(rust_short: str, registry: dict | None) -> str:
    """将 Rust 短类名转换为 binary class name（首个匹配）。用于 T76 接收者类型路由。
    注意：Java 内部类 $ 在 Rust 中转为 _，比较时需转换。"""
    if not registry:
        return ''
    from ..type_map import _registry_short_index
    _ci = _registry_short_index(registry).get(rust_short)
    return _ci.name if _ci is not None else ''


def _is_subtype(child_rust: str, parent_rust: str, registry: dict | None) -> bool:
    """判断 child_rust 是否是 parent_rust 的子类型（通过 registry 继承链+接口链查找）。
    两个参数都是 Rust 短类名（如 IOException, Throwable）。"""
    if not registry or child_rust == parent_rust:
        return False
    child_bin = _rust_type_to_binary(child_rust, registry)
    if not child_bin:
        return False

    def _short(binary: str) -> str:
        return _short_cls_g(binary)

    visited: set[str] = set()
    queue: list[str] = [child_bin]
    while queue:
        cur_bin = queue.pop(0)
        if cur_bin in visited:
            continue
        visited.add(cur_bin)
        ci = registry.get(cur_bin)
        if not ci:
            continue
        # 检查超类
        if ci.super_class and ci.super_class != _OBJECT_CLASS:
            sc = ci.super_class
            sc_short = _short(sc)
            if sc_short == parent_rust:
                return True
            if sc not in visited:
                queue.append(sc)
        # 检查接口列表
        for iface in (ci.interfaces or []):
            iface_short = _short(iface)
            if iface_short == parent_rust:
                return True
            if iface not in visited:
                queue.append(iface)
    return False


def _has_subtypes(class_binary: str, registry: dict | None) -> bool:
    """判断 class_binary 在 registry 中是否有任何（直接）子类。用于决定是否需要多态容器。"""
    if not registry:
        return False
    for binary, ci in registry.items():
        if ci is None or binary == class_binary:
            continue
        if ci.super_class == class_binary:
            return True
        if ci.interfaces and class_binary in ci.interfaces:
            return True
    return False


def _get_all_subtypes_ordered(class_binary: str, registry: dict | None) -> list[str]:
    """返回 class_binary 所有具体子类的 binary name 列表，叶节点优先（最具体子类排在前面）。
    用于生成 downcast dispatch 链：更具体的类型先试，避免父类匹配遮盖子类。"""
    if not registry:
        return []
    # BFS 收集所有子类（含传递子类）
    all_subs: list[str] = []
    queue: list[str] = [class_binary]
    visited: set[str] = {class_binary}
    while queue:
        cur = queue.pop(0)
        for binary, ci in registry.items():
            if ci is None or binary in visited:
                continue
            if ci.super_class == cur or (ci.interfaces and cur in ci.interfaces):
                visited.add(binary)
                all_subs.append(binary)
                queue.append(binary)
    # 逆序 → 叶节点（最深子类）优先
    return list(reversed(all_subs))


def _is_direct_subtype(child_rust: str, parent_rust: str, registry: dict | None) -> bool:
    """判断 child_rust 是否是 parent_rust 的直接子类型（直接超类或直接接口）。
    只检查一步，与 From<X> for Y 的生成规则一致（class_writer.py 的 T76 段只生成直接上转换）。"""
    if not registry or child_rust == parent_rust:
        return False
    child_bin = _rust_type_to_binary(child_rust, registry)
    if not child_bin:
        return False
    ci = registry.get(child_bin)
    if not ci:
        return False
    def _short(b: str) -> str:
        return _short_cls_g(b)
    # 直接超类
    if ci.super_class and ci.super_class != _OBJECT_CLASS:
        if _short(ci.super_class) == parent_rust:
            return True
    # 直接接口列表
    for iface in (ci.interfaces or []):
        if _short(iface) == parent_rust:
            return True
    return False


def _common_ref_type(a_rust: str, b_rust: str, registry: dict | None) -> str | None:
    """两个非泛型引用类型在分支合并处的公共类型：
    一方是另一方的子类型 → 取父类型；否则沿 a 的超类链找第一个同为 b 祖先的类。
    找不到（或含泛型实参）返回 None。"""
    if not registry or a_rust == b_rust or '<' in a_rust or '<' in b_rust:
        return None
    if _is_subtype(a_rust, b_rust, registry):
        return b_rust
    if _is_subtype(b_rust, a_rust, registry):
        return a_rust
    cur = _rust_type_to_binary(a_rust, registry)
    seen: set[str] = set()
    while cur and cur not in seen:
        seen.add(cur)
        ci = registry.get(cur)
        if not ci or not ci.super_class or ci.super_class == _OBJECT_CLASS:
            return None
        cur = ci.super_class
        sc_short = _short_cls_g(cur)
        if _is_subtype(b_rust, sc_short, registry):
            return sc_short
    return None


def _common_ref_type_widening(a_rust: str, b_rust: str, registry: dict | None) -> str | None:
    """槽位 widening 用的公共祖先（含泛型形态）：
    - 基名走 _common_ref_type（继承链，多级跳跃），接口 / 根类 / 无公共祖先 → None
    - 双方类型实参完全一致时保留实参（`TreeNode<K, V>` 与 `Node<K, V>` → `Node<K, V>`）；
      实参不一致（不同实例化）不做 widening → 调用方回退根类合并。
    返回的类祖先保证双方都是其子类型，宏按 all_superclasses 生成 From<Child> for Ancestor，
    存入侧可用 `.into()` 上转（保持对象标识与运行时类）。"""
    def _split(t: str) -> tuple[str, str]:
        base, _, args = t.partition('<')
        return base.strip(), args.strip()
    a_base, a_args = _split(a_rust)
    b_base, b_args = _split(b_rust)
    common = _common_ref_type(a_base, b_base, registry)
    if common is None or common == _OBJECT_CLASS:
        return None
    if _is_interface(common, registry):
        return None
    if not a_args and not b_args:
        return common
    if a_args and a_args == b_args:
        return f"{common}<{a_args}>"
    return None


def _is_interface(rust_short: str, registry: dict | None) -> bool:
    """Rust 短类名（去泛型实参）在 registry 中是否为接口。未知类型按非接口处理。"""
    if not registry:
        return False
    from ..type_map import _registry_short_index
    ci = _registry_short_index(registry).get(rust_short.split('<')[0].strip())
    return ci is not None and ci.is_interface


def _reinstantiate_generic(e: str, actual: str, expected: str) -> str | None:
    """同一泛型类的不同实例化之间的转换（Java 的 raw type / 通配符 / unchecked cast）。

    Java 侧 `AbstractPipeline` 原始类型字段可接收任意实例化的 `this`，
    `(Optional<T>) EMPTY` 是无检查转换；Rust 侧 `X<A>` 与 `X<B>` 是不同类型，
    唯一健全的转换是经 Object 边界（保持对象标识）做带运行时校验的重新实例化。
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


def _into_super_chain(actual_short: str, expected_short: str, registry: dict | None) -> str:
    """vtable 架构：子类型向父类型转换统一用 From trait（.into()），
    宏生成 From<Child> for Parent 利用 vtable trait upcasting 保留运行时类型。
    """
    return '.into()'


def _get_field_generic_signature(class_name: str, safe_fname: str, registry: dict | None) -> str:
    """在类及其继承链中查找字段的 generic_signature。
    用于 getfield 时从 JVM 类型擦除的 Object 恢复泛型类型参数名（如 TT; → T）。"""
    if not registry or not class_name:
        return ''
    ci = registry.get(class_name)
    while ci is not None:
        for f in ci.fields:
            if not f.is_static and _safe_field(f.name) == safe_fname:
                return f.generic_signature
        sc = getattr(ci, 'super_class', None)
        if not sc or sc == _OBJECT_CLASS:
            break
        ci = registry.get(sc)
    return ''


def _find_method_super_prefix_for_type(recv_rust_type: str, mname: str, registry: dict | None,
                                       descriptor: str = '') -> str:
    """基于接收者 Rust 类型（短名）查找方法 _super 前缀。"""
    binary = _rust_type_to_binary(recv_rust_type, registry)
    if binary:
        return _find_method_super_prefix(binary, mname, registry, descriptor=descriptor)
    return ''


def _resolve_method_owner(class_binary: str, mname: str, registry: dict | None,
                          descriptor: str = '') -> tuple[str, int]:
    """沿继承链解析 mname 所在的声明类（描述符精确匹配，排除 synthetic/bridge）。

    返回 (owner_binary, super_levels)：
    - 在 class_binary 自身找到 → (class_binary, 0)
    - 在第 N 层父类找到 → (父类 binary, N)
    - 整条链都没有 → ('', -1)

    用于：
    1. 重载 mangle 必须按「声明类」查（子类继承的重载方法在子类上查不到）
    2. 分派链分支的可编译性判断（方法不可解析时丢弃分支）
    """
    if not registry or not class_binary:
        return ('', -1)
    ci = registry.get(class_binary)
    lvl = 0
    # 参数部分前缀匹配：忽略返回类型差异（协变返回，如接口 Appendable.append→Appendable
    # vs 实现 Writer.append→Writer），排除 synthetic/bridge 方法
    _param_part = (descriptor.split(')')[0] + ')') if descriptor else ''
    while ci is not None:
        real = [m for m in ci.methods if not m.is_synthetic]
        if _param_part:
            found = any(m.name == mname and m.descriptor.startswith(_param_part) for m in real)
        else:
            found = any(m.name == mname for m in real)
        if found:
            return (ci.name, lvl)
        sc = getattr(ci, 'super_class', None)
        if not sc or sc == _OBJECT_CLASS:
            break
        ci = registry.get(sc)
        lvl += 1
    return ('', -1)


def _method_ref_binary_class(comment: str) -> str:
    """从 'Method pkg/Cls.name:(desc)ret' 注释中取出常量池类的完整 binary name。"""
    c = comment.strip()
    for prefix in ('InterfaceMethod ', 'Method '):
        if c.startswith(prefix):
            c = c[len(prefix):]
    dot = c.find('.')
    return c[:dot] if dot > 0 else ''


def _method_ref_descriptor(comment: str) -> str:
    """从 'Method pkg/Cls.name:(desc)ret' 注释中取出方法描述符。"""
    c = comment.strip()
    dot = c.find('.')
    colon = c.find(':', dot)
    return c[colon + 1:].split()[0] if colon > 0 else ''


def _resolve_special_method_owner(class_binary: str, mname: str, descriptor: str,
                                  registry: dict | None) -> str:
    """invokespecial（super.m()）的 JVM 方法解析：常量池类是直接父类，方法可能声明在
    更远的祖先 → 沿父类链找到最近声明类（描述符精确匹配）。找不到返回常量池类本身。"""
    cur = class_binary
    seen: list[str] = []
    while registry and cur and cur not in seen:
        seen.append(cur)
        ci = registry.get(cur)
        if ci is None:
            break
        if any((not m.is_static) and m.name == mname and m.descriptor == descriptor
               for m in ci.methods):
            return cur
        cur = getattr(ci, 'super_class', None)
    # 父类链上无声明 → 方法体来自接口 default 方法。default 方法体展开到「父类链上最早
    # 实现该接口的类」（其后代经 VTable supertrait 链继承），__base 函数归属该类。
    injected_owner = ''
    for cls in seen:
        if class_inherits_default_method(cls, mname, descriptor, registry):
            injected_owner = cls
    return injected_owner or class_binary


def class_inherits_default_method(class_binary: str, mname: str, descriptor: str,
                                  registry: dict | None) -> bool:
    """类直接实现的接口闭包（含父接口）中，是否存在 (mname, descriptor) 的 default 方法体。"""
    ci = registry.get(class_binary) if registry else None
    if ci is None:
        return False
    queue: list[str] = list(ci.interfaces or [])
    visited: set[str] = set()
    while queue:
        iname = queue.pop(0)
        if iname in visited:
            continue
        visited.add(iname)
        ici = registry.get(iname)
        if ici is None:
            continue
        if any((not m.is_static) and (not m.is_abstract) and m.name == mname
               and m.descriptor == descriptor for m in ici.methods):
            return True
        queue.extend(ici.interfaces or [])
    return False


def _resolve_interface_special_target(iface_binary: str, mname: str, descriptor: str,
                                      registry: dict | None) -> str:
    """invokespecial InterfaceMethod（`Iface.super.m()` / 接口私有方法）的 JVM 方法解析：
    常量池类是接口，方法体可能声明在其父接口 → 自身优先、再按广度遍历父接口，
    找到最近的「有方法体」的声明者（描述符精确匹配）。常量池类不是 registry 中的接口、
    或找不到方法体时返回 ''。"""
    if not registry:
        return ''
    root = registry.get(iface_binary)
    if root is None or not root.is_interface:
        return ''
    queue: list[str] = [iface_binary]
    seen: set[str] = set()
    while queue:
        cur = queue.pop(0)
        if cur in seen:
            continue
        seen.add(cur)
        ci = registry.get(cur)
        if ci is None:
            continue
        if any((not m.is_static) and (not m.is_abstract) and m.name == mname
               and m.descriptor == descriptor for m in ci.methods):
            return cur
        queue.extend(ci.interfaces or [])
    return ''


def interface_special_member_name(owner_binary: str, mname: str, descriptor: str,
                                  registry: dict | None) -> str:
    """`Iface.super.m(...)` 在实现类中的落点成员名：`Iface_super_m`（m 在接口内重载时带描述符后缀）。
    接口 default 方法体按「展开到实现类」建模，被覆盖的 default 方法体以该名字的
    非虚成员形式展开到调用者所在的类。定义侧（class_writer）与调用侧（invokespecial）共用。"""
    owner_short = _short_cls_g(owner_binary)
    owner_ci = registry.get(owner_binary) if registry else None
    rust_m = mname
    if owner_ci is not None and mname in hierarchy_overloaded_names(owner_ci, registry):
        rust_m = mangle_name(mname, descriptor)
    return f"{owner_short}_super_{rust_m}"


def _resolve_static_method_owner(class_binary: str, mname: str, descriptor: str,
                                 registry: dict | None) -> str:
    """invokestatic 的 JVM 方法解析：常量池类可以是子类，static 方法实际声明在
    祖先类 → 沿父类链找到声明类（描述符精确匹配）。找不到返回 ''。"""
    if not registry:
        return ''
    cur = class_binary
    seen: set[str] = set()
    while cur and cur not in seen:
        seen.add(cur)
        ci = registry.get(cur)
        if ci is None:
            return ''
        if any(m.is_static and m.name == mname and m.descriptor == descriptor for m in ci.methods):
            return cur
        cur = getattr(ci, 'super_class', None)
    return ''


def _resolve_static_field_owner(class_binary: str, field_name: str,
                                registry: dict | None) -> str:
    """getstatic/putstatic 的 JVM 字段解析（JVMS §5.4.3.2）：自身 → 父接口 → 父类。
    field_name 为 .class 中的原始字段名。找不到返回 ''。"""
    if not registry:
        return ''
    seen: set[str] = set()

    def _lookup(cur: str) -> str:
        if not cur or cur in seen:
            return ''
        seen.add(cur)
        ci = registry.get(cur)
        if ci is None:
            return ''
        if any(f.is_static and f.name == field_name for f in ci.fields):
            return cur
        for iface in (ci.interfaces or []):
            found = _lookup(iface)
            if found:
                return found
        return _lookup(getattr(ci, 'super_class', None) or '')

    return _lookup(class_binary)


_ROOT_VIRTUAL_METHODS: set[tuple[str, str]] | None = None


def _root_virtual_methods() -> set[tuple[str, str]]:
    """根类（OBJECT_CLASS）的 public 实例方法集 {(name, '(params)')}。

    根类由 runtime 手写、不在 registry 中；其方法集从 JDK 的 .class 字节码
    动态解析（不写方法名/类名字面量），供「整条祖先链都未声明该方法 →
    路由到根 vtable」的判定使用。
    """
    global _ROOT_VIRTUAL_METHODS
    if _ROOT_VIRTUAL_METHODS is None:
        _ROOT_VIRTUAL_METHODS = set()
        try:
            from ..classfile import parse_class_bytes
            from ..jdk_resolver import JdkResolver
            with JdkResolver() as _res:
                _data = _res.resolve(_OBJECT_CLASS)
            if _data is not None:
                _root_ci = parse_class_bytes(_data, _OBJECT_CLASS)
                for _m in _root_ci.methods:
                    # 仅 public 实例方法：protected 方法（clone/finalize）只能经子类
                    # 自身的覆写调用，不存在「经根 vtable 对任意对象调用」的语义
                    if (_m.is_static or _m.is_synthetic or _m.name.startswith('<')
                            or not (_m.access_flags & 0x0001)):  # ACC_PUBLIC
                        continue
                    _ROOT_VIRTUAL_METHODS.add((_m.name, _m.descriptor.split(')')[0] + ')'))
        except RuntimeError:
            pass
    return _ROOT_VIRTUAL_METHODS


def _parse_field_ref(comment: str) -> tuple[str, str, str]:
    """解析 'Field java/lang/System.out:Ljava/io/PrintStream;' 格式。
    返回 (class_binary_name, field_name, descriptor)。
    field_name 已经过 _safe_field 处理（$ → _, Rust 关键字加 _）。"""
    comment = comment.strip()
    for prefix in ('Field ', 'InterfaceField '):
        if comment.startswith(prefix):
            comment = comment[len(prefix):]
    # 格式：java/lang/System.out:Ljava/io/PrintStream;
    if ':' in comment:
        ref_part, descriptor = comment.split(':', 1)
    else:
        ref_part, descriptor = comment, 'Ljava/lang/Object;'
    if '.' in ref_part:
        cls, field = ref_part.rsplit('.', 1)
    else:
        cls, field = '', ref_part
    return cls, _safe_field(field), descriptor


def _class_known(cls_short: str, registry: dict | None) -> bool:
    """判断短类名是否已知（registry 中存在或是 java_runtime 手写类）。
    T71：比较时统一将 $ 替换为 _，避免 JVM 格式（Outer$Inner）与 Rust 格式（Outer_Inner）不一致。"""
    if not registry:
        return True
    if not cls_short or cls_short in _JAVA_RUNTIME_SHORT_NAMES:
        return True
    if cls_short in registry:
        return True
    norm = cls_short.replace('$', '_')
    for key in registry:
        if _short_cls_g(key) == norm:
            return True
    return False


_JAVA_RUST_NAME_CONFLICTS = frozenset()
_JAVA_RUST_RENAME: dict[str, str] = {}


_BRIDGE_CALL_RE = re.compile(r'^(?:Interface)?Method\s+([^.\s]+)\.([^:\s]+):(\(\S*\)\S+)')


def _bridge_call_target(bridge, mname: str, registry: dict):
    """bridge 方法体里被桥接的真实方法：(声明类 ClassInfo, 真实描述符)；不可解析时 None。"""
    for ins in reversed(bridge.instrs or []):
        if not (ins.opcode or '').startswith('invoke'):
            continue
        cm = _BRIDGE_CALL_RE.match((ins.comment or '').strip())
        if cm is None or cm.group(2) != mname:
            continue
        real_params = cm.group(3).split(')')[0] + ')'

        def _declared(owner_ci):
            return next((m for m in owner_ci.methods
                         if not m.is_synthetic and m.name == mname
                         and m.descriptor.startswith(real_params)), None)

        owner_ci = registry.get(cm.group(1))
        real = _declared(owner_ci) if owner_ci is not None else None
        if real is None:
            owner_bin, _ = _resolve_method_owner(cm.group(1), mname, registry, descriptor=cm.group(3))
            owner_ci = registry.get(owner_bin) if owner_bin else None
            real = _declared(owner_ci) if owner_ci is not None else None
        return (owner_ci, real.descriptor) if real is not None else None
    return None


def _resolve_bridge_target(ci, mname: str, descriptor: str, registry: dict):
    """找 descriptor 精确命中的 synthetic bridge，返回 (真实方法的声明类, 真实方法的描述符)。

    查找顺序与 JVM 方法解析一致：先沿父类链，再到实现的接口闭包（接口里的 bridge：
    `Node.OfDouble.copyInto(Object[],int)` → default `copyInto(Double[],int)`）。
    被桥接的真实方法直接读自 bridge 的字节码：javac 生成的 bridge 体只有一条同名方法调用
    （invokevirtual this.m(真实描述符)，或真实方法继承自祖先时的 invokespecial super.m(..)）。
    后者的声明类沿调用指令的 owner 向上解析——真实方法不一定与 bridge 同类
    （`EmptySpliterator.OfDouble.tryAdvance(DoubleConsumer)` 桥接到祖先的 `tryAdvance(Object)`）。
    找不到 bridge 或其目标不可解析时返回 None。"""
    def _bridge_in(c):
        return next((m for m in c.methods
                     if m.is_synthetic and m.name == mname and m.descriptor == descriptor), None)

    seen: set[str] = set()
    pending_ifaces: list[str] = []
    cur = ci
    while cur is not None and cur.name not in seen:
        seen.add(cur.name)
        bridge = _bridge_in(cur)
        if bridge is not None:
            return _bridge_call_target(bridge, mname, registry)
        pending_ifaces.extend(cur.interfaces or [])
        cur = registry.get(cur.super_class) if cur.super_class else None
    while pending_ifaces:
        iface_ci = registry.get(pending_ifaces.pop(0))
        if iface_ci is None or iface_ci.name in seen:
            continue
        seen.add(iface_ci.name)
        bridge = _bridge_in(iface_ci)
        if bridge is not None:
            return _bridge_call_target(bridge, mname, registry)
        pending_ifaces.extend(iface_ci.interfaces or [])
    return None


def _mangle_if_overloaded(cls_name: str, mname: str, comment: str, registry: dict | None) -> str:
    """查找 registry 中 cls_name 类的 mname 方法是否重载，重载则返回 mangled 名，否则原名。
    支持短名（Objects）和全路径名（java/util/Objects）查找。
    java_runtime 手写类（ArrayList/Object 等）不做 mangle，其 API 已固定。"""
    if not registry or not mname or (mname.startswith('<') and mname != '<init>'):
        return mname
    # java_runtime 手写类直接跳过 mangle（其 API 已固定，不走 jdk_classes 重命名逻辑）
    short = cls_name.rsplit('/', 1)[-1]
    if short in _JAVA_RUNTIME_SHORT_NAMES:
        return mname
    # 直接查（可能是全路径）
    target_ci = registry.get(cls_name)
    # 短名查（用全路径反查）；T71：统一 $ → _ 后比较
    if target_ci is None and '/' not in cls_name:
        norm = cls_name.replace('$', '_')
        for key, ci in registry.items():
            if _short_cls_g(key) == norm:
                target_ci = ci
                break
    if target_ci is None:
        return _JAVA_RUST_RENAME.get(mname, mname)
    # 排除 java_runtime 类（registry 中仍有其 JDK 字节码副本，但方法名不 mangle）
    if target_ci.name.rsplit('/', 1)[-1] in _JAVA_RUNTIME_SHORT_NAMES:
        return mname
    # 名字由「声明类」决定：调用目标类（常量池里的类）可能只是继承了该方法，
    # 沿父类链解析到真正声明 name+descriptor 的类，再用与定义侧相同的判定函数。
    _call_desc_m = re.search(r':(\([^)]*\)\S+)', comment)
    if mname != '<init>' and _call_desc_m:
        _owner_bin, _ = _resolve_method_owner(target_ci.name, mname, registry,
                                              descriptor=_call_desc_m.group(1))
        if _owner_bin and _owner_bin in registry:
            target_ci = registry[_owner_bin]
            if target_ci.name.rsplit('/', 1)[-1] in _JAVA_RUNTIME_SHORT_NAMES:
                return mname
        else:
            # 调用描述符只命中 synthetic bridge（如 Comparable.compareTo(Object) 分派到
            # 只声明 compareTo(Self) 的实现类）：bridge 不生成 Rust 方法，名字取它桥接到的
            # 真实方法（同名、同参数个数、唯一候选）。
            _bridged = _resolve_bridge_target(target_ci, mname, _call_desc_m.group(1), registry)
            if _bridged is not None:
                # 真实方法声明在接口（default，注入到实现类）→ 名字仍按调用目标类层次判定
                if not _bridged[0].is_interface:
                    target_ci = _bridged[0]
                comment = f'{comment.split(":")[0]}:{_bridged[1]}'
            elif not target_ci.is_interface:
                # 类链未声明该方法：只声明在接口上的成员（抽象类上调用接口抽象方法）
                from ..type_map import interface_member_local_name as _iface_local
                _local = _iface_local(target_ci, mname, _call_desc_m.group(1), registry)
                return _JAVA_RUST_RENAME.get(_local, _local)
    # 按 (name, descriptor) 判定：覆盖方法沿用 vtable 槽位所属祖先中的名字（与定义侧同源）
    _final_desc_m = re.search(r':(\([^)]*\)\S+)', comment)
    _call_desc = _final_desc_m.group(1) if _final_desc_m else ''
    _declared = next((m for m in target_ci.methods
                      if m.name == mname and m.descriptor == _call_desc), None)
    _is_mangled = (method_name_is_mangled(target_ci, _declared, registry)
                   if _declared is not None
                   else mname in hierarchy_overloaded_names(target_ci, registry))
    if not _is_mangled:
        # Java→Rust 名字冲突重命名（如 clone→jvm_clone）
        erg_name = _JAVA_RUST_RENAME.get(mname)
        if erg_name is not None:
            return erg_name
        return mname
    desc_m = re.search(r':(\([^)]*\)\S+)', comment)
    raw_desc = desc_m.group(1) if desc_m else ''
    result = mangle_name(mname, raw_desc) if raw_desc else mname
    # 重载后的名字若与 Rust 原生名字冲突也需重命名
    return _JAVA_RUST_RENAME.get(result, result)


# ── G-10：invokedynamic 实现方法命名的单一来源 ──────────────────────────

def lambda_impl_rust_name(impl_cls_bin: str, impl_mname: str, impl_desc: str,
                          registry: dict | None) -> str:
    """invokedynamic 实现方法（lambda body / 方法引用目标）的 Rust 名 —— 单一来源。

    调用点（instr/sim/dynamic.py 的 bootstrap 分析）与定义侧（emitter/class_writer
    的方法生成）都必须经此函数取名，禁止各自推导：重载 mangle、`<init>` → `new`、
    safe_ident 三步在此收拢（G-10 根因：两路独立推导得到不同哈希后缀/序号，
    产生 `no method named lambda_*` 编译错误）。
    """
    mangled = _mangle_if_overloaded(
        impl_cls_bin, impl_mname,
        f"Method {impl_cls_bin}.{impl_mname}:{impl_desc}", registry)
    return _safe_field(mangled.replace('<init>', 'new'))


class _LambdaNameLedger:
    """G-10 生成期断言：同一 invokedynamic 站点的「body 方法定义名」与「调用点引用名」恒等。

    调用点（sim/dynamic.py）经 record_reference 登记 (类, Java 方法名) → 引用名集合；
    定义侧（class_writer 每个生成/声明/存根化的方法）经 record_definition 登记同一键
    → 定义名集合；project_writer 生成收尾时 check()：
      1. 引用名必须出现在同类同名方法的定义名集合中（命名漂移直接抛错）；
      2. 引用目标的类在本轮经 class_writer 生成过，却没有任何该方法的定义
         → 定义被过滤/丢失（接口私有实例 lambda 曾被整体跳过）→ 抛错。
    按 (类, Java 方法名) 而非描述符聚合：`_mangle_if_overloaded` 会把指向 synthetic
    bridge 的 impl 引用改按桥接后的真实方法取名（描述符不同、名字与真实定义一致），
    按名字集合比对不会误伤。未在本轮生成的类（手写 java_runtime 类 / registry
    之外）不参与断言，由 Rust 编译器兜底报错。
    """

    def __init__(self) -> None:
        self.references: dict[tuple[str, str], set[str]] = {}
        self.definitions: dict[tuple[str, str], set[str]] = {}
        self.generated_classes: set[str] = set()

    def reset(self) -> None:
        self.references.clear()
        self.definitions.clear()
        self.generated_classes.clear()

    def record_reference(self, cls_bin: str, mname: str, rust_name: str) -> None:
        self.references.setdefault((cls_bin, mname), set()).add(rust_name)

    def record_definition(self, cls_bin: str, mname: str, rust_name: str) -> None:
        self.definitions.setdefault((cls_bin, mname), set()).add(rust_name)

    def check(self) -> None:
        problems: list[str] = []
        for (cls_bin, mname), ref_names in sorted(self.references.items()):
            if cls_bin not in self.generated_classes:
                continue
            def_names = self.definitions.get((cls_bin, mname))
            if not def_names:
                problems.append(f"{cls_bin}.{mname} → 调用点引用 "
                                f"{sorted(ref_names)}，但该类本轮生成时未输出此方法的任何定义（被过滤/跳过）")
                continue
            drifted = sorted(ref_names - def_names)
            if drifted:
                problems.append(f"{cls_bin}.{mname} → 调用点引用 {drifted} "
                                f"不在定义名集合 {sorted(def_names)} 中")
        if problems:
            head = '\n'.join(f'  [G-10] {p}' for p in problems[:10])
            more = f'\n  ...（共 {len(problems)} 处）' if len(problems) > 10 else ''
            raise RuntimeError('invokedynamic 实现方法命名/定义不一致（G-10 断言）：\n'
                               + head + more)


LAMBDA_NAME_LEDGER = _LambdaNameLedger()

# Java 中任何对象都可以传递给 Object 参数（引用协变），Rust 需要显式 Into<Object> 转换
# _PRIMITIVE_RUST_TYPES 已统一到 codegen/constants.py 的 PRIMITIVE_RUST_TYPES
