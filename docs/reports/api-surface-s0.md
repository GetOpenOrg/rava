# S0 API 面报告（最小 Spring Boot）

> 日期：2026-10-07 立稿；2026-10-10 dev 复算（闭包仍未产出，见 §5.1 与闸门 G1；分层规则 3 改为面外占比阈值，见 §5.5）
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
   - 专属方法中面外占比 > `out_ratio` → 暂缓，否则 → 主力。阈值由标定集数据决定（`--out-ratio auto`，见 §5.5），脚本里不写类名；
   - 补测 = 一跳面上的公开 JDK 方法（java / javax / org.w3c / org.xml / org.ietf 包下 public 类的 public / protected 方法）中 e2e 零直接引用者。

## 四、语义类目录白名单（目录名推断，请复核）

`tests/api_surface/semantic_dirs.toml`，33 个目录：

01_basics、02_oop、03_generics、06_exceptions、07_lambdas、08_numbers、09_enum、10_static、11_inner_classes、12_generics_advanced、14_functional、15_switch、16_modern、19_abstract、20_varargs、21_casting、22_autoboxing、23_algorithms、25_multicatch、26_interface_advanced、28_generics_wildcards、29_nested_generic、31_callchain、39_bits、40_numeric_edge、41_patterns_advanced、45_enum_deep、46_generics_deep、49_exceptions_deep、50_arrays_bounds、51_autobox_edge、52_lang_features、57_lambda_var。

未列入（按 API 面分层）的边界目录及理由：04 / 13 / 33 / 44 集合 API；05 / 17 / 27 / 42 字符串与正则 API；18_arrays_advanced（`java.util.Arrays` 工具面占多数）；24_object_methods（含反射调用例）；47_annotations（注解反射）；48_refs（引用 / VarHandle）；58_misc（混合）；59_method_handles（`java.lang.invoke`）。

## 五、结果（面与分层：dev 作业 `apis0-dev4-0517c6f3` @ 0517c6f3；闭包：§5.1）

### 5.1 闭包口径实测（dev 复算，两变体仍未产出）

dev（128G / 32 线程）三轮作业，作业脚本带看门狗：每 30 s 采样 RSS，超时或 RSS 超上限时按 PID 终止，并记下终止原因。第三轮另在 gdb 下运行，每 600 s 取一次全线程栈（`API_SURFACE_STACK_EVERY`）。

| 作业 | 变体 | 上限 | 结果 | RSS 曲线（秒：MiB） | 末条阶段日志 |
|---|---|---|---|---|---|
| kr2 `apis0-kr2b-1fdb1bed`（15G） | A | 2400 s | 超时 | 未采 | 按名读资源（累计 12）之后 37 分钟无输出 |
| 同上 | B | 机器内存 11891 MiB | OOM 杀（737 s，11.58 GiB） | 未采 | 无 |
| dev `apis0-dev2-b4e01dc7` @ b4e01dc7，A / B 并行 | A | 30720 MiB | 看门狗终止：RSS 31552 MiB（1260 s）；time -v 峰值 32307552 KB，用户态 1246 s（单线程） | 150:3489、300:8968、600:16814、900:21462、1200:27998 | 「按名读取：81 个调用点 → 资源 +1（累计 12）」 |
| 同上 | B | 30720 MiB | 看门狗终止：RSS 31822 MiB（1831 s）；峰值 32583116 KB | 300:6042、600:9663、900:13771、1201:19142、1501:26538、1801:30565 | 「按对象物化：64 个对象……峰值 1014 MiB」（第 1 条） |
| dev `apis0-dev3-b05da66c` @ b05da66c，只跑 A，gdb 取栈 | A | 57344 MiB | 看门狗终止：**RSS 58451 MiB ≥ 57344 MiB（2251 s）**；time -v 峰值 60662260 KB，用户态 2236 s | 300:11744、601:17070、921:24440、1221:33874、1541:41078、1841:49181、2161:53459 | 同 dev2 A（22 条阶段日志，资源阶段后无输出） |

