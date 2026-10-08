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
   **新基线（2026-10-08，d5a2cb5d）**：HelloWorld / CollectorsDemo 2178、DeepCopy 3573；`BasicImageReader$2` 与 `HostLocaleProviderAdapter` 两个入口已由
   95558d79（实例目标、非常量名直连）消掉，剩余入口切除上界 hello / collectors −128、deepcopy −774（§3.0）。团块的大部分改由日志链与字符集带入。
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
   1. `Method.invoke` 全部入口直连：枚举、实例目标、非常量名三种形状已完成；剩余入口都是「反射对象经字段 / 数组 / 列表跨方法流动」一种形状（§七第 1 项）。
   2. doPrivileged 扇出精度：**新口径下已无缺口**（按调用点克隆，各克隆动作单一）；`--gates` 上的大 Δ 是工具缺陷，已修（§二）。
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

#### 门排名工具核对（2026-10-08，新基线 4 例）

1. **模型 Δ 远低于实测 Δ**：hello `getLoggerFromFinder` 模型 17 / 实测 198，`executePrivileged` 模型 33 / 实测 955，`Charset.lookup` 模型 1 / 实测 163。
   模型是不含值流折叠的「与」可达性，多连通团块上单切几乎都近 0；排名必须以实测列为准，模型列只用于选候选与贪心顺序。
   `--gates-verify` 预算（缺省 12）因此是排名质量的瓶颈，大例要给足内存预算（`CCOMP_GATES_ARGS="--gates-mem-mb 9000"`）。
2. **非单调已能识别**：collectors 单切 `executePrivileged` 实测 2633 类（反升 931，非单调）、jcasasl 单切 `CryptoAlgorithmConstraints.permits` 反升 41，表中照标。
3. **缺陷：分派转发方法被当成单一机制的门（已修，最终形态见下）**。`AccessController.executePrivileged` 已由 `forward.rs` 按调用点克隆，
   各克隆的动作形参值集单一（`@set:` 核对；`@callers:` 无上下文无关体）；它在首达树上是全部特权块调用点的汇合，单切量是互不相关调用点之和，
   且非单调，不对应任何一个机制。
   - 试过直接排除（7f7d6ae7：静态转发方法不作候选；30cca0fb：收窄为各克隆分派目标不同者）。两版都把 `Charset.lookup` 一并排除
     （它同为转发方法，各调用点名字不同、克隆分派也不同；作业 `rdg2-7f7d6ae7` / `rdg3-30cca0fb` 中 −163 的字符集门消失），且 30cca0fb 的克隆比对让模型计算 3 s → 55 s。
     按字节码结构分不出「特权块汇合」与「按名查找」，排除会丢真门。
   - 终版：照常作候选，证据末条标明「分派转发方法（已按调用点克隆）：切除量是全部调用点下游之和，未必是单一机制，门看调用方调用点」（`Facts.forwarder`）。
     核对（作业 `rdg4-72f1aca5`，hello）：`Charset.lookup` 回到第 4 名（实测 −163），`executePrivileged` 证据带附注，模型计算 3.7 s。
4. **陈旧提示（已撤，7f7d6ae7）**：`closure.toml [gates]` 的 `precision = ["java/security/AccessController"]` 把特权块整体标为精度缺口；
   doPrivileged 已按调用点克隆，该提示不再成立，改为空表。

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
因此本文 §三至 §五的数字都属旧基线，§七各项的单项收益须在新基线上重测。

### 3.0 新基线（2026-10-08，batch-1009 d5a2cb5d，含 f19e46e0 HelloWorld OOM 修正）

| 例 | 类 | 方法 | 闭包耗时 | 峰值内存 |
|---|---:|---:|---:|---:|
| HelloWorld | 2178 | 13047 | 44 s | 1.6 GB |
| CollectorsDemo | 2178 | 13054 | 45 s | 1.6 GB |
| TestJcaSasl | 2276 | 13606 | 65 s | — |
| DeepCopy | 3573 | 22508 | 294 s | 4.6 GB |

作业：`rdg-dj-d5a2cb5d`（sg2，`--gates` hello / collectors / deepcopy / jcasasl）、`rdbase-d5a2cb5d`（sg2，`@callers:Method.invoke`）。
f19e46e0 之前的 3011 / 3043 已不作口径。HelloWorld 与 CollectorsDemo 现在几乎同集：枚举入口直连后，CollectorsDemo 自身只多 0 类。

**468 → 2178 的差距由哪些门带入**（`--gates` 单切实测 Δ；`--why` 首达链）：

| 门 | hello 单切实测 | 来路 | 归属 |
|---|---:|---|---|
| `LazyLoggers.getLoggerFromFinder` | −198 | `[boot_image] 根 Thread.start0` → `Signal$1.run` → `Terminator$1.handle` → `Shutdown.exit` → `Runtime.logRuntimeExit` → `System.getLogger` → `LoggerFinder` | 日志链（U12 线）：引导映像把信号线程入口当根，关停路径带入整套 System.Logger / JUL |
| `System$LoggerFinder.lambda$accessProvider$0` | −196 | 同上，LoggerFinder 的 ServiceLoader 查找 | 日志链 |
| `LoggingProviderImpl.demandJULLoggerFor` | −140 | 同上，JUL `LogManager` | 日志链 |
| `Charset.lookup` | −163 | 引导区根 `Charset.isSupported` → `lookup2` → `StandardCharsets.charsetForName` | 字符集（构建期求值线） |
| `Method.invoke` 其余入口（`minvoke` 切除集 4 个调用点） | −128 | `InetAddress.loadResolver` → `ServiceLoader$ProviderImpl.invokeFactoryMethod@20`；`AnnotationInvocationHandler.equalsImpl@121` | 本线（§七第 1 项），见下 |
| `Formatter.format` | −25 | `PrintStream.implFormat` | locale |
| `InetAddress.getAllByName0` | −11 | `URL.equals` → DNS | 容器元素精度 |
| `executePrivileged` | −955（模型 33） | 全部特权块的汇合 | **不是单一机制**：见 §二「门排名工具核对」 |

