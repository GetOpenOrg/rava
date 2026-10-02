# e2e 覆盖扩展到 java.base 之外的 JDK 模块

日期：2026-10-03
状态：计划（正式铺开排在 C4 收官之后；第 0 步预审可提前）
相关：`2026-09-20-e2e-coverage-plan.md`（e2e 用例硬约束）、`2026-10-01-junit-crate-as-test-harness.md`（`63_junit` 目录已预留）、`docs/reference/handwritten-boundary.md`

---

## 一、现状

### 1.1 转译器侧：已面向全部 jmod

- `generator/crates/resolve/src/classpath.rs::add_jdk` 把 `<java_home>/jmods/*.jmod` **全部**加入类路径（优先级 `java.base` → `java.desktop` → `java.logging` → `java.xml` → `java.sql` → `java.net.http`，其余按名），再叠加镜像改写类（`image::rewritten_dirs`）。
- 模块描述符、`provides` / `uses` 服务目录（`seeds.toml [services]`：JVM 默认引导层 = API 根集 + requires 闭包 + uses 绑定）、平台 / 应用类加载器（FS-C2 ✅）都按全部模块处理。
- 结论：**限制在 java.base 的是测试语料，不是转译器。** 扩展不需要新的加载机制，需要的是用例与随之暴露的缺口修复。

### 1.2 语料侧：1110 例几乎全在 java.base

- `tests/e2e/**` 的 import 全部落在 java.base（`java.util` 1044、`java.util.stream` 120、`java.io` 100 …）。
- 例外：
  - `TestAppClassLoader` 引用 `java.sql.Time`，但只取它的类加载器。
  - 使用 `Locale.GERMANY / JAPAN / FRANCE / ITALY` 格式化的若干例，会间接经过 `jdk.localedata`（CLDR 非英语数据），但没有专门覆盖。

### 1.3 候选模块的翻译难度（JDK 21 实测）

判据：模块内 `ACC_NATIVE` 方法数。为 0 即整模块可按字节码翻译，不新增手写（符合手写边界 §1）。

| 模块 | 类数 | native 方法数 |
|---|---|---|
| java.base（参照） | 7516 | 724 |
| jdk.charsets | 335 | 0 |
| jdk.localedata | 1876 | 0 |
| java.logging | 65 | 0 |
| java.sql | 78 | 0 |
| jdk.random | 10 | 0 |
| jdk.zipfs | 35 | 0 |
| jdk.crypto.ec | 91 | 0 |
| java.xml | 2219 | 0 |
| java.net.http | 349 | 0 |
| jdk.httpserver | 92 | 0 |
| java.naming | 246 | 0 |
| java.scripting | 17 | 0 |
| java.sql.rowset | 58 | 0 |
| java.prefs | 46 | 17 |
| java.management | 475 | 66 |
| jdk.jfr | 457 | 65 |
| java.desktop | 5826 | 570 |

---

## 二、目标（终态）

1. **A 档 7 个模块全部有专项 e2e，且全部通过**：jdk.charsets、jdk.localedata、java.logging、java.sql、jdk.random、jdk.zipfs、jdk.crypto.ec。
2. **B 档 java.xml、java.net.http + jdk.httpserver 全部有专项 e2e，且全部通过**；java.naming / java.scripting / java.sql.rowset 各至少一例空提供者路径。
3. 新增用例**不新增手写方法**（A、B 档 native 数为 0）；暴露的缺口一律修生成器 / 闭包分析器 / 清单，不得以手写近似实现通过。
4. 新增用例纳入全量 e2e 基线，**回归数 0**；闭包规模逐例登记，异常扇出登记为精度项。
5. C 档不纳入本计划终态，理由见 §五。

---

## 三、用例规范

沿用 `2026-09-20-e2e-coverage-plan.md` §1 的硬约束，补充模块相关的几条：

1. **单文件自洽，只比对 stdout，输出完全确定。** 固定随机种子，不打印时间、identityHashCode、临时路径、端口号。
2. **期望输出由 JDK 21 实跑生成**（`run_tests.py --update-expected --filter <Name>`），不手写。
3. **日志用例不得依赖 stderr。** `ConsoleHandler` 默认写 System.err，不参与比对；用例改用 `StreamHandler(System.out, formatter)` 或自定义 Handler，并固定 Formatter 输出格式（不含时间戳）。
4. **需要文件的用例**（zipfs 等）在 `Files.createTempDirectory` 下运行期自建，打印相对路径，结束时清理。
5. **网络用例只用本机回环**：`HttpServer.create(new InetSocketAddress(InetAddress.getLoopbackAddress(), 0), 0)` 取临时端口，不打印端口号；服务端与客户端在同一进程。
6. **每个模块按逻辑分支拆成多个用例**，一例覆盖不全就拆。类名全局唯一。
7. **目录编号**：`63_junit` 已由 JUnit 计划预留，本计划从 `64_` 起：

