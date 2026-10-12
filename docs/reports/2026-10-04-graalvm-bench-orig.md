# GraalVM 耗时验证报告（23 例）

> 原始首轮报告（2026-10-12 自 html-to-markdown/graalvm-bench/REPORT.md 移入，内容原样保留）。
> 整合演进版见 `docs/reports/2026-10-04-graalvm-baseline.md`（同日整合进本仓库，口径重排并补 rava 侧测点）；
> 首轮原始数据见 `tests/graalvm_bench/results-2026-10-04.tsv`，原始脚本见 `scripts/graalvm_bench_orig.sh`。

日期：2026-10-04　环境：macOS arm64 / Oracle GraalVM 21.0.12（JVM 与 native-image 同源）
JVM 侧统一 `-XX:+UseSerialGC`（与 native 默认 GC 对齐）；native 统一 `--no-fallback`；
计时为 `/usr/bin/time -p` 墙钟秒（含 JVM 启动开销，约 0.1s 量级）。
复现：`./run_bench.sh`（tracing agent 自动生成反射/序列化配置，个别缺口已固化补丁）。

## 一、运行耗时大（12 例，纯计算）：JVM vs 原生二进制

| 用例 | JVM (s) | 原生运行 (s) | 原生/JVM | 二进制 | 输出 |
|---|---|---|---|---|---|
| LynchBell（R1 标杆，期望 Number found: 9867312） | 0.86 | 3.62 | 4.2× | 5.9M | 一致 |
| Factorion | 0.66 | 1.59 | 2.4× | 5.9M | 一致 |
| FWord | 0.40 | 1.37 | 3.4× | 12.9M | 一致 |
| FibonacciMatrixExponentiation | 3.93 | 6.16 | 1.6× | 12.9M | 一致 |
| IQPuzzle | 1.50 | 3.98 | 2.7× | 12.9M | 一致 |
| FourIsTheNumberOfLetters | 1.90 | 2.77 | 1.5× | 12.9M | 一致 |
| FractionReduction | 6.26 | 9.00 | 1.4× | 12.9M | 一致 |
| PartitionInteger | 0.46 | 1.36 | 3.0× | 12.9M | 一致 |
| PrimorialNumbers | 2.10 | 3.74 | 1.8× | 12.9M | 一致 |
| RailwayCircuit | 0.49 | 1.88 | 3.8× | 12.9M | 一致 |
| SelfNumbers | 1.86 | 2.27 | 1.2× | 12.9M | 一致 |
| UnprimeableNumbers | 0.86 | 4.60 | 5.3× | 22.5M | 一致 |

原生构建耗时 23~41s/例（已列在 results.tsv）。

**二进制大小分布**（静态链接，含所需 JDK 运行时子集）：
- 最小 5.9M（LynchBell/Factorion）：代码路径极简，Closed World 裁剪最狠
- 中位 12.9~16M：触及 BigInteger/集合/序列化的常规负载
- 最大 33M（HTTP 两例）：JDK HttpServer + HttpClient + NIO 机器全量拉入
规律：二进制大小取决于**可达的 JDK 机器码**而非业务代码量——
用到越重的 JDK 子系统（HTTP/JNDI/序列化），裁剪剩余越多。

**结论 1：JVM 实测全部 0.4~6.3s**，印证了文档"大多几秒内跑完"的估算；
rava debug 构建下 >300s 的差距主要在解释/转译层，不在负载本身。

**结论 2：12 例纯计算原生二进制全部慢于 JVM（1.2×~5.3×）**。
这类长时运行的热循环负载，HotSpot JIT 分层优化 + 逃逸分析的优势明显，
native-image 的 AOT 代码没有运行时 profile 可用，无法追平。
原生镜像的收益在启动与内存，不在峰值算力——这一类用例不适合用来"展示原生性能"。

## 二、转译/编译耗时大（11 例，反射/序列化/动态代理）

| 用例 | JVM (s) | 原生构建 (s) | 原生运行 (s) | 二进制 | 输出 |
|---|---|---|---|---|---|
| TestHttpLoopbackSync | 0.60 | 72.38 | 1.15 | 33.1M | 一致 |
| TestHttpLoopbackAsync | 0.57 | 67.45 | 1.38 | 33.0M | 一致 |
| TestJndiNoProvider | 0.04 | 61.41 | 2.05 | 19.4M | 一致 |
| DeepCopy | 0.06 | 43.61 | 0.65 | 15.9M | 一致 |
| StockTrans | 0.05 | 39.33 | 0.66 | 15.9M | 一致 |
| TestSerialDefaultSuid | 0.08 | 39.38 | 0.69 | 16.1M | 顺序差异* |
| TestSerialLookupPairing | 0.06 | 36.96 | 0.67 | 16.0M | 一致 |
| TestSerialProxyForm | 0.08 | 40.50† | 0.49 | 16.0M | 一致† |
| TestFieldHandleProvenance | 0.06 | 25.85 | 0.61 | 7.4M | 一致 |
| TestUrlProtocolOpen | 0.08 | 42.26† | 0.54 | 20.0M | 部分一致† |
| RecordsSerializationTest | 0.08 | 46.07 | 0.70 | 15.6M | 一致 |

\* 仅 sealed 层级 `getPermittedSubclasses()` 枚举顺序不同（JVM 未规定顺序，语义等价）。
† 修复后的二次构建数据，修复已固化进 `run_bench.sh`。

**结论 3：11 例全部成功构建出原生二进制**（tracing agent + 2 处人工补丁），
构建 26~72s/例，对比 rava 登记 7~10 分钟的编译耗时低一个量级；
反射/序列化/动态代理在 agent 配置下基本开箱即用。

实测发现的两处 agent 盲区（已在脚本固化）：
1. `TestSerialProxyForm`：ObjectStreamClass 在**静态初始化阶段**走的序列化构造器路径
   agent 捕获不到，需在 serialization-config.json 手工补注册
   `TestSerialProxyForm$Tally$Form`
2. `TestUrlProtocolOpen`：`jar:` 协议必须构建期 `--enable-url-protocols=jar` 显式开启，
   agent 不会生成该开关。开启后 6 个协议功能操作（jar/jar-relative/jar-missing/file/
   file-upper/file-parts）输出与 JVM 逐行一致；剩余差异是 MalformedURLException
   消息文案（native 替换实现不追加 spec 后缀，测试自身 substring 解析越界），
   属行为文案差异而非协议功能缺陷。

## 总体结论

- **正确性**：23 例中 21 例原生输出与 JVM 完全一致；1 例仅未规定顺序的枚举差异；
  1 例协议功能一致、异常消息文案差异。
- **适用性判断**：第二类（反射/序列化重）适合 GraalVM 验证路线，构建快、配置可行；
  第一类（纯计算重）在原生下无性能优势，若目标是"跑得动"它们在 JVM 上本就是秒级，
  若目标是"跑得快"则应留在 JVM/JIT 侧。
