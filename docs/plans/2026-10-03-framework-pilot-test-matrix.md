# 框架依赖包测试矩阵：Servlet / MyBatis / Spring 与热门库 pilot 阶梯

> 日期：2026-10-03
> 来源：用户命题「像 JUnit 测试一样，创建 Servlet、MyBatis、Spring、Spring Boot 相关测试，每一个都是独立的，再加一些热门框架依赖包——分析要做哪些测试、建多少」；同日追问「SSH 三件套是否覆盖」→ §三-A 增补核查（Struts/Hibernate 原未覆盖，已补增量行）；再追问「生态还有什么热门缺口」→ §三-B 全类目扫描与候选池。
> 定位：[junit-crate-pilot](2026-09-23-junit-crate-pilot.md)（P0 已收官）与 [junit-crate-as-test-harness](2026-10-01-junit-crate-as-test-harness.md)（步骤 0 在途）的**扩展矩阵**——把 roadmap §四-3 的 P 阶梯展开为可派发的 pilot 清单，回答「哪些框架、多少测试、什么顺序、哪些 gated」。
> 事实基线：2026-10-03 仓库实测（52 jar 盘点、Proxy$Dyn 落地核查、java.sql 缺席核查、main @ 2b2e1f26）。

---

## 一、形态约定（与 JUnit 同构，零新机制）

每个框架 = **一个独立 pilot**，五件套：

| 件 | 形态 | 模板 |
|---|---|---|
| jar 资产 | `fetch_pilot_deps.sh`（pom 增行）或已在盘 | junit/hamcrest 同源 |
| lib crate | `rava build --lib NAME=JAR[:seed=...]`，独立 crate 名与固定种子表 | hamcrest/junit4 |
| crate 验收 | golden mains（lib_pilot 模式：JVM 真 jar vs 翻译 crate 逐字对账，golden 文本入库存档） | m1–m5 |
| e2e 回归 | `tests/e2e/<NN>_<name>/` 目录 + form.toml（jar 名 + 种子集 = 数据，Python 零库类名；`--pilot-libs` 指定 jar 目录，缺失自动取包，取不到判失败不静默跳过） | 63_junit（2a3f4397 定稿） |
| 台账 | tasks.md 行 + 本文状态列 | — |

**独立性**：每 pilot 的 jar / crate / 种子表 / golden / e2e 目录互不依赖，可独立增删、独立止损——不出现「测 A 必须先装 B」。**不建第二 runner**：golden 脚本只做 crate 验收，e2e 全部走 run_tests 形态接口。

**手写边界原则对齐（CLAUDE.md §0/§1，2026-10-03 用户重申）**：矩阵内一切支撑件——servlet 容器件、JDBC 驱动（D-4 拍板 = H2 jar 翻译，零手写）、各 pilot 的测试替身 fixture——**一律走 Java 侧实现 + 管线翻译**（用户域测试代码或被测 jar 本体，m5 跨 crate 回调模式的自然延伸），**不新增任何 runtime 手写层**；JDK 面（java.sql 等）从 jmods 翻译。性能或便利不是手写理由；新 pilot 若逼出运行期类定义等手写准入点，按 §1 三准入登记清单而非散写。

## 二、基础能力缺口全表（2026-10-03 对齐 fe197231；矩阵 = 本表的函数）

> 用户命题「要满足这些测试，项目还要具备哪些基础能力」。✅ 已具备 = 矩阵地基不再立项；❌/◐ = 真正的立项项，括号内记号与 §4.1 依赖源对齐。运行期字节码生成本体（byte-buddy/CGLIB/Mockito/Netty）维持排除，其静态替代路径 = K11。

