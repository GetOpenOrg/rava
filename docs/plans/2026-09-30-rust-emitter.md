# Rust 生成器（发射层）实施计划：单二进制 `rava build`

> 日期：2026-09-30
> 状态：P2 / P3 开工（子代理并行）；P0 / P1 / P4 / P5 待排
> 决策（用户 2026-09-30 拍板）：
> ① Python 生成器用 Rust 重写，与 Rust 闭包分析器合为**一个二进制** `rava build`：闭包分析 → 发射 → cargo，闭包结果进程内传递；管道形态（`rava closure … -o x.json` / `rava emit x.json`）只作调试与审计入口。
> ② **C3（发射层消费 levels / dispatch / folds）直接在 Rust 生成器做**，Python 侧不再投入。
>
> 本文**取代** [2026-09-20-rust-generator-rewrite.md](2026-09-20-rust-generator-rewrite.md) 的「前提决策」与 §3.1 行为冻结条款；
> 其 §2（crate 布局、宏编译期展开、IR 唯一货币、禁令 lint）与 §4（教训 → 硬性规则）继续有效，本文只写差异与执行细节。

---

## 一、终态（量化）

| 指标 | 现状 | 终态 |
|---|---|---|
| 生成器实现 | Python `codegen/` 29k 行 + `scripts/main.py` | Rust `generator/`；`codegen/` 与 `scripts/main.py` **0 行** |
| 入口 | `python3 scripts/main.py`（内部 `cargo run` 闭包分析器 + JSON 落盘） | `rava build <Test.java>` 单进程；闭包结果不落盘（`--dump-closure` 仅审计） |
| 发现逻辑 | 闭包分析器（Rust）为唯一真源 | 同左；发射层零发现逻辑（只消费 `Closure` 结构） |
| C3 | 未做 | 发射层按 `levels` 发 L1 不透明类型、按 `dispatch` 发 vtable 槽、按 `folds` 发常量；闭包剪掉的分支 0 翻译 |
| 模块边界 API | 类型 / 表达式以 String 往返 | 公开 API 零 String 表达式（`RsType` / `RsExpr` 为唯一货币；文本只在 render 出口产生） |
| 全局可变状态 | `configure_short_names` 等模块级全局 | 0：命名上下文显式传参 |
| JDK 类名字面量（生成器源码） | 规则 4 靠审查 | lint 单测 0 命中 |

## 二、crate 布局（在既有 workspace 上增量）

```
generator/crates/
├── classfile/   ✅ 已有：.class 解析
├── resolve/     ✅ 已有：classpath / 层次 / jimage
├── closure/     ✅ 已有：精确闭包分析 + 手写层扫描（handwritten.rs 供发射层复用）
├── input/       P1：发射层输入（registry 插入序 / 调用链 visited / 反射面 / 补种 / 折叠与 VM 常量剪枝后的规范化方法体 / 手写扫描 / 逐方法发射判定）（← closure_input / closure_folds / vm_constants / runtime_manifest / transpile 的 registry 构建）
├── ty/          P2：签名解析 + 类型映射 + 层次实参 + 签名类型 + JvmType（← sig_parse / type_map / type_args / sig_types / jvm_type）
├── ir/          P3：RsIR 节点 + render 单出口（← rs_ir / render）
├── sim/  cfg/  instr/   P4：栈模拟 / 结构化 / 指令翻译（← stack / cfg/ / instr/ / method/）
├── emit/        P5：类文件与工程发射（← emitter/ + project_writer）+ C3
└── driver/      ✅ 已有 `rava closure`；P0 加 `rava build` / `rava emit`
```

依赖单向：`classfile ← resolve ← closure`；`classfile ← ty`；`ir` **不依赖** `ty`（render 所需的短名由 `ShortNames` trait 注入，`ty` 实现它）；`sim/cfg/instr ← ty + ir`；`emit ← 全部`；`driver ← emit + closure`。

## 三、阶段

