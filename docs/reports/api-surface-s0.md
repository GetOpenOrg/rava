# S0 API 面报告（最小 Spring Boot）

> 日期：2026-10-07（框架稿；dev 关机维护，闭包与扫描未跑完，数据节待补）
> 口径：[framework-driven-api-coverage](../plans/2026-10-07-framework-driven-api-coverage.md) 步骤 1（面计算）与步骤 2 的数据部分。
> 产出物：`tests/api_surface/s0.txt`（面，每行 `owner.name:descriptor`，owner 为声明类内部名）、
> `tests/api_surface/tiers_s0.toml`（e2e 分层 + 补测清单）。两者均由 `scripts/api_surface_job.sh s0` 在 dev 上生成，本机不跑。

## 一、S0 jar 集合（18 个，`tests/lib_pilot/s0_boot/jars.txt`）

钉代 Boot 3.5.16（3.x 末线）+ Spring 6.2.19（即 Boot 3.5.16 托管版本），不上 Boot 4 / Spring 7。

| 组 | jar（版本） | 备注 |
|---|---|---|
| Spring | spring-boot / spring-boot-autoconfigure 3.5.16；spring-context / beans / core / aop / expression / jcl 6.2.19 | core / beans 早已在 pom |
| starter-logging | logback-classic / logback-core 1.5.38、slf4j-api 2.0.20、jul-to-slf4j 2.0.20、log4j-to-slf4j / log4j-api 2.24.3 | logback / slf4j 沿用 pom 已钉的同线较新版（Boot 托管 1.5.34 / 2.0.18）；pilot-libs 每坐标只留一份 |
| 其他 | snakeyaml 2.7（托管 2.4，同上沿用）、jakarta.annotation-api 2.1.1、micrometer-observation / micrometer-commons 1.15.12 | |

取包：dev 上作业模式跑 `fetch_pilot_deps.sh --no-scan`，maven 本地库落在检出目录内（`build/m2`），不写服务器 `~/.m2`；锁 102 条，坐标齐全（apis0-s0-38c29002）。

## 二、样例应用（`tests/lib_pilot/s0_boot/`）

- `S0Application`：`@SpringBootApplication` + `SpringApplication.run`；
- `AppConfig`：`@Configuration` + `@Bean GreetingFormatter`（`@Value("${s0.greeting.prefix}")` 注入参数）；
- `GreetingService`：`@Component`，构造器注入 `GreetingFormatter` 与 `@Value` 的 `List<String>` / `int`（带缺省值）；
- `GreetingRunner`：`@Component` + `CommandLineRunner`，slf4j 记两行日志，`System.out` 打六行问候；
- `resources/application.yml`：`spring.application.name`、`spring.main.banner-mode: off`、`s0.*` 配置；日志用 Boot 缺省 logback 配置（无 logback.xml）。

**真 JVM 实测**（dev，apis0-s0-38c29002，类路径只含上述 18 jar；JDK 为服务器系统 21.0.12，作业脚本此后已改为参考 JDK 21.0.11）：rc=0，启动 0.42 s。

```
INFO ... --- [s0-boot] [main] s0app.S0Application  : Starting S0Application using Java 21.0.12.1 with PID ... 
INFO ... --- [s0-boot] [main] s0app.S0Application  : No active profile set, falling back to 1 default profile: "default"
INFO ... --- [s0-boot] [main] s0app.S0Application  : Started S0Application in 0.422 seconds (process running for 0.521)
INFO ... --- [s0-boot] [main] s0app.GreetingRunner : runner start: app=s0-boot args=0
Hello Alice!
Hello Bob!
Hello Carol!
Hello Alice!
Hello Bob!
Hello Carol!
INFO ... --- [s0-boot] [main] s0app.GreetingRunner : runner done
```

`-Xlog:class+load`：实载 JDK 类 1722 个，非 JDK 类 2873 个，其中来自 S0 jar / 样例类目录的 1987 个（其余为 CGLIB / lambda / 隐藏类等运行期生成类）。golden 对账须先剥离时间戳 / PID / 耗时（后续 golden 步骤处理）。

## 三、方法

