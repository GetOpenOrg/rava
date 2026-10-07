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

### J0 基线（2026-10-05，分支 junit-deps）

1. **J0.1 jar 清点**：当前 main（6f93f1c6）的 pom 取包实测 **92 个 jar**（非本文与 V12 基线表所写 52——52 为
   3e4410a3 框架矩阵增补（+40 jar，2026-10-03 合入）前的旧集口径）。`META-INF/versions/` 实测 **class 3130 /
   目录 185 / 其他 3**（V12 表的 3,078 为旧 52 集快照）。**待用户裁定 J1 验收口径**：建议按现集 **92/92**（52 的严格
   超集，不损失范围），本文按裁定值执行。
2. **J0.2 服务器 m1–m5 golden 基线**：作业 `junit-j0-golden`（distribute_tests --job，--ref main @ 6f93f1c6，
   含 fetch_pilot_deps 服务器 mvn 取包先决验证）——结果追加于下。
3. **J0.3 期望复核**：10 例以参考 JDK（Temurin 21.0.11）+ 真 jar（junit 4.13.2 / hamcrest 3.0）javac+java 双跑，
   **10/10 双跑稳定且与 tests/expected/TestJunit*.txt 逐字一致**。
4. **依赖锁定口径裁定（用户 2026-10-05）：按测试文件实际依赖锁包，非 pom 全集**——J1/J2 验收范围 = 锁定集，
   不再按 52 或 92 口径全量核。锁定集推导（import 面 → 提供包 → jar，实测核对）：
   - `63_junit` 10 例 import = `org.junit.{Assert,Assume,runner.JUnitCore,runner.Result,runner.notification.Failure,
     Before,BeforeClass,After,AfterClass,Ignore,Test}` + `org.hamcrest.{MatcherAssert,Matcher,BaseMatcher,
     Description,CoreMatchers.*}`；
   - lib_pilot m1 仅 org.hamcrest.*；m2–m5 增加 org.junit.*；
   - 提供方核对：`hamcrest-3.0.jar`（Automatic-Module-Name: **org.hamcrest**，含 org/hamcrest/，sha256 5d66b6a4…）、
     `junit-4.13.2.jar`（Automatic-Module-Name: **junit**，含 org/junit/，sha256 8e495b63…）；
     `hamcrest-core-3.0.jar` 为 1-class 搬迁壳，不入锁；
   - **锁定集 = [hamcrest-3.0, junit-4.13.2]（类路径序，m2–m5 golden 同序）**；J1 模块名验收 = 2/2 正确 +
     合成夹具 6 例（机制泛化性由夹具保证，不靠 jar 数量）。

5. **J0.2 服务器 m1–m5 golden 基线：0/5，单根因**（作业 junit-j0-golden + junit-j0-m1diag / junit-j0-m2345diag，--ref main @ 6f93f1c6）：
   - 先决全通：服务器 mvn 取包 ✓（92 jar 导出）、参考 JDK ✓（/data/rava-jdk/jdk-21.0.11+10）、JVM 侧 golden ✓（m1 23 行）；
   - **五模式转译段全部同一失败**：`发射：方法体生成失败：com/sun/org/apache/xerces/internal/impl/XMLDTDScannerImpl.scanDTDInternalSubset:(ZZZ)Z：
     CfgAuditError: 结构树与活块集合不一致 tree=[0,1,2,3,4,5,6,7,9,10,12,13] live=[…,14]`——活块 14（RPO 可达）未入结构树；死块 8/11 剔除正确；
   - **归因**：cfg 结构化自 P4a（ea0d4c12）后零改动，守恒不变量（cfg/src/audit.rs:114）为既有机制——回归来自**闭包输入侧**（fe197231 时 m1 GOLDEN OK / 闭包 1235 类；其后 C1d-T3 / from_any / M2 / S7-2b2c / B1 / b3 合并使该 xerces 方法新入 JDK 侧闭包，其 CFG 形态暴露结构化器既有缺口）。该方法为何入 m1 闭包（无 seed 整包 hamcrest → JDK 侧链路）J1 时顺带核对；
   - **处置待裁定**：修复点 `generator/crates/cfg/`（结构化器补活块路径）——不在三个在途子代理改动面（§三冲突表），按 J4「生成器优先」属本任务范围，但引擎相邻，按开工约定报用户裁定：本任务修 vs 转 engine 队列；
   - J0 结论：**J2 验收（m1–m5 5/5）被此单点阻塞**；J1（模块归属）与该缺陷正交、可先行。
