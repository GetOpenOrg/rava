# boot layer：引导模块层按字节码建立（FS-H12 模块部分）

分支 `boot-layer`（基于集成分支 29e52b54）。代码依赖 C1d-a a2（1e623cec 的 `[boot_init]` 与 `jdk/` 去截断，现于 c1d-p0）。

> 状态（2026-10-02）：第 0 步（语料规则，见第五节）✅ 已合入集成分支 27dfb419，抽查 bl0-27dfb419 8/9（TestModuleLayerDefine 为原有失败）；
> 第 1 步起等 C1d-a a2 合入集成分支。

## 一、现状（2026-10-02 按代码与 JDK 21 字节码核实）

### 1.1 手写面

| 位置 | 内容 |
|---|---|
| `module_layer_impl.rs` | `ModuleLayer.boot()` 返回进程内唯一的空层对象（`_init_not_null`，`cf` / `parents` / `nameToModule` 全未设置）；`parents()` 返回空列表 |
| `module_impl.rs` | `getLayer` 恒 null、`isNamed`、`canUse` 恒 true、`isExported` ×2 与 `isOpen` ×2 恒 true；`unnamed_module()` 进程唯一无名模块 |
| `module_impl.rs`（native） | `defineModule0` / `addReads0` / `addExports0` / `addExportsToAll0` / `addExportsToAllUnnamed0` 与 VM 模块表（HotSpot ModuleEntryTable 对应物）；`defineModule0("java.base")` 恒拒（「already defined」） |
| `class_impl.rs` | `Class.getModule` 两份手写：`getModule`（`#[jvm_boundary]`，另一个无名单例）与 `__impl_getModule`（返回 `unnamed_module()`）；`Class.module` 字段从不写入 |
| `closure.toml` | `Module` / `ModuleLayer` 在 `[vm_boundary]` 与 `clinit_carried`（理由写的是「CDS 归档模块图与引导层由 VM 构建」） |
| `services_catalog_impl.rs` | 引导层服务目录 `__boot_catalog` 按 `closure.json` 的服务事实装填（C1d 决策 4）；`getServicesCatalogOrNull` 过渡手写 |

`Module` / `ModuleLayer` 的 9 个 vm_boundary 方法 = `ModuleLayer.boot` / `parents` + `Module.getLayer` / `isNamed` / `canUse` /
`isExported`×2 / `isOpen`×2。

### 1.2 TestModuleLayerDefine 的失败点

1. 第 58 行 `ModuleLayer.boot().configuration()`：`configuration()` 按字节码 `getfield cf` 得 null，`.resolve(...)` 抛 NPE
   （native-gaps §用例）。
2. 修好 1 之后还有第二处：`isExported` / `isOpen` 手写恒 true，`a exports impl to b`（expected `false`）与
   `api open to b`（expected `false`）会错。这两个方法回到字节码后由 `Module` 的导出 / 开放表回答（表由 `defineModules`
   与 `addExports0` 等写入）。

### 1.3 JDK 21 的引导路径（javap 核实）

HotSpot `call_initPhase2` 调 `System.initPhase2(ZZ)I`：`bootLayer = ModuleBootstrap.boot()`，随后 `VM.initLevel(2)`。
无 CDS 归档、无模块选项时 `ModuleBootstrap.boot` 的路径：

1. `ArchivedBootLayer.get()` 为 null（`CDS.initializeFromArchive` 手写空操作），进 `boot2()`；
2. `jdk.module.*` 系统属性读取（upgrade.path / path / patch.0 / main / addmods.0 / limitmods / addreads.0 / addexports.0 /
   addopens.0 / showModuleResolution / validation / enable.native.access.*）全部为 null——原生二进制没有启动选项，
   `[facts.system_properties]` 按「启动时不存在」折叠；
3. `SystemModuleFinders.systemModules(null)` → `SystemModulesMap.defaultSystemModules()` → `new SystemModules$default()`；
   `SystemModuleFinders.of(systemModules)` 给每个模块建 `ModuleReference`（`jrt:/<模块>`，读取器惰性）；
4. `Modules.defineModule(null, java.base 描述符, uri)` 先定义 java.base（HotSpot `define_javabase_module`，此后 boot 类镜像的
   `module` 指向它）；
