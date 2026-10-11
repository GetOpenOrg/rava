# E1 内存友好的缺省构建档（build-memsafe，2026-10-11）

前序：`docs/plans/2026-10-04-binary-size.md` §五（B3 档位对照）、§六（B4 暂停记录）。本文是 B4 的续作与收口。

## 一、目标与约束

- **硬约束（用户 10-06 定）**：缺省构建档要让 16 GB 机器构建全部用例。opt 3 + fat LTO 降为可选打包档（`--release-max`）。
- **三测点同报**：运行耗时、构建耗时（含峰值内存）、二进制大小。
- **验收**：在 15 GB 云服务器（sg1 / sg2，作业 cgroup 可用约 10–11 GB）上，用缺省档构建已知最大的几个闭包，全部成功、无 OOM：
  TestHttpLoopbackAsync、TestUrlParsingFaces、DeepCopy、CollectorsDemo。

## 二、终态设计

### 2.1 档位（发射层根 `Cargo.toml`，`generator/crates/emit/src/project/entry.rs`）

| 档位 | 选项 | 配置 | 定位 |
|---|---|---|---|
| dev | 缺省 | debug = "line-tables-only"，`CARGO_INCREMENTAL=0` | 开发、e2e 缺省 |
| dev-opt | `--dev-opt` | 继承 dev，opt-level 1 | 性能类测试 |
| release | `--release` | opt 3 + **thin LTO** + codegen-units 1 + strip | 内存受限的缺省发布档 |
| release-max | `--release-max` | 继承 release，`lto = true`（fat） | 大机器打包（≥ 24 GB） |
| release-small | `--release-small` | 继承 release-max，opt-level "s" | 体积档 |

### 2.2 内存感知作业数（`generator/crates/driver/src/mem_budget.rs`、`mem_probe.rs`）

每次编译执行一遍，build 与 compile 共用。

1. **可用内存 A**：`--build-mem-mb` 显式给定；否则取 min(`MemAvailable`, 各级 cgroup v2 / v1 余量, macOS `vm_stat`)。cgroup 余量中，不活跃文件页不计为占用。
2. **预算** B = 85% × A。
3. **逐 crate 估计单作业 rustc 峰值 E(c)**，系数按档位取（`Model::of`）。E 由三项相加：
   - 代码量 c（MB，不含引导映像）走两段折线 min(a₁ + b₁·c, a₂ + b₂·c)：小 crate 每 MB 贵，大声明层每 MB 便宜；
   - 引导映像模块（纯静态数据）按每 MB 系数计；
   - 超长函数（≥ 1000 行，例如资源束 `getContents` 的大数组字面量）按总行数计。该项只在 codegen-units 1 的 release 系列档位上有超线性开销。

   E 用单作业实测标定。并行时同一 rustc 多出的 codegen 线程同样占用 jobserver 作业槽，每槽带来的内存增量小于一个 crate 的 E，所以「最大的 J 个 E 之和」仍是上界。
4. **可执行文件 crate 单独计**：它要在全部库 crate 的 rlib 就绪后才启动，不与库 crate 并发。最坏峰值 = max(该 crate 的估计（LTO 档位含全程序链接 `Model::link_est`）, 最大的 J 个库 crate 的 E 之和)。
5. **作业数 J** 取使「最大的 J 个库 crate 之和 ≤ B」成立的最大值，范围 [1, 上限]。上限依次取：
   - `CARGO_BUILD_JOBS`；
   - 否则 scratch 向上最近的 cargo 配置 `[build] jobs`（多槽跑批按槽写入）；
   - 否则 CPU 核数。

   单个 crate 的估计已超过 B 时，J = 1 并告警（应换档位或机器）。探测不到内存时 J = 上限。
6. **实测兜底**：cargo 报告 rustc 被 SIGKILL（`signal: 9`，即 OOM Killer），或 cargo 本身被 SIGKILL（超时终止不算）时，作业数退半后重编。已编完的 crate 命中缓存，只重编被杀的和未编的；退到 1 作业仍 OOM 就如实报失败。所用作业数依次记入 `build_status.json` 的 `mem.oom_retries`（`compile_cmd.rs`）。
7. 决定打印为 `[cargo-env] CARGO_BUILD_JOBS=…` 一行，写入 `build_status.json` 的 `mem` 段。旧规则「重型闭包 ≥ 1700 类 → jobs 1」已删除，`Heavy` 只决定缺省超时（600 / 3000 s）。

