# TestJndiNoProvider 冷闭包转译性能（2026-10-03）

## 目标与约束

- TestJndiNoProvider 冷转译（无闭包缓存）当前 5–6 分钟（服务器），600 秒上限不放宽，要求明显低于上限。
- 只做去重复计算 / 增量 / 缓存与结构性去冗余；**不得靠缩小闭包提速**，类集 / 方法集与修前逐项相同。
- 测量：本机 `rava closure`（冷，独立 `--closure-cache`），经 `heavy_lock.py bash capped.sh`（600s / 8G 封顶）串行；
  集合对照忽略 via / perf / elapsed，其余字段逐项相同。

## 基线（0fad1f6b，本机）

- 闭包 195–205 s（`summary.elapsed_ms` 198.7 s），3540 类 / 22361 方法；HelloWorld 468 / 1822，StockTrans 3107 / 18814。
- 阶段：sites 71 s、flows 67 s、process 51 s。
- 采样剖析（≈ 99k 样本）热点：
  - `drain_flows` / `add_to_id`（≈ 31%，其中 `is_subset` 自身 12.6k）——推送绝大多数无增量；
  - `hubs_grow → hub_recv`（≈ 17%），`link_hub`（13k；其中 `special` 接收者表收集成 `TypeSet` 排序 4.4k）；
  - `dispatch_one`（16k），`apply_hw → dispatch_one`（10.5k）；
  - `reflective_writes → class_values`（8.4k，每次重跑把整个 Class 值集重新映射成类名）；
  - `Hierarchy::select → superclasses`（3.3k，同一（类, 方法）反复解析）；
  - `edge_ret → declares_memory / memory_modeled`（≈ 2.9k，每次格式化成员键查清单）；
  - `method_ctx → sysprops` 的 `BTreeSet<MemberRef>` 克隆插入（1.9k）。

## 第 1 步：重复计算去除（已完成）

| 改动 | 位置 |
|---|---|
| `declares_memory` 按方法缓存（键不变）；`memory_modeled` 并入（返回值模型 Read 即 memory_read 登记） | `hw_offset.rs` / `hw_mem.rs` / `invoke.rs` |
| 字段枚举 / 字段查找调用点的 Class 值集按站点增量处理（只映射新增部分；名字 / 模式变化时整体重算；与 `recv_done` 同口径作废） | `field_lookup.rs` / `invoke.rs` / `worklist.rs` |
| `IdSet` 大集合 `FromIterator` 直接置位，不再排序 | `idset.rs` |
| `sysprops` 未跟踪成员表改哈希集合，先查后插免克隆 | `sysprops.rs` / `sysprops_write.rs` |
| `Hierarchy::select` 按（解析类, 方法）记忆 | `resolve/hierarchy.rs` |
| 观测：按源节点的推送计数（`top_push_sources` / `push_src_by_kind`），枢纽节点标签带集合 / 调用点 / 目标数 | `graph.rs` / `flow.rs` / `stats.rs` |

结果：冷闭包 198.7 s → 126.0 s（本机，−37%），集合与基线逐项相同；HelloWorld / StockTrans 不变。

## 推送剖析（第 1 步后）

- 推送 ≈ 1.19 G 次，有增量 ≈ 4%。按边种类：`S→HP` 322 M、`HP→P` 242 M、`E→S` 152 M、`P→HP` 130 M。
- 枢纽 43 176 个，（调用点, 枢纽）接入 4.33 M，单调用点最多 343 个枢纽：
  调用点实参直接接到它接入过的每个枢纽（精确集合每增长一次换接一个新枢纽、open 集合每个 open 类型一个枢纽），
  实参每次增量都向全部历史枢纽推送。
- 推送最多的 `HP` 节点是约 200 个 `Object.equals` 精确集合枢纽（各 3–4k 接收者、≈ 1100 个中转目标、无父枢纽），
  每个出队 ≈ 1000 次、每次推 ≈ 1100 条边，几乎全部无增量。

## 发现：闭包结果依赖传播顺序（既有问题）

基线二进制（0fad1f6b）仅把 `--flow-batch` 由缺省 64 改为 7，结果变为 3589 类 / 23016 方法
（多出 `ObjectStreamClass.getInheritableMethod / getPrivateMethod <- recv(java/lang/Class)` 缺口及其放开的序列化成员：
`*$SerializationProxy`、`KeyRep`、`java/time/Ser` 等）。单调系统的不动点与求值顺序无关，
说明引擎中存在按当时状态一次性决定、之后不再重新评估的判定（非单调或漏登记读者）。

