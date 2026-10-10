# 第三方依赖包 Java 测试缺口分析与补充清单

日期：2026-10-06
范围：tests/lib_pilot（golden mains + e2e 回归）视角的第三方依赖测试补充
依据：`2026-10-03-framework-pilot-test-matrix.md`（主推 #1–#13 与 W5 候选池）、
`2026-10-03-e2e-capability-coverage.md`（K 表）、`2026-10-05-junit-e2e-deps-task.md`（J0–J4）、
`compatibility.md`、lib_pilot 在盘资产实测盘点

> **排期口径更新（2026-10-10）**：本文 §二、§五 的「A 批零门槛优先」排序已由
> [`2026-10-07-framework-driven-api-coverage.md`](2026-10-07-framework-driven-api-coverage.md)
> （按 Spring Boot 调用链排 API 面）与 10-10 用户定的推进顺序取代：反射组修复 → JUnit（J3 形态接线 +
> J4 注解驱动反射入口通用建模，让 63_junit 10 例跑通）∥ spring-core / beans 切片（§二 C，视为已拍板）
> → S0 最小 Boot 应用。W0-4（m5 `@Before` 字段写回）已于 10-05 J2A 5/5 GOLDEN OK 收口。
> 库种子与配置归用户项目、rava 不设 `runtime/lib_runtime`（10-06 定）。本文其余库清单与 main 级枚举
> 作为 S1 及以后阶段的测试候选池保留。

## 〇、终局定位与口径（先读）

**终局目标：Java 生态的第三方依赖包全量可用 rava 编译运行。** 本文档的所有分层
（零门槛/拍板/等闸门/远期）表达的是**到达终局的排期与前置能力建设顺序，不是取舍**——
没有任何库被排除在目标之外。运行验证按"主流热门优先覆盖到位、长尾随能力完备自然解锁"
推进：Top 生态（Spring 全家、持久层、日志、JSON、集合工具、HTTP 客户端等）在矩阵梯队中
全部有名有姓；长尾库共享同一套字节码语义面，能力闸门放开后即自动受益，无需逐一立项。

**构建期与字节码工具链（Maven/Gradle/ASM/byte-buddy/CGLIB 等）同样在测试目标范围内，
要求能编译、能运行**——它们不是目标域外的特例，差异只在**测试拆解方式**：操作字节码的
库天然分两面，"生成面"（ClassWriter/ByteBuddy DSL 产出 byte[]，纯计算，不加载类）可以
先行开测，"加载/运行面"（defineClass 消费产物）等 C-SYN 替代协议落地后放量；Maven/Gradle
作为完整应用的运行目标走分期攻坚（见 D 层）。

## 一、现状一句话

**资产先行、测试严重滞后**：`deps/target/pilot-libs/` 已备 92 个 jar（含矩阵 W1–W5 与
远期判据制的全部库），但已写 golden mains 只有 5 个（JUnit+Hamcrest m1–m5，2026-10-05
已 5/5 GOLDEN OK），e2e 依赖包回归 0 例在跑（63_junit 10 例定稿未接线，J4 当前 0/10）。
即：**矩阵里 13 个主推 pilot + 19 个候选 pilot 全部没有 Java 测试**。

## 二、需要补充的测试（按能力闸门分层）

### A. 零门槛立即可补（能力已解锁 + jar 已在盘，无需任何拍板）

| 优先 | 库（版本） | 建议 mains 数 | 锻炼点 |
|---|---|---|---|
| 1 | slf4j-api 2.0.20 | 3 | 门面 + ServiceLoader 静态目录（C-SPI） |
| 2 | joda-time 2.14.4 | 3 | 大量纯计算、不可变对象、Chronology 继承树 |
| 3 | jackson-core 2.22.3 | 4 | 流式解析状态机、enum switch、内部异常体系 |
| 4 | commons-lang3 3.20.0 | 5 | StringUtils/NumberUtils/反射工具（L3 分派实测） |
| 5 | commons-io 2.22.0 | 3 | IO 边界（需先做半天 IO 边界复核） |
| 6 | OGNL 3.4.14 | 3 | **反射重灾区**（矩阵注明可插队 W1 后任意位） |
| 7+ | 在盘顺手件：gson、commons-codec、commons-text、commons-csv、commons-math3、commons-jexl3、commons-beanutils、commons-collections4、picocli、eclipse-collections、commons-compress | 各 2–4 | gson 在盘但矩阵未排期，建议进 W5 |

