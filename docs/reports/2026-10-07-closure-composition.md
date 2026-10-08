# 闭包构成分析：按机制拆分、逐块处理方式与可减类数（2026-10-07，分支 closure-composition）

> 任务：把闭包按机制拆块，逐块回答三问并给出可减类数，形成收窄任务清单（本文只分析，不实施）。
> 三问：① 该机制在 rava 模型（封闭映像、无运行期类加载、无 GC、引导映像构建期求值）下是否成立——不成立则整体替换（手写准入 ② / ③）；
> ② 能否构建期求值（配置、提供者、系统属性、字符集、locale）——能则构建期求值；③ 否则按字节码翻译，接受依赖，查闭包精度缺口。
> 数据全部在云服务器以作业方式取得（本机不跑闭包）；分析脚本在本机读下载结果。

## 一、结论先行

1. **闭包不是按机制线性叠加的，而是一个共享的核心团块加少数几道「门」。**
   CollectorsDemo / DeepCopy / TestJcaSasl 三例共有 **2987 类**（三例并集 3328 类）。三例在核心团块之外各自只多出 3 / 288 / 52 类；HelloWorld 的 468 类中有 467 类也在团块内。
   团块内部是多连通的。单独切掉一个机制只少 0–180 类，切掉入口门则整块消失：
   CollectorsDemo 切掉「枚举 `values()` 反射调用」一处，类数从 2990 降到 528（−2462）。
   **这是切除上界，不是直连收益（2026-10-08 更正）。** enum-values-direct（59451c29，已合入 batch-1008）把该调用点改成直接调用后，
   HelloWorld 与 CollectorsDemo 实测都还是 **3011 类**：`Method.invoke` 仍经另外 4 个调用点入链
   （docs/plans/2026-10-08-enum-values-direct.md §六 / §八）：
   - `BasicImageReader$2.run@37`：经 `Random.<clinit>` 的 doPrivileged 扇出入链；
   - `ServiceLoader$ProviderImpl.invokeFactoryMethod@20`；
   - `AnnotationInvocationHandler.equalsImpl@121`；
   - `HostLocaleProviderAdapter.findInstalledProvider@39`。
   切除实验与直连差在多连通：切除挡住的是切点下游整条路，而直连只消掉一个入口。团块还有其余入口，只要有一个敞开，整块照旧进来。
   上界要兑现，须让 `Method.invoke` 的全部入口都不经原入口（见第 5 条）。
2. **按入口机制首达归属看：** 反射 1357（collectors）、序列化 2146（deepcopy）、安全 2197（jcasasl）。这只说明各例经哪扇门进入团块，团块本身的构成三例几乎相同。
   按自身包统计团块构成：安全 671、locale 310、集合 199、字符集 195、lang 190、NIO 152、invoke 148、并发 146、Stream 146、反射 143、时间 121、类加载 99、IO 95、日志 74。
3. **拿 S0 Boot 在 JVM 上实际加载的 JDK 类（1694）作对照：** 团块 2987 类中只有 1378 类被 Boot 实际用到，**1609 类连一个 Spring Boot 应用都不加载**。
   这 1609 类主要分布在安全（593）、locale（295）、字符集（173）三块，合计 1061 类。这三块都能构建期求值，属于问题 ②，正是收窄的主战场。
4. **按机制合并切除得到的上界：**

   | 例 | 基线 | 切后 | 切除集 |
   |---|---:|---:|---|
   | HelloWorld | 468 | 375 | allmech；加第二轮合并后 369 |
   | CollectorsDemo | 2990 | 503 | allmech |
   | TestJcaSasl | 3039 | 2325 | allmech，−714 |
   | DeepCopy | 3275 | **2111** | r2all，−1164 |

5. **推荐顺序**（详见 §七；2026-10-08 按直连实测调整，前两项前移）：
   1. `Method.invoke` 全部入口直连：枚举入口已合入；其余 4 个入口（第 1 条所列）按 enum-values-direct §八 的三种形状收口。
   2. doPrivileged 扇出精度：三例共用的门，又是 `BasicImageReader$2.run` 入口的来路；DeepCopy 合并切除到 2111。
   3. 「Lookup 访问校验折叠」，即 `SharedSecrets.ensureClassInitialized` 链。每个程序都受益，HelloWorld −82（旧基线）。
   4. JCA / 字符集 / 日志 / locale 四块的构建期求值。
   5. 类加载 / jar / 资源整体替换为封闭映像，即 boot-layer 步骤 1–5。
   6. 引用 / 信号按 ③ 收口。

## 二、方法与工具

| 文件 | 作用 |
|---|---|
| `scripts/closure_composition_job.sh` | 服务器作业：选定用例跑 `rava closure -o`，可带 `--cut-file` 或 `--cut-sets` 做反事实切除；产物写 `build/ccomp/<例>[.<切除集>].json.gz` / `.out` / `.err` |
| `scripts/closure_composition.py` | 本机分析（`uv run python`）：按 TOML 规则把类归入机制块，输出自身包、入口机制、最近机制、经过机制、根种类、入口边、JVM 类加载对照、反事实差分与 S0 面计数 |
| `scripts/closure_composition.toml` | 归类规则：27 块，最长前缀匹配；kind 分 mechanism / base / user。脚本里不写 JDK 类名特判 |
| `scripts/closure_composition_cuts/*.txt` | 切除集：每块一组「门」方法（体不处理，或 `@偏移` 跳过单个调用点）；`allmech` / `r2all` 为合并上界 |

**口径说明：**

- 入口归属取 via 首达链。类节点用「代表方法」替换：该类经直接调用进入闭包、原始 via 链最短的方法。dispatch 边与挂在类上的成员边（clinit、手写 Object 层的 reflect 成员边）不算原因。
  因此「入口机制 X 有 N 类」只表示首达路径先经过 X，**不等于去掉 X 能减 N 类**。可减数一律以反事实切除（`--diff`）为准。
