# 转译闭包分析器病理卡死：六库复现与栈采样（2026-10-13）

> 来源：lib-tests 验证轮（2026-10-12-lib-tests-merge-verification.md §四.1）发现的五个反射/序列化重库
> 转译闭包分析病态（单核 25–82 分钟、RSS 26–66G 不出发射）。首轮由两个并行子代理对五库做
> 受控复现（限时 720s、RSS 上限 20G、available<15G 即杀）+ eu-stack 栈采样，定位自旋点。
> 基线：batch-1014 @22409c66；JVM 侧五例全部正常（病理纯在转译侧）。
> 同日增补第六例：esper（EsperLengthWindowDemo，依赖走 deps-extra 双版本模块锁，故基于
> libtests-dualversion 分支 tip 15e75a69 复现，该分支待合入 batch-1015；护栏随用户新约束
> RSS 20G→16G，时限 720s 与 available<15G 不变）。

## 一、结论摘要

**同一台发动机、三种爆点档位、六例同一签名。** 全部自旋在 `generator/crates/closure/src/engine/`
的不动点求解内，入口链完全一致：

```
main → build_cmd::build_stages → rava::closure_run::analyze → closure::analyze
     → worklist::Engine::run
```

区别只在 `run` 之下哪一层先爆：

| 档位 | 回路 | 病理帧 | 实测例 |
|---|---|---|---|
| 边/hub 层 | `worklist→bytecode::event→invoke→grow_hub` | `pstrs::pstr_edge` 的 `HashMap<PSlot,BTreeSet>` insert（~52 MiB/s） | snakeyaml、esper（stack2/3：`classes::sub←open_roots`、`FlowGraph::id←hub_special`） |
| 流集层 | `worklist→flow::add_to_id→node_grown` | `IdSet::is_subset` 单调用超采样间隔（超线性退化）；`tau::node_tau` 不动点判定 | ognl+javassist、snakeyaml、databind、esper（stack1：`is_subset←add_to_id`） |
| worklist 层 | `run` 主循环自身 | 条目反复重入不退出（内联 memset 大缓冲重建） | fastjson2、spring-aop（另见 `rcall_dispatch` memmove、`classes::sub`） |

与五库回路对照（第六例结论一句话）：esper 与 snakeyaml/databind 同型——**流集层+hub 层混合档**
（早期 hub 增长回路烧边、后期 `is_subset/add_to_id` 在膨胀集上退化，增速 31.5→12.2 MiB/s 递减），
5455 类的六例最大单 jar 只放大体量、未引入新档位。

## 二、六例总表

| 例 | 库（版本/类数） | kill | RSS | 增速 | 闭包自报峰值 | 代表栈 |
|---|---|---|---|---|---|---|
| OgnlCollectionMain | ognl 3.4.14 + javassist 3.33.0-GA（168 类） | 750s（时限） | 14.47G | 16.3 MiB/s 单调 | 815 MiB | `is_subset←flow::flow←link_hub_in`；`node_grown←add_to_id`（hashbrown insert） |
| YamlRoundTripMain | snakeyaml 2.7（230 类） | 749s（时限） | 18.76G | 前期 52 MiB/s、尾 30s 仅 +3M | 966 MiB | `pstr_edge` insert←`hub_bind`；两份相隔 3min 栈**逐帧相同**：`is_subset←add_to_id` |
| FastjsonPathMain | fastjson2 2.0.65（772 类） | 559s（available<15G） | 11.30G | 18.0 MiB/s | 2518 MiB | `worklist::run` 自旋（一份在 memset 内联中） |
| DatabindAnnotationMain | jackson 2.22.3 三件 | 729s（时限） | 14.11G | ~1.0 GiB/min 线性 | 2454 MiB | `is_subset←add_to_id←run`；`node_tau←tau_check_add←add_to←grow_hub` |
| SpringProxyJdkMain | spring-aop/core/jcl 6.2.19（1503 类） | 744s（时限） | 16.49G | 前期 1.4–1.5 GiB/min 尾部趋平 | 921 MiB | `rcall_dispatch`（memmove 中）←`run`；`classes::sub←open_roots←lambda_dispatch←grow_hub` |
| EsperLengthWindowDemo | esper 7.1.0 双版本模块锁（5455 类单 jar，六例最大） | 749s（时限） | 14.70G | 前期 31.5 → 尾段 12.2 MiB/s 递减 | 1004 MiB | `is_subset←add_to_id←run`；`classes::sub←open_roots`；`FlowGraph::id←hub_special←grow_hub` |

## 三、四条共性发现

1. **日志零进展**：六例转译日志都只有 2–7 行，末行均为 `[closure] 按对象物化：…`，此后直至
   kill（559–750s）无任何输出——引擎内部无阶段级进展日志，卡死不可观测。