**阻塞点**：资源阶段之后，闭包主循环的内存近似线性增长，A 约 22 MiB/s（≈ 1.3 GiB/min），B 约 16 MiB/s。增长期间没有任何阶段日志，到 57 GiB 仍未出现收敛迹象。dev3 的 4 次取栈（601 / 1221 / 1841 / 2251 s）落在同一条路径上：

```
Engine::run → process → process_bytecode → event → invoke → invoke_inner (invoke.rs:195)
  → link_hub (hub.rs:170：分派枢纽向新链入的调用点重放已登记的 lambda 列表)
  → dispatch_one (invoke.rs:231) → invoke_lambda (lambda.rs:46) → lambda_step → lambda_connect
  → lambda_dispatch (lambda.rs:157：虚方法引用的 lambda 按开放接收者集在 G 上展开，逐接收者 dispatch_one)
  → dispatch_one (invoke.rs:251/262) → edge / edge_this / caller_edge → add_to → tau_check_add
     或 bind_lambda_params (lambda_vals.rs:25) → bind_pvs → taint_params → taint_slot（HashSet 插入）
```

第 4 次取栈是 lambda 套 lambda：接收者本身是 lambda，于是 `invoke_lambda → lambda_dispatch → dispatch_one → invoke_lambda` 嵌套了两层。从结构看，增长量与「枢纽链入的调用点数 × 枢纽上的 lambda 数 × 方法引用的开放接收者数」成正比，而 `dispatched` 去重键 `(off, r, via_lambda)` 按调用点区分，挡不住这种乘积。release 构建（debug=1、fat LTO）只有行表、没有局部变量，分析器在这一阶段也不打进度，因此**具体是哪个 Java 枢纽 / 方法引用无法从栈上读出**。要定位到类 / 方法，需要给分析器加诊断输出（枢纽重放计数、lambda 展开计数按枢纽打点），这属于分析器改动，不在本任务（只分析）范围内，列为闸门 G1。

**G1 分析器任务（`s0-g1`，2026-10-11，dev）**：加了 `--lambda-prof` 诊断（`API_SURFACE_LAMBDA_PROF=<秒>`）后，乘积路径定位到
`StreamSpliterators$WrappingSpliterator` 一族的 k9 方法引用（`buffer::accept`，Sink / Consumer 及 Int / Long / Double 变体）：
它们经 open Sink 枢纽送达约 2200–2700 个调用点，每个带 50–150 个绑定接收者。每次接收者增长都在每个调用点新建精确集合枢纽，
枢纽上的 lambda 又逐调用点派发，读者（lambda 调用单元）到数百万。逐步改造与实测见性能计划
[`2026-09-30-closure-analyzer-performance.md`](../plans/2026-09-30-closure-analyzer-performance.md) §4.9。最终形态 74ef5f3f 有两处改动：
方法引用接收者改经增长枢纽增量并入；枢纽 lambda 改为「枢纽 × lambda」一个读者，实参取枢纽节点。改后读者从 291 万降到 6.9 万，
枢纽重放从 2065 万降到 18 万，HelloWorld / DeepCopy / TestSerialLookupPairing 在种子 0/1/2 下的集合与 main 一致。

| 提交 | 变体 A | 变体 B |
|---|---|---|
| e3cafda3（改造前段） | 322 s：18.6 GB；520 s：30.4 GB，58 万方法上下文 | 525 s：21.9 GB |
| e6384922（增长枢纽） | 445 s：23.7 GB，读者 291 万 | 458 s：19.5 GB |
| 74ef5f3f（枢纽 × lambda 一个读者） | 210 s：7.9 GB；615 s：26.7 GB，61 万方法上下文，读者 6.9 万（主动停止） | 563 s：19.1 GB，64 万方法上下文 |

lambda 乘积消除后，S0 仍不收敛：A 以约 40 MB/s、每分钟约 3 万个方法上下文的速度增长。新的主项是方法上下文克隆，
142a1c54 的上下文前列（A，161 s，23.6 万方法上下文）如下：
- 按方法：`Object.<init>` 10275、`Objects.requireNonNull` 3296、`ConcurrentHashMap$Node.<init>` 2466、
  `ConcurrentHashMap.comparableClassFor` / `compareComparables` 2453、`Math.min` 2051、`MethodType.makeImpl` / `checkPtypes` 一族 1606、
  `LambdaForm.argument` 1283；
