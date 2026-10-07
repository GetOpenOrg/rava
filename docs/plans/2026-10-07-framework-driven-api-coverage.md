# 框架需求驱动的 JDK API 覆盖：从 Spring Boot 起步

> 日期：2026-10-07
> 来源：用户 10-07 提出「很多 Java 测试对应的 API 以后不一定需要，应改为分析 Spring 等流行框架实际用到的 Java API，针对这些 API 做测试」。随后拍板：「按你的建议处理，先让最基本的 Spring Boot 相关的依赖包调用的 Java API 跑通，然后再扩展到其他流行的依赖包所对应的 Java API 跑通；如果还没到对应第三方依赖包需要支持覆盖的时候，先可以暂缓这些 API 的支持或者测试」。
> 关系：本文是 [framework-pilot-test-matrix](2026-10-03-framework-pilot-test-matrix.md)（框架 pilot 阶梯）与 [third-party-test-gap-analysis](2026-10-06-third-party-test-gap-analysis.md)（第三方测试缺口）的**排期口径**。它把「框架实际走到的 JDK 方法集」定为 API 类工作的优先级依据。
> 终局不变：Java 生态第三方依赖全量可用。本文只决定**先后**，不排除任何 API。

## 一、口径

1. **API 面 = 框架调用链上的 JDK 方法集**，不是一跳直接引用。
   - 由 rava 闭包分析计算：以框架样例应用的 `main` 加种子为入口，开放世界取档案调用链。
   - `jdk_method_scan.py` 的常量池一跳引用只作为补充：它可以发现样例应用没走到的公开面，但漏掉 JDK 内部传递调用。
   - 两者取并集，按「引用该方法的框架 jar 数 × 调用链命中」排序。
2. **语义类测试不按 API 筛。** 01_basics、OOP / 多态分派、泛型、异常、lambda / indy、字符串拼接等测的是字节码到 Rust 的翻译正确性，与 API 面无关，一律保留在主力集。
3. **API 类 e2e 按面分三层**：
   - **主力**：用到的 JDK 方法与当前阶段 API 面有交集。日常抽查和合入检查都从这里选；失败按正常优先级修。
   - **补测**：API 面上有、但 e2e 零覆盖的方法。按排序补写测试；优先复用框架 golden mains 覆盖，不够再写 e2e。
   - **暂缓**：只覆盖面外 API 的用例。只在全量轮次跑；失败不阻塞、不排修，直到某个后续阶段的框架把它纳入 API 面，再自动升回主力。
   - 测试不删、不改，只调整分层。覆盖等价的冗余剪枝仍按原规则另行处理。
4. **面外 API 的支持工作同样暂缓**：缺口审计（`rava audit`）、手写补齐、性能调优，都以当前阶段的 API 面为准。

## 二、阶段

| 阶段 | 框架集合（钉代：Spring 6.2 / Boot 3.x，不上 7.x） | 验收 |
|---|---|---|
| S0 | 最小 Spring Boot 应用：spring-boot、spring-boot-autoconfigure、spring-context、spring-beans、spring-core、spring-aop、spring-expression、spring-jcl，以及 starter-logging（logback + slf4j）、snakeyaml、jakarta.annotation、micrometer-observation | S0 API 面的主力层 e2e 全过；最小 Boot 应用（ApplicationContext 启动、@Component / @Bean 注入、@Value 与 application.yml、CommandLineRunner 输出）golden 一致 |
| S1 | spring-web / webmvc 与内嵌 Tomcat、Jackson（databind / core / annotations） | 同上，加 REST 处理链 golden（网络面依 K9 口径） |
| S2 | 持久层：JDBC（K7）、HikariCP、H2、spring-jdbc / tx，之后 MyBatis / Hibernate | 同上 |
| S3+ | 按矩阵热门顺序：commons 系列、Guava、Caffeine、Netty 等 | 每加一个框架，API 面并集扩大，暂缓层相应收缩 |

各阶段依赖的基础能力（K7 JDBC、K8 并发、K9 网络、K11 子类合成等）以 framework-pilot-test-matrix §二为准。某阶段被能力闸门挡住的部分，记入该阶段「现状」节，不降低阶段目标。