- 反事实切除不健全：切掉的是门方法的分析，不是语义。它只用来回答「若该门在构建期被证明不可达或已求值，能少多少」。
  切写入点会让字段值集变空、按初值折叠，结果非单调（jarverify 组使 collectors +40、deepcopy +923）；切 assertion 则使分析停滞。两者都已在 §五 注明。
- JVM 对照取 S0 Boot 样例的 `-Xlog:class+load`（作业 apis0-kr2b-1fdb1bed），只统计 jrt / CDS 来源的 JDK 类；隐藏类与代理另计（510 + 281 + 44）。
  JVM 按需加载，这一列是「一个真实 Boot 应用运行期实际用到」的参考值，不是闭包下限。

### 门自动排名：`rava closure --gates`（2026-10-08 起取代手工切除集）

上面的手工流程是：读 §3.3 的首达链猜门 → 手写 `closure_composition_cuts/*.txt` → 作业逐组重跑 → `--diff` 对账。
它的三个弱点是：门靠人猜；多连通团块上的单门 Δ 都是 0，要靠人试组合；切到写入点 / 构造器会非单调（jarverify 组），要事后才发现。
`--gates` 把这条流程收进分析器，一次运行完成（实现：`generator/crates/closure/src/engine/gates*`，driver `gates_cmd.rs`）。

1. **候选**：沿每个类的首达溯源链（与 `--why` 同口径）上溯，统计每个节点下游经过的类数，取前 `pool` 个（缺省 max(4 × top, 40)）。
   只取消费型节点：方法体（非构造器 / 类初始化、体内无字段写入、不是根）与调用点（invokevirtual / invokeinterface / invokestatic / 非构造 invokespecial）。
   写入点和构造器不作候选，所以不会出现 jarverify 那样的非单调切除。
2. **模型量测**：基线运行同时登记全部触发边（方法源带调用点偏移）。在这张触发图上做「与」可达性：派发边以接收者实例化为条件，被切节点保留但不再触发出边，与引擎的切除同口径。
   在图上算出：
   - 每个候选的单切 Δ（类 / 方法 / 消失类的包分布）；
   - 贪心组合累计曲线：逐步取边际收益最大者。若全体单切都低于门槛（多连通），就沿模型首达树依次切剩余最大子树的入口，至多 4 步成段：段内累计达标就整段采纳，不达标就回退。
   - 同机制组合上界：同包，或消失类集合 Jaccard ≥ 0.5。
   模型是上近似：值流折叠不在图上。
3. **实测校准**：贪心前缀（第 1、2、4、8… 步与末步）、前 3 组、单切前几名，以 `--cut-file` 子进程重跑，与手工切除逐字节同口径。预算 `--gates-verify`，缺省 12。
   排名优先用实测单切 Δ；实测出现基线没有的类时标「非单调」。
   并行度取 `--gates-jobs`、CPU 数、内存预算（`--gates-mem-mb`，缺省 12288）÷ 基线峰值三者的最小值。超时（`--gates-timeout`，缺省 1800 s）的子进程会被终止，记为失败。
4. **类别**：按分析器事实与清单判定，不写类名特判。依次检查：
   1. 清单 `closure.toml [gates]` 条目（门所在类，其次调用目标类）；
   2. 首达链经 VM 驱动入口（vm-rule / vm-hook）或 VM 边界类 → **VM 驱动**；
   3. 首达链根为构建期种子（jca / locale / 注解 / …）、下游经服务 / 资源束查找首达、或门内读系统属性 → **可构建期求值**；
   4. 调用点派发目标 ≥ 枢纽阈值、开放枢纽、或下游经反射建模首达 → **精度缺口**；
   5. 只在类初始化链上 → 可构建期求值（弱）；
   6. 都不满足 → 精度缺口（按 ③ 查精度）。
   每条判定都带证据文字。**运行模型替换**目前只来自清单（类加载 / jar / 签名校验）。

输出：
- `--gates-out` 写机读 JSON：各门的 id（可直接放进 `--cut-file`）、单切 Δ（模型 / 实测）、贪心累计 Δ、包分布、类别与证据、示例首达链；
- `-o` 的 closure.json 另并入 `gates` 键；
- `--gates-md` 写人读排名表，同时打到标准输出。

作业脚本：`closure_composition_job.sh --gates <用例>…` 产出 `build/ccomp/<例>.gates.json.gz` / `.gates.md`。

```bash
uv run --group cluster python scripts/cluster/distribute_tests.py --no-monitor --skip-setup --servers us1 \
  --job gates-<短sha> --ref <sha> --job-timeout 7200 \
  --cmd "CCOMP_TIMEOUT=5400 bash scripts/closure_composition_job.sh --gates hello collectors deepcopy jcasasl" \
  --fetch 'build/ccomp/*.gz' --fetch 'build/ccomp/*.md' --fetch 'build/ccomp/*.out'
```

**作业：**

| 作业 | 内容 |
|---|---|
| `ccomp-base-b3be04ef` | 4 例基线 |
| `ccomp-cut-fbbe139a` | 第一轮 15 组切除。assertion 组停滞，作业中止；已完成的类数取自日志 |
| `ccomp-cut2-32fd1f98` | 补跑与第二轮 |

服务器为 jp1 / jp2（每台 4 GB 级），JDK 21.0.11。

复现：

```bash
# 服务器作业（例）
uv run --group cluster python scripts/cluster/distribute_tests.py --no-monitor --skip-setup --servers jp1 jp2 \
  --job ccomp-<名>-<短sha> --ref <sha> --job-timeout 3600 \
  --cmd "CCOMP_TIMEOUT=900 bash scripts/closure_composition_job.sh --cut-sets 'enumrefl charset allmech' hello collectors" \
  --fetch 'build/ccomp/*.gz' --fetch 'build/ccomp/*.out'
# 本机分析
uv run python scripts/closure_composition.py --closure collectors=<…>/collectors.json.gz \
  --classload s0boot-jvm=<…>/classload.log.gz --face tests/api_surface/s0.txt \
  --diff collectors-enumrefl=<…>/collectors.json.gz:<…>/collectors.enumrefl.json.gz --md out.md
```

## 三、基线

