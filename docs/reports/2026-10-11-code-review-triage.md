# 2026-10-11 代码审查分诊

> 对象：[第一轮](2026-10-11-code-review.md) · [第二轮](2026-10-11-code-review-round2.md) · [动态验证](2026-10-11-code-review-verification.md)。
> 核实基线 dcc1e8e1（只读核对代码）。分诊口径：10-11 主线是框架编译测试与 JDK API 翻译，审查项只即派**影响测试可信度**、**Java API 语义正确性**与**运行时性能**三类（性能为用户 10-11 追加要求），其余记待办。

## 一、即派

### A. 测试基础设施（分支 `review-test-infra`）

| 项 | 证据 | 修法 |
|---|---|---|
| 续跑接旧租约假 TIMEOUT | `scripts/cluster/dist_e2e.py:273`、`dist_job.py:249` | 接上时期限取 `max(原期限, 接上时刻 + 时限)`；ref 不符则回收重派 |
| 测试名未校验即拼远端 shell / `rm -rf` | `dist_e2e.py:115,148-151,176-179`、`dist_state.py:26` | 入口统一校验 `^[A-Za-z_][A-Za-z0-9_]*$`，远端命令 `shlex.quote` |
| 并行模式运行无超时 | `scripts/run_tests.py:1077-1080` | 删 `_run_binary`，并行段复用 `_run_bin` |
| `_parse_raw` 双调用，raw 合计翻倍且多线程写全局 | `run_tests.py:1253/1255`、`1420/1446` | 每例只调一次，主线程累加 |
| 状态 JSON 解析失败静默清空 | `scripts/cluster/dist_state.py:101-109` | 备份 `.corrupt-<ts>` 并报错退出；写入 tmp + `os.replace` |
| 分发器恒 exit 0；`--cleanup-servers` 忽略 `--servers` | `distribute_tests.py:153-164,250,383-415` | `sys.exit(main())` 带失败码；清理按所选服务器 |
| 巡检 / merge_daemon 写死 `state_jdk21.json` | `patrol_status.py:125`、`merge_daemon.py:125` | `dist_state.state_path(dir, jdk)` 唯一来源，巡检遍历所有 JDK |

### B. Java API 语义（分支 `review-api-semantics`）

| 项 | 证据 | 修法 |
|---|---|---|
| `System.arraycopy` 缺前置检查（负长度静默通过、越界前部分写入） | `runtime/java_runtime/src/java/lang/system_impl.rs:16-75` | 按 JVMS 先查 5 条件再整段复制 |
| 未捕获异常无 `at` 栈帧 | `runtime/java_runtime/src/error.rs:206-232` | 走 `Throwable.printStackTrace` 语义输出 |
| `Thread.start0` 创建失败不回滚 | `java/lang/thread_impl.rs:203-210` | 回滚 eetop / threadStatus / LIVE_THREADS |
| `Class.forName0(null)` 抛 CNFE | `java/lang/class_impl/attrs.rs:134-146` | 先判 null 抛 NPE |
| `getNestHost0` 按 `$` 猜 | `java/lang/class_impl/nest.rs:74-82` | 读 NestHost / NestMembers 元数据 |
| `FileDescriptor.sync0` 抛 IOException 且 `unwrap` | `java/io/file_descriptor_impl.rs:53-62` | 抛 `SyncFailedException`，`?` 传播 |
| zlib 负长度 `as u32` | `java/util/zip/{deflater,inflater}_impl.rs` | `u32::try_from` 失败抛 AIOOBE |
| 监视器 / Parker 侧表永不回收，地址复用后新对象继承旧监视器 | `monitor.rs:304,460`、`obj_ref.rs:364-378` | `drop_slow` 按 `has_monitor` 摘表；线程结束摘 Parker |
| `fix_bool_returns` 文本替换误改字面量 | `generator/crates/method/src/postprocess.rs:28-34` | 发射层按返回类型 Z 直接出 `Ok(true/false)`，删文本后处理 |

### C. 运行时性能（分支 `review-runtime-perf`）

`Class.for_class` 每次 `format!`（`class_impl.rs:55-83`）；反射分派全局锁 + 闭包 clone（`reflect_dispatch.rs:46-88`）；ldc 字面量每次重新驻留（`string_ext.rs:141`）；`vm_throw` 每次读 `RUST_BACKTRACE`（`error.rs:34-38`）。验收以三测点（运行 / 构建 / 体积）前后对照。

#### C 性能实测（分支 `review-runtime-perf`）

改动（只动 VM 基础设施与生成器发射形态，不新增手写方法）：
- `Class::for_class(&str)`：按 `&str` 查镜像，不再 `format!`；`java_class!` 宏的 ldc 逐调用点 `OnceLock` 改写由 `String::from("lit")` 扩到 `Class::for_class("lit")`（`rava_macros_core/src/block/ldc.rs`），类字面量与字符串常量首次执行后零分配，驻留同一性不变。
- 反射分派表：注册时排序去重后冻结为 `OnceLock<Box<[(&str, fn)]>>`，查找二分、无锁无 clone；分派器由 `Rc` 闭包改为 fn 指针（生成器 `emit/src/phase2/dispatch.rs` 发射 `as ReflectDispatch`）。上溯超类链、静态判定改走有序元数据表 `meta::row`（二分，保持原首匹配语义），描述符参数拆分零分配。
- `vm_throw`：`RUST_BACKTRACE` 读一次缓存于 `OnceLock<bool>`。

三测点（us1 同机前后；基线 39d6d123 / 微基准基线 1ae0d00f，改后 9d601f05 / 43f37f63；运行取 5 次中位数）：

