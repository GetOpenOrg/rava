# 重载命名：整个类型层次上一致、无冲突的确定规则

> 2026-10-07 · 分支 `gen-overload-hier` · 起因：`docs/known_failures.toml` TestXmlTransform
> （java_xml crate E0061 ×2 / E0308 ×1：`ContentHandler.endElement(3 参)` / `LexicalHandler.comment(char[],int,int)`
> 与子接口 `ExtendedContentHandler` / `ExtendedLexicalHandler` 的单参重载同取无后缀名）

## 一、目标（终态）

1. **一个名字只由视图决定**：Java 方法 `name:desc` 在某个 Rust 视图（类 wrapper / 接口载体）上的 Rust 名，
   是（视图类型, 名字, 描述符）的函数，判定只读视图类型自身与它的**超类型**（超类链、超接口传递闭包）。
2. **确定**：子类型、实现类、调用点、档案内容、crate 切分、处理顺序都不改变任何名字——同一方法在所有 crate、
   所有档案中取同一名字（java.base 的接口名不因 java.xml 里出现子接口而变）。
3. **无冲突**：任一视图内，参数段不同、互不覆盖的可见成员不同名；同一覆盖槽在不同视图里名字可以不同，
   视图之间以宏属性（`target` / `vtable_name` / `inherited_from`）映射，可读层只见视图名。
4. 生成器内无类名特判、无 JDK 类名字面量（`no_jdk_literals` / `jdk_literal_lint` 守护）。

## 二、命名单位与后缀

- 无后缀名 = Java 方法名（关键字转义见 `ident::safe_ident`）；后缀名 = `type_map::mangle_name(name, desc)`
  （参数段缩写，§15.1）。
- 「参数段」= 描述符 `)` 之前部分；协变返回（参数相同）是同一参数段。
- 判定结果是一个**名字集合**：视图 V 的集合含 n ⇔ V 上所有名为 n 的成员一律带后缀；否则一律原名。

## 三、规则

### 3.1 接口声明名（接口载体 / `Iface__VTable` 上的名字）

接口 I 自有的方法名 n：I 自有的同名方法参数段，并上全部超接口（传递）上**未被 I 自有实例方法覆盖**的同名
实例成员参数段，合计 ≥2 → I 上 n 带后缀；否则原名。

- 覆盖：参数段相同；或把超接口形参代入 I 视角类型实参后参数擦除相同
  （`interface Path extends Comparable<Path> { int compareTo(Path) }` 不构成重载）。
- 接口自有 static 方法与超接口同名实例成员异参同样计入（同在载体 impl 上）。
- 只看 I 与超接口：**子接口不影响超接口的名字**——这是跨 crate / 跨档案一致的根据。

例：`ContentHandler.endElement(3)` → 原名 `endElement`；`ExtendedContentHandler.endElement(String)`
（超接口 3 参未被覆盖，合计 2 种）→ `endElement_str`。两者在任何接收者视图里都不同名。

### 3.2 子接口视图（接口接收者经继承看到的超接口成员）

沿用声明名。唯一例外：来自**不同声明者**、声明名相同、参数段不同、互不覆盖（被更具体超接口同槽覆盖者不可见）
的成员——兄弟超接口各自声明的同名异参——在该视图里按描述符带后缀，载体方法以 `target` 转调声明名。
定义侧（子接口继承成员声明）与调用侧同用 `interface_view_member_name`。

### 3.3 类视图（类 wrapper 上的名字）

类 C 的「类视图参数段」：C 自有实例方法（含注入的接口 default）∪ 祖先类实例方法 ∪
**未实现的接口成员**（超类链实现的全部接口及其超接口上、超类链没有同名同描述符实现的实例成员；
泛型桥承载的接口成员不另计）。

| 规则 | 内容 |
|------|------|
| 1 自有 | C 自有（含 static）同名方法参数段 ≥2 → 带后缀 |
| 2 单调继承 | 父类集合中的名字 C 全部沿用 |
| 3 自有 ∪ 视图 | C 自有实例方法名 n：自有参数段 ∪ 类视图参数段 ≥2 → 带后缀 |
| 4 static | C 自有 static 方法与祖先实例方法同名异参 → 带后缀 |
| **5 未声明名（本次补）** | C 未声明的名字 n：类视图参数段 ≥2 → 带后缀 |

规则 5 之前，C 未声明的名字只经规则 2 从父类继承后缀；抽象类经兄弟接口或超接口层次继承到同名异参的抽象成员
（`abstract class S implements SerializationHandler` 不声明 `endElement` 时，`endElement(3)` 与
`endElement(String)` 都只在接口上）在 C 的 wrapper 上同取原名：继承成员认领（`emit/src/phase2/inherited.rs`
`RecvPass::interface_member` 的 `taken`）第二个落空，调用点按原名命中另一元数 → E0061 / E0308。
规则 5 让类视图与接口视图同一口径：**视图内同名异参即后缀**。

「只声明在接口上的成员」在类视图上的名字（`interface_member_local_name`）先取类视图集合判定，
与类自有成员、祖先类继承成员同一来源；不在集合内时再沿超类链找同名声明——异参即后缀。后一步覆盖
「只由合成桥承载的擦除接口成员」（枚举 `compareTo(Object)`）：桥不计入视图参数段，但其接口成员与类自有
`compareTo(E)` 异参，须取 `compareTo_obj`。

### 3.4 视图之间的映射

| 场景 | 映射 |
|------|------|
| 类实现接口：`impl Iface for Class` | 成员名取接口声明名，类方法名不同则 `target` |
| 类 wrapper 上的祖先类成员 | 接收者名（`receiver_member_name`）≠ 声明名时改签名名，虚方法以 `vtable_name` 指回槽名 |
| 类 wrapper 上的接口成员 | 类视图名，`target` 指回接口声明名 |
| 子接口载体上的超接口成员 | 视图名，`target` 指回声明名 |