5. 不做解析：`JLMA.newConfiguration(finder, systemModules.moduleReads())`；
6. `ModuleLoaderMap.mappingFunction(cf)`（与 FS-C2 定义加载器表同一个 `ModuleLoaderMap$Modules`）；
7. `loadModules`：`BootLoader.loadModule` / `BuiltinClassLoader.loadModule` 写 `nameToModule` / `packageToModule`；
8. `ModuleLayer.empty().defineModules(cf, clf)` → `Module.defineModules`：每个模块 `new Module(...)` → `defineModule0`；
   java.base 复用 `Object.class.getModule()`；建读边、导出 / 开放表（`addExports0` 等）；`initServices` 把各模块的
   `provides` 登记进所属加载器的 `ServicesCatalog`；
9. `addExtraReads` / `addExtraExportsAndOpens` / `addEnableNativeAccess` 按空选项为空操作；`ArchivedModuleGraph.archive`
   只在 CDS 转储时写入。

`SystemModules$default.moduleDescriptors()` 是 jlink 生成的直线代码：约 10400 行字节码、1522 个调用，62 个模块的
`Builder` 链（与 C1d 项 5 分析器按 module-info 推出的 62 个模块一致）。

### 1.4 语料缺口：jmod 里的 `SystemModulesMap` 是桩

`resolve` 的 JDK 语料来自 `jmods/*.jmod`，运行时镜像（`lib/modules`）只补「jmod 中不存在」的类（`resolve/src/image.rs`
求差集）。jlink 会**改写**一部分 jmod 中已有的类，差集补不到它们。JDK 21 java.base 中 jmod 与镜像字节不同的类（逐个 cmp）：

| 类 | jmod 版本 | 镜像版本 |
|---|---|---|
| `jdk/internal/module/SystemModulesMap` | 四个方法全返回 null / 空数组 | 指向 `SystemModules$default` / `$all` |
| `java/lang/invoke/{DelegatingMethodHandle,DirectMethodHandle,Invokers,LambdaForm}$Holder` | 少量预生成 LambdaForm | jlink `generate-jli-classes` 插件补全 |

`SystemModules$default` / `$all` 是镜像独有类，已在语料里；但 `SystemModulesMap` 取的是 jmod 桩，`defaultSystemModules()`
返回 null，`boot2` 会走 `SystemModuleFinders.ofSystem()`（运行期读 jimage + 完整解析）。

### 1.5 c1d-p0 的 `[boot_init]` 与 FS-C2 的引导段

- c1d-p0 `seeds.toml [boot_init]`：`calls`（无参 `()V` 静态方法，闭包无条件作根，生成项目 main 先按序调用）与
  `classes`（引导期类初始化，在场者按序初始化）。现有条目 `System.setJavaLangAccess`、`AccessibleObject`。发射在
  `emit/src/project/entry.rs`，运行时入口 `java_runtime::vm_boot_init`。
- FS-C2 的 initPhase3 是另一种形态：`[vm_state.field_hooks]` 在首次读写 `ClassLoader.scl` / `Thread.contextClassLoader`
  时惰性执行 `ClassLoader.__vm_init_phase3`。

### 1.6 模块读取面在现有闭包中的可达性（本分支 rava closure 实测）

见附表 A：HelloWorld 不可达任何模块读取面；凡打印异常栈的用例都经 `StackTraceElement.computeFormat` 可达 `ModuleLayer.boot()`。

### 1.7 JDK 类变成命名模块后连带改变的路径

boot layer 建好后，JDK 类的 `getModule()` 返回命名模块。下列路径随之改走模块分支，都必须在终态里成立：

| 路径 | 改变 |
|---|---|
| `StackTraceElement.computeFormat` → `isHashedInJavaBase` | 先读 `ModuleLayer.boot()`，再比对帧类模块的层与 `HashedModules` |
| `Class.getResourceAsStream` / `getResource` | 命名模块分支 → `BootLoader.findResourceAsStream(mn, name)` / `BuiltinClassLoader.findResource(mn, name)` → `SystemModuleReader` → `ImageReader` → `NativeImageBuffer.getNativeMap`（native，读 `lib/modules`） |
| `BuiltinClassLoader.loadClassOrNull` | `findLoadedModule(cn)` 命中 → `findClassInModuleOrNull` → `defineClass(cn, LoadedModule)` 读类文件字节 → `defineClass1`（现为 LinkageError） |
| `ServiceLoader` 的模块 provider | 引导层服务目录由 `Module.defineModules` 的 `initServices` 填写，不再由 `__boot_catalog` 按事实填写 |
| `Reflection.verifyModuleAccess` / `Module.isExported` | 按真实导出表判定（`jdk.internal.*` 对无名模块不导出） |

