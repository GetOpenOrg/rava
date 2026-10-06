# C4 全量前分层预检（2026-10-06）

> 目标：`--reset` 全量 e2e 前，用分层抽查找出会成片失败的系统性问题并修掉，使全量一次跑通、少返工。
> 分支 `c4-preflight`（基于集成分支 rust-closure-analyzer 5a6332af）。

## 一、超时可放宽（缺省不变）

| 层 | 选项 | 缺省 | 说明 |
|---|---|---|---|
| `rava compile` | `--build-timeout-scale K` | — | cargo build 时限 = 缺省档（普通 600 s / 重型 3000 s）× K；与 `--build-timeout` 互斥 |
| `run_tests.py` | `--transpile-timeout SEC` | 600 | 转译（闭包 + 发射）时限 |
| `run_tests.py` | `--run-timeout SEC` | 300 | 二进制运行时限 |
| `run_tests.py` | `--build-timeout-scale K` | — | 透传给 `rava compile` |
| distribute_tests.py | `--run-tests-args "..."` | 空 | 原样追加到服务器上每例 `run_tests.py -j 1 …` 命令 |
| distribute_tests.py | `--task-timeout S` | config.TASK_TIMEOUT=1800 | 单例总时限（自派发起算），记入租约，续跑沿用 |

- 每例 `[meta]` 行打印实际时限：`timeout=transpile {T}s/build {缺省|缺省×K|Ns}/run {R}s`。
- 登记于 `docs/environment-variables.md`；提交 f8248968（rava / run_tests）与 server_maintenance d62f083（分发透传，向后兼容，单测 39 项过）。

## 二、资源控制结论（不改服务器系统配置）

现有结构已保证大闭包用例独占，无需新增机制：

1. **每服务器同时只跑 1 例**：全量 / 抽查 / 作业共用 `/tmp/rava_dist.lock` 非阻塞 flock，跨实例互斥（忙则调度器等 30 s）。
2. **派发前资源判定**：`_resource_ok` 要求 load/cpu ≤ 0.6 且 MemAvailable ≥ 1500 MB。
3. **cgroup 上限**：每例在 systemd user scope 内运行，MemoryMax = 总内存 − 4 GB（16 GB Oracle 机 ≈ 11891 MB），MemorySwapMax=0；OOM 记为 `OOM（limit=… peak=…）`。
4. **重型档**：声明层生成类 ≥ 1700 → CARGO_BUILD_JOBS=1、构建时限 3000 s。

最近 OOM（TestFieldHandleProvenance / TestJndiNoProvider / TestSerialDefaultSuid，limit 11891M）在 44–53 h 前，其后 D8 / M2 / S7-2 把声明层峰值降到约 6–8 GB。opt 3 fat LTO 打包档（峰值 14.5–15.7 GB）不在 e2e 缺省路径上。

## 三、抽样

- `--per-dir 2`（从 master_passed_jdk21 的 63 目录各抽 2）+ `--tests` 补：
  - 大闭包：DeepCopy、StockTrans、TestSerial*；
  - 慢转译：TestHttpLoopbackSync/Async、TestJndiNoProvider、TestScriptEngineNone；
  - 近期失败：TestModuleLayerDefine、TestClassModuleFace、ABCProblem、StringFormatTest、TestVirtualThreadScale、TestVarHandleArray、TestMultiArray、TestCharsetNamedStreams、LynchBell / FWord / Factorion 等；
  - master 名单外的 12 目录（63_junit … 74_beans_geom）各补 2 例；
  - 本轮新增 e2e 与 default 方法相关用例。
- 放宽参数：`--run-tests-args "--transpile-timeout 1800 --run-timeout 900 --build-timeout-scale 2" --task-timeout 9000`。

