# GraalVM 参照基线：23 个耗时用例的 JVM 与 native-image 对照（2026-10-04）

测点有三项，并列关注：运行耗时、构建耗时、二进制大小。

> 来源：用户 2026-10-04 在本机实测（另一工作区 graalvm-bench，83ade29），整合进本仓库作为 R1 运行性能与产品对标的外部参照。
> 复现脚本：`scripts/graalvm_bench.sh`。用例清单与 agent 盲区补丁在 `tests/graalvm_bench/cases.tsv`。脚本新增 `REPEAT`（取中位数）与 `PGO=1`（PGO 构建）两项，本报告数据为单次运行、未开 PGO。

## 一、口径

- 环境：macOS arm64，Oracle GraalVM 21.0.12（JVM 与 native-image 同源）。
- JVM 统一用 `-XX:+UseSerialGC`，与 native 缺省 GC 对齐。
- native 统一用 `--no-fallback`，缺省优化级别，不开 PGO。
- 计时：`/usr/bin/time -p` 墙钟，含 JVM 启动（约 0.1 s 量级）。
- 二进制大小：native-image 产出的可执行文件（静态链接，含所需 JDK 运行时子集），单位 MB。`scripts/graalvm_bench.sh` 起 `results.tsv` 每行带 `bin_bytes` 列，rava 条件取 `--release` 可执行文件。
- 用例：12 例纯计算是 R1 的运行超时用例，见 `docs/plans/2026-09-30-optimization-directions.md` §三.4；11 例是反射 / 序列化 / 动态代理较重、rava 转译或编译耗时大的用例。

## 二、纯计算 12 例：JVM 与原生运行

| 用例 | JVM (s) | 原生运行 (s) | 原生 / JVM | 原生构建 (s) | 二进制 (MB) | 输出 |
|---|---:|---:|---:|---:|---:|---|
| LynchBell（R1 标杆） | 0.86 | 3.62 | 4.2× | 24.0 | 5.9 | 一致 |
| Factorion | 0.66 | 1.59 | 2.4× | 23.2 | 5.9 | 一致 |
| FWord | 0.40 | 1.37 | 3.4× | — | 12.9 | 一致 |
| FibonacciMatrixExponentiation | 3.93 | 6.16 | 1.6× | — | 12.9 | 一致 |
| IQPuzzle | 1.50 | 3.98 | 2.7× | — | 12.9 | 一致 |
| FourIsTheNumberOfLetters | 1.90 | 2.77 | 1.5× | — | 12.9 | 一致 |
| FractionReduction | 6.26 | 9.00 | 1.4× | — | 12.9 | 一致 |
| PartitionInteger | 0.46 | 1.36 | 3.0× | — | 12.9 | 一致 |
| PrimorialNumbers | 2.10 | 3.74 | 1.8× | — | 12.9 | 一致 |
| RailwayCircuit | 0.49 | 1.88 | 3.8× | — | 12.9 | 一致 |
| SelfNumbers | 1.86 | 2.27 | 1.2× | — | 12.9 | 一致 |
| UnprimeableNumbers | 0.86 | 4.60 | 5.3× | — | 22.5 | 一致 |

- 原生构建每例 23–41 s。
- 负载本身是秒级（JVM 0.4–6.3 s）。rava debug 构建下运行超过 300 s，差距在生成代码，不在负载。
- 这类长时间热循环负载，原生（无 PGO）比 JVM 慢 1.2–5.3 倍：HotSpot 的 JIT 与逃逸分析占优。

## 三、反射 / 序列化重 11 例：原生构建与运行

| 用例 | JVM (s) | 原生构建 (s) | 原生运行 (s) | 二进制 (MB) | 输出 |
|---|---:|---:|---:|---:|---|
| TestHttpLoopbackSync | 0.60 | 72.4 | 1.15 | 33.1 | 一致 |
| TestHttpLoopbackAsync | 0.57 | 67.5 | 1.38 | 33.0 | 一致 |
| TestJndiNoProvider | 0.04 | 61.4 | 2.05 | 19.4 | 一致 |
| DeepCopy | 0.06 | 43.6 | 0.65 | 15.9 | 一致 |
| StockTrans | 0.05 | 39.3 | 0.66 | 15.9 | 一致 |
| TestSerialDefaultSuid | 0.08 | 39.4 | 0.69 | 16.1 | 顺序差异 ¹ |
| TestSerialLookupPairing | 0.06 | 37.0 | 0.67 | 16.0 | 一致 |
| TestSerialProxyForm | 0.08 | 40.5 ² | 0.49 | 16.0 | 一致 ² |
| TestFieldHandleProvenance | 0.06 | 25.9 | 0.61 | 7.4 | 一致 |
| TestUrlProtocolOpen | 0.08 | 42.3 ² | 0.54 | 20.0 | 协议功能一致，异常文案差异 ² |
| RecordsSerializationTest | 0.08 | 46.1 | 0.70 | 15.6 | 一致 |

