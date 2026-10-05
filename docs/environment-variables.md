# 命令行选项与环境变量

> rava **不设项目自有环境变量**（2026-09-28 清理：原 `RAVA_*` 共 14 个全部改为命令行参数、生成文件或标准变量）。
> 新增开关一律做成命令行参数；需要传给 cargo 构建的内部数据写入 scratch 的生成文件，由 `build.rs` 读取。
> 唯一的项目变量 `RAVA_REFJDK_ROOT` 属于语料编排层（脚本读取，rava 二进制不读），见第三节。

## 一、命令行选项

### `rava build` / `rava compile` / `rava emit` / `rava jdk`（转译入口，`generator/crates/driver`）

rava 二进制在 `build/analyzer-target/release/rava`（`cargo build --release -p driver --manifest-path generator/Cargo.toml
--target-dir build/analyzer-target`；`run_tests.py` 与各 shell 脚本每批开头自动构建，新鲜时为空操作）。手写层
`runtime/java_runtime` 由可执行文件位置向上查找（`--runtime` 可覆盖），仓库根即其上两级。

```bash
RAVA=build/analyzer-target/release/rava
$RAVA build tests/e2e/01_basics/HelloWorld.java                         # 转译 + 编译 + 运行
$RAVA build Foo.java --stop-after emit --trace-class java/security/Provider
$RAVA build tests/e2e/04_collections/TestArrayList.java --stop-after emit --raw-sites /tmp/raw.txt
$RAVA build tests/e2e/01_basics/BubbleSort.java --strict
$RAVA build tests/e2e/01_basics/HelloWorld.java --stop-after emit --closure-json
$RAVA compile build/hello_world --release                              # 编译已发射的 scratch
$RAVA emit build/hello_world/closure_input/closure.json --jdk 21 --java tests/e2e/01_basics/HelloWorld.java
$RAVA jdk --json                                                       # 选中的 JDK：{home, major, source}
```

