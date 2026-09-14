# java_rta/codegen 改进计划

> 创建日期：2026-09-14  
> 范围：`codegen/` 全部模块

---

## 一、当前代码生成器的主要不足

### 1.1 控制流分析能力严重不足

**现状**：`cfg.py` 只做 back-edge goto 检测（识别 while 循环）和三指令布尔模式识别。

**已知缺失**：
- `if/else` 无结构化恢复，直接生成裸 goto 跳转（依赖 `RawStmt` 逃生舱）
- `switch` / `lookupswitch` / `tableswitch`：只弹出 key，未生成 `match` 分支
- `try/catch/finally`：完全不支持，遇到异常块会产生错误代码
- 嵌套控制流（loop 内 if、多层 break/continue）：行为未定义

**影响**：凡是含有 if/else、switch、try/catch 的 Java 方法，生成代码要么错误，要么退化为 `panic!("stub")`。

**根本原因**：基于指令序列扫描的方法（当前方法）无法正确还原 JVM 字节码的控制流结构。JVM 字节码是基于跳转的非结构化格式，必须先构建控制流图（CFG），再做结构恢复。

**改进方向**：

**阶段一**：构建基本块（Basic Block）
- 识别 leader 指令（方法入口、跳转目标、跳转后的下一条）
- 每个 leader 到下一 leader 之间是一个基本块
- 建立基本块之间的前驱/后继边

**阶段二**：支配树
- 实现 Lengauer-Tarjan 算法（或简单的迭代式支配树算法）
- 计算每个基本块的支配者（idom）

**阶段三**：结构恢复
- 识别自然循环（回边 → loop header）
- 识别 if/else（条件跳转 → 合并点）
- 识别 switch（tableswitch/lookupswitch → match）
- 输出结构化节点树（`StructNode::Loop / If / Block / Switch`）

这是正确处理任意 JVM 字节码的唯一方法，也是当前所有覆盖率瓶颈的根因。

---

### 1.2 `instanceof` 语义错误

**现状**：`instr.py` 中 `instanceof` 指令直接压入 `Lit("true")`，完全忽略运行时类型。

```python
# instr.py 当前代码（占位符）
# instanceof → 直接 push true
sim.push(Lit('true'), BOOL)
```

**影响**：任何依赖 `instanceof` 结果的分支（多态检查、类型守卫、`Pattern Matching`）都产生错误的运行时行为。这是一个语义级 bug，不是生成质量问题。

**改进方向**：
- 在 Rust 生成侧，`instanceof` 应映射到运行时类型检查
- 若目标类型是 struct（已翻译的类），用 `downcast_ref::<T>().is_some()`
- 若目标类型是接口，用 `obj.implements("InterfaceName")`
- 需要配合类层次图（见 1.5）才能正确实现

---

### 1.3 缺少 SSA 变量类型精确推导

**现状**：`stack.py` 的 `StackSim` 在每条指令处理后立即生成 `LetStmt`，不同分支路径上相同 slot 的变量类型可能不一致，无合并（φ）逻辑。

**典型问题**：
- `for (int i=0; i<n; i++)` 中，`i` 在循环头（φ入口）和循环体（递增后的值）的"合并"由栈模拟隐式处理，类型不稳定
- 跨分支的变量：`Object x = cond ? new Foo() : new Bar();` 中，两条路径上 `x` 的类型不同，合并后应该是公共超类型

**影响**：生成代码在复杂分支结构下类型不一致，导致 Rust 编译错误。

**改进方向**：
- 引入 SSA（Static Single Assignment）构建阶段
- 每个 slot 在不同赋值点分配新的 SSA 变量（`v0`, `v1`, `v2`）
- 在控制流汇聚点插入 φ 函数确定合并类型
- 变量的最终 Rust 类型由 φ 函数的操作数的公共超类型决定

---

### 1.4 变量 `mut` 标注不精确

