# 窗口 3 本体：G-1 变量提升迁到结构化 rs_ir

> 2026-09-25 · 清单第 7 项（重写门槛②）· 前置：窗口 3 结构步骤 ⑧⑨⑩（`5a14b30` / `9d4b007` / ⑩）

## 一、现状

方法体从 CFG 结构树（`cfg.structure` 的 Code / Loop / If / Switch / Try / Block + `Dispatch`）
经 `method/emit.py::TreeEmitter` **拍平**为 `entries: list[(indent, RsStmt | str)]`：

- 块结构（`if … {`、`} else {`、`loop {`、`'bN: {`、`match … {`、`X => {`、`java_try! {`、
  `try {`、`} catch (…) {`、`}`、状态机 `let mut pc` / `_ => unreachable!()`）落为**文本行**；
- 块内语句落为 `(indent, RsStmt)`。

三个提升 pass（`vars._hoist_loop_vars` / `_hoist_if_vars` / `_promote_undeclared_assigns`）
在这张扁平表上**从文本反推结构**：

| 依赖文本的操作 | 位置 |
|---|---|
| 嵌套深度 = 逐行 `_brace_delta`（剔除字面量后数 `{` `}`） | loop / if / promote 三 pass |
| 块起点 = 文本行以 `{` 结尾且不以 `}` 开头 | `_hoist_if_scan` |
| match 臂 / try 块识别 = `'=>' in text`、`text == 'try {'` | `_hoist_if_locate` |
| else 分支识别 = `} else {` 文本 | `_hoist_if_locate` |
| 变量引用 = 对**渲染后整行文本**做 `\bname\b` 正则 | `_hoist_if_select` / `_hoist_if_locate` |
| 已声明检查 = `let (mut )?name` 正则 | `_hoist_if_locate` |

问题：结构信息在 emit 时已完整存在，被拍平后再用文本启发式重建——字面量内的花括号、
标签块、match 臂与 try 宏语法都要各自打补丁（历史上「遮蔽已初始化变量」语义错误即此类）。

## 二、终态

- `emit_tree` 产出**结构化语句树**：块结构为类型化节点，块内为语句列表；
- 提升 pass 在树上工作：嵌套 = 树深度，插入点 = 父节点语句列表中的位置，else / match 臂 /
  try 由节点类型区分；
- 渲染在提升之后一次完成（`render_stmt` 递归）；
- 结构类文本匹配 = 0（`_brace_delta` 删除）。变量**引用**检测改为 IR 遍历收集 `Var` 名；
  `RawExpr` / `RawStmt` 内的引用仍需文本扫描，随 L5（Raw 消除）清零，单列审计计数。

## 三、步骤（每步独立提交，生成树逐字节不变为硬约束）

| 步 | 内容 | 验收 |
|---|---|---|
| **G1-a** ✅ `cf457e3` | rs_ir 新增块节点 `BlockStmt(kind, segs)`；`TreeEmitter` 产出树，`flatten(tree)` 还原出与现状逐项相同的 entries；提升 pass 仍消费 flatten 结果 | 23 例生成树逐字节一致 |
| **G1-b** ✅ `49692a0` | 块结构行 `StructLine(str)` 带 emitter 给出的 delta / tag；三个提升 pass 的嵌套深度与四类结构判定（loop 头 / 块起点 / else 行 / match 臂·try）改读标注，删除 `_brace_delta` 与文本特征判定 | 双算 27 例 330 万次 0 分歧（首轮「块起点」540 处分歧 = 旧判定缩进使 `} else {` / `} catch … {` 衔接行也计为块起点，照此复现）；23 例生成树逐字节一致 |
| **G1-c** ✅ | 引用检测改 IR 收集（`_refs`：Var / let 名 / 赋值目标；字段名 / 方法名 / 类型 / 字面量不计；Raw / 宏参数仍文本扫描） | 原判「不可能逐字节不变」经量化推翻：两处主决策点双算 27 例 6.4 万次 0 差异（假阳性从未落在决定位置）；4 处全部切换后 23 例生成树逐字节一致 |
| G-2（行为变更，单独批） | 前置声明去 `Default::default()` 占位 → `let x: T;`（Rust 确定赋值分析） | **会改变生成树**：需用户全量对账（面板 #16） |

## 四、风险与约束

- **逐字节不变**是 G1-a/b/c 的唯一验收口径：提升决策（插入位置、降级、G-3 分型拆分）
  必须与文本版逐点一致；分歧先插桩双算定位，不以「新行为更合理」为由接受 diff。
- `_hoist_if_vars` 的 64 轮收敛循环（每轮只提升一层）在树上可一次完成，但 G1-b 先保持
  逐轮语义不变，收敛优化留到 G1-c 之后另议。
- 验收集：窗口 3 ⑧⑨⑩ 的 23 例（控制流 / 提升回归 / TestAtomics 大闭包）；G1-b 每个 pass
  另加 TestHoistShadow / TestChmTransfer 的双种子。
