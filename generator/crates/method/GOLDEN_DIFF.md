# method crate golden 对照差异登记

基线：`scripts/golden/dump_method.py <Test.java> --jdk 21` 截获 Python `gen_method_body`
的输入 / 输出（`build/golden/method/<Test>.jsonl`），`tests/golden.rs` 逐条重放。

## 对照结果（逐字节）

| 测试 | 记录数 | 文本失配 | 登记失配 |
|---|---|---|---|
| TestHashMapOps | 659 | 0 | 0 |
| TestStreamBasic | 1310 | 0 | 0 |
| TestCompletableFuture | 7232 | 0 | 0 |
| 合计 | 9201 | 0 | 0 |

- 「文本」指函数全文，包括错误消息（`ERR <消息>` 同样逐字节对照）。
- 「登记」对照的是本次生成的外部登记：审计按计数展开，另有继承调用请求、lambda 引用和 SAM 站点。
  Python 侧的 `sam_ctor` 属于发射层登记，不在本 crate 职责内，对照前已剔除。

## 移植形态差异（输出等价）

1. **`let _ = e;` 的表示**
   - Python 用 RawStmt 表示；Rust 用 `LetStmt`，其名字为 `Ident::discard()`。
   - 为保持等价，下列位置把 discard let 当作 raw 语句处理：
     - 变量分析层 `vars/mod.rs`（`let_of`、`store_value`、`var_identity`、`promote_undeclared_assigns`）；
     - `vars/refs.rs`（按语句文本扫描标识符，且不计为声明）；
     - 状态机 let 提升 `blocks/dispatch.rs`（原样保留）。
2. **构造器 `locals[0]` 覆写省略**
   - Python 在构造器里把 `locals[0]` 显式覆写为本类类型。
   - Rust 模拟器的默认值 `RsType::class(class_name, 形参实参)` 与该覆写结果相同，因此省略这一步。
3. **全局 `STATS` 改为调用方持有的 `MethodSink`**
   - 控制流审计数据（`CfgStats`）和 `InstrLog` 都写入调用方传入的 `MethodSink`。
   - 写入时序与 Python 相同：栈下溢报错之前已经记账，所以方法退化为存根时登记依然保留。
4. **`Node::sync_term` 在 body 层统一调用**
   - 时机：`split_disjoint_try_ranges` 之后、求后继之前。
   - 作用：与 Python 节点终结指令的即时同步等价。
5. **Python `_owner` 回退未移植**
   - 回退的对象是 owner 为空的情况。golden 中所有记录的 owner 都不为空（owner ≠ 发射类的记录分别为 0 / 25 / 235 条），调用方必须提供 owner。

## 未移植分支（显式报错 `MethodError::Unported`）

| 位置 | 触发条件 | 说明 |
|---|---|---|
| `types::ir_type_of` | 类型文本无法解析为 `ir::Type` | Python 直接拼接文本，Rust 的 IR 需要结构化类型；golden 中未出现 |
| `vars/if_emit.rs` | if 提升出的变量名不是合法的 `ir::Ident` | Python 不做校验；golden 中未出现 |

## golden 未覆盖的路径

- 状态机分派路径（`blocks/dispatch.rs`，对应不可归约 CFG）：三个测试中没有方法走到这条路径。
- `fold_array` 的本类静态字段 getter 值（FoldField）：golden 中无此样本，只有单元测试覆盖。

## Python 缺陷

本轮对照没有发现需要登记的 Python 缺陷。
