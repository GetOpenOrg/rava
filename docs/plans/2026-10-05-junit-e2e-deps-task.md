# 任务书：63_junit 依赖包引入与 10 例跑通（V12-0 → V12-1 → 形态接线）

> 日期：2026-10-05
> 来源：用户「想做 63_junit 相关的任务，把依赖包引入进来，把测试跑通」；路线裁定 = 走 V12 终态（2026-10-05 用户选定）。
> 上游方案：[`2026-10-04-third-party-dependency-layering.md`](2026-10-04-third-party-dependency-layering.md)（V12，§三 终态、§五 V12-0 / V12-1）；
> [`2026-10-01-junit-crate-as-test-harness.md`](2026-10-01-junit-crate-as-test-harness.md)（步骤 0 / A / B）；
> [`2026-10-01-scripts-into-rava.md`](2026-10-01-scripts-into-rava.md) §六（形态接口）。
> 本文只写终态，不写过渡方案。

---

## 一、目标（终态，量化）

| 项 | 目标 | 今天 |
|---|---:|---:|
| `tests/e2e/63_junit` 10 例在 run_tests 下逐字对账通过 | **10 / 10** | 0（harness 无依赖包概念，javac 即失败） |
| `lib_pilot_golden.sh m1..m5` 通过（经 `--deps` 新入口） | **5 / 5** | Rust 路径在 M2 / S7-2 / B1 之后未复跑 |
| pilot jar 模块名 / 描述符正确 | **52 / 52** | 名字对 34、描述符对 22 |
| `META-INF/versions/` 条目以错误类名入索引 | **0** | 3,078 |
| `rava build` / `rava profile` 中的 `--lib` 与 `:seed=` | **0**（删除） | 存在 |
| 生成器 / Python 编排中的库类名、JDK 类名字面量 | **0** | 0（守护不变） |
| 既有 e2e 抽样（≥ 15 例，见 §六）回归 | **0** | —— |

## 二、V12 决策裁定（本任务采用）

| V12 §七 | 裁定 | 对本任务的含义 |
|---|---|---|
| 3 启动模式 | **A**：两种都支持，缺省类路径模式 | 63_junit 用类路径模式；模块路径模式只需在 `--launch` 中留出、进入 `K`，本任务不验收 |
| 4 同模块多版本 | **A**：单版本锁 | `deps.lock.toml` 内同一模块重复即报错 |
| 5 库专属手写与清单 | **A**：`runtime/lib_runtime/<模块名>/` | JUnit 需要的补种写在 `runtime/lib_runtime/junit/seeds.toml`，不进 `runtime/java_runtime/` 三份清单 |
| 1、2、6、7 | 不在本任务范围 | 留给 V12-2 及以后，由协调会话另行请示 |

## 三、与在途任务的关系（冲突面与约定）

| 在途 | 它改的区域 | 与本任务的交叠 | 约定 |
|---|---|---|---|
| engine-order 第三阶段 | `generator/crates/closure/src/engine/`（按名取类、延迟查找、`class_lookup.rs`） | V12-0 改 `resolve/classpath.rs` 的索引（JDK 包遮蔽），闭包按名取类消费该索引；文本不冲突 | 本任务**不改** `closure/src/engine/`；JUnit 跑通如需改闭包引擎，先报协调会话排期 |
| C1d-a a2 续 | 闭包早退求值、`[boot_init]`、`runtime/java_runtime/*.toml` | 只可能在 `seeds.toml` 文本上相邻 | JUnit 补种走 `runtime/lib_runtime/junit/`，不碰 java_runtime 清单 |
| B2 栈还原 + strip | emit 行表 / 运行时栈帧、`driver/src/cargo.rs`、`emit/src/project/entry.rs` | V12-1 改 driver 命令面与 `emit/src/project/lib_crates.rs`，可能与 B2 在 `emit/src/project/` 相邻 | 开工与每步提交前先合入最新 `rust-closure-analyzer`；冲突由后合者解 |
| T1-M3（未派） | 档案键、`profile.json`、链接集 | V12-1 给 `profile.json` 加 `modules` / `entries` 字段 | M3 在 V12-1 合入后再派 |
| scripts-into-rava S6 / S7（未开工） | dyn 对照并入 rava；run_tests 拆 `scripts/e2e/` | 本任务把 §六 形态接口直接落在 `run_tests.py` | 接口按 §六 形状写成独立函数（`form_of` / `Form.deps_args` / `Form.classpath`），S7 拆分时整体搬走，不重写 |
| C1d-b（暂停） | 反射收窄 | m3 遗留的运行期存根原计划移交 C1d-b | JUnit 跑通需要的反射 / 注解存根在本任务内解决，修法落在生成器或反射建模，合入时在 tasks.md C1d-b 行注明已被吸收的部分 |

