# 手写边界规范：什么可以手写

> 本文是手写层（`runtime/`）准入的唯一权威规范。`CLAUDE.md` 原则 0 / 3 只保留摘要与链接。
> 关联：[`docs/plans/2026-09-29-boundary-narrowing.md`](../plans/2026-09-29-boundary-narrowing.md)（C1d 边界收窄：实测与实施）、
> [`java-closure-analyzer.md`](java-closure-analyzer.md)（闭包分析器对手写的建模）、
> `runtime/java_runtime/{closure,seeds,vm_intrinsics}.toml`（清单）。
> 2026-09-30 与用户讨论确定。

## 一、原则

> **方法的语义以它自己的字节码为准；只有字节码无法表达该语义时才手写——没有字节码，或依赖运行期生成与加载字节码，
> 或由 VM 直接驱动。性能替换不算。**

- **判定单位是方法**，不是类、不是包。一个类里 native 方法手写、其余方法翻译字节码是常态。
  清单按类登记只为定位，准入按方法判定。
- **性能替换不是手写理由**。HotSpot 的 `@IntrinsicCandidate`（`Math.sin`、`String.indexOf`、`Arrays.equals` 等）
  JVM 执行的也不是字节码，但 rava 翻译其字节码、坚持字节码语义，不追平台内建。
  例：`Math` 的超越函数 JVM 走 libm 内建，rava 按字节码走 `StrictMath` 语义，两者都合规范（允许 1 ulp 误差）；
  e2e golden 取自 JVM 时，这类末位差异在比对口径上注明，不为对齐 golden 改手写。
- **闭包或体积膨胀同样不是手写理由**（与「性能替换不算」并列）。膨胀点先用闭包分析定位（反事实切除；
  `rava closure --gates` 门自动排名在开发中，分支 closure-gates），再按三类处理：分析器精度缺口（修精度）、
  构建期求值（引导映像）、运行模型（类 2 登记）。
- **手写不等于必须实现**：满足准入、但不在档案调用链上的方法仍是 `panic!("stub: 类.方法:描述符")` 存根，按档案调用链按需补。档案 = 一个构建单元（生产构建：用户项目；语料构建：e2e 全集）全体入口调用链的并集，开放世界下实例化集合折叠只对用户无法扩展的类型做（final / sealed / 非公开等），闭包规模以档案衡量（e2e 全集 JDK 21 实测基线 3609 类，见 T1 计划 §1.4）；当前闭包分析器仍按单测试逐例计算，档案化随 T1 实施。

## 二、准入类别（终态）

### 类 1：没有字节码——`ACC_NATIVE`

`Unsafe.*`、`Class` 元数据查询、`Thread.start0`、文件与系统调用；签名多态方法（`MethodHandle.invokeExact` /
`linkTo*`）同属此类。由 class 文件的 `ACC_NATIVE` 标志自动识别，**无需登记**。

native 方法的手写实现要在注释里说明它与 JVM 可观测行为一致的依据。`vm_intrinsics.toml` 中 `kind = "exact"`
（如 `StrictMath.sqrt` 以 Rust 原生运算逐位相同）是**native 实现的正确性说明**，不是独立的准入类别：
非 native 方法不能以 "exact" 准入。

### 类 2：运行模型替换——依赖运行期生成与加载字节码

原生二进制没有运行期类定义，JVM 在这些点生成字节码再加载执行，rava 以自有模型替代。它们**可以是非 native 方法**：

- lambda / indy 引导（`LambdaMetafactory`、`InnerClassLambdaMetafactory.spinInnerClass`）；
- LambdaForm 编译（`InvokerBytecodeGenerator.generateCustomizedCode` 等）→ 原生 LambdaForm 解释器；
- 动态代理（`Proxy.newProxyInstance` → `Proxy$Dyn`）、BMH 动态物种、序列化构造器访问器；
- `defineClass` / 隐藏类定义点及其对偶查询。

