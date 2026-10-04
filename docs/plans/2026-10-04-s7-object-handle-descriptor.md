# S7：统一对象句柄 + 每类静态描述符（方案，2026-10-04）

> 拆 crate 线（`docs/plans/2026-10-01-rustc-memory-and-crate-split.md` §7.5.4）的结构性终态项。用户 2026-10-04 批准（D7，取法 B）；S7-0 / S7-1 在 s7-desc 分支实施，进度见下文「现状」。
> 测量数据见上述计划 §7.7「2026-10-04 复测」。

> **切分轴已定（2026-10-04）：先按 JDK 模块，超阈值的模块内再按体量**（crate-split 计划 §7.5.5，与 t1-link 方案 `docs/plans/2026-10-04-t1-step2-direct-rustc-link.md` §4.8 对齐）。S7 作用于每个模块 crate 的声明层，峰值约束只在 `java_base_decl`。§九的按签名 SCC 分段是 `java_base_decl` 内部的第三级，仍是方案（V5）。

## 现状（实施记录，S7-0/1 在 s7-desc 分支、S7-2 在 s7-wrap 分支，随提交同步）

### 已验证结论

- **S7-0（描述符生成 + 守护）**：
  - runtime 新增 `class_desc.rs`：`__ClassDesc { binary_name, depth, display, supertypes }`，方法 `super_desc` / `is_subclass_of`（`display[t.depth]` 同址，O(1)）/ `is_subtype_name`（有序名单二分）。`ObjectVTable::__desc()` 缺省经 `__view_target` 委托，非类对象（数组、装箱基本类型、TypedNull、手写非类对象）为 None。
  - 宏按类发射模块级 `#[doc(hidden)] pub static X__DESC` 与 wrapper 固有常量 `X::__DESC`（声明层）；`X__inner` 的 `__desc()` 返回 `<X<Object..>>::__DESC`（实现层）。祖先描述符写作 `<Anc<Object..>>::__DESC`：类型路径不需要新增 `use`，祖先形参取 Object（宏注入的标准 bound 对 Object 恒成立）。
  - 守护：`[raw-audit]` 行末新增 `handwritten_vtable_impls=N`（扫 `runtime/java_runtime/src` 不含生成标记的 `.rs`，正则 `impl .. __VTable .. for`；非零时另出 `[vtable-impl-audit]` 明细）；单测 `repo_runtime_has_no_handwritten_vtable_impls` 断言仓库手写层为 0。现状即 0。
- **S7-1（类型判定读描述符，删按类判定代码）**：
  - `is_instance_of` / `__class_name` 改为 `ObjectVTable` 缺省实现：视图委托运行时类，否则读 `__desc()`（`is_subtype_name` / `binary_name`）。`X__inner` 不再按类展开 `matches!` 名单与类名常量方法；只有代理载体覆盖 `is_instance_of`（描述符名单 ∨ `__vm_proxy_implements`）。
  - 删 `__view_as`（trait 方法与 wrapper 实现）与 wrapper `__view_into` 的本类 / 祖先 / 接口载体臂（连同 `#[iface_carrier_views]` 属性、生成器发射与宏解析）。`__view_into` 只剩数组（`JArray`）响应，闭包分析器按数组视图处理它，不变。
  - checkcast / 擦除重建：`__class_from_object(obj, Self::__DESC, ..)`——null → 缺省；运行时类恰为目标（`as_any` downcast）→ 克隆；`__desc().is_subclass_of(desc)`（display O(1)）→ 擦除重建；否则 `__checkcast_fail`。接口目标仍经 `try_cast` 的 `is_instance_of` 名字判定 + `From<Object>` 擦除路径。
  - catch：`catch_as::<T>()` 去掉 binary name 参数与 `__view_as` 步，快路径 downcast 之后直接 `T::from`（同上擦除重建）。
  - aastore / 数组协变：`erased_array_compatible` 与 aastore 退回 `is_instance_of(元素名)`；`__array_elem_assignable(target_elem, slot)` 对类探针读描述符名单，数组探针仍 `view_into`（嵌套数组上转）。
  - 删除对象：手写 `checkcast<T>` 与自由函数 `checkcast_fail`（并入 `Object::__checkcast_fail`，冷路径）。
  - **等价性（§五、V1 论证）**：被删的每条按类路径与读描述符路径给出同一判定集合——`matches!` 名单即 `all_supertypes`，描述符 `supertypes` 是同一名单排序去重；`__view_as` / 类臂 `__view_into` 的产物（同一对象、vtable 经 supertrait 上转、同一 `any`）与 `From<Object>` 擦除重建的产物逐部件相同；接口载体臂的产物与 `try_cast` 名字判定 + 擦除路径相同。7 例 cargo check 全过，未改 e2e 期望。
