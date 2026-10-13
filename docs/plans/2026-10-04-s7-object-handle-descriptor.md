# S7：统一对象句柄 + 每类静态描述符（方案，2026-10-04）

> 拆 crate 线（`docs/plans/2026-10-01-rustc-memory-and-crate-split.md` §7.5.4）的结构性终态项。用户 2026-10-04 批准（D7，取法 B）；S7-0 / S7-1 在 s7-desc 分支实施，进度见下文「现状」。
> 测量数据见上述计划 §7.7「2026-10-04 复测」。

> **切分轴已定（2026-10-04）：先按 JDK 模块，超阈值的模块内再按体量**（crate-split 计划 §7.5.5，与 t1-link 方案 `docs/plans/2026-10-04-t1-step2-direct-rustc-link.md` §4.8 对齐）。S7 作用于每个模块 crate 的声明层，峰值约束只在 `java_base_decl`。§九的按签名 SCC 分段是 `java_base_decl` 内部的第三级，D8 已提前实施（与 S7-4 并行，b51f9531 / b093069f 合入，见 §9.7）。

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
| 9260ead7（S7-2b） | **12.82 MB**（−0.13 对 S7-2a） | **0.02 MB**（wrapper 不再实现；只剩 14 个手写 / 基本类型 / 数组 / 载体 impl） | 1252 MB（一次采样，同上）；服务器 HW 1233 MB、TSDS 6654 MB |
| aa5910b5（S7-2c，M2 后 `java_base_decl`） | **12.80 MB**（同口径基点 dd10731c 12.82，−0.02） | 0.02 MB | 本机未测；服务器 HW 1242 MB（S7-2b 1233，持平）、TSDS **6153 MB**（S7-2b 6654，−501） |

服务器（Linux，`crate_mem_profile.py` 同口径；作业 `s7m-<提交>`）：

| 提交 | HelloWorld 展开 | HelloWorld 峰值 | TSDS 展开 | TSDS 峰值 |
|---|---:|---:|---:|---:|
| 86df0737（起点，ubuntu） | 14.46 MB | 1448 MB / 27.0 s | 103.37 MB | 8540 MB / 221.0 s |
| 8f515016（S7-0，jp1） | 14.76 MB | 1440 MB / 29.2 s | 106.07 MB | 8642 MB / 248.5 s |
| a6a00c06（S7-1，ubuntu） | **13.87 MB** | **1408 MB / 25.4 s** | **98.27 MB** | **8192 MB / 209.5 s** |
| 8bdcb04d（S7-2a，jp2） | **12.94 MB** | **1359 MB / 24.6 s** | **90.19 MB** | **7597 MB / 203.9 s** |
| aa5910b5（S7-2c，jp2） | 12.80 MB | 1242 MB | 79.83 MB | 6153 MB / 155.7 s |
| dc61397b（S7-3a+b+x，us1） | 13.83 MB | 1338 MB / 25.2 s | 87.78 MB | 6571 MB / 179.1 s |
| 6f93f1c6（main 基点，S7-3 前，ubuntu） | 12.80 MB | 1232 MB / 21.9 s | 80.65 MB | 6166 MB / 152.6 s |
| 9b3b4bb7（S7-3c，ubuntu） | 12.96 MB | 1294 MB | 81.83 MB | 6522 MB |

- S7-0 只增代码：TSDS 展开 +2.70 MB、峰值 +102 MB（3432 个描述符 static 与固有常量）。
- S7-1 对起点：TSDS 展开 −5.10 MB（−4.9%）、峰值 −348 MB（−4.1%）、声明 crate 墙钟 −11.5 s；HelloWorld 展开 −0.59 MB、峰值 −40 MB。按 §7.7 余量换算，FHP（ef600555 10495 MB）预计约 10.1 GB，TJNP 约 9.0 GB。
- S7-2a 本机 HelloWorld 展开分项（对基点 d840bb83）：`impl ObjectVTable for` 0.80 → 0.31 MB（−0.49，cells / word / ref 查询与 `__erased_inner` 删除，改由 trait 缺省经句柄目标转交）；extern 块 1.52 → 1.36 MB（−0.16，cells / from_any 存储钩子删除，只留 alloc）；`impl From for` 0.34 → 0.31 MB（祖先上转改为 `__r.upcast`）。`impl <固有>` 不变（6.25 MB）：Java 方法外壳与字段访问器的数量不变，只是取视图由 `&*self.vtable` 改为 `self.__r.vt()`。
- S7-2b 本机 HelloWorld 展开分项（对 S7-2a）：`impl ObjectVTable for` 0.31 → 0.02 MB（−0.29，约 370 个 wrapper impl 全删）；`impl From for` 0.31 → 0.38 MB（+0.07，每类一条 `From<X> for Object` 取代 blanket）；`impl <固有>` 6.25 → 6.32 MB（+0.07，wrapper 固有 `is_jvm_null` / `__nn`）。净 −0.13 MB；trait 求解面少了约 370 个 `ObjectVTable` 实现者，借用检查 / coherence 的收益以服务器峰值为准。
- S7-2b 本机 TSDS 展开 88.30 MB（macOS cfg，与服务器 Linux 口径的 90.19 MB 不可直接相减）。
- S7-2c 本机展开（M2 后声明层 crate 为 `java_base_decl`，基点 dd10731c 同口径重测）：HelloWorld 12.82 → 12.80 MB，TSDS 79.92 → 79.83 MB（−0.09）。展开文本只少了 inner `__interface` 的按值接收与未命中释放，载体多一个字段——收益不在展开体量，而在 std 单态化：每个被实现的接口原有一份 `Arc<dyn I__VTable>` 的 drop / `drop_slow` 与 `Option<Arc<dyn I__VTable>>` 槽实例，现全部消失（TSDS 声明层展开中 `__Shared<dyn` / `Arc<dyn ..__VTable>` 出现 319 → 280，剩余为 `dyn ObjectVTable`）；分派侧每次 invokeinterface 少一次引用计数增减与一次查询（视图指针在载体建立时求出）。峰值以服务器为准。
- S7-3 回退定位（dc61397b 对 S7-2c：HW 展开 +1.03 MB、峰值 +96 MB，TSDS 展开 +7.95 MB、峰值 +418 MB）。本机 HelloWorld 同口径分项（基点 main 6f93f1c6 12.81 MB → dc61397b 13.85 MB）：`impl <固有>` +0.75 MB，几乎全部是 `__STATICS`（435 个类、1184 项，0.81 MB：每项两次 `transmute` 的展开文本，且宏为全部类全部静态字段展开，而登记只用 97 个类）；`static` +0.27 MB（描述符 `fields` 0.21 MB 与 `alloc`）；删掉的 `__reflect_field` 只有 0.07 MB。
- **S7-3c 收窄**：① 静态字段表只为档案内可按名访问的静态字段展开——生成器以登记同一判定（`dispatch::static_reflected`：用户树类 / 全成员反射类的全部静态字段，其余类限序列化协议名与按名查字段名）在字段属性上打 `reflect = true`，宏只为带标记的字段展开项；② 表项与 `fields` 项改为 const fn 构造（`__StaticFieldDesc::of_prim / of_ref::<T>(java, getter, setter)`、`__FieldDesc::of_ref::<T> / of_prim`），擦除与协议实例化收在运行时，展开文本每项一行。本机 HelloWorld 展开 13.85 → **12.98 MB**（对基点 +0.17 MB：`fields` 描述 + `alloc`，均为浅拷贝 / Unsafe / 反射的必需数据；`__STATICS` 103 项）。
- **S7-3c 回归修复（s7-3 分支，1150fc04）**：s73c 抽查 StockTrans 在 `ObjectStreamClass.getDeclaredSUID → Field.getLong → Unsafe 静态偏移` 处报「静态字段读-改-写 无字段闭包」。判定改为全部来自分析事实与清单，生成器不再写序列化协议字段名单：
  - 闭包事实新增 `reflect.static_fields`：可取到静态字段句柄的字段枚举 / 名字不可知的按名取法，其口径所指类（含超类型；推不出时为闭包全部类）的静态字段；另加方法句柄常量 getStatic / putStatic（kind 2 / 4）解析到的字段。档案合成取非用户侧，单测侧取用户类，并入 `ReflectFacts.static_fields`。
  - `static_reflected` = 用户树类 ∪ 全成员反射类 ∪ 按名查字段 (类, 名) ∪ 目标类不明的按名字段名（已含 serialVersionUID / serialPersistentFields）∪ `static_fields`。
  - 精度：名字不可知的按名取字段，口径改为 Class 值集所指的类（值集齐全时），不再一律推不出。清单 `[facts.field_writes] instance_field_users` 登记取到的句柄只用于实例字段的取法（`ObjectStreamClass.getDeclaredSerialFields` 按修饰符滤掉 static），这类取法不计入静态口径。它原是 StockTrans 中唯一推不出的口径，未排除时表项为闭包全部静态字段。
  - 本机 StockTrans：表项 14747 → **996**，登记类 1553 → **494**，闭包类 / 方法集合不变；HelloWorld 表项 103 → **0**（无按名访问）。
  - **真正根因（s73d-1150fc04 sg1 抽查 StockTrans 仍失败后定位）**：宏 `statics_table` 判定字段是否带 `java_field(..., reflect = true)` 标记时，按属性 `to_string()` 的子串匹配。单测走 proc_macro2 回退实现，字符串化形态与匹配串一致；真实编译走编译器后端，空白形态不同，匹配全部落空，自 9b3b4bb7 起**所有类的 `__STATICS` 在实际构建中均为空**（与闭包事实、JDK 版本无关；本机与服务器 emit 一致）。修复：按语法树解析 `cfg_attr` → `java_field` 的名值对（`syn::Meta` / `MetaNameValue`），基本类型判定改为路径标识符比较。以 `rustc -Zunpretty=expanded` 走真实后端展开核对：StockTrans 用户类表含 `of_prim::<i64>("serialVersionUID", …)`。审计：rava_macros_core 其余 `to_string()` 用法为去空白比较或同后端字符串互比，无同类依赖空白形态的语义判定。1150fc04 的事实与精度改造保留有效。
  - 运行时诊断：未命中时 panic 消息附带声明类表的登记状态与表项名（`field_reflect::describe_static`）。宏单测 `statics_table_only_reflected_fields` 守护「只为带标记的字段生成表项」。
- S7-3c 对 main 基点（服务器同机 ubuntu 同口径，s7m-6f93f1c6 / s7m-9b3b4bb7）：HelloWorld 展开 +0.16 MB、峰值 +62 MB；TSDS 展开 +1.18 MB、峰值 +356 MB。S7-3 相对 dc61397b 的回退（TSDS 峰值 +418 MB 中）已收回大部分展开体量（87.78 → 81.83 MB），剩余展开差主要是 `fields` 描述与 `alloc`；峰值差 356 MB 仍偏大，需在 1150fc04 后（表项再收窄，TSDS 静态表应接近 HelloWorld 的 0 项口径）复测再定是否追查。注：该组差值测于静态表实际为空期间（见上条根因），根因修复后需复测。
- **S7-3c 根因修复验证状态（402840af 修复 / 9ef95270 文档，s7-3 分支，已推 origin 与 github）**：本机宏单测 `statics_table_only_reflected_fields` 通过，真实后端展开核对 StockTrans 表项非空。服务器 s73d-1150fc04 抽查 12 过 / StockTrans 败 / 2 未跑（修复前，失败即本根因）；s73d-ut-1150fc04 rc=0。在途：`s73d-ut-9ef95270`（generator 全量单测）、`s73d-9ef95270`（StockTrans 与 13 个序列化相关例 + TestFieldHandleProvenance / HelloWorld / InterfaceDispatch / LambdaBasic / StreamDemo 共 19 例，服务器 ubuntu sg1 sg2 us1 kr1）。待验证：两作业结果；静态表恢复后 HelloWorld / TSDS 展开与峰值对 main 基点复测。
- 收益小于 §四 的估计：删掉的是判定类方法（`__view_into` / `__view_as` / `is_instance_of` / `__class_name`），每类 fn 数只少 2–4 个；借用检查的大头（Java 方法外壳、字段访问器、其余 ObjectVTable 方法）要到 S7-2 统一句柄 / S7-3 字段描述才动。

本机展开体量与 §六 的 19.29 MB（服务器 Linux，a7996092）口径不同：本机 cfg 只展开 macOS 分支，且起点已含 #6 后的缩减；前后对照只看同口径差值。

### 失败的方案与原因

（暂无）

### 下一步

- S7-2b（9260ead7，s7-2b 分支）已实施：本机 8 例 0 错 0 警、输出与期望一致；服务器同口径测量（作业 `s7m-9260ead7`）：声明层峰值 HelloWorld 1359 → 1233 MB、TSDS 7597 → 6654 MB；抽查 s7b-c971047f 16/16，与 M2 合并后 int-6a784b50 8/8，合入 6a784b50。
- S7-2c（aa5910b5，s7-2c 分支）已实施：接口载体字段改 `__IfaceRef<dyn I__VTable>`，`__interface` 改为视图指针填充，`__Shared<dyn I__VTable>` 删净；本机 12 例 0 错 0 警、输出与期望一致。服务器同口径测量（作业 s7m-aa5910b5）：声明层峰值 HelloWorld 1233 → 1242 MB（持平）、TSDS 6654 → 6153 MB（−7.5%）；抽查 s7c-aa5910b5 20/20，合入 63bc9213。
- S7-3（s7-3 分支）实施要点：
  - **S7-3a**：`__ClassDesc` 增 `fields`（本类自有实例字段 `__FieldDesc { java, rust, kind }`）、`field_base`（继承字段数）、`alloc`（新建默认存储装入 Object）。
  - `X__inner` 加 `#[repr(C)]`、`__PrimCell` 加 `#[repr(transparent)]`：每个字段都是一个 `__Shared` 细指针，字段 i 位于存储基址 + i 个指针宽。
  - 由此，`__unsafe_*` / `__field_slot` 改为 `impl dyn ObjectVTable` 上的非泛型固有方法，沿 `display` 查各类的 `fields`。
  - 浅拷贝改为 `Object__clone_base` 走描述符：先 `alloc`，再逐字段拷贝。基本单元按位拷贝；引用单元经该字段载体类型的 `__ref_field::<T>` 函数指针拷贝。
  - 宏里按类展开的 7 个方法全删；数组仍走 `__shallow_copy`。
  - **S7-3b**：按名字段访问（Field.get/set、MH 字段句柄、Unsafe 静态字段）落运行时 `field_reflect`，生成器不再发射按类 `__reflect_field`（TestReflectFieldMethod 生成树 0 处）。
    - 实例字段：沿接收者描述符 `display` 找声明类、按 Java 名找字段，在存储单元上读写（基本单元按种类装箱 / 拆箱，引用单元经 `__ref_field`）；声明类不在祖先链上 → IllegalArgumentException。泛型类实例字段随之可反射（原先跳过）。
    - 静态字段：宏为类 / 接口展开关联常量 `X::__STATICS`（Java 名 + 既有 getter / setter 的擦除函数指针 + 按值类型实例化的 `__static_ref::<T>` / `__static_prim::<P>`）；经访问器读写，类初始化、安全点与手写访问器语义不变；常量无 setter → `final_field`。关联常量只在 main 登记处求值，未登记类不付代码生成代价。
    - 登记沿用 `register_field_dispatch`（入参改为静态字段表），选择口径不变：用户树类全部，JDK / 库类限序列化协议名与按名反射名。
  - **S7-3x 对象释放不递归**（来源：T1b 审计，c1d-closure-bloat §21.8.5）：`__Handle` / `Object` 的 Drop 释放最后一个强引用时经 `handle::__release` 计深（线程局部深度），深度 ≥ 32 的对象移入线程本地待释放队列，由最外层释放循环清空——任意链长下释放栈深 ≤ 32 层 drop 帧；非最后强引用只减计数（快路径一次原子读）。`Object` 槽位为 pub 元组字段，Drop 内以 null 单例换出再释放。字段载体（wrapper → `__Ref` → `__Handle`、擦除字段 `Box<Object>`、接口载体 → `Object`、数组元素）都经这两处，宏无需改动。验证用例 `48_refs/TestLongChainRelease`（10⁶ 节点单链表 + 2×10⁵ 擦除字段链 / 数组链，主线程与虚拟线程各一次）。

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
    - 审计结果（runtime 64 处 `downcast*::<>`）：只有上述 3 处假定「Object 持有 wrapper」；其余目标为基本类型盒（`i32` / `u16` / `f64` …）、`JArray<_>`、`JvmRef` 载体内值、`Object` 自身、`StackTraceFrames` 等非 wrapper 存储与 `__interface` / `__view_into` 槽位，语义不变。`try_checkcast::<W>` 对类 wrapper 不再命中 as_any 臂，调用方（`try_cast`、数组逐元素兼容）均有描述符 / `is_instance_of` 后续臂兜底。
    - 手写层只两处在 wrapper 上调 trait 方法（`String::units` 的 `is_jvm_null`、`Throwable.fillInStackTrace` 的 `__class_name`），分别改固有方法与 `Object::from(..).0.__class_name()`。
  - S7-2c 实施要点（2026-10-04，s7-2c）：
    - 接口载体字段改 `__ref: __IfaceRef<dyn I__VTable>`（runtime 非泛型逻辑一份）：句柄仍是 `Object`（载体须 `Deref<Target = Object>`、null 带接口静态类型），另持接口视图指针 `vt: Option<NonNull<dyn I__VTable>>`，在 `From<Object>`（含 `iface_upcasts!` 上转）时求出一次；分派直接经 `vt`，不再每次 Rc 克隆 + 查询。`vt` 为 None 而句柄非 null 即「不实现本接口」（代理 / default 回退 / AbstractMethodError 路径不变）。
    - `ObjectVTable::__interface(self: Rc<Self>, slot)` 改为 `__interface(&self, slot)`，slot 为 `Option<NonNull<dyn I__VTable>>`，与 `__erased_vtable` 同形；类存储与 lambda 合成对象（`I__Lambda`）的应答同改；`__Shared<dyn I__VTable>` 全部删除（runtime 只剩 `__Shared<dyn ObjectVTable>` 一份）。lambda 的闭包存储 `__Shared<__DynFn>` 是按擦除签名的闭包对象，不在本步。
    - 手写层 `from_any` 审计：仅 `Throwable` 回溯帧（`StackTraceFrames`，非 wrapper）一处，形态不变。
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
- **10-11 更新**：§9.8 的模拟表明，语料档案只有删掉签名边与字段边（⑤）才能 ≤ 1.3 GB。用户裁决采纳 B 的方法 trait 出路（类型标记 crate 路线 A），方法 trait 属生成层内部细节，与 `ObjectVTable` 同等，见 §9.8.3 / §9.9。

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

