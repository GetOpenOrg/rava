# 需求登记扩展调研：清单第 4 项（抽象槽位登记缺口）+ 第 20 项（缺席直接接口宽化）

> 2026-09-24 · 只读调研（未改代码，未跑 cargo/e2e/main.py）· 基线 HEAD `630c6f3`
> 证据来源：源码走读、`git show`（254fbb4 / 92a2c1a / 7ea4188 / d8c431c / 0f46fb8）；另在 scratchpad 里用 python3.12 单独跑了 BFS 发现函数 `_discover_jdk_classes_method_level`（只在内存里计算，不写仓库、不生成 Rust），本机 JDK 21（Linux）。

## 结论摘要

1. **第 4 项的根因**：「某个抽象槽位被调用」这个需求其实已经记进了调用链（例如 `(FileSystemProvider, isSameFile)` 在 visited_methods 里）。但定义侧给子类填槽时，门控查的是**实现者或子类自己**的键，不查**槽位声明类**的键（`class_writer.py:874-879`）；调用侧在 typed 外部接收者上也不给子类登记（`invoke_virtual.py:569-581` 只处理 `this`）。结果是中间祖先实现、叶子类继承的槽位落到声明类的 trait default（抽象声明就是 stub panic，具体声明则静默执行错误的体）。
2. **第 20 项的根因**：c23a4b0 的 stub 通道（`_enqueue_iface_stub`）在**stub 通道内部**和**父类补全队列**里只把接口写进 `field_discover_classes` 集合，不回灌 `_stub_queue`（`callchain.py:787/827/854`）。所以接口「登记了但从没物化」。实测 TestAutoboxEdge 有 20 个缺席接口，全部满足 `in field_discover_classes = True`。
3. **两项同类不同源**：都是「需求已入账，消费端没闭合」，但分属两本账（成员需求账 `inherited_calls` 对类型闭包账 `field_discover_classes`），改动点、扰动面、验收口径都不同。可以同分支、分两个提交，先做第 4 项（闭包零扰动），再做第 20 项（闭包只增加接口）。
4. **改动量**：第 4 项约 20–35 行（class_writer 一个函数），另删 1 个止血手写文件（`file_system_provider_impl.rs`）；第 20 项约 15–30 行（callchain 一个函数段）。第 20 项的主要风险在编译面：接口进 registry 后，interface_gen 会发出新的 `impl I for C`，需要服务器全量批验证。
5. **FileSystems 的 vm_boundary 条目不建议撤回**：它真正的收益是截掉了反射子系统（约 1000 类、bin −143M），修好第 4 项后只需把收录理由从「codegen 缺口」改为「策略截断」。

---

## 一、原始证据与当时的止血方式

### 1.1 第 4 项（FileSystems 根因）

| 证据 | 内容 |
|---|---|
| `254fbb4`（posix 档 A 切入序 2+3） | 提交信息原文：「FileSystems 收编 vm_boundary（getFileSystem 抽象槽位的继承成员需求缺口，getDefault 等价手写）」 |
| `92a2c1a`（切入序 4+5+6） | 「FileSystemProvider.isSameFile 抽象槽位缺省承接（typed 调用的继承成员需求缺口，与 FileSystems 收编同根因）」 |
| `runtime/java_runtime/vm_boundary.txt:21-28` | 注释原文：「getFileSystem 为 UnixFileSystemProvider 的 final 声明——生成侧继承成员的需求登记不覆盖『静态类型接收者 + 抽象槽位』形态，平台 provider 子类（sun/ 存根）的 vtable 槽位缺失」 |
| `runtime/.../java/nio/file/FileSystems` 伴生 `file_systems_impl.rs` | `getDefault()` 直接返回 `DefaultFileSystemProvider::theFileSystem()`（绕开 `getDefaultProvider().getFileSystem(URI)` 这次 typed 抽象槽位调用） |
| `runtime/.../java/nio/file/spi/file_system_provider_impl.rs` | 在**声明类** FileSystemProvider 上手写 `__impl_isSameFile`，经 `try_cast::<UnixFileSystemProvider>` 精确直调。这是用手写代码补一个生成侧空槽的止血做法 |
| `docs/tasks.md:101` | 「新立项线索：①typed 调用对抽象槽位（静态类型接收者）的继承成员需求登记缺口（FileSystems 根因，codegen demand 登记扩展）」 |