| 例 | 类 | 方法 | 闭包耗时 | 峰值内存 |
|---|---:|---:|---:|---:|
| HelloWorld | 468 | 1806 | 6 s | 0.25 GB |
| CollectorsDemo | 2990 | 17770 | 67 s | 2.2 GB |
| TestJcaSasl | 3039 | 17956 | 72 s | 2.3 GB |
| DeepCopy | 3275 | 20792 | 158 s | 3.4 GB |

**基线口径差（2026-10-08 补记）：** 本文的 HelloWorld 基线是 468 类。boot-image-s4 合入后实测约 **3043**，enum-values-direct 核对时（§六 6.4）为 **3011**，
即 HelloWorld 也进了核心团块：`Enum.valueOf` 经 `enumConstantDirectory` 让 `Method.invoke` 入链，另有第一节所列 4 个入口。
因此本文 §三至 §五的数字都属旧基线，§七各项的单项收益须在新基线上重测。差距由哪些门带入，待 `rava closure --gates` 在新基线上出数后补写到这里。

### 3.1 自身包构成（类数）

S0 Boot 的 JVM 实载为参照列。

| 块 | hello | collectors | deepcopy | jcasasl | S0 Boot JVM 实载 |
|---|---:|---:|---:|---:|---:|
| 安全 / JCA | 14 | 671 | 692 | 720 | 78 |
| locale / 资源束 / 文本格式 | 21 | 310 | 310 | 310 | 15 |
| 集合 | 59 | 199 | 209 | 199 | 171 |
| 字符集 | 22 | 195 | 195 | 195 | 22 |
| java.lang 核心 | 112 | 191 | 200 | 190 | 150 |
| NIO 文件系统 / 通道 | 8 | 153 | 154 | 152 | 107 |
| 方法句柄 / lambda / indy | 14 | 148 | 183 | 148 | 156（另有隐藏类 281） |
| 并发 / 线程 | 33 | 146 | 151 | 146 | 125 |
| Stream API | 4 | 146 | 211 | 146 | 124 |
| 反射 / 注解 | 66 | 143 | 177 | 143 | 150 |
| 日期时间 | 0 | 121 | 159 | 121 | 102 |
| 类加载 / jar | 20 | 99 | 100 | 99 | 87 |
| 基础 IO | 32 | 95 | 99 | 95 | 62 |
| 日志 | 0 | 74 | 75 | 76 | 59 |
| 模块 / jimage | 10 | 68 | 76 | 68 | 70 |
| 网络 / URL | 10 | 66 | 67 | 66 | 28 |
| 正则 | 27 | 62 | 62 | 62 | 33 |
| 引用 / Cleaner | 9 | 34 | 34 | 34 | 28 |
| XML（JDK 内部 SAX 等） | 0 | 21 | 21 | 21 | 6 |
| 随机数生成器族 | 2 | 19 | 19 | 19 | 2 |
| ServiceLoader | 0 | 10 | 10 | 10 | 11 |
| 序列化 | 1 | 9 | 59 | 9 | 5 |
| 信号 / 关停 | 2 | 4 | 4 | 4 | 13 |
| JNDI / 管理 | 0 | 4 | 4 | 4 | 39 |
| 系统属性 / 进程 | 1 | 1 | 1 | 1 | 18 |
| 其他 | 0 | 0 | 0 | 0 | 33 |
| **合计** | **468** | **2990** | **3275** | **3039** | **1694** |

### 3.2 入口机制归属（首达，类数）

各例只列主要门。

| 例 | 主要入口（前驱 → 首个机制节点：类数） |
|---|---|
| hello | `SharedSecrets.ensureClassInitialized → Lookup.ensureInitialized` 125（invoke）；`OutputStreamWriter → StreamEncoder` 117（字符集）；`System.registerNatives → ConcurrentHashMap.put/…` 115（并发）；lang 68 |
| collectors | `Class.getEnumConstantsShared → Method.invoke` **1140**（反射，共 1357）；安全 263（Object 虚方法的 reflect 成员边扇出到 `AlgorithmId.toString` 等）；`registerNatives → CHM` 259；`Lookup.ensureInitialized` 194；`Tripwire.trip → PlatformLogger` 173 |
| deepcopy | `deepCopy → ObjectInputStream.<init>` **1518** / `readObject` 548（序列化，共 2146）；安全 338；invoke 246；`PrintStream.implFormat → Formatter` 189（locale） |
| jcasasl | `main → 类 Security` **1344**；`MessageDigest.getInstance` 351；`Security.getProviders` 204；`Sasl.createSaslClient` 50（安全，共 2197）；`Proxy$Dyn` 232（反射）；invoke 190 |

### 3.3 团块的门（首达链上的枢纽方法，按链经过的团块类数）

团块指不在 hello 中的类。

- **collectors**（团块 2523）：
  `StreamOpFlag.<clinit>` 1121 → `EnumMap.<init>` → `Class.getEnumConstantsShared` 1119 → `Method.invoke` 1112 → `AccessibleObject.<clinit>` 1027 → `AccessController.doPrivileged` / `executePrivileged` 1027。
  之后分叉：`Charset$2.run` 539 → `ServiceLoader$LazyClassPathLookupIterator.nextProviderClass` 509 → `BootLoader.findResources` 500 → `URLClassPath.findResource` 360 → `JarFile.initializeVerifier` 289 → `JarVerifier` 246。
- **deepcopy**（团块 2808）：
  `ObjectInputStream.<init>` 1373 → `ObjectInputFilter$Config.<clinit>` 1353 → `Constructor.newInstance` 949 → `AccessibleObject.<clinit>` 941 → `doPrivileged` 941。
  之后为 `Charset$ExtendedProviderHolder$1.run` 533 → ServiceLoader 类路径查找 444 → `URLClassPath` 302 → `JarFile.initializeVerifier` 227。