- **`__Described` trait 与 wrapper 固有常量的取舍**：取固有常量 `X::__DESC`。判定逻辑全在 runtime 的非泛型函数里，只需在每类 `From<Object>` 调用点把本类描述符作为值传入（`Self::__DESC`），从不需要以 trait 约束泛型地取描述符；trait 会多一个 Java 命名空间外的公开 trait（命名原则禁止）并给每类多一个 impl 块。
- **与 §3.2 的偏差（终态取舍，不是过渡）**：
  - **名字取 `__ClassDesc`**：`java.lang.constant.ClassDesc` 是 Java 类，生成文件显式 `use` 它时会遮蔽 prelude 通配引入（与 `__Shared` 同一约定）。
  - **wrapper 固有 `const __DESC`，不引入 `__Described` trait**：固有常量即可从类型路径取得任意类的描述符，不需要 trait 约束；不新增 Java 命名空间之外的 trait（命名原则）。名字带 `__` 前缀避开 Java 静态字段；与既有 `BINARY_NAME` 同一形态。
  - **描述符必须是 `static`**：地址即类标识（`is_subclass_of` 按地址比较），`const` 内联会在各使用点产生副本。
  - **接口按名字表达（`supertypes`），不是 `&[&ClassDesc]`**：闭包外接口没有 Rust 类型，接口载体也不是运行时类，没有类描述符；`supertypes` 即现有 `all_supertypes`（本类、父类链、闭包内外全部接口、`java/lang/Object`），判定集合与现有 `is_instance_of` 的 `matches!` 名单逐项相同。
  - **`super_` 不单存**：由 `display[depth-1]` 给出（`super_desc()`）。
  - **`upcast` / `iface_carriers` / `fields` / `alloc` / `clinit` 留给 S7-2..S7-4**：它们依赖统一句柄（S7-2）或字段描述（S7-3）、类初始化骨架（S7-4），S7-0/1 用不到。

### 实测（本机 macOS，HelloWorld；峰值为 `scripts/crate_mem_profile.py` 的 `java_runtime` rustc ru_maxrss）

| 提交 | 声明 crate 展开 | `impl ObjectVTable for` | java_runtime 峰值 |
|---|---:|---:|---:|
| 86df0737（起点） | 14.49 MB | 1.48 MB | 1388 MB |
| S7-0 | 14.79 MB（+0.30，描述符 static 与固有常量） | 1.48 MB | 1091 MB（本机峰值噪声大，以服务器为准） |
| S7-1 | **13.89 MB**（−0.90 对 S7-0，−0.60 对起点） | **0.80 MB**（`__view_into` / `__view_as` / `is_instance_of` / `__class_name` 删净） | 1401 MB（同上，以服务器为准） |
| d840bb83（S7-2 基点，含 S7-1） | 13.89 MB | 0.80 MB | 1167 / 1290 MB（两次采样） |
| 8bdcb04d（S7-2a） | **12.95 MB**（−0.94 对基点） | **0.31 MB**（wrapper impl 只剩 `__handle` / `as_any` / `__interface` / `__desc`） | 1326 / 1268 MB（两次采样；本机噪声 ±150 MB，与基点不可分，以服务器为准） |