2. **记账脱钩**：闭包阶段自报峰值 815–2518 MiB，实际 RSS 11.3–18.8 GiB（数倍；esper 例
   1004 MiB vs 14.70 GiB ≈ 14.6 倍）——失控分配发生在自报统计之后的 flow/worklist 循环，
   现行记账不覆盖该段。
3. **超线性退化实证**（Yaml 例）：集合膨胀后 `IdSet::is_subset` 单次调用耗时超过采样间隔
   （两份 3 分钟间隔的栈逐帧相同），且尾段 RSS 近零增长——时间黑洞是集合运算本身而非分配，
   疑似 `add_to_id` 的收敛条件对大 IdSet 失效、工作列表反复重入。
4. **与被译库无关**：六个不同库、类数 168–5455，同一签名；JVM 侧六例全部秒级正常。病理纯在
   闭包引擎，属 D4/V9/V10 引擎线的极端输入（V10 目标单例 ≤60s 的对立端：25–82 分钟）。

## 四、复现规程（后续任何人可重跑）

```bash
git worktree add <dir> batch-1014 --detach && cd <dir>
scripts/fetch_pilot_deps.sh --no-scan
python3 /data/workspace/heavy_lock.py cargo build --release -p driver \
  --manifest-path generator/Cargo.toml --target-dir build/analyzer-target
python3 /data/workspace/heavy_lock.py scripts/lib_pilot_golden.sh <MainName> --emit-only
# 守卫：转译 >720s、RSS>20G、MemAvailable<15G 任一即 kill -9（显式 PID，勿 pkill 模式）
#       （esper 增补轮随用户新约束 RSS 护栏下调至 16G；时限与 available 护栏不变）
# 栈采样：sudo eu-stack -p <PID>   # 注意 yama ptrace_scope=1 挡普通 attach，需 sudo；
#                                   perf_event_paranoid=4，perf 不可用
```

esper 例（EsperLengthWindowDemo）另需两点：worktree 检出 **libtests-dualversion tip 15e75a69**
（deps-extra 双版本模块锁机制在该分支，待合入 batch-1015；fetch 同命令会一并取 deps-extra 两模块）；
入口给 FQCN：`scripts/lib_pilot_golden.sh esper/com.example.esper.EsperLengthWindowDemo --emit-only`
（脚本 [1/3] 的 `java $MAIN` 原样使用入口名，简单名对带包项目 ClassNotFoundException）。

## 五、给引擎线的立项建议（按收益/难度排序）

1. **引擎心跳**：worklist 迭代计数 + 每 N 秒阶段进展日志——先让卡死可观测（成本最低）。
2. **`IdSet` 大集合表示/算法**：`is_subset` 在膨胀集上的超线性是四例（ognl/yaml/databind/esper）
   的时间黑洞——位图化或增量子集判定（脏标记）。
3. **`add_to_id` 收敛条件**：大 IdSet 下疑似失效导致反复重入——加迭代上界/单调性断言定位。
4. **`rcall_dispatch` 拷贝消除**：反射调用池派发处于 memmove 中（大块搬运）。
5. **`pstr_edge` 的 `HashMap<PSlot,BTreeSet>`**：52 MiB/s 的边集膨胀源，评估集合压缩或惰性化。
6. **记账补齐**：flow/worklist 段纳入自报统计，RSS 脱钩即报警。

与 G1 线（g1-ctx2 内存主项）的汇合点：本报告提供的是"哪里在烧"的栈证据，G1 提供"分配了什么"
的对象剖面——两侧对上即可定位具体数据结构。

## 附录：六例病理卡原文

# 转译闭包分析器病理卡死诊断 — OgnlCollectionMain

## 复现命令
```
cd /data/workspace/rava-wt/diag-clo1   # worktree @ batch-1014 (22409c66), detached
python3 /data/workspace/heavy_lock.py scripts/lib_pilot_golden.sh OgnlCollectionMain --emit-only
```
- 转译命令（脚本内展开）：
  `rava build tests/lib_pilot/OgnlCollectionMain.java --java-home .../jdk-21.0.11+10 --clean --deps tests/lib_pilot/deps/target/deps.lock.toml --cp ognl,javassist --out build/ognl_collection_main --stop-after emit`
- 分析器二进制：build/analyzer-target/release/rava（release + debuginfo）

## jar 版本（deps.lock.toml）
- cp 条目 `ognl,javassist` → `ognl:ognl:3.4.14`（pilot-libs/ognl-3.4.14.jar，168 类）+ `org.javassist:javassist:3.33.0-GA`

