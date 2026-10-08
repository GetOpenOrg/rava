# 构建期引导映像求值器（boot image evaluator）终态方案

> 2026-10-05，分支 `boot-image`（基于 rust-closure-analyzer 0192bf20）。本文只含设计与可行性探针实测；探针代码在本分支单独提交，不合入集成分支。
> 上游：c1d §25.3（a1 引擎执行 initPhase2 不可行，需另立构建期引导映像求值器）、boundary-narrowing §6.11 项 3（`build_time_init`，2026-10-01 用户决策归 C3）、
> boot-layer.md 第 2–5 步、c1d §21.9（a3 余项）。

## 0. 结论

> **2026-10-05 用户决策 U0–U6 已定（§8.1）**；第 1 步已在分支 `boot-image-s1` 实施，实测见 §5.3。
> U1 改判为「运行期取宿主值」：下文 §3.2、§5.2 B1、§6 第 2 步验收已按 U1 改写；§0 第 1 条与 §5.1 的探针数字为钉值时的历史实测。2026-10-08 U1 被 U14 部分修订（§8.4）：`line.separator`、`file.encoding`、`java.home` 改为构建期钉值（逐项实测闭包收益，无收益不钉），`sun.jnu.encoding`、`stdout/stderr.encoding` 仍运行期读取。
>
> **现状（2026-10-08）**：第 1–2 步已完成（合入集成分支 c5812d89）；第 3 步（boot-image-s3）随 batch-1007 测过、未单独合入；第 3–4 步待合批验证（boot-image-s4 c614f840，已合入 batch-1008 b52917c9，`#[jvm_boundary]` 23 → 14）；第 5 步进行中（分支 boot-image-s5，jimage + JceSecurity 6，目标 14 → 0）。U12 / U13 已定（2026-10-08，用户采纳建议），U14 已定（2026-10-08，U1 部分修订），见 §8.4。派发：u12-props（U12 ①③ + U14 line.separator / file.encoding，基于 b558e0c2）进行中；U14 java.home 随 boot-image-s5；U13 待派。第 6 步（boot-image-s6，§5.8）：非引导类构建期初始化已实现，类 −30 / 方法 −734（HelloWorld），正则链验收阻塞于映像 lambda 对象（§5.8.3）。

1. **可行，探针已在构建期跑完 HotSpot 的全部三个引导阶段。** 探针用的是 `engine/concrete` 同一个解释器的引导模式，输入是 macOS JDK 21.0.11 的 java.base 字节码，例子为 HelloWorld：
   - 次序：VM 预初始化 9 类和 3 个 VM 构造对象，然后 `initPhase1`、`initPhase2(false,false)`、`initPhase3`；
   - 规模：共 **1,609,459 条指令、10,805 个堆对象（映像可达 8,356 个）、274 个已初始化类、约 130–150 ms**；
   - 结果：`initPhase2` 返回 0；结束时 `VM.initLevel = 4`、`ClassLoader.scl = AppClassLoader`、`System.bootLayer = ModuleLayer`；
   - VM 模块表：62 个模块、771 个包；
   - 三次运行输出逐字节相同。
2. **求值器复用具体引擎，不另写解释器。** 引导模式与 a1 的调用点求值模式只在 5 处语义上分叉（§3.1），全部 native 语义由清单 `[concrete.boot]` 驱动，生成器里没有类名特判。
3. **映像物化为档案的初始状态。** 映像只取决于（JDK 版本、目标平台、清单），与用户程序无关，所以每个档案键只物化一份：
   - 语料模式下映像随 `java_base` 档案动态链接；
   - 生产模式下随档案静态链接，并按档案闭包裁剪到可达部分。
4. **抽象分析从映像出发。** 引导阶段的代码不再进闭包，Resolver、Configuration、ModuleDescriptor 等都不入链，映像中的静态字段以具体对象作为初始值集：
   - §25 的 2721 类膨胀里，约 865 类来自对引导代码的抽象分析，这部分直接消失；
   - 余下 1852 类要靠「Class 接收者逐镜像求值」（在途）才能消除，映像给它提供了每个镜像准确的 `module` / `classLoader`；
   - HelloWorld 的上界估计为 **469 + 71 = 540 类**，低于 569 的目标（§3.4）。
5. **需要新增宿主相关值的污点与运行期重算。** 探针发现 `ConcurrentHashMap.NCPU`、`Striped64.NCPU`、`VM.directMemory` 由 `availableProcessors` / `maxMemory` 直接派生。探针把它们按副作用延迟、读成 0，这是**错误值**。终态做法：
   - 给值带污点，污点值写入的静态字段生成「启动重算切片」；
   - 在污点上做控制流分支时，该类改为运行期初始化（§3.2）。
6. **a3 三问**（§4）：
   - **initLevel**：第三条路成立。映像里 `VM.initLevel = 4` 是实测值，`initLevel()` 按字节码读映像字段，`#[jvm_boundary]` 中 VM 的 10 个都可以删；前提见 §4.1。
   - **包 / 模块表**：映像可以物化 7 张表（§4.2），覆盖 L2、BootLoader、SecurityManager、Module、ModuleLayer 和 `Class.getModule`。
   - **JceSecurity**：读取结果可以物化，前提是 java.home 的只读文件树作为构建期输入（§4.3）。

## 1. 现状

### 1.1 三个引导阶段在 rava 中的落地

| HotSpot 阶段 | rava 现状 | 位置 |
|---|---|---|
| `Threads::initialize_java_lang_classes`：String、System、Class、ThreadGroup、Thread、Module、UnsafeConstants、Method、Finalizer，以及 `create_initial_thread_group` / `create_initial_thread` | 不存在。运行期各类惰性初始化，主线程由手写层构造 | runtime `thread_impl.rs` 等 |
| `initPhase1`：属性表、`VM.saveProperties`、`setJavaLangAccess`、stdin/out/err、`Terminator.setup`、`VM.initLevel(1)` | 手写的 `System.registerNatives` 承担其中的属性表、saveProperties 段和 `out` / `err` / `in`，首步与 initPhase1 同序调用翻译的 `setJavaLangAccess`；`System` 由 `[boot_init] classes` 首项在 main 前初始化（10-07 c4-regress：原经 `[boot_init] calls` 进入，属性表段先于登记触发 `ArraysSupport.<clinit>` 缓存 null JLA） | `system_impl.rs`、`seeds.toml [boot_init]` |
| `initPhase2`：`ModuleBootstrap.boot()`，建引导层 | `[[boot_init.phases]]` 已经清单化，锚点留空，所以不入链、不执行；Module、ModuleLayer、`Class.getModule` 由手写近似 | `seeds.toml`、`module_impl.rs`、`module_layer_impl.rs` |
| `initPhase3`：安全管理器、`initSystemClassLoader`、`setContextClassLoader`、`VM.initLevel(4)` | FS-C2 在 `ClassLoader.scl` / `Thread.contextClassLoader` 首次读时由字段钩子 `__vm_init_phase3` 惰性执行 | `class_loader_impl.rs` |
| `VM.initLevel()` | 手写读取模型 `__vm_at_init_level`：saveProperties 段为 0，惰性 initPhase3 段为 3，其余时间为 4 | `vm_impl.rs`（`#[jvm_boundary]`） |

### 1.2 VM 常量与属性的来源

- **VM 注入常量**：`UnsafeConstants`（ADDRESS_SIZE、PAGE_SIZE、BIG_ENDIAN、UNALIGNED_ACCESS、DATA_CACHE_LINE_FLUSH_SIZE）以及 Unsafe 的 `ARRAY_*_BASE_OFFSET` / `INDEX_SCALE`。
  它们登记在 `vm_intrinsics.toml` 的 VM 常量段，由生成器写入；分析期以事实形式折叠。
- **系统属性**有两类来源：
  - VM 属性：`java.home`、`java.vm.*`、`sun.boot.library.path` 等；
  - 平台属性：`SystemProps$Raw` 的 39 个下标，即 `os.*`、`user.*`、`file.encoding`、`sun.jnu.encoding`、`line.separator` 等。

  运行期由 `system_impl.rs::host_property` 取宿主值；分析期的 `system_properties` 事实只折叠与宿主无关的键。
- **seeds.toml `[boot_init]`**：`calls` 是 VM 启动期调用的无参静态方法，`classes` 是需要提前初始化的类，`phases` 是带锚点的引导阶段。它们都是**运行期**执行的根：分析器把它们作为根入链，生成的 main 在启动时按序调用。

### 1.3 具体引擎（a1，`engine/concrete`）

具体引擎原本的语义是「无副作用、可撤销的调用点求值」：
- 堆按纪元（epoch）划分，纪元 0 是 `<clinit>` 映像；
- 可变静态字段不可读，跨类写静态字段失败；
- 手写方法按 native 处理，op 名来自 `[concrete.natives]`。

c1d §25.3 在进入 `boot2` 之前就失败，失败点依次是 `registerNatives`、`System.props`、`SharedSecrets`、它类静态写入、`UnsafeConstants`。这几处正是引导模式需要分叉的语义。

## 2. 业界参照与取舍

| | GraalVM Native Image | Leyden / CDS 归档堆（JDK 21 full module graph） | rava 终态取法 |
|---|---|---|---|
| 构建期执行什么 | 显式或推断为「构建期初始化」的类的 `<clinit>`；JDK 自身大部分类缺省在构建期初始化 | dump 时跑一遍真实启动，归档一组**白名单**静态字段的对象图：`ArchivedModuleGraph`、`ArchivedBootLayer`、`ArchivedClassLoaders`、Integer 缓存等，其余类在运行期照常初始化 | **按 VM 引导阶段整体执行**（HotSpot 预初始化 → initPhase1 → initPhase2 → initPhase3），类集不按白名单划，由「是否碰到宿主相关值」自动判定 |
| 宿主相关值 | 不允许进映像堆（Random、Thread、FileDescriptor 等），违者构建失败；`--initialize-at-run-time` 与字段 `@RecomputeFieldValue` 替换 | 只归档对宿主不敏感的子图；`CDS.getRandomSeedForDumping` 为 SALT 提供确定种子，dump 期属性受控 | 污点追踪（§3.2）：污点值不进映像，碰到污点分支的类自动转为运行期初始化，直接派生的字段生成重算切片；全部记录在审计报告里 |
| 运行期衔接 | 映像堆在 `.data` 段，零拷贝；类初始化状态随映像 | 映射归档区；`CDS.initializeFromArchive`、`defineArchivedModules` 等 native 把归档对象接回静态字段 | 映像是档案 crate 的初始状态（§3.3）；运行期不再执行引导阶段，只执行延迟清单 |
| 对静态分析的作用 | 分析从映像堆出发，映像对象是已知常量 | 不参与 | 同 GraalVM：抽象分析的初始值集来自映像 |
| 代价 | 正确性依赖构建期初始化策略，配置面大 | 只覆盖白名单，对闭包规模无帮助 | 求值器要实现引导期 native 全集，需要清单维护；污点规则必须健全 |

**取法：GraalVM 式的「执行到底 + 映像即分析初始状态」，宿主边界用 CDS 式的「JDK 自带确定化钩子」。** 理由：
1. 本方案的首要收益是闭包缩小。CDS 白名单式归档不改变分析的输入，对 §25 没有帮助。
2. CDS 的 `ArchivedModuleGraph` 等路径要求 `CDS.isDumpingStaticArchive` 等 VM 状态为真，会走 JDK 为归档准备的分支。rava 按缺省分支执行（归档开关为假），与 `-Xshare:off` 的 JVM 行为一致，不引入第二条路径。
3. JDK 已有的确定化钩子（`getRandomSeedForDumping`）直接复用，不另造。

## 3. 终态架构

### 3.1 求值器语义：复用具体引擎的引导模式

**不新写解释器。** 引导模式与 a1 共用以下部分：
- 指令解释、对象模型、字符串驻留与镜像；
- 字段键、Unsafe 偏移模型和 native 分派。

两者只在以下 5 处分叉（探针已全部实现，开关为 `Vm.boot`）：

| # | a1 调用点求值 | 引导模式 |
|---|---|---|
| 1 | 只有 `<clinit>` 期分配进纪元 0 | 全部分配都永久（纪元 0），不撤销 |
| 2 | 可变静态字段不可读（值可能被运行期改写） | 可读：引导阶段就是运行期的开头，次序与 HotSpot 相同 |
| 3 | 不允许跨类 `putstatic` | 允许（`setJavaLangAccess`、`Reference.<clinit>` 写 SharedSecrets 等） |
| 4 | 手写方法一律作为不透明 native | 有字节码就执行字节码，只有 `ACC_NATIVE` 和清单登记的 VM 方法走 native；与运行期是否手写无关，引导阶段按 JDK 语义执行 |
| 5 | 失败即放弃该调用点 | 失败分两类：碰到**延迟值**时，该类转为运行期初始化并级联；其他失败（缺 native 语义、不支持的指令）则构建失败并报出栈，不静默降级 |

**堆模型**
- 对象只按字段键存储，没有地址；Unsafe 偏移等于字段身份。
- 数组偏移为 `16 + i × scale`，与清单中的 `ARRAY_*_BASE_OFFSET` 一致。
- `byte[]` 支持多字节组合读写，用于 `ArraysSupport`、`StringUTF16`、`ByteArray`。
- 子字的 CAS（`compareAndExchangeByte` / `Short`）在 JDK 字节码里做字对齐地址运算。对象模型没有物理布局，所以这两个方法由清单覆盖为按元素操作（探针 `unsafe_cax:B` / `S`）。这是**求值器内部的布局约定**，不进入运行期。
- VM 单元（`vm_cell:<name>`）承载 native 地址上的计数器，例如 `ThreadIdentifiers` 的 next id。物化时作为对应 native 的初始状态。

**native 全由清单驱动**：`[concrete.boot]` 包含：
- `natives`：引导期 op 表，优先于 `[concrete.natives]`；
- `statics`：VM 注入的静态字段；
- `vm_props` / `platform_props`：属性；
- `init`：VM 预初始化类，按 HotSpot 次序；
- `objects`：VM 构造的对象（初始线程组 / 线程）；
- `calls`：阶段入口。

探针新增 op 21 种（`defer`、`defer_value`、`defer_call`、`vm_record`、`vm_cell`、`unsafe_get` / `put` / `cas` / `cax`、`props:*`、`set_static:*` 等）。op 的语义是通用的，类名只出现在清单里，生成器与闭包分析器 crate 中没有 JDK 字面量。探针新增的 Rust 代码已按此检查：`mirror_module` 用「首个 `defineModule0` 即基础模块」这一 VM 规定，不写包名。

### 3.2 构建期 / 运行期边界

**宿主相关输入分类**（都作为清单登记的延迟源，求值器本身不认识它们）：

| 类别 | 例子（探针实际命中的加 ✓） | 处理 |
|---|---|---|
| 平台属性中的宿主值 | `os.version` ✓、`user.dir` / `user.home` / `user.name`、`java.io.tmpdir`、`file.encoding` 的宿主取值 | `defer_value`：返回延迟字符串，内容数组标为延迟 |
| 影响控制流的属性 | `sun.jnu.encoding` ✓（initPhase1 中 `Charset.isSupported` 分支）、`stdout/stderr.encoding` ✓、`file.encoding`、`line.separator`、`java.home` | 取决于运行宿主的（`sun.jnu.encoding`、`stdout/stderr.encoding`）：**运行期取宿主值**（U1）：`defer_value`；求值器把依赖它们的部分残差化（下文「残差化」）。由目标平台或 JDK 规范决定的（`line.separator` 按目标三元组、`file.encoding` = UTF-8、`java.home` 取构建期值）：构建期钉值（U14，2026-10-08，逐项实测有收益才钉；`java.home` 已实施，§5.7.3） |
| 时间 / 熵 | `nanoTime`（ImmutableCollections SALT）✓、`currentTimeMillis`、`/dev/urandom` | SALT 由 JDK 钩子 `CDS.getRandomSeedForDumping` 给固定种子 ✓；其余时间和熵为延迟值 |
| 机器资源 | `availableProcessors` ✓、`maxMemory` ✓ | 延迟值并带污点（终态，见下）；探针按 `defer` 读成 0 是**错误简化** |
| 文件系统 | `canonicalize0` / `getBooleanAttributes0` ✓（空 class path 即 cwd） | 宿主路径：延迟值。`URLClassPath.toFileURL` ✓ 为 `defer_call`，构建期得到占位对象、运行期重放。java.home 只读树是构建期输入，按 §4.3 处理 |
| 线程 / 进程副作用 | `Thread.start0` ✓（Reference Handler）、`setPriority0` ✓、`Terminator.setup` ✓、`VM.initializeOSEnvironment` ✓、`FileDescriptor.getAppend` ✓ | `defer`：记入**启动重放序列**，运行期按构建期次序执行 |

**延迟值的传播（终态）**
1. **污点值**：求值器值域 `CV` 加一个污点位，算术、比较、存储都会传播污点。探针只做到「延迟字符串的内容数组」这一层，终态扩展到标量。
2. **直接派生的静态字段可以重算**：污点值沿无分支的数据流写入静态字段时，求值器记下派生表达式。例如：
   - `NCPU = Runtime.availableProcessors()`；
   - `directMemory = Runtime.maxMemory()`（`VM.saveProperties` 中是先按非污点属性分支、再直接赋值）。

   该字段物化为「启动重算槽」，启动重放序列按次序重新求值这些表达式。所在类仍属构建期初始化，实例照常进映像。
3. **在污点上分支、或把污点写入对象图的类转为运行期初始化**：
   - 该类的 `<clinit>` 在运行期执行；
   - 构建期读取该类静态字段的其他 `<clinit>` 级联转为运行期，探针已实现并报出连带链；
   - 映像中不得存在依赖该类静态状态的对象（违者构建失败）。

   探针实测级联：`OSVersion`（os.version）→ `ClassLoaderHelper` → `NativeLibraries`，共 3 类。
4. **占位对象**（`defer_call` 的返回值）只允许存入字段，不允许读取它的内容；运行期重放时回填该字段。探针中的 `URLClassPath.toFileURL("")` 就是这样处理的，app class path 的 URL 在运行期建。

**残差化（U1，第 1 步已实现）**：延迟值参与求值时，求值器按出现位置自动选择运行期执行的最小单位，全部记入映像的运行期部分（`Rec`），按构建期次序重放：

| 形态 | 触发 | 运行期动作 | HelloWorld 实测（JDK 21 macOS） |
|---|---|---|---|
| 运行期初始化类 | 类的 `<clinit>` 中延迟值参与求值 | 撤回该 `<clinit>` 全部写入；首次主动使用时运行期执行 | `StaticProperty`（java.home / user.dir 等）、`NativeLibraries`（macOS 链） |
| 残差调用 | 根帧调用的被调方法内延迟值参与求值，且结果只被存放 | 撤回该调用；运行期重放，结果为占位对象 | `newPrintStream` ×2（stdout / stderr 编码） |
| 残差区段 | 根帧中延迟值决定分支 | 根帧 `[s, ipdom)` 整段运行期执行，构建期只求写入集（区段内的延迟引用以占位对象代之） | initPhase1 jnu 编码段、initPhase3 不支持编码的告警段 |
| 静态读取占位 | 读运行期初始化类的 final 引用静态字段（未被改写） | 运行期触发该类初始化后回填 | `StaticProperty.JAVA_HOME` ×2、`USER_DIR` |
| 条件脏静态 | 被撤回的 `<clinit>` 写过的静态字段 | 写入者日后在构建期重新初始化即不脏；转运行期初始化则读者级联延迟 | — |

占位对象只可存放、传递、判空（延迟调用与清单承诺非空的结果）、按声明类型 checkcast；读写其状态或比较身份即延迟。
内存缓存字段（`String.hash`、`Class.packageName` 等，清单 `memo_fields`）的写入不记日志：撤回时保留，重算得同一值。

**判定是全自动的**：清单只登记源（哪些 native / 属性 / 静态字段宿主相关），哪些类在构建期初始化由求值结果决定。每次构建输出审计表，包含延迟源命中、运行期初始化类及其连带链、重算槽和重放序列。

### 3.3 物化：映像作为档案的初始状态

**映像的键**：（JDK 版本与平台、`vm_intrinsics.toml [concrete.boot]` 的内容摘要、钉值属性）。引导阶段不执行用户代码，`java.class.path` 在原生二进制中固定，所以映像与用户程序无关。
- **语料模式**：每个档案键一份映像，随 `java_base` 档案 dylib 链接，全部测试共享。
- **生产模式**：同一份映像随档案静态链接，按「闭包可达静态字段的传递闭包」裁剪（§3.4 的联合不动点）。探针中全映像有 418 个静态字段、8,356 个可达对象、87,479 个槽位；裁剪后的规模在第 3 步实测。

**形态**
- 映像生成为一个 crate 段（`java_base` 档案内的 `boot_image` 模块），内容是：
  - 每个对象的（类 id，按 S7 每类静态描述符排列的字段槽位表）；
  - 数组元素表；
  - 静态字段初值表；
  - 驻留字符串表（1,280 个）；
  - 镜像表：87 个类，字段 `module` / `classLoader` / `componentType` 等引用映像对象；
  - VM 表：模块 / 包 / 读写导出，以及 VM 单元；
  - **身份哈希**：87 个映像对象在构建期取过 identityHashCode，并作为 HashMap / IdentityHashMap 桶位的依据。物化时必须原样带上，运行期 `identityHashCode` 对映像对象返回构建期的值。
- **装载方式**由 S7 句柄形态决定（用户决策 U4）：
  - 甲：启动时按表批量建对象，成本是一次线性扫描。按每对象约 50 ns 估算，8k 对象约 0.4 ms；
  - 乙：句柄为 arena 下标且映像区不参与引用计数时，映像直接作为 arena 前缀的静态数据，零拷贝。终态取乙，前提是 S7 句柄允许「永久区」。
- **与 java_meta 的关系**：java_meta 继续承载类元数据（反射成员表、注解、行表）。映像只按类 id 引用镜像，不复制元数据。反射缓存（`Class.reflectionData` 的 SoftReference）如果在引导期被填充，其中的 Method / Field 对象由构建期的 `getDeclaredMethods0` 产出，成员次序与 java_meta 同源，两者由一致性单测守护。
- **运行期启动序列**：
  1. 装载映像；
  2. 把映像中的初始线程绑定到 OS 主线程；
  3. 按次序执行重算槽和重放序列（start0、Terminator.setup 等）；
  4. 进入 main。

  `[boot_init]` 的 `calls` / `classes` / `phases` 和 FS-C2 的 `__vm_init_phase3` 钩子都由映像取代后删除。

### 3.4 抽象分析从映像出发

- **初始状态**：映像中每个静态字段的抽象值是其具体对象集合，抽象对象就是映像对象本身（分配点 `image:#n`，字段敏感）。镜像的 `module` / `classLoader` 都是精确值。
- **入口**：只有 main 和种子。引导阶段方法不再作根，因为它们已在构建期执行完。映像对象上的方法只在运行期代码对它们调用时才可达。
- **裁剪**：闭包不动点与映像可达性一起求，只物化从「闭包内被读取的静态字段」可达的映像对象。
- **对 §25 的效果**（HelloWorld 锚点启用后多出 2721 类，按 §25.2 的出口分类）：

| §25.2 出口 | 类数 | 有映像后 |
|---|---:|---|
| `ModuleDescriptor.toString` → stream | 约 370 | 引导代码不入链，**0** |
| `Version.compareTokens` 读 `List<Object>` | 约 250 | 同上，**0** |
| 其余（`SystemModuleReader.read`、`BootLoader.packages`、`boot2` → `loadModule` 等） | 约 200 | 引导期执行的部分为 0；运行期读模块内容的路径按真实需要入链（第 5 步 jimage） |
| 建层直接需要的类 | 45 | 0（建层在构建期完成） |
| 建层后原本折叠的分支变为可达（`getResourceAsStream` 命名模块分支等） | 1852 | 映像使每个 boot 镜像的 `module` 成为精确的命名模块。这条分支对**确实调用 `getResourceAsStream` 的程序**本来就可达，与 JVM 一致；HelloWorld 不调用，膨胀来自接收者值集不分镜像。要靠在途的「Class 接收者 classLoader / module 逐类求值」消除，映像为它提供具体值 |

- **上界实测**：映像可达对象有 150 个非数组类型，全部在 274 个已初始化类之内，其中 71 个不在现有 HelloWorld 闭包（469 类）里（java/lang/reflect 21 个，主要是 `AccessFlag` 枚举的 18 个常量类；jdk/internal/module 11 个；java/lang/module 11 个；java/lang/ref 6 个……）。
  - 即使不做联合裁剪，HelloWorld 也有 **469 + 71 = 540 ≤ 569**。
  - 只有已初始化、没有实例、静态字段也不被闭包读取的类不进闭包，所以 274 − 150 = 124 个类只算初始化状态位，不带代码。
  - 联合裁剪后，71 个中 `AccessFlag` 一类（只有 `ModuleDescriptor` 构建期使用）预计出闭包，第 3 步实测。

### 3.5 与其他线的关系

- **a3 `#[jvm_boundary]` 归零**（全仓 33 个属性；HelloWorld 审计口径 29 个方法）：VM（审计 10 个）经 §4.1 删除；Module 7、ModuleLayer 2、Class 2、ClassLoader 6、BootLoader 2 经 §4.2 删除；JceSecurity 6 经 §4.3 删除。归零后只剩 `ACC_NATIVE`（准入 ①）。
- **boot-layer 第 2–5 步**：
  - 第 2 步的 `Class.module` 钩子不再需要，映像直接带上镜像字段，HotSpot `create_mirror` 的修补在求值器里完成，探针已实现；
  - 第 3 步的锚点机制由映像取代（`[[boot_init.phases]]` 删除）；
  - 第 4 步的 `initServices` 在构建期执行，`ServicesCatalog` 进映像；
  - 第 5 步的 jimage 读取器仍然需要，因为运行期模块资源读取要用；但 `SystemModules$default` 等大方法只在构建期执行，**不再需要翻译和编译**，boot-layer 2.2 第 2 条的大方法拆分随之不必做。
- **虚拟线程**：映像中不允许出现已挂载帧的 `Continuation`，也不允许出现已启动的线程。
  - `VirtualThread` 的 `DEFAULT_SCHEDULER` 依赖 `availableProcessors`，在污点上分支，按 §3.2 第 3 条自动转为运行期初始化。
  - 有栈协程方案（10-03 定）不受影响。