- **jcasasl**（团块 2572）：
  `Security.<clinit>` 1167 → `sun/security/util/Debug.<clinit>` → `System.<clinit>` → `registerNatives` → `CHM.put` → `CHM.fullAddCount` 1012 → `ThreadLocalRandom.<clinit>` → `Random.<clinit>` → `Class.getDeclaredField` 1006 → `Reflection.filterFields` 596 → `ImmutableCollections$Set12.contains` → `Objects.equals` 592 → `URL.equals` 509 → `InetAddress.getByName` → `InetAddress.loadResolver` 478 → `ServiceLoader.findFirst` 475。

三例走的门不同，但都会汇入同一组枢纽：`doPrivileged` 的 `PrivilegedAction.run` 上下文无关派发、ServiceLoader 类路径查找、`URLClassPath` / `JarFile` 校验、JCA 提供者表。

## 四、核心团块对照 S0 Boot

团块 2987 类。各列含义：

- 团块∩JVM：S0 Boot 运行期实载。
- 团块∩S0 面：团块中出现在 `tests/api_surface/s0.txt` 的类。
- 差 = 团块 − JVM，即一个 Boot 应用都不加载的类数，作为可减规模的参考。
- S0 面方法 / 类：该块在 S0 面里的总数，不限于团块内。

| 块 | 团块 | ∩JVM | ∩S0 面 | 差 | S0 面方法 / 类 |
|---|---:|---:|---:|---:|---|
| 安全 / JCA | 671 | 78 | 25 | **593** | 150 / 45 |
| locale / 文本格式 | 310 | 15 | 14 | **295** | 74 / 14 |
| 字符集 | 195 | 22 | 5 | **173** | 23 / 5 |
| 并发 | 146 | 91 | 31 | 55 | 262 / 58 |
| lang | 190 | 141 | 48 | 49 | 460 / 53 |
| NIO | 152 | 106 | 16 | 46 | 126 / 22 |
| invoke | 148 | 107 | 4 | 41 | 24 / 8 |
| 网络 | 66 | 27 | 11 | 39 | 87 / 20 |
| 集合 | 199 | 162 | 48 | 37 | 451 / 54 |
| Stream | 146 | 110 | 18 | 36 | 97 / 25 |
| IO | 95 | 59 | 29 | 36 | 161 / 30 |
| 反射 | 143 | 113 | 21 | 30 | 126 / 23 |
| 正则 | 62 | 33 | 2 | 29 | 23 / 2 |
| 模块 / jimage | 68 | 40 | 8 | 28 | 22 / 9 |
| 时间 | 121 | 94 | 25 | 27 | 164 / 31 |
| 日志 | 74 | 50 | 6 | 24 | 47 / 8 |
| XML（内部） | 21 | 0 | 0 | 21 | 239 / 54 |
| 类加载 / jar | 99 | 81 | 7 | 18 | 45 / 12 |
| 随机数族 | 19 | 2 | 0 | 17 | 0 |
| 引用 | 34 | 24 | 5 | 10 | 10 / 5 |
| 序列化 | 9 | 4 | 3 | 5 | 28 / 10 |
| ServiceLoader / 信号 / JNDI / 属性 | 19 | 19 | 1 | 0 | — |
| **合计** | **2987** | **1378** | **327** | **1609** | |

另有 316 个 JVM 实载类不在 collectors 闭包里：invoke 49、反射 37、JNDI 35、并发 34、模块 30、其他 33、属性 17。它们是 Boot 自己的需要，不属于本文的收窄范围。

## 五、反事实切除

### 5.1 切除结果

单元格为切后类数，括号内为相对基线的变化。

| 切除集 | 门 | hello 468 | collectors 2990 | jcasasl 3039 | deepcopy 3275 |
|---|---|---:|---:|---:|---:|
| enumrefl | `Class.getEnumConstantsShared@49`（`values()` 反射调用） | 468 | **528（−2462）** | 3039 | 3275 |
| ensureinit | `SharedSecrets.ensureClassInitialized` | **386（−82）** | 2989 | 3038 | 3275（0） |
| charset | `Charset.lookup2`、`Charset$2.run`（扩展提供者 / 按名查找） | 468 | 2839（−151） | 2888（−151） | 3126（−149） |
| logging | Tripwire、`PlatformLogger.getLogger`、`System.getLogger`、`Shutdown.logRuntimeExit`、`LazyLoggers.getLogger` | 468 | 2880（−110） | 2987（−52） | 3175（−100） |
| logfinder | `LoggerFinderLoader` 提供者查找 | 468 | 2948（−42） | 3034（−5） | 3273（−2） |
| tripwire | `Tripwire.trip` | 468 | 2990 | 3039 | 3275 |
| accobj | `AccessibleObject.<clinit>@17`（doPrivileged） | 468 | 2809（−181） | 2893（−146） | 3130（−145） |
| svccp | `ServiceLoader$LazyClassPathLookupIterator.nextProviderClass` | 468 | 2988（−2） | 3037（−2） | 3273（−2） |
| urlcp | `URLClassPath.getResource/findResource/findResources` | 468 | 2978（−12） | 3027（−12） | 3263（−12） |
| jarverify | `JarFile.initializeVerifier/maybeInstantiateVerifier` | 468 | 3030（**+40**，切除副作用） | 3038（−1） | 4198（**+923**，切除副作用） |
| formatter | `Formatter.format(Locale,…)`（S0 面内，只测量） | 456（−12） | 2965（−25） | 3014（−25） | 3247（−28） |
| fulladd | `CHM.fullAddCount` | 459（−9） | 2990 | 3039 | 3275 |
| dumper | `ClassFileDumper` 目录校验 | 468 | 2988（−2） | 3039 | 3274（−1） |
| hashedmods | `StackTraceElement$HashedModules.hashedModules` | 468 | 2988（−2） | 3037（−2） | 3273（−2） |
| serfilter | `ObjectInputFilter$Config.<clinit>@120/209/220/227`（sysprop 为 null 的分支） | 468 | 2990 | 3039 | 3273（−2） |
| urleq | `URL.equals`（经 `Objects.equals` 汇合派发） | 468 | 2990 | 3039 | 3275 |
| **allmech** | 以上合并，不含 formatter / serfilter / accobj / urleq | **375（−93）** | **503（−2487）** | **2325（−714）** | 3986（消失 214、新增 925：被 jarverify 副作用污染） |
| **r2all** | allmech + serfilter + accobj + urleq | 369（−99） | 503 | 超时（>900 s） | **2111（−1164）** |