| 抽查 tag | ref | 规模 | 结果 |
|---|---|---|---|
| c4pre-f8248968 | f8248968 | 173 | 服务器被其他会话作业占用，1.5 h 仅出 14 例（13 过 / 1 败 ABCProblem），发现回归后停止 |
| c4pre-2d87d368 | 2d87d368 | 179 | 154 过 / 19 败 / 4 无结果（均为语料中已不存在的名字）/ 2 未完；有效完成 173 例，通过率 89.0%；known_failures 判定后「新失败」只剩 4 个慢测试超时与 TestMultiCatchOrder（已修） |
| c4pre-620a5c94 | 620a5c94 | 7 | 6 过（TestMultiCatchOrder、TestExceptionChain、TestLeafStackOverflow、TestSuppressed、TestJndiNoProvider、HelloWorld），TryResourcesException 按 ref 树剔除 |
| c4pre-cjk-958251ab | 958251ab | 1 | TestLocaleDateCjk 同一 hashCode NPE → 非 2d87d368 引入 |
| c4pre-cjk-5a6332af | 5a6332af | 1 | 无效：该提交 run_tests 不认放宽参数；改测 f8248968（c4pre-cjk-f8248968） |

Python 基线对照（`scripts/baseline_diff.py --results …/spot/c4pre-2d87d368`）：基线通过且语料仍在、本次未通过的回归只有 TestMultiCatchOrder 1 例，已由 620a5c94 修复并复测通过。

## 四、失败分诊