## RSS/时间轨迹（/tmp/closure-diag/OgnlCollectionMain.rss.tsv）
- 首点 et=119s RSS=4.41 GiB；末点 et=750s RSS=14.47 GiB；平均增速 ~16.3 MiB/s（22 个 30s 采样点，单调上升，无平台期）
- kill 时点：et=750s，kill 原因 **转译时长守卫（>720s）**，kill 时 RSS 14.47 GiB（未触 20 GiB RSS 守卫，available 未低于 15 G）
- kill 方式：kill -9 147743（显式 PID）

## 卡死签名
- /tmp/OgnlCollectionMain_transpile.log 全部内容仅 2 行，最后一行：
  `[closure] 按对象物化：64 个对象，回退后应用 0 组，峰值 815 MiB`
  （上一行 `[jar] ognl ← ognl-3.4.14.jar（168 类）`）
- 停滞时长：从日志产出该行到被 kill，约 **750s 全程无新日志输出**（闭包求解阶段自进入后零进展日志）
- 注意矛盾点：闭包阶段自报"峰值 815 MiB"，但进程 RSS 同期 4.4→14.5 GiB —— 闭包引擎内部记账与实际分配严重脱钩，说明失控分配发生在自报峰值之后的 flow/worklist 循环里。

## 栈采样（eu-stack，sudo；采样时点有 ±30s 网格偏移）
- stack1（et≈390s，/tmp/closure-diag/OgnlCollectionMain.stack1.txt）最深 rava 自身帧：
  ```
  #0 closure::engine::idset::IdSet::is_subset
  #1 closure::engine::flow::Engine::flow
  #2 closure::engine::flow::Engine::feed
  #3 closure::engine::hub::Engine::link_hub_in
  #4 closure::engine::hub::Engine::link_hub
  #5 closure::engine::invoke::Engine::invoke_inner
  #6 closure::engine::invoke::Engine::invoke
  #7 closure::engine::bytecode::Engine::event_inner
  #8 closure::engine::bytecode::Engine::event
  #9 closure::engine::bytecode::Engine::process_bytecode
  #10 closure::engine::worklist::Engine::process
  #11 closure::engine::worklist::Engine::run
  #12 closure::analyze → rava::closure_run::analyze → rava::build_cmd::build_stages → main
  ```
- stack2（et=482s）：eu-stack 与 gdb attach 双双瞬时失败（ptrace 竞态），无有效样本。
- stack3（et=663s，/tmp/closure-diag/OgnlCollectionMain.stack3.txt）最深 rava 自身帧：
  ```
  #0 hashbrown::map::HashMap<.., closure::engine::sets::FxHasher>::insert
  #1 closure::engine::flow::Engine::node_grown
  #2 closure::engine::flow::Engine::add_to_id
  #3 closure::engine::worklist::Engine::run
  #4 closure::analyze → rava::closure_run::analyze → rava::build_cmd::build_stages → main
  ```
- **结论**：两次采样都在 `closure` crate 的 flow/worklist 不动点求解内：一次自旋在 `IdSet::is_subset`（flow 子集判定，经由 invoke→link_hub→feed 的回调链），一次在 `add_to_id → node_grown → HashMap::insert`（节点集单调增长驱动的新分配）。工作列表永不收敛 + 节点集持续长大 = 16 MiB/s 的内存膨胀。

## JVM 侧
- 正常：tests/lib_pilot/golden/OgnlCollectionMain_jvm.txt 共 **8 行**（真 jar 侧秒级完成）。

# 转译闭包分析器病理卡死诊断 — YamlRoundTripMain

## 复现命令
```
cd /data/workspace/rava-wt/diag-clo1   # worktree @ batch-1014 (22409c66), detached
python3 /data/workspace/heavy_lock.py scripts/lib_pilot_golden.sh YamlRoundTripMain --emit-only
```
- 转译命令（脚本内展开）：
  `rava build tests/lib_pilot/YamlRoundTripMain.java --java-home .../jdk-21.0.11+10 --clean --deps tests/lib_pilot/deps/target/deps.lock.toml --cp snakeyaml --out build/yaml_round_trip_main --stop-after emit`

## jar 版本（deps.lock.toml）
- cp 条目 `snakeyaml` → `org.yaml:snakeyaml:2.7`（pilot-libs/snakeyaml-2.7.jar，230 类）

## RSS/时间轨迹（/tmp/closure-diag/YamlRoundTripMain.rss.tsv）
- 首点 et=88s RSS=4.58 GiB；et=178s 8.36 GiB；et=359s 14.82 GiB（前期 ~52 MiB/s）；末点 et=749s RSS=18.76 GiB
- 后期显著减速：et=719→749 仅 +3 MiB，RSS 在 ~18.7 GiB 平台化（未破 20 GiB 守卫）
- kill 时点：et=749s，kill 原因 **转译时长守卫（>720s）**；RSS 18.76 GiB（贴近但未超 20 GiB；available 未低于 15 G）
- kill 方式：kill -9 173068（显式 PID）