**切除上界 ≠ 直连收益（2026-10-08 补记）：** enumrefl 行的 −2462 是切除上界。enum-values-direct 已把该调用点直连，
新基线上 HelloWorld / CollectorsDemo 仍为 3011 类，原因是 `Method.invoke` 还有 4 个入口（第一节第 1 条）。
团块是多连通的：切除一点只在该点是唯一入口时才兑现整块；其余入口任一敞开，整块照旧进来。
`--gates` 的贪心组合曲线（入口段）和组合实测就是用来量这种「多入口一起关」的收益的。

**assertion 组**（`Class.desiredAssertionStatus` 体切除）使 collectors / jcasasl 的分析停滞 35 分钟以上，已从切除集删除。
该方法的 VM 初值已由 `vm_intrinsics.toml` 中 `desiredAssertionStatus0 = false` 折叠，`$assertionsDisabled` 已是常量，不需要再做反事实。
**r2all 在 jcasasl 上超时**，原因同属切除副作用：切除使某些值集变空，下游折叠条件失效，分析重复展开。不影响结论。

### 5.2 消失类的构成（按自身包）

- **collectors-enumrefl −2462：** 安全 657、locale 289、字符集 173、NIO 145、invoke 134、集合 130、时间 121、并发 113、Stream 98、类加载 79、lang 78、日志 74…
  切后 528 类比 hello 多 61 类：Stream 44、集合 10、反射 5、lang 1、其他 1。这是 CollectorsDemo 在旧基线上相对 HelloWorld 的增量（切除上界口径）。
- **jcasasl-allmech −714：** 安全 468、字符集 158、并发 29、日志 19、类加载 14…
  剩下的 2325 类以 JCA 为主：`Security.<clinit>` 加 provider 表，见 §六「安全」。
- **jcasasl-accobj −146：** 安全 88、反射 23、NIO 13、模块 13。
- **collectors-accobj −181：** 安全 88、日志 28、反射 23、模块 13。
  doPrivileged 扇出门单独切就能回收 150–180 类。§21.5 曾实测 executePrivileged 上下文敏感化约为 0，那是在枚举门敞开时的口径。
- **collectors / jcasasl-charset −151：** 全部是字符集自身包（`sun/nio/cs` 的各编码类）。
- **collectors-logging −110：** 日志 64、并发 27…
- **hello-ensureinit −82：** invoke 体系，即 `Lookup.ensureInitialized` 的访问校验与 `MethodHandles` 基础设施。

- **deepcopy-r2all −1164（3275 → 2111）**：
  - 按自身包：安全 516、字符集 164、日志 64、Stream 51、类加载 47、locale 46、模块 44、NIO 35、并发 32、IO 31…
  - 按基线入口机制，序列化占 876，即 `ObjectInputStream` 这扇门后面的团块。
  - 单项切除都只有 −2 到 −149，合并后 −1164：
    - OIS 的反射构造经 `AccessibleObject.<clinit>` 进入 doPrivileged 扇出；
    - 再叠加字符集 / 日志 / ServiceLoader 等门。
    - 多扇门同时关闭，团块才整体脱落。
  - 剩下的 2111 类以序列化本体、反射与 JCA 为主（OIS 读 `ObjectStreamClass` 时会经 `MessageDigest` 计算 SUID）。
- **deepcopy-accobj −145：** 安全 88、反射 21、模块 13、NIO 11。三例的这一门回收量几乎相同（−145 / −146 / −181），说明 doPrivileged 扇出是三例共用的门。
- **deepcopy-charset −149：** 全部是字符集自身包。
- **deepcopy-logging −101：** 日志 64、并发 27。
- **jarverify 切除使类数上升（collectors +40、deepcopy +923，新增的主要是 Xerces 790 类）：**
  - 切的是写入点：`maybeInstantiateVerifier` 写 `jv`。字段值集变空后按初值折叠为恒 null，走到了另一侧分支，结果非单调（`engine/cut.rs` 文档与 c1d §7 已警示）。
  - 因此 deepcopy-allmech 被污染：消失 214、新增 925。deepcopy 的合并上界以 r2all 为准。
  - 这一组只说明 jar 校验不能靠切一个写入点来量化。正确的量化方式是 §六「类加载」行所说的整体替换。

## 六、逐块判定

表中「三问」三列依次对应问题 ①（模型下成立？）、问题 ②（构建期可求值？）、问题 ③（翻译 + 精度缺口）。
「可减类数」中，**实测**指反事实切除的结果，**估计**指团块 − JVM 实载。「S0 面」列中，在面内的部分不能当作暂缓项删掉。

