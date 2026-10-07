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

## 五、结果（待 dev 恢复后补）

- 面规模（A / B / 一跳 / 并集 / 公开面）、按包聚合、排序前列：待补
- 闭包实测（类数、方法数、耗时、峰值内存）与 JVM 实载 JDK 类对照：待补
- 分层计数（主力 / 暂缓 / 补测）与按目录分布：待补

## 六、闸门项

- 已知（静态推断，待闭包实测确认）：K11 子类合成（`@Configuration` 的 CGLIB 增强，实载类中可见 `$$SpringCGLIB$$`）；K10 资源枚举（`SpringFactoriesLoader` / `ImportCandidates` 经 `ClassLoader.getResources` 读 `META-INF/spring.factories` 与 `AutoConfiguration.imports`）；反射实例化种子（变体 B 以 JVM 实载类补，档案口径下需分析器通用建模）。
- 实测：待补

## 七、已知失败首批暂缓判定的复核

待分层数据产出后逐例填写（9 例暂缓 + 保留的 TestClassModuleFace / TestProtectionDomainFaces / TestSetAccessibleBoundary / TestXmlSaxEvents / JUnit 10 例 / TestLocaleCurrency）。判定变化只在此列出，不改 `docs/known_failures.toml`。