| 目录 | 模块 |
|---|---|
| `64_charsets_ext` | jdk.charsets |
| `65_locale_data` | jdk.localedata |
| `66_logging` | java.logging |
| `67_sql` | java.sql（含 rowset） |
| `68_random_gen` | jdk.random |
| `69_zipfs` | jdk.zipfs |
| `70_crypto_ec` | jdk.crypto.ec |
| `71_xml` | java.xml |
| `72_http` | java.net.http + jdk.httpserver |
| `73_jndi_script` | java.naming、java.scripting |
| `74_beans_geom` | java.desktop 纯 Java 子集（第 4 步） |

---

## 四、用例清单与考验点

### 4.1 A 档（零 native、闭包可控、优先）

| 模块 | 用例（暂定名） | 覆盖分支 | 主要考验 |
|---|---|---|---|
| jdk.charsets | `TestCharsetGbk`、`TestCharsetCjkFamily`、`TestCharsetAvailable` | GBK / GB18030 / Big5 / Shift_JIS / EUC-JP / EUC-KR 编解码往返；不可映射字符的 REPORT / REPLACE / IGNORE；`availableCharsets()` 中扩展字符集是否存在（只打印存在性，不打印全集）；别名查找 | 扩展字符集提供者（`ExtendedProviderHolder` 经 ServiceLoader）跨模块装载；表驱动编解码的大静态数组 |
| jdk.localedata | `TestLocaleNumberCjk`、`TestLocaleDateCjk`、`TestLocaleCurrency` | `Locale.CHINA` / `TAIWAN` / `JAPAN` / `KOREA` / `GERMANY` 的 `NumberFormat`、`DecimalFormatSymbols`、`DateTimeFormatter.ofLocalizedDate(FormatStyle.*)`、`DayOfWeek / Month.getDisplayName`、`Currency.getSymbol(locale)`、`String.format(locale, …)` | `LocaleProviderAdapter` / CLDR 提供者的资源包类装载；闭包规模（预计最大的 A 档） |
| java.logging | `TestLoggerHierarchy`、`TestLogHandlerFormat`、`TestLogLevelFilter` | 父子 Logger 继承级别与 Handler、`setUseParentHandlers`、自定义 Formatter / Filter、`Level.parse`、`LogRecord` 参数替换、`Logger.getAnonymousLogger` | `LogManager` 初始化：读 `<java.home>/conf/logging.properties`（jmod 之外的 JDK 资源）、系统属性、关闭钩子；按 §三.3 输出到 stdout |
| java.sql | `TestSqlDateTime`、`TestSqlDriverManager` | `Date / Time / Timestamp` 的 `valueOf / toString / toLocalDate / toInstant`、纳秒、`compareTo`；自定义 `Driver` 用 `DriverManager.registerDriver` 注册、`getDriver(url)`、`getConnection` 无匹配时的 `SQLException` 文本 | 平台类加载器载入 java.sql；`DriverManager` 内部取调用者类（`Reflection.getCallerClass`，接 b3 的 CallerSensitive 工作）；驱动服务查找为空的路径 |
| jdk.random | `TestRandomGeneratorOf`、`TestRandomFactoryAll` | `RandomGenerator.of("L64X128MixRandom" / "Xoshiro256PlusPlus" …)` 固定种子序列（经 `RandomGeneratorFactory.of(..).create(seed)`）、`split` / `jump`、`RandomGeneratorFactory.all()` 的名字集合排序后打印 | 经 ServiceLoader 反射构造生成器；`@RandomGeneratorProperties` 注解读取 |
| jdk.zipfs | `TestZipFsReadWrite`、`TestZipFsWalk` | `FileSystems.newFileSystem(path, Map.of("create","true"))` 写入、读取、`Files.walk` 排序后打印、`newFileSystem(URI "jar:…")`、目录属性、已关闭文件系统的异常 | `FileSystemProvider.installedProviders()` 服务查找；zip 结构读写全走字节码 |
| jdk.crypto.ec | `TestEcKeyAgreement`、`TestEcSignVerify` | 固定种子 `SecureRandom`（`SHA1PRNG` + `setSeed`）下的 EC 密钥对生成、`SHA256withECDSA` 验签真假、ECDH 两方共享密钥相等性（只打印相等与长度，不打印随机值）、曲线参数名 | JCA 提供者跨模块注册（SunEC）；`Security.getProviders()` 顺序 |

### 4.2 B 档（零 native，规模大或路径复杂）