1. **调用链面**：`rava closure <样例类目录> --deps deps.lock.toml --cp <18 条目>`，两个变体：
   - A：只以 `main` 为入口（档案口径，反映静态可达性）；
   - B：A + JVM 实载的 1987 个框架 / 样例类作 `--seed-class`（补齐反射实例化、`spring.factories` / `AutoConfiguration.imports` 装载、条件评估等静态不可见的入口；只作命令行种子，不进 runtime/ 清单）。
   取闭包 `methods` 中 owner 为 JDK 类（`$JAVA_HOME/jmods` 内）且非 missing 的方法键。
2. **一跳面**：18 个 jar 的常量池 Methodref / InterfaceMethodref（口径同 `jdk_method_scan.py`，另带描述符），经 `scripts/jdk_index.py` 解析到声明类（JVMS §5.4.3.3 简化），与闭包方法键对齐。
3. **面** = 调用链面（A ∪ B）∪ 一跳面；排序键 =（引用 jar 数 × 调用链命中，引用 jar 数，调用链命中，字典序）。
4. **e2e 分层**（`scripts/api_surface.py tiers`）：每例单独 javac（63_junit 带 junit / hamcrest 类路径），常量池 JDK 方法集解析到声明类；
   - 语义类目录白名单 → 主力；
   - 专属方法 = 方法集 − 通用方法（被 ≥10% e2e 用例引用，如打印 / 拼接 / 装箱，不让它们把所有用例都拉进主力）；专属为空 → 主力；
   - 专属 ∩ 面 ≠ ∅ → 主力，否则 → 暂缓；
   - 补测 = 一跳面上的公开 JDK 方法（java / javax / org.w3c / org.xml / org.ietf 包下 public 类的 public / protected 方法）中 e2e 零直接引用者。

## 四、语义类目录白名单（目录名推断，请复核）

`tests/api_surface/semantic_dirs.toml`，33 个目录：

01_basics、02_oop、03_generics、06_exceptions、07_lambdas、08_numbers、09_enum、10_static、11_inner_classes、12_generics_advanced、14_functional、15_switch、16_modern、19_abstract、20_varargs、21_casting、22_autoboxing、23_algorithms、25_multicatch、26_interface_advanced、28_generics_wildcards、29_nested_generic、31_callchain、39_bits、40_numeric_edge、41_patterns_advanced、45_enum_deep、46_generics_deep、49_exceptions_deep、50_arrays_bounds、51_autobox_edge、52_lang_features、57_lambda_var。

未列入（按 API 面分层）的边界目录及理由：04 / 13 / 33 / 44 集合 API；05 / 17 / 27 / 42 字符串与正则 API；18_arrays_advanced（`java.util.Arrays` 工具面占多数）；24_object_methods（含反射调用例）；47_annotations（注解反射）；48_refs（引用 / VarHandle）；58_misc（混合）；59_method_handles（`java.lang.invoke`）。

## 五、结果（kr2 作业 `apis0-kr2b-1fdb1bed`，@ 1fdb1bed；闭包待 dev 恢复复算）

### 5.1 闭包实测（两变体均未产出）

| 变体 | 入口 | 结果 | 耗时 | 峰值 RSS | 失败点 |
|---|---|---|---|---|---|
| A | `s0app.S0Application.main` | rc=124（超时） | 2400 s（上限） | 未取到（`timeout` 先于 `/usr/bin/time` 结束） | 资源束与按名读资源阶段已过（ServiceLoader 读 log4j / slf4j provider、`sun.util.logging.resources.logging` 束），之后 37 分钟无新输出，未到达不动点 |
| B | A + 1987 个实载框架类种子 | rc=137（SIGKILL，内存上限） | 737 s | 11.58 GiB（12140864 KB，机器上限 11891 MiB） | 展开阶段，无阶段日志即被杀 |

kr2 为 15G / 8 核，作业内存上限 11891 MiB。结论：S0 档案规模的闭包在该机型上不可完成，A 是时间不够，B 是内存不够。**本节以下的「调用链命中」一律取 JVM 实载近似**：一跳方法的声明类被真 JVM 加载，就算命中（`chain_mode = "jvm-loaded"`）。dev 恢复后用同一作业脚本复算，`s0.txt` 与 `tiers_s0.toml` 随之重出。

### 5.2 面规模

| 口径 | 方法数 |
|---|---|
| 调用链面（闭包） | 0（未产出） |
| 一跳面（18 jar 常量池引用，解析到声明类） | 2868 |
| 并集 = `tests/api_surface/s0.txt` | **2868** |
| 其中公开 JDK API（java / javax / org.w3c / org.xml / org.ietf，public 类 public / protected） | 2865 |
| 其中声明类被 JVM 实载（命中） | 2087（公开 2086） |
| JVM 实载 JDK 类 / 非 JDK 类 | 1720 / 2876（S0 jar 与样例 1987） |