止血一共两处：①把整个 FileSystems 类收编为 VM 边界类（手写 getDefault）；②在 FileSystemProvider 上手写 isSameFile 的缺省承接。

### 1.2 第 20 项（缺席直接接口）

- `docs/tasks.md:60`（ice-dispatch ③）：ListIterator 未进闭包，已由 c23a4b0（compile14 的 Pecs 修复，「BFS 接口闭包 stub 通道」）顺带修复。残留记录为：「TestAutoboxEdge 闭包内 11 个缺席直接接口（Readable/AnnotatedElement/SortedSet/Lock 等）均不在 CARRIER_TYPE_POSITIONS，无 itable 分派路径，如需收口走 c23a4b0 stub 通道宽化另立项」。
- `docs/tasks.md:63`：同一条单列成待办。
- 注：c23a4b0 这个哈希在本仓库历史里不存在（历史经过压平，`cc22841` 是初始整包导入）。它对应的代码就是 `codegen/callchain.py:181-207` 的 `_enqueue_iface_stub`，以及 `:574-578`、`:827` 两个调用点。

---

## 二、根因机制（定位到代码）

### 2.1 第 4 项：槽位需求在账，定义侧门控却按实现者查

#### 登记账本与消费

- 账本：`codegen/inherited_calls.py`，形如 `{接收者 binary → {(方法名, 参数描述符)}}`。
- 消费端：`codegen/emitter/inherited_gen.py:733`（`resolve_inherited_members`）。它在接收者的 java_class! 块里补出带转发体的继承成员；宏在 `runtime/java_rta_macros/src/block/gen/virtual_dispatch.rs:440-475`（「继承成员填槽（S-16）」）用这个转发体去填本类对 `vtable_owner` 的 vtable impl。**不登记就不会填槽**：Rust 的 trait impl 没有继承，叶子类对祖先 `X__VTable` 的 impl 里，没填的槽就是 trait default（abstract 声明就是 `panic!("stub: …")`）。

#### 四个登记入口的覆盖面

| 入口 | 位置 | 是否给闭包子类登记 |
|---|---|---|
| bare Object 接收者、类虚方法 | `invoke_virtual.py:303-320`（`_emit_class_vtable_dispatch`） | 是：遍历 `_closure_subclasses` 全部子类，由 `_virtually_dispatched` 过滤 |
| `this` 接收者 | `invoke_virtual.py:564-581` | 是（S-16） |
| **typed 外部接收者**（`provider.isSameFile(..)`，静态类型是抽象类） | `invoke_virtual.py:510-560` | **否**。方法在静态类型上已声明（`_declared_here=True`，哪怕是 abstract），整段都不登记；子类循环又被 `obj_e in ('this','self')` 挡住（`:574`）。`:571-572` 的注释「外部接收者的调用按静态类型成员分派（sig-poly 站点已按子类分支登记），不在本登记范围」这个前提对「中间祖先实现」形态不成立 |
| 定义侧超类虚方法继承 | `class_writer.py:800-897`（`_emit_superclass_virtual_inheritance`），JDK 链登记在 `:889-897` | **部分**。门控 `_vm_in_cc`（`:874-879`）只认 `(本类, m, d)` 或 `(当前祖先 _vinh_super, m, d)` 在 call_chain 里，**不认槽位声明类的键** |

#### 为什么 FileSystems / isSameFile 会同时漏掉两道

