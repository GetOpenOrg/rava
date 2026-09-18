# 控制流结构化终态重写

> 创建日期：2026-09-18
> 取代：`2026-09-15-cfg-ifelse-unresolved.md`（模式匹配方案的遗留问题清单，本方案落地后全部失效）

## 1. 问题

`codegen/cfg/`（loops / guards / branches / conditions / switches）+ `codegen/method/codegen.py`
按「指令下标区间模式」识别 while / guard / if-else / switch。任何匹配不上的形态，
条件跳转落到 `sim_control` 的「弹栈后忽略」分支，goto 落到「其他 goto：忽略」分支——
**跳转被静默丢弃，生成语义错误的 Rust**。已确认实例（`java.util.HashMap.putVal`）：

| 字节码 | Java 语义 | 旧生成结果 |
|---|---|---|
| 158-162 `if_icmplt 222` | `if (binCount >= TREEIFY_THRESHOLD - 1)` | 守卫丢失，无条件 treeifyBin |
| 172 `goto 222` | 循环内嵌套 if 的 `break` | 丢失 → 死循环 |
| 81 / 193 `if_acmpeq` | `k == key \|\|` 短路 | 丢失 |
| 106-128 | `p instanceof TreeNode` 分支 | 整段丢失 |
| 234-241 | `!onlyIfAbsent \|\| oldValue == null` | 只剩后半 |
| `++size > threshold` | dup_x1 后 putfield | `size+1` 被求值两次 |
| `dup` 构造表达式 | `(x = new X())` | `X::new()?` 被构造两次 |

根因是架构性的：结构化建立在「指令区间 + 模式」上，而不是建立在控制流图上。

## 2. 终态架构

```
指令序列
  │  cfg/graph.py        基本块 + 边（cond/goto/switch/fall/exit）+ 异常处理器入口
  ▼
CFG（入口可达子图）
  │  method/blocks.py    按 RPO 逐块 StackSim 模拟：语句、出口栈、终结条件 Cond
  │                      + CFG 级归约：短路 &&/|| 归并、三元/布尔物化归并、常量条件折叠
  ▼
归约后的 CFG（每块：stmts + Terminator）
  │  cfg/structure.py    RPO → 支配树 → 回边/自然循环 → 可归约性判定
  │                      → 支配树驱动的结构化（带标签 block / loop / if / match）
  │  cfg/dispatch.py     不可归约：loop { match __pc { .. } } 状态机兜底
  ▼
结构树（Seq / Block / Loop / If / Switch / Break / Continue / BlockRef / Dispatch）
  │  cfg/structure.py    可读性规整：尾跳转消除、无用标签删除、Block∘Loop 合并、
  │                      guard 展平、空 then 取反、else-if 链、while 形态
  │  cfg/audit.py        自检：每条跳转指令必须被消费、每个活块恰好输出一次
  ▼
method/emit.py           结构树 → entries（沿用 (indent, item) 扁平格式，供既有
                         _analyze_mutation / _hoist_* / _promote_* 后处理）
```

旧模块 `basic_blocks.py / branches.py / guards.py / loops.py / switches.py` 与
`types.LoopInfo` 全部删除；`conditions.py` 改为条件代数（`Cond`）+ 原有比较符翻译。
`sim_control` 中「分支指令弹栈后忽略」的分支删除——分支指令不再进入 `sim_instr`。

## 3. 算法

### 3.1 CFG（cfg/graph.py）

- leader：入口、所有跳转/switch 目标、跳转与退出指令的下一条、异常处理器入口（`exception_table.handler_pc`）。
- 每块一个 `Terminator`：`cond(target, fallthrough)` / `goto` / `fall` / `switch(cases, default)` / `exit`。
- 只有入口可达的块参与模拟与结构化；仅由异常处理器入口可达的块标记为 handler 区域
  （try 区域现状能力保持：正常路径翻译、处理器不翻译；处理器区域的跳转在自检中单列统计，
  不计为「丢弃」，由异常对象/athrow 的改造接手）。
- `jsr` / `ret`：生成期报错（不静默）。
- RPO：DFS 时先访问跳转目标、最后访问 fall-through，使 RPO 与字节码/源码顺序一致。
- 支配树：Cooper-Harvey-Kennedy 迭代算法。
- 回边 `u→v`：`v dom u`。自然循环：同一 header 的所有回边的循环体并集。
- 可归约性：每条 retreating 边（`rpo[v] <= rpo[u]`）都必须是回边，否则不可归约。

### 3.2 逐块模拟与 CFG 级归约（method/blocks.py）

单个 `StackSim` 实例贯穿整个方法（临时变量计数器全局唯一），按 RPO 处理活块：

1. **入口状态**
   - 单一前向前驱：继承其出口栈与 locals。
   - 多个前向前驱（merge 块）：栈深必须一致；逐位置比较，渲染相同则沿用，
     不同则分配合并变量 `_mergedN`：目标类型由各前驱类型统一（`unify_types`），
     各前驱块尾追加 `_mergedN = coerce(value)`，声明 `let mut _mergedN: T;` 由结构化阶段
     放在 merge 块的「放置父节点」子树之前（保证作用域覆盖全部赋值与使用）。
   - 循环头：入口状态只取前向前驱；回边前驱的出口栈必须与之等价，否则报错。