各 jar 的一跳 JDK 方法数：spring-core 1506、spring-boot 935、spring-context 786、logback-core 682、spring-beans 557、log4j-api 432、spring-boot-autoconfigure 424、snakeyaml 335、spring-expression 268、logback-classic 259、spring-aop 222、slf4j-api 113、micrometer-commons 103、micrometer-observation 80、spring-jcl 40、log4j-to-slf4j 21、jul-to-slf4j 17、jakarta.annotation-api 2。

引用 jar 数的分布：只被 1 个 jar 引用的方法 1679 个，被 2 个引用的 464 个，被 ≥ 5 个引用的 386 个；最多 16 个 jar（`Class.getName`、`Object.<init>`、`Object.getClass`）。

### 5.3 按包聚合（前 25；列为 面 / 命中）

| 包 | 面 | 命中 | 包 | 面 | 命中 |
|---|---|---|---|---|---|
| java.util | 533 | 504 | java.security | 54 | 16 |
| java.lang | 498 | 492 | java.util.stream | 47 | 46 |
| java.io | 188 | 143 | javax.management | 46 | 0 |
| java.util.concurrent | 154 | 128 | java.text | 45 | 7 |
| java.lang.reflect | 120 | 120 | java.util.concurrent.atomic | 45 | 35 |
| java.time | 99 | 97 | java.util.logging | 44 | 23 |
| java.net | 83 | 61 | javax.xml.stream.events | 43 | 0 |
| javax.xml.stream | 81 | 0 | org.xml.sax | 39 | 15 |
| java.nio.file | 69 | 62 | java.math | 36 | 36 |
| javax.net.ssl | 61 | 0 | java.nio | 35 | 35 |
| java.beans | 59 | 47 | org.w3c.dom | 27 | 0 |
| | | | javax.management.modelmbean | 25 | 0 |
| | | | javax.lang.model.element | 25 | 0 |
| | | | java.util.function | 24 | 22 |

命中为 0 的包（javax.xml.stream、javax.net.ssl、javax.management*、org.w3c.dom、javax.lang.model）都是 jar 里可选特性的引用，例如 logback 的 SSL 与 JMX 配置、spring-core 的 StAX 工具、Boot 的注解处理元数据。最小 Boot 启动时不触达它们。这类方法在一跳面上，但不在实载面上，是闭包复算时预期会剪掉的主体。

### 5.4 排序前列（键：引用 jar 数 × 命中，引用 jar 数，命中，字典序）

前 40 均已命中，依次为：`Class.getName`、`Object.<init>`、`Object.getClass`（16 jar）；`Enum.<init>`、`Enum.valueOf`、`String.equals`、`String.valueOf(Object)`、`LambdaMetafactory.metafactory`、`Iterator.hasNext / next`（15）；`IllegalArgumentException.<init>(String)`、`IllegalStateException.<init>(String)`、`String.isEmpty`、`StringBuilder.<init> / append(C / Object / String) / toString`、`Arrays.asList`、`List.add / iterator`、`Map.containsKey / get / put / remove`（14）；`Class.isAssignableFrom`、`Object.toString`、`String.hashCode / length`、`System.arraycopy`、`Thread.currentThread`、`ArrayList.<init>() / (I)`、`HashMap.<init>`、`List.addAll / size`、`Set.add / iterator`、`ConcurrentHashMap.<init>`（13）。完整排序见 `tests/api_surface/s0.txt`。

### 5.5 e2e 分层（`tests/api_surface/tiers_s0.toml`）

| 项 | 数 |
|---|---|
| e2e 用例 | 1105（javac 失败 1：`41_patterns_advanced/TestUnnamedVar`，未命名变量需 `--enable-preview`，属白名单目录，缺省主力） |
| 通用方法（被 ≥ 10%，即 ≥ 110 例引用；`common_df = 0.1`） | 22 个（`println` 系列、`StringBuilder`、`ArrayList` / `List` / `Iterator`、`LambdaMetafactory`、`StringConcatFactory` 等） |
| **主力** | **1095**（白名单目录 674，规则 2 / 3 共 421） |
| **暂缓** | **10** |
| **补测**（面上公开 JDK 方法、e2e 零直接引用） | **1376** |
| e2e 已直接引用的面方法 | 1490 |

