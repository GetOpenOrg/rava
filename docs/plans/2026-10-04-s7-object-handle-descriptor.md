# S7：统一对象句柄 + 每类静态描述符（方案，2026-10-04）

> 拆 crate 线（`docs/plans/2026-10-01-rustc-memory-and-crate-split.md` §7.5.4）的结构性终态项。本文只是方案，**未实施**；实施须经主会话确认。
> 测量数据见上述计划 §7.7「2026-10-04 复测」。

## 一、问题

声明 crate（`java_runtime`）的 rustc 峰值随类数增长，而且增长快于线性：

- HelloWorld 有 467 个类，峰值 1.8–1.9 GB；
- TestSerialDefaultSuid 等 3 例有 3400–3900 个类：TestSerialDefaultSuid（3432 类）在 db105d7c 上峰值 11.75 GB，刚好压线；TestJndiNoProvider（3876 类）峰值 11.94 GB，超过单测试 cgroup 上限 11.88 GB，被 OOM 杀掉（§7.7「2026-10-04 复测」）。

根因已在 §7.5.4 写明：**每个 Java 类都有自己的一套 Rust 类型，凡是对这些类型的操作，宏都要按类各展开一份，rustc 也要按类各类型检查、单态化一份。**

HelloWorld 声明 crate 展开后 19.29 MB（#6 之后）。其中逻辑与具体类无关、只是因为签名里写着 `X` 才按类展开的「每类基础设施」如下：

| 项 | MB | 每类条目 |
|---|---:|---|
| `impl ObjectVTable for X`（`__view_into` / `__erased_vtable` / `__view_as` / `__shallow_copy` / `__erased_inner` / `__unsafe_*` / `__interface` / `__proxy_invoke` 与转交方法） | 2.95 | 1 个 impl，约 20 个方法 |
| `From` impl（祖先上转、`From<Object>`、`From<X> for Object`） | 0.98 | 每类 2–3 个以上 |
| `__class_init` | 0.48 | 1 |
| `__virtual_view` / `__from_parts` | 0.46 | 2 |
| 字段访问器 `__get_*` / `__set_*` | 0.45 | 每个自有字段与继承字段各 2 个 |
| `PartialEq` / `Clone` / `Debug` / `Default` | 0.56 | 4 个 impl |
| static 存储与类初始化状态 | 0.53 | 每个 static 1 个，另加 1 个 |
| 泛型类固有的 `__view_into` / `__erased_vtable` 等 | 0.23 | — |
| **合计** | **约 6.6（34%）** | |

std 泛型也按类各实例化一份：`__Shared<dyn X__VTable>` 的 new、drop 与 `drop_slow`，`Option<__Shared<dyn X__VTable>>` 视图槽的 `downcast_mut`，`Box<dyn Any>` 的 new 与 downcast。按 Digester 的 size_est 计约 25 万，占单态化总量的 21%。

## 二、现状形态

```rust
pub struct X<G..> {
    pub vtable: __Shared<dyn X__VTable>,   // 按类不同的 trait 对象类型
    pub any: __AnyRef,                     // 存储（X__inner）的类型擦除引用
    pub _jvm_null: bool,
    _p: PhantomData<G..>,
}
pub struct Object(pub Rc<dyn ObjectVTable>);   // 装入 Object 时，Rc 里放的是 wrapper 本身
```

- **null**：`Default` 也会经存储钩子 `alloc` 分配一份 `X__inner`，再把 `_jvm_null` 置为 true。
- **视图与上转**（`From<X> for Anc`、`__view_into`）：按部件重建祖先 wrapper，即同一 vtable 对象上转为 `dyn Anc__VTable`，`any` 相同。
- **虚分派**：`X__VTable::m(&*self.vtable, ..)`，O(1)。

## 三、终态形态

### 3.1 句柄

```rust
/// 所有 Java 引用的统一表示（Object 的内部表示，不是新 trait）
#[derive(Clone)]
pub struct __Handle {
    obj: Option<__Shared<dyn ObjectVTable>>,   // None = Java null，不分配
}

#[repr(transparent)]
pub struct X<G..> { h: __Handle, _p: PhantomData<G..> }   // 每类只剩这一行 struct
```

有两种取法：