| 块 | ① 模型下成立？ | ② 构建期可求值？ | ③ 翻译 + 精度缺口 | 处理 | 可减类数 | 工作 | 风险 | S0 面 |
|---|---|---|---|---|---|---|---|---|
| **反射：枚举 `values()` 反射调用** | 成立 | — | **是**：`getMethod("values").invoke(null)` 的接收类是已知的枚举类，却展开了整个 `Method.invoke` 访问器 / 句柄体系，经 `AccessibleObject.<clinit>` 进入团块 | 精度：已知类上已知名静态无参方法的反射调用按直接调用建边（a3-C 前置；c1d §21.9） | 实测：collectors **−2462**；所有用到 EnumMap / EnumSet / `Enum.valueOf` 的程序同量级（a3-C 记 TestEnumBasic 472 → 3103）。切除上界：直连已合入（59451c29），新基线仍 3011 类，须连同其余 4 个 `Method.invoke` 入口一起收口才兑现（§七第 1 项） | 分析器反射调用建模：名字与接收类都已知时直连；未知时仍走全套 | 低：名字或类不确定时保持现状，健全性不受影响 | 在面内（反射 126 方法 / 23 类）：反射保留，只收窄 |
| **invoke：`SharedSecrets.ensureClassInitialized` → `Lookup.ensureInitialized`** | 类初始化本身是 ③（已经由 `class_initializers` 承载）；`Lookup` 的访问校验在 rava 下恒通过 | **是**：调用方是 java.base 内的全权 `MethodHandles.lookup()`，访问校验在构建期就有确定结果 | — | 构建期求值：`Lookup` 访问校验按全权 lookup 折叠，或把 `SharedSecrets.ensureClassInitialized` 登记为按镜像初始化入口（`class_initializers`）、不分析其体 | 实测：hello **−82**（468 → 386）。所有程序都受益；用到 lambda 的程序本来就需要 invoke 体系，收益较小（collectors −1） | 分析器折叠，或清单登记 1 行 | 低 | 面内只有 4 类，不受影响 |
| **安全 / JCA** | 成立（提供者是纯 Java） | **是**：`java.security` 配置、提供者列表与顺序、`Security.<clinit>` 读配置、`Debug` 读 sysprop | 有：Object 虚方法的 reflect 成员边扇出到 `AlgorithmId.toString` 一类（随实例化集合收窄自然消失） | 构建期求值：`ProviderList` / `ProviderConfig` 只展开配置且实际被请求的提供者与服务类型；fix-jca-subset 的类型过滤之外，再按算法名常量收窄；jar 签名校验整体移除（封闭映像无签名 jar，见「类加载」） | 实测：jcasasl allmech **−714**（安全 468）；估计：团块差 **593** | 引导映像求值器覆盖 `Security.<clinit>` 与 provider 装载；分析器按服务类型 + 算法名收窄 | 中：算法名来自运行期字符串时要回退到全集 | 在面内（150 / 45：MessageDigest、SecureRandom 等）：保留，只收窄提供者 |
| **locale / 资源束 / 文本格式** | 成立 | **是**：缺省 locale、`LocaleProviderAdapter` 偏好（sysprop）、CLDR / JRE 适配器选择、可用 locale 列表都能在构建期定 | 有：`Formatter` 本身在面内；locale 数据类按 locale 种子收窄已做 | 构建期求值：适配器链与缺省 locale 定值，只保留实际 locale 的束；`Formatter` 翻译 | 实测：formatter −25（只是这一门，团块多连通）；估计：团块差 **295** | 引导映像求值 `LocaleProviderAdapter` 静态状态；分析器按适配器类型收窄 | 中：用户代码显式 `Locale.forLanguageTag` 时需保留对应束 | 在面内（74 / 14）：Formatter / DateTimeFormatter 不可删 |
| **字符集** | 成立 | **是**：缺省字符集（UTF-8）、`ExtendedProviderHolder`（ServiceLoader）在封闭映像下为空、`StandardCharsets` 的 PreHashedMap 是常量表 | 有：按名查找（`Charset.forName(未知名)`）展开全部标准字符集 | 构建期求值：扩展提供者为空；按名查找只在名字是常量时收窄到该字符集，名字未知时保留全集（面内的 `Charset.forName` 由用户决定） | 实测：collectors / jcasasl **−151**；估计：团块差 **173** | 引导映像求值 `Charset` 静态状态；分析器常量名查找 | 低至中 | 面内 23 / 5（UTF-8 / ISO-8859-1 等）：保留 |
| **日志（JUL / System.Logger）** | `LoggerFinder` 的 ServiceLoader 查找在封闭映像下变为构建期事实；BootstrapLogger 的 `isBooted` 判定也是 | **是**：LoggerFinder 提供者、Tripwire（sysprop `…tripwire` = false）、BootstrapLogger 状态 | — | 构建期求值（日志 / 信号链待决定 L，本块随 L 定案） | 实测：collectors −110、jcasasl −52、deepcopy −100；估计：团块差 24（Boot 自身大量用日志） | 引导映像求值 + 清单 | 低 | 在面内（47 / 8）：JUL 保留 |
| **类加载 / URLClassPath / jar** | **不成立**：封闭映像、无运行期类加载、无类路径 jar；资源嵌入映像 | — | — | 整体替换（②）：资源查找落到嵌入资源表（boot-layer 步骤 1–5：命名模块 + jimage 读取，唯一 native 为 `NativeImageBuffer.getNativeMap`）；jar 签名校验整体移除 | 实测：urlcp −12、svccp −2、jarverify 反升 +40 / +923（切的是写入点，非单调，不作数）；c1d §21.5 联合切除 jar / JCA 簇 −256；估计：团块差 18 | boot-layer 步骤 1–5（已有计划） | 中：TestClassResourceStream 等资源例需要嵌入资源完整 | 在面内（45 / 12：`ClassLoader.getResource*` 是 Spring 刚需）：保留 API，换实现 |
| **ServiceLoader** | 机制成立，但提供者集合在封闭映像下是构建期事实 | **是**：`META-INF/services` + `module-info provides` 在构建期收集 | — | 构建期求值：提供者表由引导映像生成；`LazyClassPathLookupIterator` 不再可达 | 主要收益体现在字符集、日志、网络、随机数等各块 | 引导映像 + 清单 | 低 | 面内 2 / 1：保留 |
| **模块 / jimage / jrtfs** | boot layer 构建是构建期事实；jimage 读取在映像内 | **是**：boot layer、`HashedModules`、`ModuleBootstrap` | 有：Object 虚方法扇出到 `JrtFileAttributes.toString`（collectors 120 类首达） | 构建期求值（boot layer 进引导映像）+ jimage 嵌入 | 实测：hashedmods −2；估计：团块差 28 | boot-layer 计划 | 中 | 面内 22 / 9：保留 |
| **引用 / Cleaner（GC 驱动）** | **不成立**：无 GC 模型下，引用清理由 Rc 释放触发（10-07 决定） | — | — | ③：Reference Handler / Cleaner 线程的 VM 驱动部分手写；`Reference` / `WeakHashMap` 等的类 API 翻译 | 估计：团块差 10 | 随无 GC 模型实施（C4 之后） | 中 | 面内 10 / 5（WeakHashMap、SoftReference 缓存）：API 保留 |
| **信号 / 关停** | **不成立**：Signal 分发由 VM 驱动 | — | — | ③（待决定 L） | 0–4 | 随 L | 低 | 面内 1 / 1 |
| **序列化** | 成立 | 部分：`ObjectInputFilter$Config` 的 `jdk.serialFilter*` sysprop 缺省为 null | 有：OIS 反射构造经 `AccessibleObject.<clinit>` 进入 doPrivileged 扇出 | 翻译 + sysprop 构建期求值 + doPrivileged 上下文精度 | 实测（deepcopy）：serfilter −2、accobj −145；与其他门合并（r2all）为 −1164（序列化入口 876） | 引导映像求值 sysprop；分析器 | 中 | 在面内（28 / 10）：保留 |
| **doPrivileged 扇出（跨块）** | 成立（rava 下 `AccessController` 恒放行） | — | **是**：`executePrivileged` 对 `PrivilegedAction.run` 做上下文无关派发，一处可达即扇出到全部已实例化 action | 精度：`executePrivileged` 按调用点上下文敏感（实参 action 的来源类型），或在 rava 下把 `doPrivileged(a)` 直接视为 `a.run()` 的单点调用 | 实测：accobj collectors −181、jcasasl −146、deepcopy −145；deepcopy 与其他门合并（r2all）为 −1164 | 分析器 | 中：c1d §21.5 曾测得约 0，需在新口径下复核 | 跨块 |
| **Objects.equals / 容器元素（跨块）** | 成立 | — | **是**：`Reflection.filter` 里 `Set.contains(String)` 经 `Objects.equals` 派发到 `URL.equals` → DNS 解析 → ServiceLoader | 精度：容器元素敏感（c1d §21.5 重定向 2） | 实测：fulladd hello −9；urleq 单切 0（多连通） | 分析器 | 中 | 跨块 |
| **并发 / 线程** | 成立（虚拟线程按 Continuation 方案） | — | 有：`registerNatives → CHM` 带入 `fullAddCount → ThreadLocalRandom` | 翻译 | 实测：fulladd −9（hello）；估计：团块差 55 | — | — | 在面内（262 / 58） |
| **NIO / 网络 / 时间 / 正则 / Stream / 集合 / lang / IO** | 成立 | 局部可以：默认 `FileSystemProvider`、URL 协议处理器、`InetAddress` 解析器、`ZoneRulesProvider` 都是 ServiceLoader 或 sysprop 决定 | 随上述门收窄 | 翻译；提供者部分随 ServiceLoader 构建期求值 | 估计：NIO 46、网络 39、时间 27、正则 29、Stream 36、集合 37、lang 49、IO 36 | — | 低 | 全部在面内 |
| **XML（JDK 内部 SAX / `jdk.internal.util.xml`）** | 成立 | — | 有：`Properties.loadFromXML` 一类入口 | 翻译 | 估计：21 | — | 低 | JAXP 在面内（239 / 54） |
| **随机数生成器族** | 成立 | **是**：`RandomGenerator` 工厂的 ServiceLoader | — | 构建期求值 | 估计：17 | 随 ServiceLoader | 低 | 不在面内 |

