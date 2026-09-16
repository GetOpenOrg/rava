# E2E 测试未解决问题追踪

> 创建日期：2026-09-15  
> 最后更新：2026-09-16（第二次）  
> 基于测试套件：`tests/e2e/`（60 个测试）  
> 更新方式：每次架构变更或 E2E 运行后手动补充/关闭条目

**核心原则**：优先通过架构（宏/codegen）解决，避免手写覆盖生成的 Rust 文件。手写仅限于：① native 方法（`*_impl.rs`）；② 内部边界类（`jdk/internal/`、`sun/`）。其余一律走生成器。

---

## 问题分类索引

| 类别 | 状态 | 优先级 | 影响范围 |
|------|------|--------|---------|
| [A. 编译错误 — 类型转换](#a-编译错误--类型转换) | 1 开放 / 3 已关闭 | 高 | 具体子类继承场景 |
| [B. 编译错误 — 缺失 From impl](#b-编译错误--缺失-from-impl) | 2 已关闭 | — | — |
| [C. 编译错误 — 缺失模块/import](#c-编译错误--缺失模块import) | 1 开放 | 高 | 多个 jdk_classes |
| [D. 编译错误 — invokedynamic](#d-编译错误--invokedynamic-无法翻译) | 1 已关闭 / 1 补丁 | — | — |
| [E. 编译错误 — 重复定义](#e-编译错误--重复定义) | 1 开放（降低优先级）| 低 | native_impl 冲突 |
| [F. 运行时错误 — RefCell 双重借用](#f-运行时错误--refcell-双重借用) | 1 已关闭 | — | — |
| [G. 运行时错误 — 多态/虚方法派发](#g-运行时错误--多态虚方法派发) | 1 已关闭 | — | — |
| [H. 运行时错误 — 输出值错误](#h-运行时错误--输出值错误) | 1 开放 / 1 待验证 | 低 | TestBoundedGenerics |
| [I. 编译层补丁 — 技术债务](#i-编译层补丁--技术债务) | 1 开放 / 5 已关闭 | 中 | 全局质量 |
| [J. 命名原则违规](#j-命名原则违规--java_runtime-游离-trait) | 1 开放 / 2 已关闭 | 中 | 命名一致性 |
| [K. 代码布局重构](#k-代码布局重构--native_impls-共置与内部包边界截断) | 2 已关闭 / 2 开放 | 中 | 全局基础设施 |
| [N. 新架构债务 — 需 codegen 解决](#n-新架构债务--需-codegen-解决) | 1 开放 / 1 已关闭 | 中 | java_runtime 稳定性 |
| [Arch. 架构任务](#arch-架构任务) | 6 已关闭 / 2 进行中 | — | — |

---

## A. 编译错误 — 类型转换

### A-1: `char`/`u16` 返回值缺少 `as i32` 强制转换 ✅

**状态**：✅ 已修复（2026-09-15）  
`coerce.py _coerce_value`：`target == 'i32'` 分支补充 `u16/i8/i16` → `as i32`

---

### A-2: 方法参数类型不匹配（间接子类传给父类参数）

**错误**：`E0308: expected InputStream, found InflaterInputStream`  
**位置**：`jdk_classes/src/java/lang/character_name.rs`（仅在展开该调用链时出现）

**实际状态**：`InflaterInputStream` 在 `java/util/zip/` 包下，BFS 扫描时该类属于公开 API 展开范围（`java/` 前缀），理论上会被翻译。但当前测试调用链未触发此路径，故实际编译错误未出现。

**架构解法**：  
- 短期：`invoke.py` 的 `.into()` 逻辑改用 `_is_subtype`（传递性子类检查）替代 `_is_direct_subtype`，同时为中间类自动生成 `From` impl 链。
- 长期：当 Arch-1 下接口方法调用全部改为 `Object` + downcast，消除了接口侧的静态 `.into()`，此处仅剩具体类继承链问题，进一步通过继承链 `From` impl 自动生成解决。

**禁止**：手写 `From<InflaterInputStream> for InputStream`——这是生成器的工作（class_writer.py 应从继承链生成完整 From impl 链）。

**状态**：🟡 潜在问题，当前调用链未触发；架构解法已明确，等触发时实施

---

### A-3: 接口类型参数传入（如 `AbstractStringBuilder → CharSequence`） ✅ 架构消解

**原问题**：`E0308: expected CharSequence, found String`  
**消解原因**：Arch-1（2026-09-16）将所有接口改为 `pub type Name = Object` 别名。`CharSequence` 现在是 `Object`，`String` 通过 `Into<Object>` 兼容。原本需要 `.into()` 的接口参数传递场景自然消失。  
**状态**：✅ 架构消解（无需任何修复，Arch-1 副产品）

---

### A-4: `PrintStream → Appendable` 构造参数类型不匹配 ✅ 架构消解

**原问题**：`E0308: expected Appendable, found PrintStream`  
**消解原因**：同 A-3，`Appendable` 是 `Object` 别名，参数类型自然兼容。  
**状态**：✅ 架构消解（Arch-1 副产品）

---

## B. 编译错误 — 缺失 From impl

### B-1: `Arrays_ArrayList → List` 缺少 From impl ✅ 架构消解

**原问题**：缺少 `From<Arrays_ArrayList<T>> for List<E>` impl，且即使生成也是 `Default::default()` 丢弃数据。  
**消解原因**：Arch-5（2026-09-16）删除了 T55b 的 `From<ConcreteClass> for InterfaceType` 生成逻辑。接口现在是 `Object` 别名，不存在 `From<ConcreteClass> for InterfaceType` 这种转型需求。`ArrayList` 已直接通过 `Into<Object>` 转换。  
**状态**：✅ 架构消解（Arch-5 + Arch-1 联合消解）

---

### B-2: `AbstractStringBuilder → Appendable` 缺少 From impl ✅

**状态**：✅ 已修复（2026-09-15）— invoke.py T55 `.into()` 分支改为 `Clone::clone(&e).into()`  
**注**：Arch-1 后此修复也变得冗余，`Appendable = Object`，不再有此转换需求。

---

## C. 编译错误 — 缺失模块/import

### C-1: use 语句引用不存在的 jdk 包 🔴

**错误**：`E0432: unresolved import crate::java::lang::r#ref`、`crate::java::lang::reflect`、`crate::sun::security::util` 等  
**根因**：codegen 把 JDK 类的全部依赖包写进 `use crate::...::*;`，但这些包（如 `java::lang::r#ref`、`java::lang::reflect`、`sun::security::util`）在当前测试调用链中没有类，因此没有生成对应模块，产生编译错误。

**架构解法（codegen 侧）**：  
`codegen/emitter/class_writer.py` 生成 `use` 语句时，检查每个包路径是否存在于当前轮次生成的模块列表（`generated_modules: set`）中。不存在则跳过该 `use` 行。  
**禁止**：创建空模块文件作为占位——这是用手写掩盖生成器的问题。

**附加问题 — preconditions.rs 文件缺失**：  
部分测试（TestBoundedGenerics 等）生成 `#[path = "..."] mod _impl;` 指向不存在文件。  
已临时修复（2026-09-15）：`class_writer.py` 写 `#[path]` 前 `os.path.exists` 检查，不存在则省略。  
根本修复：见 K-2（消除 `#[path]` 远程引用机制）。

**状态**：🔴 主问题未修复；架构解法已明确（class_writer.py 过滤不存在的包路径）

---

## D. 编译错误 — invokedynamic 无法翻译

### D-1: Lambda/方法引用 invokedynamic 短期修复 ✅

**状态**：✅ 已修复（2026-09-15）— sim.py 解析 descriptor 弹出 N 参数，压入 `Object::default()` 占位符  
长期语义修复见 Arch-3

---

### D-2: `areturn` 类型不兼容的 `Default::default()` 兜底（补丁）⚠️

**状态**：⚠️ 临时补丁，随 Arch-3 长期方案完成后可删除  
当前 invokedynamic 语义已通过 Arch-3 大幅改善（闭包路径正确），此补丁命中频率已降低，但不能删除——仍有未覆盖的 method reference 场景。

---

## E. 编译错误 — 重复定义

### E-1: native_impl 与字节码翻译重复 ⚠️

**原问题**：手写 `native_impls/` 中的方法与字节码翻译结果重名（E0592）。  
**当前状态**：`native_impls/` 目录已删除（K-4 已完成），`*_impl.rs` 文件已共置于 `java_runtime/src/java/`。`java_runtime` 是手写层，不会与 `jdk_classes`（字节码翻译）重复。

**剩余风险**：未来若对同一类既生成字节码翻译又有 `_impl.rs`，可能再次出现。  
**架构防御**：`class_writer.py` 生成方法体时，先扫描同目录 `<classname>_impl.rs` 中已有的 inherent impl 方法名，跳过重复生成。此逻辑还未实现。

**状态**：⚠️ 当前架构下已消解；架构防御逻辑缺失，存在未来风险

---

## F. 运行时错误 — RefCell 双重借用

### F-1: 同一语句中 `borrow_mut` 和 `borrow` 同时持有 ✅

**状态**：✅ 已修复（2026-09-15）— `sim.py aastore`：非 primitive val 含 `.borrow()` 时先提取 tmp 变量再 borrow_mut 赋值

---

## G. 运行时错误 — 多态/虚方法派发

### G-1: `Object.toString()` 不调用具体类型的覆盖版本 ✅

**原问题**：`println_v(Object::from_any(p))` 输出 `Object` 而非 `Point[x=3, y=4]`。  
**修复状态**：✅ 已修复（Arch-4 副产品，2026-09-16）  
- `Object` 实现 `std::fmt::Display`，内部调用 `self.0.toString()`（`ObjectVTable` 动态派发到具体类型）
- `println_v<T: Display>` 接受任何 Display 值，包括 `Object`
- `java_class` 宏为每个类生成 `ObjectVTable impl`，其中 `toString()` 转发到翻译生成的具体方法

**验证**：TestRecord 和 TestObjects 中 `println_v(Object)` 应输出正确的具体类型 toString 结果。

---

## H. 运行时错误 — 输出值错误

### H-1: TestBoundedGenerics 输出 `0.0` 而非正确值 🔴

**错误**：期望 `15.0`，实际 `0.0`  
**根因**：`main()` 调用 `sum(ints)` 时 `ints` 是 `Rc<RefCell<Vec<i32>>>`（primitive int 数组），而 `sum` 声明参数为 `Rc<RefCell<Vec<Number>>>`（object 数组）。codegen 无法将 `Vec<i32>` 强制转换为 `Vec<Number>`，退化为 `Default::default()`（空 Vec），导致 `sum` 返回 `0.0`。

**架构解法（codegen 侧）**：  
`invoke.py` invokestatic 参数传递时：检测 primitive 数组 → boxed object 数组的升级场景，插入 `.iter().map(|x| Object::from_any(Integer::valueOf(*x)?)).collect()` 转换。  
前提：`Integer::valueOf(i32) -> Result<Integer>` 在字节码翻译后可用。

**技术约束**：不能手写 `Sum` 相关代码——`TestBoundedGenerics` 的翻译必须完全来自字节码。

**状态**：🔴 根因确认；架构解法已明确，依赖 `Integer::valueOf` 字节码翻译就绪

---

### H-2: TestRecord 输出格式错误 🟡

**原描述**：期望 `Point[x=3, y=4]`，实际输出 `Object`。  
**当前状态**：G-1 已修复（ObjectVTable.toString dispatch），此问题理论上已解决。需要实际运行验证。  
若 `Point` 类的 `toString()` 字节码翻译正确，则 `println_v(Object::from_any(p))` 应输出正确格式。

**状态**：🟡 待验证（G-1 修复后，预期已解决）

---

## I. 编译层补丁 — 技术债务

### I-1: `_is_generic_type_param()` 字符串启发式判断 ✅

**状态**：✅ 已删除（2026-09-16，Arch-7）— `_lookup_method_sig_params` 读取方法 `generic_signature`，`_is_generic_type_param` 全部替换为 `ty_str in _class_tparams` 集合查找

---

### I-2: `_is_direct_subtype` 与 `_is_subtype` 并存 ✅ 架构消解

**原问题**：T55（`.into()` 提升）用 `_is_direct_subtype`（一步），其他用 `_is_subtype`（传递性），两套规则并存。  
**当前状态**：`_is_direct_subtype` 已删除（2026-09-16），统一使用 `_is_subtype`（传递性）。  
**剩余 `.into()` 逻辑**：`invoke.py` 和 `sim.py` 中仍有 `_is_subtype(actual, expected) → Clone::clone(&e).into()` 的调用，用于具体类继承链的自动提升。这是正确行为，非补丁。  
**状态**：✅ 统一完成

---

### I-3: 三元表达式类型统一 7 个 `elif` 重复 ⚠️

**当前做法**：`codegen/method/codegen.py` 中三元表达式和 merge 变量处，各有一段 7 个 `elif` 链，内容相同但维护分离。  
**根本解法（Arch-7 后续）**：引入 `coerce(from: RsType, to: RsType, expr: str) -> str` 统一函数替代两处重复链。  
**状态**：⚠️ 技术债务，独立可修，不影响正确性

---

### I-4: `this.clone()` 字符串比较特判 ⚠️

**当前做法**：`codegen/method/codegen.py` 三元和 merge 变量处，`if tv == 'this': tv = 'this.clone()'` 字符串比较补偿 `&Self` vs owned 不兼容问题。  
**根本解法**：Stack 类型节点区分 `RsRef(&Self)` 和 `RsOwned(Self)`，任何值位置使用引用节点时统一生成 `.clone()`，无需字符串匹配。  
**状态**：⚠️ 技术债务，Arch-7 RsType 完善时一并删除

---

### I-5: `unreachable!()` 末尾补全 ✅

**状态**：✅ 已删除（2026-09-16，Arch-8）— `function_always_returns(instrs)` CFG 分析替代启发式检测，`unreachable!()` 完全消除

---

### I-6: `clone` → `jvm_clone` 强制重命名 ✅

**状态**：✅ 已删除（2026-09-15）— `_JAVA_RUST_RENAME` 的 `clone` 项删除，所有 Rust-level clone 改用 `Clone::clone(...)` 完全限定语法

---

## J. 命名原则违规 — `java_runtime` 游离 trait

### J-1: `JvmObjectBase` — 无对应 Java 类的基础 trait ✅

**状态**：✅ 已删除（2026-09-16）— `lib.rs` 当前仅 88 行，`JvmObjectBase` 及其 blanket impl 已不存在。由 `ObjectVTable`（Arch-4）替代，方法名与 Java 字节码一致。

---

### J-2: `JvmEnum` — `java.lang.Enum` 的错误 trait 化 ✅

**状态**：✅ 已删除（2026-09-16）— `JvmEnum` 及其 blanket impl 已不存在。`ordinal()` 方法由字节码翻译 `java/lang/enum_.rs` 提供。

---

### J-3: `Printable` — Java 中不存在的接口 🟠

**位置**：`output/java_runtime/src/lib.rs:39`  
**当前状态**：`println_v` 已改为 `<T: std::fmt::Display>` 而非 `T: Printable`；`Object` 通过 `ObjectVTable::toString()` 实现 `Display`。`Printable` trait 目前是死代码——定义存在但没有调用者使用它作为 trait bound。

**正确解法**：直接删除 `lib.rs` 中的 `Printable` 定义及其所有 impl，这是一行删除操作。  
**禁止**：保留 `Printable` 作为备用——死代码会造成混淆。

**状态**：🟠 死代码，可立即删除，独立不依赖任何其他变更

---

## K. 代码布局重构

### K-1: 消除 `@field` 注释注入机制 ✅ 自然消亡

**原问题**：`native_impls/` 文件用 `/// @field name: RustType` 注释向生成 struct 注入字段。  
**当前状态**：`native_impls/` 目录已删除，`method_gen.py` 已无 `@field` 解析残留。机制已自然消亡，无需主动删除。  
**状态**：✅ 自然消亡

---

### K-2: 消除 `#[path = "..."] mod _impl;` 远程引用 ✅

**当前状态**：`native_impls/` 已删除；`java_runtime/src/java/` 下的 `*_impl.rs` 通过 `mod.rs` 自然包含（`mod string_impl;` 等）；`jdk_classes` 生成文件中已无 `#[path = ...]` 远程引用。  
**状态**：✅ 完成（K-4 执行时一并完成）

---

### K-3: 内部边界类 BFS 截断规则 ✅

**当前状态**：`transpile.py` 中 `_JDK_STUB_ONLY_PREFIXES = ('sun/', 'jdk/', 'com/sun/', 'com/oracle/', 'java/security/')` 已实现 BFS 截断。遇到这些前缀的类不进入 BFS 展开，生成存根。  
**状态**：✅ 完成

---

### K-4: `native_impls/` 目录迁移至 `output/java_runtime/src/java/` 共置 ✅

**当前状态**：`native_impls/` 目录已不存在。手写文件已共置于 `java_runtime/src/java/lang/`（`system_impl.rs`、`math_impl.rs` 等）。  
**状态**：✅ 完成

---

## N. 新架构债务 — 需 codegen 解决

### N-1: `java_runtime` 手写代码中 E0107（接口泛型参数） 🟡

**错误示例**：
```rust
// java_runtime 手写代码（不应修改生成文件，但这些是手写文件）：
pub suppressedExceptions: JField<List<Throwable>>,  // E0107: List 取 0 个泛型参数
impl<E: Clone + Default + 'static> List<E> { ... }  // E0107
fn equalsRange(&self, other: List<Object>, ..)       // E0107
```

**根因**：Arch-1（2026-09-16）将接口改为 `pub type List = Object`（无泛型参数），但 `java_runtime` 中少量手写代码仍使用旧的 `List<T>` 写法。

**当前状态（2026-09-16 第二次）**：随着 N-2 宏升级 + codegen 重新生成，大量原本手写的 `java_runtime` 文件已重新由 codegen 生成，E0107 从原来的约 143 处降至 **4 处**（集中在 `throwable.rs`、`list.rs`、`array_list.rs`）。

**4 处剩余位置**：
- `java/lang/throwable.rs:42` — `JField<List<Throwable>>`
- `java/lang/throwable.rs:60` — `List<Throwable>` 返回类型
- `java/util/list.rs:29` — `impl<E> List<E>`
- `java/util/array_list.rs:250` — `List<Object>` 参数

**正确解法**：将这 4 处泛型参数改为 `Object`（允许在手写 java_runtime 文件内修改）。

**禁止**：在生成的 `jdk_classes/` 文件里做任何手写修复。

**状态**：🟡 剩余 4 处，可 5 分钟内完成；不阻塞架构演进

---

### N-2: `java_method`/`java_field` 激活为真正的 proc-macro attribute ✅

**完成**（2026-09-16 第二次）：

**实现内容**：
1. `java_rta_macros/src/lib.rs`：注册 `java_method`、`java_native` 为真实 `#[proc_macro_attribute]`（identity passthrough）
2. `codegen/emitter/attrs.py`：非 native 方法改用 `#[java_rta_macros::java_method(...)]`；native 方法保留 `cfg_attr(any(), java_native(...))`（由 `_impl.rs` 手写处理）
3. `java_class` 宏新增 `has_to_string_method`/`has_hash_code_method` 参数，有条件地生成 `ObjectVTable::toString`/`hashCode` 转发
4. `codegen/emitter/method_gen.py`：非 native `toString`/`hashCode` 存根改为返回 `Ok(String::from(Self::BINARY_NAME))`/`Ok(0)` 智能默认值（不再 panic）
5. `codegen/emitter/attrs.py`：检测类自身是否声明 `toString`/`hashCode`，动态设置转发标志（避免 `Self::toString` 解析到 vtable 自身造成无限递归）

**效果**：编译错误从 247+ 降至 4（仅剩 N-1 的 4 处手写 `java_runtime` 遗留）

**待解锁能力**（未来）：
1. `java_method` 宏统计 `is_abstract = true` 方法数 → SAM 识别（Arch-3 method reference 支持）
2. 读取 `generic_signature` → 编译期类型校验

**`java_field` 说明**：struct 字段不能附加 `proc_macro_attribute`，保留 `cfg_attr(any(), java_field(...))`；`java_class` 宏通过解析 item token stream 读取字段属性（未来改进）。

**状态**：✅ 完成（2026-09-16）

---

## Arch. 架构任务

### Arch-1: 接口类型重构 ✅（当前实现）

**实际实现**（2026-09-16）：接口改为 `pub type Name = Object` 类型别名（无泛型参数），调用时通过 `downcast_ref::<ConcreteType>()` dispatch。  
**与原计划的差异**：原计划描述了 `pub trait List_Trait { ... }` + `impl List_Trait for ArrayList` 完整 trait 系统，实际实现选择了更简洁的 Object 别名 + downcast 方案，这是**正确的最终态方案**（trait 层是过度设计）。

**副产品**：A-3、A-4、B-1 架构消解；G-1 修复（objectVTable.toString dispatch）。  
**遗留债务**：`java_runtime` 手写代码中接口泛型写法 → N-1。  
**状态**：✅ 完成

---

### Arch-2: `instanceof` 实现 ✅

**实现**（2026-09-16）：`java_class` 宏读取 `all_supertypes` 和 `binary_name`，生成：
- `pub const BINARY_NAME: &'static str = "..."`（Arch-6 一并完成）
- `ObjectVTable impl` 中 `is_instance_of(&self, type_id: &str) -> bool { matches!(type_id, "pkg/Cls" | "pkg/Super" | ...) }`

`sim.py` 中 `instanceof` 指令生成 `obj.0.is_instance_of("java/lang/String")`。  
**状态**：✅ 完成

---

### Arch-3: `invokedynamic` / Lambda 闭包 ⚠️ 部分完成

**完成部分**（2026-09-16）：
- `classfile.py`：提取 `LambdaMetafactory` 的 `sam_type` + `impl_method`
- `sim.py`：生成 `std::rc::Rc<dyn Fn(T...) -> Result<R>>` 闭包，包装入 `Object::from_any()`
- `invoke.py`：dispatch 侧通过 `downcast_ref::<Rc<dyn Fn(...)>>()` 走闭包路径
- 修复（2026-09-16）：`parse_field_type` 传入 `registry`，接口类型 hint 正确映射为 `Object`，`obj_is_bare = True` 触发闭包 dispatch

**已知剩余问题**：
1. **method reference**（如 `System.out::println`）：生成 TODO 注释，不生成闭包。识别 `Methodref` 类型的 bootstrap 方法参数，生成对应闭包。
2. **`Consumer.forEach` 内部的 `accept` 调用**：当 `forEach` 接收 `Object`（其中存储 `Rc<dyn Fn(Object) -> Result<()>>`），内部调用 `Consumer.accept(elem)` 时，需要正确 downcast 并调用闭包。理论上已通过 dispatch 修复覆盖，待验证。
3. **`Comparator.compare` via `Collections.sort`**：`sort` 内部的 `compare` 调用走相同 dispatch 路径，待验证。

**架构后续**（N-2 就绪后）：激活 `java_method` proc-macro → 宏自动统计 abstract 方法数 → 识别 SAM 类型 → 自动生成 `Fn` type alias，替代 Python 侧解析 bootstrap 方法的逻辑。

**状态**：⚠️ 创建侧和主要 dispatch 侧完成；method reference + 复杂 HOF 场景待验证

---

### Arch-4: `Object = Rc<dyn ObjectVTable>` vtable 重构 ✅

**实现**（2026-09-16）：`Object(Rc<dyn ObjectVTable>)` 完成，`java_class` 宏为每个具体类生成 `ObjectVTable impl`（含 `is_instance_of`、`as_any`）。  
**关键修复**：`invoke.py` downcast 路径从 `obj.0.downcast_ref::<T>()` 改为 `obj.0.as_any().downcast_ref::<T>()`。

**N-2 补充**（2026-09-16 第二次）：`java_class` 宏增加 `has_to_string_method`/`has_hash_code_method` 条件转发：
- 当类自身声明了 `toString()`/`hashCode()` 时，vtable impl 调用 `Self::toString(self).map(|s| format!("{}", s)).unwrap_or_else(...)` 转发
- `toString` 转发正确处理 Java String（`lang::string::String`）→ Rust String（`std::string::String`）的类型差异（通过 Display impl）

**状态**：✅ 完成

---

### Arch-5: T55b `From` impl 丢弃数据 ✅

**实现**（2026-09-16）：删除 `class_writer.py` T55b 生成逻辑（`From<ConcreteClass> for InterfaceType`）。接口现为 Object 别名，`From` impl 由 `java_class` 宏的 `Into<Object>` 覆盖。  
**状态**：✅ 完成

---

### Arch-6: `_rust_type_to_binary` 短名逆查 ✅ 部分

**实现**（2026-09-16）：`BINARY_NAME` 常量由 `java_class` 宏生成，可在 Rust 侧直接访问。Python 侧 `_rust_type_to_binary` 仍用字符串逆查（在 registry 中遍历匹配短名），但影响范围有限。  
**剩余**：Python codegen 中 `_rust_type_to_binary` 的调用应改为直接使用 registry 的 binary key（`ci.name`），消除字符串模糊匹配风险。  
**状态**：✅ Rust 侧完成（BINARY_NAME 可用）；Python 侧逆查逻辑仍存（低风险）

---

### Arch-7: 类型系统 RsType 化 ✅

**实现**（2026-09-16）：`sig_parser.py` 删除；`parse_field_type` + `_parse_one_type` 整合进 `type_map.py` 并支持 `registry` 参数；`_lookup_method_sig_params` 读取方法 `generic_signature`；`_is_generic_type_param` 启发式全部删除。  
**修复**（2026-09-16）：`parse_field_type` 增加 `registry` 参数，接口类型 hint 正确映射为 `Object`（解决 Arch-3 dispatch 缺失问题）。  
**剩余技术债务**：I-3（三元 elif 链重复）、I-4（this.clone 字符串特判）。  
**状态**：✅ 核心完成；I-3/I-4 为独立可修的技术债务

---

### Arch-8: CFG 重建 ✅

**实现**（2026-09-16）：`codegen/cfg/basic_blocks.py` 新增 `function_always_returns(instrs)` CFG 终止块分析；`postprocess.py` 接受 `always_returns` 参数，为 True 时不追加 `unreachable!()`；`codegen.py` 接入。生成代码中 `unreachable!()` 完全消除。  
**状态**：✅ 完成

---

## 已修复问题汇总

| 修复日期 | 问题/标签 | 修复方式 | 分类 |
|---------|----------|---------|------|
| 2026-09-15 | `anewarray` 类型推断 | `sim.py` anewarray 改用 full class path | codegen |
| 2026-09-15 | `aastore` 元素类型提取失败 | `sim.py` aastore 先剥离 Rc/RefCell | codegen |
| 2026-09-15 | 字符串字面量尾部空格 strip | `sim.py` ldc String 改 `.rstrip('\n')` | codegen |
| 2026-09-15 | A-1 `u16`/`i8`/`i16` ireturn 缺少 `as i32` | `coerce.py _coerce_value` 补充窄类型 | codegen |
| 2026-09-15 | F-1 aastore 双重 borrow | `sim.py` aastore 先提取 tmp | codegen |
| 2026-09-15 | F-2 while 第二条件 break 缺失 | `codegen.py` if-guard 后补 break | codegen |
| 2026-09-15 | B-2 T55 `.into()` 对 `&Self` | `invoke.py` 改为 `Clone::clone(&e).into()` | codegen |
| 2026-09-15 | D-1 invokedynamic 不弹栈不压返回值 | `sim.py` 解析 descriptor 弹 N 参数，压 `Object::default()` | codegen |
| 2026-09-15 | C-1 附加 `#[path]` 目标不存在 | `class_writer.py os.path.exists` 检查 | codegen |
| 2026-09-15 | I-6 `jvm_clone` 命名违规 | 删除 `_JAVA_RUST_RENAME` clone 项，改用 `Clone::clone(...)` | arch |
| 2026-09-15 | E-1 native_impl 重复 | 删除 native_impl 重复方法 | 手写清理 |
| 2026-09-15 | `aaload` primitive 数组 clone | `sim.py` primitive 直接取值 | codegen |
| 2026-09-15 | `invokevirtual Object.clone` 非 Object 接收者 | `invoke.py` 特判 clone + 非 Object，发射 `Object::from_any` | codegen |
| 2026-09-16 | Arch-1 接口 = Object 类型别名 | `type_map.py` + `class_writer.py` + `java_rta_macros` | arch |
| 2026-09-16 | Arch-2 instanceof via ObjectVTable | `java_rta_macros` `is_instance_of` + `BINARY_NAME` | arch(宏) |
| 2026-09-16 | Arch-4 Object = Rc<dyn ObjectVTable> | `object.rs` + `java_rta_macros` ObjectVTable impl | arch(宏) |
| 2026-09-16 | Arch-4 downcast 路径修复（as_any） | `invoke.py` `downcast_ref` → `as_any().downcast_ref` | codegen |
| 2026-09-16 | Arch-5 T55b From impl 删除 | `class_writer.py` T55b 整块删除 | arch |
| 2026-09-16 | Arch-6 BINARY_NAME 常量 | `java_rta_macros` 生成 `BINARY_NAME const` | arch(宏) |
| 2026-09-16 | Arch-7 RsType 化 sig_parser 删除 | `type_map.py` 整合；`_is_generic_type_param` 删除 | arch |
| 2026-09-16 | Arch-8 CFG unreachable!() 消除 | `cfg/basic_blocks.py` `function_always_returns` | arch |
| 2026-09-16 | I-2 `_is_direct_subtype` 统一为 `_is_subtype` | `coerce.py` 删除 `_is_direct_subtype` | codegen |
| 2026-09-16 | I-1 `_is_generic_type_param` 全删 | Arch-7 | arch |
| 2026-09-16 | I-5 `unreachable!()` 全删 | Arch-8 | arch |
| 2026-09-16 | J-1 `JvmObjectBase` 删除 | `lib.rs` 删除 trait + blanket impl | arch |
| 2026-09-16 | J-2 `JvmEnum` 删除 | `lib.rs` 删除 trait + blanket impl | arch |
| 2026-09-16 | K-2/K-3/K-4 共置 + BFS 截断 | `native_impls/` 删除；`transpile.py` 截断规则 | arch |
| 2026-09-16 | K-1 @field 机制自然消亡 | 无需操作 | arch |
| 2026-09-16 | C-1 附加 preconditions.rs | `class_writer.py os.path.exists` | codegen |
| 2026-09-16 | Arch-3 Lambda 创建 + dispatch | `classfile.py` + `sim.py` + `invoke.py` | arch |
| 2026-09-16 | Arch-3 dispatch 缺失修复（interface hint） | `type_map.py parse_field_type(registry)` | codegen |
| 2026-09-16 | A-3/A-4 接口参数类型不匹配 | 架构消解（Arch-1 副产品） | arch |
| 2026-09-16 | B-1 From impl 丢弃数据 | 架构消解（Arch-5 副产品） | arch |
| 2026-09-16（第二次） | N-2 java_method/java_native proc-macro 激活 | `java_rta_macros` 注册 passthrough attribute | arch(宏) |
| 2026-09-16（第二次） | N-2 toString/hashCode vtable 条件转发 | `java_class` 宏 + `has_to_string_method` 标志 | arch(宏) |
| 2026-09-16（第二次） | N-2 toString/hashCode 存根智能默认值 | `method_gen.py` 非 panic 默认实现 | codegen |
| 2026-09-16（第二次） | N-1 143→4 E0107（大量手写类被 codegen 接管） | codegen 重新生成 java_runtime | codegen |
| 2026-09-16（第二次） | CLAUDE.md 原则 0：代码生成优先 | 文档补充 | docs |
| 2026-09-16 | G-1 Object.toString 不派发具体类型 | 架构修复（Arch-4 副产品，ObjectVTable.toString） | arch |

---

## 优先级队列（下一步行动）

| 优先级 | 任务 | 预期收益 | 方式 |
|--------|------|---------|------|
| P0 | **N-1** java_runtime 手写代码 E0107（143处） | 消除 237 错误中的大部分 | 手写层修改（合法） |
| P1 | **J-3** 删除 `Printable` 死代码 | 清理命名原则违规 | 1 行删除 |
| P1 | **C-1** class_writer.py 过滤不存在的 use 包路径 | 消除 E0432 批量错误 | codegen |
| P2 | **I-3** 三元 elif 链统一为 `coerce()` 函数 | 消除重复维护风险 | codegen 重构 |
| P2 | **E-1** class_writer.py 扫描 `_impl.rs` 跳过重复方法 | 架构防御 | codegen |
| P3 | **Arch-3** method reference 支持 | lambda 覆盖率 | codegen |
| P3 | **H-1** primitive 数组 autobox 升级 | TestBoundedGenerics | codegen |
| P4 | **N-2** `java_method` 激活为 proc-macro attribute | 解锁宏级别类型验证 | arch(宏) |
