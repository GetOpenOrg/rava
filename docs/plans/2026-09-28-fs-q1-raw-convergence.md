# FS-Q1：Raw 逃生舱收敛（IR 结构化）

> 关联：过渡态清单 FS-Q1、N4 TypeIR（G1 `from_rs_type` 已就绪）、N8（rustc 峰值内存）、R0（`ir` crate 规格）。

## 一、现状（2026-09-28，`JAVA_RTA_RAW_SITES` 位点剖面，TestArrayList）

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

## 三、分批

| 批 | 内容 | 验收 |
|---|---|---|
| Q1-a | IR 补 `TryExpr(inner)`（渲染 `inner?`）与 `CallerSensitive` 包装节点；`render` 单测 | 单测 |
| Q1-b | invokevirtual / invokeinterface：实参以 `list[RsExpr]` 携带，`_build_call` 产出 `MethodCall` 节点；`_emit_call_result` 发射 `LetStmt(v, value=TryExpr(..))` / `ExprStmt(TryExpr(..))` | 生成树差异仅限 `let` 可变性标注等预期形态；定向 e2e 10 例 PASS |
| Q1-c | invokestatic / invokespecial 同构 | 同上 |
| Q1-d | getfield / putfield、数组存取：`MethodCall(recv, "__get_x")` / `MethodCall(arr, "set", [i, v])` | 同上 |
| Q1-e | returns / `_clone_moved_var`：`ReturnStmt(Ok(..))`、`Call("Clone::clone", [RefExpr])` | 同上 |

每批：位点剖面前后对照（raw 事件数按位点下降）、定向 e2e（`CARGO_BUILD_JOBS=1`，≤10 例）、
`[raw-audit]` 趋势只降不升。

## 四、风险

- 类型化 `LetStmt` 进入变量提升 / 可变性分析（Raw 语句对这些 pass 不透明）：会改变 `let` / `let mut`
  与提升决策——这正是结构化的目的，但需逐批以编译 + 运行验收，而非逐字节对照。
- `_build_call` 的 caller-sensitive 包装（`reflect_dispatch::__caller_sensitive`）需节点化，保持调用处
  类名传入语义。