不做依赖序（反链）精确估计，原因是流水线编译：库 crate 产出 rmeta 后，下游即可开工，而它自己的 codegen 仍在进行。所以只有可执行文件 crate 的「不并发」是确定的；库 crate 之间按最坏情况相加。

## 三、标定数据（作业 e1m-abae4a6e，sg1 / sg2，ref abae4a6e，每档冷编译，运行 3 次取中位数）

逐 crate 峰值由 `RUSTC_WRAPPER=scripts/crate_mem_profile.py` 采集。dev-j1 = `CARGO_BUILD_JOBS=1`，等同旧缺省规则下重型闭包的 dev。

| 闭包 | crate | 代码 MB | 超长函数千行 | dev 单作业 MB | dev 8 作业 MB | release MB |
|---|---|---|---|---|---|---|
| HttpLoopbackAsync | java_base_decl | 29.52 | 0 | 5,966 | 6,264 | **7,441** |
| DeepCopy | java_base_decl | 20.86 | 0 | 4,084 | 4,285 | 5,230 |
| HttpLoopbackAsync | java_base（门面，引导映像 12.44 MB） | 0.01 | 0 | 1,722 | 1,627 | 1,744 |
| DeepCopy | java_base（门面，引导映像 11.2 MB） | 0.00 | 0 | 1,426 | 1,389 | 1,435 |
| 两例 | jdk_charsets | 6.33–6.37 | 3.10 | 1,730–1,839 | 2,334–2,371 | 1,859–1,964 |
| DeepCopy | java_base_body_9（资源束） | 4.55 | 12.67 | 1,849 | **3,043** | **2,980** |
| HttpLoopbackAsync | java_base_body_14（资源束） | 4.91 | 12.67 | 1,967 | 1,976 | 3,231 |
| 两例 | 其余 body（4.2–5.0 MB） | | 0–1.33 | 1,016–1,600 | 1,015–1,588 | 2,013–2,863 |
| 两例 | java_base_decl_1..4（1.2–3.7 MB） | | 0 | 624–1,063 | 622–1,092 | 767–1,217 |
| 两例 | 小模块 crate（0.04–1.3 MB） | | 0 | 265–847 | 266–847 | 305–1,355 |
| DeepCopy | user（release 全程序 thin LTO，全工作区 86.9 MB） | 0.34 | | 1,800 | 1,800 | **5,553** |
| HttpLoopbackAsync | user（release 全程序 thin LTO，全工作区 139.8 MB） | 0.50 | | — | — | **7,633** |

标定所得系数（MB，c 为代码 MB）：

| 档位 | 代码折线 | 引导映像 / MB | 超长函数 / 千行 | 全程序链接 |
|---|---|---|---|---|
| dev / dev-opt | min(520 + 300c, 1350 + 170c) | 120 | 0 | — |
| release | min(750 + 460c, 2000 + 190c) | 130 | 55 | 2300 + 40 × 全工作区 MB |
| release-max / small | 同 release | 130 | 55 | 2500 + 150 × 全工作区 MB（fat LTO，B3 实测 14.5–15.7 GB） |

上表全部 crate 的单作业实测都不超过估计（`scratchpad` 对照脚本逐条核对，0 条低估）。8 作业下 body_9 的 3,043 MB 是并行 codegen 线程叠加所致，规则第 3 条已说明它被作业槽上界覆盖。

旧系数（基数 400 + 75/MB）低估约一倍：声明层估 1,964 MB，实测 4,285 MB；release 全程序链接估 2,803 MB，实测 5,553 MB。
旧系数下 release 取 4 作业：DeepCopy（sg2）cgroup 峰值 9,764 MB，距上限（约 9.9 GB）只差 150 MB；HttpLoopbackAsync（sg1）cgroup 峰值 11,019 MB，可用 11,131 MB，只差 112 MB。dev 取 8 作业时，HttpLoopbackAsync 的 cgroup 峰值为 10,114 MB。两档都只是侥幸没有 OOM。

逐函数体字节（`fn` 区间）不能区分 jdk_charsets（函数体 4.2 MB，release 1,964 MB）与 java_base body（函数体约 2.2 MB，release 约 2,500 MB），没有采用。

## 四、验收与三测点对照

（待填：§4.1 验收作业结果）

## 五、状态

（待填）