| 用例 | 构建 s 前 → 后 | 运行 s 前 → 后 | 二进制字节 前 → 后 |
|---|---|---|---|
| DeepCopy | 476.06 → 467.95 | 0.284 → 0.269 | 371311800 → 370171776 |
| StreamDemo | 65.34 → 65.14 | 0.023 → 0.022 | 65361848 → 65381520 |
| TestAnnoReflect | 284.53 → 290.64 | 0.174 → 0.172 | 289032952 → 288454784 |
| TestDynamicProxy | 252.80 → 252.01 | 0.171 → 0.175 | 265451848 → 265081736 |
| sc1 | 302.51 → 304.54 | 0.277 → 0.252 | 303699912 → 303336560 |
| sc2 | 268.16 → 261.17 | 0.291 → 0.272 | 269713568 → 269045720 |
| sc3 | 326.16 → 321.22 | 0.203 → 0.201 | 316744488 → 316204336 |
| sc4 | 261.22 → 263.11 | 0.181 → 0.175 | 270642720 → 270365328 |
| TestReflectLdcHotLoop（微基准，100 万次） | 65.30 → 65.55 | **107.410 → 43.197** | 66567024 → 66577376 |

构建耗时在 ±2% 噪声内，体积持平（多数略减）；常规用例运行时热路径占比小，持平或略快；微基准提速 2.49×，峰值 RSS 持平（约 24.7 MB）。计时脚本 `scripts/perf_hotpath.sh`，结果 `cluster_results/job/perf-rt-{base,after}-us1`、`perf-rt-mb-{base,after}`。

验证：本机 cargo check（generator `--release --tests`、rava_macros_core `--tests`）通过；集群单测 ut-closure / ut-rest（作业 perf-rt-ut，82f55952）641 过 0 败；抽查 perf-rt-a4（43f37f63，jp2 + kr1）DeepCopy / StreamDemo / TestAnnoReflect / TestDynamicProxy / HelloWorld / TestReflectLdcHotLoop 6/6 PASS。

余项：
- 微基准改后仍约 43 µs / 迭代（JVM 为微秒以下），剩余开销在 `Method.invoke` 的字节码翻译路径（访问检查、参数装箱、`MethodAccessor` 链），未剖析；属后续性能项。**→ 已深剖（2026-10-11，分支 rt-perf-prof）**：见 [`2026-10-11-rt-perf-profiling.md`](2026-10-11-rt-perf-profiling.md)。kr1 同机（基线 7b0bf81a）：invoke 段 1.76 µs/调用（JVM 31 ns，56 倍），热点前三 = 分派键 String 重建簇 ~17%（`__member_key` 每次 invoke 重建类名/方法名/描述符，`method_impl.rs:36`）、分配器簇 ~16%、分派协议簇 ~10%（`is_static_descriptor` 等）；`rd::unbox_*` 文本往返量化为 idInt−idRef=178 ns/调用边界开销的一部分，已落 P-B 结构化取值微修复（hashCode 契约，9157a73e，随附 `__name_assignable` 二分化）。PartitionInteger 48 s vs JVM 0.65 s（72 倍）：大头为每叶子管线构建 25–30% + 每元素约 5 重接口分派（`__IfaceRef::enter` ×2），装箱 <3%，self 弥散无单点可修——归生成器侧单态化 / S7 线。43 µs 与本测 4.3 µs/迭代（整例）差约 10 倍未闭合（彼时 us1 / 43f37f63，见报告 §二）。
- 既有缺陷：`Method.invoke` 返回的装箱 `Integer` 经 `(Integer)` checkcast 抛 `ClassCastException: java/lang/Integer cannot be cast to java/lang/Integer`（`try_cast<Integer>` 失配，前后均复现），微基准改经 `hashCode()` 取值绕开；需另修。**→ 已修**（fix-invoke-box-cast 84d9468f：VM 装箱点经包装类 `valueOf`，batch-1011i 并入；测试 6333f82d 恢复直接强转）。

### 生成器修复（分支 `review-generator`，自 96fc1cf4）

待办「生成器 / 宏」七项（描述符 / 注解递归深度上限除外，未列入本批）：

| 项 | 提交 | 终态 |
|---|---|---|
| 宏分类两份实现 | 4f2fb0a5 | `mc/block/classify.rs` 的 `BodyFacts`（语法树访问器，token 兜底）为唯一分类来源，`base_fns` 改用之（只影响宏展开，生成树不变） |
| `java_try!` 全局计数器 | 16017542 | 标签 `'java_try_{深度}` / `'java_try_end_{深度}` / `'java_loop_{n}` 用 `Span::mixed_site` 卫生、按词法嵌套深度编号；删 plan 侧重编号 |
| 清单两套解析、IndyKind 两份 | 11ba980a | `closure::manifest::sections` 为唯一读取与校验（IndyKind 六变体、`InjectedValue`、`EmitSections`）；`input::manifest` 由 closure `Manifest` 投影并 `pub use` IndyKind。closure.json `indy_models` 类别名改取 `IndyKind::as_str`，模式 switch 调用点由 `native` 细化为 `type_switch` / `enum_switch`（行为不变） |
| 类名路径分量未校验 | 5d3a4cea | `classfile::names` 在读入档案时校验（`Error::BadClassName`），`Writer::new(out_dir, …)` 落盘前断言目标在 scratch 内 |
| `Audit::FieldNpe` 无发射点 | 61bfe804 | `sim/fields.rs::null_checked` 每发射一次接收者 `__nn()?` 即记审计；`[equiv-audit] field-npe` 计数由 0 变为实际值 |
| `jdk.rs` 测试改进程环境 | 5851d080 | `JdkEnv`（JAVA_HOME / HOMEBREW_PREFIX / JVM 目录）注入，`choose_in` / `find_major` 取 env；测试移入 `mod tests`，临时 JVM 目录沙箱，不改进程环境 |
| 产物绝对路径依赖仓库 | e11a3191 | 两目标不冲突，已实施：`java_runtime` 兄弟依赖闭包（rava_macros / rava_macros_core / rava_coro）落入 `<scratch>/vendor/`，相对路径引用；包版本取 `0.0.<内容摘要>`、文件 mtime 取真源 mtime，跨 scratch 共享 target 仍命中；真源删除的文件与出闭包的 crate 同步清除；`cargo::owned` 排除 vendor |

