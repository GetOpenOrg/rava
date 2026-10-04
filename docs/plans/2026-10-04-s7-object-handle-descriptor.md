# S7：统一对象句柄 + 每类静态描述符（方案，2026-10-04）

> 拆 crate 线（`docs/plans/2026-10-01-rustc-memory-and-crate-split.md` §7.5.4）的结构性终态项。本文只是方案，**未实施**；实施须经主会话确认。
> 测量数据见上述计划 §7.7「2026-10-04 复测」。

> **body 切分轴待按模块重新论证**（2026-10-04 用户决策）：档案 crate 按 JDK jmod 模块名命名（`java.base` → `java_base` 等，从 JDK 模块描述动态取），T1 第 2 步方案（t1-link 分支 `docs/plans/2026-10-04-t1-step2-direct-rustc-link.md`）正在论证「按模块切 crate」作为档案终态分层。`java_body_k` 的按体量均衡切分不再扩展，切分粒度与命名以该方案结论为准。

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

## 九、S7 后 java.base 声明层能否再拆成多个 crate（2026-10-04 论证，只写方案）

背景：档案 crate 按 jmod 模块切分（用户已定），模块间依赖是 DAG，可以直接拆；但模块内部（主要是 java.base）类型互指成环，Rust crate 不能循环依赖。现状下 java.base 声明层只能是一个 crate，是整个构建的峰值下限。本节回答 S7 落地后这个下限能否再降。

### 9.1 S7 后声明之间剩下的跨类引用

| 引用 | 方向 | 成环？ |
|---|---|---|
| 继承链：`X__VTable: Super__VTable`，`__class_init` 先触发父类 | 子 → 父 | 否（继承无环） |
| 接口实现：接口载体视图、`impl Iface__VTable for X__inner`（实现层） | 类 → 接口 | 否 |
| 上转 `From<X> for Anc`、`From<Object> for X` / `From<X> for Object` | 子 → 祖先 / 根 | 否 |
| 静态描述符互指：`super_` / `display` / `upcast` / `interfaces` / `iface_carriers` | 子 → 祖先 / 接口 | 否 |
| 运行时类还原（catch 中间型、`__virtual_view`）：S7 后查描述符数据，不写类型 | — | 否 |
| **方法签名里的具名类型**：可读层方法（含继承转发外壳）与 vtable trait 方法的形参 / 返回值、`__jbm_*` 外部声明 | 任意方向 | **是** |
| **字段与 static 的类型**：访问器 `__get_f() -> T`、继承字段访问器、`static F: T` | 任意方向 | **是** |

S7 去掉的是 wrapper 持有的 `__Shared<dyn X__VTable>` 与每类基础设施代码，**不去掉签名里的具名类型**（§3.3 可读层不变）。所以成环的来源只剩后两行。

**实测**（`scripts/decl_scc.py`，读生成的声明文件建图；模块包清单取自参考 JDK 的 `java --describe-module java.base`，195 个包）：

| 档案 | 类 / 接口 | 现状声明层全部引用：最大 SCC | S7 后（继承 + 签名 / 字段 / static 具名类型）：最大 SCC | 其中 java.base 子图 | 再把 vtable 签名擦除为句柄、只剩继承 / 接口 / 描述符：最大 SCC |
|---|---:|---:|---:|---:|---:|
| HelloWorld | 430 | 326（75%） | 164（38%），次大 6 | 164（java.base 占 100%） | 1 |
| Digester | 2856 | 2027（70%） | 481（16%），次大 22 | 459（java.base 占 90%） | 1 |
| TestSerialDefaultSuid | 3150 | 2268（72%） | 548（17%），次大 22 | 514（java.base 占 90%） | 1 |

- 最大 SCC 的成员按包：`java/lang`、`java/util`、`java/io`、`java/lang/invoke`、`java/security`、`sun/reflect/generics/tree`、`java/util/concurrent` 居前。根是 `Object ↔ String ↔ Class` 这类签名互指，再经 `Class` 的反射 / `invoke` 签名扩散。
- 「现状全部引用」比「S7 签名」多出的边，主要是异常类型（`exceptions` 元数据的导入）与嵌套类 / static 初始化辅助类型；S7 后这些都成为描述符里的字符串或实现层引用。
- 口径误差：统计按「每个声明文件一个类」，多类文件只计首类（TSDS 3150 / 3384）；继承转发外壳以声明文件里的 `pub fn` 行计入，已覆盖。

### 9.2 孤儿规则对按继承 DAG 切分的约束

孤儿规则：`impl Trait for Type` 必须在 trait 或 type 所在的 crate。S7 后每类要落的 impl 逐项看：

- `From<X> for Anc`：`From` 是外部 trait，impl 必须在 X 或 Anc 的 crate。放在**子类 X 的 crate**，X 本来就依赖 Anc，**足够**。
- `From<Object> for X`、`From<X> for Object`：放在 X 的 crate（Object 在根 crate）。
- 下转（checkcast `Sub::from(anc)`）：S7 里统一是 `From<Object> for Sub` 加描述符判定，放在 Sub 的 crate，不需要祖先 crate 知道子类。
- `impl ObjectVTable for X`（S7 后只剩转交）：trait 在根 crate，放在 X 的 crate。
- `impl X__VTable for X__inner`、`impl Iface__VTable for X__inner`：都在实现层（`X__inner` 是本地类型）。
- 接口 lambda 载体 `Iface__Lambda`：放在接口所在 crate。

