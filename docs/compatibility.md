# 兼容性与等价承诺规范

> 创建：2026-09-21。
> 定位：**规范级文档**——本项目对「Java 字节码 → Rust 源码」转译产物的行为承诺。与 `docs/rules.md`（生成器架构规则）平行：rules.md 约束**怎么生成**，本文约束**生成物承诺什么**。
> 评估来源与项目分歧记录：`docs/plans/2026-09-21-ruva-compat-reference.md`；语义缺口的工程跟踪：`docs/plans/2026-09-19-remaining-issues.md`（本文引用其条目编号，不复制内容）。
> 状态：**初稿**。§3 特性表由 remaining-issues 标注表 + 166 基线归类（2026-09-21）导出，随实测刷新。

---

## 1. 承诺范围

- **输入**：JDK 21 / JDK 25 javac 产出的 Java 字节码（`tests/e2e/` 166 用例语料为代表；新特性按字节码事实增量支持）。
- **承诺对象**：`java.base` 范围内的语言语义与标准库 API。项目哲学是**翻译 JDK 自身字节码**，因此承诺边界按能力呈现，而非白名单：
  - 支持的语义 → 按 §2 等级承诺；
  - 调用链内未翻译/未补全的方法 → **可见 stub**（运行期 `panic!("stub: ...")`，编译期 `stub_fallback` 审计计数）；
  - 近似/条件等价 → §5 告警呈现（告警基建待立项，当前以 §3 表格 + 归类文档承载）。
- **不在承诺范围**（不可等价，见 §2）：运行时动态加载/生成类（自定义 `ClassLoader`、ASM/ByteBuddy/CGLIB、`Proxy` 动态代理完全体、Java Agent）；第三方 jar 的自动全量转译。框架适配属远期扩展层（`ruva-compat-reference.md` §四），不在本规范。

## 2. 六级等价定义（含本项目锚点）

从强到弱。每级绑定**本项目**的实现机制，不采用外部模型的对应物（分歧评估见 compat-reference §一）。

| 等级 | 定义 | 本项目锚点 |
|---|---|---|
| **规格等价** | Rust 原语满足 Java 规格的全部约束，直接映射 | `wrapping_*` 算术、IEEE 754 `f32/f64`、`JArray`、显式窄化 cast、RAII |
| **行为等价** | 外部可观测行为一致（副作用、异常、执行顺序） | `JvmError`/真实 Throwable 异常链、CCE/ArrayStoreException、`java_try!`、try-finally、vtable 双指针虚分派、`InternalLock`（ReentrantMutex） |
| **语义等价** | 输入输出与异常一致，内部机制允许不同 | 集合为 Rust 实现、稳定排序、String 走字节码翻译保留 compact strings |
| **条件等价** | 等价性取决于运行时条件，**分档承诺** | `monitorenter`：单线程 = 行为等价；多线程互斥待 InternalLock 接入（S-11） |
| **近似等价** | 已知、可列举的偏差场景；**不允许静默**（告警契约 §5） | identity hash、null 数组、`getClass` 终态、record `hashCode`、栈回溯（详见 §3；intern 同一性已修出列） |
| **不可等价** | 无法在此架构下承诺；转译期报错或可见 stub | ClassLoader 动态加载、Agent、运行时生成类 |

**传递性规则**：类的有效等价等级 = min(自身声明等级, 全部依赖成员的等级)。用户类调用到近似等价的 JDK 方法时整体降级，不得静默宣称更高等级。终态：codegen 输出每类有效等价等级报告（待立项，规格来源即本节）。

## 3. 特性 → 等级表（e2e 归类判据）

> 用途：e2e 失败测试先查本表，命中 → 归类 `equiv-boundary`（边界偏差），不按 bug 处理；未命中 → 真缺陷，走 remaining-issues 立项。归类规程见 `docs/plans/2026-09-21-e2e-baseline-classification.md`。
> 「实测状态」为 2026-09-21 基线的最近似证据；「未实测」= 166 语料无覆盖，探针用例随对应条目修复排队。

