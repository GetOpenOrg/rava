# rava 代码审查 · 动态验证报告（2026-10-11）

> 对两轮静态审查（[第一轮](2026-10-11-code-review.md) · [第二轮](2026-10-11-code-review-round2.md)）的关键发现做实际运行核实：从零构建全部测试套件、跑 e2e 金标对照、用定制探针复现/证伪具体缺陷。
>
> **验证基线：main @ f59eb9e4**——比审查快照（177c88a8）新 **134 个提交**（团队当日合入 batch-1010r/1011x 系列）。两处「测试在 main 上红」类结论的差异由此解释，见 §二.5/§二.6。

## 一、测试基线：当前树全绿

| 套件 | 结果 |
|---|---|
| generator 全量单测（`cargo test --release`，12 crate，约 618 个测试） | **exit 0，全部通过，0 失败**（含 closure 243 例、driver closure_cli 全矩阵、emit/instr/ir/… 全部） |
| Python 单测（tests/unit 根级 + cluster，经 uv） | **98/98 通过**（40 + 58） |
| runtime/rava_coro（lib + panic/overflow/pins/registers/roundtrip/rss） | **18/18 通过**（含 panic 跨协程传播子进程重入、栈溢出 guard、百万次往返、RSS 回收） |
| runtime/rava_macros_core / rava_meta_tables | **16/16、2/2 通过** |
| runtime/java_runtime | **无法独立测试**（`cargo test` 编译失败：`src/java`、`src/jdk`、`src/sun` 模块缺 mod 文件——模块结构由 e2e scratch overlay 生成）——证实第一轮「java_runtime 只能经 e2e 验证、单测近乎为零」的结构性结论 |
| mini-e2e 金标对照（手工 `rava build` → 运行 → diff expected） | **10/10 全过**：HelloWorld、ArrayCopyDemo（正常路径）、NullStringTest、ArrayOutOfBounds、IntegerToStringRadixTest、ClinitOrder、TestSwitchString、TestArrayBounds、InheritanceChain、**TestJucSync（真线程 + JUC 同步）** |

端到端性能顺带观测：单例 `rava build`（javac→闭包→发射→scratch cargo 编译→运行）墙钟 **46.7 秒**（HelloWorld 级、与性能文档口径一致）；readability 审计实测输出全 0，与门禁声明一致。

## 二、审查发现的逐项动态核实

### 1. [P1→确认且加重] System.arraycopy 缺前置范围检查（第二轮 4.2 #1）

**探针**：`ArrayCopyProbe.java`（5 个 case），参考 JDK 21.0.11 产 JVM 金标 vs rava 产物输出：

| case | JVM（金标） | rava | 判定 |
|---|---|---|---|
| length=-1 | IOOBE `arraycopy: length -1 is negative` | **无异常**，d0=10 | 静默吞错 |
| srcPos=2,len=2（越界 1） | IOOBE `arraycopy: last source index 4 out of bounds for int[3]`，d2 未改 | IOOBE `Index 3 out of bounds for length 3`，**d2=3,20,30（已写入）** | 异常类型对、消息错 + 部分写入 |
| len=5（双侧越界） | IOOBE，d3 未改 | IOOBE，**d3=1,2,3（几乎全写）** | 部分写入 |
| srcPos=-1,len=0 | IOOBE `arraycopy: source index -1 out of bounds for int[3]` | **无异常** | 静默吞错 |
| 正常复制 | 1,2,3 | 1,2,3 | OK |

**比静态审查更严重**：不仅负 length 静默、负 srcPos+零长度也静默；越界场景目标数组被部分改写（JVM 保证 IOOBE 时零写入）。**盲区根因确认**：语料 `01_basics/ArrayCopyDemo.java` 只覆盖正常/重叠/零长度路径，无边界用例。该探针可直接转为 e2e 回归用例（探针保留在 `/tmp/rava-verify/ArrayCopyProbe.java`，五 case 全部差分可判）。

### 2. [P2→确认] Class.forName(null) 异常类型（第二轮 4.2 #6）

rava 产物输出 `ClassNotFoundException: null`，JVM 抛 `NullPointerException`——静态发现属实。

### 3. [P2→补上见证，行为正确] string switch 哈希冲突分支（第二轮 4.1 #3）

`"Aa"/"BB"`（同 hashCode 2112）+ `"CC"/"Dd"` 探针：rava 输出与 JVM **逐行一致**（Aa→Aa、BB→BB、CC→CC、Dd→default）。审查判断「实现大概率正确、只是无金标见证」成立——现在有见证了，探针建议入库。

