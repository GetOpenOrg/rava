# e2e 基础能力补齐与依赖取包落位（框架矩阵配套件）

> 日期：2026-10-03
> 来源：项目决策三条——①目标 JDK = **21 与 25** 双跑；②依赖版本「有些可不最新、有些用最新」配进 pom 并拉取；③补零第三方依赖的 e2e 能力覆盖测试 + 列出当前可做的全部工作。
> 关联：[框架矩阵](2026-10-03-framework-pilot-test-matrix.md) §二 K 表 / §四 排期。

## 一、依赖版本决策与取包结果（pom 已改，jar 已拉）

版本策略两线（唯一事实源 = `tests/lib_pilot/deps/pom.xml` 注释）：**追新**（独立库取稳定线末版）/ **钉代不追顶**（框架取代际主流，不追最新大版本）。

| 依赖 | 版本 | 线 | 字节码 | 备注 |
|---|---|---|---|---|
| jakarta.servlet-api | 6.1.0 | 钉 | 11 | 6.2 尚在 M 线 |
| jakarta.persistence-api | 3.2.0 | 钉 | 11 | 配 Hibernate 7 代 |
| hibernate-core（org.hibernate.orm） | 7.4.6.Final | 钉 | 17 | 2026 主流代；终点域 intel 件 |
| struts2-core | 7.4.0 | 钉 | 17 | jakarta 6 代对齐，备选件（传递带 fileupload2-M5 里程碑件，仅 intel 用） |
| spring-core / spring-beans | 6.2.19 | 钉 | 17 | 不上 7.x 新代（主流存量代） |
| Retrofit | 2.12.0 | 钉 | 8 | 2.x 末版不上 3.x；传递 okhttp 3.14.9——OkHttp 独立 pilot 时显式加 4.12 |
| mybatis | 3.5.19 | 新 | 8 | |
| OGNL | 3.4.14 | 新 | 8 | 3.5 是 BETA 不取 |
| **H2（D-4 拍板件）** | 2.5.252 | 新 | **21** | 恰在 21 上限，21/25 双跑兼容 |
| slf4j-api | 2.0.20 | 新 | 8 | 显式钉 2.0（logback 1.5 配对线，压过旧传递 1.7.36） |
| Logback | 1.5.38 | 钉线 | 11 | 1.6 已出，保守取 1.5 主流配对线末版 |
| Jsoup | 1.23.2 | 新 | 8 | |
| Caffeine | 3.3.0 | 新 | 11 | |
| SnakeYAML | 2.7 | 新 | 9 | |
| typesafe config | 1.4.9 | 新 | 8 | |
| Fastjson2 | 2.0.65 | 新 | 8 | 非 android 变体 |
| TestNG | 7.12.0 | 新 | 11 | 候选池 intel 件 |

取包结果：**52 → 92 jar**（含传递件；junit/hamcrest/commons 家族维持原版本）。抽查字节码全部 ≤ major 65（Java 21）——**双 JDK 目标兼容**；golden 期望先按 21 生成（`.jdk-version` 钉 21），JDK25 适配轮（tasks.md 在列）后 25 侧复跑。

## 二、e2e 基础能力补齐（零第三方依赖，tests/e2e 单文件自含）

原则：pilot 抓集成面、e2e 抓底层语义回归——m5 的 @Before 缺陷若有零依赖 e2e 先行网，9 月即可拦截。每例：public 类名=文件名、确定性输出、`--update-expected` 生成期望、进 run_tests 失败棘轮。

**已覆盖（核查过，不重复建）**：动态代理（TestDynamicProxy）、charset 编解码（TestCharsetForName / TestStreamEncoderCharsets）、注解族含 @Repeatable（47_annotations）、并发与线程（34/60 目录）、构造器反射（TestCtorReflect）、反射按接收者分派（TestReflectInvokePerReceiver）。

**新增 3 例（java.base 反射族，零门槛立即写）**：

| 测试 | 目录 | 覆盖语义 | 护住谁 |
|---|---|---|---|
| TestReflectStateWriteBack | 62_reflection | 反射 invoke 调实例方法写字段 → 直接读字段验证（@Before 同构、零依赖版） | **m5 缺陷（W0-4）的直接回归网** |
| TestGenericSuperclassReflect | 62_reflection | getGenericSuperclass / getActualTypeArguments / getGenericInterfaces（参数化超类） | hamcrest TypeSafeMatcher 族、spring ResolvableType（#11） |
| TestInvokeNullArgs | 62_reflection | invoke 的 null 参数数组 ≡ 空数组、InvocationTargetException 解包语义 | junit m3 修过的⑤⑥语义固化 |

