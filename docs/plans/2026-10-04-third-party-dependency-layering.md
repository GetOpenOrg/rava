# V12：第三方依赖分层与复用（方案，待用户确认）

> 状态（2026-10-04）：只写方案，未改代码。
> 来源：[`2026-10-04-archive-crate-layering-decisions.md`](2026-10-04-archive-crate-layering-decisions.md) 设计结论 6 与 §四 V12。
> 上游：[`2026-10-01-cross-test-compile-reuse.md`](2026-10-01-cross-test-compile-reuse.md)（T1 档案化、键 `P` / `S` / `B`）；
> [`2026-10-04-t1-step2-direct-rustc-link.md`](2026-10-04-t1-step2-direct-rustc-link.md)（按 jmod 模块切 crate，§2.5 M1 的类 → 模块归属）；
> [`2026-10-01-junit-crate-as-test-harness.md`](2026-10-01-junit-crate-as-test-harness.md)、`tests/lib_pilot/`（验收语料）。
> 本文只写终态，不写过渡方案。

## 一、结论摘要

1. **一个 Java 模块一个 crate，JDK 与第三方库同一个模型。** 第三方库按模块名切 crate：
   - 模块名的来源依次是：所选 release 的 `module-info.class`（含多版本 jar 的 `META-INF/versions/N/`）→ `Automatic-Module-Name` → JPMS jar 文件名推导 → 依赖坐标兜底；
   - crate 名 = 模块名中的 `.` 换成 `_`，例如 `com.google.gson` → `com_google_gson`、`junit` → `junit`；
   - 库之间引用成环（强连通分量）时合并成一个内部 crate，每个成员模块保留一个只做重导出的同名 crate，用户代码里的路径不随合并变化。
2. **档案仍是构建单元的一次多根分析，库没有单独的档案。** `profile.json` 按模块切成**切片**，每个 crate 只由「自己的切片 + 上游 crate 的键」生成。
   - 键改成按 crate 逐个计算（Merkle）：库 crate 的键包含 jar 内容摘要、坐标与版本、切片摘要和上游 crate 的键；
   - 由此：只改用户代码，库与 JDK 都不重编；只改库切片、JDK 切片不变时，JDK crate 不重编；
   - 只要切片和上游键相同，库 crate 在任何构建单元、任何机器上都逐字节相同，可以按内容寻址缓存和分发。
3. **crate 依赖取实际引用，不取 Maven 依赖图。** 依赖边 = 生成代码实际引用到的模块，具名模块还要落在 requires 闭包内。
   - Maven 图只提供坐标、版本和一致性告警；
   - 库与库成环时合并；库引用下游或用户提供的类时，走运行期登记表晚绑定，不产生链接边；
   - 库对 JDK 的访问按 JPMS exports 静态判定：拒绝的访问折叠为抛 `IllegalAccessError`，`--add-opens` / `--add-exports` 作为启动选项进入分析与运行期。
4. **大库内部拆分沿用 D6**：`<lib>_decl` + `<lib>_body_k`，阈值与装箱规则和 JDK 模块相同（5 MB、按类名排序均衡装箱、至少 2 箱），拆分数只由该 crate 的类集合决定，与机器无关。
5. **两种模式只在链接形态上不同。** 语料模式每个库 crate 一个 dylib（panic=unwind）；生产模式是 rlib 静态链接，`--release` 用 fat LTO。
   - 程序链接的是 crate 依赖闭包，**运行期登记的只是本入口类路径上的库**：类查找、ServiceLoader、资源查找都看不到不在类路径上的库，与 JVM 一致。
6. **本机现状里有两处归属缺口必须先修**（§2.2 实测，52 个 pilot jar）：
   - 多版本 jar（`Multi-Release: true`）的版本化条目没有处理：30 个 jar 的模块描述符只在 `META-INF/versions/9/` 下，M1 读不到——其中 18 个模块名算错（如 gson 得到 `gson`，正确的是 `com.google.gson`），另 12 个名字碰巧对，但被当成自动模块、丢了 requires；
   - 版本化类条目被当成普通类入了索引（byte-buddy 3,071 个、jackson-core 7 个）；
   - 另外，库 jar 里属于 JDK 模块所拥有的包的类，按 JPMS 应被 JDK 遮蔽，现在的类路径顺序（user → lib → jdk）会让库类反过来遮蔽 JDK。

## 二、现状

### 2.1 已有机制

| 项 | 现状 | 位置 |
|---|---|---|
| 库输入 | `rava build|profile --lib NAME=JAR[:seed=FQN,…]`；crate 名由用户给出；整包模式把 jar 全部类作种子 | `driver/src/build_libs.rs`、`driver/src/profile_cmd.rs` |
| 库 crate 发射 | 每个 `--lib` 一个 crate；依赖方向 = 命令行声明序（后声明的依赖先声明的）；对 JDK 类的引用全部指向单一的 `java_runtime` | `emit/src/project/lib_crates.rs`、`emit/src/imports/cross.rs`（`CrateRoute`） |
| 类 → 模块归属 | M1：库 jar 按 根 `module-info` → `Automatic-Module-Name` → 文件名推导；自动模块读全部模块；无名模块只用于用户类和无法命名的 jar | `resolve/src/modules.rs` |
| 档案 | 库类与 JDK 类一起算作「非用户侧」并进同一份 `profile.json`，整个档案只有一个键 `P`；库 jar 的内容摘要计入 `P` 的「非用户归档内容」 | `closure/src/profile.rs`、`profile/digest.rs` |
| 开放世界 | 非用户方法里的调用点，接收者可被用户扩展时不做 null 折叠；库的非公开类型按可扩展处理（类路径下用户可以写同包类） | `closure/src/engine/open_world.rs` |
| 档案 crate 版本 | 1b：全部档案 crate（含库 crate）共用一个内容摘要版本 `0.0.<FNV>` | `emit/src/project/archive_side.rs` |