影响：任何改变推送顺序的结构性优化（如下文实参端口）都会使结果在 3540 与 3589 两个口径之间摆动，
「集合逐项相同」作为验收在顺序敏感存在时不可靠。终态应先消除顺序依赖（定位该判定并改为可重算 / 正确登记读者），
使不动点唯一，再做改变推送顺序的结构优化。

### 根因（已查实）

判定位置：`engine/invoke.rs::reflective_writes` 的按名查方法分支。名字实参是 `V::Str` 时记为「本调用点字面量」
（`site_names`），与 Class 接收者值集里的类镜像相乘（`recv_mirrors` → `reflect_name`），非镜像 Class 值记缺口；
名字来自形参时（`Src::Param`）按设计**不**与接收者镜像相乘，只记 `recv(param-name)` 缺口。

非单调来源：形参常量格 `pvals`（`PV::join`，两个不同字符串合流即 Top）在分析时把 Const 形参直接代成 `V::Str`，
与 ldc 字面量无法区分。`ObjectStreamClass$2.run` 依次以 `"writeObject"` / `"readObject"` / `"readObjectNoData"`
调 `getPrivateMethod(cl, name, …)`、以 `"writeReplace"` / `"readResolve"` 调 `getInheritableMethod`：

1. 首个调用点接入后 `pvals[name] = Const("writeObject")`，被调方按此分析，`@3 getDeclaredMethod` 的名字是 `V::Str`，
   按「本调用点字面量」与**当时**的接收者镜像集相乘，放开各镜像类的 `writeObject` 并记缺口；
2. 后续调用点接入，`pvals` 抬为 Top，被调方失效重分析，名字变为形参来源，站点不再满足相乘条件；
3. 第 1 步已放开的成员与缺口保留（效果只增不撤），之后接收者值集的增长不再触发相乘。

于是结果取决于第 1 步发生时接收者值集已长到多大：flow-batch 7 时已含 `java/lang/Class` / `Class#<synthetic>` 及
更多镜像（3081 个类），64 时较少（3040 个类）。插桩实测两次运行均只有 `writeObject`、`writeReplace` 两个名字
被相乘（各 1 次，即只在 Const 窗口内）；同一机制另见 `Proxy$Dyn.dispatchObject @5 "toString"`（仅 batch 7 出现）。
`pstrs.rs` 模块注释已指出 `pvals` 汇合格的顺序依赖并为名字集另建了单调的形参字符串集，但这条
「Const 形参被当成站点字面量」的路径仍走 `pvals`。

终态修法（待协调方确认，涉及序列化反射面）：名字与类都来自形参时，按**调用点逐个配对**——
每个（直接或经透传链的）调用点上该名字形参的字面量，与同一调用点上 Class 实参的镜像集相乘；
调用点新增、调用点的 Class 实参值集增长时重跑。这样 `writeObject` / `readObject` / `readObjectNoData` /
`writeReplace` / `readResolve` 五个名字都与 `cl` 的镜像集配对，与求值顺序无关；
`pvals` 代入的 Const 字符串不再作为站点字面量（站点字面量只认本方法内的 ldc 来源）。

## 后续步骤（规划）

1. 定位顺序依赖的判定：对 flow-batch 64 / 7 两次运行逐阶段对照，找出首个分歧的值集 / 判定，改为单调可重算；
   修正后的唯一不动点作为新的集合基准（需与协调方确认基准变更）。
2. 调用点实参端口：字节码调用点实参汇入单个端口节点，端口流向当前枢纽；换接子枢纽后撤去端口到祖先枢纽的冗余边
   （祖先经子 → 父实参链收到相同值）。已实现原型，集合在顺序敏感问题解决前无法做逐项验收。
3. open 枢纽去冗余：调用点 open 类型集里被其它 open 类型涵盖（子类型）的枢纽不再接入。
4. 精确集合枢纽的父枢纽从「本调用点原枢纽」推广为同成员族内最大的子集枢纽，消除平行的大扇出枢纽。
5. `hub_recv` 逐接收者克隆调用点表、`link_hub` 的 `special` 表收集排序。