同一覆盖槽在不同视图名字不同（例如 `ContentHandler` 视图 `endElement`，抽象类视图 `endElement_str_str_str`）
是视图化命名的固有形态，派发仍经同一 vtable 槽，语义一致。

## 四、性质

- **确定性**：3.1–3.3 只读视图类型及其超类型的方法表；超类型在类装载时必然在注册表内（JVMS §5.3.5），
  故名字与档案、闭包大小、crate 切分无关。缓存（`caches.overloaded`、`mangle_cache`）只是记忆化。
- **单调**：规则 2 保证后代不会把祖先已带后缀的名字改回原名；规则 5 新增的后缀只出现在**不声明 n**
  的类上——声明 n 的后代按规则 3 已因同一组参数段带后缀，故规则 5 不改变任何已声明方法的定义名与 vtable 槽归属
  （`vtable.rs raw_covariant_virtual_owner` 只比较声明者）。
- **无冲突**：视图内同名成员要么参数段全同（同槽，同名正确），要么 ≥2 种参数段 → 全部按描述符后缀，
  后缀名由参数段唯一决定，互不相同。

## 五、实现位置

- `generator/crates/ty/src/sig_types/mod.rs`：`overloaded_rec`（规则 1–5）、`add_unimplemented_interface_members`、
  `interface_member_local_name`、`receiver_member_name`
- `generator/crates/ty/src/sig_types/iface.rs`：`interface_overloaded_names`（3.1）、`interface_view_member_name`（3.2）
- 调用点：`generator/crates/instr/src/naming.rs` `mangle_if_overloaded`；定义侧：`emit/src/phase2/inherited.rs`、
  `emit/src/phase2/iface_impls.rs`、`emit/src/vtable.rs`
- 单测：`ty/src/sig_types/tests.rs`（`interface_overload_spans_superinterfaces`、
  `interface_view_names_consistent_and_disjoint`、`class_view_disjoint_for_interface_only_members`）

## 六、影响面

实测口径：dev 服务器作业，`scripts/gen_trees.sh`（`--stop-after emit`）生成验收集 27 例 + HelloWorld / CollectorsDemo /
DeepCopy / TestPropertiesXmlRoundTrip / TestSaxLocatorAttributes / TestXmlSaxEvents / TestXmlTransform，
`scripts/compare_trees.sh` 逐字节对照，再按 `diff` 中 `fn <名字>` 的净增减统计改名。

### 6.1 接口层次规则（3.1 / 3.2，1b3c26d1，作业 `ovl-impact-1b3c26d1`，基线 1afed3fc）

- 33 例中 14 例有 java.base 改动（各 ~541 行，DeepCopy 3270 行），其余 19 例只有 24 行行表 / 元数据位移；
  改动全在 `java_base_decl` / `java_base_body_*`（约 1000 个文件次），raw-audit 计数一致。
- 净改名 7 个方法名族，全部是「子接口自有方法与超接口同名异参」的 JDK 接口：
  `Spliterator.OfPrimitive.tryAdvance / forEachRemaining` → `_obj`；`Iterable / Map.forEach` 族 → `forEach_consumer` /
  `forEach_obj`；`Node.Builder` 族 `copyInto` → `copyInto_obj_i` / `copyInto_arr_{int,lng,dbl}_i`；
  `remove` → `remove_obj` / `remove_obj_obj`；`ChronoLocalDate / TemporalAccessor.isSupported` → `isSupported_temporalunit`；
  `BlockingQueue.poll` → `poll_l_timeunit`。
- java.xml：`ExtendedContentHandler.endElement(String)` → `endElement_str`，`ExtendedLexicalHandler.comment(String)` →
  `comment_str`，TestXmlTransform 的 E0061 ×2 / E0308 ×1 消失（`ovl-base-1fe9e5ea`：编译通过，进入运行期）。

### 6.2 类视图规则 5（本分支，作业 `ovl-impact3-dff8d8be-m28`，基线 1fe9e5ea）

- 验收集 27 例 + 抽查集 7 例（含 TestXmlTransform 的 java_xml crate）共 34 例生成树**逐字节一致**
  （`compare_trees` 全 0，净改名 0），raw-audit 一致。规则 5 只作用于「类不声明、类视图内同名异参」的名字，
  当前语料档案内无此形态——它补的是规则空隙（第三方抽象类 / 后续档案扩大时会出现），不改动现有任何名字。
  （首跑 `ovl-impact3-dff8d8be` 两树并行生成撞 14G 上限，以每槽 28G 重跑。）
- 中途教训（`ovl-impact2-e7dd4c2e`）：曾把 `interface_member_local_name` 简化为只查类视图集合，丢了「只由合成桥
  承载的擦除接口成员」（枚举 `compareTo(Object)`）的异参判定，全部枚举的桥成员 `compareTo_obj` 被继承成员认领吞掉
  （每例 3.4–5 万行差异）；已恢复超类链异参判定并补单测，对照归零。

## 七、遗留

- TestXmlTransform：重载命名已修（1b3c26d1 起编译通过），运行期失败于 `SerializerMessages` 资源束未装载
  （java.xml 模块资源包按名装载线，与 TestXmlSaxEvents / TestRowSetProvider 同根），`known_failures.toml` 条目已改签名。

- 字面量名与后缀名撞名（Java 方法字面量名恰为另一重载的后缀名，如同时有 `put(int)` 与 `put_i()`）：
  规则不处理，语料未见；终态由名字分配阶段在视图内检测并对字面量名追加声明者区分，另立项。