结论：库 crate 已经能生成和编译（pilot m1–m5），但在键、命名、依赖和缓存粒度上，库与 JDK 绑成了一个整体。任何库的变化都会让整个档案失效，crate 名和依赖顺序靠命令行，不来自库本身。

### 2.2 pilot jar 实测（本机，只读）

数据：`tests/lib_pilot/deps/target/pilot-libs/` 的本机快照，52 个 jar。这份快照早于当前 `pom.xml`：其中是 slf4j-api 1.7.36，没有 2026-10-03 新增的框架件，服务器上已有 92 个。工具：`unzip -l`、`jar --describe-module [--release 21]`、`scripts/dep_scan.py`。M1 一列按 `modules.rs` 的规则用脚本模拟，没有运行 rava。

| 类别 | jar 数 | 例子 | M1 现状 |
|---|---:|---|---|
| 根目录有 `module-info.class` | 6 | guava（`com.google.common`）、eclipse-collections（`org.eclipse.collections.impl`，open module）、jakarta.mail、jackson-annotations、jspecify | 正确 |
| 描述符只在 `META-INF/versions/9/`（多版本 jar） | 30 | gson、jackson-core / databind、commons-lang3、assertj、byte-buddy、picocli | **18 个名字错**（`gson`→`com.google.gson`、`jackson.databind`→`com.fasterxml.jackson.databind`、`failureaccess`→`com.google.common.util.concurrent.internal` 等）；**30 个全部被当成自动模块**，丢了 requires |
| 无描述符、有 `Automatic-Module-Name` | 12 | junit（`junit`）、hamcrest（`org.hamcrest`）、httpclient / httpcore 4/5、joda-time、slf4j-api 1.7、jakarta.transaction-api（`java.transaction`） | 正确 |
| 两者都没有，按文件名推导 | 4 | commons-collections 3.2.2、commons-digester 2.1、commons-math3、listenablefuture（0 个类） | 正确 |

其他事实：
- 版本化类条目：byte-buddy 3,071 个、jackson-core 7 个。现在它们以 `META-INF/versions/9/…` 这样的「类名」进了索引；它们在 release ≥ 9 时本应覆盖同名的基础条目。
- 52 个 jar 之间**没有分裂包，也没有重复类**。
- 引用 JDK 内部包（`dep_scan` 一跳）的只有：guava、eclipse-collections → `sun/misc`（jdk.unsupported，已导出）；httpclient5、httpcore5 → `jdk/net`（jdk.net，已导出）。没有对未导出包的静态链接。
- 坐标：47 个 jar 带 `META-INF/maven/*/pom.properties`；junit、hamcrest、hamcrest-core、picocli、jspecify 这 5 个没有。坐标不能只从 jar 里取。
- `requires static` 指向类路径上可能不存在的模块，这种情况很常见：assertj → junit / net.bytebuddy / org.hamcrest，byte-buddy → com.sun.jna，gson → com.google.errorprone.annotations。

## 三、终态设计

### 3.1 切分单位与命名（问题 1）

**切分单位是模块**：一个 Java 模块对应一个库 crate，不按 Maven artifact 切。理由：
- 模块是 Java 自己的命名空间，与 D3（JDK 按 jmod 模块命名）一致；
- 模块名不含版本，用户代码里的路径不随升级变化；
- 同一个 artifact 一般就是一个模块，例外见下文的合并与拆分。

**模块名的确定**（`resolve::modules` 扩展，全部从 jar 本身和依赖锁动态读取，生成器里没有字面量）：

1. 按构建单元的目标 release `R`（等于 JDK 主版本）读取 jar 视图：多版本 jar 中 `META-INF/versions/N/`（N ≤ R，取最大的 N）的条目覆盖基础条目，这条规则对类和 `module-info.class` 同样适用；`META-INF/versions/` 下的条目不再以「类名」形式进入索引；
2. 视图里有 `module-info.class` → 具名模块，requires / exports / opens / provides / uses 都取描述符；
3. 否则，清单有 `Automatic-Module-Name` → 自动模块；
4. 否则按 JPMS 由 jar 文件名推导 → 自动模块（M1 已实现）；
5. 推导结果为空，或不是合法的 Java 限定名 → 用依赖锁中的坐标 `artifactId` 推导；再取不到就报错，要求在依赖锁里给出 `module` 字段。这样不再存在「无名的库」：无名模块只给用户类。

**crate 名**
- 模块名中的 `.` 换成 `_`。如果结果是 Rust 关键字，末尾加 `_`。
- 两个不同的模块名映射到同一个 crate 名（例如 `a.b_c` 与 `a_b.c`）时：按名序靠后的那个加 `_<模块名 FNV 的 4 位十六进制>`。规则确定，与机器无关。
- 库模块名与 JDK 镜像中的模块重名时报错（例如镜像里已有该模块，库又带来同名模块）。库的模块名以 `java.` 开头但 JDK 镜像里没有（例如 jakarta.transaction-api 的 `java.transaction`）时，crate 名照常为 `java_transaction`：类路径模式下 JPMS 不检查这个名字，名字只用于构建组织。

**一个模块由哪些类组成**（类路径语义）
- **JDK 包遮蔽**：库或用户档案中的类，如果所在的包属于 JDK 镜像中的某个具名模块，一律不进入该库（JPMS：这样的包总由所属模块提供，类路径上的副本不可见），并报告「被 JDK 遮蔽」。今天的类路径顺序 user → lib → jdk 在这一点上与 JVM 相反，必须修。
- **重复类**：同名类出现在多个 jar 中时，类路径顺序在前的 jar 胜出，其余副本丢弃并报告。类路径顺序因此是构建单元输入的一部分（§3.2 依赖锁）。
- **重名模块**（两个 jar 得到同一个模块名）：两个 jar 合成一个模块（类取并集，重复类按上一条处理），并给出警告。具名模块重名直接报错（JPMS 同样拒绝）。
- **分裂包**（同一个包出现在多个模块中）：
  - 类路径模式下，包私有访问在同一个运行期包内合法，跨 crate 无法保证 Rust 侧可见；所以把共享同一个包的模块并入同一个强连通分量的合并 crate（§3.3），与环的处理机制相同；
  - 两个具名模块之间的分裂包按 JPMS 报错；
  - 用户类与库类同包（类路径下合法）时：库 crate 中被用户类访问的包私有成员必须跨 crate 可见。生成层对 Java 访问控制的表达见 §八 待核第 6 项。

