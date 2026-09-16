# Per-test Scratch Workspace 方案（手写/生成彻底分离）

> 创建日期：2026-09-16  
> 状态：**已实施并验证**（Phase 1-4 全量落地；60 个 e2e 逐测试结果与旧架构完全一致，零回归）  
> 关联：`2026-09-16-java-class-macro-unified.md`（宏方案）、`2026-09-15-e2e-unresolved-issues.md`（既有债务）

---

## 1 目标

- **生成代码零提交**：所有 `.rs` 生成物不进 git，仓库里只有真源
- **手写代码唯一真源**：集中在指定目录，结构即复制规则
- **每测试独立工作区**：生成文件、手写副本、编译产物全部隔离
- **codegen 零保护逻辑**：scratch 是一次性的，脚本对生成文件不做任何保护

## 2 现状痛点（为什么必须改）

今天连续踩的 4 个坑，全部源于「生成与手写混在同一棵 `output/` 目录树 + 生成文件提交进 git」：

| 坑 | 根源 |
|---|---|
| `_PERMANENT` 把 git 追踪的生成文件冻结在旧宏格式 | 用「git 追踪」判别手写，但仓库里提交的其实是生成物 |
| 单测试运行删掉 600+ 已提交生成文件 | 窄作用域清理 vs 提交的并集语料，两种语义互相打架 |
| 并集语料编译 2830 个既有错误，拖垮一切单测试运行 | 提交的并集语料含大量 out-of-call-chain 存根 |
| mod.rs 声明与磁盘文件不一致（E0583/E0433/E0425） | 磁盘状态 = 历次运行的残留叠加，谁都不知道当前是什么 |

本质：**`output/` 同时承担了「手写源码目录」「生成输出目录」「编译工作区」「git 语料库」四个角色**，互相冲突。方案就是把这四个角色拆开。

## 3 目标布局

```
java_rta/
  codegen/                      # 提交：转译器
  scripts/                      # 提交：main.py / run_tests.py
  tests/e2e/                    # 提交：测试源
  runtime/                      # 提交：手写唯一真源
    java_runtime/
      Cargo.toml
      src/
        lib.rs  error.rs                      # 基础设施
        java/lang/object.rs                    # 手写类
        java/lang/{object_impl,object_ext,string_ext,...}.rs
        java/util/array_list_impl.rs           # companion 手写
        java/util/function/{bi_consumer,binary_operator,function,supplier}.rs
        jdk/internal/.../*.rs                  # internal_lock_impl 等
    java_rta_macros/             # 提交：整个 crate（lib.rs + block.rs + Cargo.toml）
  build/                         # 不提交（gitignore）— scratch 根
    target/                      # 共享编译缓存（CARGO_TARGET_DIR）
    <test_name>/                 # 每测试独立工作区
      Cargo.toml                 # 脚本生成（workspace 根）
      java_runtime/src/          # 手写副本 + 生成类文件 + 生成 mod.rs
      user/src/                  # 生成测试类 + bin
```

`output/` 整体退役删除。

## 4 单测试运行流程（关键顺序）

```
1. 清空 build/<test>/（整树删，无保护）
2. overlay：runtime/java_runtime/src/** → 复制到 build/<test>/java_runtime/src/
   ★ 必须在 codegen 之前：codegen 的 _scan_impl_files 扫描 _impl.rs 生成
     new_format_map（决定哪些方法跳过存根），手写不就位则扫描结果错误
3. codegen 生成：类文件 + mod.rs + user/src + Cargo.toml
   （java_rta_macros 不复制，Cargo.toml 里用 path 依赖指向 runtime/java_rta_macros，
     避免两份副本漂移）
4. CARGO_TARGET_DIR=build/target cargo run --bin <test>
```

**为什么保护逻辑可以全删**：scratch 里没有「别人的东西」——手写永远从 `runtime/` 重新复制，生成永远重新产出。清空 = 正确。

## 5 改动清单

### 5.1 `codegen/emitter/project_writer.py`（净删 ~60 行）

