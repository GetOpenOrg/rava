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
都解析不到 Java 类。每条格式为 `类别 宿主 方法名`，同一宿主下的同名方法合并为一条。类别如下：

- `chain`：推导链的基底已解析到 Java 类，中途某一级推不出。这是推断缺口，目标为 0。
- `camel`：基底没有类型，方法名是 Java 驼峰形。Rust 标准方法一律是蛇形，所以这类多半是 Java 值，需要逐条说明原因。
- `lower`：基底没有类型，方法名是全小写单词（get / map / set …），与 Rust 标准方法同名，语法上无法区分，不计入。

分类前的 DeepCopy 计数为 792 条，大头是与 Rust 标准方法同名的调用：map 54、get 43、collect 34、toString 32 等。
分类后的计数与逐条说明待补。

## 六、范围外

- 合并 c3e2b40c 后，precheck 的 `native-missing` 从 0 变为 18：涉及 `Module.addExports*0`、`Unsafe.get/put*Volatile` 等。
  只合并、不带步骤 2 的树上数值相同，所以不是 C6 引入的。这一项由 regress2 的 native 缺口任务覆盖。
