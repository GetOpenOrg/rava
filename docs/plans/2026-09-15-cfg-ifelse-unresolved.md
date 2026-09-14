# CFG if/else 实现 — 未解决问题记录

> 创建日期：2026-09-15  
> 状态：进行中

---

## 已完成

| 功能 | 状态 |
|------|------|
| if-else merge-point 基础结构 | ✅ |
| simple if-then（无 else 分支）| ✅ |
| if-guard（fall-through 必定退出）| ✅ |
| loop body 内嵌 if-guard（break）| ✅ |
| ternary 表达式检测 | ✅ |
| ternary 类型不一致时自动强转（bool/int、as 转换）| ✅ |
| 递归 process_block：if-else 嵌套 / loop 内嵌套 | ✅ |
| Thread.interrupt() native impl（避免 PartialEq 比较）| ✅ |
| Thread.getThreadGroup() native impl（避免 stack underflow）| ✅ |

---

## P1 未解决问题

### 1. continue 指令（goto → loop_start）

**现象**：`goto <loop_start>` 在 loop body 内部时应生成 `continue;`，当前生成空 if body 或跳过。

**根因**：`find_if_guards` 只检测 `goto >= exit_offset`（break），未处理 `goto == loop_start`（continue）。

**修复方向**：
- 在 `find_if_guards` 的 break 检测逻辑中，同时检测 `goto_tgt == instrs[lp.start_idx].offset`（= loop start offset）
- 在 `method.py` guard body 处理中，与 break 并列地生成 `continue;`

---

### 2. ternary 中 method call 结果被物化为 let 语句

**现象**：当 then/else 分支包含方法调用时（如 `Thread::virtualThreadGroup()?`），`sim_instr` 会将调用结果物化为 `let _t0 = call()?;`，导致 `then_out` 非空，ternary 检测失败，退化为 if-else 语句块。

**影响**：`Thread.getThreadGroup()` 等方法被 native_impl 覆盖规避，但其他类似情况会生成不必要的非 ternary 代码。

**修复方向**：
1. 检测 `then_out` 只含单个 let-binding 且 `then_s.stack` 有 1 个额外值时，允许 ternary（用已绑定变量名作为 then 值）
2. 或：为 ternary 场景提供"延迟物化"模式，避免 method call 提前生成 let 语句

---

### 3. do-while 循环未检测

**现象**：`find_loops` 只检测 back-edge 是无条件 goto 的情况。do-while 的 back-edge 是条件跳转（`if_icmplt <loop_start>`），不被识别为循环，生成顺序执行代码。

**修复方向**：
- 扩展 `find_loops`：检测条件跳转后向 jump（`opcode in _BRANCH_OPS and target_offset < ins.offset`）
- 将 do-while 生成为：`loop { body; if !cond { break; } }`

---

### 4. if-else 体内 method call 导致 ternary 退化（与 #2 相同根因）

对于任何 then/else 分支中含方法调用的 if-else，当前生成完整 if/else 语句块而非 ternary 表达式，导致返回类型推断失效（stack underflow）。

**受影响方法示例**：`Thread.getThreadGroup()`（已用 native_impl 规避）

---

## P2 未解决问题

### 5. switch/tableswitch/lookupswitch 未实现

**现象**：遇到 switch 字节码时生成顺序执行，语义错误。

**修复方向**：在 `cfg.py` 增加 `find_switches`，生成 `match` 表达式。

---

### 6. try/catch/finally 完全不支持

**现象**：异常处理表（exception_table）中的 handler 未被处理，try body 生成正常但 catch/finally 丢失。

**修复方向**：需要引入异常处理 CFG 分析，属于大型任务。

---

### 7. 短路逻辑 `&&` / `||` 不能识别为单一布尔表达式

**现象**：JVM 短路求值对应两个连续条件跳转，当前生成嵌套 if（功能正确但繁琐）。

**示例（String.isLatin1）**：
```rust
// 当前生成（功能正确）
(if COMPACT_STRINGS { (if (coder==0) { true } else { false }) } else { false })
// 期望
COMPACT_STRINGS && (coder == 0)
```

**修复方向**：在 `find_if_else` 中增加短路模式检测（嵌套 if-else 且 else 值相同）。

---

## native_impl 覆盖记录

以下方法因代码生成问题使用 native_impl 临时规避，未来应删除后由正确的字节码翻译替代：

| 类 | 方法 | 问题原因 |
|----|------|----------|
| Thread | isTerminated() | Thread_State 未实现 PartialEq |
| Thread | interrupt() | `this != currentThread` 需要 PartialEq |
| Thread | getThreadGroup() | then 分支 method call 导致 ternary 检测失败 |