## 卡死签名
- /tmp/YamlRoundTripMain_transpile.log 全部内容仅 3 行，最后一行：
  `[closure] 按对象物化：64 个对象，回退后应用 21 组，峰值 966 MiB`
- 停滞时长：**约 749s 全程零新日志**（进入闭包引擎后无任何阶段进展输出）
- 同样矛盾：自报闭包峰值 966 MiB，实际进程 RSS 18.76 GiB —— 失控分配在 flow/worklist 不动点循环内，发生在"按对象物化"统计之后。

## 栈采样（sudo eu-stack；三份齐全）
- stack1（et≈240s）：
  ```
  #0 hashbrown HashMap<PSlot, BTreeSet, FxHasher>::rustc_entry（insert）
  #1 closure::engine::pstrs::Engine::pstr_edge
  #2 closure::engine::hub::Engine::hub_bind
  #3 closure::engine::hub::Engine::hub_recv_in
  #4 closure::engine::hub::Engine::grow_hub
  #5 closure::engine::invoke::Engine::invoke_inner
  #6 closure::engine::invoke::Engine::invoke
  #7 closure::engine::bytecode::Engine::event_inner
  #8 closure::engine::bytecode::Engine::event
  #9 closure::engine::bytecode::Engine::process_bytecode
  #10 closure::engine::worklist::Engine::process
  #11 closure::engine::worklist::Engine::run
  #12 closure::analyze → rava::closure_run::analyze → rava::build_cmd::build_stages → main
  ```
- stack2（et≈482s）与 stack3（et≈663s）**逐帧完全一致**：
  ```
  #0 closure::engine::idset::IdSet::is_subset
  #1 closure::engine::flow::Engine::add_to_id
  #2 closure::engine::worklist::Engine::run
  #3 closure::analyze → rava::closure_run::analyze → rava::build_cmd::build_stages → main
  ```
- **结论**：早期是 hub/边集膨胀（`grow_hub→pstr_edge` 持续 insert `HashMap<PSlot, BTreeSet>`，52 MiB/s）；中后期集合巨大后，同一 `add_to_id` 里的 `IdSet::is_subset` 单次调用即耗时超过采样间隔（两份 3 分钟间隔的采样帧完全相同），且不再新增净内存 —— 典型的 **集合并查/子集判定在已膨胀数据集上的超线性退化**，工作列表不收敛。

## JVM 侧
- 正常：tests/lib_pilot/golden/YamlRoundTripMain_jvm.txt 共 **25 行**（真 jar 侧秒级完成）。

# 转译闭包分析器病理卡死诊断 — FastjsonPathMain

## 复现命令
```
cd /data/workspace/rava-wt/diag-clo1   # worktree @ batch-1014 (22409c66), detached
python3 /data/workspace/heavy_lock.py scripts/lib_pilot_golden.sh FastjsonPathMain --emit-only
```
- 转译命令（脚本内展开）：
  `rava build tests/lib_pilot/FastjsonPathMain.java --java-home .../jdk-21.0.11+10 --clean --deps tests/lib_pilot/deps/target/deps.lock.toml --cp fastjson2 --out build/fastjson_path_main --stop-after emit`

## jar 版本（deps.lock.toml）
- cp 条目 `fastjson2` → `com.alibaba.fastjson2:fastjson2:2.0.65`（pilot-libs/fastjson2-2.0.65.jar，772 类——三例中最大的 jar）

## RSS/时间轨迹（/tmp/closure-diag/FastjsonPathMain.rss.tsv）
- 首点 et=79s RSS=2.86 GiB；末点 et=559s RSS=11.30 GiB；平均增速 ~18.0 MiB/s（17 个采样点）
- kill 时点：et=559s，kill 原因 **available 内存守卫（free -g available=11G < 15）**——本机另有 ~95G 已用负载，进程自身 RSS 11.30 GiB 未触 20 GiB 守卫
- kill 方式：kill -9 184946（显式 PID）

## 卡死签名
- /tmp/FastjsonPathMain_transpile.log 全部内容仅 3 行，最后一行：
  `[closure] 按对象物化：64 个对象，回退后应用 24 组，峰值 2518 MiB`
- 停滞时长：**约 559s 全程零新日志**（与前三例一致：进入闭包引擎后无任何进展输出）
- 自报闭包峰值 2518 MiB（三例最高），实际 RSS 仍数倍于自报值——同样的记账脱钩。