## 现状（2026-10-04 交接：jndi-perf 分支上的套接字 / 网络 native 支线）

本支线起因是 TestSocketLoopbackPair main 线程 NPE，已停止，后续由协调方另派代理。分支上的提交按顺序如下：

| 提交 | 内容 | 验证（本机，只做 emit + compile） |
|---|---|---|
| 88971251 | InetAddress 退出 VM 边界，`<clinit>` 改为翻译字节码（NPE 根因）；生成类与手写伴生同名时，生成文件加 `_t` 后缀 | 已抽查 |
| e310dec2 | 套接字 native 层：Net / SocketDispatcher / UnixDispatcher / KQueue / EPoll / IOUtil / 套接字选项 / 名字服务；新增边界 e2e TestSocketErrorPaths | 抽查 jndiperf-e310dec2：1/6 |
| da1e88fb | DefaultProxySelector `init` / `getSystemProxies`：macOS 走 CFNetwork（含 PAC 展开），Linux 走 GIO，回落到 GConf；新增边界 e2e TestSystemProxySelector | 本机 macOS 冒烟通过 |
| 8e0fe281 | mod 树修正：叠层伴生 `x_impl_impl.rs` 只认生成宿主 `x_impl_t.rs`。修 HelloWorld E0432（e310dec2 引入） | HelloWorld 编译通过；新增单测 `stacked_companion_needs_generated_host` |
| ea33ff1e | NativeLibraries 的 `findBuiltinLib` / `load` / `unload`：内建库返回库名，否则返回 null；`load` 登记进程句柄 −1 与 JNI 1.8 | TestSocketErrorPaths 编译通过 |
| 36d728f3 | invokestatic：调用带 turbofish 时，形参里嵌套的被调类形参按 turbofish 实例化代换，修 `ExchangeImpl.createHttp1Exchange` 的 E0308 | ExchangeImpl 的 4 处错误清零 |

**未完成**：TestHttpLoopbackSync / Async 仍有 2 处 E0308，位置是 `sun/net/httpserver/UnmodifiableHeaders` 的桥方法
`replace(Object,Object)` 和 `replace(Object,Object,Object)`。
- 现象：UnmodifiableHeaders 中这两个桥方法的 wrapper 声明形参为 `Object`。而 Headers 自身没有声明 `replace`，它从 `Map.replace` 默认方法继承这个槽；生成 Headers 时按 `declared_by = java/util/Map` 代换签名，`K` 变成 `String`，所以槽签名是 `key: String`。wrapper 拿 `Object` 实参去调这个槽，类型不符。
- 根因：`TyCtx::method_sig_types`（`ty/src/sig_types/ctor.rs`）找覆盖链根声明时，只经 `ancestor_type_args` 走超类链。所以超类从接口继承的默认方法槽（Map → Headers）找不到根，子类桥方法只好按自身描述符擦除成 `Object`。
- 候选修法：找根时把各祖先的超接口闭包也纳入，每个接口带上相对本类的实参映射，取最远的声明者。这样根的选择与 phase2 继承槽的 `declared_by` 同源，子类的覆盖 / 桥方法与槽签名一致。修完后需要再 emit + compile TestHttpLoopbackSync，并用 `compare_trees.sh` 对照验收集的生成树，确认变化只出现在这类桥方法上。

**另派代理承接**：e310dec2 之后余下的 22 个 native-missing（DefaultProxySelector 已由 da1e88fb 补上）；修法 B。

### 2026-10-04 续：TestJndiNoProvider 回归排查与 VarHandle getAndBitwise*

**TestJndiNoProvider（抽查 jndiperf-06394403 报 `could not compile java_runtime (lib)`）本机未复现**，排查如下（均在 06394403 上）：

| 检查 | 结果 |
|---|---|
| macOS JDK emit + `rava compile`（dev，全 workspace 含 user bin） | 通过（3624 JDK 类，9m15s） |
| 同一 scratch `cargo check -p java_runtime --target x86_64-unknown-linux-gnu` | 通过 |
| Linux JDK 模块 emit（`--java-home …/linuxjdk/hybrid`，3665 类，含 EPoll / LinuxSocketOptions / LinuxFileSystem）+ Linux 目标 `cargo check -p java_runtime` | 通过 |
| 同上 + Linux 目标 `cargo build -p java_runtime`（`CARGO_INCREMENTAL=0`，含代码生成） | 通过 |