验收（云服务器 sg1 / jp2 / kr1，dev 满载未用）：

- cargo check：generator `--tests` 与 runtime（rava_macros / rava_macros_core / rava_coro）无告警通过。
- 单测 `rg-ut2-e11a3191`（jp2）：ut-closure、ut-rest 两作业 rc=0，全部 `test result: ok`、0 failed。
- 生成树 `rg-cmp3-e11a3191`（sg1，gen_trees 27 例 base 96fc1cf4 对 e11a3191）：生成 `.rs` **零差异**，raw-audit identical，closure.json `indy_models` 零差异（验收集无模式 switch 调用点）。差异只有四类，均为预期：① 各例新增 `vendor/`，Cargo.toml 宏 / 协程依赖改指 `../vendor/<名>`（修复 7）；② 日志 `[equiv-audit]` 多出 `field-npe=N`（修复 5，482–3128），其余审计计数逐例一致；③ 日志中的 scratch 路径（base 在 `base_wt/` 下）；④ 物化峰值内存噪声（±2%，对象数与回退组数一致）。宏分类统一（修复 1）与 try 标签（修复 2）只影响宏展开，不进生成树。
- seed_check：DeepCopy / HelloWorld / TestNestedTry / TestTryInLoop 均 `seed-diff-lines=0`。
- DeepCopy `--stop-after emit --perf`（sg1 同机新旧交替各 4 次）：首轮冷启动两版均约 216 s（closure 冷）；热态合计 base 10.94 / 10.99 / 10.95 s，new 10.79 / 10.80 / 10.76 s，无回归（overlay 阶段因落 vendor 多 3 ms）。
- vendor 可搬移：scratch 内 Cargo.toml 无仓库绝对路径；DeepCopy 先编宏 crate，HelloWorld scratch 共享同一 target 时宏 crate 全部 `Fresh`；搬移后 `cargo metadata` 可解析、可构建（复核用 `cp -r`，不保留 mtime，宏 crate 重编属预期；保留 mtime 的 `cp -a` / `mv` 复核见下方未完成项）。
- 抽查 `rg-spot-e11a3191`（kr1，15 例：验收集 12 例 TestNestedTry / TestTryInLoop / TestSuppressed / TestSwitchExpression / TestSwitchString / TestSwitchFallthrough / TestCasting / TestHashMapOps / TestSynchronized / TestArrayList / TestStreamBasic / TestCompletableFuture，加 HelloWorld / DeepCopy / TestAnnoReflect）：**15/15 PASS，0 失败**（分发器 exit 0）。

未完成项与下一步（10-11 收尾，用户指令暂停派发）：

1. 抽查 `rg-spot-e11a3191` 已 15/15 全过，分支可攒批合入。
2. vendor 搬移保留 mtime 的复核未完成：作业 `rg-vmove-e11a3191`（sg1）远端进程三次无 done 被归还，已停。脚本在 worktree `build/review/vendor_move.sh`（HelloWorld emit → 宏 crate 首编 → `cp -a` / `mv` 到 /tmp 再编，预期均 `Fresh`）；换空闲服务器重发即可。若搬移后仍重编，根因在 cargo 指纹含 workspace 绝对路径，终态改为 `--remap-path-prefix` 不足以解决时再议，代码层无需改动 vendor 方案。
3. 待办余项「描述符 / 注解递归无深度上限」未纳入本批。
4. 分支 `review-generator` 只推 origin 与 github，未合入 main（按派发约束），抽查已全过，待攒批。

## 二、待办（不派，主线空档或相关改动顺手做）

- 健全性：`Handoff<T>` 无界 Send（`gil.rs:158`）；`native_memory` 批量拷贝与 CAS 混访、未对齐 CAS 退化、`fill` 负数（`native_memory.rs:83-162`）；`rava_coro` Drop 中 `expect`（`stack.rs:393,409`）。→ 已修，见五。
- 生成器 / 宏：描述符 / 注解递归无深度上限 → 已修，见五（其余七项已由「一、生成器修复」处理）。
- 运行时其他：`runtime_impl.rs:44-50` 无条件读 `/proc`（macOS 恒 0）。→ 已修，见五。
- 工具：`ssh_client.py:47` AutoAddPolicy（报告所列 :77 不存在）。

## 三、已失效

- TLS lint 红：3b95ceeb 已修（verification 与本次核对一致）。
- order / hash_seed 回归：verification 证实不复现。
- string switch 哈希冲突：已补见证，行为正确。
- lookupswitch 预分配：降为加固项，并入待办。

## 四、待用户决策

- 许可与法务：仓库无 LICENSE；JDK（GPLv2+CE）字节码翻译进产物的定性；语料来源核实。发布前阻断，开发期不阻断。
- `merge_daemon.py` / `merge_queue.py`：已停用，无调用者（仅单测与文档引用），删除或恢复。
- CI：仓库无 CI 配置；现行以集群单测四作业 + 抽查代替。
- 「main 既有」失败缺机器基线、用户文档、构建进度输出：属产品化项。

## 五、健全性修复（分支 `review-soundness`）

二「待办」中健全性四项、描述符 / 注解递归深度与 `runtime_impl.rs` 的 `/proc` 一项已修。逐项如下。