**shade / fat jar**
- 带 relocation 的 shade jar 只是一个普通 jar，包名已经改过，与原库不冲突，作为一个模块处理。
- 不做 relocation 的 fat jar（uber-jar），会和单独出现的瘦 jar 产生重复类，按上面的重复类规则处理。它自身作为一个模块，超过阈值时按 §3.4 拆层。
- 嵌套 jar（`BOOT-INF/lib/*.jar` 一类，依赖专用的启动类加载器）：由构建工具展开为类路径输入，rava 不解析嵌套 jar。

### 3.2 构建单元、依赖锁与档案切片（问题 2）

**构建单元的输入**新增一份**依赖锁**（`deps.lock.toml`，由构建工具插件或 `scripts/fetch_pilot_deps.sh` 生成，rava 只读）：

```toml
release = 21
[[jar]]                       # 按类路径顺序列出
coordinate = "com.google.code.gson:gson:2.14.0"   # 可缺省（Gradle 产物、手工 jar）
path = "pilot-libs/gson-2.14.0.jar"
sha256 = "…"
# module = "…"                # 只在 §3.1 第 5 步取不到名字时需要
```

- 每个入口声明自己的类路径，写成锁里条目的子集，顺序保持锁的顺序。
- 一个构建单元里，**同一个模块只能有一个版本**（Maven / Gradle 的版本仲裁结果就是这样，语料的 `pom.xml` 也是钉死版本的）。两个入口需要同一模块的不同版本时，它们属于不同的构建单元（§七 决策 4）。

**档案仍是一次多根开放世界分析**（cross-test §6.1，不变）：
- 入口：生产模式是用户项目的全部 `main` 与种子；语料模式是全体测试的 `main` 与种子；
- 库类在各入口的单例分析中与 JDK 类一样属于非用户侧，档案取各入口的并集。

**切片**：`profile.json` 中按模块切出的那一部分事实。
- `slice(c)` 只包含属主是 crate `c` 中的类的事实：类与层级、方法与种类、方法内的折叠、集合字段中属于 `c` 的项（instantiated、clinit、dispatched、reflect、seeds、`sam_types`、模块服务、模块资源），以及 `c` 的方法发出的引用。
- 谁贡献了这些事实不重要：用户 lambda 指向库接口、下游调用库方法，都会记进被调方的切片。

**生成的不变式（下游无关性）**：

> crate `c` 的生成文本 = f(生成器版本, runtime 中与 `c` 相关的清单, `slice(c)`, `c` 的各上游 crate 的键)。

- 换句话说，下游 crate 与用户 crate 的任何事实，只能通过「改变 `slice(c)`」影响 `c`。
- 1b 已经对 JDK 侧整体做到了「与用户程序无关」，本文把同一要求细化到每个 crate。
- 必须逐项排查的泄漏点与 1b §6.4 相同：
  - vtable 槽裁剪：槽计划只看本 crate 与上游，按开放世界保留可能被覆盖的槽；
  - `I__Lambda` 合成集：只取 `sam_types ∩ c`；
  - 继承成员需求：只由 `c` 自己的事实登记；
  - 导入与短名认领：M1 的可读性过滤推广到库，只认领可读模块中的类；
  - 元数据表与登记：每个 crate 只登记自己的行（M3）。

**开放世界的范围**：依赖实例化集合的折叠，只对「`c` 外面的代码无法扩展」的类型做。这里「外面」包括下游库和用户，而不只是用户。具体按 `open_world.rs` 的现有判据：
- 接口；非 final 且有 public / protected 构造器的类；sealed 类型看许可子类；
- 类路径模式下，库的非公开类型也按可扩展处理。

这样，`c` 的折叠不依赖哪些下游库存在，同一个库在不同构建单元里的折叠一致。

**各层键（Merkle，逐 crate 计算）**

| 键 | 输入 | 用途 |
|---|---|---|
| `P`（构建单元档案键） | 不变：cross-test §6.1 | 分析缓存、覆盖判定（`--covers`） |
| `K(c)`（crate 源码键） | 档案格式、生成器摘要、`c` 相关的 runtime 清单摘要、JDK 主版本与镜像摘要、`slice(c)` 摘要、各上游 crate 的 `K`；**库 crate 另加**：模块名、jar 的 sha256（合并模块为各 jar sha256 的有序列表）、坐标（含版本，缺省时为空）、目标 release `R`、启动模式（§3.3） | `S(c)` 的前提；坐标只用于索引与诊断，**jar 的 sha256 才是身份**（快照版本、重新打包的 jar 坐标不变，内容会变） |
| `S(c)` | `c` 生成树的内容摘要（含 Cargo.toml） | 两次生成确定性闸门 |
| `B(c)`（crate 产物键） | `S(c)`、各上游 crate 的 `B`、`rustc -vV`、target、模式（corpus / production）、panic、opt / debuginfo / codegen-units / embed-bitcode / LTO、crate-type、链接参数、remap 规则 | 产物缓存与分发 |