按方法登记（现为 `vm_intrinsics.toml [[intrinsic]]` 的 `class_definition` / `class_path` / `bytecode_generator` / `stack_frame`、
`[indy]`），不按类或包整体截断。

### 类 3：VM 注入的状态与 VM 驱动的行为

字节码里没有、由 VM 发起或写入的部分：

- **状态**：类元数据表、反射对象构造所依据的元数据（`getRecordComponents0` 等）、VM 注入的隐藏字段、
  VM 直接写入的字段。
- **行为的落地语义**：VM 驱动、原生二进制没有对应设施时，语义如何落地的决定。须在清单中逐条写明：
  - 引用类语义（`WeakReference` / `SoftReference` 清空、`ReferenceQueue` 入队、`Cleaner` / finalize）：由对象释放
    （`Rc` drop）触发，不引入任何 GC，见 [`docs/plans/2026-10-07-no-gc-memory-model.md`](../plans/2026-10-07-no-gc-memory-model.md)；
  - JVMTI 通知（`VirtualThread.notifyJvmti*`）；
  - 栈遍历（`StackWalker`、`fillInStackTrace`、`@CallerSensitive` 调用者传入）；
  - VM 发起的启动入口（`initPhase1~3`）——入口方法本身翻译字节码，落地的是「谁来调用」。

这些行为的入口多数本身是 native（类 1）；本类登记的是**语义决定**，不是新的方法类别。

### 不属于手写：指令语义与运行时基础设施

- **指令级语义**归生成器与运行时基础设施，不对应任何手写 Java 方法：类初始化触发与初始化锁（JVMS §5.5）、
  `monitorenter` / `monitorexit`、数组存取与越界、异常表。（`Object.notify` / `notifyAll` / `wait0` 是 native，属类 1；根类其余有字节码的方法——`equals`、`toString`、`wait()` / `wait(J)` / `wait(JI)`、`finalize`——按字节码翻译为自由函数 `Object__<fn>_body`（`generator/crates/emit/src/project/root_bodies.rs`），`object.rs` / `object_impl.rs` 只转交，见 regress2 文档 §10.1b。）
- **运行时基础设施**不对应 Java 方法：对象模型（`java/lang/Object` 的 `ObjectVTable`）、数组与字符串的内部表示、
  异常与 `Result`（`error.rs`）、crate 骨架（`lib.rs`）。

## 三、过渡类：规模策略截断

手写不是因为语义要求，而是为了控制闭包规模或编译成本。**C1d 终态（2026-09-30）已一次性删除**：

- 内部包前缀截断（原 `closure.toml [boundary]`：`sun/`、`jdk/`、`com/sun/`、`com/oracle/`、`java/security/`）
  与放行清单（原 `[release]`、`seeds.toml [jca]` 放行）已删除，JDK 全部类按字节码翻译；
- 前缀内的过渡手写（内部边界类整体手写的 struct 与方法、因截断补的 `StreamDecoder` / `StreamEncoder`、
  `ServicesCatalog`、`CleanerImpl$PhantomCleanableRef`、JCA 服务注册表、locale 数据束登记等）全部删除，
  只保留类 1–3（清单：`runtime/java_runtime/closure.toml [vm_boundary]`）；
- `sun/reflect/generics` 不再截断：其编译成本（泛型 visitor 体系曾使 `java_runtime` 编译峰值越过 15G）
  由分析器精度收敛解决，不以截断承载。

残留的策略截断：**0**（2026-10-08）。`java/nio/file/FileSystems` 已移出（a3-X2，`getDefault` 与默认持有者链按字节码翻译）；
`javax/crypto/JceSecurity` 已移出（引导映像第 5 步，boot-image §5.7.4：6 个手写删除，整类按字节码翻译，管辖策略经 NIO
读 `${java.home}` 嵌入虚拟树中的 policy 文件，U14）；`java/net/InetAddress` 2026-10-03 移出（`<clinit>` 未翻译使静态
`impl` 为 null，`getLoopbackAddress` 即 NPE，整类改按字节码翻译）。