- **反射元数据**：见 §3.3。映像引用镜像时只用类 id，java_meta 不变。注解解析结果如果在引导期被缓存（`AnnotationType`），同样只能来自构建期执行字节码的结果。
- **C3 `build_time_init`**（2026-10-01 决策）：本方案是它的超集，即「无宿主副作用的 `<clinit>` 在构建期执行，对象图成为快照事实」。按用户程序可达性对非引导类（Formatter 的常量正则等）做构建期初始化，用的是同一个求值器和映像格式，属第 6 步。

### 3.6 确定性

1. **种子**：SALT32L 来自 `CDS.getRandomSeedForDumping`，由清单给定固定值；除此之外引导阶段的熵源全部是延迟值。
2. **身份哈希**：按分配序确定性生成（`0x1000 + n × 7919`），并且物化。运行期对映像对象返回同值，保证构建期建好的哈希表桶位在运行期仍然有效。
3. **求值次序**：单线程顺序解释，与分析器的 HashMap 迭代无关。探针连续运行 3 次，输出 md5 相同（步数、对象数、表、延迟清单）。终态验收加 `--hash-seed 0 / 12345` 与 `--flow-batch 1 / 64` 的组合矩阵，要求映像字节相同。
4. **平台**：映像键含平台。macOS 与 Linux 的 `sun.nio.fs` 实现类不同，各自产出映像；语料构建以服务器 Linux 为准。

## 4. 消费方需求（a3 余项，c1d §21.9，9ea1e102）

### 4.1 `VM.initLevel`（VM 10 个）

**第三条路成立。**
- 构建期执行 initPhase1–3，运行期从映像中的 level 4 状态开始，`initLevel()` 按字节码读 `VM.initLevel` 静态字段。探针实测映像值为 `I(4)`。
- a3 的两条路都不需要了：
  - 路①（字段读取钩子带返回值）：线程内档位覆盖只在「惰性执行引导段」时才需要。映像下不存在运行期的引导段，档位就是单一字段，与 HotSpot 完全一致。
  - 路②（急切启动序列）：闭包代价来自把 `initSystemClassLoader` 作为运行期根。映像下它在构建期执行，运行期闭包只含被 main 可达代码实际调用的加载器方法。
- `isBooted`、`isModuleSystemInited`、`isJavaLangInvokeInited` 在映像下都是读字段，可以删掉 `[facts.returns]` 中对应的事实。分析器从映像值直接得到常量，`initLevel` 为 4 时 `isBooted` 恒为真。

**前提**
1. 第 1–3 步完成：求值器引导模式、污点与重算、映像物化和装载。
2. 引导阶段中**所有**被手写替代的方法在求值器里都有字节码或 native 语义。探针已覆盖 HelloWorld 路径。
3. VM 10 个手写里的非读档位方法（`shutdown`、`isShutdown`、`getSavedProperty`、`latestUserDefinedLoader`、`isSystemDomainLoader`）回到字节码。它们读写的都是 initPhase1 填好的静态字段（`savedProps` 等），映像中已有。`latestUserDefinedLoader` 是 `ACC_NATIVE`，保留。

### 4.2 包 → 模块表与 jimage 相关项

映像可以物化下列表，表中的方法读这些表时全按字节码执行：

| # | 表（Java 层位置） | 探针实测 | 解锁的方法 |
|---|---|---|---|
| T1 | 各 `Module` 对象：`name`、`descriptor`、`loader`、`layer`、`reads`、`openPackages` / `exportedPackages` | 62 个模块 | Module 7、`Class.getModule` 2 |
| T2 | `ModuleLayer`（`System.bootLayer`、`nameToModule`、`cf`）与 `Configuration` | `bootLayer = ModuleLayer` | ModuleLayer 2；SecurityManager `<clinit>` 的 `ModuleLayer.boot()` 与 `addNonExportedPackages`，SecurityManager 随之移出 `[vm_boundary]` / `clinit_carried` |
| T3 | `BuiltinClassLoader.packageToModule`（静态 CHM）、各加载器的 `nameToModule` / `moduleToReader` | 771 个包 | ClassLoader 资源 6（L2）、BootLoader 2（`findResourceAsStream`、`getServicesCatalog`） |
| T4 | 各加载器的 `ServicesCatalog`（`initServices` 构建期执行） | 随 T1 | `getServicesCatalog`，删 `__boot_catalog` 和 `services_table` |
| T5 | VM 模块表：`defineModule0` / `addReads0` / `addExports0` / `addExportsToAll0` / `setBootLoaderUnnamedModule0` | 62 / 150 / 273 / 226 / 1 | 运行期 native `addReads0` 等的初始状态；`Class.forName(Module,…)` 的模块可见性 |
| T6 | 镜像 `Class.module`（HotSpot java.base 修补与 `create_mirror`） | 87 个镜像，按包表填写 | `Class.getModule` 不再手写 |
| T7 | `ClassLoaders` 三个内置加载器及其 `URLClassPath`（app class path 中的 URL 是 `defer_call` 占位，运行期回填） | `scl = AppClassLoader` | FS-C2 字段钩子删除 |

**jimage 读取器仍需要（boot-layer 第 5 步）**：模块**资源内容**（`currency.data`、`.class` 字节等）在运行期按需读取，映像只物化表和 `ModuleReference`，不嵌入资源字节。`SystemModuleReader` 本身是 T3 中的对象，在构建期建好；它的 `read` 方法在运行期翻译执行，并落到嵌入数据上的 `getNativeMap`。

### 4.3 JceSecurity（6 个）

**可以物化，但它不是引导阶段的内容**：HelloWorld 的 initPhase1–3 不初始化 JceSecurity。它属于用户程序可达时的构建期类初始化（C3 `build_time_init`），用同一个求值器；JceSecurity 6 个的归零 2026-10-08 起前移到第 5 步（§6，分支 boot-image-s5）。核对 JDK 21 的 `JceSecurity.<clinit>`：
- 它只建 CHM / IdentityHashMap / ReferenceQueue / WeakHashMap、`new URL("http://null.oracle.com/")`，再经 `JceSecurity$1` 调 `setupJurisdictionPolicies`；
- 后者读 `Security.getProperty("crypto.policy")`，`Security.<clinit>` 读 `${java.home}/conf/security/java.security`；
- 然后 NIO `isDirectory` / `newDirectoryStream("{default,exempt}_*.policy")` / `newInputStream`，解析出 `CryptoPermissions`；
- 没有 SecureRandom，也没有其他宿主熵。

**前提**
1. **java.home 只读树是构建期输入，不是宿主状态**。原生二进制的 java.home 是嵌入的伪树（`jdk_resources`），内容来自参考 JDK 发行版，与运行机器无关。
   - 求值器的 `UnixNativeDispatcher`（`stat0` / `lstat0` / `access0` / `opendir0` / `readdir0` / `closedir` / `open0`）和文件分派器的 read / size / close native，在清单里登记为「读嵌入树」；
   - 嵌入树的路径前缀等于钉值的 `java.home`；
   - 落在嵌入树以外的路径是宿主文件系统，按延迟值处理（该类转为运行期初始化）。

   运行期的同一组 native 读同一棵树（a3 X2 余项的终态做法），两侧一致。
2. 运行期不能改写 `java.security` 或 policy。原生二进制没有 `-Djava.security.properties`，与钉值属性同一口径（U1 / U14）。
3. `JceSecurity` 的 `verificationResults` / `verifyingProviders` 在构建期为空，可以进映像；`queue` 是 ReferenceQueue，在映像中为空队列，可以物化。

物化后，`JceSecurity` 的 6 个手写全部删除；`setupJurisdictionPolicies` 与 NIO 目录遍历代码不进运行期闭包（**如果**程序不在运行期再次调用它们）。

## 5. 探针实测（分支 boot-image，HelloWorld，本机 macOS JDK 21.0.11）

### 5.1 规模（累计值）

| 阶段 | 指令步数 | 堆对象 | 已初始化类 | 驻留字符串 | 镜像 | 耗时 |
|---|---:|---:|---:|---:|---:|---:|
| VM 预初始化（9 类 + 3 个 VM 对象） | 2,065 | 100 | 62 | — | — | — |
| `initPhase1` | 85,110 | 1,088 | 200 | 170 | 72 | 76–95 ms |
| `initPhase2(false,false)` → 0 | 1,608,125 | 10,775 | 273 | 1,278 | 86 | 129–153 ms |
| `initPhase3` | 1,609,459 | 10,805 | 274 | 1,280 | 87 | 129–153 ms |

- 映像可达：对象 8,356 个，槽位 87,479 个，静态字段 418 个，非数组类型 150 个；已取身份哈希的 87 个对象全部在映像中。
- 堆类型最多的几种：`[B` 1,675、String 1,476、`HashMap$Node` 1,012、`CHM$Node` 911、`SetN$SetNIterator` 554（垃圾，不可达）。
- VM 表：`defineModule0` 62（包 771）、`addReads0` 150、`addExports0` 273、`addExportsToAll0` 226、`setBootLoaderUnnamedModule0` 1；VM 单元 `next_thread_id = 1`。
- 结束状态：`VM.initLevel = 4`、`ClassLoader.scl = AppClassLoader`、`System.bootLayer = ModuleLayer`、`System.out = PrintStream`、`System.allowSecurityManager = 1`。
- 内存约 100 MB RSS，指令吞吐约 1,200 万步/s。

### 5.2 卡点目录（按碰到的次序）

| # | 卡点 | 探针处理 | 终态 |
|---|---|---|---|
| B1 | `sun.jnu.encoding` 决定 initPhase1 的 `Charset.isSupported` 分支 | 钉 UTF-8 | 运行期取宿主值（U1）：该分支段为残差区段，stdout / stderr 的 `newPrintStream` 为残差调用，`StaticProperty` 运行期初始化（§3.2 残差化） |
| B2 | ImmutableCollections SALT 取 `nanoTime` | `CDS.getRandomSeedForDumping` 给固定种子 | 同左（U2） |
| B3 | 初始线程组 / 线程须在 initPhase1 前由 VM 构造，当前线程在构造器之前就已设定 | 清单 `objects` 与 `current_thread` | 同左，运行期绑定 OS 主线程 |
| B4 | Finalizer 启动 | `isFinalizationEnabled = false` | 与运行期共享同一常量 |
| B5 | 线程 id 计数器是 native 地址（`getNextThreadIdOffset`） | VM 单元 | 物化为 native 初值 |
| B6 | macOS 的 `os.version` → OSVersion → ClassLoaderHelper → NativeLibraries | 级联转为运行期初始化（3 类） | 同左；Linux 映像无此链 |
| B7 | 空 `java.class.path` 即 cwd，`canonicalize0` / `user.dir` 是宿主值 | `toFileURL` 为 `defer_call` 占位 | 同左（U3） |
| B8 | `Thread.start0`（Reference Handler）、`setPriority0` | `defer` | 启动重放序列 |
| B9 | 子字 CAS 做字对齐地址运算 | 求值器布局覆盖 `unsafe_cax:B` / `S` | 同左 |
| B10 | 模块系统的 VM 表 | `vm_record` 计数 | 物化 T5（§4.2） |
| B11 | `maxMemory` / `availableProcessors` / `getAppend` / `Terminator.setup` / `initializeOSEnvironment` | `defer`（返回 0） | **前两项是值不是副作用**：`CHM.NCPU`、`Striped64.NCPU`、`Exchanger.NCPU`、`VM.directMemory` 直接派生，必须走污点与重算槽（§3.2），否则映像值错误；后三项是重放序列 |
| B12 | `Object.class.getModule()` 为 null，`Module.defineModules@564` 抛 NPE | `defineModule0` 登记包 → 模块，已有与新建镜像按包填写 `Class.module`；基本类型取首个定义的模块 | 同左（HotSpot `define_javabase_module` 语义） |

除此之外，探针在 initPhase1–3 中**没有**碰到缺字节码或缺 native 的失败。引导期清单共 128 条 native 登记（其中 35 条 noop，即 registerNatives / initIDs 等；Unsafe 读写与 CAS 54 条），详见 `vm_intrinsics.toml [concrete.boot]`。

### 5.3 第 1 步实测（U1 运行期取宿主值，分支 `boot-image-s1` 3e04048d）

Linux 为服务器作业 `bimg-aud2-3e04048d`（jp2）；macOS 为本机。四组合指 `--hash-seed` 0 / 12345 × `--flow-batch` 1 / 64。

| 项 | Linux JDK 21 | Linux JDK 25 | macOS JDK 21 | macOS JDK 25 |
|---|---|---|---|---|
| initPhase1–3 / initPhase2 | 完成 / I(0) | 完成 / I(0) | 完成 / I(0) | 完成 / I(0) |
| 未登记失败 | 0 | 0 | 0 | 0 |
| 摘要（四组合相同） | `03c8d112…a0a1` | `b2d2284a…0657` | `f819f982…7232` | `7e759918…f739` |
| 求值耗时 | 268 ms | 263 ms | 150 ms | 150 ms |
| `rava audit boot` 进程 RSS | 235 MB | 281 MB | 236 MB | 241 MB |
| 指令步数 | 1,600,400 | 1,726,599 | 1,602,115 | — |
| 映像可达对象 / 槽位 | 8,270 / 69,716 | 8,363 / 69,816 | 8,248 / 69,451 | 8,341 / 69,556 |
| 映像静态字段 / 非数组类型 | 358 / 138 | 363 / 110 | 354 / 137 | 359 / 109 |
| 构建期初始化类（其中运行期） | 252（1） | 222（1） | 249（2） | — （2） |
| HelloWorld 闭包（现状） | 468 | 428 | 469 | — |

**U1 的影响**

- **转为运行期初始化的类**：Linux 只有 `jdk/internal/util/StaticProperty` 1 个（它在 `<clinit>` 中读 java.home、user.dir、jnu 编码等，jnu 残差区段改写了属性表的 CHM 节点）。
  macOS 另有 `jdk/internal/loader/NativeLibraries`，经 `os.version → OSVersion → ClassLoaderHelper` 链转入；该链上的 2 个中间类在外层撤回时被吸收，不单独计数。
- **运行期部分（两平台、两版本相同）**：
  - 残差调用 2 个：`System.newPrintStream` ×2（stdout / stderr 编码）；
  - 残差区段 2 个：initPhase1 的 jnu 编码段 `[36, 70)`，initPhase3 的不支持编码告警段（21 为 `[367, 401)`，25 为 `[178, 212)`）；
  - 静态读取占位 3 个：`StaticProperty.JAVA_HOME` ×2、`USER_DIR`；
  - 启动重放 native 5 个：`Terminator.setup`、`VM.initializeOSEnvironment`、`toFileURL`（U3）、`Thread.setPriority0`、`Thread.start0`；
  - 宿主标量 3 个，留给第 2 步：`getAppend`、`availableProcessors`、`maxMemory`。
- **映像规模**：与钉值探针（8,356 个可达对象）相比为 8,270，减少约 1%。initPhase1 的 Charset 段在运行期执行，所以映像中没有该段建的编码对象；模块图（initPhase2）不受 U1 影响。
- **闭包上界**：上界 = 现状闭包 + 映像中不在闭包的类型 + 运行期部分新增的根。
  - 运行期部分入口类 6 个，其中只有 `java/lang/Terminator` 不在闭包；以它为根，macOS 实测闭包多 5 类。
  - 映像中不在闭包的类型：JDK 21 有 52 个（模块图、`ModuleDescriptor$*`、`AccessFlag$N`、URI 等），JDK 25 有 32 个。
  - 由此得到：Linux JDK 21 约为 468 + 5 + 52 = **≤ 525**，JDK 25 约为 428 + 5 + 32 = **≤ 465**。两者都低于 569 的目标，也低于 §0 的 540 估计。
  - 第 3 步抽象分析从映像出发，引导代码不再入链，实际值应低于这个上界。
- **第 2 步验收锁定**：运行期初始化类 Linux ≤ 1、macOS ≤ 2，只减不增。

### 5.4 第 2 步实测与验收（分支 `boot-image-s2`；3d808e77 暂停，d1dc540a 验收通过）

**暂停原因**：用户 10-06 定 C4 全量正确性优先，全量期间集成分支冻结语义改动，本线让出服务器。服务器作业未派发，无在途作业。

**已完成（3d808e77）**：
- 污点值 `CV::T`（`concrete/taint.rs`）：宿主源（清单 op `host_scalar:<下界>:<上界>`，缺省为返回类型全域，boolean 为 0..1）产生带区间的表达式；整数算术、移位、位运算、`lcmp`、窄化与扩宽按 i128 区间传播，溢出取全域，单点区间直接定值；除数区间含 0、浮点转换、其他具体取值仍按第 1 步残差化。清单改为 `availableProcessors` / `maxMemory` 取 `host_scalar:1:max`，`getAppend` 保持 `host_scalar`。
- 污点上的分支（`concrete/taint_fork.rs`）：区间可判定就按判定走；不可判定时，两侧各自无副作用地执行到直接后支配点（每侧 ≤ 4096 步，嵌套 ≤ 8 层），各侧细化分支操作数的区间，合并时不同的整数值成为选择表达式。路径中有调用、写入、分配或抛出时，撤回后按第 1 步残差化。
- 启动重算槽、污点审计、级联链与重放序列（`concrete/boot_slots.rs`）：
  - 重算槽：映像中持有污点的静态字段、实例字段和数组元素，按构建期写入次序列出；
  - 污点审计：重算槽与重放输入之外的污点为 0；重算表达式的实参不能是占位对象，否则构建失败；
  - 运行期初始化级联链：由原因串回溯上游类；
  - 启动重放序列：先重算槽，再按构建期次序执行残差记录。
  - 以上都写进报告和 JSON 的 `step2` 字段。
- U8 读后写审计（`concrete/war.rs`）：
  - 写入时刻：每次写入推进逻辑时钟，按位置（静态字段、实例字段、数组、VM 单元）记录；撤回时一并恢复。
  - 读集：读日志挂在日志标记上。残差调用、残差区段和运行期初始化登记时，捕获标记以来的读集。
  - 判定：读集中的位置若在登记之后被写过，构建失败。

**本机 macOS JDK 21 实测**（HelloWorld 档案键，`rava audit boot --jdk 21`）：

| 项 | 门槛 | 实测 |
|---|---|---|
| 结论 / initPhase1–3 | 通过 | 通过（摘要 `f1e75fdf4f1dfa83727395409ee0e89d`；第 1 步为 `f819f982…`，变化来自污点值入摘要） |
| 映像中污点值（重算槽与重放输入之外） | 0 | 0；不可独立重算 0；宿主标量取零值 0 |
| 重算槽 | `NCPU` / `directMemory` 等 | 5 个：`ConcurrentHashMap.NCPU`、`VM.directMemory`，以及 `FileDescriptor.in/out/err.append`（同一字段的 3 个实例槽） |
| 运行期初始化类 | macOS ≤ 2 | 2（`StaticProperty`、`NativeLibraries`），没有新增 |
| U8 交集 | 0 | 0（6 个残差，读集 156 个位置） |
| 求值耗时 | ≤ 0.5 s | 130 ms |

- 污点分支：HelloWorld 引导期间判定 0 次、合并 0 次。`NCPU` 只在扩容（`transfer`）和争用（`fullAddCount`）时参与分支，引导期间没有走到。合并路径只有单元测试 `decide_and_narrow` 覆盖区间判定和细化，两侧合并还没有实际例子。
- 与 §3.2 的偏差：§3.2 写的是「宿主值写入对象图即运行期初始化」。本步改为实例字段进重算槽（`FileDescriptor.append`），所以 `FileDescriptor` 仍在构建期初始化，运行期初始化数没有增加。污点分支也是在构建期合并，不转为运行期初始化。

**恢复与验收（2026-10-06，分支 `boot-image-s2` d1dc540a，已同步集成分支 9d34449b）**：

- 本机单测 `taint` / `concrete` 全过（含 `decide_and_narrow`）。
- 服务器首轮 `bimg2-aud-e69b596c`（us1）：Linux JDK 21 通过；**JDK 25 失败，U8 交集 3 项**，均为 `jdk/internal/util/Preconditions.SIOOBE_FORMATTER`，涉及 initPhase1 的 jnu 残差区段与两个 `newPrintStream` 残差调用。
  - 根因：审计口径过宽。JDK 25 的串代码在残差窗口内首次触发 `Preconditions` 初始化，窗口内先写后读 `SIOOBE_FORMATTER`。窗口撤回后，构建期在登记之后重新初始化 `Preconditions`，同一位置的写入时刻晚于登记时刻，于是被误报。
  - 修正（d1dc540a，`concrete/war.rs`）：读集只计窗口的**外部读**。窗口内先写后读的位置不计，包括窗口内触发的类初始化；窗口内分配的对象也不计，但类镜像由 VM 缓存持有，仍按已有对象处理。撤回到标记时压缩读写日志：撤回段只留外部读，写入作废，这样兄弟路径不会因被撤回路径的写入而漏报。
  - 健全性：运行期重放时，窗口内的类初始化因映像中该类已初始化而跳过，读到的是映像中同一 `<clinit>` 的结果。该 `<clinit>` 在窗口内的外部读取仍在读集内受审计；映像值若与窗口值不同，必然有某个读集位置在登记后被写，交集会报出。单测 `external_reads_drop_own_writes_and_fresh_objects` 覆盖。
- 服务器复验 `bimg2-aud-d1dc540a`（kr2，rc 0）：

| 项 | 门槛 | Linux JDK 21 | Linux JDK 25 | macOS JDK 21（本机） | macOS JDK 25（本机） |
|---|---|---|---|---|---|
| 结论 / initPhase1–3 | 通过 | 通过 | 通过 | 通过 | 通过 |
| 摘要（Linux 为 4 组合，均相同；macOS 为单次） | 相同 | `b9247cfc…c51d` | `aa61275e…96f7` | `f1e75fdf…e89d` | `dd8d4aeb…7cdc` |
| 映像中污点值（重算槽与重放输入之外） | 0 | 0 | 0 | 0 | 0 |
| 重算槽 | `NCPU` / `directMemory` 等 | 5 | 5 | 5 | 5 |
| 运行期初始化类 | Linux ≤ 1 / macOS ≤ 2 | 1（`StaticProperty`） | 1（`StaticProperty`） | 2 | 2 |
| U8 交集（残差 / 读集） | 0 | 0（5 / 114） | 0（5 / 112） | 0（6 / 115） | 0（6 / 113） |
| 求值耗时 | ≤ 0.5 s | 324 ms | 319 ms | 130 ms（暂停记录） | — |
| `rava audit boot` 进程 RSS | ≤ 300 MB | 238 MB | 254 MB | 239 MB | — |
| HelloWorld 闭包 | 不变 | 468 | 428 | — | — |

  - 重算槽 5 个为 `VM.directMemory`、`ConcurrentHashMap.NCPU`，以及 `FileDescriptor.in/out/err.append`（同一字段的 3 个实例槽）。按字段计为 3 个；§6 写的「4 个字段」为估计口径，以上表的字段 / 槽数为准，不另立决策项。
  - 读集从 155 / 156 降到 112–115，即窗口内自产位置约占 1/4。
- 服务器全量单测 `bimg2-ut-d1dc540a`（jp1，rc 0，4319 s）：generator 与 rava_macros_core 全过，0 失败（1 个 ignored 为既有）。
- 抽查 `bimg2-d1dc540a`（JDK 21，`--per-dir 0`）：HelloWorld、TestAppClassLoader 通过；TestModuleLayerDefine 运行期 NPE，与集成分支上的抽查（`c1de-sp-956db0b4`、`c1db3-212c9229` 等）同症状，不在 master_passed 中，属已知失败，验收在第 3 步。
  - 本步映像只求值、不物化，闭包类数与集成分支相同（HelloWorld 468），不影响生成产物。
  - TestBootLayer 在 `tests/e2e` 中不存在，是第 3 步验收要新增的用例。
  - TestClassModuleFace（`jbase-named=false`）由第 4 步处理，与本步无关。

### 5.5 第 3 步实测（分支 `boot-image-s3`，基于 c5812d89）

#### 5.5.1 §7 第 3 条核对：映像类型中的手写 struct（动手前，本机 macOS JDK 21 HelloWorld 档案键）

审计报告新增「映像可达对象的类型」「映像可达 lambda 对象」两节（`rava audit boot`）。实测映像可达类型 162 种（非数组 137、数组 25），lambda 对象 **0**。

**结论：映像类型中没有手写的 Java 类 struct**。137 个非数组类型的存储（`X__inner`）全部由字节码经 `java_class!` 生成；例外只有两类基础设施，均为常驻形态而非过渡类：

| 类型 | 运行期形态 | 物化处理 |
|---|---|---|
| `java/lang/Object` 实例（44，锁对象） | 手写 `object_impl.rs` 的 `Instance(u8)` | 运行时给出常量构造入口，映像按同一类型发射 |
| 数组（25 种，2,017 个） | `JArray<T>` → `__Obj<__ArrayObj<T>>` + 尾随元素 | 运行时给出「头 + 数组对象 + 定长元素」的 `#[repr(C)]` 映像形态，与 `__trailing` 的布局一致（编译期断言） |

但有 9 处**结构之外的手写旁路状态**与映像对象并存，物化时必须让旁路以映像为初值，否则同一 Java 对象在运行期出现两份：

