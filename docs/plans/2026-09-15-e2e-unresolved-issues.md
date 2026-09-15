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

---

## A. 编译错误 — 类型转换

### A-1: `char`/`u16` 返回值缺少 `as i32` 强制转换

**错误**：`E0308 mismatched types: expected i32, found u16`  
**位置**：`jdk_classes/src/java/lang/character.rs:605`  
**根因**：JVM `ireturn` 指令处理时，对于返回 `u16`（char 类型）的方法没有插入 `as i32` 转换。`sim.py` 的 `ireturn` 分支只处理 `bool` → `i32`，未处理 `u16` → `i32`。  
**影响测试**：TestStreamAdvanced、TestStringOps 及所有包含 `character.rs` 的测试  
**修复位置**：`codegen/instr/sim.py` — `ireturn` 分支，参考 `_coerce_value` 补充 `u16` case  
**状态**：🔴 未修复

---

### A-2: 方法参数类型不匹配（子类传给父类参数）

**错误**：`E0308: expected InputStream, found InflaterInputStream`  
**位置**：`jdk_classes/src/java/lang/character_name.rs:72`  
**根因**：`invoke.py` 在生成方法调用参数时，只用 `_is_direct_subtype` 检查是否需要 `.into()`，不检查传递子类关系。`InflaterInputStream` 是 `InputStream` 的间接子类，需要 `.into()` 转换。  
**影响测试**：TestStreamAdvanced、TestStringAdvanced 等  
**修复位置**：`codegen/instr/invoke.py` — 参数强制转换逻辑，改用 `_is_subtype`（已在 `areturn` 完成，invoke 侧未同步）  
**状态**：🔴 未修复

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
**状态**：🔴 未修复

---

## C. 编译错误 — 缺失模块/import

### C-1: use 语句引用不存在的 jdk 包

**错误**：`E0432: unresolved import crate::java::lang::r#ref`、`crate::java::lang::reflect`、`crate::sun::security::util` 等  
**位置**：多个 jdk_classes 生成文件的 `use` 块  
**根因**：codegen 把 JDK 类的全部依赖包都写进 `use crate::...::*;`，但这些包（如 `java::lang::r#ref`、`java::lang::reflect`、`sun::security::util`）在当前测试的 call chain 中没有类，因此没有生成对应模块。  
**影响测试**：所有包含 jdk 内部类（如 `PrintStream`、`Class`）的测试  
**修复位置**：`codegen/emitter/project_writer.py` — 写出 `use` 前检查包路径是否在已生成的模块列表中，不存在则跳过  
**状态**：🔴 未修复（高优先级，影响面最广）

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
**状态**：🔴 未修复

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
**状态**：🔴 未修复

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

### 核心发现：`java_class` 宏完全忽略了已有元数据

**每个生成的 struct 上都挂有完整的 Java 字节码元数据**：

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

**每个生成方法上也挂有完整的方法字节码元数据**（但通过 `cfg_attr(any(), ...)` 永远不激活）：

```rust
#[cfg_attr(any(), java_method(
    name = "sum",
    descriptor = "([Ljava/lang/Number;)D",
    is_abstract = false,
    is_native = false,
    generic_signature = "<T:Ljava/lang/Number;>([TT;)D"
))]
pub fn sum(mut arr: ...) -> Result<f64> { ... }
```

**当前 `java_class` 宏的实现**（`java_rta_macros/src/lib.rs:15`）：

```rust
pub fn java_class(_attr: TokenStream, item: TokenStream) -> TokenStream {
//                ^^^^^ 下划线前缀 = 完全丢弃，不读取任何属性参数
```

`binary_name`、`interfaces`、`super_class`、`is_interface`、`generic_signature` 全部被忽略。宏只生成通用的 `Into<Object>`、`From<Object>`、`Debug` 三个 impl，与类的实际 Java 类型信息完全脱节。