| # | 基础能力 | 现状（证据） | 还要做什么（立项内容） | 服务的测试（gates） | 量级 / 决策前置 |
|---|---|---|---|---|---|
| K1 | 反射元数据与 L3 分派 | ✅ T3 增强：按接收者派发、接口 `__reflect_dispatch`、错接收者 IAE（StockTrans 分派 22949→1645）；跨 crate 元数据表已扩 | 无（随 pilot 用例扩审计线） | OGNL #8、H2/mybatis #9/#10、spring-core/beans #11/#12、beanutils、picocli、assertj | 已具备 |
| K2 | 动态代理（接口面） | ✅ `Proxy.newProxyInstance` → `Proxy$Dyn` 承载（FS-R R4a）；`isProxyClass`/`getInvocationHandler` 按字节码翻译 | 无 | mybatis MapperProxy、Retrofit | 已具备 |
| K3 | ServiceLoader 静态目录（C-SPI） | ✅ 2026-09-26 落地 | lib crate 场景一次实证（随 #1 slf4j 顺带完成） | slf4j 绑定、JDBC DriverManager（K7 内） | 已具备 |
| K4 | XML 栈（DOM/XPath/XSL） | ✅ 闭包内（m1 发射集含 xpath 树实证） | 无 | mybatis XML、logback/struts2 配置、digester | 已具备 |
| K5 | jar→lib crate 管线 | ✅ `rava build --lib`（S1–S5 后）；m1/m2 GOLDEN OK @ fe197231 复核 | 无 | 一切 pilot | 已具备 |
| K6 | 数值/时区/编码 golden 口径 | ✅ DisableIntrinsic + locale/TZ 固定（run_tests golden flags） | 无 | math3、codec digest、jackson 数字、joda | 已具备 |
| **K7** | **JDBC 面（java.sql/javax.sql）**（C-SQL） | ❌ runtime 无此包（find 零命中） | **立项**：接口面从 jmods 翻译（唯一路径，手写边界 §1）+ DriverManager SPI 接 K3 + **驱动侧已拍板（D-4，2026-10-03）= H2 真驱动 jar 翻译**（内存模式 `jdbc:h2:mem:` 零网络；Java fixture 路线弃） | #9 H2 → #10 mybatis、HikariCP（+K8）、Quartz、dbcp2/dbutils（在盘）、hibernate（前置之一） | 中（含 H2 pilot 本体，见 #9） |
| **K8** | **真并发线程模型**（C-MT） | ◐ 协作单线程调度（真超时不可达已入 compatibility.md） | **决策 + 立项**：线程模型终态（real-multithreading 计划在列）——单线程深化 vs 真并发 | servlet async、Disruptor、RxJava 调度器面、HikariCP、Caffeine 补强、guava ListenableFuture | 大；先拍板方向 |
| **K9** | **网络 IO 边界**（C-NET） | ❌ 未立项 | socket/NIO 边界层立项 + 翻译域口径拍板（网络是否入翻译域） | OkHttp 网络面、httpclient/httpcore 转正（在盘）、Redis 客户端、jakarta.mail 发送面、Dubbo/RocketMQ/Nacos | 大；先决策口径 |
| **K10** | **类路径扫描 / ClassLoader 资源枚举**（C-SCAN） | ❌ 未建模 | `ClassLoader.getResources` 等建模（K3 静态目录机制的自然扩展：把资源清单也静态化） | spring-context 全量、Boot 自动装配、Dubbo SPI | 中-大 |
| **K11** | **翻译期子类合成（类代理）**（C-SYN） | ❌ 未立项 | **新能力**：闭包内已知类在生成期合成代理 / 懒加载子类——cglib/byte-buddy 的静态替代，与 K2（接口版）互补 | hibernate-core 懒加载、Spring AOP / Boot 半 | 大；独立立项 |
| K12 | 闭包收窄到位（C-C1D） | ◐ T2 在途（b1+T3 已合：返回模型收窄、分派减量） | T2 收官 + 大闭包 pilot 成本实测 | 大闭包 pilot 的 e2e 放量（guava/lang3/mybatis/spring 切片）、W4 | 在途 |
| **K13** | **跨测试编译复用 / 预构建缓存**（C-CACHE） | ❌ 无（每 scratch 现场发射编译） | 指纹键控**生成源缓存**先行；rlib 复用挂 java-runtime-core 版本化（R9）；或 T1 跨测试编译复用方案 | e2e 放量成本、W4、「依赖包就是包」终态 | 中；Q3 用户一次决策 |
| K14 | e2e 形态接口（H1） | ⏳ S6+S7 既定队列（◀ C4 收官；dyn_compare 冻结中） | S6 dyn 并入 rava → S7 run_tests 拆 scripts/e2e/ + form.toml + 63_junit 模板 | 一切 e2e 目录（63_junit、64_… 段） | 既定队列 |
| K15 | 确定性时钟源 | ◐ 待核 | 时钟注入 / 确定性沙箱复核（golden 可复现前提；roadmap「时钟/随机确定性沙箱」逼出能力） | Quartz 调度、Caffeine 过期用例、logback 时间戳 | 小（若已有注入机制）——需核 |

**jar 资产盘点（2026-10-03 取包后 **92 个**在盘，`tests/lib_pilot/deps/target/pilot-libs/`）**：commons 全家、guava 33.7.1、jackson 三件套、gson、httpclient/httpcore（4/5 两代）、joda-time、picocli、eclipse-collections、assertj、junit/hamcrest + **新拉：servlet 6.1.0 / JPA 3.2.0 / hibernate 7.4.6 / struts2 7.4.0 / mybatis 3.5.19 / OGNL 3.4.14 / H2 2.5.252（字节码恰 21，双 JDK 兼容）/ spring-core+beans 6.2.19 / slf4j 2.0.20 / Logback 1.5.38 / Jsoup 1.23.2 / Caffeine 3.3.0 / SnakeYAML 2.7 / typesafe 1.4.9 / Fastjson2 2.0.65 / Retrofit 2.12.0 / TestNG 7.12.0**。版本决策表与字节码核查见 [e2e-capability-coverage §一](2026-10-03-e2e-capability-coverage.md)。目标 JDK = **21 与 25** 双跑（golden 先 21，JDK25 适配轮后 25 侧复跑）。