### 9.7 实施记录（D8，2026-10-05，与 S7-4 并行）

**提交**：第 1 步 b51f9531（纯分析 + 单测，合入 2602f409）；第 2 步 b093069f（发射多 crate，gate-ec6cbe11 合入 10314222）。

**分析**（`generator/crates/emit/src/project/decl_segments.rs`）
- 节点 = java.base 声明层每个生成类文件；边取自文件里的 `crate::` 路径（含花括号组、`r#`、`as`、`__` 后缀词干）与属性字符串里的二进制名（按 `;` 拆）。
- INFRA 伪节点：所有类 → INFRA；INFRA → 手写运行时文件里出现的类名与钉住类（有 `_impl` / `_ext` 共置手写的宿主、整类手写的类）。含 INFRA 的 SCC 必为底段。
- Tarjan（迭代）求 SCC，Kahn 依赖先行定序；底段之外的类按拓扑序连续切成均衡段，每段 ≤ `DECL_SEGMENT_CLASSES` = 650 类；java.base 声明类数 ≤ 650 时不拆。全程无类名。
  - **10-11 起改为按源码体量切**：不分段判定 ≤ `DECL_SEGMENT_BYTES` = 5.5 MB；分段后上段按上游感知峰值预算（`UPPER_SEGMENT_PEAK`）逐段装满，见 §9.8.4。

**布局**（`decl_side.rs`）——与 §9.6 的「跨段写 crate 前缀 + 门面镜像树」不同，实施取**镜像链**，省掉了按类改写前缀：
- 底段仍叫 `java_base_decl`；上段 `java_base_decl_<j>` 只依赖前一段，`lib.rs` 为 `pub use <前段>::*;` 加本段顶层包 mod；每个包 `mod.rs` 写 `pub use <前段>::<包>::*;`（前段视图有该包时）、`pub mod <子包>;`、`pub mod x; pub use x::*;`。显式 mod 遮蔽 glob，叶子项每类只在一段，不冲突。
- 末段因此是 java.base 声明层的完整视图。门面、方法体 crate、其他模块 crate、lib crate 都用 Cargo 重命名依赖末段（`java_base_decl = { package = "java_base_decl_<k>", .. }`），可读层与方法体文本不变。
- 本机临时把上限降到 20 强制分段：HelloWorld 3 个上段、FWord 33 个上段均 cargo check 通过，无 `pub(crate)` 可见性问题。

**服务器实测**（release，`crate_mem_profile.py --release`，b093069f；java.base 2757 类左右 → 底段约 2095 + 两段各约 331）

| 例 | `java_base_decl`（底段）峰值 MB | `_1` / `_2` 峰值 MB | 底段墙钟 s |
|---|---:|---:|---:|
| FWord | 7906 | 1263 / 1279 | 215 |
| FractionReduction | 7922 | 1265 / 1281 | 215 |
| PartitionInteger | 8149 | 1266 / 1292 | 228 |
| FibonacciMatrixExponentiation | 7912 | 1274 / 1281 | 206 |
| IQPuzzle | 7912 | 1268 / 1281 | 205 |
| SelfNumbers | 7873 | 1270 / 1283 | 205 |
| FourIsTheNumberOfLetters | 7918 | 1271 / 1280 | 210 |
| PrimorialNumbers | 7911 | 1272 / 1280 | 209 |
| RailwayCircuit | 7931 | 1262 / 1282 | 209 |

（作业 scc-rel1/2/3-b093069f，服务器 us1 / jp2 / kr1。）
- 9 例声明层全部编过，最大声明 crate 7.9–8.1 GB，低于 cgroup 上限 11.88 GB，比合入前（约 11.9 GB 被杀）降约 3.8 GB。上段每段约 1.27 GB、约 20 s。
- 但 9 例 `rava compile` 仍 rc=1，作业尾有 `__RAVA_OOM_KILL__`：OOM 落在声明层之外的 crate——OOM_CULPRIT_PLACEHOLDER
- 调试档抽查 scc-b093069f：4 例 debug 构建成功、运行超时（R1 运行性能已知问题，不归 D8）。单测 scc-ut-b093069f rc=0。

**与终态目标的差距**
- 每个声明 crate ≤ 1.3 GB：上段已达标；底段 7.9 GB 是含 INFRA 的签名 SCC（约 76% 的类），现状形态下即为下限（§9.5）。底段继续降只靠 S7：泛型类方法体与描述符化把底段 SCC 收到约 22%（§9.1 / §9.4）后，同一机制自动切出 ≤ 1.3 GB 的段，D8 无需再改。

### 9.8 底段 SCC 收窄的模拟验证（10-10 任务说明，10-11 结果见 9.8.1–9.8.3）

> 用户 10-10 指示 S7-4 提前开工，并三次追加要求：**先模拟、后实施**。本节先记任务说明，模拟结果、边分类与方案随分析补在下文；**实施等用户确认后再开始**，模拟期间不改生成器、宏与 D8。

**目标（加严）**
- 所有声明 crate 无一例外 ≤ 1.3 GB。收窄后剩下的最大 SCC（§9.4 预测约 22% 的类）本身也必须 ≤ 1.3 GB，不作「下限」例外。

**方法：先模拟、后实施**
- 从当前 main 的实际声明层依赖图出发——即 D8 分段所用的同一张图（`decl_segments.rs`：`crate::` 路径 + 属性字符串二进制名 + INFRA 伪节点 + 钉底类），在 dev 上导出。
- 按 §9.1 / §9.4 设想的改动删除或改写相应的边（类初始化骨架去按类展开、描述符化后不再产生的签名边等），重算 SCC，得出模拟后最大 SCC 的类数、占比与体量，按 §9.7 的实测点折算峰值。模拟与 22% 不符时以模拟为准修正计划。
- 依赖图的提交与档案尽量对齐 4.9 GB 那次实测（10-08 CollectorsDemo，sg2，`csimpl-160789c7`，见 `docs/reports/2026-10-07-closure-composition.md`）；不能复现同一提交时写明差异，并在当前 main 上补测一次实际峰值作为新标定点。
- 模拟脚本提交进仓库（`scripts/` 下，附用法），S7-4 实施后用它重跑，对照预测与实测。

**边分类**（对剩余 SCC 逐类计数，并算「只删这一类后 SCC 还剩多少」）

| 边的类型 | 是否本质上无环 |
|---|---|
| 父类、实现的接口 | 继承关系本身无环 |
| 方法签名中的参数和返回类型 | 可以被消除 |
| 字段类型 | 引用类型的字段可以被消除 |
| trait 实现（孤儿规则：impl 必须放在类型或 trait 所在的 crate） | 一般跟着继承关系走，通常无环 |
| 类初始化骨架 | S7-4 处理的就是这一类 |

- 不属于上表的边（手写 INFRA 引用、From 转换、方法体、属性元数据字符串等）单列，说明归入哪一类或为何独立成类。
- 验证「继承与 trait 实现无环」：图上只保留这两类边求 SCC，应全是单点；否则列出成环的类。**按实际 impl 放置位置构图**（impl 放在接口所在 crate 时依赖方向反转）；不通过时指出是哪条 impl 的放置导致。
- 列出剩余 SCC 的关键枢纽类及其边。

**组合删除**（单类删除可能因其他类边仍连着而几乎不变，不能只凭单类删除下结论；单类逐删保留作补充）
- ① 当前图（基线）；
- ② 删 S7-4 处理的边（路线一）；
- ③ 路线一 + 删方法签名边；
- ④ 路线一 + 删字段类型边；
- ⑤ 路线一 + 同时删方法签名与字段类型边（即类型标记 crate 路线）。

每种组合的最大 SCC 报三项：类数、生成代码体积（D8 已有体量口径）、估算编译峰值。峰值以两个实测点标定（上段约 330 类 / 约 1.27 GB，底段约 4.9 GB），按体积而非类数拟合，写明拟合方法与误差范围，直接回答「能否放进 1.3 GB」。

**类型标记 crate 路线评估**（剩余环主要来自签名与字段时）
- 新增最底层 crate，只放全部类的类型标记（不透明句柄类型 / 描述符，不含方法与字段布局）；各声明 crate 的方法签名与引用类型字段只引用标记 crate 中的类型，签名边与字段边不再跨声明 crate 成环。用模拟验证 SCC 能压到多小（目标接近完全消除）。
- 需回答六个问题：
  1. 标记类型如何与 S7 已有的 `__Ref` / `__Handle` / `__ClassDesc` 句柄形态衔接，静态类型 → 视图转换的类型安全如何保持；
  2. 分派与视图转换走标记后有无额外运行期开销（目标 0 或常数）；
  3. 可读层命名是否仍符合翻译对照规范（Java 开发者可读、不出现底层调用）；
  4. 标记 crate 自身的编译峰值；
  5. 与孤儿规则的冲突点及解决办法；
  6. 缓存与增量编译：标记 crate 固定覆盖整个 java.base（每个 rava 版本生成一次，可预编译、可缓存）还是按档案生成？按档案生成是否使 java.base 预编译缓存失效、代价多大？结合档案模型（CLAUDE.md「只分析档案调用链上的方法内部依赖」、`docs/plans/2026-10-01-cross-test-compile-reuse.md`）给出推荐，并评估固定全集方案对标记 crate 体积与峰值的影响。

**汇报形式**：一张表，每行一种组合（①–⑤），列为最大 SCC 类数、代码体积、估算峰值、是否满足 1.3 GB；表后附边分类统计、枢纽类、类型标记 crate 六问答复与推荐方案。表与结论写进本节并推送，汇报后等确认再实施。

#### 9.8.1 模拟结果（2026-10-11，S7-4 分支 s7-4）

**数据与口径**
- 工具（均在 `scripts/` 下，用法见各文件头）：
  - `decl_scc_sim.py`：读生成的声明层，按 D8 同一张图建图，复现实际分段后做组合删边、单类删除、咽喉与贪心切边；
  - `decl_scc_full.py`：class 文件图，用于可达集与全集对照，并产出祖先清单；
  - `decl_scc_parse.py` / `decl_scc_graph.py` / `decl_scc_choke.py`：上述两者的模块；
  - `marker_crate_probe.py`：标记 crate 探针；
  - `rustc_profile.sh`：新增 `CRATE=` 选择 crate，支持 Linux；
  - `profile_union.py merge`：已对齐现 closure.json 格式。
- 单程序：CollectorsDemo，提交 0c25319e（作业 s7d-arch-0c25319e，dev），JDK 21.0.11。617 个 JDK 类，`java_base_decl` 不分段（≤ 650）。模拟器复现实际分段：一致。
- 档案：e2e 全体 1100 例（不含 63_junit），单例闭包经 `profile_union.py merge` 合成并集（提交 87954a78，作业 s7d-union3-87954a78，dev），用户类取 CollectorsDemo。ARCH_DESC
  - 注意：59d84dfe（batch-1010v，noreturn 定论窗口）合入后闭包变小，基线已换；本节档案数据早于它。
  - 前两次合成并集：52b813ae 为 7571 类，f77990e0 为 6822 类。
- 峰值换算：峰值 MB = 406 + 193.9 × 源码 MB（debug，三个实测点 22.69 MB→4806、3.29→1032、3.10→1019，最小二乘）。
  - 最大残差 12 MB（1.2%）；1.3 GB 对应源码 ≤ 4.61 MB。该拟合在中段偏保守，修正见下文「标定修正」。
  - 误差范围：3–5 MB 区间有两个实测点，按 ±5% 计；5 MB 以上是外推，按 ±10% 计。
  - 「是否 ≤ 1.3 GB」按区间上沿判定。

**统一结论表（单程序 CollectorsDemo；d8 = 现行 INFRA，placed = 共置手写随宿主）**

| 组合 | 保留的边 | 底段类数 | 源码 MB | 估峰 MB（误差） | ≤1.3 GB | 最大上段（类 / 估峰） | 去 INFRA 后最大 SCC |
|---|---|---:|---:|---|---|---|---:|
| ①/d8 | 全部 | 383 | 5.07 | 1390（±139） | 否 | 234 / 821 | 279 |
| ②/d8 | inherit + hidden + sig + field | 267 | 3.97 | 1176（±59） | 是（上沿 1235） | 350 / 1035 | 127 |
| ③/d8 | ② − sig | 227 | 3.68 | 1119（±56） | 是 | 390 / 1092 | 31 |
| ④/d8 | ② − field | 234 | 3.73 | 1129（±56） | 是 | 383 / 1082 | 92 |
| ⑤/d8 | inherit + hidden | 152 | 2.93 | 974（±49） | 是 | 465 / 1237 | 1 |
| ①/placed | 全部 | 364 | 4.88 | 1352（±135） | 否 | 253 / 858 | 305 |
| ②/placed | 同 ② | 246 | 3.72 | 1127（±56） | 是 | 371 / 1084 | 155 |
| ③/placed | 同 ③ | 208 | 3.43 | 1071（±54） | 是 | 409 / 1140 | 65 |
| ④/placed | 同 ④ | 211 | 3.32 | 1051（±53） | 是 | 406 / 1160 | 123 |
| ⑤/placed | 同 ⑤ | 124 | 2.42 | 875（±44） | 是 | 493 / **1336** | 2 |

**档案（并集）底段**

- 来源：
  - s7d-union3-87954a78：1100 例中 1088 例产出入口，12 例失败；入口阶段 2197 s；`profile_union.py merge` 82 s，RSS 32 GB；并集 7571 类、45699 方法。
  - s7d-arche-87954a78：从并集 `rava emit`，用户类取 CollectorsDemo（`--classes`）；7569 个 JDK 类，emit 9.4 s，RSS 2.3 GB。
- java.base 5145 类，D8 现分 4 段（底段 `java_base_decl` 3593 类、38.4 MB；上段 3 个各约 517 类）。模拟器复现分段：一致。
- 提交 87954a78 早于 59d84dfe 的 noreturn 修复，新基线下档案会略小。
- 「插值估峰」按下文「标定修正」的分段插值计算；「拟合估峰」是线性上界。

| 组合 / INFRA | 底段类 | 源码 MB | 拟合估峰 MB | 插值估峰 MB | ≤1.3 GB（插值） | 最大上段（类 / MB） | 去 INFRA 后最大 SCC |
|---|---:|---:|---:|---:|---|---|---:|
| ①/d8 | 3593 | 38.03 | 7780 | 8161 | 否 | 517 / 3.86 | 3324 |
| ②/d8 | 1009 | 12.57 | 2844 | 2593 | 否 | 591 / 7.64 | 590 |
| ③/d8 | 609 | 8.66 | 2086 | 1738 | 否 | 648 / 8.27 | 89 |
| ④/d8 | 861 | 11.20 | 2578 | 2293 | 否 | 612 / 7.88 | 463 |
| ⑤/d8 | 288 | 5.00 | 1375 | 1211 | 是（余量 7%） | 607 / 8.07 | 1 |
| ①/placed | 3525 | 37.49 | 7675 | 8043 | 否 | 539 / 4.18 | 3330 |
| ②/placed | 853 | 10.52 | 2445 | 2145 | 否 | 613 / 7.82 | 724 |
| ③/placed | 392 | 5.66 | 1504 | 1279 | 临界 | 594 / 7.71 | 199 |
| ④/placed | 686 | 8.69 | 2090 | 1744 | 否 | 637 / 8.39 | 571 |
| ⑤/placed | 120 | 2.45 | 882 | 944 | 是 | 628 / 8.51 | 2 |

- 档案的手写 1.06 MB（其中基础设施 0.53 MB），伴生宿主 93 个，INFRA 直连 d8 / placed 为 200 / 59。placed 与 d8 的差距远大于单程序：共置手写随宿主后，⑤ 从 288 类降到 120 类。
- 单删一种（d8，基线 3593）：macro 2285、nest 2655、sig 3337、field 3416、body 3453、inherit 3517，其余不变。只留 inherit 加一种（placed）：macro 1335、sig 684、nest 440、field 382、body 275。
- 咽喉：
  - ①/d8 的 INFRA 支配子树，前 1 / 5 / 10 / 25 覆盖 88 / 290 / 419 / 655 类；最大入口是 ModuleReader 87、VarHandle 60、RuleBasedCollator 53、Pattern 53。贪心切 10 条边，只到约 3490 类。
  - ⑤/placed：前 25 只覆盖 64 类。仍然没有咽喉。
- 只留 inherit、按放置构图：非单点 SCC 0 个。
- 上段：各组合按 650 类切出的上段都在 7.6–8.5 MB（插值约 1.5–1.7 GB），**上段同样超线**，体量上限（§9.8.3 第 1 条）对档案同样必需。
- **档案结论**：
  - 路线一（②）在档案上远不够（2145–2593 MB）。
  - 「路线一 + 删签名边」（③/placed）临界，1279 MB。
  - 只有 ⑤ 稳定在 1.3 GB 以内：d8 1211、placed 944 MB。
  - 档案底段收窄的必要条件：共置手写随宿主放置（placed），并且删掉签名边；字段边最好也删。