6. **cfg 修复与下一层暴露（2026-10-05，264e73ee）**：
   - **修复**：根因在 `simplify.rs pass_while` 守卫转换——`loop{if!c{break}else{空块}}` 规整 `while c{}` 时
     `body.remove` 连 else 臂身份项（空 Code 块，携带块号与跳转账目）整删，守恒不变量失败。终态修法：守卫判定
     收紧为全量形态（then 恰一个 Break、else 只余 Code/Decl）+ 移除守卫时 else 身份项拼接回循环体；
     scanDTDInternalSubset 的 CFG 形态（15 块六空块 do-while 续环）入 cfg 单测回归夹具。
   - **修复验证**：cfg 单测 6/6、generator workspace 12 crate 291 项全绿、本机 emit 复现转绿
     （5423 类）、服务器 m1–m5 **转译段全过**（CfgAuditError 清零）、xerces 生成文件零编译错误。
   - **下一层（新暴露，非 cfg 回归）**：m1–m5 全部推进到编译段后挂 `java_base_decl`（声明层 crate）6 错——
     E0432 `proxy_impl.rs use super::Proxy_Dyn` 断链（**手写层对生成类的引用在闭包裁剪后未随裁**——hamcrest
     闭包无动态代理使用，Proxy_Dyn 不在生成集）+ 5×E0308（`compound_element.rs` 泛型 `<E>` vs `<Object>`、
     `stack_stream_factory_{abstract_stack_walker,caller_class_finder,live_stack_info_traverser}` 族参数类型）。
     系 M2 模块切分 + B1 档案裁剪后 `--lib` 多 crate 形态**首次编译**暴露的装配/泛型发射缺陷；
     普通 e2e（spot AbstractShape ✓）不撞（闭包组合不同）。**归属待裁定**：E0432 正对 decl-scc 声明层
     分段域、E0308 族在 emit 泛型发射（B2 相邻）——报协调会话。
   - spot `cfgfix-264e73ee`（按目录抽样）：AbstractShape ✓ / ABCProblem ✗（regex
     `Pattern$CharPredicate.union` 存根命中——main 既有失败族，与本修复无关）；`--tests` 指定例
     未被 spot 模式采纳，三例指定抽查另发作业 `cfgfix-3spot-264e73ee`（结果回填）。
7. **A 族修复合入后（2026-10-05 晚）**：
   - A 族（E0432 Proxy_Dyn）修复 `d7f482e1`：伴生依赖判定大写段三类判定（手写层 pub item 全集 /
     snake 名探测 scratch / 目录+同名 .rs 保守在场）。本机 m1 golden：5008 错→0。
   - 服务器复验 `cfgfix2-golden-d7f482e1`（10314222 + A 修复 + JDK 21）：**m1/m2 GOLDEN OK**；
     m3–m5 运行期同 panic：`Unsafe 静态字段读-改-写：Result$SerializedForm.serialVersionUID
     无字段闭包（静态字段表项 []）`。
   - B/C 族 5×E0308 定性为**环境假象**：本机 shell `JAVA_HOME=graalvm-25` 压过语料 pin
     （`resolve/src/jdk.rs` 优先级缺陷），JDK 25 语料 vs JDK 21 手写层的字段 J→I 错配；JDK 21
     重发后自消。已修 `a4acb1a5`：pin 高于 JAVA_HOME（主版本不符忽略并打印、一致用其 home、
     无 pin 照常），单测三分支覆盖。
   - **m3–m5 blocker 定性（2026-10-05 晚，主会话裁定）：S7-3c 已修缺陷未同步**——`statics_table`
     的反射标记判断原来用 to_string 子串匹配恒不命中（b259fdb1 修复，已在集成分支 00fe97ce）。
     junit-deps 已合并 origin/rust-closure-analyzer@85a56289（含修复），m3–m5 golden 复测中
     （作业 junit-m345-aftermerge）。
   - d7f482e1 的已知回归：`emit::project::tests::companion_skipped_when_used_module_absent`
     失败（缺失小写段判在场 vs 注释不一致 + 夹具 Class 判缺席）——主会话已另派代理修，本任务不动。