| 特性 / API | 等级 | 实测状态（2026-09-21） | 条目 |
|---|---|---|---|
| 整数 wrapping / 位运算 / 窄化截断 | 规格等价 | TestArithmetic / TestOverflow / TestIntOverflow / TestUnsignedInt PASS | — |
| 浮点算术（IEEE 754） | 规格等价 | TestDouble PASS；**NaN 判定、`Math.rint`、小数值科学计数格式当前破坏规格——属缺陷非边界** | 待立项（归类文档 §新发现） |
| 异常链 / CCE / ASE / NPE（引用接收者） | 行为等价 | TestExceptions / TestInheritance / TestCasting PASS | — |
| `monitorenter` / `monitorexit` 互斥 | 条件等价 | 单线程/协作调度 PASS（TestSynchronized 全绿）；真并发待对象模型 Send/Sync 化 | S-11（monitor.rs 已真实化） |
| **`Thread.start/join/sleep/isAlive` + wait/notify 协作调度** | 条件等价 | **确定性输出程序=语义等价**（TestSynchronized/TestThreadJoin/TestWaitNotify 全绿——`8eca47b`：start0 就绪队列登记、join/wait/sleep 嵌套泵推进）。边界：依赖真实 interleaving 的输出不可达；限时 wait 无到点自醒（JLS §17.3 虚假唤醒语义）；无通知源的无限 wait 忙转；InterruptedException 未实现（语料无中断等待）；sleep 不驻留 | S-11 线程档位（真并发待 Send/Sync 化） |
| **虚拟线程 / 限时等待（`Thread.ofVirtual`、`FutureTask.get(timeout)`、JUnit `@Test(timeout=)`）** | 条件等价 | 线程模型方案 A：虚拟线程 = 模拟平台线程（同一协作调度器，Continuation 不建模）——TestVirtualThread JDK21/JDK25 PASS；JUnit M4 golden OK（FailOnTimeout：ThreadGroup + 线程 + CountDownLatch + FutureTask.get 限时）。**边界**：无抢占——被等待线程运行至完成，**真实超时不可达**（测试体死循环 → 挂起；sleep 超过限时 → 判为通过，JVM 判超时）；`test timed out after N milliseconds` 不产生 | 真抢占待对象模型 Send/Sync 化 |
| `Object.wait` / `notify` / `notifyAll` | 行为等价（目标） | **未实现**（`Object` 无该方法，4 用例 E0599） | 待立项（runtime 小改） |
| identity hash / 默认 `Object.hashCode` | **语义等价**（2026-09-25） | **已实测**（TestIdentityHash PASS：稳定性 / null=0 / 未覆盖类 hashCode==identityHashCode / 覆盖不影响 identity / HashSet·HashMap·IdentityHashMap 语义）。修复：ObjectVTable 默认 hashCode 原恒 0，改为实例地址，与 System.identityHashCode 同源。迭代序：未覆盖 hashCode 的对象在哈希容器中的迭代序随地址变化（JVM 同样不确定） | ~~S-6~~ 已修 |
| `Object.clone`（浅拷贝 / Cloneable 契约 / 数组 clone / 覆盖体） | **语义等价**（2026-09-25） | TestObjectClone PASS：浅拷贝、非 Cloneable 抛 CloneNotSupportedException、数组 clone 五形态、类自身 clone 覆盖体（深拷贝）、经基类视角虚分派、覆盖体内 `super.clone()` 按运行时类浅拷贝 | ~~C-1~~ 已修 |
| `finalize` | 近似等价 | TestFinalizeProbe PASS：覆盖 + 显式调用、super.finalize 链、虚分派、异常传播、Object.finalize 空体。**不建模**：GC 触发的终结调用（无 GC）；静态链未覆盖、运行时子类覆盖时 `this.finalize()` 落 Object 空体 | — |
| 弱 / 软 / 虚引用 | 近似等价 | TestReferenceTypes PASS：强可达期间 get / clear / refersTo / enqueue / 队列 poll / PhantomReference.get 恒 null / WeakHashMap。**不建模**：GC 回收与自动入队（无 GC，referent 仅经 clear 置空） | — |
| `String.intern` 同一性（`==`） | 语义等价 | **已实测**（TestStringCompare PASS：interned==lit=true、lit==heap=false——runtime 全局驻留表，字面量路径 `From<&str>` 与 `intern()` 同表取规范实例；拼接走 `from_owned` 不入表）。TestStringEdge 同机制行待复跑（invoke 域预存编译断，与本修无关） | ~~S-6 intern~~ 已修（identity-hash 同条目另一半仍未实测） |
| null 数组表示 / 数组 NPE | 近似等价 | 未实测 | S-2.1 |
| `getClass` / 类字面量同一性 | 近似等价（完整终态后为语义等价） | 基础路径 PASS（TestClassLiteral）；**数组 getClass 已实测**（TestArrayCovariance PASS：`arr.getClass()` 动态分派——JArray 协变视图委托源数组取运行时元素类型，`getSimpleName` 按 JDK 数组形态命名（`String[]`/`int[][]`）；Class 一律经 for_class 缓存，`== X[].class` 身份成立）；非数组类字面量同一性未实测 | S-5（数组臂已修） |
| record `hashCode`（31 多项式） | 近似等价 | 未实测（TestRecord 基础路径 PASS） | S-7 |
| 栈回溯 / stack frames | 近似等价（档位：帧数真实、内容近似） | **已实测**（S-19 #5，TestCustomException PASS；探针首四帧与 JDK 21 同名同序，帧内容为 Rust 栈符号属声明边界） | ~~S-19 #5~~ 已修 |
| 数组负长度 `NegativeArraySizeException` | 近似等价 | 未实测 | S-8 |
| null 接收者 `getfield`/`putfield` NPE | 近似等价 | 未实测 | S-9 |
| 类初始化触发点全集（JVMS §5.5） | 近似等价 | 部分（手写 static native、接口自身初始化未触发） | S-10 |
| 泛型擦除运行时身份 | 近似等价（A-1 完成后升语义等价） | 通配符/跨实例化 CCE 场景已知 | A-1 |
| ClassLoader / Agent / 运行时生成类 | 不可等价 | 不在语料 | — |

