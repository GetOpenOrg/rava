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

### 2026-10-04 续：Unsafe 引用 RMW 缺口与桥方法 E0308（分支 unsafe-rmw）

**Unsafe / VarHandle 字段读-改-写落空（TestSocketErrorPaths / TestSocketLoopbackPair panic `unsafe__impl.rs:195`）**
- 根因：偏移登记表记的是 **Java 字段名**，而对象的 by-name 协议（`__unsafe_ref_access` / `__unsafe_word` 等，java_class! 宏按扁平字段生成臂）
  以 **Rust 字段名**为键。凡生成器改过名的字段都会落空：关键字（`Socket.in` → `in_`，JDK `Socket.getInputStream` 用
  `IN.compareAndSet(this, null, in)`）、含 `$`（`this$0` → `this_0`）、遮蔽祖先同名字段（加 `_{SimpleClass}` 后缀）。
  引用族与基本类型族同样受影响，不是「缺引用臂」。
- 修法（生成器 + 宏，无类名特判）：
  - 生成器在类块头发射 `#[field_slots = "decl.java=rust;…"]`，只列 Java 名与 Rust 名不同的实例字段（本类 + 扁平祖先，`class_writer/fields.rs`）。
  - 宏据此生成 `ObjectVTable::__field_slot(decl, name) -> Option<&'static str>`（`struct_layout.rs`；包装层转发）。
  - 偏移表改存（声明类, Java 名）；`unsafe__impl.rs::offset_slot` 经对象的 `__field_slot` 换成 Rust 名，引用族与基本类型族（`unsafe__ext.rs`）共用。
    遮蔽字段按声明类区分，两字段互不影响。
- 边界 e2e：`tests/e2e/48_refs/TestVarHandleRefRmw.java`，覆盖关键字 / `$` / 遮蔽字段、静态、`Object[]` / `String[]` 元素、关键字名基本类型字段、
  AtomicReferenceFieldUpdater、两线程 CAS 计数。期望输出取自 JDK 21.0.11。本机 emit + compile 通过，单跑输出与 JDK 一致。
- TestSocketErrorPaths：本机 emit + compile 通过，单跑输出与 expected 一致。`docs/known_failures.toml` 中两条登记已删除。

**UnmodifiableHeaders 桥方法 E0308（TestHttpLoopbackSync / Async）**，两处根因：
1. `TyCtx::method_sig_types`（`ty/src/sig_types/ctor.rs`）找覆盖链根时只走超类链。已补 `ancestor_default_slot`：沿超类链从最远处起，对每个祖先的超接口闭包做广度搜索，
   找同名同描述符的 default；该祖先比所有类声明都远时，以这个 default 为根，实参取 `implemented_interface_views(ci)`，与 phase2 注入槽位的 `declared_by` 同源。
   改后桥 wrapper 签名为 `(String, Object)`，与 Headers 槽位一致。单测 `method_sig_types_roots_at_ancestor_inherited_default`。
2. `override_vtable_erasure`（`emit/class_writer/slot.rs`）在槽位由接口 default 继承而来时，按接口自身形参（K / V）判断 Object 化位置，误把 `String` 列入
   `vtable_erasure`，覆盖条目擦成 Object，与 Headers 槽位的 `String` 不符。改为与 owner 发射同源：先用 `adapt_interface_method` 把接口形参代换成 owner 视角的实参，
   再按 owner 自身形参判断。
- 失败路线：只修 1 时，E0308 从 wrapper 调用处转移到 vtable 覆盖条目（仍 2 处）。
- 两处修完后，java_runtime 编译通过，暴露 body crate 里下一处 E0308：`DnsClient` 中 `AtomicLong::new` 适配 `IntFunction`，
  metafactory 的基本类型拓宽（I → J）未做，实参原样传给 `new_l(i64)`。`instr/sim/dynamic/lambda_args.rs::adapt_sam_arg` 补拓宽臂：
  SAM 形参与实现形参都是基本类型、互不相同且非 boolean 时生成 `(x as T)`（Rust `as` 与 Java 拓宽同义）。

