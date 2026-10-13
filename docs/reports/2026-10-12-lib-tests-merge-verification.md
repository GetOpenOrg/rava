# lib-tests-merge 分支测试机验证报告（2026-10-12）

> 测试机验证轮：分支 `lib-tests-merge`（基线 8b528aa8，收测时已进到 5adc8238 §七）。
> 环境：worktree `/data/workspace/rava-wt/lib-tests`、参考 JDK 21.0.11（共享主检出）、
> 取包 188 jar / 依赖锁 188 条坐标（`fetch_pilot_deps.sh --no-scan` rc=0）。
> 重进程一律经 `/data/workspace/heavy_lock.py`（机器约定）；闭包病态例统一 25–50 分钟超时击杀记台账。

## 一、总体验收结论

| 项 | 结论 |
|---|---|
| ② harness 回归（向后兼容） | **通过**：m1、jc1 双 GOLDEN OK，仓内存档 golden 逐字一致零漂移（m1 转译 849s / 编译 161s；jc1 转译 33s / 编译 123s）——前提是先修复 §三.1 的 harness 回归 |
| ⑤ e2e 17 例（run_tests） | **17 passed / 0 failed / 0 skipped**，全绿 |
| ③ 47 单文件 pilot（JVM 侧） | **46/47 golden 干净生成**（含 3 处 cp 收敛修复 + 2 例手工补跑）；唯一例外 AsmCommonsMain 为语料缺陷（§四.2） |
| ③ 47 单文件 pilot（rs 侧） | **21 GOLDEN_OK / 26 非绿**（明细 §二）；A/B 批"预期可点亮"的预判与实况出入大（B 批 0 点亮），已按库归因 |
| ④ 多文件项目首例 guava | JVM 侧 ✓（11 行）/ 转译 ✓（发射完成）/ rs 编译 ✗ E0308（guava 属 C1D/T2 闸门，预期内）——项目模式管道本身走通（修复 §三.1 两处后） |

## 二、47 pilot 结果矩阵（rs 侧最终口径，含重跑覆盖）

| 库 | 入口数 | GOLDEN OK | 失败模式（首错） |
|---|---|---|---|
| slf4j-api | 3 | 2 | Slf4jBasicMain GOLDEN_DIFF：仅差 JDK helpful NPE 消息（两侧同行为同 NPE） |
| joda-time | 3 | 0 | 3 例同根因编译 E0308：`date_time_formatter_builder_time_zone_id.rs:363` `Clone::clone(&String)` 绑到 `&Object` 形参（joda 源 line 2408） |
| commons-lang3 | 5 | 5 | — |
| commons-io | 3 | 3 | — |
| ognl(+javassist) | 3 | 0 | **转译闭包病态**：37min 单核 26.8G RSS 未出发射（击杀）；JVM 侧补 javassist 后修复 |
| jsoup | 2 | 0 | 2 例同根因编译 E0407：`Elements` 的 `remove_i` 非 `Nodes__VTable` trait 成员 |
| caffeine | 2 | 0 | Manual 编译 E0308（`bounded_local_loading_cache.rs:364` 泛型参数）；Weigher 运行失败 `LocalCacheFactory.newFactory`→`Class.forName` 动态类加载（C-SYN 邻域） |
| snakeyaml | 2 | 0 | **转译闭包病态**：51min 58.1G（击杀）；JVM 侧 golden 已生成 / 手工补齐 |
| typesafe config | 1 | 0 | 编译 E0599：`serialized_config_value.rs:648` `EnumMap.put_obj_obj` 方法缺失 |
| assertj | 1 | 0 | 编译 E0407：`usingDefaultComparator` 非 `AbstractComparableAssert__VTable` 成员 |
| fastjson2 | 2 | 0 | **转译闭包病态**：82min 66G（击杀）；JVM 侧 golden 已生成 / 手工补齐 |
| logback | 2 | 0 | JVM 侧补 slf4j-api 后过；rs 编译 E0624：`logger_context.rs:652 getLifeCycleManager` private |
| servlet-api | 4 | 4 | — |
| persistence-api | 2 | 2 | — |
| spring 工具族 | 4 | 3 | GenericResolver/ReflectUtils/UtilAssert OK；ProxyJdk：JVM 侧补 spring-aop 后过，rs **闭包病态** 50min 42.7G（击杀） |
| jackson-databind | 3 | 0 | **转译闭包病态**：37min 36.3G（击杀） |
| asm | 3 | 1 | Generate OK；Transform：JVM 侧补 asm-commons 后过，rs 编译 E0624 `addForwardJump` private；Commons：**语料缺陷**（§四.2） |
| log4j2-api | 1 | 1 | — |
| kotlin-stdlib | 1 | 0 | 转译失败（干净报错）：合成名 `_i_a_-apply-CollectionsKt___Collections…` 含 `-`，非法 Rust 标识符（E-3 多语言 jar 闸门典型形态） |

合计：**21 GOLDEN_OK**；编译缺口 11（E0308×3 组 / E0407×2 组 / E0599 / E0624×2 组）；闭包病态 5 库 11 例（ognl+javassist / snakeyaml / fastjson2 / jackson-databind / spring-aop 动态代理面）；运行失败 1（动态类加载）；语料缺陷 1；行为一致仅差异常消息 1。

## 三、harness 修复（本测试轮提交，均非生成器语义）

