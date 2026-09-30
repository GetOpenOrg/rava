# P5b 工作记录：method crate 接入 emit，`rava build` 生成完整工程

## 步骤 1：方法体适配器 + emit golden 切真实方法体

- `src/method_bodies.rs`：`MethodBodies` 实现 `MethodBodyEmitter`
  - 字节码 / 局部变量表取自出处类（`declaring_class`），接口方法展开按 `type_var_view` 代换局部变量签名；
  - `InstrHooks::sam_ctor_path` 经 `SamLedger::site_ctor_path`；
  - `MethodSink` → `BodyEffects`（inherited / lambda_ref / sam_site），审计项与控制流审计留在 `BodyAudit`；
  - 错误映射：`MethodError::Cfg` 非 strict → `BodyError::Fallback`（与 Python fallback 白名单一致），其余 Fatal。
- `tests/golden.rs`：`Checked` 包装真实生成器，逐次与 bodies.jsonl 对照（文本 / 兜底 / 副作用），
  返回哨兵包裹文本以复用文件对照。
- 结果：TestHashMapOps 660 次 / 315 文件、TestStreamBasic 1310 / 452、TestCompletableFuture 7321 / 1354，全部失配 0；
  method golden 三例文本 / 登记失配 0。

### 同批并入（unported 盘点 2026-09-30）

- U3：`classfile::Operand::InvokeDynamic` 携带常量池下标 `index`，`InstrHooks::indy_cp_index` 删除；
- U9：删除死函数 `method::error::unported`；method §5 出处类为空改为显式 `MethodError::Runtime`；
- 登记更正：instr GOLDEN_DIFF 删除 lambda_args 陈旧行、已知差异 3 改为已消解、补登 BadFloat 与 S1–S6；
  input 第 9 条、method 第 5 条按实际行为重写。

## 步骤 2：`rava build` 缺省真实方法体 + 审计行

- `python3 scripts/main.py <Test.java> --generator rust --no-run` 生成完整工程；HelloWorld 与 Python 树
  逐字节一致（`Cargo.toml` 除外；`closure_input/closure.json` 的 `elapsed_ms` 为计时值，两侧恒不同）。
- 审计行（`emit::audit::audit_lines`）：`[cfg-audit]` / `[equiv-audit]` / `[raw-audit]`（手写三项）/
  `[override-audit]` / `[vm-boundary-audit]`，HelloWorld 与 Python 数值一致。
  - `[raw-audit]` 不输出 `raw_expr` / `raw_stmt` / `type_surgery_*` / `jdk_literals`：均为 Python 实现自身的度量
    （Python IR 构造事件与 Python 源码静态扫描），对 Rust 生成器无对应口径，
    `compare_trees.sh` 的审计行对照在 python ↔ rust 之间必然不同，只对照手写三项。
  - 未输出：`[readability-audit]`（生成文件扫描，可移植）、`[fallback-audit]`（B 组静默降级点）、`[shortname-audit]`。
  - 存根兜底位点统一记为 `body`（发射层不区分 Python 九吞点）。
- rust 路径未支持的 main.py 选项：`--lib`、`--batch`、`--debug`、`--trace-class`、`--precheck-only`、`--raw-sites`。