**验证（本机，最终 rava）**：generator 单测 425 通过 0 失败（含 `jdk_literal_lint` / `no_jdk_literals`），rava_macros_core 单测 11 通过；
TestVarHandleRefRmw emit + compile + 单跑与 JDK 一致；TestSocketErrorPaths 单跑与 expected 一致；TestHttpLoopbackSync emit + compile 通过（0 错误）。
单跑 TestHttpLoopbackSync 时止于 `native: sun/nio/ch/IOUtil.makePipe:(Z)J`，属已另派的 native-missing 余项（修法 B），不在本分支。

**下一步**：分布式抽查 TestSocketErrorPaths、TestSocketLoopbackPair、TestVarHandleRefRmw、TestVarHandleBitwise、TestByteArrayViewVarHandle、
TestSubwordFieldCas、TestHttpLoopbackSync、TestHttpLoopbackAsync。slot.rs 的改动影响所有「子类覆盖祖先经接口 default 继承、且实参具体化的槽位」，
抽查时应留意 Map / Collection 系子类。

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
3. 两个 HTTP 测试仍超 600 s 转译上限，在 1、2 落地前不会转绿。

### 服务器测量（0220249a，`remote_rava emit --perf`）

| 测试 | 服务器 | 闭包阶段 | 峰值 | emit jdk_classes | 对照 |
|---|---|---|---|---|---|
| TestHttpLoopbackSync | jp2 | 630 s | 8.3 GB | 5472 | jp2 上 6825d639（5404 类）为 424 s |
| TestHttpLoopbackAsync | ubuntu | 607 s | 9.1 GB | 5467 | ubuntu 上 3e0199ec 为 1110 s / 14.7 GB |

同一台 ubuntu 上闭包耗时降 45%、内存降 38%，但仍高于 600 s 上限。

## 顺序无关性修复（2026-10-04，engine-order 分支）

### 已确认结论

单调要求：效果只增不撤，故每个判定须满足「乐观（瞬时）状态下的效果 ⊆ 悲观（终态）状态下的效果」。
按此口径排查到三处判定，都已改正：

| # | 类别 | 位置 | 现象 | 修法 |
|---|---|---|---|---|
| 1 | 提前截断（推不出即丢已知名字） | `pstrs.rs` 形参字符串槽 / `class_lookup.rs` 按名取类 | 槽的上游有一路推不出时整槽返回 None，已知名字一并丢弃；瞬时窗口里槽完备、按名加载了的类在终态（推不出）下不再加载，探针 TestSerialLookupPairing 随种子在 3360 / 4132 类之间摆动 | 「推不出」改为标志位（`lookup_partial`，只升不降），已知名字照常产出、另记缺口；`slot_upstream` / `slot_names` / `param_inputs` 返回完备标志，`partial_part` 按缺口映射不完备的输入 |
| 2 | 常量字面量化（Const 代入无来源） | `absint` 的 `V::Str`、`invoke.rs::reflective_writes`、`field_lookup.rs` | 形参常量（`pvals`）、返回常量、字段常量代入为 `V::Str` 后与 ldc 字面量无法区分，被当成站点字面量与接收者镜像集相乘；Const→Top 后已放开的反射成员不撤（上文「根因（已查实）」） | `V::Str` 带来源集（`Srcs`）：ldc 为 `Src::Str(lit_id)`；格值代入时改记读点来源（`Param(k)` / `Site(off)`），与悲观态下的 `Ref` 同源；同文本合流取来源并集；`PV` 存储形去掉来源。只有非派生字符串算站点字面量（`site_lits` / `derived_str`），派生字符串走形参字符串集逐调用点配对 |
| 3 | 先到者决定（入口常量不并 Top） | `worklist.rs::open_params` | 反射 / VM / 种子入口只按声明类型 open 形参，不并入形参常量；「分析时尚无调用点记录才置 Top」的兜底只在该入口先到时生效。DeepCopy 中 `LDAPCertStore.<init>` 先经 `JdkLDAP$ProviderService.newInstance` 以 null 实参入链时 `pvals = Const(null)`，后到的反射构造器入口不抬 Top，`@22` 之后全被折死，`LDAPCertStoreParameters` / `URICertStoreParameters` 两类随种子出现 / 消失 | `open_params` 显式 `bind_pvs(t, 0, n, None)`；兜底只剩无实参值可言的入口（`<clinit>`、序列化分配的无参构造器、上下文克隆的 lambda 实现 / 具体求值节点） |