## 二、终态

### 2.1 initPhase2 是 `[boot_init]` 的按需阶段

在 c1d-p0 的 `[boot_init]` 上加一种条目，不另起机制：

```toml
# VM 引导阶段（HotSpot call_initPhase2 / call_initPhase3）：按序在 calls / classes 之后、main 之前执行。
# anchors：阶段写入的 VM 状态的读取锚点；闭包内任一锚点可达时该阶段作根并在启动时执行，否则不入链、不执行。
[[boot_init.phases]]
call = "java/lang/System.initPhase2:(ZZ)I"
args = [false, false]                  # HotSpot：DisplayVMOutputToStderr、init 日志（缺省均为 false）
anchors = [
    "java/lang/System.bootLayer:Ljava/lang/ModuleLayer;",
    "java/lang/Class.module:Ljava/lang/Module;",
]
```

- **分析器**（`closure/src/lib.rs` 处理 `boot_init` 的位置）：阶段入口按锚点条件作根；锚点判定在不动点内单调（一旦作根不撤回）。
  `Class.module` 的读取按接收者值集判定：值集只含用户类 / 库类镜像时不算锚点（它们的模块是定义加载器的无名模块，与
  initPhase2 无关，见 2.3），与 FS-C2 `mirror_hook_field` 同一张定义加载器表。
- **发射**（`emit/src/project/entry.rs`）：作根的阶段以常量实参调用，返回值非 0 即启动失败（HotSpot
  `vm_exit_during_initialization`；`initPhase2` 自己的 `logInitException` 先打印原因），退出码 1。
- **清单装载**（`input/src/manifest.rs`）：`call` 必须是静态方法，`args` 个数与类型和描述符一致，`anchors` 必须是字段键。
- **为什么不用 FS-C2 的惰性读取钩子**：
  1. `Module.defineModules` 在建层途中自己读 `ModuleLayer.boot()`，JVM 在此读到 null；惰性钩子会在这里重入；
  2. boot 类镜像的 `module` 依赖 java.base 已被定义（1.3 第 4 步），其他层的 `defineModule0` 也依赖 java.base 已登记；
  3. initPhase2 写入的状态有一部分在 `ConcurrentHashMap` 里（`packageToModule`、各加载器的 `ServicesCatalog`），
     读取它们不经过可挂钩的字段。
  按需在 main 前执行与 JVM 次序完全一致，代价只落在闭包可达锚点的程序上。
- **引导档位**：`initPhase2` 里的 `VM.initLevel(2)` 按字节码写静态字段；`VM.initLevel()` 的手写读取模型（C1d-a 归 `VM`
  的 VM 契约，FS-C2 扩展了 initPhase3 段）在阶段执行期间报告该阶段的档位（initPhase2 内为 1，与 HotSpot 一致），其余时间为 4。
  清单事实 `VM.isModuleSystemInited:()Z = true` 在 initPhase2 闭包内的读取点只有 `BuiltinClassLoader.loadModule`（清资源缓存，
  缓存此时为 null）、`loadClassOrNull` 与 `findResource*`（建层期间不调用），按事实折叠不改变行为。实施时用
  `--why` 复核这一点；若有其他读取点在阶段内执行，该事实改为按档位求值，不保留常量。
- **initPhase3 并入同一机制**：FS-C2 的 `scl` / `contextClassLoader` 字段钩子改为 `[[boot_init.phases]]` 的第二个条目
  （`call = "java/lang/System.initPhase3:()V"` 中与系统类加载器相关的部分仍由 `ClassLoader.__vm_init_phase3` 承载，
  `anchors` 为这两个字段）。这样引导阶段只有一种落地方式，次序为 phase1（calls / classes）→ phase2 → phase3。

### 2.2 系统模块描述符的运行期承载（FS-H12）

