"""调用链发现引擎（BFS）——从用户类种子出发的方法级闭包计算。

CLAUDE.md 规则 2 的核心：只追踪实际被调用的方法，未调用方法不展开。
本模块自 transpile.py 拆出（2026-09-20 窗口 0①，纯搬移零逻辑改动）；
依赖方向 transpile → callchain 单向。闭包簇 → 显式状态类的转换留待
该文件下次被写入时进行（见 docs/plans/2026-09-20-source-file-split-plan.md §3.5）。
"""

import os
import re
from collections import deque

from .constants import MAIN_DESC
from .constants import (OBJECT_CLASS as _OBJECT_CLASS, CLASS_CLASS as _CLASS_CLASS,
                        RUNTIME_JAVA_RUNTIME as _RUNTIME_JAVA_RUNTIME)
from . import fallback_audit
from .runtime_manifest import read_list


# JDK 包前缀（binary name 斜线分隔）- 这些类的方法会被 BFS 展开并翻译
# 只展开公开 API（java/ javax/）；内部实现包（sun/ jdk/ com.sun/ com.oracle/）截断为 stub
_JDK_PREFIXES = ('java/', 'javax/')

# 内部包边界：方法体全部为 panic! stub，不展开调用链
# 这是内部边界截断策略的核心——sun/ 等包的实现细节不翻译，只生成类型占位符
# 前缀名单维护在 runtime/java_runtime/boundary_prefixes.txt（P-1：库知识不进生成器）
_JDK_STUB_ONLY_PREFIXES: tuple[str, ...] = tuple(read_list('boundary_prefixes.txt'))

# K-JCA 放行（codegen/jca_services.py，清单 jca_providers.txt 的 release 行）：算法实现包
# （纯 Java 计算）与 engine / SPI / 工具类从边界前缀放行、按字节码翻译。放行项并入
# _JDK_PREFIXES（「展开并翻译」的集合）：包前缀原样；类条目在 java/ javax/ 下天然属于该集合，
# 其余前缀下的类条目（sun/security/util/ArrayUtil）须显式并入——否则其方法引用不进调用链，
# 调用点落 `stub: ArrayUtil.blockSizeCheck`（DES doFinal 实证）。类条目作前缀匹配同时覆盖其嵌套类。
from .jca_services import load_manifest as _jca_manifest, released as _jca_released
_JCA_MANIFEST = _jca_manifest()
_JDK_PREFIXES = _JDK_PREFIXES + tuple(
    r for r in _JCA_MANIFEST.release
    if r.endswith('/') or not r.startswith(('java/', 'javax/')))

# 通用边界放行（清单 boundary_release.txt，与 JCA release 行同一语义）：内部前缀下的纯 Java
# 类 / 包按字节码翻译（MH-native：sun/invoke/util 的类型转换工具）。并入 _JDK_PREFIXES 使其
# 方法引用进调用链；_is_boundary_class 据 _released_general 放行。
_BOUNDARY_RELEASE: tuple[str, ...] = tuple(read_list('boundary_release.txt'))
_JDK_PREFIXES = _JDK_PREFIXES + tuple(
    r for r in _BOUNDARY_RELEASE if r.endswith('/') or not r.startswith(('java/', 'javax/')))


def _released_general(cls: str) -> bool:
    """类是否在通用边界放行清单内（包前缀 / 类及其嵌套类）。"""
    for r in _BOUNDARY_RELEASE:
        if r.endswith('/'):
            if cls.startswith(r):
                return True
        elif cls == r or cls.startswith(r + '$'):
            return True
    return False



# 手写 impl 文件内的类型引用：crate::pkg::path::Name 全路径 + 同包裸 CamelCase 名
_IMPL_FULL_PATH_RE = re.compile(r'crate::((?:r#\w+|\w+)(?:::(?:r#\w+|\w+))*)::([A-Z]\w*)')
_IMPL_BARE_NAME_RE = re.compile(r'\b([A-Z][A-Za-z0-9_]*)\b')


def _impl_signature_type_refs(cls: str, runtime_src: str, resolver) -> list[str]:
    """手写共置 `_impl.rs` / `_ext.rs` 签名引用的 JDK 类（binary name 列表）。

    手写实现的返回/参数类型是宿主类的真实依赖：如 Thread.getThreadGroup 的
    手写签名返回 ThreadGroup——该类不在闭包时 scratch 编译 E0425。映射两条路：
    `crate::<pkg>::<Name>` 全路径；同包裸名（impl 文件经 `use super::*` 引入）。
    候选都经 resolver 验证存在，避免把注释/字符串词误当类名。"""
    from .emitter.attrs import to_snake

    *pkg, simple = cls.split('/')
    base = os.path.join(runtime_src, *pkg) if pkg else runtime_src
    out: list[str] = []
    for suffix in ('_impl.rs', '_ext.rs'):
        path = os.path.join(base, to_snake(simple) + suffix)
        if not os.path.isfile(path):
            continue
        try:
            content = open(path, encoding='utf-8').read()
        except OSError:
            continue
        # 自动生成的类文件碰巧以 _impl.rs / _ext.rs 结尾（类名本身含 Impl/Ext，
        # 如 java/net/InetAddressImpl → inet_address_impl.rs，恰落在同包 InetAddress
        # 的共置 impl 路径上）：scratch 复用模式下上一轮的生成残留会伪装成手写
        # impl，其类型引用被误入队（净 +4 field stub 的 clean/warm 闭包漂移，
        # TestCollectionsUtil 实证 365→369）。与 native_upcalls._load 同判据跳过：
        # 生成类文件恒含限定宏调用，手写 impl 恒不含（含裸 `java_class!` 的
        # doc 注释不误伤）。
        if 'java_rta_macros::java_class' in content:
            continue
        for m in _IMPL_FULL_PATH_RE.finditer(content):
            pkg_path = m.group(1).replace('::', '/').replace('r#', '')
            cand = _resolve_impl_ref(f'{pkg_path}/{m.group(2)}', resolver)
            if cand:
                out.append(cand)
        for m in _IMPL_BARE_NAME_RE.finditer(content):
            if not pkg:
                continue
            cand = _resolve_impl_ref('/'.join(pkg + [m.group(1)]), resolver)
            if cand:
                out.append(cand)
    return out


def _resolve_impl_ref(cand: str, resolver) -> 'str | None':
    """手写 impl 中的类型名 → 可解析的 binary name。嵌套类的 Rust 名以 `_` 连接
    （`Provider$Service` → `Provider_Service`）：直译不中时按 `_` → `$` 还原再试。"""
    if resolver.resolve(cand) is not None:
        return cand
    head, _, simple = cand.rpartition('/')
    if '_' in simple:
        nested = f'{head}/{simple.replace("_", "$")}' if head else simple.replace('_', '$')
        if resolver.resolve(nested) is not None:
            return nested
    return None


def _read_manifest(name: str) -> list[str]:
    """读取 runtime/java_runtime/ 下的 VM 清单文件（统一入口 runtime_manifest.read_list）。"""
    return read_list(name)


# VM 耦合边界类：公开包里由 JVM 自身引导 / 承载 VM 设施（模块系统、类加载、安全管理器等）的类。
# 它们在原生二进制里没有字节码层面的对应物，与内部包同规则：BFS 在此截断，整体手写、按需实现。
# 清单在 runtime/（手写层真源）维护，生成器不出现任何 JDK 类名。
_VM_BOUNDARY_CLASSES: frozenset[str] = frozenset(_read_manifest('vm_boundary.txt'))


# 纯数据资源束豁免（L-1）：内部包（前缀）下经结构判定为纯数据类
# （codegen/data_bundle.py）的类照常翻译。判定需要 ClassInfo，由 BFS 注册类加载器后
# 惰性求值并缓存——所有边界决策点（BFS 入队、clinit 提取）经 _is_boundary_class
# 同一入口，判定结果一致，与首次触达的先后无关。未注册加载器时不放行（纯前缀规则）。
_DATA_BUNDLE_VERDICT: dict[str, bool] = {}
_DATA_BUNDLE_LOADER: list = [None]


# 本轮入选的资源束类（binary name，已排序）：emitter 在生成 main 中登记构造闭包
#（runtime `data_bundles::register_data_bundles`）。每轮 BFS 起始清空。
DATA_BUNDLE_SEEDS: list[str] = []

# 本轮入选的 JCA 服务（jca_services.Service，已排序）：emitter 在生成 main 中登记构造闭包
#（runtime `jca::register_services`）。每轮 BFS 起始清空。
JCA_SEEDS: list = []


def set_data_bundle_loader(load) -> None:
    """注册（或以 None 清除）结构判定用的类加载器 load(binary_name) → ClassInfo|None。"""
    _DATA_BUNDLE_LOADER[0] = load
    _DATA_BUNDLE_VERDICT.clear()


def _is_data_bundle(cls: str) -> bool:
    if '[' in cls:
        return False
    verdict = _DATA_BUNDLE_VERDICT.get(cls)
    if verdict is None:
        load = _DATA_BUNDLE_LOADER[0]
        if load is None:
            return False
        from .data_bundle import is_pure_data_bundle
        verdict = is_pure_data_bundle(load(cls), load)
        _DATA_BUNDLE_VERDICT[cls] = verdict
    return verdict


