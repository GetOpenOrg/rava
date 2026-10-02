# C4 回归修复与 C6 回调零声明实施笔记（2026-10-01 ～ 10-02）

分支 `c4-regfix`。C6 的目标与删除清单见 [`2026-09-29-rust-closure-analyzer.md` §3.7.1](2026-09-29-rust-closure-analyzer.md#371-回调零声明c6)，
本文记录实施过程、验收证据与遗留审计。

## 一、提交序列

| 提交 | 内容 |
|---|---|
| 0c251b4b / 9d920193 / 26bf9671 | C4 回归修复（sigpoly 调用点登记、动态对照按模型调用点归因等，TestBmhDynamicSpecies 漏覆盖归零） |
| f4c0856e | C6 步骤 1：手写体回调边推断补全（模块单元跨文件调用图 `rtfn`、VM 规则目录 `vmrules`、根类 vtable 别名、`__site` 伴生体、宏体扫描、`T::from` 静态类型、钩子判定改为「不对应 Java 成员的 pub fn」、sysprops 改读推断） |
| 535b8834 | E0308 回归：`FileChannelImpl.open` / `<init>` 手写签名的 `parent` 形参改为与描述符一致的 `Closeable` |
| a3f4a479 | 手写体继承成员需求 `hw_inherited`；手写辅助 fn 返回类型入静态类型；审计 `hw_untyped_sites` |
| 59dbedc1 | C6 步骤 2：删除全部 `upcalls` 声明（66 文件 173 处）、`error.rs` vm-upcalls 行与声明机制 |

## 二、删除结果（终态核对）

- runtime 下 `upcalls =` 属性参数：0；`error.rs` `// vm-upcalls:` 行：0。
- 已删机制：`scan.rs upcalls_of`、`FnInfo` / `MemberHw` 的 `upcalls` 字段、`parse_upcall`、`Handwritten::vm_upcalls`、
  `lib.rs` vm-upcalls 根回调循环、`worklist.rs root_upcall`；`rava_macros` 的 `native_attr` 文档不再提 `upcalls`。
- 回调边只来自推断：`engine/hw_infer.rs hw_upcalls` 只返回推断结果。
- `closure.toml [dynamic] vm_upcall_classes` 不属本项（动态对照归因用）。

## 三、步骤 2 暴露的编译期缺口：手写体继承成员（L1 编译期事实）

**现象**：删声明后 DeepCopy / TestBmhDynamicSpecies 报 E0599——`DecimalFormat` 上没有 `setParseIntegerOnly`
（`locale_provider_adapter_impl.rs` `getIntegerInstance` 的 `df.setParseIntegerOnly(true)`）。

**根因**：
1. 手写文件随其类进入生成范围即**整体编译**，不可达的手写 fn 同样要过类型检查。其中「接收者.方法」形态、方法声明在接收者
   超类型上的调用，要求接收者类承载该继承成员（继承转发）。发射层只为生成方法体登记继承成员需求（`inherited_requests`），
   手写体的这类需求没有来源。此前由 `CompactNumberFormat` 的声明把 `setParseIntegerOnly` 拉成虚方法，掩盖了缺口。
2. 语法推断把 `Self::new_format(…)` 按名字前缀 `new_` 当成构造器，`df` 被推成实现对象 `NativeNumberFormatProvider`，
   而非辅助 fn 声明的返回类型 `DecimalFormat`。

**修法（a3f4a479）**：
- 分析器 `engine/hw_inherit.rs`：遍历闭包内全部共置手写文件与模块单元的接收者调用点，静态类型解析到 Java 类、
  方法声明在其超类型上的实例方法 → 登记 `(接收者类, 方法)`，输出到 `closure.json` / `ClosureFacts` 的 `hw_inherited`；
  发射层把它并入 `inherited_requests` 同一账本。
- 静态类型：本文件 impl 块中辅助 fn 的声明返回类型（经 `?` 剥 `Result` / `Option`，`Self` 换成 impl 类型）优先；
  构造器形态 `T::new*` 记为 `Ret`，本文件没有同名声明时才取 `T`。`let x: T`、转型、方法链走同一条 `SType` 路径。

## 四、验收（2026-10-01 用户口径）

口径：程序正确、高效运行，二进制更小，不纳入不必要的类。闭包缩小是目标，判据只看正确性：
用到被缩掉的簇的测试 e2e 通过、动态对照漏覆盖为 0、不命中存根。

缩掉的簇及其在全语料（`tests/e2e`）中的直接使用者：

| 簇 | 直接使用者 |
|---|---|
| `CompactNumberFormat` | CompactNumberFormatExample |
| `SimpleDateFormat.parse` | DateTest |
| 字符集 `forName` / 编解码 | 19 例（TestCharsetForName、TestCharsetNamedStreams、TestStreamEncoderCharsets、UTF8EncodeDecode、Base64Demo 等） |
| `Formatter %t` / `ZoneOffset` | DateTimeFormatterDemo、SwitchWithPatternMatchingThirdPreview、TestZonedDateTime、ZonedDateTimeDemo |

按 grep 统计，簇使用者共 63 例，与验收集 27 例取并集为 83 例。这 83 例的 e2e 批次交由主会话运行，
批次另加 DeepCopy 与 HelloWorld，结果待回填。单例验证见下：
DeepCopy、TestBmhDynamicSpecies、TestCtorReflect 在 59dbedc1 及其后两次合并的头上均运行通过，未命中存根。

## 五、审计：手写体接收者静态类型推不出的调用点（`hw_untyped_sites`）

`closure.json` 的 `hw_untyped_sites` 列出这样的调用点：方法名是闭包内某个 Java 方法的名字，但接收者的静态类型和语法推断
都解析不到 Java 类（同一宿主下同名合并为一条）。静态类型逐级解析在 `engine/hw_stype.rs`（`stype_desc`），断开原因决定类别：

- `chain 宿主 方法 ← 断开级`：基底与中途各级都是 Java 类，某级在类型层次上查不到或无法唯一确定。推断缺口，**终态 0**。
- `value 宿主 方法`：链中途的值不是 Java 对象（数组 / 基本类型，或手写 fn 返回的 Rust 类型），之后的同名调用是 Rust 方法
  （`JArray::get` / `to_vec`、`Iterator::map` / `collect`、`Option::unwrap_or` …），不是 Java 回调，不计入。
- `camel 宿主 方法 ← 基底类型`：基底不是 Java 类、方法名为 Java 驼峰形，附基底类型路径（推不出为 `?`），逐条说明见下。
- `lower 宿主 方法`：基底无类型、全小写单词名，与 Rust 标准方法同名，语法上无法区分，不计入。

### chain 清零（c6audit-622ea4b0 的 63 条 → 0）

c6audit-622ea4b0（kr1，21 例）的 union 为 937 条：chain 63、camel 47、lower 827。chain 的断开级分五类，修法均为推断补全：

| 断开原因 | 例 | 修法 |
|---|---|---|
| 数组 / 基本类型值上的 Rust 方法 | `String.value [B` 上的 `to_vec`、`Thread::MAX_PRIORITY()` 上的 `min` | 逐级按字段描述符解析；数组上的 `get` 取元素描述符，其余数组 / 基本类型上的调用归 `value` |
| 跨文件的手写 fn（Java 类型上的非 Java 方法） | `Class.__declared_method_meta`、`LocaleResources.__bundle_chain`、`UnixFileAttributes.ok` | 共置手写与模块单元的 impl 块 fn 返回类型入 `ClassHw.rets`，按超类型查找；返回非类型路径（元组 / 引用 / 无返回）记空路径 → `value` |
| static 字段访问器 | `Thread::NORM_PRIORITY()` | 路径调用在类型上无同名 Java 方法时取 static 字段描述符 |
| 协变返回 | `UnixPath.getFileSystem` 返回 `{FileSystem, UnixFileSystem}`（桥） | 取各返回类型中是其余全部子类型者 |
| Result / Option 适配 | `Class.ok`、`Class.unwrap_or_default` | `x.ok()?` 与 `unwrap_or_default` 透传被包装值；未经 `?` 的 `x.ok()` 是 Rust `Option`，无 Java 静态类型 |

同时补齐的静态类型来源（使原先 `?` 基底的调用点解析到 Java 类）：带类型注解的闭包形参（`|p: &UnixPath|`）、
`match r { Ok(x) => x, Err(..) => .. }`、`Clone::clone(&x)`、带元素类型注解的容器 `ptypes: JArray<Class>` 的 `get(i)`、
本文件顶层自由 fn 的返回类型。DeepCopy 实测：chain 0、value 53、camel 57、lower 770；闭包类集 1906 / 方法集 13457
与改动前逐项相同（解析更准只体现在调用边，如 `UnixPath.getByteArrayForSysCalls` 的派发多出 `UnixFileSystem.defaultDirectory`）。

### camel 逐条说明（DeepCopy 57 条，按基底类型归组）

全部不是漏掉的 Java 回调：

1. **Rust 值的 Display（44 条：`toString` 43 条，另有 `Object hashCode ← Instance`）**。格式化宏里静态类型已知的实参登记为 `toString()` 调用（Java 对象的 Display 即
   `toString`），基底是 Rust 类型时是 Rust 自己的 Display：
   - 基本类型与 `str` / `std::string::String` / `Vec`：array、error、lib、proxy_dyn、reflect_dispatch、species_dyn、
     FileInputStream、FileOutputStream、Object、String、MethodHandle、MethodHandleNatives、Constructor、Method、
     NetworkInterface、JavaLangAccess、Unsafe、ConstantPool、Reflection、ArraysSupport、Preconditions、Wrapper、
     UnixChannelFactory、GetInstance、LocaleResources 下的 `toString`；
   - `std::io::Error` / `std::backtrace::Backtrace`：FileInputStream、FileOutputStream、FileChannelImpl、Throwable、Reflection、error；
   - 运行时内部类型 `crate::error::JvmError` / `crate::sync_model::__Shared` / `Instance`：error、array、Object（`Instance` 上的 `hashCode` 是其 Rust 方法）。
2. **根类 vtable 自身（7 条）**：
   - `java/lang/Object` 的 `compareTo` / `getClass` / `hashCode ← ?` 是 `self.0.x()`，`self.0` 是 `Rc<dyn ObjectVTable>`；
   - `array getClass ← ?` 是 `view.origin.0.getClass()` / `Into::<Object>::into(T::default()).0.getClass()`；
   - `java/lang/object` 的 `getClass` / `hashCode` / `toString ← T` 是泛型 `T: Into<Object>` 的转发。
   这些都是 invokevirtual `Object.x` 在运行时的落点，分派目标由字节码调用点的派发覆盖，手写体不引入额外目标。
3. **数组镜像（2 条）**：`array getClass` / `toString ← crate::array::JArray` 是 `JArray` 自己的数组类语义（JLS §10.8），在 array.rs 手写。
4. **手写实现对象自身（3 条）**：`JavaLangAccess` 的 `countPositives` / `getBytesNoRepl` / `newStringNoRepl ← SystemJavaLangAccess`
   是 SharedSecrets 实现对象在自身 impl 内的互调，被调的是同一手写对象的 Rust fn。
5. **Option 闭包形参（1 条）**：`UnixFileSystemProvider isDirectory ← ?` 是 `attrs.as_ref().map(|a| a.isDirectory())`，
   `attrs` 为 `….ok()` 的 `Option<UnixFileAttributes>`；`UnixFileAttributes.isDirectory` 本身是该类手写（`handwritten:provides`），不是 Java 回调。

### 21 例 union 复核（c6audit-ea6f61fb）

union：chain 0、value 55、camel 63、lower 849。camel 比 DeepCopy 多 6 条：`ProcessHandleImpl toString ← i64`、
`ProcessImpl toString ← i32 / str`、`Proxy$Dyn toString ← str`、`AnnotationInvocationHandler toString ← Vec` 归第 1 组；
`AnnotationInvocationHandler toString ← ?` 不归组——它是 `memberValueToString` 里 `each(&<JArray<Object> as From<Object>>::from(…), |x| … x.toString())`
的闭包形参，即 Object 数组元素（嵌套注解代理）上真实的 `Object.toString` 回调，推断缺口在闭包形参类型。

修法（推断补全，c6-generic-closure）：`handwritten/generic_fns.rs` 登记本文件带闭包形参的辅助 fn（自由 fn 与 impl 关联 fn，
`Self::f` 按 impl 类型末段查）。闭包形参类型取自 `impl Fn*(…)` 形参或 `Fn*` 约束的类型形参（泛型列表 / where 子句）；
其中的类型形参按其余实参解出——声明为 `T` / `&T` 取实参静态类型，声明为 `X<…, T>` 取实参元素类型（作用域内容器元素，
或实参写明的 `<X<E> as Tr>::f(…)` / `X::<E>::f(…)` 的 `E`）。调用点访问闭包实参时代入，注解类型优先。
TestAnnoReflect 实测该条消失（camel 49，余 `← Vec`），21 例 union 的 camel 应为 62 条、全部归组。
回归用例：e2e `47_annotations/TestAnnoNestedArray`（嵌套注解数组 toString，expected 取 JDK 21 输出）。
c6audit-4173cebd 复跑：chain 0、value 50、camel 62，与上一轮相比只少了这一条，其余全部归入上述 5 组。

TestAnnoNestedArray 运行暴露 null_recv 健全性缺口：`first.annotationType().getSimpleName()` 的接收者被判恒 null。
`first` 是代理注解 `inners.value()[0]`，代理接口调用经 open 枢纽派发不到目标，结果类型集始终为空，读取点因此
**不驻留流图**；未建模派生（`engine/unmodeled.rs`）只给驻留节点打标，空读取点的派生标记无处存放，下游以它为接收者
的调用就按空集判了恒 null。修法：不驻留的读取点取虚序号（流图节点数之后、无出边）参与派生，`recv_unmodeled` 按虚序号查标记。

TestAnnoNestedArray 第二层嵌套注解成员丢失（`@Branch({…})` 打印成 `@Branch()`）：注解种子收集（`seeds/annotation.rs`）
把用户注解类型排除在补种之外，假定用户注解都有静态边。只出现在注解属性体里的嵌套用户注解（Branch / Leaf）没有静态边，
方法表从未入链，Branch 仅以类型级不透明发射、Leaf 不入闭包；运行期 `AnnotationType.getInstance(Branch)` 成员表为空，
AnnotationParser 按未知成员丢弃全部取值。修法：用户注解类型与 JDK 注解类型同等补种（仍只按挂载点传递可达入链，
与枚举 / Class 元素类型口径一致）。边界用例 `TestAnnoArrayMembers`：三层嵌套逐层下钻、各层空数组、8 种基本类型数组、
字符串（含转义 / 空串）、枚举、Class（用户类 / JDK 类 / 基本类型 / 数组类型）数组成员，expected 取 JDK 21.0.11 实测。

## 六、集成头 52bf5311 的日期 / 区域与 IO / 反射回归（C6 步骤 2 引起）

逐个对照 59dbedc1 删掉的声明与对应调用点，三类失败都是「声明删了、推断没接上」，不是 regress2 的提交引起的：

| 现象 | 调用点 | 推断缺口 |
|---|---|---|
| 9 例命中 `ResourceBundle.setParent` 存根（DateTest、CalendarTask、Currency 等） | `locale_resources_impl.rs` `__bundle_chain`：`chain[i].setParent(…)`，`chain: Vec<ResourceBundle>` | 索引表达式没有静态类型，容器的元素类型丢失 |
| FileIODemo 命中 `FileInputStream.read([BII)I` 存根 | `stream_decoder_impl.rs`：`self.__get_in_().read_arr_b_i_i(…)` | 字段名是 Rust 关键字时访问器带 `_` 后缀（`in` → `__get_in_`）；字段写入侧已还原，静态类型推导侧没有还原，按 `in_` 查字段查不到 |
| FieldDemo 报 `JavaLangInvokeAccess.unreflectField` 接收者为 null，动态对照漏覆盖 28 | `shared_secrets_impl.rs`：`MethodHandleImpl::__class_init()` | 类初始化入口 `T::__class_init()` 没有对应的回调目标，`MethodHandleImpl.<clinit>` 不可达，SharedSecrets 槽位没有登记 |

修法（推断补全，不加声明、不加种子）：
- 容器元素类型：`elem_type` 取 `Vec<T>` / `HashMap<K, V>` 末个泛型实参、`[T]` / `[T; N]` 的元素；作用域以 `<变量名>[]` 为键记录
  元素类型（来源为形参与带注解的 let），`v[i]` 的静态类型取它；`for x in v` / `&v` / `v.iter()` / `v.into_iter()` / `v.iter_mut()`
  的 x 绑定元素类型；同名变量重新绑定时清掉元素类型。
- 访问器字段名：`java_field_name` 统一把关键字后缀还原为 Java 字段名，静态类型推导、字段读写登记共用这一处。
- 类初始化：`T::__class_init()` 推断为回调目标 `Upcall::Init(T)`，按 JVMS §5.5 主动初始化 T（超类链与 `<clinit>`）。
- 静态类型推导从 `syntax.rs` 拆到 `handwritten/stype.rs`（文件行数约束）。

复抽（622ea4b0）又暴露一处：DateTest 命中 `NumberFormat.setParseIntegerOnly` 存根。`getIntegerInstance` 中
`df = Self::new_format(…)?`，动态类型推断按 `new_` 前缀把它当成构造 `Self`（手写实现对象，非 Java 类），接收者记为
「已推出」却解析不到类，回调边落空。修法（d78940ff）：构造器名形态、但由本文件 impl 块声明的 fn 是辅助 fn，不当作构造——
返回值动态类型推不出，按静态类型 `DecimalFormat` 的 open 接收者分派；构造登记与 fresh 绑定共用同一判定（`is_ctor_call`）。

## 七、范围外

- 合并 c3e2b40c 后，precheck 的 `native-missing` 从 0 变为 18：涉及 `Module.addExports*0`、`Unsafe.get/put*Volatile` 等。
  只合并、不带步骤 2 的树上数值相同，所以不是 C6 引入的。这一项由 regress2 的 native 缺口任务覆盖。
