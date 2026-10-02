# Rust 精确闭包分析器（rava closure）

> 关联：`docs/plans/2026-09-20-rust-generator-rewrite.md`（crate 布局，本计划是其 classfile / resolve / driver 三个 crate 的先行落地）、
> `docs/plans/2026-09-19-trace-callchain-complete.md`（trace 脚本多模式对比）、`docs/plans/2026-09-29-gap-closure.md`（gap_scan 口径）、
> 数据源 `docs/reports/jdk-scan-HelloWorld.md`（2026-09-29，git 4bf5be0f）、`docs/reports/trace-HelloWorld.md`（2026-09-19）。

## 一、现状数据

| 口径 | 类 | 翻译方法体类 | 入链方法 |
|---|---:|---:|---:|
| 现行生成器（jdk-scan-HelloWorld，JDK 21） | **1854** | **1358** | **20851** |
| 其中：仅类型存根 / 手写边界（内部包 + VM 耦合） | 193 / 303 | — | — |
| trace_callchain Two-pass RTA（09-19） | 834 | 628 | 3441 |
| trace_callchain Internal Boundary（09-19） | 174 | 93 | 388 |

HelloWorld 只做一次 `println`，却有 `java/util/stream` 169 个翻译类（2909 方法）、`java/lang/invoke` 136 个（3244）、
`jdk/internal/reflect` 90 个（1244）、`java/util/regex` 62 个、`java/time/*` 约 80 个、`sun/security/provider` 16 个。
十天内方法数从 3441 涨到 20851（×6），增量几乎都来自后加的「补漏机制」，调用链本身并没有变长。

## 二、膨胀机制诊断（按代码位置）

现行发现逻辑在 `codegen/callchain.py`（1916 行），可以概括为「全局流不敏感 RTA + 多条补漏通道 + 若干启发式种子」，外面套一个多轮不动点。
逐项如下：

| # | 机制 | 位置 | 过近似的性质 |
|---|---|---|---|
| M1 | **接口分派按 CHA 走**：接口方法入链 → 遍历 `jdk_infos` 里**全部**实现类入队，不看是否实例化 | `_propagate_virtual_targets` 接口分支；函数尾「迟至实现者清扫」 | 最大放大器：`Iterator.next`、`Consumer.accept`、`PrivilegedAction.run` 一类接口方法把闭包里每个实现者的方法体都拉进来，方法体又引入新类，形成滚雪球 |
| M2 | **全局流不敏感 RTA**：只要任一已入链方法（包括异常路径、不可达分支）里出现 `new X`，X 就算已实例化 | `instantiated_classes` | 所有虚调用对全局实例化集合做分派，不按调用点实际能流到的类型收窄 |
| M3 | **根虚方法广播**：`Object.toString/hashCode/equals` 出现一次 → 对**全部**已实例化类入队覆盖版本 | `root_virtual_targets` | 每个实例化类的 `toString` 都会带出 StringBuilder / Formatter / 数值格式化链 |
| M4 | **类型通道传递闭包**：入链方法描述符里的全部类型、catch 类型、`ldc class`、字段类型（stub 通道递归展开）、父类链、传递接口闭包 | `_enqueue_desc_types`、`_enqueue_iface_stub`、stub 通道、父类补全队列、收尾接口 drain | 只在签名里出现的类型也按「整个类」进入 registry；它的字段类型、父类、接口再继续递归 |
| M5 | **`<clinit>` 级联**：凡是 getstatic / invokestatic / new 都触发初始化，而触发点本身来自 M1–M3 的过近似集合 | `_enqueue_class_init` | 语义正确，但被上游放大（Character、Locale、Currency 等的大表初始化） |
| M6 | **反射启发式**：ldc 字符串与本类方法同名即入链；拼接前缀、巢内前缀、驼峰中缀匹配；镜像独有类 / 物种族**全部方法** + 全量反射面 | `_process` 字符串扫描、`_drain_nest_prefixes`、`_seed_image_subclasses`、`_seed_species_family` | `java/lang/invoke` 与 `jdk/internal/reflect` 的主要来源；按名字模式匹配，不做数据流 |
| M7 | **手写层依赖用正则扫描**：`_impl.rs` 里的 `crate::…` 全路径和裸驼峰名都按类型收录；upcalls / N6 分配按名字前缀判断 `provides` | `_impl_signature_type_refs`、`NativeUpcalls` | 注释、未调用的辅助函数里出现的名字也会入闭包；有重载时容易误判 |
| M8 | **种子触发条件也被放大**：locale / JCA / 注解种子按「触发成员 ∈ seen_members」激活，而 seen_members 本身来自 M1–M3 | `_seed_data_bundles`、`_seed_jca_services`、`_seed_annotation_types` | HelloWorld 不用加密，却进了 SecureRandom → sun/security/provider 16 类 |
| M9 | **补扫 / 门控补丁**：迟至静态边补扫（三道门）、`_drain_iface_edges` 规避加载副作用、到处 `sorted()` 保证确定性 | 函数尾部各段 | 这些不是分析规则本身，而是 M1–M4 互相作用后的修补层；注释里记着多次「全量重跑会扰动」的教训 |

根因：**分析的基本单位是「类 + 全局集合」，不是「调用点 + 流到该点的类型」。**
缺少精度，就只能靠加通道补漏；每加一个通道都是过近似，还会和其它通道互相放大。

## 三、精确分析的几个角度

分析器要把「链路关系」拆成下面几个互相独立、各自精确的视角，每条入闭包的边都要能说清楚是哪一类原因、从哪条字节码来的。

### 3.1 调用视角：调用点级类型传播（XTA）替代全局 RTA / CHA

- 每个方法、每个字段（静态字段、实例字段按「声明类.字段」）、每种数组元素类型各维护一个**类型集合**（可能流入的具体类）。
- `new X` 只把 X 放进**当前方法**的集合；集合沿调用实参、返回值、字段读写、数组读写传播，由 checkcast、声明类型过滤。
- 虚调用 / 接口调用的目标 = 接收者声明类型 ∩ **流到该方法的集合**，按 JVMS §5.4.6 选择方法。这样可以同时去掉 M1 / M2 / M3。
- 方法内部配合栈模拟（抽象解释）得到调用点接收者的更精确来源（局部 `new`、`this`、参数、字段、常量 null）。
- 参考：trace_callchain.py 已验证 VTA / Inter-proc VTA 比单程 RTA 少约 10% 的类；XTA 加上下面的剪枝，收益会叠加。

### 3.2 控制流视角：死分支剪枝（常量与 VM 事实）

- 在方法 CFG 上做常量传播：`static final` 常量（ConstantValue 属性）、`$assertionsDisabled`、以及 **VM 事实**。
- VM 事实写进 `vm_intrinsics.toml` 新增的 `[facts]` 段（清单即边界，生成器里不出现类名），例如：`System.getSecurityManager` 返回 null、`VM.isBooted` 为 true、`allowSecurityManager` 返回 false。
  分支条件一旦被判为常量，就不分析另一侧：SecurityManager 检查、doPrivileged 的旧路径、调试开关都会整片消失。
- 只在**能证明**是常量时才剪枝；不确定就保留两侧（保持安全）。

### 3.3 初始化视角：`<clinit>` 只由可达触发点驱动

- 触发点（JVMS §5.5：new / getstatic / putstatic / invokestatic / 子类初始化 / 带 default 方法的超接口）只从**可达指令**收集。
- `<clinit>` 自身也按 3.1 / 3.2 分析；只给 ConstantValue 常量赋值的 `<clinit>` 不产生依赖。
- 读 `static final` 编译期常量不触发初始化（javac 已内联，保持 JVMS 语义）。

### 3.4 异常视角：抛出点与处理器按可达性关联

- catch 处理器只在 try 范围内有可达的调用或 `athrow` 时才分析；catch 类型不在任何可达抛出集合里时，处理器视为不可达。
- 可达的 `athrow` / 异常构造照常入链；异常消息拼接只随真正可达的抛出点进入。

### 3.5 类型需求视角：类按需求等级分级，不再「整类入闭包」

| 等级 | 条件 | 生成内容 | 是否继续传递依赖 |
|---|---|---|---|
| L0 | 不可达 | 无 | 否 |
| L1 名字 | 只出现在签名、checkcast、instanceof、catch 中，且**从未实例化** | 不透明引用类型（一行声明，值只可能是 null） | **否** |
| L2 布局 | 已实例化（或是 L2 / L3 的父类且有字段被读写） | struct + 实际读写到的字段 | 只带出字段类型的 L1 |
| L3 行为 | 至少一个方法体可达 | L2 + 可达方法体 + 所需 vtable 槽 | 按 3.1 继续 |

- instanceof / checkcast / catch 的目标类型如果从未实例化（且没有已实例化的子类），结果可以折叠成常量（instanceof → false，checkcast 只可能是 null），相应分支继续按 3.2 剪掉。
- 接口只在「有可达接口调用」或「用于 instanceof / checkcast 且存在已实例化的实现者」时升到 L2 / L3；否则是 L1。M4 的传递闭包整体消失。

### 3.6 反射视角：字符串常量数据流替代名字模式匹配

- 在 3.1 的抽象值域里加入 `ClassConst(C)` 与 `StrConst(s)`，并沿参数、返回值、字段传播。
- 只在 `vm_intrinsics.toml` 新增的 `[reflect_sinks]` 登记的汇点（`Lookup.findStatic/findVirtual/findGetter`、`Class.getDeclaredMethod/getDeclaredField`、`MemberName.<init>` 等）上，按汇点实参的**常量对**解析目标成员。
- 实参不是常量时：记为「反射缺口」并输出到报告（可观测），由清单显式补种，不做模糊匹配。M6 的前缀、巢内、驼峰启发式全部由此替代。
- 镜像独有 / 物种类：只对数据流里实际流到的 speciesCode（或清单登记的物种）生成，不再对整族生成全量反射面。

### 3.7 手写层视角：用 `syn` 精确解析 `_impl.rs`

- 不再用正则：解析 `#[jvm_native(upcalls = …)]` / `#[jvm_boundary(…)]` 属性、`T::default()` + `_init_not_null()` 分配点、fn 签名与函数体里的**类型路径**（按 Rust 名称解析规则解析 `use` 和同模块名）。
- 每个手写 fn 都作为图上的节点（「手写体」），它的回调、分配、类型引用是这个节点的出边；**只有它被触达时**这些出边才生效。
- **回调由算法推断，声明只兜底**（`engine/hw_infer.rs`）：手写层与生成层命名空间同构，调用点按「接收者静态类型（方法调用）/ 路径类型（`T::m(…)` 关联函数）+ Rust 名 + 实参个数」反解为 Java 方法引用，与 `upcalls` 声明同等处理。
  - 接收者静态类型的来源：形参 / `let` 标注、构造调用、字段访问器 `__get_f()`，以及链式调用的返回类型（`SType::Call`：`options.iterator()` 的返回类型取 Java 描述符 → `it.next()` 的接收者为 `Iterator`）。
  - 名字匹配规则与 `hw_member` 一致：先匹配无重载时的裸名或描述符 mangle 名，全层次都不中时再按 Java 名前缀回退。手写层对重载成员也可能用裸名或缩写后缀调用。
  - Rust 内建 trait 同名方法（`clone`）不作为 Java 回调。
  - 实证：精确分派下，未声明的回调直接暴露为运行期存根（TestFilesApi：`newByteChannel` → `UnixChannelFactory.newFileChannel` → `Set.iterator`）。旧 BFS 是按 CHA 过近似，把这类遗漏掩盖了。