日志三门互相重叠（都在 `Shutdown.exit` 之下），合计约 −200 到 −660（`getLoggerFromFinder` 首达树 663 类）；它们与字符集、`Method.invoke` 三条线是
468 → 2178 的主体。旧基线的 468 没有信号线程根，也没有 `InetAddress.loadResolver` 这条 ServiceLoader 路径。
DeepCopy 另有 `DeepCopy.deepCopy` −1393（序列化入口）、`LocaleProviderAdapter.forType` −130。

**`Method.invoke` 剩余入口与切除上界**（作业 `rdcut-d442e327`，ref d442e327，含 95558d79 的实例目标直连）：

| 切除集 | HelloWorld | CollectorsDemo | DeepCopy |
|---|---:|---:|---:|
| 基线 | 2178 | 2178 | 3573 |
| `minvoke`（剩余 4 个调用点） | 2050（−128） | 2050（−128） | 3573（0） |
| `minvokebody`（`Method.invoke` 体整体） | 2050（−128） | 2050（−128） | 2799（−774） |

DeepCopy 切调用点为 0、切方法体 −774：DeepCopy 还有 `ObjectStreamClass.invoke{Read,Write}Object` / `invokeReadResolve` / `invokeWriteReplace` /
`invokeReadObjectNoData`、`HttpConnectSocketImpl.doTunneling`、`NTLMAuthenticationProxy` 两处共 8 个入口，不在 `minvoke` 中。
旧报告「2990 → 528」的上界在新基线上只剩 −128（hello / collectors）与 −774（deepcopy）：团块的其余部分已改由日志链、字符集等门带入。

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

1. **`Method.invoke` 全部入口直连**（精度；a3-C 前置；2026-10-08 实施进展）。
   - 已完成：枚举入口 `Class.getEnumConstantsShared@49`（59451c29）；实例目标与非常量名（95558d79，分支 reflect-direct）——`BasicImageReader$2.run@37`、
     `HostLocaleProviderAdapter.findInstalledProvider@39` 已不在闭包。新基线类数不变（hello / collectors 2178、deepcopy 3573），`fold_direct_calls` 1 → 2（hello）、0 → 2（deepcopy）：
     这两个入口单独关掉不减类，团块还经其余入口进来。
   - 剩余入口（`@callers:java/lang/reflect/Method.invoke:` 实测，作业 `rdbase-d5a2cb5d` / `rddiag2-d442e327`）：
     - hello / collectors：`ServiceLoader$ProviderImpl.invokeFactoryMethod@20`（5 个上下文；`Method` 来自 `getDeclaredPublicMethods` 列表、存字段）、`AnnotationInvocationHandler.equalsImpl@121`（数组元素）；
     - deepcopy 另有 `ObjectStreamClass.invokeReadObject@24` / `invokeReadObjectNoData@20` / `invokeReadResolve@20` / `invokeWriteObject@24` / `invokeWriteReplace@20`（`Method` 存字段，查找有形参类型）、
       `HttpConnectSocketImpl.doTunneling@8`、`NTLMAuthenticationProxy.isTrustedSite@12` / `supportsTransparentAuth@8`（静态字段）。
   - 全部是同一形状：`Method` 对象在别的方法里查出、经字段 / 数组 / 列表流到 `invoke`。终态方案：反射对象建「解析出的成员键」抽象值
     （查找点结果换成成员键标记对象，仿 `class_lookup` 的 forName 结果替换与 `field_handles` 的 `mark_named`；`Method.copy` / `ReflectionFactory.copyMethod` 由清单声明为保键复制），
     调用点接收者值集全为成员键标记时按标记直连（非 CS 校验），含普通 `Method` 才回退。
   - 收益上界：hello / collectors −128，deepcopy −774（§3.0）。全部完成后 a3-C 的 `Class.enumConstantDirectory` 手写可删。
2. **doPrivileged 扇出精度**（2026-10-08 复核：**无缺口，不需实施**）。
   - `executePrivileged` 已由 `forward.rs` 按调用点克隆（k = 1，转发链随最外层调用点分开）；`@callers:AccessController.executePrivileged` 无上下文无关体，
     各克隆的动作形参 `@set:` 值集单一。旧基线的 −145 至 −181 是克隆前口径。
   - `--gates` 上 `executePrivileged` 的 −955（hello）/ −1606（deepcopy）/ 非单调（collectors）是各特权块调用点之和，属工具缺陷，现已在证据中标注（§二「门排名工具核对」）。
   - 与第 1 项不同机制：第 1 项是反射对象值流，本项是派发上下文，已由 G1 覆盖。
3. **`SharedSecrets.ensureClassInitialized` 链构建期折叠**（构建期求值 / 清单）。
   - 旧基线下每个程序 −82（HelloWorld 468 → 386）。工作量最小（清单 1 行或一处折叠）。
   - 2026-10-08 新基线复测：Δ 类为 0，门已消失，暂不实施（见 7.1）。