| 选项 | 用途 |
|---|---|
| `build <A.java>…` | javac（`-g`，JDK ≥ 14 加 `--enable-preview --release N`）→ 闭包分析（同 `rava closure`）→ 闭包事实进程内直传 → 发射 scratch →（缺省）`cargo build --bin <入口 snake 名>` 并直接执行产物（`CARGO_TARGET_DIR` 缺省 `<仓库>/build/target`、`CARGO_INCREMENTAL=0`）。用户类落 `<scratch>/closure_input/`；`closure.json` 只在 `--closure-json` 时写出 |
| `compile <scratch>` | 编译 `build --stop-after emit` 已发射的 scratch（bin 名与重型判定输入读自 `build_status.json` 的 `emit` 段），更新 `build_status.json` / `build_artifacts.json`；接受 `--release` / `--dev-opt` / `--target-dir` / `--build-timeout` / `--keep-artifacts` / `--runtime`。`run_tests.py` 先并行发射、再逐例用它编译，发射进程不必常驻等待共享 target 的 cargo 文件锁 |
| `prune <scratch>…` | 删除 scratch 的 `build_artifacts.json` 登记的剩余编译产物（运行完成后调用，见 `--keep-artifacts`）；清单只含本 scratch 的包，共享依赖不动 |
| `jdk` | 打印 JDK 选择结果与来源（`--home-only` 只打印 home，供 shell 取 `JAVA_HOME`；`--json` 打印 `{home, major, source}`；`--list` 列已安装版本） |
| `emit <closure.json>` | 由既有 `closure.json` 与用户类目录重建输入后发射（不编译运行）。`--classes DIR` 缺省 `closure.json` 同目录的 `classes/`；`--java A.java`（可多次）给出源文件，决定用户类包布局与入口序 |
| `--jdk N` / `--java-home P` | JDK 选择（互斥；缺省依次：有效 `JAVA_HOME` → 仓库根 `.jdk-version`（当前 21）→ 已安装最新版；`--jdk N` 无该主版本时报错。扫描 `HOMEBREW_PREFIX`、`/opt/homebrew`、`/usr/local` 的 Cellar、`/usr/lib/jvm`，macOS 另回退 `/usr/libexec/java_home`） |
| `--runtime R` | 手写层真源 `runtime/java_runtime`（缺省自可执行文件所在目录向上查找，其次当前目录） |
| `--out DIR` | scratch 目录。build 缺省 `<仓库>/build/<入口 snake 名>`；emit 缺省为 `closure.json` 所在 `closure_input/` 的上级 |
| `--image D` | 镜像独有 / VM 支持类目录（可多次；缺省由 rava 自行派生，见 `resolve::image`） |
| `--main 类` / `--locale L` / `--root 类.方法:描述符` | 仅 build：入口类（缺省首个源文件的同名类，否则首个带 static main 的用户类）/ locale 种子 / 外部种子方法（均可多次） |
| `--clean` | 发射前清空 scratch（emit 的输入位于 scratch 内时拒绝） |
| `--stop-after STAGE` | 仅 build：最后执行的阶段 `javac` / `closure` / `emit` / `compile` / `run`（缺省 `run`）。`emit` = 只生成（不编译运行）；`compile` = 生成并 `cargo build`，产物清单写 `<scratch>/build_artifacts.json`（`{bin, executable, paths}`，仅本 scratch 的产物），rustc 全文写 `<scratch>/logs/build.log`。每次 build 都写 `<scratch>/build_status.json`：`{stage, ok, exit, signal, timeout, first_error, log, jdk:{home,major,source}, heavy, emit:{bin,jdk_classes,precheck}, exe}`（stage = 停止或失败的阶段；`emit.precheck` = 预检全量明细 `{native_missing_count, boundary_stub_count, native_missing[], boundary_stub[]}`，不受 stdout 明细 40 条上限影响，`rava compile` 原样保留；`exe` = 编译成功的可执行文件）。编译环境见下文「重型闭包的自动处理」 |
| `--build-timeout 秒` | cargo 编译超时（build / compile；缺省重型闭包 3000 秒，其余 600 秒），到时终止整个 cargo 进程组，`build_status.json` 记 `timeout: true`；build 下须配合 `--stop-after compile` 或 `run` |
| `--release` / `--target-dir D` | build / compile：release 配置编译 / 共享编译缓存目录（缺省 `<仓库>/build/target`）；build 下须配合 `--stop-after compile` 或 `run` |
| `--release-small` | build / compile：体积档（cargo profile `release-small`，产物在 `<target>/release-small/`）：继承 release（fat LTO、codegen-units 1、strip），opt-level `"s"`；`run_tests.py --release-small` 同义。档位取舍与实测见 `docs/plans/2026-10-04-binary-size.md` B3 |
| `--dev-opt` | build / compile：性能类测试档（cargo profile `dev-opt`，产物在 `<target>/dev-opt/`）：继承 dev 语义，档案侧 crate 与第三方依赖 opt-level 1、用户 crate 0；与 `--release` / `--release-small` 互斥。e2e 用例在源文件中以独占一行的 `// rava-build-profile: dev-opt` 声明，`run_tests.py` 缺省 dev 档时按声明改走该档（`--release` 时不变） |
| `--keep-artifacts` | build / compile：保留本例编译产物。缺省 compile 链接成功后即删除本例专属中间产物（`deps/` 下同 `<crate>-<hash>` 单元的 rlib / rmeta / .d、`.fingerprint`、`build/<crate>-<hash>/`），只留可执行文件；build 的 run 段之后再删可执行文件（批量编排运行后调 `rava prune`）。归属读 `build_artifacts.json`，跨例共享依赖（`rava_macros` 等）不在清单内、永不删除。需要复用编译缓存（同一 scratch 反复编译）时打开 |
| `--strict` | 严格模式：转译兜底改为硬失败；缺手写实现的 native 方法编译报错（写入 scratch 的 `java_runtime/strict.txt`，`build.rs` 读取） |
| `--lib NAME=JAR[:seed=FQN,…]` | 仅 build：jar 输入模式（可多次，声明序即 crate 依赖序）。jar 上 javac `-cp` 与类路径；无 seed = 整包（jar 全部类进 lib crate，种子 = 全部类的 public 方法），有 seed = 子集（种子类须在 jar 内，只收闭包触达的 jar 类）。每个 lib 一个 `crate-type = ["lib"]` 的 crate：public / protected → `pub`，其余 → `pub(crate)`；user 依赖全部 lib。与 `--batch` 互斥 |
| `--batch` | 仅 build：入口写 `user/src/bin/<bin>.rs`（`#[path]` 引用同级类文件），向 `user/Cargo.toml` 追加 `[[bin]]`（已有同名 bin 跳过） |
| `--trace-class 类` | 仅 build：打印该类或方法（斜线形态，如 `java/net/InetAddress`、`类.方法:描述符`）入闭包的最短 provenance 链（`      [why] …`，同 `rava closure --why`），回答“为什么被拉进闭包” |
| `--debug` | 闭包未解析调用（`[closure] unresolved: …`）与存根兜底逐条（`[cfg-audit] stub fallback (位点): 方法: 原因`） |
| `--full-precheck` | 发射后只输出完整预检明细（`[precheck]` 不截断），不出审计行；build 下须配合 `--stop-after emit`。缺省时预检每类明细封顶 40 行 |
| `--api-package P` / `--api-recursive` | 仅 build：以公开 API 包为调用链入口（包内 public 类的 public / protected 方法，边界域包跳过；可多次）。`--api-recursive` 含子包，须配合 `--api-package`。输出 `[api] …` 行（`rava audit api` 同一入口枚举） |
| `--raw-sites FILE` | Raw 逃生舱构造位点剖面追加写入 FILE（FS-Q1 热点排序），不影响生成代码。行格式 `{次数}\t{种类}\t{位点}`，次数降序；位点为构造调用处 `文件:行:列`（`#[track_caller]`），种类 `raw_expr` / `raw_stmt` / `raw_item` |
| `--closure-json` | 仅 build：另写出 `<scratch>/closure_input/closure.json`（`rava emit` 与动态对照的输入），并校验 `ClosureFacts::from_json` 与 `ClosureFacts::from_closure` 的结果逐字节一致（`Debug` 文本，不一致即转译失败）。缺省不写，同时删除该处上轮遗留的 closure.json。`run_tests.py` 开动态对照时自动加上，`gen_trees.sh` / `emit_bench.sh` 也会加 |
| `--closure-cache DIR` | 闭包分析跨运行结果缓存目录（缺省 `<仓库>/build/closure_cache`，无需配置）。键覆盖分析器可执行文件内容、JDK jmods 与镜像目录、`runtime/java_runtime` 全部文件、用户类与 `--lib` 内容、全部影响结果的参数（入口、`--root` / `--seed-class` / `--api-package` 展开后的种子、`--locale`）与条目格式 / closure.json 折叠点版本；任一变化即不命中。命中时诊断行原样重放、`closure.json` 除 `summary.elapsed_ms` / `summary.perf` 外与冷算逐字节相同，事实经 `ClosureFacts::from_json` 交给发射。冷算时（启用缓存即）总校验 `from_json` 与进程内直传一致。条目损坏即删除重算；`--trace-class`（及 `rava closure` 的 `--why` / `--flows` / `--report`）需引擎本体，不读缓存（冷算结果仍写回）。`rava closure` 同样接受本组选项 |
| `--closure-cache-max-mb N` | 缓存目录总量上限（缺省 4096），写入后超出即按最近使用时间从旧到新淘汰 |
| `--emit-jobs N` | 按类并行发射的线程数（缺省 0 = 可用核数；1 = 串行）。输出与串行逐字节一致 |
| `--perf` | 输出 `[perf]` 分阶段耗时、峰值 RSS 与逐类 / 逐方法耗时 Top-N |
| `--cut E` / `--cut-file F` / `--dump-edges F` | 仅 build：闭包归因诊断（缺省关闭），同下 `rava closure`；切除会改变闭包结果，仅供归因 |