**现状**：`stack.py` 的 `store_local` 一律生成 `let mut`，不管变量是否真的被二次赋值。

**影响**：
- 生成代码充满多余的 `mut`，不符合 Rust idioms
- Clippy 报 `unused_mut` 警告，需要 `#[allow(unused_mut)]` 全局压制

**改进方向**：在 `gen_method_body` 的最后阶段（IR 已全部生成后）做后向扫描：
1. 收集所有 `AssignStmt.target` 的变量名
2. `_analyze_mutation` 已有初步实现（`method.py`），但尚未覆盖嵌套块
3. 扩展为递归遍历所有 `IfStmt.then/else_`、`LoopStmt.body`
4. 只有被二次赋值的变量才生成 `let mut`

---

### 1.5 缺少类层次图（Class Hierarchy）

**现状**：codegen 没有继承关系信息。翻译虚方法调用（`invokevirtual`）时，直接调用 `self.method()`，不知道接收者的实际运行时类型。

**影响**：
- 无法正确实现多态：子类重写的方法无法被正确路由
- `instanceof` 无法实现（见 1.2）
- 无法做接口方法的 dispatch（`invokeinterface`）
- 无法做调用链分析（判断哪些方法可达）

**改进方向**：构建 `ClassHierarchy`：

```python
@dataclass
class ClassHierarchy:
    # class_name → (super_class, interfaces, methods)
    classes: dict[str, ClassInfo]
    
    def is_subtype(self, sub: str, sup: str) -> bool: ...
    def lookup_virtual(self, recv_class: str, method_name: str, desc: str) -> str | None: ...
    def concrete_subclasses(self, class_name: str) -> list[str]: ...
```

在 `transpile.py` 中，先构建完整的类层次图，再传入 codegen 阶段使用。

---

### 1.6 类型系统过度依赖字符串

**现状**：`jvm_to_rust` 返回 `str`，后续的类型判断全是字符串比较：

```python
if rust_ret == 'Object':
    ...
if '/' in cls:  # 判断是否为 JDK 类
    ...
if rt.startswith('Rc<RefCell<Vec<'):  # 判断是否为数组
    ...
```

**影响**：
- 字符串比较脆弱，任何格式变化都会静默失败
- 无法在类型上做结构化操作（如：取数组元素类型、判断是否为泛型等）
- 泛型参数信息丢失（所有泛型类退化为 `Object`）

**改进方向**：将 `rs_ir.py` 中已经定义的 `RsType` 节点系统应用到类型映射层：

```python
def jvm_to_rs_type(desc: str, registry=None) -> RsType:
    # 返回 RsType 节点而非字符串
    if desc == 'I': return RsPrimitive('i32')
    if desc.startswith('['): return RsGeneric('Vec', [jvm_to_rs_type(desc[1:])])
    # ...
```

这样类型检查变为 `isinstance(ty, RsGeneric) and ty.outer == 'Vec'`，不依赖字符串格式。

---

### 1.7 泛型类型精确度不足

**现状**：`sig_parser.py` 中，带泛型参数的类类型（如 `List<E>`、`Map<K,V>`）暂时简化为 `Object`：

```python
if has_type_args:
    rust_type = 'Object'  # 带泛型参数的类类型 → 暂时简化为 Object
```

数组类型也简化为 `Object`：

```python
if c == '[':
    return 'Object', next_i
```

**影响**：
- 泛型集合操作（`list.get(i)` 返回 `Object`，而非具体元素类型）需要手写 downcast
- 泛型方法的返回值类型丢失，生成大量无必要的 downcast

**改进方向**：

**阶段一**：局部变量类型表（LVTT）驱动
- `classfile.py` 已解析 `LocalVariableTypeTable`，`ParsedMethod.local_types` 已有泛型签名
- 在 `StackSim.store_local` 时，若 LVTT 提供具体泛型类型，优先使用