2. **块内模拟**：除终结跳转指令外逐条 `sim_instr`；终结指令由本层处理：
   - 条件跳转 → `Cond`（原子携带正/反两种文本，取反不产生 `!(..)` 包裹）
   - switch → 弹出 key
3. **常量条件折叠**：`Cond` 为常量（静态 `instanceof` 结果等）→ 终结改为 goto，死边删除，
   由此不可达的块标记 dead（其跳转记为「死代码」消费）。
4. **短路归并**（块 B 完成后触发，可级联）：B 仅有一个活前驱 P，P、B 均为 cond 终结，
   B 是纯条件块（无语句；出口栈与入口栈逐项同一对象；非循环头），且与 P 共享一个后继：

   | B 的位置 | 共享关系 | 归并结果 |
   |---|---|---|
   | B = P.false | B.true = P.true | `(c1 \|\| c2, P.true, B.false)` |
   | B = P.false | B.false = P.true | `(c1 \|\| !c2, P.true, B.true)` |
   | B = P.true | B.false = P.false | `(c1 && c2, B.true, P.false)` |
   | B = P.true | B.true = P.false | `(c1 && !c2, B.false, P.false)` |

   「无语句」判定前先做**临时变量回填**：块尾连续的 `let _tN = call?;`（调用结果物化）
   若在条件中恰好出现一次，则回填为表达式（`key.equals(k)?` 直接出现在 `||` 右侧，
   短路求值顺序与 Java 一致）；自后向前回填，遇到第一个无法回填的即停止（保持求值顺序）。
   归并不了 → 保持为嵌套 if（结构化阶段自然产出），跳转不丢。
5. **三元归并**（处理 merge 块 M 之前触发，循环到不动点）：M 的两个前驱 A、B 均为纯值块
   （无语句、无条件跳到 M、栈比其唯一前驱 P 多一项），P 为 cond 且后继恰为 {A, B}
   → P 的出口栈追加 `(if c { a } else { b })`，P 终结改为 goto M，A、B 被吸收。
   两臂为 `1/0` 字面量 → 布尔物化，值为 `CondExpr`（携带 `Cond`，后续 `ifeq/ifne`
   直接结构化取反，得到 `a && b` 而非 `!(!a || !b)`）。

### 3.3 结构化（cfg/structure.py）

支配树驱动（Ramsey, *Beyond Relooper*, ICFP 2022 的 Rust 适配）：

- **放置父节点** `P(Y)`：设 `D = idom(Y)`。若存在包含 D 而不包含 Y 的循环，取其中最外层循环的
  header X：`P(Y) = X`，Y 为 X 的 **out follower**（循环出口后继，放在 `loop` 之后）。
  否则 `P(Y) = D`：Y 的前向入边数 ≥ 2 时为 **in follower**（merge 节点），否则在分支点内联。
- `do_tree(X)`：
  ```
  [merge 变量声明]
  'Y1: { 'Y2: {                      // out followers，RPO 大者在外
      'X: loop {                     // 仅当 X 是循环头
          X.stmts
          'Z1: { 'Z2: { terminator(X) } do_tree(Z2) } do_tree(Z1)   // in followers
      }
  } do_tree(Y2) } do_tree(Y1)
  ```
- `do_branch(src, tgt)`：回边 → `continue 'tgt`；follower → `break 'tgt`；否则内联 `do_tree(tgt)`。
- 任意出口都由带标签的 `break` / `continue` 表达，无需任何模式。
- 正确性不变量：同层 follower 按 RPO 降序嵌套，任何前向边 `src→Y` 的 src 在文本上必位于
  `'Y` 块之内；循环体节点全部位于 `'X: loop` 之内。

**可读性规整**（结构树上的保语义重写，顺序执行）：

1. 尾跳转消除：`'Y` 块体尾部的 `break 'Y`、循环体尾部的 `continue 'X` 删除（if / match 臂向内传播尾上下文）。
2. 无引用标签的 Block 拆除；`'Y: { 'X: loop {..} }` 合并为循环自身的 `break`。
3. guard 展平：`if c { A } else { B }` 且某臂不落出（以 break/continue/return/throw 结束）
   → `if c' { 该臂 } 其余平铺`（得到 `loop { if cond { break; } ... }`）。
4. 空 then 取反、`else { if }` → `else if`。
5. `loop { if c { break; } rest }` → `while !c { rest }`。
6. 最内层循环的 `break` / `continue` 省略标签；仍被引用的标签按出现顺序命名 `'l0..` / `'b0..`。

### 3.4 不可归约兜底（cfg/dispatch.py）

javac 不产生不可归约 CFG，但生成器不得假设输入来源。判定不可归约时整方法走状态机：

```rust
let mut __pc: i32 = 0;
loop { match __pc {
    0 => { ...; __pc = if cond { 3 } else { 1 }; }
    1 => { ...; __pc = 2; }
    ...
    _ => unreachable!(),
} }
```

