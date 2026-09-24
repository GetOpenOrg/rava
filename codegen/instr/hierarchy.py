"""
层次查询：registry 继承链 / 接口链上的子类型判定、公共祖先、_super 路径计算、
短类名 ↔ binary name 换算。纯查询层，不依赖 instr 内其他模块。

从 coerce.py 拆出（2026-09-20 窗口 2，方案 §3.4）。
"""

from ..type_map import short_cls as _short_cls_g
from ..constants import OBJECT_CLASS as _OBJECT_CLASS


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
    两个参数都是 Rust 短类名（如 IOException, Throwable）。

    TypeIR 试点（收敛路线图 L1-a 单一权威化第一步）：内部实现委托
    JvmType.is_subtype_of（codegen/jvm_type.py 类型代数，闭包集带缓存），
    函数签名与调用方不变。保留旧实现的三点可观察语义（行为零变化）：
      - 严格子类型：child == parent（或解析到同一 binary）→ False，不自反；
      - java/lang/Object 恒不作为成立目标（旧 BFS 显式跳过 Object 超类边，
        registry 外以 'Object' 短名查询同理）；
      - child 短名必须能在 registry 反查到 binary，否则 False；
        parent 反查不到时按 Rust 短名占位匹配闭包（旧实现的比较键就是
        Rust 短名——闭包上未注册祖先只有短名这一跨域可比身份）。
    """
    if not registry or child_rust == parent_rust:
        return False
    child_bin = _rust_type_to_binary(child_rust, registry)
    if not child_bin:
        return False
    parent_bin = _rust_type_to_binary(parent_rust, registry)
    if parent_bin == _OBJECT_CLASS or (not parent_bin
                                       and parent_rust == _short_cls_g(_OBJECT_CLASS)):
        return False
    from ..jvm_type import JvmType
    child_t = JvmType.class_of(child_bin, registry)
    target_t = JvmType.class_of(parent_bin if parent_bin else parent_rust, registry)
    if child_t.binary == target_t.binary:
        return False  # 解析到同一 binary：严格语义不自反
    return child_t.is_subtype_of(target_t, registry)


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
    # BFS 收集所有子类（含传递子类）。
    # G-4 确定性：兄弟节点按 binary name 排序——registry 插入序可能经上游集合
    # 迭代随 PYTHONHASHSEED 漂移，直接迭代会把不确定性传染给 downcast 分派链
    all_subs: list[str] = []
    queue: list[str] = [class_binary]
    visited: set[str] = {class_binary}
    while queue:
        cur = queue.pop(0)
        for binary, ci in sorted(registry.items()):
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
        args = args.strip()
        # partition 在首个 '<' 切分：args 带原始收尾 '>'（`Node<K, V>` → 'K, V>'；
        # 嵌套实参 `Entry<String, JArray<Object>>` → 'String, JArray<Object>>'）。
        # 只剥最外层一个——嵌套实参的内层 '>' 保留，否则重组 f"{common}<{args}>"
        # 会重复收尾（`Node<K, V>>`，此前无泛型实参一致的调用方，潜伏未触发）。
        if args.endswith('>'):
            args = args[:-1].rstrip()
        return base.strip(), args
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
    from ..stack import erased_class_of
    ref = erased_class_of(rust_short, registry)
    return ref is not None and ref.is_interface


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