**取舍：翻译 jlink 生成的 `SystemModulesMap` / `SystemModules$default` / `$all` 的字节码。**

| 方案 | 结论 |
|---|---|
| 翻译 `SystemModules$*` 字节码 | 采用。这正是 JVM 无归档启动时执行的代码；与 `ModuleLoaderMap$Modules`（FS-C2 定义加载器表）来自同一个镜像 |
| CDS 归档模块图（`ArchivedModuleGraph` / `ArchivedBootLayer`） | 不采用。归档是 VM 注入的堆快照，没有字节码；对应能力是 C3 的构建期类初始化（`build_time_init`，2026-10-01 用户决策归 C3）。C3 落地后 `ModuleBootstrap` 的结果可以成为快照事实，属 C3 的优化，不在本任务 |
| `SystemModuleFinders.ofSystem()`（运行期读 jimage 中的 module-info + 完整解析） | 不采用。需要运行期镜像与 `Resolver` 全套，代码量和运行期代价都更大 |

需要的改动：

1. **语料规则**：运行时镜像中与 jmod 字节不同的类以镜像为准（`resolve/src/image.rs` 从「求差集」扩为「差集 + 字节不同者」，
   不出现类名）。JDK 21 java.base 受影响的是 1.4 表中 5 个类；4 个 `$Holder` 类改变 MH-native 的语料，验收集含 MH 用例。
2. **大方法发射**：`moduleDescriptors()` / `moduleHashes()` 是数千行直线代码。实施第一步测 `java_runtime` 的 rustc 峰值内存与耗时；
   超过现有上限时做生成器的通用拆分（按基本块把长直线方法切成若干私有函数，不按类名特判），不裁剪描述符内容——
   `ModuleLayer.boot().modules()` 是可观测的。
3. **一致性守护**：分析器的引导层（C1d 项 5 `seeds/services.rs::catalog`，按 module-info 解析）与运行期的引导层
   （`SystemModules$default`）必须是同一组模块与 provider。加单元测试对两者的模块集合与 `provides` 逐项比对。

### 2.3 `Module` / `ModuleLayer` / `Class.module` 回到字节码

1. **手写清零**：
   - 删 `module_layer_impl.rs` 整个文件；
   - 删 `module_impl.rs` 的 `unnamed_module` 与 7 个 `#[jvm_boundary]` 方法；
   - `closure.toml` 中 `Module` / `ModuleLayer` 移出 `[vm_boundary]` 与 `clinit_carried`（`<clinit>` 只建 `ALL_UNNAMED_MODULE` /
     `EVERYONE_MODULE` / `EMPTY_LAYER`，纯 Java）；
   - 删 `Class` 的两份 `getModule` 手写。
   结果：`Module` / `ModuleLayer` 的 vm_boundary 方法 9 → 0，`vm_boundary_methods` 相应减 9（同时减去 `Class.getModule`）。
2. **保留的 native（准入 ①）与一处语义修正**：`defineModule0` 等 5 个 native 与 VM 模块表保留。`defineModule0("java.base")` 改为
   HotSpot `define_javabase_module` 的语义：引导加载器上的第一次定义成功并登记，此后再定义才抛「already defined」。
3. **`Class.module` 是 VM 注入状态**（HotSpot `create_mirror` / `fixup_module_field_list`）：
   - `[vm_state.field_hooks]` 加接收者钩子 `"java/lang/Class.module:Ljava/lang/Module;" = { hook = "java/lang/Class.__vm_module", receiver = true }`；
   - 生成器给类块加 `module` 属性（`cp.module_of`，只给命名模块；与 `defining_loader` 同路），`java_meta` 生成对应表；
   - 钩子按表填写：
     - 用户类 / 库类：定义加载器的 `getUnnamedModule()`（字节码字段，`ClassLoader` 构造器建立）；
     - 命名模块中的 JDK 类：VM 模块表中（定义加载器，模块名）的条目，由 initPhase2 的 `defineModule0` 登记；
     - 数组：元素类型的模块；基本类型：java.base；
     - 无模块归属的镜像独有类 / VM 支持类：`BootLoader.getUnnamedModule()`；
   - 表中查不到命名模块，说明分析器漏判了锚点，属分析器缺陷：钩子以 panic 报出类名，不回退到无名模块。