| # | 类型（映像对象数） | 手写旁路状态 | 第 3 步处理 |
|---|---|---|---|
| S1 | `Class`（78） | `Class::for_class` 的名字 → 镜像缓存、`getPrimitiveClass` 缓存 | 两处缓存先查映像镜像表（按名排序的静态表，二分），未命中才新建 |
| S2 | `String`（1,435） | `__STRING_INTERN_TABLE` | 驻留查找先查映像驻留表（按 UTF-16 单元排序的静态表） |
| S3 | `Thread`（1）/ `ThreadGroup`（2） | `thread_impl` 的主线程构造（`platform_main_thread`）、`INITIAL_THREAD` | 启动时把映像中的 main 线程绑定到 OS 主线程（§3.3 启动序列第 2 步），不再手写构造 |
| S4 | `Thread` id | `getNextThreadIdOffset` 的进程静态计数器（初值 0） | 初值取映像 VM 单元 `next_thread_id` |
| S5 | `Module`（68）/ `ModuleLayer`（2） | `module_impl` 的 VM 模块表（`defineModule0` 运行期登记）、`unnamed_module` / `ModuleLayer.boot` 的手写单例 | VM 模块表以映像 VM 表（62 模块 / 771 包，含读与导出）为初值；两个手写单例属 a3 第 4 步删除的 `#[jvm_boundary]`，本步不改其语义，只保证映像对象与之不冲突（见 §5.5.3 余项） ；**e50d4ba3 起**两个手写单例已删，VM 模块表由映像 `modules` 登记初值，`Class.module` 经 `__vm_module` 钩子查表（§5.5.6） |
| S6 | `System` | `registerNatives` 手写建属性表与 out / err / in | `System` 构建期已初始化，运行期不再执行其 `<clinit>`；该手写体只在映像缺席时可达 |
| S7 | `ClassLoader.scl`、`Thread.contextClassLoader` | `[vm_state.field_hooks]` → `__vm_init_phase3`（FS-C2） | 映像带构建期 initPhase3 写入的值，钩子与 `__vm_init_phase3` 删除 |
| S8 | 身份哈希（87 个对象） | `__identity_hash(地址)` | 映像对象的对象头记录构建期哈希，`__identity_hash` 对映像区地址返回该值（§5.5.2 D3） |
| S9 | 枚举常量目录（`Thread$State`、`AccessFlag` 等构建期初始化的枚举） | 登记语句在 `__class_init` 慢路径内执行 | 构建期初始化类的初始化状态物化为「已完成」，慢路径不再执行，登记改由启动序列对这些类执行一次（与 `<clinit>` 前登记的次序语义相同：查询时才读静态字段） |

运行期对映像对象的**宿主相关内容**：`@deferred` 属性值（`java.home`、编码、`user.dir` 等 13 个平台属性与 2 个 VM 属性）在映像中是内容数组被登记为延迟值的 String。物化时这些 String 的内容字段（`value` / `coder` / `hash`）在启动序列中按宿主值写入（同一对象，属性表、`System.lineSeparator` 等引用它的位置自动看到宿主值）。

#### 5.5.2 物化设计（技术决策，按授权自定）

- **D1 映像位置**：`java_base_decl` crate 的 `boot_image` 模块（U5 的「java_base 档案内」取声明层：映像类型的 `X__inner` 都在该 crate，`pub(crate)` 字段可直接按名初始化，无需每类再生成构造器）。
- **D2 零拷贝形态（U4）**：全部映像对象是**一个** `#[repr(C)]` 静态结构 `BOOT_IMAGE` 的字段，每个对象为「16 字节对象头 + 值（+ 数组定长元素）」，与 `__Obj` 堆布局相同。
  - 对象头的强引用计数取常驻值（`1 << 62`）：克隆 / 释放照常增减而永不归零，不需要静态标记位，`is_unique` 恒假。
  - 对象之间、静态字段到对象的引用都是编译期常量地址（静态结构可引用自身字段）。句柄、视图指针、接口指针由运行时新增的 `const fn` 入口构造（`__Obj::image`、`__Handle::image`、`__Ref::image`、`__IfaceRef::image`、`__PrimCell::from_bits`），视图指针在常量求值期由 unsize 得到。
  - 装载成本为 0（无启动扫描），满足「启动装载 ≤ 1 ms」。
- **D3 身份哈希**：映像对象头的第二个字（堆对象为分配字节数）写「标记位 | 构建期哈希」。`BOOT_IMAGE` 是单个静态，地址区间判定即可知是否映像对象，`__identity_hash` 先做区间判定（两次比较），命中读头。这避免了 §7 第 5 条的每对象多一个字。
- **D4 静态字段与初始化状态**：生成器给构建期初始化类的静态字段带上映像初值（`java_class!` 静态字段属性），宏展开为 `static __STATIC_X_f: … = <映像初值>`；`__CLINIT_STATE_X` 初值为 3（完成）。运行期初始化类（`StaticProperty` 等）保持现状。
- **D5 残差区段**：区段 `[s, e)` 由生成器切出为合成静态方法（字节码拷贝 + `return`，入口局部变量作形参、按槽位布局补占位形参），按普通方法翻译；启动序列以映像中记录的局部值调用。残差调用、重放 native、回填、重算槽按 `step2` 的启动重放序列次序生成为一个启动函数，取代 `vm_boot_init` 的 `calls` / `classes` / `phases`。
- **D6 抽象分析从映像出发**：引擎不再以 `[boot_init]` 与引导阶段为根。构建期初始化类的 `<clinit>` 不入链；被闭包读取的静态字段，其抽象值以映像对象（每个映像对象一个分配点 `image:#n`，字段值按映像具体值）为初值；映像对象的类型随其被读取而入实例化集合。只物化从「闭包内被读取的静态字段」与启动序列可达的映像对象（联合不动点，单调）。

- **D7 档位上下文**（`engine/levels_boot.rs`）：映像导出时把引导档位（`VM.initLevel`，清单 `[concrete.boot] level`）的变化记为启动步骤 `IStep::Level`。残差调用与残差区段在其构建期档位的克隆上下文 `@level:L` 中分析；该上下文中的方法按 `[concrete.boot.level_queries]`（`VM.isBooted` ≥ 4、`VM.isModuleSystemInited` ≥ 2）折叠引导查询，不与本体共享摘要，派发枢纽按档位分族，触发的 `<clinit>` 也在档位上下文中登记。档位只沿直接调用（溯源类别 `invoke`，被调方的普通上下文为 NOCTX 或已是档位上下文）传播。实测：若沿全部同步类别（派发、lambda、反射……）传播，档位克隆失控（23 GB、200 万调用点），所以收窄到直接调用。手写方法不克隆，回调回到本体（已知局限）。档位 ≥ 全部门限时不建上下文。
- **D8 映像对象的分配点**：映像数组逐对象建分配点（`<类型>@image<n>`），容器形状类（与 `obj_at` 同一判定）的映像实例也逐对象建分配点，字段值进该对象的字段节点（`Node::O`）。其他实例按确切类型代表，与程序新建对象合流。逐对象是为了避免各映像数组、各容器的元素互相混合（模块图里有上千个数组）。
- **D9 残差调用的实参**：残差调用的被调方法，其形参取构建期记录的实参，不再 open。标量与 null 进常量格；字符串对象（非占位、非延迟，且内容数组非延迟——宿主属性值的延迟标记落在 String 的 `value` 数组上，启动序列按宿主值改写其内容）进字符串常量，内容延迟的字符串只取「非空引用」、不取字段标签；其他映像对象记为「带标签的非空引用」，标签是它 final 实例字段中的标量 / 字符串常量，与构造器摘要同一口径。引用实参的映像对象经 `image_ref` 流入形参节点。静态字段的映像初值同样走这套常量格（`image_pv`）。重放 native 与残差区段内的调用目标仍按 open 形参作根。
- **D10 数组读取按静态类型收窄来源**（`engine/bytecode.rs`，通用精度修正）：aaload 的数组来源集先按数组值的静态类型过滤（checkcast / 声明类型），再判定是否含非数组值。原实现中，链表式 `Object[]`（如 `PreHashedMap.put` 的 `a = (Object[]) a[2]`）的来源集带有同数组其他元素（String），被当成 open 数组，于是注入 `open(Object)`。

#### 5.5.3 进展与恢复入口

**进展（2026-10-06 15:30，分支 `boot-image-s3`）**

- 已提交：aaafd123（运行时映像常量构造原语）、ffdc14a5（§5.5.1 核对与物化设计）。
- 本次提交为分析侧，从映像出发：
  - `image.rs`：映像数据形态，规范编号，启动步骤含 Level；
  - `concrete/export.rs`：解释器堆导出；
  - `engine/image_start.rs`：装载映像。构建期初始化类不展开 `<clinit>`，活对象联合不动点，占位对象取来源，残差步骤作根；
  - `engine/levels_boot.rs`：D7；
  - D8–D10；
  - 去掉 `[boot_init]` 根的调用条件：映像求值成功时不再以 `boot_init` 为根。
- 尚未做：
  - 发射侧物化（D1–D5）、启动重放函数；
  - 删除 `[boot_init] calls / phases` 与 FS-C2 钩子；
  - TestBootLayer e2e。
  - 运行时 `vm_impl` 的档位重放补丁草稿（`__vm_at_init_level`）未入库。

**实测（本机 macOS JDK 21，HelloWorld）**

| 口径 | 类数 |
|---|---|
| 不从映像出发（现状） | 469 |
| 从映像出发，D7–D10 全开 | **2,986**（RSS 2.3 GB，约 40 s） |
| 跳过全部映像启动步骤（只装映像） | 366 |
| 只跳过残差调用与残差区段 | 401 |

硬门槛「≤ 540」**未达成**。超出部分全部来自运行期部分的根，链路如下：

1. **残差区段** initPhase1 `[36, 70)`：`Charset.isSupported(sun.jnu.encoding)`，参数是宿主值（U1，Linux 上取自区域环境变量）。
   - 调用链为 `Charset.lookup2` → `StandardCharsets.lookup`（名字不定）→ `Class.forName(...).newInstance()`。
   - 这是 JDK 运行期真实会走的路径。现状不走，是因为手写 `registerNatives` 不执行 initPhase1。
2. `Class.newInstance` → `getConstructor0`，异常消息分支走 `methodToString` → `Arrays.stream` → `StreamOpFlag.<clinit>` → `EnumMap` → `getEnumConstantsShared` → `Method.invoke`。
3. `Method.invoke` → `isCallerSensitive` → `isAnnotationPresent` → 注解解析 → 建 `Proxy`。
4. `Proxy$Dyn` 的 VM 钩子以 open 实参派发全部代理方法，到 `AnnotationInvocationHandler.equalsImpl` → `Objects.equals(open, open)`。
5. open `equals` 派发到全部已实例化类型，其中 `URL.equals` 来自 `toFileURL` 重放。
6. 由此展开：`InetAddress` → `ServiceLoader` → 类路径 / jar / 文件系统 / 安全……

反事实切除（`--cut`）逐个切掉以下各项，结果都仍是 2,986 类：

- `Charset.isSupported`
- `toFileURL`
- `Proxy$Dyn.__vm_proxy_invoke`
- `Class.getConstructor0@76`

原因是名字不定的 `StandardCharsets.lookup` 有三条独立入口：jnu 区段，以及 stdout / stderr 两个 `newPrintStream` 残差调用。后两者的编码名是延迟值，按 U1 在运行期取宿主值，所以 `Charset.forName(enc, …)` 的名字也不定。open `equals` 的来源同样不止一处。因此压闭包须在通用路径上做精度，不能切单根。

D9 消掉了 `newPrintStream` 一支：形参 open 时，编码名可为 null → `Charset.defaultCharset` → 同一处 `StandardCharsets.lookup`。D10 消掉了 `PreHashedMap.put` 的 open 注入。剩余放大点是 2–4，属于通用精度问题，不是映像特有：任何以非常量名调 `Charset.forName` 的程序同样会碰到。

**补测（16:20，64075e73，同口径反事实 `--cut`，不健全、只作归因）**

| 切除 | 类数 |
|---|---|
| 无（64075e73 现状，RSS 2.0 GB、50 s） | 2,986 |
| `sun/nio/cs/StandardCharsets.lookup(String)` 整个方法 | **401** |
| 同上 + `Proxy$Dyn.__vm_proxy_invoke` | 401 |
| 只切 `Class.newInstance()` | **459**（≤ 540） |
| 只切 `Class.methodToString`（异常消息支） | 2,986 |

结论修正：放大点只有一个——`StandardCharsets.lookup` 以不定名字走到 `Class.forName(…).newInstance()`（lookup@122/125），名字不定的三条入口（jnu 区段、stdout / stderr 编码）都汇到这里。`Class.newInstance` 本身让 `Constructor` 进入实例化集合，此后任何以 open 实参调用 `String.valueOf(Object)` / `StringBuilder.append(Object)` 的点都会派发到 `Constructor.toString` → `Executable.sharedToString` → `Arrays.stream` → `StreamOpFlag.<clinit>` → `EnumMap` → `Method.invoke` → 注解 → `Proxy` → open `equals` → `URL.equals` → `InetAddress`……。实测入口之一是 `Terminator.setup` → `Signal.handle@71` 的字符串拼接（映像残差根），所以单切异常消息支不够。

因此压闭包的终态方向（按收益）：
- (B) 先做：`String.valueOf(Object)` / `append(Object)` 等的实参按调用点区分（上下文敏感或按调用点克隆）。`Signal.handle@71` 的拼接实参只有 `Signal` / 处理器，不应派发到 `Constructor.toString`。`Class.newInstance` 经 `getConstructor0` → `copyConstructor` 真实分配 `Constructor`，`Constructor` 进入实例化集合本身是正确的，放大来自 open 实参的 `toString` 派发。
  - 核对（javap）：`Signal.handle@71` 拼接的是 `sig`（`Signal`），`Terminator.setup` 传入的是 `new Signal("HUP" / "INT" / "TERM")`。`--why` 只给首次发现路径，`StringBuilder.append(Object)` 的形参在全部调用点之间合流；`Constructor` 是否在别处被真实拼接（如反射异常消息）尚未核实。如果有真实拼接，`Constructor.toString` → 流 → `EnumMap` → `Method.invoke` 是 JDK 真实可达路径，压缩须落在其后的 `isCallerSensitive` 注解查询（原 (c)）与 `Proxy$Dyn` open 派发（原 (a)）上，且两者要同时做（单切 `__vm_proxy_invoke` 仍为 2,986）。下一步先用 `--flows` 查 `StringBuilder.append(Object)` 形参中 `Constructor` 的来源点。
- (A) 辅助：`lookup` 的类名集合。`classMap()` 的值是映像中的字符串常量（D8 逐对象容器），`"sun.nio.cs." + cln` 应得有限名字集，`Class.forName` 解析为有限类集。只做 (A) 不能消除 `Constructor` 的分配。
- 原 (a)–(c) 降级：单独做都不改变 2,986。

**收口（18:05，到 6 h 上限停止）**

- 分支 `boot-image-s3` 提交：aaafd123、ffdc14a5、67550efc、d8c62216、deb82e0d、64075e73（D9 健全性：内容延迟的字符串不折叠），以及 90cadb84、b2594f89、ca3393db、d6e932c7（文档）。
- 服务器单测：bimg3-ut-d8c62216 被 bimg3-ut-d6e932c7 取代，旧作业已停。bimg3-ut-d6e932c7（jp2，16:20 起）停止时仍在跑（与 C4 全量并行，C4 独占服务器），结果见 `server_maintenance/rava/test_results/job/bimg3-ut-d6e932c7/`。
- 审计 / 抽查 / JDK 25 / TestBootLayer 未做：发射侧未完成，且 C4 全量期间不发新服务器作业。

**恢复入口**

1. 先压闭包（硬门槛）。先按上面的 (A) / (B) 做；以下为原候选，按补测已降级：
   - (a) `Proxy$Dyn` VM 钩子按代理实际接口与调用点派发，不再以 open 实参派发全部方法；
   - (b) `Class.newInstance` / `getConstructor0` 异常消息分支的冷路径；
   - (c) `Method.invoke` 的 `isCallerSensitive` 注解查询，按 `@CallerSensitive` 的静态事实折叠。
   - 每做一项，都以 `rava closure tests/e2e/01_basics/HelloWorld.java -o … --why <类>` 复测。
   - `--flows "@openorig:<类型>|<节点>"` 与 `@grow:` 可定位 open 注入点。
2. 再做发射侧 D1–D5 与启动重放函数。
3. 最后删 `[boot_init] calls / phases`、FS-C2 钩子，加 TestBootLayer，跑服务器单测 / 审计 / 抽查。
4. 确定性：64075e73 上 HelloWorld `--hash-seed` 0 / 12345 × `--flow-batch` 1 / 64 四组合已复验（本机 macOS JDK 21）。映像数据（`boot_image_data`）逐字节一致；类集合（2,986）、方法集合（18,008）、折叠计数、实例化 1,991 全部一致，差异只在 via 与内部计数 `method_contexts` / `context_objects`（±0.1%）。JDK 25 尚未复验。
5. 本分支在发射侧完成前不可合入：分析已从映像出发，但映像尚未物化。

#### 5.5.4 压闭包：核实结论与两处精度（2026-10-06 晚，分支 `boot-image-s3`）

**§5.5.3 待核实项的结论：`Constructor` 确实被真实拼接**

- `--flows "@path:P1 java/lang/StringBuilder.append:(Ljava/lang/Object;)Ljava/lang/StringBuilder;|java/lang/reflect/Constructor@…"` 查得，`append(Object)` 形参中 `Constructor` 的唯一来源是 `AccessibleObject.throwInaccessibleObjectException`（下称 TIOE）@33 拼接的 `this`。
- 这是 JDK 真实代码：`Class.newInstance@72` → `doPrivileged` → `Class$1.run` → `Constructor.setAccessible` → `checkCanSetAccessible(Class,Class,Z)` 的拒绝分支。
- `Signal.handle@71` 只因 `append(Object)` 的 P1 在各调用点间合流才出现在 `--why` 里，它本身不拼接 `Constructor`。
- 因此 (B)（按调用点区分 `append` / `valueOf` 实参）**对规模无效**，已撤销。

**真正的闸门：两条冷异常消息支**

两条支路都通向 `Arrays.stream` → `StreamOpFlag.<clinit>` → `EnumMap` → `getEnumConstantsShared` → open `Method.invoke` → `isCallerSensitive` → 注解 → `Proxy` → open `equals` → ……

1. `Class.getConstructor0@76` → `methodToString`（NoSuchMethodException 消息）。参数类型数组非空时走 stream。
2. TIOE → `Constructor.toString` → `Executable.sharedToString`，无条件使用 stream。

反事实切除实测（`--cut`，不健全，只作归因，基于 64075e73，原值 2,986 类 / 18,008 方法）：

| 切除 | 类 / 方法 |
|---|---|
| `Constructor.toString` | 2,986 |
| TIOE | 2,985 |
| `Method.invoke` | 585 / 2,111 |
| `Reflection.isCallerSensitive` | 756 / 3,855 |
| `StreamOpFlag.<clinit>` | 2,978 |
| `Class.getEnumConstantsShared` | 579 |
| `StandardCharsets.lookup` | 401 |
| `methodToString` + TIOE | **524 / 1,926** |
| `methodToString` + `Constructor.toString` | 525 |

**两处通用精度（健全，替代切除）**

- **X1 c459845f：常量长度数组标签。**
  - absint 的 `Obj::Len(n)`：`newarray` / `anewarray` 的常量长度随引用标签经局部变量、形参、字段、返回值传播，`arraylength` 折叠为常量。
  - 依据：数组长度不可变（JVMS §2.7）。
  - `methodToString` 只经 `getConstructor0` 被 `newInstance@54` 以 `new Class[0]` 调用，`argTypes.length == 0` 折叠后剪掉 stream 支。
  - 单独做 X1 再反事实切 TIOE，结果为 524。
- **X2 7f7c4201：TIOE 不可达。**
  - 判定条件：`caller == null`（@14 / @40）；`callerModule == declaringModule`（@62），以及 `callerModule == Object.class.getModule()`（@74）。分三处修改：
  - **调用者类镜像结果非空。** 清单 `caller_class` 的结果按非空引用答复（`CallInfo.nonnull_ret`）。依据：`reflection_impl.rs` 的 `getCallerClass` 运行期恒有调用者或回退根类，`vm_intrinsics.toml` 已补注。
  - **形参常量格接受「非空引用」**（`bind_params` 用 `PV::of_ret`）。被调方法入口按形参序号换来源，并按描述符补类型（`absint::entry_state`）。非空性由此可以经形参传到 `checkCanSetAccessible`。
  - **引导单例**：新清单 `[vm_state] boot_singletons`，现只登记 `Class.getModule`。依据：`class_impl.rs` 中引导类共用同一模块单例。
    - 接收者是类字面量，或 Class 形参镜像值集全为引导类时，结果带 `Obj::BootSingleton(m)` 标签。
    - 同标签的两个值 `if_acmp` 折叠为相等（`ref_eq`）。
    - 形参值集上的答复记入 `mirror_field_assumed`，失效条件与接收者钩子字段相同：值集新增非引导类镜像、所指未知的 Class 对象或 open。

**结果（本机 macOS JDK 21，HelloWorld，`rava closure`）**

| 口径 | 类 | 方法 |
|---|---|---|
| 64075e73（§5.5.3） | 2,986 | 18,008 |
| X1 | 2,985 | 18,002 |
| X1 + 调用者非空 / 形参非空 | 2,983 | 17,998 |
| X1 + X2 | **524** | **1,921** |

- 硬门槛 ≤ 540 已达成。类集合与反事实「`methodToString` + TIOE」完全相同。
- 剩余构成：`sun/nio/cs` 104 个，来自 `StandardCharsets.lookup` 名字不定（U1 宿主编码），属于合法可达。(A)（lookup 有限类集）实测已由 D8 逐对象容器给出 classMap 有限值集，不再单列。
- 确定性：`--hash-seed` 0 / 12345 × `--flow-batch` 1 / 64 四组合下，类集合、方法 id + kind 集合，以及 summary 中除耗时外的全部字段逐项一致。

**恢复入口（接 §5.5.3 第 2 条起）**

1. 发射侧物化 D1–D5 与启动重放函数（设计见 §5.5.2）。
   - 运行时 `vm_impl` 档位重放补丁草稿仍在 `/tmp/bimg3_vm_impl_level.patch`（`isBooted` / `isModuleSystemInited` 取 `initLevel() >= 4 / 2`），随发射侧一起入库。
2. 删除 `[boot_init] calls / phases` 与 FS-C2 钩子，补 TestBootLayer。C4 全量结束后再发服务器单测 / 审计 / 抽查，并复验 JDK 25。
3. 本分支在发射侧完成前仍不可合入。
4. 本机验证范围：closure 库单测 171 个全过（含 `const_length_array_folds_arraylength`、`boot_singleton_results_compare_equal`）。driver `closure_cli` 中，小用例 5 个已过。
   - 大闭包用例 `closure_independent_of_hash_seed` / `_large` / `closure_independent_of_order` 涉及 JNDI / DeepCopy / Serial 族，每个 rava 约 1.9 GB、16–20 分钟。按巡检要求，这些在本机中止。
   - C4 冻结解除后上服务器补跑，与第 2 条的服务器单测一并进行。
   - 服务器作业 bimg3-ut-d6e932c7 在收口时仍显示 running，日志为空。

#### 5.5.5 发射侧物化与启动序列（2026-10-06 夜，分支 `boot-image-s3`）

**提交**

- 5d4b2769：发射侧物化映像区（`java_base/src/boot_image.rs`：`#[repr(C)] __BootImage` / `BOOT_IMAGE` 常量）与启动函数 `__boot_image_start()`（`main.rs` 在 `vm_boot_init` 前调用）；删除 `[boot_init] calls / phases`（闭包与清单两侧）；运行时映像原语（`image_rt.rs`、数组 / Unsafe 重定位、`SystemProps$Raw` 宿主表）。
- 2fe9e265：S3 映像初始线程绑定（`ImageData.current_thread` → `rt::bind_initial_thread`）；删除 FS-C2 钩子（`ClassLoader.__vm_init_phase3`、`scl` / `contextClassLoader` 字段钩子）；补 4 个 native：`Reference.getAndClearReferencePendingList`（恒 null）/ `waitForReferencePendingList`（永久阻塞），`Signal.findSignal0` / `handle0` / `raise0`（自管道 + 「Signal Dispatcher」守护系统线程回调 `Signal.dispatch`）。
- 29338223：`tests/e2e/62_reflection/TestBootLayer.java` 与期望输出（参考 JDK 21.0.11+10 实跑，37 行）。本机未跑。

**启动序列（`__boot_image_start`）**：类注册 → VM 单元 → 绑定初始线程 → 引用链接（接口视图 / 数组视图 / 类镜像 / 占位对象槽）→ 宿主改写（`SystemProps$Raw` 两表）→ 字符串驻留 → 静态字段（`__si_set_*`）→ `__boot_initialized()` → 残差步骤 → `set_level(None)`。

**本机实测（macOS，参考 JDK，HelloWorld `--stop-after emit`）**

| 口径 | JDK 21 | JDK 25 |
|---|---|---|
| 映像类 | 528 | 498 |
| 映像对象 | 614 | 618 |
| 映像区字节 | 280,837 | 287,221 |

- emit 阶段 4.78 s / RSS 445 MB（含 rava 增量构建的整条命令 50 s）。
- `[precheck] native-missing` 中 Reference / Signal 4 项已消；余 13 项为档案内 apple KeychainStore / pkcs11 / jimage / HostLocaleProviderAdapter，不在启动路径。
- `scripts/seed_check.sh HelloWorld`：seed-diff-lines = 0（2fe9e265 后复验）。
- 单测（按名过滤）：closure `image` 1、emit `boot_image` 3、`jdk_literal_lint` 2，全过；`cargo check --workspace --tests --release` 无警告。运行时 crate 本机不编译。

**测量方法（服务器执行，不新增环境变量）**

- 二进制体积：同一档案、同一 profile（缺省档）下，release 二进制字节数与集成分支基线对比，门槛 ≤ +5%。分项用 `size -A <bin>` 看 `.rodata` / `.data`，用 `nm --size-sort -S <bin> | grep BOOT_IMAGE` 看映像区本身。
- 启动装载：Linux uprobes——`perf probe -x <bin> __boot_image_start` 与 `__boot_image_start%return`，`perf record -e probe_<bin>:* -- <bin>` 取两事件时间戳差，门槛 ≤ 1 ms。辅以 `hyperfine -w 3 '<bin>'` 看总启动，与基线对比。符号须保留（测量用未 strip 的同构建产物）。

**未完成 / 恢复入口**

1. 服务器验证（C4 全量结束后）：closure_cli 三个大用例（`closure_independent_of_hash_seed` / `_large` / `closure_independent_of_order`）；生成器全量单测、审计、抽查、JDK 25；e2e HelloWorld、TestAppClassLoader、TestModuleLayerDefine、TestBootLayer、TestThreadContextLoaderInit；体积与启动两项门槛。运行时改动（`image_rt.rs`、`thread_impl.rs`、`signal_impl.rs`、`reference_impl.rs`）未经本机编译，首轮服务器编译即其编译检查。
2. D5 残差区段未物化：HelloWorld 两段都是 jnu 编码不受支持路径（JDK 21 initPhase1 `[36,70)`、initPhase3 `[367,401)`；JDK 25 `[178,212)`），UTF-8 宿主上为空操作。终态做法：由解码后的指令（`classfile::Code` 的 insns + 异常表）合成一个静态方法，类型由 `sim` 推断（无 StackMapTable），分析器登记入档案、发射器照常翻译，启动序列以 Call 步骤调用。
3. S6：手写 `System.out` / `err` / `in` / `lineSeparator` 访问器仍为运行时侧状态，映像中对应静态字段由 `static_setter` 跳过；终态随 System 手写收窄一并改为取映像值。