- 按上下文：`@level:0` 5849、`@level:2` 5844，其余是 `java.lang.invoke` 的 `DirectMethodHandle$StaticAccessor` /
  `BoundMethodHandle$Species_*` 分配点（各约 125）。

试验 929de3b8（已回退）把同一方法的抽象对象上下文限到 64 个，超出部分并入无上下文本体。S0 的增长放缓了，但 963 s 时仍不收敛：A 峰值 16.2 GB、27.9 万上下文；B 峰值 16.9 GB、枢纽 78 万。同时 DeepCopy 的闭包被拖到 16 分钟、14.6 GB 后被杀，因为合并进本体使精度崩塌。

**10-11 上下文预算任务（`s0-g1-ctx`）**：上下文无关方法（`Object.<init>` 一类 void 叶构造器链）不克隆；同一成员的对象上下文克隆
达 2048 后按「类型 + 堆上下文尾」合并，不回落本体。既有用例集合与 main 一致。S0 的方法上下文同一时刻少约 30%（A 273 s：5.6 GB、
26.5 万），但仍线性增长：A 600 s 时 20.2 GB、B 900 s 时 23.7 GB 被杀。节点分散在每成员 64–1024 个克隆的数百个成员上，单成员预算
压不住总量。试验把节点总数钉在约 33 万后，增长项换成枢纽：1.8 万 → 78 万（1059 s，18.5 GB），前列是 `Function.apply` /
`Supplier.get` / `PrivilegedAction.run` 的 open 枢纽。G1 的下一主项是枢纽的建立口径，见性能计划 §4.10。

闭包未产出，所以「闭包面对比一跳面」这一节暂时没有数据。面仍按退回口径计算（一跳 ∪ JVM 实载命中，`chain_mode = "jvm-loaded"`）。

### 5.2 面规模

| 口径 | 方法数 |
|---|---|
| 调用链面（闭包） | 0（dev 复算仍未产出，见 §5.1） |
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

**闭包口径下的规则 3（10-10 起）**：面外占比 = 专属方法中不在面上的个数 ÷ 专属方法数。占比 > `out_ratio` 判暂缓，否则判主力（只对非语义目录、专属非空的用例）。原规则「专属 ∩ 面 ≠ ∅ → 主力」在一跳全集上几乎总成立，已废弃。

**阈值由数据定**（`scripts/api_surface.py tiers --out-ratio auto`，作业缺省）：
- 标定集共 14 例，判据是区分该例的关键 API 是否在面上（§七的人工复核）：
  - 暂缓 7 例：TestCharsetAvailable、TestHttpLoopbackSync / Async、TestXmlTransform、TestRowSetProvider，以及已修复移出 known_failures 的 TestModuleLayerDefine、TestLocaleDateCjk；
  - 保留 7 例：TestClassModuleFace、TestProtectionDomainFaces、TestSetAccessibleBoundary、TestXmlSaxEvents、TestLocaleCurrency、TestBootLayer、TestThreadNatives。
- 来源：`docs/known_failures.toml` 中 deferred 为阶段（S<n>）的条目记暂缓，无 deferred 的条目记保留；另加 `tests/api_surface/out_ratio_labels.toml` 的复核判定。deferred 为「面外」的非面因素条目（TestVirtualThreadScale）不参与标定。
- 选法：在相邻占比的中点中，取分错最少的一个；并列时取所在间隔最宽的。

标定结果是**零分错**：保留例最大占比 0.4286（TestClassModuleFace），暂缓例最小占比 0.4667（TestXmlTransform），取中点得 **`out_ratio = 0.4476`**，写在 toml 头部。