另：按名放开字段（`reflective_writes` 的 Class 实参 / 接收者分支）对引用值与派生字符串另取形参字符串集
（`param_strs`）与字段字面量集（`field_strs`）：修前只在形参常量窗口内按文本放开，形参抬为 Top 后同一来源不再给出名字。

上限类判定（`MAX_SLOTS` 清输入、`MAX_NEST`、`MAX_NAMES`、`MAX_PATTERNS`、sealed 名字上限）同样是按当时规模截断，
已加计数器（`summary.perf.cap_hits`，只列非零项）实测其在验收用例中是否触发，见下表。

### 失败路线

- 单值来源 `Option<Src>`：同文本不同来源的字符串合流只能取 None（丢来源），HelloWorld 多出 32 类。改为来源集并集后消除。
- 「不完备时不产出名字」：完备标志只升不降，完备窗口里产出的效果在终态（不完备）下没有对应效果，不单调；
  唯一与判定 1 相容的单调选择是不完备时照常产出已知名字并记缺口。

### 集合变化（相对 c5741dfe 正常形态）

DeepCopy 3388 → 4911 类，StockTrans 3386 → 4907 类（方法约 20.9k → 31.0k）。增量（JCA 提供者实现、xerces / XMLDSig、SSL、
反射访问器等约 1500 类）全部来自 `Provider$Service.getImplClass` 的 className 按名取类：修前该槽在瞬时窗口外恒为推不出、
整槽丢名不加载；修后按已知名字加载。这些名字正是探针大种子（瞬时完备窗口）加载的那批，即修复非单调后恢复的类。
基线独有 `java/lang/ref/FinalReference`（`ReferenceQueue.poll0` 的 instanceof 类型级引用）：该分支是否可达取决于队列 `head` 的值集，
而 `head` 的来源正是下文「未修：反射调用池去冗余与 open 目标写入」的顺序依赖，不能定性为纯折叠精度变化（早先结论更正）。

代价：DeepCopy / StockTrans 冷闭包 34 s → 约 155 s（本机）。

### 合并集成分支后的门禁（V9）

- 2bd6f6bf 合入 3e739189（crate-split），0d392c7a 合入 21fc601b（M1 + S7）。
- 集成分支 3e739189 上 `closure_independent_of_hash_seed` 的 TestSerialLookupPairing 差异（种子 1 多出 AESCipher 一族）：
  入链点是 `Provider$Service.getImplClass @64` 的按名取类（closure JSON `via.kind = reflect`），即上表判定 1
  （形参字符串槽上游有一路推不出时整槽丢名）。本分支上该槽照常产出已知名字，TSLP 种子 0 / 1 / 2 均含 AESCipher 一族
  15 类（4934 类，三种子类集合相同）。S7 合入后差异消失只是手写层变化扰动了遍历顺序，判定 1 本身仍在集成分支上；
  根因不在手写扫描器或宏，不需要补 macro_fn_lint 类守护。
- HTTP（服务器，ca2488f0）：种子 0 冷闭包 1450 s（基线 c5741dfe 630 s），种子 1 / 2 在 12 GB 下 OOM——判定 1 修正后
  JCA / SSL 提供者实现按名入链带来的规模增长，代价问题待评估。