**需用户决策（技术取舍，已按授权先行实现）**

1. D4 偏离：静态字段在启动时经 setter 写入，不是常量初值。
2. D1 偏离：映像区放在门面 `java_base` crate，不放 decl 层。
3. 启动时有引用链接与宿主改写，装载开销非零（U4「零拷贝」的偏离）。
4. 映像使存活集变大（映像对象可达的类与方法入档案）。
5. 宿主值为 null 时保留构建期值。
6. 接口视图 / 数组视图 / 其他形态槽与类镜像在启动时链接，不进常量。
7. S5 模块身份：映像中的 Module 对象即运行期模块单例。
8. 映像中出现非生成类型即发射错误（不降级）。
9. FS-C2 已删：映像缺失时系统类加载器初始化随之缺失，映像事实上为必需。
10. D5 未物化（见上）。
11. 重定位识别按值域判定（指针值落在宿主地址区间）。
12. `vmProperties` 的键序在运行时 native 中重复一份，与 `[concrete.boot] vm_props` 须同步。
13. 宿主 native 限零参静态方法。
14. Reference Handler 与 Signal Dispatcher 现为真实 OS 线程（守护，不阻止退出）。
15. S6 手写标准流仍为侧状态（见上）。

#### 5.5.6 服务器跑通与第 4 步、第 5 步（部分）（2026-10-07，分支 `boot-image-s3`）

**跑通（服务器 dev，JDK 21）**：1ecdcb51（泛型 wrapper `__phantom` 按字段推断）、0e540b1c（映像存储类型 `X__inner` 按实现层路径）、6c344f81（宿主改写 null 判定走 `is_jvm_null`，不活对象的重定位 / 重算写入跳过）、bad5901d（基本类型镜像按描述符字符还原：映像 `IObj.mirror` 的基本类型为描述符字符，原按类型名匹配，`byte.class` 退化为类 `B`，`Unsafe.allocateUninitializedArray` 抛 `Component type is not primitive`）。

**第 4 步（模块部分）**

- e50d4ba3：`Module` / `ModuleLayer` / `Class.getModule` 回到字节码。
  - 删 `module_impl.rs` 的 `unnamed_module` 单例与 7 个 `#[jvm_boundary]`、`module_layer_impl.rs` 整个文件（`boot` / `parents` / `servicesCatalog` 钩子）、`class_impl.rs` 的手写 `getModule`。
  - VM 模块表以映像为初值：求值器的 `defineModule0`（`vm_record`）同时记 `vm.modules`（模块、定义加载器、open、位置、包），导出为 `ImageData.modules`；启动序列在字符串驻留后逐条 `rt::define_module` 登记（`Module::__vm_register`）。
  - `Class.module` 改为接收者字段钩子 `Class.__vm_module`：按「定义加载器 + 包」查 VM 模块表，未登记的包归定义加载器的无名模块（HotSpot 同）。分析侧钩子池由 `image_module_table` 以映像模块对象喂入。
  - 清单 `[vm_state.field_hooks]` 新增 `boot_noop`：`classLoader` 钩子对引导类为空操作，`module` 钩子对每个镜像都需要。
  - 删 `[vm_state] boot_singletons`（§5.5.3 X2 的「引导单例」）：`Class.getModule` 不再是手写单例，`callerModule == declaringModule` 不再按单例折叠。闭包规模影响待测（见下「未决」）。
- fcc54fb8：`Module` / `ModuleLayer` 移出 `closure.toml [vm_boundary]`，`Module` 移出 `clinit_carried`，删 `translate_nested` 的 `Module$EnableNativeAccess`。原因：嵌套类 `Module$ReflectionData` 随外层归入 `clinit_carried`，静态访问器发为存根（`stub: java/lang/Module$ReflectionData.exports`）。手写只剩 ACC_NATIVE（`defineModule0` / `addReads0` / `addExports*`）。

**第 5 步（部分）**：44be328e，`BootLoader.getSystemPackageLocation` native——引导加载器的包按 VM 模块表取所属模块的 location（映像模块表带 `defineModule0` 的 location 实参，引导层为 `jrt:/<模块名>`），未登记 → null。

**抽查 bimg3-m-fcc54fb8（JDK 21）**：8 例 7 过——HelloWorld、TestAppClassLoader、TestModuleLayerDefine、TestClassModuleFace、TestSetAccessibleBoundary、TestProtectionDomainFaces、TestStringGetCharsLegacy 通过（后 5 例已从 `docs/known_failures.toml` 删除）；TestBootLayer 运行期 NPE（见未决 1）。宽抽查 bimg3-w-f442cecf：CollectorsDemo、DeepCopy、TestSerialUserGenericCallbacks 通过，TestBootLayer 同上。

**未决**

1. TestBootLayer（第 3 步验收项）：作业 bimg3-bl-fcc54fb8 实跑，前 23 行与 JDK 相同（引导层、java.base / java.sql 模块、Configuration、无名模块均正确），在 `base.getResourceAsStream("java/lang/Object.class")` 返回 null 后 `readNBytes` NPE。原因：`input/src/resources.rs` 的资源推导按设计排除 `.class`（`path_like` 单测断言 `!path_like("p/q/A.class")`），类字节不在嵌入资源中。按 boot-layer 第 5 步（2026-10-02-boot-layer.md §2.3 第 6 条：jimage 嵌入数据 + `getNativeMap`，`.class` 字节同属模块内容）一并解决；是否放开 `.class` 资源推导属该步设计，未自行改动。其后各行（系统类加载器、线程组、属性、标准流）未覆盖到。
2. 闭包规模（需决策，2026-10-07 晚拆分实测，见下「闭包回升拆分」）：服务器 Linux JDK 21 HelloWorld 档案 `[emit]` 本分支 f442cecf 为 3053 个 JDK 类，集成分支 af1bf145 为 466（作业 bimg3-meas-af1bf145）。远超第 3 步 ≤ 540 门槛。
3. 二进制体积 / 启动：基线 af1bf145 HelloWorld release 二进制 7,761,904 字节（已无符号，`.text` 4.70 MB、`.rodata` 0.41 MB、`.data.rel.ro` 0.48 MB），整进程墙钟中位数 0.99 ms（30 次）。本分支同口径作业 bimg3-meas2-f442cecf 因 dev 关机维护被停，未得数；uprobes 测 `__boot_image_start` 需未 strip 的产物（缺省 release 已无符号），须另配。测量脚本两作业共用 `/tmp/meas_*.txt` 会串扰，重跑时须按 tag 区分文件名。
4. U11 零拷贝终态（外部静态、常量视图 / 镜像、D5 残差区段、S6 标准流）未做。

**闭包回升拆分（2026-10-07 晚，Linux JDK 21，`rava closure` HelloWorld，summary.classes；作业 bimg3-why-* / bimg3-cut-95cc94f2 / bimg3-fold-86f4f9be / bimg3-base-*）**

| 提交 / 反事实 | 类 | 方法 |
|---|---|---|
| 7f7c4201（§5.5.4 X2，本机 macOS 524） | 576 | — |
| 1605feb7（第 3 步宿主内容来源根，本机 macOS 530） | 582 | — |
| 6c344f81（2fe9e265 补 Signal native 之后，`boot_singletons` 仍在） | 3022 | 17956 |
| e50d4ba3（删 `boot_singletons`） / fcc54fb8 / 95cc94f2 | 3068 / 3055 / 3088 | 18524（95cc94f2） |
| 95cc94f2 切 `Shutdown.logRuntimeExit` | 3088 | 18524 |
| 95cc94f2 切 `Signal.dispatch` | 3087 | 18520 |
| 95cc94f2 恢复 `boot_singletons` | 3045 | 18107 |
| 95cc94f2 恢复 `boot_singletons` + 切 `logRuntimeExit` | 598 | 2149 |
| 86f4f9be（映像模块折叠，下述） | 3045 | 18108 |
| 86f4f9be 切 `logRuntimeExit` | 598 | 2149 |
| 86f4f9be 切 `logRuntimeExit` + `Signal.dispatch` | 591 | 2117 |

- 两个独立的放大入口，各自单独都能把闭包放大到约 3000 类：
  - (L) 2fe9e265 补 `Signal.handle0` 等 native 后，映像中 `Terminator.setup` 登记的 INT / TERM / HUP 处理器经 Signal Dispatcher 线程可达：`Thread.start0`（映像根）→ `Signal$1.run` → `Terminator$1.handle` → `Shutdown.exit` → `logRuntimeExit` → `System.getLogger("java.lang.Runtime")` → `LazyLoggers` → `LoggerFinder.getLoggerFinder` → `LoggerFinderLoader` → `ServiceLoader`（含 JUL 后端）。约 +2447 类。
  - (M) e50d4ba3 删 `boot_singletons` 后，`checkCanSetAccessible` 的 `callerModule == declaringModule` 不再折叠，§5.5.4 的 TIOE 链重新打开。只消 (L) 仍为 3088，两者都消为 598。
- (M) 已按终态修复（86f4f9be）：`Class.getModule` 已回到字节码（`return this.module`，字段钩子 `Class.module`）。抽象解释在已知类镜像值集上读 `class_module` 钩子字段时，按「定义加载器 + 包」查映像 VM 模块表——与运行期钩子查的是同一张表（启动序列登记）。若全部落到同一模块，结果就是该映像对象（`Obj::Image(下标)`）。引用相等按映像对象身份折叠：同下标相等，不同下标不等。镜像接收者上的调用，若唯一目标是接收者钩子字段的平凡取值（字节码形态 `aload_0 / getfield / areturn`），则按该字段读折叠。清单 `[vm_state] boot_singletons` 与 `Obj::BootSingleton` 删除。实测与恢复 `boot_singletons` 的结果相同（3045 / 598）。
- (L) 是 JDK 21 的真实运行期语义：HotSpot 上 Ctrl-C / SIGTERM 同样经 `Shutdown.exit` → `logRuntimeExit` 初始化 System.Logger 后端。该链取决于运行期日志配置与服务提供者。不用 `closure.toml` 边界截断，就压不到 540 以内；用户已决策，见下「决定」。（更正：原先此处写「即使把 `LoggerFinder` 的提供者在构建期定下，JUL 后端仍在链上」。本机 JVM 探针（JDK 21，`-Xshare:off -Xlog:class+load`，`System.exit` 路径）显示 `LogManager` 在该路径上并不初始化，JUL 后端不在 HelloWorld 的运行期装载集里，见下「决定后的实测」。）
- Linux 上的基线本身也超过 540：X2（7f7c4201）在 Linux 上为 576 类，本机 macOS 为 524，第 3 步的 ≤ 540 只在 macOS 上测过。86f4f9be 消除 (L) 之后为 598：比 1605feb7 多出的 17 类来自 Module 回到字节码（`Module$ReflectionData`、`WeakPairMap*`、`ModuleDescriptor`、`ServicesCatalog`、`BootLoader`、`HashSet`、`ImmutableCollections$SetN`）和 Signal 派发余项（`Signal$1`、`Thread$State`、`Thread$ThreadIdentifiers`、`IllegalThreadStateException`、`Permission` / `Guard` / `DomainCombiner`），`ModuleLayer` 少 1 类。598 中 `sun/nio/cs` 有 165 类（`Charset.isSupported` 映像残差根 → `StandardCharsets.lookup` 以不定名字反射），`sun/reflect/generics/tree` 有 24 类（`Locale.<clinit>` → `LocaleObjectCache` → `ConcurrentHashMap.comparableClassFor` 的泛型签名解析）。这两项在 Linux 基线 576 中已经存在。

**决定（用户 2026-10-07，取推荐项）**

1. 信号处理链 (L)：接受，行为与 JDK 一致，信号行为不做任何改动。
   - 在构建期确定 `LoggerFinder` 的提供者：生产构建时类路径资源全量打包，提供者集合在构建期可知，用它去掉 `ServiceLoader` 一段。JUL 后端保留。
   - 决定依据（主会话按 S0 Spring Boot API 面 `tests/api_surface/s0.txt` 核对）：
     - JUL 44 个方法在 S0 面内（`LogManager.getLogManager`、`Logger.getLogger`、`isLoggable` 等）；
     - System.Logger 3 个方法和 `System.getLogger` 1 个在面内；
     - `ServiceLoader` 2 个、`Runtime.addShutdownHook` 1 个在面内。
     - Spring Boot 会注册 shutdown hook，容器内 SIGTERM 优雅停机依赖 JDK 信号处理器触发这个 hook。
   - 结论：信号链 + JUL + ServiceLoader 属 S0 必需路径，不能暂缓；构建期定 `LoggerFinder` 提供者照做，对所有程序收窄 `ServiceLoader` 一段。
2. HelloWorld 闭包上限按平台分别定：以无截断实测为准，Linux 与 macOS 各给一个明确的量化目标（见下「按平台上限」）。§5.5.3 的两项精度改进（`StandardCharsets.lookup`、泛型签名）作为后续收窄项另列（见下「后续收窄排期」）。

**决定后的实测（Linux JDK 21，HelloWorld，ref 787035b3；作业 bimg3-lf3 / bimg3-lf4 / bimg3-prec-787035b3，sg1）**

用诊断切口（`--cut`，整方法或 `方法@偏移`）把 (L) 链拆成三段，分别量各段的贡献。切口只是近似：切掉 `service()` 等于把它的返回值置空，属于下近似。

| 切口 | 类 | 方法 |
|---|---|---|
| 基线 A（无切口） | 3045 | 18108 |
| E：整个 `Shutdown.logRuntimeExit`（(L) 全切） | 598 | 2149 |
| ①：`DetectBackend.<clinit>` + `LoggerFinderLoader.service`（ServiceLoader 一段） | 2999 | 17744 |
| ① + ②：再切 `System$LoggerFinder.accessProvider` @8 / @26（静态字段为空的分支） | 2998 | 17741 |
| ③：`logRuntimeExit@74`（DEBUG 级 `log` 调用） | 3045 | 18107 |
| ① + ③ | 2999 | 17743 |
| ① + ② + ③ | **628** | 2255 |

- 三段互相独立，各自单独都能保住约 3000 类，只有三段同时去掉才落到 628：
  - ① ServiceLoader 一段：`ServiceLoader` → 类路径 `findClass` → `URLClassPath` / jar → `SecureRandom` → JCA；`BootLoader.findResources` → jrt `Handler` → jimage；
  - ② `accessProvider` / `LoggerFinderLoader.service()` 的「静态字段为空」分支：`doPrivileged(PA, ACC, Permission[])` → `FilePermCompat` → `SecurityProperties` → regex → ICU；
  - ③ `logRuntimeExit` 的 DEBUG 级日志：`log` → `SimpleConsoleLogger.getCallerInfo` → `CallerFinder.<clinit>` → `StackWalker` → `StackFrameInfo` → `MethodHandleImpl`（invoke 一大块）。
- 628 比 E（598）多 30 类，就是 HotSpot 上该路径真实装载的 System.Logger 前端：`jdk/internal/logger` 17 类、`sun/util/logging` 5 类、`System$Logger` / `Level` / `LoggerFinder`、`TemporaryLoggerFinder`、`SimpleConsoleLogger`、`SurrogateLogger`、`RuntimePermission` / `BasicPermission`、`BooleanSupplier`、`Class$EnclosingMethodInfo`、`PreviewFeatures` 等。
- JVM 真值（本机 JDK 21，`-Xshare:off -Xlog:class+load`，main 之后 `System.exit`）：
  - main 之后约装载 236 类，其中日志相关类与 ①+②+③ 的 30 类一致；
  - 另装载 `ServiceLoader`（两种迭代器）、stream、jimage（12 类）、`sun.nio.fs`、`NativeLibraries`、lambda / invoke；
  - **不装载** `LogManager`（JUL 后端）、`StackWalker`、regex、JCA。
  - 结论：628 与 JDK 的运行期行为对齐；3045 中的大头（JCA、regex、StackWalker）是分析精度造成的，并非真实语义。

**终态设计：三个机制，缺一不可（只做构建期提供者确定是 −46 类）**

1. 构建期确定 `LoggerFinder` 提供者（本决定，对应 ①）。
   - 引导求值器在 `[concrete.boot] calls` 的 `initPhase3` 之后追加一次 `LoggerFinder` 查找（清单项，不写进生成器）。由求值器在构建期跑完 `LoggerFinderLoader.service()`，结果（提供者实例与 `service` 静态字段）进入映像，运行期不再走 `ServiceLoader`。
   - 前置条件：
     - 求值器能读 jimage 与模块资源（第 5 步 T3）；
     - 类路径资源在构建期全量打包，求值器可以枚举（U3：生产构建打包资源，取代延迟的 `toFileURL` 占位）。否则求值器在 `ServiceLoader` 上失败，构建报错。
   - 语义：该路径上读到的属性取构建期值；用户提供者的构造函数在构建期运行。与 U1（属性运行期取宿主值）的划界在实施时登记为 U1 的例外项，例外项仅限提供者选择（U12 已定，范围见 §8.4）。
2. 通用静态字段非空折叠（对应 ②，分析器通用机制，无类名）。
   - 条件：静态字段的映像值非空，且档案内所有可达的 `putstatic` 写入值都非空。
   - 动作：`getstatic` 后的 `ifnull` / `ifnonnull` 按「非空」折叠，空分支不可达。
   - 这条只依赖 JDK 侧事实（映像值 + 档案内写点集合），符合「折叠只对用户无法扩展的事实做」。
3. 日志级别折叠（对应 ③）。
   - `jdk.system.logger.level` 缺省为 INFO，经 `SurrogateLogger` / `SimpleConsoleLogger` 的级别字段决定 `isLoggable(DEBUG)`。
   - 级别值进映像（构建期初始化），`isLoggable` 的结果按映像值折叠，`logRuntimeExit@15 ifeq` 只剩不记日志的分支。
   - 若用户在运行期用 `-Djdk.system.logger.level=DEBUG` 打开，行为与 JDK 不同。这与机制 1 一样属于「该路径属性取构建期值」，一并登记。

**按平台上限（HelloWorld，JDK 21，无 `closure.toml` 边界截断）**

| 平台 | 上限 | 依据 |
|---|---|---|
| Linux | **≤ 640** | 实测 628（①+②+③ 切口）+ 映像中提供者实现约 5 类，留少量余量 |
| macOS | **≤ 590** | Linux 上限减平台差约 52（X2：576 / 524，1605feb7：582 / 530）；**推算值，三个机制落地后须在 macOS 上实测确认** |

- 现状：Linux 3045、macOS 未测。上限在上述三个机制全部落地后验收；只做机制 1 的预计值为 Linux ≈ 2999。
- 原第 3 步门槛 ≤ 540 是 macOS 上未计 (L) 链时定的，现由本表取代。

**后续收窄排期（三个机制之后，各自独立）**

| 项 | 实测（Linux，在 E 上切） | 预计减少 | 做完后终态 | 前置 / 待决 |
|---|---|---|---|---|
| S1 `sun/nio/cs/StandardCharsets.lookup` 以不定名字反射 | 598 → 406 | Linux −192（`sun/nio/cs` 165 → 9，另有 `jdk/internal/reflect` 8、`java/lang/invoke` 8 等）；macOS 约 −120（闭包中 `sun/nio/cs` 104 类，推算） | Linux ≈ 436，macOS ≈ 470 | 在 U1（运行期取宿主编码）下，按不定名字查字符集是合法可达，不是精度缺陷。要收窄，须先由用户重新审视 U1：例如把可选字符集固定为构建期声明的集合。U14（2026-10-08，§8.4）已定：`file.encoding` 钉 UTF-8，jnu / stdout / stderr 编码仍运行期读取，收益逐项实测 |
| S2 泛型签名精度（`Locale.<clinit>` → `LocaleObjectCache` → `ConcurrentHashMap.comparableClassFor@21` → `getGenericInterfaces`） | 598 → 557 | Linux −41（`sun/reflect/generics` 38 类全部 + `GenericSignatureFormatError`、`TypeVariable`、`Annotation`）；macOS 预计同量 | Linux ≈ 587，macOS ≈ 549 | 分析器通用机制：键类型的类签名可在构建期读出（`Locale$LocaleKey` 无签名，`BaseLocale$Key` 只有字段签名），`comparableClassFor` 的泛型接口遍历按已知键类集合折叠。收窄单独不可达（§5.6.5），终态需 `Class.genericInfo` 入映像，U13 已定（§8.4，做） |
| S1 + S2 | 未合测：lookup 与 `comparableClassFor@21` 没有一起切过。整方法切 `Class.getGenericInterfaces` 无效果（598 → 598，与 lookup 合切仍为 406），所以 S2 以 `comparableClassFor@21` 切口为准 | Linux 约 −233（按两项相加） | Linux ≈ 395，macOS ≈ 430 | 同上两项；合测值待做 |

- 排期：S2 按 U13 已定（§8.4，`Class.genericInfo` 入映像），排在三个机制之后的第一项；S1 按 U14（2026-10-08，§8.4）重估——`lookup` 的三条入口中 jnu 区段与 stdout / stderr 编码仍取决于运行宿主、保持运行期读取，名字不定仍属合法可达；`file.encoding` 钉为 UTF-8 后的收益逐项实测，无收益不钉。2026-10-08 实测（`docs/reports/2026-10-07-closure-composition.md` §7.3）：只切宿主名来源 Δ = 0（常量名 `"UTF-8"` 经 `defaultCharset` 仍按全表反射）；收窄须同时有「U1 字符集域决定」与「字符串常量上下文 + 常量表键→值」，上界 −141 类。

**待验证清单（10-07 起改为合批测试，由主会话合入验证分支后统一跑；本分支 c61b7761 只做过本机 cargo check --tests）**

1. 全量单测：bimg3-ut2-911a3a4d（sg1）的结果由主会话收取。该作业在 7000 s 处被停止，没有跑完：前 7 个测试二进制全部通过，第 8 个（10 项闭包集成测试）中 `param_string_constants_fold_switch` 和 `closure_independent_of_order` 已标 FAILED，断言细节因进程中止未输出。这两项待合批复验；后者属于确定性测试，集成分支 d7b490db 也记有「闭包确定性两项待 dev」。合并 b6ed3950（Narrowed 改名、lib_runtime 删除）之后单测未跑。重点看三项：
   - `absint::tests::image_object_results_compare_by_identity`；
   - `manifest::vm_state` 各项（`boot_singletons` 已删除）；
   - 闭包确定性两项。
2. 抽查 7 例，输出须与 JDK 相同：HelloWorld、TestAppClassLoader、TestModuleLayerDefine、TestClassModuleFace、TestSetAccessibleBoundary、TestProtectionDomainFaces、TestStringGetCharsLegacy。6878583b 上 bimg3-f 为 7/7 通过，合并后待复验。known_failures.toml 中 TestModuleLayerDefine / TestClassModuleFace / TestProtectionDomainFaces 三项已移除，复验失败即为回归。
3. 闭包数字：HelloWorld 在 Linux JDK 21 档案键下应为 3045，(M) 已折叠；去掉 (L) 后为 598。合并集成分支的 JCA 键判定收窄之后，这两个数是否变化待测。
4. 第 3 步其余验收项未测：
   - 二进制大小增量 ≤ 5%；
   - 启动装载 ≤ 1 ms；
   - TestBootLayer 输出与 JDK 相同（依赖第 5 步 jimage / getNativeMap）。
5. 闭包上限已按平台改定（上「按平台上限」：Linux ≤ 640，macOS ≤ 590），在三个机制落地后验收。
6. `param_string_constants_fold_switch` 已做静态判断，不是本分支抽象解释改动（`Image` / `Narrowed` 合并等）引起的。
   - 该测试断言闭包不含 `sun/net/www/protocol/jrt/Handler`，且类数 < 1000。
   - 本分支补 Signal native 之后，(L) 链经 ① 的 `BootLoader.findResources` 使 jrt `Handler` 可达，闭包为 3045 类，两条断言都不满足。
   - 断言本身正确，不改；机制 1–3 落地后（628 < 1000，且 ① 去掉了 jrt `Handler`）预期通过，进合批复验。

### 5.6 第 4 步（续）：静态字段非空折叠、VM / Class / SecurityManager 归零（2026-10-08，分支 `boot-image-s4`，基于 batch-1007 98e733c9）

本节提交只在本机做过 `cargo check --release --tests`（生成器 + `rava_macros_core`；`java_runtime` 不在该 workspace，未经编译），单测、闭包与 e2e 一律未跑，见下「待验证清单」。

**提交**

| 提交 | 内容 |
|---|---|
| b75e0b03 | §5.5.6 机制 ②：静态字段非空折叠（`engine/static_init.rs`） |
| e40e805c | `Class.enumConstantDirectory` 回到字节码，删除运行时常量目录 |
| cabe9fb0 | VM 移出 `[vm_boundary]`（8 个 `#[jvm_boundary]` 归零）、档位写回 `VM.initLevel`、JSR 292 核心类构建期初始化、SecurityManager 移出边界 |

**`#[jvm_boundary]` 计数（`git grep -c`，`runtime/java_runtime/src`）**

| 文件 | 98e733c9 | cabe9fb0 |
|---|---|---|
| `jdk/internal/misc/vm_impl.rs` | 8 | **0** |
| `java/lang/class_impl/members.rs`（`getModule` 已在 e50d4ba3 删除） | 1 | **0** |
| `java/lang/class_loader_impl.rs` | 6 | 6 |
| `javax/crypto/jce_security_impl.rs` | 6 | 6 |
| `jdk/internal/loader/boot_loader_impl.rs` | 2 | 2 |
| **全仓** | 23 | **14**（达第 4 步验收数；余项归第 5 步 L2 6、BootLoader 2、JceSecurity 6——JceSecurity 2026-10-08 由第 6 步前移，§6） |

`[vm_boundary] classes` 余 ClassLoader、Class、JceSecurity、BootLoader；`clinit_carried` 余 JceSecurity、Class。

#### 5.6.1 机制 ②：静态字段非空折叠（b75e0b03）

