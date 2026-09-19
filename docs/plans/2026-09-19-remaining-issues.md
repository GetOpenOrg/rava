# 遗留问题总表（R5 轮集成之后）

> 创建日期：2026-09-19
> 基线：分支 `integrate-tsb` @ `8910fa5`（CFG 结构化重写 + 擦除运行时身份阶段 1 + `<clinit>`/异常对象 + PrintStream 字节码翻译 + R5-A/B/C/D 四个错误族清零）
> 关联计划：[`2026-09-18-erased-runtime-identity.md`](2026-09-18-erased-runtime-identity.md)、[`2026-09-18-cfg-structuring-rewrite.md`](2026-09-18-cfg-structuring-rewrite.md)、[`2026-09-18-clinit-and-athrow.md`](2026-09-18-clinit-and-athrow.md)、[`2026-09-18-printstream-bytecode.md`](2026-09-18-printstream-bytecode.md)、[`2026-09-15-e2e-unresolved-issues.md`](2026-09-15-e2e-unresolved-issues.md)、[`java-rust-translation-reference.md`](java-rust-translation-reference.md)

本文记录当前已知的**全部**遗留问题：架构缺口、JVM 语义缺口、生成器内部质量、项目原则违规、验证覆盖缺口、仓库事务。每一条给出现状、根因、终态目标（量化）。所有条目的解法只允许落在生成器（`codegen/`）、宏（`runtime/java_rta_macros/`）、手写层（`runtime/java_runtime/` 的 native `*_impl.rs` 与内部边界类）；禁止修改 `build/` 下的生成文件。

---

## 0 当前已验证状态

`--clean` 重新生成 + `cargo build --release` + `timeout 30` 运行，于 `8910fa5` 实测：

| 测试 | 编译错误 | `stub_fallback` | `unconsumed` | `handler_methods` | 输出 vs `tests/expected` |
|------|---------|-----------------|--------------|-------------------|--------------------------|
| TestStringBuilder | 0 | 0 | 0 | 0 | 一致（22 行） |
| TestArrayList | 0 | 0 | 0 | 0 | 一致 |
| TestCollections | 0 | 0 | 0 | 0 | 一致 |
| TestExceptions | 0 | 0 | 0 | 0 | 一致 |
| TestStaticInit | 0 | 0 | 0 | 0 | 一致 |
| TestTryShape | 0 | 0 | 0 | 0 | 无期望文件（见 V-2） |

TestStringBuilder 闭包规模：1287 个生成文件、7378 个方法、19598 个跳转、486 个 try 区域。

---

## 问题索引

