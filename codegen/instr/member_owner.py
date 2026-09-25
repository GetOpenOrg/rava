"""
成员归属解析：JVM 方法/字段解析（JVMS §5.4.3）——沿继承链 / 接口闭包找到成员的
声明类（owner），含方法引用（javap 注释）解析、bridge 目标解析、根类方法集等
invoke / invoke_virtual / invoke_sig / emitter 共用的解析基础设施。

从 coerce.py 拆出（2026-09-20 窗口 2，方案 §3.4）。归属微调：bridge 目标解析
（_bridge_call_target/_resolve_bridge_target）与 parse_method_ref 按真实依赖
（member_owner 的接收者解析段需要它们，而 member_naming 又依赖本模块的
_resolve_method_owner）放在本模块而非计划表的 member_naming。
"""

import re

from ..type_map import short_cls as _short_cls_g
from ..constants import safe_ident as _safe_field, OBJECT_CLASS as _OBJECT_CLASS
from ..sig_types import hierarchy_overloaded_names
from ..type_map import (
    parse_descriptor_params, parse_descriptor_return,
    mangle_name, jvm_to_rust,
)
from ..render import render_expr, render_type
from .invoke_sig import receiver_type_arg_map, type_var_receiver_bound_view, _lookup_method_sig_params

from .hierarchy import _rust_type_to_binary


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


def private_interface_method_target(iface_binary: str, mname: str, descriptor: str,
                                    registry: 'dict | None') -> 'str | None':
    """invokevirtual / invokeinterface 常量池类为接口且目标为私有实例方法（Java 9+）
    时的声明接口 binary；非此形态返回 None。解析与 _resolve_interface_special_target
    同源（自身优先、广度遍历父接口、描述符精确匹配），额外要求命中方法带 ACC_PRIVATE
    且有方法体（私有方法必然非抽象）。"""
    if not registry:
        return None
    root = registry.get(iface_binary)
    if root is None or not root.is_interface:
        return None
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
        for m in ci.methods:
            if (m.name == mname and m.descriptor == descriptor
                    and not m.is_static and not m.is_abstract
                    and (m.access_flags & 0x0002)):
                return cur
        queue.extend(ci.interfaces or [])
    return None


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


_ROOT_PROTECTED_VOID: 'set[tuple[str, str]] | None' = None


def _root_protected_void_methods() -> set[tuple[str, str]]:
    """根类的 protected、void 返回实例方法集 {(name, '(params)')}（动态解析 JDK
    Object.class，不写方法名字面量）。

    子类上对它的调用（`this.m()`，整条静态祖先链未覆盖）语义即根类实现；经
    手写 Object 的同名 API 承载（调用侧另需该名在手写 API 名面中）。仅 void：
    有返回值的 protected 根方法（浅拷贝）由调用结果发射的专门形态承担。"""
    global _ROOT_PROTECTED_VOID
    if _ROOT_PROTECTED_VOID is None:
        _ROOT_PROTECTED_VOID = set()
        try:
            from ..classfile import parse_class_bytes
            from ..jdk_resolver import JdkResolver
            with JdkResolver() as _res:
                _data = _res.resolve(_OBJECT_CLASS)
            if _data is not None:
                for _m in parse_class_bytes(_data, _OBJECT_CLASS).methods:
                    if (_m.is_static or _m.is_synthetic or _m.name.startswith('<')
                            or not (_m.access_flags & 0x0004)     # ACC_PROTECTED
                            or not _m.descriptor.endswith(')V')):
                        continue
                    _ROOT_PROTECTED_VOID.add((_m.name, _m.descriptor.split(')')[0] + ')'))
        except RuntimeError:
            pass
    return _ROOT_PROTECTED_VOID


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

def _declaring_interface(iface_ci, mname: str, descriptor: str, registry: dict) -> str:
    """接口 iface_ci（或其超接口，广度优先）中声明实例方法 mname:descriptor 的接口 binary name。
    根类方法的重声明（经 Object vtable 分派）、私有 / 合成方法不算接口成员 → ''。"""
    if (mname, descriptor[:descriptor.index(')') + 1]) in _root_virtual_methods():
        return ''
    queue = [iface_ci]
    seen: set[str] = set()
    while queue:
        cur = queue.pop(0)
        if cur.name in seen:
            continue
        seen.add(cur.name)
        for m in cur.methods:
            if (m.name == mname and m.descriptor == descriptor and not m.is_static
                    and not m.is_synthetic and not (m.access_flags & 0x0002)):
                return cur.name
        queue.extend(registry[i] for i in (cur.interfaces or []) if i in registry)
    return ''