**阶段二**：调用站点泛型替换
- 记录每个 SSA 变量的声明类型（含泛型参数）
- 调用泛型方法时，用接收者的具体类型参数替换被调方法的 TypeVar

---

### 1.8 `RawExpr` / `RawStmt` 使用过多

**现状**：`rs_ir.py` 定义了 `RawExpr`/`RawStmt`/`RsRawItem` 逃生舱，在 `instr.py` 和 `method.py` 中被大量使用。

**影响**：
- 逃生舱内的字符串无法被后续 pass 分析（mutation 分析、优化）
- 难以区分"有意的 raw"（特殊语法）和"还未实现的 IR 化"
- 重构时容易漏掉 raw 内容

**改进方向**：以消除 `RawExpr`/`RawStmt` 为目标，逐步将每类用途替换为对应的 IR 节点：
1. 方法调用 → `MethodCall` / `Call`
2. 数组访问 → `Index`
3. 类型转换 → `Cast`
4. 字段访问 → `FieldAccess`

目标：`RawExpr` 只留给真正无法用 AST 表达的 Rust 语法（如属性宏、unsafe 块）。

---

## 二、改进优先级排序

| 优先级 | 改进项 | 收益 | 成本 |
|---|---|---|---|
| P1 | **1.4 精确 `mut` 标注** | 消除 unused_mut 警告 | 低（扩展现有 `_analyze_mutation`） |
| P1 | **1.2 instanceof 修复** | 消除语义级 bug | 中（需要类型检查基础设施） |
| P2 | **1.1 if/else 结构恢复** | 大幅扩展覆盖率 | 高（需要 CFG + 支配树） |
| P2 | **1.5 类层次图** | 解锁多态、instanceof、接口 dispatch | 高（需要全量类解析） |
| P3 | **1.1 switch 恢复** | 覆盖 switch 密集型代码 | 中（CFG 基础上） |
| P3 | **1.1 try/catch 恢复** | 覆盖异常处理代码 | 高（异常表解析 + 结构恢复） |
| P3 | **1.6 类型系统 RsType 化** | 消除字符串比较脆弱性 | 中（渐进替换） |
| P4 | **1.3 SSA 构建** | 类型精确性根本性提升 | 极高（架构级） |
| P4 | **1.7 泛型精确化** | 减少 downcast | 高（依赖 SSA） |
| P5 | **1.8 消除 RawExpr** | 代码质量 | 低但持续 |

---

## 三、P1 改进的具体实施方案

### 精确 `mut` 标注（扩展 `_analyze_mutation`）

当前 `_analyze_mutation` 只处理顶层 `stmts`，不递归：

```python
# method.py 当前实现（不完整）
def collect(ss):
    for stmt in ss:
        if isinstance(stmt, AssignStmt) and isinstance(stmt.target, Var):
            assigned.add(stmt.target.name)
        if isinstance(stmt, IfStmt):
            collect(stmt.then)
            collect(stmt.else_)  # ← 已有，但 else_ 可能为 None
        if isinstance(stmt, LoopStmt):
            collect(stmt.body)
```

**修复**：
```python
def collect(ss):
    for stmt in ss:
        if isinstance(stmt, AssignStmt) and isinstance(stmt.target, Var):
            assigned.add(stmt.target.name)
        elif isinstance(stmt, IfStmt):
            collect(stmt.then)
            if stmt.else_:
                collect(stmt.else_)
        elif isinstance(stmt, LoopStmt):
            collect(stmt.body)
        elif isinstance(stmt, RawStmt):
            # 保守处理：raw 代码中可能有赋值，无法分析，标记为可能 mut
            pass
```

`StackSim.store_local` 中，`LetStmt` 初始生成时全部用 `mutable=False`，最后由 `_analyze_mutation` 将需要 mut 的变量改为 `True`。

### instanceof 修复方案

