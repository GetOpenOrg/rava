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