- 上段按 650 类上限切分。⑤ 的上段平均每类体量大（底段剩下的都是带签名的大类），493 类已达 4.8 MB（线性拟合约 1336 MB，插值约 1190 MB）。底段收窄后，上段体量随之变大。实测还表明，现状里 617 类的不分段 crate 已到 1.46 GB。**D8 的段上限应从类数改为源码体量（≤ 5.5 MB，见 §9.8.3 第 1 条）**。
- 边分类（单程序 ①/d8 底段内的边数）：inherit 707、sig 1343、field 660、body 192、nest 116、exc 114、macro 38、top 23、hidden 3、attr 0。
  - 不在任务表里的种类是 body（声明层里的类初始化骨架与常量引用）、nest（内部类属性）、exc（throws）、macro（宏隐式 use）、top（顶层项）。它们都是 S7-4 路线一要去掉的按类展开，归入「类初始化骨架」一类，② 中全部为 0。
  - INFRA 边（手写引用）单列：d8 直连 110 类；placed 直连 64 类，另 43 个伴生宿主。
- 单删一种（d8 底段类数，基线 383）：nest 316、body 343、field 356、inherit 364、sig 369、macro 376、exc 380，其余 383。没有哪一种能单独决定规模，与预期一致，结论只看组合。
- 只留 inherit 加一种：sig 234、field 226、nest 198、macro 173、body 167、exc 156、hidden 152。sig 与 field 是 ② 之后剩下的主要成环来源。
- **继承与 impl 无环验证**：按实际放置构图（impl 随类型所在段、共置手写随宿主），只留 inherit 一类时，非单点 SCC 为 0 个。通过。
- 枢纽与咽喉（`--hubs` / `--cuts`，按 INFRA 支配树计算）：
  - ①/d8：类入口前列为 Formatter 31（INFRA 直连）、Pattern 19、Class 16、StreamSupport 16、ConcurrentHashMap 11。贪心逐条切 10 条边只从 383 降到 339 类（估峰 1390 → 1327）。
  - ②/d8：单条边收益最大的是 Formatter→Pattern（field，13 类），其后 Class→ClassRepository 6、ReentrantLock→Sync 5、INFRA→System 4。切 10 条后 267 → 220 类（1176 → 1100）。
  - ⑤/d8：INFRA 支配子树前 1 / 5 / 10 / 25 只覆盖 4 / 17 / 27 / 44 类，底段就是「INFRA 直连类及其祖先」，没有咽喉。
    - ⑤/placed：INFRA→System 一条边独占 15 类（System 静态字段的类型），切 10 条后 124 → 89 类（≈773）。
  - 与 c1d §4「咽喉不成立」同构：没有一处可切的咽喉，收窄只能靠整类边的删除（组合），不能靠逐点切断。
- 与 §9.4 预测对照：§9.4 预测收窄后底段约占 22% 的类。模拟结果：② 为 267 / 617 = 43%，⑤ 为 152 / 617 = 25%。只删类初始化骨架达不到 22%，要再去掉签名与字段边才接近，以模拟为准修正。
- **标定修正（10-11，s7d-prof1-f0b2c9de）**：在 59d84dfe 之后的基线上实测，不分段的 `java_base_decl`（617 类，7.39 MB）峰值 1.46 GB（rustc 本身；`cargo --timings` 1.54 GB），线性拟合给出 1839 MB，高估 26%。峰值对体量是凸的：中段（3.3–7.4 MB）斜率约 104 MB/MB，大体量段（7.4–22.7 MB）约 219 MB/MB。
  - 改用实测点分段线性插值后，1.3 GB 对应源码约 **5.9 MB**（线性拟合给的是 4.61 MB）。
  - 上表「估峰」一列保留线性拟合，作为保守上界。按插值：
    - ①/d8 底段 1218 MB，①/placed 1198 MB，单程序在现图上已 ≤ 1.3 GB；
    - ② 1103、③ 1073、④ 1078、⑤ 994 MB；
    - ⑤ 两种放置的上段为 1135 / 1190 MB。
  - 全集按插值：① 12187、② 2389、③ 1302、④ 2031、⑤ 975 MB，「只有 ⑤ 与集合无关」的结论不变。
  - 分阶段实测（峰值在展开、类型检查、借用检查三段累积，LLVM 段不抬峰）见 `docs/plans/2026-10-01-rustc-memory-and-crate-split.md` §7.8。
- 标定点说明：10-08 的 csimpl 点（23.6 MB → 4903 MB）是 S7 前的形态，没有参与拟合。用拟合式外推得 4982 MB，偏差 1.6%，说明「按体量拟合」跨形态成立。

**第 1 项：可达集 vs 全集**

- 10-08 那次 4.9 GB（`csimpl-160789c7`，CollectorsDemo，`java_base_decl` 源码 23.6 MB）是单程序可达闭包（档案化之前，单例约 2178 类），不是 java.base 全集（7573 类，jimage）。当时是 S7 之前的声明形态：每类声明体量约为现形态的 4 倍。
- 全集无法直接发射，两次实测：
  - API 面全集闭包：s7d-full-0c25319e，dev#3，40 GB 槽位被 OOM 杀（`peak=40960M`，emit 段 1397 s，作业 1710 s）；
  - 档案 `rava profile --closure ×1090` 并集：s7d-arch3-52b813ae，17 s 内 OOM（`41.8 GB`）。
- 因此全集底段改在 class 文件图上离线计算（`scripts/decl_scc_full.py`，参考 JDK 21.0.11 的 java.base 7571 类）。边种类与生成文件图对齐，见下表。可达集一侧先与生成文件图对照：组合 ②–⑤ 的底段类数差 ≤5，口径成立。① 的 class 图把常量池全部引用都记成 body 边（相当于方法体全译），只作上界。

可达集为 CollectorsDemo 单例，0c25319e，617 个 JDK 类；全集为 java.base 7571 类。估峰按下文拟合；全集里不在可达集的类，按可达集平均声明字节外推。

| 组合 / INFRA | 可达集底段（生成文件图） | 可达集底段（class 图） | 估峰 MB | 全集底段（class 图） | 其中外推 | 源码 MB | 估峰 MB | 全集 ≤1.3 GB |
|---|---:|---:|---:|---:|---:|---:|---:|---|
| ①/d8 | 383 | 506 | 1687 | 5478 | 4972 | 56.44 | 11350 | 否 |
| ②/d8 | 267 | 265 | 1174 | 1032 | 761 | 11.64 | 2662 | 否 |
| ③/d8 | 227 | 222 | 1115 | 445 | 220 | 5.88 | 1547 | 否 |
| ④/d8 | 234 | 232 | 1127 | 861 | 615 | 10.00 | 2345 | 否 |
| ⑤/d8 | 152 | 150 | 973 | 151 | 1 | 2.93 | 975 | 是 |
| ②/placed | 246 | 245 | 1126 | 999 | 742 | 11.24 | 2585 | 否 |
| ⑤/placed | 124 | 123 | 874 | 124 | 1 | 2.42 | 876 | 是 |

结论：只有 ⑤（只剩继承边加隐藏字段）的底段与集合大小无关：全集与可达集几乎相同，都是 INFRA 直连类及其祖先。②–④ 随集合增长：全集是可达集的 2–4 倍，超出 1.3 GB。因此「固定全集预编译」只有在 ⑤ 的形态下才能把底段压在 1.3 GB 内。

- 全集形态直接发射不可行（数据点）：s7d-full-0c25319e（dev#3，提交 0c25319e）以 java.base 全部 API 包为入口（`--api-package … --api-recursive`），jimage 列出 java.base 7573 类之后，闭包 / 发射阶段被 OOM 杀：`__RAVA_OOM_KILL__ limit=40960M peak=40960M`，emit 段 1397 s，作业共 1710 s，没有产出声明层。全集形态下仅发射器峰值就超过 40 GB。全集底段因此不再走 emit，改在 class 文件图上离线计算（`scripts/decl_scc_full.py`）。


#### 9.8.2 类型标记 crate 路线


前提先说清：标记 crate 里放什么，决定了它能不能断环。

- 现 wrapper 是 `pub struct X { pub __r: __Ref<dyn X__VTable> }`（S7-2，`handle.rs`）。类型参数 `dyn X__VTable` 是本类 vtable trait，trait 方法签名写着其他类的具名类型。所以 wrapper 原样搬进标记 crate，标记 crate 就得带上全部 vtable trait，也就带上全部签名边，环原样回到标记 crate。
- 标记 crate 能放的只有与签名无关的部分：`#[repr(transparent)] struct X { h: __Handle, _p: PhantomData<..> }`、`BINARY_NAME`、上转 `From<X> for Anc` 与身份语义的 std impl（`Clone` / `PartialEq` / `Debug`）。
- **硬约束是固有 impl，不是孤儿规则**：`impl X { pub fn speak(&self) .. }` 必须与 `struct X` 同 crate（§9.2）。struct 在标记 crate，可读层方法就不能写在各声明 crate 的固有 impl 里。只有两条出路：
  - (i) 每类一个方法 trait，定义并实现在声明 crate，即 §9.3 的方案 B；
  - (ii) 声明 crate 另定义可读类型 X，签名里引用他类时写标记类型。这样可读层会出现两套类型，方法体里要写转换。

1. **与 `__Ref` / `__Handle` / `__ClassDesc` 的衔接、类型安全**
   - 标记 = `__Handle` 加零尺寸类型参数，与 S7 句柄同布局（`repr(transparent)`），边界转换零成本。`__ClassDesc` 仍按类放在声明 crate（`display` / `supertypes` 是数据，不产生签名边）。
   - 视图指针 `vt: NonNull<dyn X__VTable>` 不能放进标记，因为它的类型就是签名边的载体。分派入口须在声明 crate 内由句柄现取视图，有两种取法：
     - 按 §3.1 的 A：经 `ObjectVTable` 槽位按类 id 取 `&dyn X__VTable`；
     - 标记里存擦除的 `NonNull<()>` 数据指针加 vtable 指针，在声明 crate 里重组胖指针。重组依赖胖指针布局（`ptr::from_raw_parts` 未稳定），不可取。
   - 类型安全：静态类型 → 视图的转换只发生在声明 crate 的 checkcast / `From<Object>`（描述符判定）里，标记 crate 不提供任何构造入口。不变式「标记所持对象是该类实例」由构造点保证，与现 `__Ref::from_raw` 同等级。
2. **运行期开销**：按取法 A，每次虚分派多一次间接调用加一次类 id 比较，比现状（视图指针 O(1) 直取）多一个常数。现状 B 形态的「0 额外开销」保不住，除非在调用点缓存视图（缓存要随句柄走，又回到 vt 字段）。结论：常数开销，非 0。
3. **可读层与命名合规**
   - 取 (i)：调用写法 `animal.speak()` 文本不变，但每类新增一个 Java 命名空间之外的方法 trait，违反 CLAUDE.md「代码生成命名原则」第 1 条。生成器还要逐文件精确导入，否则同名方法（`toString` 等）的 trait 探测会退化。须用户裁决。
   - 取 (ii)：方法体出现标记类型与 `.into()` 转换，违反翻译对照规范 §16 的可读性判定。
   - 两条都不满足「可读层不变」。
4. **标记 crate 自身峰值**：见本节末「标记 crate 探针」。
5. **孤儿规则**
   - `From<X> for Anc`：两者都在标记 crate，impl 也放在标记 crate，可以。标记 crate 需要知道每类的祖先，这是数据，无环。
   - `From<Object> for X`（checkcast）要读 `X::__DESC`（声明 crate）：X 与 `From` 都不在声明 crate，在声明 crate 写 impl 违反孤儿规则。只能改成声明 crate 里的自由函数或固有方法，调用点写法随之改变（又落到第 3 问）。
   - `impl ObjectVTable for X__inner`、`impl X__VTable for X__inner`：`X__inner` 在声明 / 实现层，本地类型，可以。
   - 冲突点是 checkcast 的 `From<Object>` 与第 3 问的固有方法；前者可改走描述符非泛型函数，后者无解（只能方法 trait）。
6. **缓存、增量编译与预编译 java.base**：见下文「第六问」。


**第六问：固定全集还是按档案；预编译 java.base**

档案模型（`2026-10-01-cross-test-compile-reuse.md` §1.4 / §4.1）的几个事实：
- 语料档案是 e2e 全集入口的调用链并集，JDK 侧 3609 类（全模块）。
- L1 键 P = (JDK, 生成器, 运行时, 档案内容摘要)，L2 键 B = (crate 源码, rustc, 编译选项)。
- 实测每新增一个测试，档案平均增长约 0.46 类，P 随之变化，档案 crate 重建。

三种做法对比：

| 做法 | 缓存键 | 失效时机 | 体积与峰值 | 与 CLAUDE.md 第 2 条 |
|---|---|---|---|---|
| A. 标记 crate 按档案生成 | 同 P | 档案一变，标记 crate 与各声明 crate 一起重建。标记 crate 没有带来新的失效来源 | 档案规模（语料 3609 类），见第四问 | 符合 |
| B. 标记 crate 固定覆盖 java.base 全集，声明层仍按档案 | 只依赖 (rava, JDK, rustc) | 跨档案永远命中，可以预编译 | java.base 全集约 7573 类，见第四问 | 不符：纳入档案外的类型（无行为，但仍属「不在调用链上的类加入生成范围」） |
| C. 整个 java.base 预编译（声明层加方法体全集） | 只依赖 (rava, JDK, rustc) | 永远命中 | 见 §9.8.1 第 1 项：全集底段远超 1.3 GB；API 面全集闭包分析在 40 GB 槽位上被 OOM 杀 | 不符，并且与「闭包正确且最小、以档案规模衡量」直接冲突 |

结论与推荐：
- B 只把最小的一层固定下来。声明层与方法体仍随 P 失效，预编译省下的只是标记 crate 自身的那一次编译（第四问的量级），换来的却是违反第 2 条，不值得。
- C 在体量上就不成立（全集底段见下表），是另一条线（发行预编译产物），不应作为本节收窄手段。
- 如果走标记路线，标记 crate 应按档案生成（A），与声明层共用 P。预编译 java.base 不作为本路线的理由。


**标记 crate 探针**（`scripts/marker_crate_probe.py`：按类名与祖先清单生成标记 crate，每类有 `repr(transparent)` 句柄 struct、`BINARY_NAME`、std 身份 impl，以及到每个祖先的 `From` 上转）
- java.base 全集（7571 类），不含上转：源码 7.79 MB，峰值 1.04 GB，5.5 s（本机，debug）。
- dev 实测（s7d-prof1-f0b2c9de，debug，rustc 1.99.0）：
  - java.base 全集含上转：7571 类，23263 个 `From`，lib.rs 13.07 MB，峰值 **1.46 GB**，10.7 s；
  - CollectorsDemo 可达集：615 类，1094 个 `From`，0.81 MB，峰值 0.21 GB，0.55 s。
- 标记 crate 没有过程宏展开与 trait 方法体，每 MB 的峰值远低于声明层。全集形态略超 1.3 GB，主要是祖先上转的 `From`（每类平均约 3 个）；语料档案（java.base 5145 类）按类数比例约 1.0 GB，低于 1.3 GB。
  - 第四问的答复：标记 crate 按档案生成时峰值不是约束；固定全集时约 1.46 GB，需把上转改成按需生成，或拆成两个 crate。
- 标记 crate 自身体量随「类数 × 平均祖先数」增长，不随签名增长。

**分阶段实测**（第 3 项）：见 `docs/plans/2026-10-01-rustc-memory-and-crate-split.md` §7.8。

#### 9.8.3 推荐方案（2026-10-11 用户裁决：路线 A，已定）

> **用户裁决（2026-10-11）**：类型标记 crate 路线，**用户选路线 A**。
> - 第 3 问取出路 (i)：每个类一个方法 trait，定义并实现在声明 crate。这个 trait 和 `ObjectVTable` 一样只是生成层的内部实现细节：不作为公开 API，不出现在可读层的方法体文本里（调用仍写 `animal.speak()`），命名不带 `Jvm` 前缀。
> - 接受常数级的分派开销（按 §3.1 取法 A，经 `ObjectVTable` 槽位按类 id 取视图）。
> - 标记 crate 按档案生成（第六问做法 A），不固定为 java.base 全集。
> - 语料档案的目标形态是 ⑤：删掉签名边和字段边，档案共享一个 java.base 底段。
>
> 实施分解见 §9.9。下文第 3 条中「须用户裁决」各项均已按上述裁决定案。


1. **D8 的段上限从「650 类」改为「源码体量」，立即需要，与路线选择无关。**（10-11 已实施，分支 d8-mb：先按 5.5 MB 常量，同日改为上游感知预算，实测见 §9.8.4）
   - 现状就已越线：CollectorsDemo 的 `java_base_decl` 617 类低于 650，所以不分段；源码 7.39 MB，实测 1.46 GB，超过 1.3 GB。
   - 按修正标定，体量上限取 **≤ 5.5 MB**（1.3 GB 对应约 5.9 MB，扣插值误差留 7% 余量）。
   - 按体量切分后，单程序在现图（①）上底段约 5.07 MB / 1218 MB，已满足。⑤ 的上段也不会再出现 493 类、4.8 MB 这样的超大段。
2. **生产构建（单项目档案）可以加做路线一 ②**：删类初始化骨架等按类展开的边，保留签名与字段边。
   - 底段 383 → 267 类，按插值 1218 → 1103 MB。余量从约 6% 增到约 15%，可读层与命名原则不受影响。
   - 收益是余量与段数，不是能否达标的决定因素。
3. **语料档案必须走到 ③/placed 以上，稳妥的是 ⑤**（档案表见 §9.8.1）。
   - ② 在档案上为 2145–2593 MB；③/placed 1279 MB，临界；⑤/placed 944 MB。全集 class 图上同样只有 ⑤ 与集合无关。
   - 前提：D8 把共置手写随宿主放置（INFRA 只留真正的基础设施），档案 ⑤ 从 288 类降到 120 类。
   - 删签名边（③）与删签名、字段边（⑤）都需要类型标记 crate 路线。该路线的第 3 问（每类方法 trait 违反命名原则第 1 条，或可读层出现双类型、违反 §16）与常数级分派开销须用户裁决。若采纳，标记 crate 按档案生成（第六问做法 A），峰值不构成约束（语料档案按比例约 1.0 GB）。
   - 不采纳时：语料档案不能整体共享一个 java.base 底段，只能退回按程序（或按测试组）的更小档案，与 `2026-10-01-cross-test-compile-reuse.md` 的档案复用目标冲突。须一并裁决。