暂缓 10 例（专属方法全部在面外）：

| 用例 | 面外的专属 API |
|---|---|
| 04_collections/PriorityQueueDemo | `PriorityQueue` |
| 30_streams/JavaBaseLongStatisticsTest | `LongSummaryStatistics` |
| 30_streams/JavaBaseStatisticsTest | `Int / DoubleSummaryStatistics` |
| 35_io/ScannerInputTest | `Scanner`、`PrintStream.println(D / Z)` |
| 35_io/ScannerTest | `Scanner` |
| 37_datetime/TestLocalDate | `LocalDate` 日历运算 |
| 53_io_api/TestSecurityProperties | `java.security.Security` |
| 67_sql/TestRowSetProvider | `javax.sql.rowset` |
| 67_sql/TestSqlDateTime | `java.sql.Date / Time / Timestamp` |
| 68_random_gen/TestRandomGeneratorOf | `java.util.random.RandomGenerator*` |

补测清单按包分布（前 10）：java.util 163、java.util.concurrent 116、java.lang 111、java.io 73、javax.xml.stream 70、java.time 65、javax.net.ssl 61、java.lang.reflect 52、java.net 52、javax.management 46。被最多 jar 引用的缺口有：`UnsupportedOperationException.<init>()`（10 jar）；`Boolean.equals`、`IllegalArgument / IllegalStateException.<init>(String, Throwable)`、`List.equals`（8）；`ClassLoader.getResources`、`String.toLowerCase(Locale)`、`Method.getParameterCount`、`LinkedHashSet / HashSet / LinkedHashMap / ArrayDeque.<init>(I)`、`Lock.lock / unlock`、`Stream.concat`（6–7）；`Annotation.annotationType`、`Executable.getParameters`、`Constructor.getParameterTypes`、`URL.openConnection`、`URLConnection.getInputStream`（5–6）。补测应按 `s0.txt` 的排序优先覆盖命中项；面外的 javax.xml.stream / javax.net.ssl / javax.management 等待闭包复算确认后再定。

**分层的区分度不足（须在闭包复算时一并修正）**：当前面是一跳全集（2868 个），比调用链宽，所以规则 3「专属 ∩ 面 ≠ ∅」几乎对所有用例成立，暂缓只剩 10 例。典型例子是 TestCharsetAvailable：唯一落在面内的专属方法是 `Map.containsKey`，关键 API `Charset.availableCharsets` 在面外，却判了主力。终态口径是：面取闭包调用链面（不取一跳并集），并以「专属方法中面外占比」或「关键 API 在面外」判暂缓。阈值做成 `tiers` 参数，与 `common_df` 同样写入数据文件。本轮 toml 按任务口径原样产出。

## 六、闸门项

1. **G1 闭包规模（实测）**：S0 档案在 15G 机型上闭包不可完成。A 超时 40 分钟，资源束阶段之后无进展；B 带 1987 个框架种子，在 11.6 GiB 处被 OOM 杀掉。这是 S0 一跳面以外所有口径的前置条件。dev（大内存）恢复后先复算，拿到真实峰值和不动点轮数，再判断是需要分析器内存优化，还是可以分片。
2. **G2 反射实例化与组件扫描**：Boot 的 bean 类由 `@ComponentScan` 包扫描、`AutoConfiguration.imports` 与 `spring.factories` 按名装载，再经反射构造。只以 main 为入口（A）看不见这些类，只能靠 JVM 实载种子（B）补。在档案口径下，需要分析器对「资源枚举 → `Class.forName` / 构造器反射」做通用建模（K10 资源枚举）。按原则，不往 runtime/ 加库种子。
3. **G3 CGLIB 子类合成（K11）**：`@Configuration` 增强类 `$$SpringCGLIB$$` 在运行期定义，实载类中可见。这属于运行期类定义点，需要走构建期合成或 AOT 产物，不能靠静态闭包。
4. **G4 面的精度**：在闭包产出前，「命中」是类级近似（声明类被加载即算命中），会高估。例如 `Thread.ofVirtual`、`Transformer.setOutputProperty` 因 `Thread` / `Transformer` 类被加载而计为命中，但 S0 启动路径未必调用它们。

## 七、已知失败首批暂缓判定的复核