### `scripts/run_tests.py`（e2e 跑批）

| 选项 | 用途 |
|---|---|
| （缺省） | 语料 JDK = **参考构建**（`tools/refjdk.toml`，当前 Temurin `jdk-21.0.11+10`），经 `scripts/fetch_reference_jdk.sh --check` 定位、以 `rava jdk --java-home` 注入；javac / java / jmods / golden JVM（`--update-expected`）/ 动态对照全部同源。未就位即报错并提示取包命令，**不回退**系统 JDK；环境 `JAVA_HOME` 不参与选择 |
| `--jdk N` / `--java-home P` | 实验覆盖（互斥）：改用本机 JDK N / 指定 JDK home。`[jdk]` 与 `[meta]` 行标记「非参考构建」，结果不与 expected 同源 |
| `--show-jdk` | 只解析并打印本次语料 JDK 后退出（干跑，不跑测试） |
| `--build-timeout SEC` | 单测试 cargo 构建超时，透传 `rava compile`（缺省由 rava 按重型判定：3000 / 600 秒） |
| `--debug` / `--strict` | 透传给每个测试的 `rava build` |
| `--deny SPEC` | 审计计数非零升级为整体失败（`equiv` / `fallback` / `stub-hit` 等，见 `--help`） |
| `--no-dyn` | 关闭动态对照（缺省开，见下） |

