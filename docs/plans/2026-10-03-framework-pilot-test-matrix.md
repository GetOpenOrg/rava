# 框架依赖包测试矩阵：Servlet / MyBatis / Spring 与热门库 pilot 阶梯

> 日期：2026-10-03
> 来源：用户命题「像 JUnit 测试一样，创建 Servlet、MyBatis、Spring、Spring Boot 相关测试，每一个都是独立的，再加一些热门框架依赖包——分析要做哪些测试、建多少」；同日追问「SSH 三件套是否覆盖」→ §三-A 增补核查（Struts/Hibernate 原未覆盖，已补增量行）。
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

## 二、能力现状核查（2026-10-03 实测，矩阵的推导起点）

| 能力 | 状态 | 证据 | 影响 |
|---|---|---|---|
| 动态代理 | ✅ **已落地** | `Proxy.newProxyInstance` → VM 支持类 `Proxy$Dyn` 承载全部代理实例（FS-R R4a，runtime/java_runtime/.../proxy_impl.rs），`isProxyClass`/`getInvocationHandler` 按字节码翻译 | **MyBatis 最大疑点被拆除**：MapperProxy 接口代理不需要运行期类定义；剩分派经 L3 Method 元数据的精度（junit m3 已有 L3） |
| 反射 L1/L2/L3 | ✅ | Method/Field/Constructor 元数据表 + invoke 分派（m3 验收）；跨 crate 扩表已做 | spring 切片 / MyBatis / picocli 的地基 |
| ServiceLoader 静态目录 | ✅ | 2026-09-26 计划落地 | slf4j LoggerFactory 绑定、JDBC DriverManager SPI 同族 |
| XML 栈（XPath/XSL/DOM） | ✅ 在闭包 | m1 发射集含 com/sun/org/apache/xpath 树（m1 的失败是拆 crate import 缺陷，非闭包缺口） | MyBatis 配置/映射解析面 |
| **java.sql** | ❌ runtime 无此包 | `find runtime -path "*java/sql*"` 零命中 | **MyBatis 硬前置**：JDBC 层需立项（翻译 jmods java.sql 接口面 + 手写 fake Driver/Connection/ResultSet，或手写 java.sql 子集） |
| 类路径扫描 / ClassLoader 资源枚举 | ❌ | 静态翻译模型边界 | spring-context 全量、Boot 自动装配的 gate |
| 运行期字节码生成 | ❌ 本体排除 | byte-buddy / CGLIB / Mockito 在盘 jar 属此类 | Boot 的 AOP 代理、Mockito 不入矩阵 |
| 真并发 / 抢占 | ◐ | 2026-09-26 real-multithreading 计划在列 | servlet async、guava ListenableFuture 的 gate |

**jar 资产盘点（52 个已在盘，`tests/lib_pilot/deps/target/pilot-libs/`）**：commons 全家（lang3/text/csv/io/codec/collections4/math3/beanutils/…）、guava 33.7.1、jackson 三件套（core/annotations/databind）、gson、httpclient/httpcore（4/5 两代）、joda-time、picocli、slf4j-api、eclipse-collections、assertj、byte-buddy（排除项，仅透视用）、junit/hamcrest。**不在盘**：servlet-api、mybatis、spring 全家、**OGNL、struts2、hibernate/jakarta.persistence-api（SSH 增补件，2026-10-03 核查零命中）**——pom 增行即可（fetch 脚本既有机制）。

## 三、分梯队矩阵（核心表）

> 类数 = jar 全量量级（闭包按种子裁剪）；mains = crate 验收 golden 用例；e2e = 形态目录回归用例。**每梯队内排序即建议实施序。**

### 梯队 1：纯计算 / 纯 API 热身（jar 在盘，W0 闸门后即可开工）

| # | pilot | 类数量级 | 压测面（为什么值得做） | mains | e2e | 前置 |
|---|---|---:|---|---:|---:|---|
| 1 | slf4j-api 1.7.36 | ~80 | SPI 绑定（静态目录消费）、MDC（ThreadLocal）、`{}` 消息格式化纯计算 | 3 | 5 | 无 |
| 2 | joda-time 2.14.4 | ~300 | 不可变对象族、历法纯计算、时区（tzdb 已内嵌的同源验证） | 3 | 8 | 无 |
| 3 | jackson-core 2.22.3 | ~280 | 流式 JsonParser/Generator 往返、数字/编码边角、token 状态机 | 4 | 10 | 无 |
| 4 | commons-lang3 3.20.0 | ~2900→种子子集 | **P1 既定**（roadmap）：StringUtils / 数值 / mutable / 时间格式化 | 5 | 12 | 无 |
| 5 | commons-io 2.22.0 | ~500 | IO 边界手写层的真实压力面（copy / charset / filename / Input/OutputStream 族） | 3 | 8 | IO 边界现状复核 |