- **终态：回调全部由算法推出，人工 `upcalls` 声明为 0**（2026-10-01 用户确认，见 3.7.1）。

#### 3.7.1 回调零声明（C6）

现状（2026-10-01，粗扫）：手写层 173 个 fn 共 426 条 `upcalls` 声明，分布在 66 个文件。其中 132 条的目标方法在同一 fn 体内语法可见，
294 条不可见；`error.rs` 头部的 `vm-upcalls` 行另有 18 条。c293b082（反射访问器拆箱、`MethodHandle.invoke` 调用点）用声明填补的是
推断缺口，不是语法不可见的调用，属于本节要消除的对象。

每一类回调边由下表对应的算法推出，不再逐个函数声明：

| 类别 | 推断方式 |
|---|---|
| 同文件 / 跨文件辅助函数 | 手写 fn 建全局调用图，回调、分配、类型引用沿调用传递（现只传递同文件调用） |
| 运行时内部方法（`ObjectVTable` 的 `__obj_str` 等） | 内部方法映射到对应的 Java 虚方法（`toString` / `hashCode` / `equals`），再按已实例化子类分派展开 |
| 签名多态调用点（`invoke` / `invokeExact` → `invoke__site`） | 按 `vm_intrinsics.toml [sigpoly]` 的映射，把 `__site` 体作为该成员的手写体扫描 |
| 宏内调用 | 自有过程宏由分析器调用 `rava_macros_core::expand_*` 展开后扫描；自有 `macro_rules` 同样展开，或改写为普通函数 |
| 手写体按字符串名反射 | 名字为常量时，用与 `engine/reflect.rs` 同类的常量传播解析；名字不是常量的手写代码改成直接调用 |
| VM 抛出的异常（现 `vm-upcalls` 的异常构造器） | 按 JVMS 指令语义从字节码推出（如 `aaload` → `ArrayIndexOutOfBoundsException`，`invokevirtual` → `NullPointerException`），只在含相应指令的可达方法里生效 |
| VM 驱动的回调（线程入口、关闭钩子、类初始化错误等） | 分析器规则目录中的 VM 语义规则：按 Java 语义事件触发，每条带规则 ID 与 JVMS / JDK 依据；不在各个函数上声明 |

前提：手写层保持与 Java 命名空间同构（现行约定）；手写代码里不出现名字无法推出的反射。

声明归零后，承载声明的机制一并删除，`upcalls` 属性参数不再存在：

| 删除项 | 位置 | 替代 |
|---|---|---|
| `upcalls = "…"` 参数的解析与说明 | `runtime/rava_macros/src/native_attr.rs`（宏忽略该参数）；`#[jvm_boundary]` / `#[jvm_ext]` 同 | 无（`#[jvm_native]` 等属性只保留 `no_class_init` 等其余参数；`#[jvm_boundary]` 无其余参数时整个属性删除） |
| 声明解析 `upcalls_of`、`FnInfo` / `MemberHw` 的 `upcalls` 字段 | `generator/crates/closure/src/handwritten/scan.rs`、`handwritten.rs` | 回调边只来自推断（`engine/hw_infer.rs`），`hw_upcalls` 不再并入声明 |
| `error.rs` 头部 `// vm-upcalls:` 行与 `vm_upcalls()`、`root_upcall(…, "vm-upcalls")` | `handwritten.rs`、`lib.rs` | JVMS 指令异常推导与 VM 语义规则（3.7.1 表末两行） |
| VM 钩子判定「pub fn 且带声明」 | `handwritten/hooks.rs` `is_hook` | 改为「不对应该类及超类型任何 Java 成员的 pub fn」，钩子体的回调边同样由推断得出 |
| 系统属性改写判定读声明 | `engine/sysprops.rs` `sysprops_hw` | 改读推断结果（`hw_upcalls`） |

`closure.toml [dynamic] vm_upcall_classes`（JVM 链接期直接调用的入口类，`dyn_compare` 用于归因）不属于手写层声明，不在本项范围；
它是否同样可由规则推出，在 VM 语义规则落地时一并评估。

验收：手写层 `upcalls` 声明 = 0；`error.rs` 的 `vm-upcalls` 行 = 0；上表机制已删除（仓库内 `upcalls` 属性参数与 `vm-upcalls` 行的解析代码为 0）；删除声明后，程序正确运行（e2e 通过，动态对照翻译域漏覆盖 = 0）、闭包不纳入不必要的类（过宽声明带来的多余类随删除去掉，二进制随之变小）——闭包缩小是目标而非风险，判据是运行正确，不是集合不变（2026-10-01 用户口径）；
相关 e2e 通过。实施先输出分类统计（现已可删 / 补齐哪一类推断后可删），再按类别小步提交。

### 3.8 动态视角：真实 JVM 运行轨迹作为健全性基准（只用于验证）

- `java -Xlog:class+load -Xlog:class+init` 跑同一测试，得到**实际加载 / 初始化的类**。
- 判据：动态集合中属于翻译域的类，必须 100% 被静态闭包覆盖（否则说明静态分析漏边）；静态有而动态没有的类，每一个都要能用 provenance 解释。
- 动态轨迹不参与闭包计算（运行期路径依赖输入），只作为验收时的对照。

#### 3.8.1 落地（C5，2026-09-30）

- 一次 java 运行：`-Xshare:off -agentpath:load_trace=<轨迹> -Xlog:class+load,class+init`。`-Xlog` 给加载全集（静态多出的判据）
  与「程序期」加载序列（主类 `Initializing` 之后，排除启动器 / 反射找 main）；原生 JVMTI agent 给每次加载的 Java 调用栈。
  java.lang.instrument agent 已实测否决：其装载提前加载整套 MethodHandle 基础设施，每例 42–56 个类失去调用栈。
- 噪声排除全部按规则（无类名特判）：隐藏类（lambda 代理 / LambdaForm）只计数；边界域类单列；翻译域类按调用栈自栈底上溯，
  途经手写 / native 方法 → `handwritten`，边界域帧 → `boundary-code`，经边界接口虚派发进入其翻译域实现（帧签名与闭包 refs
  中的边界域方法一致）→ `boundary-dispatch`，`closure.toml [dynamic] vm_upcall_classes`（JVM 链接期直接调用的入口）→ `vm-upcall`，
  栈底即不在闭包（VM 自启线程）→ `vm-entry`；其余一律计漏覆盖。
- 实测（JDK 21，5 例）：

| 测试 | 漏覆盖 | 静态多出 | provenance | 未归因 | 归因（handwritten / boundary-dispatch / vm-upcall / vm-entry / 边界域） |
|---|---:|---:|---:|---:|---|
| TestArrayList | 0 | 80 | 100% | 0 | 4 / 0 / 0 / 0 / 0 |
| TestHashMapOps | 0 | 60 | 100% | 0 | 4 / 0 / 66 / 0 / 31（+ boundary-code 1） |
| TestStreamBasic | 0 | 95 | 100% | 0 | 4 / 42 / 13 / 0 / 32（+ boundary-code 1） |
| TestFilesApi | 0 | 71 | 100% | 0 | 23 / 0 / 46 / 2 / 72 |
| TestSynchronized | 0 | 66 | 100% | 0 | 4 / 0 / 47 / 0 / 30 |

  耗时：每例 java 运行约 0.1 秒（转译 4–6 秒），因此缺省开启。

## 四、Rust 实现架构

沿用重写计划的 crate 布局，本计划落地其中三个 crate，外加一个分析 crate：

```
generator/
├── crates/
│   ├── classfile/     # .class + jmod（"JM" 头 + zip）解析；完整指令解码、异常表、属性（ConstantValue / Signature / 注解）
│   ├── resolve/       # 类层次查询、JVMS §5.4.3.2/5.4.3.3/5.4.6 字段 / 方法解析与选择；纯函数，无加载副作用
│   ├── closure/       # 本计划核心
│   │   ├── manifest   # closure.toml / seeds.toml / vm_intrinsics.toml（含新增 [facts] / [reflect_sinks]）
│   │   ├── absint     # 方法内抽象解释：栈/局部变量值域 {Null, IntConst, StrConst, ClassConst, Types(set), Top}
│   │   ├── cfg        # 基本块 + 常量条件剪枝（3.2）+ 异常边（3.4）
│   │   ├── xta        # 方法 / 字段 / 数组集合的跨过程传播，worklist 不动点（3.1）
│   │   ├── init       # 类初始化触发（3.3）
│   │   ├── handwritten# syn 解析手写层（3.7）
│   │   ├── seeds      # locale / JCA / 注解 / boot_init：触发条件改用精确可达集判定
│   │   ├── levels     # L0–L3 分级（3.5）
│   │   └── provenance # 每个节点的首达边：(边种类, 来源方法, 字节码偏移)
│   └── driver/        # bin `rava`：先提供 `rava closure` 子命令
```

**实际布局（2026-10-01）**：上图是立项时的划分。落地后分析器与 Rust 生成器合为单一二进制 `rava`（`rava build` 为缺省转译路径，闭包结果进程内直传发射，不再经 closure.json 文件）；`closure` crate 下为 `absint`、`manifest`、`handwritten`、`seeds`、`cache`（P6 整体结果缓存）、`cold.rs` 与 `engine/`。xta / init / levels / provenance 都在 `engine/` 内按职责拆为子模块：类型流图（`flow` / `graph` / `hub` / `gather` / `worklist` / `scc`）、调用与分派（`invoke` / `selector` / `method_lookup` / `nest`）、折叠（`fold` / `consteval` / `sysprops` / `noreturn` / `unmodeled`）、手写层（`hw*` / `vmhook`）、反射与服务（`reflect` / `services` / `sealed`）、类初始化（`class_init`）、上下文与记忆（`ctxsel` / `share` / `memo` / `setstore`）。

- 所有集合都用 `IndexMap` / `BTreeSet`，确定性由数据结构保证，不再到处加 `sorted()`。
- 性能目标：HelloWorld 闭包计算 ≤ 3s（现行 Python 转译总计 1m37s，其中发现阶段占主要部分）。

### 4.1 命令面

```
rava closure <Test.java|classes-dir|jar> --jdk 21 -o build/<test>/closure.json
rava closure … --why <Class|Class.method:desc>   # 最短 provenance 链（替代 --trace-class）
rava closure … --report md                       # 按等级 / 包 / 边种类统计（替代 jdk-scan 报告）
rava closure … --dynamic <jvm-class-load.log>    # 3.8 对照
```

**现行命令面（2026-10-01）**：`rava build <Test.java>` 一次完成闭包分析与发射（`scripts/main.py` 缺省调用）；`rava closure` 保留为诊断入口（`--why` / `--flows` / `--report`，需引擎本体，不读缓存）。
影响闭包结果的选项：`--jdk` / `--java-home` / `--image`、`--lib`、`--root` / `--seed-class` / `--main`、`--release` / `--release-bytecode`、`--locale`、`--enable-preview`、`--cold-cut`、`--flow-batch` / `--hash-seed`（集合结果与之无关，用于确定性矩阵验收）。
缓存：`--closure-cache <目录>`（`main.py` 缺省 `build/closure_cache/`）、`--closure-cache-max-mb`（缺省 4096）；键覆盖全部输入，见性能计划 §4.8。
观测：`--perf`（分段耗时与峰值内存进 `summary.perf`）、`--closure-json`（另写出 `<scratch>/closure_input/closure.json` 供动态对照与生成树对照）。
分析器不读任何自有环境变量。

