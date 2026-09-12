"""
JDK 虚方法分派表。
接口驱动的分派逻辑，替代 instr.py 中的硬编码类名 if-chain。
Phase D 后整个文件删除。
"""

from .stack import StackSim, BOOL, I32
from .rs_ir import RawExpr, RawStmt, Var, Lit, RsNamed, RsGeneric


# ── 接口 → group 映射（只含接口 binary name，不含实现类名）────────────

JDK_INTERFACE_GROUPS: dict[str, str] = {
    'java/util/List':       'list',
    'java/util/Collection': 'list',
    'java/util/Map':        'map',
    'java/util/Set':        'set',
}


def get_dispatch_group(cls_binary: str, registry: dict) -> str | None:
    """
    通过接口层次（而非类名枚举）确定分派组。
    registry: {binary_name: ClassInfo}，ClassInfo 有 interfaces 属性（list[str]）。
    """
    info = registry.get(cls_binary)
    if info is None:
        return None
    # 递归遍历所有接口（包括接口的父接口）
    visited = set()
    queue = list(info.interfaces)
    while queue:
        iface = queue.pop()
        if iface in visited:
            continue
        visited.add(iface)
        if group := JDK_INTERFACE_GROUPS.get(iface):
            return group
        # 继续查找接口的父接口
        iface_info = registry.get(iface)
        if iface_info:
            queue.extend(iface_info.interfaces)
    return None


def dispatch_virtual(sim, obj_expr, obj_ty_node, obj_e, obj_ty, cls, mname, args, class_name, registry=None):
    """
    Phase B-C 过渡期的 JDK 虚方法分派。
    优先用接口 group 查找（支持 registry 时），降级到 obj_ty 字符串前缀匹配。
    Phase D 后整个 jdk_dispatch.py 删除。

    返回 True 表示已处理，False 表示未命中（调用者继续处理）。
    """
    # 1. 尝试接口驱动查找（需要 registry）
    group = None
    if registry and cls:
        group = get_dispatch_group(cls, registry)

    # 2. 降级：从 obj_ty 字符串推断（过渡期，无 registry 时使用）
    if group is None:
        if obj_ty.startswith('ArrayList<') or obj_ty.startswith('Vec<') or cls in ('ArrayList', 'List', 'Collection'):
            group = 'list'
        elif obj_ty.startswith('HashMap<') or cls in ('HashMap', 'LinkedHashMap', 'TreeMap', 'Map'):
            group = 'map'
        elif obj_ty.startswith('HashSet<') or cls in ('HashSet', 'TreeSet', 'Set'):
            group = 'set'

    if group == 'list':
        _dispatch_list(sim, obj_e, obj_ty, mname, args)
        return True
    elif group == 'map':
        _dispatch_map(sim, obj_e, mname, args)
        return True
    elif group == 'set':
        _dispatch_set(sim, obj_e, mname, args)
        return True
    return False


def _dispatch_list(sim: StackSim, obj: str, obj_ty: str, mname: str, args: list):
    """ArrayList 方法分发（java.util.ArrayList 同构 API）"""
    if mname == 'add':
        item = args[0] if args else 'String::new()'
        sim.emit(RawStmt(f"{obj}.add({item})?;"))
        sim.push(Lit('true'), BOOL)  # 占位，会被 pop 丢弃
    elif mname == 'get':
        idx = args[0] if args else '0i32'
        if obj_ty.startswith('ArrayList<'):
            elem_ty = obj_ty[10:-1]
        elif obj_ty.startswith('Vec<'):
            elem_ty = obj_ty[4:-1]
        else:
            elem_ty = 'String'
        v = sim.fresh('_e')
        sim.emit(RawStmt(f"let {v}: {elem_ty} = {obj}.get({idx})?;"))
        sim.push(Var(v), RsNamed(elem_ty))
    elif mname == 'size':
        sim.push(RawExpr(f"{obj}.size()"), I32)
    elif mname == 'isEmpty':
        sim.push(RawExpr(f"{obj}.is_empty()"), BOOL)
    elif mname == 'remove':
        idx = args[0] if args else '0i32'
        sim.emit(RawStmt(f"{obj}.remove_at({idx});"))
    elif mname == 'set':
        idx = args[0]; val = args[1] if len(args) > 1 else 'String::new()'
        v = sim.fresh('_old')
        sim.emit(RawStmt(f"let {v} = {obj}.set_at({idx}, {val})?;"))
        sim.push(Var(v), RsNamed('String'))
    elif mname == 'contains':
        sim.push(RawExpr(f"{obj}.contains(&{args[0] if args else 'String::new()'})"), BOOL)
    elif mname == 'clear':
        sim.emit(RawStmt(f"{obj}.clear();"))
    else:
        sim.emit(RawStmt(f"/* ArrayList.{mname} */"))


def _dispatch_map(sim: StackSim, obj: str, mname: str, args: list):
    """HashMap 方法分发（java.util.HashMap 同构 API）"""
    if mname == 'put':
        k = args[0]; v_val = args[1] if len(args) > 1 else 'String::new()'
        sim.emit(RawStmt(f"{obj}.put({k}, {v_val});"))
        sim.push(RawExpr('None::<String>'), RsGeneric('Option', [RsNamed('String')]))  # 占位
    elif mname == 'get':
        k = args[0] if args else 'String::new()'
        v = sim.fresh('_v')
        sim.emit(RawStmt(f"let {v} = {obj}.get(&{k}).unwrap_or_default();"))
        sim.push(Var(v), RsNamed('String'))  # 实际类型由 _fix_coll_types 后处理修正
    elif mname == 'getOrDefault':
        k = args[0]; d = args[1] if len(args) > 1 else 'String::new()'
        v = sim.fresh('_v')
        sim.emit(RawStmt(f"let {v} = {obj}.get_or_default(&{k}, {d});"))
        sim.push(Var(v), RsNamed('String'))
    elif mname == 'size':
        sim.push(RawExpr(f"{obj}.size()"), I32)
    elif mname == 'containsKey':
        sim.push(RawExpr(f"{obj}.contains_key(&{args[0] if args else 'String::new()'})"), BOOL)
    elif mname == 'containsValue':
        sim.push(Lit('false'), BOOL)  # 简化实现
    elif mname == 'remove':
        sim.emit(RawStmt(f"{obj}.remove(&{args[0] if args else 'String::new()'});"))
    elif mname == 'isEmpty':
        sim.push(RawExpr(f"{obj}.is_empty()"), BOOL)
    else:
        sim.emit(RawStmt(f"/* HashMap.{mname} */"))


def _dispatch_set(sim: StackSim, obj: str, mname: str, args: list):
    """HashSet 方法分发（java.util.HashSet 同构 API）"""
    if mname == 'add':
        sim.emit(RawStmt(f"{obj}.add({args[0] if args else 'String::new()'});"))
        sim.push(Lit('true'), BOOL)
    elif mname == 'contains':
        sim.push(RawExpr(f"{obj}.contains(&{args[0] if args else 'String::new()'})"), BOOL)
    elif mname == 'size':
        sim.push(RawExpr(f"{obj}.size()"), I32)
    elif mname == 'remove':
        sim.emit(RawStmt(f"{obj}.remove(&{args[0] if args else 'String::new()'});"))
    elif mname == 'isEmpty':
        sim.push(RawExpr(f"{obj}.is_empty()"), BOOL)
    else:
        sim.emit(RawStmt(f"/* HashSet.{mname} */"))