## 三、分梯队矩阵（核心表）

> 类数 = jar 全量量级（闭包按种子裁剪）；mains = crate 验收 golden 用例；e2e = 形态目录回归用例。**每梯队内排序即建议实施序。**

### 梯队 1：纯计算 / 纯 API 热身（jar 在盘，W0 闸门后即可开工）

| # | pilot | 类数量级 | 压测面（为什么值得做） | mains | e2e | 前置 |
|---|---|---:|---|---:|---:|---|
| 1 | slf4j-api 2.0.20（2026-10-03 升线：logback 1.5 配对，压过旧传递 1.7.36） | ~130 | SPI 绑定（静态目录消费）、MDC（ThreadLocal）、`{}` 消息格式化纯计算 | 3 | 5 | 无 |
| 2 | joda-time 2.14.4 | ~300 | 不可变对象族、历法纯计算、时区（tzdb 已内嵌的同源验证） | 3 | 8 | 无 |
| 3 | jackson-core 2.22.3 | ~280 | 流式 JsonParser/Generator 往返、数字/编码边角、token 状态机 | 4 | 10 | 无 |
| 4 | commons-lang3 3.20.0 | ~2900→种子子集 | **P1 既定**（roadmap）：StringUtils / 数值 / mutable / 时间格式化 | 5 | 12 | 无 |
| 5 | commons-io 2.22.0 | ~500 | IO 边界手写层的真实压力面（copy / charset / filename / Input/OutputStream 族） | 3 | 8 | IO 边界现状复核 |

备选（在盘不占首波）：commons-text（2/6）、commons-csv（2/5）、picocli 4.7.7（3/6，注解反射 L1–L3 + 类型转换）、gson 流式面（2/4，排除反射 TypeAdapter 族）。

### 梯队 2：spec API + 用户侧容器件（测试 main 实现接口、经管线翻译 = m5 跨 crate 回调模式复用——非 runtime 手写）

| # | pilot | 类数量级 | 压测面 | mains | e2e | 前置 |
|---|---|---:|---|---:|---:|---|
| 6 | jakarta.servlet-api 6.0（pom 增） | ~450 | spec 接口/注解族（WebFilter/WebServlet 元数据）、ServletRequest/Response 会话模型、Filter 链分派——**容器件由测试 main 自己实现接口**（同 JunitCrossCrateMain 的 BaseMatcher 子类模式），golden = 分派与数据往返 | 4 | 10 | 代次拍板（§六-2）；async 用例 gated on real-multithreading |
| 7 | jakarta.persistence-api 3.x（pom 增） | ~150 | JPA spec jar（@Entity/@Table 注解族、EntityManager/Criteria/Metamodel 接口面）——与 servlet-api 同模式的 spec-API pilot，**Hibernate 的 API 面先行** | 2 | 4 | 无 |

### 梯队 3：反射深水 + JDBC 深水（Proxy$Dyn / L3 / XML / java.sql 合流压力面）

| # | pilot | 类数量级 | 压测面 | mains | e2e | 前置 |
|---|---|---:|---|---:|---:|---|
| 8 | OGNL 3.x（pom 增） | ~350 | **Struts2 的灵魂**：表达式引擎——反射属性导航 / 方法调用 / 索引与投影 / 类型转换 / 静态成员访问，SpEL 同族能力但独立 jar 可单测（`Ognl.parseExpression` + `getValue` 对 POJO 求值即可 golden） | 3 | 8 | 无（L3 已有）；与 #10 互证反射深水面 |
| **9** | **H2 2.x（pom 增，D-4 拍板件）** | ~2500 | **真 SQL 引擎 + JDBC 全套真实实现**：手写递归下降 SQL 解析器（无 ANTLR 依赖）、查询执行、事务/回滚、MVStore 存储、`java.sql` SPI（Driver/Connection/Statement/ResultSet/DatabaseMetaData）在内存模式（`jdbc:h2:mem:`，零网络）下的完整语义 | 4 | 8 | **C-SQL 落地**（jmods 翻译）+ 种子裁剪（server/web console 网络面、FunctionAlias 运行期反射面排除）；后台写线程仅持久形态需要，MEM 形态线程面以 dep_scan 实测为准 |
| 10 | mybatis 3.5.x（pom 增） | ~1300 | **Proxy$Dyn 分派保真**（MapperProxy 经 Method 元数据路由）、POJO getter/setter 反射、XML 配置/映射解析（XPath 栈）、动态 SQL（if/foreach/where）、TypeHandler 族、L1 缓存——**对着 #9 H2 跑真 SQL** | 5 | 10 | **P：#9 H2 green** + C-SQL（D-4 已拍板） |
| 11 | spring-core 切片 6.x（pom 增） | 种子子集 | ResolvableType / MethodParameter（**泛型元数据反射 = hamcrest TypeSafeMatcher 同款能力的放大**）、AntPathMatcher、PropertyPlaceholderHelper、StreamUtils、MultiValueMap | 4 | 10 | L3 稳定（m3 已验）；版本基线拍板（§六-3） |
| 12 | spring-beans 切片（后置） | 种子子集 | BeanUtils / 属性编辑器族（无 cglib 无容器的部分） | 2 | 6 | 11 号验收 |