| 编号 | 类别 | 条目数 | 最高优先级 |
|------|------|--------|-----------|
| [A](#a-架构缺口最高优先级) | 架构缺口 | 7 | P0 |
| [S](#s-jvm-语义缺口) | JVM 语义缺口 | 16 | P1 |
| [G](#g-生成器与宏的内部质量) | 生成器与宏内部质量 | 10 | P1 |
| [P](#p-项目原则违规) | 项目原则违规 | 3 | P1 |
| [V](#v-验证覆盖缺口) | 验证覆盖缺口 | 4 | P1 |
| [R](#r-仓库事务) | 仓库事务 | 3 | P2 |

优先级遵循项目原则：**架构问题优先于 Bug 修复**。A 类先于 S 类，S 类中多数条目在 A-1/A-2 落地后自然消解或变得容易。

---

## A. 架构缺口（最高优先级）

### A-1 存储层擦除未落地：可变泛型类无法跨实例化互转 【P0】

- **现状**：`X__inner<T>`、`X__VTable<T>` 仍带类型形参，Rust 单态化使 `X<Object>` 与 `X<T>` 是两个不相关的类型。R5-B 为「字段全 final 的泛型类」加了 `#[immutable_state]`（`From<Object>` 从擦除字段值重建实例并共享 `__identity`），只覆盖不可变类（如 `(Optional<T>) EMPTY`）。可变泛型类的不同实例化之间不能互转，子类对象不能在另一实例化下重建视图。
- **根因**：Java 泛型是擦除的（运行时只有一个 `X`），Rust 生成代码却按类型实参分裂了运行时身份。
- **终态**（即 `2026-09-18-erased-runtime-identity.md` §6 步骤 1，方案 a）：`X__inner`、`X__VTable` 为非泛型；类型变量字段以 `Object` 存储；wrapper `X<A>` 仅以 `PhantomData<A>` 携带类型实参，是同一 `Rc<X__inner>` 上的类型化视图；`From<Object> for X<A>` 对任意 `A` 成立。
- **验收指标**：`_reinstantiate_generic` 调用点 = 0；`#[immutable_state]` 重建路径 = 0（被通用视图机制取代）；依赖「带类型实参 TypeId」的运行时判定 = 0。
- **规模**：多日级重构，涉及宏 `block/mod.rs` 的 struct/vtable 展开、`coerce.py`、`fields.py`、`invoke*.py`。

### A-2 可读层禁用调用未清零 【P0】

- **现状**（TestStringBuilder 生成代码实测，不含 `*_impl.rs`）：

  | 形态 | 出现次数 | 终态 |
  |------|---------|------|
  | `Object::from_any` | 932 | 0 |
  | `.downcast::<T>()` | 839 | 0 |
  | `downcast_ref` | 283 | 0 |
  | `Rc::new` | 214 | 0 |
  | `.borrow()` | 3 | 0 |

  以上均在 [`java-rust-translation-reference.md §16`](java-rust-translation-reference.md#16-禁止出现在可读层的调用列表) 禁止列表内。`from_any` 的发射点：`_coerce_to_object` 对接口类型的分支、`invoke.py`、`invoke_virtual.py`、`sim/dynamic.py`、`blocks.py:193`。
- **根因与依赖**：依赖 A-1（类 vtable 查询取代 downcast 链）、A-3（checkcast IR 化）、A-4（接口 carrier 进类型位置）、A-5（lambda 对象化）。
- **终态**：上表全部为 0；checkcast 的可读形态为 `<T>::from(obj)` / `Into::<T>::into(obj)`，接口调用为 `it.hasNext()`。

### A-3 checkcast / instanceof 仍是字符串形态 【P0】

- **现状**：R5-B 已把「合法的子类向下转换」改为 `<T>::from(Object)`（`ObjectVTable::__view_into`），R5-C 引入 `Object::checkcast`（复用 `__view_as`）。但 `.downcast::<T>()` 字符串形态仍被 `stack.py`（hint 重定向）、`fields.py`、`invoke_sig.py`、`codegen.py` 等 15+ 处模式匹配依赖；`downcast(&self)` 与 `into(self)` 所有权语义不同。instanceof 在「静态类型是目标祖先」时仍走静态判定而非运行时判定。
- **Codegen 当前错误的三种 checkcast 生成形态**（实测触发，对应 e2e-issues C2 的 6 个用例）：

  | 表达式静态类型 | 目标类型 `T` | 当前生成（错误） | 终态生成（正确） |
  |---|---|---|---|
  | `Result<Object, JvmError>` | `T` | `expr.downcast::<T>()` | `expr?.downcast::<T>()` 或 `T::try_from(expr?)?` |
  | 已知静态类型 == `T`（如 `String` → `String`） | `T` | `expr.downcast::<T>()` | `expr`（no-op，直接省略） |
  | `Object` | `T` | `expr.downcast::<T>()` | `T::try_from(expr)?` / `Object::checkcast::<T>(expr)` |

  第二种情况是「已知源类型 == 目标类型时仍发出 downcast」，在 `String`、`Integer` 等具体类型上报 `no method named downcast found for struct String`，属于 codegen 的冗余转换，优先于 runtime 修改单独修复。

- **协变 upcast 路径缺失**（与 A-4 同源但独立于 lambda）：`List<Object>: From<ArrayList<_>>` 这类报错根因是 codegen 在生成 `java_class!` 宏块时未为每个 `implements` 接口生成 `impl From<ClassName> for InterfaceName`。当泛型参数被擦除为 `Object` 后，`ArrayList<Object>` 与 `List<Object>` 之间缺少 `From` 实现，编译器报 `the trait bound List<Object>: From<ArrayList<Object>> is not satisfied`。终态需在 `class_writer.py` 的 `java_class!` 块生成逻辑里对每个接口发出此 impl（见 A-4）。
- **终态**：rs_ir 增加 `CastExpr` / `InstanceOfExpr` 节点，所有消费方按节点而非字符串匹配；instanceof 全部按擦除类做运行时判定；失败的 checkcast 抛 `ClassCastException`（见 S-1）；协变 upcast 由 `java_class!` 生成的 `From` impl 覆盖。
- **验收指标**：codegen 中对 `downcast` 字符串的模式匹配 = 0；`expect("ClassCastException")` = 0。

### A-4 接口 carrier 未进入类型位置（T-2） 【P1】

- **现状**：接口类型的参数/返回值/局部变量/字段多数仍生成 `Object`，调用点经擦除载体转换。`Constable.describeConstable` 这类「擦除接口签名 vs 类的具体泛型返回」靠 R5-B/C 的局部转换过编译。
- **协变 upcast 子问题**：A-3 中提到的「`List<Object>: From<ArrayList<_>>` 缺失」属于 A-4 的具体落地需求。`java_class!` 宏块生成时，对类声明的每个 `implements` 接口，需同时生成：
  ```rust
  impl From<ClassName> for InterfaceName {
      fn from(v: ClassName) -> Self { /* carrier 包装 */ }
  }
  impl TryFrom<Object> for InterfaceName {
      type Error = JvmError;
      fn try_from(obj: Object) -> Result<Self, JvmError> { /* downcast + wrap */ }
  }
  ```
  这样 `let list: List<String> = ArrayList::new();` 的 upcast 就能走 `From` 而不是 `Object::from_any`，同时 `Function<Object,Object>: From<Object>` 也得到满足（见 A-5 与 S-15 的函数式接口实现）。
- **终态**：接口类型位置一律为 `I<E>` carrier；调用点为 `it.hasNext()` / `it.next()`；协变返回由 vtable 槽位的擦除签名 + carrier 的类型化视图统一处理；`java_class!` 对每个 `implements` 接口生成 `From` + `TryFrom<Object>` impl；接口 upcast 的 `Object::from_any` = 0。

### A-5 lambda 不是对象 【P1】

- **现状**：lambda 以闭包装箱（`Object::from_any` + carrier 中的 `downcast_ref` 回落）；不实现 `I__VTable`，因此 default 方法不能在 lambda 上调用，`__interface` 查询对 lambda 无效。实测影响用例：TestLambda、TestMethodRef、TestOptional、TestArraysUtil、TestPatternMatch、TestStreamBasic、TestStreamAdvanced、TestStreamCollectors、TestStringRegex、TestInterfaceStatic、TestEnumMethods、TestFunctionalInterface（12 个，对应 e2e-issues C1）。
- **设计方向**（在现有 `Rc<dyn ObjectVTable>` 架构上演进，不引入新公开 trait）：

  1. **每个函数式接口独立 struct**，签名对应各接口 arity（不用单一变参 `Lambda` 结构体，以保留类型信息供调用点使用）：
     ```rust
     // runtime/java_runtime/src/java/util/function/function.rs
     #[derive(Clone)]
     pub struct Function(pub Rc<dyn Fn(Object) -> Result<Object>>);
     impl ObjectVTable for Function { ... }
     impl From<Function> for Object { ... }   // 装箱
     impl TryFrom<Object> for Function {       // 解箱（不用 From，保留 ClassCastException 语义）
         type Error = JvmError;
         fn try_from(obj: Object) -> Result<Self, JvmError> { obj.try_checkcast::<Function>() ... }
     }
     ```
     需实现的接口（按 e2e 实测报错）：`Function`、`Supplier`、`Consumer`、`BiConsumer`、`BinaryOperator`、`Predicate`、`UnaryOperator`、`BiFunction`、`Comparator`（及其余 12 个报错接口，随测试扩大按需补充）。

  2. **`invokedynamic` 站点** codegen 生成：
     ```rust
     let f = Function::new(Rc::new(move |arg: Object| -> Result<Object> { /* body */ }));
     let obj: Object = Object::from(f);  // blanket From<T: ObjectVTable>
     ```

  3. **解箱调用点** codegen 生成 `Function::try_from(obj)?` 而非 `obj.downcast_ref::<Fn(...)>()`。

  4. **lambda 方法命名一致性**（见 G-10）：`invokedynamic` 生成的 lambda body 方法名必须与调用点保持一致；两处应在同一 codegen 逻辑中生成，不可各自独立命名。

- **终态**：每个 `invokedynamic` 站点生成实现 `I__VTable` 的合成类（与 JVM 的 LambdaMetafactory 产物同构），捕获变量为字段；`downcast_ref` 回落 = 0；`Object::from_any` 的函数式接口路径 = 0；`TryFrom<Object>` 为 12 个常用接口全覆盖。

### A-6 抽象类、枚举、手写类缺少 `__interface` 【P1】

- **现状**：抽象类跳过接口实现（由具体子类承担）；枚举与手写类（含内部边界类）没有 `__interface`。对这些类的对象做接口查询会得到 `AbstractMethodError`（此前 `FileDescriptor_1` 已踩过一次）。
- **终态**：所有声明了接口的类（抽象、枚举、手写）都经宏获得 `__interface`；手写类通过 `java_class!` 的属性声明接口集合，不手写 vtable 粘合代码。

### A-7 协变返回覆盖未建模为祖先槽位的 override 【P1】

- **现状**：`position(I)ByteBuffer` 这类协变返回覆盖在子类另立同名槽位，与祖先槽位并存。R5-A 用「转发成员按描述符完全限定分派」消除了 E0034，但经祖先类型调用时分派到的仍是祖先槽位的实现，多态语义不完整。桥接方法目前由 R5-C 从桥字节码读取真实目标来解析。
- **终态**：协变覆盖 = 祖先槽位的 override（返回值上转为祖先槽位的擦除返回类型）+ 子类侧的类型化访问器；javac 桥接方法不生成独立槽位。并存槽位数 = 0。

---

## S. JVM 语义缺口

### S-1 失败的 checkcast 不抛 `ClassCastException` 【P1】
`downcast` 失败仍是 `expect("ClassCastException")`（进程 panic，不可被 Java `catch` 捕获）。`JvmError::class_cast` 已就绪。终态：全部失败路径返回 `Err(JvmError::class_cast(..))`，`expect("ClassCastException")` = 0。依赖 A-3。

### S-2 数组无法表示 `null` + 多维数组访问 API 缺失 【P1】

**子问题 1 — null 语义**：`JArray::default()` 是空数组，`null` 数组与空数组不可区分；对 null 数组的访问不抛 NPE。终态：`JArray` 具备 null 状态，`arraylength`/`xaload`/`xastore` 在 null 上抛 `NullPointerException`。

**子问题 2 — 多维数组访问 API 缺失**【已修复，R7 轮，TestMultiArray 零差异通过】：`Object` 上未暴露数组长度与元素访问方法，codegen 生成了 `obj.set(i, v)` / `obj.len()` 等调用，但 `Object` 没有这些方法，报 `no method named set/len found for struct Object`。

根因：多维数组（`int[][]`、`String[][]`）在 runtime 以 `Rc<RefCell<Vec<T>>>` 存储，装进 `Object` 后，调用方无法通过 `Object` 直接访问内部向量。

终态：在 `runtime/java_runtime/src/java/lang/object_ext.rs` 为 `Object` 扩展数组访问 API（**不引入泛型参数，改为按元素类型分支**，原因：codegen 在运行时不知道泛型 T，无法指定类型参数）：
```rust
impl Object {
    pub fn array_length(&self) -> Result<i32>;          // arraylength
    pub fn array_load_object(&self, idx: i32) -> Result<Object>;   // aaload
    pub fn array_store_object(&self, idx: i32, val: Object) -> Result<()>;  // aastore
    pub fn array_load_int(&self, idx: i32) -> Result<i32>;          // iaload
    pub fn array_store_int(&self, idx: i32, val: i32) -> Result<()>; // iastore
    // 同理 long / float / double / byte / char / short
}
```
codegen 的 `xaload`/`xastore`/`arraylength` 指令生成改为调用上述方法；同时 `set`/`get`/`len` 等自造名称不再发出。

### S-3 装箱类型无法表示 `null` + null 引用比较语义错误 【P1】

**子问题 1 — 装箱 null**：`Integer`/`Long` 等被建模为原生值，`Integer x = null`、`Map.get` 未命中返回 null 后拆箱抛 NPE 等语义缺失。终态：装箱类型是真实对象（来自字节码翻译的 `java/lang/Integer`），自动装拆箱即字节码里的 `valueOf`/`intValue` 调用，不做特殊建模。

**子问题 2 — null 引用比较语义错误**【已修复，R7 轮：`PartialEq` null 短路 + `Object::default()` singleton】（触发用例 TestAutoboxing 剩余 diff 为子问题 1 的 `i32` 拆平路径）：
```java
Integer nullable = null;
System.out.println(nullable == null);  // Java 输出 true
```
当前 codegen 生成的 `==` 比较对 null 引用返回 false。根因：`Object` 的 `PartialEq` 通过 `__identity()` 比较两个 `Rc<dyn ObjectVTable>` 的指针，而 `Default::default()`（null）的指针并非固定值——每次调用 `Object::default()` 都新建一个 `Rc::new(())`，导致两个 null 对象的身份不同。

终态：
- null 对象用 **singleton** 实现：`thread_local!` 缓存唯一 null 对象，`Object::default()` 返回该 singleton 的 clone（同一 `Rc` 指针）。
- 或：`PartialEq` 先检测 `is_jvm_null()`，两侧均为 null 则相等，一侧为 null 则不等。
- `nullable == null` 的输出 diff = 0（TestAutoboxing 通过）。

### S-4 数组协变不完整 【P1】
`JArray` 的 `Covariant` 视图只支持上转为 `Object[]`；转为祖先类数组（`Integer[]` → `Number[]`）失败；存入错误元素类型抛 `ClassCastException` 而非 `ArrayStoreException`。终态：任意祖先元素类型的协变视图 + `ArrayStoreException`。依赖 A-1 的类型化视图机制。

### S-5 `getClass()` / 类字面量不可用 【P1】
`getClass()` 返回空 `Class`；基本类型 `Class` 只带名字；类字面量（`Foo.class`）、`desiredAssertionStatus` 缺失；`getClass() == PrintStream.class` 实际是 `null == null`。终态：每个类有唯一 `Class` 对象（按擦除类），`__class_name` 与之打通，类字面量与 `getClass()` 返回同一对象。

### S-6 identity hash 缺失 【P1】
未声明 `hashCode` 的类返回 0（全部哈希冲突，功能正确但退化为链表）；`Object.equals` 里保留了 String 内容比较的捷径；字符串字面量 intern 的同一性未建模。R5-B 已加入各视图共享的 `__identity`。终态：`Object.hashCode` 的 native 实现基于 `__identity` 生成 identity hash；`Object.equals` 为纯引用比较；`ldc` 字符串字面量经 intern 表返回同一对象；捷径 = 0。

### S-7 record 的 `hashCode` 恒为 0 【P1】
record 的 `toString`/`equals` 已由生成器按组件生成，`hashCode` 体仍是 `Ok(0)`。终态：按 `ObjectMethods` 引导方法语义组合各组件的 hash（31 多项式）。

### S-8 `NegativeArraySizeException` 缺失 【P2】
`newarray`/`anewarray`/`multianewarray` 对负长度未抛异常。终态：VM 抛出异常对象，与数组 `get/set` 的 `Result` 机制一致。

### S-9 null 字段访问不抛 NPE 【P2】
null 接收者的方法调用已抛 NPE，`getfield`/`putfield` 尚未。终态：两者一致。

### S-10 类初始化触发点不完整 【P2】
JVMS §5.5 的触发点里，手写 static native 的调用、带 default 方法的接口的初始化尚未触发 `__class_init()`。终态：§5.5 列出的触发点全覆盖。

### S-11 `InternalLock.newLockOrNull` 恒返回 null 【P2】
当前走 synchronized 回落路径，单线程下无差异。终态：随线程/同步模型（`2026-09-14-java-sync-threading.md`）一并实现。

### S-12 非嵌套 try 区域的布局偏差 【P3】
无调试信息（无 LocalVariableTable）时，catch 之后的代码仍留在 catch 体内（语义等价，形状与源码不同）；`try { a(); } catch (E e) { throw ..; } return;`（void）会把 `return` 放进 try 体。终态：不依赖调试信息，由异常表 + 支配关系推断 catch 体终点。

### S-13 不可归约控制流兜底未经实战 【P3】
`cfg/dispatch.py` 的 `loop { match __pc }` 兜底触发 0 次，与 `java_try!` 的组合没有测试覆盖。终态：补单元用例（人工构造不可归约 CFG + 异常表）。

### S-14 跨包同简单名类 【已处理，待观察】
R5-B 让 `short_cls` 对冲突类生成带包限定的 Rust 类型名（`Era` 冲突）。需确认：重载后缀（`mangle_name`）里两个同简单名类是否仍可能撞名（R5-A 提出的理论风险）。终态：后缀冲突时同样采用包限定，冲突数 = 0。

### S-15 enum 支持缺失 【已修复，R7 轮第二波】

> **R7 轮第二波已修复**（301db7d，合入 main）：TestEnumBasic / TestEnumMethods / TestSwitchEnum 全过，附带 TestStringFormat 转胜（printf 链路）。**根因与下文分析不同**：enum 表示层早已走普通类路径（S-16 转发生效），真正卡点是 **BFS 调用链对用户类不可见**——`_collect_method_refs`、`new` 的 RTA 实例化、`_load_class` 只认 JDK 前缀，`TestEnumBasic$Day.name` 的父类链解析（JVMS §5.4.3.3）不发生，`Enum.name/ordinal/toString` 永不入链成 stub。修复后常量/values()/switch-on-enum 全由既有 `<clinit>`+静态字段+CFG 机制承接；`java_enum!` 宏与 enum 特殊路径删除；`Enum.valueOf` 手写（查运行时常量目录）。遗留：泛型上下文的 `Enum::<Object>::valueOf` 命中会 CCE（依赖 A-1，已注释）；接口 default 体落载体的保守过滤边界见报告。

- **现状**（e2e-issues B3，触发用例：TestEnumBasic、TestEnumMethods、TestSwitchEnum）：enum 相关的 codegen 与 runtime 存在大量对齐缺口，实测报错超过 25 处 `mismatched types`，以及 `JvmError::Custom` 不存在、`Result::__get_next` 缺失、`downcast` 缺失等。本条目追踪 enum 支持的完整缺口。
- **已知缺口分类**：

  | 缺口 | 具体报错 | 根因 |
  |---|---|---|
  | enum 常量生成 | `mismatched types` × 25 | codegen 为 enum 常量生成了与 runtime enum 表示不匹配的类型 |
  | `ordinal()` / `name()` | stub panic 或类型错误 | enum 基础方法未在 runtime 或生成器中实现 |
  | `values()` 静态方法 | 编译失败或运行时错误 | enum 的静态初始化逻辑（`<clinit>` 中填充 `$VALUES`）未完整翻译 |
  | `switch` on enum | 编译失败 | `lookupswitch`/`tableswitch` 对 enum ordinal 的映射未建模 |
  | `JvmError::Custom` | `no variant or associated item named Custom found` | codegen 使用了 runtime 不存在的错误变体 |
  | `Result::__get_next` | `no method named __get_next found` | codegen 生成了 runtime 不存在的 API |

- **根因**：Java enum 在 JVM 字节码层面是普通类（继承 `java/lang/Enum`），其常量是该类的 `static final` 字段，`values()` 是编译器插入的静态方法。当前 codegen 在翻译 `ACC_ENUM` 类时走了独立的特殊路径，但该路径与现有 runtime 的 struct/vtable 表示方案不兼容。
- **设计方向**：优先让 enum 类走**普通类翻译路径**（`java_class!` 宏 + struct），以 `static OnceLock<Vec<EnumClass>>` 存储常量数组（`values()`），`ordinal()` 和 `name()` 由生成器从字节码生成，不需要特殊 enum Rust 关键字。这样与继承链（`Enum` 基类）、`ObjectVTable` 体系、接口实现均自然兼容。
- **终态**：TestEnumBasic、TestEnumMethods、TestSwitchEnum 全部通过；enum 常量的 `ordinal()`/`name()`/`equals()` 语义正确；`switch (enumVal)` 生成的 `match` 按 ordinal 分派；`JvmError::Custom` 等自造 API 不出现在 codegen 输出中。

### S-16 继承方法体未注入子类 vtable 槽位 【已修复，R7 轮】

> **R7 轮已修复**（16248cb，合入 main）：继承声明带转发体——常规方法调 owner 的 `__base` 自由函数（与 super 调用同源），手写方法经 `__as_Owner` 钩子执行 `__impl_<m>`；宏为子类 `__inner` 按 `vtable_owner` 填祖先 vtable 槽位。**下文「依赖 A-1」的判断被证伪**：`From<Child> for Parent` 是同一 inner Rc 的 upcast，`Into` 占位形态不可行，base 函数形态不依赖 A-1。TestLinkedList 通过；无体 `inherited_from` 声明 219→0（三树合计）。TestInheritedMethod 推进后阻塞在 A-7（`MyList.get(I)String` 协变 override 不被识别），A-7 落地后复跑。

- **现状**（e2e-issues E2，触发用例：TestLinkedList、TestInheritedMethod）：当一个具体子类继承了祖先类的方法实现（而非 override）时，emitter 将子类的该方法槽位生成为**无体声明**（`inherited_from = AncestorClass`），运行时通过 vtable 虚分派落到的是 abstract 声明而非实际实现。
  
  具体报错：`stub: java/util/AbstractCollection.iterator:()Ljava/util/Iterator;`——分析层已正确找到 `AbstractSequentialList.iterator` 并翻译，但 `LinkedList`（其子类）的 `iterator` 槽位生成为无体声明，调用时 panic 在 stub 上。

- **根因**：`codegen/emitter/` 中处理 `inherited_from` 方法的路径（`class_writer.py` 或 `method_gen.py`）在识别到「子类未 override、方法来自祖先」时，选择了生成无体声明（仅声明 vtable 槽位存在），而不是生成「将调用转发到祖先实现」的方法体。这导致该槽位在子类 vtable 中是一个空洞。

- **与 A-7 的区别**：A-7 处理的是「协变返回 override」（子类 override 了方法，但返回类型是祖先方法返回类型的子类），S-16 处理的是「子类完全不 override、直接继承祖先实现」的场景——后者在 Java 语义中应直接复用祖先的方法体。

- **修复方向**：在 `class_writer.py` / `method_gen.py` 的继承方法生成路径中，当 `inherited_from != None` 且当前类没有字节码实现时，生成转发到祖先实现的方法体：
  ```rust
  fn iterator(&self) -> Result<Iterator> {
      AncestorClass::iterator(self)   // 或等价的 UFCS 调用
  }
  ```
  需确保转发的接收者类型转换合法（子类视图 → 祖先视图）。当前系统已有继承层次的视图转换（通过 `Object::from` / `Into::<Ancestor>::into` 链），对已有字节码的具体子类可先行用 `Into::<AncestorClass>::into(self.clone())` 占位；完整实现（任意子类 → 祖先的视图构造均合法）依赖 **A-1**（`X__inner` 去掉类型形参、存储层擦除落地），在 A-1 之前部分可用。

- **终态**：TestLinkedList、TestInheritedMethod 通过；`AbstractCollection.iterator` stub 不被命中；生成代码中「有 `inherited_from` 但无方法体」的声明 = 0。

---

## G. 生成器与宏的内部质量

### G-1 变量提升是文本级的 【P1】
`codegen/method/vars.py` 基于文本做 `let` 提升。R5-A 把 `_mergedN`/lambda 绑定改为 `LetStmt`，R5-C 加了 `LetStmt.value_ty`，但提升主体仍是文本匹配；曾产生「遮蔽已初始化变量」的语义错误（`concurrent_hash_map_tree_bin.rs`）。终态：提升完全在带类型的 rs_ir 语句上进行，文本匹配 = 0。

### G-2 `Default::default()` 占位 【P1】
生成代码中 `Default::default()` 1509 处（TestStringBuilder），主要是提升后的前置声明初值与 `Self::default()` 构造。前置声明的占位会掩盖「未初始化即使用」，也要求所有类型实现 `Default`。终态：前置声明为无初值的 `let x: T;`（由 Rust 的确定赋值分析验证，与 JVM 校验器同语义），占位初值 = 0；构造路径另计。

### G-3 局部变量槽位分型 【P1】
同一 slot 先后存放不同类型的值时（`readObject0`、`LambdaFormEditor.putInCache`、javac 合成变量），R5-B/C 用「合成变量命名 `local_N`」「未命名槽位合并时类型统一」处理。终态：按活跃区间（def-use 链）拆分变量，每个区间独立命名与分型；与 LVT 的对应只用于取名。

### G-4 生成输出不确定 【P2】
提升出的 `let mut x = ...;` 行顺序在两次运行间会变化（遍历 set/dict 的顺序）。只影响 diff，但妨碍回归比对。终态：同输入两次生成的输出逐字节一致。

### G-5 宏里的遗留代码 【P3】
`block/mod.rs` 有遗留占位 `let _ = (...)`，以及 VirtualDefine 的 base 函数段里一个不可达的存根分支。终态：删除。

### G-6 未使用的导入 【P3】
`invoke_sig.py`、`fields.py` 可能残留未使用的 `_class_type_param_bounds` 导入。终态：删除。

### G-7 手写虚方法体机制需写入参考文档 【P2】
R5-B 新增：`_impl.rs` 定义 `__impl_<m>` 时，codegen 在宏块里生成 `body = "handwritten"` 的无体声明，使其进入 vtable；`native_upcalls.py` 识别 `__impl_` 函数上的 upcall 声明；`#[hash_code_vtable]` / `#[equals_vtable]` 取代 `has_hash_code_method`；`#[immutable_state]`；`#[superclass_reference_fields]`。终态：以上属性全部记入 `java-rust-translation-reference.md` 与 `2026-09-18-macro-family-design.md`。

### G-8 类型变量→上界转换经 `Object` 中转 【P2】
R5-C 为绕开「vtable override 上不允许附加 `where TV: Into<Bound>`」而让类型变量到上界的转换经 `Object` + `checkcast`。A-1 落地后类型变量字段本身就是 `Object` 存储，此转换退化为一次视图构造。终态：随 A-1 收敛，不单独处理。

### G-9 循环体内 if/else 分支被整段丢弃 【已修复，R7 轮】

> **R7 轮（2026-09-19 深夜）已修复**：instanceof 运行时化 + 槽位 widening 合入 main（分支 g9-instanceof-widening，f60dabe）。TestCasting 逐行一致通过、`instanceof_fold` 6→0、putVal 的 4 处 E0308 清零、回归 3/3 PASS。实际根因比下文更进一步：putVal 的 `e` 槽**有** LVT 声明（Node<K,V>）但落入 `hint` 通道而非 `decl_ty`，既有上转规则未触发——修复为补齐 hint 路径的上转规则 + 无 LVT 合成槽取最近公共类祖先（`_common_ref_type_widening`）。遗留：接收者是类型变量（K）的 instanceof 仍按互不为子类型折叠（预存行为）。

> **R6 轮排查结论（2026-09-19）**：根因**不在 CFG 结构化**，而在 `sim/control.py` 的 `instanceof` 静态折叠——接收者静态类型是目标超类（`Animal` 变量 vs `Dog`）时被折叠为编译期 `false`，`if (x instanceof T)` 分支被当死代码消除（TestCasting 的 4 处 instanceof 全中，循环内外皆然）。已交付：
> - `[cfg-audit]` 新增 `instanceof_fold=N` 指标（此前该类静默错误无任何指示器）；
> - 实测验证了运行时化路径（`ObjectVTable::is_instance_of` 按 all_supertypes 匹配 + checkcast 的 `<T>::from` 视图恢复）：TestCasting 全通过；
> - 但运行时化令 HashMap.putVal 的 TreeNode 分支复活，暴露**槽位定型缺陷**（JDK 类无 LVT，同一 slot 兄弟分支赋不同类型时首赋值类型成为声明类型 → E0308，TestCollections/TestArrayList 回归）。已按纪律回退发射，代码注释中保留了恢复步骤。
> - **终态修复 = instanceof 运行时化 + G-3 槽位按公共祖先 widening 一起落地**（与 R5-C 早前的独立试验结论一致）。

- **现状**（e2e-issues B2，触发用例：TestCasting）：CFG 结构化重写在循环体内的条件分支上存在缺陷——`while` 循环体里的 `if/else` 分支会被整段省略，只保留分支内某一个基本块（通常是非条件路径）。实测 TestCasting 有 4 处 `instanceof` 判断，生成代码只剩 2 个，循环内 `if/else` 完全丢失，导致输出 diff。
- **根因**：`codegen/cfg/structuring.py` 在处理循环体内嵌套的条件分支时，支配关系计算或 region 归属判断存在错误，将 if/else 的两个后继合并为同一路径，或将其中一条路径误判为循环出口而跳过。具体触发点需通过 `TestCasting` 的字节码反编译对比 CFG 图确认。
- **影响范围**：不只限于 `instanceof`，任何循环体内包含多出口 if/else 的方法体均可能触发。
- **终态**：`TestCasting` 的生成代码中循环体内分支数与原始字节码一致；`scripts/main.py` 输出的 `[cfg-audit]` 中无 `unconsumed-blocks` 计数（循环内分支丢弃的直接指示器）。
- **调试方法**：`python3 scripts/main.py TestCasting.java --no-run` 后查看生成的 `while` 循环体，与 `javap -c TestCasting` 的跳转表逐条对照，定位第一个被丢弃的 `if` 对应的 CFG 边。

### G-10 lambda 方法命名与调用点不一致 【已修复，R7 轮】

> **R7 轮已修复**（2febbae，合入 main）：命名唯一来源 `lambda_impl_rust_name()` + `LAMBDA_NAME_LEDGER` 生成期断言（名字漂移/定义缺失直接 RuntimeError）。实测根因与下文略有出入：命名并未算错，而是**定义侧从未生成**——接口 default 方法内的 lambda 体是 javac 编译的非 static 私有合成方法，被 class_writer 的「私有合成实例方法非接口契约」分支跳过。现生成到 `java_class!` 块之外的擦除实例化固有 impl（不进接口 vtable）。`no method named lambda_*` 3→0 / 22→0。TestComparator 编译通过后运行期卡在 `AccessController.doPrivileged` 存根（A-5/P-3 链路）；TestFunctionalInterface 剩 1 处 E0308（A-5 范畴）。

- **现状**（e2e-issues B4，触发用例：TestComparator、TestFunctionalInterface）：`invokedynamic` 指令生成的 lambda 实现方法（如 `lambda_thenComparing_36697e65_1`）与 call site 生成的调用名称不匹配，导致 `no method named lambda_andThen_1 found` 等编译错误。与 A-5 的宏观缺陷（lambda 不是对象）属于同一根因链但独立触发：即使 A-5 尚未落地，命名不一致本身就能独立修复。
- **根因**：`invokedynamic` 的 bootstrap 方法分析（`sim/dynamic.py`）与 lambda body 方法生成（`method_gen.py`）在命名方案上各自独立推导，未共享同一命名逻辑。部分 lambda 的哈希后缀或序号由不同代码路径计算，导致生成的方法名与调用点期望的名称不一致。
- **终态**：lambda body 方法名由单一来源（`sim/dynamic.py` 的 bootstrap 分析结果）决定，`method_gen.py` 和调用点 codegen 均从该来源读取，不独立推导；TestComparator、TestFunctionalInterface 编译通过，调用点与定义名一致。
- **优先级说明**：此 bug 独立于 A-5（lambda 对象化）存在，在 A-5 完整落地前可单独修复，消除至少 2 个用例的编译失败。A-5 落地时应将 lambda body 方法纳入合成类的方法生成流程，此时 G-10 的修复成果应被 A-5 的统一命名机制吸收。

---

## P. 项目原则违规

### P-1 Python 中的 JDK 类名字面量 【P1】
违反「Python 代码中不得出现任何 JDK 类名常量」。已知位置：
- `codegen/type_map.py:170` `_CLS_ABBREV`（重载后缀缩写表，含小写 JDK 类名）
- `codegen/type_map.py:427` `_CLASSNAME_MAP`
- `_INTERFACE_IMPLS`
- `codegen/instr/coerce.py` 中本轮之前就存在的类名字面量

终态：上述表全部删除，信息改从 `.class`（描述符、`generic_signature`、继承链、注解属性）动态解析；`grep` JDK 类名字面量命中 = 0。`_CLS_ABBREV` 在 R5-A 取消后缀截断后已基本无存在必要。

### P-2 `runtime.py` / `java_runtime` 中的手写临时实现 【P1】
核心原则 1 要求 `String`/`ArrayList`/`System` 等来自字节码翻译。`string_ext.rs` 等手写扩展、`Object.equals` 的 String 捷径（S-6）仍在。终态：手写代码仅剩 native `*_impl.rs` 与内部边界类两类。

### P-3 内部边界类按需实现的现状清单 【跟踪项】
本轮新增/修改的边界与 native 实现：`unsafe__impl.rs`（`getUnsafe`、`allocateUninitializedArray`）、`arrays_support_impl.rs`（`vectorizedHashCode`、byte `mismatch`）、`StaticProperty.USER_*`、`locale_utils_impl.rs`、`BaseLocale.hashCode/equals`、`InternalLock.newLockOr`、`Class.getPrimitiveClass`、`shared_secrets_impl.rs`、`class_impl.rs`、`throwable_impl.rs`、`stream_encoder_impl.rs`、`utf_8_impl.rs`、`vm_impl.rs`、`blocker_impl.rs`、`file_descriptor_impl.rs`、`file_output_stream_impl.rs`、`thread_impl.rs`。其余方法保持 `panic!("stub: ...")`，随测试覆盖扩大按需补全（符合原则 3b，非缺陷）。

---

## V. 验证覆盖缺口

### V-1 全量 e2e 未运行 【P1】
本轮只验证了 6 个测试。`tests/e2e/` 其余测试在 CFG 重写、擦除阶段 1、`<clinit>`/异常、命名方案变更之后的状态未知。按项目原则，全量运行放在 A 类架构改造之后；但在合入 `main` 后应至少跑一次 `python3 scripts/run_tests.py -j 4` 建立新基线并归档到 `docs/reports/`。

### V-2 TestTryShape 缺期望输出 【P2】
`tests/expected/TestTryShape.txt` 不存在，R5-B/D 用临时文件比对。终态：用 `java` 实跑生成并提交。

### V-3 可读性无自动化检验 【P2】
A-2 的禁用调用计数目前靠手工 `grep`。终态：`scripts/main.py` 在 `[cfg-audit]` 旁输出 `[readability-audit]`（各禁用形态计数），`run_tests.py` 汇总；目标值全 0。

> **R6 轮已交付前半**：`main.py` 现于每次转译后输出 `[readability-audit] from_any=N downcast=N downcast_ref=N rc_new=N borrow=N`（只统计含生成标记的文件，与 A-2 口径一致）。`run_tests.py` 汇总尚未做。

### V-4 单元测试运行器 【P3】
环境未装 pytest，`tests/unit/test_cfg_structuring.py`（17 个用例）用 `python3 -m unittest` 运行。终态：在 CLAUDE.md「常用命令」中写明单元测试命令。

---

## R. 仓库事务

### R-1 `main` 尚未更新 【P1】
`main` 在 `59c3355`，可快进到 `integrate-tsb` @ `8910fa5`。被主工作区未提交的 `docs/reports/jdk-scan-HelloWorld.md` 阻塞（该文件两边都有修改，可能属于另一个会话）。处理方式：在主工作区提交或还原该文件后执行 `git merge --ff-only integrate-tsb`。

### R-2 待清理的 worktree 与分支 【P2，需用户确认】
- 已合入、可删：R5 四个代理（`agent-a7c86c6c…`、`a91c2200…`、`ada7cb15…`、`a8a7063b…`）与上一轮四个代理（`agent-a404cfb…`、`a3e0755…`、`afae540…`、`a78105d…`）的 worktree 和分支。
- 来源不明、未合入：`worktree-agent-a4015d0b860f76189`（`481b73e`「refactor: 常量统一管理…」）。
- 备份：`backup/main-wip-2026-09-18`、`backup/main-wip-2026-09-18-b`、`java_rta_test`。
- `integrate-tsb` worktree：`main` 更新后可删。
- 临时目录：`/tmp/cfg_base_src/`、`/tmp/era_base`、`/tmp/era_*`、`/tmp/r5*`–`/tmp/r8*` 日志。

### R-3 生成的扫描报告被提交 【P3】
`docs/reports/jdk-scan-*.md`、`docs/plans/jdk-scan-*.md` 每次转译都会改写，造成 R-1 这类无意义冲突。终态：扫描报告输出到 `build/<test>/`（gitignore），仓库内只保留人工归档的版本。

---

## 推荐执行顺序（2026-09-19 R6 轮修订）

> **R6 轮记录（本日，已完成）**：R5 集成后的全量基线（32/65）中约 20 个失败是两个基础设施假回归，已修复：
> - **R6-a 陈旧生成文件复活**：复用 scratch 时，上一轮幸存的生成 `.rs`（带 `java_rta_macros::java_class` 标记、本轮未写入）被 mod 树的磁盘扫描重新挂进编译，与手写 companion 撞名（`E0592 getUnsafe` 重复，波及 6 个测试）或污染闭包（`E0433 Class`，波及 14 个测试）。修复：`project_writer._write_jdk_mod_tree` 落盘前清除此类文件（`_WRITTEN_THIS_RUN` 集合区分本轮产物）。
> - **R6-b ldc 类字面量的 `Class` 导入缺失**：`Class::for_class(..)` 发射点（`sim/consts.py`）不在 import 扫描来源里。修复：`class_writer` 指令扫描新增 `class ` 注释分支 → 引用 `java/lang/Class`（常量收敛到 `constants.CLASS_CLASS`）。顺带完成 `Class.isAssignableFrom`（生成器在类字面量处静态推导超类型闭包传入 `for_class`，运行时侧表查询），TestClassLiteral 全通过。
> - **教训（记入验证口径）**：复用 scratch 的测试结果在生成器变更后不可信，milestone 验证一律 `--clean`。

1. **批次 1 — 事实基础（剩余）**：**G-9**（循环内 if/else 整段丢失，TestCasting 静默错误，`cfg-audit` 抓不到——先补 `unconsumed-blocks` 指标再排查）＋ **V-3**（可读性审计行）＋ 用修复后的生成器重跑一次全量 e2e 建立真实基线（R6 后预计显著好于 32/65，多数旧失败是陈旧文件假象）。
2. **批次 2 — 快速增益（不依赖 A 类，可并行）**：**R7 轮第一波已交付（2026-09-19 深夜，四分支合入 main 零冲突）**：G-9 完整修复（instanceof 运行时化 + 槽位 widening，TestCasting 通过）、S-16（TestLinkedList 通过）、S-2.2（TestMultiArray 通过）、S-3.2（null singleton）、G-10（lambda 命名单一来源 + 生成期断言）。合并后定向回归：新通过 TestCasting/TestLinkedList/TestMultiArray；两个失败均为已知边界（TestAutoboxing→S-3.1 的 `i32` 拆平，TestInheritedMethod→A-7 协变 override）。**第二波已交付（R7，2026-09-20 凌晨）**：A-3 部分（checkcast 形态 2 经 getfield 类型实参代入修复归零，TestNestedGeneric 8→5；形态 1 经全库排查确认为 0 生成路径）、S-15（enum 三测试全过 + TestStringFormat 转胜，根因是 BFS 对用户类不可见）。批次 2 完毕，下一批次为 A 类架构主线（A-1 起步）。
3. **批次 3 — 架构主线（串行）**：A-1 存储层擦除 → A-3 完整 IR 化 → A-6 `__interface` 全覆盖 → A-5 lambda 对象化（吸收 G-10；含 A-4 协变 upcast）→ A-7 协变覆盖；以 A-2 计数表全 0 + V-3 审计行统一验收。
4. **批次 3' — 质量重做（与批次 3 并行）**：G-1/G-2/G-3（提升与槽位分型迁到 rs_ir）。G-2 终态会让「未初始化即使用」从静默 `Default::default()` 变编译错误，建议在 A-1 稳定后开启，避免两边同时震荡。
5. **批次 4 — 语义补齐**：随架构落地（S-1、S-4、S-5 剩余、S-6）；独立推进（S-2.1、S-3.1、S-7–S-10）。
6. **批次 5 — 清理收尾**：P-1/P-2 清零；G-4–G-8、V-2/V-4、R-2/R-3。

> 批次 2 各项互相独立，适合 worktree 子代理并行；单项收益见各条目的触发用例。