1. **rs 侧源列表路径回归（阻塞全部模式）**：分支把 `rava build` 源参数从 main 的 `tests/lib_pilot/$MAIN.java` 改为相对 `tests/lib_pilot` 的裸文件名，而调用前已 `cd "$REPO_ROOT"` → id/单文件/项目三种模式转译必挂 `file not found`。修：调用前统一补 `tests/lib_pilot/` 前缀（`RAVA_SOURCES`）。
2. **项目模式 /tmp 日志路径**：`/tmp/${MODE}_*.log` 在 MODE 含 `/` 时落在不存在目录，重定向即失败（转译从未启动）。修：`MODE_TAG`（`/`→`_`）。
3. **项目模式简单名入口的 JVM 侧缺口**：`java <简单名>` 找不到带 package 的主类；FQCN 直达可用（本次验证即走 FQCN）。简单名解析留给开发机（需从 package 声明推 FQCN）。
4. **cp 收敛 4 处**（§四.1 授权范围）：ognl→+javassist（JVM 侧 `OgnlRuntime` 要 Javassist）；SpringProxyJdk→+spring-aop（aopalliance 接口打包在 spring-aop jar 内）；logback→+slf4j-api（logback-classic 编译期要 slf4j SPI 类）；AsmTransform→+asm-commons。四处修复后 JVM 侧全部转绿。

## 四、需开发机接手的事项

1. **闭包分析器病态（最重要发现）**：5 个反射/序列化重库单核跑 25–82 分钟、RSS 26–66G 不出发射（javassist、snakeyaml、fastjson2、jackson-databind、spring-aop 动态代理面）。共性是运行期反射类型格；建议生成器侧立项（闭包剪枝或超时降级），在此之前这些库的 rs 侧验证只能跳过。
2. **AsmCommonsMain 语料缺陷**：运行期硬读上游工程产物 `target/classes/com/example/cglib/User.class`（NoSuchFile）。自足化漏网（AsmTransformMain 已做内联，此例没做），需按同法改语料。
3. **简单名入口 JVM 侧**（§三.3）。
4. **编译缺口台账**（§二矩阵）转各闸门立项：E0407（vtable trait 成员，jsoup/assertj）、E0308（joda Clone 绑定 / caffeine 泛型 / guava flat_map_spliterator 桥接）、E0599（EnumMap 方法族）、E0624（private 方法发射，logback/asm）。
5. **helpful NPE 消息**（Slf4jBasicMain 唯一差异）：JDK 21 helpful NPE 文本未实现，行为两侧一致，低优先。

## 五、e2e 17 例明细（run_tests，参考构建 jdk-21.0.11+10）

`Results: 17 passed, 0 failed, 0 skipped / 17 total`。34_concurrency +8（Barrier 族 6 + Phaser + VirtualThreadBarrier）、39_bits +7、15_switch +2 全绿；典型量级：transpile 3.7–27.6s、build 36s–3m38s、bin 77–252M。

## 六、golden 存档与复现

- 本轮新档：47 pilot 的 `*_jvm.txt` 全量 + 21 例 GOLDEN_OK 的 `*_rs.txt` + `guava_GuavaCollectionsDemo_jvm.txt`（多文件项目首档），随本提交入库。
- 台账原始件：`/tmp/libtests/ledger.tsv`（测试机），关键内容已并入 §二矩阵。
- 复现入口：`scripts/lib_pilot_golden.sh <MainName|库/入口FQCN>`；重负载建议包 `heavy_lock.py`。

## 七、判定与建议

- **分支可合入 main 的部分**：e2e 17 例、47 pilot 语料与 deps 钉值、graalvm 原始件、本轮 harness 修复与 golden 存档——均不依赖未点亮能力。
- §七 e2e 放量节奏修正建议：A/B 批并非全绿（B 批 0 点亮），按 §二矩阵的实际点亮集（lang3/io/servlet/jpa/spring 工具族/log4j2-api/asm 生成面/slf4j×2）进 75+ 目录；闭包病态 5 库与编译缺口库登记 known_failures 或暂缓，等闸门立项。

## 八、后续处置（10-13）

§四移交事项在后续分支的收口情况，截至 2026-10-13：

- **AsmCommonsMain 转绿**（`libtests-corpusfix`）：§四.2 的语料缺陷已按 AsmTransformMain 同法自足化，rs 侧第 22 个 GOLDEN OK。
- **JVM 侧 golden 全量收齐**（`libtests-jvm-goldens`）：28 项目 55 入口全跑，45 干净、19 处 CP_FIX（§三.4 的按锁补 cp 口径常态化）。
- **闭包病理五库已定位**（`libtests-closure-diag`）：§四.1 的 worklist 不动点三层爆点已定位并取得栈证据，为生成器侧剪枝/降级立项提供了直接输入。
- **上游清障收尾**（`libtests-upstream-fix`，本分支）：frameworks 项目补钉 `io.micronaut:micronaut-http:4.10.31` + `org.springframework:spring-web:6.2.19` 入 deps/pom.xml（依赖锁 188→197 条），FrameworkComparisonDemo / RouteAnnotationDemo 两入口 JVM golden 干净落档；asm 项目补上游遗漏的 `ChildClassLoader`（同包顶层类，defineClass 公开化），AsmProxyDemo JVM golden 落档。遗留：HelloEndpoints 上游即无 main（holder 类），入口口径待定夺。
