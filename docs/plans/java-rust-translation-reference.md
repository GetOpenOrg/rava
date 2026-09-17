# Java → Rust 转译对照规则

> 本文档定义 java_rta 转译器在各类 Java 语言构造上的 Rust 等价形式。  
> 目标：让 Java 开发者能够直接通过生成的 Rust 源码还原 Java 语义，所有 Rust 实现复杂度由 `java_class!` 宏和 codegen 管线封装，对读者不可见。  
> 关联文档：[`2026-09-17-java-rust-type-1to1.md`](2026-09-17-java-rust-type-1to1.md)（类型 1:1 对应任务跟踪）

---

## 目录

1. [基本原则](#1-基本原则)
2. [原生类型](#2-原生类型)
3. [引用类型与对象模型](#3-引用类型与对象模型)
4. [字段访问封装](#4-字段访问封装)
5. [继承与 Deref 链](#5-继承与-deref-链)
6. [虚方法分发（多态）](#6-虚方法分发多态)
7. [数组](#7-数组)（`Array<T>` 封装 `Rc<RefCell<Vec<T>>>`）
8. [异常处理](#8-异常处理)
9. [null 与 Optional 语义](#9-null-与-optional-语义)
10. [instanceof 与类型检查](#10-instanceof-与类型检查)
11. [String 类型](#11-string-类型)
12. [包装类型与 Autoboxing](#12-包装类型与-autoboxing)
13. [接口类型](#13-接口类型)
14. [静态字段与 `<clinit>`](#14-静态字段与-clinit)
15. [方法调用约定](#15-方法调用约定)
16. [禁止出现在可读层的调用列表](#16-禁止出现在可读层的调用列表)

---

## 1 基本原则

### 1.1 核心目标：Java 开发者只读业务逻辑，不感知 Rust 底层

这个转译器的本质是 **Java 语言的新编译后端**，不是迁移工具。  
开发者继续写 Java，构建流程自动生成原生二进制。生成的 Rust 代码是 **可读的中间层**——Java 开发者能直接对应原始 Java 逻辑，无需学习 Rust 的所有权、生命周期、trait dispatch。

```java
// Java 原代码
Animal animal = new Dog();
animal.speak();
```
```rust
// 生成的 Rust——开发者能直接对应读懂
let animal: Animal = Dog::new();
animal.speak();
// vtable dispatch、Rc<dyn Trait>、borrow 等全部由生成器封装，开发者不感知
```

**这是本质上的区别**：读者看到的是业务逻辑，不是 Rust 实现机制。

### 1.2 两个阅读层次

| 层次 | 读者 | 看到的内容 |
|------|------|-----------|
| **方法体（读者可见层）** | Java 开发者 | 与 Java 1:1 对应的字段名、方法调用、控制流 |
| 宏展开产物 | Rust 工具链 / 性能调试 | `RefCell`、`Rc`、`borrow_mut`、vtable trait impl |

**判定标准**：生成的方法体中，若出现任何 Rust 专属的底层调用（`borrow()`、`downcast::<T>()`、`Rc::new`、`Object::from_any` 等），即视为封装不足，需修复生成器。完整的禁止列表见 §16。

### 1.3 封装责任分工

| 构造特征 | 处理方 |
|---------|-------|
| 变换纯机械、信息在 `java_class!` 块内可见 | `java_class!` 宏（token 重写） |
| 需要字节码级分析、类层次信息 | codegen Python 侧（生成注解传给宏） |
| 跨所有生成类通用的 Rust trait/impl | `java_runtime` blanket impl |

---

## 2 原生类型

直接双射，无任何包装：

| Java | Rust | 说明 |
|------|------|------|
| `int` | `i32` | 相同位宽、相同溢出语义（`wrapping_*`） |
| `long` | `i64` | 同上 |
| `short` | `i16` | |
| `byte` | `i8` | |
| `char` | `u16` | Java char 是 UTF-16 码元 |
| `float` | `f32` | |
| `double` | `f64` | |
| `boolean` | `bool` | |
| `void` | `()` | |

```java
// Java
int x = 42;
long y = x + 1L;
```
```rust
// 生成的 Rust（方法体内）
let mut x: i32 = 42i32;
let mut y: i64 = (x as i64).wrapping_add(1i64);
```

原生类型的直接映射不是"绕行"，是数学意义上的一一对应，**不需要包装**。

---

## 3 引用类型与对象模型

### 3.1 Object = Rc<dyn ObjectVTable>

Java 中每个对象都是 `Object` 的子类。Rust 用 `Rc<dyn ObjectVTable>` 建模：

```rust
// java_runtime 中定义（从 Object.class 字节码翻译）
pub type Object = Rc<dyn ObjectVTable>;

pub trait ObjectVTable: Any + DynClone {
    fn hashCode(&self) -> Result<i32>;
    fn equals(&self, obj: Object) -> Result<bool>;
    fn toString(&self) -> Result<String>;
    fn getClass(&self) -> Result<Object>;
    // ... 其他 Object 方法
}
```

`Rc`（引用计数）建模 Java 的 GC 引用语义；`dyn ObjectVTable` 提供多态性。

### 3.2 具体类

每个 Java 类由 `java_class!` 宏展开为三层结构，但对读者只暴露第一层：

```rust
// 读者看到的（java_class! 输入）
java_class! {
    #[binary_name = "java/util/ArrayList"]
    pub struct ArrayList<E> {
        elementData: Vec<E>,
        size: i32,
    }
    impl ArrayList<E> { ... }
}

// 宏展开后（对读者不可见）：
// - ArrayList__inner { elementData: RefCell<Vec<E>>, size: Cell<i32>, ... }
// - ArrayList(Rc<RefCell<ArrayList__inner>>)
// - impl ObjectVTable for ArrayList__inner { ... }
// - impl<E: Clone + Default + 'static> ArrayList<E> { __get_size(), __set_size(), ... }
```

---

## 4 字段访问封装

### 4.1 原生类型字段

使用 `Cell<T>` 提供内部可变性，通过 `__get_xxx`/`__set_xxx` 访问：

```java
// Java
this.size = this.size + 1;
```
```rust
// java_class! 块内（对读者可见，与 Java 1:1）
self.size += 1;

// 宏展开后（对读者不可见）
self.__set_size(self.__get_size().wrapping_add(1i32));
```

### 4.2 引用类型字段

使用 `RefCell<Option<Box<T>>>` 存储，提供 borrow 和 clone 语义：

```java
// Java
this.cause = new RuntimeException("msg");
```
```rust
// java_class! 块内（1:1 Java 风格）
self.cause = RuntimeException::new_str("msg")?;

// 宏展开后（不可见）：调用 __set_cause(v)，内部做 *borrow_mut() = Some(Box::new(v))
```

`RefCell<Option<Box<T>>>` 选型原因：
- `RefCell`：运行时借用检查，建模 Java 字段的可变性
- `Option`：`Default::default()` 返回 `None`，解决自引用类型（如 `Throwable.cause: Throwable`）的初始化问题
- `Box`：打破递归类型的大小无限问题

**同等封装原则适用于数组**：`Rc<RefCell<Vec<T>>>` 是数组的内部存储，也必须完全隐藏，由 `Array<T>` newtype 封装（见 §7）。

---

## 5 继承与 Deref 链

### 5.1 继承字段展平

父类的实例字段以 `_super: Parent` 存储在子类内：

```java
// Java
class Dog extends Animal { ... }
```
```rust
// java_class! 展开的 inner struct（不可见）
struct Dog__inner {
    _super: Animal__inner,  // 继承字段
    // Dog 自己的字段...
}
```

子类可以直接访问父类字段（`self.name` 等），宏负责转发访问器。

### 5.2 Deref 向上转型

每个类自动获得 `Deref<Target=DirectParent>`，支持 Rust deref coercion：

```java
// Java：Dog 可以直接调用 Animal 的方法
dog.getName();   // getName 定义在 Animal
```
```rust
// 生成的 Rust：通过 Deref 自动找到父类方法
dog.getName()?;  // Deref: Dog → Animal → ... 自动解引用

// 宏展开后（不可见）：
// impl Deref for Dog {
//     type Target = Animal;
//     fn deref(&self) -> &Animal { &self.0.borrow()._super }
// }
```

不需要逐层 `From<Dog> for Animal` 显式转换。

### 5.3 __into_super() 显式向上转型

当方法接受父类参数时，codegen 生成 `__into_super()` 调用：

```java
// Java
Animal a = dog;  // 赋值给父类引用
```
```rust
// 生成的 Rust
let mut a: Animal = Clone::clone(&dog).__into_super();
```

**注意**：此处发生静态类型转换，虚方法分发能力依赖下节的 vtable 机制保留。

---

## 6 虚方法分发（多态）

### 6.1 设计目标

Java 中对象存入父类引用后，虚方法调用仍路由到实际类型：

```java
Animal[] animals = { dog, cat };
for (Animal a : animals) {
    a.speak();   // → Dog::speak / Cat::speak，不是 Animal::speak
}
```

Rust 中要实现等价语义，每个有虚方法的类生成 **per-class vtable trait**。

### 6.2 per-class vtable trait（java_class! 生成）

```rust
// java_class! 为 Animal 自动生成（宏展开，对读者不可见）
pub trait Animal__VTable: ObjectVTable {
    fn speak(&self)   -> Result<String>;
    fn getName(&self) -> Result<String>;
}

// Animal 存储的是 vtable 指针，保留子类的动态类型
pub struct Animal(Rc<dyn Animal__VTable>);
```

`Dog`、`Cat` 在其 `java_class!` 块中（`#[superclass = "Animal"]`），宏自动生成：

```rust
// 宏为 Dog 自动生成（不可见）
impl Animal__VTable for Dog__inner {
    fn speak(&self)   -> Result<String> { self.speak_impl() }     // 覆盖
    fn getName(&self) -> Result<String> { self._super.getName() } // 委托父类
}
```

### 6.3 读者视角（生成代码 1:1 对应 Java）

```java
// Java
Animal a = dog;
a.speak();
```
```rust
// 生成的 Rust（java_class! 块内，1:1）
let mut a: Animal = dog.into();   // Dog → Rc<dyn Animal__VTable>，无类型信息丢失
let _t = a.speak()?;              // 通过 vtable 路由到 Dog::speak
```

存入 `Animal[]` 时同理：

```rust
// 生成的 Rust（数组存入）
animals.borrow_mut()[1] = Animal::from(dog.clone());  // Dog 信息保留在 vtable 中
// 取出后直接调用，无 downcast 链
let _t = animals.borrow()[1].speak()?;  // vtable 分发，透明
```

### 6.4 虚方法判定规则

codegen Python 侧根据字节码 access flags 标注，`java_class!` 宏据此生成 vtable：

| 方法特征 | 是否入 vtable |
|---------|-------------|
| `ACC_STATIC` | 否 |
| `ACC_PRIVATE` | 否 |
| `ACC_FINAL`（且类也是 final） | 否 |
| 其余实例方法 | **是** |

### 6.5 当前状态

> ⚠️ **虚方法 vtable（本节）尚在规划阶段**，当前实现使用 `__into_super()` 向上转型 + 静态绑定。修复 `TestInheritance` 多态分发需实现此节。  
> 涉及改动：`java_rta_macros/src/block.rs`（生成 vtable trait）+ `codegen/method/codegen.py`（标注 `is_virtual`）。

---

## 7 数组

**封装要求**：`Rc<RefCell<Vec<T>>>` 是数组的 Rust 存储实现细节，**禁止出现在生成代码的任何可读层次**——类型注解、方法体读写、多维嵌套全部由 `Array<T>` newtype 和 codegen 封装吸收。Java 开发者看到的数组操作应与 Java 源码 1:1 对应。

### 7.1 Array<T> newtype（java_runtime 提供）

```rust
// java_runtime/src/java/lang/array.rs（永久基础设施）
#[derive(Clone, Default)]
pub struct Array<T>(Rc<RefCell<Vec<T>>>);   // 内部细节，对外不可见

impl<T: Clone + Default> Array<T> {
    pub fn new(len: i32) -> Self { ... }
    pub fn get(&self, i: i32) -> T { ... }
    pub fn set(&self, i: i32, v: T) { ... }
    pub fn len(&self) -> i32 { ... }
}
```

### 7.2 原生类型数组

```java
// Java
int[] arr = new int[10];
arr[2] = 42;
int x = arr[2];
int len = arr.length;
```
```rust
// 目标：生成的 Rust（java_class! 块内，1:1）
let mut arr: Array<i32> = Array::new(10i32);
arr[2i32] = 42i32;          // java_class! 宏重写 → arr.set(2i32, 42i32)
let x: i32 = arr[2i32];     // 宏重写 → arr.get(2i32)
let len: i32 = arr.len();
```

### 7.3 引用类型数组

```java
// Java
Animal[] animals = new Animal[4];
animals[0] = dog;
Animal a = animals[1];
```
```rust
// 目标：生成的 Rust（1:1）
let mut animals: Array<Animal> = Array::new(4i32);
animals[0i32] = dog.clone();    // 宏重写 → animals.set(0i32, dog.clone())
let a: Animal = animals[1i32];  // 宏重写 → animals.get(1i32)
```

### 7.4 多维数组

```java
// Java
int[][] matrix = new int[3][4];
matrix[1][2] = 99;
```
```rust
// 目标：生成的 Rust（1:1，嵌套 Array<Array<i32>> 内部细节不可见）
let mut matrix: Array<Array<i32>> = Array::new(3i32);
// 内层初始化由 codegen/宏处理（对读者不可见）
matrix[1i32][2i32] = 99i32;    // 宏两次重写：get(1)+set(2)
```

### 7.5 完整对比

| | Java | 目标 Rust（可见层） | 当前实现（待修复） |
|---|------|-------------------|-----------------|
| 类型注解 | `int[]` | `Array<i32>` | `Rc<RefCell<Vec<i32>>>` |
| 数组创建 | `new int[10]` | `Array::new(10i32)` | `Rc::new(RefCell::new(vec![...]))` |
| 元素写入 | `arr[i] = v` | `arr[i] = v`（宏重写） | `arr.borrow_mut()[i as usize] = v` |
| 元素读取 | `arr[i]` | `arr[i]`（宏重写） | `arr.borrow()[i as usize]` |
| 数组长度 | `arr.length` | `arr.len()` | `arr.borrow().len() as i32` |

### 7.6 实现路径

三步递进，每步独立可验证：

| 步骤 | 改动位置 | 消除的噪声 |
|------|---------|-----------|
| **Step 1** 定义 `Array<T>` | `java_runtime/src/java/lang/array.rs` | 类型注解从 `Rc<RefCell<Vec<T>>>` → `Array<T>` |
| **Step 2** codegen 改用 `Array` API | `codegen/instr/sim.py`（数组指令分支） | `borrow()`/`borrow_mut()` 从方法体消失 |
| **Step 3** `java_class!` 支持 `arr[i]` 重写 | `java_rta_macros/src/block.rs`（`VisitMut` 扩展） | 方法体内 `arr[i]` 语法与 Java 完全 1:1 |

> ⚠️ **当前状态**：Step 1–3 均未实现，生成代码中 `Rc<RefCell<Vec<T>>>` 和 `borrow()` 调用对读者可见，待修复。

---

## 8 异常处理

### 8.1 所有方法返回 Result<T>

```java
// Java
public String charAt(int i) throws IndexOutOfBoundsException { ... }
```
```rust
// 生成
pub fn charAt(&self, i: i32) -> Result<u16> { ... }
```

`Result<T>` 等价于 `std::result::Result<T, JvmError>`，`?` 运算符传播异常。

### 8.2 try-catch 块

```java
try {
    int x = parseInt(s);
} catch (NumberFormatException e) {
    System.out.println("bad");
}
```
```rust
// 生成
match parseInt_str(s.clone()) {
    Ok(x) => { let mut x = x; }
    Err(e) if e.is_instance_of("java/lang/NumberFormatException") => {
        let e = e.as_throwable();
        System::out_println(String::from("bad"))?;
    }
    Err(e) => return Err(e),
}
```

### 8.3 throw

```java
throw new IllegalArgumentException("msg");
```
```rust
return Err(JvmError::from(IllegalArgumentException::new_str("msg")?));
```

---

## 9 null 与 Optional 语义

### 9.1 null → Object::default()

Java `null` 映射为 `Object::default()`（空 `Rc<dyn ObjectVTable>`，内部为占位默认值）：

```java
Object o = null;
if (o == null) { ... }
```
```rust
// 生成
let mut o: Object = Object::default();
if _is_jnull(&o) { ... }
```

`_is_jnull` 检测 `Rc` 内部是否为默认占位实例。

### 9.2 引用类型字段的 null

字段声明为 `RefCell<Option<Box<T>>>`，`None` 对应 Java 的 `null`（见 §4.2）。

---

## 10 instanceof 与类型检查

```java
if (animal instanceof Dog) { ... }
if (animal instanceof Dog d) { d.speak(); }  // Java 16+ pattern matching
```
```rust
// 生成
if animal.is_instance_of("test_inheritance/Dog") { ... }
// pattern matching（尚未实现，规划中）
if let Some(d) = animal.try_downcast::<Dog>() { d.speak()?; }
```

`is_instance_of` 通过 `BINARY_NAME` 静态常量和继承链查找，由 `ObjectVTable` 提供。

---

## 11 String 类型

### 11.1 java.lang.String

`java.lang.String` 从 JDK 字节码翻译，不映射到 Rust `std::string::String`：

```rust
// java_runtime 中生成的（来自 String.class 字节码）
java_class! {
    #[binary_name = "java/lang/String"]
    pub struct String {
        value: Vec<u16>,   // UTF-16 存储
        coder: i8,
        hash: i32,
        ...
    }
    impl String {
        pub fn charAt(&self, index: i32) -> Result<u16> { ... }
        pub fn length(&self) -> Result<i32> { ... }
        ...
    }
}
```

字符串字面量由 codegen 通过 `String::from("...")` 构造（`from` 走 `string_impl.rs` 的 native 实现）。

### 11.2 字符串连接

```java
"Hello, " + name
```
```rust
// 生成（invokedynamic makeConcatWithConstants → String::from + concat 链）
String::concat(String::from("Hello, "), name.clone())?
```

---

## 12 包装类型与 Autoboxing

### 12.1 透明映射（当前实现）

包装类型在当前实现中透明映射为原生类型：

| Java | Rust（当前） | 备注 |
|------|------------|------|
| `Integer` | `i32` | autoboxing/unboxing 由 codegen 透明处理 |
| `Long` | `i64` | 同上 |
| `Boolean` | `bool` | 同上 |
| `Character` | `u16` | 同上 |
| `Double` | `f64` | 同上 |
| `Float` | `f32` | 同上 |

### 12.2 Autoboxing/Unboxing 透明处理

codegen 识别 `Integer.valueOf(x)` / `x.intValue()` 字节码模式，直接用原生类型跳过：

```java
// Java
Integer i = 42;              // autoboxing
int x = i + 1;               // unboxing
List<Integer> list = ...;
list.add(100);               // autoboxing
```
```rust
// 生成（透明，与 Java 代码读起来无差别）
let mut i: i32 = 42i32;
let mut x: i32 = i.wrapping_add(1i32);
list.add(100i32)?;
```

### 12.3 长期目标

包装类未来从 `Integer.class` 等字节码生成（T-4），届时 `Integer.parseInt`、`Integer.MAX_VALUE` 等静态 API 将自动可用，当前通过 `integer_impl.rs` native 方法覆盖。

---

## 13 接口类型

### 13.1 Arch-1：接口 = Object 别名

Rust 不支持带类型参数的类型别名（`type Iterator<E> = Object` 非法），接口类型在方法签名中由 codegen 动态擦除为 `Object`：

```java
// Java
public Iterator<E> iterator() { ... }
public boolean add(E e, Comparator<E> cmp) { ... }
```
```rust
// java_class! 块内（codegen 擦除后）
pub fn iterator(&self) -> Result<Object> { ... }
pub fn add(&self, e: E, cmp: Object) -> Result<bool> { ... }
```

接口参数在方法体内按 `Object` 使用，调用接口方法时 codegen 识别并路由到 `Object` 上的对应调用。

### 13.2 接口检测机制

codegen 通过扫描 registry 中各类的 `interfaces` 字段动态识别接口类型，无任何硬编码的 JDK 类名。所有接口在生成时声明为：

```rust
pub type SomeInterface = Object;   // 接口 = Object 别名（无泛型参数版本）
```

---

## 14 静态字段与 `<clinit>`

### 14.1 静态字段

```java
// Java
public static final int MAX_VALUE = 2147483647;
private static final int[] DIGITS = { ... };
```
```rust
// 生成（方法体内静态字段访问）
Integer::MAX_VALUE()    // → 2147483647i32（codegen 从 <clinit> 提取常量）
Integer::DIGITS()       // → static Rc<RefCell<Vec<i32>>> 懒初始化
```

### 14.2 <clinit> 提取

Java 类的静态初始化块 `<clinit>` 中的常量数组赋值由 `_extract_clinit_arrays` 提取，生成 Rust `once_cell::sync::Lazy` 静态值。

---

## 15 方法调用约定

### 15.1 方法名映射

Java 方法名保持原样（`camelCase`），参数类型签名附加在方法名后以解决重载：

| Java 签名 | 生成的 Rust 函数名 |
|---------|-----------------|
| `int parseInt(String s)` | `parseInt_str` |
| `int parseInt(String s, int radix)` | `parseInt_str_i` |
| `boolean equals(Object o)` | `equals_obj` |
| `void speak()` | `speak` |

参数类型缩写规则：`i`=int、`l`=long、`b`=byte、`s`=String（完整规则见 `type_map.py`）。

### 15.2 静态方法 vs 实例方法

```java
// Java 静态方法
Integer.parseInt("123")
// 生成
Integer::parseInt_str(String::from("123"))?

// Java 实例方法  
obj.toString()
// 生成
obj.toString()?
```

### 15.3 super 调用

```java
// Java
super.speak()
```
```rust
// 生成
self._super.speak()?
// 或通过 Deref 链自动解引用
```

---

## 16 禁止出现在可读层的调用列表

**凡下表中「禁止出现」的调用，一旦出现在 `java_class!` 块内（读者可见层），即视为封装缺失，需修复生成器或宏，而非接受为"正常写法"。**

### 16.1 完整禁止列表

| 禁止出现的调用 | 对应 Java 语义 | 目标替换形式 | 由谁隐藏 |
|--------------|--------------|------------|---------|
| `.borrow()` | 字段读 / 数组读 | 透明（宏重写字段访问；`Array::get` 隐藏数组读） | `java_class!` 宏 + `Array<T>` |
| `.borrow_mut()` | 字段写 / 数组写 | 透明（宏重写；`Array::set` 隐藏数组写） | `java_class!` 宏 + `Array<T>` |
| `.downcast::<T>()` | `(T) obj` 强制转型 | `T::from(obj)` 或类型推导 `obj.into()` | `java_class!` 宏生成 `impl From<Object> for T` |
| `Object::from_any(v)` | 隐式向上转型（赋给 Object 引用） | `v.into()` | blanket `From<T> for Object`（R-1 已完成） |
| `Rc::new(RefCell::new(...))` | 数组/对象创建 | `Array::new(n)` / 构造器 | `Array<T>` + `java_class!` 构造器展开 |
| `Rc<RefCell<Vec<T>>>` 类型标注 | `T[]` 数组类型 | `Array<T>` | `Array<T>` newtype（§7） |
| `__get_xxx()` / `__set_xxx()` 直接调用 | 字段读写 | `self.xxx` / `self.xxx = v`（宏重写） | `java_class!` 宏 token 重写 |

### 16.2 各调用的隐藏机制

#### `borrow()` / `borrow_mut()`

两个来源，两套隐藏机制：

```rust
// 来源 1：类字段访问（java_class! 宏已处理）
// 宏展开前（java_class! 块内，读者可见，1:1 Java）：
self.size += 1;
// 宏展开后（不可见）：
self.__set_size(self.__get_size().wrapping_add(1i32));
// __set_size 内部有 borrow_mut()，但在宏展开产物里，不在读者可见层
```

```rust
// 来源 2：数组元素访问（Array<T> 处理，待实现）
// 目标（java_class! 块内）：
arr[5i32] = 42i32;        // 宏重写 → arr.set(5i32, 42i32)
let x = arr[5i32];        // 宏重写 → arr.get(5i32)
// Array::set/get 内部有 borrow_mut()/borrow()，但不可见
```

#### `downcast::<T>()`

Java 强制转型 `(T) obj` 目前生成 `.downcast::<T>()`，应由 `java_class!` 宏生成 `From<Object>` impl 隐藏：

```java
// Java
Animal a = (Animal) obj;
```
```rust
// 当前（禁止）
let a: Animal = obj.downcast::<Animal>();

// 目标：downcast 隐藏在 From<Object> 实现里
let a: Animal = obj.into();  // 或 Animal::from(obj)
```

`java_class!` 宏为每个类自动生成：
```rust
// 宏展开产物（不可见）
impl From<Object> for Animal {
    fn from(o: Object) -> Self {
        o.downcast::<Animal>()   // downcast 在这里，读者看不到
    }
}
```

codegen 侧：`checkcast Animal` 指令 → 生成 `obj.into()` 或 `Animal::from(obj)`，而非 `obj.downcast::<Animal>()`。

#### `Object::from_any(v)`

隐式向上转型（子类 → Object），由 blanket `From<T> for Object`（R-1，已完成）处理：

```java
// Java（隐式）
Object o = dog;
```
```rust
// 当前（禁止）
let o: Object = Object::from_any(dog.clone());

// 目标
let o: Object = dog.into();   // blanket From<T> for Object，已可用
```

codegen 侧：`aastore` / `astore` 赋值给 Object 类型变量时，生成 `.into()` 而非 `Object::from_any(...)`。

### 16.3 当前状态

| 调用 | 状态 |
|------|------|
| `borrow()` / `borrow_mut()`（字段） | ✅ 已隐藏（`java_class!` 宏字段重写） |
| `borrow()` / `borrow_mut()`（数组） | ⚠️ 待修复（`Array<T>` Step 2/3，见 §7） |
| `downcast::<T>()` | ⚠️ 待修复（需 codegen 改 `checkcast` 生成 + 宏生成 `From<Object>`） |
| `Object::from_any(v)` | ⚠️ 待修复（codegen 需改用 `.into()`，blanket impl 已就绪） |
| `Rc<RefCell<Vec<T>>>` 类型标注 | ⚠️ 待修复（`Array<T>` Step 1，见 §7） |

---

## 附：实现细节索引

| 机制 | 实现位置 |
|------|---------|
| `java_class!` 宏展开 | `runtime/java_rta_macros/src/block.rs` |
| 字段 RefCell/Cell 包裹 | `block.rs` §字段存储类型 |
| per-class vtable trait 生成（规划中） | `block.rs` + `codegen/method/codegen.py` |
| Deref 继承链 | `block.rs` + R-2 |
| blanket `Into<Object>` | `runtime/java_runtime/src/lib.rs` |
| `impl From<Object> for T`（规划中） | `block.rs` 宏生成 |
| `Array<T>` newtype（规划中） | `runtime/java_runtime/src/java/lang/array.rs` |
| 字节码指令 → Rust 语句 | `codegen/instr/sim.py` |
| 虚方法调用生成 | `codegen/instr/invoke.py` |
| 类型映射 JVM → Rust | `codegen/type_map.py` |
| native 方法覆盖扫描 | `codegen/emitter/class_writer.py` `_scan_impl_files` |