8. **J1 = V12-0 类路径与模块归属修正（2026-10-05 深夜，本条目所在提交）**：
   - `classfile/src/archive.rs`：`Archive::open(path, release)` 多版本 jar 视图——清单
     `Multi-Release: true` 时 `META-INF/versions/N/`（9 ≤ N ≤ release，取最大 N）覆盖基础条目，
     版本化 `module-info.class` 作根描述符；版本化路径不以任何形态进入类索引；无清单标记则对任何
     release 完全不可见。`manifest_attr` 上移 classfile 供 resolve 复用。
   - `classfile/src/module.rs`：`ModuleDecl` 增 `exports` / `opens` 明细（(包, 目标模块；空 = 无限定)）。
   - `resolve/src/classpath.rs`：`ClassPath::new(release)`；`shadow_jdk_owned_packages()`——用户 / 库
     档案中属于 JDK 具名模块所拥有包的类不入索引（记 `shadowed`，JDK 侧同名类接管可见性）；
     跨档案重复类记 `duplicates`（JDK 档案与镜像覆盖除外）；依赖锁元数据口 `set_lib_meta`
     （`LibMeta { coordinate, module }`，J2 由 deps.lock 喂入）。
   - `resolve/src/modules.rs`：`ModuleNode` 增 `kind(Jdk|Lib|User)` / `jars` / `coordinate` /
     `exports` / `opens`；命名兜底链：描述符 → `Automatic-Module-Name`（限定名校验）→ JPMS 文件名
     推导 → 坐标 artifactId → 锁条目显式 module → 报错；具名跨侧重名、具名模块分裂包、无法命名
     为硬错误（`modules::check`，在 `build_cmd::class_path` / `closure_cmd` / `profile_cmd::jdk_path`
     三处装配点调用）；自动模块重名合并且告警；分裂包登记 `split_packages`（交 crate 计划分量
     合并）；crate 名 = 模块名 `.`→`_`（Rust 关键字加 `_`），冲突时名序靠后者加 FNV 4 位十六进制后缀。
   - `driver` / `input`：release 贯穿 `build_libs::load` / `LibCrate::from_jar`（多版本视图进 lib
     crate 类枚举）；`lib_pilot_golden.sh` 增 `--emit-only` 与 `SCRATCH_ROOT`（树对照用）。
   - 验收：合成夹具 8 例（任务书 6 例 + crate 名冲突后缀 + 命名兜底链）全过；pilot 模块名对账
     `resolve/tests/pilot_module_names.tsv`（`scripts/pilot_module_expected.sh` 以参考 JDK
     `ModuleFinder` 生成，pom 变更后重跑）**92 / 92 相符**——含此前 18 个错名 jar（gson→
     com.google.gson、jackson-databind→com.fasterxml.jackson.databind、failureaccess→
     com.google.common.util.concurrent.internal 等）与 30 个丢描述符 jar；`META-INF/versions/`
     形态类名入索引 **0**（byte-buddy 3,071 + jackson-core 7 等）。锁定集 2/2：hamcrest→
     org.hamcrest、junit→junit（均在 92 对账内）。m1–m5 与 J0（63005c6b）逐字节对照作业
     `j1-trees`（结果回填）。
   - 注：任务书引用的 `no_jdk_literals` 守护在仓库无实现（grep 无命中）；以 92/92 对账与
     本提交不新增 JDK / 库类名字面量（人工 diff 复查）为准。
   - **J1 树对照回填（2026-10-06 凌晨）**：作业 `j1-trees2`（jp1）——全部 .rs 正文与资源
     逐字节相同、raw-audit 一致；**五个 scratch 各恰 3 行 closure.json 差异**（疑似 J1 包归属
     语义预期的库模块标注，非正文回归；mv 撞上前次尝试残留目录致链路部分短路，计数仍有效）。
     **终局定性（2026-10-06 晨，本机受控对照）**：服务器两轮取证均遭 infra 死亡，改本机
     heavy_lock 取证——63005c6b vs f294a0a4、**同一 `--lib` 参数**、emit-only 各一轮：
     `compare_trees` **= 0 差异**（含计时剔除后的 closure.json）、裸 diff 正文 0 差异、
     raw-audit 一致——**J1 验收「m1 生成正文与 J0 逐字节相同」成立**。j1-trees2 的五目录
     各 3 行差异由此定性：其两侧实为「J2a 新入口（crate 名 org_hamcrest 等）vs J0 老入口
     （crate 名 hamcrest）」——J2 预期的入口面差异，非 J1 回归；m2–m5 与 m1 同码路径，
     3 行齐一性与之相符。

