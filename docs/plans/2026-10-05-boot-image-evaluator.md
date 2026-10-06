# 构建期引导映像求值器（boot image evaluator）终态方案

> 2026-10-05，分支 `boot-image`（基于 rust-closure-analyzer 0192bf20）。本文只含设计与可行性探针实测；探针代码在本分支单独提交，不合入集成分支。
> 上游：c1d §25.3（a1 引擎执行 initPhase2 不可行，需另立构建期引导映像求值器）、boundary-narrowing §6.11 项 3（`build_time_init`，2026-10-01 用户决策归 C3）、
> boot-layer.md 第 2–5 步、c1d §21.9（a3 余项）。

## 0. 结论

> **2026-10-05 用户决策 U0–U6 已定（§8.1）**；第 1 步已在分支 `boot-image-s1` 实施，实测见 §5.3。
> U1 改判为「运行期取宿主值」：下文 §3.2、§5.2 B1、§6 第 2 步验收已按 U1 改写；§0 第 1 条与 §5.1 的探针数字为钉值时的历史实测。

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
| `initPhase1`：属性表、`VM.saveProperties`、`setJavaLangAccess`、stdin/out/err、`Terminator.setup`、`VM.initLevel(1)` | 手写的 `System.registerNatives` 承担其中的属性表、saveProperties 段和 `out` / `err` / `in`；`setJavaLangAccess` 由 `[boot_init] calls` 在 main 前调用 | `system_impl.rs`、`seeds.toml [boot_init]` |
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
| 影响控制流的属性 | `sun.jnu.encoding` ✓（initPhase1 中 `Charset.isSupported` 分支）、`stdout/stderr.encoding` ✓、`file.encoding`、`line.separator`、`java.home` | **运行期取宿主值**（U1）：`defer_value`；求值器把依赖它们的部分残差化（下文「残差化」），不钉值 |
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

**可以物化，但它不是引导阶段的内容**：HelloWorld 的 initPhase1–3 不初始化 JceSecurity。它属于用户程序可达时的构建期类初始化（C3 `build_time_init`，第 6 步），用同一个求值器。核对 JDK 21 的 `JceSecurity.<clinit>`：
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
2. 运行期不能改写 `java.security` 或 policy。原生二进制没有 `-Djava.security.properties`，与钉值属性同一口径（U1）。
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
| S5 | `Module`（68）/ `ModuleLayer`（2） | `module_impl` 的 VM 模块表（`defineModule0` 运行期登记）、`unnamed_module` / `ModuleLayer.boot` 的手写单例 | VM 模块表以映像 VM 表（62 模块 / 771 包，含读与导出）为初值；两个手写单例属 a3 第 4 步删除的 `#[jvm_boundary]`，本步不改其语义，只保证映像对象与之不冲突（见 §5.5.3 余项） |
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
- **D9 残差调用的实参**：残差调用的被调方法，其形参取构建期记录的实参，不再 open。标量与 null 进常量格；字符串对象（非占位、非延迟）进字符串常量；其他映像对象记为「带标签的非空引用」，标签是它 final 实例字段中的标量 / 字符串常量，与构造器摘要同一口径。引用实参的映像对象经 `image_ref` 流入形参节点。静态字段的映像初值同样走这套常量格（`image_pv`）。重放 native 与残差区段内的调用目标仍按 open 形参作根。
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

D9 消掉了 `newPrintStream` 一支：形参 open 时，编码名可为 null → `Charset.defaultCharset` → 同一处 `StandardCharsets.lookup`。D10 消掉了 `PreHashedMap.put` 的 open 注入。剩余放大点是 2–4，属于通用精度问题，不是映像特有：任何以非常量名调 `Charset.forName` 的程序同样会碰到。

**恢复入口**

1. 先压闭包（硬门槛）。候选按收益排序：
   - (a) `Proxy$Dyn` VM 钩子按代理实际接口与调用点派发，不再以 open 实参派发全部方法；
   - (b) `Class.newInstance` / `getConstructor0` 异常消息分支的冷路径；
   - (c) `Method.invoke` 的 `isCallerSensitive` 注解查询，按 `@CallerSensitive` 的静态事实折叠。
   - 每做一项，都以 `rava closure tests/e2e/01_basics/HelloWorld.java -o … --why <类>` 复测。
   - `--flows "@openorig:<类型>|<节点>"` 与 `@grow:` 可定位 open 注入点。