¹ 只有 sealed 层级 `getPermittedSubclasses()` 的枚举顺序不同；规范未规定该顺序，语义等价。
² 加补丁后二次构建的数据，补丁见第四节。

## 三之一、二进制大小

- 分布：最小 5.9 MB（LynchBell、Factorion），中位 12.9–16 MB（用到 BigInteger、集合、序列化），最大 33 MB（两个 HTTP 用例，拉入 HttpServer、HttpClient、NIO 全套）。
- 规律：大小取决于可达的 JDK 代码量，与业务代码量无关。用到的 JDK 子系统越重（HTTP、JNDI、序列化），留下的越多。rava 的闭包与档案同样以可达 JDK 代码为主，这组数字可以逐例直接对照。
- rava 起点：本机 LynchBell `--release` 可执行文件 13.3 MB，同例原生 5.9 MB，是原生的 2.3 倍。逐例对照待第六节的同机数据。

## 四、tracing agent 盲区（native-image 需要人工补配置）

1. **TestSerialProxyForm**：ObjectStreamClass 在静态初始化阶段走序列化构造器路径，agent 捕获不到。需在 serialization-config.json 补注册 `TestSerialProxyForm$Tally$Form`。
2. **TestUrlProtocolOpen**：`jar:` 协议要在构建期用 `--enable-url-protocols=jar` 显式开启，agent 不会生成这个开关。开启后 6 个协议操作与 JVM 逐行一致。剩余差异是原生替换实现的 `MalformedURLException` 消息不带 spec 后缀，测试自身按子串解析时越界。

rava 用静态闭包分析决定可达集合（CLAUDE.md 第 2 条），不需要 tracing agent，也不需要配置文件。上面两例必须一直留在 rava 的回归验收集里，这是与 native-image 的差异化能力。

## 五、对 rava 的用途与目标（写入各计划）

| 用途 | 目标 / 用法 | 落点 |
|---|---|---|
| R1 运行性能终态 | 12 例纯计算在 rava `--release` 下运行时间 ≤ 同机 GraalVM 原生（无 PGO）耗时；debug 构建 ≤ 30 s 的原目标保留 | `2026-09-30-optimization-directions.md` §三.4 |
| 构建耗时对标 | 生产模式 `rava build --release` 单例端到端 ≤ 同机 native-image 构建耗时（23–72 s）；档案化后的语料模式逐例目标见 t1-link 方案 | `2026-09-18-product-vision.md` |
| 二进制大小对标 | 生产模式 `rava build --release` 可执行文件逐例 ≤ 同机 GraalVM 原生（缺省优化）大小，即本表 5.9–33.1 MB；HelloWorld ≤ 3 MB 的原目标保留 | `2026-10-04-binary-size.md` §三 |
| 正确性参照 | 两例非规定行为差异（枚举顺序、异常文案）作为同类差异的判定先例 | 本报告 §三 |
| 免配置能力 | 静态初始化中的序列化构造、`jar:` 协议两例保持在验收集 | 本报告 §四 |

## 六、待补测（服务器 / 同机对照）

- 在同一台机器上用 rava `--release` 跑这 23 例，与本表并列，作为 R1 的起点基线。2026-10-04 已发起服务器作业 `timing-rel-10795076`；debug 口径由抽查 `timing-dbg-10795076` 记录逐例耗时（分发脚本 2614a2b 起写 `timings_jdk21.tsv`）。服务器是 Linux x86，与本表不同机，同机对照仍需在本机补 rava `--release` 数据。
- 多条件、多环境补测（2026-10-04 发起，三项测点都记）：
  - 本机 macOS arm64：GraalVM 21 五条件（jvm-serial / jvm-g1 / ni-default / ni-o3 / ni-pgo，REPEAT=3）已完成；GraalVM 25 两条件（jvm-serial / ni-default）已完成；rava-release 在跑。
  - 服务器 Linux x86_64：GraalVM 21 五条件作业 `gvm-linux-bd52b537`（带 `bin_bytes` 列）；rava release 作业 `timing-rel-10795076`，二进制大小取 run_tests 行尾的 `bin` 列。
