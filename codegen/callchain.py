"""调用链的发射期共享状态：闭包清单判定、本轮闭包种子全局与编译前预检。

闭包（发现）的唯一来源是 Rust 闭包分析器 `rava closure` 输出的 closure.json
（codegen/closure_input.py 读取并填充本模块的种子全局）。本模块只保留发射层仍需要的部分：
  - 清单判定：`_is_vm_boundary_class`（closure.toml [vm_boundary] 逐类 VM 契约清单及其
    translate_nested 放行），emitter 与 closure_report 经同一入口求值；
  - 种子全局：反射面、注解枚举、模块资源（closure_input 填充，emitter 消费）；
  - 预检：生成产物中调用链上的 panic 存根 / 缺失 native（`precheck_from_tree`）。
"""

import os
import re

from .runtime_manifest import vm_boundary_classes, vm_boundary_translate_nested


# VM 契约边界类（closure.toml [vm_boundary] classes）：由 JVM 自身引导 / 承载 VM 设施（类加载、
# 模块系统、Unsafe 等）的类，按方法划分。清单在 runtime/（手写层真源）维护，生成器不出现 JDK 类名。
_VM_BOUNDARY_CLASSES: frozenset[str] = frozenset(vm_boundary_classes())

# VM 契约边界类中按字节码翻译的嵌套类（[vm_boundary] translate_nested：纯 Java 辅助类）
_TRANSLATE_NESTED: tuple[str, ...] = tuple(vm_boundary_translate_nested())


def _is_translate_nested(cls: str) -> bool:
    """类是否为 VM 契约边界类中放行翻译的嵌套类（条目本身及其嵌套类）。"""
    return any(cls == r or cls.startswith(r + '$') for r in _TRANSLATE_NESTED)


# 本轮经注解种子入闭包的枚举类（binary name，已排序）：emitter 在生成 main 中登记类初始化
# 钩子（Enum.valueOf → 常量目录前强制初始化，FS-R R4b）。closure_input 每轮重填。
ANNOTATION_ENUM_SEEDS: list[str] = []

# 模块资源（seeds.toml [module_resources]）：{jmod 内相对路径: 字节}，从本轮所用 JDK 的 jmod 提取，
# emitter 写入 scratch 并生成 include_bytes! 嵌入表（数据与所用 JDK 版本同源）
MODULE_RESOURCES: dict = {}

# MH-native（docs/plans/2026-09-26-mh-native.md §二-5）：常量反射引用。
# {类 binary: 成员名集合}——闭包分析器按常量数据流解析出的按名反射目标（Lookup.findStatic /
# findVirtual、new MemberName(C, "s", …) 等）；dispatch_gen 为 JDK 类发射按名分派臂
# （句柄 linkTo* / LambdaForm 成员调用经 reflect_invoke 落到它们）。每轮转译重建。
REFLECT_CONSTS: dict[str, set[str]] = {}
# 调用链上出现的全部标识符形 ldc 字符串常量：按名反射 / Unsafe 静态字段访问的字段名候选
#（ClassSpecializer 的 sdFieldName "BMH_SPECIES" 经构造实参传入、再对运行期 species 类
# getDeclaredField——无类常量配对，只能按「字符串常量 ∩ 生成类静态字段名」发射字段臂）
REFLECT_FIELD_NAMES: set[str] = set()
# 全量反射面的类（方法臂 + 字段臂全发射）：运行时镜像独有、按类名加载的 jlink 预生成类
#（BoundMethodHandle 物种类：构造器 / make / argL* getter 经 Lookup 按名解析）
REFLECT_ALL_MEMBERS: set[str] = set()


def _is_vm_boundary_class(cls: str) -> bool:
    """VM 契约边界类（closure.toml [vm_boundary]，含其嵌套类；translate_nested 放行的嵌套类除外）。

    按方法划分（与 Rust 闭包分析器同口径）：native / VM 内建 / 共置手写体提供的方法取手写，
    其余被调用到的方法按字节码翻译；`<clinit>` 不翻译（类的静态状态由 VM / 手写层承载——
    HotSpot 中这些类由 VM 引导初始化，其 `<clinit>` 会展开安全管理器 / 模块层 / 类加载子系统）。"""
    outer = cls.split('$', 1)[0]
    return outer in _VM_BOUNDARY_CLASSES and not _is_translate_nested(cls)


# 编译前预检（转译期可判定的必然存根）：代码生成后由 precheck_from_tree 填充，scripts/main.py 打印
# [precheck] 并据此决定是否跳过 cargo。两类：
#   native_missing  调用链上的 native 方法，生成体为 panic!("native: …")（缺手写实现）
#   boundary_stub   调用链上 / 触达的方法，生成体为 panic!("stub: …")（缺手写或未补译）
# 链事实 PRECHECK_CHAIN 由 closure_input 按 closure.json 的方法节点填充。
PRECHECK: dict[str, list[str]] = {'native_missing': [], 'boundary_stub': []}
PRECHECK_CHAIN: dict[str, set] = {'visited': set(), 'touched': set()}


_PANIC_STUB_RE = re.compile(r'panic!\("(stub|native): ([^"]+)"\)')


def precheck_from_tree(runtime_src: str) -> None:
    """扫描生成的 java_runtime 源码：方法体为 `panic!("stub: …")` / `panic!("native: …")`
    且在调用链上（已入链方法，或触达的边界成员）者即编译前可知的缺口，写入 PRECHECK。"""
    visited, touched = PRECHECK_CHAIN['visited'], PRECHECK_CHAIN['touched']
    natives, stubs = set(), set()
    for _dir, _subdirs, _files in os.walk(runtime_src):
        for _fn in _files:
            if not _fn.endswith('.rs'):
                continue
            try:
                _text = open(os.path.join(_dir, _fn), encoding='utf-8').read()
            except OSError:
                continue
            if 'rava_macros::java_class' not in _text or 'panic!("' not in _text:
                continue
            for _kind, _sig in _PANIC_STUB_RE.findall(_text):
                _name = _sig.split(':', 1)[0]
                if _sig in visited or _name in touched:
                    (natives if _kind == 'native' else stubs).add(_sig)
    PRECHECK['native_missing'] = sorted(natives)
    PRECHECK['boundary_stub'] = sorted(stubs)


def print_precheck(limit: int = 40) -> None:
    """[precheck] 汇总行 + 逐条明细（每类封顶 limit 行）。"""
    _n, _b = PRECHECK['native_missing'], PRECHECK['boundary_stub']
    print(f"[precheck] native-missing={len(_n)} boundary-stub={len(_b)}")
    for _kind, _items in (('native-missing', _n), ('boundary-stub', _b)):
        for _it in _items[:limit]:
            print(f"[precheck] {_kind}: {_it}")
        if len(_items) > limit:
            print(f"[precheck] {_kind}: … 其余 {len(_items) - limit} 条")
