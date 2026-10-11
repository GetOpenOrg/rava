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

## 二、待办（不派，主线空档或相关改动顺手做）

- 健全性：`Handoff<T>` 无界 Send（`gil.rs:158`）；`native_memory` 批量拷贝与 CAS 混访、未对齐 CAS 退化、`fill` 负数（`native_memory.rs:83-162`）；`rava_coro` Drop 中 `expect`（`stack.rs:393,409`）。→ 已修，见五。
- 生成器 / 宏：宏分类两份实现且已分歧（`classify.rs:18-46` / `base_fns.rs:41`）；`java_try!` 全局计数器致展开不确定（`try_macro.rs:55`）；清单两套解析、IndyKind 两份枚举（`closure/src/manifest.rs` / `input/src/manifest.rs`）；class 名路径分量未校验（`emit/src/project/layout.rs`、`fs.rs`、`input/src/build.rs`）；描述符 / 注解递归无深度上限（已修，见五）；`Audit::FieldNpe` 无发射点；`resolve/src/jdk.rs:236-284` 测试在 `mod tests` 外且改进程环境；产物以绝对路径依赖 rava 仓库（`emit/src/ctx.rs:132`、`overlay.rs:137-141`）。
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

**遗留**

- 堆上基本类型数组经 `__update_bytes` 做跨元素读-改-写（例如在 `byte[]` 上经 int 字路径做 byte CAS）时，走 stripe 锁下的读-改-写，可能丢失同时发生的普通元素写。这不在本步「直接内存」范围内，需在 `array/store.rs` 用同样的字粒度原子单元解决。
- 直接内存还有 FFI 写入方（zlib 的 `inflate` / `deflate`、`read(2)` 写入直接缓冲区、`CStr::from_ptr(c_path(address))` 读路径），它们在 Rust 内存模型之外，与 Java 线程并发访问同一缓冲区属于 Java 程序自身的数据竞争。
- 为跑 Miri，本机装了 nightly 工具链及 miri、rust-src 组件（`~/.rustup/toolchains/nightly-*`，Miri sysroot 在 `~/.cache/miri`）。
