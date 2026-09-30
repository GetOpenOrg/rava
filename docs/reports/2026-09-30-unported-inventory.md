# Rust 生成器未移植分支清点（2026-09-30）

- 基线：`triage-0930`（e1646e6d），范围为 `generator/crates/` 下全部 crate：classfile / closure / driver / resolve / ty / ir / sim / cfg / instr / method / input。
- 登记来源：各 crate 的 `GOLDEN_DIFF.md`，以及 `docs/plans/2026-09-30-python-generator-port-diffs.md`。后者只登记了 D1–D3 的 Python 修复移植，没有收录 Unported 条目。
- 方法：
  - grep `unported(`、`Unported`、`未移植`、`TODO`、`stub:`，逐处阅读触发条件，并找出对应的 Python 源。
  - 另外列出不返回 Unported、但会发射不可编译或错值占位的分支（第 3 节）。它们在对照测试中不会计为 unported，却是同一类「未移植」。

## 1. 显式未移植分支清单

| # | 位置 | 错误 | 触发条件 | 对应 Python | 登记 |
|---|---|---|---|---|---|
| U1 | `instr/src/invoke/static_call.rs:188-189` | `InstrError::Unported("invokestatic 目标短名含包路径")` | invokestatic 目标类不在短名表（表外类），导致短名带 `/` | `codegen/instr/invoke.py:856-857`：发射 `/* {cls}.{mname}(args) */` 注释充当调用值（不可编译） | 已登记（instr GOLDEN_DIFF「未移植分支」表） |
| U2 | `instr/src/sim/consts.rs:62-63` | `InstrError::Unported("ldc 常量形态 …")` | `ldc` / `ldc_w` 的常量是 MethodType、MethodHandle 或 Dynamic（condy） | `codegen/instr/sim/consts.py` 末尾的 `else: Lit(f"{operand}i32")` 兜底（语义错误的文本） | 已登记 |
| U3 | `instr/src/sim/dynamic.rs:43-47` | `unported("invokedynamic … 的常量池下标不可得")` | `InstrHooks::indy_cp_index` 返回 None。缺省实现恒为 None，驱动未实现该钩子时**所有** lambda / indy 站点都会命中 | `codegen/instr/sim/dynamic.py`：Python 指令的 operand 本身就是常量池下标 | 已登记（已知差异 3 + 表） |
| U4 | `instr/src/sim/dynamic/lambda_body.rs:22-24` | `unported("接口实现方法 … 无接收者实参")` | 接口实例实现方法的 lambda 调用实参表为空 | Python 对空实参表取下标，抛 IndexError（崩溃路径） | 已登记 |
| U5 | `method/src/types.rs:24-25`（`ir_type_of`） | `MethodError::Unported("类型文本无法解析为 ir::Type")` | 类型文本不能被 `parse_ir_type` 解析。调用点：`blocks/structured.rs:150`、`vars/if_emit.rs:76`、`vars/slot_type.rs:42,54` | Python 直接拼接类型文本，不做结构化解析 | 已登记（method GOLDEN_DIFF「未移植分支」表） |
| U6 | `method/src/vars/if_emit.rs:132` | `MethodError::Unported("变量名 …")` | if 提升出的变量名不是合法的 `ir::Ident` | Python 不校验 | 已登记 |
| U7 | `input/src/norm.rs:135` | `InputError::Format("未移植：{ty} 常量编码 …")` | 折叠常量类型为 J/F/D，且值落到前面各分支之外 | Python 的 `closure_input` 常量折叠能编码 float / double | **部分登记**，见 2.2 |
| U8 | `sim/src/error.rs:27-33`（`From<IrError> for SimError`） | `SimError::Unported(<IrError 文本>)` | `BadIdent` 以外的所有 `IrError` 都被转为 Unported。目前唯一实例是 `ir/src/lit.rs:79` 的 `IrError::BadFloat`：`FloatLit::parse` 收到非数字文本。经 `InstrError: From<IrError>` 从 `instr/src/sim/consts.rs:25,113` 传出 | Python 直接拼接浮点文本 | **未登记** |
| U9 | `method/src/error.rs:78`（`unported()` 辅助函数） | — | 无调用点（死代码） | — | 未登记（无需登记，建议删除） |

**不计入未移植的相近条目**（语义与 Python 一致，或属设计性不移植）：