每例两段：`rava build --stop-after emit`（并行模式下各例并行）→ 动态对照 → `rava compile`（逐例串行），结果读
scratch 的 `build_status.json`（超时 / 信号 / 首错行 / 日志路径）与 `build_artifacts.json`（通过例的产物清理）。

**动态对照**（闭包计划 C5，`scripts/dyn_compare.py`）：每个测试转译成功后（cargo 之前，`--no-run` 下同样执行）
用 JVMTI agent + `-Xlog:class+load,class+init` 跑一次原始 Java 程序，把 JVM 实际加载的类与 `closure.json` 对照。
结果行附 `dyn miss N / extra M prov P%`（翻译域漏覆盖 / 静态多出 / 多出类 provenance 覆盖率；有无调用栈的加载时再附
`unattr K`），明细落盘 `build/jdk<N>/logs/dyn/<test>.json`，汇总行 `[dyn-compare]` 列出全部漏覆盖。
每测试一次 java 运行（实测约 0.1 秒，相对 5–70 秒的转译可忽略），故缺省开启。agent 首次使用时以 `$CC`（缺省 `cc`）
编译，按源码 + JDK 缓存于 `build/dyn_agent/`。单独运行：`python3 scripts/dyn_compare.py build/jdk21/<test> [-o out.json]`。

启动时的 `[meta]` 行打印语料 JDK（`jdk=<tag>(参考构建)` 或 `jdk=<home>(非参考构建)`）、`CARGO_INCREMENTAL`、
`CARGO_BUILD_JOBS` 与透传选项，便于事后解读结果。

### `scripts/fetch_reference_jdk.sh`（语料参考 JDK 取包，方案 `docs/plans/2026-10-03-reference-jdk-21.md`）

```bash
scripts/fetch_reference_jdk.sh            # 确保当前平台的参考 JDK 就位（下载 + sha256 校验 + 解压，幂等），stdout 打印 JAVA_HOME
scripts/fetch_reference_jdk.sh --check    # 只检查：就位打印 JAVA_HOME，否则退出码 1 并提示取包命令
scripts/fetch_reference_jdk.sh --tag      # 清单 tag
```

| 选项 | 用途 |
|---|---|
| `--root DIR` | JDK 根目录（优先于 `RAVA_REFJDK_ROOT`；缺省主检出的 `tools/refjdk/`，git worktree 共用一份） |
| `--platform P` | 清单平台段（`linux-x64` / `linux-aarch64` / `macos-aarch64` / `macos-x64`；缺省按 `uname` 判定） |

落位 `<根>/<tag>/` 即 JAVA_HOME（macOS 包的 `Contents/Home` 提升为该目录），`.rava-refjdk` 记录 tag / 平台 / sha256，
与清单不符即视为未就位。同一根目录多进程并发取包以 `<根>/.lock-<tag>` 互斥。语料 shell 脚本（`gen_trees.sh`、
`seed_check.sh`、`profile_closure.sh`、`lib_pilot_golden.sh`、`closure_bench.sh`、`emit_bench.sh`）经
`scripts/corpus_jdk.sh` 取同一参考构建，环境变量 `JDK=N` 为实验覆盖（标记非参考构建）。

### `rava closure`（闭包分析器，`generator/crates/driver/src/closure_cmd.rs`）