## 三、步骤

1. **S0 面计算**（分析，不改生成器）
   - 写最小 Boot 样例应用（放在 `tests/lib_pilot/` 下，jar 走 `fetch_pilot_deps.sh` 的 pom，不进 runtime/）。
   - 在 dev 上用作业模式跑闭包，取 JDK 方法集。结合 `jdk_method_scan.py --jar` 对 S0 各 jar 的一跳面，产出 `docs/reports/api-surface-s0.md`（方法集与排序）和机器可读的 `tests/api_surface/s0.txt`。
   - 闭包跑不通（如 K11 子类合成、CGLIB 等）就记为闸门项，并用一跳面先行。
2. **e2e 分层**：用 `tests/api_surface/<阶段>.txt` 与每例的 JDK 方法集求交，再加语义类目录白名单，生成分层清单。
   - run_tests 增加 `--tier main|all` 等选项；分层由数据文件决定，Python 里不写 JDK 类名特判。
   - 合入检查与日常抽查默认用主力层。
3. **补测与修复**：按 S0 面排序补测试并修失败，直到 S0 验收达成；之后进入 S1。
4. **C4 全量**照原计划用全集跑（判据不变）。暂缓层的失败在分类表里单列为「暂缓」，不计入未决回归。

## 四、现状

- 10-07：口径拍板，文档立项。步骤 1 等子代理名额（当前 5 / 5 满），有空位即派。
- 10-07 用户追加：「现在有一些失败的 Java 测试所用的 API，可能在这些第三方依赖包里不会用到，可以先延后或暂缓解决」。已知失败清单首批人工判定（`docs/known_failures.toml` 的 deferred 字段）：
  - 暂缓 9 例：TestModuleLayerDefine、TestLocaleDateCjk、TestVirtualThreadScale、TestStringGetCharsLegacy（面外）；TestCharsetAvailable、TestHttpLoopbackSync / Async、TestXmlTransform（S1）；TestRowSetProvider（S2）。
  - 保留：TestClassModuleFace（Spring 6 PathMatchingResourcePatternResolver 扫描 ModuleLayer.boot()）、TestProtectionDomainFaces（Boot ApplicationHome 取 CodeSource）、TestSetAccessibleBoundary（ReflectionUtils.makeAccessible）、TestXmlSaxEvents（logback 配置走 SAX）、JUnit 10 例（框架 pilot 本身）。TestLocaleCurrency 属 expected 基线缺陷，另案。
  - S0 API 面算出后按数据复核这份判定。
- 10-07 步骤 1 开工（分支 api-surface-s0，worktree java_rta_apis0）。已完成：pom 补 S0 jar（Boot 3.5.16 / Spring 6.2.19 同代）；最小 Boot 样例 `tests/lib_pilot/s0_boot/`；面与分层脚本 `scripts/api_surface.py`（face / tiers / seeds）、`scripts/jdk_index.py`（jmods 类层次，引用解析到声明类）、作业脚本 `scripts/api_surface_job.sh`；语义类目录白名单 `tests/api_surface/semantic_dirs.toml`；报告框架 `docs/reports/api-surface-s0.md`。
  - dev 实测（apis0-s0-38c29002）：取包成功（锁 102 条）；样例在真 JVM 上 rc=0，输出已记入报告；实载 JDK 类 1722、框架 / 样例类 1987（作闭包变体 B 的种子）。
  - **交接**：闭包 A 刚启动时 dev 关机维护，作业被停。dev 恢复后在本 worktree 推送最新提交，发作业 `--cmd 'bash scripts/api_surface_job.sh s0' --fetch 'build/api_surface/s0/**' --fetch 'tests/api_surface/*' --slot-mem dev=28 --job-timeout 7000`（换新 tag），取回 `s0.txt` / `tiers_s0.toml` 入库，按 `face.json` / `tiers.json` 补报告第五～七节。作业脚本已改为参考 JDK（首轮用了服务器系统 JDK 21.0.12）。