- `cfg/src/graph.rs:144-145`：jsr/ret 返回 `CfgError("不支持的子例程指令")`，与 `codegen/cfg/graph.py:133` 同语义。
- `input/src/norm.rs:124,126,131,134,136` 和 `input/src/facts.rs:353`：输入格式校验错误。
- ty GOLDEN_DIFF 中的「不移植」项：`registry=None` 模式、`_closure_hit`。
- cfg GOLDEN_DIFF 中的「不移植」项：`parse_switch_operand`、`[cfg-dbg]` 输出。
- instr GOLDEN_DIFF 已知差异 2（`hierarchy._has_subtypes` 死代码）、差异 5（`$assertionsDisabled` 死分支）。
- `instr/src/invoke/static_call.rs:161-173` 的 `emit_unknown_stub`，以及 `instr/src/sim/mod.rs:79` 的 `panic!("stub: unsupported bytecode …")`：这两处是运行期可见的精确存根，符合项目的存根约定。

## 2. 未登记 / 登记失准子集

### 2.1 未登记

1. **U8：`From<IrError> for SimError` 把 BadFloat 映射为 Unported**。
   - sim GOLDEN_DIFF 没有提到这一映射，instr GOLDEN_DIFF 的未移植表也没有收录 BadFloat。
   - 按现状，`py_float_repr` / `to_string` 的产物都是合法数字文本，实际不可达。
   - 真正的问题是这层映射本身：它把一种「输入非法」的错误归到了「未移植」。以后新增的 IrError 变体会默认被计为未移植，golden 统计会因此失真。
2. **U9：`method::error::unported` 无调用点**。它是死代码，和登记无关，删除即可。

### 2.2 登记失准

1. **instr GOLDEN_DIFF 陈旧登记**：未移植表中仍有 `sim/dynamic/lambda_args.rs` 一行（「lambda 实现类 … 不在注册表且 SAM 实参 … 需类型变量判定」）。
   - D1 已把这条分支改为移植后的实现。
   - b85f4fc0 删掉了它遗留的未用 `unported` 导入。
   - 当前代码中已无此分支，应从表中删除。
2. **input GOLDEN_DIFF 第 9 条（「F / D 常量折叠未移植……遇到时返回显式 InputError（未移植）」）与实际行为不符**：
   - `facts.rs:343-354` 的 `parse_fold_value` 对 JSON 数字一律执行 `as_i64`。所以 F/D 的非整数值（例如 `1.5`）先在 `facts.rs:348` 报 `Format("整数常量越界")`，根本到不了 `norm.rs:135` 的「未移植」分支。
   - 只有当 F/D 的值以 JSON 字符串编码（例如 `"NaN"`），或者是布尔值时，才会走到「未移植」。
   - J 类型下的布尔值同样落入「未移植：J 常量编码」。这其实是格式错误，登记只写了 F/D，没有提到 J。
   - `FoldValue`（`facts.rs:83`）没有 Float / Double 变体，这才是根因。
3. **method GOLDEN_DIFF §5「`_owner` 回退未移植」**：它没有对应的显式错误分支。如果 owner 为空，Rust 不会报 Unported，而是按空 owner 继续生成。建议改成显式报错，不再靠调用方保证。

## 3. 静默占位（非 Unported，全部未登记）

下列分支复制了 Python 的占位文本，或者发射错误的默认值。它们不返回 Unported，所以 golden 的 unported 计数看不到。这一策略也和 U1、U2「不复制不可编译占位」的做法不一致。

