# 档案 crate 分层：决策记录与待验证清单（2026-10-04）

本文汇总 2026-10-04 主会话关于 crate 拆分、档案链接、引擎提速的讨论结论与用户决策，以及后续必须验证的事项。
细节方案归各自计划，本文只做索引与验收清单：

- 声明层瘦身 / S7：[`2026-10-01-rustc-memory-and-crate-split.md`](2026-10-01-rustc-memory-and-crate-split.md) §7（crate-split 线）
- 档案化与直接 rustc 链接：[`2026-10-01-cross-test-compile-reuse.md`](2026-10-01-cross-test-compile-reuse.md)、[`2026-10-04-t1-step2-direct-rustc-link.md`](2026-10-04-t1-step2-direct-rustc-link.md)（t1-link 分支，待合入）
- 引擎提速与顺序无关性：[`2026-10-03-jndi-transpile-perf.md`](2026-10-03-jndi-transpile-perf.md)

## 一、用户决策（2026-10-04）

| # | 事项 | 决定 |
|---|---|---|
| D1 | 语料模式档案 dylib 的 panic 策略 | `panic=unwind`（stable Rust 下 abort 两条路线均被 rustc 拒绝；panic hook 先 `exit(101)`，可观察行为不变）。推翻 X1a 中语料模式的 abort 部分，生产模式不变 |
| D2 | 生产模式（静态链接）LTO | 开发构建无 LTO；`--release` 用 fat LTO |
| D3 | 档案 crate 命名 | 按 JDK jmod 模块名（`java.base` → `java_base`、`java.net.http` → `java_net_http`…），从 JDK 模块描述动态取得，生成器不写字面量 |
| D4 | 引擎 ≤60 s 路线 | 先修闭包结果与遍历顺序无关（engine-order 线），再另起一步做逃逸对象上下文收拢（存入全局可达容器的对象折叠为单一无上下文池） |
| D5 | URL 协议可靠口径 | 先做引擎提速，偏差暂时容忍（c1d-urlhost WIP 保留，提速达标后启用） |

## 二、设计结论（讨论中确立，实施时遵守）

1. **拆分数量确定、与机器无关。** crate 切分由模块结构（及固定体量预算）决定，同一档案键在任何机器上生成逐字节相同的 crate 布局；
   不按本机内存调整拆分数量，否则内容摘要不一致、档案缓存与分发失效。**按资源调整的只有并行度**（`CARGO_BUILD_JOBS`）。
2. **拆多不等于省内存。** 每个实现 crate 编译都要加载声明层元数据，单 crate 峰值 ≈ 声明层元数据 + 自身方法体；并行编译时总内存约为同时在编各 crate 峰值之和。
   峰值下限是声明层，正确方向是让声明层变小（§7.5.4 第 1–7 项 + S7），而不是继续加 crate 数。
3. **模块间可拆，大模块内部现状不可拆。** JDK 模块图（`requires`）无环，声明层可按模块切成多个 crate 并按拓扑编译；
   `java.base` 内部类型引用成环（字段类型、方法签名、From / checkcast 互指），Rust 不允许 crate 循环依赖，现状下其声明层只能是一个 crate，是整个构建的峰值下限。
4. **档案直接依赖使用后**：逐例编译只读档案元数据（目标单例峰值 ≤ 0.8 GB、p50 ≤ 1 s，见 t1-link 方案），OOM 风险集中到「每档案键一次」的档案构建；档案声明层瘦身仍必须做。
5. **档案不是完整 jmod**：档案是全体入口调用链的并集，链外方法为存根（CLAUDE.md 第 2 条），档案规模由入口决定，不由模块体量决定；生产模式大项目的档案更大，S7 必要性更高。
6. **第三方依赖沿用同一模型**：JDK 模块 crate → 第三方库 crate（按 artifact / 自动模块名，键含库版本）→ 用户 crate；Maven 依赖图通常无环，同样可按库切分。现有计划未覆盖，见 §四 V12。

## 三、实测与更正