| 模块 | 用例（暂定名） | 覆盖分支 | 主要考验 |
|---|---|---|---|
| java.xml | `TestXmlDomParse`、`TestXmlSaxEvents`、`TestXmlStax`、`TestXmlXPath`、`TestXmlTransform` | 字符串输入的 DOM 解析与遍历、命名空间、SAX 事件序列、StAX 读写、XPath 求值、Transformer 序列化（固定缩进与编码属性）、格式错误时的 `SAXParseException` 行列号 | 工厂查找（系统属性 → `jaxp.properties` → ServiceLoader → 默认实现，大量反射）；闭包预计 3000 类以上，用于检验闭包精度与编译成本 |
| java.net.http + jdk.httpserver | `TestHttpLoopbackSync`、`TestHttpLoopbackAsync` | 回环 HttpServer 多个 context；HttpClient 同步 GET / POST、`sendAsync` + `CompletableFuture` 组合、状态码与响应头（排序后打印）、请求体发布器与响应体处理器 | NIO selector 线程、`Executor`、`CompletableFuture`；依赖 java.base 网络相关 native 已就绪；多线程下的输出确定性 |
| java.naming | `TestJndiNoProvider` | `new InitialContext()` 无提供者时 `NoInitialContextException`、`CompositeName` 解析 | 环境属性查找、服务查找为空 |
| java.scripting | `TestScriptEngineNone` | `ScriptEngineManager.getEngineByName("js")` 返回 null、`getEngineFactories()` 为空 | 服务查找为空 |
| java.sql.rowset | `TestRowSetProvider` | `RowSetProvider.newFactory().createCachedRowSet()`、手动填充行与游标移动 | 工厂反射构造 |

---

## 五、不纳入终态的模块（C 档）

| 模块 | 原因 |
|---|---|
| java.management | 66 个 native 方法（`VMManagementImpl` 等）属 VM 驱动状态，须新增手写；MXBean 输出（内存、线程数、运行时间）依赖虚拟机，无法与 JDK 逐字比对 |
| java.prefs | 17 个平台相关 native；`FileSystemPreferences` 写用户主目录，有副作用 |
| jdk.jfr | 65 个 native，深度依赖 VM 事件基础设施 |
| java.compiler | 无 native，但 `ToolProvider.getSystemJavaCompiler()` 会拉进整个 javac（jdk.compiler），与产品定位无关 |
| java.desktop（AWT 本体） | 570 个 native，图形栈。纯 Java 子集 `java.beans`（PropertyChangeSupport、Introspector）与 `java.awt.geom`（Point2D、AffineTransform、Area）作为第 4 步纳入，用例写在 `74_beans_geom` |

---

## 六、执行步骤

### 第 0 步：预审（可提前，不跑 e2e）

- 先写 A 档用例源码，期望输出由 JDK 21 生成。
- 对每例跑 `rava build <Test.java> --stop-after emit` 与 `rava audit corpus --filter <模块目录>`，产出 `docs/reports/gap-scan-corpus-<目录>.md`。
- 逐例登记以下三项，汇总到本文 §七：
  1. 闭包类数；
  2. `native-missing` / `boundary-stub`；
  3. 预检告警。
- 本步只登记，不修复。
- 负责人：C4 前若有空闲子代理可做。用例与期望输出单独提交，**不进入全量基线**，直到第 1 步。

### 第 1 步：A 档通过（C4 收官后）

- 依赖：**boot layer 合入**（引导期建层与系统模块描述符承载，跨模块服务查找的前提）、C1d-b b3 的 CallerSensitive（DriverManager）。
- 按 jdk.random → jdk.zipfs → java.logging → java.sql → jdk.charsets → jdk.crypto.ec → jdk.localedata 顺序推进（由小到大）。每个模块一个分支，抽查通过即合入。
- 修复原则：修生成器、闭包分析器或清单；不新增手写；不改合法用例；修复补边界用例。
- 验收：A 档全部用例通过；全量 e2e 回归 0；逐例闭包类数登记到 §七，超过 2000 类的说明来源。

### 第 2 步：B 档 java.xml

- 先 DOM / SAX，后 StAX / XPath / Transform。闭包与编译时间逐例登记。
- 若闭包超过 3000 类，先查扇出源再继续，不以放宽超时通过。
- 验收：5 例通过，全量回归 0。

### 第 3 步：B 档 HTTP 与空提供者组

- 依赖：java.base 网络 native 完备度。第 0 步预审若发现缺口，先在本步补齐，按手写准入①登记。
- 验收：HTTP 2 例与 naming / scripting / rowset 3 例通过，全量回归 0。

### 第 4 步：java.desktop 纯 Java 子集

- 用例限定 `java.beans`、`java.awt.geom`，运行时设 `java.awt.headless=true`。闭包不得拉进 AWT Toolkit native；拉进即为精度缺陷，按扇出处理。
- 验收：2–3 例通过，闭包中 java.desktop 的 native 方法数为 0。

---

## 七、预审记录（第 0 步填写）

| 用例 | 闭包类数 | native-missing | boundary-stub | 备注 |
|---|---|---|---|---|
| （待填） | | | | |

---

## 八、在路线中的位置

```
C4 收官 ──▶ 本计划第 1 步（A 档）──▶ 第 2 步（xml）──▶ 第 3 步（http / 空提供者）──▶ 第 4 步（beans / geom）
   ▲                ▲
   │                └── boot layer、C1d-b b3 CallerSensitive
   └── 第 0 步预审可与阶段 C 并行（不跑 e2e、不进基线）
```

与 JUnit 依赖包测试（`63_junit`）、R1 运行性能互不阻塞，可并行排期。