4. **JCA 提供者构建期求值 + jar 签名校验移除**（构建期求值 + 整体替换）。
   - jcasasl allmech −714，团块差 593。与 c1d §21.5 重定向 1 合并。
5. **字符集构建期求值**：扩展提供者为空；常量名查找收窄。三例都是 −149 至 −151，团块差 173。
   - 2026-10-08 新基线复测：只收窄宿主名或只收窄常量名，Δ 都是 0。两者同时收窄的上界为 −141。扩展提供者不能折为空。挂起，等待用户就 U1 字符集域作出决定（见 7.3）。
6. **日志链构建期求值**（随决定 L）：−52 至 −110。
   - 2026-10-08 复测（分支 seed-chain）：TLR 种子读修复后，(L) 链与 `ObjectInputFilter$Config` 的 `System.getLogger` 是 HelloWorld / CollectorsDemo / DeepCopy 大集合的剩余持有者，切除上界 HelloWorld −2767、CollectorsDemo −2721；所缺精度机制见 7.5.3。
7. **locale 适配器链构建期求值**：团块差 295（Formatter 单门只 −25 至 −28，需在其他门关闭后复测）。
   - 2026-10-08 实施（分支 locale-build，bbeb13a5）：HOST / SPI 适配器出闭包，三例 −18 至 −19 类、−132 至 −144 方法（超过 lcaux 上界，见 7.4）。不涉及 U1：`java.locale.providers` 已折为 null，偏好表恒为 [CLDR, JRE]。CLDR / JRE 本体（lcadapt 上界剩余 −112 至 −126 类）是宿主 locale 下的合法数据访问，不再按构建期求值处理，残余见 7.4 第 5 条。
8. **类加载 / 资源封闭映像整体替换**（boot-layer 步骤 1–5）：本身减量小（团块差 18），但它是 ServiceLoader 类路径查找、jar 校验等多扇门的根；jar 校验不能用切写入点量化，以整体替换后的实测为准。
9. **容器元素敏感**（`Objects.equals` 汇合派发）：c1d §21.5 重定向 2，随后续多连通门关闭而显现。
10. **引用 / 信号按 ③ 收口**：随无 GC 模型与决定 L 实施，减量小。

**口径提醒：** 团块是多连通的。§五的单项收益都是旧基线（HelloWorld 468）上的切除上界，新基线（2178 / 3573，§3.0）上要重测。
重测用 `rava closure --gates`：单切 Δ、贪心累计与组合实测一次给出。新基线上日志链（−198）、字符集（−163）单切已有显著实测 Δ，`Method.invoke` 剩余入口只占 −128（hello）；排名以实测列为准（§二「门排名工具核对」）。
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

### 7.3 第 5 项字符集构建期求值新基线复测（2026-10-08，641d48ce，分支 charset-build）

作业 `cs-cut-641d48ce`（jp2，基线 + `--cut-sets "charset cshost csext csstd"`，head 含 batch-1010 ab8dd577 / u12-props）；8bdc2721 上的 `cs-cut-8bdc2721`（sg2）逐数相同。产物在 `cluster_results/job/<tag>/01/build/ccomp/`。切除集：

- `charset`：`Charset.lookup2` + `Charset$2.run`（原第 5 项口径，全部按名查找）；
- `cshost`：宿主名来源——initPhase1 jnu 区段 `Charset.isSupported`（`boot_region` 根）、`System.newPrintStream@24`（stdout / stderr 编码）、`sun/nio/fs/Util.<clinit>@5`（jnu）；
- `csext`：`Charset.lookupExtendedCharset` + `lookupViaProviders`（扩展 / 类路径提供者）；
- `csstd`：`sun/nio/cs/StandardCharsets.lookup` 整方法（标准提供者按名反射取类，名字全部收窄的上界）。

| 例 | 基线 类 / 方法 | charset | cshost | csext | csstd |
|---|---:|---:|---:|---:|---:|
| HelloWorld | 3324 / 19847 | −149 / −519 | 0 / −1 | −5 / −21 | −141 / −484 |
| CollectorsDemo | 3324 / 19853 | −149 / −519 | 0 / −1 | −5 / −21 | −141 / −484 |
| DeepCopy | 3573 / 22505 | −148 / −518 | 0 / −1 | −5 / −21 | −141 / −485 |

charset 的 −149 = `sun/nio/cs` 138 + `sun/util/PreHashedMap*` 6 + `java/nio/charset` 5（`Charset$1` / `$2` / `ExtendedProviderHolder` / `$1` / `ThreadTrackHolder`，即 csext 的 5 类）。

结论：**门是两类名字来源的多连通门，在 U1 下合法；本分支不改分析器，第 5 项挂起待 U1 决定。** 依据：

1. **名字来源。** 标准字符集全集都经 `StandardCharsets.lookup@122` 的 `Class.forName("sun.nio.cs." + cln)` 进入，`cln` 取自 `classMap`（`PreHashedMap`，D8 常量表给出全部值）。进入 `lookup` 的名字有两类：
   - 宿主名（U1：运行期读取）：jnu 区段 `isSupported`、`newPrintStream` 的 stdout / stderr 编码（`newPrintStream` 是残余调用，名字不折叠）、`Util.<clinit>` 的 jnu；
   - 常量名：`Charset.defaultCharset@17`（pc 14 已折为 `"UTF-8"`）、`NetworkClient.<clinit>` 的 `file.encoding` 链、`HttpClient` 的 `PrintStream(…, encoding)` 等。