结论：与正在运行的三个子代理**没有代码层面的直接冲突**。需要遵守的只有两条：不碰闭包引擎目录；改动 `emit/src/project/` 前先同步集成分支。

## 四、分步实施（每步独立提交、独立验收）

### J0 基线（不改代码）

1. `scripts/fetch_pilot_deps.sh --no-scan` 确认 `tests/lib_pilot/deps/target/pilot-libs/` 齐全（本机现有 52 个 jar）。
2. 在服务器以作业跑 `scripts/lib_pilot_golden.sh m1` … `m5`（当前 `--lib` 入口），得到 M2 / S7-2 / B1 之后 Rust 路径的回归清单，逐项归因写入本文 §七。
   - 服务器需要 mvn 取包：先确认服务器项目目录下能取到 jar；只在数据目录内操作，服务器其他目录与系统配置的改动先请示用户。
3. 对 `tests/e2e/63_junit` 10 例，用 JVM 真 jar 核对 `tests/expected/TestJunit*.txt` 与 JVM 输出一致（期望文件已入库，这里只复核）。

验收：回归清单与 10 例期望复核结果入本文。

### J1 = V12-0 类路径与模块归属修正

按 V12 §4.1 前四行：
- `classfile/src/archive.rs`：`Archive::open(path, release)` 多版本 jar 视图；`META-INF/versions/N/`（N ≤ release）覆盖基础条目；版本化的 `module-info.class` 作为根描述符；`META-INF/versions/` 不进类索引。
- `resolve/src/classpath.rs`：`Origin::User` / `Origin::Lib` 中属于 JDK 镜像具名模块包的类不入索引，记入 `shadowed`。
- `resolve/src/modules.rs`：库模块命名兜底（module-info → `Automatic-Module-Name` → JPMS 文件名推导 → 坐标）、crate 名冲突后缀、重名模块合并、具名模块重名与分裂包报错；`ModuleNode` 增加 `kind`、`jars`、`coordinate`、`exports` / `opens`。

验收（同 V12-0）：52 / 52 模块名与 `jar --describe-module --release 21` / `Automatic-Module-Name` / JPMS 推导逐一相符；版本化条目入索引 0；合成夹具 6 例（分裂包、重名具名模块、重名自动模块、JDK 包遮蔽、重复类、多版本覆盖）写成生成器单测并通过；`no_jdk_literals` 通过；m1–m5 生成正文与 J0 相同（`scripts/compare_trees.sh` 逐字节）。

### J2 = V12-1 依赖锁与入口类路径（删除 `--lib`）

- `scripts/fetch_pilot_deps.sh` 额外输出 `tests/lib_pilot/deps/target/deps.lock.toml`（V12 §3.2 格式：`release`、按类路径顺序的 `[[jar]]`，含 `coordinate`、`path`、`sha256`）。锁文件是生成物，不入库。
- `rava build` / `rava profile`：新增 `--deps <deps.lock.toml>`、`--cp <锁条目名>[,…]`、`--launch "<启动选项>"`；**删除** `--lib NAME=JAR[:seed=]` 与 `CrateRoute` 中依赖命令行声明序的部分。lib crate 名取 J1 算出的模块 crate 名（`org_hamcrest`、`junit`）。
- `profile.json` 增量按 V12 §4.2：`modules[]` 加 `kind` / `crate` / `jars` / `release`；`entries[]` 加 `classpath` 与 `launch`；`profile.inputs` 加 `deps_lock` 摘要。`slice_digest` / `key` 属 V12-3，本步不加。
- 新建 `runtime/lib_runtime/`：按模块名与 `versions = "[a,b)"` 区间匹配清单，命中多个区间时报错；清单摘要进入该库的生成输入。读取逻辑与 `runtime/java_runtime/` 三份清单共用解析代码，生成器中不写库名。
- 种子：`:seed=` 退出构建单元语义。JUnit 反射发现 `@Test` / `@Before` 等由反射建模与 `runtime/lib_runtime/junit/seeds.toml` 补种覆盖；`lib_pilot_golden.sh` 需要整包翻译的 crate 验收以 `--seed-class` 入口显式给出。
- `scripts/lib_pilot_golden.sh` 改用 `--deps` / `--cp`。

验收：L-a 构建单元（m1–m5 五个 main，锁 = junit 4.13.2 + hamcrest 3.0）——入口给出顺序变化时 `P` 不变；`--covers` 对五个 main 都成立；档案类集合 = 五个单例非用户侧之并；服务器作业 `lib_pilot_golden.sh m1..m5` **5 / 5**；`grep -rn -- '--lib' generator scripts` 只剩历史文档引用。

### J3 形态接线（run_tests 消费 63_junit）

按 scripts-into-rava §六 的形状落在 `run_tests.py`，写成可整体搬到 `scripts/e2e/select.py` 的独立函数：
- `tests/e2e/63_junit/form.toml`（数据，Python 中不写库类名）：
  ```toml
  deps = "tests/lib_pilot/deps/target/deps.lock.toml"
  cp   = ["hamcrest", "junit"]        # 锁条目名，按类路径顺序
  ```