- TestHttpLoopbackSync 闭包变慢的起点是 51a4d8c5（c1d-p0 JCA，SunJSSE / SunRsaSign 登记）：HttpClientImpl 调 `SSLContext.getDefault()`，JSSE / JCA 入闭包属合理增长（3590 → 5404 类），引擎耗时增长快于规模。
- 引擎提速 A/B/C（http-perf 0220249a）：服务器 TestHttpLoopbackSync 630 s；ubuntu 上 TestHttpLoopbackAsync 1110 s / 14.7 GB → 607 s / 9.1 GB；类 / 方法集不变。集合保持的改造到不了 ≤60 s，剩余为有效工作量（1.56 G 次元素插入、221k 方法上下文）。
- 更正：此前「DeepCopy 4131 类」口径有误，闭包类数为 3153 → 3436 逐步增长，无单次膨胀。
- 档案 dylib（t1-link 实测，本机 arm64）：3,077 类档案上逐例 0.58–0.96 s / 455–507 MB（去掉 main.rs 档案侧登记表后）；静态 rlib 逐例峰值 2.3–2.6 GB；档案 dylib 链接峰值 3.48 GB、导出 65 万符号（逐例固定约 0.4 s 链接）。
- 单 crate 档案不可行：s2 `java_runtime` rlib 有 31,048 个未定义 `__rava_` / `__java_meta_` 符号（声明层 ↔ body / meta 循环）。

## 四、待验证清单

| # | 事项 | 归属 | 验收 |
|---|---|---|---|
| V1 | 按模块切 crate 是否成立：跨模块边是否只沿 `requires` 方向；手写 runtime、java_meta 登记、ObjectVTable、反射表等全局表的归属；反向边清单与消除手段 | t1-link | 反向边 = 0 或逐条有消除方案 |
| V2 | 按模块切分能否消除 31,048 个未定义符号循环；语料模式每模块一个 dylib 还是一个 dylib 含多模块，对逐例链接（导出符号量）与档案构建峰值（现 3.48 GB）的影响 | t1-link | 实测数字 |
| V3 | body 切分轴由「体量均衡」改为模块，与 crate-split 的接口 | crate-split + t1-link | 方案一致 |
| V4 | 按现规模重测 OOM 三例（TestFieldHandleProvenance、TestJndiNoProvider、TestSerialDefaultSuid）各 crate 峰值与构成，更新 §7.5.4 达标账 | crate-split | 三例服务器编过；HelloWorld 墙钟不变差 |
| V5 | S7 后 `java.base` 声明层能否按剩余引用图（继承 DAG + 具名签名）的强连通分量再拆；孤儿规则约束；可读层（`let animal: Animal = Dog::new()`）不受损 | crate-split（S7 方案） | HelloWorld / Digester 档案上实测 SCC 规模、各 crate 峰值估算；不能拆则给出声明层下限 |
| V6 | §7.6 目标：声明层 Digester ≤ 2 GB、HelloWorld ≤ 1.2 GB；每实现 crate ≤ 1.5 GB；HelloWorld `cargo build` ≤ 12 s；只改用户类时 JDK 重编 0 | crate-split | 实测 |
| V7 | Linux 侧 dylib：rpath `$ORIGIN`、rust-lld 链接、`readelf` 依赖条目、unwind 下 panic 退出码仍为 101 | t1-link 实施期 | 服务器实测 |
| V8 | 档案侧登记表（每个 main.rs 约 1370 行）移入档案模块 crate 后用户 main.rs 档案登记行 = 0 | t1-link L1 | 逐例峰值 ≤ 0.8 GB、p50 ≤ 1 s、p99 ≤ 3 s |
| V9 | 顺序无关性：DeepCopy、StockTrans、TestSerialLookupPairing（含宏访问器探针形态）、TestHttpLoopbackSync 种子 0/1/2 类集合与方法集合完全相同；`pvals` Const 字面量化修复 | engine-order | 多种子集合一致 |
| V10 | 逃逸对象上下文收拢（V9 之后）：TestHttpLoopbackSync 服务器闭包 ≤ 60 s；闭包集合变化逐项论证 | 引擎后续步 | ≤ 60 s，HTTP 两例通过 |
| V11 | 引擎达标后启用 URL 协议可靠口径（c1d-urlhost 9087cf1c），TestFileUrlHost 入集成分支 | URL 线 | 抽查通过 |
| V12 | 第三方依赖分层与复用（键含库版本、按 artifact / 自动模块名切 crate、无 module-info 的 jar 归属）写入 T1 后续步骤方案 | 待派 | 方案经用户确认 |