### B. W5 高优先候选（jar 已拉，纯计算面，随 A 批尾随）

Jsoup（解析树构建）、Caffeine（Wainer 泛型 + 并发数据结构，单线程面可测）、
SnakeYAML、typesafe config、AssertJ（fluent 链式断言）、Fastjson2 流式；
Logback 排在 slf4j pilot green 之后（绑定协商路径）。

### C. 一句话拍板即可开

- **spring-core 6.2.x 切片**（D-3 已确认代次、jar 已拉）：反射工具族 + 接口代理消费面
- **jakarta.servlet-api 6.1 / persistence-api 3.2**：无状态容器件可先行；
  **有状态件建议等 W0-4（@Before 字段写回）修复后补**

### D. 分期攻坚（终局同样覆盖，当前排期靠后，等对应能力立项放量）

这些库全部在终局覆盖范围内，标注的是"先立什么能力、再放什么量"：

| 库 | 前置能力（立项后即放量） |
|---|---|
| H2 → mybatis | C-SQL（jmods 翻译 java.sql，D-4 已拍板） |
| guava 切片 | C-C1D/T2 闭包收窄收官（预估 2000+ 类） |
| spring-context IoC / Boot | C-SCAN 类路径扫描 + 运行期类生成替代协议 |
| hibernate-core | C-SQL + C-SYN 翻译期子类合成 + ANTLR 万级闭包实测 |
| HikariCP | C-SQL + C-MT 真并发 |
| RxJava / Disruptor | C-MT 真并发 |
| OkHttp 网络面 / Redis 客户端 | C-NET 网络 IO 边界 |
| byte-buddy / Mockito / CGLIB 族 | C-SYN：运行期字节码生成的替代协议——静态翻译模型的最深水区，终局必覆盖（Mockito/懒加载代理是真实生态刚需）。**生成面（byte[] 产出）不等 C-SYN，见 F 层** |
| Maven / Gradle | 完整构建工具的运行目标：类加载层级（ClassWorlds/隔离 classloader）+ C-SCAN 资源扫描 + C-SYN（插件运行期加载）+ Gradle 侧 Kotlin 运行时——终态分期最深处 |

### E. 建议新增入池的库（当前 92 jar 未覆盖）

1. **jackson-databind（在盘 jar）**：矩阵只排了 jackson-core；databind 是反射驱动的
   POJO 绑定，是 L3 分派最好的真实语料，建议作为 jackson-core green 后的增量切片
2. **log4j2-api**：与 slf4j/Logback 构成三绑定对照，体量小
3. **kotlin-stdlib 小切片**（中期）：非 javac 产出的字节码形态（伴生对象/内联/元数据注解
   /字符串拼接 indy 变体）——"Java 生态全量"包含 Kotlin/Scala 等其他 JVM 语言产出的 jar，
   这是该维度的第一块试金石，建议在 A/B 批稳定后排期
4. **ASM（asm/asm-commons）**：操作字节码依赖包的基石库，jar 未在 92 个之列，建议入池
   并尽早开测（矩阵中"特批元目标"地位的落地）——测试拆解见 F 层

### F. 操作字节码的依赖包：测试拆解（新增专项）

这类库的核心洞察：**"生成字节码"是纯计算**（输入字节码/DSL → 输出 byte[]），
不需要把产物 defineClass 就能完整验证生成逻辑；因此测试分两面推进：

| 面 | 内容 | 前置 |
|---|---|---|
| **生成面（可先行）** | ASM：ClassWriter 从零生成类（版本/字段/方法/指令）→ toByteArray → 用 ClassReader + ClassVisitor 回读，打印版本号/方法名/指令计数做断言；转换面：ClassReader.accept + MethodVisitor 改写（如加计时/改常量）→ 回读验证。byte-buddy：DSL 定义 → make() 拿 TypeDescription/字节产出，校验描述不加载 | 无（依赖闭包规模可控，asm 全家仅数百类） |
| **加载/运行面（终局）** | defineClass 产物并调用（Mockito mock 创建、CGLIB 代理、byte-buddy load）；懒加载代理（hibernate） | C-SYN 运行期生成替代协议 |
| **完整工具运行（终局）** | Maven（ClassWorlds 隔离加载 + plexus DI + 插件体系）、Gradle（+Kotlin 运行时）作为应用完整跑起来 | 类加载层级 + C-SCAN + C-SYN + Kotlin 支持 |

