# cfg golden 差异登记

golden：`python3 scripts/golden/dump_cfg.py <Test.java> --jdk 21 --clean` → `build/golden/cfg/<Test>.jsonl`，
`cargo test -p cfg --test golden -- --nocapture` 逐条重放比对。

## 现状（2026-09-30）

| 记录 | TestHashMapOps | TestStreamBasic |
|---|---|---|
| blocks | 606/606 全等 | 1163/1163 全等 |
| analyze | 261/261 全等 | 379/379 全等 |
| structure（结构树 + S-67 ctx 改写 + simplify 后结构树） | 258/258 全等 | 423/423 全等 |
| cmp_op | 566/780 全等，214 条 §1 | 756/1108 全等，352 条 §1 |
| neg_cmp_op | 290/407 全等，117 条 §1 | 391/584 全等，193 条 §1 |

未登记差异：0。结构化覆盖 try（30 / 45 条记录）、loop（98 / 154）、loop-exit 标签（58 / 80）、switch（6 / 7）；
两例均无不可归约 CFG，状态机形态只有单元测试覆盖。

## §1 单操作数比较的空格（布局差异，语义相同）

Python `cmp_op('ifeq', a, _)` 直接拼文本 `(a==0)`；Rust 产出结构化 `Paren(Binary(Eq, a, 0))`，
经 ir 渲染器为 `(a == 0)`。测试把 Python 文本 `op0)` 规整为 ` op 0)` 后比对，归入「已登记差异」。

## §2 `fold_const` 伪指令

Python 闭包常量折叠（`codegen/closure_folds.py`）把 invoke / getfield 改写成 `fold_const` 伪指令再切块。
Rust `build_blocks` 接收 `classfile::Insn`（结构化操作数），无此伪指令；golden 重放时把它当作非控制
指令（nop）——对切块而言两者等价（blocks 记录全等即验证）。折叠在 Rust 侧由 P4b 以其他方式承载。

## §3 结构性偏离（不影响 golden 比对）

- `build_blocks` 读取结构化操作数（`Operand::Branch` / `TableSwitch` / `LookupSwitch`），不解析 switch 文本；
  Python `parse_switch_operand` 不移植。
- 结构树 `Decl` 记录节点编号（声明语句由 P4b 节点持有），`Try` 的 catch 臂记录 catch 子句序号；
  golden 重放时按编号 / 序号还原 Python 的声明行与子句描述后比对。
- 循环出口标签 `('loop-exit', header)` → `BreakLabel::LoopExit(header)`。
- 循环按体大小降序排列，同尺寸时 Python 依回边 `set` 的迭代序（整数元组哈希序），Rust 依循环头升序
  （稳定排序）。仅在两个同尺寸循环都「包含 idom(Y) 而不包含 Y」且 ctx 相同时可能不同，golden 未出现。
- `structure` 错误只比对「是否出错」，消息措辞不比对。
- 调试输出 `[cfg-dbg]`（`options.DEBUG`）不移植。
- `Cond::atom` 接收 `ir::Expr`：自动取反识别 `Unary(Not, x)` 结构；Python 对文本 `!x` / `!(x)` / `true` /
  `false` 的识别只保留字面量 `true` / `false`（`Lit::Bool`），Raw 文本不解析（ir 约定：Raw 只做原子性判定）。
- 状态机：条件终结产出 `__pc = if c { t } else { f };`（ir 渲染器把 if 表达式分多行输出）；switch 终结产出
  `match key { vals => { __pc = t; } _ => { __pc = d; } }` 语句（ir 无 match 表达式），Python 为
  `__pc = match key { vals => t, _ => d, };`，语义相同。
- 统计 `AuditStats` 不是全局单例（`STATS`），由调用方持有；`JumpKind` 枚举代替种类字符串；
  `_verify_tree`（原在 `method/codegen.py`）移入本 crate 为 `verify_tree`，跳转 pc 经 `StructNode::jump_pcs` 读取。