口径：看用例的关键 API（区分该用例的那组方法）是否在 S0 面上。机械分层结果一并列出，仅供参考，因为第五节已指出规则 3 区分度不足。**判定变化只在此列出，不改 `docs/known_failures.toml`。**

### 7.1 首批暂缓 9 例

| 用例 | 现判 | 机械分层 | 关键 API 在面上？ | 复核结论 |
|---|---|---|---|---|
| TestModuleLayerDefine | 面外 | 主力（25 / 47 面内） | 否：`ModuleLayer.defineModulesWithOneLoader`、`Configuration.resolve`、`ModuleFinder.of`、`ModuleDescriptor.Builder` 全在面外；面内的只是 `ModuleLayer.boot`、`Module.getName` 等只读面 | 维持面外 |
| TestLocaleDateCjk | 面外 | 主力（2 / 7） | 部分：`DateTimeFormatter.ofLocalizedDate` 在面上（1 jar 引用）；`DayOfWeek / Month.getDisplayName`、`LocalDate.format` 不在。失败根因是 CJK 本地化数据，S0 启动不用 CJK locale | 维持面外 |
| TestVirtualThreadScale | 面外 | 主力（10 / 12） | 是：`Thread.ofVirtual` 在面上（1 jar 引用，Spring 6.1+ 的虚拟线程执行器支持）；`Thread$Builder.start` 在面外 | **变化候选：面外 → S0 相关**。默认配置不开虚拟线程，建议仍暂缓，标「S0 可选特性」，闭包复算后再定 |
| TestStringGetCharsLegacy | 面外 | 主力（白名单目录 52） | 是：`String.getChars(II[CI)V` 在面上；`System.getSecurityManager` 也在面上 | **变化：面外 → S0 面内**，建议移出暂缓 |
| TestCharsetAvailable | S1 | 主力（1 / 2，仅 `Map.containsKey`） | 否：`Charset.availableCharsets` 在面外 | 维持 S1（机械主力属规则 3 的误判） |
| TestHttpLoopbackSync | S1 | 主力（8 / 34） | 否：`com.sun.net.httpserver.*` 全在面外，面内的只是 `URI` / `InetSocketAddress` / 流 | 维持 S1 |
| TestHttpLoopbackAsync | S1 | 主力（8 / 29） | 否：`HttpServer`、`java.net.http.HttpClient` 全在面外 | 维持 S1 |
| TestXmlTransform | S1 | 主力（8 / 15） | 部分：只有 `Transformer.setOutputProperty` 在面上（1 jar），`TransformerFactory.newInstance`、`Transformer.transform`、`StreamSource / StreamResult` 不在 | 维持 S1 |
| TestRowSetProvider | S2 | 暂缓（0 / 10） | 否 | 维持 S2（与机械分层一致） |

### 7.2 保留各例

| 用例 | 机械分层 | 关键 API 在面上？ | 复核结论 |
|---|---|---|---|
| TestClassModuleFace | 主力（4 / 7） | 是：`Class.getModule`、`Module.isNamed`、`Module.getName`、`Class.getClassLoader` | 维持保留 |
| TestProtectionDomainFaces | 主力（8 / 11） | 是：`Class.getProtectionDomain`（4 jar；Boot 用它定位应用 jar）、`ProtectionDomain.getCodeSource`、`Package.getImplementation*` | 维持保留 |
| TestSetAccessibleBoundary | 主力（4 / 5） | 是：`Field.setAccessible / get / set`、`Class.getDeclaredField`（`trySetAccessible` 在面外） | 维持保留 |
| TestXmlSaxEvents | 主力（15 / 15） | 是：`SAXParserFactory`、`SAXParser.parse`、`org.xml.sax.Attributes` 全在面上 | 维持保留 |
| JUnit 10 例（63_junit） | 主力（专属 0–4 个且全在面内） | 无 JDK 专属关键 API，由库驱动 | 维持保留 |
| TestLocaleCurrency | 主力（7 / 9） | 是：`Currency.getInstance`、`NumberFormat.getCurrencyInstance` 在面上（`getSymbol`、`getDefaultFractionDigits` 在面外） | 维持保留 |

复核差异汇总：TestStringGetCharsLegacy（面外 → S0 面内，建议移出暂缓）；TestVirtualThreadScale（关键 API 在面上，属 S0 可选特性，建议闭包复算后再定）。其余 7 例暂缓与全部保留例维持原判。