**原计划的 XML 三例（DOM/XPath、SAX、StAX）转隶 [jmod 覆盖计划](../../java_rta_closure_wt/docs/plans/2026-10-03-jmod-coverage.md) 的 `71_xml` 用例集**（TestXmlDomParse / TestXmlSaxEvents / TestXmlStax / TestXmlXPath / TestXmlTransform）——模块级覆盖归 jmod 计划为唯一事实源，本计划不重复建；其闭包（~3000 类，com.sun.org.apache.xerces 全栈）恰是 mybatis XML 面（#10）更真实的压力面。写作可随本计划先行（第 0 步预审模式：源码+期望先行、不进基线），进基线等 jmod 第 2 步。

**明示不由 e2e 承担**：ServiceLoader SPI 多 provider 面（单文件无法携带 META-INF/services 资源——由 #1 slf4j pilot 承担；**空 provider 路径已由 jmod 计划 73_jndi_script 三例补上**）；java.sql（由 jmod 计划 `67_sql` 承担，见 §一-B 联动）。

### §二-B 与 jmod 覆盖计划的合流（2026-10-03 对齐）

[jmod 覆盖计划](2026-10-03-jmod-coverage.md)（java.base 之外的 JDK 模块 e2e，64–74 目录）与本计划/pilot 矩阵的五处联动：

| jmod 目录 | 服务的能力/pilot | 联动关系 |
|---|---|---|
| `67_sql`（java.sql + rowset，A 档） | **K7（C-SQL）** | java.sql 模块 e2e 全绿 = jmods 翻译 java.sql 的实证 → #9 H2 / #10 mybatis 的排期信号 |
| `71_xml`（java.xml，B 档） | K4 + #10 mybatis XML 面 | 本计划 XML 三例并入其用例集（上文）；~3000 类闭包同时检验 C-C1D |
| `72_http`（java.net.http 回环，B 档） | **K9（网络边界）** | 回环 + 同进程用例 = 网络语义的最小实证，K9 口径决策的实测输入；Retrofit/OkHttp 网络面 pilot 的地基 |
| `66_logging`（java.logging，A 档） | #1 slf4j / W5 Logback | Handler/Formatter/Level 的 JDK 侧地基（注意 jmod 计划 §三.3：stdout-only 约束） |
| `74_beans_geom`（java.beans，第 4 步） | #10 mybatis POJO 反射 / #12 spring-beans 属性编辑器 | Introspector / PropertyChangeSupport = getter/setter 反射标准件 |

**目录编号分配（冲突消解，2026-10-03 议定）**：`63_junit` = JUnit 形态（junit 计划）；`64–74` = jmod 覆盖计划占用；**框架 pilot 的 e2e 形态目录从 `75_` 起**（75_slf4j、76_joda、77_jackson_core、78_lang3、79_io、80_servlet、81_jpa、82_ognl、83_h2、84_mybatis、85_spring_core、86_spring_beans、87_guava——等 H1 接线后逐波建）。

## 三、现在可以做的（全量清单，按可开工排序）

**零门槛（m1–m4 面已解锁，jar 已到位）**：
1. e2e 能力补齐 6 例（§二）——现在就写，先护住 m5 同族语义；
2. W1 五连：#1 slf4j 2.0.20（已拉）→ #2 joda（在盘）→ #3 jackson-core（在盘）→ #4 commons-lang3（在盘）→ #5 commons-io（在盘 + 半天 IO 边界复核前置）；
3. #8 OGNL 3.4.14（已拉）——可插队 W1 后任意位；
4. W5 池纯计算件：Jsoup / Caffeine / SnakeYAML / typesafe config / AssertJ（在盘）/ Fastjson2 流式（均已拉）；Logback 排 #1 green 之后。

**一句话拍板即可开**：
5. #11 spring-core 切片（D-3 确认 6.2 代 → jar 已拉）；
6. #6 servlet 6.1.0 + #7 JPA 3.2.0（D-2 确认 jakarta 6.1 → jar 已拉；servlet 的有状态容器件用例建议等 W0-4 修复，无状态面可先行）。

**等闸门 / 立项**：#9 H2 → #10 mybatis（等 C-SQL 立项排期）；W0-3 e2e 接线（等 S6/S7 ◀ C4 收官）；#13 guava（等 C-C1D / T2 收官）；K8–K11 判据件（真并发 / 网络 / 类路径扫描 / 子类合成）各自等决策。
