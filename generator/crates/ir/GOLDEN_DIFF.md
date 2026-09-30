# ir crate golden 差异清单

对照方式：`scripts/golden/dump_ir.py` 包装 `codegen.render` 的各渲染入口（render_type、render_expr、
render_stmt、render_cast、upcast_expr、is_atomic_rs 以及条目渲染函数），只记录最外层调用，
按（入口, IR 输入, 参数, 输出）去重后写入 `build/golden/ir/<Test>.jsonl`；
`tests/golden.rs` 把 Python IR 转成 Rust IR，渲染后逐字节比对。

## 结果（2026-09-30）

| 测试 | 唯一条目 | 全等 | render_stmt | render_expr | render_type | upcast_expr | render_cast |
|---|---|---|---|---|---|---|---|
| TestHashMapOps | 9140 | 9140（100%） | 6000 | 2941 | 171 | 26 | 2 |
| TestStreamBasic | 12222 | 12222（100%） | 7705 | 4119 | 352 | 32 | 14 |

两个测试的输出均无差异。另外逐个检查了所有子表达式，结构化判定 `Renderer::is_atomic` 与对渲染文本做
`is_atomic_rs` 扫描的结果全部一致。

## 文本载体结构化情况

Python IR 用字符串承载结构：`RsNamed.name`、`Call.func`、`Lit.value`、非标识符的 `Var.name`、
`CastExpr.target`、`StaticFieldRef.turbofish`。转换层 `tests/golden_support/parse.rs` 只在测试中存在，
负责把这些文本解析成结构化 IR，再用往返渲染校验，校验不通过就退回 `Raw`。

| 载体 | HashMapOps 结构化 / 退回 Raw | StreamBasic 结构化 / 退回 Raw |
|---|---|---|
| 类型文本（RsNamed / CastExpr.target / turbofish） | 全部 / 0（严格解析，失败即报错） | 全部 / 0 |
| Call.func | 4227 / 0 | 6398 / 0 |
| Lit：字面量形态 | 3225 / 0 | 3912 / 0 |
| Lit：表达式文本（`String::from_owned(format!(..))`） | 75 / 0 | 0 / 0 |
| Var：表达式文本 | 31 / 0 | 124 / 4 |
| upcast_expr 的字符串源 | 26 / 0 | 31 / 1 |

退回 Raw 的 5 条（输出仍与 Python 逐字相同，只是在 IR 中没有被结构化）：

| # | 输入（文本载体） | Python 输出 | Rust 输出 | 原因 |
|---|---|---|---|---|
| 1 | `Call(func='Object::from', args=[Var('\0')])` | `Object::from(\0)` | 相同（`Var` 退回 `Raw("\0")`） | 上游异常：Python 构造了名为 NUL 字符的 `Var`。经核查，这次渲染的结果没有进入生成文件，属于中间探测调用。`Ident` 拒绝这个名字，终态下产出方必须给出合法变量 |
| 2 | `Var('ReferencePipeline_3::<..>::new(.., (StreamOpFlag::NOT_SORTED()?\|StreamOpFlag::NOT_DISTINCT()?), ..)?')` | 原文 | 相同（Raw） | 文本里的 `a\|b` 运算符两侧没有空格，而 `Expr::Binary` 的渲染固定为 `a \| b`，按结构无法复现这种排版 |
| 3 | `Var('this.__get_this_0()...get(((this.__get_lastReturnedIndex()<<(6i32&0x1f))).wrapping_add(_t1))?')` | 原文 | 相同（Raw） | 同上：`<<`、`&` 两侧没有空格；另外 `0x1f` 是十六进制记号，`Lit::Int` 按十进制渲染 |
| 4 | upcast_expr 的源 `DistinctOps_1::<Object>::new(.., (StreamOpFlag::IS_DISTINCT()?\|StreamOpFlag::NOT_SIZED()?))?` | `(<源>).into()` | 相同（`Upcast{Raw(源), Bare}`） | 同 #2 |
| 5 | `Var('(if (((size>(0i64)) as i32-((size)<(0i64)) as i32)>=0) { .. } else { .. })')` | 原文 | 相同（Raw） | Python 字符串栈里内联的 `if` 表达式，运算符两侧没有空格，并且有冗余括号 |

#2 到 #5 都来自 Python 字符串栈（`instr/` 模拟器）直接拼接的表达式文本。这些产出方迁移到 Rust 后会直接构造
`Expr::Binary`、`Expr::If`、`Expr::Cast`，渲染出来的运算符两侧带空格。因此它们在生成树上的文本变化是预期内的，
属于排版差异，不影响语义；P4 阶段做生成树对照时应把它们登记为允许的差异。

## 设计差异（golden 未覆盖，Python 实际不会走到的路径）

| 项 | Python | Rust | 原因 |
|---|---|---|---|
| `IfExpr` 条件为 false 的判定 | 渲染出的条件文本等于 `'false'` | 结构判定：条件是 `Lit::Bool(false)` | 不再依据文本判定。`Raw("false")` 这种边角情形会渲染成完整的 `if false {..}`。Python 从未构造过 `IfExpr` |
| InstanceOf、try_cast、ClassRef 中的 binary 名 | 直接插入引号，不转义 | 经 `escape_str` 转义 | 真实的 JVM binary 名不含需要转义的字符，因此两者输出相同 |
| `IfStmt`、`LoopStmt` 的格式 | `render_stmt` 旧格式：空体时会产生空行 | 采用 `method/emit.py` 结构行的格式：空体不产生空行，并支持标签、else-if 链 | Python 从未构造过 `IfStmt` 或 `LoopStmt`，实际的结构块走 `BlockStmt`、`StructLine` |
| `BreakStmt`、`ContinueStmt` | 不带标签 | `Option<Label>` | 结构块里的 break 和 continue 带标签 |
| `MacroExpr.args` | 字符串列表 | `Vec<Expr>`，格式串用 `Lit::Str` 表示 | 强类型 |
| `RsGeneric(outer, params)` | 单独的节点 | 合并进 `Type::Path` 路径段上的泛型 | 同一概念只保留一种表示 |
| `RsNamed(name, path)` | 用 `path::name` 文本拼接 | `Path{global, segments}` | 同上 |
| `UnOp.op`、`BinOp.op` | 字符串 | 枚举 | 强类型 |
| `Cast.ty` 为 `RsNamed('u16')` | 当作具名类型 | 解析为 `Type::Prim(U16)` | 原始类型只保留一种表示 |
| 未知节点 | 渲染成 `/* unknown .. */` | 由枚举穷尽匹配，不存在未知节点 | 强类型 |