| # | 位置 | 发射内容 | 触发条件 | 对应 Python | 后果 |
|---|---|---|---|---|---|
| S1 | `instr/src/invoke/special/ctor.rs:243` | `/* {raw_cls}::new() */` 充当表达式 | `new` 目标类短名为空或含 `/`（表外类） | `codegen/instr/invoke.py:639` | 不可编译 |
| S2 | `instr/src/invoke/special/ctor.rs:281-283` | `/* invokespecial Method … */` | 已有对象上的构造器调用，但接收者不是 `this`/`self`，或 owner 为空 | `codegen/instr/invoke.py:689` | 静默丢弃 super/this 构造调用（错值） |
| S3 | `instr/src/sim/dynamic/lambda.rs:126-133` | `/* TODO: invokedynamic {idx} */` + `Object::default()` | lambda 引导缺少实现方法信息，或 SAM 描述符为空 | `codegen/instr/sim/dynamic.py:500` | 返回空对象（错值），使用时才会暴露 |
| S4 | `instr/src/sim/dynamic/lambda.rs:134-138` | `/* TODO: invokedynamic {idx} (impl parse failed) */` + `Object::default()` | 实现方法引用解析失败 | `codegen/instr/sim/dynamic.py:493` | 同上 |
| S5 | `instr/src/sim/dynamic/type_switch.rs:91-95` | `/* TODO S-17 … */` + `Object::default()` | `typeSwitch` 标签中含 `c`/`s`/`i` 之外的形态（EnumDesc 等 condy） | `codegen/instr/sim/dynamic.py:144` | 选择子应为 int，却压入了 Object，大概率编译失败；注释所说「可见的占位失败」并不成立 |
| S6 | `instr/src/invoke/virtual_/bare.rs:52-57` | `/* A-5: 未翻译接收者残余 … */` 或 `Default::default()` | 虚调用接收者类不在注册表，或者是接口残余形态 | `codegen/instr/invoke_virtual.py:256` | 静默空操作 / 默认值（错值） |

## 4. 分支外附录：emit crate（未合入）

`/Users/yuwei/dev/workspace/java_rta_emitter_emit`（85270bec）中有 3 处 `EmitError::Unported`，均已在该 crate 的 GOLDEN_DIFF 登记：

- `project::write_project` 的 lib crate 模式；
- `imports::base_fn` 的 bridge 重定向 `__base` 命名；
- `imports::base_fn` 的接口接收者 `__base` 命名。

## 5. 建议

### 5.1 P5b 前必须补齐

P5b 是 method → emit 接通，并对 27 例逐字节对照。以下各项不补，P5b 的对照无法进行或会出现假失配：

1. **U3 invokedynamic 常量池下标**。
   - 驱动不实现 `indy_cp_index`，27 例中所有 lambda 站点都会报 Unported，整方法退化。
   - 终态做法：`classfile::Operand::InvokeDynamic` 直接携带常量池下标，闭包分析的模式匹配同步适配，钩子随之删除。不再保留钩子作为中间形态。
2. **静默占位的策略统一**（S1–S6 与 U1、U2）。
   - 逐字节对照的基准是 Python 输出。S 类照抄 Python，所以对得上；U1、U2 不复制，只要 27 例中有一处命中就会失配。
   - P5b 之前应先在 27 例上统计 U1、U2、S1–S6 的命中数：
     - 命中数为 0 的：保持现状，并补登记。
     - 命中数非 0 的：直接实现真实翻译，不再选择「复制占位」还是「报 Unported」。
       - U2：MethodHandle / MethodType 常量在运行期物化。
       - U1、S1、S6：表外类走 `emit_unknown_stub` 式的精确 `panic!("stub: …")`。
       - S5：EnumDesc 标签的 typeSwitch 翻译。
3. **登记修正**：
   - 删除 instr GOLDEN_DIFF 中陈旧的 `lambda_args.rs` 行。
   - 按 2.2 更正 input GOLDEN_DIFF 第 9 条。
   - 在 instr GOLDEN_DIFF 中补登 S1–S6。

### 5.2 可以等 e2e 暴露

以下条目是 Python 的崩溃路径，或者按现有输入不可达，golden 中命中数为 0：

- U4：Python 在同一路径抛 IndexError。
- U5、U6：Python 不做校验；Rust 报错时方法退化为存根，运行期可见。
- U7：闭包分析器目前不产出 F/D 折叠。
  - 终态做法：`FoldValue` 增加 Float / Double 变体，`parse_fold_value` 按 `ty` 解码，`norm.rs` 发射 `ldc2_w` / `ldc`；同时把 J 类型的布尔 / 字符串值改归格式错误。
- U8：改为只把真正的未移植语义映射到 Unported，BadFloat 归为输入错误（`SimError::Env` 或新增 `BadLit`）。
- U9：直接删除。

### 5.3 终态目标

- `InstrError::Unported`、`MethodError::Unported`、`SimError::Unported`、`InputError` 中的「未移植」分支：**0**。
- S1–S6 这类不可编译或错值占位：**0**。
- 表外目标一律发射精确的 `panic!("stub: 类.方法:描述符")` 存根。