- d314fda7..06394403 之间没有依赖变更（`Cargo.toml` / `build.rs` 未动）；闭包流传播批量为常量，同一 JDK 输入下闭包确定。
- 只有 `could not compile … (lib)` 而无 `error[E…]` 诊断块，形态更像 rustc 进程被外部终止（`Caused by: … signal: 9 / 11`）或工具链差异，
  而非生成代码类型错误；`failure_extract` 只收汇总行，`Caused by` 行不入摘要，日志清理后无法区分。
- 结论：本分支不需要为此改生成器；**请协调方重跑 TestJndiNoProvider 并保留 `build/test_jndi_no_provider/logs/build.log` 全文**
  （重点看 `Caused by` 行、`__RAVA_OOM_KILL__` 标记、服务器 `rustc --version`）。若确为 OOM，属规模问题（本分支 InetAddress 退出 VM 边界与
  套接字 native 层让闭包增大），候选方案是按 `java_runtime` 声明层峰值调整服务器内存预留或继续拆分声明层，而非回退功能。

**VarHandle.getAndBitwiseOr native 缺失（2f8ec056）**：
- 根因：签名多态模型本身覆盖全部 access mode——生成器（`instr/src/sim/methods.rs::gen_signature_polymorphic`）把调用点实参装进
  `Object[]`，落到 VarHandle 上 `ACC_NATIVE | ACC_VARARGS` 的声明方法，体在手写伴生 `var_handle_impl.rs`（类别①，与 getAndAdd 等同一机制，
  `vm_intrinsics.toml` 只为引用写入 / 读取登记事实，基本类型位运算无需登记）。缺口只在手写伴生：get / set / CAS / getAndSet / getAndAdd 族已实现，
  getAndBitwise{Or,And,Xor}{,Acquire,Release} 九个方法缺失。
- 修法：九个方法整组补齐。字段经 Unsafe 统一载体 `__vh_prim_update` 一次读-改-写；数组元素在存储写锁内读-改-写；布尔 / 整数族支持，
  引用 / 浮点族抛 UnsupportedOperationException。字节数组视图族（`VarHandleByteArrayAs*`）与 getAndAdd 一样暂不支持位运算（无消费方）。
- 边界 e2e：`tests/e2e/48_refs/TestVarHandleBitwise.java`（实例 / 静态字段、int[] / long[] / boolean[]、子字字段互不影响、不支持族），
  本机 emit + compile 通过，单跑输出与 JDK 一致；TestSocketErrorPaths emit + compile 通过、native_missing 不再含 VarHandle；HelloWorld 编译通过；generator 单测 386 通过 0 失败。

## 现状（2026-10-04：http-perf 分支，TestHttpLoopbackSync / Async 转译超时）

### 二分（服务器 `remote_rava emit --perf`，各点服务器不同，耗时只作量级参考）

TestHttpLoopbackSync：

| 提交 | 服务器 | emit jdk_classes | 闭包耗时 | 峰值内存 |
|---|---|---|---|---|
| 601b0e3b | sg1 | 3590 | 130 s | 3.0 GB |
| 488b5811 | us1 | 3590 | 115 s | — |
| 51a4d8c5（c1d-p0 JCA） | kr1 | 5404 | 433 s | 7.4 GB |
| 6825d639 | jp2 | 5404 | 424 s | — |
| 35c5f0ee | kr1 | 5465 | 707 s | 11 GB |
| 90398dc8 | jp1 | 5465 | 738 s | — |
| fca1643b | kr2 | 5465 | 743 s | — |
| 3e0199ec | ubuntu | 5772 | 1110 s | 14.7 GB |

DeepCopy（闭包类 / 方法）：601b0e3b 3153 / 18917 → 488b5811 同 → 51a4d8c5 3329 / 19688 → 35c5f0ee 3407 / 20871
→ 90398dc8 / fca1643b 3407 → 3e0199ec / 3541dc13 3436 / 21020。无单次合入膨胀；协调方所说「4131」是另一口径
（emit 计数），闭包口径未复现。

### 根因

