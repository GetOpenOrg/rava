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

## 八、rs 侧台账复验（libpilot-gaps 线，2026-10-13）

> 范围：§二矩阵 26 例非绿中的**编译/运行缺口项**（闭包病态 5 库 11 例另线在诊、不在本轮）。
> 基线 `j25-r2-fix`@945ee5e7（= batch-1014 + JDK25 修复），修复落 `libpilot-gaps`（a2848077 / 43466d06 / 35cede3f）。
> 作业 lpg-r1/r2/r3/r4（dev+ubuntu 错峰，盯 summary.json）；全源抓取 `*_full.tgz` + logs/build.log 复验证据。

### 8.1 逐例结论（13 例编译/运行缺口 + 语料缺陷 1 例引用）

| 例 | 首轮签名（lib-tests 轮） | 复验结论 | 现态 / 归属 |
|---|---|---|---|
| LogbackConfigMain | E0624 getLifeCycleManager private | **GOLDEN_OK**（lpg-r1，457s） | 基线消解（batch-1014/j25-r2-fix 修复面） |
| LogbackPatternMain | E0624 同族 | **GOLDEN_OK**（lpg-r1，638s） | 同上 |
| AsmTransformMain | E0624 addForwardJump private | **GOLDEN_OK**（lpg-r1，445s） | 同上 |
| AsmCommonsMain | 语料缺陷（运行期硬读 target/classes） | 已消解（引用 `libtests-corpusfix`：语料自足化后 rs 侧第 22 绿，另线） | 不在本线 |
| JodaBasicMain | E0308 Clone::clone(&String)→&Object | 仍败，**错误面收敛 10→7**（lpg-r3）：TimeZoneId store 族与 default_name_provider 阴影绑定族消解（43466d06）；剩余 = 变量提升延迟声明注解与 sim 槽追踪类型不一致（UnsupportedDateTimeField.getInstance 形态，见 8.3-R1） | 部分修复 |
| JodaChronologyMain | 同上 | 同 JodaBasic（7 错） | 部分修复 |
| JodaPeriodMain | 同上 | 同 JodaBasic（7 错） | 部分修复 |
| JsoupParseMain | E0407 remove_i 非 Nodes__VTable 成员 | 仍败，**错误面收敛 5→3**：E0407+E0782 消解（a2848077）；剩余 = NodeIterator\<T extends Node\> 泛型上界视图（8.3-R3） | 部分修复 |
| JsoupSelectorMain | 同上 | 同 JsoupParse（3 错） | 部分修复 |
| CaffeineManualMain | E0308 泛型参数 | 仍败（4 错不变）：接收者擦除 vs 实参编组信息序（8.3-R2） | 未修（根因已清） |
| CaffeineWeigherMain | 运行失败 Class.forName 动态类加载 | 仍败：ClassNotFoundException SSLMW——jar 内 510 个构建期生成类（cache/[A-Z]{3,8}）经计算名 Class.forName 装载，全不入闭包（emit 侧 0 个）。**口径：C-SYN 邻域**（闭包合成），候选 ① 前缀合成种子（代价大）② 库白名单 seed ③ 记 known_failure；pilot 侧暂按 ③ | 未修（口径已定） |
| HoconParseMain | E0599 EnumMap.put_obj_obj | **签名迁移**：EnumMap 族已消解，首错改为 java_base_decl 手写层构造器请求缝（IOException::new_str 等 ×10，8.3-R4） | 部分修复（新缝登记） |
| AssertJChainMain | E0407 usingDefaultComparator 等 ×76 | 仍败，**错误面收敛 76→1**（lpg-r4 终验：a2848077 消 E0407×72、35cede3f 消露出的 E0201×3）；剩余 1 错 = `__set_myself` 泛型自类型字段（AbstractAssert<SELF,ACTUAL>→SELF，8.3-R3 同族） | 修复（余 1 错归 R3） |
| Slf4jBasicMain | GOLDEN_DIFF 仅 helpful NPE 消息 | 仍 DIFF（唯一差异行：JVM 侧带 JEP 358 helpful 消息文本，rs 侧同 NPE 无消息；其余 11 行逐字同）。**口径：行为一致、非编译缺口**，候选 golden 白名单「异常消息文本除外」或语义完备度立项，归主会话裁决 | 口径已定（登记） |