- 调用链里有 `(java/nio/file/spi/FileSystemProvider, isSameFile, d)`：常量池类就是抽象声明类，`enqueue_refs` 按这个键入队，`_process` 找到 abstract 声明后不再展开。
- 叶子类 `LinuxFileSystemProvider`（macOS 上是 `MacOSXFileSystemProvider`）的实现来自中间祖先 `UnixFileSystemProvider`，且全部是 sun/ 边界类。
  - RTA 补不上：叶子对象由**手写** `DefaultFileSystemProvider::instance()` 构造，不在 `instantiated_classes` 里（`callchain.py:697-735` 的 RTA 只认字节码里的 new 和 java/ 回调的 `<init>`）。祖先解析那一步遇到边界类也只进 `field_discover_classes`（`callchain.py:441-446`），`(UnixFileSystemProvider, isSameFile)` 永远进不了 call_chain。
  - 定义侧门控查不到：它查的是 `(Linux, isSameFile)` 或 `(Unix, isSameFile)`，两者都不在 call_chain；在账的键 `(FileSystemProvider, isSameFile)` 它不看。
  - 调用侧不登记：typed 接收者这一路不给子类登记（见上表）。
- 结果：叶子类对 `FileSystemProvider__VTable::isSameFile` 的槽位落到 abstract stub，只能在声明类上手写承接（92a2c1a）。`getFileSystem` 同理，当时直接把 FileSystems 收编成边界类绕开（254fbb4）。

#### 为什么同族的其他 typed 调用「碰巧能跑」

newByteChannel、delete 等槽位被 FileSystemProvider 自身 default 体里的 `this.newByteChannel(..)` / `this.delete(..)` 通过 `this` 路径登记给了全部子类，属于**间接覆盖**。`deleteIfExists` 的叶子槽落到 FileSystemProvider 的**默认体**（try delete / catch NoSuchFile），而不是 AbstractFileSystemProvider 的 final 覆盖。这次观察上等价，但属于「静默执行错误的体」形态：比 panic 更危险，因为不报错。

#### 实测扰动面（本机 JDK21，scratchpad 探针）

口径：visited 方法里的 invokevirtual 站点，常量池类是非接口类，链上最近声明非 private/final/static。「HOLE」指闭包子类 S 自身没有声明，S 的最近声明者 I 不同于槽位声明者，而且 `(S,m,d)` 和 `(I,m,d)` 都不在 call_chain。探针没有计入 `this` 路径的发射期登记，所以会**高估**。

| 测试 | 闭包类数 | 候选（槽位, 子类）对 | HOLE | 代表 |
|---|---|---|---|---|
| TestAutoboxEdge | 398 | 8 | 2 | `AbstractMap.size/entrySet` → `ClassValue$ClassValueMap`（实现在 WeakHashMap） |
| TestFilesApi | 484 | 27 | 18 | `FileSystemProvider.{isSameFile,checkAccess,exists,newByteChannel,delete,deleteIfExists}` → `LinuxFileSystemProvider`；`FileSystem.{provider,getPath}` → `LinuxFileSystem`；`Throwable.getMessage` → `NoSuchFileException` 等 4 个（实现在 FileSystemException，**具体声明形态，漏填就是静默取错 message**；目前靠 Throwable 体内 `this.getMessage` 的登记间接覆盖） |

量级很小（每测试个位数到十几条转发成员），修复后生成规模的增长可以忽略。

### 2.2 第 20 项：stub 通道只登记不回灌

#### c23a4b0 通道的结构

- `_enqueue_iface_stub`（`callchain.py:181-207`）：沿 `interfaces` 边传递闭包，把接口加入 `field_discover_classes`（`:205-206`）。
- 物化只发生在 stub 通道的处理循环里：`:787` 取的是 `sorted(field_discover_classes)` 的**快照**，`:788` `_stub_visited` 也是快照。

#### 三个漏口