### 梯队 4：大闭包集合库

| # | pilot | 类数量级 | 压测面 | mains | e2e | 前置 |
|---|---|---:|---|---:|---:|---|
| 13 | guava 33.7.1 collect/base 切片（在盘） | 子集 | Immutable 集合族、Table/BiMap/ Multimap、Chars/Strings、hash——**P3 既定** | 4 | 12 | 闭包规模实测（种子裁剪后预估 2000+ 类） |
| — | eclipse-collections（在盘，备选） | 大 | 路线图远期名单项，压轴备选 | — | — | 按需 |
| — | httpclient/httpcore（在盘，备选） | 大 | 网络 IO 边界 | — | — | **网络边界决策 gated** |
| — | struts2-core 切片（pom 增，备选） | ~1600 | XWork 拦截器链、ValueStack、类型转换族——「不带完整 web 栈」的 MVC 内核 | 3 | 6 | **gated**：#6 servlet + #8 OGNL 验收后（Struts2 = servlet + OGNL + 拦截器的组装件，三者单独可达才轮到组装） |

### 终点：入场判据制（0 tests now，不是不做）

| 目标 | 入场判据 |
|---|---|
| spring-context IoC 全量 | 类路径扫描（ClassLoader 资源枚举）建模 + L3 全量稳定 + spring-beans 切片验收 |
| **Spring Boot** | IoC 全量 + servlet 容器件（用户域 Java 实现 + 翻译路径）成熟 + **运行期类生成替代协议决策**（launcher 嵌套 jar 类加载、自动装配、内嵌容器、AOP 代理）。与 Maven/Gradle/Mockito/Netty/byte-buddy 同列静态模型边界（roadmap「明确不早期碰」的正式化——北极星不是里程碑） |
| **hibernate-core 7.4.x**（2026-10-03 版本落位：org.hibernate.orm 7.4.6 + JPA 3.2） | JDBC 层落地（#9 H2 可复用为驱动）+ #10 MyBatis ORM 面验收 + **翻译期子类合成立项**（实体懒加载代理：闭包内实体类静态已知，代理子类可由生成器翻译期合成——byte-buddy 运行期生成的静态替代，与 Mapper 代理合成同族能力）+ HQL/ANTLR 生成代码规模实测（~万级类闭包） |

### §三-A SSH 三件套覆盖核查（2026-10-03 增补）

| SSH 组件 | 原矩阵覆盖 | 增补处置 |
|---|---|---|
| Spring | 部分：core/beans 切片（W3）+ context/Boot 终点判据 | 无新增 |
| Struts2 | **未覆盖** | 拆解为「OGNL + servlet + 拦截器」三件：**OGNL 独立成 pilot（#8，灵魂件，无前置）**；struts2-core 切片挂备选（gated = #6 + #8 验收后）——组装件不做独立攻坚，组成件单独可达后再组装 |
| Hibernate | **未覆盖** | jakarta.persistence-api 先行（#7，spec jar 与 servlet 同模式）；hibernate-core 挂终点判据（byte-buddy 懒加载代理 = 运行期类生成，静态替代路径 = 翻译期合成子类，属新能力立项；HQL/ANTLR 规模 ~万级）。**MyBatis 先行（#10）是正确的第一 ORM**——接口代理已被 Proxy$Dyn 解决，Hibernate 的类代理难题在 MyBatis 验收后再攻 |

> 若「SSH」实指 **SSM**（Spring + SpringMVC + MyBatis）：MyBatis 已在 #10；SpringMVC（spring-web/webmvc）随 spring-context 同判据（请求映射反射 + 容器 + servlet 容器件）。

### §三-B 生态缺口扫描与候选池（2026-10-03 二次追问「还有什么热门的没覆盖」）

**扫描原则**：按**能力压力面去重**，不按热度堆砌——与既有 pilot 压测面重叠度高的件（如又一个 utils 聚合库对 lang3/guava 无新增压力面）降优先级；spec-API 模式（#6/#7）可无限复制但只挑有独立压力面的。

**类别缺口表**：