def _is_boundary_class(cls: str) -> bool:
    """内部包（前缀）或 VM 耦合边界类（清单，含其嵌套类）；前缀内的纯数据资源束与 K-JCA 放行类除外。"""
    if cls.startswith(_JDK_STUB_ONLY_PREFIXES):
        return not (_jca_released(cls, _JCA_MANIFEST) or _released_general(cls)
                    or _is_data_bundle(cls))
    return cls.split('$', 1)[0] in _VM_BOUNDARY_CLASSES


# java_runtime 已手写实现的类：这些类不再由 jdk_classes 翻译，避免重复定义和命名冲突
_JAVA_RUNTIME_CLASSES: frozenset[str] = frozenset({
    _OBJECT_CLASS,
})


def _desc_class_refs(desc: str) -> list[str]:
    """从方法/字段 descriptor 提取类引用（Lxxx/yyy; 形式，含数组元素类型）。"""
    return re.findall(r'L([^;]+);', desc or '')


def _discover_jdk_classes_method_level(class_infos: list, runtime_src: str | None = None, *,
                                       lib_registries: list | None = None,
                                       lib_prefixes: tuple[str, ...] = (),
                                       extra_seed_classes: list[str] | None = None,
                                       locales: tuple[str, ...] = ()) -> list:
    """方法级调用链 BFS：只追踪实际被调用的方法，不展开未调用方法的依赖类。

    调用边的三个来源：
      1. 字节码方法引用，按 JVMS §5.4.3.3 解析到声明者（父类链 → 父接口 default）
      2. 虚调用的运行期目标（RTA）：已实例化类（其 <init> 在调用链上）对虚调用目标的覆盖版本
      3. 手写 native / 内部边界方法声明的 Java 回调（runtime_src 下共置 `_impl.rs` 的 upcalls）

    jar 输入模式的扩展参数（默认值 = 既有 .java 路径行为，零改动）：
      lib_registries      - 依赖 jar 的类注册表列表（binary name → ClassInfo，枚举自
                            jar 条目）。_load_class 在 JDK jmods 之前优先查它们，
                            使跨 jar 引用（junit→hamcrest）可解析；命中者进
                            jdk_infos（调用链通道）——由驱动层按 jar 归属拆分到
                            对应 lib crate，不落 java_runtime。lib 类不进
                            class_infos（用户类通道全量翻译、不参与虚分派传播），
                            与 JDK 类同走可达性发现：RTA / 接口传播、存根与父类
                            补全通道对 lib 类统一生效。
      lib_prefixes        - jar 类的 binary name 前缀（如 ('org/junit/',)）。
                            _collect_method_refs / _translatable / 接口闭包等
                            原以 _JDK_PREFIXES 为界的门对 lib 前缀同权放开。
      extra_seed_classes  - lib crate 的种子类 binary name 列表：整包模式 = jar
                            全部类（公开 API 面 = 全部 public 类成员入链）；
                            子集模式（M2 junit Assert 子集）= 种子类列表（其余
                            jar 类经调用链可达性发现）。
    """
    from .classfile import parse_class_bytes
    from .jdk_resolver import JdkResolver
    from .native_upcalls import NativeUpcalls

    # jar 类注册表（lib crate 输入）：_load_class / 存根通道在 JDK jmods 之前
    # 优先命中——库自身与其跨 jar 引用（junit→hamcrest）从 jar 解析。
    _lib_registries: list = lib_registries or []

    def _lib_lookup(name: str):
        for _reg in _lib_registries:
            if name in _reg:
                return _reg[name]
        return None

    def _resolve_class_bytes(name: str):
        """jar 优先的类解析（存根 / 父类补全通道共用）：jar 类已解析直接取，其余走 jmods。"""
        if lib_prefixes and name.startswith(lib_prefixes):
            return _lib_lookup(name)
        _data = resolver.resolve(name)
        try:
            return parse_class_bytes(_data, name) if _data is not None else None
        except Exception:
            return None

    # 用户类（含内部类）按 binary name 索引：方法解析沿继承层次查找时同样可见。
    # 没有这一步，以用户类为常量池类的方法引用（如 enum 子类调用继承自 JDK 基类的
    # name()/ordinal()）在 _process 中找不到声明者，JVM 方法解析（JVMS §5.4.3.3）
    # 的父类链分支不会发生，基类方法永远不入调用链。
    user_infos: dict[str, object] = {ci.name: ci for ci in class_infos}
    user_names: frozenset[str] = frozenset(user_infos)

    upcalls = NativeUpcalls(runtime_src) if runtime_src else None
    # 以根类为常量池类的虚方法引用 (name, descriptor)：根类手写、不入 visited_methods
    root_virtual_targets: set[tuple[str, str]] = set()
    # 以内部边界类（接口/抽象类）为常量池类的方法引用：边界类自身手写、不入 visited_methods，
    # 但其公开 API 包内的已实例化子类型（如匿名访问器类）的覆盖版本在调用链上
    boundary_virtual_targets: set[tuple[str, str, str]] = set()

    visited_methods: set[tuple[str, str, str]] = set()
    queue: deque[tuple[str, str, str]] = deque()
    # 通过 getstatic/Field 指令发现的类：只生成存根，不展开方法体
    field_discover_classes: set[str] = set()

    def _enqueue_iface_stub(cls_bin: str) -> None:
        """类声明的直接接口（传递闭包）进 type-only stub 通道。

        interface_gen 的 _all_interfaces 沿 registry 的 interfaces 边发现类的
        接口 impl 关系（ListItr 实现 ListIterator、分派经 Iterator 载体——
        ListIterator 缺席 registry 时 Iterator impl 关系不可达 → AbstractMethodError）。
        与 _enqueue_desc_types 同通道：只生成类型存根，不展开方法体（调用未触达
        的接口方法不扩大生成范围）。
        """
        _iq = deque([cls_bin])
        _iseen: set[str] = set()
        while _iq:
            _in = _iq.popleft()
            if _in in _iseen:
                continue
            _iseen.add(_in)
            _ici = _load_class(_in)
            if _ici is None:
                continue
            for _sup in (_ici.interfaces or []):
                if (_sup not in _JAVA_RUNTIME_CLASSES
                        and (_sup.startswith(_JDK_PREFIXES)
                             or _sup.startswith(_JDK_STUB_ONLY_PREFIXES)
                             or _sup.startswith(lib_prefixes))
                        and _sup not in field_discover_classes):
                    field_discover_classes.add(_sup)
                _iq.append(_sup)

    def _enqueue_desc_types(desc: str) -> None:
        """T88：方法描述符的参数/返回类型也是类型依赖（type-only）。

        abstract 方法无方法体（instrs 为空），其签名引用的接口类型
        （如 AbstractCollection.iterator() 的 Ljava/util/Iterator;）若不收集
        则不会进闭包 → 签名引用的名字落到 prelude trait 报 E0782。
        走 field_discover_classes 通道：只生成类型存根，不展开方法体。
        与 trace_callchain.py 的 _extract_desc_classes 思路对齐。
        """
        for tcls in _desc_class_refs(desc):
            if (tcls not in _JAVA_RUNTIME_CLASSES
                    and (tcls.startswith(_JDK_PREFIXES)
                         or tcls.startswith(_JDK_STUB_ONLY_PREFIXES)
                         or tcls.startswith(lib_prefixes))):
                field_discover_classes.add(tcls)

    # 类初始化（JVMS §5.5）已入队的类
    init_enqueued: set[str] = set()
    # static 字段访问触发点：(常量池类, 字段名)，待 resolver 可用后解析到声明类
    pending_static_fields: deque[tuple[str, str]] = deque()

    # 调用链溯源：每个入队方法记录其来源方法（JAVA_RTA_BFS_TRACE=<类 binary name> 时输出入链路径）
    enqueued_from: dict[tuple[str, str, str], tuple[str, str, str] | None] = {}
    origin: list = [None]

    seen_members: set[tuple[str, str]] = set()
    instantiated_classes: set[str] = set()   # RTA：调用链上被 new 出来的类
    # 边界接口回调边（延迟解析：种子阶段 _load_class 尚未定义，收集后待
    # _propagate_virtual_targets 前 drain）
    _pending_iface_edges: list[tuple[str, str, str]] = []
    # 已确认接口形态的回调边键（迟至 stub/父类通道的实现者清扫用，见函数尾注）
    _drained_iface_keys: list[tuple[str, str, str]] = []

    def _enqueue_method(key: tuple[str, str, str]) -> None:
        if key[0] in _JAVA_RUNTIME_CLASSES:
            return
        if key not in visited_methods:
            visited_methods.add(key)
            enqueued_from[key] = origin[0]
            queue.append(key)

    # 以边界类为常量池类、实际声明在其翻译祖先上的方法引用（JVMS §5.4.3.3 方法解析沿
    # 超类链找声明者）：`secureRandom.nextInt()` 的常量池类是边界类 SecureRandom，声明在
    # java/util/Random（翻译类）——不解析则 Random.nextInt 停在存根。种子阶段类加载器
    # 未就绪 → 暂存，由不动点循环的 _drain_boundary_refs 解析入队。
    _pending_boundary_refs: list = []
    _boundary_refs_done: set = set()

    def _drain_boundary_refs() -> None:
        while _pending_boundary_refs:
            key = _pending_boundary_refs.pop()
            if key in _boundary_refs_done:
                continue
            _boundary_refs_done.add(key)
            bcls, bmeth, bdesc = key
            if bmeth in ('<init>', '<clinit>'):
                continue
            cur, seen = bcls, set()
            while cur and cur not in seen:
                seen.add(cur)
                ci = _load_class(cur)
                if ci is None:
                    break
                if any(m.name == bmeth and m.descriptor == bdesc for m in ci.methods):
                    if cur != bcls and cur.startswith(_JDK_PREFIXES) and not _is_boundary_class(cur):
                        _enqueue_method((cur, bmeth, bdesc))
                    break
                cur = ci.super_class

    def _enqueue_upcalls(cls: str, member: str) -> None:
        """被触达成员的手写实现声明的 Java 回调目标入队（native → Java 的调用边）。"""
        if upcalls is None or (cls, member) in seen_members:
            return
        seen_members.add((cls, member))
        for tcls, tmeth, tdesc in upcalls.lookup(cls, member):
            # 回调目标自身也是被触达的成员：其手写实现可继续声明回调
            _enqueue_upcalls(tcls, tmeth)
            if tcls == _OBJECT_CLASS:
                if tmeth not in ('<init>', '<clinit>'):
                    root_virtual_targets.add((tmeth, tdesc))
            elif _is_boundary_class(tcls):
                if tcls not in _JAVA_RUNTIME_CLASSES:
                    field_discover_classes.add(tcls)
                    # 接口级回调边的边界接口形态（RandomSupport 族先例）：upcall 目标
                    # 声明在边界接口（java/security/PrivilegedAction 等）上时，实现类
                    # （匿名内部类等，在公开包里由字节码翻译）无法经声明接口方法键入队。
                    # 延迟到 _load_class 可用后解析：接口则入队方法键，使
                    # _propagate_virtual_targets 的接口分支把闭包内全部具体实现类的
                    # 覆盖版本（含协变桥方法）带入调用链——实现类零枚举。
                    _pending_iface_edges.append((tcls, tmeth, tdesc))
            elif tcls.startswith(_JDK_PREFIXES):
                _enqueue_method((tcls, tmeth, tdesc))
                if tmeth == '<init>':
                    instantiated_classes.add(tcls)   # 手写实现构造的 Java 对象
            _enqueue_desc_types(tdesc)

    _pending_reflect_consts: list = []
    _reflect_seen: set = set()
    REFLECT_CONSTS.clear()

    def enqueue_refs(instrs, exception_table=()):
        (method_refs, f_classes, member_refs, boundary_refs, new_classes,
         static_refs) = _collect_method_refs(instrs, user_class_names=user_names,
                                             extra_prefixes=lib_prefixes)
        boundary_virtual_targets.update(boundary_refs)
        _pending_boundary_refs.extend(boundary_refs)
        instantiated_classes.update(new_classes)
        pending_static_fields.extend(static_refs)
        # 异常表的 catch_type：catch 分派按类层次匹配，只需类型存根（不引入任何方法）
        for _entry in (exception_table or ()):
            _catch = _entry[3]
            if _catch and _catch.startswith(_JDK_PREFIXES + _JDK_STUB_ONLY_PREFIXES):
                f_classes.append(_catch)
        for cls in f_classes:
            if cls not in _JAVA_RUNTIME_CLASSES:
                field_discover_classes.add(cls)
        if _CLASS_CLASS in f_classes:
            # Class 对象由手写层构造（Class::for_class，ldc 类字面量 / getClass 等 native），
            # 无 `new` 指令可见——不记为已实例化则根类虚方法（toString/hashCode/equals）
            # 不传播到 Class 的覆盖版本，经 Object 视图调用落 vtable 默认体（Rosetta
            # SumDataType：`"" + obj.getClass()` 输出 java/lang/Class，N6 同族）
            instantiated_classes.add(_CLASS_CLASS)
        for key in method_refs:
            if key[0] == _OBJECT_CLASS and key[1] not in ('<init>', '<clinit>'):
                # 根类虚方法（toString/hashCode/equals…）：实际目标是已实例化类的覆盖版本
                root_virtual_targets.add((key[1], key[2]))
            _enqueue_method(key)
        for cls, member in member_refs:
            _enqueue_upcalls(cls, member)
        _pending_reflect_consts.extend(_scan_reflect_consts(instrs))

    # 初始种子：用户类所有方法的引用（既有 .java 路径行为，不变）
    for ci in class_infos:
        for m in ci.methods:
            enqueue_refs(m.instrs or [], m.exception_table)
            # T88：用户方法自身描述符里的参数/返回类型也是类型依赖
            # （abstract 方法无 instrs，但其签名引用的接口类型要进闭包）
            _enqueue_desc_types(m.descriptor)

    # lib crate 种子（jar 输入模式）：种子类的 public 成员是 crate 的公开 API 面
    # （classfile access_flags 驱动，零猜测）。方法键自身入链——公开成员是消费
    # 者入口，未被他方调用的公开方法也必须有翻译体（否则退化为 panic 存根，
    # 库语义破损）；非 public 成员经调用图从 public 方法可达时自然入链。
    for _seed in (extra_seed_classes or []):
        _sci = _lib_lookup(_seed)
        if _sci is None:
            print(f"[bfs-audit] lib seed 未命中 jar 注册表: {_seed}")
            continue
        for m in _sci.methods:
            if not (m.access_flags & 0x0001):   # ACC_PUBLIC
                continue
            # main(String[]) 不入 lib 种子面：它是命令行入口而非库 API（bin
            # 入口由用户 main 承担），且其体引用 args 形参——发射侧的 main
            # 参数省略约定（method_gen：`pub fn main()`）会造出悬空引用
            #（JUnitCore.main→runMain(args) 实证）。可达性不受影响：有真实
            # 调用边时照常入链（此时按普通调用边语义发射）。
            if m.is_static and m.name == 'main' and m.descriptor == MAIN_DESC:
                continue
            _enqueue_method((_seed, m.name, m.descriptor))
            enqueue_refs(m.instrs or [], m.exception_table)
            _enqueue_desc_types(m.descriptor)

    class_cache: dict[str, object] = {}
    jdk_infos: dict[str, object] = {}

    # 语料版本自动匹配：用户 .class 的最高主版本 → 对应 JDK 语料。
    # major-44 即 JDK 主版本（65→21、69→25）；--jdk 显式指定写入 JAVA_HOME，
    # 优先级高于此自动推导（find_java_home 的解析顺序）。混合版本取最高。
    _max_major = max((getattr(ci, 'major_version', 0) for ci in class_infos), default=0)
    _prefer = _max_major - 44 if _max_major >= 45 else None

    try:
        if _prefer:
            print(f"      语料选择：用户类 class 版本 {_max_major} → JDK {_prefer} jmods"
                  f"（--jdk 显式指定优先）", flush=True)
        resolver = JdkResolver(prefer_major=_prefer)
    except RuntimeError as e:
        print(f"      警告：{e}，跳过 JDK 元数据生成")
        return [], set(), set()

    def _load_class(name: str):
        """按需解析类（不加入生成范围，仅供方法解析沿继承层次查找）。

        用户类不在 JDK 档案里，从入参 class_infos 取；jar 类（lib crate 输入）
        优先于 JDK jmods——跨 jar 引用与库自身类都从 jar 注册表解析。
        """
        if name in user_infos:
            return user_infos[name]
        if lib_prefixes and name.startswith(lib_prefixes):
            _lci = _lib_lookup(name)
            if _lci is not None:
                # 落 class_cache：虚分派传播（RTA / 接口实现扫描）按 class_cache
                # 迭代 visited_methods——lib 类不落缓存则其覆盖方法永不入链
                #（StringDescription.append(String) 覆盖实证：protected 覆盖不
                # 在公开种子面，靠传播到达），运行期命中 panic 存根。
                class_cache[name] = _lci
                return _lci
        if name not in class_cache:
            _data = resolver.resolve(name)
            try:
                class_cache[name] = parse_class_bytes(_data, name) if _data is not None else None
            except Exception as e:
                # B 组计数+警告（fallback-audit §4.1）：类置 None = 静默移出层次
                # 遍历（调用链缺口源），指名道姓记明细
                fallback_audit.record('cc-load-class', f"{name}: {e!r}")
                class_cache[name] = None
        return class_cache[name]

    # L-1：纯数据资源束的结构判定经同一类加载器（jdk.localedata 等全部 jmod 可解析）
    set_data_bundle_loader(_load_class)
    DATA_BUNDLE_SEEDS.clear()
    JCA_SEEDS.clear()

    _sig_poly_native: set[tuple] = set()   # 精确匹配失败但链上有同名 ACC_NATIVE（签名多态边界，预期内）
    _root_inherited: set[tuple] = set()    # 声明者落到手写根类 Object（根 vtable 桥接承载，预期内）
    _unresolved_calls: set[tuple] = set()  # 链上完全无同名声明（真实缺口，需排查）
    _root_method_names: frozenset | None = None

    def _object_method_names() -> frozenset:
        """手写根类 Object 声明的方法名集合（从 jmods 解析 Object.class，无字面量）。
        根类方法由 vtable 桥接机制（#[hash_code_vtable] 等）承载，不经 BFS 入队。"""
        nonlocal _root_method_names
        if _root_method_names is None:
            _names: set[str] = set()
            try:
                _data = resolver.resolve(_OBJECT_CLASS)
                if _data is not None:
                    _names = {m.name for m in parse_class_bytes(_data, _OBJECT_CLASS).methods}
            except Exception as e:
                # B 组计数+警告：根方法名空 → 根继承判定全数落入 unresolved
                fallback_audit.record('cc-root-names', repr(e))
            _root_method_names = frozenset(_names)
        return _root_method_names

    def _record_unresolved(ci, meth: str, desc: str) -> None:
        """签名多态调用（MethodHandle.invokeBasic / VarHandle.get 等）的调用点
        描述符由 javac 按实参生成，与声明描述符必然不同 → 精确匹配必败，此前
        静默 return（trace 完整版发现：HelloWorld 规模此类目标 1500+ 条）。
        命中同名 ACC_NATIVE 的归 native 边界（手写层职责）；链上完全无同名
        声明的才是真实缺口。「为什么这个方法没被翻译」从此可观测。"""
        _cur, _seen = ci, set()
        while _cur is not None and _cur.name not in _seen:
            _seen.add(_cur.name)
            if any(m.name == meth and m.is_native for m in _cur.methods):
                _sig_poly_native.add((ci.name, meth, desc))
                return
            _sc = _cur.super_class
            if not _sc or _sc in _JAVA_RUNTIME_CLASSES:
                # 链到手写根类：Object 声明的同名方法由根 vtable 桥接承载（预期内），
                # 其余才是真实缺口
                if meth in _object_method_names():
                    _root_inherited.add((ci.name, meth, desc))
                    return
                break
            _cur = _load_class(_sc)
        _unresolved_calls.add((ci.name, meth, desc))

    def _enqueue_declaring_method(ci, meth: str, desc: str) -> bool:
        """JVM 方法解析（JVMS §5.4.3.3）的接口分支：常量池类及其父类链都未声明
        (meth, desc) 时，实际执行的是父接口的 default 方法。把声明接口的该方法入队，
        使接口进入 registry、default 方法体注入实现类并进入 vtable。
        abstract 声明无方法体，不入队（不为无体方法扩大生成范围）。
        """
        if meth in ('<init>', '<clinit>'):
            return True   # 非方法解析范畴，不算未解析
        _chain = []
        _cur = ci
        _seen: set[str] = set()
        while _cur is not None and _cur.name not in _seen:
            _seen.add(_cur.name)
            _chain.append(_cur)
            _sc = _cur.super_class
            if not _sc or _sc in _JAVA_RUNTIME_CLASSES:
                break
            _cur = _load_class(_sc)
        # 本类及父类链任一处声明了 (meth, desc) → 由类层次自身承载，不属于 default 方法解析
        for _anc in _chain[1:]:
            if any(m.name == meth and m.descriptor == desc for m in _anc.methods):
                # JVMS §5.4.3.3 步骤 2：实际执行的是最近祖先类声明的方法 → 该方法入队
                if _is_boundary_class(_anc.name):
                    field_discover_classes.add(_anc.name)
                else:
                    _enqueue_method((_anc.name, meth, desc))
                return True
        _owner = None
        if _owner is None:
            _iq = deque(i for _c in _chain for i in (_c.interfaces or []))
            _iseen: set[str] = set()
            _via: dict[str, str] = {}   # 接口 → 经由哪个子接口到达（回溯继承路径用）
            while _iq and _owner is None:
                _in = _iq.popleft()
                if _in in _iseen:
                    continue
                _iseen.add(_in)
                _ici = _load_class(_in)
                if _ici is None:
                    continue
                # abstract 声明同样是有效的解析结果：实现体在实现类层次里
                # （S-18——Stream.sequential 声明于 BaseStream，实现于
                # AbstractPipeline）。default（有方法体）则由下方入队。
                _hit = next((m for m in _ici.methods
                             if m.name == meth and m.descriptor == desc), None)
                if _hit is not None:
                    _owner = (_ici, _hit)
                    break
                for _sup in (_ici.interfaces or []):
                    _via.setdefault(_sup, _in)
                    _iq.append(_sup)
        if _owner is None:
            return False   # 链上与接口闭包都无精确声明 → 调用方按未解析归类
        # 实现类 → 声明接口之间的中间接口必须进 registry（仅类型别名级 stub），
        # 否则 default 方法注入沿 interfaces 向上遍历时在缺失节点处断链。
        _step = _via.get(_owner[0].name)
        while _step is not None:
            if (_step not in _JAVA_RUNTIME_CLASSES
                    and not _is_boundary_class(_step)):
                field_discover_classes.add(_step)
            _step = _via.get(_step)
        _oci, _om = _owner
        if _om.is_abstract or _om.is_static or _oci.name in _JAVA_RUNTIME_CLASSES:
            return True   # 已解析到声明，按策略不入队（不为无体方法扩大生成范围）
        if _is_boundary_class(_oci.name):
            field_discover_classes.add(_oci.name)
            return True
        _key = (_oci.name, meth, desc)
        if _key not in visited_methods:
            visited_methods.add(_key)
            queue.append(_key)
        return True

    def _translatable(name: str) -> bool:
        return (bool(name) and name not in _JAVA_RUNTIME_CLASSES
                and (name.startswith(_JDK_PREFIXES)
                     or name.startswith(lib_prefixes))
                and not _is_boundary_class(name))

    def _enqueue_class_init(name: str) -> None:
        """类初始化触发（getstatic / putstatic / invokestatic / new）：该类及其父类链的
        <clinit> 进入调用链。内部边界类无字节码翻译，在边界处截断。"""
        _cur = name
        while _translatable(_cur) and _cur not in init_enqueued:
            init_enqueued.add(_cur)
            _ci = _load_class(_cur)
            if _ci is None:
                return
            if any(m.name == '<clinit>' for m in _ci.methods):
                _key = (_cur, '<clinit>', '()V')
                if _key not in visited_methods:
                    visited_methods.add(_key)
                    enqueued_from[_key] = origin[0]
                    queue.append(_key)
            _cur = _ci.super_class

    def _static_field_owner(name: str, field: str) -> str:
        """JVMS §5.4.3.2 字段解析：本类 → 父接口 → 父类，返回声明类。"""
        _seen: set[str] = set()
        _stack = [name]
        while _stack:
            _n = _stack.pop(0)
            if not _n or _n in _seen:
                continue
            _seen.add(_n)
            _ci = _load_class(_n) if _translatable(_n) else None
            if _ci is None:
                continue
            if any(f.is_static and f.name == field for f in _ci.fields):
                return _n
            _stack.extend(_ci.interfaces or [])
            _stack.append(_ci.super_class)
        return name

    def _drain_static_fields() -> None:
        while pending_static_fields:
            _cls, _field = pending_static_fields.popleft()
            _enqueue_class_init(_static_field_owner(_cls, _field))

    _VM_UPCALL_RE = re.compile(r'^//\s*vm-upcalls:\s*(.+)$')

    def _load_vm_roots() -> list[tuple[str, str, str]]:
        """VM 基础设施的 Java 依赖种子（Rust→Java 反向边，不在任何字节码里）：
        声明在使用处——error.rs 头部的 `// vm-upcalls:` 行，格式与 upcalls 属性
        一致（空白分隔的 类.方法:描述符）。共置 _impl.rs 的反向边由
        native_upcalls 的属性机制在被触达时按需入队；此处只收基础设施的
        无条件种子（原独立清单 vm_roots.txt 已并入，机制统一）。"""
        _roots = []
        _err_path = os.path.join(_RUNTIME_JAVA_RUNTIME, 'src', 'error.rs')
        try:
            with open(_err_path, encoding='utf-8') as _f:
                for _line in _f:
                    _m = _VM_UPCALL_RE.match(_line)
                    if not _m:
                        continue
                    for _tok in _m.group(1).split():
                        if '.' not in _tok or ':' not in _tok:
                            continue
                        _owner, _, _desc = _tok.partition(':')
                        _cls, _, _meth = _owner.rpartition('.')
                        _roots.append((_cls, _meth, _desc))
        except OSError:
            pass
        return _roots

    def _process(cls: str, meth: str, desc: str) -> None:
        # 解析类（首次遇到时）
        ci = _load_class(cls)
        if ci is None:
            return
        # 用户类由 user crate 从 class_infos 整体生成，不进 jdk_infos（否则重复定义）
        if cls not in jdk_infos and cls not in user_infos:
            jdk_infos[cls] = ci
            # 类声明的接口闭包进 stub 通道（_enqueue_iface_stub 注释——接口 impl
            # 关系的发现依赖 registry 的 interfaces 边）
            if (cls.startswith(_JDK_PREFIXES) or cls.startswith(_JDK_STUB_ONLY_PREFIXES)
                    or cls.startswith(lib_prefixes)):
                _enqueue_iface_stub(cls)
        origin[0] = (cls, meth, desc)

        # 追踪该方法的指令引用（精确匹配名字+描述符，避免重载方法误展开）
        _declared = False
        for m in ci.methods:
            if m.name == meth and m.descriptor == desc:
                _declared = True
                enqueue_refs(m.instrs or [], m.exception_table)
                # 常量反射引用的隐式本类形态：`getNamedFunction("name", type)` 一类按名
                # 查找以所在类为宿主，只有名字常量（MH-native §二-5）
                _own = {x.name for x in ci.methods}
                for _ins in (m.instrs or []):
                    _c = _ins.comment or ''
                    if _ins.opcode.startswith('ldc') and _c.startswith('String '):
                        _nm = _c[len('String '):]
                        if _nm in _own and _nm != meth:
                            _pending_reflect_consts.append((cls, _nm))
                # T88：被调方法的描述符参数/返回类型也是类型依赖
                # （abstract/native 方法无 instrs，签名引用的接口类型
                # 如 iterator()Ljava/util/Iterator; 仍需进闭包生成）
                _enqueue_desc_types(m.descriptor)
                # invokestatic / new 是类初始化触发点
                if m.is_static or m.name == '<init>':
                    _enqueue_class_init(cls)
        if not _declared:
            if not _enqueue_declaring_method(ci, meth, desc):
                _record_unresolved(ci, meth, desc)
        _drain_static_fields()

    with resolver:
        for _root in _load_vm_roots():
            if _root not in visited_methods:
                visited_methods.add(_root)
                queue.append(_root)
        # 用户类的父类链：用户类初始化先初始化其 JDK 父类
        for _uci in class_infos:
            _enqueue_class_init(_uci.super_class)
        _drain_static_fields()

        # 根类的方法被所有类继承（手写 ObjectVTable 的签名与字节码一致），
        # 其描述符中的参数/返回类型是全局类型依赖，必须进闭包
        _root_data = resolver.resolve(_OBJECT_CLASS)
        if _root_data is not None:
            try:
                for _rm in parse_class_bytes(_root_data, _OBJECT_CLASS).methods:
                    _enqueue_desc_types(_rm.descriptor)
            except Exception as e:
                # B 组计数+警告：根描述符类型闭包丢边（全局类型依赖缺口）
                fallback_audit.record('cc-root-desc', repr(e))


        _supertype_cache: dict[str, frozenset] = {}

        def _supertypes(name: str) -> frozenset:
            """name 的全部超类型（含自身、父类链、传递父接口）。"""
            if name in _supertype_cache:
                return _supertype_cache[name]
            _supertype_cache[name] = frozenset({name})   # 环保护
            acc = {name}
            _ci = _load_class(name) if name not in _JAVA_RUNTIME_CLASSES else None
            if _ci is not None:
                for _up in ([_ci.super_class] if _ci.super_class else []) + list(_ci.interfaces or []):
                    acc |= _supertypes(_up)
            _supertype_cache[name] = frozenset(acc)
            return _supertype_cache[name]

        _ACC_PRIVATE, _ACC_FINAL = 0x0002, 0x0010

        def _resolve_virtual_slot(ci, meth: str, desc: str):
            """JVMS §5.4.3.3 声明槽位解析：本类 → 父类链（最近优先）→ 接口闭包。
            调用点的常量池类未必自身声明该方法（S-18：invokeinterface
            Stream.sequential，声明在父接口 BaseStream、实现体在抽象类
            AbstractPipeline）——此时槽位沿继承层次解析，接口闭包里的
            abstract 声明与 default 同样是有效槽位。返回 (声明者, 方法) 或 None。
            """
            _cur, _seen = ci, set()
            _chain = []
            while _cur is not None and _cur.name not in _seen:
                _seen.add(_cur.name)
                _chain.append(_cur)
                _m = next((m for m in _cur.methods
                           if m.name == meth and m.descriptor == desc), None)
                if _m is not None:
                    return _cur, _m
                _sc = _cur.super_class
                if not _sc or _sc in _JAVA_RUNTIME_CLASSES:
                    break
                _cur = _load_class(_sc)
            _iq = deque(i for _c in _chain for i in (_c.interfaces or []))
            _iseen: set[str] = set()
            while _iq:
                _in = _iq.popleft()
                if _in in _iseen:
                    continue
                _iseen.add(_in)
                _ici = _load_class(_in)
                if _ici is None:
                    continue
                _m = next((m for m in _ici.methods
                           if m.name == meth and m.descriptor == desc), None)
                if _m is not None:
                    return _ici, _m
                _iq.extend(_ici.interfaces or [])
            return None

        def _drain_iface_edges() -> None:
            """解析延迟的边界接口回调边：目标确为接口则方法键入队。

            判型解析不落 class_cache（_load_class 的缓存写入会把边界类提前
            替换成已解析形态，改变后续 _ci_of/RTA 过滤的可见性——GetBooleanAction
            一类非接口目标的加载副作用曾使 AbstractClassLoaderValue 从 stub 通道
            误入方法通道）。确为接口时经 _enqueue_method → _process 正规加载。"""
            while _pending_iface_edges:
                _edge = _pending_iface_edges.pop()
                _ici = class_cache.get(_edge[0])
                if _ici is None:
                    _data = resolver.resolve(_edge[0])
                    if _data is None:
                        continue
                    try:
                        _ici = parse_class_bytes(_data, _edge[0])
                    except Exception:
                        continue
                if _ici is not None and _ici.is_interface:
                    _drained_iface_keys.append(_edge)
                    _enqueue_method(_edge)

        def _propagate_virtual_targets() -> None:
            """虚调用目标 → 运行期实际接收者类的覆盖版本。

            - 接口方法：闭包内直接实现该接口的具体类（含抽象类中途实现）
            - RTA：已实例化类 X（(X, <init>, *) 在调用链上）是虚调用目标声明类的子类型时，
              (X, m, d) 入队；X 未声明则由 _process 按方法解析规则落到最近声明者
            """
            _drain_iface_edges()
            def _ci_of(name: str):
                """实例化类的 ClassInfo：用户类在 user_infos，JDK 类在 class_cache；未解析过的
                按需解析（手写层构造的实例化类——如 Class——没有 <init> 入链，此前从未被加载，
                会被静默滤出 RTA）。"""
                return user_infos.get(name) or class_cache.get(name) or _load_class(name)

            instantiated = sorted(
                x for x in instantiated_classes
                if _ci_of(x) is not None
                and not _ci_of(x).is_interface and not _ci_of(x).is_abstract
            )
            # sorted：BFS 的传播与入队对集合按全序迭代——集合（visited_methods 等）
            # 的迭代序随 PYTHONHASHSEED 漂移，会使 stub 通道 / 父类补全通道对
            # 边缘类（AbstractClassLoaderValue 一类双通道可达者）的归属在两次
            # 转译间翻转（闭包指纹 ±1、生成树 diff 非零）。排序后处理序确定，
            # 闭包与生成树对哈希种子稳定。
            for cls, meth, desc in sorted(visited_methods):
                if meth in ('<init>', '<clinit>'):
                    continue
                ci = class_cache.get(cls)
                if ci is None:
                    continue
                decl = next((m for m in ci.methods
                             if m.name == meth and m.descriptor == desc), None)
                if decl is None:
                    # 常量池类自身未声明：沿继承层次解析槽位（父类链 → 接口闭包，
                    # abstract 同样有效）。无槽位的方法不构成虚分派目标（无覆盖可传播）。
                    _slot = _resolve_virtual_slot(ci, meth, desc)
                    if _slot is None:
                        continue
                    ci, decl = _slot
                if decl.is_static or (decl.access_flags & (_ACC_PRIVATE | _ACC_FINAL)):
                    continue
                if ci.is_interface:
                    for concrete_name, concrete_ci in list(jdk_infos.items()):
                        if concrete_ci is None or concrete_ci.is_interface:
                            continue
                        # 传递实现（经由父接口间接实现）与直接实现同等对待：
                        # 只查直接 interfaces 会漏掉非实例化但已生成的间接实现类
                        # （其 vtable 注入随之缺失）。_supertypes 有缓存与环保护。
                        if cls in _supertypes(concrete_ci.name):
                            _enqueue_method((concrete_name, meth, desc))
                            # 手写体实现者的回调链（doPrivileged 接口级回调边激活实证）：
                            # 接口边落到边界类实现者（sun/ 包 GetBooleanAction——方法体经
                            # 共置 _impl.rs 的 __impl_run 提供）时，其手写体声明的 Java
                            # 回调（Boolean.valueOf）同样入队。枚举形态的 upcalls
                            # （GetBooleanAction.run 直连）经 _enqueue_upcalls 递归载体
                            # 覆盖此链；接口级翻转后无递归载体，只在方法 BFS 期本分支
                            # 触达的实现者上补——迟至 stub 通道的实现者见函数尾清扫。
                            # 幂等（seen_members）。
                            _enqueue_upcalls(concrete_name, meth)
                for x in instantiated:
                    if x != cls and cls in _supertypes(x):
                        _enqueue_method((x, meth, desc))
            for meth, desc in sorted(root_virtual_targets):
                for x in instantiated:
                    _enqueue_method((x, meth, desc))
            for cls, meth, desc in sorted(boundary_virtual_targets):
                if meth in ('<init>', '<clinit>'):
                    continue
                for x in instantiated:
                    if cls in _supertypes(x):
                        _enqueue_method((x, meth, desc))

        # L-1 资源束种子：手写边界的数据消费入口（locale_seeds.txt 的 trigger）在调用链上
        # 时，按用户字节码推出的 locale 集（含父链）入选 CLDR 束类——构造器 + 载体方法入队、
        # 记为已实例化（ListResourceBundle.handleGetObject 等虚调用经 RTA 分派到束类）。
        # 束经类名反射装载、无静态边，只能在不动点处按触达事实补种；只补一次。
        from .locale_seed import load_manifest as _ls_manifest
        _ls_mf = _ls_manifest()
        _bundles_seeded = False

        def _seed_data_bundles() -> None:
            from .data_bundle import carrier_of
            from .locale_seed import bundle_classes, collect_locales
            _locs = collect_locales(class_infos, _load_class, extra=locales, manifest=_ls_mf)
            _names = bundle_classes(_locs, _load_class, manifest=_ls_mf)
            for _b in _names:
                _carrier = carrier_of(_load_class(_b), _load_class)
                instantiated_classes.add(_b)
                _enqueue_method((_b, '<init>', '()V'))
                if _carrier is not None:
                    _enqueue_method((_b, _carrier[0], _carrier[1]))
            DATA_BUNDLE_SEEDS[:] = _names
            print(f"      locale 种子：{len(_locs)} 个 locale → {len(_names)} 个资源束", flush=True)

        # K-JCA 服务种子：服务查找入口（jca_providers.txt 的 trigger）在调用链上时，按
        # 「engine 类在链上 × 算法名在用户字符串常量中」入选实现类——构造器入队、记为
        # 已实例化（engine 经 SPI 虚调用分派到实现类）。实现类经 Provider$Service.newInstance
        # 按类名反射构造、无静态边。可多轮：新入链的 engine 类（Cipher.init 触达
        # SecureRandom 等）在下一轮不动点补种。
        _jca_services: list = []
        _jca_algos: set = set()
        _jca_seeded: set = set()

        def _seed_jca_services() -> bool:
            from .jca_services import extract_services, select_services, user_algorithm_strings
            if not _jca_services:
                _jca_services.extend(extract_services(_load_class, _JCA_MANIFEST) or [None])
                _jca_algos.update(user_algorithm_strings(class_infos))
            _live = {k[0].rsplit('/', 1)[-1] for k in visited_methods}
            _new = [sv for sv in select_services([x for x in _jca_services if x], _jca_algos, _live)
                    if sv not in _jca_seeded]
            for sv in _new:
                _jca_seeded.add(sv)
                instantiated_classes.add(sv.impl)
                _enqueue_method((sv.impl, '<init>', '()V'))
            if _new:
                JCA_SEEDS[:] = sorted(_jca_seeded)
                print(f"      JCA 种子：{len(_jca_seeded)} 个服务 "
                      f"({', '.join(f'{x.type}.{x.algorithm}' for x in JCA_SEEDS)})", flush=True)
            return bool(_new)

        def _drain_reflect_consts() -> None:
            """常量反射引用 → 被指名方法（全部同名重载）入链并登记分派发射面。
            用户类全量翻译、分派全量发射，只需 JDK / 库类；边界类（手写）不入。"""
            while _pending_reflect_consts:
                pair = _pending_reflect_consts.pop()
                if pair in _reflect_seen:
                    continue
                _reflect_seen.add(pair)
                cls, name = pair
                if cls in user_names or _is_boundary_class(cls) \
                        or not cls.startswith(_JDK_PREFIXES + tuple(lib_prefixes)):
                    continue
                ci = _load_class(cls)
                if ci is None:
                    continue
                hits = [m for m in ci.methods if m.name == name and not m.name.startswith('<')]
                if not hits:
                    continue
                REFLECT_CONSTS.setdefault(cls, set()).add(name)
                for m in hits:
                    _enqueue_method((cls, m.name, m.descriptor))

        # 不动点：排空队列 → 传播虚调用目标 → 有新方法则继续
        while True:
            while queue:
                _process(*queue.popleft())
            _drain_boundary_refs()
            _drain_reflect_consts()
            if queue:
                continue
            if queue:
                continue
            _propagate_virtual_targets()
            if (not queue and not _bundles_seeded
                    and any(t in seen_members for t in _ls_mf.triggers)):
                _bundles_seeded = True
                _seed_data_bundles()
            if (not queue and any(t in seen_members for t in _JCA_MANIFEST.triggers)):
                _seed_jca_services()
            if not queue:
                break

        # 签名多态 / 未解析调用的可观测性（与 JAVA_RTA_BFS_TRACE 溯源互补）
        if _sig_poly_native or _unresolved_calls or _root_inherited:
            print(f"[bfs-audit] sig-poly-native={len(_sig_poly_native)} "
                  f"root-inherited={len(_root_inherited)} "
                  f"unresolved={len(_unresolved_calls)}")
            if os.environ.get('JAVA_RTA_DEBUG'):
                for _k in sorted(_unresolved_calls):
                    print(f"[bfs-audit] unresolved: {_k[0]}.{_k[1]}:{_k[2]}")

        # field_discover_classes + T76 父类链：BFS 处理，递归包含所有父类
        # T76 生成 pub _super: ParentType，需要父类类型存在于 jdk_infos
        # sorted：种子序确定（集合迭代序随哈希种子漂移，见 _propagate_virtual_targets 注）
        _stub_queue: deque[str] = deque(sorted(field_discover_classes))
        _stub_visited: set[str] = set(field_discover_classes)

        # 手写 _impl.rs / _ext.rs 的签名引用类型随宿主类入闭包（type-only stub 通道）。
        # 与 stub 扩展构成不动点：stub 通道新入的宿主类（如经 field_discover 进入的
        # DoubleToDecimal）同样要收集 _impl 签名引用——单轮扫描会漏收伴生 impl 引用
        # 的类型（E0432），曾被旧 scratch 的陈旧 appendable.rs 掩盖（TestCasting 等
        # 四红线干净树实证）。
        _impl_scanned: set[str] = set()

        def _scan_impl_refs() -> bool:
            added = False
            for _cls in list(jdk_infos.keys()):
                if _cls in _impl_scanned:
                    continue
                _impl_scanned.add(_cls)
                for _ref in _impl_signature_type_refs(_cls, runtime_src, resolver):
                    if (_ref not in jdk_infos
                            and _ref not in _JAVA_RUNTIME_CLASSES
                            and (_ref.startswith(_JDK_PREFIXES)
                                 or _ref.startswith(_JDK_STUB_ONLY_PREFIXES))
                            and _ref not in _stub_visited):
                        _stub_visited.add(_ref)
                        field_discover_classes.add(_ref)
                        _stub_queue.append(_ref)
                        added = True
            return added

        while _stub_queue or (runtime_src and _scan_impl_refs()):
            if not _stub_queue:
                continue
            cls = _stub_queue.popleft()
            if cls in jdk_infos:
                continue
            ci = _resolve_class_bytes(cls)
            if ci is None:
                continue
            try:
                jdk_infos[cls] = ci
                # stub 通道类同样收集其接口闭包（同 _process——接口 impl 关系发现）
                _enqueue_iface_stub(cls)
                # 缺口 B（分阶段首期：仅 field_discover 通道）：字段声明类型进闭包
                # （type-only）。未被指令触达的字段类型此前静默退化为 Object
                # （jvm_to_rust 对 registry 外类型的 fallback），继承字段展平时
                # 暴露访问器签名 E0308。stub 通道内的字段类型传递闭包由此展开；
                # BFS 调用链通道（_process）的字段暂不收集（闭包膨胀按 9.3 分阶段）。
                for _f in ci.fields:
                    for _ftcls in _desc_class_refs(_f.descriptor):
                        if (_ftcls not in _JAVA_RUNTIME_CLASSES
                                and (_ftcls.startswith(_JDK_PREFIXES)
                                     or _ftcls.startswith(_JDK_STUB_ONLY_PREFIXES))
                                and _ftcls not in _stub_visited):
                            _stub_visited.add(_ftcls)
                            field_discover_classes.add(_ftcls)
                            _stub_queue.append(_ftcls)
                # 递归添加父类（_super 字段需要父类类型存在）
                if (ci.super_class and ci.super_class != _OBJECT_CLASS
                        and ci.super_class not in _JAVA_RUNTIME_CLASSES
                        and ci.super_class not in jdk_infos
                        and ci.super_class not in _stub_visited):
                    _stub_visited.add(ci.super_class)
                    _stub_queue.append(ci.super_class)
            except Exception as e:
                # B 组计数+警告：stub 通道丢类（此前完全无信号）
                fallback_audit.record('cc-stub-chan', f"{cls}: {e!r}")

        # 同样为 BFS 调用链中发现的类递归添加父类
        _parent_queue: deque[str] = deque()
        for _ci in list(jdk_infos.values()):
            if (_ci and _ci.super_class and _ci.super_class != _OBJECT_CLASS
                    and _ci.super_class not in _JAVA_RUNTIME_CLASSES
                    and _ci.super_class not in jdk_infos):
                _parent_queue.append(_ci.super_class)
        while _parent_queue:
            cls = _parent_queue.popleft()
            if cls in jdk_infos:
                continue
            ci = _resolve_class_bytes(cls)
            if ci is None:
                continue
            try:
                jdk_infos[cls] = ci
                # 父类补全进来的类同样登记其接口闭包（清单第 20 项漏口 ②：Reader→
                # Readable、AccessibleObject→AnnotatedElement 等此前从未登记）；
                # 物化由收尾接口 drain 统一承担
                _enqueue_iface_stub(cls)
                if (ci.super_class and ci.super_class != _OBJECT_CLASS
                        and ci.super_class not in _JAVA_RUNTIME_CLASSES
                        and ci.super_class not in jdk_infos):
                    _parent_queue.append(ci.super_class)
            except Exception as e:
                # B 组计数+警告：父类补全队列丢类（此前完全无信号）
                fallback_audit.record('cc-parent-queue', f"{cls}: {e!r}")

        # 迟至静态边补扫（JDK25 StringLatin1.hashCode → ArraysSupport.hashCodeOfUnsigned
        # 实证）：以内部边界类为常量池类的**静态**方法引用（invokestatic）在
        # _collect_method_refs 落 boundary_refs 通道，而该通道的消费端——虚调用
        # 目标传播（RTA / 接口实现者）——只对实例方法有意义，静态边被确定性丢弃
        # （与时机无关）；发射侧却对该边照常渲染直调（ArraysSupport::hashCodeOfUnsigned(...)），
        # 被调方无手写 impl 时落 panic 存根，运行期命中即 stub 崩溃——发射集与
        # 入队集在此失配。三个通道沉降后按 boundary_virtual_targets 已登记键窄幅
        # 补扫（不重走字节码、不重跑 _propagate_virtual_targets / stub 通道——
        # LocaleSyntaxException E0425 教训）：静态声明、非 native/abstract（有字节码
        # 可译）、无手写 impl 覆盖（与发射侧 _nf_covered 同一真源：共置
        # _impl.rs/_ext.rs 的 pub fn 名，见 NativeUpcalls.provides）的键入队翻译。
        # 手写 impl 已覆盖的键不入队——其字节码引用不是手写体的依赖，入队只会
        # 无谓扩大闭包、扰动既有指纹。补扫只入方法键：类本体留在 stub 通道
        # （cls 集不变，闭包指纹零扰动）；静态键不参与虚分派传播，无需重跑
        # _propagate_virtual_targets。链式静态边（补入方法体引用更多未覆盖边界
        # 静态）与 _process 带进的新类由外层不动点覆盖；排序遍历保确定性。
        while True:
            _added = False
            for _scls, _smeth, _sdesc in sorted(boundary_virtual_targets):
                if (_smeth in ('<init>', '<clinit>')
                        or (_scls, _smeth, _sdesc) in visited_methods):
                    continue
                # 判定探测不落 class_cache（同 _drain_iface_edges 注：提前缓存
                # 会改变 _ci_of / RTA 过滤的可见性）；确需入队时经
                # _enqueue_method → _process 正规加载
                _sci = class_cache.get(_scls)
                if _sci is None:
                    _sdata = resolver.resolve(_scls)
                    try:
                        _sci = (parse_class_bytes(_sdata, _scls)
                                if _sdata is not None else None)
                    except Exception:
                        _sci = None
                    if _sci is None:
                        continue
                _sdecl = next((m for m in _sci.methods
                               if m.name == _smeth and m.descriptor == _sdesc), None)
                if (_sdecl is None or not _sdecl.is_static
                        or _sdecl.is_native or _sdecl.is_abstract):
                    continue
                # 门 1：类处于增量手写管理（impl 文件存在）才补译缺口；无 impl 文件的
                # 边界类保持整体 panic 存根节奏（CLAUDE.md 3b 按需推进，未实现即
                # 如实存根）。无此门，补扫会把全部未覆盖边界静态边（JDK21 Stream
                # 语料 ~90 条：ValueConversions/Modules/ZoneInfoFile 族）及其传递
                # 引用整体展开——正是 4b776b6 记录的 stub 通道类型闭环重开。
                if upcalls is None or not upcalls.has_impls(_scls):
                    continue
                # 门 2：成员已由手写 impl 提供则不入队（字节码引用不是手写体依赖）
                if upcalls.provides(_scls, _smeth):
                    continue
                # 门 3：缺口方法自含可译——其字节码的外部方法引用必须落在已服务
                # 域（公开 API / 用户类 / 同类自身 / 手写 impl 已覆盖成员）。
                # 指向未覆盖边界机制的引用（BootLoader.loadLibrary → 匿名
                # PrivilegedAction、FloatingDecimal.parseHexString → $ASCIIToBinary
                # 转换器族）说明该方法的翻译需要展开新的边界机制——发射层对
                # 边界内部结构（冲突改名的嵌套类型、doPrivileged 的 Object 载体）
                # 的渲染不完整（E0433 BootLoader_1 / 可读层 from_any 实证），这类
                # 边保持既有 panic 存根（命中即如实报缺口，按 3b 节奏手写补全）。
                # 判定只看 _collect_method_refs 的 boundary_refs 通道：公开 API /
                # 用户类 / lib 类引用本就走方法通道可译；边界引用按
                # 同类自身 / impl 覆盖白名单放行。
                _refs_ok = True
                for _rcls, _rmeth, _rdesc in _collect_method_refs(
                        _sdecl.instrs, user_class_names=user_names,
                        extra_prefixes=lib_prefixes)[3]:
                    if _rcls != _scls and not upcalls.provides(_rcls, _rmeth):
                        _refs_ok = False
                        break
                if not _refs_ok:
                    continue
                if os.environ.get('JAVA_RTA_BFS_EDGE_AUDIT'):
                    print(f"[bfs-audit] late-static-edge: {_scls}.{_smeth}:{_sdesc}")
                _enqueue_method((_scls, _smeth, _sdesc))
                _added = True
            while queue:
                _process(*queue.popleft())
            if not _added:
                break

        # 迟至实现者清扫（doPrivileged 接口级回调边激活实证）：接口分派传播只扫
        # 当时已在 jdk_infos 的实现类，而边界类实现者（GetBooleanAction）多经
        # field_discover 的 stub 通道**晚于**方法 BFS 进闭包——其 run 覆盖与手写体
        # 回调（Boolean.valueOf）都不入队，小闭包测试（TestHashMapOps 经
        # Arrays$LegacyMergeSort.<clinit> → doPrivileged 触达）运行期命中 valueOf
        # 存根。三个通道沉降后按已确认的接口边键窄幅补扫（只补接口边实现者，不重跑
        # 整轮 _propagate_virtual_targets——stub 通道的类型闭环不重开，
        # LocaleSyntaxException E0425 实证全量重跑的扰动）。排序遍历保确定性；
        # _process 带进的新类若也是实现者，外层不动点覆盖。
        while True:
            _added = False
            for _ecls, _emeth, _edesc in sorted(_drained_iface_keys):
                for _c_name in sorted(jdk_infos):
                    _c_ci = jdk_infos[_c_name]
                    if _c_ci is None or _c_ci.is_interface:
                        continue
                    if _ecls in _supertypes(_c_name):
                        if (_c_name, _emeth, _edesc) not in visited_methods:
                            _enqueue_method((_c_name, _emeth, _edesc))
                            _enqueue_upcalls(_c_name, _emeth)
                            _added = True
            while queue:
                _process(*queue.popleft())
            if not _added:
                break

        # 收尾接口 drain（清单第 20 项，c23a4b0 stub 通道宽化）：_enqueue_iface_stub
        # 只把接口写入 field_discover_classes 账本，物化只发生在 stub 通道处理循环
        # ——而该循环消费的是入口快照，循环内 / 父类补全 / 迟至补扫登记的接口从未
        # 物化（登记了但不在 registry：interface_gen 不发 impl I for C、超接口链断开、
        # bare 分派落 Default::default()）。三通道沉降后按账本物化**接口本体**：
        # 不回灌 stub 通道（接口不展开字段类型 / 超类，不重开 4b776b6 类型闭环），
        # 新增类恒 ⊆ 账本且全部 is_interface。_enqueue_iface_stub 已按接口边传递
        # 闭包，单轮即不动点；排序遍历保双种子确定性。
        for _iname in sorted(field_discover_classes):
            if _iname in jdk_infos:
                continue
            _ici = _resolve_class_bytes(_iname)
            if _ici is None or not _ici.is_interface:
                continue
            jdk_infos[_iname] = _ici

    _trace_cls = os.environ.get('JAVA_RTA_BFS_TRACE')
    if _trace_cls:
        for _key in sorted(k for k in visited_methods if k[0] == _trace_cls):
            _path = []
            _step = _key
            while _step is not None and len(_path) < 64:
                _path.append(f"{_step[0]}.{_step[1]}:{_step[2]}")
                _step = enqueued_from.get(_step)
            print("      [bfs-trace] " + "\n          <- ".join(_path))

    # B 组 callchain 五点（fallback-audit §4.1）的警告清单：丢类此前完全无信号，
    # 现逐条指名（封顶 20 行防爆屏；全量计数见 [fallback-audit] 行）
    _fb_warns = fallback_audit.warnings()
    for _w in _fb_warns[:20]:
        print(f"[fallback-audit] 警告: {_w}")
    if len(_fb_warns) > 20:
        print(f"[fallback-audit] 警告: … 其余 {len(_fb_warns) - 20} 条见 "
              f"JAVA_RTA_DEBUG=1 逐触发明细")

    return list(jdk_infos.values()), visited_methods, field_discover_classes