规则：

- **单独计数，终态为 0**；不得与类 1–3 混在一起，不得作为新增手写的理由。
- 闭包或编译规模超预算时修分析器精度（死分支折叠、常量传播、逃逸判定），不恢复截断或手写。

## 四、struct 归属

- **终态下 Java 类的 struct 一律由字节码生成**，手写代码不拥有 struct。现行 3b 的「内部边界类 struct 与全部方法
  完整手写」是过渡形态，随前缀截断一起取消。
- VM 注入的隐藏字段（类 3 状态）由**清单声明、生成器追加**到生成的 struct。这与被禁止的 `/// @field` 注释注入
  不同：声明在清单里（清单即边界），不在源码注释里，生成器代码里也不写类名。
- 例外：运行时基础设施（`java/lang/Object` 等，见二）的内部表示由手写层定义。

## 五、文件布局

- 手写与生成共置，不存在独立的 `native_impls/` 目录。类 `X` 的手写方法放 `<x>_impl.rs`，与生成的 `<x>.rs` 并列，
  只写 `impl X { ... }` 块，由生成的 `mod.rs` 自然包含。
- 禁止 `/// @field name: Type` 注释注入与 `#[path = "..."] mod _impl;` 远程引用。
- 手写文件靠「无 `rava_macros::java_class` 生成标记」识别，codegen 不覆盖。
- **私有辅助子模块目录**（超大手写文件按职责拆分的唯一形态，频繁修改的手写文件保持 ≤~600 行）：手写文件
  `<stem>.rs` 可拥有同名目录 `<stem>/`（Rust 非 mod.rs 子模块布局），整棵子树是宿主的私有模块树，**不是 Java 包**。
  - 宿主仅限两类：共置手写文件（`<x>_impl.rs` / `<x>_ext.rs`，辅助目录名以 `_impl` / `_ext` 结尾）与 crate 根下的
    基础设施模块文件（`lib.rs` 除外，如 `array.rs` → `array/`）。识别规则：目录旁有同名宿主文件且宿主属上两类。
    JDK 包段不以 `_impl` / `_ext` 结尾、crate 根基础设施名不是 JDK 顶层包名，二者不可能与 Java 包目录重合；
    发射期仍检查（类的包目录落入辅助目录即报错）。
  - 书写约定：宿主声明子模块恰为 `mod <name>;` 与 `use <name>::*;` 两行；辅助文件以 `use super::*;` 取得宿主的
    名字空间，不写其他 `super::` / `self::` 相对路径，不写内层属性 `#![…]`（`//!` 文件说明允许）；供宿主调用的项
    用 `pub(super)`（`pub fn` 是手写方法的登记口径，只用于 `impl X` 块内的 Java 方法）。
  - 统一识别（`generator/crates/closure/src/handwritten/layout.rs` 为唯一实现，`runtime/java_runtime/build.rs`
    同口径）：overlay 按宿主所在包把辅助目录定向到宿主的模块 crate；生成 mod.rs 不进入辅助目录（与手写 mod.rs
    目录同样保留整棵子树）；闭包分析器、手写事实扫描（`pub fn` 登记、raw-audit 手写审计）、方法体层的手写探测、
    build.rs 的 native 状态表都把辅助文件并入宿主的「手写单元」——单元文本 = 宿主 + 辅助文件按路径序拼接（剥去上述
    声明行），语义等同拆分前的单文件，拆分是纯搬移、分析结果不变；行表按单元内逐文件成表。
  - 守护：`layout::tests::runtime_helpers_follow_convention` 检查真源全部辅助文件的书写约定与宿主声明。
  - 现有辅助目录（2026-10-05 拆分，纯搬移）：`array/{vtable,view}.rs`、`java/lang/class_impl/{attrs,members,nest}.rs`、
    `jdk/internal/misc/unsafe__impl/{access,primitive,memory}.rs`。