2. **只切宿主名 Δ = 0。** cshost 切除后，`lookup` 改由 `Charset.defaultCharset` → `StandardCharsets.charsetForName` 首达（来路 `Thread.start0` → 信号处理 → `Shutdown.exit` → `System.getLogger` → `ICUBinary` 资源读取 → `String.getBytes()`）。名字是常量 `"UTF-8"`，但分析器没有字符串常量上下文：`lookup(name)` 只分析一份，`classMap.get` 取全部值。
3. **只做常量名收窄同样 Δ = 0。** 宿主名在每个程序都存在，它们合法地链入全部标准字符集。csstd −141 是「两类来源都收窄」的上界，兑现需要两件事同时成立：
   - **(a) 用户重新审视 U1。** jnu / stdout / stderr 编码改为构建期声明的字符集域，例如清单声明「可选宿主编码集」、运行期宿主值不在集合内时按 initPhase1 已有语义回退 UTF-8，口径类似 GraalVM `AddAllCharsets` 的反面。U14 只钉了 `file.encoding`，不覆盖这三个；
   - **(b) 分析器按字符串常量建上下文。** `Charset.lookup` → `lookup2` → `charsetForName` → `StandardCharsets.lookup` → `canonicalize` / `aliasMap.get` / `classMap.get` 在常量名下克隆，`PreHashedMap.get` 对常量键只取该键的值。这需要常量表从「值集」升级为「键 → 值」（读 `PreHashedMap` 子类 `<init>` 的 `ht` 初始化），并新增字符串选择器上下文（现有 `selector.rs` / `ctxsel.rs` 只克隆 int 常量与 null）。
   
   只做 (b) 不做 (a)，收益为 0 类；只做 (a) 不做 (b)，收益也为 0（第 2 条）。
4. **扩展 / 类路径提供者不能折为空。** csext 的 −5 不健全：
   - 档案含 jdk.charsets，`ExtendedProviderHolder` 的 `ServiceLoader.loadInstalled(CharsetProvider)` 在封闭档案里的构建期事实是 {`ExtendedCharsets`}，不是空集；
   - `lookupViaProviders` 的 `ServiceLoader.load(CharsetProvider, SCL)` 也会枚举引导层的模块提供者，同样得到 `ExtendedCharsets`。
   
   所以「扩展提供者集合为构建期事实」能确定的只是这个集合本身（服务目录已由 `seeds.services` 给出），类数不变。只有档案剔除 jdk.charsets 时，这 5 类才可删，而那属于模块集决定，不是字符集求值。
5. **正确性缺口（与收窄方向相反，本分支未修）。**
   - 现象：`sun/nio/cs/ext/AbstractCharsetProvider.lookup@75` 的 `Class.newInstance` 是反射缺口 `open(java/lang/Class)`（`reflect.gaps` 35 条之一）。`classMap` 是实例字段 `TreeMap`，由 `ExtendedCharsets.<init>` 经 `charset(name, className, aliases)` 以常量实参写入，`[facts.reflect.value_maps]` 只覆盖封存静态 HashMap / CHM。
   - 后果：闭包里只有 `AbstractCharsetProvider` / `ExtendedCharsets` 2 类，没有任何 `sun/nio/cs/ext` 字符集类。运行期 `Charset.forName(只在 jdk.charsets 中的名字)`，例如 `x-IBM930`，以及 `availableCharsets()` 的扩展部分，都会与 JVM 不同。
   - 测试覆盖：现有 e2e 64_charsets_ext 用的 GB18030 / Big5 / Shift_JIS / EUC-JP / EUC-KR 在 Linux JDK 21 属标准提供者，覆盖不到这个缺口。
   - 健全修复的终态：值映射事实扩展到「实例字段映射 + 写入辅助方法的形参常量」，按名取类的候选为全部写入值。修复后闭包会**增加** jdk.charsets 的字符集类，需在语料档案里实测增量。

后续入口：

- 用户就 (a) 作出决定后实施 (b)。验收：`closure_composition_job.sh --cut-sets csstd` 的 −141 为兑现上界，实施后基线应接近 3183 / 3183 / 3432。
- 第 5 条缺口独立立项（反射建模），与收窄无关。

### 7.4 第 7 项 locale 适配器链（2026-10-08，bbeb13a5，分支 locale-build）

作业（jp1 / jp2，产物在 `cluster_results/job/<tag>/01/build/ccomp/`）：

- `lc-cut-2ef758c8`：切除集 `lcfmt` / `lcaux` / `lcadapt`；
- `lc-gates-279572aa`：门排名；
- `lc-enum-751066bc`：枚举常量标记；
- `lc-diag2-751066bc`、`lc-diag5-b30a2622`、`lc-diag6-fbd752ac`、`lc-diag7-fbd752ac`：溯源，用到 `@trace` / `@openorig` / `@in` / `@objs` / `@m`；
- `lc-img-fbd752ac`：修复 (a)(b) 后实测；
- `lc-heap-bbeb13a5`：修复 (c) 后实测，含 `cargo test --release -p closure`（229 passed）。

切除集（`scripts/closure_composition_cuts/`）：

- `lcfmt`：Formatter 取本地化数据的入口；
- `lcaux`：HOST / SPI 两个辅助适配器的全部可达面；
- `lcadapt`：`LocaleProviderAdapter.forType` 整体，即本地化数据一次也不查的上界。

三者都不健全，只作上界。