- `form_of(java_file) -> Form | None`（读所在目录的 form.toml，缓存）；`Form.deps_args()` → `["--deps", <abs>, "--cp", "hamcrest,junit"]`；`Form.classpath()` → jar 绝对路径列表。
- 接线点①：转译调用 `rava build` 时追加 `Form.deps_args()`。
- 接线点②：期望输出生成与复核的 javac / java 加 `-cp <classes>:<jars>`。
- 接线点③：动态对照的 JVM 类路径**从 scratch 的 `profile.json` `entries[].classpath` 读取**，不从命令行另传。这样 S6 把 dyn 判定并入 rava 时沿用同一数据源，Python 侧这处改动不被丢弃。lib 类在 dyn 归类中记 lib 域。
- `--pilot-libs DIR` 选项（缺省 `tests/lib_pilot/deps/target/pilot-libs`，不新增环境变量）。批次开头对选中的形态统一取包一次；取包后仍缺 jar 时该形态用例全部记失败（`deps-fetch-fail`，进失败棘轮），批次退出码非 0，**不静默跳过**。
- 台账、并行、内存纪律、产物清理全部继承；lib crate 产物经 `build_artifacts.json` 纳入 `rava prune` 清理。

验收：`python3 scripts/run_tests.py --no-run --filter HelloWorld` 与改前输出一致（非形态目录零行为变化）；`python3 -m compileall -q scripts`；服务器抽查 63_junit 10 例全部完成转译与编译（运行结果在 J4 收敛）。

### J4 10 例跑通

- 逐例归因失败，修法优先级：生成器 / 宏 → 反射与注解建模 → `runtime/lib_runtime/junit/` 补种 → 手写（只限 V12 §3.6 准入：库的 `ACC_NATIVE`）。
- 重点检查：B1 按档案裁剪元数据后，用户测试类的方法级注解（`@Test`、`@Before`、`@BeforeClass`、`@Ignore`、`@Test(expected=, timeout=)`）在 `java_meta` 中保留；JUnit 的 `TestClass` 扫描、`FrameworkMethod.invokeExplosively` 的反射调用链可达；`FailOnTimeout` 的线程路径（m4 已验证，真实超时不可达已入 compatibility.md）。
- 不进种子、不在本任务支持的（compatibility.md 挂账）：Parameterized / Enclosed Runner、`@Rule` 规则族。
- **合法 Java 测试不改、不放宽超时**；期望文件与 JVM 输出不一致时修期望（J0 已复核），不改测试源。

验收：服务器抽查 63_junit **10 / 10**；`lib_pilot_golden.sh m1..m5` **5 / 5**；既有 e2e 抽样（§六）零回归。

## 五、执行约束

1. 工作区：worktree `/Users/yuwei/dev/workspace/java_rta_junit`，分支 `junit-deps`，从 main 起；只在该分支提交，不直接改 main。
2. 本机只编译与单测：`cargo build --release -p driver …`、`(cd generator && cargo test --release)`、`(cd runtime/rava_macros_core && cargo test --release)`，重命令经 `CARGO_BUILD_JOBS=2 python3 /Users/yuwei/dev/workspace/heavy_lock.py <cmd>`。e2e、golden、抽查全部走服务器（`server_maintenance/rava/distribute_tests.py` 的 `--spot` / `--job`）。
3. 每步提交前三项闸门通过；`closure_independent_of_hash_seed` 若单独失败属上游已知不稳定（engine-order 在修），注明即可。
4. 提交信息中文，不加 Claude-Session 等署名行；推 origin 与 github，报完整 40 位哈希。
5. 生成器 crate 与 Python 编排中不写 JDK 类名、库类名字面量；不改 `build/` 下生成文件；不用 git stash / reset --hard / rebase；结束进程只按 PID。
6. 每步开工前先合入最新 `rust-closure-analyzer`；J1、J2 是结构性改动，各自单独先合。
7. 频繁改动的源文件保持 ≤ ~600 行，超出按职责拆子模块。

## 六、抽查口径

- J1 / J2：`HelloWorld DeepCopy StockTrans LambdaBasic ReflectionBasic TestModuleLayerDefine TestServiceLoaderLayers TestSerialDefaultSuid TestJndiNoProvider TestHttpLoopbackSync TestUrlProtocolOpen TestCallerSensitiveHandle` + 作业 `lib_pilot_golden.sh m1..m5`。
- J3 / J4：上列 + `tests/e2e/63_junit` 全部 10 例。
- 已知失败不阻塞：TestHttpLoopbackSync 转译超时（main 同）、`__RAVA_OOM_KILL__` 标记的 OOM。

## 七、实施记录

（J0 回归清单、各步提交哈希、抽查结果按步追加）