| 阶段 | 内容 | 验收 |
|---|---|---|
| P0 | `rava build` 外壳：javac、scratch overlay（手写靠无宏标记识别）、进程内闭包 → `emit` → cargo；与 P5 同批接通（不经 Python 中转） | 同 P5 |
| P1 | 输入层：`Registry`（BTreeMap，确定性迭代）从 classfile + `Closure` 构建；vm_constants / folds；runtime manifest；手写扫描复用 `closure::handwritten` | 单测 + P2 golden 共用 |
| **P2** | `ty` crate | 类型层 golden 全等（§四）+ `test_jvm_type.py` 41 例转写为 Rust 单测 |
| **P3** | `ir` crate | IR 渲染 golden 全等（§四） |
| P4 | `sim` / `cfg` / `instr`（~1.2 万行，风险最高） | 按方法 A/B：每方法渲染体与 Python 逐字节一致 |
| P5 | `emit`（emitter/ 7.4k 行 + project_writer），切换 `rava build` 纯 Rust | `compare_trees.sh` 验收集 27 例逐字节一致 + raw-audit 一致 + 抽查通过数不降 |
| C3 | 在 `emit` 上做 levels / dispatch / folds 消费 | 抽查（`distribute_tests.py --spot`）通过数 ≥ 基线；raw-audit 不回归 |
| 退役 | 删除 `codegen/`、`scripts/main.py`、golden 转储脚本 | `grep -r codegen` 工作流无残留 |

顺序说明：P5 之前一律以 Python 产物为对照真源（逐字节一致），语义改动（C3、bug 修复）在 P5 切换后只在 Rust 侧做。移植期若发现 Python bug，**不修 Python**，在差异清单登记，P5 切换后由 Rust 修复并以 e2e 验证。

## 四、golden 基建（每阶段的机械验收手段）

Python 模块内部状态大量以字符串往返，Rust 侧不复刻其 API 形态，只对**可观测输出**对照：

- **转储脚本** `scripts/golden/dump_<layer>.py`：在一次正常转译（验收集中的测试）的 registry 上调用被移植模块的公开函数，把「输入 → 输出」写成 JSONL（`build/golden/<layer>/<test>.jsonl`，不入库）。脚本只读调用，**不修改 `codegen/` 语义**；随 Python 一并退役。
- **Rust 对照测试**：`generator/crates/<crate>/tests/golden.rs` 读 JSONL，用同一输入（类名 / 描述符 / 签名，Registry 从 scratch 的 classes 构建）调用 Rust 实现，逐条比对；golden 文件不存在时测试跳过并打印提示（CI 由转储脚本先行生成）。
- **P2 覆盖面**：registry 内每个类的 superclass / superinterface / ancestor 类型实参、outer instance、类型参数与界；每个字段的 `jvm_to_rust` / `parse_field_type`；每个方法的 `parse_method_param_types`、`mangle_name`、`descriptor_to_suffix`、`hierarchy_overloaded_names`、`interface_member_local_name`；`short_cls` 全表；`from_signature` / `from_descriptor` 的 JvmType 规范化输出。
- **P3 覆盖面**：在 `render_file` / `render_fn` / `render_expr` 入口截获 IR 节点，dataclass → JSON（带节点类型标签）与渲染文本成对落盘；Rust 反序列化为 `ir` 节点、渲染、逐字节比对；短名表一并落盘，供 `ShortNames` 注入。

## 五、并行编排（子代理）

- 并行写代码的子代理**各用一个 worktree + 分支**（从 fd6c5351 起）：同目录会互相打断编译、争 `index.lock`、争 cargo target 锁。
- 子代理只编译 `generator/`（`--target-dir` 置于各自 worktree 的 `build/gen-target`），**不跑 e2e**；golden 转储需要的一次转译可跑（单测试，`--no-run`）。
- 每个子代理只新增自己的 crate + 转储脚本 + workspace `members` 一行；不改其他 crate、不改 `codegen/`、不改 `runtime/`。
- 交付：分支上的提交（`cargo test -p <crate>` 与 `cargo clippy -p <crate>` 通过，golden 全等或差异清单明示）。主会话审查后合并回 `rust-closure-analyzer`。
- 当前批次：P2（`rust-emitter-ty`）、P3（`rust-emitter-ir`）。

## 六、硬性规则（在 2026-09-20 方案 §4 基础上）

1. 源文件 ≤ ~600 行、单函数 ≤ 150 行；按职责拆子模块。
2. crate 源码中无 JDK 类名字面量（规则 4）；确需的锚点从 `closure` 的 manifest 读取或经 `constants` 集中且由 lint 登记。
3. 无全局可变状态；无 `unwrap` 掩盖未移植语义（未移植分支返回 `Err`/`todo` 标注的显式错误并出现在差异清单）。
4. 迭代顺序一律确定性（`BTreeMap` / 稳定排序）；与 Python 字典插入序相关的输出以 golden 暴露并在代码中显式建模顺序。
5. 注释、文档、提交信息用中文；标识符英文。