| 例 | 基线 类 / 方法 | lcfmt | lcaux | lcadapt | (a)(b) fbd752ac | (a)(b)(c) bbeb13a5 |
|---|---:|---:|---:|---:|---:|---:|
| HelloWorld | 3324 / 19847 | 0 / −5 | −15 / −104 | −145 / −1027 | 0 / −6 | **−19 / −132** |
| CollectorsDemo | 3324 / 19853 | — | — | — | 0 / −6 | **−19 / −132** |
| DeepCopy | 3573 / 22505 | 0 / −5 | −14 / −98 | −130 / −978 | 0 / −27 | **−18 / −144** |

（基线 `cs-cut-641d48ce`；切除作业未跑 CollectorsDemo，它与 HelloWorld 的基线逐数相同。）

资源：
- HelloWorld 闭包耗时 1:25 → 1:27，RSS 2.16 → 2.32 GB；
- DeepCopy 闭包耗时 3:38 → 4:18，RSS 4.25 → 4.79 GB。

增量来自修复 (c) 新分出的工厂产物内部对象。

结论：

1. **构建期配置已钉值，不涉及 U1。**
   - `java.locale.providers` 在启动表中不存在，读取折叠为 null，`LocaleProviderAdapter.<clinit>` 的偏好表恒为 [CLDR, JRE]，FALLBACK 只经常量 `forType(Type.FALLBACK)` 进入。
   - 所以 HOST / SPI 进闭包不是配置不确定，而是分析器精度缺口：`forType` 的实参里有 open[Type]，`Class.forName(type.getAdapterClassName())` 取 Type 全部常量的类名。
2. **open[Type] 的三个来源，已全部修复（均为通用精度修复，不含类名特判）。**
   - **(a) 克隆上下文里的枚举 `<clinit>`。** `Type.<clinit>` 被按档位上下文克隆（`#@level:2`），克隆体分配的常量不带枚举常量标记，按普通 Type 抽象对象流入静态字段。
     - 修复：`enum_consts.rs::enum_const_mark` 不再要求 NOCTX，标记只按（类，偏移）区分。
     - 效果：`@trace:Type` 由 11591 条降为 13 条。
   - **(b) 共享空元素数组。** `ArrayList.DEFAULTCAPACITY_EMPTY_ELEMENTDATA`（映像对象 182，长度 0）的元素节点收到全程序 `ArrayList.add` 的写入，汇成 2225 节点的 SCC，再经 `adapterPreference` 迭代流到 `findAdapter`。
     - JVM 数组长度不可变，向零长数组写入必抛越界。
     - 修复：`image_start.rs::image_array_site` 把零长、非占位、无宿主来源的映像数组登记为 `empty_arrays`，与字节码零长分配点同口径，写入暂存而不汇合。`concrete/apply.rs::mat_obj` 的物化数组同样按 `elems.is_empty()` 处理。
   - **(c) 工厂产物的堆上下文截断。** `adapterPreference = Collections.unmodifiableList(list)` 本身按调用点分开（`UnmodifiableRandomAccessList@5377:31#@5363:282`，只包着 `ArrayList@5363:42`）。但它的迭代器 `UnmodifiableCollection$1` 以它为上下文分配，截断到 HEAP_DEPTH=2 后链为 `@7957:0#@5377:31`，丢了调用点。
     - 后果：全程序 `unmodifiableList` 产物的迭代器汇合，`c` 的读取汇集 6 个包装、9 个列表，其中 `Collectors.toList` 的 `ArrayList@72287:4` 收全局 `Optional.value` 与 EnumSet 迭代的 open 值。
     - 修复：`classes.rs::obj_at` 中，分配方上下文是调用点上下文（非对象）时，调用点并入本分配点段（`@5377:31@5363:282`），链长不变、身份含调用点。
3. **兑现量超过 lcaux 上界。**
   - lcaux 只切 HOST / SPI 面（−15 / −104）。(c) 是程序全局的堆精度改进，其余包装视图迭代器的混合也一并消除，HelloWorld 多出 −4 类、−28 方法。
   - (a)(b) 单独只有方法级收益（−6 / −27），因为 (c) 仍让 open[Type] 到达 `forType`。
4. **lcfmt 只有 −5 方法，Formatter 不是独立门。**
   - Formatter 的本地化数据入口与 `BreakIterator`（`ConditionalSpecialCasing.isFinalCased` → `getWordInstance`）、`DecimalFormatSymbols` 等共用 CLDR / JRE 适配器。
   - 关掉任一入口，其余入口仍到达。
5. **残余（lcadapt 上界剩余：HelloWorld −126 类 / −895 方法，DeepCopy −112 类 / −834 方法）是合法数据访问。**
   - CLDR / JRE 适配器及其资源束、`BreakIterator` 规则、`DecimalFormatSymbols` 数据，由宿主默认 locale（U1，运行期读取）驱动，在每个程序里都可达。不能以构建期求值去掉。
   - 能继续收窄的只有精度缺口：
     - `ConditionalSpecialCasing.isConditionMet`：门排名模型 −12、切除实测 −15，条件分支按 locale 语言常量判定；
     - 格式串常量下不走本地化分支：Formatter 的 `%d` 无 `,` 标志时 `localizedMagnitude` 只取零位字符。
   - 两者都需要字符串常量上下文，与 7.3 第 3 条 (b)「分析器按字符串常量建上下文」同一机制，建议并入该项。

抽查建议（(b)(c) 是全局精度改动，影响面超出 locale）：