```python
# instr.py: instanceof 指令
def _handle_instanceof(sim, check_class):
    obj_expr, obj_ty = sim.pop()
    # 生成: obj.downcast_ref::<ClassName>().is_some()
    check_name = short_cls(check_class)
    result = RawExpr(
        f'({render_expr(obj_expr)}).downcast_ref::<{check_name}>().is_some()'
    )
    sim.push(result, BOOL)
```

需要 `Object` 类有 `downcast_ref::<T>() -> Option<&T>` 方法（在 `java_runtime` 手写层实现）。

---

## 四、if/else 结构恢复路线图

分三步渐进实现，每步都保持向后兼容：

**Step 1**：基本块切割（约 150 行）
```python
def build_basic_blocks(instrs: list[Instr]) -> list[BasicBlock]:
    # 识别 leader：方法入口 + 跳转目标 + 跳转后的下一条
    # 每个 BasicBlock: instrs + succs (前向边集合)
```

**Step 2**：简单 if 识别（条件跳转 → 两路分叉 → 合并点）
```python
def find_if_else(bbs: list[BasicBlock]) -> list[StructNode]:
    # 条件跳转 BB → then_bb / else_bb → merge_bb
    # 生成 IfStmt(cond, then_stmts, else_stmts)
```

**Step 3**：自然循环识别（回边 → loop header）
```python
def find_loops_cfg(bbs: list[BasicBlock]) -> list[LoopNode]:
    # 回边 (n → h)：h 是 n 的支配者
    # 生成 LoopStmt(header_bb, body_bbs, exit_bb)
```

每步完成后独立测试，不影响现有正常工作的代码路径。

---

---

## 五、应规避的架构陷阱

本节记录同类代码生成器项目中已经发生的、代价高昂的设计失误，供 java_rta 在演进时主动规避。

### 5.1 语义桩伪装成合理实现

**陷阱**：将"尚未实现"的功能以看似合理但语义错误的桩代码形式混入生成结果，而非显式 `panic!` 或编译期错误。调用方在运行时遭遇难以追踪的逻辑错误，而不是在生成阶段就被明确告知。

**java_rta 当前已有此问题**：`instanceof` 指令硬编码返回 `true`，`switch` 指令只弹栈不生成分支。任何依赖这两个指令结果的程序都会静默产生错误行为，且无任何编译期警告。

**规避原则**：
- 未实现的指令/功能，一律生成 `todo!("opcode: {}")` 或 `unimplemented!(...)` 形式的 panic，永远不生成伪造的合理返回值
- 在生成的代码文件头部加注释标记哪些方法是 stub，方便后续追踪
- 建立"已知 stub 列表"，每次 stub 数量应递减而非递增

---

### 5.2 类型系统用字符串而非结构化 ADT 表示

**陷阱**：用字符串表示类型（如 `'Rc<RefCell<Vec<i32>>>'`），后续所有类型判断都退化为字符串模式匹配（`startswith`、`in`、`==`）。任何格式微小的变化（如空格、别名）都会导致静默的判断失败，且完全没有类型安全性。

**典型危害**：
```python
if rt.startswith('Rc<RefCell<Vec<'):  # 脆弱：格式稍变即失效
    elem = rt[len('Rc<RefCell<Vec<'):-3]  # 魔法切片
```

`rs_ir.py` 已经定义了完整的 `RsType` ADT（`RsPrimitive`/`RsNamed`/`RsGeneric`/…），但 `type_map.py` 的 `jvm_to_rust` 仍返回字符串，两层之间没有打通。

**规避原则**：
- `jvm_to_rust` 的返回值应迁移为 `RsType` 节点
- 类型判断全部改用 `isinstance(ty, RsGeneric) and ty.outer == 'Vec'` 形式
- 禁止在代码中出现对类型字符串的 `startswith` / `endswith` 判断

---

### 5.3 逃生舱（RawExpr / RawStmt）无节制增长