## 栈采样（sudo eu-stack；进程在 et=559 被 kill，只到第 2 档采样，无 stack3）
- stack1（et≈240s）：
  ```
  #0 __memset_avx512_unaligned_erms（libc）
  #1 closure::engine::worklist::Engine::run        ← memset 内联自 worklist 主循环
  #2 closure::analyze → rava::closure_run::analyze → rava::build_cmd::build_stages → main
  ```
- stack2（et≈482s）：
  ```
  #0 closure::engine::worklist::Engine::run        ← 直接自旋在 worklist 主循环体内
  #1 closure::analyze → rava::closure_run::analyze → rava::build_cmd::build_stages → main
  ```
- **结论**：与前两例同一台发动机、不同档位——本例卡在 `worklist::Engine::run` 自身（含大缓冲区 memset/重建），即工作列表条目反复重入（re-enqueue）导致主循环永不退出；fastjson2 的 772 类让 hub/invoke 边数放大，崩在 worklist 层而非更深的 IdSet 判定。

## JVM 侧
- 正常：tests/lib_pilot/golden/FastjsonPathMain_jvm.txt 共 **12 行**（真 jar 侧秒级完成）。

# 闭包分析器病理卡死诊断 —— DatabindAnnotationMain

## 复现命令

```bash
cd /data/workspace/rava-wt/diag-clo2   # worktree：batch-1014（22409c66），detached
python3 /data/workspace/heavy_lock.py scripts/lib_pilot_golden.sh DatabindAnnotationMain --emit-only
# 转译日志：/tmp/DatabindAnnotationMain_transpile.log
# 实际分析器命令行（PID 149772）：
#   build/analyzer-target/release/rava build tests/lib_pilot/DatabindAnnotationMain.java \
#     --java-home .../jdk-21.0.11+10 --clean \
#     --deps tests/lib_pilot/deps/target/deps.lock.toml \
#     --cp jackson-databind,jackson-core,jackson-annotations \
#     --out build/databind_annotation_main --stop-after emit
```

## jar 版本（deps.lock.toml 坐标）

| 条目 | 坐标 |
|---|---|
| jackson-annotations | com.fasterxml.jackson.core:jackson-annotations:2.22（77 类） |
| jackson-core | com.fasterxml.jackson.core:jackson-core:2.22.3（213 类） |
| jackson-databind | com.fasterxml.jackson.core:jackson-databind:2.22.3（812 类） |

## RSS / 时间轨迹（/tmp/closure-diag/DatabindAnnotationMain.rss.tsv）

- 分析器启动 08:04:58，单线程（NLWP=1），**99.6% CPU（单核打满）**
- 起点：etimes≈111s 时 RSS 3.56 GiB（launch 后 javac+JVM golden 先行，转译约在 etimes 0–40s 起步）
- 增长近似线性：**约 +1.0 GiB/分钟**（111s:3.56G → 351s:7.42G → 488s:9.47G → 729s:14.11G）
- **kill 时点与原因：etimes=729s（12 分 9 秒）触发 720 秒时限守卫，kill -9；此时 RSS 14.11 GiB（未到 20 GiB 守卫，available 61 GiB 充足）**
- 守卫触发前日志最后一行已在 etimes≈66s 写出 → 停滞约 663 秒

## 卡死签名

`/tmp/DatabindAnnotationMain_transpile.log` 全文 8 行，最后一行：

```
[closure] 按对象物化：512 个对象，回退后应用 276 组，峰值 2454 MiB
```

- 该行写于 08:06:04（etimes≈66s），此后 **663 秒（≈11 分钟）零日志推进**，直到 729s 被杀
- 前置四行为 64/128/256/512 对象批次的"按对象物化"翻倍序列（峰值 487→598→1258→2454 MiB 同步翻番），卡死发生在 512 对象批次的闭包引擎 worklist 运行中

## 栈采样（eu-stack -p，sudo；gdb/eu-stack 直连被 yama ptrace_scope=1 拦截）

三次采样均单线程、全部落在 generator/crates/closure 的 `engine` 模块内：

| 采样 | etimes | 最深 rava 自身帧（调用链） |
|---|---|---|
| stack1 | ~400s | `closure::engine::idset::IdSet::is_subset` ← `closure::engine::flow::Engine::add_to_id` ← `closure::engine::worklist::Engine::run` |
| stack2 | 488s | `closure::engine::worklist::Engine::run`（直接在 run 主循环） |
| stack3 | 660s | `closure::engine::tau::Engine::node_tau` ← `tau::Engine::tau_check_add` ← `flow::Engine::add_to` ← `hub::Engine::hub_recv_in` ← `hub::Engine::grow_hub` ← `invoke::Engine::invoke_inner` ← `invoke::Engine::invoke` ← `bytecode::Engine::event_inner` ← `bytecode::Engine::process_bytecode` ← `worklist::Engine::process` ← `worklist::Engine::run` |