- 静态字段的常量格 = 初值 ⊔ 档案内全部可达 `putstatic` 写入值。初值：映像 `build_time` 中的类取映像值（映像只导出非缺省值，缺席即缺省值；污点 / 占位为 Top），其余类取缺省值。
- 静态写入值按「非空」入格：确定非空、无标签的引用记为非空引用（不同对象 / 不同字符串合流仍为非空引用，与 null 合流为可空）。映像值非空且全部可达写入非空时，`getstatic` 后的 `ifnull` / `ifnonnull` 按非空折叠。
- 写入来源超出字节码的字段（反射 / Unsafe / VarHandle / 手写写入）由 `field_open` 先行排除；规则只看字段与映像，无类名。实例字段不取非空引用（`allocateInstance` / 反序列化的对象不经构造器）。
- 附带修正：构建期初始化类的基本类型静态字段原先以缺省 0 为初值（`<clinit>` 不再展开，写入集里没有初值写入），现取映像值。
- **HelloWorld 单独效果（预计）：≈ 0 类**。§5.5.6 的 ② 切口针对 `System$LoggerFinder.accessProvider` / `LoggerFinderLoader.service()` 的「静态字段为空」分支，而这些字段在不做 ①（构建期确定提供者）时映像值就是 null，② 不折叠；表中 ①+② = 2998 与 ① = 2999 只差 1 类，也说明 ② 的收益要等 ① 落地。① 与 ③ 待用户（本步未做）。

#### 5.6.2 VM：8 个 `#[jvm_boundary]` 回到字节码（cabe9fb0）

三问逐方法判定（JDK 21 字节码，javap 核对）：

| 方法 | 字节码 | 判定 |
|---|---|---|
| `initLevel()` / `isBooted()` / `isModuleSystemInited()` | 读静态 `initLevel`，`>= 4` / `>= 2` | 运行模型成立：映像值 4（initPhase3 写入）；按字节码 |
| `shutdown()` / `isShutdown()` | `initLevel(5)` / `initLevel == 5` | 按字节码；删进程级 `SHUTDOWN` 标记 |
| `getSavedProperty(key)` | `savedProps.get(key)`，`savedProps` 为空抛 ISE | 构建期可求值：`savedProps` 是 initPhase1 保存的属性表，在映像中；按字节码 |
| `isSystemDomainLoader(loader)` | `loader == null \|\| loader == getPlatformClassLoader()` | 按字节码（原手写恒 true 与 JDK 不等价：app 加载器为 false） |
| `latestUserDefinedLoader()` | 调 native `latestUserDefinedLoader0`，null 时回落平台加载器 | 字节码；native 按 HotSpot `JVM_LatestUserDefinedLoader` 手写：自栈顶逐帧取首个「非引导、非平台」定义加载器，跳过 `MethodAccessorImpl` / `ConstructorAccessorImpl` 子类帧（帧源 `vm_stack`） |
| `setJavaLangInvokeInited()` / `isJavaLangInvokeInited()`（原无属性的手写） | 置位 / 读静态 `javaLangInvokeInited` | 按字节码；HotSpot `initialize_jsr292_core_classes` 在 initPhase1 之后初始化 `MethodHandle` / `ResolvedMethodName` / `MemberName` / `MethodHandleNatives`，后者 `<clinit>` 置位。清单 `[concrete.boot] calls` 新增 `{ init = [类...] }` 项（`BootCall::Init`，阶段之间由 VM 初始化的类），在 initPhase1 与 initPhase2 之间按 HotSpot 次序初始化这 4 类，映像中 `javaLangInvokeInited = true` |

`vm_impl.rs` 只剩 3 个 `#[jvm_native]`：`initialize`、`latestUserDefinedLoader0`、`getNanoTimeAdjustment`。

档位的单一字段化：
- 原做法：线程局部 `ExecState.boot_level` + `__vm_at_init_level` + `image_rt::set_level`，`initLevel()` 手写读它。
- 现做法：`IStep::Level { decl, name, level }` 携带档位字段（`[concrete.boot] level`），启动序列重放残差步骤前把构建期档位直接写入 `VM.initLevel`（`g.store(ILoc::Static ..)`，与映像静态字段写入同一路径）；导出时序列末尾追加一条恢复映像值的档位步骤（末步已是该值则不加）。JSON：`{"level": l, "at": [类, 字段]}`。删除 `boot_level` / `set_level` / `__vm_at_init_level`。
- 无映像（求值失败）时 `System.registerNatives` 直接调 `VM.saveProperties`，此时档位即 VM 初值 0，与字节码的 `initLevel() != 0` 检查一致。

清单：
- `[facts.returns]`：保留 `isBooted` / `isModuleSystemInited` = true（映像值 4，唯一写入方 `initLevel(int)` 只增），注释改为字节码口径；删 `isJavaLangInvokeInited`（由映像值 + ② 的基本类型初值给出常量）。
- `[vm_constants] null_returns`：删 `getSavedProperty`（准入「手写无条件给出」不再成立），表为空。
- 求值器（`concrete/vm.rs`）：引导求值中**有字节码的方法不取返回值事实**。原先 initPhase1–3 期间 `isBooted` / `isModuleSystemInited` 也按事实读成 true，与 JVM 引导期次序不一致（如 `ReflectionFactory.config()` 在模块系统就绪前应返回 `DEFAULT_CONFIG` 且不缓存）；现按字节码读档位。native（`desiredAssertionStatus0`）不受影响。
- `closure.toml`：删 `[vm_boundary]` 的 `jdk/internal/misc/VM`。

#### 5.6.3 Class：`enumConstantDirectory` 回到字节码（e40e805c）

- 原手写读运行时常量目录（`java_class!` 宏在类初始化后为「自身类型 static 字段」登记取值闭包）。JDK 体是 `getEnumConstantsShared()` → 反射调 `values()` → 建 HashMap 缓存于字段，语义可由字节码表达，判定为翻译。
- 删除：`members.rs` 手写体；`lib.rs` 的 `CONSTANT_DIRECTORY` / `register_constant_directory` / `lookup_constant` / `constant_directory_entries` / `constant_directory_universe`；宏 `class_init.rs` 的 `constant_directory_registration` 及 `__class_init` / `__boot_initialized` 中的登记。
- 影响：`Enum.valueOf` 的 `values()` 走反射调用，闭包精度依赖集成线上的「枚举 values 直接调用收窄」。

#### 5.6.4 SecurityManager 移出边界（cabe9fb0；T1 / T2 / T5 / T6 余项）

- T1 / T2 / T5 / T6 已在 e50d4ba3 / fcc54fb8 完成（§5.5.6），余项只有 T2 一栏的 SecurityManager：`<clinit>` 为 `getRootGroup`、锁、CHM 与 `ModuleLayer.boot()` → `addNonExportedPackages`，引导层已在映像中，按字节码可执行。
- `SecurityManager` 移出 `[vm_boundary]` 与 `clinit_carried`；手写只剩 ACC_NATIVE `getClassContext`（准入 ③ 栈遍历）。
- `Class` 留在 `clinit_carried`：`<clinit>` 只有 `registerNatives` 与两个空数组，构建期已在 VM 预初始化中执行；`Class` 移出 `[vm_boundary]`（struct 归生成器）时一并移出，属第 5 / 6 步。

#### 5.6.5 S2（泛型签名精度）

**诊断**（us1 Linux JDK 21 HelloWorld `rava closure --flows @concrete`，作业 bimg4-s2c-9c78c3ac / s2d-8cebe1b8 / s2e-8cebe1b8（`@grow` / `@edge` 探针）/ s2f-13bafec6）：

- 诊断粒度：`[concrete]` 结果原按调用点覆盖写，「28 组」只是最后一个上下文的结果；9c78c3ac 改为按调用点累计各上下文结果（成功列出各组实参，至多 64 组；回退同列）。
- `comparableClassFor@21`（HashMap / ConcurrentHashMap 同形）在多数上下文具体求值成功，但每个方法各有一个上下文回退为抽象调用边，`sun/reflect/generics` 由该边拉入。回退原因：
  1. **接收者超过 64 个**：键来源为 open(Comparable) / 宽集合的上下文（树化桶 `TreeBin` 克隆、映像对象上下文等）；
  2. 修复前另有 `[Ljava/lang/String;`（数组的接口）与 Thread、ReferenceHandler、InnocuousThread、WeakReference、ModuleReferenceImpl 等**非 Comparable 接收者**。
- 非 Comparable 接收者的根因（s2e 探针）：instanceof 收窄节点 `@1` 正确过滤为 Comparable 的子集，但「类 × 接口」收窄保留 open(Thread)（Thread 的子类可能实现 Comparable）；`Object.getClass` 是 final，走非虚调用 `edge_recv`，接收者值集在那里被物化为 `Feed::S`，丢失来源节点，`getClass` 结果按 open(Thread) 全量展开成镜像（探针显示 `@8` 的增长为「直接」注入）。
- **修复（终态做法）**：
  - 8cebe1b8：接口类型判定站点（instanceof 成立一侧 / checkcast 目标为接口 I）登记为节点的接口界 `open_bounds`；镜像流 `mflow` / G 增长重开 `mirror_reopen` 只展开同时 ⊂ I 的子类型。
  - 13bafec6：非虚调用物化接收者时，若含 open 的来源都是同一接口判定站点，以 `Recv::Bounded(值集, I)` 带上接口界，`getClass` 结果按界展开。
  - 实测 s2f-13bafec6：两个 `comparableClassFor@21` 的成功上下文接收者全为 Comparable（String、Integer、Long、Character、File、UnixPath、StandardOpenOption、TextStyle、LocaleProviderAdapter$Type），数组接收者回退消失；闭包仍为 3043 类 / 18,093 方法（未变）。
- **结论：S2 的预期减量（≈ −41 类）单靠收窄不可达**，原因有二：
  1. **真泛型接收者**：File、Integer、UnixPath 与各枚举实现 `Comparable<T>`，带类签名属性。桶树化时 `getGenericInterfaces` 在运行期确实解析签名（`ClassRepository.make` → `SignatureParser`）。§5.6.5 原案「集合内各类均无签名 → null」的折叠对它们不成立。
  2. **open(Comparable) 上下文**：键来源为开放类型（用户可扩展），接收者超过 64 个且不可枚举，只能走抽象边。
- **终态方向（待决，见 §5.6.7）**：在构建期把热路径类镜像的 `Class.genericInfo`（`ClassRepository`）物化进引导映像。即保留映像镜像上 memo 字段的写入，运行期命中缓存不再解析签名。open(Comparable) 的上下文仍要为映像外的类保留解析路径，所以 `sun/reflect/generics` 能否出闭包取决于开放世界下的可达性判定。这是架构选择，未实施。
- 其余 `[concrete]` 回退（与 S2 无关，留档）：
  - `Runtime$VersionPattern.<clinit>@2`、`LocaleResources.<clinit>@29`：求值读到 VM 承载的静态字段 `Integer$IntegerCache.high`；
  - `Formatter.format@11`：格式串实参来自形参，不可枚举。

#### 5.6.6 待验证清单（合批测试；本节提交只做过 cargo check）

1. 单测（`cd generator && cargo test --release`）：
   - `engine::static_init::tests::{image_nonnull_and_nonnull_writes_stay_nonnull, nullable_write_breaks_nonnull, default_initial_outside_image, primitive_image_value_is_initial, distinct_nonnull_values_join_nonnull, instance_writes_keep_plain_lattice}`；
   - `absint::tests::nonnull_static_field_folds_null_test`；
   - `manifest::concrete::tests::boot_calls_phase_and_init`；
   - `image::tests::json_roundtrip`（`IStep::Level` 新形态）；
   - 全量单测中 `param_string_constants_fold_switch` / `closure_independent_of_order` 沿用 §5.5.6 待验证清单第 1、6 条。
2. `java_runtime` 编译（任一 e2e 即覆盖）：`vm_impl.rs` 的 `latestUserDefinedLoader0`（`ClassLoaders::platformClassLoader` / `Class::__vm_defining_loader` / `vm_stack::capture_java_frames`）、`system_impl.rs`、宏去掉常量目录登记后的全部生成类。
3. 引导映像（**已实测**，bimg4-s2-cabe9fb0，us1 Linux JDK 21，HelloWorld `rava closure`：status ok，initPhase1 55,243 步 → JSR 292 4 类初始化「完成」74,696 步 → initPhase2 返回 I(0) → initPhase3 完成，共 1,616,260 步、321 ms；已初始化 256 类、映像对象 8,385；运行期初始化只有 `jdk/internal/util/StaticProperty`（Linux 1，不增）；WAR 交集 0、映像污点 0）。仍待核对：HelloWorld `rava audit boot`（或 build 报告）中 initPhase1 之后出现「VM 初始化 java/lang/invoke/MethodHandle / ResolvedMethodName / MemberName / MethodHandleNatives　完成」一行，未登记失败 = 0，运行期初始化类数不增（Linux 1 / macOS 2）；映像中 `VM.javaLangInvokeInited = true`、`VM.initLevel = 4`；启动序列中档位步骤为 `VM.initLevel` 静态写入，末尾恢复为 4。引导期不取返回值事实后，initPhase2 / 3 结果与 §5.4 一致（返回 0、`scl = AppClassLoader`）。
4. 抽查（输出与 JDK 相同）：
   - 第 4 步回归集：HelloWorld、TestAppClassLoader、TestModuleLayerDefine、TestClassModuleFace、TestSetAccessibleBoundary、TestProtectionDomainFaces、TestStringGetCharsLegacy；
   - 枚举 `valueOf`（常量目录删除）：TestEnumBasic、TestEnumAdvanced、SwitchExpressions；
   - `getSavedProperty`：TestIntegerCacheSpec；
   - 停机（`shutdown` / `isShutdown` 改写 `initLevel`）：TestShutdownHooks、TestSystemExitEnv；
   - `javaLangInvokeInited` / JSR 292 构建期初始化：TestMethodHandleDirect，及任一反射调用用例（`MethodHandleAccessorFactory.useNativeAccessor` 依赖该标记）；
   - `VM.directMemory` / `savedProps`：TestDirectBuffer；
   - `latestUserDefinedLoader`：任一 `ObjectInputStream.readObject` 用例（如 DeepCopy、TestSerialUserGenericCallbacks）。
5. 闭包数字（Linux JDK 21 HelloWorld，`rava closure` summary.classes）：基线 3045（(L) 链在）/ 598（切 `logRuntimeExit`）。**实测 cabe9fb0：3043 类 / 18,093 方法**（bimg4-s2-cabe9fb0；基线 3045 / 18,108 测于 86f4f9be，其后并入的 JCA 键判定收窄等也在本数内，差值不单归本节）。待测：切 `logRuntimeExit` 的对照数（预期 ≈ 598）。本节预期：② 单独 ≈ 0；VM 回到字节码后 `isSystemDomainLoader` / `latestUserDefinedLoader` / `getSavedProperty` 的字节码入链（小幅增加，预计 < 10 类）；`enumConstantDirectory` 经反射 `values()` 的增量取决于集成线收窄；S2 见 §5.6.5。
6. 映像必需（§5.6.8，dcabf9e3）：
   - **全量语料映像求值 ok 数**（合批时统计）：全量 e2e 中「引导映像求值失败」导致的构建失败数应为 0；按 build_status / 构建日志统计 ok 数 = 参与构建的用例数，失败的逐例记首个失败阶段与失败栈末帧；
   - 单测全量通过（`analyze` 签名改为 `Result`；`ClosureFacts::from_json` 缺 `boot_image_data` 即错）；
   - `java_runtime` 编译：`system_impl.rs`（`registerNatives` 空体、删 `host_property` / `derived_vm_property`）、`meta.rs`（删 `VM_CONST_PROPERTIES` / `VM_DYNAMIC_PROPERTIES` 外部表）、`lib.rs`（删 `vm_boot_init`）；
   - 系统属性抽查（输出与 JDK 相同，属性表现只来自映像）：TestSystemStableProps、TestBootLayer、TestIntegerCacheSpec、TestDirectBuffer。
7. 接口界收窄（§5.6.5，9c78c3ac / 8cebe1b8 / 13bafec6）：
   - 全量单测，重点为 `closure_independent_of_order`（`open_bounds` 在 InstanceOf / CheckCast 事件登记；`Recv::Bounded` 依赖来源节点当时的值集，须与工作队列次序无关）及反射 / getClass 相关闭包单测；
   - 抽查 getClass 密集的用例：TestTreeMapComparable 类用例、任一 HashMap 树化用例、反射 `getGenericInterfaces` 用例，输出与 JDK 相同；
   - 已实测：HelloWorld `--flows @concrete` 中 `comparableClassFor@21` 的接收者全为 Comparable（s2f-13bafec6）。
8. TestClassResourceStream（batch-1007 抽查回归）：**根因不在本分支**。`getResource` / `getSystemResource` 的 4 行差异（self-url、fqcn-abs、jdk-res、loader-eq）来自 `class_loader_impl.rs` 中这两个方法在集成分支仍是返回 null 的 `#[jvm_boundary]`。10-07 通过的 url2m-95b2d84e-r2 跑在 `c1d-url-b2` 分支（95b2d84e，`EmbeddedClassPath::findResource` / `findResources`），该分支尚未并入集成线与 batch-1007。待 `c1d-url-b2` 并入后复验，本分支不改。（2026-10-08：c1d-url-b2 已合入 batch-1008 04600e21，随合批复验。）

#### 5.6.7 遗留与待决

1. ① 构建期确定 `LoggerFinder` 提供者、③ 日志级别取映像值：待用户，本步未做；② 的 HelloWorld 收益依赖 ①。
2. ~~无映像（求值失败）时的回退路径~~：已取消，映像必需，见 §5.6.8。
3. `getSavedProperty` 改为真实语义（读映像 `savedProps`），`[vm_constants]` 剪枝失效；`Integer$IntegerCache` 等调用点的闭包变化待测（见清单第 5 条）。
4. ~~`concrete/vm.rs` 740 行~~：已按职责拆为 `vm.rs`（值 / 非正常完成 / 对象与方法信息表示 / `Vm` 本体与 `Env`，353 行）、`vm_heap.rs`（分配、数组、字段读写与纪元检查、撤销、身份哈希，243 行）、`vm_link.rs`（字符串 / 类镜像驻留、方法解析 / 选择 / 方法信息，159 行），纯搬移无语义改动。
5. `Class` 仍在 `[vm_boundary]` 与 `clinit_carried`（native 与 struct 归属），随第 5 / 6 步处理。
6. S2 的类减量：构建期物化热路径镜像的 `Class.genericInfo` 进引导映像（§5.6.5 终态方向），待决。

#### 5.6.8 映像必需：取消无映像回退（dcabf9e3）

终态原则：原生二进制只有「从构建期引导映像出发」一条启动路径，映像求值失败即构建失败，不保留运行期引导的第二条路径。

- **失败即失败**：`closure::analyze` 返回 `Result<Closure, BootFailure>`；`BootFailure` 带首个失败阶段与原因（求值器的失败文本，含未登记的 native / VM 操作名，如「native 未登记 X」）、失败点调用栈（外→内，展示末 30 帧）与审计报告。`rava build` / `rava profile` 以该错误结束；`rava closure --boot-report` 失败时仍写报告再返回错误。清单 `[concrete.boot] calls` 无引导阶段、阶段方法所在类不在类路径上（缺参考 JDK）同为失败。失败不写闭包缓存。
- **去 Option**：`Closure.boot_image`、`BootImage.data`（删 `ok` 字段）、`ClosureFacts.boot_image`、`BuildInput.boot_image` 均为必有；closure.json / 档案 `profile.json` 缺 `boot_image_data` 即格式错误；发射层无条件发射 `boot_image.rs` 与 main 中的 `__boot_image_start()` 调用（`write_facade` / `write_boot_image` 去掉「有无映像」分支）。
- **删除的回退代码**：
  - 分析器：无映像时以 `seeds.toml [boot_init]` 类作初始化根（`root_init` 一并删除）；
  - 清单：`seeds.toml [boot_init]` 段（System、AccessibleObject）与分析器 / 发射层两侧的读取字段——两类在映像中已于构建期初始化（System 在 `[concrete.boot] init`，AccessibleObject 作为 `java/lang/reflect/Method` 的超类随之初始化），有映像时这张表本就全是空操作；
  - 发射层 main 的 `vm_boot_init(&[...])` 调用与运行时 `vm_boot_init`；
  - `System.registerNatives` 的运行期属性表构建（`ConcurrentHashMap` 直挂、`VersionProps.init`、`VM.saveProperties`、`setJavaLangAccess`）——只在无映像时运行；现回到 native 本义（绑定 JNI 入口，原生二进制为空操作），`System.props` / `VM.savedProps` 由构建期 initPhase1 写入映像，宿主相关键经 `SystemProps$Raw` 启动重放取值。随之删除 `host_property` / `derived_vm_property`、java_meta 的 `VM_CONST_PROPERTIES` / `VM_DYNAMIC_PROPERTIES` 表（发射与运行时 `meta::vm_const_properties` / `vm_dynamic_properties`）与输入侧 `SysPropFacts`。closure.json 的 `system_properties`（分析器折叠表、档案一致性校验）保留。
- 不受影响：`#[jvm_boundary]` 计数不变（14）；`gil.rs` 的 `__boot_initialized`（映像启动序列标记构建期已初始化类）保留。

验证见 §5.6.6 第 6 条。

#### 5.6.9 U13：`Class.genericInfo` 入映像（2026-10-08，分支 `u13-generic`，d147fe31 / 41d45f3c）

用户 10-08 采纳 §8.4 U13。本节实现 §5.6.5 的终态方向：具体求值在类镜像上写入的内存缓存物化进引导映像，运行期命中缓存后不再解析签名。

**机制**

- **清单**：`vm_intrinsics.toml [concrete] image_memo_fields = ["java/lang/Class.genericInfo"]` 声明可入映像的镜像缓存字段，生成器内不写类名。`ClassRepository` 追加进 `[concrete.boot] calls` 在构建期初始化（位于 initPhase3 之后），使其静态字段（`NONE` 等）成为映像对象，供片段按静态字段引用。
- **导出**（`engine/concrete/persist.rs`）：每组实参冷 / 热两次求值后、撤销之前，检查撤销日志。条件是全部写入都落在类镜像的 `image_memo_fields` 字段上，且值的对象图只由三类对象组成：本求值纪元内新建的对象、类镜像、由不变静态字段持有的映像对象（`image_roots`）。满足时，按（镜像, 字段）导出片段（对象按广度优先编号，跳过缺省值字段，字段排序）。以下情况不可物化，给出原因：
  - 写入静态字段、写入非镜像对象、写入非映像缓存字段；
  - 值含污点；
  - 新建对象取过身份哈希，或是 lambda；
  - 引用不由静态字段持有的映像对象。
- **入闭包口径**（`engine/concrete.rs`）：
  - 可物化的组合只按**热求值**轨迹入闭包。热求值的 inited 并入求值触及的类（`Trace.touched`），使片段对象的类型被初始化 / 实例化。
  - 不可物化的组合沿用冷 / 热轨迹并集。
  - `--flows @concrete` 诊断逐组标注 `⇒映像` 或 `（并：原因）`。
- **并入映像**（`engine/image_memo.rs`）：与第 6 步的构建期初始化扩展合为同一套追加与规范化流程，见 §5.8.5（下列为合并前 d147fe31 的做法，已被 §5.8.5 取代：值写进镜像对象体、`image_finish` 单独重排）。
  - 片段中的静态字段引用须属于构建期初始化类，且映像值是对象。
  - 同一（镜像, 字段）只写首个片段，因为缓存值与求值实参无关。
- **发射**（`emit/.../boot_image/start.rs`）：启动序列在链接之后，对每条 `mirror_memos`（镜像与值均为活对象）生成 `<Class as From<Object>>::from(rt::mirror(desc)).__set_<slot>(From::from(v))`，写入运行期镜像；值取记录本身（`IMemo.val`），不读镜像对象体。

**实测**（us1 Linux JDK 21，`rava closure`，作业 u13-s4-c614f840 / u13-s4b-41d45f3c）

基线 b558e0c2 本身在闭包分析中发散：嵌套数组膨胀至 OOM，作业 u13-base / u13-m1 失败，与本改动无关。因此实测改在 c614f840（batch-1008 之前的 boot-image 头）上应用本分支补丁（`git diff b558e0c2 HEAD | git apply -3`）。

| | 基线（c614f840） | U13 |
|---|---|---|
| HelloWorld 类 / 方法 | 3043 / 18,093 | 3043 / 18,090 |
| CollectorsDemo 类 / 方法 | 3043 / 18,125 | 3043 / 18,122 |
| HelloWorld 映像对象 / 活对象 | 8,385 / 7,542 | 8,733 / 7,770（`mirror_memos` 9 条） |
| CollectorsDemo 映像对象 / 活对象 | — | 9,615 / 7,773（`mirror_memos` ≥ 13 条） |

`sun/reflect/generics` 共 47 类，`GenericSignatureFormatError`、`TypeVariable`、`ParameterizedType`、`WildcardType`、`GenericArrayType` 各 1 类，前后不变。

两个 `comparableClassFor@21` 的逐组标注（41d45f3c 诊断）：

- String、Integer、Long、Character、File、StandardOpenOption、TextStyle、`LocaleProviderAdapter$Type` 全部 `⇒映像`。
- 唯一例外是 `UnixPath`：`（并：写镜像 sun/nio/fs/UnixPath 的非映像缓存字段 java/lang/Class.reflectionData）`。原因是 UnixPath 无类签名，`getGenericInterfaces` 退到 `getInterfaces`，后者写 `reflectionData` 缓存。该组冷轨迹里没有签名解析，不影响 `sun/reflect/generics` 的可达性。

**结论：机制生效，但 −41 类不可达，`sun/reflect/generics` 留在闭包内。** 留下的原因有两条，都不是 genericInfo 缓存能消除的：

1. **注解解析路径**：U13 之后 `SignatureParser` 的首个入闭包原因由 `[concrete] comparableClassFor` 变为 `AnnotationParser.parseSig`，链路为 `AnnotatedElement.isAnnotationPresent` → `Method.getAnnotation` → `Executable.declaredAnnotations` → `AnnotationParser.parseAnnotations`。注解成员类型签名的解析与 `Class.genericInfo` 无关，属于档案里的反射 / 注解入口。
2. **open(Comparable) 回退**：HashMap / ConcurrentHashMap 的 `comparableClassFor@21` 各有一个上下文「接收者超过 64 个」，原因是键来源为 open(Comparable)（树化桶等）。开放世界下用户类可实现 `Comparable<T>` 且带签名，运行期必须保留 `ClassRepository.make` → `SignatureParser.parseClassSignature` 的解析路径。档案只依赖 JDK 侧事实（核心原则 2），不能以映像缓存替代。