| 项 | 修法 | 见证 |
|---|---|---|
| `Handoff<T>` 无界 Send（`gil.rs`） | 只实现 `unsafe impl Send for Handoff<Thread>`，SAFETY 写明理由：Thread 不是 Send 只因 `dyn ObjectVTable` 没带 Send 约束；对象存储是原子引用计数加线程安全单元；线程对象本就跨 OS 线程共享。其他 `T` 可以构造，但不能跨线程移动 | `gil.rs` 内的编译期断言：`Handoff<Thread>: Send`，且 `Handoff<Rc<u8>>` 不是 Send（`AmbiguousIfSend` 手法，与 static_assertions 的 `assert_not_impl_any!` 相同）。独立片段实测：恢复无界实现后报 E0283；把 `Handoff(Rc)` 移进 `thread::spawn` 报 E0277 |
| `native_memory` 原子与非原子混访（`native_memory.rs`） | 新 crate `runtime/rava_rawmem`。直接内存的读、写、填充、拷贝与 CAS 全部落在包含该地址的 8 字节对齐 `AtomicU64` 上：普通访问用 Relaxed，字内部分写用 CAS 合并，读-改-写用 SeqCst CAS 循环，只替换自身字节。同一地址的并发访问因此都是同宽原子操作，没有数据竞争，也没有混宽原子访问。Unsafe 的 get/put 走普通读写（不要求对齐），CAS 族走 `update`。取消标记（`Files.copy`）的 `read_volatile` 改走同一原子单元 | rava_rawmem 单测 8 例：各宽度对齐 CAS 见证旧值；未对齐 CAS 被拒且不写；负长度被拒；任意对齐读写往返；批量读写、填充、重叠拷贝与 memmove 一致；CAS 计数与同字相邻字节的批量写并发不丢更新；同字两半并发 CAS。e2e 新增 `TestDirectMemoryAtomics` |
| 未对齐 CAS | 抛 IllegalArgumentException（`Unaligned atomic update ..., not supported`），不再静默退化为非原子读-改-写。依据：HotSpot 对 int/long CAS 不查对齐，结果随平台而异（x86 靠 split lock 仍原子，aarch64 无 LSE2 时对齐故障，表现为 InternalError）；JDK 自身对无法以单字原子完成的更新规定抛 IAE（`Unsafe.compareAndExchangeShort`：「Update spans the word, not supported」）。这里取后者，各平台一致。VarHandle 视图（byteBufferView / byteArrayView）在 Java 侧已对未对齐的原子访问抛 IllegalStateException，到不了 native | 同上 |
| `fill` / `copy` 负 bytes | native 入口抛 IAE（`negative size: n`）。Java 侧 `Unsafe.setMemory` / `copyMemory` / `copySwapMemory` 在调用 `*0` native 之前已执行 `checkSize`（负数抛 IAE）和 `checkPrimitivePointer`（JDK 21 `Unsafe.java` 462、735、797 行），这些公开包装按字节码翻译（a3-U1）。native 侧检查属纵深防御，守住 native 入口本身 | `negative_length_rejected` |
| `rava_coro` Drop 中的 `expect`（`stack.rs`） | 两处改为 `let-else` 加 `debug_assert!`：release 下泄漏该栈槽，不在 drop 中 panic；debug 下照常断言 | `drop_with_unknown_chunk_leaks_instead_of_aborting`（release 下不 panic、计数一致；debug 下 `should_panic`） |
| 描述符数组维数递归（`descriptor.rs`） | 前导 `[` 改为循环计数，上限 255（JVMS §4.3.2 / §4.4.1），超限返回 None | `array_dimensions_capped_at_255`：255 维可解析且往返一致；256 维在字段、方法参数、返回值三处都被拒；65535 维被拒，不栈溢出 |
| 注解元素值递归（`class.rs` / `extras.rs`） | `annotation` / `element_value` / `anno_refs` 带深度参数，上限 `MAX_ANNOTATION_DEPTH = 256`（JVMS 不设上限，javac 产物远低于此），超限报 `Error::AnnotationTooDeep`。`anno_refs` 原本容忍格式错，现在只上抛深度错 | `element_value_depth_capped`（256 层通过，257 层报错，十万层 `@` 链报错且不栈溢出）、`anno_refs_depth_capped` |
| `Runtime.totalMemory` 的 `/proc`（`runtime_impl.rs`） | 按 cfg 分平台：Linux / Android 读 `/proc/self/statm`；Apple 用 Mach `task_info(MACH_TASK_BASIC_INFO).resident_size`；其他平台不可得，结果为 0 | 独立片段对 aarch64-apple-darwin、x86_64-unknown-linux-gnu、x86_64-unknown-freebsd 三个目标做 `cargo check` 均通过（nightly `-Zbuild-std`）；Linux 实测返回常驻集 |

配套改动：`overlay.rs` 不再逐个硬编码 `rava_macros` / `rava_coro`，改为把 java_runtime 清单里每个 `path = "../<兄弟 crate>"` 统一改写为绝对路径。overlay 单测断言 rava_rawmem 为绝对路径，且清单中不再残留 `"../`。

**Miri**：`cargo +nightly miri test` 跑 rava_rawmem 全部 8 例，取 4 个种子（`-Zmiri-many-seeds=0..4`），均通过，未报 UB 或数据竞争。并发用例在 Miri 下降为 40 次迭代。rava_coro 含内联汇编栈切换与 mmap，Miri 无法执行，未跑。没有用 loom：rava_rawmem 只用 std 原子，Miri 的弱内存模拟已覆盖。

**验收结果**

- 本机：generator `cargo check --release --tests` 无误；emit 单测 93 + 2、classfile 15、rava_rawmem 9（含偏移 0..8 × 长度 0..40 穷举）、rava_coro release 与 debug 均通过。
- 全量单测作业 `rs-ut2`（sg1，a6c8fcac）：`ut-closure`（closure_cli 11 例，3654 s）与 `ut-rest` 两条都是 rc=0，无失败。
- 抽查 `rs-spot2`（sg1，a6c8fcac）：11/11 PASS（DeepCopy、HelloWorld、StreamDemo、TestAtomics、TestByteArrayViewVarHandle、TestDirectBuffer、TestDirectBufferPaging、TestThreadJoin、TestThreadNatives、TestUnsafePrimitiveArray、TestVarHandleBitwise）。另有 1 次 infra 失败，是该 ref 上的旧 VarHandle 版 TestDirectMemoryAtomics（无期望文件），以 rs-spot3 为准。
- 抽查 `rs-spot3`（kr1，a3b7ca0a）：TestDirectMemoryAtomics（Unsafe 版）PASS，1/1。

**计时**（作业 `rs-bench5`，kr1 同机，前 96fc1cf4 → 后 a3b7ca0a，各 5 次）