| 类别 | 已覆盖 | 缺口（代表件） | 评注 |
|---|---|---|---|
| HTML/XML | JDK XML 栈（闭包内，m1 实证） | **Jsoup**（HTML+CSS 选择器，热度顶级） | 纯计算，完美候选 |
| 日志 | slf4j-api（API 侧，#1） | **Logback**（实现侧，slf4j 原配）、Log4j2 | 与 #1 组成完整日志栈闭环；log4j2 的 config-plugin 反射系统是备选压力面 |
| 缓存 | guava cache（#13 切片内） | **Caffeine**（事实标准） | 并发原语压力面 |
| 配置/YAML | —（commons-configuration2 在盘） | **SnakeYAML**、**typesafe config** | 小而纯计算，Boot 配置栈地基件 |
| JSON | jackson-core #3、gson 流式（备选） | **Fastjson2**（中文生态国民级）、org.json | fastjson2 流式面先行，反射 ObjectReader/Writer gated |
| 声明式 HTTP | httpclient（在盘，网络 gated） | **Retrofit**、**OkHttp** | Retrofit 可 **测试替身 Call 全离线测** = Proxy$Dyn + 注解元数据的教科书消费者（与 #10 互证）；OkHttp 纯计算件（HttpUrl/Headers/Cookie）可先行、网络面 gated |
| 响应式/并发 | —（real-multithreading 在列） | **RxJava**、**Disruptor**（LMAX 环形队列） | 调度器/真并发 gated；Disruptor 是线程模型的终极压力面 |
| 测试 | junit4/hamcrest ✅、assertj（在盘） | TestNG、JUnit5 | junit 计划文档已注 JUnit5 Launcher 显著更大；assertj 进池（在盘） |
| 工具聚合 | lang3/io/text/csv、guava | **Hutool**（中文生态）、vavr | 压测面与 lang3/guava 重叠度高——按切片低调排；vavr 函数式数据结构有独立面 |
| 字节码 | byte-buddy（排除） | **ASM**（库本体） | **元目标**：库本体 = 纯字节码状态机可静态翻译，「用翻译器翻译字节码工具」；特批件不占常规波次 |
| spec API | servlet #6、persistence #7 | **jakarta.validation**（Bean Validation）、**jakarta.el**（表达式语言）、json-p、websocket；mail/activation/transaction（在盘未用） | spec-API 模式复制；el 与 OGNL/SpEL 成语言引擎三连 |
| 表达式引擎 | OGNL #8、SpEL（随 #11） | **commons-jexl3（在盘）**、MVEL、Aviator | jexl3 在盘顺手，与 OGNL 互证 |
| 安全/密码 | commons-codec（在盘） | BouncyCastle、Tink | BC ~3000 类；**JCA 注册表已落地（2026-09-25）使其架构可达**，大备选 |
| 连接池/DB 周边 | mybatis #10（驱动 H2 #9） | **HikariCP**、dbcp2（在盘）、Jedis/Lettuce | HikariCP gated 真并发+JDBC；Redis 客户端 gated 网络 |
| Office/PDF | — | PDFBox、POI/EasyExcel | 大件 IO 混合，中后期 |
| 中文微服务 | — | Dubbo、RocketMQ、Nacos、ShardingSphere、Seata | 全家 gated（网络 + 类加载 + 动态 SPI），与 Boot 同列终点域 |

**候选池（W5+ / 替换位，提拔哪批 = §六-7 拍板）**：

| 档 | 件 | mains | e2e | 前置 |
|---|---|---:|---:|---|
| 高优先候选 | **Jsoup 3/8、Logback 3/8、Caffeine 3/6、SnakeYAML 2/6、typesafe config 2/4、Retrofit 3/6、Fastjson2 流式 2/5、AssertJ 2/5** | ~20 | ~48 | 多数无前置（W0 绿即可）；Retrofit 等 Proxy$Dyn 分派保真（与 #10 互证） |
| 在盘顺手件 | commons-codec 2/4、math3 3/6（S-19 数值延伸）、jexl3 2/4、beanutils 2/4（L3）、collections4 2/5、jakarta.mail 2/4（MIME 纯计算面）、activation 1/2、transaction 1/2 | ~13 | ~29 | 无（jar 在盘） |
| spec 增补 | jakarta.validation 2/4、jakarta.el 2/4、json-p 2/4 | 6 | 12 | 无（pom 增行） |
| gated 维持 | HikariCP、RxJava、Disruptor、OkHttp 网络面、Redis 客户端、PDFBox/POI、BouncyCastle、Log4j2（与 Logback 择一）、Dubbo 系、Hutool/vavr（低重叠优先）、ASM（特批元目标） | — | — | 各自判据如上表 |

候选池全量 ≈ **+19 件 / ~39 mains / ~89 e2e**；主推 #1–#13 与候选池合并的理论全景 ≈ 32 crate / ~85 mains / ~200 e2e——**不建议全景一次排期**，按「波次滚动 + 压测面去重」消化。

## 四、依赖排序与排期（完整版：条件 → 任务 → 解锁）

### 4.0 现状对齐（2026-10-03 fetch origin/main @ fe197231）