### 未修：反射调用池去冗余与 open 目标写入（剩余顺序依赖）

现象：DeepCopy / StockTrans / TSLP 类集合已三种子一致，方法集合仍随运行在 0–13 个方法间摆动，全部经 `ReferenceQueue.poll` /
`remove` 的结果可达：`LocaleResources$ResourceReference.getCacheKey`、`ResourceBundle$KeyElementReference.getCacheKey`、
`Bundles$BundleReference.getCacheKey`、`FileCleanable.performCleanup` / `cleanupClose0`、`Provider.implPutIfAbsent` 等。
另：引擎外的 std `HashMap`（RandomState：handwritten* / manifest* / seeds / jca / locale / services / `engine/seeds.rs` /
`cut.rs` / `hw_inherit` / loaders）使同一 `--hash-seed` 的两次运行顺序也不同，复现需多跑几次。

根因（`--flows '@path:…|open:java/lang/ref/Reference'` 与 `@grow` 查实）：

1. 闭包内没有 `ReferenceQueue.enqueue`，`poll0` 的 `head = (rn == r) ? null : rn`（`rn = r.next`）是自环，`O(q, head)` 的初值只能来自外部。
2. 好的运行里初值来自 `VarHandleReferences$FieldInstanceReadWrite.compareAndExchange @44 → Unsafe.compareAndExchangeReference`
   的写入值（目标实参 = 方法句柄通道实参池），经 `hw_site_fields` 逐对象接到 `O(q, head)`（`head` 按名放开，偏移可写）。
3. 目标实参来自 `Node::RN`（实参池去冗余视图，`reflect_call.rs::rcall_absorb`）：已逃逸对象 x 若属于池中某 open 类型 o 就不列出。
   去冗余的前提是「open 视图与逐个列出结果一致」，但 `hw_site_fields` 对 open 目标只写 o 自身的引用字段（子类字段不可枚举），
   逐个列出的 x 则写它的全部字段。q（`ReferenceQueue`）先于 `open(Object)` 入池就被列出、`head` 被写入；后于它入池就被涵盖、
   `head` 不被写入——结果取决于入池先后。

试过的两条修法（均正确但代价不可接受，已撤回）：

- 去冗余只在 x 于 o 之外的引用字段都不可按偏移读写时才涵盖，字段之后放开时补列（单调）：`head` / `next` 等常用名按名放开，
  绝大多数对象被列出，DeepCopy 冷闭包 > 10 min 未完成（修前约 3.5 min）。
- open 目标写入另接全部已逃逸子类的引用字段（使 open 视图真正涵盖逐个列出）：Unsafe / 句柄解释器的大值集经 `U` 汇入全部
  已放开字段，同样 > 10 min 未完成。

### 交接 / 下一步

终态方向：让「open 目标写入」与「去冗余」共用同一个等价定义，且不引入大值集。建议从写入点收窄入手——差异只出在偏移不是符号偏移的
未收窄站点（`FieldInstanceReadWrite.*` 的偏移来自 VarHandle 对象的 `fieldOffset` 字段），按 VarHandle 对象的字段来源
（字段句柄来源标记 `fh_marks` / `field_handles.rs`）把站点收窄到句柄所指字段：收窄站点上逐个列出与 open 涵盖结果相同
（`site_field_nodes` 已对二者同口径），去冗余随之与顺序无关，Unsafe 写入也不再撒到目标对象的全部引用字段。
收窄覆盖后再复查剩余未收窄站点（FieldReflector 等）是否仍有同类差异。验收照旧：DeepCopy / StockTrans / TSLP（正常 + 探针）
与 HTTP 种子 0 / 1 / 2 类 / 方法 / 反射集合一致。

### 合并 21fc601b 后的生成器单元测试（到时限的 WIP 状态）