### 4.2 输出 `closure.json`（Python 生成器的唯一闭包输入）

```json
{
  "classes":  { "java/io/PrintStream": { "level": "L3", "via": ["new", "java/lang/System.initPhase1:()V", 42] } },
  "methods":  { "java/io/PrintStream.println:(Ljava/lang/String;)V": { "via": ["invokevirtual", "HelloWorld.main:([Ljava/lang/String;)V", 5] } },
  "fields":   { "java/io/PrintStream.charOut": "rw" },
  "instantiated": ["…"],
  "clinit":   ["…"],
  "dispatch": { "java/lang/Object.toString:()Ljava/lang/String;": ["java/lang/String"] },
  "reflect":  { "surfaces": { "…": ["…"] }, "gaps": [] },
  "seeds":    { "locale": [], "jca": [], "annotation_enums": [], "data_bundles": [], "module_resources": [] },
  "folds_version": 2,
  "folds":    [{ "method": "…", "dead_pcs": [[65, 80]], "dead_handlers": [90], "dead_catches": [{ "start": 0, "end": 40, "handler": 60, "catch_type": "…" }], "consts": [{ "pc": 12, "kind": "getfield", "value": false, "type": "Z" }] }]
}
```

`dispatch` 与 `folds` 交给发射层使用：vtable 只发射实际存在分派目标的槽；折叠点直接生成常量。

现行顶层字段（2026-10-01）：`classes`（列表，每项 `name` / `level` / `domain` / `via`）、`methods`、`instantiated`、`clinit`、`class_init`（以参数传入的 Class 的初始化事实）、`dispatch`、`folds` / `folds_version`（含 `consts`、`null_recv`、`noreturn`、死区间与死处理器）、`indy_models`、`reflect`、`seeds`、`refs`、`missing`、`unresolved`、`summary`。
`null_recv` 只在接收者值集为空且没有任何未建模来源时导出：手写 / native / boundary / intrinsic / 伪方法 / VM 钩子 / Missing 方法的返回值、受污染数组的元素、受污染接收者或基址上的字段读取与实例调用返回值，均视为未建模来源（6ced8de5、380bcf80）。
`folds` 的格式约定见 §7.3「折叠点导出」。

## 五、可以移除的现行机制

分析器接入后，下面这些 Python 机制**整体删除**（删除前逐项向用户确认）。2026-09-30 C4 已按本表执行
（分支 `closure-c4-cleanup`；验收集 27 例生成树删除前后逐字节一致，raw-audit 仅 `jdk_literals` 2 → 1）：

| 现行机制 | 替代 | 状态 |
|---|---|---|
| 接口 CHA 遍历、迟至实现者清扫、`_drain_iface_edges` / `_pending_iface_edges` | 3.1 XTA 分派 | ✅ 已删（随 `_discover_jdk_classes_method_level`） |
| `instantiated_classes` 全局集合、`root_virtual_targets` 广播、`boundary_virtual_targets` 传播 | 3.1 调用点级集合 | ✅ 已删（同上） |
| `_enqueue_desc_types`、`_enqueue_iface_stub`、stub 通道、父类补全队列、收尾接口 drain | 3.5 分级（L1 不传递） | ✅ 已删（同上） |
| 迟至静态边补扫及三道门 | 统一规则：边界静态方法 = 手写体节点（有 impl）或存根（无 impl），一次判定 | ✅ 已删（同上） |
| `_process` 里的字符串 / 拼接 / 驼峰启发式、`_drain_nest_prefixes`、`_seed_species_family` 全量反射面 | 3.6 常量数据流 + `[reflect_sinks]` | ✅ 已删（同上；`_scan_reflect_consts` 一并删除） |
| `_impl_signature_type_refs` 正则、`provides` 名字前缀判定 | 3.7 syn 解析 | ✅ 已删：`_impl_signature_type_refs` / `_resolve_impl_ref`、`codegen/native_upcalls.py` 整体 |
| `callchain.py` 发现部分整体（`_discover_jdk_classes_method_level`、`_collect_method_refs`、`_scan_reflect_consts`） | `rava closure` + `closure.json` | ✅ 已删：callchain.py 1946 → 165 行，只留边界判定、种子全局、预检（`precheck_from_tree` / `print_precheck`） |
| scripts：`trace_callchain*.py`、`rta.py`、`analyze_callchain.py`、`dep_scan.py` 的闭包部分、`scan_jdk_boundary.py` | `rava closure --why/--report/--dynamic` | ✅ 已删：`trace_callchain*.py`（3 个）、`rta.py`、`analyze_callchain.py`、`scan_jdk_boundary.py`；`dep_scan.py` 保留（一跳依赖透视，无闭包部分，`fetch_pilot_deps.sh` 使用） |

同批删除的发现附属机制：`codegen/locale_seed.py`（→ `seeds/locale.rs`）、`jca_services.py` 的服务抽取 / 种子选择
（→ `seeds/jca.rs`，只留清单查询与放行判定）、`runtime_manifest` 的注解触发函数（→ `seeds/annotation.rs`）、
`fallback_audit` 的 cc-* 兜底点、`closure.toml [vm_boundary].whole_class`（只有 Python BFS 读）、
`main.py --closure-json`（改由 closure_input 每轮装载）；`--trace-class` 改为转交 `rava closure --why`。

**保留**：closure.toml / seeds.toml / vm_intrinsics.toml 三清单（新增 `[facts]`、`[reflect_sinks]` 两段）、JVMS 解析规则、边界截断、`vm-upcalls` 根、locale / JCA / 注解 / 模块资源种子（只是触发条件改成精确可达）。

## 六、实施阶段

状态标记：✅ 已完成（附提交）· 🔄 进行中 · ⏳ 未开始。

| 阶段 | 内容 | 验收 | 状态 |
|---|---|---|---|
| C0 | `classfile` + `resolve` crate：jmod 读取、完整解码、层次与 JVMS 解析；与 `codegen/classfile.py` 做解析结果 golden 对照 | JDK 21 / 25 的 java.base 全部类解析结果与 Python 逐字段一致 | ✅ 已完成（679cd5d3） |
| C1 | `closure` 引擎：absint + cfg + xta + init + 异常；清单读取；provenance | HelloWorld 能输出 closure.json；每个节点都有 via；`--why` 可用 | ✅ 已完成（41dc1d6d；手写层 syn 扫描随本阶段落地） |
| C1b | 值来源追踪：形参级 / 返回值级类型集（VTA 精度）替代方法级 XTA 集 | 7.1 未达标两项达标；动态对照翻译域漏覆盖 = 0 | ✅ 已完成（8596056b） |
| C1c | 手写层 `__set_` 识别 → 字段常量折叠（全写入来源）→ 容器对象按分配点区分 → 手写数组写入按调用点建模 → 流水线对象敏感 + 类型测试折叠 → 反射返回值；closure.json 导出 `folds` | 7.3 所列 7 个用例达标；含 FileIOTest 的动态对照翻译域漏覆盖 = 0 | ✅ 已完成（1e09a9a4）：第 0 步 bfcb75d7、第 1 步 32a789c6、第 2 步、第 3 步（CPA 实测否决，改为手写数组写入模型）、第 3b 步、第 4 步 97b0393c（反射目标可靠性）、耗时（CollectorsDemo 28 s → 2.7 s） |
| C1d | 边界收窄：手写只留 VM 契约层，其余按字节码翻译（见 6.1；独立计划 `docs/plans/2026-09-29-boundary-narrowing.md`） | 每个内部包边界前缀都有放行实测数据与去留结论；`[boundary]` 只剩 VM 契约类；放行包的手写代码删除清单经用户逐项确认 | 🔄 进行中：第 1 步 ✅ 79480fa9（`--release` 34 前缀实测）；手写边界规范 ✅ 1c73648d（`docs/reference/handwritten-boundary.md`）；`--release-bytecode`（模拟删除手写）复测 ✅（计划 §6.6）；精度缺口 G3 ✅ 2208ddda（`[facts.array_returns]`）、G1 ✅ 69c2e14d（静态分派转发按调用点克隆，k = 1）、G2 实测否决（类集不变、耗时 ×6，改为 G2′ 收窄 open 引入点），复测与 8 个待重测前缀的去留建议见计划 §6.7；Unsafe / VarHandle 引用写入建模到对象字段与静态字段 ✅ feaa09a2（`[facts.array_writes]` 的 `fields` / `last`，修正 CompletableFuture 分派缺失）；精度线（计划 §6.9）：G2′ ✅ 02bf00ff / c731a473，系统属性折叠 ✅ c501c499 / bfae1dee，类镜像作静态字段基址 ✅ 0333ede2，按名取类（常量前缀 + 常量表）✅ f1d00887；精度二期 ✅ a6b4c6d5（边界收窄计划 §6.10）；精度三期（`closure-prec3`：VarHandle 可达性收窄、G4–G6、ServiceLoader 服务目录事实、SystemJavaLangAccess、数组汇聚、选择子克隆、类初始化事实，见 `2026-09-30-optimization-directions.md`）9 项完成，TestCharsetForName 回归已修（98bc2e68，服务 provider 父类链按字节码分析；17fc2e7e 动态对照补方法粒度与 `cut` 标记），✅ 已合入 d8212bee（试合并 e2e 16 例 14 通过；TestStreamEncoderCharsets / TestNetworkInterface 主干同样失败，另行修复）；合入后续修 ✅（均已进 main）：VM 钩子规则 3bb4394a（手写文件中带回调边、不匹配类及超类型任何方法名的 pub fn 作为伪方法入口）、null_recv 未建模来源补全 6ced8de5 / 380bcf80（null_recv 3257 → 1570，无新增站点；数组属主的 invokevirtual 改非虚）、嵌套宿主边 f67d1de8（`engine/nest.rs`：解析到他类声明的 private 成员时双方 nest host 记为 Type 级），31 例动态对照翻译域漏覆盖全部 0；待：一次性删除全部非 VM 契约过渡手写（`c1d-final` 1e623cec，已并入 `c1d-prec`：删除后暴露缺口的修复进行中，另一会话的 VM 注入状态修复 dc9fd946 / d8a0b082 已经 b1ac5b7c 并入主干，由 `c1d-prec` 合并取舍；编译成本回到合理范围前不合入）；2026-10-02 拆为三线：**C1d-a**（`c1d-p0`）具体求值器 `engine/concrete/` 判定 OOB 消息、PTI 校验、getGenericInterfaces 三道闸门——正式 HelloWorld（无 `--cut-file`）现 ≈3091 类 / 闭包 ≈600 s，f0 切断 319 类 / 2 s，目标 ≤360 类且 ≤3 s，达标后合入 1e623cec，并把剩余 139 个 `#[jvm_boundary]`（19 文件）按准入类别登记归零、删除该宏及分析器解析；**C1d-b**（`c1d-pick`，20ef4fdc 已合入 4b73ea61）反射实参池分两通道收窄序列化闭包（DeepCopy ≤1640 类、fold_props ≥42），兼 m3 `serialVersionUID` 静态字段闭包；**native-gaps** 补 native 缺口（TestModuleLayerDefine、TestSecurityManagerContext 修复中）；regress2 ✅ 6c7eb831；FS-C2（应用类加载器）另立计划 `2026-10-02-fs-c2-app-classloader.md`。待：全量语料重测（JDK 21，Python 生成器删除的判据，见 `2026-10-01-python-generator-deletion.md`） |
| C2 | `handwritten`（syn）+ seeds + reflect 数据流 + `[facts]` / `[reflect_sinks]` 清单段 | 反射缺口清单可观测；手写层边与现行 upcalls 对照无缺失 | ✅ 已完成（分散落地，2026-09-30 核对）：syn 手写扫描（C1 / C1c 第 0 步，`closure::handwritten`；宏内调用点 8078f2b1）、补种（`closure::seeds`：注解 / locale / JCA / data bundle）、反射常量数据流（`engine/reflect.rs`，`[facts.reflect]`，反射缺口随 report 输出）、`[facts]` 各段；现行 upcalls 的 Python 机制已随 C4 删除 |
| C3 | `levels` + `dispatch` / `folds`；发射层支持 L1 不透明类型、按 `dispatch` 发射 vtable 槽、折叠点发射常量 | 生成器改动遵守原则 4（无类名字面量） | ✅ 在 Rust 生成器实施（2026-09-30 决策，见 `2026-09-30-rust-emitter.md`；Python 侧不再投入）。2026-10-01 派发 `emitter-c3`（起点 closure-prec3 98bc2e68，计划 `2026-10-01-emitter-c3.md`）：null_recv 抛 NPE、noreturn 终止控制流与死区间 0 翻译、consts 常量、class_init 事实、按 dispatch 发 vtable 槽、L1 不透明类型。前置已合入：null 虚视图抛 NPE、栈序物化待求值条目、注册钩子 turbofish（`emitter-final`，c7a9d7b4）。✅ 已合入（30df3e74，2026-10-01；单测全过、e2e 抽查 14 例全过、动态对照漏覆盖 0）：6 项全部实现——null_recv 抛 NPE、noreturn 终止控制流、运行时初始系统属性表与分析器折叠同源、class_init 事实、按 dispatch 保留 vtable 槽（手写继承覆盖所在槽族强制保留 8075b674）、L1 不透明类型（`java_class_opaque!`；分析器级别阶梯新增 L2 layout，01711d08；L1 静态类型访问 L2 属主成员时发射侧上转，96de08ea；经未建模来源进入值流的虚调用点属主升为 L2，修动态代理接口误判 null_recv，43d727a2）；不透明类不参与 S4 拆层，整类留在声明层。folds 的 Python 消费侧已合入（invoke 折叠保留调用，cad4a84c）；v2 按条目输出 `dead_catches`（Python 移植期差异 D2），Python 与 Rust `input` crate 两侧消费 ✅ b7f76452 |
| C4 | 接入：`transpile.py` 读 closure.json；删除第五节所列 Python 机制 | 全量 e2e（JDK 21）通过集合 ⊇ 冻结的 Python 基线；gap_scan precheck 无新增缺口 | 🔄 接入 ✅（`codegen/closure_input.py`，Rust 生成器由 `input` crate 消费，`rava build` 进程内直传见 `emitter-perf2`）；第五节 Python 机制删除 ✅（2026-09-30，`closure-c4-cleanup`，生成树逐字节一致）；Python 脚本并入 rava（`scripts-rava` ✅ bb0b7736：名字作用域统一、m3 编译错误归零）、逐例清理产物（`run-tests-prune` ✅ ad9e938d）；待：C1d-a / C1d-b / native-gaps / C6 后续合入后跑全量 e2e（JDK 21，基线 `2026-10-01-python-baseline-jdk21.txt` 1029 例；JDK 25 不设 Python 基线，2026-10-01 用户决定） |
| C5 | 3.8 动态对照纳入 `run_tests.py`（每个测试记录 JVM 加载集与静态闭包的差集） | 翻译域漏覆盖 = 0；静态多出的类 100% 有 provenance 说明 | ✅ 已完成：`scripts/dyn_compare.py` + JVMTI agent `scripts/dyn_agent/load_trace.c`，run_tests 缺省开（`--no-dyn` 关），明细 `logs/dyn/<test>.json`；实测见 §3.8.1 |
| C6 | 手写层回调零声明（3.7.1）：辅助函数全局传递、内部 vtable 方法映射、sigpoly `__site` 体、宏展开、常量反射、JVMS 指令异常、VM 语义规则 | 手写层 `upcalls` 声明 = 0；`error.rs` `vm-upcalls` = 0；`upcalls` 属性参数及其解析机制删除；e2e 通过且动态对照漏覆盖 = 0；不纳入不必要的类（闭包可以缩小） | 🔄 `upcalls` 声明与解析机制已删（59dbedc1，实施笔记 `2026-10-01-c4-regression-fixes.md`）；`c4-regfix` ✅ 已合入 1ba0d9aa（null_recv E0277、`hw_untyped_sites` chain 归零、桥方法并入继承槽不再误发存根）；后续 `c6-generic-closure`（4173cebd）进行中：泛型辅助 fn 的闭包形参推断、TestAnnoNestedArray null_recv 违例（分析判恒空、运行期非空，属可靠性缺陷）修复 |

