# e2e 期望文件按 JDK 版本分目录（2026-10-12 派发，用户 2026-10-13 裁决口径）

分支 `expected-dirs`（基 batch-1014）。裁决口径：

1. `tests/expected/` 按 JDK 版本分目录：`jdk21/`、`jdk25/`……现有平铺期望全部移入 `jdk21/`
   （git mv 保留历史）；
2. 结构可扩展：未来版本各自 `jdk<N>/` 目录，目录名由本次语料 JDK 主版本推导（`--jdk N` /
   `--java-home` / 参考构建任一来路），**不写死 21/25 两值**；
3. `scripts/run_tests.py`：期望读取与生成路径版本化——参考 JVM 生成期望（`--update-expected`）
   写入 `jdk<N>/`；`--jdk N` 运行读 `jdk<N>/`，缺失时按现有缺省生成逻辑（golden JVM
   javac+java）写入 `jdk<N>/` 后照常对比；全部引用平铺路径的脚本/工具/单测一并适配；
4. JDK25 版本差异 4 例（`docs/reports/2026-10-12-jdk25-sem-triage.md` 第二轮「版本差异」清单，
   以报告清单为准）：在参考 JDK 25 下生成 `tests/expected/jdk25/<例>.txt` 提交入库。

## 布局

```
tests/expected/
├── jdk21/                  # 参考构建 21.0.11 生成的期望全集（1144 例，git mv 自平铺）
├── jdk25/                  # 只放版本差异用例（4 例，参考 JDK 25 生成；其余用例运行时按需生成不入库）
└── {record_custom.data, sqrt.dat, stocks.db, test.tmp}
                            # 运行期 fixture 数据文件：e2e Java 源字面引用 tests/expected/<file>
                            # （写入+回读的工作文件，非期望输出），不分版本，保持平铺
```

## 语义

- **读取**：`_expected_path = tests/expected/jdk<主版本>/<Class>.txt`，主版本经 `apply_jdk_choice`
  （`rava jdk --json`）解析后设置 `EXPECTED_LAYER`；主版本解析不出即报错退出（期望无平铺兜底，
  静默回退会把全部测试判成 SKIP）。
- **生成**（`--update-expected`）：golden JVM 输出写入 `jdk<N>/`（版本目录不存在则创建）。
- **实验 JDK 运行缺失补生成**：`--jdk N` / `--java-home`（`JDK_REFERENCE is None`）运行遇期望缺失，
  按 `--update-expected` 同一实现（golden JVM，即**本次语料同一 JDK**）生成写入 `jdk<N>/` 后照常
  diff——JDK25 全量跑会按需把常规用例生成进 jdk25/（内容与 jdk21/ 一致的多数；是否入库由操作者
  决定）。参考构建运行**不**自动生成，缺失维持原缺省 SKIP（新例须显式 `--update-expected` 基线化）。
- **fixture 例外**：4 个数据文件是测试程序的定点输出/回读工作文件（源码字面路径），移入版本目录
  会破坏 e2e 源与转译体两侧，保持平铺（裁决「全部移入」的工程边界：仅指期望输出 .txt）。

## 改动清单（全量）

| 文件 | 改动 |
|---|---|
| `tests/expected/*.txt` → `tests/expected/jdk21/` | 1144 例 git mv（保留历史）；4 个 fixture 数据文件不动 |
| `scripts/run_tests.py` | ①`EXPECTED_LAYER` 全局 + `_expected_path`/`_expected_rel` 版本化；②`_update_expected` mkdir 版本目录后写入；③`_ensure_expected`：实验 JDK 运行缺失→golden 生成入 `jdk<N>/`（顺序/并行两模式的过滤点接入）；④`_diff` 标头、docstring、`--update-expected` help 同步 |
| `scripts/opt_profile_bench.py` | 期望读取 `tests/expected/jdk<.jdk-version>/<name>.txt`（版本取 `.jdk-version` 唯一真源；缺失仍 want=None 只计时） |
| `scripts/diag/vt_repeat.sh` | `EXP=tests/expected/jdk$(cat .jdk-version)/$T.txt` |
| `scripts/direct_mem_bench.sh` / `_multi.sh` / `_profiles.sh` | `write_bench` 期望落位按检出布局自适应：已有 `jdk*/` 版本目录则写入之，基线 ref 旧检出（平铺口径）写平铺——bench 跨基线对照不破 |
| `tools/refjdk.toml` | 头注释：21.0.11 = `tests/expected/jdk21/` 生成版本；其他版本目录由对应 JDK 经 `--java-home` 生成 |
| `runtime/java_runtime/src/jdk_resources/mod.rs` | 文档注释金本位路径 `jdk21/`（纯注释） |
| `README.md` | 目录树行注明版本分目录与顶层 fixture 例外 |

未触及：`scripts/cluster/*`（分发器只调 run_tests，不直接引用期望路径）、`dyn_compare.py` /
`e2e_form.py` / `tests/unit/*`（grep 全仓无平铺期望路径引用；单测 `test_reference_jdk` 导入
run_tests 只用 JDK 选择面，不触期望路径）。

## JDK25 版本差异 4 例（报告 §第二轮 3.7 清单）

`TestParseDoubleEdge`（hex 浮点错误消息）、`TestNativeFrameTrace`（Thread.sleep→sleepNanos 链）、
`TestObjectWaitFrames`（Object.java wait 行号）、`TestSimpleDateFormatRound`（时区短名策略）。
生成口径：服务器 JDK25 构建，`run_tests.py --java-home <jdk25> --update-expected --filter …`
（生成用的确切构建版本见下「验收」）。

## 现状（验收回填）

- [x] 目录迁移 + 脚本适配 + 单测全绿（test_known_failures 的 failure_patterns 第 117 条缺 since
      为**基线既有失败**，主检出同败，与本改造无关）
- [ ] jdk25 4 例生成入库
- [ ] JDK21 5 例混合 PASS（读 jdk21/ 生效）
- [ ] JDK25 4 例版本差异 + 3 例常规 PASS（读 jdk25 / 缺失补生成生效）
- [ ] 期望重生成字节一致（两版本各一例）
- [ ] distribute 抽查模式 2 例冒烟