def _close_open_type_args(sim, stack_idx: int) -> None:
    """菱形构造结果（类型实参待推断的 `X<_, A>`）直接作方法调用的接收者
    （`new Task<>(helper, spliterator).invoke()`）：值不流经任何带类型的位置（局部声明、形参、
    字段、返回值），待推断的类型实参没有约束来源 → 取其擦除（根类），写回构造处的 turbofish。"""
    import re as _re_open
    from ..rs_ir import LetStmt as _LetStmt, RawExpr as _RawExpr, RsNamed as _RsNamed, Var as _Var
    expr, ty = sim.stack[stack_idx]
    ty_s = render_type(ty)
    _open = r'(?<![A-Za-z0-9_])_(?![A-Za-z0-9_])'
    if '<' not in ty_s or not _re_open.search(_open, ty_s):
        return
    if not isinstance(expr, _Var):
        # 构造表达式本身在栈上（`new X<>(..).m()` 未绑定临时变量）
        _base, _args = ty_s.split('<', 1)
        _open_tf = f"{_base}::<{_args[:-1]}>::"
        _code = render_expr(expr)
        if _code.startswith(_open_tf):
            _closed = _re_open.sub(_open, 'Object', _args[:-1])
            sim.stack[stack_idx] = (_RawExpr(f"{_base}::<{_closed}>::" + _code[len(_open_tf):]),
                                    _RsNamed(f"{_base}<{_closed}>"))
        return
    if any(_loc[0] == expr.name for _loc in sim.locals.values()):
        return      # Java 局部变量：类型实参由声明 / 后续用法确定
    base, args = ty_s.split('<', 1)
    open_tf = f"{base}::<{args[:-1]}>::"
    closed_args = _re_open.sub(_open, 'Object', args[:-1])
    for stmt in reversed(sim.stmts):
        if isinstance(stmt, _LetStmt) and stmt.name == expr.name:
            if not (isinstance(stmt.value, _RawExpr) and stmt.value.code.startswith(open_tf)):
                return
            stmt.value = _RawExpr(f"{base}::<{closed_args}>::" + stmt.value.code[len(open_tf):])
            closed_ty = _RsNamed(f"{base}<{closed_args}>")
            if stmt.ty is not None:
                stmt.ty = closed_ty
            if stmt.value_ty is not None:
                stmt.value_ty = closed_ty
            sim.stack[stack_idx] = (expr, closed_ty)
            return