4. **服务目录**：删 `ServicesCatalog::__boot_catalog`、`build.rs` 的 `services_table` 生成与 `getServicesCatalogOrNull` 过渡手写。
   引导层目录由 `Module.defineModules` 的 `initServices` 按描述符填写。分析器的服务事实继续决定哪些 provider 类入闭包
   （`ServiceLoader` 按名加载 provider，事实与运行期目录一致由 2.2 第 3 条守护）。
5. **已加载类**（VM 状态，准入 ③）：原生镜像中闭包内的类都已由各自的定义加载器加载。
   - `ClassLoader.findLoadedClass0`：类的定义加载器（FS-C2 表）等于本加载器时返回该类，否则 null；
   - `findBootstrapClass`：只返回引导加载器定义的类。
   这样 `BuiltinClassLoader.loadClassOrNull` 不论走 `findLoadedModule` 分支还是委派分支，都在 `findLoadedClass` 一步命中，
   不进入 `findClassInModuleOrNull` → `defineClass`。
   **已实施**（c1d-a2c 57ca684a，`class_loader_impl.rs`；抽查 c1db-sp-57ca684a 加载器 5 例 + HelloWorld + TestCustomException 7/7）。
6. **模块内容读取器**：命名模块的资源经 `SystemModuleReader` → `ImageReader` → `BasicImageReader`（`USE_JVM_MAP` 缺省为真、
   读取方为引导类）→ native `NativeImageBuffer.getNativeMap`。
   - 生成器把闭包推出的资源集（`input/src/resources.rs`）写成 jimage 格式的嵌入数据；
   - `getNativeMap` 是唯一手写（准入 ①），返回覆盖该数据的直接缓冲区；
   - `jdk/internal/jimage` 按字节码翻译；
   - `ClassLoader` 资源族的手写（FS-C2 保留的嵌入表查询）改读同一份数据，资源只有一个来源。

### 2.4 闭包预算

- **HelloWorld**：类 / 方法集合与现状逐项相同（266 / 723）。前提是 HelloWorld 闭包中不可达任何锚点（附表 A 实测）。
- **可达锚点的程序**：付出 initPhase2 的代价（`ModuleBootstrap` + `SystemModules$default` + `Configuration` / `Module.defineModules`
  + 模块读取器族）。实施第一步给出验收集每例的增量，按包列出。
- **栈回溯格式化**：`isHashedInJavaBase` 先无条件读 `ModuleLayer.boot()`。凡是格式化栈帧的程序都可达锚点，这与 JVM 语义一致，
  不做折叠。

## 三、实施步骤（每步 ≤40 分钟，单独提交）

第 0 步是语料来源规则的改变，影响所有测试，按「结构性改动单独先合」独立提交、独立抽查（MH / lambda / 反射），合入集成分支后
后续步骤在它之上进行；它不依赖 C1d-a a2。第 1 步起依赖 a2（`[boot_init]`）。

| 步 | 内容 | 验证 |
|---|---|---|
| 0 | 语料规则：镜像中与 jmod 字节不同的类以镜像为准（2.2 第 1 条），不夹带其他内容 | resolve 单测；生成对照（见第五节）；协调方抽查 MH / lambda / 反射 |
| 1 | `[[boot_init.phases]]` 清单形态、装载校验、分析器按锚点作根、发射；initPhase2 条目登记，提交时锚点留空。本地临时填入锚点实测 TestCustomException / TestStackWalkerFrames / TestAppClassLoader 三例的增量（类 / 方法 / 转译耗时 / 编译耗时）与 `SystemModules$default` 的 rustc 峰值内存，写入附表 C；单例超过 300 类另立精度项；rustc 超限则做大方法通用拆分 | 单元测试；HelloWorld 生成树逐字节不变 |
| 2 | `Class.module` 钩子与类块 `module` 属性；`findLoadedClass0` / `findBootstrapClass` 按定义加载器；`defineModule0` java.base 语义 | TestAppClassLoader / TestClassNestNatives |
| 3 | 锚点启用；删 `module_layer_impl.rs`、`module_impl.rs` 的 7 个方法、`Class.getModule` 手写；移出 `[vm_boundary]` / `clinit_carried` | TestModuleLayerDefine 原样通过；新边界用例 TestBootLayer |
| 4 | 服务目录走 `initServices`；删 `__boot_catalog` / `services_table` / `getServicesCatalogOrNull`；分析器 / 运行期引导层一致性单测 | TestCharsetForName、ServiceLoader 用例 |
| 5 | 模块内容读取器：jimage 嵌入数据 + `getNativeMap`；`ClassLoader` 资源族改读同一数据 | 读 JDK 资源的用例（ICU 规范化、tzdb、字体 / 区域数据） |
| 6 | initPhase3 并入 `[[boot_init.phases]]`，删 FS-C2 的两个字段钩子 | TestAppClassLoader / TestParallelCapable |