基准是 sun.misc.Unsafe 绝对地址热循环（DirectMemBench，跑在 rava 生成的 release 二进制上）：

- CAS 段：getAndAddInt 加 compareAndSwapLong，500 万次；
- 普通段：putInt / getInt，204.8 万次；
- 批量段：copyMemory 4 KiB 加 setMemory 96 B，2 万次。

| 段 | 前 | 后（a3b7ca0a） |
|---|---|---|
| CAS | 413–441 ms（中位 417） | 439–445 ms（中位 441，+6%） |
| 普通 | 63–66 ms | 57–58 ms（−10%） |
| 批量 | 3 ms | 59–61 ms（**20 倍回退**） |
| 进程总耗时 | 0.517–0.548 s | 0.595–0.599 s |
| 构建 | 转译 58.9 s、编译 11 m 52 s | 转译 58.6 s、编译 11 m 46 s |
| 二进制 | 53 168 904 B | 53 181 072 B（+12 KB） |

批量段回退的原因：拷贝经 512 字节栈缓冲，`spans` 闭包每个字都切段并逐字节复制。57564806 改了两处：

- 读、写、填充改为「头段 / 整字 / 尾段」显式循环；
- 源与目标字内偏移相同的拷贝走逐字一读一写的快路径。

本机微基准（`copy` 4 KiB 加 `fill` 96 B，2 万次）由 26 ms 降到 3 ms，与 memcpy 基线同量级。偏移不同的拷贝由 27 ms 降到 4.7 ms。Miri 2 种子 9/9 通过。**57564806 的 kr1 同机复测未做**（用户指令暂停后收尾），列入下一步。

**遗留**

- 堆上基本类型数组经 `__update_bytes` 做跨元素读-改-写（例如在 `byte[]` 上经 int 字路径做 byte CAS）时，走 stripe 锁下的读-改-写，可能丢失同时发生的普通元素写。这不在本步「直接内存」范围内，需在 `array/store.rs` 用同样的字粒度原子单元解决。
- 直接内存还有 FFI 写入方（zlib 的 `inflate` / `deflate`、`read(2)` 写入直接缓冲区、`CStr::from_ptr(c_path(address))` 读路径），它们在 Rust 内存模型之外，与 Java 线程并发访问同一缓冲区属于 Java 程序自身的数据竞争。
- 预先存在的缺陷（main 96fc1cf4 上同样存在，与本分支无关）：在 `ByteBuffer.allocateDirect` 上用 `MethodHandles.byteBufferViewVarHandle` 做原子访问时，程序启动即抛 `IllegalStateException`（无消息）。kr1 上 rs-bench4 前后两版二进制都是约 7 ms 内抛出。因此 TestDirectMemoryAtomics 改用 Unsafe 绝对地址；VarHandle 直接缓冲视图需另立任务查根因。
- 下一步：① 57564806 在 kr1 同机复测 DirectMemBench，判据是批量段 ≤5 ms，CAS 和普通段不劣于 a3b7ca0a。命令见下条。② CAS 段的 +6% 来自 `update` 的 `&mut dyn FnMut` 动态调用与两次掩码运算；若要求持平，可以把 `update` 改为泛型 `impl FnMut` 内联（native_memory 调用点不变）。③ 堆数组跨元素读-改-写（见上条）。④ VarHandle 直接缓冲视图 ISE（见上条）。
- 计时脚本已入库 `scripts/direct_mem_bench.sh`：内嵌 DirectMemBench.java 与期望输出；对当前检出与基线 ref（缺省 96fc1cf4）分别 `run_tests.py --release --keep-artifacts`，再把生成的二进制各跑 5 次。复测命令：`distribute_tests.py --job <tag> --ref 57564806 --servers kr1 --cmd "bash scripts/direct_mem_bench.sh"`。
- 为跑 Miri，本机装了 nightly 工具链及 miri、rust-src 组件（`~/.rustup/toolchains/nightly-*`，Miri sysroot 在 `~/.cache/miri`）。

<<<<<<< HEAD
### 余项处置（分支 `sound-rem`，自 batch-1011k 5328072a）

**rs-bench6 补记**：57564806 的 kr1 复测（rs-bench6，fdc47d90）批量段 7 ms（基线 96fc1cf4 为 3 ms，判据 ≤5 ms 未达标）；CAS 393 ms vs 基线 401、普通段 58 vs 64 略优。批量段仍差约 2 倍：512 个逐字「一读一写」未展开，循环控制开销与访存流水不足（原子访问不能向量化，只能靠展开与批量寄存器暂存逼近 memcpy 的访存吞吐）。

**① 批量快路径重做（fc2e979d，必做项）**：头-整字-尾结构不变，整字体改为 **8 字 ×8 展开**——`read` / `write` / `fill` 的整字段每轮 8 个字成组访存；同偏移拷贝快路径按 8 字块处理，**块内先全部读出再全部写入**（对任意同偏移重叠安全：写字 `i` 只可能覆盖字 `i + d` 的源，`d = (dst − src)/8 ≥ 1`，而那些字要么在本块更靠后、已随本块读出，要么在已处理的块里），**块间按重叠方向排序**（目标在源之后且重叠时自尾向头降序，否则升序）；对齐地址经 `word_aligned` 免二次掩码。正确性测试扩充：偏移 0..8 × 长度 0..80 穷举（跨 64 字节展开块边界）、新增同偏移拷贝穷举（偏移 0..8 × 长度 0..72 × 目标-源差 ±8 / ±16 / ±64，含重叠两方向）、`copy_overlapping_matches_memmove` 补 520 / 200 / 129 / 65 等跨块同偏移用例。Miri（作业 `srem-miri1`，jp2，`-Zmiri-many-seeds=0..2`）2 种子 × 10 例全过、无 UB 报告。kr1 计时见下方复测表（srem-bench2）。