4. **咽喉切边不作为手段**：各组合下贪心切 10 条边，只降 5–18%。底段由 INFRA 直连类及其签名闭包整体构成，与 c1d「咽喉不成立」一致。

**后续**
- 档案级（并集档案 scratch）的分阶段实测（作业 B）本轮没有做，S7-4 实施后用 `scripts/rustc_profile.sh` 的 `CRATE=` 补测。
- 实施后用 `scripts/decl_scc_sim.py` 重跑生成层图，对照本节预测，并用新实测点更新标定。

#### 9.8.4 D8 按体量切分的实施与实测（2026-10-11，分支 d8-mb）

**实施**（33e23c00；`rustc_profile.sh` 修正 4461a0d3）
- `decl_segments.rs`：`DECL_SEGMENT_CLASSES = 650` 换成 `DECL_SEGMENT_BYTES = 5_500_000`（按 §9.8.3 第 1 条与 §9.8.1「标定修正」取值，常量注释写明来源）。
- `plan` 按各类权重切段；`cut` 改为两步：先按上限贪心取最少段数 k，再二分求 k 段内最小的段上界，按它贪心切。
  - 每段 ≤ 上限，只有单个分量本身超限时才独占一段。
  - 段号单调、无空段；只依赖拓扑序与权重，结果确定。
  - 原中点均衡法在体量不均时会超限（例如 6+6+6、上限 10，原法切出 12 的段）。
- `decl_side.rs`：权重 = 类文本落盘字节（`Writer` 原样写出 `em.text`，两者一致）+ 该类落在上段时 mod 行的上界。
  - mod 行上界按「本类独占新包」逐类全额计入：包头、`pub use <前段>::<包>::*;`、各级 `pub mod`。只会偏大，有单测守护。
  - 另预留 lib.rs 固定行 1 KB。
  - 「不分段」判定计入手写真源全量（不分段时底段 crate 含全部 overlay）。
  - 底段（含 INFRA 的 SCC）原样不切。
- 模拟器同步：`decl_scc_sim.py` / `decl_scc_graph.py` / `decl_scc_parse.py` 改为按同一权重和同一切法复现 D8。
- `rustc_profile.sh` 有测量缺陷：依赖步原先按 `-p <依赖名>` 逐个编，特性合一与目标 crate 的单元图不同，计时步会重编前段，`/usr/bin/time` 把前段峰值算进来（首轮 d8mb-prof-33e23c00 上段 1.02–4.22 GB 均为失真值）。
  - 修正：依赖步改为 `cargo build -p <crate>`；另输出本 crate rustc 的 time-passes 最大 RSS（`rustc_peak`）。

**实测**（dev，debug，rustc 1.99.0 stable + `RUSTC_BOOTSTRAP=1`；作业 d8mb-prof2-4461a0d3 / d8mb-prof3-4461a0d3；源码 = crate `src/` 下全部 .rs 字节；峰值 = 本 crate rustc 的 time-passes 最大 RSS，`/usr/bin/time` 值括注）

| 例 | crate | 类 | 源码 MB | 峰值 MB |
|---|---|---:|---:|---:|
| HelloWorld | `java_base_decl`（底段） | 342 | 4.62 | 1122（1.06 GB） |
| HelloWorld | `java_base_decl_1` | 237 | 2.17 | 604（0.57 GB） |
| CollectorsDemo | `java_base_decl`（底段） | 384 | 5.12 | 1191（1.13 GB） |
| CollectorsDemo | `java_base_decl_1` | 234 | 2.16 | 576（0.56 GB） |
| DeepCopy | `java_base_decl`（底段） | 1895 | 21.82 | **4417**（4.31 GB） |
| DeepCopy | `java_base_decl_1` | 369 | 2.76 | 938（0.91 GB） |
| DeepCopy | `java_base_decl_2` | 349 | 2.76 | 975（0.93 GB） |
| FWord / PartitionInteger / RailwayCircuit | `java_base_decl_1`（底段 17.3–17.8 MB） | 377–378 | 3.14 | 967–972 |

- 三例的全部非底段声明 crate ≤ 3.14 MB、≤ 0.98 GB，达标。
- CollectorsDemo 原先不分段（617 类，7.39 MB，1.46 GB），现切为底段 5.12 MB（1.19 GB）加一个上段，两者都 ≤ 1.3 GB。
- DeepCopy 底段 21.8 MB、4.4 GB：这是含 INFRA 的签名 SCC，D8 不可再切，归底段收窄（§9.8.3 第 2、3 条）。
- 抽查 d8mb-33e23c00（dev，33e23c00）：HelloWorld、DeepCopy、CollectorsDemo、TestSerialLookupPairing、FWord 5/5 通过。emit 单测（d8mb-ut-4461a0d3）91 例全过。

**第一轮遗留：5.5 MB 对上段不够保守**（已由下文「上游感知预算」解决）
- 上段的峰值同时取决于本段体量和上游链（底段加前面各段）的元数据体量。上游元数据在展开、解析阶段按需载入：
  - 同为 2.2 MB 左右的上段，上游 5 MB 时 macro_expand 后 RSS 为 214 MB，上游 22 MB 时为 336 MB。
- 7 个上段实测点手工拟合：峰值 MB ≈ −9 + 240 × 本段 MB + 13 × 上游 MB，残差 ≤ 32 MB（HelloWorld 上段 −32，其余 ≤ 3）。
  - 每 MB 斜率 240，高于底段标定中段的约 104：底段含手写 overlay 和整类手写，每字节展开量小。§9.8.1 的标定只对底段成立。
- **探针验证**（d8mb-probe-6086a4e6，临时分支 d8-mb-probe 上限放到 6.0 MB，不合入）：DeepCopy 的上段合成一个 718 类、5.52 MB 的段，上游 21.8 MB。
  - 实测 **1597 MB**（`/usr/bin/time` 1.51 GB），拟合式预测 1599 MB。
  - 结论：段体量接近 5.5 MB 的上段会超 1.3 GB。现有三例上段只有 2.2–3.1 MB，所以没有触发。
- 按拟合式，≤ 1.3 GB 的本段上限是 (1309 − 13 × 上游 MB) / 240：
  - 上游 5 MB 时为 5.2 MB；
  - 上游 22 MB 时为 4.3 MB；
  - 上游 40 MB（语料档案底段量级）时为 3.3 MB；
  - 上游 60 MB 时为 2.2 MB。
- 协调者 10-11 裁决：采用上游感知预算（终态，覆盖语料档案），不用固定 4.0 MB 常量。

**上游感知预算（a1771258）**
- `decl_segments.rs`：
  - 新增 `PeakModel` 与常量 `UPPER_SEGMENT_PEAK`：峰值 = −9 MB + 240 × 本段字节 + 13 × 上游字节，限值 = `DECL_PEAK_TARGET_MB` 1300 − `DECL_PEAK_MARGIN_MB` 64 = 1236 MB。
  - 余量 64 MB 取标定最大残差 32 MB 的 2 倍：语料档案的上游（约 40–50 MB）超出标定区间（≤ 22 MB），属外推。
  - 注释写明标定来源（本节、8 个点、残差 ≤ 32 MB）。
- 切法 `cut`：逐段贪心装满。
  - 本段 = 固定行 1 KB + 各分量权重；上游 = 底段（手写真源全量 + 固定行 + 底段类权重）+ 前面各段。
  - 当前段非空且再装一个会使预测峰值超限，就开新段；单个分量本身超限时独占一段。
  - 段数最少：240 > 13，前段装得越满，后段可达位置越远，所以贪心最优。原「最少段数下均衡」的二分去掉了。
  - 只依赖拓扑序与字节数，结果确定。
- `DECL_SEGMENT_BYTES` = 5.5 MB 只保留为「整个声明层是否分段」的判定（底段形态，含手写 overlay，按 §9.8.1 标定）。
- 权重偏大（mod 行逐类全额计入），本段与上游都按上界计。两个系数为正，所以预测只会偏高。
- 模拟器 `decl_scc_graph.py` / `decl_scc_parse.py` 同步。
- 单测新增：
  - 预算随上游收缩；
  - 每段满足模型（单个超限分量除外）；
  - 探针压力（718 × 7.7 KB、上游 21.8 MB）必被切开且每段达标；
  - HelloWorld 量级不切。
- 各段本段预算（MB）= (1245 − 13 × 上游 MB) / 240：上游 5 MB 时 4.92，22 MB 时 4.01，40 MB 时 3.02，60 MB 时 1.94；上游 ≥ 95.8 MB 时无可行段。
  - 语料档案 java.base 声明层总量约 50 MB（§9.8.1：底段 38.4 MB 加 3 个上段各约 3.9 MB）。末段上游 ≤ 50 MB，预算 ≥ 2.5 MB，可覆盖。
  - 底段收窄后上游起点更低，预算更宽。

**复测**（dev，作业 d8mb-prof4-a1771258，口径同上；预测按上式，未扣余量）

| 例 | crate | 类 | 源码 MB | 上游 MB | 预测 MB | 实测峰值 MB |
|---|---|---:|---:|---:|---:|---:|
| HelloWorld | `java_base_decl_1` | 237 | 2.17 | 4.62 | 571 | 594（0.56 GB） |
| CollectorsDemo | `java_base_decl_1` | 234 | 2.16 | 5.12 | 576 | 576（0.56 GB） |
| DeepCopy | `java_base_decl_1` | 518 | 3.92 | 21.82 | 1215 | **1221**（1.14 GB） |
| DeepCopy | `java_base_decl_2` | 200 | 1.60 | 25.74 | 711 | 725（0.69 GB） |
| FWord | `java_base_decl_1` | 377 | 3.14 | 17.35 | 970 | 967（0.94 GB） |

- 底段与第一轮相同：HelloWorld 4.62 MB / 1122 MB，CollectorsDemo 5.12 MB / 1191 MB，DeepCopy 21.82 MB / 4417 MB，FWord 17.35 MB。
- 全部上段 ≤ 1.3 GB。新增 3 个点的残差 +6、+14、−3 MB，在 64 MB 余量内。
- **探针同等压力**：DeepCopy 上段（718 类，共 5.52 MB，上游 21.8 MB）在 6.0 MB 探针下合成一段，实测 1597 MB。现预算切为 3.92 MB + 1.60 MB 两段，实测 1221 / 725 MB。
  - 第 1 段装到预算边缘（预测 1215 ≤ 1236），是模型在上限处的直接验证。
- 抽查 d8mb-a1771258（dev）：HelloWorld、DeepCopy、CollectorsDemo、TestSerialLookupPairing、FWord 5/5 通过。emit 单测（d8mb-ut-a1771258）93 例全过。
- 遗留：
  - 13 × 上游的系数只在上游 ≤ 26 MB 内实测，档案量级（40–50 MB）待 S7-4 档案构建时复测。上游 ≥ 95.8 MB 时镜像链本身越线，需改为按实际依赖连段。
  - DeepCopy 底段 4.4 GB 归底段收窄。


### 9.9 路线 A 的实施分解（2026-10-11，分支 s7-marker）

裁决见 §9.8.3。终态（⑤/placed）：
- **标记 crate `java_marker`**：按档案生成，覆盖档案内全部 JDK 模块的类。每类只有与签名无关的部分：`#[repr(transparent)]` wrapper `X { __r: __Handle, _p }`、身份类 std impl（`Default` / `Clone` / `PartialEq` / `Debug`）、`BINARY_NAME`、描述符 `X__DESC` 与 `X::__DESC`、`From<Object>`（描述符判定后包句柄）、`From<X> for Object`、`From<X> for Anc`（句柄搬移）、分配 / 偏移钩子的外部声明，以及标记层基础设施（K4）。
- **声明 crate**（D8 各段）：vtable trait `X__VTable`（签名里的类类型全部来自标记 crate）、方法 trait `X__Methods` 及其对 wrapper 的实现、静态存储与类初始化。方法 trait 的地位与 `ObjectVTable` 相同：生成层内部细节，可读层方法体文本不出现（调用仍写 `animal.speak()` / `Dog::new()`），名字不带 `Jvm` 前缀。
- 声明 crate 之间只剩继承边（`X__VTable: Super__VTable`，以及继承转发外壳调用祖先 vtable）与隐藏字段边。D8 机制不变，底段即「INFRA 直连类及其祖先」（模拟：档案 120 类、约 944 MB）。

**分派形态（§3.1 取法 A）**：wrapper 只持句柄。分派入口 `__vt()` 以本类的 display 深度（宏展开期已知的常数）问句柄所持运行时类存储：`ObjectVTable::__erased_vtable(&self, depth, slot)`，存储按 `depth` 走 `match`，只比较一次槽类型，再把自身经 supertrait 上转后的视图指针填入。开销是一次间接调用加一次类型 id 比较，常数级，与类层次深度无关。

每步可单独合入，都要通过宏单测、生成器单测（emit / instr / driver 相关）与 dev 抽查。抽查集：HelloWorld、CollectorsDemo、DeepCopy、TestDynamicProxy、TestAnnoReflect、TestSerialLookupPairing、TestXmlTransform，加各步注明的相关例。

| 步 | 内容 | 文件归属 | 退出条件 |
|---|---|---|---|
| **K1** 类 wrapper 去视图指针 | wrapper `{ __r: __Ref<dyn X__VTable> }` 改为 `{ __r: __Handle }`，删 `__Ref`。分派入口改为 wrapper 固有 `__vt()`，即上文的按深度取视图。上转改为句柄搬移（`Anc::__from_parts(child.__r)`）。`From<Object>` / `__virtual_view` 在描述符判定后直接包句柄，不再取视图。分配钩子返回 `__Handle`。引导映像常量改为 `__from_image(__Handle::image(..))`。invokespecial 的 `(x).__r.vt()` 改为 `(x).__vt()`。 | runtime `handle.rs`、`java/lang/object.rs` / `object_ext.rs`、`array/vtable.rs`；宏 `gen/wrapper/*`、`type_conversions.rs`、`storage_hooks.rs`、`struct_layout.rs`、`virtual_dispatch/*`、`interface.rs`、`rewrite.rs`；生成器 `instr/invoke/special.rs`、`emit/project/boot_image/values.rs` | wrapper struct、身份 impl、转换 impl 的展开文本不含 `__VTable`（宏单测守护）；单测与抽查全过，相关例加 InheritanceChain、InterfaceDispatch、TestCollections；§4.3 热循环 release 中位数报数（用户已接受常数开销，只报数不设门槛） |
| **K2** 接口载体去视图指针 | 接口载体只持 `Object`（`__IfaceRef` 删去 `vt`）。invokeinterface 入口每次经 `ObjectVTable::__interface` 现取视图：每次分派的开销等于原来建立载体时那一次查询，按运行时类实现的接口数有界，属常数级。引导映像的 `__IfaceRef::image` 随之去掉视图参数。 | runtime `handle.rs`；宏 `interface.rs`；生成器 `emit/phase2/sam_objects.rs`、`boot_image/values.rs` | 接口载体展开文本不含 `__VTable`；单测与抽查全过，相关例加 InterfaceDispatch、LambdaBasic、StreamDemo |
| **K3** 方法 trait（单 crate 内，struct 不动） | wrapper 固有 impl 中的可读层成员移入每类方法 trait `X__Methods`（与 wrapper 同类型形参），由 wrapper 实现。成员包括：Java 方法外壳、继承转发外壳、构造器 `new` / `__init_on`、静态方法、实例与静态字段访问器、`__class_init`、`_init_not_null`。生成器为每个文件精确导入用到的方法 trait（`use … as _;`）：调用点所属类取自字节码 invoke / 字段指令的 owner，也就是生成器已知的接收者静态类型，不做全量 glob。手写文件显式导入。 | 宏 `gen/wrapper/methods.rs` 及新子模块；生成器 `emit/imports`、`instr`（owner 记录）；手写 `runtime/**/_impl.rs` 的 `use` | 生成树对照（`compare_trees.sh`）：方法体文本逐字节不变，只有文件头 `use` 增加；静态调用点的形参推断与固有 impl 一致（宏单测）；声明层与方法体 crate 峰值不升（同机 ±5%）；`no_jdk_literals` 通过；单测与抽查全过 |
| **K4** 运行时基础设施分层 | 手写 INFRA 拆为两层。**标记层基础**新建手写真源 `runtime/java_marker/`，overlay 进标记 crate，包括 `__Handle`、`Object`、`ObjectVTable`（签名中的类类型为标记类型；缺省方法体中依赖行为的部分经 `unsafe extern "Rust"` 链接期声明转交行为层，与 `rava_layer` 拆层同一机制）、`__ClassDesc` / `__FieldDesc` / `__TypedNull`、`JvmError` / `Result`、`sync_model` / `obj_ref`、`JArray` 句柄。**行为层**留在 `java_runtime`。 | runtime 文件迁移、`overlay.rs`、`decl_side.rs`（crate 依赖）、`closure.toml` 等清单路径 | 标记层基础单独成 crate 并通过编译，`java_runtime` 依赖它，此时标记 crate 尚不含类；手写守护单测（`handwritten_vtable_impls` 等）改为扫两处；单测与抽查全过 |
| **K5** 类初始化骨架去按类展开（S7-4 路线一，即 ②） | 去掉声明层 body / nest / exc / macro / top 五类按类展开边（§9.8.1 边分类）。 | 宏 `class_init.rs` 等；生成器声明层发射 | 模拟器（`decl_scc_sim.py`）② 口径与发射一致；声明层展开不增；单测与抽查全过 |
| **K6** 标记 crate 生成 | `rava_layer` 增加 `marker` 层：同一 `java_class!` 块在标记 crate 只展开标记部分（见本节开头），`decl` 层不再展开这些。生成器按档案发射 `java_marker` crate。声明文件中只用作类型的引用写 `java_marker::` 路径；声明 crate 各包 `pub use` 标记类型，可读层与方法体文件的 `use` 文本不变。用户 crate、lib crate 仍按 `full` 单 crate 展开，形态相同。 | 宏 `gen/layer.rs`、`gen/mod.rs`；生成器 `emit/project/{decl_side,layout,module_side,lib_crates}.rs` 与新 `marker_side.rs` | 标记 crate 的展开文本不含 `__VTable` / `__Methods`；D8 图（只计 `crate::` 边）的底段 = 模拟 ⑤；HelloWorld / DeepCopy 声明 crate 全部 ≤ 1.3 GB；单测与抽查全过 |
| **K7** D8 共置手写随宿主（placed） | `<x>_impl.rs` / `_ext` 随宿主类进入其所在段，钉底只留 K4 后的行为层 INFRA 直连类。 | `decl_segments.rs`、`decl_side.rs`、`overlay.rs` | 模拟器 placed 口径与发射一致；单测与抽查全过 |
| **K8** 档案验收 | 语料档案（并集）构建，复测底段与全部上段峰值，对照 §9.8.1 预测（⑤/placed 底段约 944 MB），用新实测点更新标定与 D8 上游系数（§9.8.4 遗留）。 | 文档、`scripts/decl_scc_sim.py` | 所有声明 crate ≤ 1.3 GB（档案与 HelloWorld / DeepCopy 单程序）；§4.3 热循环报数 |