| 用例 | 类别 | 根因 | 修复提交 | 复测 |
|---|---|---|---|---|
| ABCProblem（Python 基线内，96 h 前通过） | 回归 / 系统性 | lambda 合成对象为每个闭包接口生成 vtable，default 条目按「本接口自身声明有体」取体；子接口覆盖超接口 default 时（JDK 21 `Pattern$BmpCharPredicate.union` 覆盖 `CharPredicate.union`），超接口 vtable 落回超接口声明，其体不在调用链上即命中存根（`stub: Pattern$CharPredicate.union`），在调用链上则执行错误的 default。printf → Formatter.<clinit> → Pattern.compile 链路均受影响 | 958251ab：各接口 vtable 的 default 条目统一取闭包上的极大声明（JVMS §5.4.6），新增 e2e TestLambdaSubIfaceDefault | c4pre-2d87d368 通过 |
| （探查发现）lambda 接收者调用含外部虚调用的 default | 系统性缺口 | `iface_default_body` 拒绝 default 体内任何未在本接口声明的虚调用（`System.out.println`、`StringBuilder`、参数接口上的调用），这类 default 在载体上无体；实现类经继承展开不受影响，但 lambda 合成对象只能经载体 `__default_<m>` 执行，命中即存根 panic | 2d87d368：只在所有者为本接口 / 传递超接口 / 根超类、且方法不在接口层次内声明时拒绝；新增 e2e TestLambdaDefaultForeignCall | 见 c4pre-2d87d368 |
| FibonacciMatrixExponentiation（不在 Python 基线，Python 侧亦 > 300 s） | 性能 / 既有 | fib(10^7) 的 BigInteger Toom-Cook 乘法 + 递归 toString，debug 档运行极慢；非卡死、非 2d87d368 回归：b093069f / 1079507631c4 / e983141d 三次抽查（均早于 2d87d368、debug 档）已在 300 s 运行时限超时，dev-opt 档实测 121–245 s 通过（2026-09-30 优化方向文档），JVM 约 6–9 s；本次放宽至 900 s 仍超时 | 不修（不改测试）；归资源 / 性能类已知失败，后续由运行期性能线或 dev-opt 档处理 | — |
| FractionReduction（不在 Python 基线） | 性能 / 既有 | debug 档运行慢（优化方向文档：debug > 300 s、release 12.75 s，GraalVM 参照 9.00 s）；非 2d87d368 回归：b093069f（kr2）/ 1079507631c4（sg1）/ e983141d（sg1）三次抽查均在 300 s 运行时限超时，本次 sg2 放宽至 900 s 仍超时 | 不修（不改测试）；归资源 / 性能类已知失败，由运行期性能线处理 | — |
| IQPuzzle（不在 Python 基线） | 性能 / 既有 | 千万级节点 DFS、上亿次对象分配与虚调用，debug 档运行慢（优化方向文档：debug > 300 s、release 4.86 s，GraalVM 参照 3.98 s）；非 2d87d368 回归：b093069f（ubuntu）/ 1079507631c4（sg2）/ e983141d（kr2）三次抽查均在 300 s 运行时限超时，本次 us1 放宽至 900 s 仍超时 | 不修（不改测试）；归资源 / 性能类已知失败，由运行期性能线处理 | — |
| RailwayCircuit（不在 Python 基线） | 性能 / 既有 | debug 档运行慢（优化方向文档：2026-10-01 全量新增 > 300 s 超时用例；debug > 300 s，release 曾失败（JVM 0.81 s），GraalVM 参照 1.88 s）；非 2d87d368 回归：b093069f（kr2）/ 1079507631c4（kr1）/ e983141d（ubuntu）三次抽查均在 300 s 运行时限超时，本次 kr2 放宽至 900 s 仍超时 | 不修（不改测试）；归资源 / 性能类已知失败，由运行期性能线处理 | — |
| StringFormatTest | 基础设施（非回归） | 该例已于 acc13430（e2e 冗余剪枝 184 例）删除，2d87d368 / 264e73ee 树中均无；`master_passed_jdk21.txt` 残留 181 条已剪枝路径，分层抽样与 `--tests` 名单未按 ref 树过滤，派出即「No test files found」（cfgfix-264e73ee 同因）。printf → Formatter 链路由现存 `17_string_advanced/TestStringFormat` 与 ABCProblem 覆盖 | server_maintenance b8435ab：抽查名单按 `--ref` 的 tests/e2e 树剔除不存在用例并记日志 | — |
| TestCharsetAvailable（不在 Python 基线，不在 master_passed） | 已知待办 / 既有 | 运行即抛未捕获 NullPointerException（无 Java 栈）；2026-10-03 入库起从未通过：jmod 覆盖文档记为 run error / R9，m2-f0c12c3b（us1）与 main-charset-e519e22c（jp2）两次抽查同症状失败，二者均早于 2d87d368；与 lambda default 改动无关 | 不在本分支修（jdk.charsets 线） | — |
| TestDomBuildTree（不在 master_passed / Python 基线） | 已知待办 / 既有 | 运行期 `FactoryConfigurationError: Provider ...xerces...DocumentBuilderFactoryImpl not found`（ServiceLoader / 反射按名加载 JAXP 实现类未入档案）；jmod 覆盖文档入库起记为 run error / R1，m2-f0c12c3b（ubuntu，10-04，早于 2d87d368）同一异常且 dyn-compare 完全相同（漏覆盖 134 / 多出 2556，首条 miss 为 xerces JAXPConstants），闭包自 M2 起未变；e2enew-da8abee1（10-03）为另一症状（SecuritySupport lambda 存根） | 不在本分支修（java.xml / 按名加载线） | — |
| TestJunitAssertFamily / TestJunitHamcrestBridge（63_junit，不在 master_passed） | 基础设施 / 既有 | 转译阶段 javac `cannot find symbol`：run_tests 尚无依赖包 classpath（junit4 + hamcrest），63_junit 十例入库起在 run_tests 下即失败 | 不在本分支修（J3 形态接线，docs/plans/2026-10-05-junit-e2e-deps-task.md） | — |
| TestMultiCatchOrder（Python 基线内，master_passed 内） | 回归 / 生成器（早于 2d87d368） | user 编译 E0425：同一 try 的兄弟 catch 复用 javac 同槽名 `e`，7f075394（10-03）把 catch 头绑定登记为已声明后，delta 0 的 `} catch … {` 衔接行不清前一兄弟块的声明，后一 catch 体的首次赋值未升 let；cfgfix-264e73ee（10-05）已同症状失败 | 620a5c94：衔接行（Catch / Else）先丢弃前一兄弟块声明；单测 2 例；档案 469 类发射对照仅此一行变化 | c4pre-620a5c94 |
| TestHttpLoopbackSync / Async（不在 master_passed） | 已知待办 / 既有 | 此前各抽查均 600 s 转译超时；放宽后进到运行期，0.3 s 命中 `native: sun/nio/ch/IOUtil.fdLimit:()I`（ACC_NATIVE 未手写） | 不在本分支修：套接字 native 层（e310dec2 续，jmod 覆盖 java.net.http 线）；已登记 known_failures | — |
| TestLocaleCurrency | 已知（基线缺陷） | `sym=[CN¥]`：CLDR 随 JDK 21 小版本漂移，expected 基线问题；m2-f0c12c3b 同 diff；known_failures 早有条目 | — | — |
| TestLocaleDateCjk（不在 master_passed） | 待定 | 入库起从未通过：e2enew-da8abee1（10-03）为 stub 命中 `LocaleProviderAdapter.getJavaTimeDateTimePatternProvider`；本次为 `DateTimeTextProvider$LocaleStore.<init>` 中 HashMap.put 的 String.hashCode 抛 NPE（键 String 未初始化）。症状变化区间较大，为排除 958251ab / 2d87d368，对 5a6332af 与 958251ab 单例复测 | 958251ab 同一 NPE，非 2d87d368 引入；已登记（jdk.localedata / java.time 文本提供者线） | c4pre-cjk-958251ab（f8248968 复核在跑） |
| TestProtectionDomainFaces（不在 master_passed / Python 基线） | 已知待办 / 既有 | 运行期 `native: BootLoader.getSystemPackageLocation`（ACC_NATIVE 未手写）；refl-c97ceed1、hw-spot-d5f1599a 同症状，更早 refl-6f93f1c6 为存根 | 不在本分支修：C1d a3-L1 BootLoader 归零 / 引导映像第 5 步；已登记 known_failures | — |
| TestRowSetProvider（不在 master_passed / Python 基线） | 已知待办 / 既有 | `MissingResourceException: com.sun.rowset.RowSetResourceBundle`（java.sql.rowset 资源包按名装载未建模）；m2-f0c12c3b 同症状 | 不在本分支修：模块资源包按名装载；已登记 known_failures | — |
| TestVarHandleArray | 基础设施（非回归） | 仓库任何提交中都不存在该用例（git log --all 无记录），系预检名单误带的旧抽查失败名（r1n-sp-5f87b295 同为「No test files found」）；分发侧 b8435ab 起按 ref 树剔除 | — | — |
| TestVirtualThreadScale（不在 master_passed / Python 基线） | 已知 / 既有（时序） | `all sleeping at once: true→false`：debug 档下 1e5 个 VT 创建过慢，「同时休眠」判定不成立；vt6-spot-4a983fe8、vt-int-2602f409、r1n-sp-204f8cbe / 4d937120 等同 diff，dev-opt 档历史输出一致 | 不在本分支修：虚拟线程终态 / 运行期性能线；已登记 | — |
| TestXmlTransform（不在 master_passed / Python 基线） | 生成器缺陷 / 既有潜伏 | java_xml crate E0425 / E0573 16 处：小写 Java 类名（java_cup `lr_parser` / `sym` / `virtual_parse_stack`、xsltc `parser_actions`）的结构体名与其模块名同名，类型位置解析到模块；入库时 XSLTC 未入闭包（run error / R1），闭包扩展后暴露；plan / sam_objects 只影响发射、不改闭包与命名，与 2d87d368 无关 | 不在本分支修：生成器命名线（NameScope 对小写类名的类型 / 模块消歧）；已登记 | — |
| TryResourcesException / VarargsDemo | 基础设施（非回归） | 已于 acc13430 剪枝删除，预检名单误带（TryResourcesException 也在 620a5c94 复测名单中，新分发已按 ref 树剔除） | 分发 b8435ab | — |
| TestModuleLayerDefine / TestClassModuleFace | 已知待办 / 既有 | 引导映像线。TestClassModuleFace（不在 master_passed / Python 基线）2026-10-03 入库起从未通过：java.base 的 Module 未命名（jbase-named / jbase-name / exported-internal / own-loader 四行），e2enew-da8abee1（10-03）、refl-6f93f1c6（10-05）两次早于 2d87d368 的抽查以及 c1d-a3 分支 bae31727 / 60965b11 抽查的 diff 与本次逐行相同，非回归 | 不在本分支修（引导映像 / a3 线） | — |

