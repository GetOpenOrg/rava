# 交接：lib-tests-merge 分支测试（html-to-markdown Java 测试并入 rava）

> 日期：2026-10-12
> 分支：`lib-tests-merge` @ `7608bbf0`（基线 main `e44f78d6`，已推 origin / github，**未合并 main**）
> 计划：上游仓库 `html-to-markdown/docs/plans/2026-10-12-html-to-markdown-java-tests-merge.md`
> 本机（开发机）只做了轻量验证：脚本 `bash -n`、pom XML 合法性、结构计数、17 个纯 JDK e2e 的
> javac + JVM 期望生成（refjdk 21.0.11 + GOLDEN_JVM_FLAGS + 固定 locale/TZ）。**未跑任何转译、
> 未下载 jar**——全部 rava 侧验证在测试机做。

## 一、分支内容（6 个提交）

| 提交 | 内容 |
|---|---|
| `80743ffe` | `scripts/lib_pilot_golden.sh` 新增两种模式：`<MainName>` 单文件直达（cp 表在脚本内 `pilot_cp_table`）、`<库>/<入口>` 多文件项目（projects/<库>/src 全量编译 + `--main` FQCN + cp 取 form.toml）；既有 m1–m5 / sc / sb / jc 不变 |
| `9fee66a7` | 47 个单文件 pilot → `tests/lib_pilot/`（剥 package；AsmTransformMain 改内联生成目标类等三处自足化） |
| `7c50d688` | 29 个多文件项目 / 75 源文件 → `tests/lib_pilot/projects/<库>/`（原样保留 package 与辅助类）+ form.toml + README（闸门表） |
| `4ab80067` | `tests/lib_pilot/deps/pom.xml` 补钉 30 个库（版本取上游 java-examples 钉值） |
| `29f07363` | e2e 17 例 + 17 期望（34_concurrency +8、39_bits +7、15_switch +2） |
| `7608bbf0` | graalvm 基线首轮原始件 3 个入库（脚本 / 报告 / results 数据） |

## 二、验证步骤（建议序）

```bash
git fetch origin && git checkout lib-tests-merge
scripts/fetch_reference_jdk.sh                # 参考 JDK 21.0.11（如未就位）
scripts/fetch_pilot_deps.sh --no-scan         # 取包生成依赖锁（新增 30 库会下载，首跑较久）

# 1) harness 回归（向后兼容验收——既有 golden 输出必须与分支前逐字一致）
scripts/lib_pilot_golden.sh m1
scripts/lib_pilot_golden.sh jc1

# 2) 单文件 pilot——先 A 批零门槛（JVM 侧 + rs 侧都应能出）
scripts/lib_pilot_golden.sh Slf4jBasicMain
scripts/lib_pilot_golden.sh Lang3StringUtilsMain
scripts/lib_pilot_golden.sh JodaBasicMain
scripts/lib_pilot_golden.sh IoFilenameMain
scripts/lib_pilot_golden.sh OgnlEvalMain
# ……其余 42 个同入口（B/C/E 批清单见 commit 9fee66a7 或计划文档 §三）

# 3) 多文件项目——端到端首例
scripts/lib_pilot_golden.sh guava/GuavaCollectionsDemo
# ……29 项目全部入口见 tests/lib_pilot/projects/README.md 表

# 4) e2e 17 例（期望已入库，本地 JVM 已验证零 FAIL；跑 rava 侧对账）
scripts/run_tests.py BarrierBasic PhaserDemo VarintDemo WalRecordDemo IfElseRiskDemo PatternSwitchDemo …
```

## 三、预期与判定口径

- **JVM 侧 golden**：47 pilot + 29 项目全入口**预期全绿**（上游示例均可编译运行）。
  例外见 §四 前置。
- **rs 侧**：按缺口分析 A–F 闸门分级——A/B 批（slf4j、joda、lang3、io、ognl、jsoup、caffeine、
  snakeyaml、hocon、assertj、fastjson2、logback）**预期可点亮**（GOLDEN OK）；C 批 servlet/jpa/spring
  工具族大概率可点亮；E 层 databind / asm / log4j2 / kotlin-stdlib 与 C-SYN / C-MT / C-NET / 引擎 /
  完整应用 / 多语言各项目**预期失败**——这是正常状态，逐项记台账（哪个入口、失败在转译 / 编译 /
  运行哪一段、首个错误），不要为点亮它们改生成器语义（那是各闸门立项的事）。
- **e2e 17 例**：期望输出由 refjdk 21.0.11 生成，rava 侧按 run_tests 常规对账。

## 四、已知前置与收敛点（README 亦有记）

1. **cp 收敛**：个别项目 JVM 侧首跑报 `NoClassDefFoundError` 时，把缺的锁条目名（artifactId）
   补进该项目 form.toml 的 `cp` 即可——传递 jar 均已被 Maven 取进 pilot-libs；单文件 pilot 的
   cp 表在脚本 `pilot_cp_table`，同理改脚本。
2. **redis** 4 入口需本机 `localhost:6379` 有服务，无服务记台账后置。
3. **aspectj**：注解面 javac 直接过，切面生效需织入（上游用 aspectj-maven-plugin）；首跑取
   未织入的确定性行为即可，织入口径后置。
4. **polyglot**：`Context.eval("js")` 需 `org.graalvm.polyglot:js` 实现——deps/pom.xml 未钉，
   首跑若要跑它先补钉再 fetch。
5. **lombok**：依赖 javac 注解处理器自动发现（classpath 上 lombok jar）——golden 脚本的 javac
   缺省应能触发；若不触发，加 `-proc:full` 类参数再议。

## 五、回滚面

分支未合并 main、主检出干净（无未跟踪残留）。测试不通过直接弃分支即可，main 零影响；
graalvm 原始件 3 个只在本分支，弃分支前注意它们会一起丢（必要时单独摘出）。

## 六、未做项

- rs 侧全部 golden（本交接的主体工作）；
- e2e form.toml 目录放量（75_–87_ 预留，等 W0-3 接线，不属本批）；
- 上游 html-to-markdown 仓库的 graalvm-bench/tests 23 个重复语料副本仍在（与 rava e2e 逐字节
  相同，未删，待用户定）。