- **A**：句柄只存 `obj`。虚分派入口经 `ObjectVTable` 上一个按类槽位返回 `&dyn X__VTable`，例如 `fn __vt(&self, class: ClassId) -> *const ()` 再转回胖指针。每次分派多一次间接调用加一次类 id 比较。
- **B（推荐）**：句柄 = `obj` 加一个非持有的类视图指针 `Option<NonNull<dyn X__VTable>>`，在构造或视图转换时求出一次。分派仍是 O(1) 的一次间接调用，与现状相同，没有引用计数。

  这个指针不能放进与类无关的 `__Handle` 里（它的类型是 `dyn X__VTable`），所以 wrapper 实际是 `{ h: __Handle, vt: NonNull<dyn X__VTable> }`：

  - wrapper 仍按类各有一个类型。
  - **但** `__Shared<dyn X__VTable>` 不再存在。按类的 Arc new / drop / `drop_slow` 实例全部消失，只剩 `__Shared<dyn ObjectVTable>` 一份。
  - wrapper 的 Clone / Drop 只作用于 `__Handle`，不按类单态化。

  B 保留了现状的分派代价，又消除了 std 按类实例。推荐 B。

`any`（存储的类型擦除引用）并入 `obj`：现状里 `vtable` 与 `any` 指向同一个对象的两个视角。终态由 `ObjectVTable::as_any` 取得存储，不再单独持有一份 Rc。

**null**：`obj = None`，不再分配存储。

- 这样修正了现状「null 也分配一份 `X__inner`」的开销。
- 等价性：可观察的只有 null 判定、`__class_name`（TypedNull 语义）与身份。
- `vt` 对 null 取该类的静态「null 视图」，即每类一份 `static` 零尺寸实现。它的方法体统一返回 NPE，保持现有入口检查 `if self._jvm_null` 的语义。所有入口检查都改为 `self.h.obj.is_none()`。

### 3.2 每类静态描述符

```rust
pub struct ClassDesc {
    pub binary_name: &'static str,
    pub super_: Option<&'static ClassDesc>,
    pub interfaces: &'static [&'static ClassDesc],
    pub depth: u16,                                  // 祖先显示表：display[depth] == self
    pub display: &'static [&'static ClassDesc],      // 自根至本类的类祖先链（O(1) 子类型判定）
    pub upcast: &'static [fn(NonNull<()>) -> NonNull<()>],   // 与 display 对齐：本类 vtable 指针 → 祖先 vtable 指针
    pub iface_carriers: &'static [(&'static ClassDesc, fn(Object) -> Box<dyn Any>)],
    pub fields: &'static [FieldDesc],                // 反射字段、浅拷贝、Unsafe 槽位（名、类型标签、cell 偏移）
    pub alloc: fn() -> __Shared<dyn ObjectVTable>,   // new / Default（非 null）/ 浅拷贝目标
    pub clinit: &'static __ClassInitCell,            // 类初始化状态机
}
```

- 宏按类只生成：一个 `static X__DESC: ClassDesc`（数据），以及 `impl __Described for X { const DESC: &ClassDesc = &X__DESC; }` 这一行。
  - `__Described` 是 runtime 内部的实现细节，与 `ObjectVTable` 同为 `object.rs` 的私有基础设施，**不是 Java 命名空间之外的公开 trait**。命名原则见 CLAUDE.md「代码生成命名原则」。
  - 也可以不引入 trait，改为在 wrapper 上生成一个 `const DESC`。择定放到实施 S7-1。
- runtime 手写一份**非泛型**实现，读描述符完成：
  - `instanceof` / `checkcast`：按 display 表 O(1) 判定类祖先，接口查 `interfaces` 闭包；
  - 视图（`__view_into` / `__view_as` / `__erased_vtable` / `__virtual_view`）；
  - 浅拷贝（`Object.clone`，按 `fields` 逐 cell 复制）；
  - 反射字段与 Unsafe 槽位（`__reflect_field` / `__field_slot` / `__unsafe_*`）；
  - 类初始化状态机（`__class_init` 的泛型骨架加一个按类的 `<clinit>` 函数指针）；
  - `PartialEq`（句柄身份）、`Debug`、`Default`（null 句柄）、`Clone`（句柄克隆）。
- 槽式视图（`__view_into(slot: &mut dyn Any)`）保留。槽类型两两不同，这条论证与 V1（§7.5.3）相同。命中判定改为「槽的 `TypeId` → 描述符」，表在 runtime 侧按首次查询登记，祖先填值统一走 `upcast`。
  - 每类剩下的代码只有：泛型 wrapper 的 `From<Object>`（类型实参不同，类型就不同，必须按实例化各一份）。它化为一行 `__view::<Self>(obj, Self::DESC)`，按 T 单态化的只是这一行。

