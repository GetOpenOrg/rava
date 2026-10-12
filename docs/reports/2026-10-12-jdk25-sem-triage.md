# JDK 25 语义失败分诊（j25-sem，2026-10-12）

分支 `j25-sem`（基 batch-1013 @ adf4cc1f → 并入 `b1013-fix` @ abdd523b → 本分支修复 304edfb1），
worktree `/data/workspace/rava-wt/j25-sem`，作业结果目录（worktree 本地，gitignore）
`cluster_results/job/j25s-*`。

## 0. 背景与输入完整性

- 任务输入：2026-10-12 主会话 JDK25 失败集复跑（by_commit/c7cfb932684b 归档 468 例，跑在 main@99de3268）
  → **987 过 / 138 败**。138 清单在 `cluster_results/by_commit/99de32683615/failed_jdk25.txt`。
- **主检出的 `cluster_results/failed_tests_jdk25.txt` 与 `error_logs/` 已被 17:28 的 63_junit 大槽复跑覆盖**
  （只剩 3 条 junit OOM 日志），138 例的逐例错误日志不可恢复（dev 槽目录 e2e 清理亦无残留）。
  本轮分类据此改为**在 j25-sem 分支服务器复跑重建**（作业 tag `j25s-*`）。
- 99de3268→adf4cc1f 区间无语义修复（全为 S7 K3/K6、invoke-key-opt、plainseg-bisect、coerce-check 性能线），
  138 例中的语义失败在 adf4cc1f 上应仍失败，除非属 dev 档性能类（release 下转过，见 §3）。

## 1. 前置：批级编译回归（非本任务语义类，但污染一切探测）

batch-1013（adf4cc1f）有 K6 前置项装箱工厂 trait 化的批级 E0599 回归（`valueOf_z` 在
`boolean::Boolean` 上不存在，成片编译失败）——协调者预警并给出修复分支 `b1013-fix`（abdd523b）。
本分支已按指示先并入（`j25-sem = b1013-fix + 修复`）。污染证据：作业 `j25s-rerun` G3
（adf4cc1f，15 例）中 IQPuzzle / TestReflectProbe / TestMethodHandleCombinators /
TestSecurityManagerContext 4 例同签 E0599，其余 11 例为我方缺省 600s 转译超时（配置问题，
复跑已改 `--transpile-timeout 2400`）。**对 138 的归因须剔除这两族。**

## 2. 三类分类（138 例）

### 2.1 OOM 类（75 例 @14G 槽；只列不修，G1/主会话大槽线处理）

按任务书口径列出（逐例错误日志已失，名字级来源：任务书 + failure_patterns + 交接文档）：

- **63_junit 8 例**（22G/40G 槽峰值 39.6–39.9G 仍 OOM，闭包分析规模，G1 线在攻）：
  TestJunitAssertThrowsAssume、TestJunitFailureMessages、TestJunitBeforeClassState、
  TestJunitLifecycleOrder、TestJunitIgnoreAndExpected、TestJunitNewInstancePerTest、
  TestJunitTimeoutFast、TestJunitRunnerResult。
- **已知转译 OOM（transpile-oom-14g / 交接文档）**：DeepCopy、DataEncryptionStandard、Digester、
  TestHttpLoopbackSync、TestHttpLoopbackAsync。
- 其余 ~62 例为 14G 槽上限下的 OOM（含转译与编译段），逐例名不可恢复；本分支复跑（11G 作业槽上限，
  更紧）将继续把这些暴露为 OOM，作业 `j25s-rA/rB` 的 summary.json `oom` 字段即现名单（交接见 §5）。

### 2.2 已知已修 / 已知未修类（对照 failure_patterns 与交接文档认领，非本轮新修）

已修（patterns status=fixed/pending，复跑预期转过；rA/rB 正在出数）：
- unbox-root-virtual-stub（748e0987）：TestReflectBoxCast；
- direct-invoke-override-not-dispatched（677c07d4）：TestReflectFieldMethod；
- module-unnamed-handwritten（e50d4ba3）：TestModuleLayerDefine、TestSetAccessibleBoundary；
- transpile-stack-overflow（b4da5107）：TestDomResultNode；
- canonicalize-requires-existing-path（4308ba57）：TestFileCanonicalPaths；
- url-protocol-handler-by-name（c4-url）：TestUrlParsingFaces、TestUrlProtocolOpen、TestEmbeddedUrlRebuild；
- transitional-handwritten / carrier-thread-local-stub：TestRandomAccessFile、TestUnixFileNatives；
- offset-field-folded：AmicablePairs、RamanujanPrimes、TestCommonPool、TestParallelArrayCas；
- l3-dispatch-serial-callback / static-field-table：StockTrans 族；
- root-object-handwritten-frames（78af2fa8）：TestObjectWaitFrames（注意交接文档另有「JDK 版本输出差异」口径）。

