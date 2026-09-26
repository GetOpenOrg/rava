"""[equiv-audit] 近似/条件等价发射点计数。

规格来源：docs/compatibility.md §4「告警契约 / 告警目录（seed）」。这是该契约的
最小可执行形态（§4.4）：codegen 在可检测的近/条件等价发射点计数，
scripts/main.py 聚合输出 ``[equiv-audit]`` 行，scripts/run_tests.py 顺序/并行
两模式解析并在结尾汇总（完全对齐已落地的 [readability-audit] 模式）。

口径（每个 ID 一句话）——计数是「该形态的发射点数」，不是缺陷数：

- 目标不是全 0（近似等价允许存在），而是**可观测**：runner 汇总后可经
  ``--deny equiv`` / ``--deny equiv::<id>`` 升级为整体失败（对齐 rustc lint 模型，
  compatibility.md §4.3）。
- 计数按「转译进程」累计（与 cfg-audit 的 AuditStats 同生命周期）：main.py 每次
  运行是一个新进程，天然按测试隔离；batch 模式下为本次转译全部文件之和。

各 ID 的口径与埋点位置：

===============  =============================================================
ID               口径（触发条件 / 埋点）
===============  =============================================================
identity-hash    S-6：默认 hashCode / System.identityHashCode 路径。
                 埋点：invoke_virtual.py 的三处默认路径（类型变量接收者直调、
                 bare Object 直调、整条祖先链未声明 → 根 vtable 路由）+
                 invoke.py 的 invokestatic identityHashCode 调用点。
intern-identity  S-6：``String.intern`` 调用点（intern 后 ``==`` 的同一性是
                 近似等价；字面量 ldc 的 intern 语义在 runtime 侧，无独立
                 codegen 发射形态，不在本计数内）。埋点：invoke_virtual.py
                 按 mname == 'intern' 匹配（方法名来自字节码常量池）。
null-array       S-2.1：数组 null 表示语义的作用面——全部 xaload/xastore/
                 arraylength 发射点（Java 语义里这些指令在 null 数组引用上抛
                 NPE，JVMS §6.5；当前由 JArray Repr::Null 承载，行为近似）。
                 埋点：instr/sim/arrays.py。数组**创建**指令不计（归 neg-array）。
boxed-null       S-3：装箱 null 路径——null 字面量（aconst_null 的
                 Object::default()）流入具体类型槽位时被替换为
                 Default::default() 零值的发射点。埋点：instr/coerce.py
                 _coerce_from_null 命中分支（参数 / 字段存储两条消费路径共用）。
class-literal    S-5：类字面量（ldc ``class `` → Class::for_class）与
                 getClass 调用点（同一性近似：每次构造新 Class 对象）。
                 埋点：instr/sim/consts.py 的 ldc class 分支 +
                 invoke_virtual.py 按 mname == 'getClass' 匹配（含数组
                 getClass 的 Class::for_class 早路径）。
record-hash      S-7：record hashCode 生成——已按 ObjectMethods 31 多项式等价实现
                 （emitter/class_writer.py _patch_record_method_blocks），不再埋点，
                 计数恒 0；ID 保留以兼容报告列。
neg-array        S-8：newarray/anewarray/multianewarray 发射点中潜在负长度
                 路径——长度是运行期值，codegen 无法静态判定，**当前全部计数**；
                 S-8 修复（运行期抛 NegativeArraySizeException）后本计数转为
                 「创建点总量」观测口径，条件化（常量长度折叠）另行立项。
                 埋点：instr/sim/arrays.py 三个创建分支。
field-npe        S-9：Object 接收者的 getfield/putfield——null 接收者字段
                 访问不抛 NPE 的路径（类型化接收者的访问器缺口同属 S-9，
                 但按本任务口径只计 Object 接收者）。埋点：instr/sim/fields.py。
class-init       S-10：类初始化触发点缺口——(a) 手写静态 native 的调用点
                 （*_impl.rs 实现不在 java_class! 宏块内，入口无
                 ``Self::__class_init()?;`` 注入）；(b) invokeinterface 调用
                 接口 default 方法（带 default 方法的接口自身初始化未触发）。
                 埋点：instr/invoke.py _gen_invokestatic（目标 is_native）+
                 instr/sim/methods.py（invokeinterface → 接口非抽象方法）。
===============  =============================================================

本批不埋的 ID（告警目录 seed 共 11 个）：

- ``stacktrace``（fillInStackTrace / 栈帧）：无独立 codegen 发射点（行为在
  runtime 的异常构造路径），随 S 候选条目立项后另行接入。
- ``monitor-mt``（monitorenter 多线程互斥，条件等价，S-11）：**已补埋
  （2026-09-21，S-20 monitor.rs 落地后）**——埋点两处：instr/sim/dynamic.py
  的 monitorenter 指令发射分支 + method/codegen.py 的 ACC_SYNCHRONIZED
  方法前导 MonitorGuard（实例锁/静态锁各计 1）。
"""

from __future__ import annotations

# 发射口径的 ID 全集（按 compatibility.md §4 目录顺序，扣除 stacktrace）
IDS: tuple[str, ...] = (
    'identity-hash', 'intern-identity', 'null-array', 'boxed-null',
    'class-literal', 'record-hash', 'neg-array', 'field-npe', 'class-init',
    'monitor-mt',
)

_counts: dict[str, int] = {i: 0 for i in IDS}


def record(equiv_id: str, count: int = 1) -> None:
    """在某个等价发射形态的发射点计数 +count（只读计数，不影响发射内容）。"""
    _counts[equiv_id] = _counts.get(equiv_id, 0) + count


def summary() -> str:
    """[equiv-audit] 行：只列非零项（readability-audit 风格）；全零输出 none。"""
    items = ' '.join(f"{k}={v}" for k, v in _counts.items() if v)
    return f"[equiv-audit] {items or 'none'}"


def snapshot() -> dict[str, int]:
    """当前计数的拷贝（runner 侧聚合用）。"""
    return dict(_counts)


def reset() -> None:
    """清零（与 cfg AuditStats.reset 同约定，供复用进程的场景）。"""
    for k in _counts:
        _counts[k] = 0