12. **J2a 验收与 J2b（2026-10-06 凌晨，2174812a）**：
   - `j2a-golden2`（kr1，a67c33c2）：**新入口 `--deps/--cp/--seed-class` 下 m1–m5 5/5
     GOLDEN OK**（J2A-GOLDEN-5OF5）——J2 核心验收落地。

13. **§3.6 改判执行：rava 仓库第三方库专属文件归零（2026-10-06 晨，主会话转达用户裁定）**：
    - junit-deps 快进同步到 **f9298933**（J1+J2 由主会话合入集成分支；冲突 --release-small /
      --boot-report 已由主会话解好；companion 修复 399f064d 在链——此前「已知回归」消除）。
    - 第三方库不设专属补种 / 手写 / 清单：**`runtime/lib_runtime/` 整目录删除**（含 junit
      骨架 manifest.toml / seeds.toml）、**`closure::lib_runtime` 读取器删除**。今后用户项目
      的外部事实声明（JNI 回调目标、配置值驱动的反射目标）作为构建单元选项从用户项目路径
      读取，现阶段不实现；JNI ABI 与构建期捕获登记 B5 缓行。方案 §3.6 由主会话改写
      （87447716 / 24532f6c，随 gate 通过推 main）。
    - **J4 改写为分析器任务**「注解驱动反射入口的通用建模」：流 = 类常量 / Class 值 →
      getMethods / getDeclaredMethods / getDeclaredFields → getAnnotation /
      isAnnotationPresent / 修饰符过滤 → Method.invoke / Constructor.newInstance /
      Field.get / Field.set。验收：runtime/ 下 junit 专属文件 0；生成器与分析器无 junit /
      hamcrest 字面量；63_junit 无任何补种全过 + 动态对照漏覆盖 0；补一个用户自定义注解 +
      反射发现 + invoke 的 e2e 用例（expected 取真 JDK 21 输出）。C4 冻结期属语义改动：
      全量开跑前合入 / 全量后合入 / 或附 compare_trees 27 例 0 差异证据。
    - **J3 分阶段**：先做不碰 `scripts/run_tests.py` 的部分（form.toml 解析 + Form 独立模块 +
      --pilot-libs 预置），C4 预检（超时参数）合入并广播后再接线挂接点；63_junit 按用户项目
      对待，form.toml 放 `tests/e2e/63_junit/` 用例目录。
    - C4 全量期间服务器作业一次一个；本提交为结构性删除的单独提交，推送后报主会话优先合入。
   - 锁条目名坐标真源修正（a67c33c2）：mvn dependency:list 解析坐标（92/92 有坐标；
     junit→junit、hamcrest-3.0→hamcrest，sha256 与 J0 一致）。
   - J2b（2174812a）：profile.json §4.2（modules 富化在档案层、closure.json 模块行最小面保
     缓存兼容、富化字段入摘要剔除面保 --covers 一致、deps_lock 摘要入 P）；EntryClosure 增
     cp/launch；`closure::lib_runtime` 区间读取器 + `runtime/lib_runtime/junit/` 骨架
     （versions="[4,)"，seeds 由 J4 填）。gate：34 ok + 唯一已知回归（第 7 条）。
   - 单测 gate：generator 全量 + 宏 crate——closure（含重型真实 JDK 闭包测试）/ driver / emit
     69+1 / input / ty / classfile / cfg 全绿；**唯一失败 = `companion_skipped_when_used_module_absent`
     （d7f482e1 已知回归，主会话另派代理修，本任务不动，见第 7 条）**。宏 crate本轮未触碰
     （J1 零文件），沿用当日早间 gate 绿。

9. **J0 闭环：m1–m5 golden 5/5（2026-10-05 深夜）**：作业 `junit-m345-aftermerge`
   （sg1，junit-deps@63005c6b = ba3955bd 合并 S7-3c 修复后）——**m3 / m4 / m5 全部
   GOLDEN OK**；连同 `cfgfix2-golden-d7f482e1` 的 m1 / m2，**五个模式 5/5 逐字对账通过**。
   第 7 条的定性（S7-3c 未同步）完全成立：合并即愈，`--lib` 老入口下 J0 基线闭环。
   J2 切 `--deps` 新入口后须再次 5/5（J2 验收项）。