已知未修（open，登记在案、非本轮范围）：
- `jdk25-stackframeinfo-reflective-element-flow`（G1/反射建模）：TestStackWalkerFrames、TestStackWalkerLines；
- `resource-bundle-by-name`：TestXmlSaxEvents（TestXmlTransform 混见 xsltc 已修与该项）；
- `native-unimplemented`（IOUtil.fdLimit 等）：TestHttpLoopbackSync/Async（与 OOM 叠加）；
- `debug-profile-too-slow`：FibonacciMatrixExponentiation、IQPuzzle、RailwayCircuit、TestVirtualThreadScale——
  **release 下 FractionReduction 实测已 PASS（j25s-rB G8）**，此族在 release 口径大量转过；
- 运行超时：TestArraysSetAll（j25s-rB G10 实测 run timeout >10m，同交接文档「运行超时」类）。

### 2.3 新语义类（本轮修复对象）

- **逃逸分析 call_virtual↔call_lambda 无限互递归（已修 304edfb1）**：
  症状 = JDK25 转译在闭包完成后 abort（`fatal runtime error: stack overflow`，RUST_MIN_STACK=128M 仍溢出）。
  实证：FunctionalCompositionTest（sg1，gdb 栈 `escape::analysis::Analysis::call_virtual(analysis.rs:485) ↔
  call_lambda(analysis.rs:549)` 数千层）。根因：OLD/FRESH_UNKNOWN 接收者上的 call_virtual 按 SAM 名遍历
  全部 lambda 站点，call_lambda 对 5|9 实现句柄直呼 call_virtual（cap=OLD 回落）——SAM 名与实现名互为对方
  （或单站点 sam==实现名）即无限环；JDK21 档案无此形态，JDK25 首次触达。修法：在途 lambda 站点键
  （owner,name,desc）判环，命中按未知被调保守返回（同 `Callee::Native` 口径），无环链不变。
  影响面：所有触发该环的 JDK25 用例（已证 FunctionalCompositionTest；rA/rB 复跑将继续认领同类）。
- **其余新语义簇：待 rA/rB 复跑日志聚类**（见 §5 交接）。

## 3. 复跑口径更正（重要）

主会话 138 复跑若为 dev 档（e2e 分发器缺省不传 `--release`），则 debug-profile-too-slow 族
（≥5 例）与部分超时例在 release 口径下会转过——本轮复跑统一 `--release`（任务书配方），
FractionReduction 已实证转过。分类与验收以 release 为准；dev 档差异在报告中单列。

## 4. 修复与验收状态（截至本报告落笔）

| 项 | 状态 |
|---|---|
| b1013-fix 并入（E0599 批级回归） | ✅ 已并入 j25-sem（abdd523b），双远端核对 |
| escape 互递归环判定（304edfb1） | ✅ 代码落分支；generator 本机 cargo check（--release --tests，heavy_lock）过；**JDK25 代表例 FunctionalCompositionTest 与 JDK21 回归（FunctionalCompositionTest/TestChmTransfer/TestHoistShadow，参考构建 21）验收作业 `j25s-fix1` 在跑**（2 命令，7 服务器池） |
| 138 例现状重建 | 🔄 `j25s-rA`（组 1–7）/`j25s-rB`（组 8–14）复跑中（127 例，宽超时 + keep-artifacts + 流式回显 + 周期打包）；已出：FractionReduction PASS、TestArraysSetAll run-timeout、FunctionalCompositionTest 栈溢出（已修） |

## 5. 交接（下一棒从这里继续）

1. 盯 `/data/workspace/rava-wt/j25-sem/cluster_results/job/{j25s-rA,j25s-rB,j25s-fix1}/summary.json`
   （勿裸 pgrep）。rA/rB 完成后：解包各 `NN/j25s2-g*-logs.tgz`，用逐例 run.log/build.log 聚类
   （聚合脚本雏形 `/tmp/j25s_collect.py`），把 §2.2 的「预期已修」坐实、新簇落 §2.3。
2. OOM 类现名单 = rA/rB summary 的 `oom` 条目 + §2.1 已列名字，交主会话大槽批量重跑。
3. j25s-fix1 两作业 rc=0 即代表例验收完成；若 FunctionalCompositionTest 仍败，读
   `build/j25sfix-a.out`（服务器检出 `/data/rava-spot-job-j25s-fix1`）。
4. 派发器均为 setsid 脱离进程，本子代理结束后继续运行；如需停止：`pkill -f "job j25s-rA"` 等。
5. 未竟事项：junit 8 例 OOM（G1 线）、HttpLoopback native 缺口、resource-bundle-by-name、
   StackFrameInfo 反射流（G1 线）均维持 open 登记，不在本分支修复。
