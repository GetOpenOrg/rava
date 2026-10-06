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
| c4pre-2d87d368 | 2d87d368 | 179 | 见第四节 |

## 四、失败分诊

| 用例 | 类别 | 根因 | 修复提交 | 复测 |
|---|---|---|---|---|
| ABCProblem（Python 基线内，96 h 前通过） | 回归 / 系统性 | lambda 合成对象为每个闭包接口生成 vtable，default 条目按「本接口自身声明有体」取体；子接口覆盖超接口 default 时（JDK 21 `Pattern$BmpCharPredicate.union` 覆盖 `CharPredicate.union`），超接口 vtable 落回超接口声明，其体不在调用链上即命中存根（`stub: Pattern$CharPredicate.union`），在调用链上则执行错误的 default。printf → Formatter.<clinit> → Pattern.compile 链路均受影响 | 958251ab：各接口 vtable 的 default 条目统一取闭包上的极大声明（JVMS §5.4.6），新增 e2e TestLambdaSubIfaceDefault | 见 c4pre-2d87d368 |
| （探查发现）lambda 接收者调用含外部虚调用的 default | 系统性缺口 | `iface_default_body` 拒绝 default 体内任何未在本接口声明的虚调用（`System.out.println`、`StringBuilder`、参数接口上的调用），这类 default 在载体上无体；实现类经继承展开不受影响，但 lambda 合成对象只能经载体 `__default_<m>` 执行，命中即存根 panic | 2d87d368：只在所有者为本接口 / 传递超接口 / 根超类、且方法不在接口层次内声明时拒绝；新增 e2e TestLambdaDefaultForeignCall | 见 c4pre-2d87d368 |
| FibonacciMatrixExponentiation（不在 Python 基线，Python 侧亦 > 300 s） | 性能 / 既有 | fib(10^7) 的 BigInteger Toom-Cook 乘法 + 递归 toString，debug 档运行极慢；非卡死、非 2d87d368 回归：b093069f / 1079507631c4 / e983141d 三次抽查（均早于 2d87d368、debug 档）已在 300 s 运行时限超时，dev-opt 档实测 121–245 s 通过（2026-09-30 优化方向文档），JVM 约 6–9 s；本次放宽至 900 s 仍超时 | 不修（不改测试）；归资源 / 性能类已知失败，后续由运行期性能线或 dev-opt 档处理 | — |
| FractionReduction（不在 Python 基线） | 性能 / 既有 | debug 档运行慢（优化方向文档：debug > 300 s、release 12.75 s，GraalVM 参照 9.00 s）；非 2d87d368 回归：b093069f（kr2）/ 1079507631c4（sg1）/ e983141d（sg1）三次抽查均在 300 s 运行时限超时，本次 sg2 放宽至 900 s 仍超时 | 不修（不改测试）；归资源 / 性能类已知失败，由运行期性能线处理 | — |
| TestModuleLayerDefine / TestClassModuleFace | 已知待办 | 引导映像线 | 不在本分支修 | — |

## 五、全量建议参数

```
distribute_tests.py --reset … \
  --run-tests-args "--transpile-timeout 1800 --run-timeout 900 --build-timeout-scale 2" \
  --task-timeout 9000
```

并发维持每服务器 1 例（flock）；全量期间建议暂停其他会话的作业，否则服务器全部处于「已有其他实例」等待状态。

## 六、遗留风险

- 服务器被其他作业占用时抽查 / 全量吞吐极低（本次 1.5 h 仅 14 例）。
- 2d87d368 让更多 JDK 接口 default 体落到载体，编译风险以抽查结果为准。