### 8.2 本轮修复（3 提交，均本地 emit-only 实证 + 服务器复验）

1. **a2848077（E0407 族）**：协变桥成员槽位归属取槽族根——`covariant_bridge_pending::slot_root` 由「精确描述符最远声明者」改为按各声明层 `resolve_virtual_slot` 的槽族根；`resolve_bridge_member::climb_slot` 在槽主发射不在本项目 ems（JDK 档案侧类 T1 复用）时采信声明侧 virtual_in 而非停在中间祖先。实证：jsoup elements.rs 的 `Nodes_super_remove_i` 毒条目消失。
2. **43466d06（E0308 store 族）**：无 LVT 合成局部的 Object 装箱——`needs_object_boxing` 补无声明分支（槽上既有局部 Object + 存入非 Object 引用值 → box_object）。joda 全 jar 无 LVT，`aconst_null` 首存定型 Object 后的具体类值存储原直发 `Clone::clone(&v)`（期望类型驱动解析到 `<Object as Clone>::clone`）。实证：`local_9_1 = Object::from(Clone::clone(&local_11))`；阴影绑定族（E0599 array_load_object ×4）级联消解。
3. **35cede3f（E0201 族，a2848077 露出）**：接收者真实方法已占同一 vtable 槽位时不再补桥成员（`resolve_bridge_member` 补 recv 侧同槽守卫，与既有 inherited_real 守卫对偶）。

### 8.3 剩余清单（根因 + 候选；failure_patterns.toml 已登记 7 条）

- **R1 提升声明注解 × sim 槽追踪类型不一致**（joda ×3 剩余，`synth-local-object-boxing-gap` 余项）：延迟声明 `let mut local_N: 具体类;` 的注解取分支合并类型，槽追踪类型停在 null 首存的 Object——装箱赋值 `Object::from(..)` 撞具体注解（旧形 E0381 阴影 let，转换后仍败例非回归）。候选：method crate blocks/merge 的 join 类型回写槽追踪。
- **R2 接收者擦除 × 实参编组信息序**（caffeine-manual ×3，`erased-receiver-arg-marshal-mismatch`）：调用文本接收者已 `Into::<LocalCache<Object,Object>>` 擦除而实参按 K/V 签名先行编组（`Clone::clone(&key)`）。候选：接收者擦除路径回写 sig_params 或编组延后；归类型收敛路线。
- **R3 泛型上界视图**（jsoup ×2 剩余，`generic-field-bound-view-mismatch`）：`NodeIterator<T extends Node>` 字段按 T 发射、体 checkcast 到上界 Node，T↔上界的边界转换缺失。
- **R4 手写层构造器请求缝**（hocon 首错族，`handwritten-ctor-request-seam`）：手写 `*_impl.rs`（按闭包裁剪编译，856297e7 显性化）的 `X::new_str(..)` 请求不进字节码调用链 → 壳构造器面缺位；与 j25-r2 交接项 3（CheckOutputDeviceIsATerminal 的 IllegalArgumentException::new_str）同族。候选：input/closure 扫描手写请求作种子，或手写边界类构造器面免裁剪。
- **R5 动态类加载（计算名）**（caffeine-weigher，`dynamic-classload-buildtime-generated`）：C-SYN 邻域，口径见 8.1。
- **R6 helpful NPE 消息文本**（slf4j，`helpful-npe-message-text`）：口径见 8.1。
- **R7 既有已修登记**：`covariant-bridge-slot-root-mismatch`（fixed）、`synth-local-object-boxing-gap`（partial）。

### 8.4 验收

- 修复面代表例：E0624 族 3 例 GOLDEN_OK（lpg-r1，**lpg-r4 复跑仍全绿——3 提交无回归**）；assertj 76→1、jsoup 5→3、joda 10→7（lpg-r3/r4）。
- JDK21 e2e 回归抽 3 例（lpg-r3 job08）：FunctionalCompositionTest / TestGetFields / TestCollectionsFactories **3/3 PASS**（Elapsed 23m04s）。
- 转绿面回归哨兵：LogbackConfigMain / LogbackPatternMain / AsmTransformMain 复跑 GOLDEN_OK ×3（lpg-r4，确认 3 提交不回退）。
- generator `cargo check --release --tests`（heavy_lock）：emit/sim/instr/driver 全过；sim/emit 单测 105 例全绿。
