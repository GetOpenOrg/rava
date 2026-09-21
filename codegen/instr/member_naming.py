"""
命名与 ref 解析：重载 mangle（_mangle_if_overloaded）、invokedynamic 实现方法命名
（G-10 单一来源 lambda_impl_rust_name + _LambdaNameLedger 断言）、字段/slot 引用
解析、类已知性判定。依赖 member_owner（owner 解析决定名字的声明类视角）。

从 coerce.py 拆出（2026-09-20 窗口 2，方案 §3.4）。
"""

import re

from ..type_map import short_cls as _short_cls_g
from ..constants import safe_ident as _safe_field, JAVA_RUNTIME_SHORT_NAMES as _JAVA_RUNTIME_SHORT_NAMES
from ..sig_types import hierarchy_overloaded_names
from ..type_map import mangle_name
from .member_owner import _resolve_method_owner, _resolve_bridge_target


def _parse_slot(op: str, operand: str) -> int:
    if '_' in op:
        return int(op.split('_')[-1])
    return int(operand.strip()) if operand else 0


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
    # 接收者类（常量池类 / 栈上静态类型）：wrapper 成员名（本类声明 + 继承成员
    # 的 receiver_member_name）都按接收者重载态发射——最终名字必须同视角。
    recv_ci = target_ci
    # 声明类解析：调用目标类（常量池里的类）可能只是继承了该方法，沿父类链解析到
    # 真正声明 name+descriptor 的类（手写类判定 / bridge 描述符换源用）。
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
                from ..sig_types import interface_member_local_name as _iface_local
                _local = _iface_local(target_ci, mname, _call_desc_m.group(1), registry)
                return _JAVA_RUST_RENAME.get(_local, _local)
    # 按 (name, descriptor) 判定，单一权威 hierarchy_overloaded_names：
    #   - 类接收者：接收者视角（wrapper 上本类声明与继承成员同名——inherited_gen
    #     的 receiver_member_name 机制），声明者态可能因接收者新增同名重载而发散；
    #   - 接口接收者：声明者视角（接口继承成员按声明接口的重载态命名——
    #     interface_gen 机制，如 Sink 从 Consumer 继承的 accept 裸名）。
    _name_ci = target_ci if recv_ci.is_interface else recv_ci
    _is_mangled = mname in hierarchy_overloaded_names(_name_ci, registry)
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