10. **J2a = V12-1 依赖锁与入口类路径（2026-10-06 凌晨，本条目所在提交；J2b=profile.json 增量与
    runtime/lib_runtime 另行提交）**：
   - `driver/src/deps_lock.rs`（新）：`deps.lock.toml` 读取与校验（条目名 = 坐标 artifactId /
     文件名 stem，须唯一；sha256 必填、jar 在位）；`select` 子集**取锁序**（入口给出序不影响
     类路径 → `P` 不随入口序变化）。
   - `scripts/fetch_pilot_deps.sh`：取包后生成 `tests/lib_pilot/deps/target/deps.lock.toml`
     （92 条：release=参考 JDK 主版本、坐标取 jar 内 pom.properties（75 条有）、sha256）。
   - 命令面（`--lib` 删除，出现即报指点错误）：`rava build`/`rava closure`/`rava profile`/
     `rava audit` 墇改 `--deps <锁> --cp <条目名>[,…] [--launch <启动选项>] [--seed-class FQN[,…]]`；
     种子不再挂库声明（`:seed=` 退出），整包翻译以 `--seed-class` 显式给出。
   - lib crate 装配：`build_libs::from_lock`——crate 名 = jar 模块的 crate 名（J1 模块图：
     hamcrest → `org_hamcrest`、junit → `junit`），序 = 锁序；`LibCrate` 去 wholesale 字段
     （发射集 = 闭包触达；整包经显式种子覆盖达成）。
   - `lib_pilot_golden.sh` 切 `--deps/--cp/--seed-class`（m1 = hamcrest 全类种子；m2–m5 在此
     之上加 junit 种子，序与原 `:seed=` 一致）。
   - 验证：driver 29+4 用例绿（含改写后的 `lib_crate_and_precheck_only` 端到端锁流用例：
     jar → 锁 → `--cp greet` → crate 名 greet、跨 crate 可见性、user 依赖行不变）；
     generator 全量 gate：全绿，唯一失败仍为已知回归 `companion_skipped_when_used_module_absent`
     （第 7 条，他代理在修）。服务器作业：`j2a-golden`（新入口 m1–m5 5/5，结果回填）、
     `j1-trees2`（J1 vs J0 逐字节对照——首轮 `j1-trees` 因 J0 旧脚本不识 `--emit-only` 走了
     完整 golden（意外复验 5/5），树落默认 build/ 致对照空跑；本轮修正取 `build/*_main`
     并剥编译产物）。

11. **J2a 后续与 J2b = profile.json 增量与 lib_runtime（2026-10-06 凌晨，本条目所在提交）**：
    - **锁条目名修正**（j2a-golden 首轮暴露）：junit / hamcrest 等 5 个 jar 不带 pom.properties，
      条目名回退文件名 stem 致 `--cp hamcrest,junit` 报「不在依赖锁中」。坐标真源改
      `mvn dependency:list -DoutputAbsoluteArtifactFilename`（剥行尾 module 注记、version 取
      倒数第三段兼容 classifier），jar 内 pom.properties 只作回退——92/92 条全有坐标。
    - **profile.json §4.2 增量**：`PROFILE_FORMAT 1→2`；modules[] 行在档案层富化
      kind/crate/jars[{path=文件名, sha256, coordinate}]/release（`profile::build` 合并后统一做，
      逐入口 closure.json 模块行保持最小面——缓存兼容；富化字段入内容摘要剔除面，
      `--covers` 重并两侧一致；jar 绝对路径不进 P，身份经 `profile.inputs.deps_lock`
      （锁摘要，含 sha256 与序）**入 P**）；entries[] 增 classpath（锁条目名）与 launch；
      EntryClosure 增 cp/launch。
    - **`--launch` 入库**：rava profile 顶层选项，记 profile.entries[].launch（本步只入库传递，
      启动语义随 V12 模块路径模式验收）。
    - **runtime/lib_runtime/**（V12 §3.6）：`closure::lib_runtime` 读取器——模块目录
      manifest.toml `versions = "[a,b)"`（下含上不含、开界省略；单串或数组，数组展开多段、
      命中多段报错），版本比较数值段感知（4.13.2 > 4.9）；`select(module, version)` 命中
      唯一性校验。骨架 `runtime/lib_runtime/junit/`（versions="[4,)"，seeds.toml 由 J4 填
      @Test 发现路径补种）。清单摘要进该库 K(c) 属 V12-3，本步读取器就位。
    - 验证：closure 160+2 用例绿（含区间/多命中/坏格式夹具）；driver 全套绿（profile_cli 的
      `--covers` 断言经内容摘要剔除面修正后通过）。全量 gate 见提交说明（唯一失败仍为
      第 7 条已知回归）。