### 3.3 可读层不变

方法体中的调用写法不变：`animal.speak()`、`Dog::new()`、`(Animal) obj` 对应的 `Animal::from(obj)`。

- 句柄、描述符、`vt` 都是 wrapper 的私有部件，可读层看不到。
- 禁止列表（`java-rust-translation-reference.md §16`）里的调用不会因为 S7 出现在方法体中。

## 四、需先论证的事项（§7.5.4 遗留）

### 4.1 手写 vtable 实现者如何取得描述符

**现状（2026-10-04 实查）**：`runtime/` 下已经没有任何手写的 `impl X__VTable for`，只有 `object.rs` 里非类对象的 `impl ObjectVTable`。

- 当初列这一项时担心的 locale provider，现在走字节码翻译加生成器。
- 非类对象包括：基本类型盒、`()`、`TypedNull`、数组 `Rc<__RefSlot<Vec<T>>>`、`JvmRef<T>`，以及 lambda / 闭包的 `__DynFn` 载体。

所以描述符只需覆盖 `java_class!` 生成的类。非类对象定义 `ObjectVTable::__desc() -> Option<&'static ClassDesc>`，返回 None，保持现有缺省行为：不应答视图、`__class_name` 报自身，基本类型盒报包装类名。

- 实施 S7-0 先加一个守护：raw-audit 计数手写 `impl .*__VTable for`，终态为 0，新增即报错。
- 将来若出现准入的手写类，例如 VM 注入类，由清单声明，生成器为它发射描述符。手写只写方法体，与 CLAUDE.md 手写边界 §1 一致：struct 一律由生成器生成。

### 4.2 `JArray` 协变视图与描述符

`JArray<T>` 不是 `java_class!` 类。它的协变视图（`Repr::Covariant`）以 `Object` 为源，读写时按目标元素类型重建 wrapper，即 `T: From<Object>`；aastore 检查走 `__view_into` 填槽，再走 `is_instance_of`。

S7 下：

- **元素读**：`T::from(obj)`，即一行 `__view::<T>(obj, T::DESC)`，语义不变。
- **aastore 检查**：两步（填槽、按名判定）合并为一次描述符子类型判定，即 `elem_desc` 的 display / interfaces 判定。这比现状少一次槽试填，而且结果相同：现状的两步就是同一祖先关系的两种查法。

  元素类型是数组时（多维数组），仍委托源数组判定，这一点不变。
- **数组类本身没有描述符**：数组类名按元素描述符拼接，与现状的 `TypedNull` 元素探针取元素类名相同。数组的 `getClass` / `instanceof` 继续走 `JArray` 自己的实现。

因此 `JArray` 不需要改形态，只把两处调用点换成描述符入口。

### 4.3 虚分派从句柄取 `&dyn X__VTable` 的开销

- **方案 B 下分派开销与现状相同**：一次 vtable 间接调用，不增减引用计数。
- **方案 A 每次分派多一次 `ObjectVTable` 槽查找**（间接调用加比较），所以 A 只作为 B 的退路。

性能验收用热循环用例，release 构建，背靠背各 5 次，取中位数，要求退化 ≤ 3%：

- 虚分派密集：`InheritanceChain`、`InterfaceDispatch`；
- 集合：`TestCollections`、`StockTrans`；
- 字符串拼接：`HelloWorld` 循环版，现有 `perf` 语料中取。

视图转换（checkcast / 上转）在 B 下需要经 `upcast` 求一次 `vt`，代价是一次函数指针调用。现状是 Arc 克隆加 trait 上转，S7 只少不多。

## 五、等价性

1. **对象身份**：现状身份是 `vtable.__identity()`，即同一对象的指针。S7 中是 `obj` 的指针，与 vtable 对象是同一个分配，身份不变。
2. **祖先视图的部件**：现状的上转结果是「同一 vtable 对象、同一 any、同一 null 标志」。S7 中是同一个 `obj`，`vt` 经 `upcast` 指向同一具体对象的祖先 vtable。分派到同一实现，结论与 V1 论证相同。
3. **null**：现状的 null 是「分配了存储、`_jvm_null = true`」。所有可观察路径都先检查 `_jvm_null`：入口 NPE、`PartialEq` null 短路、`is_jvm_null`、TypedNull 的类名。存储从不被读。所以「不分配」与现状不可区分。

   例外：null wrapper 装入 Object 后，`__class_name` 报静态类。S7 中 `Object::from(null X)` 产出 `__typed_null(X::DESC.binary_name)`，与现有接口载体的 TypedNull 同形。