| 漏口 | 位置 | 现象 |
|---|---|---|
| ① stub 通道内部 | `callchain.py:827` 在循环体里调用 `_enqueue_iface_stub(cls)`：只写集合，不追加到 `_stub_queue`/`_stub_visited`。同一循环里的字段类型分支（`:834-840`）和父类分支（`:843-848`）**都会**回灌 | stub 通道类（CharBuffer、FileChannel、ArrayDeque、Path、Stream、NavigableSet、ReentrantLock……）的直接接口缺席 |
| ② 父类补全队列 | `callchain.py:853-873` 完全不调用 `_enqueue_iface_stub` | 经父类补全进来的类（Reader、AccessibleObject、Executable、AbstractInterruptibleChannel、Permission、AbstractExecutorService……）的接口缺席 |
| ③ 迟至补扫 | `:876-975`（静态边补扫、实现者清扫）会再调 `_process`，其中的 `_enqueue_iface_stub` 和 `enqueue_refs` 的 type-only 引用都写在快照之后 | 本例未观测到，但机制上存在（第三通道的 type-only 需求会整体丢失） |

`field_discover_classes` 作为返回值只用于 `transpile.py:205-217` 的扫描报告计数，**不参与物化**，所以「在集合里」不等于「在 registry 里」。

#### 为什么缺席会造成「无 itable 分派路径」

`interface_gen._all_interfaces`（`interface_gen.py:110-128`）靠 registry 沿超接口展开。缺席的接口本身 `registry.get() is None`：`resolve_interface_impls`（`:215-218`）直接 `continue`，不发 `impl I for C`，它的超接口链也在这里断开（ListIterator 事故正是这个形态）。bare 接收者分派（`invoke_virtual.py:171-173`）要求 `registry.get(iface)` 存在，否则落到「A-5 未翻译接收者残余」的 `Default::default()`。

#### 实测（TestAutoboxEdge，当前 HEAD）

缺席直接接口共 **20 个**，全部 `in field_discover_classes = True`（登记了但没物化），来源只有漏口 ①（stub 通道）和漏口 ②（父类补全）：

```
Readable(Reader:父类队列, CharBuffer:stub)  TypeDescriptor$OfField(Class:stub)  TypeDescriptor$OfMethod(MethodType:stub)
AnnotatedElement(Class/Module/Parameter:stub, AccessibleObject:父类队列)  GenericDeclaration  Member
nio.channels.{Channel, GatheringByteChannel, InterruptibleChannel, ScatteringByteChannel, SeekableByteChannel}
nio.file.Watchable(Path:stub)  security.Guard(Permission:父类队列)  util.Deque(ArrayDeque:stub)
util.SortedSet(NavigableSet:stub)  concurrent.{ExecutorService, Future}  locks.Lock(ReentrantLock:stub)
stream.BaseStream(Stream:stub)  sun.nio.cs.HistoricallyNamedCharset(Unicode:父类队列)
```

与 tasks.md 记载的「11 个」口径不一致。推测原因：当时的基线树较早，也可能排除了 reflect/invoke/sun/security 子类。另外，**`java/util/Deque` 在 `CARRIER_TYPE_POSITIONS` 里**（`jvm_type.py:647`），却依然缺席。这说明缺席与载体铺设门控无关，问题出在 registry 物化上；tasks.md「均不在 CARRIER_TYPE_POSITIONS」的归因不准确。TestFilesApi 同口径缺席 18 个。

---

## 三、两项是否同源

**判断：同类缺陷，不同源；不是同一登记机制的两个漏口。**

| 维度 | 第 4 项 | 第 20 项 |
|---|---|---|
| 账本 | `inherited_calls._requests`（成员需求，发射期） | `field_discover_classes`（类型闭包需求，BFS 期） |
| 失配形态 | 消费端门控**按错误的键**查（实现者/子类，而非槽位声明者）；调用侧 typed 路径缺登记 | 消费端**只读快照**，后续登记不回灌 |
| 生产方 | 调用链 visited_methods 已含槽位键 | `_enqueue_iface_stub` 已写入集合 |
| 修复对闭包的影响 | 零（visited 与 jdk_infos 不变，只增加转发成员） | 只增加接口（新增类 ⊆ 旧 `field_discover_classes`） |
| 运行期症状 | 虚分派命中 abstract stub / 静默走声明类体 | 接口 impl 关系断链：AbstractMethodError 类问题、bare 分派 `Default::default()` |