- locale / 格式：`TestFormatLocale`、`TestLocaleConstants`、`TestStringFormat`、`FormatOutput`、`TestLocaleCurrency`、`TestLocaleNumberCjk`、`TestLocaleDateCjk`、`TestLocaleBundleFamilies`、`TestDateTimeFormat`、`TestLocaleLanguageTag`；
- 集合 / 包装视图 / 收集器：`CollectorsDemo`、`StreamToUnmodifiableCollections`、`TestStreamCollectors`、`TestCollectorsMore`、`CollectorsTeeingTest`；
- 枚举：`TestEnumSetMap`；
- 日志格式：`TestLogHandlerFormat`。

与其他线的交叠：

- **batch 代理二分 `container_elements_per_object`：** (b)(c) 同属容器元素 / 堆上下文精度，合批时注意两者对同一批对象的命名与汇合口径，以合批后的 `closure_composition_job.sh` 实测为准；
- **log-chain：** 日志链 `System.getLogger` 链上的 `unmodifiableList` / `List.of` 产物迭代同样受益于 (c)，其切除上界需在本分支合入后复测；
- **reflect-marker：** `forType` 的 `Class.forName(常量类名)` 现由枚举常量标记的字段值给出（`method_lookup.rs::enum_field_values`），与 Method 查找标记对象直连互不重叠；
- **boot-image-s6 / u12-props：** (b) 依赖映像对象的 `placeholder` / `host` 标志判定零长数组是否可被替换；s6 若新增追加对象的种类，需保持这两个标志的语义。`java.locale.providers` 的折叠来自 U12 启动表，不新增属性。

后续入口：

- 本项收口。残余随「字符串常量上下文」立项（与 7.3 (b) 合并）。
- 资源增量（DeepCopy +0.54 GB）在合批全量时观察。若内存不足机器受影响，(c) 的并段规则只在分配方为工厂调用点上下文时生效，可由 `obj_at` 单点调整。

### 7.5 种子链：`Charset.isSupported` 根 → CLDR 适配器 / `new URL(securerandom.source)`（2026-10-08，分支 seed-chain）

任务：收窄从 `[boot_region] 根 Charset.isSupported` 出发、经越界异常消息（`checkIndex` → `Preconditions.outOfBoundsMessage` → `String.format` → `Formatter`）与
`ThreadLocalRandom` 种子（`TLR.<clinit>` → `RandomSupport` → `SecureRandom` → `SeedGenerator`）到达 `LocaleProviderAdapter.forType`（反射建 CLDR 适配器，约 725 类）
与 `new URL(securerandom.source)`（jrt / jar 协议处理器）的链。基于 batch-1011 c9d00b9b。

作业（产物在 `cluster_results/job/<tag>/01/build/ccomp/`）：

- `sc-m1-c9d00b9b`：基线 + 种子链节点切除（`TLR.<clinit>@185`、`SecureRandom.getSeed`、`Preconditions.outOfBoundsMessage`、`SeedGenerator.<clinit>@8`）；
- `sc-m2-d705cb20`：快照读修复后三例；
- `sc-m3-d705cb20`：HelloWorld 九组切除（下表）；
- `sc-g1-d705cb20`：`--gates` 门排名；
- `sc-m4b-c9d00b9b` / `sc-m4n2-6da7e301`：切 `Shutdown.logRuntimeExit` 后，修复前 / 后三例对照；
- `sc-ut-6da7e301`：全量单测。

#### 7.5.1 实测

种子链节点切除（c9d00b9b，HelloWorld 3304 / 18999）：四个节点单切、组合切，类 Δ 全为 0，方法 Δ 0 至 −11。链上每个节点都不是独立门。

HelloWorld 九组切除（d705cb20）：

| 切除 | 类 / 方法 | 说明 |
|---|---:|---|
| 基线 | 3304 / 18999 | |
| `StringLatin1.toLowerCase@95`（特殊大小写分支） | 3304 / 18998 | |
| `ConditionalSpecialCasing` 四个入口 | 3288 / 18882 | −16，见 7.5.3 第 4 条 |
| `LocaleProviderAdapter.forType` | 3153 / 17888 | CLDR 仍在（经其他入口） |
| **`Shutdown.logRuntimeExit`** | **537 / 1746** | 见下 |
| `SecurityConstants.<clinit>` | 3299 / 18984 | |
| `URL.getURLStreamHandler` | 2269 / 13021 | 不健全上界 |
| `Charset.isSupported` | 3304 / 18998 | 根本身不是门 |
| `ConditionalSpecialCasing` + `forType` | 3149 / 17867 | |
| 以上全部 | 537 / 1746 | 与单切 `logRuntimeExit` 相同 |

结论：**快照修复（7.5.2）之后，(L) 链 `Shutdown.logRuntimeExit` 是 HelloWorld 上两个目标的唯一持有者。**

- 切 `logRuntimeExit` 后，`Formatter` / `Locale` / `TLR` / `Preconditions` / `Charset` 仍在闭包（越界异常消息链是 JVM 真实会走的路径，不切），而 `SecureRandom` / `SeedGenerator` / `CLDRLocaleProviderAdapter` / `jrt/Handler` / `LogManager` 全部出闭包。
- 也就是说，`Charset.isSupported` 根经越界消息到 Formatter 这段只拉进 Formatter / Locale 本身；CLDR 适配器的反射构造与 `new URL(securerandom.source)` 都要经 (L) 链（`Logger` → `ResourceBundle` / `ServiceLoader` → `SplittableRandom.<clinit>` → `RandomSupport.initialSeed` → `SecureRandom`；以及 JUL `LogManager` 的 locale / jar URL 访问）才到达。
- boot-image 计划 §5.9.5 与 §5.9.7（log-chain2，同在 c9d00b9b 上）记录的「切 logRuntimeExit 类差 0、路径 A 与 ③ 收益上限 0、不实现」以快照修复前为前提——那时 TLR 种子链是 (L) 链的后备持有者（下表「修复前 + 切」= 3304）。快照修复后两者不再互为后备，路径 A + ③ 的类收益上限变为 HelloWorld −2767、CollectorsDemo −2721，**该决定应重新评估**（§5.9.7 已加注）。