统一尾部：`closure::analyze` ← `rava::closure_run::analyze` ← `rava::build_cmd::build_stages` ← `rava::build_cmd::run_build` ← `rava::main`。

**核心病理帧**：`closure::engine::tau::Engine::node_tau` / `closure::engine::idset::IdSet::is_subset`（worklist→bytecode→invoke→hub 增长回路内，node_tau 的不动点/子集判定）。

## JVM 侧

正常：`tests/lib_pilot/golden/DatabindAnnotationMain_jvm.txt` 6 行（脚本 [1/3] 阶段完成），卡死纯在 rava 转译侧。

## 结论

与预期病理模式吻合：单核 99.9%、闭包阶段（"按对象物化"512 批次后）无日志推进、RSS 持续线性增长（引擎内部结构泄漏式增长，非分配后释放的瞬时峰值）。

# 闭包分析器病理卡死诊断 —— SpringProxyJdkMain

## 复现命令

```bash
cd /data/workspace/rava-wt/diag-clo2   # worktree：batch-1014（22409c66），detached
python3 /data/workspace/heavy_lock.py scripts/lib_pilot_golden.sh SpringProxyJdkMain --emit-only
# 转译日志：/tmp/SpringProxyJdkMain_transpile.log
# 实际分析器命令行（PID 173701）：
#   build/analyzer-target/release/rava build tests/lib_pilot/SpringProxyJdkMain.java \
#     --java-home .../jdk-21.0.11+10 --clean \
#     --deps tests/lib_pilot/deps/target/deps.lock.toml \
#     --cp spring-aop,spring-core,spring-jcl \
#     --out build/spring_proxy_jdk_main --stop-after emit
```

## jar 版本（deps.lock.toml 坐标）

| 条目 | 坐标 |
|---|---|
| spring-aop | org.springframework:spring-aop:6.2.19（289 类） |
| spring-core | org.springframework:spring-core:6.2.19（1199 类） |
| spring-jcl | org.springframework:spring-jcl:6.2.19（15 类） |

## RSS / 时间轨迹（/tmp/closure-diag/SpringProxyJdkMain.rss.tsv）

- 分析器启动 08:18:25，单线程（NLWP=1），**99.6% CPU（单核打满）**
- 起点：etimes=144s 时 RSS 5.14 GiB（增速比 Databind 例更快，前期约 +1.4~1.5 GiB/分钟）
- 轨迹：144s:5.14G → 354s:9.97G → 504s:12.61G → 654s:16.13G → 684s:16.90G → 744s:16.49G（尾部趋平）
- **kill 时点与原因：etimes=744s（12 分 24 秒）触发 720 秒时限守卫，kill -9；此时 RSS 16.49 GiB（未到 20 GiB 守卫，available 43 GiB 充足）**

## 卡死签名

`/tmp/SpringProxyJdkMain_transpile.log` 全文 14 行，最后一行：

```
[closure] 资源束：1 个调用点（1 个 基名推不出）；基名 [] + 字面量回退 631 个 → 束类 +0、属性文件 +0（累计 1 / 0）
```

- 该行写于 08:18:59（etimes≈34s），此后 **710 秒（≈11.8 分钟）零日志推进**，直到 744s 被杀
- 前置日志已越过"按对象物化：64 个对象"（15 组，峰值 921 MiB）、locale 种子与资源束/按名资源发现阶段；卡死发生在闭包引擎 worklist 主循环（无任何批次推进输出）

## 栈采样（sudo eu-stack -p；普通 eu-stack/gdb 被 yama ptrace_scope=1 拦截）

三次采样均单线程、全部落在 generator/crates/closure 的 `engine` 模块内：

| 采样 | etimes | 最深 rava 自身帧（调用链） |
|---|---|---|
| stack1 | ~264s | `closure::engine::worklist::Engine::run`（直接在 run 主循环） |
| stack2 | ~490s | `closure::engine::reflect_call_pool::Engine::rcall_dispatch`（处于 __memmove_avx512 拷贝中）← `worklist::Engine::run` |
| stack3 | ~660s | `closure::engine::classes::Engine::sub` ← `invoke::Engine::open_roots` ← `lambda::Engine::lambda_dispatch` ← `lambda::lambda_step_in` ← `lambda::lambda_step` ← `invoke::Engine::dispatch_one` ← `hub::Engine::hub_recv_in` ← `hub::Engine::grow_hub` ← `invoke::Engine::invoke_inner` ← `invoke::Engine::invoke` ← `bytecode::Engine::event_inner` ← `bytecode::process_bytecode` ← `worklist::Engine::process` ← `worklist::Engine::run` |