- `cargo test --release --no-fail-fast` 全量结果：除下面两项外全部通过，包括 jdk_literal_lint、no_jdk_literals 与 closure 单元测试 148 例。
- `driver/tests/build_cli.rs` 原有 2 例失败：`proxy_interface_owner_not_opaque` 和 `locale_bundle_parent_not_null_recv`。
  - 报错在发射阶段：`XMLDTDScannerImpl.scanDTDInternalSubset:(ZZZ)Z` 的 CfgAuditError「结构树与活块集合不一致」，live 比 tree 多出块 14。
  - 触发点：本分支的档案给出 `dead_pcs [[166,168]]`，即形参 `complete` 恒为真。JDK 内唯一的调用方传的是字面量 `true`，这个折叠本身成立。
  - 规整后，`do { if (!scanDecls(complete)) {…return false;} } while (complete)` 变成一个循环：循环头没有语句，跳转臂是空回边块 162（`iload_1; pop; goto 87`），出口臂则直接 break。
  - 根因在结构化器 `cfg/src/simplify.rs::pass_while`：它把 `loop { if c { break } else { <空回边块> } }` 改写成 while 形态时，整条 if 都被删掉，臂里的空回边块跟着丢了。
  - 修法：把两臂中的无语句块原位留在循环体内。新增单元测试 `while_guard_keeps_empty_latch`；修后 build_cli 14/14 通过。
- `closure_cli::closure_independent_of_hash_seed` **仍失败**：StockTrans 种子 0 比种子 1 多出方法
  `LocaleResources$ResourceReference.getCacheKey`。这正是上一节「未修：反射调用池去冗余与 open 目标写入」所述的剩余顺序依赖，按「交接 / 下一步」继续处理。

### 2026-10-04 续：重载撞名修复与第三次 open 写入尝试（engine-order，阶段 2）

- 5760b43a30eddbd4d11aba6694fad76fe0bb1e66：重载名修饰区分完整数组维数（`ty::type_map::descriptor_to_suffix` 与
  `closure::handwritten::descriptor_suffix` 每维重复 `arr_`）。修前 `String[][]` 与 `String[][][]` 同为 `arr_str`，
  `DTDGrammar.resize` 两重载声明去重为 `_1` 而调用点仍用原名，致 6 例 E0308（DeepCopy / StockTrans / TestUrlProtocolOpen /
  TestSerialDefaultSuid / TestSerialLookupPairing / TestSerialProxyForm）。本机 StockTrans `--stop-after emit` 复核声明与调用点均为
  `resize_arr_arr_str_i` / `resize_arr_arr_arr_str_i`。同提交加 `--flows '@hwopen'` 诊断：列出未收窄、写字段的 hw 站点的
  目标规模、open 类型与写入规模。
- `@hwopen`（DeepCopy 种子 0，终态）：未收窄且目标含 open 的站点 57 个。主要有三类：
  - `VarHandleReferences$FieldInstanceReadWrite.*` 及 `Unsafe.*Acquire/Release/Plain` 转发：目标 186 + open（含 `Object`），写入 186+351 open；
  - `ObjectStreamClass$FieldReflector.setObjFieldValues`：目标 6045，写入 7943+351 open；
  - CLQ / LTQ / Striped64 的 VarHandle 调用点：目标 ≤ 6，规模小。
- 第三次尝试（未提交，补丁留在本机 `build/logs/open_writes_wip.patch`）：open 目标 o 按 (o, 口径) 分组，接到 o 子类型声明的、
  按口径可写的引用实例字段 `U(f)`，字段新登记或开放时补接（单调）。DeepCopy 种子 0 跑了 32 min 仍未完成（基线 172–214 s；
  本机负载 12，但量级不可接受），已撤回。这与上文「失败路线」第二条本质相同：写入值经 `U(f) → F(f)` 汇入未知接收者读，代价不可接受。
  按「同一问题两轮修复不过即停」，V9 剩余顺序依赖在此停下。