**这是所有 Arch 问题的根源**：本可以在 Rust 编译期由宏自动派生的行为，全都降级为 Python 侧脆弱的字符串分析和运行时补丁。

---

### 当前已有的基础设施（升级起点）

以下已有实现是架构升级的出发点，**不需要从零构建**：

**1. `Object` 已是 `Rc<dyn Any>`（`java_runtime/src/java/lang/object.rs:8`）**

```rust
pub struct Object(pub Rc<dyn std::any::Any>);
// from_any: Rc::new(v)，downcast: downcast_ref::<T>()
```

目标态升级路径：`Rc<dyn Any>` → `Rc<dyn JvmObject>`（其中 `JvmObject: Any`），保留 `as_any()` 以兼容 `downcast_ref`。**不是从零重写，只是把 trait object 的接口扩展。**

**2. `JvmObjectBase` 已存在，但是 blanket no-op impl（`java_runtime/src/lib.rs:70`）**

```rust
pub trait JvmObjectBase {
    fn hashCode(&self) -> Result<i32> { Ok(0) }
    fn equals(&self, _other: Object) -> Result<bool> { Ok(false) }
    fn jvm_clone(&self) -> Result<Object> { panic!("stub") }
}
impl<T> JvmObjectBase for T {}  // 全部类型默认实现 = no-op
```

目标态：将 `JvmObjectBase` 扩展为 `JvmObject`（加入 `jvm_type_id`、`jvm_is_instance_of`、`as_any`），由 `java_class` 宏为每个类生成具体 impl，删除 blanket impl。

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

升级后这两个 impl 继续有效（`from_any` 改为 `Object(Rc::new(val) as Rc<dyn JvmObject>)`），宏扩展时只需要在已有代码基础上添加 `JvmObject` impl 生成，不需要改变 `Into`/`From` 逻辑。

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

**目标态方案：`Object` 改为 `Rc<dyn JvmObject>` + 接口改为 Rust trait**

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
pub type List = Object;  // 运行时统一为 Object，通过 JvmObject 动态派发
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

`java_runtime` 中 `Object` 改为：
```rust
pub struct Object {
    inner: Rc<dyn JvmObject>,  // 真正的 dyn dispatch
}
```

**codegen 变更**：
1. `java_rta_macros/src/lib.rs` — `java_class` 宏：读取 `is_interface` / `interfaces` 参数，为接口生成 trait + explicit impls；为实现类生成 `JvmObject` impl（替代 blanket no-op）
2. `java_runtime/src/java/lang/object.rs` — `Object(Rc<dyn Any>)` 升级为 `Object(Rc<dyn JvmObject>)`，`JvmObject` 加入 `fn as_any(&self) -> &dyn Any` 以保留 `downcast` 能力
3. `java_runtime/src/lib.rs` — `JvmObjectBase` blanket impl 删除，改为 `JvmObject` trait（含 `jvm_type_id`、`jvm_is_instance_of`、`jvm_to_string`）
4. Python `class_writer.py` T55b 整块删除（宏通过 `interfaces` 字段自动生成正确 From impl）

**影响**：修复所有集合类方法调用、修复 G-1 虚方法派发。

---

### Arch-2: `instanceof` 始终返回 `true`

**问题**：`instanceof` 生成为字面量 `true`，类型判断失效。

**目标态方案：宏从 `binary_name` + 继承链自动派生 `is_instance_of`**

`java_class` 宏读取 `binary_name`、`super_class`、`interfaces`，自动为每个类生成：

```rust
// java_class 宏自动生成（无需 Python 侧任何改动）
impl JvmObject for ArrayList<Object> {
    fn jvm_type_id(&self) -> &'static str { "java/util/ArrayList" }

    fn jvm_is_instance_of(&self, type_id: &str) -> bool {
        // 宏从 binary_name + super_class chain + interfaces 静态展开
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
    // ...
}
```

`instanceof` 指令生成：`obj.inner.jvm_is_instance_of("java/lang/String")`  
`checkcast` 指令生成：断言类型后 downcast，失败则 `ClassCastException`