- 删：`_PERMANENT` 硬编码集合
- 删：`git ls-files` 探测 + 生成标记判别（整个 try 块）
- 删：非 batch 清理分支的 marker/`_impl`/`_ext` 豁免逻辑
  （scratch 由脚本层在 codegen 之前清空，codegen 不再清理）
- 删：`_scan_impl_files` 里对 `output/` 的路径假设，改为参数传入
- 保留：mod.rs 磁盘扫描合并逻辑（scratch 内手写+生成共存时，手写模块
  object.rs / X_impl.rs 仍需正确声明——现有 companion/extra_pub 逻辑正好覆盖）
- 新增：`handwritten_dir` 参数（overlay 由脚本层做，codegen 只需知道扫描路径）

### 5.2 `scripts/main.py`

- 新增 `--scratch <dir>`（缺省 `build/<主类snake名>`）
- 转译前执行 overlay（`shutil.copytree(..., dirs_exist_ok=True)`）
- 生成 workspace `Cargo.toml`：`java_rta_macros = { path = "<repo>/runtime/java_rta_macros" }`

### 5.3 `scripts/run_tests.py`

- 每测试 scratch：`build/<test_stem>/`（复用现有 batch 隔离逻辑，改为默认行为）
- 失败保留现场（debug）、成功可选清理（`--keep` 控制，默认保留最近 N 个）
- 统一 `CARGO_TARGET_DIR=build/target`：syn/quote/宏 crate 指纹稳定 → 缓存命中，
  每测试只重编 java_runtime + user（窄语料本来就小）

### 5.4 git

- `.gitignore` 追加 `/build`、`/output`（过渡期）
- `git rm -r --cached output/`（生成物全部出库）
- 手写文件 `git mv` 到 `runtime/`（清单见 §6）
- 删除 `output/` 目录

## 6 手写文件迁移清单（已盘点，共 25 个 .rs + 3 个 Cargo.toml）

| 类别 | 文件 |
|---|---|
| 基础设施 | `src/lib.rs`、`src/error.rs` |
| 手写类 | `java/lang/object.rs` |
| Arch-1 接口存根 | `java/util/function/{bi_consumer,binary_operator,function,supplier}.rs` |
| companion `_impl` | `java/lang/{object_impl,string_ext,system_impl,double_impl,throwable_impl,thread_impl,math_impl,float_impl,null_pointer_exception_impl}.rs`、`java/util/array_list_impl.rs`、`java/io/print_stream_impl.rs`、`jdk/internal/misc/internal_lock_impl.rs`、`jdk/internal/util/{arrays_support_impl,preconditions_impl}.rs` |
| companion `_ext` | `java/lang/object_ext.rs` |
| 宏 crate | `java_rta_macros/`（Cargo.toml + lib.rs + block.rs） |
| 构建文件 | 根 Cargo.toml（模板化）、java_runtime/Cargo.toml |

判别规则（迁移脚本用）：`.rs` 且不含 `java_rta_macros::java_class` 标记 且非 `mod.rs`。

## 7 收益

1. codegen 的全部启发式保护逻辑消失——今天 4 个坑的温床直接拆除
2. 生成文件永不进 git：无语料震荡、无 500+ 文件的提交、`git status` 永远干净
3. 单测试天然隔离：并集语料的 2830 个既有错误不再拖累任何单测试运行
4. BEFORE/AFTER 回归对比 = 两个 scratch 目录，天然同作用域公平
5. safe-delete 守卫不再触发（build/ 全新目录 + gitignore）

## 8 风险与注意

| 风险 | 应对 |
|---|---|
| overlay 顺序错误（codegen 先跑） | 流程固化在脚本里，main.py 单入口；加断言：codegen 前检查 scratch 里 _impl.rs 已存在 |
| 每测试重编 java_runtime | 共享 target dir 缓存宏与依赖；窄语料编译量小（HelloWorld 级别秒级） |
| rust-analyzer 无固定工作区可挂 | 保留一个常驻 scratch（如 `build/dev/`，跑最近一次的测试），`.rs` 分析指向它 |
| `native_status.toml` 等状态文件位置 | 随手写目录迁移，确认 codegen 读写路径同步改 |
| 手写文件改动后需重跑才生效 | 与现状一致，无回退 |