服务器（Linux，`crate_mem_profile.py` 同口径；作业 `s7m-<提交>`）：

| 提交 | HelloWorld 展开 | HelloWorld 峰值 | TSDS 展开 | TSDS 峰值 |
|---|---:|---:|---:|---:|
| 86df0737（起点，ubuntu） | 14.46 MB | 1448 MB / 27.0 s | 103.37 MB | 8540 MB / 221.0 s |
| 8f515016（S7-0，jp1） | 14.76 MB | 1440 MB / 29.2 s | 106.07 MB | 8642 MB / 248.5 s |
| a6a00c06（S7-1，ubuntu） | **13.87 MB** | **1408 MB / 25.4 s** | **98.27 MB** | **8192 MB / 209.5 s** |
| 8bdcb04d（S7-2a，jp2） | **12.94 MB** | **1359 MB / 24.6 s** | **90.19 MB** | **7597 MB / 203.9 s** |

- S7-0 只增代码：TSDS 展开 +2.70 MB、峰值 +102 MB（3432 个描述符 static 与固有常量）。
- S7-1 对起点：TSDS 展开 −5.10 MB（−4.9%）、峰值 −348 MB（−4.1%）、声明 crate 墙钟 −11.5 s；HelloWorld 展开 −0.59 MB、峰值 −40 MB。按 §7.7 余量换算，FHP（ef600555 10495 MB）预计约 10.1 GB，TJNP 约 9.0 GB。
- S7-2a 本机 HelloWorld 展开分项（对基点 d840bb83）：`impl ObjectVTable for` 0.80 → 0.31 MB（−0.49，cells / word / ref 查询与 `__erased_inner` 删除，改由 trait 缺省经句柄目标转交）；extern 块 1.52 → 1.36 MB（−0.16，cells / from_any 存储钩子删除，只留 alloc）；`impl From for` 0.34 → 0.31 MB（祖先上转改为 `__r.upcast`）。`impl <固有>` 不变（6.25 MB）：Java 方法外壳与字段访问器的数量不变，只是取视图由 `&*self.vtable` 改为 `self.__r.vt()`。
- 收益小于 §四 的估计：删掉的是判定类方法（`__view_into` / `__view_as` / `is_instance_of` / `__class_name`），每类 fn 数只少 2–4 个；借用检查的大头（Java 方法外壳、字段访问器、其余 ObjectVTable 方法）要到 S7-2 统一句柄 / S7-3 字段描述才动。

本机展开体量与 §六 的 19.29 MB（服务器 Linux，a7996092）口径不同：本机 cfg 只展开 macOS 分支，且起点已含 #6 后的缩减；前后对照只看同口径差值。

### 失败的方案与原因

（暂无）

### 下一步

