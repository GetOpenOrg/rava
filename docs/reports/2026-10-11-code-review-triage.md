# 2026-10-11 代码审查分诊

> 对象：[第一轮](2026-10-11-code-review.md) · [第二轮](2026-10-11-code-review-round2.md) · [动态验证](2026-10-11-code-review-verification.md)。
> 核实基线 dcc1e8e1（只读核对代码）。分诊口径：10-11 主线是框架编译测试与 JDK API 翻译，审查项只即派**影响测试可信度**与 **Java API 语义正确性**两类，其余记待办。

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

## 二、待办（不派，主线空档或相关改动顺手做）

- 健全性：`Handoff<T>` 无界 Send（`gil.rs:158`）；`native_memory` 批量拷贝与 CAS 混访、未对齐 CAS 退化、`fill` 负数（`native_memory.rs:83-162`）；`rava_coro` Drop 中 `expect`（`stack.rs:393,409`）。
- 性能：`Class.for_class` 每次 `format!`（`class_impl.rs:55-83`）；反射分派全局锁（`reflect_dispatch.rs:46-88`）；ldc 字面量每次重新驻留（`string_ext.rs:141`）；`vm_throw` 每次读 `RUST_BACKTRACE`（`error.rs:34-38`）。
- 生成器 / 宏：宏分类两份实现且已分歧（`classify.rs:18-46` / `base_fns.rs:41`）；`java_try!` 全局计数器致展开不确定（`try_macro.rs:55`）；清单两套解析、IndyKind 两份枚举（`closure/src/manifest.rs` / `input/src/manifest.rs`）；class 名路径分量未校验（`emit/src/project/layout.rs`、`fs.rs`、`input/src/build.rs`）；描述符 / 注解递归无深度上限；`Audit::FieldNpe` 无发射点；`resolve/src/jdk.rs:236-284` 测试在 `mod tests` 外且改进程环境；产物以绝对路径依赖 rava 仓库（`emit/src/ctx.rs:132`、`overlay.rs:137-141`）。
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