共同的元规律可以沉淀为设计约束：**需求账本的消费端必须是不动点，并按「需求被记录的键」消费，不能按「推测的实现者键」消费。**这条应写进重写规格（见第六节）。

---

## 四、修复方案

### 4.1 第 4 项：定义侧「槽位需求」门控（推荐的单一决策点）

**决策点**：`codegen/emitter/class_writer.py::_emit_superclass_virtual_inheritance`，JDK 链登记分支（`:874-897`）。

**做法**：
1. 按 `id(call_chain)` 记忆化建索引 `{class → {(m, param_desc)}}`，一次转译只建一次。
2. 对当前类 ci 的**整条超类链**（含 `_vinh_super` 以上直到 Object），求并集 `_slot_demanded = {(m, pd)}`，表示这个槽位在链上任一声明层被调用链索要过。
3. JDK 分支的门控改为 `_vm_in_cc or (_vm.name, pd) in _slot_demanded`。**只放宽 `:879` 的 JDK 登记门**；`:918`、`:966` 的用户链全量发射路径继续用原来的 `_vm_in_cc`，控制爆炸半径。
4. 新放行的条目追加 `_virtually_dispatched(槽位声明类 ci, m, pd, registry, sub_bin=ci.name)` 过滤：与调用侧同口径的 K-6b 类型变量签名和 bridge 见证，避免继承成员擦除缺口带来的 E0053。

**为什么选定义侧而不是调用侧**（调用侧方案是去掉 `invoke_virtual.py:574` 的 `this` 限制）：
- 定义侧以 BFS 的 call_chain 为唯一真源，覆盖**所有**调用形态：typed、bare、this、方法引用，以及手写 `_impl.rs` 的 upcall 入链。调用侧只能逐个发射路径补，手写体发起的 typed 调用不经过发射器，永远补不到。
- 与既有的 `_vm_in_cc` 语义同构（「调用链收录了这个槽位，就给子类填」），只是把键从「实现者」换成「槽位」。
- 调用侧的 `this` 子类登记（`:564-581`）修复后基本冗余，但**暂不删除**（K-6b 口径已在那里验证过），列为后续清理项。

**附带清理**（同一提交）：
- 删除 `runtime/java_runtime/src/java/nio/file/spi/file_system_provider_impl.rs`（isSameFile 缺省承接）。修复有效的话，TestFilesApi 的 `mismatchSame` 行仍然通过，这就是见证。
- `vm_boundary.txt:21-27` 把 FileSystems 的收录理由改写为策略截断：`getDefaultProvider` 的系统属性、ClassLoader 反射链会展开反射子系统（约 1000 类，bin −143M，见 d8c431c 合入说明）。**保留**这个条目。
- 更正 `invoke_virtual.py:571-572` 的注释前提。

**改动量**：约 20–35 行 Python，删 23 行手写，改约 6 行注释。

**风险**：
- 闭包零扰动：不改 BFS，`visited_methods`、`jdk_infos` 不变，闭包指纹应逐项相同。
- 生成树只增加 `inherited_from=` 转发成员；探针量级是每测试 2–18 条。
- 主要风险：转发目标是祖先上的 panic 存根时（实现者方法不在调用链），这个槽从「声明类 stub」变成「实现者 stub」，panic 信息反而更准，不是回归。另有 K-6b 类型变量签名的擦除错位，已由第 4 步过滤。
- 次要风险：新增转发成员与本类手写 `_impl.rs` 同名（E0201）。inherited_gen 既有的 `provided` / own_names 去重应该能吸收，需要服务器批确认。

### 4.2 第 20 项：stub 通道回灌（c23a4b0 通道宽化）

**决策点**：`codegen/callchain.py` 的 stub 通道段（`:784-873`），也就是 `_enqueue_iface_stub` 的消费端。