| 选项 | 用途 |
|---|---|
| `<Test.java \| 类目录>` | 输入；`.java` 先经 javac 编译到临时目录 |
| `-o closure.json` / `--report md` | 闭包结果 / 报告 |
| `--why 类或方法` / `--flows 查询` | 入闭包的 provenance 链 / 类型流诊断（均可多次；查询形式见下表） |
| `--lib jar` / `--image D` / `--root M` / `--seed-class C` / `--locale L` / `--release P` / `--release-bytecode P` | 转译接入与放行实测（均可多次） |
| `--cut 类.方法:描述符[@偏移]` | 反事实切除（可多次）：不带偏移 = 方法体不处理（节点保留）；带偏移 = 该偏移处的调用 / 字段 / new 事件不执行。只宜切消费型节点（方法体、派发点），切构造器 / 写入点会让字段按初值折叠，结果非单调 |
| `--cut-file F` | 切除条目文件（每行一条，空行与 `#` 注释跳过；条目多时用） |
| `--dump-edges F` | 触发边转储：方法（`M:`）/ 类型提及（`C:`）/ 类初始化（`I:`）/ 分配（`A:`）/ 枢纽（`H:`）节点间的全部触发边，派发边第三列为接收者分配条件 |
| `--flow-batch N` / `--hash-seed N` | 顺序无关检验：流传播批量（缺省 64，1 = 逐个排空）/ 内部表哈希初值（缺省 0） |

`--flows` 查询形式（结果打印在 summary 之前，每个查询一段；不带 `@` 前缀即按方法标签片段查）：

| 查询 | 回答 |
|---|---|
| `<方法标签片段>` | 匹配方法的形参 / 返回 / 站点 / 所分配数组元素节点的值集，及流入各节点的来源节点 |
| `elem:<数组分配名片段>` | 数组元素节点的值集及来源 |
| `@path:<节点子串>\|<类>`（`<类>` 可写 `open:<类型>`） | 该值从哪条路径流入节点：沿含该值的流边反向到最近引入点的最短路径 |
| `@srcs:<类>` / `@opens:<类型>` | 含该类 / open(类型)、但无同值前驱的节点（引入点） |
| `@openorig:<类型>\|<节点子串>` / `@openinj:<类型>` | 节点上 open 值的直接注入点 / 全部直接注入点 |
| `@openstat` | 各 open 类型的节点数、引入点数、在实例化集上展开的类数（前 40） |
| `@merge:<N>` | 值集 ≥ N 的节点中由小值集来源汇入最多类者（污染起始汇点，前 40） |
| `@array` | 未知数组写入汇点的值集及来源 |
| `@nullrecv` | 接收者恒为 null 的活虚调用点 |
| `@callers:<方法子串>` / `@m:<序号>` | 无上下文方法本体的全部调用点 / 方法节点序号对应的方法 |
| `@grow:<节点子串>` | **记录型**：匹配节点（含并入同一代表的成员）每次增长——`#序号 节点 ← 来源 +{新增}` |
| `@trace:<类或分配名>` / `@trace:open:<类型>` | **记录型**：每个获得该值的节点及来源，按到达先后——`#序号 节点 ← 来源`（最快定位「值从哪进来」） |
| `@edge:<节点子串>` | **记录型**：以匹配节点为目标新建的流边——`#序号 源 → 目标 [过滤类型] {建边时源集合}` |

记录型查询由 `rava closure` 在分析前登记、传播中逐条记录：来源为流边源节点，或直接注入时的当前站点
（`直接 方法@偏移`）；`#序号` 是全部记录型查询共用的到达次序。每条记录同时实时写 stderr（`[flows <查询>] #序号 …`），
分析未结束（超时被杀）时仍可读到。不带记录型查询时不记录、无额外开销，闭包结果与是否登记无关。
诊断脚本 `scripts/diag/c1d_closure_probe.sh` 把 stdout / stderr 分别落盘。

```bash
rava closure tests/e2e/01_basics/HelloWorld.java --flows '@trace:java/lang/StringBuilder' --flows '@grow:P1 java/io/PrintStream.println'
rava closure tests/e2e/01_basics/HelloWorld.java --cut 'java/lang/String.format:(Ljava/lang/String;[Ljava/lang/Object;)Ljava/lang/String;' -o /tmp/c.json
rava build tests/e2e/01_basics/HelloWorld.java --stop-after emit --cut-file /tmp/cuts.txt --dump-edges /tmp/edges.tsv
```

### `rava audit`（编译前缺口审计）