各阶段独立 worktree、独立提交；C4 之前 Python 管线保持不变，Rust 分析器只做旁路输出与对照。

### 6.1 手写与字节码翻译的分工（2026-09-29 用户确认）

**默认字节码翻译，手写只留字节码表达不了的「VM 契约层」**（同 GraalVM native-image：只对 VM 层做 `@Substitute`，其余分析并编译原字节码）。

- 内部包按前缀截断的原始理由是规模（跟随内部包调用链 111 → 635 类），而这个膨胀来自 Python BFS 的过近似（第二节 M1–M9），
  不是这些代码本身必须手写。精确分析落地后，截断的收益需要重新实测。
- 手写的代价：语义要逐个复刻 JDK；对分析不可见（需要 syn 推断 `__set_` 接收者、人工声明 upcalls，
  TestRecordComponents 的 `getAccessor` 漏报即出于此）；JDK 升级要逐个核对；未手写即 panic 存根。
- 手写终态范围与准入规则见 [手写边界规范](../reference/handwritten-boundary.md)（2026-09-30 修订：方法语义以字节码为准，
  字节码无法表达才手写，性能替换不算；规模策略截断为过渡类，终态 0）。
- 分析器口径（C1c 起执行）：`[vm_boundary]` 公开包类**按方法划分**：共置手写体按精确名提供的方法、native、VM 内建
  取手写效果，其余被调用到的方法按字节码建模（发射层同样翻译）。内部包前缀边界在 C1d 之前保持截断语义。
- C1d 做法：在精确分析下把内部包边界前缀逐包放行（`sun/nio/cs`、`jdk/internal/util`、`sun/util` 等），量出每个包放行后
  闭包实际增加的类数与方法数。增量小的包改为字节码翻译，并删除对应手写代码（删除逐项经用户确认）；
  真正依赖 VM 的包（`jdk/internal/misc`、`jdk/internal/vm`、`jdk/internal/reflect` 的访问器生成）保留手写。
  最终 `closure.toml [boundary]` 从「按包前缀截断」收窄为「只列 VM 契约类」。

## 七、终态目标（量化）

| 指标 | 现状 | 终态 |
|---|---:|---:|
| HelloWorld 总类 | 集成分支 ad9e938d：260 JDK + 1 用户类（仍有 `[boundary]` 截断）；c1d-p0 去截断正式 ≈3091（C1d-a 闸门目标 ≤360） | ≤ 250 |
| HelloWorld 翻译方法体类（L3） | 1358 | ≤ 120 |
| HelloWorld 入链方法 | 20851 | ≤ 1500 |
| 无 provenance 的闭包节点 | 不可观测 | 0 |
| 翻译域内动态加载类未被覆盖 | 不可观测 | 0 |
| 名字模式类反射启发式 | 4 类 | 0 |
| 生成器中的补扫 / 门控补丁段 | 5 段 | 0 |
| 手写层人工 `upcalls` 声明（含 `error.rs` `vm-upcalls`） | 0 ✅（59dbedc1） | 0 |
| 闭包计算耗时（HelloWorld） | c1d-p0 去截断正式 ≈600 s（f0 切断 2 s） | ≤ 3s |
| 全量 e2e | 现行通过集 | 不减少 |

2026-10-01 实测（JDK 21，HelloWorld，main d842653d）：总类 250（Type 75 / Init 30 / Alloc 12 / Code 133）、翻译方法体类 131、入链方法 605、闭包计算 219 ms；无 provenance 节点 0；验收 31 例动态对照翻译域漏覆盖 0。除翻译方法体类按 §七 订正后的相对口径（≤ 134）考核外，上表各项均已达到终态。

性能、内存与工程化终态（全量语料 ≤ 10 s / ≤ 1 GB、预算降级、缓存、健壮性）另见 [`2026-09-30-closure-analyzer-performance.md`](2026-09-30-closure-analyzer-performance.md)。

上限依据：trace Internal Boundary 模式（同样有边界截断，只用单程分析）为 174 类 / 388 方法；现行机制额外承担的 VM 初始化（`vm-upcalls` 根、System 初始化链、手写层回调）预留约 40% 余量。
C1 完成后以实测替换这些上限，并写回本节（只允许下调）。

2026-09-29 订正：HelloWorld 翻译方法体类现值 130，超出 ≤ 120。增量 11 类全部来自 gap 批次 4 的放行翻译
（`Character` / `CharacterData*` 码点分表、`RandomSupport`），是「字节码优先于手写」的正当代价，不是分析过近似。
该项改按 C1c 验收的相对口径考核：≤ 运行时下限（103）× 1.3 = 134。

### 7.1 C1 实测（2026-09-29，JDK 21，HelloWorld）

| 指标 | Python 现状 | C1 实测 | 终态 | 状态 |
|---|---:|---:|---:|---|
| 总类 | 1854 | 255（type 55 / init 30 / alloc 15 / code 155） | ≤ 250 | 未达标 |
| 翻译方法体类（translate ∩ code） | 1358 | 154 | ≤ 120 | 未达标 |
| 入链方法 | 20851 | 883（bytecode 753 / 边界手写 97 / native 27 / root 6） | ≤ 1500 | 达标 |
| 无 provenance 节点 | 不可观测 | 0 | 0 | 达标 |
| 闭包耗时 | 数十秒 | ≈ 170 ms | ≤ 3s | 达标 |
| missing / unresolved | — | 0 / 0 | 0 | 达标 |

**动态对照**：`java -Xshare:off -Xlog:class+load` 中，HelloWorld 之后 JVM 加载了 90 个类，其中 84 个不在静态闭包内，已全部归因，翻译域没有漏边：

| 类别 | 数量 | 归因 |
|---|---:|---|
| `java/lang/invoke` LambdaForm / BMH / ASM / `sun/invoke/util` / `ReferencedKey*` | 72 | JVM 的 indy 链接基础设施；rava 在分析层原生建模 indy（`[indy]` 清单段），不经 MH 链接 |
| launcher 反射找 main（MainMethodFinder、PublicMethods、Class$ReflectionData 等） | 8 | 启动器路径，rava 的入口直接调 main |
| CharBuffer / HeapCharBuffer / CoderResult / Readable | 4 | 边界类 `StreamEncoder` 由 `stream_encoder_impl.rs` 手写完成编码，不走 JDK CharsetEncoder |

**未达标根因**：按方法粒度的 XTA 类型集不区分上下文。公共汇点（`String.valueOf(Object)`、`Helpers.objectToString`、
`ConcurrentHashMap.putVal` / `hashCode`、`ImmutableCollections.probe`）的形参集合被所有调用方的实参并集污染，
单个站点派发到 23–25 个目标（例：`println(String)` → `valueOf` → `ProtectionDomain.toString` → `Permissions`，
`Thread.toString` → `Thread$Constants` → `ProtectionDomain`）。
**对策（C1b）**：absint 的引用值携带来源（形参 i / 调用返回 / 字段 / new 精确类 / 数组 / catch），
引擎按「形参级 + 返回值级」维护类型集，派发只看实参值来源集合的并集。
这样得到方法内流敏感、方法间按形参区分的 VTA 精度。验收：上两项指标达标，动态对照翻译域漏覆盖仍为 0。