**依赖顺序**：
- K1 → K2 → K4 → K6 → K8；
- K3、K5、K7 与 K1、K2 互不依赖，K6 以 K3、K5 为前提，K8 以 K7 为前提。

**说明**：
- K1 不是过渡形态。wrapper 只持句柄就是终态 wrapper 的数据形态，K6 只把它整体挪进标记 crate。
- K3 同理：方法 trait 在单 crate 内先成形，K6 只改它所在的 crate。
- 任务说明里举的首步例子是「先建标记 crate、搬入句柄与上转」。它做不成独立首步：固有 impl 必须与 struct 同 crate（§9.2），struct 一旦搬走，声明 crate 的固有方法立即失效。所以先在单 crate 内去掉 wrapper 的签名依赖（K1、K2）、把方法面改为 trait（K3），再一次挪 crate（K6）。

**现状（2026-10-11）**：
- K1 已完成，提交 7cef9081（s7-marker）。
  - wrapper 只持 `__Handle`，`__Ref` 已删。
  - `__erased_vtable` 按深度 `match`。
  - 宏单测新增 `wrapper_holds_handle_only`，守护 wrapper 展开只含 `__Handle`、按深度取视图。
- 验证结果（dev）：
  - 宏单测 20/20 通过。
  - 生成器全量单测通过：rc=0，emit 93 例，耗时约 5570 s（JOBS=1）。
  - 抽查 12/12 通过：七例加 InheritanceChain、InterfaceDispatch、TestCollections、TestGenerics、Polymorphism。
  - TestXmlTransform 在缺省转译时限 600 s 下超时。超时发生在闭包期，K1 不改闭包。放宽时限（`--transpile-timeout 1800`）后通过。
- §4.3 热循环报数随 K2 一并测得（见下方 K2 节）。
- K2 已完成，提交 847493cb（s7-k2，基于 main c7cfb932）。
  - 接口载体只持 `__IfaceRef`，`__IfaceRef` 只持 `Object`，删去视图指针 `vt` 与接口类型形参，载体类型与接口签名无关。
  - invokeinterface 入口 `__IfaceRef::enter::<dyn I__VTable>()` 每次经 `ObjectVTable::__interface` 现取视图。查询协议（`Option<NonNull<dyn I__VTable>>` 槽）不变，所以 sam_objects 生成的 lambda 应答不用改。
  - `__IfaceRef::{new, image, null}` 合为一个 `const fn new(Object)`：载体缺省 null、`From<Object>`、引导映像常量都经它建立。
  - 引导映像的接口槽一律发射 `I::__from_image(__IfaceRef::new(<对象引用>))`，不再判定「对象的类是否有 `impl I for X` 块」。为此只服务引导映像的 `ClassEmission::iface_views` 与 `ImplTexts::views` 一并删去。
  - 宏单测新增 `carrier_holds_object_only`，守护以下两点：
    - 载体字段是 `__ref : __IfaceRef ,`，不含 `__IfaceRef <`；
    - `__VTable` 只出现在 vtable trait 定义、`enter :: < dyn I__VTable >` 与 `< dyn I__VTable > :: m` 中。
- K2 验证：
  - 宏单测 17/17（缺省特性）、21/21（`plan` 特性）。
  - 生成器 `cargo check --tests` 通过。
  - dev 抽查 5/5：InterfaceDispatch、LambdaBasic、StreamDemo、HelloWorld、DeepCopy，tag `s7k2-847493cb`。
  - 全量单测与七例抽查集由协调会话合批跑。
- §4.3 计时：
  - 测法：jp1（8 核 15 GB，云服务器），`scripts/graalvm_bench.sh`，`MODES=rava-release REPEAT=5`，运行取 5 次中位数，每例先测 K2 前再测 K2 后。
  - 作业 tag 为 `s7k2-{base,k2}-<用例>`，K2 前即 main c7cfb932。
  - 构建 = 转译 + 编译（秒），二进制为字节数，输出与 JVM 逐字一致（SAME）。

  | 用例 | 构建 前 / 后 | 运行 前 / 后 | 二进制 前 / 后 |
  |---|---|---|---|
  | InterfaceDispatch | 169.39 / 165.02 | 0.008 / 0.007 | 11 120 568 / 11 028 120（−92 448，−0.83%） |
  | InheritanceChain | 165.00 / 166.60 | 0.008 / 0.008 | 11 087 888 / 10 995 248（−92 640，−0.84%） |
  | TestCollections | 165.91 / 165.48 | 0.008 / 0.008 | 11 105 816 / 11 013 176（−92 640，−0.83%） |
  | HelloWorld | 165.53 / 165.82 | 0.008 / 0.008 | 11 093 840 / 11 001 264（−92 576，−0.83%） |
  | StockTrans | 未测 | 未测 | 未测 |

  - **结论**：
    - 运行耗时在计时分辨率内无变化：§4.3 列的用例在 release 下只跑约 8 ms，以进程启动为主。
    - 构建耗时差在 ±1% 噪声内。
    - 二进制每例稳定小约 92 KB（−0.83%）。来源是载体与 `__IfaceRef` 不再按接口单态化，引导映像也不再有接口视图常量。
    - K1 在 c7cfb932 之前已合入，K2 前测点已含 K1，所以这组数字只反映 K2 的增量。K1 相对 S7-3 的开销没有单独测量。
    - 语料里没有真正的热循环用例：`tests/perf` 只有 VirtualThreadScale，也没有 HelloWorld 循环版。分派开销要定量，须补一个百万次级接口 / 虚分派循环用例，留到 K8 档案验收时一并做。
  - 旁证：dev 上 K2 前 StockTrans 测得构建 882.70 s、运行 0.039 s、二进制 79 085 128 B。该次之后 dev 转跑 JDK25 全量，计时改到 jp1 重做，dev 这一点不作对照。jp1 上这一对被其他作业占机阻塞 26 分钟以上，单次构建约 15 分钟，前后两次加起来超过一小时，所以中止、未测，留到 K8 档案验收时连同热循环用例一起补测。
- K3 实现已提交（s7-k3，基于 main c6b126cc），**未达验收，未合入**。提交：72ed9339（宏方法 trait + 生成器按 owner 导入）、430f6bd8（合成调用 / 反射分派 / 引导映像 / main 导入，runtime 补 `use`，宏单测修正）、f5d9e2ca（invokevirtual 接收者静态类型登记）、f3368359（接口载体不导入方法 trait）。
  - 宏（`runtime/rava_macros_core/src/block/gen/wrapper/method_trait.rs`）：每类生成 `pub trait X__Methods<G…>: Sized` 与 `impl X__Methods for X`。`const fn`、关联常量与 std 同名成员（`clone` / `eq` / `hash` 等）仍留固有 impl。接口载体成员也仍为固有。
  - 生成器：`NameScope.methods` 记录用到的方法 trait，`ctx.ty.use_methods(bin)` 登记，项目写出时按 `em.scope` 渲染 `use …::X__Methods as _;`。`instr/src/method_traits.rs` 的 `record_insn` / `record_member` 自 owner 沿超类链找声明类，止于接口或 Object。登记来源：逐指令 owner；invokestatic / invokevirtual / invokespecial 入口（覆盖 indy 辅助、`concat_stringify`、ObjectMethods 等合成调用）；invokevirtual 接收者静态类型比 owner 更具体时（协变返回、上游已知子类）；`phase2/dispatch.rs` 反射分派臂（`String::valueOf_obj`）；`boot_image` `Plan.methods`；`main.rs` 钩子类与主类（跳过接口和手写类）。
  - runtime：约 86 个手写文件只加 `use … as _;`，diff 经核对不含其他改动。除 `_impl.rs` / `_ext.rs` 外，也涉及非共置的基础设施文件：`error.rs`、`lib.rs`、`array_impl.rs`、`monitor.rs`、`reflect_dispatch.rs` 等。
- K3 验证（jp2 / sg1）：
  - 宏单测 21/21（作业 s7k3-it2b）。生成器本机 `cargo check --tests` 通过。
  - 生成树对照（9 例：HelloWorld、DeepCopy、InheritanceChain、TestGenerics、InterfaceDispatch、TestDynamicProxy、CollectorsDemo、TestZonedDateTime、TestArrayList；基线 c6b126cc 对 430f6bd8）：按「层 + 相对路径」归一后，方法体文本逐字节不变，只多出 `use …__Methods as _;` 行。闭包类集合、raw-audit 一致。`__Methods` 只出现在 `use` 行，可读层方法体中没有。其余差异都是 D8 段归属变化与 `java_base_body_N` 重新编号：D8 按源码体量切段，`use` 行增加了体量。DeepCopy 的引导映像 crate 路径也因此变化。
  - 工作区 `cargo check`（f5d9e2ca，作业 s7k3-it3）：TestDynamicProxy、TestZonedDateTime、TestGenerics、InterfaceDispatch、TestArrayList 通过；430f6bd8 上 HelloWorld、CollectorsDemo、InheritanceChain 已通过。DeepCopy 失败：`user/src/main.rs` 有 14 处 E0432，原因是导入了接口载体的 `__Methods`（`InvocationHandler`、`java.security.Key` 系列）。f3368359 已修，**未复测**。
  - **峰值内存未达标**（sg1 同机，`/usr/bin/time` maxRSS，基线 → K3）：

    | 用例 | crate | 基线 | K3 | 变化 |
    |---|---|---|---|---|
    | HelloWorld | java_base_decl（底段） | 1 022 680 KB / 31.1 s | 1 270 836 KB / 38.5 s | +24% |
    | HelloWorld | java_base_body_1 / _2 | 860 / 989 MB | 879 / 1 006 MB | +2% |
    | DeepCopy | java_base_decl（底段） | 4 154 428 KB / 122.2 s | 5 430 308 KB / 165.9 s | +31% |
    | DeepCopy | 声明上段 | decl_1 1 080 MB、decl_2 601 MB | decl_1 799 MB（段数 3 → 2） | — |
    | DeepCopy | body 最大段 | 1 951 MB | 1 937 MB | −1% |

    方法体 crate 在 ±5% 内。声明底段升 24–31%，超出 ±5%。成因待分：一是每类多出 trait 定义加 trait impl（成员签名展开两遍），以及 trait 方法解析的类型检查开销；二是 D8 按体量重新切段，底段吸收了原上段内容（DeepCopy 段数 3 → 2）。
- K3 未完成项（交回时状态）：
  1. DeepCopy 复测：f3368359 之后全部 9 例工作区 `cargo check` 通过。
  2. 声明底段峰值归因与收回。先固定 D8 段划分再对照（关闭 `use` 行计入体量，或按基线段表强制划分），区分「段重切」与「trait 展开」两部分。若 trait 展开占主：
     - trait 只声明签名、方法体留在 trait impl，避免在 trait 定义里再展开缺省体；
     - 字段访问器、`__init_on`、`_init_not_null` 这类不跨 crate 使用的内部成员留固有 impl；
     - 评估以上两项后再定是否需要按类拆 trait。
     目标：同机 ±5%。
  3. 生成器全量单测（含 `no_jdk_literals` / `jdk_literal_lint`）未跑。本机只跑过 `cargo check --tests`。
  4. 七例抽查集未跑（只做到 `cargo check`，未运行比对输出）。
  5. std 同名成员与接口载体成员仍为固有 impl：K6 搬 struct 后跨 crate 失效。需在 K6 前改名（如经 `__`-前缀内部名转发），或整体并入方法 trait。
- K3 经验（供后续步骤）：
  - 手写文件不止 `_impl.rs`：基础设施文件（`error.rs`、`lib.rs`、`monitor.rs`、`reflect_dispatch.rs`、`array_impl.rs` 等）也直接调用生成类的方法，同样需要显式导入。K4 把基础设施拆进 `runtime/java_marker/` 时，标记层文件不得调用 `X__Methods`（标记层无方法面）。依赖行为的调用须留在行为层，或经 `extern "Rust"` 转交。
  - 合成调用（indy 降级、ObjectMethods、反射分派臂、引导映像、main 钩子）不经字节码 invoke 指令，必须在生成点逐一登记。漏登记表现为 E0599 / E0425（找不到关联函数）。补登记以 invoke 入口统一登记为主，避免散点。
  - K6：`X__Methods` 的定义与 impl 须留在声明 crate（与方法体同层），不能进标记 crate。孤儿规则允许：trait 本地、类型外来。导入路径须随之改指声明 crate 的 `pub use`。
- **K3 收尾（10-11 晚，s7-k3 15276d82）：峰值 +24–31% 分解归因、切段漂移修复、trait 展开收窄到 +8%**
  - **归因（固定切段分解）**：
    - 成因 1（切段漂移，主因之一）：K3 把文件头全部 `use …__Methods as _;` 无条件保留在声明层文件（`prune_uses` 对匿名导入不作单名剪枝）。D8 建图按 `crate::` 路径扫全文件 → 每条匿名导入都是类→类边；钉底文件（伴生宿主、手写引用类）的导入边级联把闭包拉进底段：HelloWorld 底段 342→476 类、DeepCopy 1895→2458（权重也计入这些行：两例底段分别 1 186 行 / 65 KB、11 363 行 / 638 KB）。模拟器同期失配（实际 {0:2326} vs 复现 {0:2122}）：`as _` 别名在 `local` 表互相覆盖。
    - 成因 2（trait 展开，主因之二）：每类成员签名在 trait 声明 + trait impl 各一份。rustc 分段剖面（`-Z time-passes`，本机，修复前口径基线→修复前）：expand_crate 351→426 MB、type_check_crate 696→837、MIR_borrow_checking 877→1012、monomorphization graph walk 1032→1193、generate_crate_metadata 1052→1202——全管线抬升，是签名双份 + trait 项进 crate 元数据的结构性成本。
    - 成因 3（次要）：匿名导入行的名称解析（修复后底段仍 538 行 / 30 KB）。
  - **修复（15276d82，三个成分）**：
    1. 声明层文件头匿名导入按词剪枝：`use crate::…::X__Methods as _;` 同单名规则——词干 `X` 出现在声明层文本（属性串 / 签名 / 反射分派尾巴）才保留。纯外壳文件的声明层跨类方法调用全部出自宏生成外壳，所指类名必在属性串或签名里，按词剪枝不漏。
    2. 留体文件精确保留：留在声明层的方法体（泛型类整类、wrapper 上下文体、不可函数化体）按推断类型解析方法调用，接收者类名可不在文本中（实测例：`concurrent_hash_map_tree_bin.rs` 的 `_t1.compareTo(_t3)`，全文件无 `String` 词），词剪枝不安全——剥体计划（宏 `plan.rs` `StayBodies`）给出留体 fn 的 (java_method name, descriptor)，生成器按方法体发射前后差分出它所需的方法 trait（`ProjectState::method_traits`），拆层时按名单精确保留；缺属性的留体 fn 退全保留。中间测点：只做成分 1 而留体文件全保留时底段 370 类（多拉 28 类），成分 2 把它收回基线。
    3. 内部成员留固有 impl：字段访问器（`__get_*` / `__set_*`）、`_init_not_null`、`__init_on` 不再入 trait——它们的跨 crate 调用点都在实现层或对本类，固有即可解析；trait 项省一份签名（底段 trait 成员 11 073 → 7 891，−29%）。K6 搬 struct 后固有失效，并入 K6 前置项（见下）。
    4. 模拟器 `decl_scc_graph.py`：匿名导入按 D8 的 `crate::` 路径无条件建边（`as _` 别名不再互相覆盖），复现与实际分段一致。
  - **修复后实测（本机 dev，32 核 / 120 GB，预热依赖后单 crate 重建 `/usr/bin/time` %M KB；基线 9baa7c22 → K3 2e310b56 → 修复 15276d82；切段与基线逐类一致：HelloWorld 底段 341 / 上段 237）**：

    | crate（HelloWorld） | 基线 | K3 | 修复 | 修复 vs 基线 |
    |---|---:|---:|---:|---:|
    | java_base_decl（底段） | 1 031 756 | 1 295 500 | 1 111 940 | +7.8% |
    | java_base_decl_1 | 544 928 | 422 244 | 588 680 | +8.0% |
    | java_base_body_1 / _2 | 865 604 / 1 061 384 | 887 232 / 1 021 992 | 878 384 / 1 017 132 | +1.5% / −4.2% |

  - **剩余与结论**：底段 / 上段 +7.8% / +8.0%，仍超 ±5%；body crate 达标。剩余成分是 java 方法外壳的 trait 展开（底段 7 891 项 trait 成员中 7 453 项为方法外壳，94%）：这正是 K6 需要的跨 crate 可读成员面，「每类一个全量方法 trait」形态内不可再减。进一步压低的已量化选项（供 K4 / 用户取舍，本轮未实施）：
    1. `__class_init`（底段 90 项）与静态字段访问器（348 项）也留固有 impl：−5.6% trait 项，估 −0.5–1.5 pp；再扩大 K6 前置项。
    2. 选择性 trait：只为「方法被跨文件调用」的类生成 trait（HelloWorld 底段实测 75% 的类被用到，25% 可省，估 −2–3 pp）；需两遍发射（全部文件的方法 trait 账本汇总后回写块内属性再触发展开），正确性风险在合成调用登记的完备性（漏登记即 E0599）。
    3. 1+2 叠加估到 +4–5%（临界达标）。
    4. 或修订 K3 过渡期阈值（±5% → ±10%）：D8 自身的 ≤1.3 GB 段目标不受影响（修复后 HelloWorld 底段约 1.11 GB < 1.3 GB；DeepCopy 底段基线本就 4.15 GB 超 1.3 GB，归 K5/K7 底段收窄）。
  - **验证状态**：
    - 本机：`cargo check --release --tests` 过；宏单测 26/26（`--features plan`）与缺省 21/21、emit layers 4/4；生成树工作区构建 0 错（HelloWorld、TestDynamicProxy，dev）。生成树对照（基线 9baa7c22 对 15276d82，HelloWorld 全部 1 045 个 `java_class!` 块按「层 + 相对路径」归一后逐字节一致，差异全部是文件头 `use crate::` 行）。
    - 本机 CollectorsDemo（同法）：底段 1 108 680 → 1 191 856（+7.5%）、上段 524 224 → 571 276（+9.0%）、body 两段 +2.8% / +1.5%。
    - 服务器单测 B（`s7k3-fix1-utB`，sg2）：workspace 除 driver 全部 + driver bins/各 CLI + 宏测试共 37 个测试目标 637 通过 0 失败（含 `no_jdk_literals` / `jdk_literal_lint`、emit layers / method_trait / decl_plan 新测试）。
    - 服务器（其余作业 `s7k3-fix1-*`，进行中）：复测（同机基线 vs 修复 × HelloWorld / CollectorsDemo / DeepCopy 逐 crate 峰值 + 全工作区构建，即 DeepCopy f3368359 后复测）、单测 A（closure_cli）、DeepCopy release 二进制对照 batch-1011k。结果见下「K3 收尾服务器数据」。
  - **K6 前置项（K3 相关，搬 struct 进标记 crate 前必须处理）**：std 同名成员（E0034）与接口载体成员（固有）之外，新增本轮留固有的字段访问器、`_init_not_null`、`__init_on`——须改名经 `__`-前缀内部名转发、并入方法 trait，或由 K6 的标记层机制另行承载。**前两类已于 10-11 完成（见下「K6 前置项完成」节）**；第三类与载体内 `is_jvm_null` / `__default_*` 留作 K6 标记层机制处理。
  - **待验证清单（交主会话合批）**：七例抽查集（HelloWorld、CollectorsDemo、DeepCopy、TestDynamicProxy、TestAnnoReflect、TestSerialLookupPairing、TestXmlTransform）＋相关例（InheritanceChain、InterfaceDispatch、LambdaBasic、StreamDemo、TestCollections、TestGenerics、Polymorphism、TestZonedDateTime、TestArrayList）；生成树对照（`compare_trees.sh`，基线 9baa7c22 对 15276d82：预期方法体文本不变、差异只在文件头匿名导入行与 D8 段归属复原）。