## 四、验收

- TestModuleLayerDefine 原样通过（测试与 expected 都不改）。
- 新边界用例 `tests/e2e/62_reflection/TestBootLayer.java`（源码与 JDK 21.0.11 实测 expected 见附录 B）。覆盖：
  - boot layer 模块集合（稳定子集）、`parents`、`configuration`；
  - `findModule`；
  - `Module.getDescriptor`（java.base / java.sql 的 requires、exports）；
  - `isExported` / `isOpen`（无条件与限定，含 `jdk.internal.misc` 不导出）；
  - `getLayer`；
  - 命名模块类（引导 `String`、平台 `java.sql.Time`）、未命名模块类（用户类、嵌套类、数组）、基本类型、数组的 `getModule`。
- `non_native_overrides` = 0。
- `vm_boundary_methods` 减去 `Module` / `ModuleLayer` 的 9 个与 `Class.getModule`。
- HelloWorld 闭包与现状集合一致；可达锚点的用例报告增量。
- 抽查清单（≤10）：TestModuleLayerDefine、TestBootLayer、TestAppClassLoader、TestClassNestNatives、TestStackWalkerFrames、
  TestCustomException、TestCharsetForName、一例 MH 组合子用例、HelloWorld、ReflectionGetSource。

## 五、第 0 步实测（语料规则）

- 求差方式：首次对某个 JDK 整体 `jimage extract`（JDK 21：28126 个类、2.8 s），与全部 jmod 逐字节比对，只保留差异类，
  缓存 `rava/jimage/<指纹>/{only,rewritten}/<模块>` + `index.txt`（共 352 KB）；以后只读索引，不再打开 jmod。
- JDK 21.0.11 全部 69 个模块中，镜像改写类恰为 1.4 表中 java.base 的 5 个；镜像独有类 18 个，与旧缓存一致。
- 改写类装入 JDK 语料时来源角色仍为 JDK、`module_of` 仍报 java.base（resolve 单测守护，不含类名）。
- 生成对照（同一 rava、切换改写类，`--stop-after emit`）：HelloWorld、LambdaBasic、TestMethodHandleDirect、
  TestBmhDynamicSpecies、TestMethodHandleCombinators、TestReflectInvokeShapes、TestModuleLayerDefine、TestCustomException
  八例闭包类 / 方法集合完全相同。`SystemModulesMap` 无一例入链（按需建层前不可达）。生成差异只在 4 个 `$Holder` 类的文件
  （镜像版方法更多，均为调用链外存根），以及随之变化的分片 `use` 行 / `mod.rs`。
- TestMethodHandleDirect 新语料编译通过，运行输出与 expected 一致。

## 附表 A：模块读取面可达性（实测）

`rava build --stop-after emit` 后读 `closure.json`（本分支 29e52b54 + rava 新构建）：

| 用例 | 类 / 方法 | 可达的模块读取面 |
|---|---|---|
| HelloWorld | 266 / 723 | 无 |
| TestStackWalkerFrames | 1454 / 7957 | `ModuleLayer.boot`、`Class.getModule`、`StackTraceElement.computeFormat` / `isHashedInJavaBase`、`Module.getLayer` / `isNamed` / `isExported` / `canUse`、`BuiltinClassLoader.findLoadedModule` |
| TestCustomException | 1419 / 7687 | 同上 |
| TestAppClassLoader | 1422 / 7708 | 同上 |