- **scratch 与真源同步**：overlay 把本轮落盘的手写文件记入 scratch 根 `.rava_overlay`；复用 scratch 时上轮清单中
  本轮未落盘、且无生成标记的文件删除、空目录移除（runtime/ 中删除 / 改名的文件与辅助目录、非 `.rs` 资源、手写
  `mod.rs`、改定向到别的模块 crate 的目录都覆盖）；带生成标记的文件从不因此删除。

## 六、登记与审计

- **清单即边界**：边界、放行、补种、VM 承载、手写登记全部在 `runtime/java_runtime/` 的 TOML 清单中
  （读取入口 `generator/crates/input/src/manifest.rs`、`generator/crates/closure/src/manifest.rs`），生成器代码里不写类名。
- **每个非 native 的手写方法在清单里登记类别**：运行模型（类 2）/ VM 行为（类 3）/ 策略截断（过渡），附依据。
- raw-audit 按类别计数：
  - 策略截断：终态 0，只减不增；
  - 类 2 / 类 3：有上限的枚举，新增须登记并说明依据；
  - 未登记的非 native 手写：直接计为回归（与 `non_native_overrides` 同口径，该项 2026-09-28 已清零）；
  - 反向检查：登记为非 native 的方法必须确有字节码。
  - 根类（`Domain::Root`，不入注册表）同口径：非 native 方法一律翻译字节码（档案链外为 panic 存根），`<init>` 体非单条 `return` 时计 `HwAudit::Override`（`root_bodies::emit`）；分析器侧根类事实 native → 手写、有码 → 字节码（`closure/src/engine/facts/kinds.rs`），不再整类视为手写。
- 分析器对手写的建模：手写返回对分析不透明、只能取 open(返回类型)，精度低于字节码（C1d 实测：`jdk/internal/misc`
  放行后 CollectorsDemo 闭包 −105 类）。这是收窄手写的直接收益之一。

## 七、现状（2026-10-08，引导映像第 5 步后）

全仓 `#[jvm_boundary]` = **0**（引导映像第 4 步 33 → 14，第 5 步 14 → 0：ClassLoader 资源族 6、BootLoader 2、
JceSecurity 6，见 boot-image §5.6、§5.7）。

| 调用目标 | 处理 |
|---|---|
| JDK 全部类（`java/`、`javax/`、`jdk/`、`sun/` 等） | 翻译字节码；`ACC_NATIVE` 手写（类 1），含运行期类定义点（类 2）与 VM 驱动行为（类 3） |
| `closure.toml [vm_boundary]`：只剩 `java/lang/Class` | 类 3：struct 承载 VM 注入的镜像状态（类元数据指针、数组组件类型等）；`<clinit>` 构建期在 VM 预初始化中执行（`clinit_carried`）；按方法划分：native / 内建 / 按精确名提供的手写取手写（`vm_boundary_methods` 计数），其余翻译字节码 |
| `[vm_boundary].translate_nested` | VM 契约类的纯 Java 嵌套类，按字节码翻译 |
| 已移出 `[vm_boundary]` | `VM`、`Module`、`ModuleLayer`、`SecurityManager`（第 4 步）；`ClassLoader`、`BootLoader`、`JceSecurity`（第 5 步）；`FileSystems`、`InetAddress`。手写只剩各自的 `ACC_NATIVE` |
| 模块资源 / `${java.home}` 文件 | 发射层把闭包读取的模块资源写成 jimage 嵌入（唯一手写 native `NativeImageBuffer.getNativeMap`）；`java.home` 构建期钉值（U14），其下 `conf/security`、`lib/modules`、`lib/tzdb.dat` 为只读嵌入虚拟树，由 NIO / `UnixFileSystem` 的 native 手写读取（`jdk_resources`） |
| VM 引导期 | 构建期引导映像求值（`vm_intrinsics.toml [concrete.boot]`），原 `seeds.toml [boot_init]` 已删除 |