- **K3 收尾交接（10-12 晨，子代理到点停止；作业在跑，结果收集法如下）**：
  - `s7k3-fix2-15276d82`（jp2，复测：基线 9baa7c22 vs 15276d82 × HelloWorld / CollectorsDemo / DeepCopy 逐 crate `/usr/bin/time` 峰值 + 全工作区构建 + 模拟器分段复现）：完成后日志尾有 `===== mem.txt =====` 全表（`cluster_results/job/<tag>/01_jp2.log`），`--fetch` 已声明，`build/k3fix.tgz` 会落到结果目录。首个同名作业 `s7k3-fix1-15276d82` 跑完 rc=0（44 min）但未取回产物（spot 目录已清理），数字以 fix2 为准。
  - `s7k3-fix1-utA`（us1，closure_cli）：`01_us1.log` 流式收 `test result` 行；跳过项为 `closure_independent_of_hash_seed / of_order`（确定性单测，按 12.3 不在本机跑）。
  - `s7k3-fix2-rel`（kr2，DeepCopy release 二进制对照 batch-1011k，`CARGO_BUILD_JOBS=1`）：`s7k3-fix1-rel`（JOBS=2）在 15G 服务器 OOM（限 11891M）——本分支基线 9baa7c22 不含 E1（build-memsafe，在 batch-1011h 才合入）；JOBS=1 重试中，若仍 OOM 改在 dev 跑或合入 E1 后复测。
  - 数字回填位置：本节「K3 收尾」的实测表与「K3 收尾服务器数据」（待建小节）。预期：服务器同机比值与本机一致（底段 / 上段约 +8%、body ±5% 内）；若出入大于 2 pp，优先核对两例的段成员表（模拟器复现行）。
  - 主会话合批抽查清单见上「待验证清单」。
- **K6 前置项完成（10-11，s7-k3 8efa6444 + 5c3c8bdc）**：std 同名方法与接口载体方法移入方法 trait，交付于 s7-k3 分支（K3 之后两提交），未合入。
  - **盘点结论**：
    - std 同名成员的实际命中面（HelloWorld 闭包实测 + 名单核对）：`clone`（类覆盖 `Object.clone`：HashMap / Hashtable / ArrayList / HashSet / Locale / Properties / Enum / Thread / MemberName / Reference / AbstractMap / HashtableEntry 等，HelloWorld 闭包 12 类、DeepCopy 更广）、`eq`（`Objects.eq` / `AbstractMap.eq` / `Collections.eq` 等静态）、`from`（`Random.from` 静态工厂）；其余 8 个名单名（ne / into / to_owned / try_from / try_into / clone_from / clone_into / default）在语料未命中（`default` 是 Java 关键字，不可能出现）。`hash` 不在名单——wrapper 未实现 std `Hash`，无 E0034 面（K3 全量已验证）。
    - 接口载体成员形态：载体固有 impl 含 static 方法、static 字段访问器、`__class_init`、实例分派（`__IfaceRef::enter` 现取视图）、继承转发（`inherited_from`）与 default 体持有（`__default_<m>`）；接口文件整体属声明层（`Layer::Body` 为空）。
  - **改动**：
    1. std 同名成员（11 名单，`STD_COLLISIONS` 口径）统一改 `__` 前缀内部名（`clone` → `__clone`）后移入方法 trait：单一规则 `ty::ident::escape_std_trait_method`，定义侧（input/plan `member_rust_name`、emit/vtable `member_rust_name`（槽位名）、ty/sig_types `receiver_member_name` / `interface_member_local_name` / `interface_declared_name`、class_writer inherit / super_inherit / methods / stub、method/sig 兜底）与调用侧（instr/naming 非重载原名分支）同一口径；根类（手写，含 `Object__clone_base`）与带描述符后缀的重载名不改；反射分派臂按 Java 名匹配、臂体调内部名，行为不变。宏 `STD_COLLISIONS` 保留为手写块兜底（改名后的标识符不在名单，正常入 trait）。
    2. 接口载体成员移入 `I__Methods`（宏 interface.rs 经 `method_trait::split_parts`，与类同形）：static 方法、static 字段访问器、`__class_init`、实例分派与继承转发；`const fn`（`__from_image`）、关联常量（`BINARY_NAME` / `__IMAGE_NULL`）、`is_jvm_null`、`__default_*` 留固有（身份面 / 内部名，K6 搬 struct 后由标记层机制另行承载——与 K3 留固有的字段访问器、`_init_not_null`、`__init_on` 同类遗留，见下）。
    3. 登记（漏登记即 E0599 / E0432）：`record_member` 受理接口（只登记 owner——载体的继承转发外壳在 owner 上，不沿超接口链走）；phase2 `inherited.rs` / `iface_impls.rs` 的转发体发射点、`head.rs` `init_interfaces`（宏生成的 `<I>::__class_init()` 触发）、`resolve_static_field`（getstatic 常量池 owner 可以是引用它的类 / 实现类，访问器在声明接口上——实测 `Signal$NativeHandler.toString` 的 `SIG_IGN`）、boot_image、entry.rs 钩子类清单同步放开接口。
    4. D8 权重不计接口方法 trait 匿名导入行（decl_side `iface_methods_import_bytes`）：接口文件不拆层、导入行必然保留，计入权重会把纯名称解析开销（K3 归因的次要成分）按 240 B/MB 斜率推高上段预算——未扣时 DeepCopy 有一类跨段（decl_1 → decl_2），扣后三例切段与 70dd51f8 逐类一致；对无接口导入行（K3 及更早）的树是零操作。类侧匿名导入口径不变。
  - **验证（本机 dev）**：
    - 宏单测 23（缺省）/ 28（`plan`），新增 `renamed_std_member_moves_to_trait`、`carrier_members_on_methods_trait`；生成器单测 ty 24 / input 65 / instr 20 / method 22 / emit 95（含新增 `iface_methods_import_bytes_excluded`）、`jdk_literal_lint` / `no_jdk_literals` 各 2——全绿。
    - 生成树对照（基线 70dd51f8 同 closure 重发射）：HelloWorld / CollectorsDemo / DeepCopy 三例全部 crate 的文件集合逐类一致（切段不漂移）；文件差异仅为接口 `__Methods` 匿名导入行 + `clone`/`eq`/`from` 的 `__` 前缀改名（声明与调用点，DeepCopy 全树 80 处改名调用点，含用户源 `p1.__clone()` 与反射分派臂）。三例工作区 `cargo check` 0 错；HelloWorld / CollectorsDemo / DeepCopy / TestSerialLookupPairing 四例经 rava-link 运行，输出与 JVM 期望逐字一致（DeepCopy 覆盖用户 clone、反射 clone 臂与序列化深拷贝；全树差异逐行核对只含 `__Methods` 导入行与三名的前缀改名）。
    - 服务器作业（tag `s7k6-*`）：
      - `s7k6-macro` ✅（jp2）：宏单测 23（缺省）/ 28（`plan`）全过。
      - `s7k6-seed` ✅（jp2，rc=0，`scripts/diag/s7k6_seed_sets.sh`）：四例 × 种子 0/1/2 的类 / 方法 / 反射成员集合与基线 70dd51f8 逐项全同（HelloWorld 580/1897/61、CollectorsDemo 619/2022/62、DeepCopy 3001/16425/977、TestSerialLookupPairing 3003/16410/971；日志尾 `seed-sets: ALL-IDENTICAL`，全表在 `cluster_results/job/s7k6-seed/`）。
      - `s7k6-uta`（us1，closure_cli 全量，收尾时在跑）：结果看法——`cluster_results/job/s7k6-uta/01_us1.log` 收 `test result` 行（跳过项 `closure_independent_of_hash_seed` / `of_order` 属确定性重单测，服务器上照跑）；rc 见 summary.json。
      - `s7k6-peak4`（三例逐 crate 单独重建峰值对基线 70dd51f8，方法同 s7k3-fix2，`scripts/diag/s7k6_peak.sh`）：**交回时在跑，数字待回收**。前两跑（`s7k6-peak` / `s7k6-peak2`）因脚本 cwd 漂移早退、`s7k6-peak3` 因基线 worktree 路径与同服务器 seed 作业共用而冲突——均已修复（479f3745 / 4d28aa29）；peak3 已产出**新侧三例全量逐 crate 数据**（HelloWorld：decl 底段 1 124 388 KB / decl_1 584 456 / body_1 907 960 / body_2 1 071 044；DeepCopy 底段与全部上段 / body 段齐全，见 `cluster_results/job/s7k6-peak3/01/build/s7k6-peak/rows.txt`）。回收法：`cluster_results/job/s7k6-peak4/`（fetch 已声明 summary.txt 与 rows.txt，逐 crate 表在日志尾与 `01/build/s7k6-peak/summary.txt`），判定口径 = 声明 crate 峰值对 70dd51f8 不超 +10%。
  - **遗留**：K3 留固有的字段访问器（`__get_*` / `__set_*`）、`_init_not_null`、`__init_on` 与本轮留固有的 `is_jvm_null`、`__default_*`、载体关联常量——K6 搬 struct 后固有跨 crate 失效，须由 K6 的标记层机制另行承载（或并入方法 trait，届时权衡 trait 项体量 vs 标记层）。
  - **待验证清单（交主会话合批）**：K3 七例集（HelloWorld、CollectorsDemo、DeepCopy、TestDynamicProxy、TestAnnoReflect、TestSerialLookupPairing、TestXmlTransform）＋ K3 相关例（InheritanceChain、InterfaceDispatch、LambdaBasic、StreamDemo、TestCollections、TestGenerics、Polymorphism、TestZonedDateTime、TestArrayList）＋ 本轮新受影响例（PolymorphicCopy——用户直调 clone；ArraysStream、HashMapStream、StreamToUnmodifiableCollections——接口方法重度使用；TestObjectNativeFrames——Object 方法帧）。
- **协议面保留名单扩充（10-12，b1013-fix，修 batch-1013 全量编译回归）**：
  - **缝的成因（跨分支语义冲突，非单侧缺陷）**：s7-k3 分支（72ed9339 起）把类 wrapper 可读层成员移入 `X__Methods`；主 lineage 在分支点（9baa7c22）之后继续给运行时手写层加了直调生成类成员的代码——84d9468f（`reflect_dispatch::box_value` 的 `Boolean::valueOf_z` … `Double::valueOf_d` 装箱 switch）、9aed6cd6（`error.rs` 的 `thread.dispatchUncaughtException`）、1b8728ec（`file_descriptor_impl.rs` 的 `SyncFailedException::new`、zip 系列）等。s7-k3 侧验证（三例工作区 check + HelloWorld 运行）不含这些调用（分支早于它们），主侧验证时成员尚固有——两侧各自全绿，合批 f7b69314 后 33/33 抽查 E0599。**教训：方法 trait 化这类「可读层成员寻址方式」变更与手写层新增直调互为盲区，合批前须以对方分支头为基跑一次工作区编译**（K3 的 compare_trees 只对方法体文本，不含手写 overlay 文件的编译面）。
  - **修法一（宏，留固有名单）**：装箱工厂族 `valueOf_*`（按描述符后缀命名的全部，`moves_to_trait` 前缀规则、不类名特判）加入留固有名单——它们是 VM 装箱协议面（`reflect_dispatch::box_value`、`reflect/Array.get` 装箱、`proxy_dyn` 重装箱路径直调），与访问器（`__get_*`/`__set_*`）、`_init_not_null`、`__init_on` 同级。无后缀 `valueOf`（枚举 `valueOf(String)`、Character 唯一重载）仍入 trait（生成器按 owner 登记导入；手写层 use 行对齐）。宏单测 `boxing_factory_stays_inherent` 守护。**K6 注意**：该族与字段访问器同列「K6 搬 struct 后固有跨 crate 失效」的遗留名单，届时并入标记层机制或方法 trait。
  - **修法二（手写层对齐 use 行，25 文件）**——`use crate::<包>::X__Methods as _;`（既有 200+ 行同一形态；TypeRef 与既有引用同类同路径，闭包零变化）：
    - **batch-1013 直接暴露（5 文件）**：`reflect_dispatch.rs`（`Character__Methods`——无后缀 `valueOf`）；`error.rs`（`Thread__Methods`——`dispatchUncaughtException` 实例调用）；`file_descriptor_impl.rs`（`SyncFailedException__Methods`）；`adler32_impl.rs` / `deflater_impl.rs`（本类 `__class_init` + `InternalError__Methods`）。
    - **系统性缝（`#[jvm_native]` 注入 `Self::__class_init()?`，20 文件）**：`jvm_native` 宏对静态 native 注入类初始化触发点，而 `__class_init` 在方法 trait 上——每个伴生文件（`use super::x::X` 非 glob 形态）都缺本类导入。批量补：console、random_access_file、process_handle_impl(_info)、finalizer、accessor_utils、native_method/native_constructor/direct_method_handle_accessor 三件、continuation_support、linux/mac_osx_socket_options、e_poll、k_queue、net(_ext)、socket/unix_dispatcher、bsd_file_system、bsd/linux_native_dispatcher、perf（含 `ByteBuffer__Methods`/`ByteOrder__Methods`——allocate / order_byteorder / nativeOrder / BIG_ENDIAN）、default_proxy_selector（含 `Proxy`/`Proxy_Type`/`InetSocketAddress`）、net_impl（含 `Inet4/6Address`、`InetSocketAddress`）。**同包 glob 伴生（`use super::*`，system/thread/class/string_utf16/strict_math 等 13 文件）经包 mod.rs 的 `pub use X::*` 已覆盖，无需处理**（33 例抽查实证）。
  - **排查方法（复用）**：路径调用扫描（`X::m` 正则 × use 表交叉 × 留固有名单）＋ `jvm_native` 静态注入面枚举＋ 主侧 diff 的 camelCase 实例调用增量的三合一；残余风险＝伴生文件不在既有抽查闭包内的实例调用（需类型推断，静态不可判），下批抽查覆盖新伴生宿主时自然暴露。
  - **验收（全过，abdd523b）**：宏单测 plan 39 / 缺省 35 全过、generator `cargo check --tests` 过；服务器作业 `bfx-acc`（us1，rc=0）：ArraysStream / HelloWorld / TestJunitAssertFamily PASS，加验 TestDirectBuffer / TestDirectBufferPaging（覆盖 zip 伴生修复）共 5/5；`bfx-seed`（jp1，rc=0）：四例种子 0/1/2 闭包集合对 batch-1013 基线（adf4cc1f）逐项全同（HelloWorld 641/2108/61、CollectorsDemo 662/2206/62、DeepCopy 2660/15390/801、TestSerialLookupPairing 2662/15380/795，`seed-sets: ALL-IDENTICAL`——s7-k3 的集合性质不变）。