### 7.2 C1b 实测（2026-09-29，JDK 21，HelloWorld）

| 阶段 | 总类 | translate ∩ code | 入链方法 | 耗时 |
|---|---:|---:|---:|---:|
| C1（方法级 XTA） | 255 | 154 | 883 | ≈ 170 ms |
| 值级 VTA（形参 / 返回 / 站点节点） | 255 | 155 | 830 | — |
| 数组按分配点建模 + `Object.clone` 返回接收者（`[facts] receiver_returns`） | 221 | 134 | 619 | — |
| 手写体 syn 调用点实参类型推断（回调实参不再取值池） | 210 | 123 | 540 | — |
| 数组下标奇偶敏感 + 透传方法逐调用点接回（`requireNonNull` 等） | **200** | **115** | **466** | ≈ 45 ms |

7.1 的两项未达标指标均已达标（总类 200 ≤ 250；翻译方法体类 115 ≤ 120），missing / unresolved = 0 / 0，
两次运行输出逐字节一致。动态对照：HelloWorld 之后 JVM 加载 90 类，85 个不在静态闭包内，归因类别与 7.1 相同，翻译域漏覆盖 = 0。

C1b 引入的机制（全部通用，无类名特判）：

| 机制 | 作用 |
|---|---|
| 值级类型流节点 `P(m,i)` / `R(m)` / `S(m,off)` / `F` / `E(site, 奇偶)` | 实参按来源接到被调形参；虚调用接收者只取接收者值本身的类型集 |
| 数组分配点抽象对象 | 元素类型按分配点分离；写入按分配点实际分量类型收窄；只有目标未知（Top）的写入进入全局数组汇点 |
| 下标奇偶域（absint `V::Par`） | 键值交错数组（`Map.of` → `MapN(Object...)`）的键、值分开，`probe` 不再派发到值类型 |
| 透传摘要（`Analysis::returned_params`） | 返回值只来自形参的方法，结果逐调用点取实参，不经 `R` 汇合 |
| `[facts] receiver_returns` | native 浅拷贝（`Object.clone`）结果 = 接收者类型集 |
| 手写体 syn 调用点类型推断 | 手写回调实参按局部变量 / 分配 / 恒等转换推出具体类型，推不出才退回值池 |
| 差分传播 + lambda 重入保护 | 流边只推新增类型；绑定方法引用接收者为 lambda 自身时不再无限递归 |
| `--flows <方法 \| elem:<数组> \| @array>` | 类型流诊断：节点类型集及其来源边 |

### 7.3 C1b 冒烟：lambda / 流 / 反射测试

| 测试 | 总类 | translate ∩ code | 入链方法 | 耗时 |
|---|---:|---:|---:|---:|
| HelloWorld / TestSwitchString / PatternSwitchTest | 200 / 200 / 202 | 115 / 115 / 116 | 466 / 463 / 477 | < 60 ms |
| TestRecordComponents | 1340 | 1059 | 8286 | ≈ 20 s |
| TestStreamBasic | 1373 | 1099 | 8554 | ≈ 21 s |
| ChineseRemainderTheorem / CollectorsDemo | 1334 / 1334 | 1059 / 1059 | 8271 / 8268 | ≈ 20 s |

凡含 lambda / 流的测试都收敛到同一个约 1330 类的吸引子。根因有两处，都是上下文无关分析的固有缺陷：

1. **字段恒值未折叠**：`AbstractPipeline.evaluate` 按 `isParallel()` 分支，`parallel` 字段只在 `parallel()` 中写 true。
   程序不调 `parallel()` 时它恒为 false，但分析未折叠这个分支，于是 `evaluateParallel` → `ForEachOrderedTask` →
   `VarHandles` / ForkJoin 整条并行路径都进了闭包。
2. **公共汇点形参上下文无关**：`StringBuilder.append(Object)` → `String.valueOf(Object)` → `toString()` 的形参集合是
   全部调用方实参的并集。例：`Set12.<init>` 的重复元素异常消息 `"duplicate element: " + e0` 把所有 `Set.of` 元素灌进去，
   单个站点于是派发 163 个 `toString`（`SpeciesData` / `Formatter` / `Currency` / `SimpleDateFormat` …）；
   `hashCode` / `equals` 汇点（`HashMap.hash`、`Objects.hashCode`、`probe`）同理，各派发 106 个目标。
   反射返回值（`Method.invoke` → open(Object)）进入同一汇点，也会触发全量派发。

**对策（C1c）**，按以下顺序实施：

0. ✅ **手写层写字段识别**（前置，也修 C1b 的潜在漏边）：手写体中的 `recv.__set_<字段>(v)`（现有 259 处）
   按接收者推断出的类型定位字段，把 `v` 的类型接进字段节点，并把该字段标记为「有非字节码写入」。
   接收者类型推不出时，同名字段在所有已知声明类里都按 open 处理（安全回退）。
   现行兜底是「类自身有手写函数 → 引用字段 open」（`field_handwritten`），它盖不住跨文件写入，
   例如 `monitor.rs` 写 `Thread$FieldHolder.threadStatus`。

   **已完成（2026-09-29）**：
   - 扫描：`handwritten.rs` 按源码顺序维护带块作用域的静态类型环境。
     - 来源：impl 块 self 类型、形参注解、`let` 注解 / 初值。
     - 遮蔽：for / while / if / match / 闭包的模式绑定都会遮蔽同名变量。
   - 接收者静态类型 `SType` 有三种形态：
     - 具名类型；
     - `T::m(…)` 的返回：按 Rust 名匹配方法，且返回类型唯一；
     - `x.__get_f()` 的字段类型。
   - 关键字字段名：访问器带 `_` 后缀的（`__set_in_`）还原为 Java 名。
   - 引擎：
     - 写入值接进字段节点，读出值汇入手写方法的值池。
     - 接收者推不出时按字段名回退：写入使同名字段 open，读取让同名字段流入值池。
     - `field_handwritten` 收紧为只对边界类字段 open。
     - 顺带修正 `resolve_type`：末段模块与类型同名（`charset::Charset`）时两种前缀都尝试。
   - 实测：

     | 用例 | 类 | translate∩code | 方法 | 手写写入字段 | 按名回退 |
     |---|---|---|---|---|---|
     | HelloWorld | 200 | 115 | 457（原 466） | 16 | 0 |
     | FileIOTest | 247 | 141 | 673 | 20 | 0 |

   - 动态对照：两者翻译域漏覆盖 = 0。
     - HelloWorld 的缺失清单与 C1b 基线逐项相同。
     - FileIOTest 另缺 3 类：`Long$LongCache` 在 StringConcatFactory 引导中加载；`CharsetDecoder` 和 `UTF_8$Decoder` 走 JDK StreamDecoder 路径，而 rava 手写的 StreamDecoder 用 Charset 重载直连解码，不经过它们。三类都在手写边界之外。
1. ✅ **字段常量折叠**：每个字段维护一个写入值集，由以下几部分组成：
   - 初值：实例字段取默认值；`static final` 取 `ConstantValue`，否则取默认值。
   - 可达 `putfield` / `putstatic` 写入的抽象值，以及可达构造器 / `<clinit>` 中的写入。

   值集只含单一常量时，`getfield` / `getstatic` 在 absint 中视为该常量，死分支剪枝随之生效。
   采用乐观假设：值集长出新值时，读该字段的方法失效并重处理（单调，终止性由值集有限高度保证）。
   以下写入来源让字段直接变为 open（不折叠），判定只看字节码形状 / 手写体语法，不列类名：
   - 手写层 `__set_<字段>`（第 0 步），以及边界类 / 手写类的全部字段。
   - 以字符串常量点名字段的反射式写入：`objectFieldOffset(Class, "名")`、`findVarHandle(…, "名", …)`、
     `findStaticVarHandle`、`getDeclaredField("名")`、`AtomicXxxFieldUpdater.newUpdater(…, "名")`。
     字段所属类取同一调用的 Class 实参（ldc 类字面量），取不到时同名字段全部 open。
   - 反序列化：闭包中出现 `ObjectInputStream.readObject` 可达时，所有已实例化、实现 `Serializable`
     的类的非 static、非 transient 字段 open。

   **已完成（2026-09-29）**：
   - 常量格：`PV { Const(V), Top }`，缺席即 ⊥；只折叠 int 系 / long / null / 字符串常量，float / double 恒为 Top。
   - 三类值表，都单调增长，长出新值时读者失效重算：
     - 字段值集 `fvals`：初值 ∪ 可达写入；`static final` 仍按 `<clinit>` 唯一写入求常量。
     - 形参值集 `pvals`：各字节码调用点实参的并。根 / 手写 / VM 入口 / lambda SAM 入口固定为 Top。
     - 返回值集 `rvals`：只对唯一目标（static / special / private / final 方法 / final 类）的字节码方法使用。
   - 「不返回」按乐观假设处理：被调方法还没有任何可达返回点时，按不返回处理，调用点之后不可达。
     - void `return` 同样记作返回点。漏记会让乐观阶段在几十个方法处提前收敛，实测 TestStreamBasic 因此多出 240 类。
     - 队列排空时关掉乐观假设，把得到过「不返回」答复的方法按「值未知」重算。
     - 所以导出的不可达代码只从跳转、switch、return、athrow 之后开始（见下文 v1 规则 7）。
   - 字段 open 的判定：
     - 边界类 / 根域字段，或有手写访问器的字段（`__get_` / `__set_` 所在类，例如 `System.out` 由 `System::out()` 提供）。
     - 手写 `__set_`（第 0 步）。
     - 反射点名写入，按字节码形状判定：同一调用里有字符串常量实参，且形参含 `Class` 或接收者是 `Class`。
     - 清单 `vm_intrinsics.toml [facts.field_writes]`：
       - `enumerators`：返回字段句柄数组，接收者类的全部字段 open，推不出时全部字段 open；
       - `deserializers`：可达即非 static、非 transient 字段全部 open。
   - 实测（JDK 21；括号内为第 0 步提交 bfcb75d 的同口径数据）：

     | 用例 | 总类 | translate∩code | 方法 | 折叠方法 / 常量点 | 耗时 |
     |---|---:|---:|---:|---:|---:|
     | HelloWorld | 186（200） | 105（115） | 419（457） | 49 / 57 | 55 ms |
     | TestSwitchString | 186（200） | 105（115） | 411（454） | 48 / 56 | 58 ms |
     | PatternSwitchTest | 189（202） | 107（116） | 452（468） | 46 / 45 | 65 ms |
     | FileIOTest | 233（247） | 131（141） | 629（673） | 66 / 64 | 84 ms |
     | ChineseRemainderTheorem | 232（1334） | 138（1059） | 536（8268） | 59 / 74 | 73 ms |
     | TestStreamBasic | 361（1373） | 249（1099） | 1354（8551） | 111 / 176 | 0.6 s |
     | TestRecordComponents | 537（1340） | 384（1059） | 2787（8283） | 383 / 547 | 2.7 s |
     | CollectorsDemo | 1338（1334） | 1061（1059） | 8391（8265） | 283 / 191 | 36 s |

   - 全部用例 folds 自检违约 = 0，两次运行输出逐字节相同。
   - TestStreamBasic 的并行路径整体剪掉：`evaluate` 的 `isParallel()` 折叠为 false，ForkJoin / `VarHandles` 不再入闭包。
     这部分约 1000 类，所以第 2、3 步按原计划全量实施。
   - CollectorsDemo 仍在吸引子中：`toMap` 重复键消息里的 `String.format` 让 `Locale` 常量表被带进来，
     HashMap 键集合在全局汇合，`compareComparables` 于是派发到 `BigDecimal.compareTo`，一路引到 `BigInteger` / ForkJoin。
     这属于第 2 步（容器对象敏感）要解决的问题。它的 36 s 是在吸引子规模上多轮失效重算的代价，靠第 2 步收缩规模消除。
   - 动态对照：8 个用例在翻译域的漏覆盖均为 0。「运行时加载了、第 0 步有、现在没有」的类只有以下几种，
     都在翻译域之外或属于第 0 步的过近似：
     - JVM 的 indy / lambda 引导（`java/lang/invoke`、`sun/invoke`、asm），rava 用自己的 lambda 模型替代；
     - launcher 反射调用 main（`MethodHandleAccessorFactory` 等）；
     - 手写边界 `sun/nio/cs` 的编码路径（`CharBuffer` / `CoderResult` / `Readable`）；
     - VM native `ObjectMethods.bootstrap` 内部的流 / `subList`。
   - 基线订正：第 0 步的 HelloWorld 200 / 115 / 457 缺了 `println`。原因是 `System.out` 由手写访问器提供，字段节点类型集为空。
     本步已按「手写访问器字段 = open」修正，下文验收中的 HelloWorld 基线改为本步数据 186 / 105 / 419。