备选（在盘不占首波）：commons-text（2/6）、commons-csv（2/5）、picocli 4.7.7（3/6，注解反射 L1–L3 + 类型转换）、gson 流式面（2/4，排除反射 TypeAdapter 族）。

### 梯队 2：spec API + 手写 harness（用户侧实现容器件 = m5 跨 crate 回调模式复用）

| # | pilot | 类数量级 | 压测面 | mains | e2e | 前置 |
|---|---|---:|---|---:|---:|---|
| 6 | jakarta.servlet-api 6.0（pom 增） | ~450 | spec 接口/注解族（WebFilter/WebServlet 元数据）、ServletRequest/Response 会话模型、Filter 链分派——**容器件由测试 main 自己实现接口**（同 JunitCrossCrateMain 的 BaseMatcher 子类模式），golden = 分派与数据往返 | 4 | 10 | 代次拍板（§六-2）；async 用例 gated on real-multithreading |
| 7 | jakarta.persistence-api 3.x（pom 增） | ~150 | JPA spec jar（@Entity/@Table 注解族、EntityManager/Criteria/Metamodel 接口面）——与 servlet-api 同模式的 spec-API pilot，**Hibernate 的 API 面先行** | 2 | 4 | 无 |

### 梯队 3：反射深水（Proxy$Dyn / L3 / XML 的合流压力面）

| # | pilot | 类数量级 | 压测面 | mains | e2e | 前置 |
|---|---|---:|---|---:|---:|---|
| 8 | OGNL 3.x（pom 增） | ~350 | **Struts2 的灵魂**：表达式引擎——反射属性导航 / 方法调用 / 索引与投影 / 类型转换 / 静态成员访问，SpEL 同族能力但独立 jar 可单测（`Ognl.parseExpression` + `getValue` 对 POJO 求值即可 golden） | 3 | 8 | 无（L3 已有）；与 #9 互证反射深水面 |
| 9 | mybatis 3.5.x（pom 增） | ~1300 | **Proxy$Dyn 分派保真**（MapperProxy 经 Method 元数据路由）、POJO getter/setter 反射、XML 配置/映射解析（XPath 栈）、动态 SQL（if/foreach/where）、TypeHandler 族、L1 缓存 | 5 | 10 | **java.sql 层立项拍板**（§六-4）；fake Driver 手写 |
| 10 | spring-core 切片 6.x（pom 增） | 种子子集 | ResolvableType / MethodParameter（**泛型元数据反射 = hamcrest TypeSafeMatcher 同款能力的放大**）、AntPathMatcher、PropertyPlaceholderHelper、StreamUtils、MultiValueMap | 4 | 10 | L3 稳定（m3 已验）；版本基线拍板（§六-3） |
| 11 | spring-beans 切片（后置） | 种子子集 | BeanUtils / 属性编辑器族（无 cglib 无容器的部分） | 2 | 6 | 10 号验收 |

### 梯队 4：大闭包集合库

| # | pilot | 类数量级 | 压测面 | mains | e2e | 前置 |
|---|---|---:|---|---:|---:|---|
| 12 | guava 33.7.1 collect/base 切片（在盘） | 子集 | Immutable 集合族、Table/BiMap/ Multimap、Chars/Strings、hash——**P3 既定** | 4 | 12 | 闭包规模实测（种子裁剪后预估 2000+ 类） |
| — | eclipse-collections（在盘，备选） | 大 | 路线图远期名单项，压轴备选 | — | — | 按需 |
| — | httpclient/httpcore（在盘，备选） | 大 | 网络 IO 边界 | — | — | **网络边界决策 gated** |
| — | struts2-core 切片（pom 增，备选） | ~1600 | XWork 拦截器链、ValueStack、类型转换族——「不带完整 web 栈」的 MVC 内核 | 3 | 6 | **gated**：#6 servlet + #8 OGNL 验收后（Struts2 = servlet + OGNL + 拦截器的组装件，三者单独可达才轮到组装） |

### 终点：入场判据制（0 tests now，不是不做）

| 目标 | 入场判据 |
|---|---|
| spring-context IoC 全量 | 类路径扫描（ClassLoader 资源枚举）建模 + L3 全量稳定 + spring-beans 切片验收 |
| **Spring Boot** | IoC 全量 + servlet 容器件手写成熟 + **运行期类生成替代协议决策**（launcher 嵌套 jar 类加载、自动装配、内嵌容器、AOP 代理）。与 Maven/Gradle/Mockito/Netty/byte-buddy 同列静态模型边界（roadmap「明确不早期碰」的正式化——北极星不是里程碑） |
| **hibernate-core 6.x** | JDBC 层落地 + #9 MyBatis ORM 面验收 + **翻译期子类合成立项**（实体懒加载代理：闭包内实体类静态已知，代理子类可由生成器翻译期合成——byte-buddy 运行期生成的静态替代，与 Mapper 代理合成同族能力）+ HQL/ANTLR 生成代码规模实测（~万级类闭包） |