### 已知失败登记（docs/known_failures.toml）

分诊确认「入库起从未通过、非回归」的用例登记进 `docs/known_failures.toml`，使全量判定不报为新失败（只登记，不改测试、不放宽超时）：

| 用例 | 签名 | 归属线 |
|---|---|---|
| TestModuleLayerDefine | 未捕获 NullPointerException | 引导映像第 3 步 |
| TestClassModuleFace | `+jbase-named=false` | 引导映像第 4 步 |
| TestDomBuildTree | xerces `DocumentBuilderFactoryImpl not found` | java.xml 按名加载 |
| TestCharsetAvailable | 未捕获 NullPointerException | jdk.charsets |
| 63_junit 十例 | `javac 编译失败` | junit e2e 依赖包 J3 |

慢测试运行超时（FibonacciMatrixExponentiation / FractionReduction / IQPuzzle / RailwayCircuit）按清单口径（超时 / 无结果不算已知）不可登记，全量判读时人工按本表剔除；StringFormatTest 已删除，分发侧已过滤。以本清单复判 c4pre-2d87d368 当时结果：90 / 106 通过，已知 5，剩余「新失败」只有上述 4 个慢测试与 StringFormatTest。

## 五、全量建议参数

```
distribute_tests.py --reset … \
  --run-tests-args "--transpile-timeout 1800 --run-timeout 900 --build-timeout-scale 2" \
  --task-timeout 9000
```