**② CAS 段回调泛型内联（5bb96f9d）**：`rava_rawmem::update` → `native_memory::update` → `unsafe__ext::prim`（含 `masked`）全链改泛型 `F: FnMut(u64) -> Option<u64> + ?Sized`，各访问器的比较 / 加法闭包单态化后内联进 CAS 重试循环，热路径无 dyn 间接调用；`?Sized` 让 VarHandle 入口的 `&mut dyn` 实参仍可进入（退化为原形态）。终态计时见 srem-bench3。

**③ 堆数组跨元素读-改-写（ed4244b1）——先修正诊断，再定方案**：

- 经核，旧实现的分条索引取**数组存储基址**（`slots.as_ptr() >> 4`），同一数组的全部跨元素读-改-写恒落同一条、彼此本就互斥；「可能丢失同时发生的普通元素写」的准确形态是——跨元素读-改-写在 T1 逐元素读、T3 逐元素原子提交，**与元素内访问（普通读写、元素 CAS）之间只有逐元素原子与逐位置一致性，没有整区间一步原子**：窗口内并发的元素写可以与 T1 的旧值混出「从未同时存在过的字节组合」。对有数据竞争的程序这是 JMM 逐位置语义下的合法结果（每个位置的写序自洽、hb 序访问不丢）；但低于 HotSpot 的整区间线性化（x86 lock cmpxchg / aarch64 独占对原生给出整区间原子）。
- 原方案「用同样的字粒度原子单元」**不可行**：元素路径是 `AtomicU8/16/32`，字路径是 `AtomicU64`，二者并发即**混宽原子访问**——Rust 内存模型下未定义，Miri 明确按 UB 拒绝（`miri/src/concurrency/weak_memory.rs` 文档："Mixed size atomic accesses must not race with any other atomic access, whether read or write, or a UB will be thrown"，issue rust-lang/miri#345 的结论也是定义为 UB）。把元素路径也改成字粒度 CAS 合并会让 byte[] / short[] 的每次普通元素写变 CAS 循环（bulk 写慢一个数量级），不取。
- 落地（ed4244b1 + 8cf51774）：分条锁改为**锁住区间覆盖的全部 8 字节对齐字**（`width ≤ 8` 至多两字，地址序加锁无死锁；重叠区间必共享至少一个对齐字——保留互斥，同时放宽同数组不重叠区间的并行）；两覆盖字可哈希到**同一条**（同一 16 字节块内的相邻字），非重入互斥锁两次即自死锁——按分条序号去重只锁一次（8cf51774，srem-spot1 上 TestHeapArrayAtomicView 在 byte[] 存储基址 ≡ 8 (mod 16) 时无限挂起的根因）；`update_bytes` / `__update_bytes` 回调泛型内联；原子性边界与不可行论证写进 `array/store.rs` 模块头注。回归测例 `TestHeapArrayAtomicView`（34_concurrency，JDK21 生成期望）：同偏移双线程各 20 万次 getAndAddInt 计数不丢（400000）、跨 16 字节分条边界的重叠区间（[14,18) 加 1<<16 与 [16,20) 加 1，只共同动全局字节 16）各 100 次 = 200、long 路径（byte[] 上跨 8 元素）往返一致。
- 待办（低优先）：跨元素读-改-写对**未对齐且跨 16 字节分条**的整区间线性化仍只是互斥级（元素内访问不持条），如需完全对齐 HotSpot 只能引入字粒度元素存储（见上，代价明确后才可议）。

**④ byteBufferView 直接缓冲 VarHandle 启动即抛 ISE——根因（已查清，未修）**：

- JDK 21 实测：`MethodHandles.byteBufferViewVarHandle(int[].class, …)` 的 flavor 运行时类名是 **`java.lang.invoke.VarHandleByteArrayAsInts$ByteBufferHandle`**（前缀仍是 ByteArray——ByteBuffer 视图与 byte 数组视图共用 `VarHandleByteArrayAs*` 外层类；数组视图是 `$ArrayHandle`，缓冲视图是 `$ByteBufferHandle`）。
- rava 的访问模式分派（`var_handle_impl.rs`）以 `_is_array_flavor`（类名含 **`"$Array"`**）判数组族：`$ByteBufferHandle` 不含该子串（它的 `$` 后是 `ByteBufferHandle`），于是**落入 Field 族** `_field_coords` → `_field_offset` 按名协议读 `fieldOffset` 得 None（该 flavor 只有 `be` 等字段）→ `_state_err()`（`var_handle_ext.rs:138`，`IllegalStateException::new()` **无消息**）——首次访问模式调用即抛，即旧版 TestDirectMemoryAtomics「启动即抛 ISE」的全貌。堆 ByteBuffer 视图同症（与直接 / 堆无关）；`byteArrayViewVarHandle` 不受影响（`$ArrayHandle` 两处判定都命中，rs-spot2 已验证）。
- 修复量级：需在 `var_handle_ext.rs` 新增 `$ByteBufferHandle` 视图族（按 `be` 定字节序，plain 模式走缓冲字节视图、原子模式走 `native_memory::update` 并按 JDK 口径对未对齐原子访问抛 ISE），与既有字节数组视图族同量级的通用机制，非低风险顺手改——按派发约定「修复视风险决定」**留待专项**，本步只交付根因。

**复测（kr1，scripts/direct_mem_bench.sh，各 5 次；批内对照消除机况与合批内容差异）**

- 抽查 srem-spot1（ed4244b1，9 例）：8/9——TestDirectMemoryAtomics、TestByteArrayViewVarHandle、TestUnsafePrimitiveArray、TestVarHandleBitwise、TestAtomics、TestDirectBuffer、TestDirectBufferPaging、TestParallelArrayCas 过；TestHeapArrayAtomicView 运行超时（见上，③ 的同条双锁死锁，8cf51774 修复）。srem-spot2（8cf51774）复跑该例：**1/1 PASS**。合计 9/9。
- srem-bench4（8cf51774，终态 = ① + ② + ③；rawmem 代码自 fc2e979d 起未再改动，本表即 ① 的验收数字）与 srem-bench5（5328072a，batch-1011k 头、无 sound-rem 的控制组——rs-bench6 的 after ref fdc47d90 不含 batch-1011i/j/k，其 58/393 不可与含批状态直接对比）：