## 七、推荐收窄任务（按回收量 × 覆盖面排序，本文不实施）

1. **`Method.invoke` 全部入口直连**（精度；a3-C 前置；2026-10-08 按直连实测改写）。
   - 枚举入口 `Class.getEnumConstantsShared@49` 已直连（enum-values-direct 59451c29，已合入 batch-1008）。
   - 但新基线上 HelloWorld / CollectorsDemo 仍为 3011 类：切除实验的 2990 → 528 是上界，只有其余 4 个入口也不经 `Method.invoke` 体时才兑现。
   - 其余入口按 enum-values-direct §八 收口：
     - 实例目标（`BasicImageReader$2.run@37`、`AnnotationInvocationHandler.equalsImpl@121`）：按接收者值集虚派发接边；
     - 反射对象跨方法流动（`ServiceLoader$ProviderImpl.invokeFactoryMethod@20`、`equalsImpl`）：反射对象建「解析出的成员键」抽象值；
     - 非常量名（`HostLocaleProviderAdapter.findInstalledProvider@39`）：名字按拆段口径求候选集。
   - 每收口一个入口，就用 `--gates` 在新基线上复测，看贪心组合里剩余入口的累计 Δ。
   - 全部完成后，a3-C 的 `Class.enumConstantDirectory` 手写可删。
2. **doPrivileged 扇出精度**（前移；`executePrivileged` 按调用点上下文敏感，或在 rava 下把 `doPrivileged(a)` 视为对 `a.run()` 的单点调用）。
   - 这是三例共用的门：单门实测 −145 至 −181（旧基线）。
   - 它还是第 1 项 `BasicImageReader$2.run` 入口的来路：`Random.<clinit>` → `getReflectionFactory` → doPrivileged 按全部 PrivilegedAction 实现派发。所以它与第 1 项同属关闭 `Method.invoke` 入口的前置。
   - DeepCopy 的 OIS 反射构造经它进入团块，与第 4–6 项合并后为 −1164（3275 → 2111）。
   - c1d §21.5 曾测得约 0，那是在枚举门敞开时的口径，需要在新口径下复核。
3. **`SharedSecrets.ensureClassInitialized` 链构建期折叠**（构建期求值 / 清单）。
   - 旧基线下每个程序 −82（HelloWorld 468 → 386）。工作量最小（清单 1 行或一处折叠）。
   - 2026-10-08 新基线复测：Δ 类为 0，门已消失，暂不实施（见 7.1）。
4. **JCA 提供者构建期求值 + jar 签名校验移除**（构建期求值 + 整体替换）。
   - jcasasl allmech −714，团块差 593。与 c1d §21.5 重定向 1 合并。