# MH-native（docs/plans/2026-09-26-mh-native.md §二-5）：常量反射引用。
# {类 binary: 成员名集合}——闭包内方法字节码以 `ldc class C` + `ldc String s` 常量组
# 指名的 C 的方法（MethodHandles.Lookup.findStatic/findVirtual、new MemberName(C, "s", …)
# 等按名查找）。这些方法经 BFS 入链翻译，并由 dispatch_gen 为 JDK 类发射按名分派臂
# （句柄 linkTo* / LambdaForm 成员调用经 reflect_invoke 落到它们）。每轮转译重建。
REFLECT_CONSTS: dict[str, set[str]] = {}

# 类常量之后在该窗口内出现的首个字符串常量视为成员名（javac 对
# `find*(C.class, "name", MethodType.methodType(...))` 的发射：两常量相邻）。
_REFLECT_WINDOW = 3


def _scan_reflect_consts(instrs) -> list[tuple[str, str]]:
    """指令序列中的 (类常量, 随后的字符串常量) 对（按名反射查找的常量形态）。"""
    out: list[tuple[str, str]] = []
    seq = [i for i in (instrs or []) if i.comment]
    for k, ins in enumerate(seq):
        c = ins.comment
        if not (ins.opcode.startswith('ldc') and c.startswith('class ')):
            continue
        cls = c[6:].split()[0] if c[6:].split() else ''
        if not cls or cls.startswith("["):
            continue
        for nxt in seq[k + 1:k + 1 + _REFLECT_WINDOW]:
            if nxt.opcode.startswith('ldc') and nxt.comment.startswith('String '):
                name = nxt.comment[len('String '):]
                if name and ' ' not in name:
                    out.append((cls, name))
                break
    return out