| 段 | 基线 96fc1cf4 | 控制组 5328072a | sound-rem 8cf51774 |
| --- | --- | --- | --- |
| CAS | 400–407（中位 403） | 440–455（中位 446） | 414–431（中位 417，**比控制组快 6.5%**——② 的泛型内联兑现） |
| 普通 | 63–64（中位 64） | 73–75（中位 74） | 79–81（中位 80，比控制组慢 8%） |
| 批量 | 3 | 7–8（中位 7） | **4–5（中位 5，判据 ≤5 ✓**；比控制组快 29%） |
| 二进制 | 53 168 904 B | 52 221 904 B | 52 250 176 B |

结论：普通段与 CAS 段相对 **96fc1cf4 基线**的回退（64→74、403→446）**主体来自 batch-1011i/j/k 合批内容本身**（控制组 5328072a 不含 sound-rem 已如此；rs-bench6 的 after ref fdc47d90 不含该批，其 58/393 不可与含批状态直接对比；批内基线数字跨作业稳定 ±1ms，非机况噪声），非本分支引入，转主会话归因（嫌疑 nogc-escape 的运行时改动或闭包变化）。sound-rem 在其上：CAS 446→417–428（**−4~−6.5%**，② 的泛型内联兑现）、批量 7→4–6（**中位 5，判据 ≤5 ✓**）、普通段 74→78–84（**+8%**）。普通读写路径语义零改动（diff 仅 update 泛型化与注释），+8% 判为 java_runtime 编辑引起的代码布局 / LTO 内联漂移：cd4de4d4 给 `rava_rawmem::load` / `store` 加 `#[inline]`（允许跨 crate 内联进 Unsafe getXxx/putXxx 热循环）复测无改善（srem-bench6：普通 78–84、CAS 422–434、批量 4–6，CAS 差值在作业间噪声内），保留该属性（无害且方向正确）。如需收口普通段，属随批的布局噪声再归因，不阻塞本分支判据（批量达标、CAS 改善）。

**待验证清单（交主会话，合批 batch 时带上）**

- e2e 抽查并集追加：`TestHeapArrayAtomicView`（新增，34_concurrency）、`TestDirectMemoryAtomics`、`TestByteArrayViewVarHandle`、`TestUnsafePrimitiveArray`、`TestVarHandleBitwise`、`TestAtomics`、`TestDirectBuffer`、`TestDirectBufferPaging`、`TestParallelArrayCas`（srem-spot1 已先行跑过一轮，合批时随批复跑）。
- 单测：`cargo test -p rava_rawmem`（native 档；本分支只经 Miri 2 种子与 DirectMemBench 行为验证，native 全量单测未单独跑）。
- java_runtime 改动（native_memory / unsafe__ext / array）无法在本机 cargo check（模块文件按 scratch 生成）；已由 srem-bench3 的 DirectMemBench 构建（全量编译 java_runtime）与 srem-spot1 的 9 例构建覆盖，合批单测作业照常。
- Miri：如需复验，`cd runtime/rava_rawmem && MIRIFLAGS=-Zmiri-many-seeds=0..2 cargo +nightly miri test`（作业 srem-miri1 在 jp2 装好 nightly + miri）。
=======
## 六、main 普通段 +25% 回退：二分、根因与修复（分支 `plainseg-bisect`，10-12）

tasks.md「剩余工作」第 12 条（用户 10-12 裁决）：kr1 direct_mem_bench 普通段 64→74 ms（96fc1cf4 → batch-1011k 控制组 5328072a，srem-bench5 复现 73–75），回退主体定位 + 修复。判据：普通段 ≤66 ms、批量段 ≤5 ms、CAS 与 446 ms 控制组持平。

### 二分（作业 psg-bis1，kr1 同机同作业，每 sha 独立 worktree 构建 DirectMemBench 各跑 5 次）

| sha | 意义 | `[profile.release]` lto | 普通段 ms | CAS ms | 批量 ms |
|---|---|---|---|---|---|
| 96fc1cf4 | 基线（main 当日位） | `true`（fat） | 64–65 | 403–430 | 3 |
| c6b126cc | batch-1011h 头 | `"thin"` | 87–90 | 474–481 | 3 |
| fe923a2f | batch-1011j、nogc-escape 合并前一刻 | `"thin"` | 87–88 | 468–478 | 3 |
| 5df7e9ba | nogc-escape 合并进 1011j | `"thin"` | 87–90 | 467–473 | 3 |
| 5328072a | batch-1011k 头控制组（srem-bench5） | `"thin"` | 73–75 | 440–455 | 7–8 |
| 8cf51774 | 5328072a + sound-rem（srem-bench4） | `"thin"` | 79–84 | 414–431 | 4–6 |

**回退在 96fc1cf4..c6b126cc 区间已全量出现**（64→88，+37%），不是 1011i/j/k 的合批内容：c6b126cc 与 fe923a2f 同水位，nogc-escape（5df7e9ba）与 1011i/j 各线对普通段无贡献；1011k 的 rava_rawmem 反而在 thin 下把 88 拉回 74（原子字寻址替逐字节循环）。用户看到的 64→74 是「thin LTO 切换 +24 ms、rawmem 改善 −14 ms」的净额；sound-rem 的 +8%（74→79–84）是 thin 模块摘要漂移，同因。

### 根因（作业 psg-gen1：四个 sha 各 `rava build --stop-after emit` 转译 DirectMemBench，生成树逐文件对照）