并发维持每服务器 1 例（flock）；全量期间建议暂停其他会话的作业，否则服务器全部处于「已有其他实例」等待状态。

判读：用 `server_maintenance/rava/known_failures.py <tag> --known docs/known_failures.toml`（22 条登记）；
四个慢测试（FibonacciMatrixExponentiation / FractionReduction / IQPuzzle / RailwayCircuit）超时按本文第四节人工剔除；
基线对照用 `scripts/baseline_diff.py --results <结果目录>`。全量 ref 取 c4-preflight 并入集成分支后的提交（须含 620a5c94）。

## 六、结论与遗留风险

**可以开跑（go）**，前提：620a5c94（TestMultiCatchOrder 修复）先并入集成分支，全量以含该提交的 ref 发起，并按上节参数与独占服务器执行。依据：

- 2d87d368 / 958251ab 未引入任何回归：173 例有效结果中无一例因接口 default 体落载体而编译失败，ABCProblem 与两例新 e2e 通过；
- 唯一的 Python 基线回归 TestMultiCatchOrder 已修复并复测通过；
- 其余失败全部定性为既有（14 例登记 known_failures、4 例慢测试超时），或名单问题（4 例，分发侧已修）。

遗留风险：

- 服务器被其他会话作业占用时吞吐极低（c4pre-2d87d368 179 例用时约 7 h）；全量 1045 例须暂停其他作业，否则 9000 s 单例时限内排队即可能耗尽。
- 慢测试在 debug 档必超时（4 例 × 900 s 空耗），全量墙钟按此计入；如需缩短，可只对这 4 例用 dev-opt 档（测试源文件声明，属测试改动，需用户定）。
- TestXmlTransform 暴露的小写类名与模块同名问题是生成器缺陷，全量中凡闭包含 XSLTC 的用例都会同样编译失败，需另派生器命名修复。
- 两个 NPE 类登记（TestModuleLayerDefine / TestCharsetAvailable）签名较泛，同测试内新出现的 NPE 也会被判为已知。
- master_passed_jdk21.txt 仍残留 181 条已剪枝路径，--per-dir 抽样依赖分发侧过滤；如全量后重生成 master_passed 可一并清理。