**注意**：继承链需要传递闭合。宏只能看到当前类的直接父类/接口，`super_class` 的父类需要递归展开。方案：
- 宏生成时只展开直接层次，传递闭合在运行时通过调用 `super.jvm_is_instance_of()` 完成
- 或：Python codegen 在 `java_class` 属性中写入完整传递闭合后的 `all_supertypes` 字段，宏直接展开

**codegen 变更**：
1. `java_rta_macros` — 读取 `binary_name`、`interfaces`，生成 `JvmObject` impl
2. `java_runtime/src/types.rs` — `Object` 改为 `Rc<dyn JvmObject>`，`isinstance` 方法委托给 `inner`
3. Python `sim.py` — `isinstance` 生成 `obj.inner.jvm_is_instance_of("java/lang/String")`
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
impl JvmObject for Comparator_Fn { ... }
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

**目标态方案：`Object = Rc<dyn JvmObject>` 天然支持动态派发**

`Arch-1` 完成后（`Object` 改为 `Rc<dyn JvmObject>`），此问题自动解决：

```rust
pub trait JvmObject: 'static {
    fn jvm_type_id(&self) -> &'static str;
    fn jvm_to_string(&self) -> String;      // 派发到具体类型的 toString()
    fn jvm_hash_code(&self) -> i32;
    fn jvm_equals(&self, other: &Object) -> bool;
    fn jvm_is_instance_of(&self, type_id: &str) -> bool;
    fn jvm_clone(&self) -> Object;
}

// java_class 宏自动生成：
impl JvmObject for Point {
    fn jvm_to_string(&self) -> String {
        // 转发到翻译生成的 toString() 方法（若存在）
        self.toString().unwrap_or_else(|_| format!("{}@...", Self::BINARY_NAME))
    }
    // ...
}
```

`Object::to_print_string()` 调用 `self.inner.jvm_to_string()`，自动 dispatch 到 `Point::toString()`。

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

### 架构变更优先级与依赖关系

```
D-1 短期（invokedynamic 弹出+占位）   → 立即解锁 7 个 lambda 测试编译
     │
     ▼
java_class 宏重写（核心前置工作）：
  ├── 读取 binary_name → 生成 BINARY_NAME const
  ├── 读取 interfaces  → 自动生成 From impl（替代 T55b）
  ├── 读取 is_interface → 不同代码路径
  └── 派生 JvmObject trait（含 jvm_type_id、is_instance_of）
     │
     ├─→ Arch-2（instanceof 正确实现）    → 5+ 测试
     │
     ├─→ Arch-6（_rust_type_to_binary 消除）→ 稳定性
     │
     └─→ Object 改为 Rc<dyn JvmObject>（核心类型重构）
              │
              ├─→ Arch-1（接口 trait + dyn dispatch）→ 10+ 测试
              ├─→ Arch-4（虚方法派发，Arch-1 副产品）→ 5+ 测试
              └─→ Arch-5（From impl 正确，Arch-1 副产品）→ 自动修复
```

| 顺序 | 工作项 | 影响 | 前置 |
|------|--------|------|------|
| 1 | **D-1 短期**（invokedynamic 弹/压占位） | 消除 7 个测试的 E0308 | 无 |
| 2 | **`java_class` 宏重写**（读取所有元数据） | 所有 Arch 的基础 | 无 |
| 3 | **Arch-2**（instanceof + BINARY_NAME） | 5+ 测试 + 消除 Arch-6 | 宏重写 |
| 4 | **Object 改 `Rc<dyn JvmObject>`** | 基础类型重构 | 宏重写 |
| 5 | **Arch-1**（接口 → Rust trait + dyn dispatch） | 10+ 测试语义修复 | Object 重构 |
| 6 | **Arch-3 长期**（invokedynamic → 真实闭包） | Lambda 语义 | Arch-1 |

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