- **K6 本体（10-12，s7-k6 2e3e9c2f + 5e7b21b8，基于 batch-1013 31e4762d）**：`java_class!` 块拆出
  类型标记层（`#[rava_layer = "marker"]`）——类 struct 的声明承载从声明层分离。
  - **与文档分解的偏差（先行记录）**：K6 的文档前提是 K4（手写基础设施分层）与 K5（类初始化骨架
    去按类展开），两者均未做（K4 未派、K5 未派）。标记部分引用 `__Handle` / `Object` / `__ClassDesc`
    等，全部在手写 INFRA（overlay 进声明底段 crate）；没有 K4 的两层拆分，**不存在能放在声明链
    之下的独立 `java_marker` crate**（标记 crate 依赖底段、底段又要标记类型 → 环）。本轮取过渡形态：
    **标记文件与声明层文件同目录同段（同 crate）**，K4 落地后只剩文件搬移与路径前缀。K5 未做不
    影响 K6 的机制正确性，只影响 D8 图收窄（⑤ 口径）的最终验收（归 K8）。
  - **宏（`runtime/rava_macros_core`）**：`Layer::Marker` 只展开与签名无关的部分——wrapper
    struct、`__from_parts` / `__from_image` / `__virtual_view` / `is_jvm_null` / `__nn` 固有 impl、
    身份 std impl（Default/Clone/PartialEq/Debug）、`From<X> for Object`、`BINARY_NAME`、描述符
    `X__DESC` / `X::__DESC`、`From<Object> for X`、`Into<Object>`（R-1）、`From<X> for Anc` 上转、
    alloc/offsets 钩子外部声明（`place_fns` 对 marker 同 decl）。`Layer::Decl` 不再展开这些
    （`__vt` 固有 impl 留声明层——引用本类 vtable trait，见遗留）。`Layer::Full` 拼回两段（`__vt`
    自 `__from_parts` impl 拆出为独立 impl 块，项序与拆前不同、内容不变）；用户 crate / lib crate
    零变化。接口遇 marker 层展开为空（接口本轮不拆）。
  - **生成器（`emit`）**：新 `project/marker_side.rs`——D8 分段**之后**对已拆层根模块类发射标记文件
    `<stem>_m.rs`（同目录同段；未拆层类不发射；同包 `Foo` / `FooM` 撞名退 `<stem>__m`，确定性）。
    宿主声明文件头补一行 `pub use super::<标记模块>::<类名>;`——标记部分剥去后类名不在本模块，
    声明层展开（`impl X` / `X__VTable` / `X__Methods` 实现）与实现层文件的
    `use <声明层>::<包>::<类模块>::*;` 都要它；包级两 glob（类模块与标记模块）重复导出同一
    `X`，同一定义不歧义。**声明文件块文本与 D8 分段输入逐字节不变**（改写在分段之后）——切段
    逐类与基线一致由构造保证。底段包 mod.rs 由 `mod_tree` 按磁盘扫描自动声明标记模块；上段由
    `DeclSegs::write_crates` 按 `marker.hosts` 补 `pub mod` / `pub use` 行。
  - **K6pre 遗留清单归位（本轮形态）**：类 wrapper `is_jvm_null` 随标记层（from_parts impl 一部分）；
    `_init_not_null` / `__init_on` / 字段访问器 `__get_*`/`__set_*` / `valueOf_*` 留**声明层文件**
    固有 impl——同 crate 有效（访问器体引用 vtable 槽、`__init_on`/`valueOf_*` 外壳引用体函数
    外部声明）；接口载体 `is_jvm_null` / `__default_*` / 载体关联常量不动（接口不拆层）。**K4 后
    搬 crate 时的剩余项**：访问器与 `__vt`（引用 vtable trait，不能进标记 crate）须并入方法 trait
    或调用点内联取视图（`self.__r.view::<dyn X__VTable>(depth)`）；`_init_not_null` / `__init_on` /
    `valueOf_*` 与钩子外部声明可随标记文件走。
  - **实施中修复**：首版把体函数 / base 函数生成挂在「声明层才算」的分支下，实现层（Body）静默
    丢定义——cargo check 全绿但链接缺符号（`__jbm_*` / `*_base` undefined）。本机以
    `cargo build --bin` 链接验证抓出（HelloWorld 链接失败→修复→过）。**教训：拆层类改动必须做
    本地链接级验证，check 不覆盖 extern 声明 / 导出定义的配对。**
  - **本机验证（dev，JDK 21）**：宏单测 41（`--features plan`）/ 35（缺省）全过（新增
    `marker_layer_is_signature_free`——标记展开无 `__VTable` / `__Methods` / `__vt`、
    `decl_layer_keeps_dispatch_face`、`wrapper_holds_handle_only` 改完整展开口径）；emit 单测 99
    （新增 marker 文本切分 / 未拆层不发射 / 撞名避让 / 上段标记模块行 4 例）；generator
    `cargo check --release --tests` 0 警告；HelloWorld / InterfaceDispatch / TestGenerics /
    TestDynamicProxy 四例 emit→编译→rava-link 链接→运行输出与 JVM 期望逐字一致（TestDynamicProxy
    1843 类，覆盖装箱工厂协议面 `valueOf_*` 固有寻址与代理路径）；双发 emit 树确定（时间戳版本号
    外零差异）；HelloWorld 树 542 个标记文件（底段 317 + 上段 225）。**生成树对照（基线 31e4762d
    同闭包重发射，逐文件 diff）**：新增 542 个 `_m.rs`、删除 0；修改 595 个文件全部为三类——542
    个类文件只多一行 `pub use super::<m>::<Name>;`、48 个包 mod.rs 只多标记模块行、8 个
    Cargo.toml 只差时间戳版本号，另 5 个为 vendor 宏 crate 源码（本轮宏改动）——**方法体与声明层
    块文本逐字节不变**。
  - **服务器验收（tag 前缀 s7k6b-，基线 31e4762d）**：
    - `s7k6b-macro` / `s7k6b-macro2`（宏单测，jp2）✅ **plan 41/41、缺省 35/35**（rc=0）。
    - `s7k6b-seg`（kr2）首跑脚本 cwd 漂移（`seg_listing` 改 cwd 后第二用例 find 失败——单用例
      冒烟未覆盖，与 s7k6_peak 首跑同类）；修复（子壳化 + 基线 worktree 复用 checkout）后
      `s7k6b-seg2` ✅ **三例切段逐类一致**（HelloWorld 788 / CollectorsDemo 809 / DeepCopy 2681
      个段内类文件全同）；标记文件 HelloWorld 542 个 / 6.15 MB、CollectorsDemo 559 个 /
      6.42 MB、DeepCopy 2215 个（字节数脚本首版 xargs 分批少计，修为逐文件求和，4e182e4e）。
    - `s7k6b-seed`（jp1，`scripts/diag/s7k6_seed_sets.sh 31e4762d`，四例 × 种子 0/1/2 闭包集合）、
      `s7k6b-det`（us1，`closure_independent_of_*` 两项）、`s7k6b-peak`（sg1，`s7k6_peak.sh
      31e4762d`，三例逐 crate 固定切段重建峰值）**交回时在跑 / 排队**（集群被 JDK25 扫尾占用；
      seed 首跑两次脚本外故障——worktree 已存 + 被回收重派，已修脚本幂等续跑 fb938aa3），结果
      回收法见下「K6 交接」。**本机补证**（dev，同 JDK 21）：① 闭包集合对照——HelloWorld ×
      种子 0/1/2，新（s7-k6）对基线（31e4762d）的类 / 方法 / 反射成员集合逐项全同
      （641/2108/61，与 bfx-seed 基线数字相同）；② **确定性两项本机过**
      （`closure_independent_of_hash_seed` / `closure_independent_of_order`，2 passed / 1 ignored
      （`_large`），release 档 3655 s）；③ **构造性证据**：本分支对 `closure` / `instr` / `ty` /
      `closure_cmd` 等分析器全部源码的 diff 为空（改动全在宏展开与 emit 侧）——闭包集合性质由
      构造保证。TestSerialLookupPairing 的本机对照出现 2709 → 6475（多出 JCA provider 族）的
      假差异：与 dev 上并行的 JDK25 作业 / 共享缓存环境有关（同口径 HelloWorld 全同、分析器源码
      零差异可排除代码回归），以服务器 seed 作业（隔离检出、同脚本双侧）为准。
    - **本机峰值 A/B**（`rustc_profile.sh`，`-Z time-passes` 的 rustc 自计 RSS，同机隔离）：
      HelloWorld 底段 1331 → 1444 MB（**+8.5%**）、上段 decl_1 613 → 644 MB（**+5.1%**）、
      CollectorsDemo 底段 1413 → 1453 MB（**+2.8%**），均在 ±10% 预算内；增量来源＝标记文件对
      块文本的复制（解析 / 展开成本，无新增展开项——标记项与拆前同类项相同）。
    - **标记文件体量**：HelloWorld 542 个 / 6.15 MB（均 11.3 KB，含完整文件头——块属性串携带
      全部祖先 / 接口名，按词剪枝剪不下去；K4 搬 crate 时可再剥离方法签名与未用名，K8 档案
      验收时复测）。
  - **待验证清单（交主会话合批抽查，批级）**：K3 七例集（HelloWorld、CollectorsDemo、DeepCopy、
    TestDynamicProxy、TestAnnoReflect、TestSerialLookupPairing、TestXmlTransform）＋ K6pre 新增例
    （PolymorphicCopy、ArraysStream、HashMapStream、StreamToUnmodifiableCollections、
    TestObjectNativeFrames）＋ K3 相关例（InheritanceChain、InterfaceDispatch、LambdaBasic、
    StreamDemo、TestCollections、TestGenerics、Polymorphism、TestZonedDateTime、TestArrayList）＋
    本轮 struct 搬移直接受影响面（标记层承载的 `From<Object>` 强转 / `From<X> for Anc` 上转 /
    `Clone`·`Debug`·`PartialEq` 身份 impl / `__from_image` 引导映像常量 / 描述符 `__DESC` 反射面）：
    TestCoerceBoxCast、TestMhBoxCast、TestReflectBoxCast、TestUnboxWrapperFaces、
    TestWidenFacesInvoke、TestWrapperValueFaces、ReflectionAPI。本机已过四例（HelloWorld、
    InterfaceDispatch、TestGenerics、TestDynamicProxy——含 1843 例的装箱工厂协议面）。
  - **K6 交接（10-12 晨，子代理到点停止；作业在跑，回收法如下）**：分支 s7-k6 @ fb938aa3
    （实现 2e3e9c2f + 脚本 5e7b21b8 / 4e182e4e / fb938aa3 + 文档 f8f53d6b / 592533ae），已推
    origin / github。已收：`s7k6b-macro2`（jp2，宏单测 plan 41 / 缺省 35）、`s7k6b-seg2`
    （kr2，三例切段逐类一致——HelloWorld 788 / CollectorsDemo 809 / DeepCopy 2681 文件全同，
    标记文件 542 / 559 / 2215 个）；本机已过：确定性两项（hash_seed / order，release 3655 s）、
    HelloWorld 三种子闭包集合对基线全同、四例 + CollectorsDemo 共五例 emit→链接→运行逐字一致、
    峰值 A/B（HW 底段 +8.5% / 上段 +5.1% / CD 底段 +2.8%）。在跑（收尾即出）：
    - `s7k6b-seed`（jp1）：幂等续跑——23 份闭包 json 已在前两次运行产物里，缺
      base.TestSerialLookupPairing s2，之后出对照表；日志尾 `seed-sets: ALL-IDENTICAL` 即过，
      全表在 `cluster_results/job/s7k6b-seed/`（`--fetch` 已声明 summary.txt 与 *.log）。
    - `s7k6b-det`（us1 排队）：`cargo test -p driver --test closure_cli closure_independent`——
      **本机已过（两项 ok）**，服务器作业为重复确认，可等可撤。
    - `s7k6b-peak`（sg1 在跑）：`s7k6_peak.sh 31e4762d`——逐 crate 表在日志尾与
      `01/build/s7k6-peak/summary.txt`、rows.txt（fetch 已声明）；判定口径 = 声明 crate 峰值对
      基线不超 +10%（DeepCopy 底段记录不作判据）。本机同工具 A/B 已在 ±10% 内。
    - 回收命令：`cat cluster_results/job/<tag>/summary.json` 看 status/rc；`tail
      cluster_results/job/<tag>/01_*.log`。若 seed 需重派：同 tag 重跑分发器即可（脚本已幂等，
      fb938aa3 起）。
  - **K7 / K8 衔接**：K7（共置手写随宿主）独立可并行——标记文件按类同段落盘，宿主判定不受影响；
    K4 落地后标记文件整体挪入 `java_marker` crate，届时 D8 图只剩 inherit / hidden 边（⑤ 口径）
    的验证归 K8 档案验收（连同标记 crate 自身体量——本轮已随 `s7k6b-seg` 产出逐例统计）。