| 标定例 | 面外占比 | 标签 | 标定例 | 面外占比 | 标签 |
|---|---|---|---|---|---|
| TestXmlSaxEvents | 0.0 | 保留 | TestXmlTransform | 0.4667 | 暂缓 |
| TestThreadNatives | 0.1053 | 保留 | TestModuleLayerDefine | 0.4681 | 暂缓 |
| TestSetAccessibleBoundary | 0.2 | 保留 | TestCharsetAvailable | 0.5 | 暂缓 |
| TestLocaleCurrency | 0.2222 | 保留 | TestLocaleDateCjk | 0.7143 | 暂缓 |
| TestProtectionDomainFaces | 0.2727 | 保留 | TestHttpLoopbackAsync | 0.7241 | 暂缓 |
| TestBootLayer | 0.4082 | 保留 | TestHttpLoopbackSync | 0.7647 | 暂缓 |
| TestClassModuleFace | 0.4286 | 保留 | TestRowSetProvider | 1.0 | 暂缓 |

参与规则 3 的 428 例面外占比分布（区间下界：例数）：0.0：115、0.1：64、0.2：56、0.3：44、0.4：41、0.5：36、0.6：26、0.7：21、0.8：13、0.9 以上：12，其中恰为 1.0 的 10 例。分布单调下降，没有天然断点，所以阈值只能由标定集决定，不能靠分布形状。

| 项 | 数 |
|---|---|
| e2e 用例 | 1110（javac 失败 1：`41_patterns_advanced/TestUnnamedVar`，未命名变量需 `--enable-preview`，属白名单目录，缺省主力） |
| 通用方法（被 ≥ 10%，即 ≥ 110 例引用；`common_df = 0.1`） | 22 个（`println` 系列、`StringBuilder`、`ArrayList` / `List` / `Iterator`、`LambdaMetafactory`、`StringConcatFactory` 等） |
| **主力** | **989**（原规则 1095） |
| **暂缓** | **121**（原规则 10） |
| **补测**（面上公开 JDK 方法、e2e 零直接引用） | **1370** |
| e2e 已直接引用的面方法 | 1496 |

暂缓按目录（前列）：04_collections 13、35_io 13、37_datetime 8、62_reflection 8、48_refs 7、33_maps 6、53_io_api 6、30_streams 5、58_misc 5、59_method_handles 5、34_concurrency 4、38_math 4，其余 20 个目录共 37 例。明细（每例的面内 / 面外专属方法）见作业产物 `tiers.json` 的 `detail`。注意：面仍是退回口径（一跳 ∪ JVM 实载），闭包面产出后，同一阈值下的暂缓集会随面收窄而变化，届时重出。

原规则下的暂缓 10 例（专属方法全部在面外，新规则下仍全部暂缓）：

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

**区分度（10-10 已修正）**：原规则 3「专属 ∩ 面 ≠ ∅ → 主力」在一跳全集上几乎总成立，例如 TestCharsetAvailable 只因 `Map.containsKey` 落在面内就判了主力。现已改为上文的面外占比阈值，TestCharsetAvailable 的占比为 0.5，判暂缓。剩下的一项是把面本身换成闭包调用链面，这取决于 G1。

## 六、闸门项

1. **G1 闭包规模（实测，未解除；lambda 乘积已消除）**：S0 档案闭包在 dev 上仍不可完成。原始状态：A（只以 main 为入口）到 RSS 58451 MiB（2251 s）被终止；B（加 1987 个种子）到 31822 MiB（1831 s）被终止。栈落在「分派枢纽向新调用点重放 lambda（`link_hub`）× 方法引用 lambda 按接收者展开（`lambda_dispatch`）」的乘积路径上（§5.1）。解除条件：A / B 都在 dev 上产出闭包，峰值 ≤ 16 GiB，耗时 ≤ 30 min。

   **10-11 分析器任务 `s0-g1` 的结果**：诊断与改造已完成。枢纽 lambda 改为按「枢纽 × lambda」只建一个读者，方法引用接收者经增长枢纽增量并入，读者从 291 万降到 6.9 万，既有用例集合与 main 一致。但 S0 仍线性增长：A 在 615 s 时 26.7 GB、61 万方法上下文。新的主项是方法上下文克隆（`Object.<init>` 等叶方法上万份，`@level` 档位上下文与 `java.lang.invoke` 分配点上下文为主），见 §5.1。

   **10-11 上下文预算任务 `s0-g1-ctx` 的结果**：上下文无关方法不克隆、对象上下文克隆预算 2048 按类型合并，既有用例集合与 main 一致；
   同时修了枢纽 lambda 读者的顺序依赖（batch-1011b `closure_independent_of_order`）。S0 仍不收敛：A 600 s 20.2 GB、B 900 s 23.7 GB
   被杀；方法节点总数钉住后，增长项换成枢纽（78 万），见 §5.1 与性能计划 §4.10。

   **剩余工作**：枢纽建立口径（open / 精确集合枢纽按调用点 × 接收者增长的乘积）的收敛改造，完成后复测 S0。G1 是 S0 一跳面以外所有口径的前置条件。