- 规模跃升点是 51a4d8c5（c1d-p0 JCA：SunJSSE / SunRsaSign 注册）。HttpClientImpl 调 `SSLContext.getDefault()`，
  JSSE / JCA 入闭包是正确的，**不是** URL 协议（c1d-urlhost）或序列化收窄（c1d-t2b）问题。
- 耗时是引擎对规模的超线性：本机 3541dc13 闭包 5428 类 / 33929 方法，404 s、峰值 RSS 7.07 GB；
  枢纽 119 719 个、（调用点, 枢纽）接入 11.0 M、单调用点最多 620 个枢纽；`S→HP` 推送 1.65 G 次；
  `add_to_id` 3.69 G 次，其中有增量 102 M 次、插入元素 1.56 G；阶段 flows 163 s、sites 114 s、process 99 s。
- 枢纽转储：`Object.equals` 族 1931 个大枢纽、424 个根（无父），其中 408 个根有更早的子集枢纽覆盖约 95% 接收者——
  父枢纽只取「本调用点原枢纽」，同成员其它调用点上的大子集枢纽无法复用，形成平行大扇出。

### 已做（集合保持，本分支提交）

| 改动 | 位置 |
|---|---|
| A：精确集合枢纽按（成员, 接口调用）分族，族内按集合大小升序登记；新枢纽在族内找最大的已展开子集枢纽为父（最多试 16 个，至少不劣于本调用点原枢纽），只展开差集 | `engine/hub.rs`（`hub_parent` / `sorted_subset` / `sorted_minus`）、`engine.rs`（`hub_family`） |
| B：同一值集里 `open(o)` 被另一 `open(p)`（`o ⊂ p`）涵盖时只接后者（G 中 ⊂ o 的接收者全在 ⊂ p 内，目标与结果相同） | `engine/invoke.rs::invoke_inner` |
| C：枢纽 `recvs` 只存本枢纽自身展开的接收者，不再复制父枢纽的（父链由报告侧按链遍历）；枢纽记 `set` 供族内子集判定与标签 | `engine/defs.rs`、`hub.rs`、`report.rs`、`stats.rs` |

本机 TestHttpLoopbackSync（同一 3541dc13 基线、串行冷闭包）：

| | 基线 | A+B | A+B+C |
|---|---|---|---|
| 耗时 | 404 s（墙钟；争用下 sys 59 s） | 274 s | 285 s |
| 峰值 RSS | 7.07 GB | 7.46 GB | 6.39 GB |
| 枢纽 / 接入 / 单点最多 | 119 719 / 11.0 M / 620 | 64 501 / 2.15 M / 159 | 同左 |
| `add_to_id` 调用 / 有增量 / 插入元素 | 3.69 G / 102 M / 1.56 G | 1.41 G / 102 M / 1.57 G | 同左 |
| `S` 源推送 | 1.80 G | 280 M | 同左 |
| 类 / 方法 | 5428 / 33929 | 相同 | 相同 |

`closure_bench.sh --diff`：类集 / 方法集逐项相同；差异只在顺序噪声字段（folds、`reflect.gaps` 多 2 条
`ObjectStreamClass.getPrivateMethod` recv 缺口、method_contexts、rcall 统计），与基线二进制仅改 `--flow-batch 1024`
得到的差异同类（该对照 278 s，类 / 方法相同，folds / contexts / rcall 不同），属既有顺序依赖，非本改动引入。

其它用例（同机串行，基线 → 本分支）：DeepCopy 39.4 s → 35.1 s、StockTrans 39.2 s → 32.8 s，类 / 方法 / folds 逐项相同，
只差 `summary` 里的 method_contexts（±50）、context_objects（±1）、rcall 池计数（±1）这类顺序噪声；HelloWorld 闭包逐字节相同。

### ≤60 s 目标评估

A+B+C 之后无效推送已基本去掉，剩余耗时由**有效**工作量决定：1.56 G 次元素插入、221k 方法上下文（对象敏感
`HEAP_DEPTH=2`）。集合保持的引擎改造在本机最多再压到约 150–200 s 量级，达不到 ≤60 s（本机约 33 s 对应服务器 60 s）。
达标需要改变上下文 / 精度机制，属终态设计决策，需协调方定：
1. 逃逸对象上下文收拢：存入全局可达容器（ConcurrentHashMap / 注册表）的对象，其堆上下文折叠为单一无上下文池，
   不再按分配点 × 接收者上下文复制；预期上下文数与元素插入同比例下降。
