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
- 微基准改后仍约 43 µs / 迭代（JVM 为微秒以下），剩余开销在 `Method.invoke` 的字节码翻译路径（访问检查、参数装箱、`MethodAccessor` 链），未剖析；属后续性能项。
- 既有缺陷：`Method.invoke` 返回的装箱 `Integer` 经 `(Integer)` checkcast 抛 `ClassCastException: java/lang/Integer cannot be cast to java/lang/Integer`（`try_cast<Integer>` 失配，前后均复现），微基准改经 `hashCode()` 取值绕开；需另修。

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

- 健全性：`Handoff<T>` 无界 Send（`gil.rs:158`）；`native_memory` 批量拷贝与 CAS 混访、未对齐 CAS 退化、`fill` 负数（`native_memory.rs:83-162`）；`rava_coro` Drop 中 `expect`（`stack.rs:393,409`）。
- 生成器 / 宏：描述符 / 注解递归无深度上限（其余七项已由「一、生成器修复」处理）。
- 运行时其他：`runtime_impl.rs:44-50` 无条件读 `/proc`（macOS 恒 0）。
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