- S7-2a（8bdcb04d）已在 s7-wrap 分支实施：抽查（分布式）与服务器同口径测量（作业 `s7m-8bdcb04d`，HelloWorld / TSDS）待主会话执行，结果补入上表与拆 crate 方案 §7.7。
- S7-2b（Object 直接持有内部对象、去 blanket `From`、null → `__typed_null`、约 68 处 downcast 审计）按 S7-2a 服务器实测再定；接口载体与 lambda 载体仍持 `__Shared<dyn I__VTable>`，收敛到句柄随 S7-2b / 接口载体线处理。

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
  - 实施要点（2026-10-04，s7-wrap）：
    - runtime 新增 `__Handle(Option<__Shared<dyn ObjectVTable>>)` 与 `__Ref<V>{ h, vt: Option<NonNull<V>> }`（vt 不持有，指向 h 所持对象）；wrapper 只剩 `__r: __Ref<dyn X__VTable>`，`Default` = `__Ref::NULL` 不分配，`_init_not_null` 才经 alloc 钩子分配。
    - 删掉 `vtable` / `any` / `_jvm_null` 三字段与 `cells` / `from_any` 两个存储钩子、`__erased_inner`；上转 = `__r.upcast(|v| v as &dyn Anc__VTable)`，`From<Object>` 由句柄目标经 `__erased_vtable` 填 vt；wrapper 的 `impl ObjectVTable` 只剩 `__handle` / `as_any` / `__desc`，其余走 trait 缺省委托句柄目标。
    - Object 仍是 `Rc<wrapper>`（S7-2a）；Object 直接持有内部对象、去掉 blanket `From` 另作 S7-2b，S7-2a 实测后再定。
  - S7-2b 实施要点（2026-10-04，s7-2b）：
    - wrapper 不再实现 `ObjectVTable`：按类只生成 `impl From<X> for Object`（一行转交非泛型 `__Handle::into_object(desc)`：非 null 直接交出句柄所持存储，null → 按描述符的类型化 null，`__desc` / `__class_name` / `is_instance_of` 报静态类，与原 null wrapper 同答）；`is_jvm_null` 改为 wrapper 固有方法。
    - 删 blanket `From<T: ObjectVTable> for Object`，基本类型盒、数组、`JvmRef`、lambda 载体逐类型显式 `From`；`ObjectVTable::__handle` / `__view_target` 及各缺省方法的转交臂删除（Object 只持运行时类对象，不再有「视图对象」）。
    - downcast 审计：`__class_from_object` / `catch_as` 删「持有对象即 W」快路径（恒不命中），`Object.equals` 的 String 快路径改按描述符；接口载体与 lambda 载体收敛到句柄另作 S7-2c。
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

## 九、java.base 声明层能否再拆成多个 crate `java_base_decl_k`（V5；2026-10-04 论证，只写方案）

> 本节覆盖两种形态：现状形态（不等 S7，§9.5）与 S7 后形态（§9.1–9.4）。最底层 crate 的规模以 §9.5 的 INFRA 口径为准：手写基础设施引用 `String` / `Class` / `Throwable` 等，必须与它们同 crate。§9.4 的估算已按此修正。

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

**按 A 拆**（S7 后签名图，含 INFRA 约束，§9.5；段上限 650 类）

| 档案（java.base 类数） | 不拆（S7 单 crate） | 最底层 crate（含 INFRA 的 SCC） | 切段（`decl_scc.py --seg 650`） | 最大单 crate 峰值 |
|---|---:|---:|---|---:|
| HelloWorld（430） | 约 0.93 GB | 254 类（59%），约 0.65 GB | 不拆（拆了只拉长关键路径） | **约 0.93 GB** |
| Digester（2586） | 约 4.4 GB | 572 类（22%），约 1.16 GB | 4 段：650 / 650 / 650 / 636 | **约 1.3 GB**，加上游元数据 ≤ 1.6 GB |
| TSDS（2857） | 约 4.8 GB | 634 类（22%），约 1.26 GB | 5 段：650 ×4 / 257 | **约 1.3 GB**，加上游元数据 ≤ 1.6 GB |

- java.base 声明层能拆。段数随档案规模取，规则是每段 ≤ 约 650 类。**下限由含 INFRA 的签名 SCC 决定**：在三个档案上是 254–634 类，约 0.65–1.26 GB。
- 对 §7.6 目标：
  - HelloWorld 声明 crate ≤ 1.2 GB：S7 单 crate 已满足，小档案不拆（拆分只会拉长关键路径）。
  - Digester 声明 crate ≤ 2 GB：S7 + A 拆分后最大单 crate ≤ 1.6 GB，满足；只做 S7 不拆，约 4.4 GB，不满足。
  - 3 个 OOM 例：最大单 crate 约 1.5–1.7 GB，远离 cgroup 上限。