5. **字符集构建期求值**：扩展提供者为空；常量名查找收窄。三例都是 −149 至 −151，团块差 173。
6. **日志链构建期求值**（随决定 L）：−52 至 −110。
7. **locale 适配器链构建期求值**：团块差 295（Formatter 单门只 −25 至 −28，需在其他门关闭后复测）。
8. **类加载 / 资源封闭映像整体替换**（boot-layer 步骤 1–5）：本身减量小（团块差 18），但它是 ServiceLoader 类路径查找、jar 校验等多扇门的根；jar 校验不能用切写入点量化，以整体替换后的实测为准。
9. **容器元素敏感**（`Objects.equals` 汇合派发）：c1d §21.5 重定向 2，随后续多连通门关闭而显现。
10. **引用 / 信号按 ③ 收口**：随无 GC 模型与决定 L 实施，减量小。

**口径提醒：** 团块是多连通的。§五的单项收益都是旧基线（HelloWorld 468）上的切除上界，新基线（约 3011–3043）上要重测。
重测用 `rava closure --gates`：单切 Δ、贪心累计与组合实测一次给出。第 1、2 项完成前，第 3–9 项的单切 Δ 多半接近 0（团块仍经 `Method.invoke` 入口整体进来），要看贪心组合曲线。
建议每完成一项，用 `closure_composition_job.sh` 在 4 例上重跑基线与 r2all，作为验收数据（jarverify 组是写入点切除、非单调，合并上界不含它更准，下一轮可剔除）。

### 7.1 第 3 项新基线复测（2026-10-08，06645419，分支 ensure-init）

作业：`ei-gates-06645419`（us1，`--gates hello collectors`）、`ei-cut-06645419`（jp2，基线 + `--cut-sets ensureinit`）、`ei-base-06645419`（DeepCopy 基线）。产物在 `cluster_results/job/<tag>/01/build/ccomp/`。

| 例 | 基线 类 / 方法 | 切 `SharedSecrets.ensureClassInitialized` 后 | Δ 类 | Δ 方法 |
|---|---:|---:|---:|---:|
| HelloWorld | 2178 / 13044 | 2178 / 13041 | 0 | −3 |
| CollectorsDemo | 2178 / 13051 | 2178 / 13048 | 0 | −3 |
| DeepCopy | 3573 / 22505 | 3560 / 22437 | −13 | −68 |

`--gates` 的候选池（hello 68 / collectors 72）里没有这条链上的任何方法，单切与贪心组合都不出现。

结论：**这扇门在新基线上已不存在，第 3 项不实施。** 依据如下：

- 切除是**不健全的上界**：它连同目标类初始化一起去掉了。即便如此，三例可回收的也只有：
  - `Lookup.ensureInitialized`、`checkSecurityManager`、`makeAccessException` 3 个方法；
  - DeepCopy 另有 `HttpCookie` 及 12 个内部类。
- `HttpCookie` 这 13 类属于目标类初始化本身的语义：`getJavaNetHttpCookieAccess` 在 access 为 null 时初始化 `HttpCookie`。按 ③ 的口径这部分不可删，健全折叠的收益是 **0 类**。
- 旧基线下的 −82 是 invoke 体系（`MethodHandles` / `Lookup` / `VerifyAccess` / `ClassFileDumper` …）经这条链首达。新基线上，这些类改由 `ConcurrentSkipListMap.<clinit>` → `MethodHandles.lookup()` → `Lookup.findVarHandle` → `resolveOrFail` → `checkSymbolicClass` → `VerifyAccess.isClassAccessible` 进入。所以即使把这条链折叠掉，类也一个不少。
- 只为 3 个方法加一个通用的「访问检查恒真」折叠机制，收益与成本不相称。

后续入口：

- 第 1、2 项完成、并且 j.u.c 的 `<clinit>` VarHandle 查找（`findVarHandle` 链）也收口之后，用 `closure_composition_job.sh --cut-sets ensureinit` 复测。若那时 Δ 类 > 0，再按以下终态方案实施：
  - 清单声明「同模块全权 lookup 对公开类的访问检查恒真」这一事实；
  - 分析器据此折叠 `makeAccessException` 分支，`MethodHandles.lookup()` 不因此链入闭包；
  - 生成器不写类名。
- 「目标类已在引导映像中初始化 → `SharedSecrets.getXxxAccess` 的 null 分支不可达」由引导映像第 6 步（非引导类构建期初始化）覆盖。DeepCopy 的 `HttpCookie` 13 类就在这条线上。

### 7.2 注解成员签名解析新基线复测（2026-10-08，3f4fee31，分支 annot-sig）

切除集 `annsig` / `annall` / `sigall`，作业 `as-cut-3f4fee31`、`as-gates-3f4fee31`。`parseSig` 折叠的健全收益为 0 类、至多 −2 方法，不实施；`sun/reflect/generics` 由 open(Comparable) 回退独立保留。数据与依据见 `docs/plans/2026-10-05-boot-image-evaluator.md` §5.6.10。

## 八、S0 Spring Boot（待 dev 恢复）

S0 Boot 需要 dev 级内存（28 GB 槽），dev 关机期间不跑。dev 恢复后执行：

```bash
H=$(git rev-parse closure-composition)
uv run --group cluster python scripts/cluster/distribute_tests.py --no-monitor --skip-setup --servers dev --slot-mem dev=28 \
  --job ccomp-s0boot-${H:0:8} --ref $H --job-timeout 9000 \
  --cmd 'CCOMP_TIMEOUT=7200 bash scripts/closure_composition_job.sh s0boot' \
  --fetch 'build/ccomp/*.gz' --fetch 'build/ccomp/*.out'
# 分析（本机）
R=cluster_results/job/ccomp-s0boot-${H:0:8}/01/build/ccomp
uv run python scripts/closure_composition.py --closure s0boot=$R/s0boot.json.gz \
  --classload s0boot-jvm=cluster_results/job/apis0-kr2b-1fdb1bed/01/build/api_surface/s0/classload.log.gz \
  --face tests/api_surface/s0.txt --md build/ccomp/s0boot.md
```

切除对照可在上面的 `--cmd` 中加 `--cut-sets 'enumrefl allmech'`。