### §三-A SSH 三件套覆盖核查（2026-10-03 增补）

| SSH 组件 | 原矩阵覆盖 | 增补处置 |
|---|---|---|
| Spring | 部分：core/beans 切片（W3）+ context/Boot 终点判据 | 无新增 |
| Struts2 | **未覆盖** | 拆解为「OGNL + servlet + 拦截器」三件：**OGNL 独立成 pilot（#8，灵魂件，无前置）**；struts2-core 切片挂备选（gated = #6 + #8 验收后）——组装件不做独立攻坚，组成件单独可达后再组装 |
| Hibernate | **未覆盖** | jakarta.persistence-api 先行（#7，spec jar 与 servlet 同模式）；hibernate-core 挂终点判据（byte-buddy 懒加载代理 = 运行期类生成，静态替代路径 = 翻译期合成子类，属新能力立项；HQL/ANTLR 规模 ~万级）。**MyBatis 先行（#9）是正确的第一 ORM**——接口代理已被 Proxy$Dyn 解决，Hibernate 的类代理难题在 MyBatis 验收后再攻 |

> 若「SSH」实指 **SSM**（Spring + SpringMVC + MyBatis）：MyBatis 已在 #9；SpringMVC（spring-web/webmvc）随 spring-context 同判据（请求映射反射 + 容器 + servlet 容器件）。

## 四、规模与排期

**合计近期（主推 #1–#12，含 SSH 增补 #7 JPA api + #8 OGNL）**：lib crate **12 个** / crate 验收 mains **≈42** / e2e 回归 **≈103**（junit 既有计划 10–20 另计；原 10 pilot 口径精确值 37/91，本文以逐行和为准）。备选池（struts2 切片 +~9 e2e、commons 家族 +~11 e2e）按需滚动。

| 波次 | 内容 | 规模 | 闸门 |
|---|---|---|---|
| **W0（在途）** | m1 E0433 修复（拆 crate import 生成缺陷）→ m1–m5 Rust 路径复跑绿 → e2e 接线（S6/S7，63_junit 模板定型） | 闸门 | **一切 lib 模式 pilot 的入场券** |
| W1 | #1 slf4j → #2 joda → #3 jackson-core → #4 lang3 → #5 commons-io（一次一个，资源纪律） | 18 mains / 43 e2e | W0 绿 |
| W2 | #6 servlet + #7 JPA api（同模式 spec-API 双件） | 6 / 14 | W0 绿；§六-2 |
| W3 | #8 OGNL → #9 mybatis（JDBC 层立项先行）→ #10 spring-core 切片 → #11 spring-beans | 14 / 34 | §六-3/4 拍板 |
| W4 | #12 guava 切片；备选（struts2 切片等）按需 | 4 / 12 | 闭包规模实测达标 |

**每 pilot 五步流程模板**：① `dep_scan.py` 透视（jdk-internal 扫描 = 准入红线；反射/线程/IO 面盘点）→ ② 种子表定稿（form.toml 数据）→ ③ golden mains 对账（lib_pilot 模式，golden 入库）→ ④ form.toml + e2e 目录（S6/S7 后）→ ⑤ tasks.md 登记。

**资源纪律**：一次一个 pilot；golden mains 不受 per-test 成本约束（一 pilot 一 workspace）；e2e 放量等 C1d 收窄（大闭包 per-test 成本）与预构建缓存（A 终态）联动裁决；全量 e2e 归用户服务器。

## 五、licensing 一句话

Apache-2.0 覆盖 servlet / mybatis / spring / guava / commons / jackson / eclipse-collections / joda；MIT 仅 slf4j。衍生作品内部 pilot 无碍，publish 前统一法务口径（junit 计划 §五同款）。

## 六、待拍板

1. **总规模**：A 全矩阵（12 crate / ~42 mains / ~103 e2e）vs **B 首波五连（5 crate / ~18 mains / ~43 e2e，推荐）**后按波次滚动拍板。
2. **servlet 代次**：jakarta 6.x（推荐，与终态 Spring 6/Boot 3 对齐）vs javax 4.0.1（Spring 5 代）。
3. **Spring 基线**：6.x（推荐，jakarta 命名空间 + AOT 友好）vs 5.3（javax，反射面更老但社区存量最大）。
4. **MyBatis JDBC 层**：java.sql 接口面从 jmods 翻译 + 手写 fake Driver/Connection/ResultSet（推荐，接口面大但语义薄）vs 手写 java.sql 最小子集（快但边界口径要自定）。
5. **Boot 终点定位**：确认按 §三终点表以入场判据制挂远期（不降低终态目标，只排定可达顺序）。
6. **SSH 增量纳入方式**（2026-10-03 追问）：推荐 OGNL（#8）+ JPA api（#7）进主推、struts2-core 切片与 hibernate-core 挂入场判据（§三-A）——如你要求 Struts2/Hibernate 更激进排期，需连带拍板「翻译期子类合成」新能力立项。