确定性注意：ClassWriter 对同一输入的字节产出应当稳定（同一 ASM 版本内），断言优先用
结构化回读（版本/名字/指令计数）而非裸字节哈希；computeFrames 相关的确定性在 pilot
探路阶段先验证一次再固化 golden。

## 三、测试形态规约（沿用 lib_pilot 既定，不新增机制）

- 每库 = 独立 pilot 五件套：jar 资产（deps/pom.xml 唯一版本源）→ `rava build --lib` lib crate
  + 固定种子表 → 单文件 golden main（静态导入被测 API、逐项编号断言、只打印 stdout）
  → e2e 回归目录 + form.toml（等 J3/W0-3 接线后放量 75_–87_）→ tasks.md 台账
- 确定性铁律照旧：禁无种子随机/时间/identityHashCode；HashMap 遍历先排序；异常自 catch 打印
- 一次一个 pilot、golden 先行、`--update-expected` 双侧对账入库

## 四、验收缺口（补测试之外必须接通的两条线）

1. **J4**：63_junit 10 例从 0/10 到 10/10（依赖 harness 支持依赖包，随 J2/J3）
2. **W0-3**：75_slf4j 起的 e2e 形态目录接线（等 H1 = C4 收官 + S6/S7）

## 五、建议的执行顺序（第一批）

A 批按表中优先 1→5 逐个补（合计约 18 个 mains），OGNL 插队；随后 B 批高优先 6 件；
F 层的 ASM 生成面与 A/B 批同属性（纯计算），建议 asm 入池后紧随 A 批开测；期间推动
J2/J3 接线让 63_junit 与新 pilot 能进 e2e 回归。预计 A+B 批完成后，
第三方依赖 golden mains 从 5 个增至 ~40 个，覆盖矩阵主推 13 件中的 10 件。

D 层各库不设"永不"：每一项都随对应能力立项进入放量序列，终态即 Java 生态第三方依赖
全量可编译运行；本清单随 C-SQL/C-C1D/C-NET/C-MT/C-SCAN/C-SYN 的落地逐批更新。

## 六、待确认的具体测试清单（main 级枚举）

> 形态一律按 lib_pilot 规约：单文件自含 main、静态导入被测 API、编号断言、只打印 stdout、
> 固定输入固定时区；JVM（真 jar classpath）与翻译 crate 双跑对账入 golden。

### 第一批 A（W1 五连 + OGNL 插队，6 库 21 mains，零门槛）

| 库 | main（断言内容） |
|---|---|
| slf4j-api 2.0.20 | `Slf4jBasicMain`（LoggerFactory/getLogger/级别开关/{}/异常格式化，NOP 绑定下无异常返回）；`Slf4jFormatterMain`（MessageFormatter 占位符转义/数组/-null 纯计算矩阵）；`Slf4jSpiMain`（ServiceLoader 绑定探测路径 → NOP，C-SPI 实证） |
| joda-time 2.14.4 | `JodaBasicMain`（固定时区固定时刻的 parse/format + 不可变链 plusDays/withDayOfWeek）；`JodaPeriodMain`（Period/Duration/Interval 区间计算）；`JodaChronologyMain`（ISO/GJ/Buddhist 继承树 + 字段枚举排序输出） |
| jackson-core 2.22.3 | `JacksonStreamParseMain`（JsonFactory 逐 token 解析固定 JSON 的序列断言）；`JacksonStreamGenMain`（JsonGenerator 生成 → 精确字符串比对）；`JacksonNumberEdgeMain`（long/double/BigDecimal 边界解析）；`JacksonJsonPointerMain`（JsonPointer 路径切分/compose/匹配） |
| commons-lang3 3.20.0 | `Lang3StringUtilsMain`（15+ 方法矩阵：abbreviate/swapCase/wrap/splitPreserve 等）；`Lang3NumberUtilsMain`（isCreatable/compare/toInt 边界）；`Lang3ReflectMain`（MethodUtils/FieldUtils 对固定类的调用——L3 反射分派实测）；`Lang3TupleMain`（Pair/Triple equals/hashCode）；`Lang3TimeFormatMain`（DateFormatUtils/DurationFormatUtils 固定时区） |
| commons-io 2.22.0 | `IoCopyMain`（IOUtils.copy/toByteArray 内存流往返）；`IoFilenameMain`（FilenameUtils normalize/extension/separatorsToSystem 矩阵）；`IoEndianMain`（EndianUtils swap 读写 + CharSequenceReader/CopyUtils） |
| OGNL 3.4.14 | `OgnlEvalMain`（Ognl.getValue 属性链/方法调用/索引）；`OgnlContextMain`（context 变量绑定与类型转换）；`OgnlCollectionMain`（投影 .{name} / 选择 {?...} / 过滤） |