统一尾部：`closure::analyze` ← `rava::closure_run::analyze` ← `rava::build_cmd::build_stages` ← `rava::build_cmd::run_build` ← `rava::main`。

**核心病理帧**：`closure::engine::reflect_call_pool::Engine::rcall_dispatch`（反射调用池派发，伴随大块 memmove）与 `closure::engine::classes::Engine::sub`（lambda 开根路径 open_roots 下的类集差运算）；两例共有的引擎增长回路为 hub::grow_hub → invoke → bytecode::event → worklist。

## JVM 侧

正常：`tests/lib_pilot/golden/SpringProxyJdkMain_jvm.txt` 9 行（脚本 [1/3] 阶段完成），卡死纯在 rava 转译侧。

## 结论

与预期病理模式吻合：单核 99.9%、闭包阶段（资源束发现之后、worklist 主循环内）无日志推进、RSS 持续爬升（前期 1.4-1.5 GiB/min，尾部趋平——与 worklist 队列趋于饱和一致）。

# 第六例：esper — 转译闭包分析器病理卡死诊断（EsperLengthWindowDemo）

> 增补于同日晚间：esper 是 jvm-goldens 批（libtests-dualversion）新点亮的第 6 个病理库——JVM 侧
> golden 已绿，转译闭包爆 57.6G（验证轮实测）。依赖走 deps-extra 双版本模块锁，本卡基于
> libtests-dualversion tip 15e75a69 复现（该分支待合入 batch-1015）；RSS 护栏随用户新约束 20G→16G。

## 复现命令
```
cd /data/workspace/rava-wt/espercard   # worktree @ libtests-dualversion tip 15e75a69, detached
scripts/fetch_pilot_deps.sh --no-scan   # 主模块锁 + deps-extra 两模块一并导出
python3 /data/workspace/heavy_lock.py cargo build --release -p driver \
  --manifest-path generator/Cargo.toml --target-dir build/analyzer-target
python3 /data/workspace/heavy_lock.py scripts/lib_pilot_golden.sh esper/com.example.esper.EsperLengthWindowDemo --emit-only
```
- 入口须给 FQCN：脚本 [1/3] 的 `java $MAIN` 原样使用入口名，简单名对带包项目 ClassNotFoundException。
- 实际分析器命令行（PID 777428，转译日志 /tmp/esper_com.example.esper.EsperLengthWindowDemo_transpile.log）：
  `build/analyzer-target/release/rava build tests/lib_pilot/projects/esper/src/com/example/esper/EsperLengthWindowDemo.java tests/lib_pilot/projects/esper/src/com/example/esper/EsperPatternDemo.java --main com.example.esper.EsperLengthWindowDemo --java-home .../jdk-21.0.11+10 --clean --deps tests/lib_pilot/deps-extra/esper/target/deps.lock.toml --cp esper,antlr4-runtime,slf4j-api,janino,commons-compiler --out build/com.example.esper.esper_length_window_demo --stop-after emit`

## jar 版本（deps-extra/esper/target/deps.lock.toml，6 条）
| 条目（cp） | 坐标 | 类数 |
|---|---|---|
| esper | com.espertech:esper:7.1.0 | **5455（六例最大单 jar）** |
| antlr4-runtime | org.antlr:antlr4-runtime:4.7 | 218 |
| janino | org.codehaus.janino:janino:3.0.7 | 437 |
| commons-compiler | org.codehaus.janino:commons-compiler:3.0.7 | 28 |
| slf4j-api | org.slf4j:slf4j-api:1.7.25 | 34 |

（锁内另有 cglib:cglib-nodep:3.2.5 未上 cp；cp 合计 6172 类。）

## RSS/时间轨迹（/tmp/closure-diag/EsperLengthWindowDemo.rss.tsv）
- 分析器启动 17:41:22，单线程（NLWP=1），**98.6–99.8% CPU（单核打满）**
- 首点 et=29s RSS=1.52 GiB；et=150s 5.24 GiB；et=240s 6.65 GiB；et=390s 9.78 GiB；末点 et=749s RSS=14.70 GiB
- 分段增速明显**减速**：29–150s ≈31.5 MiB/s → 240–390s ≈21.4 MiB/s → 598–749s ≈12.2 MiB/s（趋平未平台化）
- 轨迹缺口说明：et 243–390s 段监控脚本两次自身崩溃重启（set -u 下 `local` 链式赋值展开顺序、算术
  上下文误用 `-f` 两个 bash 坑），150s 缺口前后数据连续一致；护栏全程在岗，kill 为显式 PID
- 同机背景：期间另有 b1015-fix worktree 的 esper/templates 转译验证进程共跑（外部 24.7G 级），avail
  下行有共因，但本例 kill 判定不受影响（avail 最低 34.1G，远离 15G 护栏）
