# 宏家族设计：完整 Java 语言构造封装体系

> 创建日期：2026-09-18  
> 关联文档：
> - [`java-rust-translation-reference.md`](java-rust-translation-reference.md) — Java→Rust 转译对照规则（§16 可读层禁用调用列表）
> - [`2026-09-18-product-vision.md`](2026-09-18-product-vision.md) — 产品定位：Java 语言的新编译后端
> - [`2026-09-17-java-rust-type-1to1.md`](2026-09-17-java-rust-type-1to1.md) — 类型 1:1 任务列表

---

## 设计原则

**一个宏/类型 = 一个 Java 语言构造。名称直接来自 Java 关键字。**

判断标准（来自 `java-rust-translation-reference.md §16`）：  
在生成的可读层（`java_class!` 宏块内部、方法体），以下调用**一律不得出现**：

| 禁止出现的调用 | 应封装在 |
|--------------|---------|
| `borrow()` / `borrow_mut()` | `Array<T>` 类型方法 |
| `downcast::<T>()` | `impl From<Object> for T` |
| `Object::from_any(v)` | blanket `impl From<T> for Object` (R-1) |
| `Rc<RefCell<Vec<T>>>` 类型标注 | `Array<T>` 类型别名 |
| `__into_super()` 链调用 | `Deref<Target=Parent>` coercion |
| `ObjectVTable` trait 名（外部可见） | 宏内部展开细节 |

---

## 宏家族分层结构

### 第一层：类定义宏

#### `java_class!`（已存在，需扩展）

**职责**：封装单个 Java 类的全部 Rust 复杂度。

当前实现：
- 生成 inner struct
- 生成字段 getter/setter
- 实现 `ObjectVTable` trait
- 方法签名翻译
- `VisitMut` 改写方法体（`Box<RefCell<T>>` → `RefCell<Option<Box<T>>>`）

**需要扩展**：
- 生成 per-class vtable trait（`ClassName__VTable: ObjectVTable`）
- 将类自身包装为 `Rc<dyn ClassName__VTable>` 而非 `Rc<RefCell<inner>>`
- 生成 `impl From<Object> for ClassName`（隐藏 `downcast`）
- 生成 `impl Deref for Child { type Target = Parent }`（替代 `__into_super()`）
- `VisitMut` 扩展：将 `arr[i]` 语法改写为 `arr.get(i)` / `arr.set(i, v)`

**典型 codegen 输出**：
```rust
java_class! {
    #[superclass(Animal)]
    #[interfaces(Runnable, Comparable<Dog>)]
    pub struct Dog {
        name: String,
        age: i32,
    }

    impl Dog {
        #[java_override]
        pub fn speak(&self) -> Result<()> {
            println!("Woof! I am {}", self.name)?
        }
    }
}
```

**宏展开后（不可见层）**：
```rust
// per-class vtable trait
trait Dog__VTable: Animal__VTable { ... }

// 类型包装
pub struct Dog(Rc<dyn Dog__VTable>);

// From/Into 链（隐藏 downcast）
impl From<Object> for Dog { fn from(o: Object) -> Dog { o.downcast::<Dog__VTable>().into() } }
impl Deref for Dog { type Target = Animal; ... }
```

---

#### `java_interface!`（待实现）

**职责**：封装 Java `interface`，生成 Rust trait 定义。

与 `java_class!` 的关键差异：
- 无字段，无构造器
- 生成一个 `InterfaceName__Trait: ObjectVTable` Rust trait
- 所有 `default` 方法生成 trait 默认实现
- `abstract` 方法生成无体方法签名

**典型 codegen 输出**：
```rust
java_interface! {
    pub trait Comparable<T> {
        fn compareTo(&self, other: T) -> Result<i32>;
    }
}
```

**宏展开后**：
```rust
pub trait Comparable__Trait: ObjectVTable {
    fn compareTo(&self, other: Object) -> Result<i32>;  // T 擦除为 Object（Arch-1）
}
pub type Comparable = Rc<dyn Comparable__Trait>;
```

---

#### `java_enum!`（待实现）

**职责**：封装 Java `enum`，支持带方法和字段的枚举。

Java `enum` 特征：
- 枚举常量是该类型的实例（不是纯整数）
- 可以有字段、构造器、方法
- 可以 `implements Interface`