- 墙钟：分层 crate 之间是依赖链。cargo 按 rmeta 流水线化，下游可以在上游元数据产出后开工，但不能完全并行。段数要与 `CARGO_BUILD_JOBS` 和关键路径一起定，留到实施时实测，档案规模阈值从实测求得。
- 前提与风险：
  1. 估算依赖 S7 每类 1.6 MB 的目标值，必须在 S7-5 实测后回填。
  2. 签名 SCC 的规模随档案增长（HelloWorld 164 → TSDS 514）。若生产档案远大于语料档案，下限随之上移；届时按 B 的类型层擦除再论证。
  3. 段切分与 t1-link 方案的「按模块切 crate」同构：模块是第一层切分，模块内按签名 SCC 拓扑分层是第二层，两层都由数据驱动、不写类名。body / meta 层的切分轴同样以 t1-link 方案的结论为准。

### 9.5 现状形态（不等 S7）能否按 SCC 拆（V5 ①，2026-10-04 补测）

**口径**
- 现状声明层引用图 = 声明文件 `use` 列表（#3 剪枝后，即正文与宏属性实际引用的名字）加继承 / 接口实现。它覆盖字段类型、方法签名、From / checkcast、vtable trait、接口载体、异常元数据。
- 宏按类展开引用到的外类名字都要经文件头 `use` 进作用域，所以这张图包括了宏展开产生的边。
- 加一个伪节点 INFRA，代表手写基础设施（`object.rs` / `error.rs` / `array.rs` / `meta.rs` / `gil.rs` 等无生成标记、非 `_impl.rs` 的文件）：
  - 全部类 → INFRA（`Object` / `ObjectVTable` / `Result` / GIL）；
  - INFRA → 手写正文引用到的类（去掉注释与字符串后按名字匹配）。实测 61–71 类，含 `String`、`Class`、`Throwable` 和 VM 抛出的异常类。
  - `<x>_impl.rs` 的引用并入类 x。
- 含 INFRA 的 SCC 必须同 crate，就是最底层 crate。
- 工具：`scripts/decl_scc.py <scratch> --module-pkgs <java.base 包清单> --seg 650`。

**实测**（java.base 子图）

| 档案 | java.base 类 | 现状：含 INFRA 的 SCC | 现状切段 | S7 后：含 INFRA 的 SCC | S7 后切段 |
|---|---:|---:|---|---:|---|
| HelloWorld | 430 | 365（84%） | 1 段 | 254（59%） | 1 段 |
| Digester | 2586 | 1933（74%） | 3 段：1933 / 650 / 3 | 572（22%） | 4 段：650 / 650 / 650 / 636 |
| TSDS | 2857 | 2187（76%） | 3 段：2187 / 650 / 20 | 634（22%） | 5 段：650 ×4 / 257 |

不加 INFRA 时的最大 SCC：现状 326 / 1836 / 2027 类；S7 后 164 / 459 / 514 类（§9.1）。INFRA 把两种形态的底层都撑大了。

**峰值估算**
- 现状单价：ef600555 两点，HelloWorld 430 类 1.47 GB、TSDS 3150 类 8.41 GB，得每类约 2.55 MB，截距约 0.37 GB。
- S7 单价：§9.4 口径，每类 1.6 MB，截距 0.24 GB。
- 下游段另加上游元数据解码，≤ 0.3 GB。

| 档案 | 现状不拆（java.base 声明层） | 现状按 SCC 拆：最大 crate | S7 不拆 | S7 + 按 SCC 拆：最大 crate | §7.6 目标 |
|---|---:|---:|---:|---:|---:|
| HelloWorld | 约 1.47 GB | 约 1.30 GB（底层 365 类），省 0.17 GB 但多一级依赖链 | 约 0.93 GB | 不拆，约 0.93 GB | ≤ 1.2 GB |
| Digester | 约 6.96 GB | 约 5.30 GB（底层 1933 类） | 约 4.4 GB | ≤ 1.6 GB | ≤ 2 GB |
| TSDS | 约 7.66 GB | 约 5.95 GB（底层 2187 类） | 约 4.8 GB | ≤ 1.6 GB | — |