- 上游 `B` 必须进入 `B(c)`：上游重编后元数据的 SVH 改变，rustc 会拒绝旧的下游产物（E0460）。所以 JDK 档案的任何变化都会让库 crate 重编。库 crate 体量小，这个代价可以接受。反过来，库的变化不会传到 JDK 的 `B`。
- 1b 的「全部档案 crate 共用一个包版本」改为每个 crate 各取 `0.0.<S(c) 的 FNV>`。
- 档案的产物身份由各 crate 的 `B(c)` 拓扑排序后组成的清单给出（§3.5 `manifest.json`），不再是一个整体的 `B`。

**在开放世界下跨项目共享，靠的是什么**
- 两个构建单元（两个用户项目，或语料与某个项目）只要某个库的 `slice(c)` 和上游键相同，`K(c)`、`S(c)`、`B(c)` 就相同，产物可以直接复用。
- 库 crate 的上游是 JDK 模块 crate，而生产模式下每个项目的 JDK 档案就是它自己的闭包，两个项目的 `java_base` 一般不同，所以**跨项目的命中主要发生在同一个项目的反复构建之间**。真正的跨项目共享有两种做法，见 §七 决策 1：
  - 多个项目组成一个构建单元（团队 / 单仓），推荐；
  - 按库的全部公开 API 预先建档，不推荐作为缺省。

### 3.3 依赖图与 crate DAG（问题 3）

**crate 依赖边 = 生成代码实际引用到的模块**，与 M2 对 JDK 模块的做法相同：
- 具名库模块：边必须落在 requires 传递闭包内（运行期 requires ∪ `requires static`），越界即生成失败。`requires static` 指向的模块不在类路径上时，引用它的方法在各入口分析中本来就到不了；
- 自动模块：可读性是「读全部」，边就取实际引用；
- Maven 依赖图**不决定 crate 边**，只用来做两件事：
  - 坐标与版本；
  - 一致性告警：crate 边不在 Maven 传递闭包内，说明是未声明的依赖，常见于 optional / provided；Maven 声明了但没有实际引用的依赖，不产生边。

**拓扑关系**
```
JDK 模块 crate（java_base、java_logging、jdk_unsupported …，按 requires DAG）
   ↑
库 crate（按实际引用 DAG；环合并为一个分量）
   ↑
user（bin）
```
- 库 → JDK 的边，指向库实际引用到的 JDK 模块 crate。例如 guava → `java_base`、`java_logging`、`jdk_unsupported`。
- JDK → 库的链接边恒为 0：JDK 使用库实现（ServiceLoader、JCA provider、反射）一律走运行期表（t1-link R2 / R3）。

**环（强连通分量）**
- 来源：
  - 自动模块之间的双向引用（slf4j 1.x 的 `LoggerFactory` 静态引用 `org.slf4j.impl.StaticLoggerBinder`，而这个类由依赖 slf4j-api 的绑定 jar 提供）；
  - optional 依赖造成的互引；
  - 分裂包（§3.1）。
- 具名模块之间不会成环（JPMS 拒绝 requires 环）。
- 处理：
  1. 按 crate 依赖图求强连通分量。非平凡分量合并为一个**分量 crate**，名字 = 分量内名序最小的模块 crate 名加 `_scc`，如 `ch_qos_logback_classic_scc`；
  2. 每个成员模块保留一个同名的**门面 crate**（`org_slf4j`、`ch_qos_logback_classic`），只按包 `pub use` 分量 crate 中本模块的包，同时导出 `__rava_register_module()`；
  3. 下游与用户只引用门面 crate，所以用户代码里的路径与是否合并无关。这与 `java_base` 重导出 `java_base_decl` 的做法一致（D6）；
  4. 分量 crate 超过阈值时按 §3.4 拆层。
- 环在不在，取决于档案内容（环上的方法是否被翻译），所以 crate 布局随切片变化。但门面名稳定，`K` 已经包含切片，不影响缓存的正确性。

**库引用下游或用户提供的类**
- 场景：类路径上下游 jar 或用户提供某个类，库按名字静态引用它。slf4j 1.x 的绑定类如果由用户代码提供，就是这种情况。
- 若由下游库提供：那一定与该下游库成环，已按上一条合并。
- 若由用户提供：库 crate **不得**依赖用户 crate（否则库 crate 不能共享）。这样的引用走**晚绑定**：
  - 库侧在引用点生成按「类名 + 成员」查运行期登记表的调用；
  - 提供方的 crate 在 `__rava_register_module()`（用户侧是 `register_user`）里登记函数指针；
  - 没有登记时抛 `NoClassDefFoundError`，与 JVM 的惰性解析一致。
- 这类引用由分析器识别，计入 `slice(c)`，raw-audit 计数 `[lib-late-bind] n=…`。

**库对 JDK 的访问**（JPMS 强封装，JDK 17 起缺省开启）

| 访问 | JVM 行为 | rava |
|---|---|---|
| 已导出的包，含 jdk.unsupported 的 `sun.misc`、jdk.net | 正常 | 普通 crate 边，指向该 JDK 模块 crate |
| 静态链接到未导出的包（针对旧 JDK 编译的库） | 解析时抛 `IllegalAccessError`，除非启动时给了 `--add-exports` | 分析器按 JDK 模块描述符的 exports 加启动选项静态判定；拒绝时该指令折叠为抛 `IllegalAccessError`，不生成对那个类的引用、没有 crate 边；允许时为普通边 |
| 深反射（`setAccessible`、`privateLookupIn`） | 抛 `InaccessibleObjectException`，除非给了 `--add-opens` | 运行期由 `Module.isOpen` 按字节码回答，导出 / 开放表由引导层建立（[`2026-10-02-boot-layer.md`](2026-10-02-boot-layer.md)） |

**启动选项**（`--add-opens` / `--add-exports` / `--add-reads`，以及选择类路径还是模块路径的启动模式）
- 它们是**每个入口**的输入，在 JDK 中对应 `jdk.module.addopens.N` 等系统属性。
- 在档案上按折叠的并处理：全部入口取值相同时照常折叠（缺省全为 null，与引导层方案一致）；不同时不折叠，由各程序在启动时把自己的取值交给 VM 引导。
- `profile.rs` 今天在系统属性表不一致时直接报错，本文要求它把启动选项这一族改为上述并语义。