- 终态方向不变，见上文「交接 / 下一步」，补两点：
  1. 不能再从「open 写入扩到子类型字段」入手，必须先收窄偏移来源。`FieldInstanceReadWrite` 的偏移来自字段句柄，
     口径应是「只经字段句柄 / MemberName 取得的字段」，与 `Gate::Handle` 同口径，不含反序列化放开的字段。可在清单里增设
     一类「字段句柄访问器」成员，与 `handle_interpreters` 分开：后者还带值池语义，不宜混用。再按 `fh_marks` 收窄到句柄所指字段。
  2. `Unsafe.*Acquire` 等转发方法的调用点是共享的，目标是各调用方的并集，需按调用方上下文克隆，或者把转发方法看作透明转发；
     否则收窄效果会在转发点丢失。
- 门禁现状：`closure_independent_of_hash_seed` 仍失败，StockTrans 种子 0 / 1 差 1 个方法
  （`LocaleResources$ResourceReference.getCacheKey`）。其余生成器单元测试全过（t1.log：closure 148、build_cli 14/14 等）。
- TestHttpLoopbackSync 转译超时排查（协调方抽查 order-5760b43a 在 jp2 上 10 min 超时；order-27e33897 上同样超时）。
  本机经 heavy_lock 跑 `rava build … --stop-after emit`，两个独立 worktree 各自构建，负载相近：

  | 提交 | 耗时（real） | JDK 类 | 峰值内存 |
  |---|---|---|---|
  | main e519e22c | 399 s | 5441 | 9.2 GB |
  | engine-order 5760b43a | 995 s | 6420 | 15.6 GB |

  确认是本分支变慢：多出 979 类，耗时 2.5 倍。来源与上文「集合变化」相同：判定 1 修正后，`Provider$Service.getImplClass`
  按已知名字加载 JCA / SSL 提供者实现。这些是修复非单调后恢复的类，不是噪声，所以不能靠撤回判定 1 来提速。
  要做到「不慢于 main」，需要在保持集合不变的前提下给引擎提速，或者让提供者按名取类的精度提高
  （只取实际被请求的算法，不取整张提供者表），属于另立的工作项。本轮未做。

### 2026-10-04 续：按名取类延后放行，闭包与哈希种子无关（engine-order，阶段 2 收尾）

协调方更正：main（e519e22c）自身的 `closure_independent_of_hash_seed` 也会失败。TestSerialUserGenericCallbacks 在种子 0
下 4167 类，种子 1 下 3391 类，差的是 JCA / SSL 提供者类。合入判据改为：① 本机 TestHttpLoopbackSync / TestJndiNoProvider
的转译耗时和类数都不高于 main；② 该单测稳定通过。

**根因**：非字面量的按名取类站点（`Class.forName(x)` 等）在分析中途可能「名字齐全」，到不动点时却变成推不出（某支变成
无约束任意串，或者形参 / 字段名字集不完备）。旧口径在中途齐全时就按名加载，加载结果单调保留，所以闭包取决于求值先后，
也就是取决于 HashMap 迭代顺序。种子 0 下 `Provider$Service.getImplClass` 在名字集还只含少量提供者时被求值并放行，
之后整张提供者表都按名加载（+776 类）。

**终态做法**（与 `seed_ctor_lookups` 同一思路，只在不动点上作判定）：

1. **延后放行**（`engine/class_lookup.rs`、`worklist.rs`）：
   - 名字是字符串常量的站点直接解析。
   - 非字面量站点求值齐全时，先挂起（`lookup_pending`），返回空名字集。
   - 工作队列排空时，`lookup_release` 把挂起站点重跑一次（`lookup_trial`）。仍齐全就放行（`lookup_released`），
     之后按单调口径照常求值。
   - 求值中出现无约束任意串，或 `Gap::Fail`，站点记为不确定（`lookup_unsure`，单调）。不确定站点永不按名加载，
     结果接所指未知的 Class（top）。
   - 空闲钩子顺序：`seed_round` → `lookup_release` → `nr_drain`。
