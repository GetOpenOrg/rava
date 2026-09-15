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

### B-1: ~~Arrays_ArrayList → List 缺少 From impl~~ ✅ 已修复（2026-09-15）

**修复方案**：`class_writer.py` T55b 扩展为同时收集祖先类的接口，生成传递 `From` impl。  
**影响测试**：TestLinkedList、TestComparator、TestStreamAdvanced、TestStreamBasic  
**状态**：✅ 已修复

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

### D-1: Lambda/方法引用 invokedynamic 生成 `todo!()`

**错误**：`E0308: mismatched types`、`error: internal compiler error`（因 todo! 类型不确定）  
**位置**：生成代码中所有含 `invokedynamic` 的方法  
**根因**：`sim.py` 对 `invokedynamic` 生成 `todo!("invokedynamic: ...")` 占位，该表达式类型为 `!`（never），在具体类型上下文中可能引发推断失败。  
**影响测试**：TestLambda、TestMethodRef、TestFunctionalInterface、TestComparator、TestStreamBasic、TestStreamAdvanced、TestStreamCollectors（7 个测试）  
**修复思路**：  
  - 短期：改为 `Default::default()` 或 `Object::default()`，至少编译通过  
  - 长期：识别 bootstrap 方法，生成对应的 Rust 闭包包装在 `Object` 中  
**修复位置**：`codegen/instr/sim.py` — `invokedynamic` 分支  
**状态**：🔴 未修复

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

以下问题超出局部 bug fix 范畴，需要设计层面解决。每个问题均给出最终目标态的具体实现方案。

---

### Arch-1: 接口无法携带数据

**问题**：`List<E>`、`Comparable<T>` 等接口在 Rust 中生成为 `struct List<E>(PhantomData<E>)`，不携带任何具体实现类的数据。调用 `list.size()` 等方法时永远 panic（stub）。

**目标态方案：以 `enum` 承载多态 + `impl Trait` 统一接口**

每个接口对应一个同名 `enum`，所有已知实现类作为 enum variant：

```rust
// 由 codegen 根据全量 BFS 实现类列表自动生成
pub enum List<E> {
    ArrayList(ArrayList<E>),
    LinkedList(LinkedList<E>),
    ArraysArrayList(Arrays_ArrayList<E>),
    // ...每个实现类一个 variant
}

impl<E: Clone + Default + PartialEq> List<E> {
    pub fn size(&self) -> Result<i32> {
        match self {
            List::ArrayList(inner) => inner.size(),
            List::LinkedList(inner) => inner.size(),
            List::ArraysArrayList(inner) => inner.size(),
        }
    }
    // 所有接口方法均做 match dispatch
}

// From impl 改为指向 enum variant
impl<E> From<ArrayList<E>> for List<E> {
    fn from(v: ArrayList<E>) -> Self { List::ArrayList(v) }
}
```

**codegen 变更**：
1. `class_writer.py`：发现接口定义时，生成 `enum` 而非 `struct PhantomData`
2. 分析阶段收集"接口 → 所有实现类"映射（BFS 已有 registry），写进 `InterfaceInfo.implementations`
3. 为 enum 的每个 variant 生成 match arm，转发到 inner 类型的方法
4. `From` impl 改为 `List::ArrayList(v)` 而非空转换

**影响**：彻底修复 G-1（虚方法派发）和所有集合类接口方法调用错误。

---

### Arch-2: `instanceof` 始终返回 `true`

**问题**：`instanceof` 指令当前生成为字面量 `true`，导致类型判断逻辑失效（如 `if (obj instanceof String)` 永远走 true 分支）。

**目标态方案：`Object` 携带类型 discriminant**

在 `Object` 结构体中加入类型标识，`instanceof` 检查时对比：

```rust
// java_runtime/src/types.rs
pub struct Object {
    pub _type_id: &'static str,           // 二进制类名，如 "java/lang/String"
    pub _fields: Rc<RefCell<std::collections::HashMap<String, Box<dyn std::any::Any>>>>,
}

// Object::from_any 在转换时捕获具体类型
impl Object {
    pub fn from_any<T: 'static>(val: T, type_id: &'static str) -> Self {
        Object { _type_id: type_id, _fields: ... }
    }
}
```

**codegen 变更**：
1. `sim.py` — `instanceof` 分支：生成 `(obj._type_id == "java/lang/String")` 而非 `true`
2. `sim.py` — `checkcast` 分支：生成 `if obj._type_id != "..." { return Err(ClassCastException) }`
3. `sim.py` — 所有转为 `Object` 的地方（`Object::from_any` 调用）：传入第二参数为当前类的 `BINARY_NAME` 常量（由 `class_writer` 在 struct 上方生成 `const BINARY_NAME: &str = "java/lang/String";`）
4. `class_writer.py` — 每个生成类顶部加 `const BINARY_NAME: &str = "...";`

**影响**：修复 TestCasting、TestPatternMatch 等所有依赖类型判断的测试。

---

