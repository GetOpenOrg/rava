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
- `error.rs` 的 `vm-upcalls` 保留，作为无条件根（VM 基础设施确实需要）。

### 3.8 动态视角：真实 JVM 运行轨迹作为健全性基准（只用于验证）

- `java -Xlog:class+load -Xlog:class+init` 跑同一测试，得到**实际加载 / 初始化的类**。
- 判据：动态集合中属于翻译域的类，必须 100% 被静态闭包覆盖（否则说明静态分析漏边）；静态有而动态没有的类，每一个都要能用 provenance 解释。
- 动态轨迹不参与闭包计算（运行期路径依赖输入），只作为验收时的对照。

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

- 所有集合都用 `IndexMap` / `BTreeSet`，确定性由数据结构保证，不再到处加 `sorted()`。
- 性能目标：HelloWorld 闭包计算 ≤ 3s（现行 Python 转译总计 1m37s，其中发现阶段占主要部分）。

### 4.1 命令面

```
rava closure <Test.java|classes-dir|jar> --jdk 21 -o build/<test>/closure.json
rava closure … --why <Class|Class.method:desc>   # 最短 provenance 链（替代 --trace-class）
rava closure … --report md                       # 按等级 / 包 / 边种类统计（替代 jdk-scan 报告）
rava closure … --dynamic <jvm-class-load.log>    # 3.8 对照
```

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
  "folds_version": 1,
  "folds":    [{ "method": "…", "dead_pcs": [[65, 80]], "dead_handlers": [90], "consts": [{ "pc": 12, "kind": "getfield", "value": false, "type": "Z" }] }]
}
```

`dispatch` 与 `folds` 交给发射层使用：vtable 只发射实际存在分派目标的槽；折叠点直接生成常量。
`folds` 的格式约定见 §7.3「折叠点导出」。

## 五、可以移除的现行机制

分析器接入后，下面这些 Python 机制**整体删除**（删除前逐项向用户确认）：

| 现行机制 | 替代 |
|---|---|
| 接口 CHA 遍历、迟至实现者清扫、`_drain_iface_edges` / `_pending_iface_edges` | 3.1 XTA 分派 |
| `instantiated_classes` 全局集合、`root_virtual_targets` 广播、`boundary_virtual_targets` 传播 | 3.1 调用点级集合 |
| `_enqueue_desc_types`、`_enqueue_iface_stub`、stub 通道、父类补全队列、收尾接口 drain | 3.5 分级（L1 不传递） |
| 迟至静态边补扫及三道门 | 统一规则：边界静态方法 = 手写体节点（有 impl）或存根（无 impl），一次判定 |
| `_process` 里的字符串 / 拼接 / 驼峰启发式、`_drain_nest_prefixes`、`_seed_species_family` 全量反射面 | 3.6 常量数据流 + `[reflect_sinks]` |
| `_impl_signature_type_refs` 正则、`provides` 名字前缀判定 | 3.7 syn 解析 |
| `callchain.py` 发现部分整体（`_discover_jdk_classes_method_level`、`_collect_method_refs`、`_scan_reflect_consts`） | `rava closure` + `closure.json` |
| scripts：`trace_callchain*.py`、`rta.py`、`analyze_callchain.py`、`dep_scan.py` 的闭包部分、`scan_jdk_boundary.py` | `rava closure --why/--report/--dynamic` |

**保留**：closure.toml / seeds.toml / vm_intrinsics.toml 三清单（新增 `[facts]`、`[reflect_sinks]` 两段）、JVMS 解析规则、边界截断、`vm-upcalls` 根、locale / JCA / 注解 / 模块资源种子（只是触发条件改成精确可达）。

## 六、实施阶段

| 阶段 | 内容 | 验收 |
|---|---|---|
| C0 | `classfile` + `resolve` crate：jmod 读取、完整解码、层次与 JVMS 解析；与 `codegen/classfile.py` 做解析结果 golden 对照 | JDK 21 / 25 的 java.base 全部类解析结果与 Python 逐字段一致 |
| C1 | `closure` 引擎：absint + cfg + xta + init + 异常；清单读取；provenance | HelloWorld 能输出 closure.json；每个节点都有 via；`--why` 可用 |
| C1b | 值来源追踪：形参级 / 返回值级类型集（VTA 精度）替代方法级 XTA 集 | 7.1 未达标两项达标；动态对照翻译域漏覆盖 = 0 |
| C1c | 手写层 `__set_` 识别 → 字段常量折叠（全写入来源）→ 容器对象按分配点区分 → 选择性上下文敏感（CPA）；closure.json 导出 `folds` | 7.3 所列 7 个用例达标；含 FileIOTest 的动态对照翻译域漏覆盖 = 0 |
| C2 | `handwritten`（syn）+ seeds + reflect 数据流 + `[facts]` / `[reflect_sinks]` 清单段 | 反射缺口清单可观测；手写层边与现行 upcalls 对照无缺失 |
| C3 | `levels` + `dispatch` / `folds`；发射层支持 L1 不透明类型、按 `dispatch` 发射 vtable 槽、折叠点发射常量 | 生成器改动遵守原则 4（无类名字面量） |
| C4 | 接入：`transpile.py` 读 closure.json；删除第五节所列 Python 机制 | 全量 e2e（JDK 21 + 25）全绿；gap_scan precheck 无新增缺口 |
| C5 | 3.8 动态对照纳入 `run_tests.py`（每个测试记录 JVM 加载集与静态闭包的差集） | 翻译域漏覆盖 = 0；静态多出的类 100% 有 provenance 说明 |

各阶段独立 worktree、独立提交；C4 之前 Python 管线保持不变，Rust 分析器只做旁路输出与对照。

## 七、终态目标（量化）

| 指标 | 现状 | 终态 |
|---|---:|---:|
| HelloWorld 总类 | 1854 | ≤ 250 |
| HelloWorld 翻译方法体类（L3） | 1358 | ≤ 120 |
| HelloWorld 入链方法 | 20851 | ≤ 1500 |
| 无 provenance 的闭包节点 | 不可观测 | 0 |
| 翻译域内动态加载类未被覆盖 | 不可观测 | 0 |
| 名字模式类反射启发式 | 4 类 | 0 |
| 生成器中的补扫 / 门控补丁段 | 5 段 | 0 |
| 闭包计算耗时（HelloWorld） | Python 发现阶段数十秒 | ≤ 3s |
| 全量 e2e | 现行通过集 | 不减少 |

上限依据：trace Internal Boundary 模式（同样有边界截断，只用单程分析）为 174 类 / 388 方法；现行机制额外承担的 VM 初始化（`vm-upcalls` 根、System 初始化链、手写层回调）预留约 40% 余量。
C1 完成后以实测替换这些上限，并写回本节（只允许下调）。

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

0. **手写层写字段识别**（前置，也修 C1b 的潜在漏边）：手写体中的 `recv.__set_<字段>(v)`（现有 259 处）
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
1. **字段常量折叠**：每个字段维护一个写入值集，由以下几部分组成：
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
2. **容器对象按分配点区分**：数组分配点模型扩展到「容器形态类」的对象，按 `new` 的位置分开追踪其元素字段的类型。
   容器形态按字节码判定：类持有 `Object[]` / 引用数组字段，或持有引用字段且其方法对该字段值做虚调用。
   只做一层对象敏感：`HashMap` 的 `table` / `Node.key`、`ImmutableCollections` 的元素数组等按分配点分离。
   `HashMap.put(K,V)` 这类大方法无法靠调用点克隆切断，由对象敏感解决。
3. **选择性上下文敏感（CPA）**：对小型方法按「实参类型集合」做上下文键克隆（Cartesian Product 思路），不按调用点编号。
   多个传入相同类型集合的调用点共用一份克隆，深度不做硬性上限，由类型集合的有限性保证终止。
   选中条件：指令数有上限，且形参值流向虚调用接收者、返回值，或字符串拼接 indy 的实参（拼接对实参调用 toString）。
   `append(Object)` → `valueOf(Object)` → `toString()`、`Objects.hashCode`、`doPrivileged`、`Set12.<init>` 的
   `"duplicate element: " + e0` 由此按实际实参派发。
4. **反射返回值**：C2 的 `[reflect_sinks]` / 反射数据流按字节码给出反射目标的返回类型，替代 open(Object)。

第 1 步完成后单独测一次 TestStreamBasic，量出并行流路径（ForkJoin / VarHandles）占多少类，再定第 2、3 步做多深。

**折叠点导出（C3 / C4 的衔接，格式 v1 已定）**：Rust 闭包剪掉的分支，Python 生成器必须同样不翻译，
否则生成代码会引用闭包外的类，编译失败。closure.json 为每个存在不可达代码或常量折叠点的方法导出：

```json
"folds_version": 1,
"folds": [{
  "method": "java/util/stream/AbstractPipeline.evaluate:(Ljava/util/stream/TerminalOp;)Ljava/lang/Object;",
  "dead_pcs": [[65, 80]],
  "dead_handlers": [90],
  "consts": [{"pc": 12, "kind": "getfield", "value": false, "type": "Z"}]
}]
```

1. **`dead_pcs`**：不可达指令的半开区间 `[start, end)`。
   - 两端都落在指令起点上。
   - 按 start 排序，互不重叠，相邻区间合并。
   - 条件跳转被常量裁掉的一侧就体现在这里。Python 在 CFG 结构化之前把只剩一个活后继的条件跳转改写成 goto 或直通，不另设分支字段。
2. **`dead_handlers`**：起点不可达的异常处理器 pc，Python 据此删除对应的异常表项。
   - 只要某 try 区间内还有可达指令，引用它的 handler 就不会进 `dead_pcs` / `dead_handlers`。
   - 不输出「try 区间部分删除，却保留引用它的 handler」的组合。
3. **`consts`**：被折叠成常量的读取点。`kind` 取 `getfield` / `getstatic` / `invoke` 三种，栈效应如下：
   - `getfield`：替换后弹出 receiver。
   - `invoke`：替换后弹出全部实参，有 receiver 也一并弹出。
   - 实参 / receiver 表达式的副作用由 Python 保留求值、丢弃结果，JSON 只给 pc。
4. **`value` 与 `type`**：`type` 是 JVM 描述符（`Z/B/C/S/I/J/F/D`、`Ljava/lang/String;`）。
   - null 写成 `"value": null`，`type` 给声明类型。
   - `J` / `D` 的值用字符串编码（`"9007199254740993"`），避免 JSON 数值丢精度。
   - `Z` 用 JSON 布尔值，其余整型用 JSON 整数。
5. **确定性**：`folds` 按 method 排序，`consts` 按 pc 排序，同输入逐字节相同。
6. **版本**：顶层 `folds_version` 当前为 1。Python 遇到不认识的版本时忽略 folds、按原样翻译、不报错，两边可以各自先合入。

原有的 `dead_branches` 输出整体由 `folds` 取代。Python 只消费这份数据，不另写判定；消费侧（C3）由用户实现，验收用 `compare_trees.sh`：
- 不带 folds 时，生成树逐字节不变；
- 带 folds 时，只有预期的方法体变化，并且生成代码引用的类都在闭包之内。

**C1c 验收**：
- HelloWorld、TestSwitchString、PatternSwitchTest、TestRecordComponents、TestStreamBasic、CollectorsDemo、
  ChineseRemainderTheorem 7 个用例：总类 ≤ 400，translate ∩ code ≤ 250，单测试耗时 ≤ 3s。
- HelloWorld 指标不回退（第 0 步后 200 / 115 / 457）。
- 上述 7 个用例以及 FileIOTest（手写层写字段密集，检验写入来源是否收全）的动态对照，翻译域漏覆盖 = 0。
- closure.json 输出 `folds_version: 1` 与 `folds`（格式如上），结果确定；`dead_branches` 删除。

## 八、风险与对策

| 风险 | 对策 |
|---|---|
| 精度提高后漏边 → 运行期命中 panic 存根 | 3.8 动态对照作为硬验收；抽象值为 Top 时退回「声明类型 ∩ 已实例化」的安全集合；gap_scan 继续按产物扫描 |
| 发射层假设「类型都在 registry 里」（接口 impl 链、vtable 槽） | C3 先改发射层：L1 类型、按 dispatch 发槽；接口链断开时由 levels 把接口升到 L2（规则化，不做特判） |
| 手写层 syn 解析与宏属性格式耦合 | 属性语法由 `rava_macros` / `rava_gen` 共享的解析函数提供（重写计划 R1 拆库后复用同一实现） |
| XTA 在大型 lib（JUnit pilot）上的规模 | 集合用位图（类编号化）；按强连通分量压缩调用图；以 lib pilot 语料做性能基线 |