2. **JCA 服务实现类的反射构造点**（`seeds.toml [jca] instantiation_hosts`，`seeds/jca.rs`）：
   `Provider$Service.getImplClass` 恒为 top，不按类名字段的字符串集解析。所指的类由 JCA 规则按被请求的算法补种。
   这样提供者类不会因为这个站点被整表纳入。
3. **字段名配对只取声明为 `String` 的形参**（`engine/invoke.rs` `reflective_writes`）：
   - 修前，凡是带 `Class` 形参的调用，任意引用实参都被当成字段名。例如 `HashMap$TreeNode.find` / `putTreeVal` →
     `compareComparables(Class, Object, Object)` 的映射键。
   - 结果是映射键上的全部字符串常量都按名放开字段，包括 `config`。这会展开 `ReflectionFactory.config`，多纳入约 82 类：
     `jdk/internal/reflect/Unsafe*FieldAccessor`、`VarHandle*` 等。
   - 字段名配对登记为 `LookupWrap { field: true }`，在调用方经 `field_wrap_call` 按「类 × 名」配对。
4. **配对去重**（`fpair_done`）：同一（类，名）只放开一次。修前 `TreeBin` / `TreeNode` 调用点上反复重放，单例 CPU 27 min 以上不收敛。

**多种子结果**（`rava closure`，种子 0 / 1 / 2；类 / 方法 / 反射）：

| 用例 | 种子 0 | 种子 1 | 种子 2 | 一致 |
|---|---|---|---|---|
| StockTrans | 3386 / 20860 / 831 | 同 | 同 | 是 |
| DeepCopy | 3388 / 20879 / 842 | 同 | 同 | 是 |
| TestSerialDefaultSuid | 3393 / 20873 / 846 | 同 | 同 | 是 |
| TestSerialProxyForm | 3389 / 20863 / 836 | 同 | 同 | 是 |
| TestSerialUserGenericCallbacks | 3391 / 20870 / 842 | 同 | 同 | 是 |
| TestSerialLookupPairing | 3389 / 20864 / 840 | 同 | 同 | 是 |

TestSerialUserGenericCallbacks 三个种子都是 3391，与 main 种子 1 的值相同，没有多纳入提供者类。StockTrans 与 main 同为
3386 类。单次闭包实耗约 40–45 s（不含锁排队）；main StockTrans 约 60 s。

**集合变化**：
- 相对 main，反射集少 3 项：`sun/net/www/protocol/{file,jar,jrt}/Handler.<init>`。
  `URL$DefaultFactory.createURLStreamHandler` 的 forName 只在 default 分支上；file / jar / jrt 三个协议在 switch 各支里直接
  `new`，仍在闭包内。
- 协议名在不动点上是 top，所以该站点是不确定站点。main 只在中途窗口期按名加载过这三个名字，属于顺序依赖带入的类。
- 其余不确定站点：`ObjectInputStream.resolveClass`、`FactoryFinder.getProviderClass`、ServiceLoader `nextProviderClass`、
  `ResourceBundle`、`ClassWriter.getCommonSuperClass` 等。
- 放行站点：`StandardCharsets.lookup`、`LocaleProviderAdapter.forType`、`ClassSpecializer` loadSpecies、
  `CalendarSystem.forName`、`Security.getSpiClass`、`OIDMap$OIDInfo.getClazz` 等。

**合入判据实测**（7426e583，已合并上游 dd10731c，发射器与 main 相同；本机经 heavy_lock 交替跑，main 二进制取自 dd10731c）：