2. **G2 反射实例化与组件扫描**：Boot 的 bean 类由 `@ComponentScan` 包扫描、`AutoConfiguration.imports` 与 `spring.factories` 按名装载，再经反射构造。只以 main 为入口（A）看不见这些类，只能靠 JVM 实载种子（B）补。在档案口径下，需要分析器对「资源枚举 → `Class.forName` / 构造器反射」做通用建模（K10 资源枚举）。按原则，不往 runtime/ 加库种子。
3. **G3 CGLIB 子类合成（K11）**：`@Configuration` 增强类 `$$SpringCGLIB$$` 在运行期定义，实载类中可见。这属于运行期类定义点，需要走构建期合成或 AOT 产物，不能靠静态闭包。
4. **G4 面的精度**：在闭包产出前，「命中」是类级近似（声明类被加载即算命中），会高估。例如 `Thread.ofVirtual`、`Transformer.setOutputProperty` 因 `Thread` / `Transformer` 类被加载而计为命中，但 S0 启动路径未必调用它们。

## 七、已知失败首批暂缓判定的复核

### 7.0 10-10 复核（dev 数据，新规则 3，`out_ratio = 0.4476`）

现有暂缓条目（`docs/known_failures.toml`）与机械分层逐例对照如下。面仍为退回口径（闭包未产出）。

| 用例 | toml 现判 | 面外占比（面外 / 专属） | 机械分层 | 结论 |
|---|---|---|---|---|
| TestCharsetAvailable | S1 | 0.5（1 / 2） | 暂缓 | 一致，维持 S1（原规则下判主力，属误判，已纠正） |
| TestHttpLoopbackSync | S1 | 0.7647（26 / 34） | 暂缓 | 一致，维持 |
| TestHttpLoopbackAsync | S1 | 0.7241（21 / 29） | 暂缓 | 一致，维持 |
| TestXmlTransform | S1 | 0.4667（7 / 15） | 暂缓 | 一致，维持 |
| TestRowSetProvider | S2 | 1.0（10 / 10） | 暂缓 | 一致，维持 |
| TestVirtualThreadScale | 面外 | 0.1667（2 / 12） | 主力 | **有差异**。维持暂缓，toml 补写依据：面外的只有 `Thread$Builder.start`、`Thread.isAlive`；关键 API `Thread.ofVirtual` 只被 1 个 jar 引用，计为命中只是因为 `Thread` 类被实载（G4 类级高估）；S0 缺省不开虚拟线程，失败原因是规模时序，属非面因素。不进入阈值标定集，闭包面产出后再复核 |
| TestStringGetCharsLegacy | （已移出） | 0.1（语义目录 52） | 主力 | 10-07 复核建议移出暂缓；该例已在 f442cecf 修复，并从 known_failures 删除，无需再动 |

未暂缓的已知失败（TestBootLayer 0.4082、TestThreadNatives 0.1053）判主力，与 toml 一致。10-07 复核后已修复移出的 TestModuleLayerDefine（0.4681）、TestLocaleDateCjk（0.7143）判暂缓，与当时「关键 API 在面外」的结论一致。保留各例（§7.2）判主力，占比都 ≤ 0.4286。**toml 唯一改动**：TestVirtualThreadScale 的 `deferred_reason` 补上上述依据，阶段字段不变。

以下 7.1 / 7.2 为 10-07 首轮复核（kr2 数据，原规则 3），保留备查。

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
