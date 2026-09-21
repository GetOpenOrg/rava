# 任务管理（当前活跃）

> 创建：2026-09-16（取代已归档的 `docs/tasks-history.md`）
> 定位：**只保留开放的问题与任务**。完成即关闭删除，不累积历史。
> 历史任务（T01-T81 全记录）见 `docs/tasks-history.md`；R5 轮集成后的完整遗留清单见 `docs/plans/2026-09-19-remaining-issues.md`。

---

## ⚠️ 执行约束（最高优先级，不可绕过）

1. **架构问题优先**：先做架构改造，测试错误待架构完成后自然消解，禁止因为测试失败而中断架构工作转去修 Bug。
2. **架构完成前禁止全量测试**：定向验证（红线集 + 金丝雀）除外，全量 run_tests.py 只在架构节点合入后由主会话统一执行。
3. **子代理串行执行**：一次只运行一个子代理（用户指定，内存约束）；前一个完成并合入验证后再启动下一个。
4. **任务执行顺序**：陈旧树筛 + run 族定向复验 → A-8 / S-20 快速收益 → A-1 剩余主体（类 vtable 去形参化）→ downcast 链清零。

---

**关联追踪文档**：
- `docs/plans/2026-09-19-remaining-issues.md` — **当前主线**：A/S/G/P/V/R 六类遗留问题全清单（架构缺口/JVM 语义/生成器质量/原则违规/验证/仓库）
- `docs/plans/java-rust-translation-reference.md` — 翻译对照（宏家族 §16）
- `docs/tasks-history.md` — T01-T81 历史全记录

**基线（2026-09-21）**：
- **166 全量（用户 Ubuntu，JDK21）**：87 PASS / 79 FAIL；**跑批树落后 main，混有陈旧污染**——fcb04ce 干净树复核 18 例，TestStringBuilder / TestOptionalFull / TestIncDec / TestShortCircuit 已 PASS（假象），真实失败面待复验收敛。归类全记录：`2026-09-21-e2e-baseline-classification.md`（新立项 A-8 / S-19 / S-20；S-8 / S-9 实测升级）。
- 红线 19/19 全绿（a20a31f 实测）；TestStringBuilder 编译 0 错误、输出 22 行与 Java 逐字一致（fcb04ce 复核 PASS，含金丝雀）。
- **streams 三测全绿（2026-09-21 午后，P-3 合入后主会话 --clean 6/6）**：TestStreamBasic / TestStreamAdvanced / TestStreamCollectors + TestStringBuilderOps 全 PASS，输出逐字一致。
- 架构里程碑链：vtable 双指针多态 → CFG 支配树结构化（含 try/catch `java_try!`、`<clinit>` 语义、异常对象）→ println 字节码化 → K-6 槽位签名模型 → 擦除运行时身份阶段 1（非泛型 `I__VTable` + `__interface`）→ S-18 接口分派 BFS（fcb04ce）→ P-3 边界补全（7b64afa，join + DoubleToDecimal）。
- 教训：**全量跑批必须 tee 落盘**（本轮 224 分钟 stdout-only，38 个 run 族无 stderr 只能二次定向）。

---

## 🔴 活跃任务（当前队列）

| 任务 | 状态 | 目标 / 说明 |
|------|------|------------|
| **陈旧树筛 + run 族定向复验** | 等用户拉平后执行 | 用户侧 pull 到 fa7cf10 后按归类文档 §4.2 清单 `--filter` 定向复验（30 run + 待复验 output），把 79 失败收敛到真实面 |
| **A-8 同文件辅助类未进闭包** | 166 归类新增，最大单一杠杆（15 用例） | 编译族 15 例统一 E0433/E0425；证据：scratch 内辅助类文件未生成 |
| **S-20 `Object.wait/notify/notifyAll`** | 166 归类新增（4 用例） | runtime 补三方法接 InternalLock；与 monitorenter 真实化联动 |
| **数组视图 coerce 族** | 166 归类新增 | 实参位置 `Object`→`JArray<T>` 视图转换未发射（TestArrayCopy E0308 实证）；JDK25 批 8 例同型 E0308 待判同根因 |
| **downcast 链移除（859 处）** | A-1 后主推 | A-2 可读层清零主杠杆：`from_any`/`downcast_ref`/`.downcast::<T>()` 在方法体清零；纯 Python 侧 + 宏封装；前置全就绪 |
| **A-1 剩余主体：类 vtable 去形参化** | 擦除阶段 2，多日级重构 | 非泛型 `X__inner` + 类型化视图：可变泛型类跨实例化互转、子类对象跨实例化重建；同时解锁 downcast 清零的存储层基础 |