**结论**
- 现状最大 SCC 占 java.base 的 74–84%，**不是「远小于全体」**。现状形态按 SCC 拆只把最大 crate 削掉约 24%（Digester 6.96 → 5.30 GB），离 2 GB 仍远，HelloWorld 也达不到 1.2 GB。现状形态的声明层峰值下限就是底层 SCC：Digester 约 5.3 GB，TSDS 约 6.0 GB。**不满足 §7.6**。
- 现状底层大的原因：
  - wrapper 持有 `__Shared<dyn X__VTable>`，`From` / checkcast / 接口载体 / 运行时类还原的代码按类型写出；
  - 异常元数据经 `use` 引入异常类。
  
  这些在 S7 后变成描述符数据，或者移进实现层（§9.1），所以底层 SCC 从 74–84% 降到 22%（HelloWorld 59%）。
- 拆分机制（按 SCC 凝聚 DAG 拓扑切段、跨段路径前缀、门面 crate）在两种形态下完全相同，是终态机制，不是过渡形态。它可以先于 S7 落地（三个 OOM 例的峰值 crate 估计降约 1.7 GB），S7 落地后同一机制自动得到 ≤ 1.6 GB 的段。先后顺序由协调方定。§7.6 的达标只取决于 S7。

### 9.6 孤儿规则与宏展开跨 crate 可见性（V5 ③）

**孤儿规则**
- §9.2 对 S7 形态的逐项结论适用于现状形态，现状多出来的 impl 也都落在子类 X 自己的 crate：
  - `impl ObjectVTable for X`（wrapper）；
  - `PartialEq` / `Clone` / `Debug` / `Default for X`；
  - `From<X> for Anc`、`iface_upcasts!` 的 `From<X> for Iface`；
  - `From<Object> for X`、`From<X> for Object`。
- 泛型类的 `impl<A> From<X<A>> for Anc<A>`：`Anc<A>` 覆盖了 A，`X<A>` 是本地类型，满足 RFC 2451。
- 唯一的硬约束仍是固有 impl：`impl X { .. }` 与 `struct X` 同 crate。手写 `<x>_impl.rs` 随类 x 进它的段，已计入图。

**宏展开的可见性**
- `java_class!` 生成的跨类项都是 `pub`：`__from_parts`、`__class_init`、`BINARY_NAME`、字段访问器、静态存储、vtable trait。唯一的 `pub(crate)` 是 `X__inner` 的字段，只在本类实现层使用，不跨类、不跨段。
- 宏输出里的外类名字都不带路径，经文件头 `use` 解析，宏本身不写 `crate::` 绝对路径。（`generic_sig.rs` 的 `crate::error::Result` 只用于宏内部的类型分析，不进输出。）跨段因此只改文件头 `use` 的前缀，宏不需要改。

**路径与门面**（这是约束，也是实施项）
- 今天实现层靠 crate 根 `use java_runtime::*;` 让 `crate::java::…` 解析到声明层。多个声明段各有自己的 `java::lang` 等模块子树，这个技巧失效：
  - 本段的 `java` 模块遮蔽 glob；
  - 多个上游 glob 的同名模块互相冲突。
- 所以生成器必须按被引用类所在的段写 crate 限定前缀：同段 `crate::`，跨段 `java_base_decl_<k>::`。这与 t1-link 接口 3（跨模块前缀按类所属模块给出）是同一机制，按「类 → crate」表统一给出。
- 门面 crate `java_base` 给下游模块与用户提供单一路径。它生成一棵镜像模块树：每个包是显式的 `pub mod`，包内对拥有该包类的每个段写 `pub use java_base_decl_<k>::<包路径>::*;`。
  - 显式模块遮蔽 glob，所以子模块名不冲突；
  - 每个类只在一个段里，所以叶子项不冲突。

**可读层（V5 ④）**
- 方法体与调用点文本不变：`let animal: Animal = Dog::new(); animal.speak();`。
- 变化只在文件头 `use` 的 crate 前缀，例如 `use java_base_decl_2::java::lang::Object;`；下游模块写 `use java_base::java::lang::Object;`。