```bash
rava audit api java/lang java/util [--recursive]      # 公开 API 包为入口 → docs/reports/gap-scan-api-<包>.md
rava audit corpus [--filter S…] [-j N]                # e2e 语料逐例 → docs/reports/gap-scan-corpus-<过滤|all>.md
rava audit native [--filter S…] [-j N]                # 同上，只取 handwritten:native / boundary 缺口 → docs/reports/native-gap-scan.md
```

缺口 = 调用链可达（闭包方法节点）且生成体为方法体存根（`native-missing`：native 方法无手写体；`boundary-stub`：
其余存根）的成员，与 build 的 `[precheck]` 同一归类（第二阶段收尾后、物理拆层前扫描）。发射只在内存进行，不写
scratch；javac 产物落 `build/audit/` 下临时目录，用完即删。corpus / native 逐例起子进程（`rava audit test`，缺省
`-j 2`，单例超时 30 分钟），任一例失败则报告照写、退出码非 0。另可带 `--jdk` / `--java-home` / `--runtime` /
`--closure-cache`（缺省 `build/closure_cache`）。口径是静态可达（过近似；终态按档案口径——构建单元全体入口并集、闭包规模以档案衡量，当前实现仍按单测试逐例计算），运行期是否执行用
`rava closure <Test.java> --why <方法>` 核对。

### 重型闭包的自动处理（无需配置）

rava 的编译阶段（`driver/src/cargo.rs`，build 与 compile 共用）以发射时统计的声明层 `java_runtime` 类数判定，
≥ 1700 视为重型闭包（16G 机器上单 rustc 峰值约 14G，双作业会被 OOM 杀）：

- 生成的 workspace `Cargo.toml` 固定 `[profile.dev] debug = "line-tables-only"`（调试信息减量，rustc 内存明显下降）；
- 编译环境 `CARGO_INCREMENTAL=0`；重型时自动 `CARGO_BUILD_JOBS=1`，打印 `[cargo-env]` 一行；显式设置 `CARGO_BUILD_JOBS` 时尊重显式值；
- 构建超时缺省 600 秒，重型自动放宽到 3000 秒（`--build-timeout` 显式值优先）；判定结果写入 `build_status.json` 的 `heavy` 段。

## 二、生成程序的运行期诊断

生成的二进制读取 Rust 标准变量 `RUST_BACKTRACE`：

| 取值 | 效果 |
|---|---|
| `1` | main 线程因未捕获异常退出时，除 Java 格式异常栈外附打印 Rust 回溯 |
| `full` | 另在每个 VM 抛异常点（NPE / 数组越界 / ClassCastException …）打印 `[vm-throw]` 与 Rust 回溯（输出量大） |

```bash
RUST_BACKTRACE=1 build/analyzer-target/release/rava build Foo.java
```

## 三、会读取 / 设置的标准变量