切 `logRuntimeExit` 时修复前后对照（三例，m4）：

| 例 | 修复后基线 d705cb20 | 修复前 + 切 c9d00b9b | **修复后 + 切 6da7e301** |
|---|---:|---:|---:|
| HelloWorld | 3304 / 18999 | 3304 / 18998 | **537 / 1746** |
| CollectorsDemo | 3304 / 18995 | 3304 / 18994 | **583 / 1898** |
| DeepCopy | 3553 / 21572 | 3553 / 21573 | 3553 / 21572 |

（HelloWorld 修复前基线 `sc-m1` 为 3304 / 18999，类集合与修复后相同。）

- **两条链互为后备，单关任一条都是 0。** 修复前切 `logRuntimeExit`，`TLR.<clinit>@185` 的 `new SecureRandom()` 仍把 `SecureRandom` → `SeedGenerator` → `new URL(securerandom.source)` → jar / jrt 协议处理器 → … 整个大集合带进来（3304 不变）；修复后不切 `logRuntimeExit`，(L) 链照样持有（3304 不变）。两者都关，HelloWorld / CollectorsDemo 降到 537 / 583。所以快照修复是必要的一半，另一半是 (L) 链（7.5.3）。
- **DeepCopy 的持有者是同一缺口的另一个入口**：`ObjectInputFilter$Config.<clinit>@38` → `System.getLogger("java.io.serialization")` → `LazyLoggers.getLogger@15` → `getLoggerFromFinder` → `LoggingProviderImpl` → `LogManager` → `Logger.setupResourceInfo` → `ResourceBundle` → `ServiceLoader`（服务类型未定）→ `SplittableRandom.<clinit>` → `RandomSupport.initialSeed` → `SecureRandom.getSeed` → `SeedGenerator`。HotSpot 上调用方同在 java.base，`isSystem` 为真，走惰性 / 替身日志器，同样不加载 `LogManager`。7.5.3 第 1、2 条一并覆盖这两个入口；`logRuntimeExit` 只是其中之一，终态不应按入口切。

#### 7.5.2 已实施：启动快照读不随属性表逃逸失稳（d705cb20 / 6da7e301）

- 缺口：`ThreadLocalRandom.<clinit>` 经 `VM.getSavedProperty("java.util.secureRandomSeed")` 读种子开关。这个读取读的是 VM 启动时保存的属性快照，运行期 `System.setProperty` 改不到它；但分析器把它当普通属性读，HelloWorld 上 Properties 逃逸（`sysprops_unstable.all`）后读值失稳为 Top，`TLR.<clinit>@185` 的 `new SecureRandom()` 分支保活。
- 修复（通用，清单驱动）：`vm_intrinsics.toml` 的属性读者条目新增 `snapshot = true` 标志（`manifest/sysprops.rs::PropRead.snapshot`），标了的读者只按启动表取值，不登记逃逸、不查失稳键（`engine/sysprops.rs::prop_read`、`sysprops_key.rs::absent_read`）。`snapshot` 与 `receiver` 互斥（解析时报错）。登记一条：`VM.getSavedProperty`。
- 效果：`TLR.<clinit>` 常量 pc172 = null、pc177 = false，死区 [183, 236]（`SecureRandom` 分支不可达）。单独看三例类数不变（HelloWorld / CollectorsDemo 3304、DeepCopy 3553），因为 (L) 链仍持有同一大集合；与 (L) 链关闭合起来才兑现（7.5.1 第二表：HelloWorld −2767、CollectorsDemo −2721）。单测 `snapshot_read_ignores_props_escape`（closure_cli.rs）守护折叠本身。
- JDK 21 的 `RandomSupport.secureRandomSeedRequested` 用 `doPrivileged(new GetPropertyAction(k))` 读同一键，不经快照，不受本修复影响（见 7.5.3 第 1 条）。

#### 7.5.3 剩余缺口（关闭 (L) 链所需，均未实施）

HotSpot 上 java.base 调用方（`Shutdown.logRuntimeExit`、`ObjectInputFilter$Config.<clinit>` 等）的路径：`System.getLogger`→ `LazyLoggers.getLogger` → `DefaultLoggerFinder.isSystem(module)` 为 true → `getLazyLogger` → `useLazyLoggers()` → `JdkLazyLogger` → `BootstrapLogger.getLogger` → `useSurrogateLoggers()` → `SurrogateLogger`，不加载 `LogManager`。分析器上已折叠：`BootstrapLogger.isBooted()` = true、`useLazyLoggers` 的 CUSTOM 分支、`useSurrogateLoggers` 的 `detectedBackend == JUL_DEFAULT`。仍走偏处：`LazyLoggers.getLogger@15` 因 `isSystem` 未折叠而进 `getLoggerFromFinder` → `LoggingProviderImpl` → `LogManager`。所缺机制：