根类 `java/lang/Object`（2026-10-09，object-bytecode 分支）：非 native 方法（`equals`、`toString`、`wait` 三重载、`finalize`）按字节码翻译，手写只剩 native（`getClass`、`hashCode`、`clone`、`notify`、`notifyAll`、`wait0`）与 `ObjectVTable` 运行时契约；HelloWorld / DeepCopy 闭包与基线相同（regress2 §10.1b-实施）。

已移出 `[vm_boundary]`、整类按字节码翻译（手写只剩 `ACC_NATIVE`）：`VM`、`Module`、`ModuleLayer`、`SecurityManager`（引导映像第 4 步）、`Unsafe`（a3-U0~U3）、`FileSystems`（a3-X2）、`InvokerBytecodeGenerator`（a3-X1，类定义点按 `[[intrinsic]] class_definition` 登记）。

原 `[boundary]` 前缀、`[release]`、`seeds.toml [data_bundle]`、`seeds.toml [boot_init]`（引导初始化改由构建期引导映像求值器承担，`[concrete.boot]` 在 `vm_intrinsics.toml`，引导映像第 3–4 步）已删除。截断的原始理由
（`docs/reports/2026-09-14-impl-strategy.md`：跟随内部包类数 111 → 635）是 Python BFS 过近似口径；
精确闭包分析下的实测与精度收敛项见 C1d 计划 §6.12。

## 八、类 2 / 类 3 实施盘点（2026-10-08）

**类 2 已完成**

- lambda / indy（`vm_intrinsics.toml [indy]`：lambda、字符串拼接、`typeSwitch` / `enumSwitch`、`ObjectMethods`）；
- LambdaForm 编译 → 原生 LambdaForm 解释器（MH-native）；
- 动态代理 `Proxy$Dyn`、BMH 动态物种 `Species_Dyn`（Java 源在 `runtime/java_support/`）；
- 序列化构造器访问器；`@CallerSensitive` 注入调用器；
- 类路径运行模型 `class_path` 4 条（`EmbeddedClassPath`）。

清单登记（`[[intrinsic]]`）共 13 条：`class_definition` 7、`class_path` 4、`bytecode_generator` 1、`stack_frame` 1；
另有 `exact` 1 条，属 native 实现的正确性说明（见类 1），不计入类 2。

**类 2 未完成**

- 模块层 / jimage：引导映像第 5 步，进行中（分支 boot-image-s5）；
- `ClassLoader` 资源方法 6 个、`BootLoader` 2 个 `#[jvm_boundary]`：随第 5 步删除；
- `JceSecurity` 6 个 `#[jvm_boundary]`：构建期求值，与第 5 步同任务；
- 用户代码 `defineClass` / 隐藏类的通用入口：未统一落地。

**类 3 已完成**

- 类 / 反射 / 注解 / 行表元数据（`java_meta`）；
- VM 注入的静态与隐藏字段（`[vm_constants.injected_statics]`，清单声明、生成器追加）；
- `@CallerSensitive` 调用者显式传入（`[caller_sensitive]`）；
- `fillInStackTrace` / `StackWalker`（`vm_stack.rs`）；
- VM 启动入口 initPhase1–3：构建期引导映像求值器（第 1–4 步）；
- Continuation 有栈协程（2026-10-03 决定，`continuation_impl.rs`）。

**类 3 未完成**

- 引用类语义：无 GC 模型，C4 之后实施（见类 3 条目）；
- 日志 / 信号链（(L) 链，`Shutdown.exit` → `logRuntimeExit`）：信号行为与 JDK 一致（2026-10-07 已定）；链上 `LoggerFinder` 提供者构建期确定、日志级别按映像值折叠（U12 已定，2026-10-08，引导映像计划 §8.4），机制 ①③ 进行中（分支 u12-props）；
- JVMTI 通知（`VirtualThread.notifyJvmti*`）：当前空操作，语义决定待在清单逐条登记。