**陷阱**：`RawExpr` / `RawStmt` 逃生舱在短期内方便快捷，但每增加一处，后续的 mutation 分析、优化 pass、测试验证就多一块"暗区"。积累到一定规模后，整个 IR 层形同虚设——所有有趣的内容都在 raw 字符串里，无法被程序分析。

**java_rta 当前状态**：`instr.py` 中大量使用 `RawExpr(f"...")`，`method.py` 中也有 `RawStmt` 用于复杂情况。当前 `_analyze_mutation` pass 对 `RawStmt` 只能保守处理，无法判断其中是否有赋值。

**规避原则**：
- 每次新增 `RawExpr`/`RawStmt` 时必须同时提 issue，描述"如何 IR 化"
- 在代码审查中，`RawExpr` 数量增加视为需要特别审查的信号
- 设定目标：每个版本 `RawExpr` 总数递减

---

### 5.4 方法名 mangle 设计不顾可读性

**陷阱**：为了保证唯一性，方法名 mangle 规则将完整的 JVM 描述符编码进方法名，产生极长、极难阅读的名称（如 `someMethod_Ljava_lang_String_ILjava_util_List_`）。这类名称在编译错误消息中出现时，开发者几乎无法从中定位到原始 Java 方法。

**规避原则**：
- mangle 名称应**人类可读**：用类型缩写（`str`/`obj`/`int`/`arr`）而非完整 JVM 类名
- `mangle_name` 的当前实现（`add_obj`、`set_str_int`）方向正确，应坚持这一风格，不引入长格式
- 即使在少数碰撞场景下，也优先用数字后缀（`method_2`）而非编码完整描述符

---

### 5.5 手写覆盖粒度过粗导致维护发散

**陷阱**：若手写覆盖的单位是"整个类"，当类中只有少数方法需要手写时，开发者必须维护整个类的所有方法（包括可以自动生成的部分）。随着生成逻辑不断改进，手写版本与自动生成版本之间的差距越来越大，合并越来越困难，最终形成"两套实现并行，都有 bug"的局面。

**规避原则**：
- 手写覆盖从第一天起就以**方法**为粒度设计，而非以类为粒度
- 生成的类文件应能通过文件包含（`include!`）或 trait delegation 接受方法级 patch
- 整类手写应被视为临时方案，每个整类手写文件都应有对应的"何时可以移除"说明

---

### 5.6 多模式生成缺少独立验证

**陷阱**：代码生成器支持多种输出模式（如"生成 JDK 类"vs"只生成用户类"、"带调用链剪枝"vs"全量生成"），但测试只覆盖最常见的单一模式。特殊模式下的代码路径在大规模生成后才发现问题，此时错误数量庞大，根因分散，修复困难。

**java_rta 当前状态**：`transpile.py` 支持方法级调用链剪枝（`visited_methods`）和全量生成两种模式，但测试覆盖以全量模式为主。

**规避原则**：
- 每种 CLI 模式 / 主要参数组合都有独立的最小 e2e 测试（HelloWorld 级别）
- 新增模式时，先写测试再写实现
- CI 中确保所有模式的测试都被执行，不允许只有"默认模式"通过

---

### 5.7 错误修复引入回归而不自知

**陷阱**：在修复一个 codegen bug 时，改动了多个模块共享的核心函数（如类型映射、名称生成），导致原本正常工作的其他类/方法出现新的编译错误。由于生成代码量大，回归不立即显现，积累后一次性爆发数千个错误。

**规避原则**：
- 核心函数（类型映射、名称 mangle、控制流分析）的每次改动都必须运行全套 e2e 测试
- 建立"黄金样例"集（至少 10 个典型 Java 程序），每次改动后对比生成代码的 diff，确认无意外变化
- 大型 codegen 改动拆分为小步，每步单独验证，不一次性合并多个 bug 修复

---

*本文档不含过渡方案。所有改进均面向最终状态设计。*