4. **视图槽**：槽类型两两不同，任一查询至多一个命中，结果只取决于「槽类型 → 填入值」映射。描述符表是这个映射的数据形态，与 V1 的链函数等价。
5. **类初始化**：状态机与 `<clinit>` 调用次序不变，只是骨架从按类展开变为一份泛型代码。按类的只有 `<clinit>` 函数指针与状态 cell。

## 六、量化目标

| | 现状（#6 后） | S7 终态 |
|---|---:|---:|
| HelloWorld 声明 crate 展开体量 | 19.29 MB | ≤ 13.5 MB（每类基础设施 6.6 → ≤ 1.0 MB） |
| HelloWorld 声明 crate 峰值（Linux） | 约 1.85 GB | ≤ 1.3 GB |
| 3 个 OOM 例（3400–3900 类）声明 crate 峰值 | 10.8–11.9 GB（TestJndiNoProvider OOM） | ≤ 6 GB（详见 §7.7 大例构成，按实测换算） |
| TestSerialDefaultSuid 声明 crate 展开体量 | 148.74 MB | ≤ 100 MB（ObjectVTable 26.0 + From 8.1 + 视图 / 访问器 / 类初始化约 11 → ≤ 6） |
| std 按类实例（Arc / Weak / downcast / `Box<dyn Any>`） | 每类约 10 个 | 0（常数个） |
| null wrapper 的存储分配 | 每个 1 次 | 0 |
| 手写 `impl X__VTable for` | 0 | 0（守护） |
| 虚分派热循环 release 退化 | — | ≤ 3% |

## 七、步骤（每步单独提交、单独实测；宏单测 + 生成器单测 + 真编译抽查）

- **S7-0 守护与描述符生成**：
  - 宏为每类额外生成 `static X__DESC`；runtime 加 `ClassDesc` 类型。
  - 守护：raw-audit 计数手写 `__VTable for`，终态为 0。
  - 这一步只增不减，用于对照。
- **S7-1 子类型判定与视图走描述符**：
  - `is_instance_of` / checkcast / aastore / `__view_as` 改读描述符；
  - 删掉 `impl ObjectVTable for X` 中对应的按类方法；
  - V1 链式委托的等价性论证原样沿用。
- **S7-2 句柄与 null**：
  - wrapper 改为 `{ h, vt }`；`Default` 不再分配；
  - 删掉 `__Shared<dyn X__VTable>`；入口检查改为 `obj.is_none()`。
- **S7-3 浅拷贝 / 反射字段 / Unsafe 槽位走 `fields` 描述**：删掉按类的 `__shallow_copy` / `__reflect_field` / `__unsafe_*`。
- **S7-4 类初始化骨架去按类展开**。
- **S7-5 性能验收**（§4.3）与 3 个 OOM 例、Digester 实测；更新 §7.5.4 账。

每步的抽查用例沿用 §7.5.3 列出的覆盖面（异常、继承与转型、泛型、接口、数组、Unsafe 与 clone），另加 3 个 OOM 例。

## 八、不做

- **本方案不含「Java 方法外壳改成经 `Deref` 继承」**，单列为 S7 之后的候选：
  - 体量不小：TestSerialDefaultSuid 有 19,444 个 `inherited_from`，继承转发外壳（`<Owner as From<Self>>::from(clone(self)).m(..)`）10,009 个、4.56 MB，加上虚外壳 7.7k 个、2.95 MB，合计约 17.7k 个函数、7.5 MB，占展开体量约 5%。HelloWorld 只有 872 个，所以小例上看不出来。
  - 但重载改名（K-6 `vtable_name` / `target`）在 `Deref` 方法解析下会产生歧义；而且只有 S7-2 把句柄统一之后，`Deref` 到祖先才是零成本的指针转换。
  - 所以要等 S7-2 落地后，再单独论证。
- **不把 wrapper 改成纯 `__Handle`（方案 A）作为缺省**：A 有分派开销，见 §4.3。