**典型 codegen 输出**：
```rust
java_enum! {
    pub enum Direction {
        NORTH, SOUTH, EAST, WEST;

        pub fn opposite(&self) -> Direction {
            match self {
                Direction::NORTH => Direction::SOUTH,
                Direction::SOUTH => Direction::NORTH,
                Direction::EAST => Direction::WEST,
                Direction::WEST => Direction::EAST,
            }
        }
    }
}
```

**宏展开后**：
- 生成 Rust `enum` + `impl`
- 自动实现 `ObjectVTable`（`hashCode`/`equals`/`toString`/`getClass`）
- 实现 `ordinal()` / `name()` 等标准 enum 方法

---

### 第二层：语句级宏

#### `java_try!`（待实现）

**职责**：封装 Java `try-catch-finally` 语义。

Java `try` 与 Rust `?` 的差异：
- Java 可以捕获特定异常类型（多态）
- Java `finally` 无论异常与否都执行
- Java 可以多个 `catch` 块

**典型 codegen 输出**：
```rust
java_try! {
    try {
        risky_operation()?;
    } catch (IOException e) {
        handle_io_error(e)?;
    } catch (RuntimeException e) {
        handle_runtime(e)?;
    } finally {
        cleanup()?;
    }
}
```

**宏展开后**：
```rust
let _result = (|| -> Result<()> {
    risky_operation()?;
    Ok(())
})();
let _result = match _result {
    Err(e) if e.is_instance_of::<IOException>() => {
        let e = e.downcast::<IOException>();
        handle_io_error(e)?;
        Ok(())
    },
    Err(e) if e.is_instance_of::<RuntimeException>() => { ... },
    other => other,
};
cleanup()?;  // finally 块
_result?
```

---

#### `java_switch!`（待实现）

**职责**：封装 Java `switch` 语义（传统 statement + 现代 expression）。

翻译目标：
- `switch` statement → Rust `match` + fallthrough 模拟
- `switch` expression（Java 14+，`->`语法）→ Rust `match` 表达式
- `tableswitch` / `lookupswitch` JVM 字节码均由此宏覆盖

**典型 codegen 输出（expression 形式）**：
```rust
let result = java_switch! { value =>
    1 => "one",
    2 | 3 => "two or three",
    _ => "other",
};
```

**典型 codegen 输出（String switch）**：
```rust
java_switch! { s =>
    "hello" => { greet()? }
    "bye" => { farewell()? }
    _ => { unknown()? }
}
// 宏展开：调用 s.hashCode()，再 equals 比较（Java String switch 语义）
```

---

#### `java_cast!`（待实现）

**职责**：封装 Java `(Type) expr` 强转和 `instanceof` 模式匹配。

**典型 codegen 输出**：
```rust
// (Dog) animal — 强转
let dog: Dog = java_cast!(animal, Dog);

// if (animal instanceof Dog d) — 模式匹配（Java 16+ pattern）
if java_cast!(animal, Dog d) {
    d.bark()?;
}
```

**宏展开后**：
```rust
// 强转
let dog: Dog = animal.into();  // 依赖 impl From<Object> for Dog

// 模式匹配
if let Ok(d) = animal.downcast::<Dog__VTable>() {
    let d = Dog(d);
    d.bark()?;
}
```

**说明**：`java_cast!` 是过渡方案，最终目标是 codegen 直接生成 `animal.into()` / `if let`，不需要宏。

---

### 第三层：方法属性宏

#### `#[java_virtual]`（待实现）

**职责**：标记可被子类覆盖的虚方法。

```rust
impl Animal {
    #[java_virtual]
    pub fn speak(&self) -> Result<()> {
        println!("...")?
    }
}
```

**作用**：
- 将该方法加入 `Animal__VTable` trait 定义
- 在 `java_class!` 展开时，为 `Animal(Rc<dyn Animal__VTable>)` 生成对应的委托调用

---

#### `#[java_override]`（待实现）

**职责**：标记覆盖父类方法的实现。

```rust
impl Dog {
    #[java_override]
    pub fn speak(&self) -> Result<()> {
        println!("Woof!")?
    }
}
```

**作用**：
- 验证父类中确实存在同名 `#[java_virtual]` 方法（编译时检查）
- 在 `Dog__VTable` 的 `impl Animal__VTable for Dog__inner` 中使用此实现