### 第二批 B（W5 高优先 + Logback，7 库 15 mains）

Jsoup（`JsoupParseMain` 文档解析树结构断言、`JsoupSelectorMain` CSS 选择器命中矩阵）；
Caffeine（`CaffeineManualMain` 手动 get/put/失效 + 命中统计、`CaffeineWeigherMain` 权重/容量淘汰——单线程确定性）；
SnakeYAML（`YamlRoundTripMain` dump→load 往返等价、`YamlTypesMain` 标量/集合/锚点类型还原）；
typesafe config（`HoconParseMain` HOCON 固定文本 → 层级/替换/单位解析）；
AssertJ（`AssertJChainMain` fluent 链式断言消息内容比对）；
Fastjson2（`FastjsonStreamMain` JSONReader 流式读、`FastjsonPathMain` JSONPath 命中）；
Logback（排 slf4j green 后：`LogbackPatternMain` 固定 pattern → ListAppender 捕获比对、`LogbackConfigMain` programmatic 配置输出级别过滤）。

### 第三批 F（ASM 生成面，2–3 mains，asm 入池后紧随 A 批）

`AsmGenerateMain`（ClassWriter 从零生成类 → toByteArray → ClassReader/ClassVisitor 回读，
断言版本/字段/方法/指令计数）；`AsmTransformMain`（ClassReader.accept + MethodVisitor
改写 → 回读验证指令变化）；可选 `AsmCommonsMain`（AdviceAdapter/SerialVersionUIDAdder）。

### 拍板即做 C（3 件 ~10 mains）

spring-core 切片（`SpringReflectUtilsMain` ReflectionUtils/ClassUtils、
`SpringProxyJdkMain` ProxyFactory 的 JDK 接口代理面、`SpringUtilAssertMain`、
`SpringGenericResolverMain`GenericTypeResolver）；
servlet-api 无状态面（4 mains 按 matrix）；JPA 无状态面（2 mains 按 matrix）。

### 入池新增 E（3 库 ~6 mains）

jackson-databind（`DatabindSerMain`/`DatabindDeserMain`/`DatabindAnnotationMain`，
排 jackson-core green 后）；log4j2-api（`Log4j2ApiMain`）；kotlin-stdlib 中期小切片
（`KotlinStdlibMain` 集合/字符串顶层函数——非 javac 字节码试金石）。

### 在盘顺手件（可选 X 批，每库 1–2 mains）

gson（往返+TypeToken）、commons-codec（Base64/Hex）、commons-text（相似度/转义）、
commons-csv（固定 CSV 解析）、commons-math3（统计/组合）、commons-beanutils（属性拷贝）、
commons-collections4（Bag/BidiMap）、picocli（命令行解析不执行）、eclipse-collections
（select/collect/Bag）。

### 不属于"写测试"但必须接通的两条线

J4（63_junit 10 例 0/10 → 10/10，随 J2 `--deps` 新入口 + J3 form.toml 接线）；
W0-3（75_–87_ e2e 目录放量，等 H1）。

**数量合计（A+B+F+C+E 确认后）：约 28 库 / 55 个 golden mains，加上 X 批可达 ~70。**