- **kill 时点与原因：et=749s 触发 720s 时限守卫，kill -9 777428（显式 PID）；RSS 14.70 GiB 未触
  16 GiB 护栏（协调方当天下调 20G→16G 后首例），available 34.1 GiB 未触 15G 护栏**

## 卡死签名
- /tmp/esper_com.example.esper.EsperLengthWindowDemo_transpile.log 全部 7 行，最后一行：
  `[closure] 按对象物化：64 个对象，回退后应用 0 组，峰值 1004 MiB`
  （前置 1 行 jdk + 5 行 [jar] 装载：esper 5455 类等）
- 该行写于 17:41:36（进程 17:41:22 启动，et≈14s），此后 **约 735 秒（12.3 分钟）零日志推进**，直到 749s 被杀
- 同样的记账脱钩：闭包阶段自报峰值 1004 MiB，实际 RSS 14.70 GiB（≈14.6 倍）——失控分配发生在
  自报统计之后的 flow/worklist 循环，与五例一致

## 栈采样（sudo eu-stack -p 777428；三份齐全）
- stack1（et≈448s，补采窗口内）：
  ```
  #0 closure::engine::idset::IdSet::is_subset
  #1 closure::engine::flow::Engine::add_to_id
  #2 closure::engine::worklist::Engine::run
  #3 closure::analyze → rava::closure_run::analyze → rava::build_cmd::build_stages → rava::build_cmd::run_build → main
  ```
- stack2（et≈478s）：
  ```
  #0 closure::engine::classes::Engine::sub
  #1 closure::engine::invoke::Engine::open_roots
  #2 closure::engine::invoke::Engine::invoke_inner → invoke
  #4 closure::engine::bytecode::Engine::event_inner → event → process_bytecode
  #7 closure::engine::worklist::Engine::process → run
  #9 closure::analyze → rava::closure_run::analyze → build_stages → run_build → main
  ```
- stack3（et≈659s）：
  ```
  #0 closure::engine::graph::FlowGraph::id
  #1 closure::engine::hub::Engine::hub_special
  #2 closure::engine::hub::Engine::hub_recv_in
  #3 closure::engine::hub::Engine::grow_hub
  #4 closure::engine::invoke::Engine::invoke_inner → invoke
  #6 closure::engine::bytecode::Engine::event_inner → event → process_bytecode
  #9 closure::engine::worklist::Engine::process → run
  #11 closure::analyze → rava::closure_run::analyze → build_stages → run_build → main
  ```
- **结论**：同一台发动机第 6 例，入口链与前五例完全一致。三份采样横跨两档回路：stack1 是**流集层**
  经典签名（`IdSet::is_subset ← flow::add_to_id ← run`，与 YamlRoundTripMain stack2/3、
  DatabindAnnotationMain stack1 同帧）；stack2/3 是**边/hub 层**增长回路
  （`worklist→bytecode::event→invoke→grow_hub`，deep 端分别为 `classes::sub←open_roots`——与
  SpringProxyJdkMain stack3 同帧——与 `FlowGraph::id←hub_special`）。5455 类的 esper 未引入新档位，
  只把体量放大（六例最大单 jar，30s 内 RSS 即破 1.5G、前段增速 31.5 MiB/s 为六例最高）。

## JVM 侧
- 正常：tests/lib_pilot/golden/esper_EsperLengthWindowDemo_jvm.txt 共 **4 行**（真 jar 侧秒级完成；
  [告警] [] 为 Esper win:length(3) 真实语义，golden 存 JVM 真值，见 libtests-dualversion 5606bb37 提交说明）。

## 结论
第六例确认：病理纯在转译侧闭包引擎，与五库同一 worklist 不动点发动机。esper 例为**流集层+hub 层
混合档**（snakeyaml/databind 同型）：早期 hub 增长回路烧边、后期 `is_subset/add_to_id` 在膨胀集上
退化，RSS 增速 31.5→12.2 MiB/s 递减、749s 时限到点仍未收敛。JVM 侧 4 行正常。

## 优化目标阶梯（2026-10-13 用户裁决，已定）

- **现行档 32G**（用户已裁决的全语料转译上限；T6 全量扫描按此认证）；
- **中期收紧档 16G**：G1 优化落地、T6 基线重扫确认水位后启用；
- **理论地板 ~2–4G**：最终追求——病理族合法不动点状态 1–2.5G（六例自报峰值实测带）+ 精简后引擎开销推算，逐步逼近；
- **节奏**：T6 权威清单 + junit 族三档带 + 本报告六库卡为对账基线；G1 每落一步优化（§五六项）→ 重扫画水位线 → 达到档位即收紧立档。水位表首版随 T6 报告建立，后续每轮增列。