结论：
- HelloWorld 不可达任何锚点，按 2.1 的按需作根闭包不变，满足「不得增长」。
- 打印异常栈的程序（`Throwable.printStackTrace` → `StackTraceElement.toString` → `computeFormat`）都可达
  `ModuleLayer.boot()`，会付出 initPhase2 代价。这是 JVM 语义（`isHashedInJavaBase` 无条件读引导层），不折叠。
  增量（类 / 方法 / 转译耗时 / 编译耗时）在实施第 1 步实测并按包列出，写入附表 C；如果单例增量超过 300 类，另立「栈帧格式化只读 java.base 层」的精度项，
  在分析器值流里证明 `computeFormat` 只比较 java.base 的哈希，不在本计划内降低语义。

## 附录 B：TestBootLayer

`tests/e2e/62_reflection/TestBootLayer.java`（实施第 3 步入库）：

```java
import java.lang.module.ModuleDescriptor;
import java.util.Set;
import java.util.TreeSet;
import java.util.stream.Collectors;

/**
 * 引导模块层（boot layer）：ModuleBootstrap.boot 在 initPhase2 按系统模块描述符建层、写入 System.bootLayer。
 * 覆盖：层内模块集合（稳定子集）、findModule、Module.getDescriptor、isExported / isOpen（无条件与限定）、
 * getLayer、命名模块类（引导 / 平台）与未命名模块类（用户类、嵌套类、数组、基本类型）的 getModule。
 */
public class TestBootLayer {
    static class Nested {}

    public static void main(String[] args) {
        ModuleLayer boot = ModuleLayer.boot();
        Set<String> names = boot.modules().stream().map(Module::getName).collect(Collectors.toCollection(TreeSet::new));
        System.out.println("boot same = " + (ModuleLayer.boot() == boot));
        System.out.println("has java.base = " + names.contains("java.base") + ", java.sql = " + names.contains("java.sql")
                + ", java.logging = " + names.contains("java.logging"));
        System.out.println("parents = " + boot.parents().size() + ", parent is empty = " + (boot.parents().get(0) == ModuleLayer.empty()));
        System.out.println("configuration has java.base = " + boot.configuration().findModule("java.base").isPresent());

        Module base = boot.findModule("java.base").orElseThrow();
        Module sql = boot.findModule("java.sql").orElseThrow();
        System.out.println("findModule(no.such) = " + boot.findModule("no.such").isPresent());
        System.out.println("String module == base: " + (String.class.getModule() == base));
        System.out.println("base named = " + base.isNamed() + ", name = " + base.getName() + ", layer is boot = " + (base.getLayer() == boot));
        System.out.println("base loader = " + base.getClassLoader());
        System.out.println("sql loader = " + sql.getClassLoader().getName());

        ModuleDescriptor bd = base.getDescriptor();
        System.out.println("base descriptor = " + bd.name() + ", open = " + bd.isOpen() + ", automatic = " + bd.isAutomatic()
                + ", requires = " + bd.requires().size());
        System.out.println("base exports java.lang unqualified = "
                + bd.exports().stream().anyMatch(e -> e.source().equals("java.lang") && !e.isQualified()));
        System.out.println("base packages has jdk.internal.misc = " + bd.packages().contains("jdk.internal.misc"));
        ModuleDescriptor sd = sql.getDescriptor();
        System.out.println("sql requires = " + sd.requires().stream().map(ModuleDescriptor.Requires::name)
                .collect(Collectors.toCollection(TreeSet::new)));
        System.out.println("sql exports = " + sd.exports().stream().map(ModuleDescriptor.Exports::source)
                .collect(Collectors.toCollection(TreeSet::new)));
        System.out.println("sql reads base = " + sql.canRead(base) + ", base reads sql = " + base.canRead(sql));

        Module user = TestBootLayer.class.getModule();
        System.out.println("isExported(java.lang) = " + base.isExported("java.lang")
                + ", to user = " + base.isExported("java.lang", user));
        System.out.println("isExported(jdk.internal.misc) = " + base.isExported("jdk.internal.misc")
                + ", to user = " + base.isExported("jdk.internal.misc", user));
        System.out.println("isOpen(java.lang) = " + base.isOpen("java.lang") + ", to user = " + base.isOpen("java.lang", user));
        System.out.println("isExported(no.such.pkg) = " + base.isExported("no.such.pkg"));

        System.out.println("user named = " + user.isNamed() + ", name = " + user.getName()
                + ", layer = " + user.getLayer() + ", descriptor = " + user.getDescriptor());
        System.out.println("user == scl unnamed: " + (user == ClassLoader.getSystemClassLoader().getUnnamedModule()));
        System.out.println("user exports / opens any = " + user.isExported("x.y") + " / " + user.isOpen("x.y", base));
        System.out.println("user reads base = " + user.canRead(base));
        System.out.println("nested == user: " + (Nested.class.getModule() == user));
        System.out.println("Nested[] == user: " + (Nested[].class.getModule() == user));
        System.out.println("java.sql.Time module = " + java.sql.Time.class.getModule().getName());
        System.out.println("int module = " + int.class.getModule().getName());
        System.out.println("String[][] module = " + String[][].class.getModule().getName());
        System.out.println("platform unnamed != user: " + (ClassLoader.getPlatformClassLoader().getUnnamedModule() != user));
    }
}
```