| 用例 | 指标 | main | engine-order | 结论 |
|---|---|---|---|---|
| TestJndiNoProvider | 闭包类 / 方法 / 反射 | 3841 / 24568 / 1617 | 3841 / 24556 / 1610 | 本分支是 main 的真子集 |
| TestJndiNoProvider | `rava closure` real（user） | 184 s（160 s） | 281 s（212 s） | **慢约 33%（按 user 计）** |
| TestJndiNoProvider | 发射 JDK 类 | 3839 | 3839 | 持平 |
| TestHttpLoopbackSync | 闭包类 / 方法 / 反射 | 5443 / 33929 / 1739 | 5442 / 33923 / 1732 | 本分支是 main 的真子集 |
| TestHttpLoopbackSync | `rava closure` real（user） | 293 s（280 s） | 342 s（331 s） | **慢约 17–18%** |
| TestHttpLoopbackSync | 发射 JDK 类 | 5441 | 5440 | 少 1（`RSAKeyPairGenerator$PSS`） |

- 发射全程：本分支 JNDI 231 s、HTTP 503 s。main 的同轮发射只要 7–9 s，是命中了 main 工作区已有的闭包 / 生成缓存，
  不可比。合并上游前（发射器不同）的一轮：JNDI 261 s vs 230 s，HTTP 453 s vs 342 s。
- 少掉的项全部来自中途窗口期按名加载：
  - `sun/net/www/protocol/{file,jar,jrt}/Handler`、`java/util/logging/Handler`、asm `Handler` 的构造器，
    都是 `*Handler` 名字的不确定站点；
  - JCA 的 `RSAKeyPairGenerator$PSS`；
  - 队列类的 `offer` / `poll`。
- 判据 ② 满足：`closure_independent_of_hash_seed` 通过，6 例 × 3 种子闭包完全一致。
- 判据 ① 只满足一半：类数不高于 main，但闭包耗时高于 main。StockTrans 一类序列化用例反而更快（约 40 s vs main 约 60 s）。

**未解决与下一步**（交新代理）：
1. 闭包耗时回到不高于 main。疑点：
   - 每轮空闲放行后，挂起站点所在方法要重跑（`lookup_trial` 经 swork 重处理整个方法），放行轮数多时累积开销大。
   - `class_lookup` 每次都对 `methods[m].key.to_string()` 与 `instantiation_hosts` 做字符串比较。应在构造时把清单
     解析成方法号集合。
   - `field_wrap_call` 的类 × 名笛卡尔积。`fpair_done` 只挡住了重复开放，枚举本身没有省掉。

   建议先用 `--perf` / 采样定位 HTTP / JNDI 两例的热点，再按「只重跑站点、不重跑整方法」改放行路径。
2. 服务器上 TestHttpLoopbackSync 在 main 上本身也是 10 min 转译超时（发射阶段为主）。要过超时线，需要另立的引擎 / 发射提速，
   不属于本项。
3. 上文 `@hwopen` 的 open 写入顺序依赖仍未根治，目前靠延后放行消除了它对单测的影响。终态方向见「交接 / 下一步」。

### 2026-10-05：闭包耗时回到 main 以下（engine-order，阶段 3）

实施要点（动手前记）：
- 采样显示三个疑点都不是热点。真正的耗时在通用流传播：`add_to_id` 里的 `is_subset` 约占 24%；E→S 推送约占全部推送的一半，
  有效的只有约 0.2%。原因是同一批数组分配点（如十几个 `Type[]`）的元素节点各自连到同样的数千个读站点，每次增量都在
  「分配点 × 站点」条边上重复推送。
- 改法：数组元素读站点仿字段汇集节点（`gather.rs`），按（元素槽, 分配点集合）共享 `G` 节点。元素节点以 Object 过滤流入
  `G`，`G` 再按站点静态分量类型流到站点。过滤逐元素进行，到达的集合与逐分配点接边相同。
- 清单查表 ② 顺手改成构造时解析的成员集合，放行轮次与首轮放行时刻记入 `--perf`。