### Arch-3: `invokedynamic` / Lambda 无法表达

**问题**：Java lambda 编译为 `invokedynamic`，当前 codegen 生成 `todo!("invokedynamic: ...")`，导致类型推断失败（`!` 类型无法统一）。

**目标态方案：闭包包装进 `Object`，函数式接口用 `Arc<dyn Fn>` 表达**

```rust
// java_runtime/src/types.rs
// 函数式接口载体（替代 PhantomData struct）
pub struct FunctionalObject {
    pub _fn: Arc<dyn Fn(Vec<Object>) -> Result<Object> + 'static>,
}

impl Object {
    pub fn from_fn(f: impl Fn(Vec<Object>) -> Result<Object> + 'static) -> Self {
        // 用 FunctionalObject 包装并存入 Object
    }
}
```

**codegen 变更**：
1. `sim.py` — `invokedynamic` 分支：解析 bootstrap 方法的 `MethodHandle` 引用，识别 lambda body 对应的合成方法（`lambda$main$0` 等），生成捕获变量 + `Arc::new(move |args| ...)` 闭包
2. 函数式接口的方法调用（如 `Comparator.compare(a, b)`）：生成 `match obj { FunctionalObject(f) => f(vec![a, b]) }`
3. 短期过渡：`invokedynamic` 生成 `Object::from_fn(|_| Ok(Object::default()))` 而非 `todo!()`，至少保证编译

**影响**：修复 TestLambda、TestMethodRef、TestFunctionalInterface、TestComparator、TestStreamBasic 等 7 个测试的编译错误。

---

### Arch-4: 虚方法派发（`Object.toString()` 不调用具体类型）

**问题**：将具体类型（如 `Point`）转为 `Object` 后调用 `toString()`/`to_print_string()`，不会 dispatch 到 `Point::toString()`，而是返回 `"Object"`。

**目标态方案：`Object` 内嵌 vtable 函数指针**

在 `Object` 中存储关键虚方法的函数指针，在 `from_any` 时绑定具体类型实现：

```rust
// java_runtime/src/types.rs
pub struct Object {
    pub _type_id: &'static str,
    pub _to_string: Arc<dyn Fn() -> Result<String> + 'static>,   // String 指 java::lang::String
    pub _hash_code: Arc<dyn Fn() -> Result<i32> + 'static>,
    pub _equals: Arc<dyn Fn(&Object) -> Result<bool> + 'static>,
    pub _fields: Rc<RefCell<std::collections::HashMap<String, Box<dyn std::any::Any>>>>,
}

// 每个生成类实现 JvmObjectBase trait
pub trait JvmObjectBase {
    fn to_java_string(&self) -> Result<super::java::lang::String>;
    fn java_hash_code(&self) -> Result<i32>;
    fn java_equals(&self, other: &Object) -> Result<bool>;
}

impl Object {
    pub fn from_any_with_vtable<T: JvmObjectBase + Clone + 'static>(val: T, type_id: &'static str) -> Self {
        let val = Arc::new(val);
        Object {
            _type_id: type_id,
            _to_string: Arc::new({
                let v = val.clone();
                move || v.to_java_string()
            }),
            _hash_code: Arc::new({ let v = val.clone(); move || v.java_hash_code() }),
            _equals: Arc::new({ let v = val.clone(); move |o| v.java_equals(o) }),
            _fields: Rc::new(RefCell::new(Default::default())),
        }
    }
    pub fn to_print_string(&self) -> Result<String> { (self._to_string)() }
}
```

**codegen 变更**：
1. `class_writer.py`：每个生成类 `impl JvmObjectBase for MyClass`，转发到已生成的 `toString()`、`hashCode()`、`equals()` 方法
2. `sim.py` — `Object::from_any` 调用点：改为 `Object::from_any_with_vtable(val, MyClass::BINARY_NAME)`
3. `Object::to_print_string()`：调用 `(self._to_string)()`，触发具体类型的 `toString()`

**影响**：修复 G-1（TestRecord、TestObjects 等所有涉及 `println(Object)` 的测试）。

---

### 架构变更优先级

| 优先级 | 架构项 | 解锁测试数 | 前置依赖 |
|--------|--------|-----------|---------|
| 1 | **Arch-2（instanceof type tag）** | 5+ | 无 |
| 2 | **Arch-3（invokedynamic 短期过渡）** | 7 | 无 |
| 3 | **Arch-1（接口 enum dispatch）** | 10+ | Arch-2（需要 type_id） |
| 4 | **Arch-4（Object vtable）** | 5+ | Arch-1、Arch-2 |

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
| 2026-09-15 | `areturn` 只检查直接子类，传递子类不生成 `.into()` | `sim.py` areturn 改用 `_is_subtype` |
| 2026-09-15 | `native_impl string.rs` 与字节码翻译重复定义（E0592） | 删除 native_impl 中的重复方法 |
| 2026-09-15 | `array_list.rs iterator()` 返回 `Iterator<Object>` 找不到类型 | 改回 `Result<Object>` |