**启动模式**
- **类路径模式（缺省，对应 `java -cp`）**：全部库类与用户类在同一个无名模块中，`Class.getModule()` 返回无名模块，服务发现按 `META-INF/services`。
- **模块路径模式（对应 `java -p`）**：库按描述符定义为引导层的具名模块，服务发现按 `provides`。
- crate 的切分在两种模式下相同（只是构建组织），区别只在运行期的模块语义和开放世界判据（模块路径下，具名模块的非公开类型不可扩展）。所以启动模式进入 `K`。

**链接集与登记集**
- 程序的**链接集** = 全部 JDK 档案模块 crate，加上本入口类路径上库的 crate 依赖闭包。闭包中可能包含不在本入口类路径上的库，原因是档案并集里的 `requires static` 或 optional 引用。
- 程序的**登记集** = 全部 JDK 档案模块，加上本入口类路径上的库，按拓扑序逐个调用 `__rava_register_module()`。只有登记过的库，类查找、`Class.forName`、ServiceLoader、资源查找、反射元数据才能看到。
- 链接了但没登记的库（这种库只会作为 optional 依赖进入链接集），它的类第一次被主动使用时抛 `NoClassDefFoundError`，与 JVM 上「类路径没有这个 jar」的行为一致。这个检查挂在类初始化屏障上，只对库 crate 的类生效。

### 3.4 大库内部拆分（问题 4）

沿用 D6 与 crate-split §7.5.5，规则一字不改，只是作用对象从 JDK 模块推广到库 crate（含分量 crate）：
- **拆层判据**：crate 全部生成块文本 ≥ `BODY_CRATE_BYTES`（5 MB）时，拆成 `<lib>_decl` + `<lib>_body_1..N`；不到阈值的单 crate 完整展开，没有 `rava_layer`，也没有 `export_name` 外壳；
- **装箱**：类按 binary name 排序，按文本字节均衡装箱，箱数 = max(⌈体字节 / 5 MB⌉, 2)；
- **命名**：`<lib>` 是对外的模块 crate，`pub use <lib>_decl::*` 并静态吸收各实现层，声明层 ↔ 实现层的符号环闭合在 `<lib>` 的产物内部。下游与用户只看到 `<lib>`；
- **确定性**：拆层与装箱只依赖 `slice(c)` 生成的文本，同一个 `K(c)` 在任何机器上都得到同一布局。不按本机内存调整拆分数，按资源调整的只有并行度（`CARGO_BUILD_JOBS` / 档案构建的 rustc 并发数），即设计结论 1；
- **链接符号**：`__rava_<二进制名转义>__<函数>_<指纹>` 以类的 binary name 为前缀。一个构建单元里，同名类在类路径上只有一份（§3.1 重复类规则），同一个模块也只有一个版本（§3.2），所以库 crate 之间、库与 JDK 之间都不会撞符号。

哪些库会越过阈值，要看切片体量，见 §八 待验证第 1 项。按全 jar 的类数看，候选是 guava（1,965 类）、eclipse-collections（3,831）、commons-math3（1,301）、jackson-databind（813）、byte-buddy（3,047 个非版本化类）；但切片只含档案可达的方法，多数会远小于全 jar。

### 3.5 两种模式的落地差异与缓存（问题 5）

| | 语料模式 | 生产模式 |
|---|---|---|
| 构建单元 | e2e 全集（含 `63_junit` 等库形态目录），依赖锁来自 `tests/lib_pilot/deps/pom.xml` | 一个用户项目（或决策 1 中的多项目单元），依赖锁来自构建工具插件 |
| 库 crate 产物 | 每个库 crate 一个 dylib（拆层的库是一个 dylib，内部吸收 decl / body），依赖上游 dylib；panic=unwind；install_name / rpath 规则同 t1-link §3.4 | rlib；panic=abort；开发构建不开 LTO，`--release` 用 fat LTO、cgu 1、opt 3（D2），`embed-bitcode=yes`，所以 release 的 `B` 与开发构建不同 |
| 单例链接 | `--extern` 链接集里的每个 crate（JDK 模块 + 本入口的库闭包），`-C prefer-dynamic` | `--extern` 同一链接集的 rlib，静态链接，`-dead_strip` / `--gc-sections` |
| 用户 `main.rs` | 按拓扑序逐个调用登记集里的 `<m>::__rava_register_module()`，然后 `register_user` 并启动（t1-link §3.3） | 同左 |
| 分发 | 调度方的 store 按 crate 存：`store/crates/<target>/<B(c)>.tar.zst`；档案清单列出各 crate 的 `B`；服务器只拉本机缺的 crate | 本机 `~/.cache/rava/crates/<target>/<B(c)>/`；可选配置远端 store（决策 6） |

**本机布局（两种模式通用）**
```
~/.cache/rava/crates/<target>/<B(c)>/        每个 crate 一个目录：dylib 或 rlib、rmeta、crate 清单（K、S、B、上游 B、sha256）
~/.cache/rava/archive/jdk<M>/<A>/lib/        某次档案的链接目录：指向上面各 crate 产物的硬链接（不能硬链接时复制），加 std dylib 副本
~/.cache/rava/archive/jdk<M>/<A>/manifest.json
```
- `A` = 档案链接集清单的摘要，即拓扑序的 (crate, `B(c)`) 列表。可执行文件的 rpath 指向 `<A>/lib/`；dylib 之间用 `@loader_path` / `$ORIGIN` 互相找到，所以一次档案的全部 dylib 必须在同一目录，由链接目录保证。
- 这里修订了 t1-link §3.4 的「每档案一个 `B` 目录」：产物按 crate 存放，档案只是链接视图。安装、flock、`manifest.json` 最后写入并改名生效等规则不变，改为对每个 crate 和每个链接目录分别执行。