该模式下模拟层启用 `spill_all`：块尾栈值全部落到函数级变量、所有局部声明提升到函数顶部
（match 臂之间不共享词法作用域）。同名异型局部无法提升 → 生成期报错。

## 4. 与 StackSim 的接口

- `StackSim` 增加「同一表达式对象多处引用」的物化规则，修复 dup 语义：
  - `pop()`：弹出的表达式若仍有同一对象留在栈上且非平凡（非 Var/Lit/NewPending）
    → 先 `let _tN = expr;`，栈上所有同一对象替换为 `Var(_tN)`（`++size > threshold`、
    `dup; putfield` 等只求值一次）。
  - `pop_for_store()` + `store_local()`：`dup; xstore` 形态不引入临时变量——存入局部后，
    栈上同一对象替换为该局部（`(e = p.next) == null` → `e = p.next; if e.is_null()`）。
- 分支/goto/switch 指令不进入 `sim_instr`；`sim_control` 的忽略分支与 `sim_dynamic` 的
  `_switch_key` 分支删除。
- 作用域深度（`enter_scope/exit_scope`）不再由结构化层驱动：块级作用域问题统一由
  既有的 `_hoist_*` / `_promote_undeclared_assigns` 后处理解决。
- `classfile.py` 解析 `exception_table` 并挂到 `ParsedMethod.exception_table`。

## 5. 自检规则（cfg/audit.py）

每个方法生成完成后强制执行，失败抛 `CfgAuditError`（`class_writer` 不得吞掉，直接终止转译）：

1. 入口可达块中的每条 `if* / goto / goto_w / tableswitch / lookupswitch` 指令，必须有且只有
   一种消费记录：`structured`（其所在块的 terminator 被结构树输出）、`short-circuit`、
   `ternary`、`const-fold`、`dead`（常量折叠后不可达）、`dispatch`。
2. 每个活块在结构树中恰好出现一次。
3. 报错信息格式：`Class.method:descriptor 未消费跳转 pc=[..]`。

统计：`transpile` 结束时输出一行
`[cfg-audit] methods=N jumps=J consumed=J unconsumed=0 dispatch=K handler_jumps=H stub_fallback=S`。

量化目标：**unconsumed = 0**（硬性，非 0 即失败）；`dispatch` 对 javac 产物为 0。

## 6. 验证

```
python3 scripts/main.py tests/e2e/04_collections/TestCollections.java --no-run --clean
cd build/test_collections && CARGO_TARGET_DIR=../target cargo build --bin test_collections > /tmp/cfg_tc_build.log 2>&1
timeout 10 ../target/debug/test_collections
```
同样处理 TestArrayList / TestStringBuilder。验收：

- 三个测试编译 0 错误；不再卡死在 `HashMap.put`。
- 对照 `javap -c -p` 人工核对 `HashMap.putVal / resize / treeifyBin`。
- 自检统计 unconsumed=0。
- `tests/unit/test_cfg_structuring.py`：合成指令序列验证嵌套 break、短路、不可归约兜底与自检报错。

## 7. 实现定稿补充

- **跳转线程化**（blocks.py `_thread_jumps`）：只含一条 `goto` 的块不产生代码，指向它的边直接改指最终目标；
  `break` 经中转 goto 到达出口时，`&&` / `||` 的两个出口因此落在同一目标并可合并。被线程化的 goto 记为 structured。
- **直线块合并（fuse）**：单前驱的直线后继并入前驱，减少结构树节点与标签。
- **块代码置于标签块之外**（structure.py `do_tree`）：块自身的直线语句不含跳转，放在 in-follower 标签块之前，
  局部变量声明与 Java 同层可见；标签块只包住分支部分。
- **switch 臂按目标分组**：同目标 case 合并为 `v1 | v2 =>`，与 default 同目标的 case 由 `_` 覆盖，保证每个目标只展开一次。
- **尾位置规整**：尾位置循环内指向「等价于落出」标签的 break 改为循环自身的 `break`；
  紧跟同一跳转的分支臂尾跳转下沉消除。二者使 `for` 形态恢复为 `while cond { .. }`。
- **循环头临时变量回填**：`let _tN = call?;` 仅服务于循环条件时回填进条件，使 `while cond {` 成立。
- **汇合变量声明位置**：随 follower 的标签块就位；归约后不再是 follower 的节点，其声明置于函数顶部。
- **异常处理器**：只翻译正常路径；含异常表的方法在生成代码首行写明未翻译的处理器清单，
  自检统计单列 `handler_methods` / `handler_jumps`（不静默）。catch 分派依赖异常对象模型，随该模型落地后接入。
- **自检统计输出**：`scripts/main.py` 在转译后打印 `[cfg-audit] ...`；`CfgAuditError` 穿透 class_writer 的 stub 降级直接中止转译，
  其余翻译异常计入 `stub_fallback`。
- **单元测试**：`tests/unit/test_cfg_structuring.py` 直接构造块图（不依赖 JDK），覆盖 while 形态、嵌套 break、
  汇合标签块、switch 分组、不可归约兜底与自检报错。