## 9 实施步骤

```
Phase 1  仓库结构（先行，独立提交）
         ├── 建 runtime/，按 §6 清单 git mv 手写文件
         ├── .gitignore 加 /build、/output
         ├── git rm -r --cached output/（生成物出库）
         └── 此时 codegen 尚未适配 → 仓库暂不可构建（可接受，Phase 2 紧跟）

Phase 2  codegen 简化
         ├── project_writer.py 删保护逻辑（§5.1）
         ├── scratch 清空语义上移到脚本层
         └── handwritten 扫描路径参数化

Phase 3  脚本改造
         ├── main.py：--scratch + overlay + path 依赖 Cargo.toml
         ├── run_tests.py：per-test scratch + 共享 target
         └── 删除 output/ 目录

Phase 4  验证
         ├── HelloWorld 单测试通过（对齐旧窄语料 0 错误基线）
         ├── 代表性 e2e：02_oop / 04_collections / 06_exceptions / 22_autoboxing
         ├── batch 全量 60 测试（隔离 scratch，对比旧并集错误量级不劣化）
         └── 宏改动冒烟：改 block.rs 一行 → 重跑确认生效
```

Phase 1-2 半天内可完成；建议在动 Phase 1 之前先把当前工作区里
`codegen/instr/invoke.py`（JDK 子类过滤）与 `project_writer.py`（mod.rs 磁盘扫描）
两个真实 bug 修复提交掉——它们与工作区布局无关，新方案下同样需要。

---

## 10 实施记录（2026-09-16 晚）

### 落地内容

- Phase 1（commit ff5937b）：runtime/ 建立（22 个手写 .rs + build.rs + 宏 crate 整体 git mv），output/ 出库 667 文件，.gitignore /output
- Phase 2：project_writer.py 删除全部保护/清理逻辑（_PERMANENT 硬编码、git ls-files 探测、非 batch 清理、user 清理），保留 _write 标记检查作为 overlay 冲突消解
- Phase 3：main.py 新增 prepare_scratch（overlay + 占位 mod.rs + Cargo.toml 路径重写 + --clean）；run_tests.py per-test scratch + 共享 CARGO_TARGET_DIR；CLAUDE.md/README 同步更新
- Phase 4：验证见下

### 实施中发现的新问题：共享 target 元数据哈希碰撞

多 scratch 共享 CARGO_TARGET_DIR 时，cargo 以「包名+版本+依赖」计算 artifact 元数据哈希。
不同 scratch 的同名同版本路径包（java_runtime / user）哈希相同 → **跨工作区复用陈旧
artifact**（表现为幻影编译成功/幻影模块缺失，极具迷惑性）。

修复：`codegen/constants.py` 新增 `scratch_pkg_version(out_dir)` —— 版本号带 out_dir 的
CRC32。同一 scratch 复跑版本不变（增量缓存有效），不同 scratch 互不碰撞；syn/quote 与
绝对路径的宏 crate 仍全局共享缓存（编译一次）。

### 验证结论（Phase 4）

| 验证项 | 结果 |
|---|---|
| HelloWorld 端到端 | ✅ PASS（对齐旧窄语料 0 错误基线） |
| TestArrayList 逐错误对比（新 vs 旧架构同 codegen） | ✅ 122 = 122，逐错误码一致（E0308×97 / E0599×17 / E0107×6 / E0597×1），全部为 N-1 既有债务 |
| 全量 60 e2e（-j 4，7m15s） | 10 passed / 50 failed，**与旧架构基线逐测试完全一致（60/60），零回归** |
| 宏改动冒烟 | ✅ runtime/ 宏改动被 scratch 感知并重编 |

50 个失败全部为既有债务（41 编译 + 6 运行时 + 3 输出差异），与旧架构基线集合一致，
追踪见 `2026-09-15-e2e-unresolved-issues.md`。