- main 较本文初稿基线（2a3f4397）新增 12 提交，全部闭包分析器域：C1d-b1（返回模型收窄）+ C1d-b T3（**L3 反射分派按接收者、接口 `__reflect_dispatch`、错接收者 IAE**；StockTrans 分派 22949→1645）+ from_any ②；**T2 在途**。T3 对反射深水 pilot（#8/#10/#11）是直接利好。
- **scripts-into-rava S1–S5 ✅（bb0b7736）**：Python 入口并入 rava（main.py 已删）、lib_pilot_golden.sh 已走 `rava build --lib`、**m3 编译错误 0**。S6–S8 在列：S6（dyn 并入 rava）◀── **C4 收官**（dyn_compare 冻结）→ S7（run_tests 拆 scripts/e2e/ 预留形态接口）→ S8（Python 归零）。
- **JUnit 步骤 0 在 tasks.md 登记「可提前；m3 编译 0 ✅，运行期存根由 C1d-b b0 处理」**——初稿发现的 m1 E0433（拆 crate import 缺陷）：**2026-10-03 worktree 复跑核实 = m1 GOLDEN OK（fe197231）**，已被 S1–S5 / C1d-b 顺带修复，**F1 闸门清除**。
- **W0-2 复跑结果（2026-10-03，fe197231）：m1–m4 全 GOLDEN OK；m5 GOLDEN DIFF，双层根因**——① 库存档 m5 golden 失真：N13 改名（dbbcb057）把测试 `subject` 由 `"java-rta"` 机械替换为 `"rava"` 并手工改了 golden 文本但**未重跑**（subject="rava" 使 prefixOk 由过转败，fresh JVM = fail=2，存档仍 fail=1）；② **真缺陷：翻译侧两个 @Test 读 `subject` 均为 `was null` = `@Before setUp()` 的字段写入未生效**（@Before 未发现 or 反射写回丢失，二选一待定位；m3/m4 无 @Before 用例故未暴露）。**无状态跨 crate 回调（even/prefix/length 六检查）翻译侧全 PASS**——坏面精确限定在「回调带字段状态」。fresh 输出存证 /tmp/m5_{jvm,rs}_fresh.txt；worktree golden 已还原未提交。缺陷归闭包分析器域（T 系列队列，与 T2 并轨，W0-4 跟踪）。
- 全量 e2e 与重命令已走 **8 台服务器分发**（distribute_tests.py）；**T1 跨测试编译复用 ◇ Q3**（用户一次决策）= per-test 成本闸门的替代路径（与预构建缓存 C-CACHE 组合或二选一）。
- 时序主链：阶段 C（闭包分析器）收官 → C4 → S6 → S7 → H1（e2e 形态接口）。**golden mains 不在这条链上**，W0 复跑与 W1 反射件可提前。

### 4.1 依赖源记号（五类）

| 类 | 记号与现状（2026-10-03） |
|---|---|
| 管线能力 C | C-L3 反射分派 ✅（T3 增强）· C-Proxy 动态代理 ✅（Proxy$Dyn）· C-SPI 静态目录 ✅ · C-XML ✅（闭包内）· **C-SQL java.sql ❌ 需立项** · C-MT 真并发 ◐（real-multithreading 待拍板）· C-NET 网络 IO 边界 ❌ · C-SCAN 类路径扫描 ❌ · C-SYN 翻译期子类合成 ❌（新能力立项）· C-C1D 闭包收窄 ◐（b1+T3 已合，T2 在途；e2e 成本闸门）· C-CACHE 预构建 lib crate 缓存 ❌（A 终态；替代/组合路径 T1-Q3） |
| 修复闸门 F | F1 = m1 E0433（拆 crate import 缺陷）——**✅ 2026-10-03 复跑核实：m1 GOLDEN OK（fe197231），已被 S1–S5/C1d-b 顺带修复，清除** |
| harness H | H1 = e2e 形态接口（S6+S7 合入 + form.toml + 63_junit 模板）；gated on C4 收官；只 gate e2e 目录，**不 gate golden mains** |
| 前序验收 P | pilot green（golden mains 全绿；e2e 部分在 H1 前暂缓） |
| 拍板 D | §六 1–7，关键位标注在各任务卡 |

### 4.2 任务卡（波次内即实施序）

**W0 闸门（唯一可立即开工组，F1 无外部依赖）**

