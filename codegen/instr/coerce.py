"""
纯辅助函数：类型强制转换、字段/方法查找、字面量处理。
无 StackSim 状态，可被 invoke.py 和 sim.py 安全导入。
"""

import re
from ..constants import safe_ident as _safe_field
from ..type_map import (
    parse_descriptor_params, parse_descriptor_return,
    mangle_name,
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

    m = re.match(r'(?:([^.]+)\.)?(\w+(?:<\w+>)?):(\([^)]*\).+)', comment)
    if not m:
        return (None, comment, [], 'V')

    raw_cls = m.group(1)
    mname   = m.group(2).replace('__init__', '<init>').replace('__clinit__', '<clinit>')
    desc    = m.group(3)

    if raw_cls:
        raw_cls = raw_cls.split('/')[-1].split('.')[-1].replace('$', '_')

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
        return f"({expr_str} as i32)"
    return expr_str


def _coerce_to_object(val_str: str, ty: str) -> str:
    """将任意类型的值强制转换为 Object。
    基本类型用 .into()（有 From<T> for Object 实现）；
    其他类型用 Object::from_any(.clone())（通用装箱）。"""
    if ty in ('i32', 'i64', 'f32', 'f64', 'bool', 'i8', 'i16', 'u16'):
        return f"{val_str}.into()"
    return f"Object::from_any({val_str}.clone())"


_NULL_OBJECT_EXPRS = frozenset({'Object::default()', 'Object::default().clone()'})

# 接口类型 → 已知实现类型集合（用于接口子类型强制转换）
_INTERFACE_IMPLS: dict[str, frozenset[str]] = {
    'CharSequence': frozenset({'String', 'StringBuilder', 'StringBuffer', 'AbstractStringBuilder'}),
    'Map': frozenset({'HashMap', 'TreeMap', 'LinkedHashMap', 'Hashtable', 'WeakHashMap',
                      'IdentityHashMap', 'ConcurrentHashMap', 'Properties', 'EnumMap',
                      'ImmutableCollections_Map1', 'ImmutableCollections_MapN'}),
    'List': frozenset({'ArrayList', 'LinkedList', 'Vector', 'Stack', 'AbstractList',
                       'ImmutableCollections_List12', 'ImmutableCollections_ListN',
                       'ProviderList_ServiceList', 'ImmutableCollections_SubList'}),
    'Set': frozenset({'HashSet', 'LinkedHashSet', 'TreeSet', 'EnumSet',
                      'ImmutableCollections_Set12', 'ImmutableCollections_SetN',
                      'TreeMap_EntrySet', 'HashMap_KeySet', 'ConcurrentHashMap_KeySetView'}),
    'Collection': frozenset({'List', 'Set', 'ArrayList', 'LinkedList', 'HashSet',
                              'LinkedHashSet', 'TreeSet', 'Vector', 'ArrayDeque'}),
    'Iterable': frozenset({'Collection', 'List', 'Set', 'ArrayList', 'HashSet', 'LinkedList'}),
    'Queue': frozenset({'LinkedList', 'ArrayDeque', 'PriorityQueue'}),
    'Deque': frozenset({'LinkedList', 'ArrayDeque'}),
    'SortedMap': frozenset({'TreeMap'}),
    'SortedSet': frozenset({'TreeSet'}),
    'NavigableMap': frozenset({'TreeMap'}),
    'NavigableSet': frozenset({'TreeSet'}),
}


def _coerce_to_interface(actual: str, expected: str) -> bool:
    """当 actual 需要强制转换为 Default::default() 时返回 True。
    覆盖两类情况：
    1. actual 是 expected 接口的已知实现类（如 HashMap → Map）
    2. Vec 元素类型不匹配（Rc<RefCell<Vec<Object>>> → Rc<RefCell<Vec<T>>>）
    """
    # Vec 元素类型不匹配：两者都是 Rc<RefCell<Vec<T>>> 但元素类型不同
    _VEC_PREFIX = 'Rc<RefCell<Vec<'
    _VEC_SUFFIX = '>>>'
    if (expected.startswith(_VEC_PREFIX) and expected.endswith(_VEC_SUFFIX) and
            actual.startswith(_VEC_PREFIX) and actual.endswith(_VEC_SUFFIX)):
        exp_elem = expected[len(_VEC_PREFIX):-len(_VEC_SUFFIX)]
        act_elem = actual[len(_VEC_PREFIX):-len(_VEC_SUFFIX)]
        if exp_elem != act_elem:
            return True
    exp_base = expected.split('<')[0]
    act_base = actual.split('<')[0]
    if exp_base == act_base:
        return False
    return act_base in _INTERFACE_IMPLS.get(exp_base, frozenset())


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
        if not sc or sc == 'java/lang/Object':
            break
        path_parts.append('_super')
        if sc == to_cls:
            return '.'.join(path_parts) + '.'
        ci = registry.get(sc)
    return ''


def _find_field_super_prefix(class_name: str, safe_fname: str, registry: dict | None) -> str:
    """T76: 找到字段 safe_fname 在继承链中的位置，返回 _super 访问前缀。
    bytecode getfield/putfield 的 comment 里的 cls 是接收者静态类型（不是声明类），
    所以需要通过字段名在注册表中查找来计算 _super 路径。
    返回 '' 表示字段在当前类直接字段中，返回 '_super.' 或 '_super._super.' 等。"""
    if not registry or not class_name:
        return ''
    ci = registry.get(class_name)
    if ci is None:
        return ''
    # 当前类直接字段中是否有该字段
    direct_names = {_safe_field(f.name) for f in ci.fields if not f.is_static}
    if safe_fname in direct_names:
        return ''
    # 向上遍历继承链查找
    path_parts: list[str] = []
    sc = ci.super_class
    while sc and sc != 'java/lang/Object' and sc in registry:
        path_parts.append('_super')
        parent_ci = registry[sc]
        parent_names = {_safe_field(f.name) for f in parent_ci.fields if not f.is_static}
        if safe_fname in parent_names:
            return '.'.join(path_parts) + '.'
        sc = parent_ci.super_class
    return ''


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
    cur_short = current_binary.rsplit('/', 1)[-1].replace('$', '_') if '/' in current_binary else current_binary.replace('$', '_')
    if target_cls_short == cur_short or target_cls_short == current_binary:
        return ''  # 同类调用（私有方法）：不需要 _super 路由
    ci = registry.get(current_binary)
    if ci is None:
        return '_super.'  # fallback
    path_parts: list[str] = []
    sc = ci.super_class
    while sc and sc != 'java/lang/Object':
        path_parts.append('_super')
        sc_short = sc.rsplit('/', 1)[-1].replace('$', '_') if '/' in sc else sc.replace('$', '_')
        if sc_short == target_cls_short or sc == target_cls_short:
            return '.'.join(path_parts) + '.'
        sc_ci = registry.get(sc)
        if sc_ci is None:
            break
        sc = sc_ci.super_class
    return '_super.'  # fallback: 至少一级 _super（目标类在继承链上但未在 registry 中）


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
    if descriptor:
        if any(m.name == mname and m.descriptor == descriptor for m in real_methods):
            return ''
    else:
        if any(m.name == mname for m in real_methods):
            return ''
    # 向上遍历继承链查找（同样排除 synthetic/bridge）
    path_parts: list[str] = []
    sc = ci.super_class
    while sc and sc != 'java/lang/Object' and sc in registry:
        path_parts.append('_super')
        parent_ci = registry[sc]
        parent_real_methods = [m for m in parent_ci.methods if not m.is_synthetic]
        if descriptor:
            found = any(m.name == mname and m.descriptor == descriptor for m in parent_real_methods)
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
    for binary in registry:
        last = binary.rsplit('/', 1)[-1] if '/' in binary else binary
        # Java 内部类 $ → Rust _
        if last.replace('$', '_') == rust_short:
            return binary
    return ''


def _is_subtype(child_rust: str, parent_rust: str, registry: dict | None) -> bool:
    """判断 child_rust 是否是 parent_rust 的子类型（通过 registry 继承链+接口链查找）。
    两个参数都是 Rust 短类名（如 IOException, Throwable）。"""
    if not registry or child_rust == parent_rust:
        return False
    child_bin = _rust_type_to_binary(child_rust, registry)
    if not child_bin:
        return False

    def _short(binary: str) -> str:
        return binary.rsplit('/', 1)[-1].replace('$', '_') if '/' in binary else binary

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
        if ci.super_class and ci.super_class != 'java/lang/Object':
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
        return b.rsplit('/', 1)[-1].replace('$', '_') if '/' in b else b
    # 直接超类
    if ci.super_class and ci.super_class != 'java/lang/Object':
        if _short(ci.super_class) == parent_rust:
            return True
    # 直接接口列表
    for iface in (ci.interfaces or []):
        if _short(iface) == parent_rust:
            return True
    return False


def _find_field_super_prefix_for_type(recv_rust_type: str, fname: str, registry: dict | None) -> str:
    """基于接收者 Rust 类型（短名）查找字段 _super 前缀。"""
    binary = _rust_type_to_binary(recv_rust_type, registry)
    if binary:
        return _find_field_super_prefix(binary, fname, registry)
    return ''


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
        if not sc or sc == 'java/lang/Object':
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
        if key.rsplit('/', 1)[-1].replace('$', '_') == norm:
            return True
    return False


_JAVA_RUST_NAME_CONFLICTS = frozenset()
_JAVA_RUST_RENAME: dict[str, str] = {}


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
            if key.rsplit('/', 1)[-1].replace('$', '_') == norm:
                target_ci = ci
                break
    if target_ci is None:
        return _JAVA_RUST_RENAME.get(mname, mname)
    # 排除 java_runtime 类（registry 中仍有其 JDK 字节码副本，但方法名不 mangle）
    if target_ci.name.rsplit('/', 1)[-1] in _JAVA_RUNTIME_SHORT_NAMES:
        return mname
    visible = [m for m in target_ci.methods if not m.is_synthetic]
    same = sum(1 for m in visible if m.name == mname)
    # 计入接口 default 方法（未覆盖时由 class_writer 注入到 impl 块）
    if target_ci.interfaces and not target_ci.is_interface:
        _iq = list(target_ci.interfaces)
        _iv: set[str] = set()
        _seen_sigs: set[tuple] = {(m.name, m.descriptor) for m in visible}
        while _iq:
            _in = _iq.pop(0)
            if _in in _iv:
                continue
            _iv.add(_in)
            _ici = registry.get(_in)
            if _ici:
                _iq.extend(_ici.interfaces or [])
                for _dm in _ici.methods:
                    if (not _dm.is_abstract and not _dm.is_static and not _dm.is_synthetic
                            and _dm.name == mname
                            and (_dm.name, _dm.descriptor) not in _seen_sigs):
                        same += 1
                        _seen_sigs.add((_dm.name, _dm.descriptor))
    if same <= 1:
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


# Java 中任何对象都可以传递给 Object 参数（引用协变），Rust 需要显式 Into<Object> 转换
_PRIMITIVE_RUST_TYPES: frozenset[str] = frozenset({
    'i32', 'i64', 'f32', 'f64', 'bool', 'i8', 'i16', 'u16', '()'
})