**失效规则**（由键自动推出，不需要另写判断逻辑）

| 变化 | 受影响的 crate |
|---|---|
| 只改用户代码，没有新到达非用户方法 | 无（只编用户 crate） |
| 用户代码新到达库方法 | 该库 `K` 变 → 该库及其下游库重编；JDK 不变 |
| 用户代码或库新到达 JDK 方法 | 对应 JDK 模块及其下游（含全部依赖它的库）重编 |
| 库升级或 jar 内容变化（sha256 变） | 该库及下游重编；JDK 切片不变时 JDK 不重编 |
| 依赖锁增删库，或改类路径顺序 | 受影响的库；重复类的胜出方变化时，切片随之变化 |
| 生成器、runtime 清单变化 | 生成器变化：全部；某库专属清单变化：只有该库及下游 |
| 工具链或模式参数变化 | 全部的 `B`（`K` / `S` 不变，生成可以复用） |

**GC**：crate 目录按链接目录引用计数，被最近 N 个档案链接目录引用的 crate 保留（N 沿用 cross-test §4.6 的 4 个键 / 8 个键），其余回收。回退链（本机命中 → 拉取 → 本机构建）沿用 t1-link §4.6，粒度改为单个 crate。

### 3.6 库专属的手写与种子

- **手写边界不变**（CLAUDE.md 第 1 条）：库的 `ACC_NATIVE` 方法（例如 commons-daemon 的 JNI、JNA 绑定）按第 ① 类准入手写。没有其他准入理由。
- **位置**：`runtime/lib_runtime/<模块名>/`，与 `runtime/java_runtime/` 对称：
  - `closure.toml` / `seeds.toml` 只写本库的边界、补种与手写登记；
  - `src/<pkg>/<x>_impl.rs` 在生成时 overlay 进该库 crate，与 `<x>.rs` 共置（第 3 条）；
  - 清单按模块名匹配，带 `versions = "[a,b)"` 区间，命中多个区间时报错。
- **种子**：库的可达性只由入口决定。`--lib … :seed=` 一类的整包或子集种子退出构建单元语义。
  - 反射、注解驱动的入口（例如 JUnit 发现 `@Test` 时由框架反射调用用户方法，gson / jackson 按字段反射）由分析器的反射建模与 `seeds.toml` 补种覆盖；
  - crate 验收（lib_pilot golden）需要整包翻译时，在验收的构建单元里以 `--seed-class` 入口显式给出。
- 库专属清单的摘要只进入该库的 `K`，改动不波及 JDK。

## 四、与现有实现的接口（问题 6）

### 4.1 `resolve::modules` 扩展点

| 扩展 | 位置 | 内容 |
|---|---|---|
| 多版本 jar 视图 | `classfile/src/archive.rs` | `Archive::open(path, release)`：`META-INF/versions/N/`（N ≤ release）覆盖基础条目；版本化的 `module-info.class` 作为根描述符；`META-INF/versions/` 不再进入类索引 |
| JDK 包遮蔽 | `resolve/src/classpath.rs` | 建索引时，`Origin::User` / `Origin::Lib` 中属于 JDK 镜像具名模块包的类不入索引，记入 `shadowed`（可观测） |
| 库模块命名兜底与冲突 | `resolve/src/modules.rs` | §3.1 第 5 步（依赖锁坐标）、crate 名冲突后缀、重名模块合并、具名模块重名与分裂包报错；`ModuleNode` 增加 `kind: Jdk | Lib`、`jars: Vec<(path, sha256)>`、`coordinate`、`exports` / `opens`（访问判定用） |
| 包归属 | 同上 | `package_owner` 改为对全部具名与自动库模块建立；分裂包在这里检测，交给 crate 计划做分量合并 |
| crate 计划 | 新 `resolve/src/crates.rs`（或 emit 侧 `project/crate_plan.rs`） | 输入：档案的模块集与实际引用边；输出：crate 列表（模块 crate / 门面 / 分量 / decl / body_k）、依赖边、拓扑序。JDK 与库走同一条代码路径，M2 的模块切分就是它在「只有 JDK」时的特例 |
| 访问判定 | `closure` 引擎 | 库 → JDK 未导出包的静态链接按 exports 加启动选项判定，拒绝时折叠为抛 `IllegalAccessError`；库 → 用户提供类的晚绑定识别 |

`lib_crates.rs` 的「命令行声明序」与 `CrateRoute` 由 crate 计划取代：`target(bin)` 改为「类 → 模块 → crate」，`reachable` 改为「crate 依赖闭包」。

### 4.2 `profile.json` 增量

- `modules` 段每行增加 `kind`、`crate`（所属 crate；门面或分量成员另注 `scc`）、`jars[{path, sha256, coordinate}]`、`release`、`slice_digest`、`key`（`K(c)`）。
- `entries[]` 每项增加 `classpath`（依赖锁条目名，按顺序）与 `launch`（启动模式与启动选项）。
- `profile.inputs` 增加 `deps_lock` 摘要。`P` 的输入不变，库 jar 的内容摘要已经计入。

### 4.3 命令面

| 命令 | 终态 |
|---|---|
| `rava profile` | 新增 `--deps <deps.lock.toml>`；入口行用 `--cp <锁条目名>[,…]` 声明类路径，`--launch "<启动选项>"` 声明启动选项；`--lib NAME=JAR[:seed=]` 删除（crate 名不再由用户给出，种子见 §3.6） |
| `rava build <Main.java>` | `--deps` / `--cp` 同上；单例生产构建时，构建单元 = 本程序 |
| `rava archive build <profile.json> [--mode corpus\|production] [--release]` | 按 crate 拓扑序，对每个 crate 查 `B(c)`：命中就跳过，未命中就生成并编译；然后组装链接目录 `<A>` |
| `rava archive explain <profile.json> [--since <旧 manifest>]` | 列出每个 crate 的 `K` / `S` / `B` 与命中状态；带 `--since` 时逐 crate 给出键变化的原因（切片、jar、上游、工具链……）。这是失效排查的唯一入口 |
| `rava compile` | 按 `manifest.json` 的链接集与登记集拼 rustc 参数（t1-link §3.6），对库没有特判 |

