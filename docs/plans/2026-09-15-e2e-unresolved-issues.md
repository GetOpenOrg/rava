# E2E 测试未解决问题追踪

> 创建日期：2026-09-15  
> 基于测试套件：`tests/e2e/`（60 个测试）  
> 更新方式：每次 E2E 运行后手动补充/关闭条目

---

## 问题分类索引

| 类别 | 问题数 | 优先级 | 影响范围 |
|------|--------|--------|---------|
| [A. 编译错误 — 类型转换](#a-编译错误--类型转换) | 4 | 高 | 15+ 测试 |
| [B. 编译错误 — 缺失 From impl](#b-编译错误--缺失-from-impl) | 2 | 高 | 10+ 测试 |
| [C. 编译错误 — 缺失模块/import](#c-编译错误--缺失模块import) | 1 | 高 | 多个 jdk_classes |
| [D. 编译错误 — invokedynamic 无法翻译](#d-编译错误--invokedynamic-无法翻译) | 1 | 中 | lambda/stream 测试 |
| [E. 编译错误 — 重复定义](#e-编译错误--重复定义) | 1 | 中 | 特定 native_impl 冲突 |
| [F. 运行时错误 — RefCell 双重借用](#f-运行时错误--refcell-双重借用) | 1 | 中 | TestSorting |
| [G. 运行时错误 — 多态/虚方法派发](#g-运行时错误--多态虚方法派发) | 1 | 低 | TestRecord 等 |
| [H. 运行时错误 — 输出值错误](#h-运行时错误--输出值错误) | 2 | 低 | TestBoundedGenerics 等 |
| [I. 编译层补丁 — 待架构升级后删除](#i-编译层补丁--待架构升级后删除) | 6 | 中 | 全局质量 |
| [J. 命名原则违规 — `java_runtime` 游离 trait](#j-命名原则违规--java_runtime-游离-trait) | 3 | 中 | 命名一致性 |
| [K. 代码布局重构 — native_impls 共置与内部包边界截断](#k-代码布局重构--native_impls-共置与内部包边界截断) | 4 | 高 | 全局基础设施 |

---

## A. 编译错误 — 类型转换

### A-1: `char`/`u16` 返回值缺少 `as i32` 强制转换

**错误**：`E0308 mismatched types: expected i32, found u16`  
**位置**：`jdk_classes/src/java/lang/character.rs:605`  
**根因**：JVM `ireturn` 指令处理时，对于返回 `u16`（char 类型）的方法没有插入 `as i32` 转换。`sim.py` 的 `ireturn` 分支只处理 `bool` → `i32`，未处理 `u16` → `i32`。  
**影响测试**：TestStreamAdvanced、TestStringOps 及所有包含 `character.rs` 的测试  
**修复位置**：`codegen/instr/coerce.py` — `_coerce_value` 中 `target == 'i32'` 分支补充 `u16/i8/i16`  
**状态**：✅ 已修复（2026-09-15）

---

### A-2: 方法参数类型不匹配（子类传给父类参数）

**错误**：`E0308: expected InputStream, found InflaterInputStream`  
**位置**：`jdk_classes/src/java/lang/character_name.rs:72`  
**根因**：`invoke.py` 在生成方法调用参数时，只用 `_is_direct_subtype` 检查是否需要 `.into()`，不检查传递子类关系。`InflaterInputStream` 是 `InputStream` 的间接子类，需要 `.into()` 转换。  
**影响测试**：TestStreamAdvanced、TestStringAdvanced 等  

**重要约束**：不能简单改为 `_is_subtype`（传递性）。`.into()` 必须有对应的 `From` impl 才合法；当前 `class_writer.py` 只为直接父类/接口生成 `From` impl（T55b），对间接子类用 `.into()` 会产生 E0277。两个规则必须保持一致：
- 当前正确做法：`.into()` 仅在有 `From` impl 时生成 → 用 `_is_direct_subtype`（一步）
- **架构解法（Arch-1）**：`java_class` 宏读取 `interfaces` 和 `super_class`，通过 `JvmObject` 的 trait 方法实现真正的动态协变，从根本上不需要 `.into()` 这种静态转型。届时 `_is_direct_subtype` 和 `_is_subtype` 两套规则均可删除。

**修复位置**：临时无法修复（改 `_is_subtype` 会引入新的 E0277）；根本修复见 Arch-1。  
**状态**：🔴 已确认接受当前编译错误，不引入新补丁，等 Arch-1 一并解决

---

### A-3: 接口类型参数传入（如 `AbstractStringBuilder → CharSequence`）

**错误**：`E0308: expected CharSequence, found String`  
**位置**：`jdk_classes/src/java/lang/abstract_string_builder.rs:305`  
**根因**：`appendChars` 方法声明参数类型为 `CharSequence`（接口），实际传入 `String`（实现类）。codegen 没有为实参插入 `.into()`。  
**影响测试**：TestStringBuilder、TestStringFormat  
**修复位置**：`codegen/instr/invoke.py` — 参数强制转换  
**关联**：与 A-2 同一修复位置  
**状态**：🔴 未修复

---

### A-4: `PrintStream → Appendable` 构造参数类型不匹配

**错误**：`E0308: expected Appendable, found PrintStream`  
**位置**：`jdk_classes/src/java/io/print_stream.rs:368`  
**根因**：`Formatter::new_append` 接受 `Appendable`（接口），`this.clone()` 是 `PrintStream`（实现类）。同 A-3，invoke 侧缺少 `.into()` 插入逻辑。  
**影响测试**：多个包含 `Formatter` 的测试  
**修复位置**：`codegen/instr/invoke.py`  
**状态**：🔴 未修复

---

## B. 编译错误 — 缺失 From impl

### B-1: ~~Arrays_ArrayList → List 缺少 From impl~~ ⚠️ 编译层修复（语义仍有问题）

**修复方案**：`class_writer.py` T55b 扩展为同时收集祖先类的接口，生成传递 `From` impl。  
**影响测试**：TestLinkedList、TestComparator、TestStreamAdvanced、TestStreamBasic  
**遗留问题**：当前生成的 `From` impl 实现为 `Default::default()`，丢弃了被转换的具体值（见 Arch-5）。编译可以通过，但运行时接口对象是空的，方法调用会 panic。  
**状态**：⚠️ 编译错误已消除，运行时语义待修复（依赖 Arch-1）

---

### B-2: `AbstractStringBuilder → Appendable` 缺少 From impl

**错误**：`E0277: the trait bound Appendable: From<&AbstractStringBuilder> is not satisfied`  
**位置**：`jdk_classes/src/java/io/print_stream.rs`  
**根因**：`From` impl 是为 owned 值生成的，但代码里传入的是 `&AbstractStringBuilder` 引用。T55b 生成的 `From<AbstractStringBuilder> for Appendable` 无法满足 `From<&AbstractStringBuilder>` 约束。  
**影响测试**：TestStringBuilder、TestStringOps  
**修复思路**：调用点改为 `.clone().into()` 而不是 `(&this).into()`，或者 invoke.py 对引用类型加 clone  
**状态**：✅ 已修复（2026-09-15）— invoke.py T55 `.into()` 分支改为 `Clone::clone(&e).into()`，确保传递 owned 值

---

## C. 编译错误 — 缺失模块/import

### C-1: use 语句引用不存在的 jdk 包

**错误**：`E0432: unresolved import crate::java::lang::r#ref`、`crate::java::lang::reflect`、`crate::sun::security::util` 等  
**位置**：多个 jdk_classes 生成文件的 `use` 块  
**根因**：codegen 把 JDK 类的全部依赖包都写进 `use crate::...::*;`，但这些包（如 `java::lang::r#ref`、`java::lang::reflect`、`sun::security::util`）在当前测试的 call chain 中没有类，因此没有生成对应模块。  
**影响测试**：所有包含 jdk 内部类（如 `PrintStream`、`Class`）的测试  
**修复位置**：`codegen/emitter/project_writer.py` — 写出 `use` 前检查包路径是否在已生成的模块列表中，不存在则跳过  

**附加问题 — `preconditions.rs` 文件缺失**：部分测试（TestBoundedGenerics 等）生成的 `jdk_classes/src/jdk/internal/util/preconditions.rs` 包含：
```rust
#[path = "../../../../../native_impls/jdk/internal/util/preconditions.rs"]
mod _impl;
```
但 `output/native_impls/jdk/internal/util/preconditions.rs` 不存在，导致 `couldn't read ... preconditions.rs: No such file or directory`。  
临时修复：创建空占位文件。根本修复：`class_writer.py` 在写 `#[path = ...]` 前调用 `os.path.exists(_impl_abs)`，不存在则省略该行（已实现，2026-09-15）。  
**状态**：⚠️ 主问题（use 引用不存在包）未修复；附加问题（#[path] preconditions）已修复

---

## D. 编译错误 — invokedynamic 无法翻译

### D-1: Lambda/方法引用 invokedynamic 未实现

**错误**：后续指令操作错误的栈顶值，生成语义错误的代码（如 `downcast::<Serializable>()`），导致 E0308  
**位置**：生成代码中所有含 `invokedynamic` 的方法（如 `Comparator.comparingInt`）  
**实际生成代码**：`/* TODO: invokedynamic 63 */`（注释，不是 `todo!()`）  
**根因（关键）**：`sim.py` 对 `invokedynamic` 只输出注释，**既不弹出捕获变量，也不压入返回值**。JVM 语义是：消耗 N 个操作数（lambda 捕获的变量），向栈压入 1 个函数式接口实例。  
当前结果：`invokedynamic` 后栈上仍残留原捕获变量（如 `keyExtractor: Object`），后续 `checkcast Serializable` 将其强转为 `Serializable`，再 `areturn` 时类型与声明 `Comparator<Object>` 不符 → E0308。  
**影响测试**：TestLambda、TestMethodRef、TestFunctionalInterface、TestComparator、TestStreamBasic、TestStreamAdvanced、TestStreamCollectors（7 个测试）  
**修复方案**：  
  - **短期（修复编译）**：解析 invokedynamic 方法描述符确定参数数量，从栈弹出对应操作数；压入 `Object::default()` 占位返回值。这样后续 `checkcast`/`areturn` 拿到正确的 `Object` 类型，不再产生 E0308。  
  - **长期（修复语义，见 Arch-3）**：识别 bootstrap 方法，生成 Rust 闭包 `Arc<dyn Fn>` 并包装入 `Object`  
**修复位置**：`codegen/instr/sim.py` — `invokedynamic` 分支  
**状态**：✅ 短期方案已实现（2026-09-15）— sim.py 已解析 descriptor 弹出 N 个参数并压入 Object::default() 占位符；long-term 语义修复见 Arch-3

---

### D-2: `areturn` 类型不兼容的 `Default::default()` 兜底（补丁）

**错误**：编译可通过，但运行时语义错误（返回空默认值而非真实值）  
**位置**：`codegen/instr/sim.py` areturn 分支（lines 566-571）  
**根因**：当 `actual_ty`（栈顶类型）与 `ret_ty`（声明返回类型）不兼容时，sim.py 降级为 `Default::default()`。这是一个**临时补丁**，真正的类型不兼容来源是 D-1（invokedynamic 栈状态错误）。  
修复 D-1 后（invokedynamic 短期方案：弹出操作数 + 压入 Object::default()），`checkcast` 的源类型变为 `Object`，`areturn` 可正常处理，此补丁可以删除。  
**状态**：⚠️ 临时补丁，根本原因是 D-1

---

## E. 编译错误 — 重复定义

### E-1: ~~native_impl 中的方法与字节码翻译重复~~ ✅ 部分修复（2026-09-15）

**修复方案**：从 `native_impls/java/lang/string.rs` 删除了已有字节码翻译的 `substring_i_i`、`substring_i`；`native_impls/java/util/array_list.rs` 的 `iterator()` 返回类型改回 `Object`。  
**剩余风险**：若后续字节码翻译了其他 native_impl 中手写的方法，仍会出现 E0592。  
**根本修复**：`_scan_native_impls` 扫描出 native_impl 已定义的方法名，`class_writer` 生成时跳过该方法的字节码翻译。  
**状态**：⚠️ 临时修复，根本方案未实施

---

## I. 编译层补丁 — 待架构升级后删除（共 6 项）

以下修复已使代码可编译，但属于局部补丁，根本原因是架构问题。记录于此以便追踪清理。

### I-1: `_is_generic_type_param()` 字符串启发式判断（5+2处）

**当前做法**：在 `invoke.py`（4处）和 `sim.py`（1处）中，当 expected 类型是 `Object`、实际类型名长度 ≤ 2 且首字母大写时，跳过 `Object::from_any()` 包装。

**已知遗漏（V5）**：`codegen/method/codegen.py:498` 和 `:531` 的 if/else 分支类型合并处同样调用 `Object::from_any()`，但未引入 `_is_generic_type_param` 保护。当 `ty_str == 'Object'` 而实际值是泛型参数 `T` 时会错误包装，与 `invoke.py` 的处理不一致。  
**临时修复**：在这两处加入 `and not _is_generic_type_param(ety_str)` / `and not _is_generic_type_param(ty_str)` 判断，与 `invoke.py` 保持一致。  
**问题**：靠字符串形状猜测泛型参数，脆弱且不准确（`Rc` 首字母大写也会被误判，须同时校验 `.isalpha()`）。  

**根本原因**：`jvm_to_rust()` 只读方法描述符（`descriptor`），丢弃 `generic_signature`。以 `Reference.<init>` 为例：

```rust
// java_method 注解中同时存在两个字段：
descriptor        = "(Ljava/lang/Object;)V"   // 擦除后：Object
generic_signature = "(TT;)V"                  // 真实类型：泛型参数 T
```

codegen 读取 `descriptor` → `expected = "Object"` → 错误包装 `Object::from_any(referent)`；  
若改为读取 `generic_signature` → `expected = RsGenericParam("T")` → 直接传入 `referent`，无需启发式。

**架构解法（Arch-7）**：`jvm_to_rust` 同时接受 `generic_signature` 参数，返回 `RsType` 枚举节点；codegen 在生成 `invokespecial/invokestatic/invokevirtual/putfield` 时优先使用 `generic_signature` 中的参数类型，此处 5 个补丁全部删除。

**状态**：✅ 已实施（2026-09-16，Arch-7）

---

### I-2: `_is_direct_subtype` 与 `_is_subtype` 并存（见 B-1、A-2）

**当前做法**：T55（子类型 `.into()` 提升）使用 `_is_direct_subtype`（只走一步），而其他子类型检查用 `_is_subtype`（传递性）。两套规则并存，约定隐式。  
**架构解法（Arch-1）**：`java_class` 宏通过 `interfaces` 字段自动生成正确的 trait impl，运行时靠 `dyn JvmObject` 动态协变，不再需要静态 `.into()` 链，两套规则均删除。

---

### I-3: 三元表达式类型统一 7 个 `elif`（重复两处）

**当前做法**：`codegen/method/codegen.py` 中三元表达式和 merge 变量处，各有一段 7 个 `elif` 链（内容相同，重复维护），处理 bool/int 转换、null 表达式、Object↔具体类型等情形。  
**架构解法（Arch-7）**：引入 `coerce(from: RsType, to: RsType, expr: str) -> str` 统一函数，`_null_exprs` 字符串集合替换为 `RsType::is_null_literal()` 方法，两处重复 elif 链合并为一次调用。

---

### I-4: `this.clone()` 字符串比较特判

**当前做法**：`codegen/method/codegen.py` 三元和 merge 变量处，`if tv == 'this': tv = 'this.clone()'`，靠字符串比较补偿 `let this = self;` 产生 `&Self` 引用与值位置不兼容问题。  
**架构解法（Arch-7）**：Stack 类型节点区分 `RsRef(&Self)` 和 `RsOwned(Self)`，任何值位置使用引用节点时统一生成 `.clone()`，无需字符串匹配。

---

### I-6: `clone` → `jvm_clone` 强制重命名（违反命名原则2）

**当前做法**：`codegen/emitter/method_gen.py:157-159` 和 `codegen/method/codegen.py:102-104` 中定义 `_JAVA_RUST_RENAME = {'clone': 'jvm_clone'}`，全部生成代码中 Java `clone()` 方法被重命名为 `jvm_clone`。  
**受影响范围**：搜索生成代码可见 150+ 处 `jvm_clone` 调用，覆盖 `java/lang/Object`、`ArrayList`、`Enum` 等所有类。  
**问题**：`jvm_clone` 是人造的 `jvm_` 前缀名称，不存在于 Java 命名空间，违反 CLAUDE.md 命名原则2。  
**引入原因**：Rust 中 `clone()` 是 `Clone` trait 的方法，与 Java `clone()` 共享名字会产生歧义或冲突。  

**确认的最终态方案**：删除 `_JAVA_RUST_RENAME`，保留原名 `clone`。两者可共存：

```rust
// Java Object.clone() 翻译为 inherent 方法
impl Point {
    pub fn clone(&self) -> Result<Object> { ... }   // Java clone()，返回 Result<Object>
}
// Rust Clone trait 独立实现
impl Clone for Point {
    fn clone(&self) -> Self { ... }                 // Rust Clone::clone()，返回 Self
}
// 调用时用完全限定语法消歧：
let rust_copy: Point = Clone::clone(&p);            // Rust 级别 clone
let java_copy: Object = p.clone()?;                 // Java 语义 clone
```

签名不同（`Result<Object>` vs `Self`），Rust 允许 inherent 方法与 trait 方法同名，调用 `p.clone()` 优先解析 inherent 方法（Java clone），Rust-level clone 用 `Clone::clone(&p)`。

**修复位置**：删除 `codegen/emitter/method_gen.py` 和 `codegen/method/codegen.py` 中 `_JAVA_RUST_RENAME` 的 `clone` 项。不依赖任何架构变更，可立即实施。  
**状态**：🟠 已知违规，方案已确认，独立可修（不依赖 Arch-7）

---

### I-5: `unreachable!()` 末尾补全

**当前做法**：`codegen/method/postprocess.py`：当函数末尾最后一行不是 `Ok(...)` 时追加 `unreachable!()`，防止 Rust 报 E0317（if 缺少 else）。  
**根本原因**：`while(true)` 循环被展平为直线代码后，goto 回边丢失，Rust 看到 `if` 没有 else，函数在某条路径"掉出"末尾。  
**架构解法（Arch-8，CFG 重建）**：构建 CFG + 支配树，识别自然循环，生成正确的 `loop { ... if cond { break; } }` 结构，函数末尾永远是 `Ok(...)` 表达式，此补丁删除。  
**状态**：✅ 已实施（2026-09-16，Arch-8）— `function_always_returns(instrs)` CFG 分析替代启发式检测，`unreachable!()` 完全消除

---

## F. 运行时错误 — RefCell 双重借用

### F-1: 同一语句中 `borrow_mut` 和 `borrow` 同时持有

**错误**：`already borrowed: BorrowMutError` (panic at runtime)  
**位置**：`user/src/TestSorting.rs`（生成代码）  
**根因**：Java `arr[j] = arr[j+1]` 翻译为：
```rust
arr.borrow_mut()[j] = arr.borrow()[j+1];  // panic!
```
右侧 `arr.borrow()` 持有不可变借用，左侧 `arr.borrow_mut()` 同时请求可变借用，违反 RefCell 规则。  
**影响测试**：TestSorting  
**修复思路**：把 aastore 和 aaload 拆成两个语句（先读后写）：
```rust
let _tmp = arr.borrow()[j+1].clone();
arr.borrow_mut()[j] = _tmp;
```
**修复位置**：`codegen/instr/sim.py` — `aastore` 分支，检测 rhs 是否来自同一数组引用  
**状态**：✅ 已修复（2026-09-15）— 对非基本类型 val_str 含 `.borrow()` 时先提取 tmp 变量再赋值

---

## G. 运行时错误 — 多态/虚方法派发

### G-1: `Object.toString()` 不调用具体类型的覆盖版本

**错误**：输出 `Object` 而非 `Point[x=3, y=4]`  
**位置**：TestRecord（`println_v(Object::from_any(p.clone()))`）  
**根因**：`println_v` 调用 `Object` 类型的 `to_print_string()`，返回 `format!("{}", self)`，而 `Object` 的 `Display` impl 不会 dispatch 到具体类型的 `toString()`。运行时多态缺失。  
**影响测试**：TestRecord、TestObjects（任何把具体类型转为 Object 再打印的场景）  
**修复思路**：在 `Object` 内部用函数指针或 `Arc<dyn Fn() -> String>` 存储 `toString` 实现；或生成代码时在 `from_any` 处捕获 `to_string_fn`。  
**状态**：🔴 未修复（架构层面问题）

---

## H. 运行时错误 — 输出值错误

### H-1: TestBoundedGenerics 输出 0.0 而非正确值

**错误**：期望 `15.0`，实际输出 `0.0`  
**根因**（已确认）：`main()` 调用 `sum(ints)` 时，`ints` 是 `Rc<RefCell<Vec<i32>>>` (primitive int array)，而 `sum` 声明参数为 `Rc<RefCell<Vec<Number>>>` (object array)。codegen 无法将 `Vec<i32>` 强制转换为 `Vec<Number>`，退化为 `Default::default()`（空 Vec），导致 `sum` 计算空数组，返回初始值 `0.0`。

生成代码（`test_bounded_generics.rs:73`）：
```rust
let _t1: f64 = Self::sum(Default::default())?;  // 实际 ints 完全没传入
```

根本原因是 **泛型数组的装箱语义缺失**：Java `int[]` 传给 `T[] where T extends Number` 需要先 autobox 成 `Integer[]`，再从 `Integer[]` 到 `Number[]`（协变）。codegen 当前没有处理 primitive 数组 → boxed object 数组的转换。  
**影响测试**：TestBoundedGenerics、TestGenericMethod 等所有泛型方法接受数组参数的测试  
**修复位置**：  
  - `codegen/instr/invoke.py` — invokestatic 参数传递时检测 primitive array → boxed array 的升级，插入 `.iter().map(|x| Integer::valueOf_i(*x).unwrap()).collect()` 等转换  
  - `codegen/types.py` — 泛型数组类型 `[TT;` descriptor 解析时，识别 bound 并生成正确的 Rust 类型  
**状态**：🔴 根因已确认，未修复

---

### H-2: TestRecord 输出格式错误

**错误**：期望 `Point[x=3, y=4]`，实际输出 `Object`（详见 G-1）  
**关联**：G-1  
**状态**：🔴 未修复

---

## 架构层面的根本性问题

以下问题超出局部 bug fix 范畴，需要设计层面解决。

### 核心发现：生成的 Rust 文件中已有完整字节码元数据，但均被忽略

**关键点 1 — `java_class` 宏完全忽略了类级元数据**

每个生成的 struct 上挂有完整的 Java 字节码元数据：

```rust
#[java_rta_macros::java_class(
    binary_name       = "java/util/ArrayList",
    super_class       = "java/util/AbstractList",
    interfaces        = "java/util/List;java/util/RandomAccess;java/lang/Cloneable;java/io/Serializable",
    is_interface      = false,
    generic_signature = "<E:Ljava/lang/Object;>Ljava/util/AbstractList<TE;>;...",
)]
pub struct ArrayList<E: Clone + Default + 'static> { ... }
```

当前宏实现（`java_rta_macros/src/lib.rs:15`）：
```rust
pub fn java_class(_attr: TokenStream, item: TokenStream) -> TokenStream {
//                ^^^^^ 完全丢弃，不读取任何属性参数
```
`binary_name`、`interfaces`、`super_class`、`is_interface`、`generic_signature` 全部被忽略。宏只生成通用的 `Into<Object>`、`From<Object>`、`Debug` 三个 impl，与类的实际 Java 类型信息完全脱节。

---

**关键点 2 — `java_method` / `java_field` 注解含完整字节码元数据，但 codegen 生成时不读取 `generic_signature`**

每个字段/方法注解携带完整的 Java 字节码元数据。以 `Reference<T>` 为例：

```rust
// 字段示例（Reference.referent）：
#[cfg_attr(any(), java_field(
    name              = "referent",
    descriptor        = "Ljava/lang/Object;",   // ← 类型擦除后：Object
    access            = "private",
    modifiers         = "",
    is_static         = false,
    generic_signature = "TT;",                  // ← 真实类型：泛型参数 T（不要删这行！）
))]
pub referent: JField<T>,

// 方法示例（Reference.<init>）：
#[cfg_attr(any(), java_method(
    name              = "<init>",
    descriptor        = "(Ljava/lang/Object;)V", // ← 参数类型擦除为 Object
    access            = "package",
    modifiers         = "",
    is_static         = false,
    is_native         = false,
    is_abstract       = false,
    is_synthetic      = false,
    generic_signature = "(TT;)V",               // ← 参数真实类型：泛型参数 T（不要删这行！）
))]
pub fn new_obj(mut referent: T) -> Result<Self> { ... }
```

这些字段均不应删除——它们是对应 Java `.class` 文件中 `MethodInfo`/`FieldInfo` + `Signature` 属性的完整镜像，是架构升级的信息来源。

**codegen 当前只读 `descriptor`，丢弃 `generic_signature`**。以 `SoftReference::new_obj` 调用 `Reference::new_obj(referent)` 为例：
- 读 `descriptor` → `expected = "Object"` → 错误插入 `Object::from_any(referent.clone())` → 生成 `Reference<Object>` → E0308
- 若读 `generic_signature` → `expected = RsGenericParam("T")` → 直接传入 `referent.clone()` → 正确

当前用 `_is_generic_type_param()` 字符串启发式猜回已丢弃的泛型信息（I-1）。根本解法是 codegen 在处理每条指令时优先使用 `generic_signature` 中的真实类型。

---

**这是所有 Arch 问题和 I 节补丁的共同根源**：本可以在宏层面（编译期）和 codegen 层面（读取 generic_signature）自动处理的行为，全都降级为 Python 侧脆弱的字符串启发式分析和补丁。

---

### 设计原则：Object 只在真正的多态边界出现，上层代码保持具体类型

Java 泛型类型擦除（type erasure）是编译器层面的实现细节，不代表运行时语义。在生成的 Rust 代码中，`Object` 不应该被滥用为"任意类型"的替代，应遵守以下分层规则：

```
┌─────────────────────────────────────────────────────────────┐
│              类型使用的三个层次                              │
├──────────────────┬──────────────────────────────────────────┤
│ 具体类型层        │ T、String、ArrayList<E>                   │
│                  │ → Rust 泛型参数 / 具体 struct              │
│                  │ 方法体内、字段访问、泛型方法参数            │
├──────────────────┼──────────────────────────────────────────┤
│ 多态边界层        │ 参数本身就是 Object（非泛型擦除产物）       │
│                  │ → Object = Rc<dyn JvmObject>              │
│                  │ println(Object)、equals(Object) 等         │
├──────────────────┼──────────────────────────────────────────┤
│ 异构容器层        │ List<Object>、Object[]                    │
│                  │ → Vec<Object>，存储不同运行时类型           │
└──────────────────┴──────────────────────────────────────────┘
```

**判断一个类型应该是 `T`（具体）还是 `Object`（多态）的规则**：

| 情况 | 正确 Rust 类型 | 依据 |
|------|---------------|------|
| `generic_signature` 中是 `TT;`、`TE;` 等 | 泛型参数 `T`、`E` | 擦除产物，上层保持具体 |
| `generic_signature` 中是 `Ljava/lang/Object;` | `Object` | 声明就是 Object，真正的多态引用 |
| `generic_signature` 不存在，`descriptor` 是 `Ljava/lang/Object;` | `Object` | 无泛型信息，确实是 Object |
| `generic_signature` 中是 `Ljava/util/List<TE;>;` | `List<E>` | 参数化类型，保持 |

**结论**：codegen 处理每条指令时，应**优先读取 `generic_signature`**，只有在无泛型签名、或签名确认是 `Object` 时，才生成 `Object` 类型。这是 Arch-7（RsType 化）的核心设计约束。

---

### 当前已有的基础设施（升级起点）

以下已有实现是架构升级的出发点，**不需要从零构建**：

**1. `Object` 已是 `Rc<dyn Any>`（`java_runtime/src/java/lang/object.rs:8`）**

```rust
pub struct Object(pub Rc<dyn std::any::Any>);
// from_any: Rc::new(v)，downcast: downcast_ref::<T>()
```

目标态升级路径：`Rc<dyn Any>` → `Rc<dyn ObjectVTable>`（内部派发 trait），保留 `as_any()` 以兼容 `downcast_ref`。

**关于 `ObjectVTable` 的命名说明**：这个 trait 是 `Object` 类型内部的实现细节，**不是**额外引入的 `JvmObject` 这类游离在 Java 命名空间之外的新名称。它的方法签名来源于 `java/lang/Object.class` 字节码中的 `hashCode`、`equals`、`toString`、`getClass` 等方法。在单 crate 最终态中，该 trait 由字节码翻译 `java/lang/object.rs` 时生成，不需要在 `java_runtime` 中单独定义。

**2. `JvmObjectBase` 已存在，但是 blanket no-op impl（`java_runtime/src/lib.rs:70`）**

```rust
pub trait JvmObjectBase {
    fn hashCode(&self) -> Result<i32> { Ok(0) }
    fn equals(&self, _other: Object) -> Result<bool> { Ok(false) }
    fn jvm_clone(&self) -> Result<Object> { panic!("stub") }
}
impl<T> JvmObjectBase for T {}  // 全部类型默认实现 = no-op
```

`JvmObjectBase` 是当前多 crate 架构下的过渡设施。目标态：**整体废弃，由字节码翻译 `java/lang/Object.class` 生成的 `ObjectVTable` 替代**，`java_class` 宏为每个类生成具体 impl，删除 blanket impl。

**3. 宏已生成 `Into<Object>` / `From<Object>`**

```rust
// 宏当前生成（java_rta_macros/src/lib.rs:46-53）：
impl Into<Object> for ArrayList<E> {
    fn into(self) -> Object { Object::from_any(self) }
}
impl From<Object> for ArrayList<E> {
    fn from(obj: Object) -> Self { obj.downcast::<Self>() }
}
```

升级后这两个 impl 继续有效（`from_any` 改为 `Object(Rc::new(val) as Rc<dyn ObjectVTable>)`），宏扩展时只需在已有代码基础上添加 `ObjectVTable` impl 生成，不需要改变 `Into`/`From` 逻辑。

**4. `java_method` 属性改为真正的 proc macro attribute（Arch-3 前置）**

当前 `cfg_attr(any(), java_method(...))` 在编译期永远不触发。要让宏读取方法级别元数据（如 `is_abstract` 计数判断函数式接口），需要：
1. 在 `java_rta_macros` 中注册 `#[proc_macro_attribute] pub fn java_method(...)`
2. Python codegen 将 `cfg_attr(any(), java_method(...))` 改为 `#[java_rta_macros::java_method(...)]`
3. 宏读取 `is_abstract`、`descriptor`、`generic_signature` 生成 trait 方法 shim

---

### 各 Arch 问题的正确解法：以宏为核心

---

### Arch-1: 接口无法携带数据

**问题**：`List<E>` 等接口生成为 `struct List<E>(PhantomData<E>)`，不携带任何具体实现类的数据。

**目标态方案：`Object` 改为 `Rc<dyn ObjectVTable>` + 接口改为 Rust trait**

`java_class` 宏读取 `is_interface = true` 后，为接口生成 Rust trait，而非 PhantomData struct：

```rust
// java_class 宏看到 is_interface = true，生成：
pub trait List_Trait {
    fn size(&self) -> Result<i32>;
    fn get(&self, index: i32) -> Result<Object>;
    fn add_obj(&self, e: Object) -> Result<bool>;
    // ...所有抽象方法由宏从 java_method 属性列表生成
}

// 接口引用类型（供变量声明使用）
pub type List = Object;  // 运行时统一为 Object，通过 ObjectVTable 动态派发
```

实现类通过宏自动实现 trait：
```rust
// java_class 宏看到 interfaces = "java/util/List;..." 后，为 ArrayList 生成：
impl List_Trait for ArrayList<Object> {
    fn size(&self) -> Result<i32> { ArrayList::size(self) }
    fn get(&self, index: i32) -> Result<Object> { ArrayList::get_obj(self, index) }
    // ...转发到已有方法
}
```

`Object` 类型内部结构（在单 crate 最终态中，由 `java/lang/object.rs` 字节码翻译定义）：
```rust
// 内部派发 trait：方法签名来源于 java.lang.Object 的字节码方法
// （不对外暴露为 JvmObject 或其他非 Java 命名空间的名称）
pub(crate) trait ObjectVTable: 'static {
    fn hashCode(&self)                  -> Result<i32>;
    fn equals(&self, obj: Object)       -> Result<bool>;
    fn toString(&self)                  -> Result<String>;
    fn getClass(&self)                  -> Result<Class<Object>>;
    fn as_any(&self)                    -> &dyn std::any::Any;
}

pub struct Object {
    inner: Rc<dyn ObjectVTable>,  // 真正的 dyn dispatch
}
```

**接口方法调用的派发机制（已确认）**：

接口变量在运行时就是 `Object`（与 JVM 运行时模型一致），`ObjectVTable` 只含 `java.lang.Object` 的方法（`hashCode`/`equals`/`toString`/`getClass`），接口自身方法（如 `List#get`）通过 downcast 到具体类型调用：

```rust
// list: Object，codegen 知道其声明类型是 List<String>
// list.get(0) 在 Java 层调用 List 接口方法，生成：
list.downcast::<ArrayList<String>>().get(0)?
// downcast 失败则运行时 panic（ClassCastException 语义）
```

`ObjectVTable` 不持有接口方法 vtable；接口 Rust trait（`List_Trait`、`Comparable_Trait` 等）仅作为静态约束供 `impl` 使用，不是运行时派发机制。此设计与 JVM 字节码的 `invokevirtual`/`invokeinterface` 分离语义一致。

**codegen 变更**：
1. `java_rta_macros/src/lib.rs` — `java_class` 宏：读取 `is_interface` / `interfaces` 参数，为接口生成 Rust trait；为实现类生成 `ObjectVTable` impl（替代 blanket no-op），方法名与 Java 方法名一一对应
2. `java_runtime/src/java/lang/object.rs` — `Object(Rc<dyn Any>)` 升级为 `Object(Rc<dyn ObjectVTable>)`；`ObjectVTable` 的方法签名来自 `java.lang.Object` 字节码（`hashCode`、`equals`、`toString`、`getClass`）
3. `java_runtime/src/lib.rs` — `JvmObjectBase` blanket impl 整体删除（过渡设施，最终态由字节码生成的 `ObjectVTable` 替代）
4. Python `class_writer.py` T55b 整块删除（宏通过 `interfaces` 字段自动生成正确 From impl）
5. Python `invoke.py` — 接口方法调用（`invokeinterface`）生成 `obj.downcast::<ConcreteType>().method()` 而非 `.into::<InterfaceType>().method()`

**影响**：修复所有集合类方法调用、修复 G-1 虚方法派发。

---

### Arch-2: `instanceof` 始终返回 `true`

**问题**：`instanceof` 生成为字面量 `true`，类型判断失效。

**目标态方案：宏从 `binary_name` + 继承链自动派生 `instanceof` 实现**

`java_class` 宏读取 `binary_name`、`super_class`、`interfaces`，自动为每个类生成 `ObjectVTable` impl，其中包含 `instanceof` 的静态判断：

```rust
// java_class 宏自动生成（无需 Python 侧任何改动）
// ObjectVTable 方法名与 java.lang.Object 的字节码方法一一对应
impl ObjectVTable for ArrayList<Object> {
    fn hashCode(&self) -> Result<i32>  { /* 翻译自字节码或默认实现 */ }
    fn equals(&self, obj: Object) -> Result<bool> { /* 翻译自字节码 */ }
    fn toString(&self) -> Result<String> { /* 翻译自字节码 */ }
    fn getClass(&self) -> Result<Class<Object>> {
        // 宏从 binary_name 静态生成，返回对应 Class 对象
        Ok(Class::for_name("java/util/ArrayList"))
    }
    fn as_any(&self) -> &dyn std::any::Any { self }
}

// instanceof / checkcast 使用独立的辅助方法（不放进 ObjectVTable）：
// 宏从 binary_name + super_class chain + interfaces 静态展开
impl ArrayList<Object> {
    pub fn is_instance_of(type_id: &str) -> bool {
        matches!(type_id,
            "java/util/ArrayList"
            | "java/util/AbstractList"
            | "java/util/AbstractCollection"
            | "java/lang/Object"
            | "java/util/List"        // 来自 interfaces = "..."
            | "java/util/RandomAccess"
            | "java/io/Serializable"
        )
    }
}
```

`instanceof` 指令生成：读取 `obj` 运行时类型后调用对应类的 `is_instance_of("java/lang/String")`，通过 `as_any().downcast_ref` 判断。  
`checkcast` 指令生成：断言类型后 downcast，失败则 `ClassCastException`。

**注意**：继承链需要传递闭合。方案：Python codegen 在 `java_class` 属性中写入完整传递闭合后的 `all_supertypes` 字段，宏直接展开为 `matches!` 模式，无需运行时递归调用 super。

**codegen 变更**：
1. `java_rta_macros` — 读取 `binary_name`、`interfaces`，生成 `ObjectVTable` impl + `is_instance_of` 关联函数
2. `java_runtime/src/java/lang/object.rs` — `Object` 改为 `Rc<dyn ObjectVTable>`，`instanceof` 检查通过 `as_any().downcast_ref` + `is_instance_of` 实现
3. Python `sim.py` — `instanceof` 生成对应类的 `is_instance_of("java/lang/String")` 调用
4. Python `class_writer.py` — `java_class` 属性中增加 `all_supertypes` 字段，记录传递闭合超类列表

**影响**：修复 TestCasting、TestPatternMatch 等。

---

### Arch-3: `invokedynamic` / Lambda 无法表达

**问题**：`invokedynamic` 不维护栈状态（见 D-1），lambda 语义完全丢失。

**目标态方案：函数式接口对应 `dyn Fn` trait，宏自动生成调用 shim**

`java_method` 属性中已有 `generic_signature` 包含函数式接口的完整类型。当接口满足 `@FunctionalInterface` 条件（恰好一个抽象方法），宏生成对应的 `Fn` trait 别名：

```rust
// java_class 宏看到 is_interface = true + 只有一个 is_abstract = true 的方法时：
// 为 Comparator 生成：
pub type Comparator_Fn = Arc<dyn Fn(Object, Object) -> Result<i32>>;

// invokedynamic 目标类型变为 Object，存储 Arc<dyn Fn>
impl ObjectVTable for Comparator_Fn { ... }
```

**短期方案（解除编译阻塞）**：
`sim.py` 的 `invokedynamic` 分支：
1. 从方法描述符解析参数数量 N
2. 从栈弹出 N 个操作数
3. 压入 `Object::default()` 占位值

这样后续 `checkcast`/`areturn` 拿到 `Object` 类型，不再产生 E0308。

**长期方案**：识别 `lambda$main$N` 合成方法，生成 `Arc::new(move |args| ...)` 闭包。

**codegen 变更**：
1. `sim.py` — `invokedynamic` 分支：解析操作数数量，弹出后压入占位值（短期）
2. `java_rta_macros` — 识别函数式接口，生成 `Fn` 类型别名（长期）

---

### Arch-4: 虚方法派发（`Object.toString()` 不调用具体类型）

**问题**：将具体类型转为 `Object` 后调用 `toString()`，不会派发到具体类型。

**目标态方案：`Object = Rc<dyn ObjectVTable>` 天然支持动态派发**

`Arch-1` 完成后（`Object` 改为 `Rc<dyn ObjectVTable>`），此问题自动解决。`ObjectVTable` 的方法名直接来自 `java.lang.Object` 的字节码方法：

```rust
// ObjectVTable 由 java/lang/object.rs 字节码翻译定义，方法名与 Java 一致
// java_class 宏自动为每个类生成 impl：
impl ObjectVTable for Point {
    fn toString(&self) -> Result<String> {
        // 转发到翻译生成的 toString() 方法（若存在），命名空间与 Java 一致
        Point::toString(self)
    }
    fn hashCode(&self) -> Result<i32> { Point::hashCode(self) }
    fn equals(&self, obj: Object) -> Result<bool> { Point::equals(self, obj) }
    fn getClass(&self) -> Result<Class<Object>> { Ok(Class::for_name("user/Point")) }
    fn as_any(&self) -> &dyn std::any::Any { self }
}
```

`Object::to_print_string()` 调用 `self.inner.toString()`，自动 dispatch 到 `Point::toString()`。命名无需前缀 `jvm_`，直接用 Java 方法名。

**codegen 变更**：Arch-1 的副产品，不需要额外修改。

---

---

### Arch-5: T55b 生成的 `From` impl 丢弃具体类型数据

**问题**：`class_writer.py` T55b 为每个接口生成：

```rust
impl From<ArrayList<E>> for List<E> {
    fn from(v: ArrayList<E>) -> List<E> { Default::default() }  // v 被丢弃！
}
```

**根因**：Python 侧 T55b 是应该由 `java_class` 宏完成的工作。宏已经有 `interfaces = "java/util/List;..."` 信息，完全可以自动生成正确的转型。  
当前 `Default::default()` 是因为不知道 `List<E>` 的内部结构（PhantomData struct），宏升级后（Arch-1 接口改为可携带数据的 trait/enum），From impl 自然正确。

**正确方案**：Arch-1 完成后，`java_class` 宏读取 `interfaces` 字段自动生成 From impl，Python T55b 整块删除。  
**状态**：🔴 运行时语义错误，Arch-1 完成后自动修复

---

### Arch-6: `_rust_type_to_binary` 短名逆查可用 `BINARY_NAME` 常量替代

**问题**：`_is_subtype`、`_find_field_super_prefix` 等函数通过 Rust 短类名（`"Comparator"`）逆查 JVM 二进制名（`"java/util/Comparator"`），反查可能匹配错误（同短名的不同包类）。

**根因**：`binary_name` 已经在 `java_class` 属性里，宏却忽略了它。若宏自动为每个类生成 `const BINARY_NAME: &'static str = "java/util/Comparator"`，则 Python 侧根本不需要做逆查，直接用这个常量即可。

**正确方案**：
1. `java_class` 宏读取 `binary_name` 参数，生成 `pub const BINARY_NAME: &'static str = "...";`
2. Python `codegen/instr/coerce.py` 的 `_rust_type_to_binary` 被 `codegen` 侧的 registry 直接查找（已有 `ci.name` 是 binary name）取代，不再做字符串模糊反查
3. `_is_subtype` 参数改为传 JVM 二进制名，不再依赖 Rust 短名转换

**状态**：🔴 潜在错误，Arch-2 重构时一并修复

---

### Arch-7: 类型系统 RsType 化（消除 I 节所有补丁）

**问题**：`jvm_to_rust()` 返回 `str`，类型信息以字符串形式在整个 codegen 中流转。泛型变量（`T`、`E`、`K`、`V`）、具体类型（`Object`、`String`）、参数化类型（`List<Object>`）无法区分，只能靠字符串启发式猜测（I-1）；null 表达式靠字符串内容集合判断（`_null_exprs`），极为脆弱。

**根本原因**：codegen 处理每条指令时，只读方法描述符（`descriptor` 字段，类型已擦除），不读 `generic_signature` 字段（含原始泛型信息）。

**目标态方案**：

1. **新增 `RsType` 枚举**（`codegen/types.py`）：
   ```python
   @dataclass
   class RsGenericParam:   name: str          # T、E、K、V
   @dataclass
   class RsConcrete:       name: str          # Object、String
   @dataclass
   class RsParameterized:  base: str; params: list[RsType]  # List<Object>
   @dataclass
   class RsArray:          elem: RsType       # Vec<T>
   @dataclass
   class RsNull:           pass               # null literal
   ```

2. **`jvm_to_rust` 改为接受可选 `generic_sig` 参数**，返回 `RsType`；优先解析 `generic_signature`（若存在），降级到 `descriptor`。

3. **Stack 传播 `RsType` 节点**：`StackSim.push()` 接受 `RsType` 而非字符串。

4. **统一 `coerce(from: RsType, to: RsType, expr: str) → str`** 替代所有分散的 elif 链。

5. **效果**：I-1（_is_generic_type_param）、I-3（三元 elif 链）、I-4（this.clone 特判）三处补丁完全删除；I-2（_is_direct_subtype）在 Arch-1 完成后删除。

**`sig_parser.py` 的归宿（确认废弃，不是改造）**：

`sig_parser.py` 是 codegen 从**已生成的 Rust 文件中反向读取**注解信息的工具，其存在本身就是架构问题的症状——codegen 生成 Rust 文件时本就持有完整类型信息，不应该再从生成物回读。

Arch-7 完成后：
- `jvm_to_rust()` 直接从 `.class` 字节码阶段的 `generic_signature` 字段获取类型信息，返回 `RsType` 节点
- 类型信息在 Python 内存中以 `RsType` 节点流转，不再经过"写入 Rust 文件 → 反向解析"的迂回路径
- `sig_parser.py` **整体删除**，不做改造

**修改文件**：`codegen/type_map.py`（内联 sig_parser + 新增 jvm_to_rs_type）、`codegen/instr/invoke.py`、`codegen/method/codegen.py`、`codegen/instr/sim.py`、`codegen/emitter/class_writer.py`；已删除 `codegen/sig_parser.py`  
**状态**：✅ 已实施（2026-09-16）— _is_generic_type_param 全部删除，sig_parser.py 已删除，jvm_to_rs_type 新增，_lookup_method_sig_params + _coerce_arg 统一化

---

### Arch-8: CFG 重建（消除 I-5，修复控制流正确性）

**问题**：字节码是扁平指令流 + `goto` 跳转。转译器将 `goto` 展平为直线代码，`while(true)` 的回边丢失，`break`/`continue` 无从表达，只能在函数末尾贴 `unreachable!()`（I-5）。更严重的是，`while(true)` 的循环体代码有时会被错误重复（goto 目标被内联多次），或完全消失（goto 超出翻译范围）。

**目标态方案**：

1. **构建 CFG**：每个方法字节码 → 基本块图，识别所有跳转边（条件跳转、goto、tableswitch）。
2. **支配树分析**：找到每个基本块的支配节点，识别自然循环（back edge = 从后继跳到支配祖先）。
3. **循环恢复**：每个自然循环生成 `loop { ... }` 结构，循环出口生成 `break`/`continue`。
4. **条件分支恢复**：每个支配路径对生成 `if { ... } else { ... }` 结构。
5. **效果**：函数末尾永远有完整返回，`unreachable!()` 补丁删除；`break`/`continue` 语义正确；goto 重叠引起的代码重复问题消失。

**修改文件**：新建 `codegen/cfg/` 模块（已有 `branches.py`、`loops.py` 雏形），`codegen/method/codegen.py` 主流程接入 CFG 输出  
**状态**：✅ 已实施（2026-09-16，Arch-8）

**实施摘要（2026-09-16）**：
- 新建 `codegen/cfg/basic_blocks.py`：实现 `BasicBlock` 数据类 + `build_basic_blocks(instrs)` + `function_always_returns(instrs)` 三个函数
- `function_always_returns` 通过 CFG 终止块分析判断函数是否在所有路径上都有 return/throw；发散循环（loop {} 无 break）无终止块，自动返回 True
- 更新 `codegen/cfg/__init__.py`：导出新符号
- 更新 `codegen/method/postprocess.py`：`_add_ok_return` 接受 `always_returns: bool` 参数，为 True 时不追加 `unreachable!()`
- 更新 `codegen/method/codegen.py`：调用 `function_always_returns(instrs)` 并将结果传入 `_add_ok_return`
- 效果：生成代码中 `unreachable!()` 完全消除（user/ 和 jdk_classes/ 均为 0 处），通过测试数不变（12/60，因其他架构问题仍阻塞）

---

### 架构变更优先级与依赖关系

```
D-1 短期（invokedynamic 弹出+占位）   → 立即解锁 7 个 lambda 测试编译
     │
     ▼
java_class 宏重写（核心前置工作）：
  ├── 读取 binary_name  → 生成 BINARY_NAME const
  ├── 读取 interfaces   → 自动生成 From impl（替代 T55b）
  ├── 读取 is_interface → 不同代码路径
  └── 生成 ObjectVTable impl（方法名与 java.lang.Object 字节码方法一致）
     │
     ├─→ Arch-2（instanceof 正确实现）    → 5+ 测试
     │
     ├─→ Arch-6（_rust_type_to_binary 消除）→ 稳定性
     │
     └─→ Object 改为 Rc<dyn ObjectVTable>（核心类型重构）
              │  ObjectVTable 由 java/lang/object.rs 字节码翻译定义
              │  命名空间与 Java 一致，不引入 JvmObject 等额外名称
              │
              ├─→ Arch-1（接口 trait + dyn dispatch）→ 10+ 测试
              ├─→ Arch-4（虚方法派发，Arch-1 副产品）→ 5+ 测试
              └─→ Arch-5（From impl 正确，Arch-1 副产品）→ 自动修复
```

| 顺序 | 工作项 | 影响 | 前置 |
|------|--------|------|------|
| 0a | **V5**（`codegen.py:498,531` 补 `_is_generic_type_param`） | 修复 if/else 分支泛型参数错误包装 | 无，2 行改动 |
| 0b | **C-1**（创建 `preconditions.rs` 空文件） | 解除若干测试的文件缺失编译阻塞 | 无，新建文件 |
| 0c | **I-6**（删除 `jvm_clone` 重命名）| 150+ 处 `jvm_clone` 恢复为 `clone`，消除命名原则违规 | 无，独立可修 |
| 1 | **D-1 短期**（invokedynamic 弹/压占位） | 消除 7 个测试的 E0308 | 无 |
| 2 | **Arch-7**（RsType 化 + generic_signature 读取，废弃 sig_parser.py） | 消除 I 节全部补丁，提升全局类型精度 | 无 |
| 3 | **Arch-8**（CFG 重建） ✅ | 消除 I-5 unreachable!()，修复控制流正确性 | 无 |
| 4 | **`java_class` 宏重写**（读取所有元数据） | 所有 Arch 的基础 | 无 |
| 5 | **Arch-2**（instanceof + BINARY_NAME） | 5+ 测试 + 消除 Arch-6 | 宏重写 |
| 6 | **Object 改 `Rc<dyn ObjectVTable>`** | 基础类型重构，ObjectVTable 由 java/lang/object.rs 字节码翻译定义 | 宏重写 |
| 7 | **Arch-1**（接口 → Rust trait + downcast dispatch） | 10+ 测试语义修复，废除 J-1/J-2/J-3 游离 trait | Object 重构 |
| 8 | **Arch-3 长期**（invokedynamic → 真实闭包） | Lambda 语义 | Arch-1 |

---

## J. 命名原则违规 — `java_runtime` 游离 trait

以下 trait 存在于 `output/java_runtime/src/lib.rs`，违反 CLAUDE.md「生成的 Rust 代码应与 Java 命名空间自然一致」原则。它们都是过渡态设施，最终态全部废除。

### J-1: `JvmObjectBase` — 无对应 Java 类的基础 trait（违反原则1 + 原则2）

**位置**：`output/java_runtime/src/lib.rs:70`  
**当前代码**：
```rust
pub trait JvmObjectBase {
    fn getClass(&self) -> Result<java::lang::Object> { ... }
    fn hashCode(&self) -> Result<i32> { Ok(0) }
    fn equals(&self, _other: java::lang::Object) -> Result<bool> { Ok(false) }
    fn jvm_clone(&self) -> Result<java::lang::Object> { panic!("stub") }  // ← jvm_ 前缀违反原则2
}
impl<T> JvmObjectBase for T {}  // blanket no-op impl
```
**违规**：
- 原则1：`JvmObjectBase` 不对应任何 Java 标准库类，是人造名称
- 原则2：方法名 `jvm_clone` 带 `jvm_` 前缀，Java 中方法名是 `clone`

**最终态废除路径**：由 `java/lang/object.rs` 字节码翻译生成 `ObjectVTable` trait（含 `hashCode`、`equals`、`toString`、`getClass`），`java_class` 宏为每个类生成具体 impl，blanket no-op impl 整体删除。其中 `jvm_clone` 方法名直接改回 `clone`（见 I-6），不等 Arch-1。  
**状态**：🔴 过渡设施，`jvm_clone` 命名部分独立可修（见 I-6），其余 Arch-1 完成时废除

---

### J-2: `JvmEnum` — `java.lang.Enum` 的错误 trait 化（违反原则1）

**位置**：`output/java_runtime/src/lib.rs:60`  
**当前代码**：
```rust
pub trait JvmEnum {
    fn ordinal(&self) -> Result<i32> { Ok(0) }
}
impl<T> JvmEnum for T {}  // blanket no-op impl
```
**违规**：原则1 — `JvmEnum` 不对应 Java 命名空间中的任何类型。`java.lang.Enum` 是具体类（class），不是接口（interface），不应在 `java_runtime` 中以 Rust trait 形式重新实现。  
**最终态废除路径**：`ordinal()` 方法由 `java/lang/enum_.rs` 字节码翻译生成为 inherent 方法（`impl Enum { fn ordinal() ... }`），blanket impl 删除。  
**状态**：🔴 过渡设施，Arch-1 完成时废除

---

### J-3: `Printable` — Java 中不存在的接口（违反原则1）

**位置**：`output/java_runtime/src/lib.rs:36`  
**当前代码**：
```rust
pub trait Printable {
    fn to_print_string(&self) -> String;
}
```
**用途**：用于 `println_v` 函数的 Rust-only 类型派发（调用 `obj.to_print_string()` 而非 `obj.toString()`）。  
**违规**：原则1 — Java 标准库中不存在 `Printable` 接口，这是纯 Rust 基础设施。  
**最终态废除路径**：Arch-1（`Object = Rc<dyn ObjectVTable>`）完成后，`println_v(Object)` 直接调用 `self.inner.toString()`，通过 `ObjectVTable` 动态派发到具体类型的 `toString()`，`Printable` trait 整体删除。  
**状态**：🔴 过渡设施，Arch-4（Arch-1 副产品）完成时废除

---

## K. 代码布局重构 — native_impls 共置与内部包边界截断

以下任务源自 CLAUDE.md 规则 3 的更新（2026-09-15）：手写代码与生成代码共置，内部包边界截断 BFS。这些任务是基础设施层面的重构，与具体测试失败无直接关联，但完成后将消除 E-1（重复定义）、`@field` 注入等一整类问题的根因。

---

### K-1: 消除 `@field` 注释注入机制

**当前做法**：`native_impls/` 中的文件用 `/// @field name: RustType` 注释向生成的 struct 注入字段；`_scan_native_impls()`（`method_gen.py:95-103`）解析这些注释，`class_writer.py:226` 将字段插入 struct。

**问题**：字段声明隐藏在注释里，Rust 编译器无法验证；Python 解析注释是脆弱的文本处理；struct 定义分裂在两处（codegen 生成 + 注释注入），不自然。

**目标态**：内部边界类（`jdk/internal/`、`sun/`）由手写文件完整定义 struct（含所有字段），codegen 不生成 struct，无需注入。

**修复位置**：
- `codegen/emitter/method_gen.py` — 删除 `@field` 解析逻辑（lines 95-103）及 `extra_fields` 返回值
- `codegen/emitter/class_writer.py` — 删除 `extra_fields` 参数及注入逻辑（lines 196-226）
- `codegen/emitter/project_writer.py` — 删除 `extra_fields_map` 传递
- `output/native_impls/jdk/internal/misc/internal_lock.rs` — 将 `/// @field _mutex: MutexHolder` 改为 struct 声明中的真实字段（配合 K-3 一并完成）

**状态**：🔴 未修复（依赖 K-3 先完成）

---

### K-2: 消除 `#[path = "..."] mod _impl;` 远程引用方式，改为共置文件

**当前做法**：codegen 生成的每个类文件末尾追加 `#[path = "../../../../../native_impls/..."] mod _impl;`，将 native_impl 以子模块方式远程引入。

**问题**：
- 路径字符串跨越 5 层 `../`，脆弱且难以维护
- native_impl 位于子模块 `_impl` 中，`impl super::Xxx` 访问父级 struct 需要额外路径
- 与 `@field` 机制的远程注入一起构成了隐式双向耦合

**目标态**：手写文件（`<classname>_impl.rs`）与生成文件（`<classname>.rs`）并列存放，由 `mod.rs` 用 `mod <classname>_impl;` 自然包含，`impl Xxx { ... }` 直接写，无需 `super::` 前缀。

**修复位置**：
- `codegen/emitter/class_writer.py` — 删除生成 `#[path = ...] mod _impl;` 的逻辑
- `codegen/emitter/project_writer.py` — 生成 `mod.rs` 时，若对应 `<classname>_impl.rs` 存在则加入 `mod <classname>_impl;`
- `output/native_impls/` 目录下所有文件 — 迁移到 `output/src/` 对应路径下（见 K-4）

**状态**：🔴 未修复（依赖 K-4 文件迁移）

---

### K-3: 内部边界类 BFS 截断规则实现

**当前做法**：BFS 调用链分析（`codegen/emitter/project_writer.py`）对所有包均递归展开，`jdk/internal/`、`sun/` 包内的类被当作普通类翻译字节码，导致级联引入大量内部类依赖（635 类 vs 截断后 111 类）。

**目标态**：BFS 在遇到 `jdk/internal/` 或 `sun/` 包时停止展开。该类标记为「内部边界类」：
- codegen 不生成 struct，只生成模块声明（`mod internal_lock;`），由手写文件提供完整实现
- 手写文件须已存在于 `output/src/<pkg>/` 对应路径，否则报错提示需要手写

**修复位置**：
- `codegen/emitter/project_writer.py` — BFS 入队时检查包前缀，`jdk/internal/` 和 `sun/` 开头的类不入队，改为标记为边界类并加入 `boundary_classes` 集合
- `codegen/emitter/class_writer.py` — 对 `boundary_classes` 中的类跳过 struct 生成，只输出 `pub mod <classname>;`（由手写文件提供实现）
- 手写文件初始版本（各类按调用链按需补全）：
  - `output/src/jdk/internal/misc/internal_lock.rs` — 已有 native_impl，迁移后补全 struct 定义（K-4）
  - `output/src/jdk/internal/util/preconditions.rs` — 新建（537 次引用，0 个 native，15 个可翻译方法）

**优先级说明**：先实现叶子层（无内部依赖）边界类，再实现有依赖层。按 `impl-strategy.md` 阶段规划推进。

**状态**：🔴 未修复（高优先级，影响翻译规模和稳定性）

---

### K-4: `native_impls/` 目录迁移至 `output/src/` 共置

**当前做法**：手写文件在 `output/native_impls/` 独立目录，按 Java 包层次组织但与生成代码分离。

**目标态**：手写文件迁移到 `output/src/` 下对应位置，与生成文件并列：

| 当前路径 | 目标路径 | 类型 |
|---------|---------|------|
| `output/native_impls/java/lang/string.rs` | `output/src/java/lang/string_impl.rs` | public API native |
| `output/native_impls/java/lang/system.rs` | `output/src/java/lang/system_impl.rs` | public API native |
| `output/native_impls/java/util/array_list.rs` | `output/src/java/util/array_list_impl.rs` | public API native |
| `output/native_impls/jdk/internal/misc/internal_lock.rs` | `output/src/jdk/internal/misc/internal_lock.rs` | 内部边界类（完整文件） |

迁移步骤：
1. 新建目标路径文件，将 `impl super::Xxx` 改为 `impl Xxx`（无需 `super::`，在同一模块内）
2. `mod.rs` 加入 `mod <classname>_impl;`（public API 类）或 `pub mod internal_lock;`（边界类）
3. 删除原 `native_impls/` 目录（配合 K-2 一并完成）
4. 内部边界类文件补全 struct 定义（消除 K-1 中 `@field` 的需求）

**状态**：🔴 未修复（是 K-1、K-2、K-3 的执行层，最后统一完成）

---

## 已修复问题汇总（本 session）

| 修复日期 | 问题 | 修复方式 |
|---------|------|---------|
| 2026-09-15 | `anewarray` 类型推断错误（使用 short_cls 而非完整路径） | `sim.py` anewarray 改用 full class path |
| 2026-09-15 | `aastore` 元素类型提取失败（未考虑 `Rc<RefCell<Vec<T>>>` 包裹） | `sim.py` aastore 先剥离 Rc/RefCell |
| 2026-09-15 | 字符串字面量尾部空格被 strip | `sim.py` ldc String 改 `.rstrip('\n')` |
| 2026-09-15 | `InternalLock` PartialEq 推导失败 | 引入 `MutexHolder` 包装类型 |
| 2026-09-15 | default 方法命名冲突（E0592 reversed） | `class_writer` 预扫描 `used_rust_names` |
| 2026-09-15 | `Arrays_ArrayList → List` From impl 缺失 | `class_writer` T55b 扩展祖先类接口收集 |
| 2026-09-15 | `areturn` 只检查直接子类，传递子类不生成 `.into()` | `sim.py` areturn 改用 `_is_subtype`（仅对非 invokedynamic 场景有效，见 D-1） |
| 2026-09-15 | `native_impl string.rs` 与字节码翻译重复定义（E0592） | 删除 native_impl 中的重复方法 |
| 2026-09-15 | `array_list.rs iterator()` 返回 `Iterator<Object>` 找不到类型 | 改回 `Result<Object>` |
| 2026-09-15 | 生成结构体缺少 `PartialEq`，`JField<T>` 缺少 `PartialEq` impl → E0369 | `class_writer.py` 所有 derive 加 PartialEq；`types.rs` 为 JField<T> 实现 PartialEq |
| 2026-09-15 | `while(true)` 展平后函数末尾 E0317 | `postprocess.py` 尾部加 `unreachable!()`（临时补丁 I-5） |
| 2026-09-15 | 三元/merge 表达式 `Object as Struct` 非法 as 转换 → E0605 | `codegen.py` 改为 `Default::default()` / `.downcast::<T>()` / `Object::from_any(x)` |
| 2026-09-15 | `this` 在三元/merge 值位置报 `mismatched types &Self vs Self` | `codegen.py` 三元和 merge 变量处替换 `this` → `this.clone()`（临时补丁 I-4） |
| 2026-09-15 | `putfield` 泛型类字段赋值时，泛型参数 `T` 被包装为 `Object::from_any(T)` → E0308 | `sim.py` putfield 加 `_is_generic_type_param` 跳过包装（临时补丁 I-1） |
| 2026-09-15 | T55 传递子类型生成 `.into()` 但无对应 From impl → E0277 | `coerce.py` 新增 `_is_direct_subtype`，sim.py/invoke.py 换用（临时补丁 I-2） |
| 2026-09-15 | `invokespecial/invokestatic/invokevirtual` 泛型参数被 `Object::from_any` 包装 → E0308 | `invoke.py` 三处调用加 `_is_generic_type_param` 跳过包装（临时补丁 I-1） |
| 2026-09-15 | I-6 `jvm_clone` 命名原则违规：`clone` → `jvm_clone` 重命名 | 删除 4 处 rename 逻辑，所有 Rust-level clone 改用 `Clone::clone(...)` 完全限定语法 |
| 2026-09-15 | `invokevirtual Object.clone` 在非 Object 接收者（如 byte[]）上加 `?` → E0277 | `invoke.py:397` 特判 `clone` + 非 Object 类型，发射 `Object::from_any(obj.clone())` |
| 2026-09-15 | 构造器内 `Clone::clone(this)` 中 `this` 是 owned，需要 `&this` | `sim.py` putfield 区分 `val_str == 'this'` 时发射 `Clone::clone(&this)` |
| 2026-09-15 | A-1：`u16`/`i8`/`i16` 作为 `ireturn` 值时缺少 `as i32` 转换 | `coerce.py _coerce_value`：`target == 'i32'` 分支补充 `u16/i8/i16` → `as i32` |
| 2026-09-15 | `aaload` primitive 数组元素用 `Clone::clone` 但 `Vec<i32>[idx]` 是 `i32` 非引用 → E0308 | `sim.py` aaload：primitive 类型直接取值；非 primitive 改为 `Clone::clone(&arr.borrow()[idx])` |
| 2026-09-15 | F-1：`aastore` 同一 RefCell 同时 `borrow_mut` 和 `borrow` → 运行时 panic | `sim.py` aastore：非 primitive val 含 `.borrow()` 时先提取 tmp，再 borrow_mut 赋值 |
| 2026-09-15 | F-2：`while (cond1 && cond2)` 循环第二条件 if-guard 缺少 `break;` → 死循环 | `codegen.py` if-guard 块后，若 continue_idx 超过最内层循环末，补充 `break;` |
| 2026-09-15 | B-2：T55 `.into()` 对 `this`（`&Self`）生成 `From<&T>` 但只有 `From<T>` → E0277 | `invoke.py` T55 `.into()` 分支改为 `Clone::clone(&e).into()` |
| 2026-09-15 | C-1 附加：`#[path = "..."] mod _impl;` 目标文件不存在 → E0583 | `class_writer.py` 写 `#[path]` 前 `os.path.exists` 检查，不存在则省略 |
| 2026-09-15 | D-1：invokedynamic 不弹栈不压返回值 → 后续指令类型错误 | `sim.py` 已实现：解析 descriptor 弹出 N 参数，压入 `Object::default()` |
| 2026-09-16 | I-1（Arch-7）：`_is_generic_type_param` 启发式删除，`sig_parser.py` 删除 | `type_map.py` 内联 sig_parser 功能 + 新增 `jvm_to_rs_type`；`invoke.py` 用 `_lookup_method_sig_params` 读 registry 中方法 generic_signature；`codegen.py` 用 `ty_str in _class_tparams` 替代启发式 |