## 4. 告警契约（设计态，待 `[equiv-audit]` 立项）

1. **近似等价、条件等价触发降级场景，不允许静默编译通过**。结构化告警格式：

   ```
   [warning] NearApprox: <特性名称> at <源文件位置>
     Behavior: <偏差描述>
     Safe when: <安全使用条件>
     Risk: <可能引发的问题>
     See: docs/compatibility.md#<锚点>
   ```

2. **废弃 API 告警为独立正交类别**（`Deprecated:` 前缀），可与等价告警同时出现在同一特性上。
3. **`--deny` 分级升级**：默认告警放行；`--deny near-approx` 全局升级为错误；`--deny near-approx::<id>` 细粒度升级（对齐 rustc lint 模型）。
4. **最小可执行形态**：`[equiv-audit]` 审计行——codegen 在可检测的近/条件等价发射点（monitorenter 生成、identity hash 路径、null 数组操作等）输出计数，`run_tests.py` 汇总（完全仿照已落地的 `[readability-audit]`：`scripts/main.py` 发射、runner 顺序/并行双模式解析）。**已立项落地（2026-09-21，`codegen/equiv_audit.py`）**：
   - 发射口径 10 个 ID（下表扣除 `stacktrace`——无 codegen 发射点）：`main.py` 转译后输出 `[equiv-audit] <id>=<n> …`（只列非零项），各 ID 的口径与埋点位置见 `codegen/equiv_audit.py` 模块注释。计数是**发射点数而非缺陷数**——目标是可观测，不是全 0。`monitor-mt` 已随 S-20 监视器真实化（`monitor.rs`，2026-09-21）补埋（monitorenter 指令发射 + ACC_SYNCHRONIZED 方法前导两处）。
   - runner（`scripts/run_tests.py`）两模式解析并在结尾输出 `[equiv]` 汇总（各 ID 总计 + 非零测试名单）；run 族失败自动直跑二进制抓 stderr 分类子族（`stub-hit` / `native-hit` / `s8-crash`（capacity overflow，S-8 崩溃族）/ `runtime-panic`），追加在失败五分类之后（`[run-classify]` 行）。
   - `--deny` 分级：`--deny equiv`（任一非零即整体失败）、`--deny equiv::<id>`（细粒度）、`--deny stub-hit`（run 失败 stub 子族）；默认全放行。

### 告警目录（seed）

| ID | 触发点 | 等级 | 条目 |
|---|---|---|---|
| `identity-hash` | `System.identityHashCode` / 默认 hashCode 路径 | 近似等价 | S-6 |
| `intern-identity` | `String.intern` + `==` | ~~近似等价~~ 已修（runtime 驻留表；计数维持发射点口径） | ~~S-6~~ |
| `null-array` | JArray null 表示 / null 数组访问 | 近似等价 | S-2.1 |
| `boxed-null` | 装箱类型 null 路径 | 近似等价 | S-3 |
| `class-literal` | `getClass`/类字面量同一性 | 近似等价（数组 getClass 臂已动态化，S-5 数组侧已修；计数维持发射点口径） | S-5 |
| `record-hash` | record `hashCode` | 近似等价 | S-7 |
| `neg-array` | `NegativeArraySizeException` | 近似等价 | S-8 |
| `field-npe` | null 接收者 `getfield`/`putfield` | 近似等价 | S-9 |
| `monitor-mt` | `monitorenter` 多线程互斥 | 条件等价 | S-11 / tasks.md P1 |
| `stacktrace` | `fillInStackTrace` / 栈帧 | 近似等价 | S-19 #5（已修；无 codegen 发射点维持不埋点） |
| `class-init` | JVMS §5.5 未覆盖触发点 | 近似等价 | S-10 |

埋点状态（2026-09-21）：除下两条外均已接入 `[equiv-audit]`——

- `monitor-mt`：monitorenter 发射点正被并行任务 S-20（锁真实化）改动，待其合入后在 monitor 发射位置补埋（见 `codegen/equiv_audit.py`）。
- `stacktrace`：无独立 codegen 发射点（`fillInStackTrace` 行为在 runtime 侧），随 S 候选条目立项后另行接入。

## 5. 与外部模型（ruva）的关系

六级分类、传递性规则、告警/deny 模型、反射静态注册表路线**采纳**；Deref 继承、`Vec<u16>` 字符串、`Option` null、`Mutex` synchronized 等已评估**不采纳**（本项目对应机制保真度更高，逐条理由见 compat-reference §一）。本文是本项目自己的承诺，与外部模型文本冲突时以本文为准。