### 4. [P3→确认行为、修正严重度] lookupswitch 撒谎长度「DoS」（第二轮 4.10）

构造：合法 lookupswitch 类（npairs=3）→ 十六进制改 npairs=500,000,000 → 喂 `rava closure`：

- 正常内存环境：闭包**优雅完成**（1.92s，RSS 446MB）——类被记入 `closure.json` 的 `missing` 列表，分析继续，无崩溃、无 OOM。
- `ulimit -v 2GB` 下：`memory allocation of 4000000000 bytes failed` → **实锤 `Vec::with_capacity` 确实信任撒谎计数**（4×10⁹ 字节分配尝试）。

**结论**：代码行为与静态分析一致（先分配后校验属实），但严重度应从「DoS（本地开发机可直接被 OOM）」**降级为加固项**：默认 overcommit 下这只是瞬时虚拟预留（with_capacity 不触碰页），首个截断读取即 Err 释放；仅在禁用 overcommit 或设虚拟内存限额的环境下才 abort。修复仍建议做（分配上限取 `min(n, remaining()/8)`，一行），排期为加固而非事故。

### 5. [P2→已失效（快照漂移）] TLS lint 在 main 上红（第一轮并发 #4）

`thread_locals_only_in_carrier_slots` 在 f59eb9e4 **通过**（0.00s）。审查快照 177c88a8 上的失败是真实的（修复批次 1010r 未合）；验证期间团队合入的 134 提交包含了修复。**方法论教训**：所有「main 上红/既有」类结论必须钉住 commit 快照。

### 6. [→已失效（同上）] order/hash_seed 回归

`closure_independent_of_order`、`closure_independent_of_hash_seed` 均为普通 `#[test]`（非 ignore），已包含在全量套件 exit 0 中——当前树通过。治理审查记录的「8.5 小时/5 次合入滞留窗口」是历史事实，与当前绿并不矛盾。

### 7. [P1→确认（结构级）] resolve/jdk.rs 错位测试（第一轮 Rust #1）

`cargo test -p resolve` 中该测试以路径 **`jdk::choose_pin_wins_over_mismatched_java_home`** 运行（正确位置应为 `jdk::tests::…`）——直接证实它位于 `#[cfg(test)]` 模块之外。正常构建的 rlib 为纯 bitcode（nm 不可读），符号级「编进产物」确认不可行，但源结构 + 测试路径证据已充分。机械修复建议不变。

### 8. 其他确认项

- **java_runtime 独立不可构建**（E0583×3）→ 已在 §一记录，证实「151 个 .rs 仅 4 个单测」的成因是结构性依赖 scratch overlay。
- **产物清理行为**：`rava build` 运行完产物后默认删除二进制（除非 `--keep-artifacts`）——验证循环首跑 rc=127 的根因，属预期行为，DX 文档可提一笔。
- **`rava closure` 对畸形类的容错**：坏 magic → 明确报错（`用户类中没有 static main`）；可解析但方法体坏 → 记 `missing` 继续。不可信输入的整体降级路径健康。

## 三、过程中发现的新事实

1. **共享机器竞争**：验证期间团队批次自动化（b1011d/e spot 作业）并行使用本机，closure 全矩阵测试与它们争 CPU，全量套件耗时约 1.5 小时（平时应显著更短）。
2. **两轮审查的「快照漂移」风险实例化**：134 提交在一天内合入，两处「测试红」结论在验证时已被修复——审查报告已注明快照，后续审查应把「钉住 commit」作为硬要求。
3. run_tests.py 的 `_ensure_rava` 与手动 cargo 构建共享 target 目录锁，测试与跑批不能并行——日常使用需串行（与工具链审查的锁竞争发现一致）。

## 四、结论与建议

- **正确性主结论**：当前树（f59eb9e4）单测全绿、e2e 抽样全过、语义探针除已报告缺陷外全部与 JVM 一致——两轮审查「主线质量高、缺陷集中在已识别的具体点」的判断得到动态印证，未发现新的未知缺陷类别。
- **最优先可执行项**：System.arraycopy 前置检查（探针已就绪可作回归）；建议随修复把 ArrayCopyProbe 与 StringSwitchCollision 探针转正为 e2e 语料（各有明确差分判据）。
- **降级项**：lookupswitch/tableswitch 预分配从「DoS」降为「加固」；TLS lint、order/hash_seed 从待办清单移除（已修复）。
- **流程项**：审查/验证结论一律钉 commit 快照；「main 既有」机器基线（治理审查 Top1 建议）的价值被本次快照漂移再次证实。