### 4.4 lib_pilot 语料作为验收

- **构建单元 L-a**：m1–m5 五个 main，依赖锁 = junit 4.13.2 + hamcrest 3.0。预期 crate：JDK 模块 crate，加 `org_hamcrest`、`junit`（junit 是自动模块，名字来自 `Automatic-Module-Name`）。
- **构建单元 L-b**（类别覆盖）：commons-lang3、gson、jackson-core / databind / annotations、guava 及其传递依赖，各配一个小 main。覆盖：多版本 jar 描述符、具名模块 requires 校验、open module、`jdk.unsupported` 边、多 jar 的传递 DAG。
- **构建单元 L-c**（环与晚绑定）：slf4j-api 1.7.36 + 一个 1.x 绑定 jar（成环），另一入口只带 slf4j-api 1.7.36 并由用户提供 `StaticLoggerBinder`（晚绑定）。另外用合成 jar 夹具覆盖分裂包、重名模块、JDK 包遮蔽、重复类，写在生成器单元测试里。
- golden 对账沿用 `scripts/lib_pilot_golden.sh` 的口径（JVM 真 jar 对翻译产物，逐字比较），e2e 回归走 `63_junit` 形态目录（junit-crate-as-test-harness 步骤 A）。

## 五、分步实施（每步可独立验收）

| 步 | 内容 | 依赖 | 验收（量化） |
|---|---|---|---|
| V12-0 | 类路径与模块归属修正：多版本 jar 视图、JDK 包遮蔽、库模块命名兜底、crate 名冲突规则、重名模块合并、分裂包 / 重复类检测；`ModuleNode` 新字段 | 无（只动 classfile / resolve，可先于 M2 合入） | 52 个 pilot jar 的模块名与 `jar --describe-module --release 21`（具名）/ `Automatic-Module-Name` / JPMS 推导逐一相符：**52 / 52**（今天名字错 18、描述符丢失 30）；`META-INF/versions/` 形态的类名入索引 **0**（今天 3,078）；合成夹具 6 例（分裂包、重名具名模块、重名自动模块、JDK 包遮蔽、重复类、多版本覆盖）全过；`no_jdk_literals` 通过；m1–m5 生成正文不变 |
| V12-1 | 依赖锁与入口类路径：`--deps` / `--cp` / `--launch`，`profile.json` 增量（§4.2），删除 `--lib`；`fetch_pilot_deps.sh` 输出 `deps.lock.toml`（含 sha256 与坐标）；`runtime/lib_runtime/` 清单读取 | V12-0 | L-a 档案：入口顺序、锁内条目顺序不变时，`P` 不随入口给出顺序变化；`--covers` 对五个 main 都成立；档案类集合 = 五个单例非用户侧之并 |
| V12-2 | crate 计划（与 M2 合并实施）：库 crate 命名、依赖 = 实际引用、具名模块越界即失败、分量合并 + 门面、D6 拆层推广到库；`CrateRoute` 由 crate 计划取代 | V12-0、M2 | L-a、L-b 生成并 cargo 构建通过；`[module-audit] out_of_reads=0`（含库）；库 → 用户 / 下游的链接边 **0**；L-c 夹具生成 1 个分量 crate 与 2 个门面，用户路径与无环时逐字节相同；服务器抽查 m1–m5 golden 不变 |
| V12-3 | 切片与逐 crate 键（与 M3 合并实施）：`slice(c)`、`K(c)` / `S(c)` / 每 crate 包版本；下游无关性守护测试；登记集按入口类路径；未登记库的 `NoClassDefFoundError` 屏障；晚绑定登记表 | V12-2、M3 | ① L-a 增加一个只改用户侧的入口后，`org_hamcrest`、`junit` 与全部 JDK crate 的 `S` 不变（字节差 0）；② L-b 中把某个库换成内容不同但不新增 JDK 方法的版本后，JDK crate 的 `S` 不变；③ 守护测试：给构建单元加一个下游库后，上游各 crate 的字节差为 0（上游切片不变时）；④ L-c 未登记库的类首次使用时的输出与 JVM 一致 |
| V12-4 | 产物按 crate 存放（与 L2 合并实施）：`~/.cache/rava/crates/`、链接目录 `<A>`、`rava archive build` / `explain`、逐 crate 回退与 GC；release 模式 `embed-bitcode` / fat LTO | V12-3、L2 | 只改库切片后重建：JDK crate 重编 **0**、拉取 **0** 字节；只改用户代码：rustc 只编用户 crate（库 0、JDK 0）；服务器上每个库 `.so` 的 `NEEDED` 恰为其 crate 依赖 + std，`RUNPATH=$ORIGIN`；`explain --since` 对 ①② 两种变化分别报出正确原因 |
| V12-5 | 语料接入：`63_junit` 形态经 `rava profile --deps` 进入全集档案；pilot 阶梯（梯队 1）逐个进入语料构建单元 | V12-4、K14（scripts-into-rava S6 / S7） | 全集档案加入库后，单例 p50 ≤ 1 s、p99 ≤ 3 s、峰值 ≤ 0.8 GB 不变差（t1-link §六）；库 crate 单 crate 编译峰值 ≤ 1.5 GB；`lib_pilot_golden.sh m1..m5` 零回归；e2e 抽样 ≥ 10 例零回归 |

- V12-0 与 V12-1 不依赖 M2，可以先做；V12-2 / 3 / 4 分别与 M2 / M3 / L2 合并实施，不另起一条 crate 布局线。
- 每一步都只扩展 JDK 模块切分已有的代码路径（类 → 模块 → crate），不为库另写一套。