| 变量 | 读取 / 设置方 | 说明 |
|---|---|---|
| `CC` | `dyn_compare.py` | 编译动态对照 JVMTI agent 的 C 编译器（缺省 `cc`；需要能找到 `$JAVA_HOME/include` 下的 jvmti.h） |
| `JAVA_HOME` | rava（`resolve::jdk`）、各脚本 | JDK 位置。rava 直接调用时按 `resolve::jdk` 优先级参与选择；语料编排（`run_tests.py` / 语料 shell 脚本）不读环境值，选中参考构建（或显式覆盖）后写回，javac / java / jmods 均经它取得 |
| `RAVA_REFJDK_ROOT` | `fetch_reference_jdk.sh`（经它：`run_tests.py`、`corpus_jdk.sh`） | 语料参考 JDK 根目录（缺省主检出的 `tools/refjdk/`）。服务器由分发脚本设为数据目录（如 `/data/rava-jdk`），抽查 / 作业的独立检出共用一份 |
| `JDK` | 语料 shell 脚本（`corpus_jdk.sh`） | 设为主版本 N 时改用本机 JDK N（实验，非参考构建）；未设 = 参考构建 |
| `CARGO_BUILD_JOBS` | 用户 / rava（`driver/src/cargo.rs`） | rustc 并行作业数，显式设置时优先于重型闭包自动判定 |
| `RUST_BACKTRACE` | 生成程序 | 见第二节 |
| `CARGO_TARGET_DIR` | rava 自动设置 | 共享编译缓存（`--target-dir`：`run_tests.py` 传 `build/jdk<N>/target`，缺省 `build/target`），无需手动设置 |
| `RAVA_FRAME_LINES` | rava compile 自动设置，链接器包装 `rava-link` 读取 | 旁路行表路径（`<scratch>/closure_input/frame_lines.json`），供链接期构建地址 → Java 帧表（二进制体积 B2）；内部通道，无需也不应手动设置 |
| `CARGO_INCREMENTAL` | rava 自动设为 `0` | 关闭增量编译：宽闭包下增量元数据是 OOM 的主要诱因 |
| `LANG` / `LC_ALL` | `run_bg.sh` 设为 `C.UTF-8`；`run_tests.py` 对 golden JVM 与被测二进制设 `LC_ALL=en_US.UTF-8` | 保证非 ASCII 输出一致；固定默认 locale（JVM 另加 `-Duser.language=en -Duser.country=US`，macOS 的 JVM 取系统偏好而非 LANG），golden 不随机器变化 |
| `TZ` | `run_tests.py` 对 golden JVM 与被测二进制设为 `UTC` | 固定默认时区（JVM 另加 `-Duser.timezone=UTC`）；运行时按 JDK 语义取 TZ |
| `XDG_CACHE_HOME` | rava（`resolve::image`） | JDK 镜像解包缓存根目录（缺省 `~/.cache`，缓存在 `<根>/rava/`） |
| `HOMEBREW_PREFIX` | rava（`resolve::jdk`） | macOS 自定义 brew 前缀，优先于 `/opt/homebrew`、`/usr/local` 扫描 |
| `PILOT_LIBS` | `lib_pilot_golden.sh` | lib pilot 依赖 jar 目录（缺省 `tests/lib_pilot/deps/target/pilot-libs`，一般不需要设置） |
| `OUT_DIR` | cargo → `build.rs` | cargo 标准变量：`build.rs` 在此生成 `jdk_feature.rs`（语料 JDK 版本常量） |

## 四、常用组合

```bash
# 服务器 / 16G 机器后台跑批（rava 自动单作业 + 自动放宽超时）
scripts/run_bg.sh j21 python3 scripts/run_tests.py --filter TestCipherDesModes

# 取参考 JDK（新机器 / 清单换版本后一次）
scripts/fetch_reference_jdk.sh

# 实验：JDK 25 全量（非参考构建，输出标记）
python3 scripts/run_tests.py --jdk 25 -j 4

# 严格模式回归（兜底即失败）
python3 scripts/run_tests.py --strict --deny fallback

# 排查运行期异常的抛出位置
RUST_BACKTRACE=full build/analyzer-target/release/rava build Foo.java
```

## 五、旧环境变量对照（2026-09-28 删除）

| 旧变量 | 现在的做法 |
|---|---|
| `RAVA_JDK` | `--jdk N`（或 `JAVA_HOME` / `.jdk-version`）；语料编排缺省参考构建 |
| `RAVA_BUILD_TIMEOUT` | `--build-timeout SEC`（rava build / compile、`run_tests.py`）；重型闭包自动 3000 秒 |
| `RAVA_HEAVY_CLASSES` | 阈值固定 1700；需要时显式设 `CARGO_BUILD_JOBS` |
| `RAVA_STRICT` | `--strict` |
| `RAVA_DEBUG` / `RAVA_BFS_EDGE_AUDIT` / `RAVA_CFG_DEBUG` | `--debug` |
| `RAVA_BFS_TRACE` | `--trace-class CLASS` |
| `RAVA_RAW_SITES` | `--raw-sites FILE` |
| `RAVA_THROW_BT` / `RAVA_UNCAUGHT_BT` | `RUST_BACKTRACE=full` / `RUST_BACKTRACE=1` |
| `RAVA_JDK_FEATURE` | `build.rs` 在 `OUT_DIR` 生成 `jdk_feature.rs` 常量 |
| `RAVA_M3_ASSERT` / `RAVA_M3_AUDIT` | 删除：M3 纯位类型断言常开，差分审计已完成 |
| `RAVA_MT` | 2026-09-27 起已无作用（并行后端唯一），删除 |
| `JAVA_RTA_*` | 项目改名为 rava 时统一改名，随后全部删除 |