2. ✅ **容器对象按分配点区分**：数组分配点模型扩展到「容器形态类」的对象，按 `new` 的位置分开追踪其元素字段的类型。
   容器形态按字节码判定：类持有 `Object[]` / 引用数组字段，或持有引用字段且其方法对该字段值做虚调用。
   只做一层对象敏感：`HashMap` 的 `table` / `Node.key`、`ImmutableCollections` 的元素数组等按分配点分离。
   `HashMap.put(K,V)` 这类大方法无法靠调用点克隆切断，由对象敏感解决。
   **阶段性落地（2026-09-29，验收未达，由第 3、4 步收尾）**：
   - 对象敏感：
     - 方法节点键为 `(MemberRef, ctx)`，`NOCTX` 为上下文无关版本；容器形态按字节码判定：
       类型变量泛型签名、`Object[]` / 引用数组字段、容器类型字段。
     - 抽象对象按分配点命名（`类@方法:偏移`），分配链深度 `HEAP_DEPTH = 2`。
     - 字段节点三种：`O(对象, 字段)` 对象级、`U(字段)` 接收者未知的写入、`F(字段)` 字段总汇。
     - folds 按方法合并各克隆后导出，格式仍为 v1。
   - 性能架构（CollectorsDemo 274 s → 39 s，TestStreamBasic 2.6 s → 0.5 s，修漏报之前的口径）：
     - 站点级监听：值集节点登记读取它的 `(方法, 偏移)`，节点增长只重跑这些站点的事件，不再整方法重处理；
     - 分派备忘：字节码方法每个 `(偏移, 接收者类)` 只接一次边，被调方透传摘要变化时清空调用方备忘；
     - 开放接收者站点集：类型层级增长时只重排登记过的站点；
     - `IdSet`（有序小 Vec）替代 B 树集合，FxHash 替代 SipHash；
     - 接收者按被调方法声明类过滤，字段读写按字段所属类过滤。
   - 两个漏报修复：
     - `hw_member` 精确匹配分支漏拷手写体的字段访问（`fields`）。
     - **`[vm_boundary]` 按方法划分**（§6.1）：此前 `Class` 等类的全部方法都按手写处理，没有同名手写 fn 的方法
       （如 `Class.getRecordComponents`）效果为空，其体内对 native `getRecordComponents0` 的调用、以及后者经手写体写入的
       `RecordComponent.accessor` 从分析中消失。结果 `getAccessor()` 被折叠为常量 null，`Method.invoke` 漏出闭包。
       现在未手写提供的方法按字节码建模，与发射层实际翻译一致。
   - 实测（JDK 21，全部修复后）：

     | 用例 | 总类 | translate∩code | 方法 | 上下文 | 抽象对象 | 耗时 |
     |---|---:|---:|---:|---:|---:|---:|
     | HelloWorld | 212 | 119 | 533 | 634 | 30 | 83 ms |
     | TestSwitchString | 212 | 119 | 526 | 615 | 29 | 64 ms |
     | PatternSwitchTest | 214 | 120 | 539 | 628 | 29 | 69 ms |
     | FileIOTest | 256 | 143 | 715 | 824 | 32 | 87 ms |
     | ChineseRemainderTheorem | 266 | 159 | 702 | 792 | 30 | 80 ms |
     | TestStreamBasic | 381 | 258 | 1455 | 2573 | 152 | 0.5 s |
     | TestRecordComponents | 557 | 389 | 2928 | 36409 | 1114 | 381 s |
     | CollectorsDemo | — | — | — | — | — | > 900 s（超时） |

   - 动态对照：前 7 例相对第 1 步基线的新增漏报 = 0（TestRecordComponents 的 `Method.invoke` 漏报已消除）；
     folds 自检违约 = 0；两次运行输出一致。
   - 基线再订正：第 1 步的 HelloWorld 186 / 105 / 419 没有建模 `Class` 未手写方法的字节码（`Throwable.<clinit>` →
     `Class.desiredAssertionStatus`、`Random.<clinit>` → `getDeclaredField` 等运行时真实路径），偏小且不安全。
     验收基线改为本步数据 212 / 119 / 533。
   - 未达标原因：`Method.invoke` 入闭包后返回 open(Object)，与 `String.format` 实参（所有调用方的并集）一起进入公共汇点，
     派发面扩到反射 / 格式化 / 数值整条链。由第 3 步（CPA，按实参对象克隆，拆开可变参数数组）与
     第 4 步（反射返回值按反射目标给出类型）解决。

3. ✅ **手写层数组写入按调用点建模**（原计划的 CPA 经实测否决，见下）。
   - **CPA 实测否决**（2026-09-29，已实现后移除）：对小方法按实参抽象值逐值克隆（每形参值集 ≤ 4、组合 ≤ 8、字节码 ≤ 1200）。
     8 例中 7 例总类不变；CollectorsDemo 只少 3 个类，上下文 17457 → 42485，耗时 56 s → 218 s。
     污染源不在「小方法把不同调用方的实参混在一起」，而在下面两处，CPA 都切不开。
   - **根因一：手写方法的数组写入全局汇合**。`System.arraycopy`、`Unsafe` 引用写入等手写体原来共用一个元素汇点：
     任一调用方的源数组元素流进所有调用方的目标数组，`Object[]` 元素集在全局汇合。修正：
     - 手写方法的每个调用点 `(调用方, 偏移, 被调方)` 单独建节点：`A(site, i)` 为第 i 个实参（含接收者），
       `W(site, j)` 为写入第 j 个实参数组的元素来源；新数组到达 `A` 时按写入规格连边，元素按目标数组的分量类型过滤。
     - 写入规格由清单声明：`vm_intrinsics.toml [facts.array_writes]`（`dst` / `values` / `elements` / `produced`，
       下标按描述符形参，不含接收者）。`arraycopy` 只写 `dst`、来源为源数组元素；`Unsafe.put*/getAndSet/CAS` 的引用变体只写 `dst`；
       `getReference*`、`MethodHandle.invokeExact` 等声明为不写。
     - 未声明的手写方法：手写体（syn 扫描，传递闭包）含数组操作（`JArray`、`__view_into`、`try_cast_array`、`array_store*`）
       时按保守规则——非接收者引用形参均可被写，来源为其余形参、各实参数组的元素和手写体产出值；不含数组操作即不写。
       `equals(Object)` 等从此不再被当作写入者。
     - 手写体的值池拆成两个：`POOL`（形参 + 产出）供回调实参，`PROD`（手写体分配 / 构造 / 字段读取 / 回调返回）作为产出。
   - 实测（JDK 21；总类 / translate∩code / 方法 / 上下文 / 抽象对象 / 耗时）：

     | 用例 | 总类 | translate∩code | 方法 | 上下文 | 抽象对象 | 耗时 |
     |---|---:|---:|---:|---:|---:|---:|
     | HelloWorld | 223 | 130 | 606 | 700 | 30 | 91 ms |
     | TestSwitchString | 223 | 130 | 599 | 683 | 29 | 74 ms |
     | PatternSwitchTest | 225 | 131 | 612 | 696 | 29 | 78 ms |
     | FileIOTest | 265 | 153 | 778 | 871 | 32 | 97 ms |
     | ChineseRemainderTheorem | 267 | 160 | 715 | 800 | 30 | 84 ms |
     | TestStreamBasic | 381 | 259 | 1452 | 2071 | 139 | 0.2 s |
     | TestRecordComponents | 557 | 390 | 2931 | 5091 | 277 | 1.0 s |
     | CollectorsDemo | 1206 | 948 | 7616 | 17457 | 919 | 45 s |

   - 动态对照：相对第 1 步基线（`.base.json`）新增漏报只有 CollectorsDemo 的 `DirectMethodHandleAccessor`，
     它来自 launcher 反射调用 main（rava 直接调用 main，不走该路径），第 1 步基线是经一条不可行的
     `AccessibleObject.<clinit>` → `Currency` → `SimpleDateFormat` → 反序列化 → `Method.invoke` 链偶然覆盖的。翻译域漏覆盖 = 0。
     folds 自检违约全部为 0；两次运行输出逐字节相同。
   - 基线再订正：HelloWorld 212 / 119 / 533 → 223 / 130 / 606，增量全部来自 gap 批次 4（BCP 47 语言标签放行翻译）：
     `Character` / `CharacterData*` 码点分表 10 类（`CharacterData.of(int)` 按码点选表，静态不可判定）与 `RandomSupport`。