- **K4 实施（10-12，分支 s7-k4，基于 s7-k6 a07fbb6f）**：目标 = 解开声明链前提——标记部分依赖的
  `__Handle` / `Object` / `__ClassDesc` 等全在手写 INFRA（overlay 进声明底段 crate），无 K4 则不存在
  可置于声明链之下的独立 `java_marker` crate（标记 crate 依赖底段、底段又要标记类型 → 环）。分阶段
  落地，每阶段链接级验证（K6 教训：check 不覆盖 extern 声明 / 导出定义的配对）。
  - **阶段 1（e6edfa24，已推 origin/github）**：`runtime/java_marker` 独立 crate 建立真源——
    `obj_ref`（`__Obj`；`group::Ops` / `Header` 转公开供行为层引用）与 `sync_model`（持锁登记
    `HoldSlot` / `hold_slot` 自 exec_context 移入 `held::slot`；`__process_static!` / `__DynFn!`
    宏随 crate 根，java_runtime 再导出保持 `crate::__process_static!` 旧路径）搬入。**行为经
    `unsafe extern "Rust"` 链接期声明转交**（与 `rava_layer` 拆层同一机制）：分配路径的区域登记
    （`region::__on_alloc` → `__rava_marker_region_on_alloc`）与释放路径的监视器摘表
    （`monitor::forget` → `__rava_marker_monitor_forget`），定义在 java_runtime `marker_hooks.rs`
    （`#[export_name]`，签名取自同一类型 → 漂移即链接失败）。`java_runtime` 依赖它并按原模块路径
    整体再导出（`pub use java_marker::{obj_ref, sync_model}`——`crate::obj_ref::*` 等既有引用零变化）；
    `object.rs` 的 `impl __Obj<dyn ObjectVTable>::is_jvm_null` 随 `__Obj` 改泛型入 obj_ref（orphan），
    `ObjectVTable` 特化形态（含 debug 断言）待其移入后恢复。发射侧：overlay 镜像 java_marker 进
    scratch（版本唯一化、真源已无删除、生成文件保留）；vendor 兄弟闭包排除之（生成类标记文件后续
    要进该 crate，静态 vendor 拷贝不可承载）；声明层 Cargo.toml 注入依赖；workspace 成员与 archive
    版本戳补齐；**D8 把 java_marker 真源字节计入底段（foundation）体量**——INFRA 换 crate 不换切段
    输入，切段逐类与之前一致由构造保证。
    验证：generator `cargo check --release --tests` 过；宏单测 plan 41/41；**HelloWorld emit→编译→
    rava-link→运行输出与 JVM 逐字一致**（extern 钩子链接级验证——符号未配对即链接失败，此跑证明
    声明/定义闭环）。
  - **阶段 2（a867b085，已推 origin/github）**：K6 偏差注记的「剩余项清单」全量落地（宏侧，过渡
    形态仍同 crate）：`moves_to_trait` 放开——字段访问器（`__get_*` / `__set_*`）、`_init_not_null`、
    `__init_on`、装箱工厂族 `valueOf_*`、接口载体 `__default_*` 移入方法 trait（K3 收尾为压峰值
    留固有的名单；K4 搬 wrapper struct 进标记 crate 后固有 impl 必须与 struct 同 crate，只能经声明
    crate 的方法 trait 承载）；`__vt()` 自独立固有 impl 改入方法 trait 成员（调用点与访问器同经
    `X__Methods` 解析；invokespecial 的 `(obj).__vt()` 由 owner 登记覆盖；字段指令与 invoke 指令
    同经 `record_member`，生成侧导入登记无需改动）。**接口载体拆层**（`expand_interface` 按
    `meta.layer` 分段）：标记段 = 载体身份面（struct、Default / From 两向 / PartialEq / Debug /
    Deref、`__from_image` / `__IMAGE_NULL`、`BINARY_NAME`、`is_jvm_null`）；声明段 = `I__VTable` +
    statics + 初始化状态机 + `I__Methods` 及其实现（孤儿规则允许：trait 本地、载体类型外来）；
    Full 拼回项序不变；mod.rs 标记层不再对接口早退为空。手写层（b1013-fix 清单口径）：
    `reflect_dispatch` / `reflect/array_impl` / `proxy_dyn_impl` 补七个包装类 `__Methods` 匿名导入。
    验证：宏单测 plan 41/41、缺省 35/35（守护测试随新形态更新：装箱工厂 / 访问器 / `__vt` /
    `__default_*` 应在 trait 声明）；**HelloWorld、CollectorsDemo、DeepCopy 三例 emit→编译→
    rava-link→运行输出与 JVM 逐字一致**（覆盖装箱协议面 / 访问器 / 虚分派 / lambda-sam / 接口
    default / 反射 clone 链）。
  - **K4-2a（✅ 10-13 落地，阶段 3 的前置首步——交接节「可行出路」的实现）——手写伴生
    固有 impl 迁移到 `__XHw{N}` 扩展 trait**：wrapper struct 挪入 java_marker 后伴生文件的
    `impl X {` 全部 E0390（阶段 3 受阻根因）；本步把声明层的这一固有依赖转成本地 trait：
    - **形态**：生成器对每个有 hw 块的类在其声明文件 `java_class!` 块后发射
      `pub trait __XHw{N}<泛型组>: X__Methods<自类型实参> [where …] { 条目; }`——泛型组 /
      实参 / where / 条目签名（模式规范化：`mut self`→`self`、`mut x`→`x`、复合模式→`_`）/
      cfg 属性全部机械镜像自伴生 `impl __XHw{N} for X` 头（`input::handwritten::hw_impl_blocks`
      文本级扫描）。**多块逐块成 trait**（一 trait 一 impl：`class_impl.rs` 三块 →
      `__ClassHw` / `__ClassHw2` / `__ClassHw3`；跨文件同理 `string_ext` → `__StringHw`、
      `string_impl` → `__StringHw2`——块序 = 文件遍历序 × 位置序，与改写脚本同一口径；
      Rust 不允许同 (trait, 类型) 双 impl，合并块则动体，故逐块）。**块后追加**：拆层后
      只在声明层（宏块尾文本不进 body crate），标记文件也不含（marker_text 只取宏块）。
    - **改写**（`scripts/diag/k4_2a_rewrite_hw_impls.py`，幂等）：104 份 `*_impl.rs`/`*_ext.rs`
      第 0 列固有 impl 块 → `impl __XHw{N} for X {`（体不动；本地辅助类型 `Prim` / `Instance` /
      `_BitOp` 与手写根类 `Object` 不改——按「名字在本树有定义」判定；fn 体内缩进 impl 不碰）；
      块内顶层条目剥 `pub` / `pub(crate)` / `pub(super)`（trait impl 条目不允许可见性修饰）；
      文件头补 `use super::__XHw{N};`（本类全部块——跨块 / 跨文件互调都在作用域）。
      `--fix-callers`：非伴生手写文件的 hw 调用面（UFCS `X::m(` 与单主接收者 `.m(`）
      补 `use crate::<包>::__XHw{N} as _;`（16 份；多主 / 常用泛型名跳过，编译错误兜底）。
    - **调用点导入**：`ty::HwNames`（binary → 各块条目名集）经 `EmitShared` 注入 `TyCtx`，
      `NameScope` 登记 (binary, 块序号)，`imports::fill` 发射 `use …::__XHw{N} as _;`
      （与 `X__Methods` 同机制；`prune_uses` 对 `__` 前缀名单恒保留——留体 fn 无需账本）。
      登记点（按 Java 成员名匹配块：名 / `名_` 重载后缀 / `__impl_` 前缀，双向匹配）：
      `record_member`（指令链上声明类与接口 owner）、`resolve_static_field`（字段名 + `set_`
      前缀——手写静态访问器）、phase2 继承转发（手写成员 `__impl_` 体 / 接口载体 target
      直调——宏合成转发体的调用点所在文件）、字面量锚点（ldc UTF-16 串
      `String::from_utf16_lit`、类字面量 `Class::for_class`、拼接 `String::of`、静态字段
      UTF-16 初值）。
    - **口径保持**：`impl_methods` 名单 = `pub fn` 扫描 ∪ hw 块条目（原 `pub(crate)` /
      `pub(super)` 条目并入——名单语义本就是「wrapper 上由手写提供」，核过 55 名无一与
      类模型方法 rust 名相撞，verdict 无翻转；种子集合作业再守一道）；闭包分析器对
      `impl __XHw{N} for X` 块内条目按 `is_pub` 计（`closure/scan.rs`——闭包 / 种子口径
      不变）；行表 `companion_fns` 收集 hw trait impl 块（栈帧行表不丢，
      TestStackWalkerFrames 关键）；`java_runtime/build.rs` `scan_impls` 块态识别 hw
      trait impl（native_status 实现集不漂移）；D8 体量权重不计 hw 匿名导入行（同接口
      方法 trait 口径——纯名字解析行，`decl_side::hw_trait_import_bytes`）；trait 声明文本
      本体（约 110 文件 × 0.3–2 KB）照常计权（段边界允许的「trait 声明行」漂移口径）。
    - **验证（tag 前缀 s7k4d-，10-13 交接时在跑——回收法见下「K4-2a 验收与交接」）**：
      宏单测 36/42 ✅（jp2）；input 25 / ty 67 / closure 252 / emit 101 / driver build_cli 17
      全绿（本机；emit 测试夹具补 java_marker crate——阶段 1 起 9 个 project 测试未运行到
      的既有破损，本步修复）。链接级（s7k4d-hw1，sg2 挤占后改派 jp1）、种子四例
      （s7k4d-seed，kr1）、确定性两项（s7k4d-det，us1）交回时在跑（集群被其他会话的
      rr-* 作业挤占，分发器在 30s 重试循环中等待资源启动——分发器本机 setsid nohup
      存活，不随会话终止）；切段（s7k6b_segments.sh 对 a867b085——预期差全部为段边界
      移动（trait 声明行口径），无类增删）与逐 crate 峰值（s7k6_peak.sh 对 a07fbb6f，
      ±10%）未派发。
  - **K4-2a 验收与交接（10-13）**：
    - **在跑作业回收**：`cat cluster_results/job/s7k4d-hw1|s7k4d-seed|s7k4d-det/summary.json`
      看 status/rc；日志 `tail …/01_*.log`；分发器（本机 setsid nohup，日志
      `/tmp/s7k4d-*-dispatch.log`）被杀时同 tag 重派即可（脚本幂等，闭包 json 已在即跳过）。
      hw1 失败（rc=1）时先看编译错误清单——预期形态为漏登记的 hw 调用点 E0599（补登记点
      或 fix-callers 名单）与个别签名镜像偏差（hw_trait_decl 对应条目），逐个修后重派。
    - **未派发作业**：切段 `bash scripts/diag/s7k6b_segments.sh <基线 042465b7（阶段 2 尖）
      或 a867b085>`（新侧忽略标记文件；差异应全为段边界移动——trait 声明文本 ~110 文件
      ×0.3–2 KB 计权 + hw 导入行不计权（decl_side::hw_trait_import_bytes））；峰值
      `bash scripts/diag/s7k6_peak.sh a07fbb6f`（±10% 判据，hw 导入行是纯解析开销）；
      链接级五例 = HelloWorld / CollectorsDemo / DeepCopy / TestStackWalkerFrames /
      TestAnnoReflect（`run_tests.py --filter` 多过滤项一次跑）。
    - **阶段 3 重放（下一步）**：本步落地后 `git cherry-pick c66f7814`（或按「阶段 3」节
      清单重做）应可过 E0390 面；已知将再撞两处：①`prune_marker_header` 剪 `__VTable` /
      `__Methods` 后缀单名导入（E0432，交接节已记修法——`prune_uses` 对标记文件头加剪
      后缀名）；②伴生文件里的 **std trait impl**（`impl From<&str> for String` /
      `impl Add<…> for String` / `impl Display for String` 等）在 wrapper 挪入 java_marker
      后是孤儿规则 E0210（非 E0390——固有 impl 之外的另一面，c66f7814 只钩走了 String
      构造两枚）。处置需设计（搬 java_marker 需行为面 / 留声明层违孤儿规则 / 经
      `__XHw` 承载 std trait 不可能）——建议先 cherry-pick 看实际错误清单再定。
      另注意 object_ext.rs 等随迁文件的 `use crate::…::__ClassHw as _;` 在 java_marker
      侧不可解析（`crate::` 变指 java_marker）——c66f7814 的钩子化会替换这些调用点，
      重放时按冲突逐个处理。
    - **机械改写的可重复性**：`scripts/diag/k4_2a_rewrite_hw_impls.py`（幂等；新增伴生
      固有 impl 后重跑 + `--fix-callers` + 生成器扫描自动跟上；ordinal 随遍历序稳定）。
  - **阶段 3（⛔ 10-12 原子落地受阻，已回退到可编译状态——续作入口）——INFRA 主体搬移 +
    标记文件挪入 java_marker**：
    - **受阻根因（原清单外的结构性缺口，续作必须先解）**：伴生手写文件（`<x>_impl.rs` /
      `_ext.rs`，约 100 份：`class_impl` / `string_impl`+`string_ext` / `thread_impl` /
      `throwable_impl` / `double_impl` / `shutdown_impl` …）对**生成类 wrapper 的固有
      impl**（`impl Class {`、`impl String {` 等；native/边界成员的真实定义，生成侧以
      `#[impl_methods = "..."]` 名单引用它们，调用点（含生成的方法体）按固有方法解析）。
      wrapper struct 随标记文件挪入 java_marker 后，定义 crate 变为 java_marker，这些
      固有 impl 在声明层 crate 全部 **E0390**（固有 impl 必须与 struct 同 crate，§9.2
      的硬约束——K3/K6 只把**宏生成**的可读层成员移入了方法 trait，手写面未动）。服务器
      实测（s7k4c-cddp，c66f7814）：DeepCopy 编译失败；另一处独立小缺口同场暴露——标记
      文件头的 `use crate::…::X__VTable;` 单名导入（`prune_uses` 的词规则：块内出现
      `X` 即保 `X__VTable`）在 java_marker 侧不可解析（vtable trait 留声明层），E0432
      （`standard_charsets_m.rs` 的 `CharsetProvider__VTable`）。
    - **不可行的绕行（本轮论证排除，续作勿再走）**：
      - 伴生整体挪入 java_marker：其方法体大量调用声明层方法 trait（`__get_*` /
        `Class__Methods` 等），且违反「标记层不含 `__Methods`」的用户要求（10-12 两项
        之一）；
      - 只挪部分类的标记文件：标记文件的引用闭包（祖先 ∪ 接口 ∪ 字段类型）在
        java.base 上不可截断（与手写最小 Class 占位同一级联），部分挪 = 全量挪；
      - wrapper struct 双定义（两 crate 各一份）：不同名义类型，From/identity 全断。
    - **可行出路（建议的续作首步，工作量以文件数计）**：手写面从固有 impl 迁到**声明层
      扩展 trait**——生成器对每个有 `impl_methods` 的类发射 `pub trait __XHw<..>:
      X__Methods<..> { /* 与 impl_methods 同形的签名（生成侧已有全部签名——今天的
      `// [meta]` 注释）*/ }`（声明层 crate，trait 本地 + 类型本地，孤儿规则允许），
      伴生文件把 `impl X {` 机械改写为 `impl __XHw for X {`（体不动），调用点由生成器
      的导入机制补 `use …::__XHw as _;`（与 b1013-fix 的方法 trait 导入同一机制；手写
      调用点按既有排查法补 use 行）。**落地后 wrapper struct 不再有声明层固有依赖**，
      阶段 3 的搬移清单即可原样执行。改写约 100 份伴生文件（模式机械：impl 头替换 +
      `Self::__class_init` 一类注入点核对 `#[jvm_native]` 语义不变），建议单独成步
      （K4-2a）并先在 3–5 份文件上走通宏 / 生成器配合面。
    - **已落地并保留的部分（854dc7f3，零操作安全件）**：接口宏拆层修正——`expand_interface`
      的 `Layer::Decl` 不再包含载体身份面（iface_marker；原 `_` 臂同时给了 Full 与
      Decl，标记文件独立成 crate 后会重复定义）；当前无接口文件带层属性，零操作；宏守护
      单测 `interface_layers_split_carrier_face`（宏单测 36（缺省）/42（plan）过，
      s7k4c-macro，jp2 rc=0）。
    - **完整实现已做并在分支历史（c66f7814，revert 6f9ac2a0 保留可 cherry-pick）**：
      运行时搬移与 39 枚 `__rava_marker_*` 钩子（error 拆分 / object 三件 / array 系统 /
      handle / desc / java_fmt / _is_jnull 族 + java_runtime 按原路径再导出）、
      marker_side v2（标记文件落 java_marker、接口拆层、不透明类文件双发、宿主再导出
      改 `pub use java_marker::<pkg>::<m>::<Name>;`）、java_marker 包 mod 树 / lib.rs
      补全 / prelude / vendor 重定向、闭包分析器双根扫描（种子集合构造性不变）+ 缓存
      指纹、InstrFacts / root_api / handwritten_vtable_impls 扫两处。**续作两处修补**：
      （a）`prune_marker_header` 加剪 `__VTable` / `__Methods` 后缀的单名导入（E0432 的
      修法，一处小改）；（b）上述 __XHw 扩展 trait 落地后重放。generator `cargo check
      --release --tests` 在 c66f7814 即过（0 警告），宏单测 42/36 过——受阻仅在
      scratch 链接级。
    - **standalone 口径变化（回退后不生效，续作落地时再记）**：java_marker / java_runtime
      真源 standalone `cargo check` 非绿（getClass 签名引用的 Class 只在 scratch）；
      以「scratch 全构建 + 宏单测 + generator check」为口径。
    - **阶段 3 验证作业（tag 前缀 s7k4c-）回收状态**：`s7k4c-macro` ✅（jp2，rc=0，
      36/42——含新守护）；`s7k4c-hw`（口径误用系统 python，作废）/ `s7k4c-hw2`
      （c66f7814，rc=1 编译失败——证据如上）/ `s7k4c-cddp`（同，DeepCopy compile
      error 3m56.6s）/ `s7k4c-hw3`（kill，证据已足够）；回退后 `s7k4c-hw4` ✅
      （854dc7f3，jp2 rc=0）：分支尖 = 阶段 2 形态 + 宏修正，HelloWorld
      emit→编译→rava-link→运行对比 PASS——交接状态可编译可运行。阶段 2 基线验收
      `s7k4-seed`（jp1）/ `s7k4-det`（us1）/ `s7k4-seg`（kr2）/ `s7k4-peak`（sg1）
      交回时仍在排队（批闸门），回收法见上「阶段 2 服务器验收」节，收数后归阶段 2 记录。
  - **阶段 2 服务器验收（tag 前缀 s7k4-，基线 s7-k6 a07fbb6f，交回时在跑）**：
    - `s7k4-macro` ✅（jp2，rc=0）：宏单测 **plan 41/41、缺省 35/35**。
    - `s7k4-seed`（jp1）：`s7k6_seed_sets.sh a07fbb6f` 四例 × 种子 0/1/2 闭包集合对照——日志尾
      `seed-sets: ALL-IDENTICAL` 即过（`--fetch` 声明 build/s7k6-seed/summary.txt）。
    - `s7k4-det`（us1）：`cargo test -p driver --test closure_cli closure_independent
      -- --include-ignored` 确定性两项。
    - `s7k4-seg`（kr2）：`s7k6b_segments.sh a07fbb6f` 三例切段逐类对照——**预期有差**（阶段 2
      声明文件含 trait 声明行，体量增 → D8 重切），差异应全部为段边界移动（无类增删）；绝对值
      记入本节。
    - `s7k4-peak`（sg1）：`s7k6_peak.sh a07fbb6f` 三例逐 crate 固定切段重建峰值，判定口径 =
      声明 crate 峰值对 s7-k6 不超 +10%（阶段 2 估 +0.5–1.5 pp，s7-k6 基线 HW 底段 +8.5%——
      临界，若破线归因 trait 项回涨，阶段 3 收回后复测）。
    - 回收：`cat cluster_results/job/<tag>/summary.json`（status/rc）+ `tail …/01_*.log`；脚本
      已幂等可重派。
  - **K5 / K7 / K8 衔接**：K5（类初始化骨架去按类展开）与本阶段无依赖，可并行派发；K7（共置手写
    随宿主）在阶段 3 落地后钉底 = 行为层 INFRA 直连类；K8 档案验收在阶段 3 后做（⑤ 口径的 D8 图
    验证 + 标记 crate 自身体量随 `s7k4-seg` 口径统计）。
- **K4 及后续方案**（10-11 用户暂停派发新代理，K4 未派，以下为续作入口）：
  - **K3 收尾**：10-11 晚已完成分解归因与两轮修复（15276d82，见上「K3 收尾」节）：切段漂移全部收回（与基线逐类一致）、底段峰值 +24–31% 收窄到 +8%（本机口径）、body crate 达标、DeepCopy 工作区复测与全量单测已发服务器作业。**剩余决策**：~~须用户裁决后合入~~ **已裁决（2026-10-12 用户）：阈值放宽至 ±10%，K3 随下一批合入**（batch-1012 未等 K3；固有 impl 保留 / 选择性 trait 的进一步压缩并入 K6 取舍；s7k3-fix2 三例同机复测 +7.1–9.5% 见上表）。
  - **K4** 运行时基础设施分层：**10-12 已开工（分支 s7-k4，见上「K4 实施」节）**——阶段 1
    （obj_ref / sync_model 入 java_marker + extern Rust 钩子，e6edfa24）与阶段 2（K6 剩余项清单
    入方法 trait + 接口载体拆层，a867b085）已推双远端并过链接级验证；阶段 3（INFRA 主体搬移 +
    标记文件挪入，原子）为续作入口，设计与不可行方案排除见该节。新增两项要求的落实：
    - 标记层基础只含类型与句柄，不含任何 `use …__Methods`——阶段 1/2 的 java_marker 真源
      零方法 trait 引用（宏单测 `marker_layer_is_signature_free` 守护标记展开面）；
    - 迁移前先扫 K3 加入的 `use` 行，确定哪些文件属行为层——阶段 2 已按 b1013-fix 清单补齐
      装箱协议面三文件；阶段 3 的行为层钩子清单见「K4 实施」节。
  - **K5** 类初始化骨架去按类展开：与 K3、K4 无依赖，可与 K3 收尾并行。
  - **K6** 标记 crate 生成：前提是 K3（含第 5 项改名——✅ 已随 K6 前置项完成，std 同名与接口载体成员均已在方法 trait 上）、K4、K5。**机制已随 10-12 K6 本体落地（同 crate 标记文件过渡形态，见上「K6 本体」节）**；独立 `java_marker` crate 待 K4（标记文件搬移 + 路径前缀 + 访问器/`__vt` 并入方法 trait）。方法 trait 与其 impl 留声明 crate；标记层展开文本不含 `__Methods`（✅ 宏单测守护）。
  - **K7** D8 共置手写随宿主：独立。注意 K3 增加的 `use` 行改变了按体量切段的结果，模拟器（`decl_scc_sim.py`）的体量口径需同步计入导入行，或改为不计 `use` 行。
  - **K8** 档案验收：按上表；连同 K2 留下的 §4.3 热循环用例、StockTrans 计时一并补测。
- 下一步：
  - K4 阶段 3（INFRA 主体搬移 + 标记文件挪入 java_marker，原子——见「K4 实施」节）；阶段 2 的
    服务器验收（s7k4-*）回收后视峰值结论决定是否先做峰值归因；
  - K5（类初始化骨架）与 K4 剩余互不依赖，可并行派发。