---

#### `#[java_synchronized]`（待实现）

**职责**：封装 Java `synchronized` 方法语义。

```rust
impl Counter {
    #[java_synchronized]
    pub fn increment(&self) -> Result<()> {
        self.count += 1
    }
}
```

**宏展开后**：
```rust
pub fn increment(&self) -> Result<()> {
    let _guard = self.__monitor.lock()?;
    self.count += 1
}
```

---

### 第四层：类型封装

#### `Array<T>`（待实现）

**职责**：将 `Rc<RefCell<Vec<T>>>` 完整封装，在可读层提供数组下标语法。

完整设计见 `java-rust-translation-reference.md §7`。

**实现步骤**：

1. **Step 1**：在 `java_runtime/src/java/lang/array.rs` 定义 `Array<T>`：
   ```rust
   pub struct Array<T>(Rc<RefCell<Vec<T>>>);
   impl<T: Clone + Default> Array<T> {
       pub fn new(len: i32) -> Self { ... }
       pub fn get(&self, i: i32) -> T { ... }
       pub fn set(&self, i: i32, v: T) { ... }
       pub fn len(&self) -> i32 { ... }
   }
   impl<T> Index<i32> for Array<T> { ... }  // 支持 arr[i] 语法
   impl<T> IndexMut<i32> for Array<T> { ... }
   ```

2. **Step 2**：codegen 将 `newarray` / `anewarray` 指令改为 `Array::new(len)`，将数组类型改为 `Array<T>` 而非 `Rc<RefCell<Vec<T>>>`。

3. **Step 3**：`java_class!` VisitMut 识别数组下标语法 `arr[i] = v` → 调用 `arr.set(i, v)`（若 `borrow_mut()` 出现则改写）。

---

#### `JvmResult<T>` / `Result<T>`（已存在，无需新增）

当前 `Result<T>` 即为 `JvmResult<T>` 的别名，已由 `java_runtime` 提供。方法签名中的 `-> Result<T>` 对应 Java 的受检异常语义。

---

## 完整宏责任表

| 宏 / 类型 | Java 构造 | 封装的 Rust 复杂度 | 实现状态 |
|---------|---------|-----------------|---------|
| `java_class!` | `class` | 所有权、vtable、字段访问、Deref 链 | 部分实现，需扩展 |
| `java_interface!` | `interface` | trait 定义、类型擦除 | 待实现 |
| `java_enum!` | `enum` | 枚举实例化、方法实现 | 待实现 |
| `java_try!` | `try-catch-finally` | 异常路由、finally 保证 | 待实现 |
| `java_switch!` | `switch` | tableswitch/lookupswitch/String switch | 待实现 |
| `java_cast!` | `(T) expr` / `instanceof` | downcast、类型匹配 | 待实现（过渡方案） |
| `#[java_virtual]` | 虚方法 | vtable 注册 | 待实现 |
| `#[java_override]` | `@Override` | vtable 覆盖 + 编译时验证 | 待实现 |
| `#[java_synchronized]` | `synchronized` | Monitor 锁获取/释放 | 待实现 |
| `Array<T>` | `T[]` | `Rc<RefCell<Vec<T>>>` + borrow | 待实现 |

---

## 实现顺序建议

按对当前测试通过率的影响排序：

1. **per-class vtable trait**（修复 TestInheritance 多态失败）
   - `java_class!` 扩展：生成 `ClassName__VTable` trait
   - `#[java_virtual]` + `#[java_override]` 属性
   - 预计修复：TestInheritance、TestCasting 运行阶段失败

2. **`Array<T>` newtype**（清除可读层 `borrow()` 调用）
   - Step 1-3 见上方
   - 预计修复：无新 e2e，但提升代码可读性

3. **`impl From<Object> for T`**（清除可读层 `downcast` 调用）
   - `java_class!` 扩展：为每个类生成 `From<Object>` impl
   - codegen 将 `checkcast T` 改写为 `.into()`

4. **`java_try!`**（解锁异常处理测试）
   - 需先解析异常表（`Code` 属性中的 exception_table）
   - 预计修复：TestExceptions、TestTryResources

5. **`java_interface!` / `java_enum!`**（完整类型体系）

6. **`java_switch!`**（解锁 switch 测试）

7. **`#[java_synchronized]`**（最后，影响最小）
