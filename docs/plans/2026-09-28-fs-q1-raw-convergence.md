# FS-Q1：Raw 逃生舱收敛（IR 结构化）

> 关联：过渡态清单 FS-Q1、N4 TypeIR（G1 `from_rs_type` 已就绪）、N8（rustc 峰值内存）、R0（`ir` crate 规格）。

## 一、现状（2026-09-28，`main.py --raw-sites` 位点剖面，TestArrayList）

`raw_stmt=76629`、`raw_expr=40830`。高度集中：

| 位点 | 事件 | 形态 |
|---|---|---|
| `invoke_virtual._emit_call_result:664` | 16572 stmt | `let v = recv.m(args)?;` |
| `fields.sim_fields:332` | 14030 expr | getfield 取值表达式 |
| `arrays.sim_arrays:189` | 10111 stmt | 数组元素存储 `arr.set(i, v)?;` |
| `invoke._gen_invokespecial:640` | 5937 expr | 构造 / super 调用 |
| `invoke._gen_invokestatic:901` | 5815 stmt | `let v = C::m(args)?;` |
| `stack._clone_moved_var:133` | 5507 expr | `Clone::clone(&x)` |
| `returns.sim_returns:121/33` | 9925 stmt | `return Ok(..);` |
| 前 10 位点合计 | 约 70% | |

## 二、根因

调用族在模拟阶段就把实参渲染成字符串（`arg_str`）再拼调用串（`_build_call`），IR 只剩最外层
一个 Raw。单把外层语句换成 `LetStmt` 只是 raw_stmt → raw_expr 平移；实参节点必须从栈模拟一路
以 `RsExpr` 携带到发射点，Raw 才真正消失。

## 三、分批（2026-09-28 修订：自底向上）

实地勘查：`_pop_receiver_and_args` 从栈上取到的是节点，但强制转换层（`coerce._coerce_to_object`、
`invoke_sig._coerce_arg`、`_render_cast` 等约 22 个 f-string 返回点）立刻把它们变成字符串；只把外层语句
换成 `LetStmt` 只会让 raw_stmt 平移成 raw_expr。因此改为自底向上：

| 批 | 内容 | 验收 |
|---|---|---|
| Q1-a ✅ | IR 补 `TryExpr(inner)`（渲染 `inner?`）；单测 | 单测 |
| Q1-b ✅ | 强制转换层节点化：每个转换函数补「节点入 → 节点出」版本（`Call('Object::from', [..])` / `UpcastExpr` / `CastExpr` 等），渲染与旧字符串**逐字符一致**；旧字符串版本改为「节点版 + render_expr」薄封装 | 生成树逐字节一致 |
| Q1-c ✅ | 调用族实参以节点携带：`_pop_receiver_and_args` → 强制转换节点版 → `MethodCall` / `Call` 节点；caller-sensitive 包装节点化 | 生成树逐字节一致（语句层仍渲染为同一文本） |
| Q1-d ✅ | 语句层类型化：`LetStmt(v, value=TryExpr(..))` / `ExprStmt(TryExpr(..))` 替代 RawStmt（进入变量提升 / 可变性分析） | 编译 + 定向 e2e（≤10 例，`CARGO_BUILD_JOBS=1`）；`[raw-audit]` 与位点剖面按位点下降 |
| Q1-e 🔄 | getfield / putfield、数组存取、returns、`_clone_moved_var` 同构推进 | 同 Q1-c / Q1-d |

每批：`main.py --raw-sites` 位点剖面前后对照，趋势只降不升。

## 四、风险

- 类型化 `LetStmt` 进入变量提升 / 可变性分析（Raw 语句对这些 pass 不透明）：会改变 `let` / `let mut`
  与提升决策——这正是结构化的目的，但需逐批以编译 + 运行验收，而非逐字节对照。
- `_build_call` 的 caller-sensitive 包装（`reflect_dispatch::__caller_sensitive`）需节点化，保持调用处
  类名传入语义。

## 五、进度（2026-09-28）

| 提交 | 内容 | 验收 |
|---|---|---|
| `cd7621e` | Q1-c/d：invokevirtual 实参节点携带 + 调用结果 LetStmt / ExprStmt | 编译 + e2e |
| `a4bb653` | getfield 读取、aastore 三条发射路径 | 4 例生成树逐字节一致 |
| `c80ba28` | invokestatic、returns（ReturnStmt） | 4 例逐字节一致 |
| `6c3c5d5` | `_clone_moved_var`（Call + RefExpr） | 4 例逐字节一致 |
| `11e5a4d` | invokespecial 构造调用 | 唯一差异为变量提升假阳性消除（字符串字面量曾被 Raw 文本扫描误计为变量引用） |
| `7881b66` | putfield / putstatic | TestFieldEvalOrder 逐字节一致 |
| `673ab28` | invokevirtual 接收者节点携带 | 4 例逐字节一致 |
| `d1f32e6` | 基本类型数组存储、newarray / anewarray | 4 例逐字节一致 |
| `6f8396d` | iinc（AssignStmt）、athrow（ReturnStmt Err） | 4 例逐字节一致 |
| `ffafad2` | 算术 wrapping 族；let 省略标注改为白名单判定 | 4 例逐字节一致 |
| `43ec8e7` | `_coerce_value` 节点版（构造后与字符串实现比对、不一致回落 Raw）；实参文本叶子不再以 Var 承载复合文本 | 4 例逐字节一致 |

编译验证（逐批串行）：TestArrayList / TestCustomException / TestEnumAdvanced / TestEnumBasic /
TestStreamBasic / TestHashMapOps / TestFieldEvalOrder / TestStringBuilder / TestStringBuilderOps 全 PASS。

`[raw-audit]`（TestFieldEvalOrder，每次转译）：起点 raw_expr≈45.1K / raw_stmt≈56.6K →
raw_expr≈15.8K / raw_stmt≈7.8K（合计 −77%）；run_tests 口径单测试 raw 102.6K → 23.6K。

保持 Raw 的形态（逐字节一致约束下暂不动）：经强制转换的值叶子（`_coerce_value` / `_coerce_stored_value`
字符串实现）、Object 装箱的 let、@CallerSensitive 包装调用、未知类存根。下一批：`_coerce_stored_value` /
`_coerce_to_object` 值叶子（字段写入、areturn）、基本块合并（blocks._merge_entry）、三元融合（fusion._ternary_value）、
移位 / 位运算（原文无空格，BinOp 渲染带空格，需按「构造后比对」模式处理）、invokedynamic。