def _resolve_virtual_sig_params(sim, cls: str, mname: str, params: list, ret: str,
                                class_name: str, registry: dict | None):
    """接收者解析与泛型实参映射（§3.7 从 _gen_invokevirtual 上移）：
    在弹出参数前 peek 接收者类型，建立 callee 类型参数 → 接收者实参的映射
    （如 HashMap<E,Object> → K=E），并按「接口经具体类接收者 / bridge 擦除描述符 /
    常量池类」三级策略解析被调方法的真实形参类型列表。会改写接收者栈位
    （类型变量 → 上界视图、菱形 `_` 实参 → 擦除闭合）。"""
    # 在弹出参数前先 peek 接收者类型（在栈顶之下 len(params) 个位置），
    # 解析泛型实参以建立 callee 类型参数 → 接收者实参的映射（如 HashMap<E,Object> → K=E）
    _recv_targ_map: dict | None = None
    _recv_is_this = False
    _recv_ty = ''
    _recv_stack_idx = len(params)
    if len(sim.stack) > _recv_stack_idx:
        import re as _re_recv
        # 接收者是 this（继承到类里的接口 default 方法体）：接口类型形参即本类类型形参
        _recv_is_this = render_expr(sim.stack[-(_recv_stack_idx + 1)][0]) == 'this'
        # 类型变量接收者（task.makeChild(..)，task: K，K extends B<..,K>）：方法定义在
        # 上界类的 wrapper 上 → 接收者换成上界类型视图（与 getfield/putfield 同规则），
        # 之后的签名查找、实参映射、分派均按上界类型进行。无类上界时保持原样。
        _rv_e, _rv_t = sim.stack[-(_recv_stack_idx + 1)]
        _bv_e, _bv_t = type_var_receiver_bound_view(sim, _rv_e, _rv_t)
        if _bv_t is not _rv_t:
            sim.stack[-(_recv_stack_idx + 1)] = (_bv_e, _bv_t)
        _close_open_type_args(sim, len(sim.stack) - (_recv_stack_idx + 1))
        _recv_ty = render_type(sim.stack[-(_recv_stack_idx + 1)][1])
        _recv_targ_map = receiver_type_arg_map(_recv_ty, cls, registry)
    sig_params_v = None
    from ..stack import erased_base as _erased_base_v
    _recv_base_v = _erased_base_v(_recv_ty)
    if registry and _recv_base_v and _recv_base_v != cls and not _recv_is_this:
        # 接口方法经具体类接收者调用（`Map<Long,String> m = new HashMap<>(); m.put(k, v)`，
        # 局部变量的 Rust 类型是构造出的类实例化）：Rust 侧解析到类自身的方法，
        # 形参类型按类的声明签名 + 接收者实参确定，而非接口的擦除载体形态
        _recv_cls_ci = registry.get(_rust_type_to_binary(_recv_base_v, registry) or '')
        _call_cls_ci = registry.get(_rust_type_to_binary(cls, registry) or '')
        if (_recv_cls_ci is not None and not _recv_cls_ci.is_interface
                and _call_cls_ci is not None and _call_cls_ci.is_interface):
            sig_params_v = _lookup_method_sig_params(
                _recv_base_v, mname, params, ret, registry, sim.class_type_params,
                receiver_targ_map=receiver_type_arg_map(_recv_ty, _recv_base_v, registry),
                receiver_is_this=False,
                receiver_type=_recv_ty,
            )
    if registry and sig_params_v is None:
        # 调用描述符在接收者类上只命中 synthetic bridge（`copyInto(Object[],int)` →
        # `copyInto(Integer[],int)`）：Rust 侧只生成被桥接的真实方法，形参类型按真实方法确定。
        # 接收者类链上有真实（非 synthetic）精确声明时优先（JVM 方法解析序：自身/父类链
        # 声明先于接口桥接——`AbstractIntSpliterator.tryAdvance(Consumer)` 是真声明，
        # 不得走 OfInt 的桥接解析拿 IntConsumer 形参）
        _br_recv_bin = (class_name if _recv_is_this
                        else _rust_type_to_binary(_recv_base_v, registry)) if (_recv_is_this or _recv_base_v) else ''
        _br_recv_ci = registry.get(_br_recv_bin or '')
        _br_desc_full = '(' + ''.join(params) + ')' + ret
        _br_has_real = False
        if _br_recv_ci is not None and not _br_recv_ci.is_interface:
            _br_walk, _br_seen = _br_recv_ci, set()
            while _br_walk is not None and _br_walk.name not in _br_seen:
                _br_seen.add(_br_walk.name)
                if any(not m.is_synthetic and m.name == mname
                       and m.descriptor == _br_desc_full for m in _br_walk.methods):
                    _br_has_real = True
                    break
                _br_walk = registry.get(_br_walk.super_class) if _br_walk.super_class else None
        if (_br_recv_ci is not None and not _br_recv_ci.is_interface
                and not _br_has_real):
            _br_desc = '(' + ''.join(params) + ')' + ret
            _br_target = _resolve_bridge_target(_br_recv_ci, mname, _br_desc, registry)
            if _br_target is not None and _br_target[1] != _br_desc:
                _, _, _br_params, _br_ret = parse_method_ref(f"{mname}:{_br_target[1]}")
                if len(_br_params) == len(params):
                    sig_params_v = _lookup_method_sig_params(
                        _short_cls_g(_br_target[0].name), mname, _br_params, _br_ret, registry,
                        sim.class_type_params,
                        receiver_targ_map=receiver_type_arg_map(
                            _recv_ty, _short_cls_g(_br_target[0].name), registry),
                        receiver_is_this=_recv_is_this,
                        receiver_type=_recv_ty,
                    ) or [jvm_to_rust(_p, registry) for _p in _br_params]
    if sig_params_v is None:
        sig_params_v = _lookup_method_sig_params(
            cls, mname, params, ret, registry, sim.class_type_params,
            receiver_targ_map=_recv_targ_map,
            receiver_is_this=_recv_is_this,
            receiver_type=_recv_ty,
        )
    return sig_params_v