3b. ✅ **流水线 / 捕获闭包对象敏感 + 类型收窄**（CollectorsDemo 的剩余膨胀）。
   - 根因二：`ReduceOps$3ReducingSink@makeSink` 只有一个抽象对象。`ReduceOps$3` 的捕获字段不是类型变量类型，
     不满足容器形态判定，`makeSink` 不按 `ReduceOp` 实例克隆；所有 spliterator 的 `forEachRemaining` / `tryAdvance`
     都喂给同一个 `accept`，流元素集膨胀到 1372 类。
   - 已实施：
     - **容器形态扩到函数式接口字段**：字段类型是函数式接口（JLS §9.8：去掉 `Object` 公有方法与被 default 覆盖者后恰一个抽象方法）
       的类按容器处理——sink、`ReduceOp`、`Collector` 实现、捕获型匿名类的方法按接收者对象克隆。
     - **新鲜工厂调用点上下文**：有引用形参、返回值来自本方法分配的容器 / 引用数组（或另一个新鲜工厂）的静态字节码方法，
       在无上下文的调用方里按调用点克隆（`@方法:偏移` 作堆上下文链首，不进入值集），`Collectors.toList()` 等每个调用点各得一个对象。
     - **`clone` 按调用点接回接收者**：`returns_receiver` 的方法在调用点把接收者来源直接接到结果，不经上下文无关的形参节点汇合。
     - **零长数组**：常量长度 0 的 `newarray` / `anewarray` 分配点暂存元素写入；同一分配点出现非 0 长度时补回。
     - **checkcast 成为独立来源**：类目标的 checkcast 结果以本偏移为来源，引擎按目标类型收窄输入；
       原先转换类型只记在值的静态类型上，控制流汇合（类型不同）即丢失。
     - **open 交集**：open(类 C) 经接口 I 过滤保留 open(C)（展开时再与接收者类型求交），C 为 final 且不实现 I 时为空；
       原先放宽为 open(I)，`open(LambdaForm$Name)` 经 `Serializable` 过滤即展开到 819 类。
     - **手写分配的容器取抽象对象**：手写体的分配 / 构造 / `<init>` 回调按伪偏移（自 `u32::MAX` 递减）建抽象对象，
       字段写入不再落到「未知接收者」节点再流向该类全部对象。
     - 诊断：`--flows` 增加 `@path:<节点>|[open:]<类>`（逆向最短来源链）、`@merge:N`、`@callers:<方法>`、`@m:<序号>`、
       `@opens:<类>`（open 引入点）、`@openstat`（各 open 类型的节点数 × 展开类数排名）、`@array`。
   - 实测（JDK 21；总类 / translate∩code / 方法 / 上下文 / 抽象对象 / 耗时）：

     | 用例 | 总类 | translate∩code | 方法 | 上下文 | 抽象对象 | 耗时 |
     |---|---:|---:|---:|---:|---:|---:|
     | HelloWorld | 223 | 130 | 606 | 712 | 35 | 91 ms |
     | TestSwitchString | 223 | 130 | 599 | 695 | 34 | 75 ms |
     | PatternSwitchTest | 225 | 131 | 612 | 708 | 34 | 79 ms |
     | FileIOTest | 265 | 153 | 778 | 883 | 37 | 94 ms |
     | ChineseRemainderTheorem | 267 | 160 | 715 | 814 | 37 | 83 ms |
     | TestStreamBasic | 351 | 242 | 1312 | 1992 | 134 | 0.2 s |
     | TestRecordComponents | 557 | 390 | 2927 | 5668 | 332 | 0.9 s |
     | CollectorsDemo | 1196 | 936 | 7498 | 19332 | 1076 | 23 s |

   - 动态对照：相对 `.base.json` 各例漏报数均下降，新增漏报仍只有 launcher 的 `DirectMethodHandleAccessor`；翻译域漏覆盖 = 0。
     folds 自检违约全部为 0，fold_check 全部通过；两次运行输出逐字节相同。
   - 否决的方向（实测）：堆上下文深度 2 → 3，CollectorsDemo 不变，TestStreamBasic 242 → 259。
     同一个源列表上的多次 `collect` 共用同一个 `ReferencePipeline$Head` 对象，按接收者对象的敏感性切不开不同 Collector 的流水线；
     要切开须对 `collect` / `makeRef` 按实参敏感，即第 3 步已否决的 CPA 代价。
   - 剩余膨胀的归因（诊断实验，结果不健全，只用于定位）：
     - 丢弃全部 open 值：CollectorsDemo 936 → 522，TestRecordComponents 390 → 162（低于运行时下限，说明 open 承载了真实路径）；
       只丢弃 open(Object)：936 → 908、390 → 353；只丢弃手写方法返回的 open(Object)：936 → 913。
       主要来源是手写 / VM 层产出的 open(声明类型)——边界手写方法按擦除后的声明类型返回 open，
       经 `Object` 形参的 `toString` / `equals` / `hashCode` 分派扩散到全部已实例化类。
     - TestRecordComponents 的 136 个超出类分散在异常路径（`printStackTrace`、`parseInt` 异常）、反射访问器的备选分支
       （`MethodAccessorGenerator`、`NativeMethodAccessorImpl`）等静态可行、运行时未走的路径，没有单一污染点。
     - `Collectors.duplicateKeyException` → `String.format` → `Formatter` / `java.time` / regex 是可行路径，不属膨胀。
   - 两例的 ×1.3 由第 4 步（反射返回值）与 C1d（边界手写改为字节码翻译，手写层的 open 产出随之变为精确数据流）承接。

4. ✅ **反射目标的可靠性**（原题「反射返回值」，实测后改向）。
   - 改向原因：诊断实验把反射调用的返回值改为精确类型（丢弃 `Method.invoke` 的 open(Object) 返回），
     8 例 translate∩code 全部不变。第 3b 步归因里的膨胀来自边界手写的 open 产出，不在反射返回值上；
     反而反射**目标**缺少建模：`Method.invoke` / `Constructor.newInstance` / MH 按名查找的被调方法不入闭包，是漏报来源。
   - 模型（清单 `vm_intrinsics.toml [facts.reflect]`，生成器无类名）：
     - 类镜像：`Class` 值按所指类区分为抽象对象 `java/lang/Class#<类>`（类型仍是 `Class`，不做上下文克隆与字段拆分）。
       来源是类字面量（ldc）与 `mirror_of_receiver`（`Object.getClass`，逐调用点把接收者类集换成镜像集）。
     - 成员枚举（`methods` / `constructors` / `record_accessors`，手写 native）：接收者镜像所指类的对应成员成为反射对象；
       接收者推不出所指类（open(Class) 或非字面量来源）记为反射缺口，导出 `closure.json reflect.gaps`，不做模糊扩展。
     - 按名查找（`method_lookups`：`Class.getMethod` / `getDeclaredMethod`、`Lookup.findStatic` / `findVirtual` /
       `findSpecial` / `resolveOrFail` / `resolveOrNull`）：同一调用里的类字面量 + 方法名常量点名反射目标。
       字段查找（`objectFieldOffset(Class, String)` 等）不在此列——初版按「含 Class 形参 + 字符串常量」的形状登记，
       把 `Class.annotationData` / `reflectionData` 等同名字段误当成方法，CollectorsDemo 多出 19 个类。
     - 反射调用（`method_invokers` / `constructor_invokers`：`NativeAccessor.invoke0` / `newInstance0`、
       `MethodHandle.invokeExact` / `invokeBasic`）可达后，成员入链：用户类被枚举即全部方法 / 构造器入链；
       非用户类只有按名查找点到的方法入链，构造器不入链（非用户类的反射构造只经清单补种，C2）。
       入链成员形参按声明类型 open（同 VM 入口），构造器同时实例化其类。
   - 实测（2026-09-30，JDK 21）：

     | 用例 | 总类 | translate∩code | 方法 | 上下文 | 对象 | 耗时 |
     |---|---:|---:|---:|---:|---:|---:|
     | HelloWorld | 223 | 130 | 606 | 712 | 35 | 87 ms |
     | TestSwitchString | 223 | 130 | 599 | 695 | 34 | 73 ms |
     | PatternSwitchTest | 225 | 131 | 612 | 708 | 34 | 78 ms |
     | FileIOTest | 265 | 153 | 778 | 883 | 37 | 94 ms |
     | ChineseRemainderTheorem | 267 | 160 | 715 | 814 | 37 | 84 ms |
     | TestStreamBasic | 351 | 242 | 1312 | 1992 | 134 | 0.2 s |
     | TestRecordComponents | 557 | 391 | 2962 | 5703 | 332 | 1.1 s |
     | CollectorsDemo | 1208 | 947 | 7588 | 19536 | 1082 | 28 s |

   - 动态对照：7 例漏报数相对 `.base.json` 均下降 2–5 个。新增漏报只有 CollectorsDemo 的 `DirectMethodHandleAccessor`：
     JVM 在 `EnumMap` 构造时经 `Class.getEnumConstantsShared` 反射调用 `values()` 加载它；rava 该方法由手写按运行时常量目录重建
     （`class_impl.rs`），不走 `Method.invoke`，运行时不执行，不属翻译域漏覆盖。基线里它来自 `AccessibleObject.<clinit>` 经
     `Currency` → `SimpleDateFormat` → `ObjectInputStream` 的不可行路径，本步收窄后不再出现。
     folds 自检违约 = 0，fold_check 全部通过；两次运行输出一致。
   - 反射缺口：TestRecordComponents 1 个（`getDeclaredMethods0 <- Class`），CollectorsDemo 3 个
     （`getDeclaredMethods0` / `getDeclaredConstructors0` 的 open(Class) 接收者）；均来自 JDK 内部按非字面量 Class 的反射，由 C2 补种核对。
   - CollectorsDemo 936 → 947：新增的 11 个类都是 MH 基础设施按名查找的真实目标（`MethodHandleImpl.loop` / `tryFinally` /
     `tableSwitch`、`BoundMethodHandle.copyWith*` 等），查找点所在方法本身已在闭包内，按可靠性口径保留。
     这些查找点是否运行时可达，由 C1d 对 MH / LambdaForm（VM 契约第 3 类，运行模型替换）的建模决定。
   - 两例的 ×1.3 与 CollectorsDemo 的耗时（28 s，验收要求 ≤ 3 s）仍未达标：×1.3 由 C1d 承接；
     耗时在 C1d 放行实测前单独处理（C1d 每包 × 8 例实测，单例 30 s 不可接受）。

**耗时（2026-09-30 完成）**：CollectorsDemo 28 s → 2.7 s，全部是求解算法改进，不设时间预算、不降精度；
CollectorsDemo 每一步都与真不动点基线（逐站点全量重跑验证过的输出）逐项比对一致。

1. 求解正确性（先于提速）：
   - 第 4 步的增量求解不是真不动点：字节码调用点已接入枢纽后重跑时提前返回，枢纽上逐调用点派发的 lambda
     接收者增长不再处理，漏掉派发目标。TestStreamBasic 的 `AbstractCollection.toString` 因此取不到列表元素。
   - 改为 lambda 调用各自登记为读者单元（`LCall`）：只读自己的输入值集，增长时只处理新增接收者。
2. 精度（同时减少计算量）：
   - 逃逸模型：open 值只代表已逃逸的对象。抽象对象到达逃逸汇点（手写体 / native 的值池、VM 回调返回值、未知数组）
     后，才与字段的未知接收者视图相连；未逃逸对象只经字节码可见的引用访问。
   - 清单 `[facts.memory_reads]`：`Unsafe.getReference*` / `getAndSetReference` / `compareAndExchangeReference`
     的返回值按调用点读自形参所指对象（数组元素 / 引用实例字段），替代手写返回的 open(返回类型)。
   - `MethodHandle.invoke` / `linkTo*` 与 `invokeExact` / `invokeBasic` 同口径登记为调用器。
   - 手写体 `let x = T::new*(…)` 绑定的不可变局部变量上的调用，接收者只取该方法新建的 `T`（`fresh`）；
     手写体对 `self` 字段的访问按接收者对象逐个接入。
3. 求解结构：
   - 派发枢纽：同一调用成员在同一接收者集合上的派发共用一个枢纽。调用点实参汇入 `HP`、目标返回值汇入 `HR`，
     边数从「调用点 × 接收者」降为「调用点 + 接收者」。精确集合增长时换接新枢纽，以原枢纽为父，只派发增量。
   - G 按类型惰性建子集索引：open 展开与 catch 存活判定与 |G| 无关。
     open 展开过的方法 / 站点按 (open 类型, 接收者上界) 索引，G 增长时只重跑受影响者。
   - 站点级去重（`dispatched` / `hub_linked` / `recv_done` / `lambda_done`）：同一分析结果下重跑只处理新增接收者。
     字段站点与值无关的部分（登记类 / 值集 / 手写访问器）只接一次。