def _collect_method_refs(instrs, user_class_names: frozenset[str] = frozenset(),
                         extra_prefixes: tuple[str, ...] = ()) -> tuple:
    """从指令注释中提取方法引用和字段所属类引用（JDK 类 + 用户类）。

    用户类的方法引用也进 method_refs：用户类自身方法虽全部作为种子展开，但以
    用户类为常量池类的**继承方法**调用（如 enum 子类调用基类的 name()/ordinal()）
    必须经 _process 走 JVM 方法解析（JVMS §5.4.3.3）落到最近声明祖先，否则
    基类方法永远不入调用链。

    extra_prefixes：jar 输入模式的库类前缀（如 ('org/junit/',)）——库类与
    JDK 公开包同权进各引用通道（方法 / 字段 / 描述符 / 类字面量 / new）。

    Returns:
        (method_refs, field_classes, member_refs, boundary_refs, new_classes, static_field_refs):
          method_refs   - (cls, method, descriptor) 三元组，用于 BFS 展开
          field_classes - 通过 getstatic/Field 指令发现的类名，只生成存根不展开方法体
          member_refs   - (cls, member) 被触达的成员（方法含内部边界类的方法、字段），
                          用于查询手写实现声明的 Java 回调（见 native_upcalls.py）
          static_field_refs - getstatic/putstatic 的 (cls, field)：类初始化触发点（JVMS §5.5）
    """

    def _add_type_refs(desc: str) -> None:
        """T88：签名类型引用进闭包（type-only）。

        被调方法的参数/返回类型、字段声明类型里的接口（如
        AbstractCollection.iterator() 的 Ljava/util/Iterator;）若不进闭包，
        签名引用的接口类型不会生成 → 名字落到 prelude trait 报 E0782。
        走 field_classes 通道：只生成类型存根，不展开方法体。
        """
        for tcls in _desc_class_refs(desc):
            if _is_boundary_class(tcls):
                field_classes.append(tcls)
            elif tcls.startswith(_JDK_PREFIXES) or tcls.startswith(extra_prefixes):
                field_classes.append(tcls)

    method_refs = []
    field_classes = []
    member_refs = []
    boundary_refs = []   # 以内部边界类为常量池类的方法引用（边界类型上的虚调用目标）
    new_classes = []     # new 指令 / 构造器引用实例化的类
    static_field_refs = []
    for instr in (instrs or []):
        c = instr.comment
        if not c:
            continue
        if c.startswith(('Method ', 'InterfaceMethod ')):
            # "Method java/io/PrintStream.println:(Ljava/lang/String;)V"
            rest = c.split(' ', 1)[1]
            dot = rest.find('.')
            colon = rest.find(':', dot)
            if dot > 0 and colon > dot:
                cls = rest[:dot]
                meth = rest[dot+1:colon]
                desc = rest[colon+1:]
                if '[' not in cls:
                    member_refs.append((cls, meth))
                # stub-only 优先检查（java/security/ 等是 java/ 的子前缀，必须先匹配）
                if _is_boundary_class(cls) and '[' not in cls:
                    field_classes.append(cls)
                    boundary_refs.append((cls, meth, desc))
                elif (cls.startswith(_JDK_PREFIXES) or cls in user_class_names
                      or cls.startswith(extra_prefixes)) and '[' not in cls:
                    method_refs.append((cls, meth, desc))
                _add_type_refs(desc)
        elif c.startswith('InvokeDynamic '):
            # "InvokeDynamic samName:dynDesc impl:Cls.method:implDesc samtype:samDesc"
            # lambda / 方法引用的实现方法由闭包直接调用，属于调用链的一部分
            for tok in c.split(' ')[2:]:
                if tok.startswith('samtype:'):
                    # SAM 接口的函数式描述符类型（缺口 D）：通常已被 impl: 令牌的
                    # 方法描述符通道覆盖，此处防御性收集
                    _add_type_refs(tok[len('samtype:'):])
                    continue
                if not tok.startswith('impl:'):
                    continue
                rest = tok[5:]
                colon = rest.find(':')
                dot = rest.rfind('.', 0, colon) if colon > 0 else -1
                if dot > 0:
                    cls = rest[:dot]
                    meth = rest[dot+1:colon]
                    desc = rest[colon+1:]
                    if '[' not in cls:
                        member_refs.append((cls, meth))
                    if _is_boundary_class(cls) and '[' not in cls:
                        field_classes.append(cls)
                    elif (cls.startswith(_JDK_PREFIXES)
                          or cls.startswith(extra_prefixes)) and '[' not in cls:
                        method_refs.append((cls, meth, desc))
                        if meth == '<init>':
                            new_classes.append(cls)   # 构造器引用 X::new
                    _add_type_refs(desc)
        elif c.startswith('Field '):
            # "Field java/nio/charset/CodingErrorAction.REPLACE:Ljava/nio/charset/CodingErrorAction;"
            # getstatic/putstatic/getfield/putfield - 只发现声明类，不展开其方法体
            rest = c[6:]  # 去掉 "Field "
            dot = rest.find('.')
            if dot > 0:
                cls = rest[:dot]
                if (cls.startswith(_JDK_PREFIXES) or cls.startswith(_JDK_STUB_ONLY_PREFIXES)
                        or cls.startswith(extra_prefixes)) and '[' not in cls:
                    field_classes.append(cls)
                    if instr.opcode in ('getstatic', 'putstatic'):
                        _fend = rest.find(':', dot)
                        static_field_refs.append((cls, rest[dot+1:_fend if _fend > dot else len(rest)]))
                colon = rest.find(':', dot)
                if colon > dot:
                    if '[' not in cls:
                        member_refs.append((cls, rest[dot+1:colon]))
                    _add_type_refs(rest[colon+1:])
        elif c.startswith('class '):
            # ldc / ldc_w 装载的类字面量（`X.class` / `X[].class`）：栈上是
            # java/lang/Class 实例，该类型必须进闭包（目标类本身不需要转译 ——
            # Class 对象只承载 binary name，见 Class::for_class）。
            field_classes.append(_CLASS_CLASS)
            # 字面量目标类本身以 stub 进入闭包：Class.isAssignableFrom 需要它的
            # 父类链 + 接口闭包（registry 按解析出的 ClassInfo 提供层次）。
            # 数组字面量（`[Ljava/lang/String;`）无层次可言，跳过。
            _lit_cls = c[6:].split()[0] if c[6:].split() else ''
            if _lit_cls and not _lit_cls.startswith('[') and '/' in _lit_cls:
                field_classes.append(_lit_cls)
        elif c.startswith('['):
            # 数组形态的类操作数注释（multianewarray / anewarray / checkcast /
            # instanceof 的 "[[Ljava/lang/String;" 裸描述符）：元素类型进闭包
            # （type-only）。此前所有分支都要求 '[' not in c，数组元素类型
            # 完全不进闭包（trace 完整版发现的缺口 C）。_desc_class_refs 的
            # 正则天然吃数组描述符，只提取 L...; 内层类名。
            _add_type_refs(c)
        elif c.startswith(_JDK_PREFIXES + _JDK_STUB_ONLY_PREFIXES + tuple(extra_prefixes)) \
                and '[' not in c and _is_boundary_class(c.split()[0]):
            # stub-only 内部类的 new/checkcast 指令 → 仅生成存根，不展开方法体
            cls = c.split()[0]
            field_classes.append(cls)
        elif c.startswith(_JDK_PREFIXES + tuple(extra_prefixes)) and '[' not in c:
            # new / checkcast / instanceof / anewarray: comment = class binary name
            cls = c.split()[0]
            if instr.opcode == 'new':
                # 只有 new 产生该类的运行期实例（RTA 的实例化集合）；
                # 构造器本身由紧随其后的 invokespecial <init> 方法引用入队
                new_classes.append(cls)
            field_classes.append(cls)
        elif instr.opcode == 'new' and '[' not in c and c.split()[0] in user_class_names:
            # 用户类的 new（如 enum 常量在 <clinit> 中构造）：进 RTA 实例化集合，
            # 虚调用目标才能传播到该类的覆盖/继承方法。用户类不走 field_classes
            # 通道（不进 jdk_infos，user crate 整体生成）。
            new_classes.append(c.split()[0])
    return method_refs, field_classes, member_refs, boundary_refs, new_classes, static_field_refs