| 任务 | 依赖 | 验收判据 | 解锁 |
|---|---|---|---|
| W0-1 m1 复跑核实（E0433） | 最新 main（不等任何队列） | **✅ 2026-10-03 完成：GOLDEN OK（fe197231），缺陷已被 S1–S5/C1d-b 顺带修复** | W0-2 |
| W0-2 m2–m5 复跑 | W0-1 ✅ | **部分达成（2026-10-03）：m1–m4 ✅ 4×GOLDEN OK；m5 ❌ DIFF → W0-4** | m1–m4 面（断言/Runner/timeout/跨 crate 编译链）已解锁 **W1 五连、#8 OGNL、#11 spring-core 切片、W5 池纯计算件**；m5 面（跨 crate 回调带字段状态）推迟到 W0-4；步骤 A 另等 H1 |
| W0-3 e2e 形态接线（= junit 计划步骤 A：form.toml + 63_junit） | **H1**（◀ C4 收官）+ D-2 | 63_junit 首批 ≥10 例绿 + m1–m5 零回归 | 一切 e2e 目录放量（64_… 编号段） |
| **W0-4 m5 缺陷修复**（@Before 字段写丢失 / 库存档失真重生成） | 归属闭包分析器域（与 T2 并轨，排期由用户裁决） | 定位修复后 m5 复跑 GOLDEN OK + **m5 golden 库存档重生成**（替换 dbbcb057 的失真存档） | 「用户类被 lib 回调 + 字段状态」面加固：#6 servlet 容器件的状态用例最受益；无状态回调面已验证可用（m5 六检查 PASS） |

**W1 纯计算五连（依赖 W0-2；一次一个 pilot）**

| 序 | 任务 | 依赖 | 验收（mains/e2e） | 解锁 |
|---|---|---|---|---|
| #1 | slf4j-api | W0-2 | 3 / 5 | Logback 软前置；SPI 静态目录 lib 场景实证 |
| #2 | joda-time | W0-2 | 3 / 8 | — |
| #3 | jackson-core | W0-2 | 4 / 10 | — |
| #4 | commons-lang3 | W0-2 | 5 / 12 | P1 兑现 |
| #5 | commons-io | W0-2 + **IO 边界现状复核**（半天前置） | 3 / 8 | PDFBox/POI 的 IO 面参考 |

**W2 spec-API 双件（依赖 W0-2 + D-2；容器件模式 = m5 复用）**

| 序 | 任务 | 依赖 | 验收 | 解锁 |
|---|---|---|---|---|
| #6 | jakarta.servlet-api | W0-2 + D-2 | 4 / 10（async 用例 gated C-MT） | **struts2 前置一** |
| #7 | jakarta.persistence-api | W0-2 | 2 / 4 | hibernate API 面前置 |

**W3 反射深水 + JDBC 深水五件（依赖 W0-2；#9/#10 另需 C-SQL）**

| 序 | 任务 | 依赖 | 验收 | 解锁 |
|---|---|---|---|---|
| #8 | OGNL | W0-2（无别的，可插队 W1 后任意位） | 3 / 8 | **struts2 前置二**；表达式引擎族基座 |
| **#9** | **H2（D-4 拍板件）** | W0-2 + **C-SQL 落地**（jmods 翻译）+ 种子裁剪（server/web console 网络面、FunctionAlias 反射面排除） | 4 / 8（CRUD 往返 / 函数聚合 / DatabaseMetaData / 事务回滚） | **#10 mybatis 的驱动基座**；dbcp2/dbutils/Quartz/HikariCP 的真 JDBC 面 |
| #10 | mybatis | W0-2 + C-SQL + **P：#9 H2 green** | 5 / 10 | **Retrofit 最稳位**；**hibernate 前置一** |
| #11 | spring-core 切片 | W0-2 + D-3 | 4 / 10 | spring-beans；泛型元数据实证反哺 hibernate 元模型 |
| #12 | spring-beans 切片 | **P：#11 green** | 2 / 6 | **spring-context 前置一** |

**W4 大闭包（依赖 C-C1D 收窄到位 = T2 收官后量成本）**

| 序 | 任务 | 依赖 | 验收 | 解锁 |
|---|---|---|---|---|
| #13 | guava 切片 | C-C1D + 闭包规模实测（预估 2000+ 类） | 4 / 12 | eclipse-collections 同模式 |

**W5 候选池滚动（依赖 W0-2；序 = 建议提拔序）**

| 序 | 件 | 附加依赖 | 备注 |
|---|---|---|---|
| 1 | Jsoup | — | 纯计算顶级候选 |
| 2 | Logback | P：#1 green（软） | 与 slf4j 闭环 |
| 3 | Caffeine | — | 并发原语面（协作调度器下先行，C-MT 后补强） |
| 4 | SnakeYAML / 5 typesafe config | — | 小而纯 |
| 6 | Retrofit | P：#10 green（互证位） | 测试替身 Call（用户域 Java 实现）全离线 |
| 7 | Fastjson2 流式 / 8 AssertJ | — | 反射面 gated；AssertJ 在盘 |
| 插队 | 在盘顺手件（codec/math3/jexl3/beanutils/collections4/jakarta.mail…） | jexl3 建议 #8 后（同族互证） | 零取包成本 |