4. 重分析：
   - 调用方对被调方分析的依赖只有透传摘要，摘要变化才重处理调用方（返回常量经 rdeps、「尚无返回」经 never 各自失效）。
   - 重分析后只执行与上次已执行分析不同的偏移上的事件，并只清这些偏移的去重记录。
5. 常数因子：
   - `IdSet` 超过 64 元素附位图：差集 / 并入与大集合规模无关，两侧都有位图时按字求差。
   - 类型集收窄按过滤类型缓存子类型判定行。
   - 流传播时边表借出不克隆，同一过滤类型只收窄一次，Object 过滤直接推增量。
   - release 开 `lto = "fat"`、`codegen-units = 1`。

实测（2026-09-30，JDK 21，release）：

| 用例 | 总类 | translate∩code | 方法 | 上下文 | 对象 | 耗时 |
|---|---:|---:|---:|---:|---:|---:|
| HelloWorld | 223 | 130 | 605 | 807 | 50 | 89 ms |
| TestSwitchString | 223 | 130 | 598 | 790 | 49 | 71 ms |
| PatternSwitchTest | 225 | 131 | 611 | 803 | 49 | 76 ms |
| FileIOTest | 265 | 153 | 777 | 978 | 52 | 84 ms |
| ChineseRemainderTheorem | 267 | 161 | 716 | 911 | 52 | 80 ms |
| TestStreamBasic | 379 | 257 | 1442 | 2335 | 177 | 0.14 s |
| TestRecordComponents | 555 | 389 | 2913 | 5272 | 337 | 0.38 s |
| CollectorsDemo | 1204 | 943 | 7558 | 17904 | 1064 | 2.7 s |

- 对第 4 步：
  - TestStreamBasic +28 类，是第 1 项漏传修正后的真不动点。列表元素流到 `toString` 后带上 open(Object)，
    来源是 TimSort 的临时数组：`Array.newArray`（native）返回 open(Object)，从中读出的元素也是 open(Object)。
    `String.valueOf` 因此派发到 G 中全部 `toString`，引出 `Thread` / `ProtectionDomain` / `Permissions` 一族。
    后续把 `Array.newArray` 按调用点建模为数组分配（元素类型取 Class 实参）即可收窄；这个 native 属 VM 契约第 1 类。
  - TestRecordComponents −2（`HashMap$KeySet` / `HashMap$KeyIterator`）、CollectorsDemo −4：第 2 项精度改进的净效果。
- 动态对照：7 例漏报均不增；唯一的新增漏报仍是上文说明过的 `DirectMethodHandleAccessor`（边界域）。
  folds 自检违约 = 0，fold_check 8 例 1453 条全部通过，两次运行输出一致。

第 1 步完成后单独测一次 TestStreamBasic，量出并行流路径（ForkJoin / VarHandles）占多少类，再定第 2、3 步做多深。

**折叠点导出（C3 / C4 的衔接，格式 v2 已定）**：Rust 闭包剪掉的分支，Python 生成器必须同样不翻译，
否则生成代码会引用闭包外的类，编译失败。closure.json 为每个存在不可达代码或常量折叠点的方法导出：

```json
"folds_version": 2,
"folds": [{
  "method": "java/util/stream/AbstractPipeline.evaluate:(Ljava/util/stream/TerminalOp;)Ljava/lang/Object;",
  "dead_pcs": [[65, 80]],
  "dead_handlers": [90],
  "dead_catches": [{"start": 0, "end": 40, "handler": 60, "catch_type": "java/lang/InstantiationException"}],
  "consts": [{"pc": 12, "kind": "getfield", "value": false, "type": "Z"}]
}]
```

1. **`dead_pcs`**：不可达指令的半开区间 `[start, end)`。
   - 两端都落在指令起点上。
   - 按 start 排序，互不重叠，相邻区间合并。
   - 条件跳转被常量裁掉的一侧就体现在这里。Python 在 CFG 结构化之前把只剩一个活后继的条件跳转改写成 goto 或直通，不另设分支字段。
2. **`dead_handlers`**：起点不可达的异常处理器 pc，Python 据此删除对应的异常表项。以下两种情况 handler 都会进这里：
   - try 区间内没有可达指令；
   - catch 类型在闭包里从未实例化（没有子类型被 new，也没有被 VM / 手写层抛出）。
   不输出「try 区间部分删除，却保留引用它的 handler」的组合。
   **`dead_catches`**：逐项删除的异常表项 `{start, end, handler, catch_type}`（按异常表原值逐字段匹配）。
   catch 类型不在闭包类集合内时输出——多 catch（`catch (A | B e)`）共用处理器，A 在闭包内、B 不在时，
   只删 B 的表项，处理器与 A 的表项保留，生成代码不再引用 B。catch-any 不输出；处理器的全部表项都死时
   处理器并入 `dead_handlers`，不再逐项列出。
3. **`consts`**：被折叠成常量的读取点。`kind` 取 `getfield` / `getstatic` / `invoke` 三种，栈效应如下：
   - `getfield`：替换后弹出 receiver。
   - `invoke`：**调用照常执行**，只丢弃返回值、改压常量。折叠的是返回值，不是调用：被调方的副作用
     （如 CompletableFuture.postFire 内的 postComplete）必须保留；被调方仍在闭包内（分析器照常接边）。
   - getfield 的 receiver 表达式副作用由 Python 保留求值、丢弃结果，JSON 只给 pc。
4. **`value` 与 `type`**：`type` 是 JVM 描述符（`Z/B/C/S/I/J/F/D`、`Ljava/lang/String;`）。
   - null 写成 `"value": null`，`type` 给声明类型。
   - `J` / `D` 的值用字符串编码（`"9007199254740993"`），避免 JSON 数值丢精度。
   - `Z` 用 JSON 布尔值，其余整型用 JSON 整数。
5. **确定性**：`folds` 按 method 排序，`consts` 按 pc 排序，同输入逐字节相同。
6. **版本**：顶层 `folds_version` 当前为 2（v2 = v1 + `dead_catches`）。消费侧（Python `closure_folds.py`、Rust `input` crate）只认当前版本，遇到其他版本忽略 folds、按原样翻译、不报错。
7. **不可达的起点**：活指令的顺序后继落入 `dead_pcs`，只允许出现在跳转、switch、return、athrow 之后。
   例如调用一个永不返回的方法，其后的代码不标死。Rust 侧导出前自检，Python 侧违反即报 `FoldError`。
8. **`invoke` 的范围**：`kind: "invoke"` 只出现在 invokevirtual / invokespecial / invokestatic / invokeinterface 上，
   invokedynamic 不折叠。
9. **附加字段（v2 内，忽略即保持原行为）**：`null_recv`（接收者恒为 null 的活虚调用点）；`noreturn_calls`（定论不返回的活调用点）与
   `noreturn_dead_pcs`（以这些调用为控制流终点时另外不可达的区间，与 `dead_pcs` 不相交）。消费 `noreturn_*` 时在调用后终止控制流并删去该区间，
   规则 7 相应放宽为「死区只从跳转 / switch / return / athrow 或 noreturn 调用之后开始」。详见边界收窄计划 §6.11。

原有的 `dead_branches` 输出整体由 `folds` 取代。Python 只消费这份数据，不另写判定。

**消费侧已就绪**（`codegen/closure_folds.py`，closure_input 每轮装载 closure.json）：在 classfile 解码后、
VM 常量守卫剪除之前单点规范化指令序列（调用链 BFS 与生成代码共用）。
- 条件跳转恒直通改写为 `pop`，恒跳转改写为 `pop` + `goto`；switch 的死目标改指向活目标，只剩一个活目标时改写为 `pop` + `goto`。偏移沿用原指令字节。
- getstatic 折叠为装载指令；getfield 折叠为合成指令 `fold_const`，先弹出 receiver（有副作用的保留求值）再压入常量；invoke 保留原调用指令（按调用指令识别引用的消费方——导入收集、super 调用的 `_base` 函数收集等——照常看到），在 `Instr.fold` 挂装载指令，`sim_instr` 照常翻译调用、以 `let _ =` 丢弃结果，再压入常量。
- 异常表：删掉 dead_handlers 的表项、dead_catches 列出的表项，以及受保护区间已全死的表项；其余区间端点收拢到活指令起点。Rust 侧 `input/src/norm.rs` `apply_fold` 同一规则。
- 违约输入直接报 `FoldError`：端点不在指令起点、活的非跳转指令顺序落入死区、const 的 kind 与指令不符、Z 不是布尔值、handler 在死区但未列入 dead_handlers。
- 单元测试 `tests/unit/test_closure_folds.py`（21 项）；手工 closure.json 的端到端探针（静态字段 / 实例字段 / 调用三类折叠 + 死分支）生成体符合预期。

验收用 `compare_trees.sh`：
- 不带 folds 时，生成树逐字节不变；
- 带 folds 时，只有预期的方法体变化，并且生成代码引用的类都在闭包之内。

**C1c 验收**：
- HelloWorld、TestSwitchString、PatternSwitchTest、TestRecordComponents、TestStreamBasic、CollectorsDemo、
  ChineseRemainderTheorem 7 个用例：translate ∩ code ≤ 运行时下限 × 1.3，单测试耗时 ≤ 3s。
  运行时下限 = JVM 实际加载且闭包判为翻译域方法体类的类数（`-Xlog:class+load` 对照）。
- HelloWorld 指标不回退（第 3 步再订正后 223 / 130 / 606，见上文）。
- 目标重估（2026-09-29）：原「总类 ≤ 400、translate ∩ code ≤ 250」对 TestRecordComponents / CollectorsDemo 不可达——
  两例的运行时下限分别为 254 / 300，健全的闭包不可能低于它。改为相对下限的比值：

  | 用例 | 运行时下限 | 现值 | 比值 |
  |---|---:|---:|---:|
  | HelloWorld | 103 | 130 | 1.26 |
  | TestSwitchString | 101 | 130 | 1.29 |
  | PatternSwitchTest | 104 | 131 | 1.26 |
  | FileIOTest | 126 | 153 | 1.21 |
  | ChineseRemainderTheorem | 131 | 160 | 1.22 |
  | TestStreamBasic | 199 | 242 | 1.22 |
  | TestRecordComponents | 254 | 391 | 1.54 |
  | CollectorsDemo | 300 | 947 | 3.16 |

  （现值为第 4 步后。）未达标两例由 C1d 解决，归因见第 3b、4 步。
- 上述 7 个用例以及 FileIOTest（手写层写字段密集，检验写入来源是否收全）的动态对照，翻译域漏覆盖 = 0。
- closure.json 输出 `folds_version`（现为 2）与 `folds`（格式如上），结果确定；`dead_branches` 删除。

## 八、风险与对策

| 风险 | 对策 |
|---|---|
| 精度提高后漏边 → 运行期命中 panic 存根 | 3.8 动态对照作为硬验收；抽象值为 Top 时退回「声明类型 ∩ 已实例化」的安全集合；gap_scan 继续按产物扫描 |
| 发射层假设「类型都在 registry 里」（接口 impl 链、vtable 槽） | C3 先改发射层：L1 类型、按 dispatch 发槽；接口链断开时由 levels 把接口升到 L2（规则化，不做特判） |
| 手写层 syn 解析与宏属性格式耦合 | 属性语法由 `rava_macros` / `rava_gen` 共享的解析函数提供（重写计划 R1 拆库后复用同一实现） |
| XTA 在大型 lib（JUnit pilot）上的规模 | 集合用位图（类编号化）；按强连通分量压缩调用图；以 lib pilot 语料做性能基线 |