1. **转发克隆的按调用点返回值。** `doPrivileged` / `executePrivileged` 已按调用点克隆（G1，`forward.rs`），但克隆体的返回值在 `rvals` 按成员汇合，调用点取到的是全部特权块返回值之并（Top）。
   `isSystem` = `doPrivileged(new DefaultLoggerFinder$1(m))`（`run()` = `VM.isSystemDomainLoader(m.getClassLoader())`），`RandomSupport.secureRandomSeedRequested` = `doPrivileged(new GetPropertyAction(k))`（各克隆内已折叠为 null），都卡在这里。
   终态做法：方法节点返回值按节点记录（现有 `nret` 只记实例方法，扩到静态克隆节点）；`Oracle::invoke_result` 带上调用点偏移，被调是分派转发方法时按 `static_ctx` 同一规则求出克隆节点、取其节点返回值，并按节点登记依赖。需要改 absint 的 Oracle 接口与依赖复核，属结构性改动，本分支未做。落点：
   - `absint/step.rs` 的 invoke 分支已有指令偏移 `off`，`Oracle::invoke_result` 增加该参数（4 处实现：`engine/facts/oracle.rs`、`engine/ctor_init.rs` 与两个测试桩）；
   - `Facts` 只持 `&Ctx`，查克隆节点需要只读的「(调用方节点, 偏移) → 上下文 id」与「(成员, 上下文) → 节点」映射（`ctxsel.rs::site_ctx_in` 现在边查边建，要拆出只查不建的读法，查不到即退回 `rvals`，健全）；
   - 节点返回值：`obj_rets.rs::obj_ret_note` 现在对静态方法直接返回，改为静态克隆节点也记 `nret`，移到 `Ctx` 供 oracle 读；读者按节点登记新依赖（`Dep::Ret` 只在按成员汇合值变化时复核，汇合值已是 Top 后节点值变化不会通知读者）。
   - 单独落地时类数不变（`RandomSupport` 的读值折叠了，但 (L) 链仍持有大集合），须与第 2、3 条一起验收。
2. **调用方模块。** `System.getLogger` 用 `Reflection.getCallerClass()` 取调用方；要把 `Shutdown.class.getModule()` 折成映像里 java.base 的 Module（类加载器 null），需要 `@CallerSensitive` 调用点按静态调用方给出类字面量，并经静态方法 `LazyLoggers.getLogger` / `isSystem` 传到 `DefaultLoggerFinder$1`（形参常量上下文）。
3. **`logManagerConfigured` 乐观折叠。** 它也决定 §5.9.7 所说的另一条 finder 入口：`JdkLazyLogger` 取用时 `LazyLoggerAccessor.wrapped` → `BootstrapLogger.getLogger(accessor)`，`useSurrogateLoggers()` 为真时建 `SurrogateLogger`，为假才 `createLogger` → `LazyLoggers$1.apply` → `getLoggerFromFinder`。 只由 `redirectTemporaryLoggers` 写 true，而它只经 `LogManager` 可达；需要按「写入点可达才计入」的原始类型静态字段折叠（§5.9.4 所述 ② 式机制），与 1、2 形成互为前提的环，须乐观求解（先假设 false，写入点进闭包再撤销）。
4. **`Locale.ROOT` 的大小写特判。** `SocketPermission.init@302` 等处 `toLowerCase(Locale.ROOT)`，ROOT 的语言是 ""，不可能等于 "tr" / "az" / "lt"，特殊大小写分支永不执行。`ref_eq` 现在只折 null / 映像对象 / 类字面量，不折内容不同的字符串常量；可健全地折为 false。但 `Locale` 是运行期初始化类，ROOT 不是映像对象，`language` 读不出常量；收益上界只有 −16 类（上表 `ConditionalSpecialCasing` 行），优先级低。

`--gates`（`sc-g1-d705cb20`）没有把 `logRuntimeExit` 排为候选：它的排名只取首达链上的枢纽方法（`executePrivileged` 465 非单调、`Charset.lookup` 150、`getLoggerFromFinder` 71 …），而 `logRuntimeExit` 是 `Thread.start0` → Terminator 信号链上的叶子入口。人工切除表是本项的依据。

#### 7.5.4 `param_string_constants_fold_switch` 状态

断言 HelloWorld < 1000 类、闭包不含 `sun/net/www/protocol/jrt/Handler`。当前 3304 类、含 jrt Handler，**仍失败**。切 `logRuntimeExit` 后为 537 类、无 jrt Handler，满足断言——即 (L) 链关闭（U12 ③ 终态）后此测试自然通过；不需要也不应在种子链上另做切除（种子链节点都不是独立门，7.5.1）。

#### 7.5.5 续作入口

- 入口是 boot-image 计划 §5.9.4（U12 ③）：按 7.5.3 第 1 → 2 → 3 条的顺序实施。验收：HelloWorld ≤ 640 类（boot-image 计划 §5.5.6 的 Linux 目标；本测切除上界 537）、CollectorsDemo 同量级（上界 583）、DeepCopy 的 `SecureRandom` / `SeedGenerator` / `LogManager` 为 0（`ObjectInputFilter$Config` 入口随第 1、2 条关闭）、`param_string_constants_fold_switch` 通过。
- 第 1 条同时折叠 `RandomSupport.secureRandomSeedRequested`，使 `SplittableRandom` / `ThreadLocalRandom` 种子路径不再经 `SecureRandom`（JVM 在默认配置下也不走），是独立的正确性 / 精度收益。
- 与其他线的交叠：本分支只改 `sysprops.rs` / `sysprops_key.rs` / `manifest/sysprops.rs` / `closure_cli.rs` 与 `vm_intrinsics.toml` 读者段；与 annot-sig / fix-1010 无文件交叠；charset-ext / reflect-marker 也改 `vm_intrinsics.toml`（不同段，三方合并干净）与本报告（不同小节）。

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