## P1 · 功能缺口

| 任务 | 来源 | 说明 |
|------|------|------|
| record `hashCode` 恒为 `Ok(0)` | R5 遗留 | `toString`/`equals` 已真实化，`hashCode` 未实现（S-7） |
| `monitorenter`/`monitorexit` 为 no-op | 同步系列（原 T80） | 当前 pop 忽略——单线程正确；多线程需接 `InternalLock`（实现已就绪，`parking_lot::ReentrantMutex`）；与 S-20（wait/notify 建模）同批推进 |
| TestTryShape 缺期望文件（V-2） | R5-D 新增测试 | ~~`tests/expected/TestTryShape.txt` 未入库~~ **已过期**：166 全量中 TestTryShape PASS，expected 已入库，可关闭 |
| 手写静态 native 不触发类初始化；带 default 方法的接口自身不初始化 | S-10 剩余 | 见 remaining-issues S-10 |

## P2 · 翻译质量

| 任务 | 来源 | 说明 |
|------|------|------|
| mut 标注递归覆盖 | T54 | 嵌套块 |
| 括号优化 | T60 | 表达式优先级 |
| 布尔压缩 | T70 | `if c {1i32} else {0i32}` → bool（R5 已修部分：lxor/比较产物） |
| 语义桩计数 | T72 | 指令级统一标记未做 |
| 类级并行解析 | T65 | 测试级并行已实现（run_tests.py），类级未做 |
| 生成输出非确定性 | R5-D 发现 | 提升 `let` 的顺序随 set 迭代序变化，影响 diff 对比 |
| IR 结构化收敛 | T05/T06/T07/T50/T58/T61/T67 | RawExpr/RawStmt 消除，架构级 |

## P3 · 长期重构（不阻塞主线）

> 详细设计见 `docs/plans/2026-09-17-java-rust-type-1to1.md` 与 `docs/plans/2026-09-18-erased-runtime-identity.md`。

| 任务 | 启动条件 | 说明 |
|------|---------|------|
| **T-1 bounds 进宏** | 无依赖 | `impl ArrayList<E>` 的 Rust bounds 由 `java_class!` 从 `generic_signature` 注入，codegen 只写裸参数名 |
| **T-2 接口泛型进类型位置**（擦除阶段 1 已落地：非泛型 `I__VTable` + 载体 struct + `ObjectVTable::__interface`） | T-1 完成后 | 剩余：接口名在类型位置仍擦除为 `Object`（调用点读 `Into::<Iterator<E>>::into(..).hasNext()` 而非 `it.hasNext()`）；lambda 对象实现 `I__VTable`；抽象类/枚举/手写类的 `__interface` 覆盖 |
| **T-4 包装类走生成** | T-3 完成后 | `Integer`/`Long` 等从字节码生成（String 已走生成） |
| **R-2 `Deref<Target=Parent>` 替换 From 继承链** | R-1 ✅ 已落地 | 删除 `class_writer.py` 的 `From<Child> for Parent` 链生成；Rust deref coercion 自动处理多层向上引用 |
| **R-3 `#[derive(Debug)]` 替换宏生成 Debug** | 无依赖 | 前提：生成类字段类型均满足 `Debug` |
| **M-3 方法签名类型决策进宏** | IR 结构化完成后 | 宏从 `#[descriptor]`/`#[generic_signature]` 自行做 JVM→Rust 类型选择，Python 只传原始字符串 |

---

## 维护规则

1. 完成一项 → 从本表删除，证据（commit / 测试结果）记入对应追踪文档
2. 新发现问题 → 入表并标注来源（e2e 普查 / 代码审查 / 实验发现）
3. 每 e2e 全量运行后刷新基线数字
4. 本文档不记历史——需要查完成记录去历史文档或 git log