**终点域解锁链（判据已在 §三终点表）**：spring-context ← C-SCAN + P(#11+#12) → SpringMVC →（+C-SYN）Boot AOP 半；hibernate ← C-SQL + P(#10) + C-SYN + ANTLR 规模实测；HikariCP ← C-SQL（驱动 = #9 H2）+ C-MT；Disruptor/RxJava ← C-MT；OkHttp 网络面/Redis/httpclient ← C-NET；BouncyCastle ← JCA ✅ + 规模实测；ASM 特批元目标；Dubbo 系 ← C-NET + C-SCAN + 动态 SPI。

### 4.3 条件达成 → 立即可做（速查表）

| 条件达成 | 立即可做 |
|---|---|
| F1 核实通过（m1 绿） | W0-2（m2–m5 复跑） |
| W0-2（m1–m4 绿，已达成） | W1 五连、#8 OGNL、#11 spring-core 切片、W5 池全部纯计算件（Jsoup/Caffeine/SnakeYAML/typesafe/AssertJ/Fastjson2 流式/在盘顺手件） |
| W0-4（m5 修复） | 「跨 crate 回调 + 字段状态」面（servlet 容器件状态用例、@Before 式生命周期断言） |
| H1 就绪（C4 → S6 → S7 合入） | W0-3 接线 → 各 pilot e2e 目录逐波放量 |
| D-2 拍板（servlet 代次） | #6、#7 |
| C-SQL 落地 | #9 H2 → #10 mybatis →（+C-MT）HikariCP、Quartz |
| C-MT 落地 | servlet async 用例、Disruptor、RxJava 调度器面、Caffeine 补强 |
| C-NET 决策 | OkHttp 网络面、Redis 客户端、httpclient 转正、jakarta.mail 发送面 |
| C-SCAN 建模 + P(#11+#12) | spring-context 全量 → SpringMVC |
| C-SYN 落地 + P(#10) + C-SQL | hibernate-core |
| C-C1D 到位（T2 收官） | W4 guava、大闭包 e2e 放量成本复核（与 C-CACHE/T1-Q3 组合决策） |
| P(#6 + #8) green | struts2-core 切片 |

### 4.4 资源与纪律

- **一次一个 pilot**；golden mains 一 pilot 一 workspace 不受 per-test 成本约束；e2e 逐例成本闸门 = C-C1D + C-CACHE/T1-Q3。
- golden 先行、e2e 后置到 H1；全量 e2e 归 8 台服务器分发，本地定向 ≤10 例纪律不变。
- 同层 pilot 零相互依赖、独立止损；组装件（struts2/Boot/hibernate）显式列前置，不与组成件抢位。
- 每 pilot 五步流程模板：dep_scan 透视（jdk-internal 准入红线）→ 种子表定稿 → golden mains 对账 → form.toml + e2e 目录（H1 后）→ tasks.md 登记。

**规模合计（主推 #1–#13，含 D-4 拍板新增 #9 H2）**：lib crate **13 个** / mains **≈46** / e2e **≈111**（junit 既有 10–20 另计）；候选池全量 ~19 件 / ~39 mains / ~89 e2e 按波次滚动提拔。

## 五、licensing 一句话

Apache-2.0 覆盖 servlet / mybatis / spring / guava / commons / jackson / eclipse-collections / joda；MIT 仅 slf4j。衍生作品内部 pilot 无碍，publish 前统一法务口径（junit 计划 §五同款）。

## 六、待拍板

1. **总规模**：A 全矩阵（12 crate / ~42 mains / ~103 e2e）vs **B 首波五连（5 crate / ~18 mains / ~43 e2e，推荐）**后按波次滚动拍板。
2. **servlet 代次**：jakarta 6.x（推荐，与终态 Spring 6/Boot 3 对齐）vs javax 4.0.1（Spring 5 代）。
3. **Spring 基线**：6.x（推荐，jakarta 命名空间 + AOT 友好）vs 5.3（javax，反射面更老但社区存量最大）。
4. ~~MyBatis JDBC 层~~ → **已拍板（2026-10-03）：H2 真驱动 jar 翻译**——java.sql 接口面从 jmods 翻译（唯一路径，手写边界原则 §1），驱动侧 = H2 2.x 内存模式（`jdbc:h2:mem:` 零网络）独立成 pilot **#9**（真 SQL 引擎 + JDBC SPI 全套真实实现，净赚一个真实大库压测面），mybatis 后移 #10、对着 H2 跑真 SQL；Java fixture 小驱动路线弃。
5. **Boot 终点定位**：确认按 §三终点表以入场判据制挂远期（不降低终态目标，只排定可达顺序）。
6. **SSH 增量纳入方式**（2026-10-03 追问）：推荐 OGNL（#8）+ JPA api（#7）进主推、struts2-core 切片与 hibernate-core 挂入场判据（§三-A）——如你要求 Struts2/Hibernate 更激进排期，需连带拍板「翻译期子类合成」新能力立项。
7. **候选池提拔（W5+）**（2026-10-03 二次追问）：高优先候选八件（Jsoup/Logback/Caffeine/SnakeYAML/typesafe config/Retrofit/Fastjson2 流式/AssertJ）建议按此序滚动提拔，或在盘顺手件（零取包成本）插队；gated 维持件按 §三-B 各自判据，不随热度提前。