2. 值集稀疏表示的批量插入（按枢纽一次合并有序块）——只降常数。
两者都可能改变 folds 等精度字段，需先完成下节的顺序无关性，才能用「类 / 方法集逐项相同」做验收。

### 顺序无关性

**保证（本分支 A / B / C）**：
- A 的父枢纽选择依赖族内登记顺序，但只影响枢纽树形，不影响任何接收者的目标：子枢纽 = 父枢纽目标 ∪ 差集展开目标，
  父枢纽本身对其集合完备（`expanded && pending.is_empty()` 才可选为父），所以每个调用点得到的目标集只取决于该点的接收者集。
- B 是纯值集函数（同一值集内的子类型关系），与到达顺序无关；值集增长时 open 集合只增，被涵盖的 `open(o)` 的目标始终
  ⊂ 涵盖者的目标，不存在「先接后撤」。
- C 只改存储，报告侧按父链取并集，结果相同。
- 以上三项均为单调更新（集合只增、链接只增），不引入新的顺序依赖。

**既有非单调点**：
1. 已查实：`pvals` Const 形参被当成站点字面量（见上文「根因（已查实）」），Const→Top 时已放开的反射成员不撤回。
2. 1b 宏形式回归探针（a8c386e4，即 10cfb657^ 的 meta.rs 宏访问器形式），TestSerialLookupPairing：
   seed 0 → 3360 类 / 20715 方法，seed 2 → 4132 类 / 24938 方法，seed 0 严格 ⊂ seed 2。
   - seed 2 独有：com/sun/crypto（219 类）、jdk/internal/org/jline（150）、sun/security/ssl（81）、Process*、PolicyFile 等；
     `reflect.members` 多 393 条（JCA 服务实现构造器），`reflect.allocations` 多 83 条，`gaps` 多 11 条（含
     `PolicyFile.getInstance` 的 `Class.getConstructor`）。
   - `--why javax/crypto/JceSecurity`（seed 2）：KeyFactory.nextSpi → Provider$Service.newInstance → 反射 getImplClass →
     PolicySpiFile → PolicyFile → PKCS12 → Mac → JceSecurity，即服务实现类值集放大后不收回。
   - folds 分歧：seed 0 把 `ServiceLoader.layer`、`ProviderList$PreferredList.getAll` 的 List 字段折成 null，seed 2 未折，
     说明 seed 0 中这些字段从未写入非空，seed 2 中经过放大后的路径写入了。
   - 已排除：keyed 门（`keyed.rs`）退化为 `Any` 的事件两种子完全相同（各 6 条，插桩比对），不是分歧点。
   - 与协调方背景一致：`ProviderConfig.doLoadProvider` 返回值经 `ProviderConfig.provider` 字段回流；推断仍是
     「瞬时更悲观的状态产生效果后不撤回」这一类（同非单调点 1 的模式），具体判定位置尚未定位。

**修法（终态）**：所有「按当时状态一次性决定」的判定改为对其输入单调：
- 站点字面量只认本方法 ldc 来源；形参来源的名字按调用点逐个配对（上文方案），输入增长时重跑。
- 对 2：在探针上做逐阶段对照（seed 0 / 2 各转储每轮 `doLoadProvider` 返回值集、`ProviderConfig.provider` 字段值集、
  `Provider$Service` 实现类值集），找出首个「seed 2 有、且其来源输入在 seed 0 终态中不存在」的值，即瞬时状态的产物；
  将产生它的判定改为随输入重算（或只用单调格：Const 合流只升不降，且依赖 Const 的效果登记为该形参的读者，Top 时作废重放）。
- 验收：DeepCopy / StockTrans / TestSerialLookupPairing / TestHttpLoopbackSync 在 seed 0/1/2 下类集 / 方法集相同，
  扩展 `closure_cli::closure_independent_of_hash_seed`（慢，经 heavy_lock 跑）；探针（a8c386e4 宏形式）seed 差异消失。
  该定位超出本步范围，按协调方指示先记于此、另行派步。

### 下一步

1. 顺序无关性（上节修法），完成后以唯一不动点作为新集合基准。
2. 上下文收拢（≤60 s 的必要条件），需协调方确认精度口径。
3. 测量：本分支提交上 `remote_rava emit` 两个 HTTP 测试（结果补记于此）。