所以 U13 的收益限于运行期：热路径 JDK 类首次树化时不再解析签名，映像多 348 个对象（HelloWorld）。闭包规模不变。

**未完成 / 待续**

- 在 b558e0c2 之后、batch-1008 修好发散的头上重测类数与映像数字（命令同上：`rava closure` 两例 + `--flows @concrete`）。
- e2e 运行期验证未做：需验证启动序列的 `__set_genericInfo` 写入可编译，且 `HashMap` 树化等用到 genericInfo 的测试输出不变。按合批测试流程，随下一批验证分支统一测。
- 单元测试作业 u13-ut-41d45f3c（jp2）在报告时仍在运行。b558e0c2 的发散可能影响依赖闭包的单测，判读时需与基线同口径对照。
- 若要让 `sun/reflect/generics` 出闭包，前提是同时消除上面两条路径。注解成员签名解析需要把注解元数据的构建期求值（java_meta）与 `AnnotationParser` 的可达性一并改造；open(Comparable) 路径在开放世界档案下不可删除。这两项是独立课题，不属于 U13 范围。后续量化见 §5.6.10。

#### 5.6.10 U13 后续：注解成员签名解析出闭包——新基线量化，不实施（2026-10-08，分支 `annot-sig`，基于 batch-1010 65d4882b）

作业：`as-cut-3f4fee31`（jp2，5 例基线 + `--cut-sets 'annsig annall sigall'`）、`as-gates-3f4fee31`（sg2，`--gates hello collectors deepcopy`）。产物在 `cluster_results/job/<tag>/01/build/ccomp/`。作业脚本 3f4fee31 起接受仓库内 `.java` 路径作用例（产物名取小写类名）。

切除集（`scripts/closure_composition_cuts/`）：

- `annsig`：只切 `AnnotationParser.parseSig` 方法体，即「注解成员签名在构建期折叠」的上界；
- `annall`：切 `AnnotationParser` 全部入口（`parseAnnotations` / `parseSelectAnnotations` / `parseParameterAnnotations` / `parseMemberValue`），即「注解元数据全部构建期求值、运行期不解析字节」的上界；
- `sigall`：`parseSig` 加 `Class.getGenericInfo`，用于归因 `sun/reflect/generics` 整包。后者是 open(Comparable) 回退路径，开放世界下不可删，这组只作归因。

| 例 | 基线 类 / 方法 | annsig Δ 类 / 方法 | annall Δ 类 / 方法 | sigall Δ 类 / 方法 |
|---|---:|---:|---:|---:|
| HelloWorld | 2178 / 13044 | −1 / −21 | −35 / −212 | −50 / −273 |
| CollectorsDemo | 2178 / 13051 | −1 / −21 | −35 / −212 | −50 / −273 |
| DeepCopy | 3573 / 22505 | 0 / −2 | −31 / −198 | −33 / −246 |
| TestAnnoReflect | 2182 / 13051 | 0 / −2（+1） | −28 / −179 | −49 / −254 |
| TestAnnoDeepAccess | 2189 / 13068 | 0 / −2 | −22 / −161 | −49 / −254 |

各组切除后 `sun/reflect/generics` 剩余类数：annsig、annall 都是 47（不变）；sigall 剩 4（DeepCopy 剩 18）。

**结论：注解成员签名折叠的健全收益是 0 类、至多 −2 方法，不实施。** 依据：

- HelloWorld / CollectorsDemo 的 −1 类 −21 方法是切除不健全造成的假象，不是可兑现的收益。
  - 切掉 `parseSig` 后返回值集为空，注解类型 `Class` 流不出来，于是 `AnnotationType.getInstance` → `AnnotationType.<init>` → `Method.getDefaultValue` 一支失去接收者；`[annotation]` 种子触发成员（`getRawAnnotations`）随之不可达，`Class$AnnotationData` 与 `Class.createAnnotationData` / `getDeclaredMethods` 等 19 个方法一起消失。
  - 健全的构建期折叠仍会产出注解类型 `Class`，这些类与方法照样在闭包里。
  - 剩下的 2 个方法是 `SignatureParser.parseTypeSig` 与 `AnnotationParser.toClass`，DeepCopy 与两个注解用例实测也正是 −2。
- `sun/reflect/generics` 47 类在 annsig 下一个不少。原因是 open(Comparable) 回退（§5.6.9 第 2 条）独立地把 `ClassRepository.make` → `SignatureParser.parseClassSignature` 及整棵签名树拉进来，`parseSig` 只多用了其中的 `parseTypeSig` 一个入口。两条路径一起切（sigall）才回收 43–49 类；而 open(Comparable) 路径在开放世界档案下不可删（核心原则 2）。
- 整条注解解析（annall）的上界是 −22 至 −35 类，主要是 `java/util/stream` 12 类（注解解析体内的流式处理带入）、`sun/reflect/annotation` 6–10 类、`java/lang/annotation` 2–6 类。
  - HelloWorld / CollectorsDemo / DeepCopy 里注解解析的唯一来路是 `Method.invoke` → `Method.isCallerSensitive` → `Reflection.isCallerSensitive` → `isAnnotationPresent(CallerSensitive.class)`，起点是 `ServiceLoader$ProviderImpl.invokeFactoryMethod`（日志链 `LoggerFinderLoader`）。
  - 这条来路属于 `Method.invoke` 全入口直连（`reflect-direct`）。直连后若这条来路不再经 `Method.invoke` 体，三例的 −35 / −31 类可能随之兑现（以复测为准），无需单独改造注解机制。
  - 带注解反射的用例（TestAnnoReflect / TestAnnoDeepAccess）运行期确实要解析用户注解。在开放世界档案下，`AnnotationParser` 必须保留，以保证运行期观察与 JVM 一致；它们的 −22 / −28 类不是本项可回收的量。
- 以手写取代 `parseSig`（描述符直译为 `Class`）属于性能替换，不满足手写准入（C1d 1e623cec 已删除 76a260a0 的同类手写），不作为备选方案。
  - `Class::__from_descriptor_checked`（`runtime/java_runtime/src/java/lang/class_impl.rs`）是那次手写留下的辅助函数，目前无调用方，可以随下一次手写层清理一并删除。

后续入口：

- `reflect-direct` 合入后，在新头上用 `closure_composition_job.sh --cut-sets 'annsig annall' hello collectors deepcopy` 复测。若 HelloWorld 的基线已不含 `AnnotationParser`，本项自然关闭。
- 若届时 annsig 在某个用例上的 Δ 类 > 0（前提是 open(Comparable) 路径已经不再首达 `sun/reflect/generics`，例如档案构成变化），再按终态方案实施：沿 U13 的 `image_memo_fields` 机制，把构建期已初始化类上的注解缓存（`Class.annotationData`、`Class.annotationType`）物化进映像。注解实例是动态代理，需先给映像导出加上「代理实例 + `AnnotationInvocationHandler`」的可物化判定。生成器内不写类名，字段在清单中声明。

### 5.7 第 5 步与第 6 步的 JceSecurity 部分：jimage 嵌入、`${java.home}` 虚拟树、`#[jvm_boundary]` 14 → 0（2026-10-08，分支 `boot-image-s5`，基于 batch-1008 b558e0c2）

提交：82c49923（jimage 嵌入 + ClassLoader / BootLoader 回字节码 + U14 钉值）、f196dd71（JceSecurity 回字节码 + 虚拟树 NIO）。本节只有本机 `cargo check` 结论，运行期结论待合批验证（§5.7.5）。

#### 5.7.1 jimage 嵌入（T3 / T4 / T7）

- **写侧**：发射层 `emit/project/jimage.rs` 把闭包读取的模块资源（档案侧 + 用户推导，带所属模块）写成标准 jimage：小端 7 字头（magic `0xCAFEDADA`）、redirect / offsets 表、位置属性、MUTF-8 字符串表、完美哈希（`0x01000193` 乘法哈希，与 `ImageStringsReader.hashCode` 同规则），以及 `/modules/<模块>/<目录>` 与 `/packages/<包>` 两棵目录树。单测 `jimage_tests.rs` 按 JDK 读侧规则（`BasicImageReader.getLocationIndex` / `ImageLocation.decompress`）回读全部位置，覆盖 2000 名完美哈希。
- **嵌入**：映像作为用户侧元数据 `MODULE_IMAGE`（8 字节对齐）发射，运行时 `meta::module_image()` 取切片。语料模式每测试各带一份（只含该档案读取的资源），可接受。
- **读侧**：唯一手写是 `NativeImageBuffer.getNativeMap`（类 ①，`ACC_NATIVE`）：路径为 `${java.home}/lib/modules` 时返回映像的 `DirectByteBuffer(addr, len)`（即 JNI `NewDirectByteBuffer`），否则 null。`BasicImageReader` 走 MAP_ALL 分支，不开 FileChannel；`ImageReader` / `SystemModuleReader` / jrt 协议全部按字节码执行。
- **删除**：`closure_tables`（模块服务表、`closure_pool`）、输入侧 `module_resources` 表、`seeds.toml [services] population` 与分析器的 population 循环。引导层服务目录由构建期 `Module.defineModules` 登记进映像（与分析器按 boot layer `provides` 建的目录同源）。`EmbeddedClassPath` 只承载类路径资源（含 `META-INF/services`）。

#### 5.7.2 ClassLoader 6、BootLoader 2 回字节码

| 删除的手写 | 去向 |
|---|---|
| `ClassLoader.getResource` / `getResources` / `getResourceAsStream`（实例 3） | 字节码：父委派 → `BootLoader.findResource(s)` → `ClassLoaders$BootClassLoader` / `BuiltinClassLoader.findResource` → 模块资源经 `SystemModuleReader`（jimage），类路径资源经 `URLClassPath`（`EmbeddedClassPath` intrinsic） |
| `ClassLoader.getSystemResource` / `getSystemResources` / `getSystemResourceAsStream`（静态 3） | 字节码：`getSystemClassLoader()` 后同上 |
| `BootLoader.getServicesCatalog` | 字节码：读 `BootLoader.SERVICES_CATALOG`，内容由构建期 `Module.defineModules` 写入映像 |
| `BootLoader.findResourceAsStream` | 字节码：`ClassLoaders.bootLoader().findResourceAsStream` → `SystemModuleReader.open` → jimage |

`ClassLoader`、`BootLoader` 移出 `[vm_boundary]`；`ClassLoader$ParallelLoaders` 移出 `translate_nested`。`boot_loader_impl.rs` 只剩 3 个 native（`getSystemPackageNames`、`getSystemPackageLocation`、`setBootLoaderUnnamedModule0`）。

#### 5.7.3 U14：`java.home` 构建期钉值与 `${java.home}` 虚拟树

- **决定（用户 2026-10-08，U14）**：`java.home` 构建期钉值，原生二进制不依赖宿主 JDK 安装；`conf/security` 等由构建期嵌入。`java.home = /java-runtime`（`jdk_resources::JAVA_RUNTIME_HOME`），`sun.boot.library.path = /java-runtime/lib`。钉值在 `vm_intrinsics.toml` 的 `vm_props` 与 `system_properties.values` 中声明（两处同值），并从延迟属性列表移除；生成器代码无属性名字面量。`line.separator`、`file.encoding` 的钉值不在本步范围；`sun.jnu.encoding`、`stdout/stderr.encoding` 仍按 U1 运行期取宿主值。
- **虚拟树**（`runtime/java_runtime/src/jdk_resources/tree.rs`，只读）：

| 相对 java.home | 内容 |
|---|---|
| `conf/security/java.security` | 嵌入的安全属性文本（原有） |
| `conf/security/policy/{limited,unlimited}/*.policy`（5 个） | 参考 JDK 21 / 25 原文件（两版逐字节相同），`include_bytes!` |
| `lib/modules` | 本程序 jimage（`meta::module_image`） |
| `lib/tzdb.dat` | 时区数据（原有） |

  目录为文件路径的各级前缀；stat 形态为目录 `S_IFDIR|0555`、文件 `S_IFREG|0444`；写 / 建 / 截断得 `EROFS`，树内不存在得 `ENOENT`。虚拟 fd 从 -2 递减，虚拟 `DIR*` 句柄为负 i64，与真实空间不碰撞。
- **NIO native 接入**：路径落入树内或句柄为虚拟句柄时，读嵌入数据，否则走宿主系统调用。
  - `UnixNativeDispatcher` 共 15 个：open0、openat0、close0、read0、stat0、lstat0、fstatat0、fstat0、access0、dup、opendir0、fdopendir、readdir0、closedir、realpath0。
  - `UnixFileDispatcherImpl` 共 4 个：read0、pread0、seek0、size0。
  - `UnixFileSystem.getBooleanAttributes0`。
  - 这些都是类 ① native 的既有手写，不新增手写类别。

#### 5.7.4 JceSecurity 6 回字节码（第 6 步的 JceSecurity 部分）

删除 `javax/crypto/jce_security_impl.rs` 的 6 个 `#[jvm_boundary]`：`canUseProvider`、`isRestricted`、`getVerificationResult`、`getDefaultPolicy`、`getExemptPolicy`、`verifyExemptJar`。JceSecurity 移出 `[vm_boundary]` 与 `clinit_carried`，整类按字节码翻译：

- `<clinit>` → `setupJurisdictionPolicies`：先读 `Security.getProperty("crypto.policy")`（java.security 中为 `unlimited`），再取 `StaticProperty.javaHome()`。
- 之后经 `Files.isDirectory`（stat0）/ `isReadable`（access0）/ `newDirectoryStream` / `newInputStream` 读虚拟树中的 policy 文件，解析出 `defaultPolicy` / `exemptPolicy`。目录流在 Linux 上是 open + dup + fdopendir，在 macOS 上是 opendir。
- `getVerificationResult` → `ProviderVerifier.verify`：OpenJDK 构建 `savePerms=false`，直接返回。

管辖策略不是手写常量，而是字节码在运行期解析嵌入文件的结果。全仓 `#[jvm_boundary]` 现为 **0**（14 = ClassLoader 6 + BootLoader 2 + JceSecurity 6，全部删除）；`[vm_boundary] classes` 只剩 `java/lang/Class`。

**未做（第 6 步其余部分）**：非引导类的构建期初始化（C3 `build_time_init`）。JceSecurity 仍在运行期初始化：首次用 JCA 时 `<clinit>` 读虚拟树。待 `build_time_init` 落地后，可把 `isRestricted=false` 等事实折进映像，以收窄闭包。

#### 5.7.5 风险与待验证

- **闭包增长**：JceSecurity 字节码可达后，`ProviderVerifier.verifyExemptJar` 一侧（JarURLConnection / JarFile）、`<clinit>` 中的 `new URL("http://...")`（http 协议处理器）、`newDirectoryStream` 的 glob → 正则，都可能带入新类。需实测 JCA 例的闭包规模；收窄依赖第 6 步的 `build_time_init`。
- **引导期读树**：java.home 钉为 `/java-runtime` 后，构建期 initPhase2 若触碰 `${java.home}/lib/modules`，求值器会报未登记 native。预期不会（`SystemModuleReader` 惰性），需在 HelloWorld 引导审计中确认。
- **其他待实测**：`DirectByteBuffer(long,long)` 私有构造是否被闭包保留；`BasicImageReader` 的 FileChannel 分支是否膨胀闭包；映像中的服务目录是否列出闭包外的提供者。
- **既有缺口（非本步）**：JDK 25 `UnixFileDispatcherImpl.available0` / `isOther0` 的 native 未实现。
- **待验证清单**：
  - 生成器单测：`jimage_tests` 与 emit / input / resolve / closure 全量单测。
  - e2e：TestBootLayer、TestClassResourceStream、TestStringGetCharsLegacy、TestAppClassLoader、TestModuleLayerDefine，以及 ServiceLoader 相关例。
  - JCA：TestAesGcmRound、TestCipherDesModes、TestMacHmacDigest、TestRsaSignVerify。
### 5.9 U12 / U14 实测（2026-10-08，分支 `u12-props`，基于 batch-1008 b558e0c2）

用户 2026-10-08 定（§8.4）：U12 作为 U1 的例外，只限 (L) 链（`Runtime.exit` / `Shutdown.exit` → `logRuntimeExit`），落地 §5.5.6 的机制 ①（构建期确定 LoggerFinder 提供者）和 ③（`isLoggable(DEBUG)` 按映像值折叠）；U14 把 `line.separator` 按目标三元组钉值、`file.encoding` 钉为 UTF-8。约束是每项都测闭包类数，没有收益的不钉值。

#### 5.9.1 测量口径

- b558e0c2（batch-1008）在服务器上的闭包分析 OOM，属于合批回归，不是本分支引起的。因此对照在 **c614f840 + 本分支改动的移植补丁**上测：c614f840 缺 `jdk_resource` / `class_path_files`，补丁只保留 `module_has_resource`。测量环境是 kr1，Linux JDK 21.0.11，指标为 `rava closure` 的 summary.classes / methods，并附 `--boot-report`。
- 变体：
  - A：基线；
  - E：`--cut logRuntimeExit`；
  - B：清单 `[concrete.boot] calls` 在 initPhase3 后追加 `System$LoggerFinder.getLoggerFinder`；
  - Bjh / Bjr：B 加上 `java.home` 钉值，分别钉为参考 JDK 路径和 `/java-runtime`；
  - ls1 / ls2 / fe：`line.separator` 两种钉法，以及 `file.encoding`。

| 变体 | HelloWorld 类 / 方法 | CollectorsDemo 类 / 方法 | 映像（residual_calls / objects / inited） |
|---|---|---|---|
| A 基线 | 3043 / 18,093 | 3043 / 18,125 | 2 / 8385 / 256 |
| ls1、ls2（line.separator 钉值） | 3043，类集合差 0 | 3043，类集合差 0 | — |
| fe（file.encoding） | 3043，类集合差 0 | 3043，类集合差 0 | — |
| E（切 logRuntimeExit） | 3043，类集合差 0 | 3043，类集合差 0 | — |
| B（U12 ①） | 3043 / 18,092 | 3043 / 18,124 | **3** / 8407 / 260 |
| Bjh / Bjr（B + java.home 钉值） | 3043 / 18,092 | 3043 / 18,124（Bjh） | **3** / 8407 / 260 |

#### 5.9.2 U14 结论：不钉值

- `line.separator` 与 `file.encoding` 钉值后，两个主测例的类集合都没有变化（差 0）。按“没有收益的不钉值”，这两项**本分支不钉**：`vm_intrinsics.toml` 的 `platform_props` 仍是 `@deferred`，`[facts.system_properties] dynamic` 仍含 `line.separator`。
- `file.encoding` 本来就经 `[facts.system_properties.values]` 与 SystemProps 字节码取 `"UTF-8"`；平台侧的 `file_encoding`（JDK 21）只用来供给 `native.encoding`，钉它没有闭包意义。
- 将来钉值时（例如 §5.9.4 的大集合收窄后收益显现），沿用 boot-image-s5 对 `java.home` 已用的同一机制：
  - `[concrete.boot.platform_props] line_separator = "\n"`；
  - 从 `[facts.system_properties] dynamic` 中移除；
  - 在 `values` 中加 `"line.separator" = "\n"`。
  
  Windows 目标的 `"\r\n"` 随 Windows 目标支持按三元组给出。jnu 编码与 stdout / stderr 编码保持运行期读取。

#### 5.9.3 U12 ① 结论：构建期跑不完，残差化反而有害，清单不登记

- B 求值成功（status ok），但 `getLoggerFinder` 成了**残差调用**。原因链（boot report 原文）：`accessProvider` → `AccessController.doPrivileged(PA, null, Permission[])` → `createWrapper` → `AccessControlContext.<init>` → `FilePermCompat.newPermPlusAltPath` 读静态字段 `compat`。为此要运行 `FilePermCompat.<clinit>` → `SecurityProperties.privilegedGetOverridable` → `Security.<clinit>`，后者读 `StaticProperty.JAVA_HOME`。`StaticProperty` 因启动重放改写 `ConcurrentHashMap$Node.val`（user.dir / user.home 等宿主相关属性）而属于运行期初始化类，于是 `Security` / `SecurityProperties` / `FilePermCompat` 相继成为运行期初始化类，整个调用被残差化。
- **只钉 `java.home` 不够**（Bjh、Bjr 的 residual 仍为 3），因为判定的粒度是整个类：`StaticProperty` 只要有一个字段依赖宿主，所有静态读取都是占位对象。
- 残差化的危害：残差调用会成为启动根（why 链显示 `[boot_image] 根 System$LoggerFinder.accessProvider`）。启动时会急切执行 `LoggerFinderLoader.service()` → `ServiceLoader` / `SimpleConsoleLogger` / `StackWalker` 链，比 HotSpot 只在首次记录日志时才加载更差。所以 a943b9c7 的登记行已撤销，清单里改为注释说明。保留的两个引导 native 作为 ① 的基础设施，与运行期语义一致，不改变现有闭包：
  - `AccessController.getProtectionDomain` → `const:null`（1af54b60，与运行期 `access_controller_impl.rs` 相同）；
  - `SystemModuleReader.containsImageLocation` → `module_resource:module`（8782f786，按参考 JDK 模块内容回答，构建期不读 `${java.home}/lib/modules`）。
- **① 落地前提**（按依赖顺序）：
  1. `java.home` 钉值（boot-image-s5 已在做，`/java-runtime`）；
  2. **运行期初始化类的字段粒度读取**：`StaticProperty.JAVA_HOME` 等不依赖宿主的静态字段在构建期按映像值读取，只有被重放改写的字段才给占位对象。这是本项的核心缺口，改动点在求值器的运行期初始化判定与静态读取（`engine/concrete`，报告的「运行期初始化类的静态读取（占位对象）」一节列出了命中点）；
  3. 构建期读取 `${java.home}/conf/security/java.security`（s5 的虚拟树 `/java-runtime`），让 `Security.<clinit>` 能在构建期跑完。
- **语义问题（落地前必须先解决）**：`LoggerFinderLoader.service()` 末尾调用 `BootstrapLogger.redirectTemporaryLoggers()`，会把 `logManagerConfigured` 置为 true。如果构建期跑完 service()，后端为 JUL_DEFAULT 时运行期 `getLazyLogger` 的 `useLazyLoggers()` 会变成 false，转而走 `getLoggerFromFinder` → `LoggingProviderImpl` → JUL `LogManager`。这与 HotSpot 的 exit 路径（SurrogateLogger，不加载 LogManager）不一致，也与 ③ 按 surrogate 级别折叠的前提冲突。① 的终态必须只把「提供者已确定」这一事实放进映像，同时保持 `logManagerConfigured` 与 HotSpot 启动后的状态一致（例如只求值 `DetectBackend` 与提供者查找，不跑 `redirectTemporaryLoggers`），求值时还需要核对 HotSpot 进程在 main 前是否已经执行过 service()。

#### 5.9.4 U12 ③：未做，续作入口

- 折叠点是 `Shutdown.logRuntimeExit@15 ifeq`（`isLoggable(DEBUG)`）。终态链路：
  - 后端为 JUL_DEFAULT 时 SurrogateLogger 的级别是常量 `JUL_DEFAULT_LEVEL = INFO`；
  - `jdk.system.logger.level` 只影响后端 NONE 的 SimpleConsoleLogger；
  - DetectBackend 的判定：SCL 上找到 LoggerFinder 时为 CUSTOM；否则有 java.logging 的 DefaultLoggerFinder 时为 JUL_DEFAULT / JUL_WITH_CONFIG；都没有时为 NONE。
- 实现方式是 ② 式的原始类型静态字段折叠：映像值与全部可达 putstatic 取并，`useSurrogateLoggers` / `logManagerConfigured` 得到常量后，经 `getLazyLogger` 分支收窄到 surrogate，再按常量级别折叠 `isLoggable`。前提是 ① 的映像状态满足 §5.9.3 的语义问题。
- 预期收益只能在 §5.9.5 的大集合解除后度量：当前 E（直接切断整条 logRuntimeExit）的类差也是 0。

#### 5.9.5 468 与 3043 的差距（只记录，本分支不修）

- c614f840 上 **E = A = 3043**，类集合差 0：(L) 链已经不是 HelloWorld 大集合的唯一持有者（§5.6 第 5 条记录的「切 logRuntimeExit → 598」已不成立）。因此 U12 / U14 当前都测不出类减量。
- `--why` 指向的其他持有根（hw_Ew，类集合与 hw_Bjh 相同）：
  - `java/util/regex/Pattern`：`[boot_region] 根 Charset.isSupported` → `Charset.checkName` → `String.charAt` → `checkIndex` → `Preconditions.outOfBoundsMessage` → `String.format` → `Formatter.<clinit>` → `Pattern`（越界异常消息路径）；
  - `java/util/logging/LogManager`：`[hw-type] Proxy` → `Proxy$Dyn.__vm_proxy_invoke` → `AnnotationInvocationHandler.toStringImpl` → Stream 并行归约 → `Spliterator$OfDouble.forEachRemaining` → `Tripwire.trip` → `PlatformLogger.getLogger` → `LazyLoggers` → `LoggingProviderImpl` → `LogManager`；
  - `java/security/SecureRandom`：`createWrapper` → `FilePermCompat` → `SharedSecrets.ensureClassInitialized` → `Lookup.ensureInitialized` → `Module.isExported` → `WeakPairMap.expungeStaleAssociations` → `ConcurrentHashMap.addCount` → `ThreadLocalRandom.<clinit>` → `SecureRandom`；
  - `javax/crypto/Cipher` 不在闭包内。
- 收窄方向（交给对应的收窄线，不在本分支做）：异常消息路径的 `String.format`、Proxy 反射调用的 `toString`、`Tripwire`、`ensureClassInitialized` 链（闭包收窄 ③ 线）。

#### 5.9.6 与 boot-image-s5 的重叠

- `java.home` 的钉值机制（`vm_props` + `system_properties.values`，并从 dynamic 列表移除）由 s5 落地；U14 将来钉值时用同一组 TOML 段，不另起新段。
- 本分支的 `module_resource` 引导 native（构建期回答模块资源是否存在）与 s5 的 `getNativeMap`（运行期）互补，不冲突。
- 两边都没有处理「运行期初始化类的字段粒度读取」（§5.9.3 前提 2）。
- 合批时 `vm_intrinsics.toml` 的 `[concrete.boot.natives]` 段可能有文本冲突，两边条目并存即可。