`tests/expected/TestBootLayer.txt`（JDK 21.0.11 实测）：

```
boot same = true
has java.base = true, java.sql = true, java.logging = true
parents = 1, parent is empty = true
configuration has java.base = true
findModule(no.such) = false
String module == base: true
base named = true, name = java.base, layer is boot = true
base loader = null
sql loader = platform
base descriptor = java.base, open = false, automatic = false, requires = 0
base exports java.lang unqualified = true
base packages has jdk.internal.misc = true
sql requires = [java.base, java.logging, java.transaction.xa, java.xml]
sql exports = [java.sql, javax.sql]
sql reads base = true, base reads sql = false
isExported(java.lang) = true, to user = true
isExported(jdk.internal.misc) = false, to user = false
isOpen(java.lang) = false, to user = false
isExported(no.such.pkg) = false
user named = false, name = null, layer = null, descriptor = null
user == scl unnamed: true
user exports / opens any = true / true
user reads base = true
nested == user: true
Nested[] == user: true
java.sql.Time module = java.sql
int module = java.base
String[][] module = java.base
platform unnamed != user: true
```

## 附表 C：按需建层的闭包增量（第 1 步实测后填写）

| 用例 | 类 | 方法 | 转译耗时 | 编译耗时 |
|---|---|---|---|---|
| TestCustomException | 480 → 3193（+2713） | 1867 → 18618 | 1 s → 18 s（`rava closure`） | 未测（超 300 类，锚点不启用） |
| TestStackWalkerFrames | 3108 → 3257（+149） | 17934 → 19283 | — | 未测 |
| TestAppClassLoader | 3108 → 3257（+149） | 17891 → 19236 | — | 未测 |

实测于 c1d-a2c b47568ec，本地临时 runtime（锚点 `System.bootLayer`，`ModuleLayer` 移出 vm_boundary）。TestCustomException
超过 300 类，按第 1 步规则另立精度项（URL / URI 协议事实 + 分派变宽限于 phase 帧），第 2–5 步以其为前置；
二分与路径见 `2026-10-01-c1d-closure-bloat.md` §23.5。`SystemModules$default` 的 rustc 峰值内存未测。

**2026-10-05 补测（c1d-boot 8bb25e10，详见 c1d 文档 §25）**：锚点加上 `Class.module`、删掉两处 `Class.getModule` 手写之后，
HelloWorld 经 `FileOutputStream.<clinit>` → `SharedSecrets.ensureClassInitialized` → `VerifyAccess.isClassAccessible` 读到
`Class.module`，从而作根，单例闭包 469 → 3190 类。全部用例都会这样作根。档案并集（服务器 1084 例，7886 类）只增加约 20 类，
都是 `jdk/internal/module` 引导类。所以档案规模不是瓶颈，瓶颈是生产构建的单例规模。

第 2–5 步暂不开工，前置条件见 c1d §25.4：`Class` 实例方法按接收者镜像求 `classLoader` / `module`、容器元素类型、实例汇合点。
三项合入后重测，判据为 HelloWorld ≤ 569 类。具体求值器执行 initPhase2 的尝试在 `System.<clinit>` / SharedSecrets / `System.props`
处失败，因为那需要构建期引导映像语义（§25.3），不在本线范围内。