2. 再做发射侧 D1–D5 与启动重放函数。
3. 最后删 `[boot_init] calls / phases`、FS-C2 钩子，加 TestBootLayer，跑服务器单测 / 审计 / 抽查。
4. 确定性（`--hash-seed` 0 / 12345 × `--flow-batch` 1 / 64）在本次提交上尚未复验。
5. 本分支在发射侧完成前不可合入：分析已从映像出发，但映像尚未物化。

## 6. 分步计划（每步单独提交，验收数字为硬门槛）

| 步 | 内容 | 验收 |
|---|---|---|
| 1 | 引导模式入正式代码：清单 `[concrete.boot]`、5 处语义分叉、21 种新增 op、审计报告（`rava audit boot`）；Linux JDK 21 / 25 两个映像 | HelloWorld 档案键下 initPhase1–3 跑完，initPhase2 返回 0，未登记失败 = 0；`--hash-seed` × `--flow-batch` 4 组合映像摘要相同；求值耗时 ≤ 0.5 s、RSS ≤ 300 MB |
| 2 | 污点与重算槽；延迟值传播到标量；运行期初始化类级联、重放序列 | 映像中污点值 = 0（审计）；`NCPU` / `directMemory` 等 4 个字段进入重算槽；运行期初始化类 ≤ 2（macOS）/ ≤ 1（Linux），即第 1 步按 U1 的实测值（§5.3），只减不增 ；**✅ d1dc540a 实测**：污点值 0，重算槽 3 字段 / 5 槽，运行期初始化 Linux 1 / macOS 2，U8 交集 0，Linux 21 / 25 四组合摘要一致，耗时 ≤ 324 ms，RSS ≤ 254 MB（§5.4） |
| 3 | 映像物化（档案内 `boot_image`）与装载；抽象分析从映像出发（联合裁剪）；删 `[boot_init]` 的 `calls` / `phases` 与 FS-C2 钩子 | HelloWorld 闭包 ≤ 540 类（目标 ≤ 569），二进制大小增量 ≤ 5%；启动装载 ≤ 1 ms；HelloWorld、TestAppClassLoader、TestModuleLayerDefine、TestBootLayer 输出与 JDK 相同 |
| 4 | a3 归零第一批：VM（审计 10 个方法，全仓属性 8 个）、Module 7、ModuleLayer 2、Class 2，T1 / T2 / T5 / T6；SecurityManager 移出边界 | `#[jvm_boundary]` 全仓 33 → 14（vm_impl 8、module_impl 7、module_layer_impl 2、class_impl 2 归零）；TestClassModuleFace、TestProtectionDomainFaces、TestSetAccessibleBoundary 通过 |
| 5 | jimage 嵌入数据与 `getNativeMap`（boot-layer 第 5 步），T3 / T4 / T7；L2 6、BootLoader 2 | `#[jvm_boundary]` 14 → 6；TestClassResourceStream 通过 |
| 6 | 非引导类的构建期初始化（C3 `build_time_init`），用户程序可达类按同一规则判定；嵌入 java.home 树的 NIO native；JceSecurity 6 | `#[jvm_boundary]` 6 → 0；JCA 用例通过；CollectorsDemo 等冷独占正则链 0 类 |
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
| U1 | 运行期取宿主值，不钉值：`sun.jnu.encoding`、`stdout/stderr.encoding`、`file.encoding`、`line.separator`、`java.home` 为延迟值，相关类转为运行期初始化，initPhase1 的 Charset 分支在运行期执行。影响实测见 §5.3 |
| U2 | 固定 SALT 种子（经 `CDS.getRandomSeedForDumping`） |
| U3 | 运行期重放 `toFileURL` |
| U4 | 直接做 arena 永久区零拷贝，不做批量建对象的过渡形态；所依赖的 S7 句柄设计作为本线前置定稿 |
| U5 | 映像放在 `java_base` 档案内 |
| U6 | 与「Class 接收者逐镜像求值」并行 |
| U7 | （2026-10-06）延迟调用的非空承诺维持现状：`toFileURL` 占位对象由清单承诺非空，判空在构建期定值 |
| U8 | （2026-10-06）第 2 步加审计：残差重放的读集 ∩ 延迟点之后构建期的写集非空即构建失败；交集实测见 §5.4 |
| U9 | （2026-10-06）内存缓存字段（`memo_fields`）撤回时保留缓存值（现状） |

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