## 六、量化目标（终态）

| 项 | 目标 | 今天 |
|---|---:|---:|
| pilot jar 模块名 / 描述符正确 | 52 / 52 | 名字对 34、描述符对 22 |
| 版本化条目以错误类名入索引 | 0 | 3,078 |
| 库 → 用户 / 下游的链接边；JDK → 库的链接边 | 0 / 0 | 无此口径（库依赖靠命令行声明序） |
| 只改用户代码时库与 JDK 的重编 | 0 / 0 | 每测试现场生成并编译库 crate |
| 只改库切片（JDK 切片不变）时 JDK 的重编与拉取 | 0 crate / 0 字节 | 整个档案失效（单一 `P` / `B`） |
| 同一 `slice(c)` 与上游键下，库 crate 的跨构建单元字节差 | 0 | —— |
| 带库的单例编译 + 链接 | p50 ≤ 1 s，p99 ≤ 3 s，峰值 ≤ 0.8 GB | 同 t1-link 目标 |
| 库 crate（含 `_decl` / `_body_k`）单 crate 编译峰值 | ≤ 1.5 GB | —— |
| 生成器中库名 / JDK 类名字面量 | 0 | 0（守护不变） |

## 七、待用户决策

1. **库档案的粒度：跨项目共享靠什么**
   - A（推荐）：构建单元切片。库 crate 的内容由构建单元的入口决定；跨项目共享靠把多个项目组成一个构建单元（团队 / 单仓），或者靠切片碰巧相同时的内容寻址命中。符合第 2 条「档案 = 构建单元调用链并集」与「正确且最小」。
   - B：按库的全部公开 API 预先建档（入口 = 库导出包中全部 public / protected 方法），产物与使用方无关，可以像 Maven 制品一样发布。代价：JDK 档案随之膨胀到库全 API 的闭包，guava 一类库的全 API 闭包会把 JDK 档案推大到数千类，违背「最小」；而且库的上游 JDK 档案也要按同一口径预建。
   - C：A 为缺省，B 只作为发布形态（需要时单独立项）。
2. **成环时的命名**
   - A（推荐）：内部分量 crate `<最小模块 crate 名>_scc`，加每个成员的同名门面 crate；用户路径稳定。
   - B：只有分量 crate，用户路径直接写分量 crate 名；名字随环的有无变化。
3. **启动模式**
   - A（推荐）：两种都支持，缺省为类路径模式（与绝大多数 `java -cp` 部署一致），模块路径模式作为构建单元选项进入 `K`。
   - B：只支持类路径模式。
4. **同一构建单元内同一模块多版本**
   - A（推荐）：单版本锁（与 Maven / Gradle 仲裁一致）；需要不同版本的入口放进不同的构建单元。
   - B：按 (模块, jar sha256) 给库类分命名空间，多个版本并存于一个档案。它要求分析器把库类像用户类一样按入口隔离，并入时再按实例合并，复杂度高，收益只在语料。
5. **库专属手写与清单的位置**
   - A（推荐）：`runtime/lib_runtime/<模块名>/`（清单 + `<x>_impl.rs`，带版本区间），只进该库的 `K`。
   - B：并入 `runtime/java_runtime/` 的三份清单。改动会让全部 JDK crate 的键失效。
6. **生产模式的远端共享缓存**
   - A（推荐）：逐 crate 内容寻址的 store 本机缺省开启，远端 store（团队共享）为可选配置，协议沿用 cross-test §4 的分块校验。
   - B：只用本机缓存。
7. **修订 t1-link 的产物布局**：档案产物键从「每档案一个 `B`」改为「每 crate 一个 `B(c)`，档案 = 链接目录」（§3.5）。推荐采纳：库与 JDK 分开失效，正是这一点让「只改库时 JDK 重编为 0」成立。L2 尚未实施，不产生返工。

## 八、待验证与待核（本文没有运行编译 / 测试）

1. **库切片体量**：在 L-a / L-b 档案上，统计每个库 crate 的生成文本字节，确认哪些越过 5 MB 阈值，以及各自的拆层数与编译峰值。
2. **真实类路径中环的频率**：在服务器的 92 个 jar 加 framework 矩阵构建单元上求分量，统计非平凡分量的个数与成员。
3. **dylib 个数对单例的影响**：一个入口链接 15 个 JDK 模块，再加 N 个库 dylib，测 N = 5 / 20 / 50 时 dyld / ld.so 的加载开销与单例链接时长（t1-link 结论：导出符号总量决定链接时长，dylib 个数影响小，待在库上复核）。
4. **硬链接链接目录**：服务器上 `~/.cache` 与作业目录是否同一文件系统；不能硬链接时，复制带来的磁盘与耗时。
5. **`NoClassDefFoundError` 屏障的开销**：只挂在库 crate 类的初始化屏障上，在 L-b 档案上测热路径开销（预期是每个类初始化一次原子读）。
6. **生成层的 Java 访问控制表达（待核）**：lib crate 的 pub 面今天由 access_flags 驱动（junit-crate-as-test-harness §一）。需核对包私有与 protected 成员在库 crate 中的 Rust 可见性；如果是 `pub(crate)`，用户类与库同包访问包私有成员（类路径下合法）会编译失败。终态要求：跨 crate 可见性不低于 JVM 运行期允许的访问，Java 访问控制由 javac 与分析器保证，不借 Rust 可见性表达。
7. **fat LTO 与多 rlib**：生产 `--release` 下带 10 个以上库 rlib 时，首构与只改用户代码的耗时（t1-link 的 465 类档案实测只改用户代码为 47 s）。
8. **启动选项的并语义**：`profile.rs` 的系统属性表从「不一致即报错」改为「启动选项族不一致即不折叠」后，引导层路径的翻译增量（类 / 方法数）。
