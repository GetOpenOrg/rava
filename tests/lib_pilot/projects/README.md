# lib_pilot 多文件用户项目（2026-10-12 自 html-to-markdown/java-examples 迁入）

每个目录 = 一个**多文件用户项目**：`src/`（原样保留 `package com.example.*` 与辅助类，不改内容）+
`form.toml`（依赖声明，与 63_junit 同机制）+ `golden/`（每个 main 入口一对
`<库>_<入口简名>_{jvm,rs}.txt`）。运行与对账入口：

```bash
scripts/fetch_pilot_deps.sh --no-scan                       # 先取包生成依赖锁（测试机）
scripts/lib_pilot_golden.sh guava/GuavaCollectionsDemo      # <库>/<入口>；入口可 FQCN 或简单名
```

golden 脚本一次全量编译项目 src、以声明入口运行（javac / rava build 均多源 + `--main`）。
JVM 侧 golden 全入口先行；**rs 侧随能力闸门点亮**（缺口分析 A–F 层，2026-10-06），未点亮的
记回归清单台账，不阻塞迁入。

## 目录与入口清单

| 项目 | 入口（main） | 闸门（rs 点亮序） |
|---|---|---|
| guava | GuavaCollectionsDemo、GuavaCacheDemo | C1D/T2 闭包收窄 |
| eclipse-collections | EclipseCollectionsDemo、EcContainersDemo、EcDeepApiDemo、EcWordCountDemo | C1D/T2 |
| commons-collections4 | 文章示例 1 例 | 近零门槛 |
| thumbnailator | ThumbnailatorDemo | 近零门槛 |
| pqc | PqcKemDemo、PqcSignDemo | 近零门槛 |
| lombok | LombokAnnotationsDemo | 编译期注解改写 |
| asm | AsmProxyDemo（加载面；生成面见单文件 pilot） | C-SYN |
| cglib | CglibProxyDemo、BeanCopierDemo、BulkBeanDemo、DispatcherDemo、LazyLoaderDemo、ImmutableBeanDemo、CallbackFilterDemo、AnnotationLogDemo | C-SYN |
| instrument | MonitorDemo、CacheProxyDemo、PerformanceCompareDemo | C-SYN |
| aspectj | AspectJDemo | 织入（见下） |
| rxjava | RxBasicDemo、RxOperatorsDemo、RxCacheDemo、RxThreadDemo | C-MT |
| disruptor | DisruptorChainDemo、DisruptorPipelineDemo | C-MT |
| pekko | Actor 2 例 | C-MT |
| redis | RedisDataStructuresDemo、RedisDauDemo、RedisSessionLockDemo、RedisClient | C-NET（见下） |
| netty | NettyEmbeddedChannelDemo、NettyNioEchoDemo | C-NET |
| okhttp | OkHttpDemo、OkioDemo | C-NET（MockWebServer 回环） |
| kafka | KafkaClientsDemo | C-NET（Mock 面，无需 broker） |
| kotlinlib | KotlinStdLibDemo、CoroutinesFutureDemo | 多语言 jar（E-3） |
| scala-interop | ScalaInteropDemo | 多语言 jar |
| groovy | GroovyTemplateDemo | 多语言 jar |
| clojure | ClojureApiDemo | 多语言 jar |
| polyglot | PolyglotDemo | 原生桥接 B5（见下） |
| templates | FreeMarkerDemo、ThymeleafDemo、VelocityDemo | 引擎编译期生成 |
| esper | EsperLengthWindowDemo、EsperPatternDemo | 引擎编译期生成 |
| timefold | TimefoldScheduleDemo | 约束求解重计算 |
| querydsl | QuerydslCollectionsDemo | APT 免（PathBuilder） |
| tomcat | EmbeddedTomcatDemo | 完整应用 |
| containers | ContainerShowdownDemo | 完整应用（Tomcat / Jetty / Undertow / Netty 四容器回环对比） |
| frameworks | FrameworkComparisonDemo、RouteAnnotationDemo；另 HelloEndpoints 为**非入口** holder 类（见下） | 完整应用 |

## 双版本依赖模块（deps-extra，方案 A）

依赖锁是「每坐标一份 jar」的扁平表，而 Maven 单 pom 对同一 GA 坐标只能产出一个版本
（依赖调解）。当某项目需要的版本与主模块锁钉值冲突时，**在 `tests/lib_pilot/deps-extra/<模块>/`
开独立模块并存**（独立 pom，不继承主 pom；独立 `target/pilot-libs/` 与 `target/deps.lock.toml`），
不改主锁、不降级已存档 golden 的钉值。机制议题与裁决记录：
`docs/plans/2026-10-13-lib-pilot-dual-version-mechanism.md`。

- **何时开新模块**：项目运行所需的某依赖版本 ≠ 主锁钉值（典型：上游库自己声明的传递钉值，
  主锁里被别的传递线调解成了另一版本），且主锁钉值有已存档 golden 背书不可改。
- **form.deps 用法**：冲突项目 form.toml 把 `deps` 键指到模块锁
  （如 `deps = "tests/lib_pilot/deps-extra/esper/target/deps.lock.toml"`），`cp` 照常列锁条目名。
  e2e（`scripts/e2e_form.py`）与 golden（`scripts/lib_pilot_golden.sh` 项目模式）读到该键即以该锁
  解析 cp 与 `--deps`；锁内 jar 路径相对锁所在目录。**模块须自持该项目全部 cp 条目**（其他依赖
  与主模块同版钉值重复声明即可）。无 `deps` 键 / 指向主锁的行为与旧完全一致。
- **取包**：`scripts/fetch_pilot_deps.sh` 先跑主模块（行为不变），再循环处理 `deps-extra/*/` 各模块
  （同一 rm -rf 清残留、copy-dependencies 导出与锁生成口径）。
- **现有模块**：`esper`（esper 7.1.0 + antlr4-runtime 4.7，主锁经 hibernate 调解为 4.13.2 会
  ATN 版本失配）；`thymeleaf`（thymeleaf 3.1.3 + ognl 3.3.4，主锁钉 3.4.14 有三个单文件 pilot
  golden 背书；模块另自持 FreeMarker / Velocity 等项目 cp）。

## 非入口类登记

- **HelloEndpoints**（frameworks）：上游「三个框架现在的样子」的**注解编译演示 holder 类**——
  同一 /hello 接口的 Spring / JAX-RS / Micronaut 三种注解写法各一个静态内部类，私有构造器、
  **无 main**，路由信息由 RouteAnnotationDemo 反射读取。不参与 golden（无 JVM / rs 输出），
  上游亦如此（非运行入口），不计入点亮台账。

## 已知运行前置（测试机上处理）

- **aspectj**：注解面 javac 直接过，但切面生效需织入（上游用 aspectj-maven-plugin 编译期织入）。
  golden 先取未织入的确定性行为，或后置到带 `-javaagent` 的运行口径——首跑时定。
- **redis**：Jedis 连 `localhost:6379`，JVM 侧 golden 需本机 Redis；无服务时该 4 入口记台账后置。
- **polyglot**：`Context.eval("js", …)` 需 `org.graalvm.polyglot:js` 实现在类路径（上游 pom 未含，
  首跑时补钉）；GraalVM JDK 上开箱即用。
- **cp 收敛**：各 form.toml 的 `cp` 已列直接与已知传递依赖；若 JVM 侧首跑报
  NoClassDefFound，把缺的锁条目名补进 `cp` 即可（传递 jar 均已被 Maven 取进 pilot-libs）。
- **mockwebserver**：okhttp 项目的回环服务（确定性端口由库管理）。