### 5.8 第 6 步：非引导类的构建期初始化（C3 `build_time_init`，2026-10-08，分支 `boot-image-s6`，基于 batch-1009 d5a2cb5d）

#### 5.8.1 设计

引导映像导出后**保留求值器**（`Engine.ext_vm`）。分析中首次初始化、且不在映像构建期初始化集合中的类（用户类与 JDK 类同一规则，无类名特判），先交给求值器尝试在构建期执行 `<clinit>`：
- 成功：结果追加进映像，类按构建期初始化处理（不展开 `<clinit>`，静态字段取映像值，已出现的字段节点补传播）；
- 失败：照旧在运行期初始化，原因记入 `summary.build_time_init.failed`。

**与程序无关（档案约束）**。档案内各入口的映像引导部分逐字节相同，所以扩展结果按**扩展组**组织，组键与程序无关：
- `c:<类>`：该类静态字段可达、归属该类的对象，该类的静态字段，以及重定位槽；
- `s:<内容>`：扩展期新建的驻留字符串及其内容数组；
- `m:<类型>`：扩展期新建的类镜像。

单个程序按发现次序追加扩展组，分析结束后 `canonicalize()` 按键排序重编号（只动扩展对象，引导对象下标不变）。档案对每个入口做 `absorb()`：同键复用，且内容须相同，不同即报错；新键追加。全部并入后再规范化。代码：`closure/src/image_ext.rs`，数据结构 `IGroup`，`ImageData.ext_base` / `ext_steps` / `ext`。

**尝试隔离（在线检查，`engine/concrete/ext_init.rs`）**。程序之间只差「哪些类已初始化、以什么次序」，所以一次尝试必须与此前的扩展类隔离，也必须与运行期可变的状态隔离：
- **读**：
  - 它类静态字段只许读 final / 只在 `<clinit>` 写的字段（`init_only`）和内存缓存字段；
  - 它类对象只许读不变字段、内存缓存字段、类镜像，以及稳定类型的字段；
  - 它类数组只许读冻结的：字符串内容，或经稳定类型取到的数组。
- **写**：只许写本类静态字段（`putstatic` 与 `set_static` native）和本次尝试新建的对象。内存缓存字段的写入在外层尝试结束时撤销。
- **初始化**：
  - 依赖类嵌套尝试，各在自己的日志标记下进行；依赖失败则本类失败。
  - 依赖引导中转为运行期初始化的类，或形成初始化环，即失败。
- **身份哈希**：本类对象取 `fnv32("类#序号")`，共享对象取 `fnv32(键)`，其余对象失败。
- **效果**：成功后核对残差、VM 侧登记、占位 / 延迟值、宿主来源、模块表、污点、VM 单元等计数，任何增长即失败。`<clinit>` 抛异常即失败（运行期照常抛）。
- **导出**：静态字段可达的对象不得含污点、lambda、占位、延迟或宿主对象，也不得引用映像之外的对象（`ext_export.rs`）。
- **失败时**：回滚到尝试标记，撤销嵌套成功的类（`done` 截断、认领撤回）；回滚本身失败时置 `broken`，此后不再尝试。

**发射侧**：扩展对象、静态字段、构建期初始化类与重定位步骤沿用引导映像同一套物化路径（`boot_image/start.rs`），发射器不需要改动。

**根模块限制（23637bec）**：映像物化在根门面 crate（根类所在模块），所以扩展类与扩展组对象的类型只能属根模块。用户类、其他 jmod 模块的类目前一律在运行期初始化（原因「不在映像根模块」）。否则分析按构建期初始化处理，发射侧却因类不在根 crate 而跳过静态字段与初始化标记，运行期会执行未入闭包的 `<clinit>` 链。

终态是按 crate 分段物化：各模块 crate 与用户 crate 各带一段扩展映像，引用上游段的对象；启动序列按 crate 依赖序登记。用户类扩展组随用户 crate 生成，不进档案。

#### 5.8.2 实测（服务器 us1，JDK 21.0.11，`scripts/closure_composition_job.sh hello collectors jcasasl`；基线 d5a2cb5d 与 c1b77e0a 同属作业 `bimg6-m1-c1b77e0a`，加根模块限制后的 23637bec 为作业 `bimg6-m2-23637bec`）

| 用例 | 类 d5a2cb5d → c1b77e0a → 23637bec | 方法 | 构建期初始化成功 / 运行期（23637bec） | java/util/regex 类 |
|---|---|---|---|---|
| HelloWorld | 2178 → 2148 → **2148**（−30） | 13047 → 12301 → 12313（−734） | 1328 / 440 | 61 → 61 |
| CollectorsDemo | 2178 → 2148 → **2148**（−30） | 13054 → 12298 → 12310（−744） | 1328 / 440 | 61 → 61 |
| TestJcaSasl | 2276 → 2245 → **2246**（−30） | 13606 → 12819 → 12861（−745） | 1355 / 500 | 61 → 61 |

- 删除的类全部是减项，没有新增类：BreakIterator 链 14 类（`sun/text/*`、`java/text/BreakIterator*`）、Vector / Stack 5 类、`HexFormat`、`Duration`、`Invokers$Holder`、`VarHandleGuards` 等。
- **第 5 步风险压回**：JarFile、http 协议处理器、`Globs` 在基线 d5a2cb5d 已为 0（JceSecurity 字节码可达没有带入），本步后仍为 0。
- **验收「CollectorsDemo 冷独占正则链 0 类」未达成**：`Formatter.<clinit>` 的唯一失败原因是映像含 lambda 对象。`Pattern.compile` 的节点持有 `Pattern$BmpCharPredicate` lambda（`Pattern.lambda$Single$14`）。`FloatingDecimal$HexFloatPattern`、`Period` 的失败原因相同。
- `#[jvm_boundary]` 保持 0（本步不涉及手写层）。

失败根因（HelloWorld，c1b77e0a 共 400 类，含嵌套传递；23637bec 另有 68 类因「不在映像根模块」转运行期，`summary.build_time_init.reasons` 已按根因归并）：

| 类数 | 根因 | 终态处理 |
|---|---|---|
| 83 / 77 / 4 / 3 | 读 `SharedSecrets.javaLangAccess` / `javaLangRefAccess` / `javaNioAccess` / `javaLangInvokeAccess`（非 final、由 setter 写入） | 「一次写入」字段事实：所有写入点均为声明类内只写该字段的 setter，且 setter 的调用点只在 `<clinit>` 或引导阶段方法中（JDK 侧全局调用点扫描，与程序无关） |
| 71 | 读 `System.props` | 正确拒绝：属性在运行期取宿主值（U1 / U14）。下游的 `Debug`、`ThreadLocal`、`ZoneRulesProvider` 等随之运行期初始化 |
| 27 / 11 | 写 `SharedSecrets.*Access`（类 `<clinit>` 注册自身访问器） | 与上一项同一事实：写入目标是「一次写入」字段且当前为空时，作为本类效果随组导出 |
| 18 / 12 / 7 / 3 / 2 | 无具体语义：`AtomicLong.VMSupportsCS8`、`Class.getDeclaredFields0`、`MethodHandleNatives.resolve`、`StackStreamFactory.checkStackWalkModes`、`System.currentTimeMillis` | 前四项补 `[concrete.boot]` native 语义；`currentTimeMillis` 正确拒绝 |
| 12 / 8 | 初始化环（`KnownOIDs`、`IsoFields$Field`） | 环内的类整体作为一个尝试单元 |
| 9 | 读 `Enum.hash`（缓存的身份哈希） | 引导对象的身份哈希已定，`Enum.hash` 按内存缓存字段处理 |
| 3 | 映像含 lambda 对象 | 见 §5.8.3 |

#### 5.8.3 遗留：映像中的 lambda 对象（正则链验收的唯一阻塞）

映像格式要增加 lambda 对象 `IBody::Lam { site, captured }`，其中 `site` 为调用点（类、方法、描述符、pc）。分析器的 `Lam` 已有 `imp`、`bargs`、`desc`、`captured`，还要补记调用点。

发射侧：翻译器对映像列出的调用点，在类上额外生成站点工厂 `__lambda_site_<pc>(捕获值…) -> Result<Object>`，函数体复用 `instr/src/sim/dynamic/lambda.rs` 的同一套降级（`call_args` / `closure_body` / `boxed_closure`），捕获值改为形参。启动序列以映像中的捕获值调用工厂。隐藏类名、SAM 账本登记都与站点同源。

此项跨分析器、映像格式、翻译器和发射器。由于档案映像各测试共享，发射侧缺陷会影响全部用例，因此单独成步，须先做全量单测与抽查。

#### 5.8.4 待验证与风险

- **待验证**：
  - closure / emit / input 全量单测；
  - e2e 抽查 HelloWorld、CollectorsDemo、DeepCopy、TestJcaSasl 与 JCA 四例，确认扩展组物化正确，被删类无运行期回落；
  - `scripts/seed_check.sh` 两次生成确定性；
  - 语料档案 `absorb` 不报「扩展组与其他入口不一致」。
- **风险**：
  - 扩展类在 `image_init` 之前若已有缓存的静态初值，会以旧值参与传播。已对已出现的字段节点调用 `image_field` 补传播，但未覆盖其他缓存。
  - `Rc::make_mut` 在映像数据被共享时会整体复制（性能项）。
  - final 静态字段若之后被 native 改写，读取结果会过期（与引导映像同一前提）。
- **恢复入口**：
  - `engine/image_start/ext.rs`（接入）、`engine/concrete/ext_init.rs`（隔离规则）、`engine/concrete/ext_export.rs`（组导出）、`closure/src/image_ext.rs`（合并 / 规范化）；
  - 根因收窄按 §5.8.2 表逐项推进，每项用同一作业命令复测类数。

#### 5.8.5 与 U13 镜像缓存的整合：映像追加对象统一为扩展组（2026-10-08，合入 batch-1010 35d936c4）

合并前有两套「往引导部分之后追加对象并重排编号」的机制：U13 的 `image_finish`（`m.base` 之后按镜像排序重排 memo 块）与本步的 `canonicalize`（`ext_base` 之后按组键重排扩展组）。二者若并存，`image_finish` 的重排会丢掉交错其间的扩展对象，`canonicalize` 遇到不属于任何组的 memo 对象无法编号；U13 还把缓存值写进镜像对象体，镜像是引导对象时引导部分随程序而变，档案 `boot_eq` 必然失败。终态取第一种做法：**镜像缓存也是与程序无关键的扩展组**，只保留 `canonicalize` / `absorb` 一套流程，`image_finish` 删除。

- **组键**：`f:<镜像类型>#<声明类>.<字段>`（`image_ext::memo_key`）。缓存值只取决于镜像所代表的类，与求值实参、程序无关，所以同键内容相同，档案按键求并。组内对象是片段对象（广度优先次序），`nsteps = 0`。
- **缓存值不写进镜像对象**：`ImageData.mirror_memos: Vec<IMemo { mirror, decl, name, val }>`，每条与一个 `f:` 组一一对应，按组键排序。镜像对象体保持构建期 VM 的原样，引导部分逐字节与程序无关。分析侧镜像成为活对象时，`image_drain` 把该镜像的缓存（`MemoState::of`）与字段同一口径传播；发射侧直接取 `IMemo.val`。
- **新建镜像同经构建期求值器**：片段引用的类镜像若映像中没有，由保留的求值器 `ExtVm::mirror` 新建（`Vm::mirror`，VM 字段在新建时写定），经 `ext_append(…, roots)` 按 `m:<类型>` 组导出——与扩展期新建镜像同键同内容，扩展类之后再引用该镜像时直接复用（`Ext.ids` 已登记）。共享组（`m:` / `s:`）对象的身份哈希一律取键哈希 `fnv32(键)`（与扩展期查询同值），同键组内容与是否查询过无关。
- **追加对象接入分析**（`Engine::image_appended`，扩展组与镜像缓存组共用）：活标记扩容、新建镜像登记 `mirror_obj`；程序此前已取过的类镜像随即成为活对象（与引导镜像在程序取镜像时成为活对象同一口径），否则缓存值不传播、发射侧因镜像不活而不写缓存。
- **按对象值表**（batch-1009 6db384e8）：活对象内容传播（`image_drain`）对抽象对象的字段值另记入按对象值表；扩展组与镜像缓存组追加的对象同经 `image_drain`，镜像缓存值与镜像字段同一循环，一并记入，无需另行处理（重复传播按格并，幂等）。
- **可物化判定**（`image_memo_prepare`，取代 `image_memo_ok`）：静态字段引用规则不变；片段对象类型须属映像根模块（与 §5.8.1 根模块限制同口径）；所需镜像在此备好，失败即该组按冷 / 热并集入闭包。
- **规范化**（`ImageData::canonicalize`）：全部组按键排序重编号，`mirror_memos` 的镜像号与值随组重定位后按键排序；校验组键不重复、`mirror_memos` 与 `f:` 组一一对应。对象号只由组键集合决定，与发现次序（扩展尝试与具体求值的交错）无关；步骤号只有 `c:` 组的重定位步骤，随组排序。
- **档案合并**（`ImageData::absorb`）：同键组比对对象数、步骤数、对象内容；`f:` 组另比对重定位后的缓存记录（镜像号与值），不同即报「扩展组 … 与其他入口不一致（镜像缓存值）」。新键组连同其缓存记录追加。
- **单测**：`image::tests::json_roundtrip`（`ext` 含 `f:` 组、`mirror_memos` 四元组）；`image_ext::tests::memo_groups_absorb_and_canonicalize`（两种发现次序、两种合并次序结果相同，记录重定位正确）、`absorb_rejects_divergent_memo`（缓存值不同即拒绝；记录与组不对应即拒绝）。
#### 5.8.6 构建期初始化结局与处理次序无关（2026-10-09，分支 fix-1011）

**共同根因**：引擎是单调不动点，形参常量格 `pvals`、返回常量格 `rvals` 只升不降。只要某次分析看到「尚未定论」的状态并按值未知答复，未知值就永久并入格中，此后状态定论、给出更精确的值也无法撤回。哪次分析先于定论运行取决于处理次序，因此闭包会随哈希种子与 `--flow-batch` 变化。终态要求：凡答复依赖尚未定论的状态，一律答 ⊥（`Ret::Never` + `Dep::Never`，收尾阶段重算），或者在分析之前先定论；不靠排序掩盖。

本节修复了三处这样的「未定论 → 未知」：

1. **档位上下文登记 `<clinit>` 先于构建期初始化尝试**（5fcd911f，已合入 batch-1012 e5200a3e）
   - **现象**：TestModuleLayerDefine 的 `reflect_new_array_element_precision` 在不同种子下，`GB18030$Encoder` / `HKSCS$Encoder.<clinit>` 时有时无。
   - **根因**：`Engine::init` 先调用 `level_init` 登记类的 `<clinit>`，后尝试构建期初始化扩展（`image_init`）。如果类先经档位上下文到达，`<clinit>` 已入链，扩展随后成功也撤不回；如果先经 `init` 到达，扩展成功，`<clinit>` 就不入链。
   - **修复**：`init` 中先尝试构建期初始化，再调用 `level_init`；`level_init` 遇到未尝试过的类先尝试（`image_settled_build_time`），构建期已初始化的类不登记 `<clinit>`。

2. **getstatic 读静态字段时，声明类的构建期初始化尚未尝试**（7d56de55）
   - **现象**：seed-chain 上 `profile_union_key_and_coverage` 失败（`--flow-batch 1 --hash-seed 7` 时 content_digest 变化），差异是 `CharsetEncoder.onMalformedInput` [4,14] 的空值折叠。
   - **根因**：`StreamEncoder.<init>` 读 `CodingErrorAction.REPLACE` 时，若 `CodingErrorAction` 尚未尝试扩展，`field_value` 答复未知，`onMalformedInput` 的 P1 永久成为 Top。若扩展先完成，同一读取得到映像值，P1 为非空常量，[4,14] 折叠。
   - **修复**：
     - 方法体分析前，`image_settle_reads` 扫描 getstatic，对未初始化的声明类先尝试扩展。执行读取本就触发声明类初始化（JVMS §5.5），这里只是把尝试提前；只在死代码里读的类会多一次尝试，但结局与次序无关。
     - 每类只尝试一次（`ImgState.tried`），结局即定论。
     - 扩展成功时，`ceval_drop` 作废读过这些类静态字段的辅助分析记忆（辅助分析没有方法上下文，不经预先定论），并让取用者重算。

3. **键为拼接值的属性读取，候选模式尚未登记**（bfe1bbb9）
   - **现象**：修复 2 之后 seed-chain 仍有差异：`GetIntegerAction.privilegedGetProperty`（两个重载）、`Integer.getInteger(String)`、`GetIntegerAction.run` 的空值折叠在缺省次序下有，fb1 次序下没有。两种次序的 `pvals` 相同（作业 pd5）。
   - **根因**：读取点的候选模式 `pkeys[(方法, 键来源)]` 由 `prop_key_site` 在本次分析之后的调用事件里登记。`Integer.getInteger(String,Integer)` 首次分析时尚无登记，`System.getProperty(nm)` 答复未知，经 `Integer.decode` 的非空路径并入 `rvals`，从此撤不回。缺省次序下 `decode` 尚未分析，乐观的「不返回」答复恰好截断了这条路径，等到登记完成后重算才得到 null。
   - **修复**：`Ctx.pkeys_seen` 记录登记过的读取点，包括求不出模式的。未登记的读取点答复 ⊥ 并记 `Dep::Never`；调用事件照常发出，`prop_key_site` 首次登记时把该方法标脏重算。登记之后按模式求值；求不出模式时按值未知，与原先一致。定论阶段（`bottom_never` 为假）退回值未知，保证终止。

**作业**（服务器 jp2 / kr2 / jp1；`fix1011-*`）：
- dg1–dg3：Encoder 差异诊断。
- t1-5fcd911f：两项单测通过。
- ut-5fcd911f：全量单测只剩已知失败。
- pd1–pd6：缺省次序与 fb1 次序的 MinimalMain / NullView 折叠对照。pd3 / pd4 定位并验证修复 2，pd5 排除形参常量差异，pd6（叠加修复 3）两例 `folds equal True`，剩余只有 via 差异。
- scp2–scp4：seed-chain 叠加修复后跑两项单测。scp3（修复 1、2）上 profile 仍失败；scp4（修复 1–3，jp2）上 `reflect_new_array_element_precision` 与 `profile_union_key_and_coverage` 均通过。

**残留：具体求值站点的镜像缓存在站点回退后不撤回**（未修，fix-1011 头 fddb9bcb 上 `profile_union_key_and_coverage` 因此失败）
- **作业**：
  - ut2-fddb9bcb：全量单测。失败项为已知三项，加上本项。
  - pf3：档案对照。默认与 fb1 + seed 7 的 classes / methods 只有 via 不同；digest 差在 `boot_image_data.ext` / `mirror_memos`。
  - cd1：单例对照。两种次序的构建期初始化集合相同（1962 类，失败集合相同）。只有 MinimalMain 在默认次序下多出约 42 个枚举的 `m:<枚举>` 与 `f:<枚举>#java/lang/Class.genericInfo` 组；NullView 两种次序相同。
- **根因**：具体求值入口 `Class.getGenericInterfaces` 按接收者镜像逐组求值（`concrete_call`）。每次处理调用点时，对当前接收者集合中尚未应用的组合，经 `image_memo_apply` 把镜像缓存追加进映像。接收者集合随分析增长，超过 `COMBO_LIMIT` 或混入非镜像成员后，站点永久回退（`concrete.fallback`），但先前已追加的组不撤回。默认次序先以约 42 个枚举镜像的中间集合处理过该站点；fb1 次序在集合到达失败形态之前没有处理过它。站点最终都是回退，映像内容却不同。
- **终态方向**：
  - 未回退站点的已应用组合等于最终组合（集合只增，回退不可逆），残留只来自最终回退的站点。
  - 因此映像的镜像缓存组应等于「最终未回退站点」所贡献的组：分析结束时剔除只由回退站点贡献的 `f:` 组，以及只被这些组引用的 `m:` 组，然后再规范化。
  - 难点：剔除后要重编号；活标记与分析侧已记录的映像对象号要同步；`m:` 组可能被扩展期共享，需按引用判定。
  - 另一种做法是推迟镜像缓存入映像到站点定论之后，但那样会丢失分析期的缓存值传播（热轨迹精度）。
- seed-chain 叠加三处修复（scp4）时该测试恰好通过，属于次序巧合。

**恢复入口**：
- `engine/levels_boot.rs`：`image_settled_build_time`。
- `engine/image_start/ext.rs`：`image_ext`、`image_settle_reads`。
- `engine/worklist.rs`：`analysis()`。
- `engine/sysprops.rs`：`derived_result`。
- `engine/sysprops_key.rs`：`prop_key_site`。

#### 5.9.7 日志链续作（2026-10-08，分支 `log-chain`，基于 b9f47c33）

**测量口径**
- 测量环境：kr1 / us1，Linux，JDK 21.0.11。指标为 `rava closure` 的 summary.classes / methods。
- 「E」表示 `--cut AccessController.executePrivileged:(PrivilegedAction;AccessControlContext;Class)`，即切断 doPrivileged 的统一执行口。它用来衡量日志链在扣除 doPrivileged 大集合之后的份额。
- b9f47c33 上 HelloWorld 的基线是 **3324**，不是 §5.8 / enum-values-direct 记录的 2178。差异来自 6db384e8（映像容器对象的字段值记入按对象值表）：这是正确性修复。修复前，引导层 HashMap 迭代被折成不可达，2178 偏小，是不正确的值。因此本节一律以 3324 为基线。
- 协调方的门模型（gates）低估了实际份额，下文排序只按实测列。

**求值器补齐：DetectBackend 构建期求值（机制 ①）**
- 清单 `[concrete.boot] calls` 增加 `{ init = ["jdk/internal/logger/BootstrapLogger$DetectBackend"] }`。只初始化后端判定类，**不调用** `getLoggerFinder` / `LoggerFinderLoader.service()`，所以构建期不执行 `redirectTemporaryLoggers`，`logManagerConfigured` 保持与 HotSpot 启动后相同的 false。这同时绕开了 §5.9.3 中 `accessProvider` 残差化的问题。
- 判定过程（ServiceLoader(LoggerFinder, SCL) → `loadInstalled(DefaultLoggerFinder)`）依次撞到以下缺口，都按 HotSpot 同义补成通用 native，事实写在清单里：

| 缺口 | 触发位置 | 补法 |
|---|---|---|
| `Class.getDeclaredMethods0` | StreamOpFlag → EnumMap → `getEnumConstantsShared` | op `class_declared_methods`（`engine/concrete/reflect.rs`）。反射对象由 VM 直接填写布局字段，字段名在 `[concrete.vm_fields]` 中以 `method_*` 登记；`slot` 为类文件方法下标；注解取原始字节 |
| `getDeclaredConstructors0` | `ServiceLoader.getConstructor` | op `class_declared_constructors`，布局字段 `ctor_*` |
| `ConstantPool` 取得 | — | op `class_constant_pool`，布局字段 `constant_pool_oop` |
| 本地反射调用 | `DirectMethodHandleAccessor$NativeAccessor.invoke0`、`Method$Direct.invoke0` | op `reflect_invoke`。实参与返回值只支持引用类型；目标抛出异常时求值失败（不建模 InvocationTargetException） |
| `ClassLoader.findBootstrapClass` | `ServiceLoader.loadProvider` → `Class.forName(Module, String)` | op `boot_class`。按 `defineModule0` 记录的包 → 模块 → 加载器（null）判定 |

- 构建期产生的反射对象经 `Class.reflectionData`（软引用）可达。导出映像时，软引用按「可随时清除」的语义清除：清单 `[concrete] soft_references` / `null_queues` 登记了软引用类型与空队列类型；referent 只在别处也可达时保留。这样反射对象不入映像。
- 结果：boot report 为「通过」，inited 335，residual_calls 仍为 2（没有新增残差）。DetectBackend 与 StreamOpFlag 都成为构建期初始化类。

**分析器：映像对象身份标记**
- `Obj::Image(id, finals)` 携带映像对象 id（3d7b3c64）。`ref_eq` 对两个映像标记按 id 折叠 `if_acmp`。
- `field_value` 对 static final 字段的处理：字节码常量为未定形对象（`Obj::Fields`）时，优先采用映像值（`image_final`）。
- 收益：`BootstrapLogger.useLazyLoggers` 中 `detectedBackend == CUSTOM` 折成 false（[15,17) 不可达）。

**实测**

| 测例 | b9f47c33 类 / 方法 | 3d7b3c64 类 / 方法 | 差 |
|---|---|---|---|
| HelloWorld | 3324 / 19,847 | 3315 / 19,761 | −9 / −86 |
| CollectorsDemo | 3324 / 19,853 | 3315 / 19,767 | −9 / −86 |
| DeepCopy | 3573 / 22,505 | 3562 / 22,406 | −11 / −99 |
| HelloWorld E | 2773 | 2764 / 15,926 | −9 |
| CollectorsDemo E | — | 2764 / 15,946 | |
| DeepCopy E | — | 2885 / 16,929 | |

- HelloWorld 减少的 9 个类是 EnumMap 及其 5 个内部类、`StreamOpFlag$MaskBuilder`、`StreamOpFlag$Type`、`BootstrapLogger$DetectBackend$1`。E 下减少的是同一组类。

**切断实验**（HelloWorld，3d7b3c64）

| 变体 | 类 / 方法 |
|---|---|
| S：切 `LoggerFinderLoader.service` | 3245 / 19,153 |
| ES | 2758 / 15,887 |
| EG：E + 切 `LazyLoggers.getLoggerFromFinder` | 2756 / 15,880 |
| b9f47c33 上的旧实验 | L（切 logRuntimeExit）3324；F（切 getLoggerFromFinder）3252；LE 929；FE 2834 |

- 结论：扣除 doPrivileged 大集合之后，finder / service 本身只持有约 6–8 个类。E → LE 的 1844 类差由 logRuntimeExit 链上的**其他节点**持有，不在 finder 上。①③ 的收益上限因此受这些节点约束，下一步应先用 `--why` 定位它们。

**E → LE 的 1844 类差：当前头已不存在（2026-10-08，分支 `log-chain2`，b8988868，作业 lg2-why1）**