**做法**（窄口径，只物化接口）：
1. 把 `_enqueue_iface_stub` 改成**返回**新发现的接口名列表（或者接收一个 `sink` 回调），在 stub 通道和父类补全队列两处把返回值追加到 `_stub_queue` / `_stub_visited`，与 `:834-848` 的字段、父类分支同构。修复漏口 ① 和 ②。
2. 父类补全队列（`:853-873`）对每个新入的类同样调用 `_enqueue_iface_stub`。
3. 漏口 ③（迟至补扫后新写入的集合项）：两个迟至循环结束后做一次**只处理接口**的收尾 drain，即 `field_discover_classes - jdk_infos` 中 `is_interface` 的项。不重走 stub 通道全量，因为代码注释多处警示「LocaleSyntaxException E0425 / 4b776b6 stub 通道类型闭环重开」。
4. 所有 drain 都按 sorted 顺序迭代，保证双种子确定性（与 `069e1be` 同口径）。

**改动量**：约 15–30 行 Python。

**风险**：
- 闭包扰动：只增加接口。TestAutoboxEdge 预计 +20（398 → 约 418，+5%），TestFilesApi +18。接口 stub 不入队方法（interface 的 `<clinit>` 常量除外，需要核实），`visited_methods` 应当不变。
- **编译面是主要风险**：接口进 registry 后，`resolve_interface_impls` 会为实现类发 `impl I for C`，每个成员经 `_locate` 转发（多数落到存根），并通过 `inherited_calls.request` 追加成员。高风险形态有三种：
  1. 方法级泛型：`ExecutorService.<T>submit`、`AnnotatedElement.<T extends Annotation>getAnnotation`、`Future<T>`。
  2. 载体门控内的接口（Deque）进 registry 后，**类型位置**从 Object 回落变成载体 `Deque<Object>`，签名文本连锁变化。
  3. 边界、手写类作实现者（Class 是 vm_boundary 手写，`recv.handwritten` 已跳过；MemberName、Unicode 是 sun/ 存根）。
- 膨胀评估：只物化「已登记」的接口，不引入新的登记源。闭包增量有上界（= 旧集合差集），不会链式膨胀。生成规模的增量主要来自 impl 块（每个接口 × 实现类 × 成员数），Deque 在 ArrayDeque 上约 40 条。

---

## 五、验证方案

### 5.1 第 4 项

| 口径 | 验收条件 |
|---|---|
| 闭包指纹 | 修复前后 `jdk_class_infos` 名单与 `visited_methods` 计数**逐项一致**；`[bfs-audit] sig-poly-native / root-inherited / unresolved` 三计数不变 |
| 生成树 diff | 只允许 `#[java_method(... inherited_from = ...)]` 转发成员的**新增**（grep 计数差 = 新增登记条数），不允许其他行变化 |
| 双种子 | `PYTHONHASHSEED=1/2` 生成树 diff = 0 |
| 见证 | 删除 `file_system_provider_impl.rs` 后 TestFilesApi 全绿（Linux 服务器侧；macOS 另轮），`mismatchSame=-1` 行由生成的转发槽承载 |
| 审计线 | `raw_audit` type_surgery、equiv-audit、fallback-audit 计数不变 |

回归集（本地 ≤10 例一轮，其余交服务器）：
- 第一轮：**TestFilesApi**、TestPrintStreamApi、TestLocalDate、TestStringEdge、TestArrayList（posix 档 A 原 5 例），TestAutoboxEdge、TestExceptionChain（Throwable.getMessage 槽）、TestSuppressed、TestBridgeMethod（桥成员）、TestGenericBoundsCombo（K-6b 类型变量）。
- 第二轮：TestHashSetOps、TestMapIteration、TestTreeMapSet（ImmutableCollections 族）、TestStreamBasic、TestStreamAdvanced（AbstractPipeline 槽）、TestCustomException、TestIterator。
- 服务器全量：169 语料生成树逐字节 diff，只允许转发成员新增。

### 5.2 第 20 项

| 口径 | 验收条件 |
|---|---|
| 闭包指纹 | **新增类 ⊆ 修复前 `field_discover_classes` 且全部 `is_interface`**（最强不变式：不引入任何非接口类、不引入账外类）；删除类 = ∅；`visited_methods` 计数不变（如果接口 `<clinit>` 入链，需要逐条列举说明） |
| 缺席计数 | 探针口径的「缺席直接接口」在 TestAutoboxEdge、TestFilesApi 归零（或只剩有意排除的 sun/ 项并写明理由） |
| 双种子 | 生成树 diff = 0 |
| 审计线 | bfs-audit 三计数不变；`cc-stub-chan` / `cc-parent-queue` fallback 计数不增 |