结论：**继承与转型类引用全部可以按「子在下游」放置，孤儿规则不构成额外约束**。真正的约束来自**固有 impl**：`impl X { pub fn m(..) -> T }` 必须和 `struct X` 在同一个 crate。签名里的 T 在哪个 crate，X 就依赖哪个 crate，所以签名 SCC 内的类必须同 crate。

### 9.3 两种拆法与对可读层的影响

**A（推荐）：可读层不变，按签名 SCC 拆。**
- 固有方法、访问器、static 照旧是 `impl X { .. }`。`let animal: Animal = Dog::new(); animal.speak();` 一字不变。
- crate 划分必须是签名图凝聚 DAG 的拓扑分层：
  - 最大 SCC 单独成一个 crate（它含 `Object` / `String` / `Class`，位于底层）；
  - 其余类（各自 SCC ≤ 22）按拓扑序切成 k 段，商图无环。
- 生成器按档案自动求 SCC 与分层，不写类名。闭包变化只会挪动段边界，不需要手写规则。

**B：方法面改为下游 trait，类型层只按继承 DAG 拆。**
- 类型层只放 struct、描述符、vtable trait，再把 vtable 签名里的引用类型擦除为句柄（`#[repr(transparent)]` wrapper 与句柄同布局，边界零成本转换）。实测这一层的 SCC 为 1，可以任意按继承 DAG 拆。
- 可读层方法改为每类一个方法 trait，定义并实现在下游 crate（trait 是本地类型，孤儿规则允许）。方法 trait 只引用类型层，彼此不依赖，也可以任意拆。
- 调用写法 `animal.speak()` / `Dog::new()` 在文本上不变，但要求 trait 在作用域内（生成器管理 `use`）。有三个代价：
  1. 每类新增一个生成器自造的 trait。这与 CLAUDE.md「代码生成命名原则」第 1 条冲突（Java 命名空间之外的自造 trait），须用户裁决。
  2. 方法解析变成 trait 探测：同名方法（`toString` 等）有成千个 trait 提供，每个调用点的候选集合都很大。须按文件精确导入，否则实现层类型检查会退化。
  3. 编译错误与 rustdoc 的可读性下降。
- **不推荐**：拆分收益 A 已经拿到大半（见 9.4），B 的代价落在可读层与命名原则上。

### 9.4 结论与峰值估算

**估算口径**
- 现状（c39591d1）两点线性：HelloWorld 472 类 1.47 GB，TSDS 3393 类 9.10 GB。得出每类约 2.6 MB，截距约 0.24 GB。
- S7 后每类基础设施代码与 std 按类实例消失。按 §六 的目标（3400–3900 类 ≤ 6 GB），每类约 1.6 MB。
- 拆分后每个下游 crate 另加上游元数据的解码开销：按每个被引用的上游类约 0.1 MB 计，上限约 0.3 GB。

**按 A 拆**

| 档案（java.base 类数） | 不拆（S7 单 crate） | 最大 SCC crate | 其余 k 段（每段峰值） | 最大单 crate 峰值 |
|---|---:|---:|---:|---:|
| HelloWorld（430） | 约 0.93 GB | 164 类，约 0.5 GB | 266 类合为 1 段，约 0.7 GB | **约 0.7 GB**（不拆约 0.93 GB，不拆更省墙钟） |
| Digester（2586） | 约 4.4 GB | 459 类，约 1.0 GB | 2127 类分 3 段，每段约 710 类，约 1.6 GB | **约 1.6 GB** |
| TSDS（2857） | 约 4.8 GB | 514 类，约 1.1 GB | 2343 类分 4 段，每段约 585 类，约 1.5 GB | **约 1.5 GB** |

- java.base 声明层能拆。段数随档案规模取，规则是每段 ≤ 约 600–700 类。**下限由最大签名 SCC 决定**：在三个档案上是 164–514 类，约 0.5–1.1 GB。
- 对 §7.6 目标：
  - HelloWorld 声明 crate ≤ 1.2 GB：S7 单 crate 已满足，小档案不拆（拆分只会拉长关键路径）。
  - Digester 声明 crate ≤ 2 GB：S7 + A 拆分后最大单 crate 约 1.6 GB，满足；只做 S7 不拆，约 4.4 GB，不满足。
  - 3 个 OOM 例：最大单 crate 约 1.5–1.7 GB，远离 cgroup 上限。
- 墙钟：分层 crate 之间是依赖链。cargo 按 rmeta 流水线化，下游可以在上游元数据产出后开工，但不能完全并行。段数要与 `CARGO_BUILD_JOBS` 和关键路径一起定，留到实施时实测，档案规模阈值从实测求得。
- 前提与风险：
  1. 估算依赖 S7 每类 1.6 MB 的目标值，必须在 S7-5 实测后回填。
  2. 签名 SCC 的规模随档案增长（HelloWorld 164 → TSDS 514）。若生产档案远大于语料档案，下限随之上移；届时按 B 的类型层擦除再论证。
  3. 段切分与 t1-link 方案的「按模块切 crate」同构：模块是第一层切分，模块内按签名 SCC 拓扑分层是第二层，两层都由数据驱动、不写类名。body / meta 层的切分轴同样以 t1-link 方案的结论为准。