| 变体（HelloWorld） | 类 / 方法 |
|---|---|
| ES | 2758 / 15,887 |
| LES：ES + 切 logRuntimeExit | 2758 / 15,886 |
| LE：E + 切 logRuntimeExit | 2764 / 15,925 |

- 在 b8988868 上，切 logRuntimeExit 已经不减任何类：LE = E = 2764，LES = ES = 2758（只少 1 个方法）。可见 logRuntimeExit 链不持有这 1844 个类。ES 与 LE 之差共 6 个类，全部是 `LoggerFinderLoader.service` 与 `ServiceLoader.loadInstalled` 的直接产物（`BootstrapLogger$LogEvent`、`LoggingProviderImpl` 等）。
- 旧 LE（b9f47c33，929 类）与新 LE（2764）逐类比对，前沿边如下（按首次发现链归因，共 1839 类）：

| 持有点 | 类数 |
|---|---|
| `LocaleProviderAdapter.forType@61` 反射实例化 CLDR 适配器（`CLDRLocaleProviderAdapter.<init>`） | 725 |
| `String.valueOf(Object)@11` 的 toString 派发 | 246 |
| `executePrivileged(PrivilegedExceptionAction)@29` 派发 | 125 |
| stream 管线（`AbstractPipeline.evaluate` / `copyInto` / `sourceSpliterator`） | 161 |
| `StreamEncoder.<init>@4`（字符集） | 69 |
| 其余（`FileDescriptor.closeAll`、`computeIfAbsent`、`URL.openConnection`、`Pattern.compile` …） | 513 |

- 两边到 `forType` 的链完全相同：`[boot_region] Charset.isSupported` → `checkName` → `String.charAt` 越界消息 → `String.format` → `Locale.<clinit>` → … → `ThreadLocalRandom.<clinit>` → `SecureRandom.getSeed` → `Provider.<clinit>` → `String.toUpperCase(Locale)` → `ConditionalSpecialCasing` → `BreakIterator.getWordInstance` → `LocaleProviderAdapter.forType`。
- 差别在 `forType@68` 的 `getDeclaredConstructor()` 是否返回：
  - 旧 LE 中，`getConstructor0` 的循环体不可达（`ReflectionFactory.getExecutableSharedParameterTypes`、`Constructor.copy`、`Constructor.newInstance` 都不在闭包里）。`getDeclaredConstructor` 因此被判为不返回，`@75 newInstance` 之后整片不可达。
  - 新 LE 中循环体可达，CLDR 适配器被实例化，下游整片回来。
- 运行期 `getDeclaredConstructor()` 必然返回，所以旧 LE 的 929 是不健全的偏小结果，与 2178 同类（见「测量口径」）。转变发生在 b9f47c33..b8988868 之间：构建期求值器补了反射 native（ffff4c51 / 173f43de），映像标签也在其中（3d7b3c64）。
- 结论如下：
  - E 下日志链只剩 finder / service 的约 6–8 个类，logRuntimeExit 本身不持有大集合；
  - 1844 类实际由 `Charset.isSupported` 根经越界消息与 `ThreadLocalRandom` 种子链持有，交给 charset-build 与越界消息线；
  - 路径 A 的收益上限是 S 量级：基线下 −70（3315 → 3245），E 下 −6–8。路径 A 仍按终态做：它是精度改进，并且是 ③ 的前提。

**路径 A 与 ③ 的收益上限：0 类，本线不实现（2026-10-08，`log-chain2` 29140373 = 合入 batch-1011 a88d7075 之后，作业 lg2-cut3，sg1）**

| 变体（HelloWorld） | 类 |
|---|---|
| 基线 | 3304 |
| L：切 `Shutdown.logRuntimeExit` | 3304（0） |
| F：切 `LazyLoggers.getLoggerFromFinder` | 3233（−71） |
| LF：L + F | 3233（−71） |
| G：切 `System.getLogger(String)` | 3228（−76） |

- ③（折叠 logRuntimeExit 中的 `isLoggable(DEBUG)`）只能删去 logRuntimeExit 体内 if 分支下的内容。整个 logRuntimeExit 被切掉都不减类（L = 基线；E 下也是 LE = E，见上），所以 ③ 的类收益上界是 0。
- 路径 A（折叠 `LazyLoggers.getLogger` 中的 `isSystem(module)`）只删去一条边：`LazyLoggers.getLogger@15 → getLoggerFromFinder`。getLoggerFromFinder 还另有一条入口：
  - 入口链：`getLazyLogger` → `JdkLazyLogger.<init>` → `LazyLoggerAccessor.makeAccessor`；运行期取用时经 `LazyLoggerAccessor.wrapped` → `createLogger@37` → `LazyLoggers$1.apply`（即 loggerSupplier），再到 getLoggerFromFinder。
  - 这条入口在 HelloWorld 闭包中经 `PlatformLogger.getLogger@41 → getLazyLogger` 可达，与 isSystem 的取值无关，而且运行期也确实会走：惰性 logger 首次使用时就取 finder。
  - 因此 F 的 −71 不能经路径 A 取得，路径 A 的类收益同样是 0。
- 路径 A 本身还有一个结构性缺口：`LazyLoggers.getLogger` 的 module 形参按全部调用方汇合。`LoggerFinderLoader$TemporaryLoggerFinder.getLogger` 是另一个调用方，它的 module 经 `LazyLoggerAccessor.moduleRef`（WeakReference）一路传来，值未知。要折叠，需要「常量实参驱动的调用点克隆」，不能只做调用者镜像常量化加 const_eval 标签绑定。实现代价大，类收益为 0。
- 决定：路径 A 与 ③ 在本线都不实现。日志链剩下的 71–76 类由 PlatformLogger / Tripwire 根经惰性 logger 的取用链持有，终态解法有两种：
  - 映像求值器在构建期把这些 PlatformLogger 的后端判定落成映像值，使 `BootstrapLogger.useLazyLoggers` / `useSurrogateLoggers` 成为映像常量；
  - 由 Tripwire 收窄线切掉 Tripwire 根。
- locale-build 修复 3（`obj_at` 工厂对象保留调用点）合入后，基线变为 3304。上表已按新基线重测。

**为什么 `useSurrogateLoggers` 仍未折叠**
- `useSurrogateLoggers = detectedBackend == JUL_DEFAULT && !logManagerConfigured`。前半已可按映像值得到。但 `logManagerConfigured` 的唯一写点 `redirectTemporaryLoggers` 只在 `LoggerFinderLoader.service()` 中调用，而 service() 仍经由 `Tripwire` → `PlatformLogger` 上下文与 `LazyLoggers.getLoggerFromFinder`（@15，非系统模块分支）可达。按「映像初值 ⊔ 可达 putstatic」，该字段为 {false, true}，不能折叠。
- 终态解法是路径 A：折叠 `LazyLoggers.getLogger` 的 `isSystem(module)`。
  - logRuntimeExit 的调用者模块是 java.base（@CallerSensitive，调用点静态可知）；
  - `isSystem` 经 `DefaultLoggerFinder$1` 字段 → doPrivileged 按调用点返回 → `Module.getClassLoader` → `VM.isSystemDomainLoader`；
  - 需要以下能力：调用者模块的常量化、映像 Module 标记穿过 `DefaultLoggerFinder$1` 的字段、doPrivileged 的按调用点返回值（不经 executePrivileged 汇合）、`Module.loader` 的映像读取。
  - 打通后，exit 路径走 `getLazyLogger`；只剩 `useLazyLoggers()` 的取值依赖 `logManagerConfigured`。此时还需要「service() 仅经由非 exit 根可达」的按上下文值域，或由 Tripwire 收窄线切掉 Tripwire 根。
- 本分支未实现路径 A，工作量大，涉及 doPrivileged 返回值的上下文敏感化。log-chain2 实测确认其类收益上限为 0，不再实现（见上「路径 A 与 ③ 的收益上限」）。

**③ isLoggable(DEBUG)**
- 未做。前提是 `useSurrogateLoggers` 折成 true；之后按 `JUL_DEFAULT_LEVEL = INFO` 折叠 `SurrogateLogger.isLoggable`。实测切掉整个 logRuntimeExit 不减类，③ 的类收益上限为 0，不再做。

**仍持有日志链的其他根**（交给对应的线）
- `Tripwire.ENABLED`（doPrivileged 读属性）；
- `Charset.isSupported`（charset-build 线负责）；
- PlatformLogger 级别上下文不精确。

**重叠**
- boot-image-s6 同样编辑 `vm_intrinsics.toml` 的 `[concrete.boot]` / `[concrete.natives]`，合批时可能有文本冲突，两边条目并存即可。
- reflect-direct 的 `Method$Direct.invoke0` 在求值器中映射为 `reflect_invoke`，与其运行期直连互不影响。
- `Obj::Image` 签名改为带 id，`field_hooks.rs` 一并改动；boot-image 各线合批时注意此处。

**单测 `closure_cli::param_string_constants_fold_switch` 不归本线所能转过**（batch-1009 验证记录归因为「日志链膨胀」，经核对不成立）
- 该测要求两项：HelloWorld 闭包不含 `sun/net/www/protocol/jrt/Handler`，且总类数 < 1000。
- 用已取回的闭包 JSON 逐级回溯 via，`jrt/Handler` 在**所有变体**中都在，包括 LE（切 logRuntimeExit + executePrivileged，929 类）。可见它不由日志链持有，持有者是 `URL$DefaultFactory.createURLStreamHandler@128`：协议名 switch 未折叠。上游有两条：
  - LE 下：`[boot_region] 根 Charset.isSupported` → `checkName` → `String.charAt` 越界消息 → `String.format` → `Locale.<clinit>` → `LocaleObjectCache` → `ConcurrentHashMap.addCount` → `ThreadLocalRandom.<clinit>` → `SecureRandom.getSeed` → `SeedGenerator.<clinit>` → `URLSeedGenerator.init` → `new URL(String)`。协议来自 `securerandom.source` 属性，非常量。
  - 3d7b3c64 基线下：`BootLoader.findResourceAsStream` → `BuiltinClassLoader.findResourceOnClassPath` → `EmbeddedClassPath.url` → `URL.<init>(String, String, int, String, URLStreamHandler)`。协议形参在该调用点不是常量。
- 类数 < 1000 至少需要同时切掉日志链与 executePrivileged 汇合点（LE = 929）。单独一项都不够：E 为 2764，S / G 的量级见上表。
- 结论：该测要等三条线都完成才能转过，本分支单独不能使其通过：
  1. charset-build（`Charset.isSupported` 根）；
  2. 越界消息 / `ThreadLocalRandom` 种子链收窄；
  3. 路径 A，加上 doPrivileged 按调用点返回。
  
  此外，`EmbeddedClassPath.url` 的协议常量化需要另行处理。

**续作入口**
- ~~先在 ES 变体上用 `--why` 定位 E → LE 的 1844 类的持有节点~~ 已完成（见上「当前头已不存在」）；
- 路径 A 的改动点：`absint` 中 @CallerSensitive 调用者模块常量化，以及 `engine/facts` 中 doPrivileged 的按调用点返回值；
- ③ 在路径 A 之后做。

## 6. 分步计划（每步单独提交，验收数字为硬门槛）

| 步 | 内容 | 验收 |
|---|---|---|
| 1 | 引导模式入正式代码：清单 `[concrete.boot]`、5 处语义分叉、21 种新增 op、审计报告（`rava audit boot`）；Linux JDK 21 / 25 两个映像 | HelloWorld 档案键下 initPhase1–3 跑完，initPhase2 返回 0，未登记失败 = 0；`--hash-seed` × `--flow-batch` 4 组合映像摘要相同；求值耗时 ≤ 0.5 s、RSS ≤ 300 MB |
| 2 | 污点与重算槽；延迟值传播到标量；运行期初始化类级联、重放序列 | 映像中污点值 = 0（审计）；`NCPU` / `directMemory` 等 4 个字段进入重算槽；运行期初始化类 ≤ 2（macOS）/ ≤ 1（Linux），即第 1 步按 U1 的实测值（§5.3），只减不增 ；**✅ d1dc540a 实测**：污点值 0，重算槽 3 字段 / 5 槽，运行期初始化 Linux 1 / macOS 2，U8 交集 0，Linux 21 / 25 四组合摘要一致，耗时 ≤ 324 ms，RSS ≤ 254 MB（§5.4） |
| 3 | 映像物化（档案内 `boot_image`）与装载；抽象分析从映像出发（联合裁剪）；删 `[boot_init]` 的 `calls` / `phases` 与 FS-C2 钩子 | HelloWorld 闭包 ≤ 540 类（目标 ≤ 569），二进制大小增量 ≤ 5%；启动装载 ≤ 1 ms；HelloWorld、TestAppClassLoader、TestModuleLayerDefine、TestBootLayer 输出与 JDK 相同；HelloWorld 闭包上限已由 §5.5.6「按平台上限」取代。**派发状态（2026-10-08）**：机制 ①③（U12）+ U14 `line.separator` / `file.encoding` 钉值合为一个任务，🔄 进行中（分支 u12-props，基于 b558e0c2）；S2 `Class.genericInfo` 入映像（U13）⏳ 待派（有空名额即派） |
| 4 | a3 归零第一批：VM（审计 10 个方法，全仓属性 8 个）、Module 7、ModuleLayer 2、Class 2，T1 / T2 / T5 / T6；SecurityManager 移出边界 | `#[jvm_boundary]` 全仓 33 → 14（vm_impl 8、module_impl 7、module_layer_impl 2、class_impl 2 归零）；TestClassModuleFace、TestProtectionDomainFaces、TestSetAccessibleBoundary 通过 ；**模块部分 ✅ fcc54fb8**：module_impl 7、module_layer_impl 2 归零，Module / ModuleLayer 移出 VM 边界，三例通过（bimg3-m-fcc54fb8，§5.5.6）；**VM / Class / SecurityManager（cabe9fb0，§5.6）**：vm_impl 8、class_impl 归零，全仓 23 → 14，SecurityManager 移出边界；待合批验证 |
| 5 | jimage 嵌入数据与 `getNativeMap`（boot-layer 第 5 步），T3 / T4 / T7；L2 6、BootLoader 2、JceSecurity 6（2026-10-08 由第 6 步前移）；U14 `java.home` 构建期钉值，JceSecurity 策略文件改为构建期事实 | `#[jvm_boundary]` 14 → 0；TestClassResourceStream 通过；JCA 用例通过 ；**部分**：44be328e 补 native `BootLoader.getSystemPackageLocation`（TestStringGetCharsLegacy 通过，§5.5.6）；**实现 ✅ 82c49923 / f196dd71（§5.7.1–5.7.4）**：ClassLoader 6、BootLoader 2、JceSecurity 6 归零，三类移出 VM 边界，全仓 `#[jvm_boundary]` = 0；待合批验证（batch-1009） |
| 6 | 非引导类的构建期初始化（C3 `build_time_init`），用户程序可达类按同一规则判定；嵌入 java.home 树的 NIO native（已随第 5 步实施） | `#[jvm_boundary]` 保持 0；CollectorsDemo 等冷独占正则链 0 类；**部分 ✅ boot-image-s6（§5.8）**：扩展组 + 尝试隔离落地，HelloWorld / CollectorsDemo 类 2178 → 2148、TestJcaSasl 2276 → 2246，构建期初始化 1328 类（限根模块）；JceSecurity 带入风险实测为 0；**正则链未达**（61 类）：阻塞于映像 lambda 对象（§5.8.3，单独成步）；待合批验证 |
| 7 | 语料全量 | 档案并集类数不超过现状（7886）；失败数不超过基线 |

## 7. 风险

1. **污点规则的健全性**：如果漏标一个宿主源，宿主值会被烘进二进制，跨机器行为就会出错。缓解：
   - 延迟源清单由审计守护：`ACC_NATIVE` 方法在引导期被调用而没有登记 op 时构建失败，没有缺省 op；
   - 加跨平台对照单测：同一份字节码在 Linux 与 macOS 上的映像差异只允许出现在平台属性键上。
2. **引导期 native 面的维护量**：JDK 升版时 `initPhase*` 调用的 native 会变（25 的 `ThreadIdentifiers`、`ScopedValue` 等）。
   失败即构建失败、报出栈，所以不会静默出错；代价是升版时需要补清单。
3. **映像与运行期手写层的不一致**：映像中的对象布局由字节码字段决定。运行期的手写 struct（过渡类）如果字段不同，装载就会出错。S7 和「struct 一律由字节码生成」（规范 §1）是前提，第 3 步之前须核对引导映像类型中有无手写 struct。
4. **生产模式映像裁剪与闭包的联合不动点**：映像可达集依赖闭包读取哪些静态字段，闭包又依赖映像值集，两者需要单调。抽象值只增不减，所以单调成立；实施时要用顺序矩阵验收。
5. **身份哈希物化**：运行期 `identityHashCode` 需要区分映像对象和新对象。如果 S7 句柄不能廉价地判定「映像区」，就要每个对象多一个字。
6. **macOS 与 Linux 的映像分叉**：本机探针用 macOS 类，服务器语料用 Linux 类。验收以 Linux 为准，本机只做小例子。

## 8. 需用户决策

### 8.1 已定（2026-10-05）

| # | 决定 |
|---|---|
| U0 | 立项；作为 a3 余项、boot-layer 第 2–5 步的前置，取代第 2–3 步的锚点机制 |
| U1 | 运行期取宿主值，不钉值：`sun.jnu.encoding`、`stdout/stderr.encoding`、`file.encoding`、`line.separator`、`java.home` 为延迟值，相关类转为运行期初始化，initPhase1 的 Charset 分支在运行期执行。影响实测见 §5.3。**2026-10-08 被 U14 部分修订**（§8.4）：`line.separator`、`file.encoding`、`java.home` 改为构建期钉值 |
| U2 | 固定 SALT 种子（经 `CDS.getRandomSeedForDumping`） |
| U3 | 运行期重放 `toFileURL` |
| U4 | 直接做 arena 永久区零拷贝，不做批量建对象的过渡形态；所依赖的 S7 句柄设计作为本线前置定稿 |
| U5 | 映像放在 `java_base` 档案内 |
| U6 | 与「Class 接收者逐镜像求值」并行 |
| U7 | （2026-10-06）延迟调用的非空承诺维持现状：`toFileURL` 占位对象由清单承诺非空，判空在构建期定值 |
| U8 | （2026-10-06）第 2 步加审计：残差重放的读集 ∩ 延迟点之后构建期的写集非空即构建失败；交集实测见 §5.4 |
| U9 | （2026-10-06）内存缓存字段（`memo_fields`）撤回时保留缓存值（现状） |
| U14 | （2026-10-08）`java.home` 构建期钉值为嵌入虚拟树 `/java-runtime`（`sun.boot.library.path` 同树 `lib`），原生二进制不依赖宿主 JDK 安装，`conf/security` 等构建期嵌入；钉值经 `vm_intrinsics.toml` 清单声明。取代 U1 中 `java.home` 一项（U1 其余属性中 `line.separator` / `file.encoding` 另行钉值，不在第 5 步范围）。实施见 §5.7.3 |

### 8.2 第 1 步提出的新决策项（2026-10-06 已定，见 §8.1，本表存档）

| # | 事项 | 选项 | 建议 |
|---|---|---|---|
| U7 | 延迟调用的非空承诺：`toFileURL` 在当前目录不可规范化时返回 null，映像却按非空占位处理（判空在构建期定值） | 清单承诺非空（现状）/ 判空也延迟（initPhase3 中依赖它的段落改为残差区段） | 承诺非空：原生二进制当前目录不可用时 JVM 本身也无法启动应用类加载 |
| U8 | 残差重放与构建期写入的次序（WAR）：残差调用 / 区段在运行期执行时，构建期已把延迟点之后的写入烘进映像；若重放读到这些写入，会与 JVM 次序不同 | 第 2 步审计（重放读集 ∩ 延迟点后的写集 = 0，否则构建失败）/ 不处理 | 第 2 步加审计；第 1 步未测交集 |
| U9 | 内存缓存字段（`memo_fields`，新增 `Class.packageName`）撤回时保留缓存值 | 保留（现状）/ 撤回 | 保留：重算得同一值，不是运行期改写 |

### 8.3 原始选项（存档）

| # | 事项 | 选项 | 建议 |
|---|---|---|---|
| U0 | 是否立项（C3 `build_time_init` 已定归 C3，本方案把它扩为 VM 引导阶段整体执行，并取代 boot-layer 第 2–3 步的锚点机制） | 立项并作为 a3 余项、boot-layer 第 2–5 步的前置 / 维持 boot-layer 原方案 | 立项：一个机制解决 a3 余项全部 33 个 `#[jvm_boundary]`、锚点膨胀和 `SystemModules$default` 的大方法编译 |
| U1 | 控制流敏感属性的钉值：`sun.jnu.encoding`、`stdout/stderr.encoding`、`file.encoding`、`line.separator`、`java.home`（伪树前缀） | 钉值（原生二进制无 `-D`）/ 运行期取宿主值（相关类全部转为运行期初始化，initPhase1 的 Charset 分支随之在运行期执行） | 钉 UTF-8 / `\n` / 伪树 |
| U2 | 固定 SALT 种子（经 `CDS.getRandomSeedForDumping`）：Set.of / Map.of 的迭代次序在同一二进制的每次运行中都固定，JVM 则每次随机（无 CDS 时） | 固定 / 运行期随机（ImmutableCollections 转为运行期初始化，引导映像中大量 `Set.of` 对象将无法物化，方案基本不成立） | 固定（与 CDS dump 的 JDK 自身做法一致） |
| U3 | 空 class path（cwd）的 app class path URL | 运行期重放 `toFileURL`（与 JVM 一致）/ 固定为空（原生二进制没有类路径） | 运行期重放 |
| U4 | 映像装载形态 | 甲：启动时批量建对象（约 0.4 ms）/ 乙：arena 永久区零拷贝（依赖 S7 句柄设计） | 终态取乙；S7 未定前先实施甲作为装载层，不影响映像格式 |
| U5 | 映像所在的档案层 | 放在 `java_base` 档案内 / 独立 `boot_image` crate | `java_base` 内（引用的类全属 java.base） |
| U6 | 实施次序与在途精度线 | 先做映像，再测 §25.2 的 1852 类 / 先等「Class 接收者逐镜像求值」 | 并行：两者正交，第 3 步验收时合测 |

#### 5.5.6 用户决策 U11（2026-10-07）：启动链接改为零拷贝终态

§5.5.5 待决项 1、3、6（静态字段启动期 setter 写入、启动期链接与宿主改写、接口 / 数组视图与 Class 镜像启动期链接）偏离 U4「直接做零拷贝永久区」。用户 2026-10-07 采纳协调者建议：

- **按终态重做为零拷贝**：映像在构建期全部落为 Rust 常量（静态字段初值、接口 / 数组视图、Class 镜像均为常量），启动期只覆盖确实依赖宿主的少数字段（`sun.jnu.encoding`、`stdout/stderr.encoding` 等，按 U1 / U14：换行符、`file.encoding`、`java.home` 2026-10-08 起改为构建期钉值）。
- **顺序**：C4 冻结解除后，先在服务器上把现有实现（3e309fec）编译、跑通，排除正确性问题；再单独一步改零拷贝；最后实测体积（≤+5%）与启动装载（≤1 ms）。
- 不采纳「实测达标即接受现状」的备选。
- §5.5.5 其余待决项（2、4、5、7–15）按代理实现认可；D5 物化与 S6 标准流随零拷贝一步排期。

### 8.4 U12 / U13 / U14 与去除无映像回退（已定，2026-10-08）

| # | 事项 | 决定 |
|---|---|---|
| U12 | U1 例外：§5.5.6 终态设计机制 ①③ 所在的日志 / 信号链（(L) 链，`Runtime.exit` / `Shutdown.exit` → `logRuntimeExit`） | **已定（2026-10-08，用户采纳建议）**：接受为 U1 的例外，仅限该路径——日志路径相关属性取构建期值；用户 `LoggerFinder` 提供者的构造函数在构建期运行；运行期 `-Djdk.system.logger.level` 不再生效。此前因 U12 挂起的机制 ①③ 解除挂起，按 §5.5.6 实施。**派发**：🔄 进行中（分支 u12-props，基于 b558e0c2，与 U14 的 `line.separator` / `file.encoding` 合为一个任务） |
| U13 | S2：`Class.genericInfo` 写入引导映像（S2 接收者精度已修，`Recv::Bounded`，§5.6.5；HelloWorld 闭包不降，约 3043 类；§5.6.7 #6） | **已定（2026-10-08，用户采纳建议）**：做——热路径类镜像的 `genericInfo`（`ClassRepository`）在构建期算好写进映像，运行期命中缓存不再解析签名。**派发**：⏳ 待派（有空名额即派） |
| — | 去除无映像回退（映像求值失败即构建失败，不保留运行期引导的第二条路径；dcabf9e3，§5.6.8，batch-1008 合批验证中） | **保留（2026-10-08 用户确认）** |
| U14 | U1 部分修订：系统属性按来源分别取值。用户原话：「只要有利于缩小闭包，也可以进行调整，把一部分这类属性改成编译时的值」 | **已定（2026-10-08，用户改判）**。判据：属性值由目标平台或 JDK 规范决定、与运行宿主无关的，构建期钉值，依赖类随之进入引导映像；值真正取决于运行宿主环境的，仍运行期读取。逐项：`line.separator` 按目标三元组钉值（unix `\n`、windows `\r\n`）；`file.encoding` 钉为 UTF-8（JEP 400，JDK 18 起缺省）；`java.home` 钉为构建期值（原生二进制运行时不依赖 JDK 安装目录，jimage 与 conf 资源在构建期嵌入，JceSecurity 策略文件因此成为构建期事实）；`sun.jnu.encoding`、`stdout.encoding`、`stderr.encoding` 维持运行期读取（取决于宿主 locale 与终端）。**每项改动实测闭包类数变化；没有收益的项不钉值。** U1 被本项部分修订；U12 不变。**派发**：`line.separator`、`file.encoding` 🔄 进行中（u12-props，随 U12）；`java.home` 🔄 进行中（归第 5 步分支 boot-image-s5，JceSecurity 策略文件改为构建期事实） |

**待核对**：闭包构成报告（`docs/reports/2026-10-07-closure-composition.md`）基线 HelloWorld 468 类，与 boot-image-s4 起实测约 3043 / 3011 类（§5.6.5、enum-values-direct）落差很大，原因待查；门排名（`rava closure --gates`，分支 closure-gates）在新基线上出数后解释。