回归集：
- 第一轮：**TestAutoboxEdge**、**TestArrayDeque**（Deque，载体门控内）、TestTreeMapSet（SortedSet）、TestFilesApi（Channel 族、Watchable）、TestPrintStreamApi（Readable via Reader）、TestStreamBasic（BaseStream）、TestPecs、TestIterator、TestLinkedList（ice ③ 同链防回归）、TestAtomics（locks 邻域）。
- 服务器全量 169：**重点看 compile 面**（E0053、E0433、E0599、E0201），任何新增编译错误都按「接口进 registry 暴露的既有 interface_gen 缺口」归类，另开条目，不在本项内打补丁。

---

## 六、依赖关系

| 关联项 | 关系 |
|---|---|
| **A-4 类型位置载体化**（`CARRIER_TYPE_POSITIONS` 终态置 None） | 第 20 项是 A-4 终态的**前置**：门控放开后，缺席接口仍然因为 `carrier_type()` 的 `registry.get() is None` 回落 Object（`jvm_type.py:678-680`）。Deque 已在门控里仍然缺席就是实证 |
| ice ①（default 注入扫父类链，`33a63e9`）、InterfaceConflict existing_sigs 父类链缺口（`1075f11` 诊断） | 同属「沿层次遍历只看局部」家族，与第 4 项不撞文件（interface_gen 对 class_writer 的不同函数） |
| R-2′ 上转发射统一 | 写入域是 hierarchy/invoke_sig 和 `class_writer.py:1287-1293` 注释，与第 4 项的 `:800-897` 同文件不同段，协调合并顺序即可 |
| TypeIR 余项 / 窗口 3（method/vars.py） | 无交集 |
| **重写主线（R0）** | 两项都是小件 Python 修补，不阻塞 R0。但第三节的元规律（需求账本消费端必须是不动点、按记录键消费）应写入重写规格。重写版的 BFS 建议把 stub 通道、父类补全、迟至补扫统一成**单一 worklist 不动点**，从结构上消灭快照型漏口；成员需求建议以「槽位键」为唯一主键 |
| 清单内顺序建议 | 先第 4 项（零闭包扰动，可以本地闭环），再第 20 项（需要服务器全量编译面） |

---

## 七、开放问题

1. **计数口径**：tasks.md 记载 11 个，当前 HEAD 实测 20 个（TestAutoboxEdge，Linux JDK21）。需要确认当时的基线和排除规则；本报告以 20 为准。
2. **第 20 项的物化深度**：只物化接口本体（推荐），还是走完整 stub 通道语义（连带字段类型、超类）？后者会重开 4b776b6 记录的类型闭环风险。
3. **RTA 与手写构造**：手写 `_impl.rs` 里构造的公开包对象不进 `instantiated_classes`，它们的覆盖方法不会经 RTA 入链。第 4 项的定义侧修复只保证填槽，不保证实现体被翻译；当实现者是 java/ 类且体不在链上时，仍然会落到实现者 stub。是否要把「手写构造」纳入 RTA（例如 upcalls 声明 `<init>` 的扩展）需要另外评估。
4. **`this` 路径子类登记的去留**：定义侧修复后，`invoke_virtual.py:564-581` 与之重复，是否在第二步合并为单一来源？
5. **漏口 ③ 的全语料量级**：本例未观测到，需要服务器全量探针统计「迟至补扫后写入、未物化」的 type-only 需求（不限于接口）。
6. **平台差异**：探针在 Linux 语料上跑。macOS 的 `MacOSXFileSystemProvider → BsdFileSystemProvider → Unix` 三层链多一个中间层，第 4 项修复应在两侧各验一轮（服务器轮 + 本机轮）。