- 直接路径的生成物**逐字节一致**（user/direct_mem_bench.rs、jdk_unsupported/sun/misc/unsafe_.rs、java_base 内 jdk/internal/misc 包装；仅 0b91182b 的 `Class::for_class(&str)` 取参形态一处，不在热循环）；手写层（unsafe__ext / native_memory）在 96fc1cf4..fe923a2f 期间无改动。**变慢的不是代码，是代码的优化形态。**
- 生成的工作区 Cargo.toml：96fc1cf4 为 `lto = true`，c6b126cc/fe923a2f/5df7e9ba 均为 `lto = "thin"`。翻转点在 **0e3cd83d（build-memsafe 合入 batch-1011h，用户 10-06 定内存受限缺省档）**：该合并中 entry.rs 的唯一代码生成差异就是把 `[profile.release]` 的 lto 从 fat 改 thin（fat 移入新档 release-max），其余为构建编排（并行度 / OOM 退半）。
- 机制：Unsafe 直接内存一次 putInt/getInt 经过 4–6 层跨 4 个 crate 的调用链（user 循环 → sun.misc.Unsafe 包装（jdk_unsupported）→ jdk.internal.misc.Unsafe 绝对地址包装（java_base_body）→ (Object,long) native → unsafe__ext（java_runtime）→ native_memory → rava_rawmem）。fat LTO 全程序单模块把这串整链内联进用户循环；thin LTO 按模块摘要做导入决策，跨 crate 小包装不再稳定内联，每调用多付几纳秒的调用与 Result/Object 约定开销——普通段 204.8 万次 put+get 对此线性放大（+24 ms ≈ 每对 +5.8 ns），CAS 段（500 万次原子调用）与批量段同因同向。生成源码不变（psg-gen1）而三段齐回退、跨 sha 的 thin 数字随语料/运行时编辑漂移（c6b126cc 88 → 5328072a 74；sound-rem 再 +8%），都是 thin 模块导入决策敏感性的表现。

### 修复（7977eca4，本分支）

driver 编译段在 `--release` 时按内存余量就地决定工作区 `[profile.release]` 的 lto 形态（新模块 `generator/crates/driver/src/release_lto.rs`）：

- `--release-lto fat|thin|auto`（`rava build` / `rava compile`，`run_tests.py --release-lto` 同名透传）钉死 / 复现；
- 缺省 auto：fat 全程序链接峰值估计（`mem_budget::Model::of(ReleaseMax)`，大闭包实测 14.5–15.7 GB 的上包络）≤ 探测可用内存（系统 MemAvailable 与 cgroup 余量取小）× 50% 才就地改写 Cargo.toml 为 fat，否则保持 thin——10-06「16 GB 机器 cgroup 内全部标杆可构建」的约束在内存不足的机器上原样成立（探测不到取 thin）；
- 只改 lto 一行，产物仍在 `target/release/`；`--release-max` / `--release-small` / dev 档不经此；auto fat 实测 OOM 先降级 thin 重编（估计上包络兜底）再走作业数退半；选择与理由记入 `build_status.json` 的 `release_lto` 字段并打印 `[lto]` 行。driver 单测 4 例（钉死 / auto 界值 / 只改 release 段且幂等 / 缺形态报错）。

### 验收（kr1 同机；判据：普通段 ≤66 ms、批量 ≤5 ms、CAS 与 446 ms 控制组持平）

**修复分两半**：① 7977eca4 driver 自动 LTO（内存余量足够的大机器自动回提 fat；kr1 仅 15.9 GB 内存，fat 链接估计 12 GB > 可用 × 50%，auto 在 kr1 正确地保持 thin）；② b46fdb7d 直接内存热路径 `#[inline]`（rava_rawmem 载入/存储/读改写、native_memory 同名与 is_raw、unsafe__ext get/put/cas、access 十个访问器）——thin 下显式导入内联，与 LTO 档位无关的兜底。

- **psg-verify1**（7977eca4，① 无 ②，kr1 auto=thin）普通段 76–78 / CAS 491–532 / 批量 7——确认仅靠档位机制在内存受限机器上不收口，② 必要。
- **psg-verify2**（b46fdb7d = ① + ②，kr1）：

| 段 | 基线 96fc1cf4（fat，verify1 [before]） | 修复分支 b46fdb7d（thin + inline） | 判据 |
|---|---|---|---|
| 普通段 | 61–67（中位 64） | **63–65（中位 63）✓** | ≤66 |
| CAS | 404–413 | **357–368 ✓**（优于基线：inline 后 getIntVolatile / compareAndSetInt 链也收入） | 与 446 持平 |
| 批量 | 3 | 7–8 ✗（判据 ≤5）→ 7a84d579 补批量链 #[inline]，psg-verify3 复测（见 tasks.md） | ≤5 |

- **psg-spot1**（7977eca4，dev 档）：直接内存六测 TestDirectBuffer / TestDirectBufferPaging / TestDirectMemoryAtomics / TestByteArrayViewVarHandle / TestUnsafePrimitiveArray / TestVarHandleBitwise 全 PASS；PartitionInteger dev 档 5 min 运行超时为**已知既有**（tasks.md 活跃表无 GC 行：dev 抽查 8/9 即此例超时，release 46 s）。psg-spot2 在 b46fdb7d（含 ②）复跑直接内存五测。**psg-part1**（7977eca4，jp2，release，auto 按 cgroup 9.7 GB 正确选 thin）：PartitionInteger RC=0、输出与期望逐字节一致（OUTPUT_MATCH）。
- driver 单测 psg-utdrv（b46fdb7d 前的 7977eca4 起跑，cargo test -p driver）与本机 4+8 例（release_lto 与 build_opts 解析）。
- 本机 cargo check：generator `--release --tests` 过；rava_rawmem `--release --tests` 过；java_runtime 原始检出缺 overlay 生成的 java/jdk/sun 模块，本机不可独立检（既有形态），由服务器构建（spot / part1 / verify2 的全语料编译）覆盖。
- **候选（未做，另立任务）**：CAS 段 `update` 的 `&mut dyn FnMut` 改泛型内联（sound-rem ② 已验证 −4~−6.5%）；kr1 级 15 GB 机器要拿到 fat 水位需降低 fat 链接峰值（如 fat LTO + 多 codegen-units 档）或收窄声明层（build-memsafe 遗留 1）。
>>>>>>> plainseg-bisect
