# C1d 后闭包膨胀归因（HelloWorld 251 → 1982 类）

> 分支 c1d-prec（基于 c1d-final 4cb37dd2 合入后）。工具：`rava closure --cut / --cut-file` 反事实切除、`--dump-edges` 触发边转储（缺省关闭；`main.py` 同名选项透传，见 docs/environment-variables.md）。
> 全部数字为真实分析器运行结果（离线图模拟的结论见 §6，已证伪，不作依据）。

## 1. 基线（合入 4cb37dd2 后）

| 用例 | 类 | 方法 |
|---|---:|---:|
| HelloWorld | 1982 | 13299 |
| TestStreamBasic | 1997 | 13403 |
| Digester | 2656 | 17655 |
| CollectorsDemo | 1982 | 13299 |

**CollectorsDemo 与 HelloWorld 数字完全相同的核查**（协调者疑为跑错输入）：重测确认输入为 `tests/e2e/04_collections/CollectorsDemo.java`（入口 `CollectorsDemo.main`）。两份结果逐项对比：类集合只差用户类本身（`HelloWorld` ↔ `CollectorsDemo`），方法集合只差各自 4 个用户方法（HelloWorld：main / <init> / greet / repeat；CollectorsDemo：main + 3 个 lambda），**JDK 部分 1981 类 / 13295 方法逐一相同**。数字相同不是跑错，而是两个入口收敛到同一个 JDK 不动点——这本身就是「区域是吸引子」的直接证据（见 §4）。

JVM 实测对照（`-Xlog:class+load`）：HelloWorld 实际加载 556 类；闭包中 java/util/stream 162 类（JVM 加载 0）、java/util/regex 63（0）、java/text 35（0）、sun/util/locale/provider 40（0）、sun/nio/cs 105（5）。

## 2. 反事实：主会话怀疑的路径逐条实测（HelloWorld）

| 切除 | 类 | 方法 | Δ类 |
|---|---:|---:|---:|
| 无 | 1982 | 13299 | — |
| `Preconditions.outOfBoundsMessage`（charAt OOB → String.format） | 1982 | 13299 | 0 |
| `AccessController.executePrivileged@29`（action.run 派发，上界） | 1546 | 9858 | −436 |
| `ByteArray.<clinit>`（VarHandles / LambdaForm） | 1963 | 13097 | −19 |
| `ClassFileDumper.getInstance` | 1895 | 12924 | −87 |
| logger（`System.getLogger` / `LazyLoggers.getLogger` / `PlatformLogger.getLogger`） | 1893 | 12653 | −89 |
| `SharedSecrets.ensureClassInitialized` | 1982 | 13296 | 0 |
| `MethodHandles.lookup` | 1946 | 12529 | −36 |
| `ParameterizedTypeImpl.validateConstructorArguments` | 1981 | 13298 | −1 |
| **OOB 消息 + validateConstructorArguments 同时切** | **383** | **1477** | **−1599** |
| `String.format(String,Object[])` 单独 | 384 | 1482 | −1598 |
| `String.format@0`（仅 `new Formatter` 分配点） | 384 | 1481 | −1598 |

**结论一：HelloWorld 的膨胀有一个真实咽喉——`String.format`，入口只有两条：`Preconditions.outOfBoundsMessage` 与 `ParameterizedTypeImpl.validateConstructorArguments`（泛型实参个数不符时的异常消息）。单切任一条 ≈0，两条同切 −1599 类。**

## 3. Formatter 内部分解（切 PTI、只留 OOB 入口开放）

- 切掉全部 print* 分支（DateTime / Integer / Float / Character / Boolean / String / HashCode）及正则回退解析：仍 1966 类。膨胀不来自格式化本身。
- 切掉 Formatter 全部方法 → 384；逐个放回（leave-one-in），**只有两个方法各自单独就能打开整片区域**（各 1957 类）：
  - `Formatter.<clinit>`：`Pattern.compile(常量格式说明符正则)`；
  - `Formatter.<init>()`：`Locale.getDefault(Locale.Category.FORMAT)`。
- 两者单切均 0（互为替补），同切见 §7 的非单调现象。

`Formatter.<clinit>` 打开的区域按首次发现生成树（非支配关系，仅示路径）：
`Pattern.compile → Pattern.atom → family → String.toLowerCase(Locale) → StringLatin1.toLowerCaseEx → ConditionalSpecialCasing.isFinalCased → BreakIterator.getWordInstance → LocaleProviderAdapter.forType → Class.getDeclaredConstructor → methodToString → stream 管线…`

区域内较大的汇聚点（在「只开 clinit」状态下切除的实测）：

| 切除 | 类 | Δ |
|---|---:|---:|
| 无 | 1957 | — |
| `executePrivileged@29` | 1139 | −818 |
| `LocaleProviderAdapter.forType` | 1725 | −232 |
| `String.valueOf(Object)@11`（toString 派发） | 1904 | −53 |
| `ConditionalSpecialCasing.isFinalCased` | 1942 | −15 |
| `BreakIterator.getWordInstance` | 1943 | −14 |
| `Pattern.family` | 1956 | −1 |
| `Class.methodToString` / `Objects.equals@11` / `HashMap.hash@9` | 1957 | 0 |

区域没有内部支配核：进入区域的任何一个入口都能拉满整片。

## 4. 其余三个用例：咽喉不成立

| 用例 | 基线 | 切 String.format ×2 + formatted | 切 Formatter 全部方法 |
|---|---:|---:|---:|
| TestStreamBasic | 1997 / 13403 | 1996 / 13366 | 1973 / 13213 |
| Digester | 2656 / 17655 | 2656 / 17652 | 2632 / 17449 |
| CollectorsDemo | 1982 / 13299 | 1981 / 13262 | 1958 / 13109 |

用到 stream / 集合的程序经其他路径进入同一片 ~1600 类的 JDK 区域；HelloWorld 的咽喉对真实程序没有意义。

CollectorsDemo 重测后结论不变（「String.format 咽喉不迁移」对它成立：切 String.format −1 类，切 Formatter 全部方法 −24 类）。切掉 Formatter 后它仍从别处进入同一区域，例如 Pattern 的 Unicode 属性分支经
`SecurityProperties.<clinit> → includedInExceptions → String.split(String) → Pattern.compile(String) → Pattern.<init> → compile → expr → sequence → atom → family`
进入——又是一个**常量正则**（`String.split` 的分隔符来自字面量 `","`）。这说明 §5 的常量实参部分求值要覆盖 `Pattern.compile` 本身（所有常量正则来源：Formatter、String.split、各 `<clinit>` 常量），而不是只对 Formatter 做特判。更进一步：`String.split(String,int,boolean)` 字节码开头就是单字符快速路径判定（`length()==1 && ".$|()[{^?*+\\".indexOf(ch)==-1` → 跳到 @95 不经 Pattern）；实参为常量 `","` 时该分支条件可在常量上求值，`Pattern.compile` 调用点（@120 之后）根本不可达。即这一入口只需「常量字符串上的纯方法求值 + 分支折叠」，不需要正则解析器求值。

## 5. 主会话候选修复的判定

- **checkIndex 区间分析**：`outOfBoundsMessage` 的前驱是数百个 checkIndex / checkFromIndexSize / checkFromToIndex 调用点（String / StringBuilder / nio Buffer / zip / BigInteger / VarHandle 字节数组视图…），其中大量下标来自用户数据，OOB 是真实可达的 Java 语义（`"abc".charAt(5)` 就走这条）。即使全部证明也只关一条入口（§2：单切 = 0），另一入口 PTI 仍开。**收益 0，不实施。**
- **executePrivileged 上下文敏感**：@29 派发到的 81 个 action 逐个核对分配点（`CharacterName.<init>`、`Currency.initStatic`、`Charset$ExtendedProviderHolder.extendedProviders`、`ServiceLoader.loadProvider`、`ClassFileDumper` …），每个 action 都是在其分配方法里直接交给 doPrivileged 的；分配方法可达即 run 真实可达。executePrivileged 只是公共漏斗，上下文敏感不会剪掉任何一个 action。**可证明的收益 0，不实施。**（−436 是「砍掉全部特权动作」的不可靠上界。）
- **ByteArray / logger / ensureClassInitialized / MethodHandles.lookup**：真实边际 −19 / −89 / 0 / −36，且均为真实可达路径。

**结论二：C1d 后的增长主体是「按真实字节码展开 JDK」的应有结果（原先由手写边界截断）。在不牺牲正确性的前提下，可靠地剪掉这片区域需要内容敏感分析，而不是区间或调用上下文：**

1. 常量实参的解析器部分求值：`Pattern.compile(常量)` 证明不走 `family` / `CharPredicates` 的 Unicode 属性分支；`Formatter.parse(常量)` 证明只有 `%s` / `%d`。
2. locale 语言常量传播：`Locale.ROOT` 或确定 locale 的 `getLanguage()` 常量化，使 `lang == "tr"/"az"/"lt"` 分支（→ ConditionalSpecialCasing → BreakIterator → LocaleProviderAdapter）可判定。但默认 locale 来自 `user.language`，运行期可为 tr，这部分对 `toLowerCase()` 无参调用不可剪。
3. 即便 1、2 全部做到，§4 说明 stream / 集合用例仍经其他入口进入区域。终态目标应定为「每个入口单独可证」，逐入口计量，不能以 HelloWorld 的咽喉为验收。

本次**未提交任何精度改动**：候选修复经实测均无可靠收益，按「只剪可证明不可达」的约束不做近似剪枝。

## 6. 方法学教训

- **离线图模拟不可信**：此前基于触发边图的模拟（派发边带接收者分配条件）得出「7 个闸门需同时关闭」，与真实运行不符（真实是 String.format 的 2 个入口）。原因：图模拟不含值流（字段 / 返回值集合、常量折叠随可达集变化），派发条件只看类型是否分配，不看值是否流到该接收者。归因必须用真实分析器反事实。
- **诊断开关分隔符缺陷**：初版（环境变量形态，已改为 CLI）以 `;` 分隔条目，与描述符里的 `;` 冲突，含引用类型描述符的条目全部失效。此前「各路径单切均为 0」的数据作废，本文数据均为修正后重测；CLI 形态每个 `--cut` 一条，不再有分隔符。

## 7. 非单调现象与「空集当恒 null」

反事实切除会改变值集，结果不单调（切得更多，闭包反而更大）：

| 切除 | 类 | Δ | 机制 |
|---|---:|---:|---|
| `String.valueOf(Object)@11` | 2655 | +673 | 切除改变了返回值集；新增 JarVerifier / PKCS7 / PolicyFile / URLConnection 等。失去的折叠包括 `Policy.isSet` 的 `getfield PolicyInfo.policy = null`、`URL.openConnection` / `toExternalForm` 的 handler = null、`URI.isAbsolute` 的 scheme = null、`ArrayDeque.head = 0`、`SharedSecrets.javaUtilJarAccess = null` |
| `Formatter.<clinit>` + `Formatter.<init>()` | 2736 | +754 | 构造器被切后 `Formatter.a` / `l` 无写入，按初值折叠为恒 null，null 处理分支转活 |

- `Policy.isSet` 形成自支撑环：`ProtectionDomain.toString@147` 转活 → `mergePermissions` → `getPolicyNoCheck` → 写入非 null 的 `PolicyInfo.policy` → `isSet` 不再折叠 → @147 活。基线停在最小不动点（折叠成立），切除后的运行停在更大的不动点。
- 基线中上述折叠经核对是一致的：URL / URI 在基线中未实例化；`ArrayDeque.head` 的写入方法不可达；`PolicyInfo` 只由 `Policy.<clinit>` 以 null 构造。**本次实验未发现基线里的 null_recv 误折叠**；「空集当恒 null」只在人为制造不完整建模（切除构造器 / 写入点）时显现。这正说明凡建模不全处（未建模的写入：VarHandle / Unsafe CAS、手写 native 写字段）都会按同一机制误剪，与协调者的提醒一致。字段写入的建模完整性应作为独立审计项。
- 对归因工具的要求：只切「消费型」节点（方法体、派发点）；切构造器或写入点会引入空集折叠，结果不可用。

## 8. 提交

- 9c3610ac 闭包分析器诊断开关（初版为环境变量，违反「不设自有环境变量」规范）
- 随后改为 CLI：`rava closure/build --cut / --cut-file / --dump-edges`，`main.py` 透传

缺省关闭，闭包结果与基线逐数一致（四个用例的类数 / 方法数不变）；分析器 `cargo test` 通过。

## 9. 需主会话 e2e 验证

（§8 的诊断开关）无闭包语义改动。诊断开关缺省关闭，理论上无需 e2e；若要回归，跑验收四例 HelloWorld / TestStreamBasic / Digester / CollectorsDemo 即可。

## 10. 字段写入来源审计（null_recv / 字段折叠的正确性前提）

字段读折叠（含「空集当恒 null」的 null_recv）成立的前提是**每个写入来源都已建模**。逐类列出写入来源与建模方式：

| 写入来源 | 建模 | 状态 |
|---|---|---|
| 可达字节码 putfield / putstatic | `field_put` 并入值集 `fvals`；static final 取 `<clinit>` 唯一赋值 / ConstantValue | 完整 |
| 共置手写（`<类>_impl.rs` / `_ext.rs`）`__set_<字段>` | 按手写方法可达性扫描（同文件传递闭包）→ `open_field` | 完整 |
| VM 基础设施手写文件（monitor.rs 等非共置文件）的写访问器 | **缺口，已补**：`handwritten/vm_writes.rs` 扫描 `__set_<名>` / `T::set_<名>(`，按名放开（基础设施恒可达）。现有两处：`Thread.interrupted`、`Thread$FieldHolder.threadStatus` | 已闭合 |
| VM 边界类（closure.toml `[vm_boundary]`）/ 根域字段 | `fi.open` 恒不折叠 | 完整 |
| 手写 fn 名与字段同名（VM 注入值的访问器） | `fi.open` | 完整 |
| 按名反射写入，名字为常量（objectFieldOffset / findVarHandle / newUpdater / getDeclaredField） | 字节码形状规则（Class 实参 + 字符串常量） | 完整 |
| 按名反射写入，名字**非常量** | **缺口，已补**：清单 `[facts.field_writes.name_resolvers]` 登记 10 个按名取字段身份的入口；名字取自形参时取各调用点常量（`pstrs`），形参槽出现过非常量实参或无调用点记录即进入时（`ptaint`）保守回退——类字面量放开该类及超类全部字段，推不出类则全部不折叠；返回 Field 句柄的入口按字段枚举处理（句柄写入口可达才放开） | 已闭合 |
| 字段句柄枚举 + Field.set* / unreflectSetter / unreflectVarHandle / sun.misc.Unsafe 偏移 | `enumerators` × `handle_writers`（`handle_bridges` 除外） | 完整 |
| Unsafe put/CAS、VarHandle set/CAS（引用） | `[facts.array_writes] fields = true`（类型流）；字段身份来自上两行的偏移 / 句柄取得点 | 完整（身份侧依赖上两行） |
| 反序列化 | `deserializers` 可达 → 非 static 非 transient 全部放开 | 完整 |
| 按类镜像强制初始化（`Unsafe.ensureClassInitialized`）运行的 `<clinit>` 写入 | **缺口，已补**：清单 `[facts.reflect] class_initializers`，Class 实参（类字面量 / 值集镜像）所指类按 JVMS §5.5 初始化；推不出所指类的记反射缺口 | 已闭合（运行时配套见下） |
| VM 在 `<clinit>` 之后改写的 static final（`UnsafeConstants.*`、`System.in`） | 分析器按 `<clinit>` 值折叠；运行时同样未注入 | **运行时缺口**（见下） |

实测（`--flows @foldfields` 列出全部折叠字段并按来历分 clinit / writes / initial；`--flows @bynamesites` 列出名字非常量的按名调用点）：

- 四例中名字非常量、且落到按名取字段写入能力的站点为 0（非常量名的站点集中在 MemberName / findStatic / ServiceLoader.fail 等方法名或只读 getter）；按名解析入口改动后四例折叠字段与闭包逐数不变。
- 按镜像初始化建模后 `SharedSecrets.javaUtilJarAccess`（HelloWorld / TestStreamBasic / CollectorsDemo）、`javaIOAccess` / `javaLangModuleAccess`（Digester）不再按 null 折叠——这三处是基线里真实的误折叠（HotSpot 下 `ensureClassInitialized(JarFile.class)` 后字段非 null，调用方经它取访问器）。

| 用例 | 类（前 → 后） | 方法（前 → 后） | 折叠字段（前 → 后） |
|---|---:|---:|---:|
| HelloWorld | 1982 → 1987 | 13299 → 13338 | 494 → 493 |
| TestStreamBasic | 1997 → 2002 | 13403 → 13442 | 494 → 493 |
| CollectorsDemo | 1982 → 1987 | 13299 → 13338 | 494 → 493 |
| Digester | 2656 → 2666 | 17655 → 17720 | 802 → 800 |

新增类即被初始化类 `<clinit>` 的真实依赖（HelloWorld：`JavaUtilJarAccessImpl`、`Runtime$Version*`、`ZipFile$1`、`JavaUtilZipFileAccess`）。

运行时配套（需 e2e）：

1. `Unsafe.ensureClassInitialized` 原为 no-op（按惰性协议推迟到首次主动使用），丢失 `<clinit>` 副作用；改为按名 `ensure_class_initialized`。
2. 钩子只为用户类与注解枚举登记；闭包导出 `seeds.mirror_inits`（按镜像初始化的目标类），发射层（emit `class_init_hooks`）一并登记。
3. 仍属运行时缺口、未在本步改动：`UnsafeConstants` 的 VM 注入值（BIG_ENDIAN / PAGE_SIZE / UNALIGNED_ACCESS 折叠为 0，TestDirectBuffer 已知失败同源）、`System.in`（运行时无 `in` 访问器，HotSpot 由 initPhase1 / setIn0 设置）、`Field/Method/Constructor.signature`（class_impl.rs 从不设置，泛型反射信息丢失）。终态是在运行时补齐这些 VM 注入值；补齐后以字段同名手写访问器出现，分析器自动 `fi.open`，无需分析器改动。

值集精度注记：DirectMethodHandle.checkInitialized / shouldBeInitialized、VarHandles.makeFieldHandle 的 Class 实参值集含推不出所指类的镜像，记为反射缺口（`reflect.gaps`）；Digester 的 `mirror_inits` 达 559 类（值集混入全部 getClass 镜像），只影响钩子表长度，不影响闭包。

## 11. 成员声明类初始化点与钩子表收窄（§10 值集精度注记的终态处理）

### 11.1 两类初始化点

`Unsafe.ensureClassInitialized` 的 JDK 调用方按 Class 实参来源分两类：

- **成员声明类初始化点**：实参恒为「正被链接 / 访问的成员的声明类」。清单 `[facts.reflect]` 按链接路径登记：
  - `handle_owner_initializers`：`DirectMethodHandle.checkInitialized@9` / `shouldBeInitialized@104`、`VarHandles.makeFieldHandle@442`
  - `reflect_owner_initializers`：`MethodHandleAccessorFactory.ensureClassInitialized`、`UnsafeFieldAccessorFactory.newFieldAccessor@59`
- **一般调用点**：类字面量或形参（`SharedSecrets` → `Lookup.ensureInitialized`、各 `Holder` 的 `<clinit>` 等），仍按 §10 的值集求目标。

### 11.2 成员声明类初始化点：分析上由结构不变量 S 覆盖，不按值集求目标

值集是 `MemberName.clazz` 一类跨全部成员的汇合，按它求目标会把所有 getClass 镜像都拉进来（§10 的 3 条缺口与 559 类钩子表即由此而来）。改为依赖不变量 S：

- **S1**：可达的静态方法（`<clinit>` 除外）或构造器 ⇒ 声明类初始化（JVMS §5.5 invokestatic / new）。在 `method_ctx` 按边强制执行，与进入路径无关（普通调用、反射、句柄一视同仁）。
- **S2**：反射暴露的方法 ⇒ 声明类初始化（reflect.rs 的 expose，已有）。
- **S3**：按反射 / 句柄取得的静态字段 ⇒ 声明类初始化。覆盖三条路径：
  - 按名入口：解析到的静态字段初始化其声明类。Class 实参不是字面量时，按值集所指类解析；值集不齐全时按名兜底，即闭包中声明该名静态字段的类都初始化，补种阶段逐轮扫描新增类。
  - ldc 静态字段句柄（kind 2 / 4）。
  - 字段枚举：接收者所指类及其超类中声明非常量静态字段的类。

**Soundness 论证**：成员声明类初始化点在运行期传入的类 C，必定是某个静态方法、构造器或静态字段 m 的声明类，并且 m 此刻正被链接或访问。

- m 是方法或构造器：m 在闭包内可达，由 S1 / S2 可知 C 已初始化。
- m 是静态字段：m 必经按名入口、ldc 句柄或枚举取得，由 S3 可知 C 已初始化。
- 按名入口所属类推不出时，走兜底的按名扫描。运行期镜像只能指向闭包里的类，因此闭包中声明该名静态字段的类全部被初始化，覆盖所有可能的 C。

**实测**：S1 在 4 例上单独加入时闭包 0 变化，说明不变量本已成立，现在只是显式化。

### 11.3 运行期钩子表：只登记确有初始化点可达的类

判定条件：类 C 登记为钩子，当且仅当以下任一成立：

- C 是一般调用点值集或字面量的目标；
- C 经链接路径 R 链接（R 的取值：
  - `method-handle` 边 → handle；
  - `reflect` 边 → reflect；
  - 按名取字段 → 两路都记），且 R 的成员声明类初始化点可达。

钩子只决定 `<clinit>` 在初始化点**提前**执行。即使不登记，`java_class!` 注入的 `__class_init` 也会在成员实际访问时触发初始化：

- 静态方法 / 构造器入口；
- `NAME()` / `set_NAME` 访问器；
- 反射字段闭包的静态臂，以及 Unsafe `_static_ref_get/set` 走的正是这一路。

因此收窄不会丢失任何 `<clinit>`，只影响「链接时初始化」与「首次访问时初始化」之间的时序差。

### 11.4 前后数字（mi = 504fa6f8，mo = 本步）

| 用例 | 类 | 方法 | 钩子表 mirror_inits | 反射缺口（本步后） |
|---|---:|---:|---:|---:|
| HelloWorld | 1987 → 1987 | 13338 → 13338 | 78 → 17 | 4（同基线） |
| TestStreamBasic | 2002 → 2002 | 13442 → 13442 | 78 → 17 | 4（同基线） |
| CollectorsDemo | 1987 → 1987 | 13338 → 13338 | 78 → 17 | 4（同基线） |
| Digester | 2666 → 2662 | 17720 → 17691 | 559 → 150 | 4（同基线） |

- **反射缺口**：剩余 4 条（`getDeclaredConstructors0` / `getDeclaredMethods0` 的 Class 值集）与 mirror-init 之前的基线逐条相同。`ensureClassInitialized` 相关缺口已归零。
- **Digester 删掉的 16 个 `<clinit>`**：全部是只经 checkInitialized / shouldBeInitialized / MHAF 值集混入的目标，经 S 判定不可达：
  - Runnable、MemorySegment、SegmentAllocator、CallSite；
  - Policy$Parameters、SecureRandomParameters、CertStoreParameters、Configuration$Parameters；
  - AbstractMemorySegmentImpl、GlobalSession、MemorySessionImpl、NativeMemorySegmentImpl、ScopedAccessError；
  - PKCS12KeyStore、JavaKeyStore、JavaKeyStore$JKS。

  Digester 连带去掉 4 个类、29 个方法。
- **Digester 新增 2 个 `<clinit>`**：CertificateIssuerExtension、SubjectAlternativeNameExtension。来源是 S3 的按名兜底：`UnparseableExtension.<init>@26` 在 `OIDMap.getClass` 所得镜像上调用 `getDeclaredField("NAME")`。这 2 个是基线漏掉的真实依赖。
- **HelloWorld 钩子表 17 类**：FileDescriptor、FilePermission、PrintStream、PrintWriter、Thread$ThreadNumbering、BoundMethodHandle、DMH / DelegatingMethodHandle / Invokers / LambdaForm 的 Holder、VarHandleGuards、URL、Buffer、ResourceBundle、ForkJoinPool、JarFile、ZipFile。
- **Digester 钩子表 150 类**：因为 reflect 路径可达（MHAF 在链上），reflect 边链接的声明类（sun/nio/cs 字符集、locale 适配器、j.l.invoke 的反射暴露）按条件登记。
- **动态对照**：用 504fa6f8 的生成树，并配合 f66c4ad8 修正后的基准 JVM 配置跑 dyn_compare，结果为：
  - TestStreamBasic 漏覆盖 0；
  - HelloWorld 漏覆盖 0；
  - CollectorsDemo 只剩 LambdaMetafactory（prec3 负责）。

### 11.5 已知未覆盖（既有，非本步引入）

有一种情况 S3 不成立：按名取字段的入口处，名字非常量（`ptaint`）且 Class 实参也推不出。此时字段按名全部放开，但不初始化任何声明类。若出现，终态做法是按名兜底扩展为「闭包中全部含非常量静态字段的类」。

## 12. 合入 rust-closure-analyzer（ef1daf34）

冲突以主干形态为准，C1d 终态语义保留：

- **手写删除保留**：C1d 删掉的 5 个过渡手写文件（java_lang_access / arrays_support / byte_array / stream_decoder / stream_encoder 的 `_impl.rs`）继续删除，不接收主干对它们的机械改动。
- **`input::boundary` 取 C1d 终态**：只判 VM 契约类，不带类路径与资源束判定。
- **`api_roots` 不再跳过「边界域包」**：终态已无包前缀截断。
- **`dyn_compare` 直读 TOML，不 import codegen**：
  - 域规则取 `[vm_boundary] classes / translate_nested`；
  - 基准 JVM 的 `-D` 取自 `vm_intrinsics.toml [facts.system_properties.values]`；
  - `codegen/runtime_manifest.system_property_values` 已撤回。
- **形参字符串常量改用主干的单调集 `pstrs.rs`**，按名取字段的污染判定（`ptaint`）保留：
  - 污染槽的读者站点从 `pstr_readers` 取；
  - 主干的 `Src::Str` 现在带字面量编号，合流的字面量可逐个取回，所以按名取字段把它们计入名字全集，不再判为不可知。
- **驱动层**：闭包诊断选项（`--cut` / `--cut-file` / `--dump-edges`）与主干新增选项并存；`main.py` 的 rust 路径把诊断参数转交 `rava build`。
- **合并后的机械修复**：`method::vars::slot_type` 的 null → `Default::default()` 改走计数构造器 `Expr::raw`；`class_writer` 去掉手写覆盖副本（C1d 已删 `hw_overrides`）。

### 12.1 4 例闭包对照（合入前 b671d95f → 合入后）

| 用例 | 类 | 方法 | 类初始化 | mirror_inits | 反射缺口 |
|---|---|---|---|---|---|
| HelloWorld | 不变 | +5 | 不变 | 17 → 17 | 不变 |
| TestStreamBasic | 不变 | +5 | 不变 | 17 → 17 | 不变 |
| CollectorsDemo | 不变 | +5 | 不变 | 17 → 17 | 不变 |
| Digester | 不变 | +4 | 不变 | 150 → 150 | 不变 |

新增方法均为 `MethodHandle.linkToVirtual / linkToStatic / linkToSpecial / linkToInterface`；前三例另加 `MethodHandle.invoke`。这正是主干 8162d43e 列出的变化：

- `DirectMethodHandle.makePreparedLambdaForm@313` 的链接器名经合流取回；
- `MethodHandle.invoke` 由 `makeExactOrGeneralInvoker` 的调用方引入。

合并本身没有引入其它集合变化。

## 13. 内容感知精度（§5 结论二的四项）：设计前实测

§5 列了四项内容感知精度候选：

1. `Pattern.compile(常量正则)` 证明不走 Unicode 属性分支；
2. `Formatter.parse(常量格式串)` 证明只出现部分转换符；
3. `String.split` 单字符快速路径在常量实参上折叠；
4. locale 语言常量化，判定 `tr / az / lt` 分支。

动手前先量**上界**：假定某项做到完美，它最多能剪掉什么。方法是用 `--cut` 把该项能剪掉的消费点全部切掉。这样得到的是不可靠上界：它假定所有来源都是常量、都走窄分支。基线为合入后的 8bfeb2c5（§12.1）。

### 13.1 切除集合（每项所能剪掉的全部消费点）

| 项 | 切除 |
|---|---|
| 正则 | `Pattern.family`（方法体；`\p` / `\P` / 属性名分支的唯一入口，调用方为 atom / sequence / range） |
| split | `String.split(String,int,boolean)@121 / @134 / @144`（快速路径以外的 `Pattern.compile` 与 `Pattern.split*`） |
| locale | `StringLatin1.toLowerCase / toUpperCase @95`、`StringUTF16.toLowerCase / toUpperCase @132`（`lang == "tr"/"az"/"lt"` 为真时进入 `*Ex(…, true)` 的调用点） |
| 格式串 | `FormatSpecifier.print(Formatter,Object,Locale)` 的 `@11 / @146 / @156 / @166 / @186`（DateTime / Float / Character / Boolean / HashCode；只留 `%s` / `%d`），以及 `Formatter.parse` 的正则回退 `@141 / @174` |

### 13.2 上界实测（类 / 方法）

| 用例 | 基线 | 正则 | split | locale | 格式串 | 四项全切 |
|---|---:|---:|---:|---:|---:|---:|
| HelloWorld | 1987 / 13343 | 1986 / 13185 | 1987 / 13341 | 1987 / 13340 | 1975 / 13217 | 1974 / 13053 |
| TestStreamBasic | 2002 / 13447 | | | | | 正则 + split + locale：2001 / 13284 |
| CollectorsDemo | 1987 / 13343 | 1986 / 13185 | | | | 正则 + split + locale：1986 / 13180 |
| Digester | 2662 / 17695 | | | | | 正则 + split + locale：2661 / 17536；四项：2649 / 17410 |

即使四项全部做到完美，也只能剪掉 **13 类（0.65%）**。split 与 locale 两项连一个类都剪不掉，只少 2 / 3 个方法。原因与 §2–§4 相同：这片区域有许多独立入口，关掉其中一两个不起作用。

### 13.3 可靠收益：各项被真实可达的动态输入挡住

上界只是天花板。按「只剪可证明不可达」的要求，还要看每项能不能在 4 例上**可靠地**成立。下列方法 4 例的闭包里都有（已逐一核对）：

- **正则：可靠收益 0。**
  - `CompactNumberFormat.parseNumberPart` 在运行期拼出 `"[\\Q" + 小数分隔符 + "\\E\\p{Nd}]+"` 再 `Pattern.compile`。这是真实的 `\p{Nd}`，`family` 真实可达。
  - 同时这个正则不是常量（中间拼进了 locale 数据），常量实参求值对它不适用。
- **格式串：可靠收益 0。**
  - `SimpleConsoleLogger.format` 的格式串来自 `getSimpleFormatString()`：可由系统属性覆盖，缺省值本身就含 `%1$tb` 等 DateTime 转换。
  - `PrintStream.implFormat`（printf）、`String.formatted`、`AbstractLoggerWrapper.logrb` 的格式串都来自调用方数据。
  - 这些都流进同一个 `Formatter.parse`，而 `FormatSpecifier` 的字段值集是全局共享的。所以即使按调用点特化，也关不掉 DateTime / Float / 各类 Illegal*Exception 分支。
  - 表中「格式串」一列的 −12 类，主体是 `Formatter$DateTime`、`ChronoZonedDateTime$1` 和 Illegal*Exception 族，正好被上面这些动态格式串挡住。
- **locale：上界本身只有 −3 个方法。**
  - `StringUTF16.toLowerCaseEx` 遇到 `Σ` / `İ` 时，无论 locale 如何都进入 `ConditionalSpecialCasing`。
  - `StringLatin1.toUpperCaseEx` 遇到 ERROR 映射字符（如 `ß`）时，走 `toUpperCaseCharArray`。
  - 这两条入口取决于字符串内容，与 locale 无关。
  - `Formatter.<init>()` 的 `Locale.getDefault(FORMAT)` 来自 `user.language` 等运行期属性，不是常量（§5）。
- **split：上界只有 −2 个方法，0 个类。**
  - 要做到它，需要在 absint 里对常量字符串求值 `length / charAt / indexOf`，并把 `Pattern.compile` 调用点按实参特化。
  - 但 `Pattern.compile` 已经从别的调用点（上面的 `parseNumberPart` 等）真实可达。

### 13.4 决定

四项都**不实施**。能可靠剪掉的最多是 split + locale 的 5 个方法、0 个类，却要给分析器加上常量字符串求值、调用点特化，外加两个 JDK 解析器的部分求值器。这些机制复杂度高、正确性风险大，而且在 4 例上换不来一个类。

终态目标按本节实测改定：对验收 4 例，内容感知精度可剪掉的类数为 **0**。与 JVM 实测加载数（HelloWorld 556 类）之间的差距，属于「真实可达但本次运行没有执行」的 JDK 路径。例如：

- `CompactNumberFormat` 的数字解析；
- 缺省格式串里的 DateTime 格式化；
- 希腊字母的大小写转换。

这类路径不是「不可达」，按「只剪可证明不可达」的约束，闭包分析不应该剪。若要缩小生成物，应走另一条独立路线，在闭包之外处理，例如按使用剖面裁剪、或延迟翻译。这些不属于闭包精度的范围。

### 13.5 论证：为什么在 soundness 约束下剪不掉；终态的替代路径

**闭包的语义契约。** 闭包是「该程序在**任意输入、任意宿主环境**下可能执行的方法」的上近似。闭包外的方法发射为 `panic!("stub: …")`。所以剪掉一个方法，等于断言「对所有运行都不执行它」。断言错一次，就是一次运行期存根命中，与 TestStreamBasic 的 `StreamOpFlag$Type.values` 同类（§14.2）：合法程序、合法输入，二进制却崩溃。这是正确性回归，不是性能取舍。

**四项的分支条件都是运行期数据，不是程序常量。** 决定走哪条分支的量分四类：

| 项 | 决定分支的量 | 为什么不是常量 |
|---|---|---|
| 正则 `family` | 正则串内容 | `CompactNumberFormat.parseNumberPart` 用 locale 数据（小数分隔符）拼出正则，本身就含 `\p{Nd}`；即使别处都是常量，这一处也使 `family` 真实可达 |
| 格式串转换符 | 格式串内容 | `SimpleConsoleLogger` 的格式串可由系统属性 / 环境改写，缺省值就含 `%1$tb`；`printf` / `formatted` 的格式串来自调用方数据；全部汇入同一个 `Formatter.parse` |
| locale 分支 | `user.language` 与字符串内容 | locale 取自宿主环境（§5）；`Σ` / `İ` / `ß` 等字符的特殊大小写路径与 locale 无关，只取决于被转换的字符串，而字符串来自输入 |
| split 快速路径 | 分隔串内容 | 快速路径以外的 `Pattern.compile` 已从其他调用点真实可达，剪掉 split 的回退也剪不掉任何类 |

每一类都有**具体反例输入**：程序处理含 `ß` 的用户字符串、宿主 `LANG=tr_TR`、设置 `java.util.logging.SimpleFormatter.format`、格式化一个 `CompactNumberFormat` 解析的数字。只要存在一个反例，该分支就不是「不可达」，只是「本次测试没执行」。内容感知分析能证明的，只是**常量实参**所在调用点的窄分支；而 4 例闭包里，这些窄分支的类同时被上表的动态入口拉入。所以可靠收益是方法级个位数、类级 0（§13.2 / §13.3 实测）。

**与 JVM 实测加载数的差距不是精度缺口。** HelloWorld 闭包 1987 类，JVM 实测加载 556 类。差距由两部分组成：

- 真实可达、本次未执行的路径（上表各项、异常与错误路径、宿主相关分支）：soundness 下必须保留；
- 分析不精确造成的伪可达：这是闭包精度要消的部分，已由 §6–§12 的死分支折叠、常量传播、逃逸判定、反射面收窄处理。剩余的伪可达按 dyn_compare 的 miss/extra 继续逐项核对，不以内容感知的名义处理。

**终态的替代路径（在闭包之外缩小生成物或其代价）：**

1. **构建期声明的封闭世界事实**：把「运行期数据」在构建期显式固定下来，作为 VM 事实进入分析。形态与现有 `[facts.system_properties.values]` 相同：由构建配置声明 locale、日志格式串等，分析器按常量折叠。被剪掉的分支不发射存根，而是发射「违反构建声明」的明确失败，报出声明项。契约由用户显式选择，缺省不启用，所以缺省二进制的 soundness 不变。
2. **编译代价与闭包规模解耦**：闭包大主要贵在 rustc 的时间与峰值内存。按 4bae5479 的实测（依赖图 95% 同环），把声明层与方法体层拆成多个 crate，让编译峰值按 crate 分摊，而不是随闭包线性增长。目标：验收 4 例的 `java_runtime` 编译峰值内存与闭包类数脱钩。
3. **冷路径改为共享实现**：真实可达但冷的 JDK 路径（DateTime 格式化、特殊大小写、Unicode 属性类）跨测试相同。把 JDK 翻译产物做成按 JDK 版本缓存的预编译库（per-test scratch 只编用户类与少量热路径），那么这些路径只编译一次，不再计入每个测试的构建代价。

三条都不改变「闭包 = 所有可能执行的方法」这一契约。内容感知精度在这一契约下的终态收益定为：类 **0**、方法 ≤ 5（split + locale），不实施。

## 14. C1d 删除后的缺口修复与 rust-closure-analyzer（b1ac5b7c 系）合并取舍

### 14.1 修复提交

| 提交 | 缺口 | 处理 |
|---|---|---|
| c3e8e21f | Digester E0433（`URLConnection` 未在导入作用域） | Rust 生成器引用收集：常量池类是子类、static 成员实际声明在父类时，补声明类的导入 |
| 7ed10818 | TestStreamBasic 运行期命中 `StreamOpFlag$Type.values` 存根（§14.2） | 按名查方法的常量名另对 Class 接收者值集里的类镜像点名 |
| 3e65aa3b | System.in 为 null | `initPhase1` 语义由手写访问器给出（`BufferedInputStream(FileInputStream(FileDescriptor.in))`，`setIn0` 改写同一槽） |
| 34d5989a | TestDirectBuffer（direct buffer 内存上限为 0）、TestMethodHandleCombinators（Unsafe access out of array bounds） | VM 注入常量清单驱动（§14.4）+ `VM.<clinit>` 按字节码翻译 |
| f8203ba3 | Field / Method / Constructor 的 `signature` 为 null（泛型反射拿不到类型参数） | build.rs 元数据表携带 Signature 属性，反射工厂写入 `signature`（无属性为 null，同 HotSpot `Reflection::new_*`） |

### 14.2 `StreamOpFlag$Type.values` 存根命中：类粒度动态对照的盲区

- 路径：`StreamOpFlag` 的 `EnumMap(StreamOpFlag.Type.class)` → `Class.getEnumConstantsShared` → `this.getMethod("values")` → 反射调用。
- 闭包原先只把常量方法名 `values` 与**已知成员枚举面**配对，未对 `getMethod` 接收者（Class 值集里的类镜像 `StreamOpFlag$Type`）点名，`values()` 不入反射面 → 发射为存根 → 合法程序运行即 panic。
- 类粒度 dyn_compare 报 miss 0：`StreamOpFlag$Type` 这个**类**早已在闭包内（枚举常量经 `<clinit>` 可达），缺的只是该类上的一个**方法**。这正是精度三期 17fc2e7e 补方法粒度对照（`--methods`，`mmiss`）的原因；本次合并后 `mmiss` 口径随 prec3 一并进入 `c1d-prec`。
- 修复（7ed10818）：按名查方法的常量名对接收者镜像值集逐类点名入反射面；接收者所指未知记为反射缺口（`reflect.gaps`），不静默丢弃。

### 14.3 `Unsafe.<clinit>` 不翻译：分析器爆炸的诊断与终态修法

- 试验：把 `jdk/internal/misc/Unsafe` 也列入 `translate_clinit`（让 `ARRAY_*` 常量由字节码 `arrayBaseOffset(Class)` 计算）。分析器在 TestDirectBuffer 上跑不完（超时）。
- 机制：`Unsafe.<clinit>` 构造 `theUnsafe` 实例并写入静态字段，Unsafe 实例从此进入全局逃逸集 G；Unsafe 的 `getReference` / `compareAndSet*` 等方法返回 / 写入 open 值，open 值按「全部逃逸对象」展开，而 G 随之扩大 → 自反馈，逐轮扩大到几乎全部 JDK。
- 处理：`Unsafe.<clinit>` 不翻译；它写入的布局常量是 VM 注入状态（HotSpot 由 `arrayBaseOffset0` 等 VM 原语给出），按 §14.4 清单驱动，不需要 `<clinit>`。
- 终态修法（未做，记录在此）：open 展开改按「经 open 汇合点实际逃逸的对象集」而非全局 G——Unsafe 实例本身不应作为 open 值的候选来源。做完后 `Unsafe.<clinit>` 可以翻译，§14.4 清单里 Unsafe 的 19 项也可改回字节码计算（只留 VM 原语 `arrayBaseOffset0` / `arrayIndexScale0` 为 native）。

### 14.4 VM 注入常量（手写边界类 ③）：清单驱动

- 清单：`vm_intrinsics.toml [vm_constants.injected_statics]`，`"类.字段" = "crate:: 下取值表达式"`，24 项（UnsafeConstants 5 项、`Unsafe.ADDRESS_SIZE`、9 个 `ARRAY_*_BASE_OFFSET`、9 个 `ARRAY_*_INDEX_SCALE`）。
- 取值：`runtime/java_runtime/src/vm_constants.rs`（只给取值；字段集合与类型适配在清单与生成器）。
- 生成器：登记字段发为读取值表达式的访问器（按字段描述符做宽度转换），setter 不落存储（同 HotSpot `UnsafeConstantsFixup` 在 `<clinit>` 后改写）。
- 分析器：登记字段视为开放，不按 ConstantValue / `<clinit>` 初值折叠（否则 `BIG_ENDIAN = false`、`PAGE_SIZE = 0` 会被折进分支）。
- 生成器与分析器代码里无类名：全部来自清单。

### 14.5 与 dc9fd946 / d8a0b082 / 7cddf808 的对照（合并 b1ac5b7c 系到 `c1d-prec`）

| 项 | 主干（另一会话） | `c1d-prec` 取舍 | 结论 |
|---|---|---|---|
| UnsafeConstants 平台常量 | 手写 `unsafe_constants_impl.rs` 访问器 | 删除；清单 `[vm_constants.injected_statics]` + `vm_constants.rs`（§14.4） | 已覆盖（终态形式） |
| Unsafe 数组布局常量 | 未处理（`core_ARRAY_*` 过渡常量仍在） | 同清单 18 项，删 `core_ARRAY_*` / 手写 `isBigEndian` | `c1d-prec` 额外覆盖 |
| `VM.directMemory` / `pageAlignDirectMemory` | 手写 `vm_impl.rs` | 删除；`VM.<clinit>` 翻译（`translate_clinit`），`System.initPhase1` 调字节码 `VM.saveProperties` | 已覆盖（字节码） |
| `SharedSecrets.ensureClassInitialized`（GZIPTest） | 手写 `shared_secrets_impl.rs` | 删除；字节码 → `Unsafe.ensureClassInitialized`（钩子表按 `mirror_inits`） | 已覆盖（字节码） |
| JCA 别名 | 改 `jca.rs` 注册表 | `jca.rs` 已随 C1d 删除，JCA 服务按字节码（`Provider` 表） | 已覆盖（字节码） |
| System.in | `system_impl.rs` 一份 | 保留 `c1d-prec` 一份（3e65aa3b），主干那份丢弃 | 只留一份 |
| `user.name` | `posix::passwd_name` / `current_user_name` | 吸收（`System` 属性与 `ProcessHandleImpl$Info` 共用） | 吸收 |
| `Runtime.maxMemory` | `posix::default_max_heap` | 吸收（VM 堆上限，类 ③） | 吸收 |
| `OperatingSystem` / `URLUtil` / 旧式 Cleaner 放行 | `closure.toml [release]` | 无 `[boundary]` 前缀即无放行需要，`[release]` 不带回 | 不需要 |
| 宏：`<clinit>` 前登记常量目录 | `rava_macros` `class_init.rs` | 自动合入 | 吸收 |
| `FileInputStream.available0`（进程管道 FIONREAD） | 手写 native | 保留（`ACC_NATIVE`，类 ①） | 吸收 |
| `Unsafe.fullFence` | 手写 native | 保留（`ACC_NATIVE`，类 ①） | 吸收 |
| ScopedMemoryAccess 对齐访问族 | 手写 68 行 | 删除（有字节码，性能 / 便利替换不属准入；只留 `registerNatives`） | 改为字节码 |
| `BootLoader.loadClass(Module,String)` | 手写 | 删除（字节码足够） | 改为字节码 |
| 引导服务目录 | `services_catalog_impl.rs` | 迁入 `boot_loader_impl.rs` 的 `boot_catalog()`（引导层模块服务登记是 VM / 启动器状态，类 ③），按分析器服务事实装填 | 保留（类 ③） |
| `StaticProperty` / `JavaLangAccess` 手写 | 主干仍有 | C1d 已删，不带回 | 不带回 |
| 7cddf808 Ident Unicode 校验 | ir | 自动合入 | 吸收 |

合并中另外按终态处理的分析器项（精度三期 d8212bee 带入）：

- `boundary_cut` / `core_` 转发 / provider line / 纯数据束载体：终态无包前缀截断与 `core_` 转发器，相关代码与 `cut` 输出（含 dyn_compare 的 `cut` / `caller_cut` 标记）删除。
- 类初始化事实两套并存：主干 `[facts.class_init.initializers]`（输出 closure.json `class_init`；01b88d7d 起发射层经 `class_init_targets` 读取，`unknown` 时退回 `<clinit>` 全表）与 `c1d-prec` 的 `[facts.reflect] class_initializers`（`mirror_inits`，生成器钩子表消费，另含成员声明类初始化点收窄 §11）。只留后者，删 `engine/class_init.rs` 与清单段。
- 系统属性只读形参判定（`sysprops_readonly`）改用主干的 `memo_enter` / `memo_leave` 帧（记忆与处理次序无关），替代原 `in_progress` 集合。
- 服务目录事实（`seeds.toml [services]`）保留。

### 14.6 合并后闭包对照（合并前 s2 = 34d5989a+f8203ba3 → 合并后）

| 用例 | 合并前（类 / 方法） | 合并后（类 / 方法） | 分析耗时 |
|---|---:|---:|---:|
| HelloWorld | 1987 / 13385 | 2868 / 16695 | 55 s |
| TestDirectBuffer | 1992 / 13481 | 2871 / 16772 | 77 s |
| TestMethodHandleCombinators | 1989 / 13418 | 2872 / 16775 | 66 s |
| TestStreamBasic | 2002 / 13489 | 2879 / 16758 | 53 s |
| CollectorsDemo | 1987 / 13385 | 2868 / 16727 | 91 s |
| Digester | 2662 / 17744 | 2869 / 16704 | 61 s |

**合并后首测分析器失控（HelloWorld > 300 s、RSS 3.9 GB，合并前 12 s）**，归因与修复：

- 二分：去掉 `seeds.toml [services]` 即回到 12 s / 1838 类；去掉 `population` 根仍失控。
- 机制：`ResourceBundle.getServiceLoader` 的服务 Class 来自 `getResourceBundleProviderType`（按 bundle 基名拼出 `<包>.spi.<名>Provider` 再 `Class.forName`），值集含 open。精度三期的健全回退「所指未知 → 目录里全部服务」在有 `[boundary]` 截断时碰不到（`jdk/`、`sun/` 内的查找点被截），C1d 终态下真实可达，一次选入 jlink / jshell / javax.sound / JMX 等全部模块服务的 provider。
- 修复（终态口径）：所指未知的服务 Class 只能是**闭包内**的类——原生程序里只有闭包内的类有类镜像（`Class.forName` 找不到闭包外的类）。未知站点只选闭包内的目录服务，并登记为「目录服务类入闭包即重跑」的站点（`engine/services.rs` `unknown_sites`、`classes.rs` `touch` 钩子），健全性与原回退相同。
- 量化：未知回退本身现只多 19 类（实测：未知站点不选任何服务时 HelloWorld 2849 类）。

**合并后类数 +880 的来源：已知镜像的服务 provider，属健全性补齐，不是回归。** 与无服务事实（1838 类）相比多出 1011 类，主体是 `java/security/Provider` 的 8 个模块 provider（SunEC、SunPKCS11、XMLDSigRI、JdkLDAP、JdkSASL、SunPCSC、JGSS、SASL；`sun/security/*` ≈ 330 类、`com/sun/org` 113 类），其次 FileSystemProvider（jrtfs / zipfs）、ExtendedCharsets、LoggingProviderImpl、CLDR locale 元数据。`ProviderConfig$ProviderLoader` 以 `Provider.class` 字面量查找服务——只要 JCA 可达，JVM 就可能按 `java.security` 的 provider 列表加载它们。合并前的 `c1d-prec` 没有服务事实，这些 provider 不在闭包内，运行期按名取 provider 会命中存根，是不健全的。

所以缩小闭包的着力点不在服务事实，而在 **JCA 在 HelloWorld 中可达本身**（§2–§4 的伪可达链：`CryptoAlgorithmConstraints` / `MessageDigest` 经 jar 校验与 URL 处理入链）。剩余的精度项：

- `ResourceBundle$3.run` 的 `ResourceBundleProvider.class.isAssignableFrom(c)` 守卫未建模，未知回退因此仍选入闭包内的 `java/security/Provider`、`RandomGenerator`（19 类）。终态修法：`ldc A; aload c; invokevirtual Class.isAssignableFrom; ifeq` 在真边上把 c 的镜像值集收窄为 A 的子类型，open 镜像带上界 A；服务查找按上界只选 A 的子类型服务。需要在 absint 的条件边上做按局部变量的值收窄，并给 open 镜像加上界表示，属分析器新能力，单列为精度项。

## 15. S4 / S5 拆层下的编译成本对照（c1d 25381ad8 + 47305eb9 + 0c0802ba 对 集成分支 c9d0a0ca）

方法：`scripts/crate_profile.sh`，两边在同一把 heavy_lock 内背靠背运行（先 base 后 c1d），`CARGO_BUILD_JOBS=2`、`CARGO_INCREMENTAL=0`，
依赖预编，强制重编 workspace 自有 crate（java_runtime / java_meta / java_body_* / user）；peak 为单个 rustc 的 `/usr/bin/time -l` 最大 RSS。

| 用例 | 分支 | JDK 类 | 生成源（声明层 / 实现层） | body crate 数 | cargo 墙钟 | java_runtime（声明 crate） | body crate 峰值区间 | java_meta | 最大单 rustc 峰值 |
|---|---|---:|---|---:|---:|---|---|---|---:|
| HelloWorld | base | 246 | 3.7 / 3.1 MB | 2 | 15.3 s | 10.4 s / 1172 MB | 465–477 MB | 0.6 s / 397 MB | 1172 MB |
| HelloWorld | c1d | 2866 | 31.8 / 41.5 MB | 9 | 186.6 s | 121.0 s / 5110 MB | 1086–1478 MB | 7.2 s / 1652 MB | 5110 MB |
| Digester | base | 1167 | 14.2 / 17.0 MB | 4 | 75.1 s | 46.6 s / 4146 MB | 1102–1168 MB | 2.5 s / 1290 MB | 4146 MB |
| Digester | c1d | 2867 | 31.8 / 41.6 MB | 9 | 187.2 s | 124.2 s / 5148 MB | 1027–1474 MB | 5.8 s / 2656 MB | 5148 MB |

差距：

- HelloWorld：墙钟 ×12.2（+171 s），最大峰值 ×4.4（+3.9 GB）；Digester：墙钟 ×2.5（+112 s），最大峰值 +1.0 GB。
- 两例在 c1d 下收敛到同一 JDK 不动点（2866 / 2867 类，生成源几乎相同），编译成本与用例无关——即 §1–§4 的「区域是吸引子」在编译侧的体现。
- 关键路径是声明 crate `java_runtime`（121–124 s，占墙钟 65%）：body crate 之间可并行（JOBS=2 下 9 个 body crate 约 2 × 60 s），
  但都依赖声明 crate；声明 crate 峰值 5.1 GB，是 V 系列目标 ≤ 2 GB 的 2.6 倍。按 §7.5.4 的达标账（峰值 ≈ 0.72 GB + 0.048 GB × 声明层宏展开 MB）
  外推，c1d 声明层展开约 92 MB。
- 结论：C1d 合入后编译成本由闭包规模决定（≈2870 类），不是拆层失效。缩小手段仍是 §13.5 的精度路线（JCA / 服务 provider / Formatter 区域），
  以及声明层体量的结构性削减（S7 统一对象句柄）；单靠增加 body crate 数只能分摊实现层，不改变声明 crate 关键路径。

## 16. 决策依据：c1d 下 HelloWorld 多出的类从哪里来（71bd241f，`rava closure -o` 与 `--why`）

c1d 2825 类，集成分支 c9d0a0ca 248 类：多出 2578 类，少 1 类。

**按包统计多出的类（前 15）**

| 包 | 多出 | c1d / base |
|---|---:|---:|
| java/util | +259 | 298 / 39 |
| java/lang/invoke | +120 | 123 / 3 |
| sun/nio/cs | +93 | 106 / 13 |
| java/util/stream | +91 | 93 / 2 |
| java/lang | +90 | 172 / 82 |
| sun/security/util | +75 | 75 / 0 |
| java/security | +71 | 76 / 5 |
| java/util/regex | +61 | 61 / 0 |
| java/util/concurrent | +53 | 63 / 10 |
| sun/security/ec | +50 | 50 / 0 |
| com/sun/org/apache/xml/internal/security/algorithms/implementations | +43 | 43 / 0 |
| sun/nio/fs | +43 | 43 / 0 |
| java/text | +43 | 43 / 0 |
| java/io | +42 | 66 / 25 |
| sun/security/x509 | +41 | 41 / 0 |

按领域合并：安全（sun/security、java/security、javax/crypto、XMLDSig、SunJCE）606；jdk/internal 其他 283；java/util 274；text / locale 192；invoke 130；java/lang 113；nio fs 101；charset 95；time 92；stream 91。

**入链路径（`--why` 首次发现链，从被拉入的类往入口方向读）**

1. **Formatter 区域（总入口）**：`HelloWorld.greet` → `System.out` → `UTF_8.<clinit>` → `Charset.<init>` → `checkName` → `String.charAt` → `checkIndex` → `Preconditions.outOfBoundsMessage` → `String.format` → `Formatter.<clinit>` → `Pattern.compile`。
   越界消息路径在 Java 语义上是真实可达的，§2 已实测：只切这一条入口 Δ=0，必须同时切 `ParameterizedTypeImpl` 才能 −1599。
2. **regex → locale → stream / 反射**：`Pattern.family` → `CharPredicates.forUnicodeProperty` → `String.toUpperCase` → `ConditionalSpecialCasing` → `BreakIterator.getWordInstance` → `LocaleProviderAdapter.forType` → `Class.getDeclaredConstructor` → `Class.methodToString` → `Arrays.stream` → `ReferencePipeline`。
   stream 的 91 类与 locale 的 192 类主要从这里进入。
3. **ResourceBundle 服务 → JCA**：`ResourceBundle.getBundleImpl` → `loadBundle` → `CacheKey.hasProviders` → `getServiceLoader` → `ServiceLoader.load`，按服务事实选中闭包内的服务（§14.6 的未知回退）→ `SecureRandom.<init>` → `getDefaultPRNG` → `sun/security/provider/SecureRandom.init` → `MessageDigest`。
   接着由 `ProviderConfig$ProviderLoader.<clinit>` → `ServiceLoader.load(Provider.class)` 选入 SunEC / XMLDSig 等模块 provider。安全领域的 606 类主要从这里进入。
4. **反射调用 → LambdaForm**：`Method.invoke` → `DirectMethodHandleAccessor$NativeAccessor.invoke` → `ReflectiveInvoker.<clinit>` → `Lookup.findVirtual` → `DirectMethodHandle.make` → `LambdaForm.<clinit>`。
   这条路径带入 invoke 的 120 类。
5. **ClassFileDumper → 文件系统**：`ClassFileDumper.<init>` → `validateDumpDir` → `Path.of` → `FileSystems.getDefault` → `DefaultFileSystemProvider` → `UnixFileSystemProvider`。
   nio fs 的 101 类从这里进入；`sun/nio/fs/Util.<clinit>` → `Charset.forName` → `ExtendedProviderHolder` → `ServiceLoader` 又接回第 3 条。
6. **时区 → java/time**：`Calendar.defaultTimeZone` → `TimeZone.getDefault` → `ZoneInfoFile.getZoneInfo` → `LocalDateTime.<clinit>` → `LocalDate`。
7. **货币 → 日志**：`DecimalFormatSymbols.getCurrencySymbol` → `Currency.<clinit>` → `Currency$CurrencyProperty.info` → `PlatformLogger.getLogger` → `LazyLoggers` → `BootstrapLogger`。

这些链彼此互为替补（§3、§7）。第 1 条入口打开 Formatter / regex 之后，第 2–7 条在区域内部互相连通，任何单点切除的收益都接近 0。合理的收窄只有两条路：

- 内容感知精度：常量实参求值，分支 / 守卫收窄，例如 §14.6 的 `isAssignableFrom` 上界；
- 按运行期实际可达程度，把 VM 启动形态的状态作为事实注入，例如 ClassFileDumper 的开关、时区与 locale 的确定值。

## 17. 9 例批次（71bd241f，`run_tests.py --filter`，顺序执行）

8 例通过，失败 1 例 DeepCopy。每例的 transpile 约 17 s，build 2 m 47 s – 3 m 10 s，二进制约 264 MB。
TestInterfaceInheritedOverloads 通过：取消 `[boundary]` 后，`ArraysSupport.toArrayReversed` 体内对 `Arrays.copyOfRange` 的调用已入链。

DeepCopy 失败归因：序列化路径触发了 `ExceptionInInitializerError ← NullPointerException`。

- 调用栈：`ObjectInputFilter$Config.<clinit>` → `System.getLogger` → `BootstrapLogger$DetectBackend.<clinit>` → `ServiceLoader.iterator` → `ModuleLayer.layers(ClassLoader)` → `ClassLoaderValue.get(null)` → `BootLoader.getClassLoaderValueMap()`。
- 最后一步返回 null：`BootLoader` 在 closure.toml 中是 VM 边界类，它的 `<clinit>` 不翻译，所以 `CLASS_LOADER_VALUE_MAP` 从未赋值；而 `getClassLoaderValueMap` 按字节码翻译，只是读取这个静态字段。
- 集成分支上，这条路径被 `[boundary]` 截断，不会走到。
- 修法待定，有两个方向：
  - 把 `BootLoader` 列入边界类中按字节码翻译 `<clinit>` 的名单。其 `<clinit>` 另含 `defineUnnamedModule`、`setBootLoaderUnnamedModule0`、`NativeLibraries.newInstance`，需要逐项核对 VM 契约。
  - 由 VM 注入常量清单给 `CLASS_LOADER_VALUE_MAP` 赋初值。

## 18. 精度分步方案（c1d-prec 72873705 = 合入集成分支 a1e781a8 后）

口径：闭包缩小是目标。每一步的判据是 e2e 通过、动态对照漏覆盖为 0、无存根命中。下文「切除模拟」指用 `rava closure --cut-file` 把某一精度手段将证明不可达的调用点切掉，用来量这一手段在终态下的上界。切除模拟本身不健全，只用于测量，不进入实现。

### 18.0 实测底数（HelloWorld，`rava closure -o`）

| 配置 | 类 | 方法 | 说明 |
|---|---:|---:|---|
| e0：无切除 | 3092 | 18742 | 比 §16 的 2825 多 268 类，全部来自合入带来的已知服务 provider（`sun/security/pkcs11` 72、`javax/crypto` 27 等），属于健全性补齐（同 §14.6） |
| e6：只切区域**内部**（CHM 两处、FormatSpecifier.print 六个非 `%s` 分支、Formatter.parse 的两个正则回退、VM.saveProperties@71、HashMap.<init>(IF)@74、ClassFileDumper.<init>@52） | 3079 | 18595 | −13：入口还开着，区域内部互为替补（§16），切内部基本无效 |
| f0：切四个**入口**（Preconditions.outOfBoundsMessage、PTI.validateConstructorArguments、Class.getGenericInterfaces、CHM.fullAddCount） | 319 | 1228 | 区域整体脱落 |
| f1：f0 + e6（本节全部精度手段的上界） | 317 | 1202 | code 223 / layout 60 / type 17 / init 15 / alloc 2 |

结论一：**有效的是入口，不是内部。** 闸门是 3 个入口：OOB 消息、PTI.validateConstructorArguments、getGenericInterfaces（经 CHM.comparableClassFor）。三者同时关掉，区域才会脱落（3092 → 约 320）；任何一个开着，类数都会回到约 3000。fullAddCount、Formatter 内部、saveProperties、HashMap、dumper 在闸门关掉后只值 0～10 类。因此下面各步的 Δ 分两栏：「终态缺此步」是在 f1 上撤掉这一步的切除后的类数（留一法，表示这一步在终态里的价值）；「单独落地」是只做这一步时 HelloWorld 的变化。

留一法实测（f1 基础上撤掉一项）：

| 撤掉的一项 | 类 |
|---|---:|
| 无（f1） | 317 |
| CHM.comparableClassFor@21 + getGenericInterfaces | 区域重开（>5 min 未结束，约 3000） |
| CHM.fullAddCount | 327（+10） |
| Formatter 内部（print 六分支 + parse 两回退） | 317（+0） |
| VM.saveProperties@71 | 317（+0） |
| HashMap.<init>(IF)@74 | 319（+2） |
| ClassFileDumper.<init>@52 | 317（+0） |
| 以上全部撤掉，只留 OOB + PTI 两个入口 | 区域重开（>90 s 未结束） |

和 JVM 对照（`-Xshare:off -verbose:class`，HelloWorld 初始化 437 类）：f1 与 JVM 已初始化集合的交集是 202 类。其余 115 类中，77 类是 layout/type 级的接口与声明（Serializable、Comparable、CharSequence 的父接口、`JavaXxxAccess` 等）；其余是异常类、CharacterData0x、CHM 内部节点、Float/DoubleToDecimal 等可达的代码。f1 已经接近 rava 运行模型下的健全下限。

### 18.1 ≤ 250 的可行性：如实说明

- **健全终态的估算是 ≈ 355 类**，不是 ≤ 250：
  - f1 = 317 是各项精度手段都做到极致时的**类数下限**，前提是把越界消息路径整条切掉。但 `String.charAt` 越界时，`Preconditions.outOfBoundsMessage` → `String.format("Index %s out of bounds for length %s", …)` 是真实会执行的 Java 语义，健全的分析不能切。
  - 常量实参求值（18.5）能做到的是：只保留 `%s` 这一种转换。这样仍要保留 Formatter、Pattern（`Formatter.<clinit>` 无条件 `Pattern.compile` 一个常量正则）、Locale 等，JVM 实测约 40 类（单独执行一次 `String.format("%s", …)` 的增量）。
  - 合计约 317 + 40 ≈ 355 类，其中 code 级约 260。
- **集成分支的 248 来自 `[boundary]` 截断**，不是来自精度。截断本身不健全：§17 的 DeepCopy 失败就是截断掩盖的路径。拿它作合入门槛，等于要求健全分析逼近一个不健全的数。
- 再往下，只剩改变语义的手段：
  - 构建期声明「越界不发生」；
  - 把 Formatter 的静态状态做成构建期堆快照（类似 GraalVM 的 image heap）。

  前者不健全。后者是另一条架构线（构建期执行 `<clinit>`），不在本方案范围内。
- **请主会话裁定门槛**，二选一：
  - (a) 改为 **HelloWorld 总类 ≤ 360，且 code 级 ≤ 270**。这是 18.2–18.8 全部落地后的健全终态，各步数字可逐项验收。
  - (b) 坚持 ≤ 250。那就必须先批准一种构建期语义声明（例如「越界消息惰性化」）。它不属于精度手段，需要单独立项。

  本节按 (a) 编写。

### 18.2 步骤 P0：BootLoader `<clinit>` 按字节码翻译（修 DeepCopy，§17）

- 机制：
  - 把 `jdk/internal/loader/BootLoader` 加入 `closure.toml` 的 `translate_clinit`（同 `jdk/internal/misc/VM`）。
  - `<clinit>` 的四个外调逐项核对 VM 契约：
    1. `JLA.defineUnnamedModule(null)`：字节码，经 `System$2` 到 `new Module(null)`。
    2. `JLA.addEnableNativeAccess`：字节码。
    3. `setBootLoaderUnnamedModule0`：`ACC_NATIVE`，属手写准入 ①。手写到 `boot_loader_impl.rs`，语义是登记引导类加载器的未命名模块。
    4. `NativeLibraries.newInstance(null)`：字节码。
  - 引导类的模块必须是同一个对象。现在 `module_impl.rs` 的进程单例要改成：`setBootLoaderUnnamedModule0` 登记的那个 Module；`Class.getModule` 对引导类返回 `BootLoader.getUnnamedModule()`（字节码方法，会触发 `<clinit>`）。这样 `CLASS_LOADER_VALUE_MAP` 等静态字段由字节码赋值，不需要 VM 常量注入。
- 预期 HelloWorld 类数：+0～+5（BootLoader 已在闭包里；新增的是 `<clinit>` 链上的 NativeLibraries 等）。
- 验收用例：DeepCopy 通过；TestForNameInit、HelloWorld、TestInterfaceInheritedOverloads 不回归；动态对照漏覆盖为 0；无存根命中。
- 改动：
  - `runtime/java_runtime/closure.toml`（`translate_clinit`）；
  - `runtime/java_runtime/src/jdk/internal/loader/boot_loader_impl.rs`（native `setBootLoaderUnnamedModule0`）；
  - `runtime/java_runtime/src/java/lang/module_impl.rs`、`class_impl.rs`（`getModule` 改用 BootLoader 的未命名模块）；
  - `vm_intrinsics.toml` 登记该 native。生成器 crate 不改。

- 实施结论（2026-10-02，读代码后修正上面「引导类的模块必须是同一个对象」一条）：
  - 只做 `translate_clinit` + native `setBootLoaderUnnamedModule0`（`#[jvm_native]` 由 build.rs 自动登记 native_status，`vm_intrinsics.toml` 无需新条目）。
  - `Class.getModule` 不改接 `BootLoader.getUnnamedModule()`。理由：
    1. JVM 中引导加载器的无名模块只承载 `-Xbootclasspath/a` 上的类；JDK 类属命名模块 java.base，用户类属应用加载器的无名模块。任何类的 `getModule()` 都不是它，改接反而偏离 JVM。
    2. 原生二进制没有引导类路径（`hasClassPath` 恒 false），所以 VM 登记是 no-op，模块对象仍由字节码建立，`getUnnamedModule()` 照常返回它。
    3. DeepCopy 只依赖 `CLASS_LOADER_VALUE_MAP` 由 `<clinit>` 字节码赋值，与 `getModule` 无关。
  - 顺带发现的手写层重复（不在 P0 内，记为后续清理项）：`class_impl.rs` 里有两套模块单例。生效的是 `#[jvm_boundary] getModule`，它用自己的 `THE_MODULE`，loader 为 null；另有 `__impl_getModule` → `module_impl::unnamed_module()`，loader 为系统加载器，但两者全仓无调用方，是死代码。终态应只保留一个单例：用户类对应应用加载器的无名模块，即 loader = 系统加载器。改它会改变 `getModule().getClassLoader()` 的可观测值，需要单独验收。

### 18.3 步骤 P1：3 条 asm Type/Frame 漏覆盖（dyn_compare 归因修正）

- 定位（已实测）：这 3 条不是闭包漏类。
  - JVM 栈是：HelloWorld.greet@7（indy 站点，已由 indy 模型覆盖）→ `MethodHandleNatives.linkCallSite` → `StringConcatFactory` → … → `LambdaForm.compileToBytecode` → `InvokerBytecodeGenerator` → asm `Type` / `Frame`。
  - 这是 JVM 自己的链接过程，rava 的运行模型从不执行这段（准入 ②：运行模型替换）。
  - `dyn_compare.attribute()` 的 model_sites 规则取的是「站点之上的第一个字节码帧」。链接中途的 JDK invoke 帧都在 c1d 闭包里，于是归因在链接内部恢复，误报为漏覆盖。
- 修法：
  - 模型站点正上方是 `closure.toml [dynamic] vm_upcall_classes` 的帧（即 `MethodHandleNatives`）时，整段归给该 indy 模型，不再往下找。
  - 规则由清单驱动，脚本里不写类名。
- 预期 HelloWorld 类数：Δ = 0。漏覆盖从 3 降到 0（HelloWorld 2、TestInterfaceInheritedOverloads 1）。
- 验收用例：
  - 对 HelloWorld、TestInterfaceInheritedOverloads 跑 dyn_compare，漏覆盖为 0；
  - `python3 -m unittest tests.unit.test_dyn_compare` 新增「链接段归 indy 模型」用例。
- 改动：`scripts/dyn_compare.py`、`tests/unit/test_dyn_compare.py`。

### 18.4 步骤 P2：守卫收窄（absint 条件边上按局部变量收窄值集）

- 机制：在 `ifeq` / `ifne` / `if_acmpeq` / `if_acmpne` 的真假边上，对来源局部变量收窄镜像或类型值集：
  - `x.getClass() == C.class`：真边 x ∈ {C}，假边去掉 C；
  - `instanceof C`：真边与 C 求交；
  - `A.class.isAssignableFrom(c)`：真边把 c 收窄为 A 的子类型；open 镜像带上界 A（§14.6）。
- 服务查找按上界只选 A 的子类型服务。`ResourceBundle$3.run` 的未知回退因此不再选入 `java/security/Provider`、`RandomGenerator`（§14.6 实测 19 类）。
- `CHM.comparableClassFor`：守卫本身只排除 String；getGenericInterfaces 是否可达，取决于进入 CHM 树化箱的键的类集合。
  - 键类集合的来源：`putVal` 的 key 参数，经 `treeifyBin` → `TreeBin.<init>` → `comparableClassFor`，需要参数值集在方法间传播。
  - 已有的「方法参数类型值集」在本步扩到这条链。
  - 若闭包内进入 CHM 的 Comparable 键只有 String，getGenericInterfaces 就不可达。
  - 否则改由 18.5 的具体求值器对键类候选集逐个求值 `getGenericInterfaces`：签名串是构建期常量，解析轨迹确定。
  - 实测 getGenericInterfaces 是独立闸门：OOB 与 PTI 都切掉、只留它开着，区域仍重开（留一法 >5 min）。所以本步与 18.5 至少一条必须关掉它。
- 预期 HelloWorld 类数：
  - 终态缺此步：区域重开（>5 min 未结束，约 3000） 类；
  - 单独落地：Δ ≈ 0（OOB / PTI 入口仍开着）。
  - 在全闭包形态下，ResourceBundle 一项 −19。
- 验收用例：
  - HelloWorld；
  - 带 ResourceBundle 的用例（TestFormatLocale、TestLocaleLanguageTag）；
  - 序列化、反射用例（DeepCopy、TestAnnoValues），用来确认收窄不误杀。
- 改动：`generator/crates/` 闭包分析器的 absint 条件边（`engine/` 下的分支求值模块与值集表示，open 镜像加上界字段）、`engine/services.rs`（按上界选服务）。不涉及清单。

### 18.5 步骤 P3：常量实参求值（确定性具体求值器）

- 机制：
  - 调用点实参全是常量（ldc / 已折叠的静态常量）、被调方法体只读常量和自身新建的对象、步数有界时，在分析期按字节码**具体执行**被调方法。
  - 执行轨迹上实际经过的方法和分支计入闭包；未经过的分支不计入。
  - 产生的对象把字段的具体值并入该字段的值集，供后续非常量代码的分派收窄。
  - 任一条件不满足（遇到非白名单 native、写外部静态字段、超出步数），整次回退到现有抽象解释。这样保证健全。
- 覆盖的入口：
  1. **OOB 消息**：`outOfBoundsMessage(常量 kind, …)` 按常量 kind 只走对应分支。`String.format(常量格式串)` → `Formatter.parse(常量)` 得到具体的 FormatSpecifier 列表。所有常量格式串的转换字符并集写入 `FormatSpecifier.c` 的值集；`print` 的 switch 按值集只留 `%s`（printString）分支。
     - 只要闭包内出现一个非常量格式串，`c` 的值集就是 open，退回现状。
  2. **Formatter.parse 的正则回退 @141/@174**：常量格式串能被手写解析器吃完时不经过回退。
  3. **HashMap.<init>(IF)@74**：所有调用点的 loadFactor 都是正常量，`loadFactor <= 0 || isNaN` 的分支死，`"Illegal load factor: " + float` 不可达。这需要参数常量值集按调用点合并。
  4. **ClassFileDumper.<init>@52**：dumper 的属性键由调用点常量传入；`privilegedGetProperty(常量键)` 交给 18.7 的属性事实（键缺省即 null）；`Boolean.parseBoolean(null)` = false，`validateDumpDir` 不可达。
  5. **PTI.validateConstructorArguments**（泛型签名构建期校验）：
     - PTI 只由泛型签名解析器创建，签名串是 class 文件里的构建期常量。
     - 分析器对闭包内所有带签名的类校验「参数化类型的实参个数 = 原始类的类型形参个数」。全部通过时，错误分支（`String.format` 的 `%d`）死。
     - 有类不通过时只放开那一处。
- 预期 HelloWorld 类数：
  - 本步加上 P2、P4 一起关掉全部入口后，约 355 类（f1 = 317，加上 Formatter `%s` 下限约 40）。
  - 单独落地：Δ ≈ 0（fullAddCount、getGenericInterfaces 入口仍开着）。
  - 终态缺此步：OOB / PTI 入口重开，约 3000 类；Formatter 内部 317（+0）；HashMap 319（+2）；dumper 317（+0）。
- 验收用例：
  - HelloWorld；
  - 真正使用 `%d` / `%f` 的用例（TestStringFormat、StringFormatTest、FormatOutput）（确认值集 open 时退回、不漏）；
  - TestForNameInit；
  - DeepCopy；
  - 动态对照漏覆盖为 0。
- 改动：
  - `generator/crates/` 闭包分析器新增具体求值子模块（`engine/concrete/`，按 ~600 行拆分：解释器、堆、白名单）；
  - 字段值集表（并入具体值）；
  - 参数常量值集按调用点合并；
  - 泛型签名校验（`input` crate 读签名，`engine` 校验）；
  - 具体求值可调用的 native 白名单放在 `vm_intrinsics.toml`，不在代码里写类名。

### 18.6 步骤 P4：单线程期 CAS 事实（CHM.fullAddCount）

- 机制：
  - `CHM.addCount` 只在 `counterCells != null` 或 `U.compareAndSetLong(BASECOUNT)` 失败时进入 `fullAddCount`；而 `counterCells` 只在 `fullAddCount` 内被赋值。
  - 闭包内不可达任何线程启动点时（`Thread.start0` 等，由 `vm_intrinsics.toml` 登记为线程派生点），`Unsafe.compareAndSet*` 对未被其他线程触及的字段必然成功，CAS 的失败边死。
  - 线程派生点一旦入闭包，就整体重跑（同 §14.6 的 touch 钩子）。x1 实测 `Thread.start` 经 `CleanerImpl.start` 可达，所以这条事实依赖闭包，必须带重跑。
- 预期 HelloWorld 类数：
  - 终态缺此步：327（+10） 类；
  - 单独落地：Δ ≈ 0。
  - 多线程用例不受影响：派生点可达时事实不成立。
- 验收用例：HelloWorld；起线程的用例（ThreadTest、TestThreadJoin、TestVirtualThread），确认事实撤销后闭包与现状相同；动态对照漏覆盖为 0。
- 改动：
  - `vm_intrinsics.toml`（新 `[facts.single_thread]`：线程派生点、CAS 族）；
  - 闭包分析器 `engine/` 的 CAS 折叠与 touch 重跑钩子（`classes.rs` touch）。

### 18.7 步骤 P5：VM 启动态事实走 TOML 清单注入

- 机制：把 VM 启动形态里确定的状态写进清单，由分析器读取并折叠。
  1. **saved properties 读取器**：`VM.saveProperties(Map)` 的参数就是初始属性表。`props.get(常量键)` 按 `[facts.system_properties]` 折叠：原生二进制没有 `-D`，键不在 `values` / `dynamic` 中即为 null。
     - 新增 `map_readers` 条目，声明哪些方法的哪个 Map 形参承载初始属性表。
     - 效果：@71 `Long.parseLong` 不可达（`sun.nio.MaxDirectMemorySize` 缺省）。
  2. **ClassFileDumper 开关**：`MethodHandles$Lookup.<clinit>` 以常量键 `jdk.invoke.MethodHandle.dumpClassFiles` 构造 dumper，该键缺省即 null。与 18.5 第 4 项配合。
  3. **服务目录**：`seeds.toml [services]` 已有，本步不变。
  4. **时区、locale**：`user.timezone`、`user.language`、`user.country` 在 `dynamic` 中，取自宿主环境，运行期才知道，**不能**健全地折叠成常量。
     - 只作为构建期**声明**的 opt-in 事实（§13.5）：例如 `rava build --locale en-US --timezone UTC` 写进清单，生成的二进制在启动时也固定这些值，语义自洽。
     - 未声明时保持现状。
- 预期 HelloWorld 类数：
  - saveProperties 终态缺此步 317（+0）；
  - dumper 见 18.5；
  - 时区 / locale 在 HelloWorld 的 f1 形态中不出现，Δ = 0。它们的收益在真正用到 `%d`、Calendar 的用例上（§16 第 6、7 条链）。
- 验收用例：HelloWorld；TestAnnoValues；TestDateTimeFormat（未声明时与现状一致）。
- 改动：`runtime/java_runtime/vm_intrinsics.toml`（`[facts.system_properties] map_readers`；opt-in 的 locale / timezone 声明段）；闭包分析器读清单的模块（`input` 的清单解析、`engine` 的属性折叠）；`driver` 的 build 选项。

### 18.8 步骤 P6：ServiceLoader 已知服务类型

- 机制：
  - `ServiceLoader.load(ldc C)` 或 `load(C, loader)` 的服务 Class 是常量时，只选目录里 C 的 provider（已实现）。
  - 本步把「常量」扩到 18.5 求值器能求出的 Class，以及 18.4 带上界 A 的 open 镜像（只选 A 的子类型服务）。
  - 未知且无上界时，保持 §14.6 的健全回退。
- 预期 HelloWorld 类数：f1 形态下 Δ = 0（ServiceLoader 不可达）。全闭包形态下与 18.4 合计 −19 起。真正的收益在用到 ResourceBundle、Charset.forName 的用例上。
- 验收用例：TestFormatLocale、TestLocaleConstants；Digester；动态对照漏覆盖为 0。
- 改动：`engine/services.rs`。清单不变。

### 18.9 实施顺序与合入点

1. P0、P1 互相独立，先做，各自单独合入。它们不改变 HelloWorld 类数，修的是 DeepCopy 和漏覆盖。
2. 闸门步骤 P2、P3 分别开发，各自以单元测试和切除对照验收：「单独落地 Δ ≈ 0」是预期，不算失败。两者都完成后，三个闸门全部关掉，HelloWorld 从约 3092 降到约 365，届时合入并跑 9 例批次。
3. P4（−10）、P5（HelloWorld −0～2，收益在其他用例）在闸门之后做，每步单独合入，HelloWorld 降到约 355。
4. P6 在 P2、P3 之后做。
5. 每步提交前记录 4 例闭包类数（HelloWorld、Digester、DeepCopy、TestForNameInit），写进本节表格。

### 18.10 类初始化事实：`class_init` 与 `mirror_init` 归并结论（2026-10-02，读代码）

- 现状：`c1d-prec` 合入 01b88d7d（aa829d0b）时已归并为一套。分析器只有 `engine/mirror_init.rs`，清单只有 `[facts.reflect] class_initializers` / `handle_owner_initializers` / `reflect_owner_initializers`，输出 `seeds.mirror_inits` → 输入 `mirror_init_classes` → 发射层 `class_init_hooks`。全仓代码已无 `engine/class_init.rs`、`[facts.class_init]`、`class_init_targets`，只剩历史计划文档里的记载。
- 主干那套被舍弃的两点，终态都不需要：
  1. **`unknown` 时退回 `<clinit>` 全表**：所指类推不出时，分析期该类的 `<clinit>` 本来就没进链，静态字段值集已经缺了它的写入。运行期补钩子修不了闭包的健全性，还会把钩子表放大到全部带 `<clinit>` 的类。终态口径是把它记为反射缺口（`reflect.gaps`，按站点列出），目标是缺口为 0。现有用例里 `ensureClassInitialized` 相关缺口已经是 0（§11）。
  2. **`ensureClassInitialized0` 及形参序号表**：`Unsafe` 是 VM 边界类，`ensureClassInitialized` 是共置手写（按名调用 `ensure_class_initialized`），它的 native 内层不会出现在调用链上。`mirror_init` 按描述符扫描全部 `Class` 形参，不需要序号表。
- `mirror_init` 多出来的、主干没有的部分是成员声明类初始化点的收窄（§11）：
  - `handle_owner_initializers` / `reflect_owner_initializers` 不按 `MemberName.clazz` 这类汇合值集求目标；
  - 运行期目标取按句柄 / 反射实际链接到的成员声明类（`linked_owners` × `live_routes`）；
  - 按名取静态字段、所属类推不出时，扫描闭包内声明了该名静态字段的类。
- 结论：不需要再改代码。命名维持 `mirror_init`（清单段位于 `[facts.reflect]`，语义是「按类镜像初始化」），旧计划文档里的 `class_init` 记载加注已被取代。
- **合入 be1b97be 时的复核（2026-10-02）：**
  - 集成分支的 `class_init`（`engine/class_init.rs` 与 `[facts.class_init.initializers]`）由 1b841e40 引入（2026-10-01，早于合并基 01b88d7d），不属于 C6 / regress2 / native-gaps。
  - 合并基之后集成分支只在该表加了一项：native-gaps 的 462ab7b0 登记了 `JavaLangAccess.getEnumConstantsShared`（修 TestStackWalkerFrames：`StackWalker.<clinit>` 的 `EnumSet.noneOf(Option.class)` 报「not an enum」）。
  - 这一项修补的是集成分支上手写的 `java_lang_access_impl.rs`：手写体按名调用初始化钩子，但没有登记目标类。
  - c1d 已在 1e623cec 删除这份手写，`JavaLangAccess` 由 `System$2` 按字节码翻译。链路是 `Class.getEnumConstantsShared` → `getMethod("values").invoke(null)` → `MethodHandleAccessorFactory.ensureClassInitialized`，由 `reflect_owner_initializers` 覆盖，因此不需要单列这一项。
  - 同类的 native-gaps 修复 5bcc93ae（手写体 `T::__class_init()` 建模为类初始化）已随合并保留在 `engine/hw_infer.rs`。
  - 覆盖关系：`class_initializers` + `handle_owner_initializers` / `reflect_owner_initializers` 覆盖了集成分支三项登记的全部初始化点。唯一的语义差异是集成分支在 `unknown` 时退回 `<clinit>` 全表，c1d 改为记为反射缺口（见上第 1 点）。
  - 依赖用例：TestStackWalkerFrames、TestForNameInit（在 13265bd3 的抽查里），以及 TestStackWalkerLines、TestEnumSetMap、TestClassForName、TestMethodHandleDirect、TestVolatilePrimitiveAccess（下一轮补抽）。


## 19. 交接（2026-10-02）

原执行者在此收尾，C1d 由多个新子代理接手。本节写明接手所需的现状；机制细节以 §18 为准，不再重复。

### 19.1 分支、提交与集成分支的关系

| 分支 / worktree | HEAD | 用途 | 与集成分支（rust-closure-analyzer）的关系 |
|---|---|---|---|
| `c1d-pick`（`../java_rta_c1d_pick`） | 20ef4fdc | 从 c1d 精度线挑出的、可独立合入的修复 | 基于 01b88d7d。依次为 f72d7dc9、6a1d5aac、f212e863、51f8109a、b957535c、20ef4fdc。集成分支已合到 51f8109a（15ac9d00）；**b957535c、20ef4fdc 待合**。20ef4fdc 的抽查（tag `c1d-20ef4fdc`：DeepCopy、HelloWorld、TestDynamicProxy、TestReflectInvokeShapes、TestBmhDynamicSpecies、TestEnumBasic）在服务器上，结果尚未取回 |
| `c1d-serial-wip` | 17f7d04b | 序列化回调按名查找的半成品（§19.4） | 基于 c1d-pick 20ef4fdc。**不可合入** |
| `c1d-prec`（`../java_rta_c1d_prec`） | aa829d0b | C1d 精度主线：删除过渡手写，闭包按调用链收窄 | 已合入集成分支 01b88d7d。其中 1e623cec 删除了过渡手写 `java_lang_access_impl.rs`、`stream_decoder_impl.rs`，修复 TestRandomAccessFile、TestCharsetNamedStreams。HelloWorld 闭包约 3092–3125 类，`--stop-after closure` 约 600 s。**P2/P3 落地、HelloWorld ≤360 前暂停抽查** |
| `c1d-p0`（`../java_rta_c1d_p0`） | 本节提交 | 在 c1d-prec 上做 §18 分步 | aa829d0b 之后依次为 9604c5b7（P1：dyn_compare 归因穿过 vm_upcall 帧）、5ba54578（P0：BootLoader `<clinit>` 走 translate_clinit，native `setBootLoaderUnnamedModule0` 空实现）、bb9b1b70（§18.10 文档）、本节。慢和类数膨胀都继承自 c1d-prec，不是 P0 引入的。暂停抽查的条件同 c1d-prec |

恢复抽查时（HelloWorld ≤360 之后），名单加上 TestRandomAccessFile、TestCharsetNamedStreams。名单只放直接相关的用例，一般不超过 10 例。

### 19.2 三个入口闸门：现状与终态

终态目标：HelloWorld 闭包总数 ≤360（代码类 ≤270），`--stop-after closure` 降到秒级。

下面三个入口都是独立闸门：只要有一个开着，区域就会重开，结果约 3000 类，留一法测出来超过 5 min。三者**目前都开着，P2/P3 均未开工**。

| 闸门 | 入口 | 现状 | 关掉它的步骤 |
|---|---|---|---|
| OOB 消息 | `Preconditions.outOfBoundsMessage` → `String.format` → Formatter 全部转换分支 | 开 | P3（§18.5 入口 1、2）：常量 kind 只走对应分支；所有格式串都是常量时，`FormatSpecifier.c` 的值集是有限集，`print` 的 switch 只留 `%s` |
| PTI 校验 | `ParameterizedTypeImpl.validateConstructorArguments` 的错误分支（`String.format` `%d`） | 开 | P3（§18.5 入口 5）：对闭包内带签名的类做构建期校验，全部通过时错误分支死 |
| getGenericInterfaces | `ConcurrentHashMap.comparableClassFor`（经 `treeifyBin` → `TreeBin.<init>`） | 开 | P2（§18.4）：`putVal` 的 key 参数值集沿 treeify 链传播，Comparable 键只有 String 时不可达；否则用 P3 求值器逐个求值 |

P4（fullAddCount CAS 事实，§18.6）还关着第四个较小的入口。实施顺序见 §18.9。

### 19.3 P2 / P3 的具体做法、关键文件和函数

**P2 守卫收窄**
- absint 在 `generator/crates/closure/src/absint.rs` 中：
  - `cond(opc, a, b)`（约 467 行）是整数条件求值；
  - `INSTANCEOF` 事件在约 927、996 行生成；
  - 现在条件边上不收窄值集。
- 做法：
  1. 在 `ifeq/ifne/if_acmpeq/if_acmpne` 的真假后继上，给来源局部变量挂一个收窄后的值集。比较对象有三种：`getClass()==C.class` 的结果、`instanceof C` 的结果、`A.class.isAssignableFrom(c)` 的结果。
  2. 在值集表示里给 open 镜像加上界字段。
  3. `engine/services.rs` 的 `service_lookup` 在走未知回退（`unknown`）时，按上界只选 A 的子类型服务。
  4. `engine/worklist.rs` 约 244 行已经按 `INSTANCEOF` 收集了类型，可以作为收窄的起点。
- 方法参数值集要沿 `putVal → treeifyBin → TreeBin.<init> → comparableClassFor` 传到方法间；现有的参数值集在 `engine/flow.rs` 和 `engine/invoke.rs`。

**P3 常量实参求值**
- 新建 `engine/concrete/`，按约 600 行拆成三部分：
  - 解释器：确定性逐条执行字节码；
  - 堆：只容纳本次求值新建的对象；
  - 白名单：可调用的 native 列表，放在 `vm_intrinsics.toml` 新增节里。
- 求值的接入点：
  - 在 `engine/invoke.rs` 的调用点处理中，实参全是常量时先试具体求值。
  - 求值成功：只把轨迹上的方法、分支计入闭包，并把字段的具体值并入 F 节点值集（`engine/sets.rs` / `setstore.rs`）。
  - 求值失败：整次回退到抽象解释。
- 常量来源：
  - 已有的 `param_strs`（`engine/pstrs.rs`）只收集字符串，需要扩成按调用点合并的常量值集（int/float/字符串），HashMap loadFactor 入口要用；
  - `engine/consteval.rs`、`engine/fold.rs` 是现有常量折叠，可以复用。
- PTI 校验：
  - `input` crate 解析泛型签名；
  - 在 engine 中逐类比对「参数化类型的实参个数 = 原始类的类型形参个数」。
- 编码约束：不写 JDK 类名字面量。入口全部由清单或字节码形态识别。

### 19.4 序列化回调按名查找（regress2 第 1 项：StockTrans、TestSerialDefaultSuid）

终态要求：类未知、方法名和形参已知的反射查找（如 `ObjectStreamClass` 按名查 `writeObject`、`readObject`、`writeReplace` 等），只计入**已实例化且声明了该方法**的类，不按 CHA 全量拉入。

c1d-pick 的现状：
- b957535c 起，`engine/invoke.rs` 的 `reflective_writes` 只用站点名 `site_names`（`V::Str` 加 `a.lits()`）与接收者镜像求叉积。
- 名字只来自形参、接收者又不是常量时，记缺口 `recv(param-name)`。
- 因此这两例的回调没有被计入，运行时会命中存根。

c1d-serial-wip 17f7d04b 的半成品：
- 改动：
  - `engine/method_lookup.rs` 新增 `open_name_lookup(name)` 和 `open_names_on(id, only)`。它们遍历 `self.g` 中已实例化的类及其超类链，对声明了该名字方法的类调用 `reflect_name`。
  - `engine.rs` 新增字段 `open_lookup_names: BTreeSet<String>`。
  - `on_g_grow` 对新实例化的类调用 `open_names_on(id, None)`。
  - `invoke.rs` 中，`class_recv && classes.is_empty()` 时，不在 `site_names` 里的名字改为走 `open_name_lookup`，取代原来的缺口。
- 实测：

  | 用例 | 类数 | 说明 |
  |---|---|---|
  | DeepCopy | 1925 | c1d-pick 为 1640。fold_props 由正常降到 0，sysprops `all` = true，原因是 `privilegedGetProperties` 的常量合并失去属性表标记 |
  | StockTrans | 1923 | |
  | TestSerialDefaultSuid | 1930 | |

  共暴露 221 个序列化回调，涉及 File、FilePermission、各异常类等。原因是「已实例化」这个集合太宽。
- 收窄方案，二选一或组合：
  1. 只对真正到达序列化写入点的类型计名：`ObjectStreamClass.lookup` / `writeObject0` 中 `obj.getClass()` 接收者的值集。这需要把站点的接收者镜像集合作为过滤条件传给 `open_name_lookup`，不再全局开名。
  2. 按调用点的 `argTypes` 常量数组（`getDeclaredMethod(name, Class[])` 的形参类数组）过滤：签名不符的同名方法不计入。
- 验收：
  - DeepCopy ≤1715 且 fold_props 不回退；
  - StockTrans、TestSerialDefaultSuid 通过；
  - 生成器、闭包单测全过。

### 19.5 native-gaps 移交的两处过近似（不手写，修闭包精度）

- **linkToNative**：
  - 进入路径：`Method.invoke` → `DirectMethodHandleAccessor$NativeAccessor.invoke@91` → `[signature] methodAccessorInvoker():MethodHandle` → `[reflect]` class MethodHandle → `linkToNative`。
  - 签名里出现 MethodHandle 返回类型，反射模型就把 MethodHandle 全部成员视为反射可达。这是过近似。
  - 终态：签名里的返回类型只引入类型本身，不暴露成员；成员暴露只来自按名查找。
- **com/sun/media/sound 的 4 个 native**（DirectAudioDeviceProvider、PortMixerProvider 等）：
  - 怀疑来自 ServiceLoader 服务类型不精确（未知回退选进了 sound 的 provider），与 P2 / P6（§18.8）是同一个问题。
  - 服务器 trace 作业 `why-93e0f28e`（`rava audit api java/lang java/util --trace-class …`）的结果待取回，用来确认引入链。
- native-gaps 一侧的记录：native-gaps 分支 `docs/plans/2026-10-02-native-gaps.md` §三。

### 19.6 测量脚本与可删除的临时物

所有脚本都必须经 `python3 /Users/yuwei/dev/workspace/heavy_lock.py` 运行。脚本在 `/tmp` 下，不在仓库里，接手者可以照抄。

| 脚本 | 用法 |
|---|---|
| `/tmp/c1d_meas.sh <tag> <Test...>` | 先构建 c1d-pick worktree 的 rava，再对每个用例跑 `rava build <f> --stop-after emit --closure-json --clean`。把最新的 `build/*/closure_input/closure.json` 复制到 `/tmp/c1d_<tag>.<Test>.json`，并打印 classes / code / methods / fold_props / all。换分支时改脚本开头的 worktree 路径 |
| `/tmp/c1d_ut.sh` | 构建后跑 workspace 测试（排除 driver）和 `driver --bins`，参数为 `--test-threads=1` |
| `/tmp/c1dp0_time.sh` | 在 c1d-p0 和 c1d-prec 上计时 HelloWorld `--stop-after closure`。约 600 s，这类长测量应改为服务器作业 |
| `/tmp/c1d_item1.py` | 生成 17f7d04b 改动的补丁脚本，已提交，可删 |

可删除：
- `/tmp/c1d_mw.*`、`/tmp/c1d_ser.*`、`/tmp/c1dfix_dbg.sh`、`/tmp/c1dfix_dbg.log`、`/tmp/c1dp0_*.log`、`/tmp/c1d_item1.py`；
- 自有 target-dir：`/tmp/c1dpick`、`/tmp/c1dprec`、`/tmp/c1dp0`。

worktree 处理：
- `../java_rta_c1d_pick`：b957535c、20ef4fdc 合入集成分支后可删；c1d-serial-wip 由接手者另开 worktree。
- `../java_rta_c1d_prec`、`../java_rta_c1d_p0`：P2/P3 接手者继续用，不删。
- 已删：`java_rta_c1d_bis`、`/tmp/c1dbis`、`/tmp/c1dfix`。

## 20. P2 / P3 接手实测与结论（2026-10-02，c1d-p0 a830222d 起）

### 20.1 切除对照（HelloWorld，`rava closure --cut-file`，本机经 heavy_lock）

| 标签 | 切除 | 类 | 代码类 | 方法 | 耗时 |
|---|---|---:|---:|---:|---:|
| f0 | OOB 消息 + PTI 校验 + getGenericInterfaces + fullAddCount | 319 | 224 | 1228 | 1 s |
| g1 | OOB + PTI + getGenericInterfaces（fullAddCount 开） | 329 | 234 | 1329 | 1 s |
| g2 | OOB + PTI + fullAddCount（getGenericInterfaces 开） | 3091 | 2634 | 18728 | 588 s |
| h1 | f0 去掉 getGenericInterfaces，改切 `ClassScope.computeEnclosingScope` | 371 | 264 | 1495 | 1 s |

切除文件形如（每行 `类.方法:描述符`）：

```
jdk/internal/util/Preconditions.outOfBoundsMessage:(Ljava/lang/String;Ljava/util/List;)Ljava/lang/String;
sun/reflect/generics/reflectiveObjects/ParameterizedTypeImpl.validateConstructorArguments:()V
java/lang/Class.getGenericInterfaces:()[Ljava/lang/reflect/Type;
java/util/concurrent/ConcurrentHashMap.fullAddCount:(JZ)V
```

### 20.2 getGenericInterfaces 闸门：P2 关不掉，爆点在外层作用域查找

- **Integer 键真实可达**：`TreeBin.putTreeVal` 的 key = {String, Integer}，Integer 来自
  `ConcurrentHashMap.computeIfAbsent`（上下文 `CHM@2139:16`），即 `CoderResult$Cache` 的
  `Map<Integer, CoderResult>`（字符集解码畸形长度缓存）。整数键在同一桶碰撞 8 次以上、表长 ≥64 时会树化，
  健全分析排除不了。所以 §18.4 的「Comparable 键只有 String 时不可达」前提不成立，P2 单独关不掉这个闸门。
- **区域从哪里重开**（g2 对 g1 的首达边界 + `reflect.gaps`）：
  - getGenericInterfaces 子树本身只有约 156 个方法；
  - 真正的爆点：`Reifier` 访问 `TypeVariableSignature` → `CoreReflectionFactory.findTypeVariable` →
    `AbstractScope.lookup` → `getEnclosingScope` → `ClassScope.computeEnclosingScope` →
    `Class.getEnclosingMethod` / `getEnclosingConstructor` → `getDeclaredMethods0` / `getDeclaredConstructors0`；
  - 接收者是 `open(Class)`：签名串在抽象层未知，`makeNamedType` 走 `Class.forName(未知名)`，所得镜像未知，
    于是反射缺口 `getDeclaredMethods0 <- open(Class)`，`Method.invoke` 对全部成员放开，区域重开。
- h1 说明：只要外层作用域查找不放开未知镜像，泛型区域本身只加约 52 类（371 − 319）。
- 终态做法（P3）：签名串、类型实参名、外层类 / 外层方法都是构建期 class 文件数据，按接收者镜像逐个**具体求值**
  `getGenericInterfaces`；轨迹只含这些签名真正经过的解析 / 实化路径，`forName` 的名字是常量，不产生未知镜像。
  接收者镜像集合取自 `comparableClassFor` 里 `getClass()` 的镜像值集（闭包内有限）。

### 20.3 OOB 与 PTI 闸门的字节码事实（javap）

- **OOB**：`Preconditions.outOfBoundsMessage` 的格式串全部是常量，只用 `%s`；
  其中 `"Range [%s, %<s + %s) out of bounds for length %s"` 的 `%<s` 让 `Formatter.parse` 走
  `FORMAT_SPECIFIER_PATTERN` 正则回退（`FormatSpecifier(String, Matcher)` 构造）。
  `Formatter.parse(String)` 是静态方法：`indexOf` / `charAt` 循环，`Conversion.isValid(c)` 时
  `new FormatSpecifier(char)`，否则走正则。`FormatSpecifier.print` 按字段 `c` 分支。
  - 格式串经 `String.format(fmt, …)` → `Formatter.format(fmt, …)` → `format(l, fmt, …)` → `parse(fmt)` 透传。
  - 现状的缺口：`taint_params` 只认 `PV::Const(Str|Null)` 为干净实参；透传（实参是调用方形参、
    合流后 PV 为 Top）会把被调形参槽记为污染，形参字符串集（`pstrs.rs`）因此在透传链上失去完备性。
    终态：污染沿子集边传播——实参全部来源是字面量或**未污染**的调用方形参槽时，被调槽保持干净；
    调用方槽被污染时沿边传给后继。
- **PTI**：构造器调用 `validateConstructorArguments`，@15 `if_icmpeq` 比较
  `rawType.getTypeParameters().length` 与实参个数，错误分支 `String.format("…%s: %d formal argument(s) %d actual argument(s)", …)`，
  `%d` 引入 locale / ResourceBundle / ServiceLoader 区域。
  - 在具体求值的轨迹里，校验走成功路径（实参个数由签名决定），错误分支不进轨迹；
  - 抽象可达的 PTI 构造（若有其它入口）仍按 §18.5 第 5 项做构建期签名校验。

### 20.4 已知过近似（记录，不在本线处理）

- `EnumSet.of(e)` 经 `getDeclaringClass` → `getSuperclass`，所得 Class 值 open，`class_init` 退化为未知（C1d-b）。
- `CHM.comparableClassFor` 的 `getClass()` 结果含无所指的 `java/lang/Class` 与 `TreeBin`、
  `ReentrantLock$NonfairSync` 的镜像：`TreeBin.<init>` 读 `Node.key` 字段是全体 CHM 共用的字段节点
  （P0 = 15 类 + open(Unsafe) + open(Thread)）。具体求值按镜像逐个进行，这些镜像不实现 Comparable 时轨迹在
  `instanceof` 处即返回 null，不影响结果；精度改进归容器上下文线。

### 20.5 实施次序（取代 §18.9 中 P2 先行的安排）

1. 形参字符串集的透传完备性（污染沿子集边传播），OOB 与 Formatter 入口的前置条件。
2. `engine/concrete/`：确定性具体求值器（解释器 / 堆 / native 白名单，各 ≤600 行）。
   - 入口：调用点实参全部可枚举（字面量、未污染形参槽的字符串集、类字面量 / 有限镜像集）。
   - 轨迹上的方法入闭包，但不做抽象分析；结果对象图物化为类型流（实例化类型、字段值集、返回值集）。
   - 失败（非白名单 native、写外部静态、步数超限）整次回退抽象调用边。
   - native 白名单与内存缓存字段（镜像上的 `genericInfo` 等）在 `vm_intrinsics.toml` 新增节声明。
3. 三个闸门依次接入：`Formatter.parse`（OOB）→ 按镜像求值 `getGenericInterfaces`（含 PTI 成功路径）→ 其余 PTI 入口的签名校验。
4. P2 守卫收窄降为精度项（ResourceBundle 服务上界），不再是闸门前提。

预算：f0 = 319；泛型区域按 h1 抽象形态 +52，具体轨迹应明显更少；Formatter `%s` 约 +40。
≤360 是否可达取决于两段轨迹的实际规模，接入后实测记入本节。

### 20.6 正式口径超时的归因与污染重入修复（2026-10-02）

- 服务器 4 例（HelloWorld、TestRandomAccessFile、TestCharsetNamedStreams、DeepCopy）在 f88259df 上 transpile 10m01s 超时。
  本机正式口径（不带 `--cut-file`）HelloWorld `rava closure`：f88259df 与其父提交（代码同 c1d-prec）都在 300 s 内未完成，
  RSS 约 3 GB。这是 1e623cec 取消截断后的既有状态（§19.1、§20.1 g2：约 3091 类、588–600 s），不是透传污染引入的；
  污染重跑有上界（每槽只入 `ptaint` 一次）。正式口径回到秒级只能靠关闭三个闸门。
- 顺带修复（5904b512）：`taint_slot` 原在接边中途（`edge` → `bind_params`）同步 `rerun_site`，重入调用事件后
  `call_vals` 被置空，外层调用点余下接边跳过 `pstr_site`、丢失形参字符串集（健全性问题）。改为入站点队列。
  f0 切除形态类集逐项不变（319 / 224 / 1228）。

### 20.7 闸门之后的收尾：`#[jvm_boundary]` 归零

- 现状：1e623cec 之后 `runtime/` 中仍有 139 个 `#[jvm_boundary]`，分布在 19 个文件（Class / ClassLoader / Module /
  ModuleLayer / VirtualThread / Proxy$Dyn / FileSystems / JceSecurity 等）。
- 终态：计数为 0。
  1. 逐个方法按 `docs/reference/handwritten-boundary.md` 判定：`ACC_NATIVE` 用 `#[jvm_native]`；运行模型替换、VM 注入状态 / VM 驱动行为两类在清单中登记类别；
  2. 其余全部改为按字节码翻译，删除手写；
  3. 删除 `rava_macros` 的 `jvm_boundary` 宏与分析器中的相关解析。
- 修复中遇到的测试问题：JDK 能跑的合法 Java 测试一律不改；修复时补充覆盖边界情况的 e2e 用例，expected 取真 JDK 输出。
- 次序：排在三个闸门关闭、HelloWorld 正式口径 ≤3 s 且 ≤360 类之后。

**`vm_boundary_methods` 计数口径与 a3 范围（2026-10-02，c1d-p0 144a33a4）**

- **为什么集成分支是 30、合并后是 86：口径不同，不是新增手写。**
  - 集成分支的 `audit_override` 只审计公开 API 类（`!lang::in_public_api(n)` 时直接返回），`jdk/internal/*` 不计入。
  - c1d 在 80c5c1df 中去掉了这道过滤，审计面改为全部 JDK 类。为了保持 `non_native_overrides=0`，1e623cec / 34d5989a 把 Unsafe、VM、BootLoader、ClassLoaders 登记进了 `[vm_boundary]`。
  - 这 56 个方法的手写在集成分支上也存在，只是没有计数。两边 `java/*` 部分同为 30。
- **HelloWorld 的 86 个按类分布：**

  | 类 | 数 | 方法形态 | 准入落点（终态） |
  |---|---:|---|---|
  | `jdk/internal/misc/Unsafe` | 44 | 包在 native 外层的 Java 方法：CAS 循环、`*0` 内存访问的转发、`getUnsafe`、屏障、`objectFieldOffset` → `objectFieldOffset1` | 方法本身翻译；① 落在内层 native（`compareAndSet*` / `*0` / 屏障 native）。清单 `name_resolvers.offset`、`array_writes`、`memory_reads`、`class_initializers` 里登记的 Java 层成员改为指向内层 native（`ensureClassInitialized` → `ensureClassInitialized0`） |
  | `jdk/internal/misc/VM` | 9 | 读写 `initLevel`、`savedProps`、`javaLangInvokeInited`；`latestUserDefinedLoader` 转发 `latestUserDefinedLoader0` | 方法本身翻译；③ 落在 VM 注入的状态（`initLevel = SYSTEM_BOOTED` 等，经 `[vm_constants.injected_statics]` 声明）与内层 native ①。现手写 `getSavedProperty` 恒返回 null，属于近似 |
  | `java/lang/VirtualThread` | 10 | `start`、`run`、`park`、`unpark`、`joinNanos` 等 | 方法本身翻译；③ 落在 `Continuation` 的 VM 驱动 native（`enterSpecial` / `doYield`）与调度承载 |
  | `java/lang/ClassLoader` | 9 | `getParent`、`getSystemClassLoader`、资源查找、`getClassLoader(Class)` | 方法本身翻译；③ 落在 `Class.classLoader` 注入字段，内建加载器层级随 ClassLoaders 整类翻译 |
  | `java/lang/Module` | 7 | `isNamed`、`isExported`、`isOpen`、`canUse`、`getLayer` | 方法本身翻译；③ 落在启动模块图的注入输入。现手写是「单一未命名模块」近似 |
  | `jdk/internal/loader/ClassLoaders` | 3 | 三个内建加载器的 getter | 整类按字节码翻译（与 FS-C2 同一终态） |
  | `java/lang/Class` | 2 | `enumConstantDirectory`、`getModule` | 方法本身翻译；③ 落在 `Class.module` 注入字段 |
  | `java/lang/ModuleLayer` | 2 | `boot`、`parents` | 方法本身翻译；③ 同 Module。现手写是空层近似 |

  结论：这 86 个都不是 native，方法本身也都不属于准入三类；准入只发生在它们下层的 native（①）和 VM 注入状态（③）上。**86 个全部属于 a3 的归零范围**，终态 `vm_boundary_methods = 0`。
- **a3 的归零口径以审计计数为准，不只看属性个数。**
  - `runtime/` 里 `#[jvm_boundary]` 当前共 140 处，HelloWorld 链上命中的是上表 86 个。其余 54 处中，`InvokerBytecodeGenerator`（6）、`ClassSpecializer$Factory`、`MethodAccessorGenerator`、`Proxy$Dyn` 是运行期类定义点（② 运行模型替换），改为在清单中登记类别。
  - `JceSecurity`（6）、`BootLoader`（5）、`FileSystems`、`CDS`、`EventHelper`、`SecurityPropertyModificationEvent`、`GetInstance$Instance` 逐个方法按规范判定：可翻译的翻译，native 用 `#[jvm_native]`。
  - 完成后删除 `jvm_boundary` 宏与审计中的 `VmBoundary` 分类。`[vm_boundary]` 只保留 ③ 状态声明（注入字段 / `clinit_carried`），不再豁免方法。

### 20.8 具体求值接入后的正式口径实测与构成（2026-10-02，c1d-p0 73de95c4）

**已落地（按提交）**

- 4a3e92a6：
  - 闭世界镜像展开（`open_mirrors`）；
  - VM 注入字面量（`allowSecurityManager = 1`）；
  - 具体求值的实例入口；
  - `getGenericInterfaces` 按镜像具体求值；
  - `vm_singleton`；
  - Unsafe 具体操作（`field_offset` / `cas_reference`）；
  - `init_only` 稳定字段；
  - 映像快照。
- 7bb326d8：
  - instanceof 收窄：成立一侧的局部变量以 instanceof 偏移为来源、按目标类型过滤，与 checkcast 同口径；
  - 不变静态字段持有的映像对象按字段节点物化（`MV::Static`，如 `ClassRepository.NONE`）。
  - 修掉的两类具体求值回退：「容器形态映像对象」与「接收者 Class 不是所指已知的类镜像」。
- 73de95c4：符号字段偏移。
  - `objectFieldOffset(类字面量, 名字常量)` 折叠为 `V::Offset`，经 static final 字段传递；
  - Unsafe 引用读写调用点按偏移只触及所指字段（清单 `name_resolvers.offset` 与 `array_writes` / `memory_reads` 的 `offset`）；
  - 声明了内存效果的手写方法，其体内对其它内存操作的调用（如 `putReferenceOpaque` 体内的 `putReference`）不另建模；
  - 效果：`CHM.comparableClassFor` 的具体求值组合 3 → 2，`open(Thread)` / `open(Unsafe)` 不再污染 `Node.key`。

**正式口径（不带 `--cut-file`）**

- HelloWorld `rava closure`：423 类 / 305 代码类 / 1722 方法，2–3 s；
- 全部具体求值成功、无回退：

  | 调用点 | 组合 |
  |---|---:|
  | `Formatter.<clinit>@7` | 1 |
  | `Formatter.format@11` | 7 |
  | `HashMap.comparableClassFor@21` | 1 |
  | `CHM.comparableClassFor@21` | 2 |

**切除对照（同一提交）**

| 切除 | 类 | 代码类 |
|---|---:|---:|
| 无（正式口径） | 423 | 305 |
| OOB 消息（`outOfBoundsMessage`） | 371 | 265 |
| `fullAddCount` | 415 | 297 |
| `getGenericInterfaces` / PTI 校验（各自单切） | 423 | 305 |
| OOB + `fullAddCount` | 362 | 256 |
| 四项全切（f0） | 362 | 256 |

- **GGI / PTI 闸门已关**：单切不变，与 OOB + `fullAddCount` 同切也不再减少——泛型区域只剩具体求值轨迹。
- **OOB（+52～61）是调用链上的真实可达，不是过近似**：
  - 用户代码 `StringBuilder.append(String)` → `inflateIfNeededFor` → `inflate` → `StringLatin1.inflate` →
    `StringUTF16.checkBoundsOffCount` → `Preconditions` 出错分支 → `String.format`；
  - 该链带入 Formatter、`FORMAT_SPECIFIER_PATTERN` 正则图、Locale / DecimalFormatSymbols 等；
  - 要排除需证明 `dstOff + len ≤ dst.length >> 1`（`newBytesFor(value.length)` 与 `count` 的关系），属关系型数值推理；
    或证明全程序 `String.coder` 恒为 LATIN1（构造器按运行期压缩结果写入），都不在本线能力内。
  - 另一条入口 `CHM.toString`（`registerNatives` 手写体 `format!("{}", v)` 的 Object Display 分派、
    `reflect_dispatch.unbox_*` 的 `__obj_str`）切掉只少 2 类，不改变结论。
- **`fullAddCount`（+6～8）**：`addCount` 的 CAS 失败分支；单线程不会失败，但分析不建模线程，属健全保守。
- **下限**：两项同切仍是 362（> 360）。§20.1 的 319 是 a830222d 时的口径，其后加入的健全性修复与具体求值轨迹
  （Formatter 正则图、getGenericInterfaces 解析 / 实化路径）不随切除消失。

**结论**：≤360 在现有分析能力下不可达。正式口径 423 / 305、≤3 s；超出部分由 OOB 真实调用链（约 52）与
CAS 竞争分支（约 8）构成。

**后续项 a5（C1d-a，不挂起）**

1. **OOB：`checkBoundsOffCount` 出错分支，采用关系型边界推理。**
   - 两条候选：
     - **关系型边界推理**：证明 `dstOff + len ≤ dst.length >> 1`。
     - **全程序 `String.coder` 分析**：证明 `coder` 恒为 LATIN1，`inflate` 不可达。
   - 取舍：

     | 维度 | 关系型边界推理 | 全程序 `String.coder` 分析 |
     |---|---|---|
     | 覆盖面 | 所有 JDK 内部「下标 / 偏移由结构保证」的检查点：`Preconditions.check*`、`checkBoundsOffCount`、`Arrays.copyOfRange`、`System.arraycopy` 前置、Buffer 索引等；每个被证伪的出错分支都切掉同一片 `String.format` / Formatter / Locale 区域 | 只有 `inflate` 一类入口 |
     | 稳定性 | 与程序处理哪些字符无关 | 只要程序中出现非 Latin-1 字符的来源（任何类中的字面量、`char` 运算、IO 解码、`Character` 转换），或 VM 注入 `COMPACT_STRINGS = false`，结论就失效。只对玩具程序成立，真实程序几乎全部退回 |
     | 代价 | 需要数值域与堆不变量 | 只需一个字段的值集 |

   - 结论：采用关系型边界推理。
   - 做法，三层：
     1. **方法内差分约束 / 八边形域。**
        - 变量：局部变量、`arraylength`、移位 / 加减常数。
        - 接在 absint 条件边上（与 P2 守卫收窄同一入口）。
        - `newBytesFor(n)` 的返回值携带 `length == n << 1`，按方法摘要（返回值与形参的关系）跨调用传递。
     2. **类不变量。**
        - 形如 `this.count ≤ this.value.length` 的字段间关系。
        - 由全部写点（构造器与所有 `putfield count` / `putfield value`）归纳验证：每个写点在其路径约束下保持不变量才成立；任何一处不成立则不变量不成立。
        - 候选不变量由检查点的前置条件反推生成，不靠清单。
     3. **检查点判定。**
        - `Preconditions` 系列与 `checkBoundsOffCount` 的出错分支：在调用点约束下若不可满足，该分支不入轨迹。
        - 判定按调用点进行，不改被调方法的抽象摘要。
   - 健全性：数值域只在整数不溢出的前提下使用。`newBytesFor` 自带溢出检查分支，溢出路径照常可达。
   - 用户下标（如 `list.get(i)`，`i` 来自输入）证不出时保持可达，属于正确行为。
   - 目标：HelloWorld 正式口径 ≤371 类（§20.8 切除 OOB 的实测值）。
   - 再往下的目标在 a2（1e623cec 合入后的形态）实测之后定（a2 已合入 62f46bb2，现状见 §22.9）。

2. **`fullAddCount`：CAS 竞争分支，只记录，不实施。**
   - 属于线程模型范围：单线程下 CAS 不会失败，但只有分析能证明「该 CHM 实例在发布前 / 只被单线程触及」时，才能剪掉这条分支。这归线程逃逸分析，不在 C1d 范围内。
   - 约 +8 类，计入健全保守。

**合入集成分支后的实测（c1d-p0 144a33a4 = 合入 be1b97be）**

- 生成器单测全过。
- HelloWorld `--stop-after emit`：424 个 JDK 类 + 1 个用户类，`non_native_overrides=0`，`vm_boundary_methods=86`。
- 冲突解法：vm_boundary 类的 `<clinit>` 缺省按字节码翻译，手写承载的改为在 `clinit_carried` 中正向登记，取代原来的 `translate_clinit`；`ClassLoader$ParallelLoaders` 加入 `translate_nested`。
- 1e623cec 早已在 c1d-p0 的祖先中，a2 无需另行合入。
- FS-C2（ClassLoaders 整类按字节码翻译）与本分支冲突时，后合入的一方按「整类翻译、不进 vm_boundary」解冲突。

## 21. a3–a5 拆分（2026-10-02，c1d-p0 5c6dd98f）

供多个子代理并行接手。每项可独立验收，验收以 HelloWorld `--stop-after emit` 的 `[raw-audit] vm_boundary_methods`
与 `[vm-boundary-audit]` 明细为准（下文「降幅」均指该计数），外加所列 e2e（服务器跑，expected 取 JDK 21）。

### 21.0 现状底数（合入 28090062 后实测）

HelloWorld `vm_boundary_methods = 86`，构成与 §20.7 表不同（FS-C2 合入后的变化）：

| 类 | 数 | 归属 |
|---|---:|---|
| `jdk/internal/misc/Unsafe` | 44 | a3-U1 / U2 / U3 |
| `jdk/internal/misc/VM` | 10 | a3-V（FS-C2 新增 `initLevel`） |
| `java/lang/VirtualThread` | 10 | a3-T |
| `java/lang/ClassLoader` | 6 | a3-L2（只剩 6 个资源查找方法） |
| `jdk/internal/loader/BootLoader` | 5 | a3-L1 |
| `java/lang/Class` | 2 | `enumConstantDirectory` → a3-C；`getModule` → boot layer |
| `java/lang/Module` + `ModuleLayer` | 9 | boot layer（`2026-10-02-boot-layer.md` §2.3），不在本拆分内 |

- **ClassLoaders 3 个已经清零**：FS-C2（4a98f5e3）把 ClassLoaders 整类改为字节码翻译，`class_loaders_impl.rs` 已删。
  FS-C2 §4.2 交给 C1d-a 的两条（`ServicesCatalog.getServicesCatalogOrNull` 过渡手写、JLA 手写里的
  `createOrGetClassLoaderValueMap`）在 c1d-p0 中已随 1e623cec 删除（`runtime/` 中不存在），不需要另立子任务。
  boot layer 计划 §1 表里的 `services_catalog_impl.rs` 也已不存在，接手时以代码为准。
- **本拆分的范围是 86 − 9（Module / ModuleLayer）− 1（`Class.getModule`）= 76**，加上 HelloWorld 链外的 37 处
  `#[jvm_boundary]`（§21.3），终态是 `vm_boundary_methods = 0`、`runtime/` 中 `#[jvm_boundary]` 为 0、宏删除。
- `runtime/` 中 `#[jvm_boundary]` 共 135 处：`unsafe__impl.rs` 75、`virtual_thread_impl.rs` 10、`vm_impl.rs` 8、
  `module_impl.rs` 7、`jce_security_impl.rs` 6、`invoker_bytecode_generator_impl.rs` 6、`class_loader_impl.rs` 6、
  `boot_loader_impl.rs` 5、`module_layer_impl.rs` 2、`class_impl.rs` 2，其余 8 个文件各 1。
  `unsafe__impl.rs` 的 75 处里约 31 个方法在 JDK 21 中本身就是 `ACC_NATIVE`（`getReference` / `compareAndSetInt` /
  `park` / `allocateInstance` …），只是属性标错，不计入审计。

### 21.1 子任务表

「做法」三选一：**译**（删手写、按字节码翻译）、**① native**（`#[jvm_native]`，实现与 JNI 语义等价）、
**③ 登记**（VM 注入状态 / VM 驱动行为，在清单登记类别，手写体落到状态访问点而不是 Java 方法）。

| 编号 | 方法（HelloWorld 审计） | 文件 | 做法 | 依赖 / 冲突 | 降幅 |
|---|---|---|---|---|---:|
| **a3-U0** | `unsafe__impl.rs` 中 JDK 为 `ACC_NATIVE` 的约 31 个方法 | `unsafe__impl.rs` | 属性改为 `#[jvm_native]`；不改方法体 | 无依赖；**结构性改动，最先单独合入**，U1–U3 在其后开工 | 0（只改标注） |
| **a3-U1** | 裸地址内存 15：`allocateMemory` / `reallocateMemory` / `freeMemory` / `setMemory`×2 / `copyMemory`×2 / `copySwapMemory`×2 / `getByte(J)` / `putByte(JB)` / `getInt(J)` / `putInt(JI)` / `getLong(J)` / `putLong(JJ)` | `unsafe__impl.rs` 内存段 | 译；① 落在内层 `allocateMemory0` / `reallocateMemory0` / `freeMemory0` / `setMemory0` / `copyMemory0` / `copySwapMemory0` 与 `getByte(Object,long)` 等 native | ◀ U0。与 U2 / U3 同文件不同段，文本冲突在合并时就地解决 | −15 |
| **a3-U2** | 原子与内存序 20：`getAndAdd{Int,Long}` / `getAndBitwise{And,Or}Int` / `getAndBitwiseOrLong` / `getAndSet{Int,Reference}` / `get{Int,Reference}{Acquire,Opaque}` / `put{Int,Reference}{Opaque,Release}` / `weakCompareAndSet{Int,Reference}` / `loadFence` / `storeFence` / `storeStoreFence` | `unsafe__impl.rs` 原子段；`vm_intrinsics.toml` 的 `array_writes` / `memory_reads`；`closure/src/engine/hw_mem.rs`、`concrete/`（`cas_reference`） | 译；① 落在 `compareAndSet*` / `getReferenceVolatile` / `putReferenceVolatile` / 屏障 native。清单中按 Java 层成员登记的内存效果改指内层 native，分析器经字节码走到内层 | ◀ U0。与 U3 同改 `vm_intrinsics.toml`（不同节） | −20 |
| **a3-U3** | 布局 / 反射 / 类初始化 9：`getUnsafe` / `objectFieldOffset`×2 / `staticFieldOffset` / `staticFieldBase` / `arrayIndexScale` / `ensureClassInitialized` / `shouldBeInitialized` / `allocateUninitializedArray`；另 `Unsafe` 移出 `clinit_carried` 与 `[vm_boundary].classes` | `unsafe__impl.rs` 布局段；`closure.toml`；`vm_intrinsics.toml` 的 `name_resolvers.offset`、`class_initializers`、`[vm_constants.injected_statics]`；`closure/src/engine/hw_offset.rs` | 译；① 落在 `objectFieldOffset1` / `staticFieldOffset0` / `staticFieldBase0` / `arrayIndexScale0` / `ensureClassInitialized0` / `shouldBeInitialized0`；`<clinit>` 的布局常量已由 §14.4 注入，`<clinit>` 改为按字节码翻译。符号偏移折叠（73de95c4）的识别点改到 `objectFieldOffset1` | ◀ U0；U1 / U2 合入后才能移出 `[vm_boundary]`（该步放 U3 最后一个提交） | −9 |
| **a3-V** | VM 10：`initLevel` / `isBooted` / `isModuleSystemInited` / `isJavaLangInvokeInited` / `setJavaLangInvokeInited` / `shutdown` / `isShutdown` / `getSavedProperty` / `isSystemDomainLoader` / `latestUserDefinedLoader` | `vm_impl.rs`；`system_impl.rs`；`class_loader_impl.rs`（`__vm_init_phase3`）；`vm_intrinsics.toml` 的 `[facts.returns]`、`[vm_constants]`（`getSavedProperty` 恒 null 条目删除）、`[vm_state.field_hooks]` | 译全部 10 个。③ 落在字段：`VM.initLevel:I` 加读取钩子（阶段内报该阶段档位，其余时间为 4；写入经 `VM.initLevel(int)` 字节码，`SYSTEM_SHUTDOWN` 写入即停机标记），取代 0c4e47c6 的线程内档位覆盖与 `__vm_at_init_level`；`javaLangInvokeInited` 经 `injected_statics` 注入 true。`getSavedProperty` 改读 initPhase1 保存的快照（真值，不再恒 null）。`latestUserDefinedLoader` → ① `latestUserDefinedLoader0`（栈帧来源同 `getCallerClass`） | 与 boot layer 冲突：同改 `vm_impl.rs` / `system_impl.rs` / `class_loader_impl.rs` 与 `[boot_init]`。**排在 boot layer 第 1 步合入之后**，或与 boot layer 同一会话串行 | −10 |
| **a3-T** | VirtualThread 10：`<init>` / `alive` / `isTerminated` / `joinNanos` / `park` / `parkNanos` / `run` / `start`×2 / `unpark`；另移出 `[vm_boundary].classes` 与 `clinit_carried` | `virtual_thread_impl.rs`；`jdk/internal/vm/continuation_impl.rs`、`continuation_support_impl.rs`；`thread_impl.rs`；`monitor.rs`；新 crate `runtime/rava_coro/`；`closure.toml` | 译 VirtualThread 与调度器（`ForkJoinPool` / `CarrierThread` / `UNPARKER`）；③ 只落在 `Continuation` 的 VM 方法（`enterSpecial` / `doYield` / `pin` / `unpin` / `isPinned0`），实现为有栈协程（2026-10-03 用户定终态，方案 A 作废）。细分 T1–T6 见 §21.8 | T1 → T2 → T4；T1b ◀ T1、在 T4 前合入；T3 与 T2 并行、在 T4 前合入；T5 ◀ T1；T6 收尾。与 a3-V 无文件冲突 | −10 |
| **a3-L1** | BootLoader 5：`getServicesCatalog` / `hasClassPath` / `loadClass(Module,String)` / `loadClassOrNull` / `loadLibrary`；另移出 `[vm_boundary].classes` | `boot_loader_impl.rs`（删除）；`closure.toml`；新 native：`NativeLibraries.findBuiltinLib` / `load` / `unload`、`BootLoader.getSystemPackageLocation`、`NativeImageBuffer.getNativeMap`、`ClassLoader.findBootstrapClass` | 译；① 落在上列 native：内建库按静态链接处理（`findBuiltinLib` 对内建库名返回库名，`load` 对内建库成功），引导类查找落到类宇宙表，jimage 资源按运行时镜像读取。`getServicesCatalog` 读 `SERVICES_CATALOG`，由 boot layer 的 `initServices` 填充 | `getServicesCatalog` ◀ boot layer 第 1 步（服务目录）；其余 4 个无依赖，可先做。与 a3-L2 冲突在 `ClassLoader` 对 `BootLoader` 的调用面 | −5 |
| **a3-L2** | ClassLoader 6：`getResource` / `getResourceAsStream` / `getResources` / `getSystemResource` / `getSystemResourceAsStream` / `getSystemResources` | `class_loader_impl.rs`（只删这 6 个与对应 `__impl_*`；`__vm_init_phase3` 归 a3-V） | 译；路径为 `parent` → `BootLoader.findResource` → 内建加载器（FS-C2 已整类翻译）→ `URLClassPath`。无新增手写 | ◀ a3-L1。与 a3-V 同文件不同函数 | −6 |
| **a3-C** | `Class.enumConstantDirectory` | `class_impl.rs` | 译（走 `getEnumConstantsShared` → 反射调用 `values()`，反射元数据表已有） | 与 boot layer 同文件（boot layer 删 `getModule` 两份手写）：不同函数，合并时就地解决 | −1 |

### 21.2 各项验收用例（≤10）

| 编号 | 用例 |
|---|---|
| a3-U0 | HelloWorld、TestAtomics、TestDirectBuffer（只验证「行为不变 + 审计数不变」） |
| a3-U1 | TestDirectBuffer、TestByteArrayViewVarHandle、TestDefineClassRejects、TestUnixFileNatives、TestRandomAccessFile、HelloWorld；补边界用例：`ByteBuffer.allocateDirect` 的 `putLong` / `getLong` 跨页、`order(LITTLE_ENDIAN)` 交换拷贝 |
| a3-U2 | TestAtomics、AtomicDemo、TestChmTransfer、TestParallelArrayCas、TestVolatilePrimitiveAccess、TestJucSync、TestCommonPool、TestCompletableFuture、HelloWorld（闭包类数不得上升） |
| a3-U3 | HelloWorld（闭包类数与 `CHM.comparableClassFor` 具体求值组合数不得上升）、TestReflectProbe、TestConcurrentClinit、TestForNameInit、TestByteArrayViewVarHandle、DeepCopy、TestSerialDefaultSuid |
| a3-V | HelloWorld、TestServiceLoaderLayers、TestShutdownHooks、TestSystemExitEnv、TestSystemPropsSpec、TestAppClassLoader、TestDirectBuffer（`maxDirectMemory` 经 `getSavedProperty` 取值）、TestMethodHandleDirect（`isJavaLangInvokeInited`）；补边界用例：`-XX:MaxDirectMemorySize` 缺省时 `VM.maxDirectMemory()` 等于 `Runtime.maxMemory()` 的可观察面 |
| a3-T | 分子任务列在 §21.8.4（合计不超过 10 个 e2e：TestVirtualThread、TestVirtualClockPark、TestThreadStates、TestThreadInterrupt、TestSleepParkClock、TestCommonPool、TestSynchronized，加新增边界用例 3 个） |
| a3-L1 | TestAppClassLoader、TestServiceLoaderLayers、TestClassForName、TestCharsetNamedStreams、TestNetworkInterface（`loadLibrary("net")`）、TestSecureRandomApi |
| a3-L2 | TestAppClassLoader、TestServiceLoaderLayers、TestParallelCapable、TestCharsetForName；补边界用例：`getSystemResource` 查不到返回 null、`getResources` 父子加载器都命中时的枚举次序（expected 取 JDK 21） |
| a3-C | TestEnumBasic、TestEnumAdvanced、TestEnumSetMap、SwitchExpressions、TestIntegerCacheSpec |

### 21.3 HelloWorld 链外的 `#[jvm_boundary]`（不计入 86，终态同样归零）

| 编号 | 方法 | 文件 | 做法 | 验收 |
|---|---|---|---|---|
| **a3-X1** | 运行期类定义点：`InvokerBytecodeGenerator` 6（`generateCustomizedCode` / `generateLambdaFormInterpreterEntryPoint` / `lookupPregenerated` / `isStaticallyInvocable`×3）、`MethodAccessorGenerator.generateSerializationConstructor`、`ClassSpecializer$Factory.generateConcreteSpeciesCode`、`Proxy$Dyn` 的 VM 钩子 | 各自 `_impl.rs`；`vm_intrinsics.toml` 的运行模型替换节 | ② 运行模型替换：在清单登记类别，属性去掉；`isStaticallyInvocable` / `lookupPregenerated` 不定义类，逐个判定能否译（能译则译） | TestMethodHandleCombinators、TestMethodHandleDirect、TestBmhDynamicSpecies、TestDynamicProxy、DeepCopy、TestSerialProxyForm |
| **a3-X2** | `JceSecurity` 6（`canUseProvider` / `isRestricted` / `getVerificationResult` / `getDefaultPolicy` / `getExemptPolicy` / `verifyExemptJar`）、`FileSystems.getDefault`、`CDS.initializeFromArchive`、`EventHelper.isLoggingSecurity`、`SecurityPropertyModificationEvent.<init>`、`GetInstance$Instance.toArray` | 各自 `_impl.rs`；`closure.toml`（`JceSecurity` / `FileSystems` / `InetAddress` / `SecurityManager` 移出 `[vm_boundary].classes` 与 `clinit_carried`） | 逐方法判定：可译则译（JCE 策略文件按运行时镜像 `conf/security/policy` 读取，签名校验按 JDK 自带 provider 走字节码）；`CDS.initializeFromArchive` 为 native → `#[jvm_native]`（无归档，no-op） | DataEncryptionStandard、TestCipherAlgorithmParameters、TestCipherDesModes、TestMessageDigestApi、TestSecureRandomApi、SecurityDemo、TestUnixFileNatives |
| **a3-Z** | 收尾：删 `rava_macros::jvm_boundary`、`emit` 审计的 `HwAudit::VmBoundary` 分类、`closure/src/handwritten/hooks.rs` 中的属性解析；`[vm_boundary]` 只保留 ③ 状态声明（`classes` 清单删除或改名为状态声明） | `runtime/rava_macros/src/lib.rs`、`generator/crates/emit/src/{audit.rs,class_writer/methods.rs,ctx.rs}`、`generator/crates/input/src/boundary.rs`、`closure.toml` | — | ◀ 全部 a3 子任务与 boot layer 第 3 步；验收：生成器单测、HelloWorld 审计行不再含 `vm_boundary_methods`、9 例抽查 |

### 21.4 a4

a4（TestCharsetNamedStreams，自 c4-regfix 移交）已由 b124e5ac 修复（`ModuleLayer` 移出 `clinit_carried`，CLV 非 null），
本机 5c6dd98f 上 emit + 编译 + 运行与 expected 一致。只需服务器抽查确认，不再拆分。

### 21.5 a5：OOB 关系型边界推理（§20.8 后续项 1）

三步串行，前一步的单元测试是后一步的前提。目标 HelloWorld 正式口径 ≤371 类。

| 编号 | 内容 | 文件 | 依赖 / 冲突 | 验收 |
|---|---|---|---|---|
| **a5-1** | 方法内差分约束 / 八边形域：变量取局部变量、`arraylength`、移位与加减常数；接在 absint 条件边上；方法摘要携带「返回值与形参的关系」（`newBytesFor(n)` → `length == n << 1`），跨调用传递 | 新模块 `generator/crates/closure/src/absint/relational/`（每文件 ≤600 行）；`absint.rs` 条件边入口 | 与 C1d-b 的 P2 守卫收窄同一入口（`absint` 条件边），开工前先同步集成分支 | 单元测试：`newBytesFor` 摘要、`checkBoundsOffCount` 在 `inflate` 调用点约束下不可满足；整数溢出路径保持可达 |
| **a5-2** | 类不变量：候选由检查点前置条件反推（如 `this.count ≤ this.value.length`），由全部写点（构造器与所有 `putfield count` / `putfield value`）在路径约束下归纳验证，任一写点不保持即不成立 | `absint/relational/invariants.rs`；`engine/write_audit.rs`（写点来源） | ◀ a5-1 | 单元测试：`AbstractStringBuilder` 的 `count` / `value` 不变量成立；人为构造一处破坏写点时不成立 |
| **a5-3** | 检查点判定：`Preconditions.check*` / `checkBoundsOffCount` 出错分支在调用点约束下不可满足则不入轨迹；按调用点判定，不改被调方法的抽象摘要 | `engine/invoke.rs`、`engine/flow.rs` 的调用点接边 | ◀ a5-1、a5-2 | HelloWorld 正式口径 ≤371 类、≤3 s；TestStringBuilder、TestStringBuilderOps、StringBuilderCharTest 通过；补边界用例：下标来自输入的 `sb.charAt(i)` 越界时异常消息与 JDK 一致（出错分支仍可达）；9 例抽查 |

`fullAddCount`（CAS 竞争分支，约 +8 类）属线程逃逸分析，只记录不实施（§20.8 后续项 2）。

**a5-4（精度待查，2026-10-03 登记）**：TestUnixFileNatives 闭包偏大——服务器 transpile 3m33s、二进制 391M（c1d-p0 b17a6496 前后的 a2 抽查）。该例只调文件系统 API，量级应与 HelloWorld + `sun.nio.fs` 相当；先用 `rava closure --why` / `--flows '@trace:<类>'` 找引入面最大的入口，再定收窄手段。目标：transpile ≤60 s、类数的引入链逐条可解释。
  - 旁证（2026-10-03，Linux 目标 JDK 21.0.12 jmods 实测 `rava emit --full-precheck`，闭包 2961 类）：文件系统 native 补全后（UnixNativeDispatcher 49 个全承载、FileDispatcherImpl / UnixFileDispatcherImpl 的 transfer / map 补齐），闭包内仍缺 18 个 native，全部与文件 API 无关，是膨胀的指纹：`sun.security.pkcs11.Secmod` nss* 5 个、`sun.security.pkcs11.wrapper.PKCS11` 5 个、`sun.security.smartcardio.PCSC` / `PlatformPCSC` 2 个、`jdk.internal.jimage.NativeImageBuffer.getNativeMap`、`jdk.internal.loader.NativeLibraries` findBuiltinLib / load / unload、`BootLoader.getSystemPackageLocation`、`ClassLoader.defineClass0`。其中 `NativeLibraries` 3 个、`getSystemPackageLocation`、`getNativeMap` 共 5 个是 a3-L1 计划补的 ① native（BootLoader 译后真实需要），不算膨胀；收窄判据之一：其余 13 个（pkcs11 10、smartcardio 2、`defineClass0` 1）从闭包消失、不补手写（`defineClass0` 若查明真实可达，归 a3-X1 运行期类定义点处理）；macOS 目标另有 `KeychainStore._scanKeychain`、`HostLocaleProviderAdapterImpl.getDefaultLocale` 两个同类指纹。

  - 引入链归因（2026-10-03，TestFileStoreMountLookup，macOS 宿主 JDK 21，c1d-p0 合入 b55ef981 后 `rava closure -o` 实测 2885 类；服务器 r4 dyn-compare 报 extra 2156，二进制 392M、transpile 2m06 + build 9m07）。下列计数为 via 链经过该节点的类数（含重叠），via 是首次发现链、不是唯一来源：
    - **a5-4a `doPrivileged` 动作合流**（经过 `AccessibleObject.<clinit>` 1092 类）：`AccessibleObject.<clinit>` 只取 `ReflectionFactory`，但 `AccessController.executePrivileged` 里 `action.run()` 的接收者值集按全程序合流，首次发现时把 `Charset$ExtendedProviderHolder$1.run`（742）挂到这条链上。收窄手段：`executePrivileged` 按调用点求接收者（上下文敏感一层），或把 `doPrivileged` 系列登记为「参数回调」由调用点直接接 `run` 边。验收：`AccessibleObject.<clinit>` 子树不再含 `Charset` / `ServiceLoader`。
    - **a5-4b 引导加载器的类路径查找**（经过 `ServiceLoader$LazyClassPathLookupIterator` 684、`BuiltinClassLoader.findMiscResource` 669、`JarFile.initializeVerifier` 435）：`BootLoader.findResources` → `findMiscResource` → `URLClassPath$JarLoader` → `JarURLConnection` → `JarVerifier` → `SignatureFileVerifier` → `PKCS7.verify` → `Signature.getInstance`，引入 XMLDSig（`com/sun/org` 113、`org/jcp`）、`sun/security/ec` 84、x509 / provider / jca、`sun/security/pkcs11` 26（Linux 目标的 pkcs11 / smartcardio 缺失 native 指纹即来自此处）。事实：未设 `-Xbootclasspath/a`（`jdk.boot.class.path.append` 为空）时 `ClassLoaders.BOOT_LOADER` 的 `ucp` 为 null，`findMiscResource` 的类路径分支不可达。收窄手段：具体求值器把 `jdk.boot.class.path.append` 按引导期属性求值为空，`BootLoader` 的 `ucp` 静态值为 null 后条件边剪掉。验收：TestFileStoreMountLookup / TestUnixFileNatives 闭包不含 `java/util/jar/JarVerifier`、`sun/security/pkcs11`、`sun/security/smartcardio`。
    - **a5-4c jrt 协议**（`sun/net/www/protocol/jrt` 47：jimage 32、jrtfs 16）：同在 a5-4b 的 `JarLoader.getJarFile` → `URL.openConnection` 之下，a5-4b 剪掉后随之消失，不单独处理。
    - **a5-4d `Formatter` → `Pattern` → ICU 归一化**（`java/util/Formatter` 139，其中 `jdk/internal/icu` 65）：`System.<clinit>` → `ConcurrentHashMap.toString` → `StringBuilder.append` → `inflate` → `checkBoundsOffCount` 出错分支 → `Preconditions.outOfBoundsMessage` → `String.format`，即 a5-1..a5-3 的 OOB 检查点；不另立手段，a5-3 验收时同时核对该例 `Formatter` 子树消失。另 `Pattern.compile` → `normalize` 的 `CANON_EQ` 分支在常量 flags 下不可达，可由具体求值器按常量参数剪枝，归 a5-3 一并做。
    - 其余大包（`java/lang/invoke` 123、`sun/nio/cs` 108、`java/util/stream` 93、`java/util/concurrent` 87、`java/util/regex` 61、locale 56、logging 28）多数挂在上述子树下，a5-4a/b 完成后重测再定是否另立项。
    - 与集成分支对照（2026-10-03，同机 macOS，集成分支 369a5392 的 rava 对 c1d-p0 b4f053e8 之后）：TestFileStoreMountLookup 1611 → 2912（+1317），TestDateTimeFormat 1476 → 2910（+1444）。增量就是上面几条扇出：集成分支仍有 `[boundary]` 前缀截断，`sun/`、`jdk/` 下的链在截断点停住；c1d-p0 去截断后，a5-4a / a5-4b 的过近似链全部展开。主要类别：JarVerifier 子树约 400、`com/sun/org` 113、`sun/security/ec` 84、`jdk/internal/icu` 60、jimage 32、logging 28。服务器 dyn-compare 报的 extra 2156 / 2254 以 JDK 实际装载为基准，口径不同，构成相同。去截断还暴露了一处真实缺口：`LocaleData` 改为按字节码取束后，要用 JRE（FALLBACK）族的束，已在 seeds.toml 按访问器补齐（d1b1b2ba，DTF +25 类，属必需）。
    - 目标：TestFileStoreMountLookup 闭包 ≤900 类、transpile ≤60 s；a5-4a / a5-4b 各补一个边界 e2e（`doPrivileged` 嵌套不同动作；`ServiceLoader.load` 在无 `-Xbootclasspath/a` 下的服务查找结果与 JDK 一致）。
    - DeepCopy 实测与回收估算（2026-10-03，同机 macOS，`rava closure`；c1d-p0 00f4a379 对集成分支 cc5bb175）：
      - 闭包 3139 类、分析 3m03s；集成分支 1820 类、23 s；多 1329、少 10。JDK 实际装载 959 类：c1d-p0 覆盖 814，集成分支覆盖 710。目标 ≤1640，需净减约 1500。
      - 多出的 1329 类按首次发现链归类（依次匹配，先命中先算）：

        | 来源 | 类数 |
        |---|---|
        | a5-4b 引导类路径查找 / JarVerifier | 401 |
        | ICU 归一化（`SocketPermission.init` → `String.toLowerCase` → `ConditionalSpecialCasing` → `Normalizer2`） | 248 |
        | LoggerFinder / 日志后端探测（`ObjectInputFilter$Config.<clinit>` → `System.getLogger`） | 231 |
        | `Formatter` | 58 |
        | `ObjectStreamClass` SUID / Proxy | 39 |
        | 其他（`sun/nio/cs` 40、`sun/util/resources` 21、`sun/reflect/generics/tree` 16 等） | 342 |

      - **a5-4a 单独回收约 0 类。** 闭包里规模最大的 12 个特权动作，其分配点都在自身的合法路径上，而非经 `executePrivileged` 合流而来：`ICUBinary$1` 在 `getRequiredData`，`DetectBackend$1` 在 `<clinit>`，`ObjectStreamClass$1` 在 `getSerialVersionUID`，其余同理。RTA 下这些动作已实例化，`run` 照样可达；按调用点求接收者只会改变 via 归属，不改变类集合。前文「1092 类」是首次发现链的归属数，不是可回收数。a5-4a 降为精度 / 可解释性项，不计入收窄预算。
      - **a5-4b 回收上界 401，下界约 160。** 上界是多出类中首次发现链经 `findMiscResource` / `JarLoader` / `initializeVerifier` / `LazyClassPathLookupIterator` 的部分；全闭包子树为 537，其中 136 类集成分支也有。下界是子树中所属包在子树外不出现的类，共 159，主要是 `sun/security/ec*`、XMLDSig、`sun/security/x509`、`java/util/zip`、jimage 解压器。JarVerifier、ec、XMLDSig 没有别的入口，预计接近上界。a5-4b 完成后约 2740 类，仍高于 1640。
      - **达到 ≤1640 还需另立两项：**
        - a5-4e：ICU 归一化入口。`StringLatin1.toLowerCase` 仅在语言为 tr / az / lt 时走 `toLowerCaseEx`；JDK 对该例装载 `jdk/internal/icu` 0 类，c1d-p0 为 83 类。
        - a5-4f：日志后端探测。JDK 装载 `jdk/internal/logger` 17 类、`java/util/logging` 0 类；c1d-p0 分别为 30 类、28 类。
        - 两项合计上界约 480。加上其他 342 中随 a5-4b、a5-4e、a5-4f 消失的部分，才可能接近 1640。各项完成后按本例重测再定。
    - **c1d-p0 合入门槛（2026-10-03 用户决策：先收窄再合）**：DeepCopy、TestDateTimeFormat、TestFileStoreMountLookup 三例的闭包类数与分析时间均不高于当时集成分支（DeepCopy 约 1820 类 / 23 s；DTF 1476、FSML 1611 类，同机同口径）；终态目标不变（DeepCopy ≤1640、FSML ≤900）。实施顺序 a5-4b → a5-4e → a5-4f，不足再从余下 342 类中找源；每步小步提交、开新步前同步集成分支，达标后推送抽查。
    - **逐步实测**（同机 macOS，`rava closure --jdk 21`，时间扣除全机锁等待）：

      | 步骤 | DeepCopy | DTF | FSML |
      |---|---|---|---|
      | 集成分支（门槛） | 1820 / 23 s | 1476 | 1611 |
      | c1d-p0 9acf7bc9 | 3139 / 3m03s | 2910 | 2912 |
      | s1 构造器查找只在 Class 值集齐全时点名 | 3069 / 66 s | 2910 / 48 s | 2912 / 46 s |
      | s2 instanceof 否定分支收窄 + 钩子字段不按 open | 3065 / 75 s | 2884 / 53 s | 2886 / 52 s |

      - s1：22eb9e72 的构造器查找把值集里的全部镜像点名。DeepCopy 的序列化路径（`ObjectStreamClass.getExternalizableConstructor@5`、`canonicalRecordCtr`、`ReflectionFactory.newConstructorForSerialization@46`）上，Class 值集经流不敏感合流（`Objects.requireNonNull` 返回值等）带入约 2340 个镜像并含 open，结果暴露 3498 个 JDK 构造器（集成分支 8 个）。改为值集齐全才点名；不齐全时同构造器枚举，只给用户类分派臂，查找点记反射缺口。暴露构造器降为 105，DeepCopy 少 70 类，分析时间 3m03s → 66 s；DTF、FSML 不受影响。补边界用例 TestCtorLookupRuntimeClass（经容器 / Object 返回值 / lambda 流转后 `getClass().getDeclaredConstructor()`，JDK 类与用户类混合，值集齐全照常点名）。
      - s2（a5-4b 第一步）：instanceof 的否定分支建模。分支站点以 if 指令偏移为节点，接「非该类型子类型」过滤边（`NOT_SUB` 过滤位：去掉是子类型的类，open(o) 只在 o 是子类型时去掉）。原先否定分支沿用原值，`Class.getResourceAsStream@44` 的 `!(cl instanceof BuiltinClassLoader)` 分支照样派发到 `BuiltinClassLoader.findResource`。另一处是 `Class.classLoader`：它登记了字段钩子（`__vm_defining_loader` 按定义加载器精确给出 App / Platform 加载器），但 Class 是边界类，读站点仍被加了 open(ClassLoader)。改为钩子字段不按 open 处理。DeepCopy 只少 4 类，DTF、FSML 各少 26 类。补边界用例 TestInstanceofElseDispatch（类 / 接口过滤、子类在否定侧排除、经父类实现接口、null 落入否定侧、链式 instanceof）。
      - s2 之后重排 DeepCopy 的首次发现子树（`/tmp` 归属脚本，按经过节点计数），最大的几支如下，都不是 a5-4b/e/f 原先设想的单一入口：
        - `ObjectInputFilter$Config.<clinit>` 共 1332 类，其中 `System.getLogger` 1163 类。`LazyLoggers.getLogger` 用 `DefaultLoggerFinder.isSystem(module)` 选懒日志器还是 `getLoggerFromFinder`。isSystem 的结果是特权动作返回的 Boolean，分析器不对布尔返回值做分支裁剪，`getLoggerFromFinder` 恒可达。运行期实际取懒日志器：`Class.getModule()` 的手写单例 loader 为 null。
        - 由此进入 `LoggerFinderLoader.<clinit>`（906 类）→ `SecurityConstants.<clinit>` → `SocketPermission.init` → `String.toLowerCase(Locale)`（783 类）→ `ConditionalSpecialCasing` → `BreakIterator.getWordInstance` → `LocaleProviderAdapter` → `CLDRLocaleProviderAdapter.<init>`（691 类）。a5-4e 原设想按 tr / az / lt 收窄，但 `StringUTF16.toLowerCase` 遇到 Σ（U+03A3）时不分语言都进 `ConditionalSpecialCasing`（FINAL_CASED 条件），按语言常量收窄剪不掉这条边。
        - `CLDRLocaleProviderAdapter.<init>` 的 `doPrivileged(PrivilegedExceptionAction)` 按全程序合流派发到 `URLClassPath$3.run`，下接 `JarLoader`、`JarVerifier`、`PKCS7`，共 493 类。`URLClassPath$3` 的分配点 `getLoader(URL)` 在合法路径上：`ServiceLoader.loadProvider` → `Class.forName(Module, String)` → `ClassLoader.loadClass(Module, String)` → `BuiltinClassLoader.findClassOnClassPathOrNull`，其中 `Module.loader` 被分析器视为 open。`getLoader(URL)` 按 URL 是否以 `/` 结尾在 FileLoader 与 JarLoader 之间选择；应用类路径是 `""` → cwd 目录，运行期只走 FileLoader，但静态不可判定。
        - 结论：三支都要靠值层面的建模才能剪掉，按原定的「常量 / 可达性」手段收窄不了。需要的能力有三项：① 布尔 / 引用返回值的过程间常量（isSystem）；② Module 字段按手写写入精确建模（边界类字段整体去掉 open，试验仅少 10 类，需与 ① 配合）；③ doPrivileged 按调用点派发（a5-4a，s2 后它挂着 493 类的归属，与 ① ② 合用才有回收）。按这条路线达到门槛需要新的设计，待定。
    - **(a)(b)(c) 设计与实测上界（2026-10-03，c1d-p0 d2501802，DeepCopy s2 口径 3065 类）**。协调方定的顺序：先做 (a) 过程间返回值常量传播（对准 `LazyLoggers.getLogger` / `isSystem`），再做 (b) Module 字段精确建模，最后 (c) doPrivileged 按调用点派发。动工前先测了回收上界。结论：三项合计回收不足 300 类，达不到门槛，需要重定方向。
      - **设计要点**（若仍实施，按此做）：
        - (a)：方法返回值摘要扩到 boolean / int 常量、null、精确类型 / 常量对象（`Obj` 标签）。现有 `Oracle::invoke_result` 的导出值只有 Int / Long / Null / Str，具体求值只接受常量实参。扩展包括两部分：特权动作返回值经 `executePrivileged` 回传，`run` 的摘要按动作类精确时传回调用点；调用点用摘要裁条件边。
        - (b)：边界类字段按手写写入点取值。`Class.getModule` 的手写单例 `THE_MODULE` 名为 null、loader 为 null，`unnamed_module()` 经 `__set_loader` 写系统加载器。由清单声明手写写入值，生成器 / 分析器据此给出字段值集，不再整体按 open。
        - (c)：`AccessController.executePrivileged` 的 `action.run()` 按调用点的实参值集派发（上下文敏感一层）。
      - **实测 1：用 `[facts.returns]` 模拟 (a)(b) 的理想结果**（临时改清单、只折返回值，实验后已还原）：
        - 只给 `DefaultLoggerFinder.isSystem = true`：3065，回收 0。
        - 再加 `BootstrapLogger.useLazyLoggers` / `useSurrogateLoggers`：3050。
        - 再加 `Pattern.has = false`：3041。
        - 再加 `ProviderConfig.getProvider = null`：3040。
        - 原因：§21.5 s2 小结里的「1163 / 906 / 783 / 493」都是首次发现链的归属数，不是可回收数。闭包是多连通的，剪掉 `getLoggerFromFinder` 后，同一批类从别的入口照样可达。
      - **实测 2：边沿精确切除模拟**。用 `rava closure --dump-edges` 导出 DeepCopy 全部 358521 条边，从根做可达性，删掉指定节点的出边后重算类数，结果与 `--cut` 等价。
        - 单节点切除可回收类数：

          | 节点 | 回收 |
          |---|---|
          | `AccessController.executePrivileged`（PrivilegedAction 变体） | 456 |
          | `executePrivileged`（PrivilegedExceptionAction 变体） | 95 |
          | `JarVerifier.processEntry` | 84 |
          | `StandardCharsets.lookup` | 74 |
          | `SunEC$ProviderService.newInstance` | 70 |
          | `Init$1.run`（XMLDSig 初始化） | 64 |
          | `PKCS7.<init>` | 62 |
          | `ObjectInputStream.readObject` | 47 |
          | `LazyLoggers.getLoggerFromFinder`（即 (a) 目标） | 3 |

        - 按派发目标拆 `executePrivileged`：
          - PA 变体：`SunEC$1.run` 72、Collator 提供者 lambda 18、`XMLDSigRI$2.run` 13、`ProviderConfig$3.run` 11、`Currency$1` 10、`LogManager$1` 9、`SunPCSC$1` 8，其余为个位数。
          - PEA 变体：`Init$1.run` 64、`URLJarFile$1` 12、`SunPKCS11$1` 9、`URLClassPath$3` 9。
          - 这些动作的分配点都在各自的合法路径上（如 `SunEC.<init>`、`Init.init`），RTA 下按调用点派发也照样可达。所以 (c) 的回收约为 0，与 §21.5 a5-4a 的结论一致。只有把整台 `executePrivileged` 删掉才有 456，而那不是正确的切除。
        - jar 校验与 JCA 簇 12 个节点（`JarVerifier.processEntry`、`PKCS7.<init>`、`SunEC$ProviderService.newInstance`、`Init$1.run`、`SunPKCS11$1`、`SunPCSC$1` 等）一起切：回收 256 类，降到约 2809。
        - 删掉全部 Object 方法（`equals` / `hashCode` / `toString`）的 hub 派发边：类数仍为 3065，说明还有冗余路径。
        - 已知的旁路入口：
          - `SecurityConstants`：经 `Sun.<init>` ← `ProviderList$3` ← `AbstractList.hashCode` ← `CopyOnWriteArrayList.hashCode` ← `ImmutableCollections$SetN.probe`，即合流元素上的 `Object.hashCode` 派发。
          - ICU：经 `PreHashedMap.get` → `AVA.equals`。
      - **toLowerCase → `ConditionalSpecialCasing`（Σ 路径）的论证：静态上确实可达，是正确的下限，不硬砍。**
        - DeepCopy 闭包内调用 `String.toLowerCase(Locale)` 的方法有 26 个，另有 6 个调用无参 `toLowerCase()`。其中多数接收者不是编译期常量，例如：
          - `URLUtil.urlNoFragString`（URL 的 host）；
          - `MessageFormat.findKeyword`；
          - `MimeTable.findViaFileExtension`；
          - `SocketPermission.getCanonName`；
          - `Provider.getEngineName`；
          - `DNSName.constrains`。
        - String 的 coder 由内容决定。接收者非常量时，UTF16 分支可达。
        - `StringUTF16.toLowerCaseEx` 遇到 U+03A3 时，不分语言都调 `ConditionalSpecialCasing.toLowerCaseEx`（FINAL_CASED 条件），再走 `isFinalCased` → `BreakIterator.getWordInstance(locale)` → `LocaleProviderAdapter` → CLDR / ICU 断句。
        - 可以按常量收窄的只有 Latin1 侧：`StringLatin1.toLowerCaseEx` 只在 lang 为 tr / az / lt 时进入，`URL.lowerCaseProtocol(Locale.ROOT)` 这类常量 Locale 调用点可以折掉。可是只要还有一个 UTF16 侧调用点可达，这条链就整体保留。
        - 因此这条链的规模（首次发现归属约 783 类，多数与其他入口共有）应计入正确下限。JDK 实跑时 `jdk/internal/icu` 装载 0 类，是因为输入里没有 Σ，属于动态与静态口径的差别。
        - 反过来说，只有当这些调用点本身因上游收窄（jar / JCA 簇）而不可达时，这条链才会随之消失。
      - **任务 2（撤手写 `BootLoader.findResourceAsStream`）受阻**：
        - 按字节码执行：rava 只有一个未命名模块，`Class.getModule` 的模块名、loader 都为 null。路径是 `Module.getResourceAsStream` → `BootLoader.findResourceAsStream(null, …)` → `BuiltinClassLoader.findResource(null)` → `findResourceOnClassPath`；引导 `ucp` 为 null（未设 `jdk.boot.class.path.append`），结果返回 null。TestLocaleBundleFamilies、TestNormalizerResourceForms 会因取不到 ICU / 断句资源而失败。
        - 忠实的终态是命名模块加 jimage：`SystemModuleReader` → `ImageReader` → `NativeImageBuffer.getNativeMap`，这一个 native 是唯一的手写，嵌入数据由 `input/src/resources.rs` 生成。这就是 `docs/plans/2026-10-02-boot-layer.md` §1.7、§2.3 第 6 项的步骤 1–5，按该计划须等 c1d-p0 合入后才开工。
        - 曾考虑把 boot append 指向由文件 native 供给的嵌入伪目录，但这属于过渡方案，而且会重新打开 `URLClassPath` / `JarLoader` 簇，与 a5-4b 冲突。不采用。
        - 结论：该方法保留到 boot-layer 步骤 5 落地后再删（届时 `non_native_overrides` 回到 0），或由协调方决定把 boot-layer 步骤 1–5 提前并入本线。
      - **建议的重定向**（按实测回收量排序，供协调方与用户核对门槛数值）：
        1. JCA 提供者在分析期按配置求值（`ProviderConfig` / `ProviderList` 只展开实际加载的提供者），连同 jar 签名校验簇，上界约 256 类。
        2. 容器 / 元素敏感的 Object 方法派发（`SetN.probe`、`PreHashedMap.get` 一类），用于消除上述旁路。
        3. Locale 语言常量折叠（只作用于 Latin1 侧）。
        4. a5-4b 引导 `ucp` 为 null，与 boot-layer 步骤 1–5 合并考虑。
        - 即使已识别的簇全部回收，合计也只有约 256–550 类，DeepCopy 停在约 2500–2800，离 1820 的门槛仍远。门槛能否按「去截断后的正确下限」重新核定，需要协调方与用户拍板。

### 21.6 并行编排

```
a3-U0 ──▶ a3-U1 ─┐
       ├▶ a3-U2 ─┼─▶ a3-U3（末提交移出 [vm_boundary]）─┐
       └─────────┘                                     │
a3-L1（getServicesCatalog 除外）──▶ a3-L2 ─────────────┤
a3-C ──────────────────────────────────────────────────┤
a3-X1、a3-X2 ──────────────────────────────────────────┤
boot layer 第 1 步 ──▶ a3-V、a3-L1 余下的 getServicesCatalog ──┤
a3-T1 ─▶ a3-T2 ─▶ a3-T4 ─▶ a3-T6（T3 ∥ T2、T5 ◀ T1）──┤
boot layer 第 3 步 ────────────────────────────────────┴─▶ a3-Z
a5-1 ──▶ a5-2 ──▶ a5-3（与 a3 无文件冲突，可同时进行）
```

同时开工上限按全机锁与内存预算定：第一批可并行 a3-U0（合入后 U1 / U2 并行）、a3-L1、a3-C、a3-X1、a3-X2、a5-1。

### 21.7 a3 各子任务验收数字（2026-10-03，c1d-p0 f2bdcf6e 口径）

底数：HelloWorld `vm_boundary_methods = 86`；`runtime/` 中 `#[jvm_boundary]` 135 处（分布见 §21.0，f2bdcf6e 复核不变）；
HelloWorld `--stop-after emit` 闭包 498 类（b17a6496 合并同步实测）。各项的「前 → 后」按单独合入计（降幅可叠加），
「闭包」一栏是 HelloWorld 闭包类数的上限，e2e 一栏是 §21.2 / §21.8.4 所列用例在服务器抽查的通过数。

| 编号 | HelloWorld 审计（本类计数） | `#[jvm_boundary]`（文件内处数） | 其它数字 | 闭包 | e2e |
|---|---|---|---|---|---|
| a3-U0 | Unsafe 44 → 44（只改标注） | `unsafe__impl.rs` 75 → 44；`#[jvm_native]` +31 | 生成树逐字节不变（`scripts/compare_trees.sh` 差异 0 文件） | 498 | 3/3 |
| a3-U1 | Unsafe −15（44 → 29） | `unsafe__impl.rs` −15 | 新增 native 6 个，全部 `ACC_NATIVE`；`non_native_overrides` 保持 0 | ≤498 | 6/6 + 边界 1 |
| a3-U2 | Unsafe −20 | `unsafe__impl.rs` −20 | `vm_intrinsics.toml` 中按 Java 层原子成员登记的内存效果条目 → 0（全部改指内层 native） | ≤498 | 9/9 |
| a3-U3 | Unsafe −9（U1–U3 全合后 0） | `unsafe__impl.rs` → 0 | `closure.toml` 中 `jdk/internal/misc/Unsafe` 出现次数 → 0；`CHM.comparableClassFor` 具体求值组合数不上升 | ≤498 | 7/7 |
| a3-V | VM 10 → 0 | `vm_impl.rs` 8 → 0 | `[vm_constants]` 中 `getSavedProperty` 恒 null 条目 → 0；线程内档位覆盖 `__vm_at_init_level` 引用 → 0 | ≤498 | 8/8 + 边界 1 |
| a3-T | VirtualThread 10 → 0 | `virtual_thread_impl.rs` 10 → 0 | 见 §21.8.4（T1–T6 各自的数字） | ≤498 | 见 §21.8.4 |
| a3-L1 | BootLoader 5 → 0 | `boot_loader_impl.rs` 5 → 0（文件删除） | 新 native 6 个（`NativeLibraries` 3、`getSystemPackageLocation`、`getNativeMap`、`findBootstrapClass`）；`closure.toml` 中 `BootLoader` → 0 | ≤498 | 6/6 |
| a3-L2 | ClassLoader 6 → 0 | `class_loader_impl.rs` 6 → 0 | 新增手写 0 | ≤498 | 4/4 + 边界 1 |
| a3-C | Class 2 → 1（余 `getModule` 归 boot layer） | `class_impl.rs` 2 → 1 | 新增手写 0 | ≤498 | 5/5 |
| a3-X1 | 链外（不计入 86） | `invoker_bytecode_generator_impl.rs` 6 → 0；`method_accessor_generator_impl.rs`、`class_specializer_factory_impl.rs`、`proxy_dyn_impl.rs` 各 1 → 0 | 运行模型替换登记条目 = 保留下来的手写方法数（逐个可对上） | ≤498 | 6/6 |
| a3-X2 | 链外 | `jce_security_impl.rs` 6 → 0；`file_systems_impl.rs`、`cds_impl.rs`、`event_helper_impl.rs`、`security_property_modification_event_impl.rs`、`get_instance_instance_impl.rs` 各 1 → 0 | `closure.toml` 中 `JceSecurity` / `FileSystems` / `InetAddress` / `SecurityManager` → 0 | ≤498 | 7/7 |
| a3-Z | `vm_boundary_methods` 行消失 | 全仓 0（其中 `module_impl.rs` 7、`module_layer_impl.rs` 2 由 boot layer 清零） | `rg jvm_boundary runtime/ generator/` → 0；生成器单测全过 | ≤498 | 9 例抽查 9/9 |

合计：HelloWorld 审计 86 → 10（a3 全部）→ 0（boot layer 第 3 步之后，由 a3-Z 确认）；`#[jvm_boundary]` 135 → 9（a3 全部）→ 0。

### 21.8 a3-T 虚拟线程：VirtualThread 字节码翻译 + Continuation 有栈协程（2026-10-03 用户定终态）

**终态**：`VirtualThread`、缺省调度器 `ForkJoinPool`（含 `CarrierThread`、`ForkJoinWorkerThread`）、延时调度器 `UNPARKER`
（`ScheduledThreadPoolExecutor`）全部按字节码翻译；手写只剩 `jdk.internal.vm.Continuation` 的 VM 方法（准入第③类：
VM 驱动的执行流切换），实现为**有栈协程**。目标规模**百万级虚拟线程**。
方案 A（2026-09-24：虚拟线程映射为平台线程、不建模 Continuation）**作废**；`virtual_thread_impl.rs` 的方案 A 注释与
`continuation_support_impl.rs` 中「映射 OS 线程」的说明随 T4 删除改写。完成后 `vm_intrinsics.toml` 与 `closure.toml`
中 `VirtualThread` 出现次数为 0，HelloWorld 审计的 VirtualThread 承载方法 10 → 0。

#### 21.8.1 手写面（JDK 21 `Continuation` 的全部 native，共 6 个）

| native | 语义（与 HotSpot 等价） |
|---|---|
| `registerNatives()` | no-op（已有） |
| `enterSpecial(Continuation c, boolean isContinue, boolean isVirtualThread)` | `isContinue = false`：从栈池取一块栈，在新栈上以 `Continuation.enter(c, false)`（字节码翻译体，内部 `enter0` → `target.run()`，`finally` 置 `done`）为入口切入；`isContinue = true`：切回 `c` 上次 `doYield` 保存的上下文。两种情况都在 `c` 让出或执行完毕时返回；执行完毕时栈归还栈池 |
| `doYield()` → `int` | 当前（最内层）已挂载的 Continuation 未被 pin：保存上下文、切回其 `enterSpecial` 调用点，日后被继续时返回 0；被 pin：不切换，直接返回 pin 原因码（2 CRITICAL_SECTION / 3 NATIVE / 4 MONITOR，与 `Continuation.pinnedReason` 的 tableswitch 一致），字节码随后走 `onPinned0` → `VirtualThread` 在载体上停泊 |
| `pin()` / `unpin()` | 当前 Continuation 的临界区计数 +1 / −1；`unpin` 在计数为 0 时抛 `IllegalStateException`（与 HotSpot 一致）；不在 Continuation 内时为 no-op |
| `isPinned0(ContinuationScope scope)` → `int` | 自最内层向外找第一个 `scope` 匹配的 Continuation，期间任一层被 pin 即返回其原因码，否则 0 |

嵌套 Continuation 的作用域匹配、`yieldInfo`、`parent` 链、`mount` / `unmount` 簿记全部是 `Continuation` 的字节码，
native 只切换「最内层」一层。`Thread` 的 `currentCarrierThread` / `setCurrentThread` / `scopedValueCache` /
`setScopedValueCache` 已是 ① native（`thread_impl.rs`），按 §21.8.3 调整承载槽位。

#### 21.8.2 栈与上下文切换（新 crate `runtime/rava_coro/`）

- **独立 crate**：不依赖 `java_runtime`，可单独 `cargo test`（与 `rava_macros` 同样以绝对 path 依赖，不复制进 scratch）。
  `java_runtime` 只经它的 `Stack` / `Context` / `switch` 三个入口使用。
- **栈（终态选型，2026-10-03 x86 复测后修订）**：每个 Continuation 一个独立栈槽，保留 `RESERVE`（缺省 1 MiB，经
  `rava_coro::stack::set_reserve` 在首次建栈前调整；项目不设自有环境变量，见 `docs/environment-variables.md`），只按需提交。
  地址空间预算：10⁶ × (1 MiB + 1 页) ≈ 1 TiB，低于 x86_64 / aarch64 Linux 的 128 TiB 用户空间和 macOS arm64 的上限。
  - **slab 多栈预留**：一次 `mmap`（`MAP_NORESERVE`）预留一块含多个槽的区间，新块槽数 = 现有总槽数（容量倍增），夹在
    16 ～ 1024 之间；10⁶ 个存活栈约 1000 个映射，**映射数与存活协程数脱钩**，缺省 `vm.max_map_count`（65530）下即支持
    10⁶，不要求调任何 sysctl。取栈总从基址最低、有空槽的块取（块内后进先出），存活栈集中在少数块。
  - **归还**：至多 512 个空槽为热槽（只交还栈顶 16 KiB 以下的页，常驻 ≤ 8 MiB，与核数无关），其余整槽交还
    （Linux `MADV_DONTNEED`；macOS `MADV_FREE_REUSABLE`，复用时 `MADV_FREE_REUSE`）。`madvise` 不拆分映射。块内存活数归零且
    已有另一个空块时整块 `munmap`——`madvise` 不回收页表（10⁵ 个 1 MiB 间隔的栈约占数十 MiB 页表），空块整块释放才交还
    页表与地址空间；只留一个空块作缓冲，避免单协程反复建销时反复映射。
  - **溢出检测两层**：①**软件栈界（所有平台，Java 语义的依据）**：每个执行流（协程与平台线程）带一个栈界
    `limit = 栈底 + SHADOW(64 KiB) + YELLOW(64 KiB)`，`switch` 与 guard 区间一起存入 / 换出线程局部；宏在每个返回
    `Result` 的 Java 方法序言注入 `__stack_check()?`（不内联的 `rava_coro::stack_exhausted()`：栈指针低于栈界即抛
    `StackOverflowError`，构造异常期间以 `YellowZone` 临时放开 YELLOW）。SHADOW 覆盖检查点之间的用量（一个 Java 帧 +
    其调用的不经检查点的运行时代码）。②**硬件 guard（只在不增加映射时启用）**：每槽底一页；Linux ≥ 6.13 以
    `madvise(MADV_GUARD_INSTALL)` 装 guard 标记（不拆分 VMA，首块探测一次，旧内核 `EINVAL` 即停用），macOS 逐槽 `mprotect`
    （无小额映射上限）。命中 guard 即「has overflowed its stack」abort，是检查点之外失控原生递归的兜底。
    残余风险：Linux < 6.13 上无硬件 guard，SHADOW 之外的失控原生递归（非 Java 方法，如深层 `Drop` 链）会越过槽底写入相邻槽，
    无法被识别；生成代码的递归都经 Java 方法检查点，运行时手写代码不得有无界递归（T1b 审计）。
  - **否决的方向**：①逐栈 `mprotect` guard——每栈 2 个 VMA，缺省 65530 下 32748 个存活即 `ENOMEM`（kr1 实测），调 sysctl 属系统
    配置，用户机器上改不了；②Loom 式拷栈（挂起时帧拷到堆、载体栈复用）——Rust 帧内有指向栈内的裸指针（局部变量引用、
    FP 链、`&mut` 借用跨 `switch`），恢复必须回到原地址；回原地址意味着按槽绑定运行栈，跨载体迁移与同槽协程互斥会退化为串行
    或死锁；③分段栈 / 栈拷贝增长——同样受「帧不可搬」约束，且需要编译器配合序言（Rust 无此支持）。
  建栈 `mmap` 失败（地址空间耗尽）按 JDK 平台线程耗尽的形态抛 `OutOfMemoryError`（消息同 JDK「unable to create native thread:
  possibly out of memory or process/resource limits reached」），不静默降级。
- **上下文切换**：naked 函数（`#[unsafe(naked)]` + `naked_asm!`）两份，按 `target_arch` 选择，其余平台编译期报错（不提供退化实现）：
  - aarch64（AAPCS64）：保存 / 恢复 x19–x28、x29（FP）、x30（LR）、SP、d8–d15；
  - x86_64（SysV）：保存 / 恢复 rbx、rbp、r12–r15、RSP、返回地址（RIP），以及 MXCSR 控制位与 x87 控制字。
  切换函数是普通 `extern "C"` 调用，调用方保存寄存器由编译器处理。
- **入口蹦床**：新栈的第一帧是蹦床，栈顶 16 字节对齐，帧链终止（aarch64 FP = 0、LR = 0；x86_64 RBP = 0，CFI 标
  `.cfi_undefined rip`），保证回溯与栈遍历在蹦床处干净停止，不走进载体栈。

#### 21.8.3 语义约束

**pinned 判定**（`doYield` / `isPinned0` 的依据，每个 Continuation 一组计数，存于下文的执行上下文块）：

| 原因 | 计数来源 |
|---|---|
| MONITOR（4） | `monitor.rs` 的 `enter` / `exit`（含 `synchronized` 方法与 `Object.wait` 期间仍持有的重入层数）在 Continuation 内执行时 ±1；持锁数 > 0 即 pinned。监视器所有者仍按 OS 线程 `ThreadId` 记录——持锁期间必然 pinned、不会换载体，所以所有者标识保持有效 |
| NATIVE（3） | 栈上有 native 帧：`#[jvm_native]` 方法体经宏包裹进出 ±1（只计数、无其它开销）；类初始化协议执行 `<clinit>` 期间 ±1（HotSpot 由 VM 帧调用 `<clinit>`，同为 NATIVE）。`#[jvm_native(unpinned)]` 豁免 HotSpot 不留 native 帧的入口：`Continuation` 自身的 5 个 VM 入口与方法句柄签名多态成员（`invokeBasic` / `invokeExact` / `invoke` / `linkTo*`，内建适配器直接跳转） |
| CRITICAL_SECTION（2） | `Continuation.pin()` / `unpin()` 计数 |

判定次序与 HotSpot `is_pinned0` 相同：CRITICAL_SECTION → MONITOR → NATIVE，取第一个成立的。pinned 时 `VirtualThread` 的字节码在载体上停泊
（`parkOnCarrierThread`），载体 OS 线程阻塞，与 JDK 21 行为一致。

**与 GIL、thread_local 的交互**：

- 运行时已无全局解释器锁（#42 并行后端，`gil.rs` 只保留 `blocking` / `safepoint` 钩子与 DestroyJavaVM 登记）。
  载体是普通平台线程（`ForkJoinWorkerThread` 经 `Thread.start0` 派生），虚拟线程不计入 DestroyJavaVM 的非守护等待集
  （`VirtualThread` 恒为守护，字节码已保证）。`blocking` / `safepoint` 不切换协程——协程只在 `doYield` 处让出。
- **线程身份分两槽**：`thread_impl.rs` 的 `CURRENT` 拆为 `CARRIER`（OS 线程对应的平台 `Thread`，派生时设定，永不变）
  和 `CURRENT`（`currentThread()` 返回值）。`currentCarrierThread()` 读 `CARRIER`；`setCurrentThread(t)` 只写 `CURRENT`。
  挂载 / 卸载时由 `VirtualThread.mount` / `unmount` 的字节码调用 `setCurrentThread` 切换，native 不自行切换。
- **ScopedValue**：绑定在 `Thread.scopedValueBindings` 字段（随 `Thread` 对象走，无需处理）；查找缓存
  `SCOPED_VALUE_CACHE` 留在载体槽，由 `Continuation.run` 的字节码在挂载时 `setScopedValueCache(scopedValueCache)`、
  卸载时取回并置 null。`findScopedValueBindings` 的「栈上无 runWith 帧」判定在虚拟线程上同样成立（绑定已经由字段承载）。
- **执行级状态迁入执行上下文块**：凡生存期可能跨越一次调用（从而可能跨越 `doYield`、在另一载体上恢复）的线程局部状态，
  不得直写 `thread_local!`，统一放进每个执行流一块的「执行上下文块」：平台线程一块，每个 Continuation 一块，
  载体线程局部只存指向当前块的一个指针，由 `enterSpecial` / `doYield` 在切换时换指针。现有须迁入的：
  `stack_stream_factory_abstract_stack_walker_impl.rs` 的 `ANCHORS` / `NEXT_ANCHOR`、`reflect_dispatch.rs` 的
  `BAD_ARG` / `CS_CALLERS` / 逃逸异常表，以及上面的三个 pin 计数；`vm_impl.rs` 的 `BOOT_LEVEL` 由 a3-V 删除，不迁。
  留在载体线程局部的只有 `CARRIER` / `CURRENT` / `SCOPED_VALUE_CACHE` 与这一个指针。
- **TLS 地址缓存**：LLVM 视线程局部地址在函数内不变，可能跨 `doYield` 调用复用旧载体的地址。所有经执行上下文块
  的访问走一个 `#[inline(never)]` 取指针函数，每次访问重新读取；`__process_static!` 宏的线程局部分支同样改走该入口。
  守护：runtime 单元检查（与 `jdk_literal_lint` 同机制）统计 `runtime/` 中 `thread_local!` / `__process_static!` 线程局部
  定义，只允许出现在执行上下文模块与 `thread_impl.rs` 的三个载体槽。

**panic 跨栈传播**：生成工作区为 `panic = "abort"`，`create_java_vm` 的钩子打印默认信息后以退出码 101 退出——
在协程栈上 panic 与平台线程同一出口，不发生跨栈 unwind。要求：①回溯在蹦床处终止（§21.8.2），stderr 与平台线程 panic
同形；②蹦床对 unwind 形态（将来改 `panic = "unwind"` 时）同样正确：入口函数体包 `catch_unwind`，载荷存入执行上下文块，
切回 `enterSpecial` 调用点后在载体栈上 `resume_unwind`，绝不让 unwind 穿过汇编帧。Java 异常走 `Result`，由
`Continuation.enter0` / `VirtualThread.run` 的字节码处理，不经过 native。
**栈溢出**：Java 方法递归由软件栈界判定，抛可捕获的 `StackOverflowError`（协程与平台线程同一机制，§21.8.2）；硬件 guard
（启用时）触发 SIGSEGV，载体线程装 `sigaltstack` 处理器，故障地址落在当前协程 guard 页内时输出与 Rust 平台线程相同的
「thread '…' has overflowed its stack」并 abort，其余故障交还原处理器。

#### 21.8.4 细分与验收

| 编号 | 内容 | 文件 | 依赖 | 验收数字 |
|---|---|---|---|---|
| **a3-T1** | `rava_coro`：slab 栈（多栈预留 + 热 / 冷槽 + 空块释放）、软件栈界与切换时随执行流换出、条件硬件 guard、aarch64 / x86_64 切换汇编、入口蹦床、guard 故障识别 | 新 crate `runtime/rava_coro/`（每文件 ≤600 行） | 无 | crate 单测：10⁶ 次往返切换正确且单次切换 ≤50 ns（release，本机 aarch64 与服务器 x86_64 各测一次）；**缺省 `vm.max_map_count`（65530）下** 10⁵ 个协程同时挂起，映射增量 ≤256、slab 块 ≤256；全部完成后 RSS（Linux 计入 VmPTE 页表，macOS 物理足迹）回落到起点 +16 MiB 以内、slab 块 ≤1；callee-saved 寄存器（含 d8–d15、MXCSR）逐个被破坏后恢复的检查全过；软件栈界：带检查点的递归在栈界 ±1 页处判定耗尽、YellowZone 放开后可再用 32 KiB、栈界随 `switch` 换出换入；硬件 guard 启用时（macOS、Linux ≥ 6.13）子进程（串行）退出码为 SIGABRT、stderr 含 overflowed |
| **a3-T1b** | Java 栈溢出语义：宏在返回 `Result` 的 Java 方法序言注入 `__stack_check()?`；`JvmError::stack_overflow`（YellowZone 内构造 `StackOverflowError`）；平台线程入口 `init_platform_thread`；`StackOverflowError` 经闭包 VM 规则 `stack-check` 入闭包 | `rava_macros_core`（`block/class_init.rs` 同位注入）、`error.rs`、`thread_impl.rs`、`closure/src/engine/vmrules.rs` | ◀ T1 | 平台线程与协程内无界递归都抛可捕获的 `StackOverflowError`，捕获后继续执行；e2e TestArraysDeepOps 通过；TestVirtualThreadCarrier 含虚拟线程内递归 SOE 捕获一项；release 下 roundtrip 切换仍 ≤50 ns |
| **a3-T2** | `Continuation` 6 个 native、执行上下文块、三类 pin 计数（`monitor.rs`、`#[jvm_native]` 宏包裹、类初始化协议） | `jdk/internal/vm/continuation_impl.rs`；新执行上下文模块；`monitor.rs`；`gil.rs`（类初始化段）；`rava_macros` | ◀ T1 | `continuation_impl.rs` 中 `#[jvm_native]` 6、`#[jvm_boundary]` 0；`non_native_overrides` 0；抽查 HelloWorld、TestSynchronized、TestThreadStates、TestThreadInterrupt、TestSleepParkClock、TestCommonPool 与一个含 `<clinit>` 的例不回归（计数器挂在 monitor / native 包裹 / 类初始化上）。方案 A 在 T4 前仍在，虚拟线程不走 Continuation，pinned 的 e2e 随 T4 验收；CRITICAL_SECTION（`Continuation.pin` / `unpin`）按终态语义实现，**公开 API 不可达、无 e2e**：refjdk 21 java.base 字节码中 `Continuation.pin` / `unpin` 只出现在 `Continuation` 自身，而 rava 的 javac 固定 `--release 21`、与 `--add-exports` 互斥，用户代码无法直接调用 `jdk.internal.vm.Continuation`；正确性由 `rava_coro` 单测与代码审阅保证（2026-10-03 协调者同意改口径） |
| **a3-T3** | 线程身份两槽、执行级状态迁入执行上下文块、TLS 访问入口、thread_local 守护检查 | `thread_impl.rs`、`stack_stream_factory_abstract_stack_walker_impl.rs`、`reflect_dispatch.rs`、`sync_model.rs` | 与 T2 并行，T4 前合入 | `thread_local!` 定义：runtime 中只余执行上下文模块 1 处 + `thread_impl.rs` 3 个载体槽，守护检查通过；TestStackWalkerFrames、TestReflectFieldMethod、TestThreadStates 3/3 |
| **a3-T4** | 删方案 A：`VirtualThread` 10 个承载方法删除（只留 6 个 JVMTI / registerNatives `#[jvm_native]`）；`VirtualThread` 移出 `[vm_boundary].classes` 与 `clinit_carried`；调度器按字节码翻译 | `virtual_thread_impl.rs`、`continuation_support_impl.rs`、`closure.toml` | ◀ T2、T3 | HelloWorld 审计 VirtualThread 10 → 0；`closure.toml` / `vm_intrinsics.toml` 中 `VirtualThread` 0 次；HelloWorld 闭包 ≤498 类且 `VirtualThread.<clinit>` 不在闭包内（`--trace-class` 确认）；e2e TestVirtualThread、TestVirtualClockPark、TestThreadStates、TestThreadInterrupt、TestSleepParkClock、TestCommonPool、TestSynchronized 7/7；边界用例 **TestContinuationPinned**（虚拟线程在 `synchronized` 内 sleep / park——MONITOR；在 `<clinit>` 内 sleep——NATIVE；两种情形均在载体上停泊并正确完成，expected 取参考 JDK 两次一致的运行）与 **TestVirtualThreadCarrier**（让出后在另一载体恢复：`currentThread()` 身份、`ThreadLocal` / `InheritableThreadLocal` 值、`isVirtual()`、中断状态、`join(Duration)` 超时，输出与载体编号无关）与 JDK 一致 |
| **a3-T5** | panic 与栈溢出：蹦床 `catch_unwind` / `resume_unwind`、回溯终止、`sigaltstack` 处理器 | `rava_coro`；`lib.rs`（`create_java_vm`） | ◀ T1 | 子进程测试（串行）：协程内 panic 退出码 101、stderr 与平台线程 panic 同形；硬件 guard 启用时协程内无检查点无限递归退出为 SIGABRT 且 stderr 含 overflowed；`panic = "unwind"` 构建下 crate 单测：协程内 panic 在载体上被 `catch_unwind` 捕获 1/1 |
| **a3-T6** | 规模验收 | 边界用例 **TestVirtualThreadScale**（10⁵ 个虚拟线程各 `sleep` 后汇总，进入常规 e2e）；百万规模用例放 `tests/perf/`，服务器单独作业 | ◀ T4、T5 | TestVirtualThreadScale：10⁵ 全部完成、峰值 RSS ≤2 GiB、墙钟 ≤10 s；百万作业（缺省内核参数，不调 sysctl）：10⁶ 个虚拟线程同时处于 `sleep` 停泊，全部完成，峰值 RSS ≤24 GiB（每个停泊线程已提交栈 ≤16 KiB + 堆对象），墙钟 ≤120 s；载体 OS 线程数 = `availableProcessors` + `UNPARKER` 1 条 |

a3-T 合计新增 e2e 边界用例 3 个（TestContinuationPinned、TestVirtualThreadCarrier、TestVirtualThreadScale），expected 取 JDK 21，
输出与平台、载体编号、调度次序无关。

#### 21.8.5 实施记录

**T1 `rava_coro`（分支 a3t-vthread）**
- 布局：`src/lib.rs`（`Context` / `switch` / `Entry`）、`src/stack.rs`（slab 栈、`StackError`；见下「T1 修订」）、`src/guard.rs`
  （guard page 故障识别）、`src/arch/{aarch64,x86_64}.rs`（切换与蹦床）；单测 `tests/{roundtrip,registers,rss,overflow}.rs`。
  `java_runtime/Cargo.toml` 以 `path = "../rava_coro"` 依赖，overlay 与 `rava_macros` 同样改写为绝对路径（不复制进 scratch）。
- 切换：`switch(save, load, arg) -> usize` 记录切出方的 guard 区间、装入切入方的，再进 naked `raw_switch`；
  切入后把 `load.sp` 清零，二次恢复同一上下文即断言失败（防双重恢复）。aarch64 帧 160 B（x19–x30、d8–d15），
  x86_64 帧 64 B（rbp、rbx、r12–r15、MXCSR、x87 CW、返回地址）；新栈初始帧在 x86_64 写入缺省 MXCSR 0x1F80 / CW 0x037F。
- 蹦床：aarch64 `x29 = x30 = 0` + `.cfi_undefined x30`，x86_64 `rbp = 0` + `.cfi_undefined rip`；入口函数
  `extern "C" fn(arg, data) -> !` 不返回（返回即 `brk` / `ud2`）。
- guard page：进程级 SIGSEGV / SIGBUS 处理器（`SA_ONSTACK`，链接 std 原处理器）；`switch` 维护线程局部「当前栈 guard
  区间」，故障地址落在区间内时按 std 同形输出 `thread '<名>' (<tid>) has overflowed its stack` + `fatal runtime error:
  stack overflow, aborting` 后 abort；载体首次切入协程时补建 64 KiB sigaltstack（std 只给它创建的线程建）。
- 栈池：原计划「核数 × 64」改为固定 512 块——池内每块至多常驻 KEEP，核数线性的上限在 64 核服务器上空闲常驻约 48 MiB，
  超出 +16 MiB 验收；池只用于摊薄 mmap / mprotect，512 块足够每核 8 块周转。`set_pool_limit(0)` = 不池化。
- 实测（本机 aarch64，macOS 16 KiB 页，release）：10⁶ 次往返 30.1 ms，单次切换 **15.1 ns**（≤50 ns）；10⁵ 协程各触碰
  8 KiB 栈后挂起，物理足迹峰值 1621 MiB，全部完成后 3.5 → 17.1 MiB（+13.6 MiB，含池 512 块 × 16 KiB = 8 MiB；≤16 MiB）；
  callee-saved 寄存器（x19–x29、d8–d15 / rbp、rbx、r12–r15、MXCSR、x87 CW）1000 轮双向哨兵检查 0 偏差；
  guard page 子进程 SIGABRT、stderr 含 overflowed。x86_64 数字待服务器跑 crate 单测补录。
- macOS 的 RSS 口径取 `proc_pid_rusage` 的 `ri_phys_footprint`：`resident_size` 把 `MADV_FREE_REUSABLE` 归还的页计到被
  回收为止，不反映归还效果；Linux 取 `/proc/self/statm` 驻留页（`MADV_DONTNEED` 立即生效）。

**T1 修订：映射数与存活数脱钩（2026-10-03，kr1 x86 复测失败后）**
- kr1（Linux 6.8 x86_64，`vm.max_map_count` 65530）：切换 14.44 ns、callee-saved、guard abort、跨线程 resume 均过；
  10⁵ RSS 用例在 32748 个存活栈处 `mprotect` `ENOMEM`——逐栈 guard 每栈 2 个 VMA。终态要求缺省内核参数下 10⁶。
- 选型见 §21.8.2「栈」：slab 多栈预留 + 软件栈界（Java SOE 语义）+ 只在不增映射时启用的硬件 guard；逐栈 mprotect、
  Loom 拷栈、分段栈否决（理由同节）。
- 实现：`stack.rs` 改 slab（`BTreeMap` 块表 + 有空槽块集合，取最低基址块；空槽 `槽号 << 1 | 热` 后进先出；热槽 ≤512；
  空块多于 1 个即 `munmap`）；`guard.rs` 线程局部改为 `Bounds { guard, limit }`，`Context` 携带、`switch` 换出换入；
  新 `limit.rs`：`stack_exhausted`（不内联）、`YellowZone`、`init_platform_thread`（macOS `pthread_get_stackaddr_np`，
  Linux `pthread_getattr_np`）；`set_reserve` 只在首次建栈前生效、下限 256 KiB（`MIN_RESERVE`）。
- 首版 slab 不释放块，macOS 实测 10⁵ 完成后物理足迹 +65 MiB：全为页表（1 MiB 间隔的栈每 32 MiB 一张 16 KiB L3 表，
  `madvise` 不回收）；Linux RSS 不计页表会掩盖这一项，故 Linux 口径改为 RSS + VmPTE，并加空块整块释放。
- 实测（本机 aarch64 macOS，release）：单次切换 15.0 ns；10⁵ 协程挂起峰值 1624 MiB，全部完成后 4.2 → 11.8 MiB（+7.6 MiB）、热槽 16 个、
  slab 余 1 块；软件栈界在深度 2866（每帧约 300 B）、距可用区底 128 KiB 处判定耗尽；硬件 guard（mprotect）子进程 SIGABRT。
- kr1 复测（x86_64 Linux 6.8，4a1b008c）：单次切换 14.96 ns；10⁵ 挂起时映射数 37 → 38（slab 104 块），RSS + VmPTE 起点 5.4 MiB、
  峰值 1381 MiB、全部完成后 13.1 MiB（+7.7 MiB）；软件栈界在深度 2866、距可用区底 128 KiB 处判定耗尽；无 `MADV_GUARD_INSTALL`，
  guard 用例按预期跳过；callee-saved、跨线程 resume 通过。输出中旧口径「池 N 块」指热槽个数（`pooled_stacks`），与 slab 块数
  （`mapped_chunks`）不同：完成后余下的 1 个空块（16 槽）全部是热槽，热槽 16 个、slab 1 块，两者一致；测试输出已改为分别打印。

**T1b Java 栈溢出语义（2026-10-03）**
- 宏：`entry_checks`（原 `null_receiver_check`）在每个返回 `Result` 的方法入口依次注入空接收者检查与 `__stack_check()?`；
  接口载体分派（直达实现类 vtable 体、不经 wrapper）同样注入。`__stack_check`（`lib.rs`，`#[inline(always)]`）调
  `rava_coro::stack_exhausted()`，耗尽时走冷路径 `JvmError::stack_overflow`：`YellowZone::enter` 放开余量后构造
  `StackOverflowError`；已在余量区内（构造途中再次耗尽）按 HotSpot 红区同义 abort。
- 平台线程：`create_java_vm` 与 `spawn_java_thread` 入口调 `init_platform_thread`，栈界 = 栈底 + max(SHADOW, 栈大小 / 16) + YELLOW
  （macOS `pthread_get_stackaddr_np`，Linux `pthread_getattr_np`）；协程栈界由 `switch` 携带（T1）。
- 闭包：宏注入调用对分析器不可见，`StackOverflowError` 改由 VM 规则 `stack-check`（无条件，落到 `stack_overflow` 构造入口，
  与 `null-check` / `heap-exhausted` 同构）入闭包；原定「VM 抛出异常清单」路线弃用。单测：`vmrules` 规则存在、
  `closure_cli::stack_overflow_error_in_minimal_closure`（最小程序闭包含该类且经 stack-check）。
- 闭包规模（实测，JDK 类数）：合入 b3 前 HelloWorld 497 → 498、StockTrans 3105 → 3106、CarmichaelPseudoprimes 2898 → 2899；
  合入 71d80723 后 HelloWorld 466 → 467（`--why` 确认唯一来路为 `stack-check`）、StockTrans 3106。此前没有任何路径抵达
  `StackOverflowError`，各程序一律 +1 类。T4 的「HelloWorld ≤498」按含此类的口径复核。
- 体积（本机 aarch64，dev = 语料缺省 profile）：HelloWorld 文件 44.75 → 45.21 MB（+1.0%），`__TEXT` 11.16 → 11.57 MB（+3.7%）；
  CarmichaelPseudoprimes（2899 类）文件 296.0 → 299.2 MB（+1.1%），`__TEXT` 80.97 → 84.15 MB（+3.9%）；release（调用基准，498 类）
  文件 17.25 → 17.73 MB（+2.7%），`__TEXT` 8.00 → 8.37 MB（+4.7%）。
- 耗时（同机中位数，release 5 轮、dev 3 轮；调用基准 = 递归 fib(32) + 5×10⁷ 次实例方法调用）：release fib 10 → 17 ms（+70%，约 1 ns / 调用）、
  实例调用循环 101 → 114 ms（+13%）；dev fib 83 → 142 ms、循环 679 → 1083 ms（+60%）；CarmichaelPseudoprimes（dev，
  printf 为主）user 0.12 → 0.14 s。开销来自每个方法入口一次不内联的线程局部读 + 比较（检查本身 1.28 ns，`check_cost`）。
- 残余：运行时手写代码无界递归审计（§21.8.2 残余风险）尚未做，列入 T1b 后续。

**T1b-2 叶子方法省略入口栈检查（2026-10-03）**
- 判定在生成器、只看字节码：`Code::is_leaf`（无 invokevirtual / invokespecial / invokestatic / invokeinterface /
  invokedynamic）且字节码 ≤512 字节（`LEAF_MAX_CODE_LEN`：省掉检查后叶子帧落在检查点之间的 `SHADOW` 余量内，限长保证
  该帧远小于 64 KiB）；手写体不计。生成器在 `#[java_method(..)]` 上标 `leaf = "true"`，宏 `entry_checks` 见标注只留
  空接收者检查。无类名字面量。
- **入口须唯一对应本体**（`leaf_entry`）：只对 static / private / 构造器 / `<clinit>` / final 方法、或承载类为 final 的方法
  省略。可覆盖的虚方法即使本体是叶子也保留检查——其 wrapper 入口分派到子类非叶子覆盖体时，「基类叶子声明 + 子类递归
  覆盖」构成的递归环上将没有任何检查点（`self.next()` 经基类 wrapper → vtable → 子类体 → 基类 wrapper …），无界递归直接
  撞 guard / 越界。开放世界下不以「当前无覆盖」为据省略。接口载体分派的检查不变。
- 单测：`rava_macros_core` `class_init::tests` 3 项（非叶子有检查、叶子只留空接收者检查、非 Result 无检查）；
  生成器 `attrs::tests::leaf_body_from_bytecode`（5 种调用指令、长度上限）、`leaf_entry_requires_exact_target`（可覆盖虚方法
  不省、final 类 / final / private / static 省、无体不省）。
- 边界 e2e **TestLeafStackOverflow**（`49_exceptions_deep`）：static 叶子、private 叶子、final 类叶子 getter、基类叶子被子类递归
  覆盖（经基类引用）、接口 default 叶子被实现类递归覆盖（经接口引用）五种递归各两轮，均抛可捕获的 `StackOverflowError`
  且深度 >500，之后叶子方法正常调用；expected 取 refjdk 21。生成树中用户类标 `leaf` 的恰为 inc / bump / Box.get / Box.set，
  `Node.next` 与 `Step.step` 未标。该例 JDK + 用户入口中标 `leaf` 的约 1620 个（占 `java_method` 标注约 9%）。
- 调用基准（release，本机 aarch64，各 3 轮）：

  | 配置 | fib(32) | 5×10⁷ 实例调用 |
  |---|---|---|
  | 检查全关（`__stack_check` 恒 Ok） | 9–10 ms | 101–104 ms |
  | 检查全开（T1b） | 17–20 ms | 114–123 ms |
  | T1b-2（`add` 为非 final 虚方法，保留检查） | 16–18 ms | 114 ms |
  | T1b-2，`add` 改 `final`（`CallBenchFinal`） | 17–19 ms | 101–104 ms |

  fib 递归体本身不是叶子，开销不变；叶子入口的开销归零（循环回到检查全关水平）。

**T2 Continuation native 与 pin 计数（2026-10-03）**
- 判定逻辑与运行时解耦：`rava_coro/src/pins.rs` 的 `Pins`（三类计数 + 外层链 + 作用域身份；`yield_reason` 同 HotSpot
  `freeze_internal` 次序，`scope_reason` 同 `is_pinned0` 逐层规则；MONITOR 取载体上整条执行流链之和，对应
  `held_monitor_count` 按线程计）。单测 `tests/pins.rs` 7 项：根执行流 pin 无操作、临界区计数与「pin underflow」、
  三类原因次序、外层持锁 pin 住内层让出、作用域逐层查找、卸载后换载体重挂，以及在真实协程上「三种 pin 各不切换、
  解除后让出并在载体上恢复」的往返——CRITICAL_SECTION 无 e2e 的正确性依据。
- 执行上下文块：新模块 `java_runtime/src/exec_context.rs`（`ExecContext { pins }`，repr(C)；平台线程块与当前块指针是本模块
  唯一的 `thread_local!`，取址函数 `current()` 不内联）。`mount` / `unmount` 由 `enterSpecial` 在切入前 / 切回后于同一载体上调用，
  协程侧不碰线程局部。T3 迁入的执行级状态加在本块上。
- 计数挂点：`monitor.rs` 的 `enter` / `exit` 与 `MonitorGuard`（成功进出才计）；`gil.rs` 的 `clinit_enter`（返回 `Run` 时）/
  `clinit_exit`；`#[jvm_native]` 宏（`rava_macros/src/native_attr.rs`）在方法体首句插 `NativeFrame` 守卫，参数
  `unpinned` 豁免（见 §21.8.3 表）。守卫缓存块地址：计数 > 0 期间执行流不换载体。
- `continuation_impl.rs`：6 个 `#[jvm_native]`、0 个 `#[jvm_boundary]`。协程记录 `Coroutine`（repr(C)，首字段 `ExecContext`，
  `doYield` 由当前块地址还原记录，不查表）按 Continuation 身份登记在 64 分片侧表、持有该 Continuation 一个引用——挂起的
  协程栈上 `enter` 帧本就持有它（无追踪式回收的对象模型下自环本就不可回收），登记期间身份不会复用；执行完毕时在载体上
  摘除、栈归池，`enter` 抛出的异常经记录带回、由 `enterSpecial` 继续抛出。`tail`：首次进入挂空 `StackChunk`，挂起期间
  `sp = bottom + 1`，运行与完毕时 `sp = bottom`（满足 `finish` / `run` 的「空 ⇔ 完毕」断言）。建栈失败抛 `OutOfMemoryError`
  （JDK 建线程失败同消息）。不在 Continuation 内调 `doYield`、继续未登记的 Continuation 属 VM 不变量破坏，panic。
- 方案 A 仍在（T4 删除），本步 Continuation 不在虚拟线程路径上，相关字节码方法仍是档案外存根；T4 接通后核对
  `enterSpecial` 手写体对 `Continuation.enter` / `StackChunk.<init>` 的调用点推断与 `tail` 字段读写不被常量折叠。

**T3 线程身份两槽与执行级状态迁移（2026-10-03）**
- 载体槽（`thread_impl.rs`，HotSpot `JavaThread` 的对应物）：`CURRENT`（`_vthread`，`currentThread` / `setCurrentThread`）、
  `CARRIER`（`_threadObj`，`currentCarrierThread`；派生平台线程时两槽同设、终结时清空，主线程首次 `currentThread` 时同设）、
  `SCOPED_VALUE_CACHE`（`_scopedValueCache`，由 `Continuation.run` 字节码在挂载 / 卸载时存取）。三槽只经 6 个 `#[inline(never)]`
  存取函数访问（TLS 地址缓存，见 §21.8.3）。
- 随执行流走的状态迁入执行上下文块 `ExecState`（`exec_context.rs`，`state()` 取当前块）：反射实参拆箱失败标记、
  @CallerSensitive 调用者栈、反射目标逃逸异常（`reflect_dispatch.rs`）、StackWalker 锚定帧流（`AnchoredWalk`，
  `stack_stream_factory_abstract_stack_walker_impl.rs`）、引导段 initLevel（`vm_impl.rs`）。这些状态跨 Java 调用存活，期间
  执行流可能让出到别的载体、同一载体上也会穿插别的执行流。
- 按载体键的 VM 设施改取 `currentCarrierThread`（HotSpot 挂在 `JavaThread` 上）：`Unsafe.park` 的许可、`monitor.rs` 的
  `current_thread_identity`（park / 中断 / wait 的设施键）、`clear_current_interrupted`（VM 抛 InterruptedException 时清
  `threadObj()` 的中断）、`enter_blocking_status`（阻塞时写 `threadObj()` 的 threadStatus）。与之配对的字节码侧：
  `VirtualThread.unpark` 被 pin 时 `U.unpark(carrier)`、`interrupt` 时 `carrier.setInterrupt()`。监视器所有者按 OS 线程计
  不变（持锁期间虚拟线程被 pin，不换载体）。
- 守护检查：生成器单测 `closure::handwritten::thread_local_lint`——`runtime/java_runtime/src` 中 `thread_local!` /
  `#[thread_local]` 只允许出现在 `exec_context.rs`（平台块与当前块指针）与 `java/lang/thread_impl.rs`（3 个载体槽）。
- TestVirtualThread 生成 + 编译通过（2924 JDK 类，与 T2 同；方案 A 尚在，本步不改闭包）。

**T4 删方案 A、虚拟线程接通 Continuation（2026-10-03）**
- `virtual_thread_impl.rs` 只余 6 个 `#[jvm_native]`（registerNatives + 5 个 notifyJvmti*），10 个 `#[jvm_boundary]` 承载方法删除；
  `VirtualThread` 移出 `[vm_boundary].classes` 与 `clinit_carried`，三份清单中 `VirtualThread` 0 次。`VirtualThread` 全部方法、
  `<clinit>` 的 `DEFAULT_SCHEDULER`（`ForkJoinPool` + `CarrierThread` 工厂）与 `UNPARKER`（`ScheduledThreadPoolExecutor`）按字节码翻译；
  `ContinuationSupport.isSupported0` 为 true，`ThreadBuilders.newVirtualThread` 走 `VirtualThread`。生成器未改动。
- 类数：HelloWorld 467 → 465（`VirtualThread` 只以布局入闭包——`Thread.yield` 的 instanceof；`<clinit>` 不在闭包内，静态字段
  读写为 `stub:` 存根）；TestVirtualThread 2924 → 2946（+22：调度器 / 载体线程 / 延时调度器 / Continuation 链）。
  TestVirtualThread 审计 `vm_boundary_methods` 94 → 84。以上为 8c79c524 基线上的对照；并入 74a8977e（JCA 注册补全等）后
  HelloWorld 464、TestVirtualThread 3123、TestContinuationPinned 3118、TestVirtualThreadCarrier 3119，`non_native_overrides` 0。
- **调用点推断核对**（`rava closure TestVirtualThread --why`）：`Continuation.enter:(Ljdk/internal/vm/Continuation;Z)V` 与
  `StackChunk.<init>:()V` 的入闭包链首边均为 `[handwritten] Continuation.enterSpecial`（手写体调用点推断），其上
  `Continuation.run@122` ← `VirtualThread.runContinuation@72` ← ForkJoinTask 分派；`VirtualThread.<clinit>` 经
  `ThreadBuilders.newVirtualThread@6` 的 new 入闭包。
- **`tail` 不被常量折叠**：字节码中 `tail` 的写只有 `postYieldCleanup` 置 null，非 null 值只来自手写体 `enterSpecial` 的
  `__set_tail(StackChunk::new()?)`。生成的 `Continuation.isStarted` 为 `Ok(!this.__get_tail().is_jvm_null())`（读字段，未折为常量），
  `run` 的 `isStarted()` 两个分支（`enterSpecial(this, false / true, ..)`）都保留；`isEmpty` 只在断言里用到，`$assertionsDisabled`
  折叠后为存根，符合预期。
- 边界 e2e（`60_real_threads`，expected 取 refjdk 21 两次一致的运行）：
  - **TestContinuationPinned**：`synchronized` 内 sleep（载体不变）、park（`getState` = WAITING，unpark 后在原载体恢复）、
    限时 sleep 被中断（TIMED_WAITING → InterruptedException、中断状态清除——走 `carrier.setInterrupt` 与载体侧清中断）、
    parkNanos 超时；两个虚拟线程同时触发 `<clinit>`，其中 sleep（载体不变、后到者等初始化完成，值 42,42）；之后未被 pin 的
    虚拟线程照常运行。
  - **TestVirtualThreadCarrier**：16 个虚拟线程各 50 次 yield / parkNanos / sleep 交替，每次恢复后核对 `currentThread()` 身份、
    `ThreadLocal` / `InheritableThreadLocal`、`isVirtual()`、名字；中断状态跨 yield 保持、park 遇中断立即返回、sleep 抛出并清除；
    `join(Duration)` 对 park 中线程超时返回 false、unpark 后 true；虚拟父线程的 ITL 传给虚拟子线程；1 万个虚拟线程各让出一次全部完成。
    输出与载体编号无关。
  - 本机只做诊断性运行（非验收）：两例与 TestVirtualThread 输出均与 expected 一致。

**T5 panic 跨栈传播与栈溢出（2026-10-03）**

- `rava_coro::catch_entry`（`catch_unwind` + `AssertUnwindSafe`）与 `PanicPayload`：协程入口函数体的统一包装。
  `continuation_impl.rs` 的 `coroutine_entry` 用它包住 `Continuation.enter(c, false)`：`Err(JvmError)` 照旧存 `thrown`，
  逃逸的 panic 载荷存入该 Continuation 的协程记录 `panicked`（与 `thrown` 同处，随记录切回载体）。`enterSpecial` 在
  完成时摘除记录、栈归池之后 `resume_unwind`，unwind 只在载体栈上继续，不穿过蹦床汇编帧。生成工作区为 `panic = "abort"`
  时钩子在协程栈上直接 `exit(101)`，`catch_entry` 不起作用、零成本。
- 新 `rava_coro/tests/panic.rs`（子进程即本测试二进制以 `--ignored --exact child_*` 重入，串行）：
  - 钩子与 `create_java_vm` 同形（默认钩子 + `exit(101)`），在同名线程上分别平台线程 panic、协程内 panic：退出码均 101，
    stderr 归一（去行列、OS 线程号）后同为 `thread 'vm-worker' panicked at tests/panic.rs` + 消息行；
  - `RUST_BACKTRACE=1` 下协程内 panic 的回溯含协程内帧、止于 `rava_coro::arch::aarch64::trampoline`，不含载体侧挂载帧；
  - unwind 构建（crate 单测即 unwind）：协程内 panic 经 `catch_entry` 捕获、切回载体 `resume_unwind`，被载体的
    `catch_unwind` 捕获 1/1，载荷消息不变。
- 栈溢出：sigaltstack 处理器与 `guard_page_overflow_aborts`（子进程 SIGABRT、stderr 含 `has overflowed its stack`）
  已在 T1 落地，本步复跑通过（本机 macOS 硬件 guard 启用；Linux < 6.13 无 `MADV_GUARD_INSTALL` 时跳过，由软件栈界拦截）。

**T4 回归修复：小闭包 `Thread__VTable::run` E0782（2026-10-04）**

- 现象：抽查 a3t-1e6933ee 中 HelloWorld、TestSleepParkClock、TestThreadStates、TestThreadInterrupt、TestVirtualClockPark
  编译失败，`error[E0782]: expected a type, found a trait`，位置 `thread_impl.rs` 的 `run_java_thread`。与平台无关，
  本机 HelloWorld emit + compile 即复现（T4 自验只覆盖了三个大闭包）。
- 根因：`run_java_thread` 以 vtable trait 完全限定路径 `Thread__VTable::run(&*t.vtable)` 调 `run()`。方案 A 时期手写
  VirtualThread 调 `spawn_java_thread`，任何闭包都经它推断出这条虚调用、保留 run 的槽位；T4 删方案 A 后，未启动
  线程的程序档案里没有 `start0`，也没有 run 的覆盖者，run 槽位被裁剪（`rava_moved = "plain"`），trait 上无此方法，
  而手写自由 fn 总是参与编译。
- 修复（手写侧，不动生成器）：改为方法调用形态 `t.run()`。槽位保留时，固有方法是 wrapper，按 vtable 分派；槽位裁剪时，
  档案内无覆盖者，plain 体就是正确目标。闭包推断照常沿该调用点。运行时中已无其他 `X__VTable::m(...)` 完全限定调用。
- 核对：
  - HelloWorld 编译通过，运行输出正确；
  - TestThreadStates、TestThreadOverridesSpec、TestThreadJoin（`extends Thread` 覆盖 run，槽位为 wrapper）输出与
    expected 一致；
  - `cargo check --target x86_64-unknown-linux-gnu`（hello_world 工作区全体、rava_coro 含测试）通过；
  - generator cargo test 0 失败。
- 教训：手写自由 fn 不随档案裁剪，只能引用任何档案下都存在的符号。虚调用一律写成方法调用形态，不写 vtable trait
  完全限定路径。自验须含小闭包（HelloWorld）。

**a3-T 现状与交接（2026-10-04，原执行者收尾，T6 由新代理接手）**

- 分支 a3t-vthread：
  - T1、T1b、T1b-2、T2、T3 已合入集成分支；
  - T4（1e6933ee）、T5（278c7319）与本修复待一次抽查后合入。
  - 抽查名单：TestVirtualThread、TestContinuationPinned、TestVirtualThreadCarrier、TestVirtualClockPark、TestThreadStates、
    TestThreadInterrupt、TestSleepParkClock、TestCommonPool、TestSynchronized、TestThreadUncaught、DeepCopy、HelloWorld。
- T6 未开始提交，草稿留在 worktree（未跟踪）：
  - `tests/e2e/60_real_threads/TestVirtualThreadScale.java`：10⁵ 个虚拟线程各 `sleep(2000)`；起跑时若已有线程完成则计
    lateStarts；输出 finished / sum / all sleeping at once / alive after join。
  - `tests/expected/TestVirtualThreadScale.txt`：refjdk 两次一致（4 行：`finished: 100000`、`sum: 4999950000`、
    `all sleeping at once: true`、`alive after join: 0`）；refjdk 墙钟 2.5 s、RSS 196 MB。
  - 本机诊断（debug 构建，非验收）：输出正确但 `all sleeping at once: false`，墙钟 14.6 s，峰值 RSS 2.66 GB，均超 T6
    指标（≤10 s、≤2 GiB）。起跑 10⁵ 个虚拟线程超过 2 s，每线程内存约 26 KB。
  - 接手先做剖析：每线程开销的分布（协程栈已提交页、Continuation / VirtualThread / StackChunk 对象、调度队列）与
    start 路径耗时，再按 §21.8.4 的 T6 行验收。百万作业放 `tests/perf/`，服务器单独跑。
- 未完成的跟进：T1b 审计手写运行时代码中的无界递归。

**T1b 审计：手写运行时代码的无界递归（2026-10-05，分支 vthread-t6）**

范围：`runtime/java_runtime/src`、`runtime/rava_coro/src`、`runtime/java_meta`、`runtime/rava_meta_tables`（运行期代码；
`rava_macros*` 只在编译期执行，不在范围内）。方法：按函数体抽调用名建图，列出自调用与强连通分量（名字级，含同名不同函数的
误报），逐个人工核对。判据：递归深度能否由程序（用户数据或用户构造的对象结构）推到无界，且两次递归之间不经过
Java 方法入口检查点（`__stack_check` / `__enter`）。经 Java 方法往返的递归（手写 → Java → 手写）每轮至少过一个检查点，
由软件栈界兜住，只要求一轮的手写帧落在 SHADOW（64 KiB）内。

| 位置 | 递归形态 | 深度上界 | 结论 |
|---|---|---|---|
| `java/lang/invoke/method_handle_ext.rs` `interpret` → `eval_function` → `interpret`（invokeBasic / invokeExact / invoke、resolvedHandle 分支） | LambdaForm 解释器按句柄组合层数递归，不经 Java 方法 | 用户构造的组合子层数（如循环里反复 `filterReturnValue`），**无界** | **已改**：`interpret` 入口判定栈界（`__stack_check()?`），耗尽抛 `StackOverflowError`，与 HotSpot 调深层句柄链同 |
| Java 对象的 drop（编译器生成的 drop glue：`Arc<dyn ObjectVTable>` → 字段 `__Handle` → `Arc` …；`JvmError` 的 cause 链同） | 释放最后一个引用时沿引用链逐层 drop | 对象图中仅被前驱引用的链长（`LinkedList` 节点、单链表、长 cause 链），**无界** | **需改，越界**：属对象模型（`object.rs` / `handle.rs` 与宏生成的字段载体），见下「drop 链」 |
| `java/lang/class_impl.rs` `for_class` ↔ `class_for_descriptor` | 数组类镜像按组件递归建 | 数组维数 ≤ 255（JVMS §4.4.1） | 有界 |
| `java/lang/class_impl.rs` `__name_assignable`；`array.rs` `__view_into` / `__array_elem_assignable` / `is_instance_of` / `__shallow_copy`；`try_array_view` 环 | 数组协变判定按组件 / 视图源递归 | 维数 ≤ 255；协变视图只包一层（视图再取视图走同类型快路径） | 有界 |
| `java/lang/reflect/array_impl.rs` `__multi_new` | 按维递归建多维数组 | 维数 ≤ 255（`newInstance` 先校验） | 有界 |
| `anno_pool.rs` `skip_value` ↔ `skip_anno` | 注解元素值嵌套 | 类文件中的注解嵌套层数（javac 产出为源码嵌套层数，受属性长度限制） | 有界 |
| `reflect_dispatch.rs` `reflect_invoke` → `injected_invoker::invoke` → `reflect_invoke` | 注入调用器转发到模板类 | 2 层（模板类不再是注入类） | 有界 |
| `meta.rs` / `vm_stack.rs` / `monitor.rs` / `exec_context.rs` / `continuation_impl.rs` 等其余自调用 | 同名委托（`libc::…`、vtable 方法、`Pins::pin`）或循环实现（`class_extends` 上溯 64 层封顶） | — | 误报，无递归 |

**drop 链（需改，未在本分支改）**：生成的对象是 `Arc` 计数，最后一个引用释放时由 drop glue 递归释放字段，深度等于只被前驱持有的
引用链长。release 下每层约 2 帧、百字节量级，dev 更大；1 MiB 的协程栈上数千节点的链即越过 SHADOW，Linux < 6.13（无硬件 guard）
时写入相邻槽，平台主线程（8 MiB）上 10⁵ 量级亦溢出。HotSpot 的回收不递归，这是 rava 独有的崩溃面。终态做法：对象引用载体
（`Object` / `__Handle` 的 `Drop`）在「本次释放的是最后一个强引用」时按执行流深度计数，深度超过阈值（如 32）即把该 `Arc` 移入
本载体的待释放队列而不就地递归，最外层 drop 返回前循环清空队列——释放顺序变化不可观察（无终结器），栈深恒定。改动面是
对象模型与宏生成的字段载体（S7-3 范围），已报协调者另行派发（2026-10-05 协调者转给 S7-3 代理，作为其后续小步 S7-3x）。

**pinned：TestContinuationPinned 的 `sync parkNanos: elapsed>=25ms` 偶发 false（2026-10-05，分支 vthread-t6）**

- 复现（作业 vt6-probe-8e2f10f7 / 03，kr2，dev 构建）：顺序 300 次失败 9 次，8 进程并行 320 次失败 26 次；绝大多数是 1d
  （pin 中 `parkNanos(30ms)` 提前返回），另有 1 次是 1c 在 60 s `sleep` 中途被观察到 `RUNNABLE`（同一机制：pin 中的停泊被虚假唤醒）。
- 机制：pin 中的停泊落在载体的 park 许可上（`parkOnCarrierThread` → `U.park`，HotSpot 的 Parker 同样挂在载体 JavaThread 上）。
  载体的许可同时被 ForkJoinPool 的唤醒协议使用：`signalWork` / `reactivate` 先写 `v.phase` 再看 `v.access == PARKED` 才
  `unpark(owner)`，工作线程在 `access = PARKED` 之后、真正 park 之前若已看到 phase 变化就不 park——发信方仍会 unpark，留下一个多余许可；
  `VirtualThread.unpark` 对 PINNED 线程的 `U.unpark(carrier)` 在被唤醒方已离开 park、尚未 `setState(RUNNING)` 时同理。多余许可留在载体上，
  下一个在该载体上 pin 停泊的虚拟线程立即返回。`LockSupport.parkNanos` 的规范允许虚假返回，测试第 1d 段的断言并不受规范保证。
- 对照（作业 vt6-race-91685ba3，诊断程序 `scripts/diag/PinnedRace.java` 循环 300 轮 1c+1d，顺序 1 进程 + 并行 8 进程，各 2700 轮）：

  | | 1c 有中断（intr） | 1c 无中断（nointr，s 改为 pin 中 parkNanos 10 ms） |
  |---|---|---|
  | 参考 JDK 21.0.11（HotSpot） | 提前返回 9 / 2700（0.33%） | 3 / 2700（0.11%） |
  | rava dev | 61 / 2700（2.3%），中途 RUNNABLE 5 | 36 / 2700（1.3%） |

  提前返回全部发生在 n 与 s 同载体时；无中断时同样出现，说明多余许可不只来自中断路径。**参考 JDK 自身也会失败**，rava 只是频率高
  约 7–10 倍：dev 构建（opt-level 0）的生成代码把上述两个窗口（工作线程 `access = PARKED` 到复位、被唤醒方 park 返回到置 RUNNING）
  拉长了。
- 结论：根因在 JDK 21 的设计（载体许可被调度器与 pin 停泊共用），不是 rava 运行时的偏差；monitor::park / unpark 的许可语义与
  HotSpot Parker 一致，运行时没有可做的忠实修正——任何「吞掉多余许可」的改法都会丢掉调度器真实的唤醒。处置需用户裁定：
  ① 测试第 1d 段按规范改成「循环 parkNanos 至截止时间」或放宽断言（违反「合法测试不改」，需用户批准）；② 维持现状，承认约 3% 的
  偶发失败，随生成代码提速（R1 / 运行档位）下降。已报协调者。
- 裁定与处置（2026-10-05 用户批准取 ①，作为「合法测试不改」的明示例外，只改这一段）：第 1d 段改为
  `while ((now = System.nanoTime()) < deadline) LockSupport.parkNanos(deadline - now);`，断言 `elapsed>=25ms` 与输出不变；
  参考 JDK 下输出一致。1c（pin 中 `sleep` 被观察到中途 RUNNABLE）属同一机制的更低频表现，不改测试。

**T6 剖析：10⁵ 虚拟线程的每线程开销与 start 路径（2026-10-05，分支 vthread-t6）**

- 口径：作业 vt6-prof-373114f1（jp2，x86_64 8 核 16 GB），dev 构建（opt-level 0、debug-assertions 开），`tests/perf/VirtualThreadScale.java`
  n = 10⁵，`scripts/diag/vt6_probe.sh` 计时与内存分两次跑，第三次挂进程内采样器（`scripts/diag/sampler.c`）。采样器以 ITIMER_PROF
  计时，内核节拍限制下实效约 250 Hz，多线程时同一时刻只挂一个待决信号，载体线程的样本偏少；主线程内部的占比可信，跨线程占比只作参考。
- 计时与内存：

  | 模式 | 建线程（主线程循环） | start | 全程墙钟 | user / sys | 峰值 RSS | 输出 |
  |---|---|---|---|---|---|---|
  | unstarted（只建不启） | 4.0–4.6 s（40–46 µs/个） | — | 7.7 s（含持有 3 s） | 3.7 / 0.4 s | 395 MiB（anon 314 MiB） | — |
  | split（先全建，再全启；体内 park） | 4.6–4.9 s | 12.9–15.9 s（130–160 µs/个） | 27.3 s | 67 / 4.4 s | 1.78 GB（PTE 198 MiB） | 正确，all sleeping true |
  | park（建与启同一循环） | 建 + 启 27.3 s | — | 36.5 s | — | 1.78 GB | 正确，true |
  | sleep（验收形态，sleep 2 s） | 建 + 启 23.5–26.7 s | — | 32.9 s | 124 / 7.3 s | 1.38 GB | `all sleeping at once: false` |

  split 的 start 每 1/10 进度约 0.85–1.3 s，基本线性；start50（+1.9 s）与 start100（+3.0 s）两处尖峰正对 `TrackingRootContainer`
  的 CHM 在 49152 / 98304 项时扩容（`transfer` 由主线程单线程完成）。没有随存活数增长的退化。
- 每线程内存（park 模式峰值，10⁵ 个同时停泊）：协程栈已提交约 10 KB/个（dev 帧大；热槽只留栈顶 16 KiB，冷槽整槽交还，生效正常）、
  页表约 2 KB/个（1 MiB 跨度的槽各自占页表页）、堆对象约 5.5 KB/个（未启动时约 3.2 KB/个：VirtualThread、Continuation、
  runContinuation lambda、Thread 字段与 FieldHolder 等）。合计约 17.8 KB/个，**峰值 1.78 GB 已在 2 GiB 内**，内存指标不是瓶颈。
- 主线程 CPU 分布（split，5314 样本 ≈ 21 s CPU / 27 s 墙钟，主线程基本跑满）：

  | 段 | 包含占比 |
  |---|---|
  | 建线程（`Thread.Builder.unstarted` → `VirtualThread.<init>`） | 13.6% |
  | `VirtualThread.start` | 49.6% |
  | 　其中 `TrackingRootContainer.onStart` → CHM keySet `add`（含扩容 `transfer` 18.9%） | 30.3% |
  | 　其中 `submitRunContinuation` → `ForkJoinPool.execute`（poolSubmit / push / signalWork → 唤醒载体） | 约 17% |
  | unpark 阶段（`VirtualThreads.unpark` → 再次 submit） | 26.6% |

  横切的叶子成本（主线程包含占比）：Unsafe 数组元素访问经擦除视图（`unsafe__impl::_erased_ref_array`）8.3%；
  `Object::__typed_null_of`（全局 `HashMap<&str, Object>` + SipHash，取类型化 null）6.4%；`String::from(&str)`（ldc 每次执行都
  新建并查全局驻留表）5.7%，sleep 模式 11.6%；`unsafe__impl::offset_slot`（字段偏移 → HashMap 查表并克隆两个 String，再按名匹配）
  4.8%；`monitor::unpark`（全局 `PARKERS` 互斥表 + SipHash）5.9%。协程本身（`enterSpecial`、`Stack::new`、`doYield`）在全部线程
  前 160 名里都没有出现，低于约 1.5%。载体线程上 `ForkJoinPool.scan` 占全部样本 34%。
- 结论：
  - start 路径的耗时来自翻译出的 JDK 代码（CHM、ForkJoinPool、Thread 构造）在 opt-level 0 下的常数因子，加上几处运行时协议的
    热点（ldc 驻留、类型化 null、Unsafe 偏移解码与数组视图、parker 侧表），**不是虚拟线程专属机制**，也没有随规模退化的算法问题。
  - 要达到墙钟 ≤10 s，主线程每个虚拟线程（建 + 启 + 唤醒）的 CPU 须从约 210 µs 降到约 80 µs 以内（2.7 倍）。本代理边界内能动的只有
    `monitor.rs` 的 parker 侧表（≤6%），远不够。
  - 剩余杠杆都在他人边界：① dev 档位的 opt-level（`emit/src/project/entry.rs`，V12）——预计最大；② ldc 按调用点缓存驻留实例
    （生成器，R1）；③ 类型化 null 改为按描述符静态缓存（`object.rs`，S7-3）；④ Unsafe 字段偏移解码与引用数组访问去掉逐次查表与
    String 克隆（`unsafe__impl.rs`，S7-3「Unsafe 槽位」）。已报协调者裁定。
- opt-level 杠杆实测（作业 vt6-opt1-4a983fe8，jp1，提交 4a983fe8，环境变量 `CARGO_PROFILE_DEV_OPT_LEVEL=1`
  `CARGO_PROFILE_DEV_DEBUG_ASSERTIONS=false`，不改 entry.rs）：dev 构建 8:32、峰值 5.7 GiB，二进制 426 MiB。

  | 模式 | opt 0（vt6-prof-373114f1） | opt 1 |
  |---|---|---|
  | split 墙钟 / 峰值 RSS | 27.3 s / 1.78 GB | 6.4 s / 0.98 GB（建 0.69 s、启 2.07 s、唤醒到 join 1.46 s；sleeping true） |
  | sleep 墙钟 / 峰值 RSS | 32.9 s / 1.38 GB | 8.6 s / 1.43 GB（建 + 启 3.38 s；sleeping **false**） |

  opt 1 下墙钟与内存都过线，剩 sleep 模式的 `all sleeping at once`：主线程建 + 启 10⁵ 个须在 2 s 睡眠窗口内完成（≤20 µs/VT），
  实测约 34 µs/VT，还差 1.7 倍，需要杠杆 ②–④ 与 CHM / ForkJoinPool 常数继续压。
- 分工（2026-10-05 a3-T6 收尾）：
  - 杠杆 ② ldc 逐调用点缓存驻留实例、③ 类型化 null 按描述符静态缓存、④ Unsafe 字段偏移解码与引用数组访问免查表 / 免 String
    克隆，三项移交 **R1（运行性能线）**，不在 a3-T 内实施。
  - 杠杆 ① dev 档位 opt-level 待用户决定（实测数据见上表：opt 1 构建 8:32、峰值 5.7 GiB、二进制 426 MiB）。
  - TestVirtualThreadScale 的达标（sleep 模式 `all sleeping at once: true`，≤20 µs/VT）依赖 R1 与 opt-level 决定；在此之前该例
    预期失败，不阻塞 a3-T6 合入。a3-T6 本身的交付止于：monitor 侧表分片（4a983fe8）、剖析结论与 opt-level 实测、
    TestContinuationPinned 第 1d 段按规范改写。

### 21.9 a3 实施记录（2026-10-05，分支 c1d-a3）

口径：HelloWorld `--stop-after emit` 审计；闭包类数为本机 `rava closure` 实测，前后在同一提交上只差该子项的改动（反向应用补丁取底数）。
HelloWorld emit JDK 类数全程 467，各子项均未增加。

| 子项 | 提交 | HelloWorld `vm_boundary_methods` | 闭包类数（前 → 后） | 说明 |
|---|---|---|---|---|
| U0 | 33283267 | 77 → 77 | — | Unsafe 中 JDK 为 `ACC_NATIVE` 的 33 个方法属性改 `#[jvm_native]`，只改标注（审计按 ACC_NATIVE 标志计数，故不变） |
| U1 | 62372b34 | 77 → 62 | TestDirectBuffer 524 → 524（字节码方法 +13） | 原始内存 15 个公开包装按字节码翻译，手写只留 `ACC_NATIVE` 的 `*0` 族；新增边界用例 TestDirectBufferPaging（跨页 putLong / getLong、两种字节序交换拷贝、分配清零，expected 取 JDK 21） |
| U2 | ab0f5fcc | 62 → 42 | TestAtomics 3114、TestCompletableFuture 3119、TestDirectBuffer 524，均不变 | 原子 / 访问序变体 17 个与栅栏 3 个按字节码翻译；`vm_intrinsics.toml` 中按 Java 层原子成员登记的 `array_writes` / `memory_reads` 条目（9 行）删除，内存效果由内层 native 的既有登记承担 |
| U3 | f91c7b90 | 42 → 33 | HelloWorld JDK 467、TestAtomics 3114、TestDirectBuffer 524，均不变 | 布局 / 偏移 / 类初始化包装 10 个与 `<clinit>` 按字节码翻译；补 native 落点 `registerNatives`、`objectFieldOffset0/1`、`staticFieldOffset0`、`staticFieldBase0`、`arrayIndexScale0`、`ensureClassInitialized0`、`shouldBeInitialized0`，`arrayBaseOffset0` 改独立实现。Unsafe 移出 `[vm_boundary]` 与 `clinit_carried`，unsafe 系 `#[jvm_boundary]` 归零 |
| L1 | 3e15de4b | 33 → 29 | TestDirectBuffer 524 → 534、GZIPTest 517 → 527；ServiceLoader / AppClassLoader / ZipFs 用例不变 | BootLoader `loadLibrary` / `hasClassPath` / `loadClass` / `loadClassOrNull` 按字节码翻译。+10 类是 JDK 本地库装载路径本身（`NativeLibraries$LibraryPaths` / `$NativeLibraryContext` / `$CountedLock` / `$Unloader` 等），属字节码取代截断的必要部分；落到既有 native `findBuiltinLib` / `load` 与 `findBootstrapClass` |
| X1 | 95826e8f、44889a8b | 链外 | TestMethodHandleCombinators 3106、TestBmhDynamicSpecies 3102、TestDynamicProxy 3105、DeepCopy 3382、TestSerialProxyForm 3383，均不变 | `InvokerBytecodeGenerator` 只手写两个类定义点；`isStaticallyInvocable`×3 与 `lookupPregenerated` 删手写并撤 `[[intrinsic]]` 登记（调用面只有截断的生成链与引导类 assert，后者经 `$assertionsDisabled` 折叠）。IBG 移出 `[vm_boundary]` 与 `clinit_carried`（`<clinit>` 按字节码翻译，方法 +3）。已登记 `class_definition` 的手写属性改 `#[jvm_native]`（与 `makeInjectedInvoker` 同口径）；`Proxy$Dyn.__vm_proxy_invoke` 是 VM 钩子，去掉属性 |
| X2 一 | 22528eeb | 链外 | — | `CDS.initializeFromArchive` 是 `ACC_NATIVE`，属性改 `#[jvm_native]` |
| X2 二 | e336f8ef | 链外 | TestUnixFileNatives 3127 → 3119；TestFilesApi / FileIODemo 3102 → 3095 | `FileSystems.getDefault` 按字节码翻译，删 `file_systems_impl.rs`，FileSystems 移出 `[vm_boundary]` 与 `clinit_carried`。`getDefaultProvider` 的系统属性分支经属性事实折叠，不展开反射链。手写返回的一般 `FileSystem` 值曾让 jrtfs 实现入闭包；翻译后取精确值，净减 7–8 类 |

`#[jvm_boundary]` 全仓 123 → 33：vm_impl 8、module_impl 7、class_loader_impl 6、jce_security_impl 6、boot_loader_impl 2、module_layer_impl 2、class_impl 1、class_impl/members 1。

**调用点事实不随被调方改成字节码而迁移**：`name_resolvers.offset` 折叠（`facts.rs::field_offset`）、`class_initializers`、
`static_offset_getters`、`class_loads` 都按成员在调用点或可达时生效，与被调方是手写还是字节码无关。所以这些事实仍登记在公开包装上。
内层 native 只拿到形参，挂在那里永远不会触发，§21.1 U3 行「识别点改到 `objectFieldOffset1`」不需要。
U3 之后，emit `class_writer/methods.rs` 的 `core_` 适配已无运行时用户，由 Z 一并删除。

**未完成项的阻塞（实测判定）**

- **a3-C（`Class.enumConstantDirectory`）阻塞于反射调用精度。** 翻译后经 `getEnumConstantsShared` 走
  `getMethod("values").invoke(null)`，TestEnumBasic 闭包 472 → 3103。每个枚举生成的 `valueOf(String)` 都调
  `Enum.valueOf`，所以凡是用到枚举 `valueOf` 的程序都会多出约 2600 类。EnumSet / EnumMap 用例（`getUniverse`）已在承担同一代价（3103）。
  前置：「已知类上已知名的静态无参方法的反射调用」按直接调用建边，不展开 `Method.invoke` 的访问器 / 句柄体系，
  归 C1d 精度项。前置完成后 a3-C 直接删手写（已验证 `constant_directory_entries` 可随之删除）。
- **a3-L2（ClassLoader 资源 6）与 L1 余项（`findResourceAsStream`、`getServicesCatalog`）阻塞于 boot layer 第 2–3 步。**
  翻译路径 `BootLoader.findResource` → `BuiltinClassLoader.findResource` 依赖 `packageToModule` / `nameToModule`
  与系统模块读取器（jimage）。TestClassResourceStream 要求 `getSystemResource("java/lang/String.class")` 非 null、
  模块资源（currency.data 等）可读，引导层未建时翻译即回归。
- **SecurityManager 移出 `[vm_boundary]` / `clinit_carried` 阻塞于 boot layer。** 手写只有 native `getClassContext`，
  但 `<clinit>` 调 `ModuleLayer.boot()` → `addNonExportedPackages` 遍历层内模块描述符，依赖 Module / ModuleLayer 回到字节码。
- **a3-V（VM 10）需要先做设计决定。** 翻译后 `initLevel()` 读静态字段，现行的线程内档位覆盖
  （`__vm_at_init_level`：saveProperties 段为 0、惰性 initPhase3 段为 3）无法用单一字段表达。终态两条路：
  ① 字段读取钩子带返回值（生成器新能力：`[vm_state.field_hooks]` 现只在访问前调用钩子，不改读出值）；
  ② 启动序列按 HotSpot 改为急切执行（initLevel 1 → 2 → 3 → 4），但 initPhase3 的 `initSystemClassLoader` 会进入所有闭包。
  `[facts.returns]` 中 isBooted / isModuleSystemInited / isJavaLangInvokeInited 按成员登记，翻译后照常生效，不影响闭包。
- **X2 余项 JceSecurity 6 需要嵌入 java.home 的 NIO 虚拟层。** 翻译后 `<clinit>` 经 `Files.isDirectory` / `isReadable` /
  `newDirectoryStream("{default,exempt}_*.policy")` / `Files.newInputStream` 读取 `${java.home}/conf/security/policy/<crypto.policy>`。
  伪 java.home（`jdk_resources`）现只接入 `FileInputStream.open0`。终态做法是在 `UnixNativeDispatcher` 的
  `stat0` / `lstat0` / `access0` / `opendir0` / `readdir0` / `closedir` / `open0` 与文件分派器 read / size / close 上接入嵌入树，
  并嵌入 JDK 的 `conf/security/policy/{unlimited,limited}` 文件。`getVerificationResult` 对引导类 provider 走
  `ProviderVerifier`（codeBase 为 null），不展开 JAR 签名校验栈。单独立项，验收见 §21.3 X2 行。
- Module 7 / ModuleLayer 2 / `Class.getModule` 归 boot layer 第 2–3 步；Z 在全部完成后进行。

审计余量：HelloWorld `vm_boundary_methods` 29 = VM 10、Module 7、ClassLoader 6、BootLoader 2、ModuleLayer 2、Class 2。

**a3x2 验证后的两项修正（2026-10-05）**

- **TestCharsetNamedStreams：`ServiceConfigurationError: Provider sun.nio.cs.ext.ExtendedCharsets not found`（005282bf）。**
  L1 把 BootLoader.loadClass 改为字节码翻译后，`ServiceLoader.loadProvider` → `Class.forName(Module, String)` 按
  `module.getClassLoader()` 分派。原手写 `Class.getModule` 对所有类都返回同一个无名模块，其加载器为 null，于是
  走 `BootLoader.loadClassOrNull` → `findBootstrapClass`。平台加载器定义的类在这里按定义加载器表返回 null。
  修正后 `getModule` 对非引导加载器定义的类返回其定义加载器的 `getUnnamedModule()`，保持
  `getModule().getClassLoader() == getClassLoader()`，provider 经 `ClassLoader.loadClass(Module, String)` 的
  `findLoadedClass` 命中。
- **closure_independent_of_hash_seed：TestSerialLookupPairing 种子 0 比种子 1 多 `FinalReference`（780ba97d，生成器）。**
  - **触发点：** `Lookup.findVarHandle` 内的 `resolveOrFail(byte, Class, String, Class)` 调用点（反射式字段写入的形状规则）。
    若只有点名 `"head"` 的调用方先接入，名字形参在常量格上是 `"head"`，而类形参已合流为非常量。原实现先把这个
    常量名并入点名、再试配对；类不是字面量，于是按名放开全部 `head` 字段，且放开不撤回。
  - **后果：** `ReferenceQueue.head` 不再折叠，`poll` / `poll0` 的非空分支存活，`instanceof FinalReference` 生效。
  - **为何与顺序有关：** 终态下名字形参抬为非常量、登记字段配对，不再按名放开，所以结果取决于调用点接入的先后。
  - **修正：** 本方法字面量总放开；常量格给出的名字只在未登记配对时放开。
  - **验证：** Linux 参考 JDK 下种子 0 / 1 / 2 的类 / 方法 / 反射成员集合一致。a3 合入前的运行时 3393 类，
    a3 当前运行时 3386 类，均不含 `FinalReference`。
  - **为何在 a3 出现：** a3 合并集成分支（d7315af6）后调用点接入次序改变，缺陷才暴露。clsfact 分支同样失败，
    同属这一生成器缺陷。

## 22. jar/URL 来源精度：现状 / 交接（2026-10-04，c1d-p0 74a8977e）

C1d-a 按子代理时限（tasks.md 执行约束第 8 条）在此交接。本项**尚未改代码**：分支 c1d-p0 与集成分支 74a8977e 同步，
没有未提交改动。下面是实测、来源分析、批准的方案与验收口径。

### 22.1 `--cut` 实测（StockTrans，反事实切除，不是改动）

同步前基线 3285 类（同步后复测 3286）。`F` = `java/net/URL$DefaultFactory.createURLStreamHandler:(Ljava/lang/String;)Ljava/net/URLStreamHandler;`，
`R` = `jdk/internal/loader/URLClassPath$3.run:()Ljdk/internal/loader/URLClassPath$Loader;`。

| 切除内容 | 类数 | 变化 |
|---|---:|---:|
| ① `R@97` / `R@139`（两处 `new JarLoader`） | 3281 | −4 |
| ① 再加 `R@127`（`new Loader`） | 3279 | −6 |
| ② 只切 `sun/net/www/protocol/jar/Handler.openConnection` | 3264 | −21 |
| ① + ② | 2860 | −425 |
| 直接切 `JarFile` 的三个校验入口 | 2916 | −369 |
| 同步后复测 e1：`F@120` / `F@124`（`new jar.Handler`）+ `R@97` / `R@139` | 3283 | −3 |
| e3：e1 再加 `F@176`（反射 `newInstance`） | 2860 | −426 |
| e2：e3 再加 `R@127` | 2858 | −428 |
| e4：只切 `F@176` + `R@97` / `R@139` / `R@127` | 3281 | −5 |

- 签名校验链（PKCS7、SignerInfo、X509Key、AlgorithmId、SignatureFileVerifier、ManifestEntryVerifier、JarVerifier 等）
  约 369～428 类，要**同时**堵住 class path 来源和协议处理器来源才会出闭包，单堵任一条几乎不降。
- e1 / e3 说明 DefaultFactory 里**两条**分支都产出 jar `Handler`：`@120` 的 `new`，以及 `@136..@184` 按
  `"sun.net.www.protocol." + protocol + ".Handler"` 反射加载（protocol 名字推不出时按前缀放开）。只剪常量分支不够。

### 22.2 来源一：class path（甲）

- `ClassLoaders.<clinit>` → `URLClassPath("")` → `toFileURL("")` 得到当前目录的 URL。末尾是否带 `/` 取决于运行期
  `File.isDirectory()`，分析器推不出，`$3.run` 的 JarLoader 分支因此活着。
- 另一条直连：`URLClassPath.<init>:(Ljava/lang/String;Z)V@185` 无条件 `jarHandler = new jar.Handler()`，经
  `JarLoader.<init>` → `JarLoader.newURL` → `URL.<init>(String,String,int,String,URLStreamHandler)` 显式传入 handler，
  写进 `URL.handler`（`--flows '@trace:sun/net/www/protocol/jar/Handler'` 第 1～23 条）。JarLoader 不可构造时这条随之断开。
- JDK 语义：`java.class.path` 为 `""` 时唯一元素是当前目录；HotSpot 在 main 之前建好该 URL，取不到当前目录时启动失败
  （`Properties init: Could not determine current working directory.`）。所以 app class path 恒为 `file:<cwd>/`，走 FileLoader。
- **rava 与 JDK 的两处偏差**（甲要先对齐）：
  1. `ClassLoaders.<clinit>` 的时机：rava 经 `ClassLoader.scl` / `Thread.contextClassLoader` 的字段钩子
     `__vm_init_phase3` 首次访问时才跑（FS-C2，`docs/plans/2026-10-02-fs-c2-app-classloader.md`）。程序若在此之前删掉
     自己的当前目录，URL 就不带 `/`，会建 JarLoader。终态：与 HotSpot initPhase 一致，至少在 main 之前建立。
  2. `user.dir` 取不到：`runtime/java_runtime/src/java/lang/system_impl.rs:238` 回退为空串；JDK 是启动失败。终态：启动失败，消息同 HotSpot。
- 对齐后由清单声明「启动目录是目录」，分析器沿路径值推出 URL 以 `/` 结尾。单独收益只有 −4～−6，要和乙一起才兑现。

### 22.3 来源二：URL 协议处理器（乙）

- `DefaultFactory.createURLStreamHandler(protocol)` 是 `defaultFactory` 单例上的接口调用，不按调用点区分；protocol 形参
  按全部调用点汇合，`hashCode` 的 `lookupswitch`（`[facts.string_ops]` 已能对常量折叠）因此三支（file / jar / jrt）
  加反射分支全活。`new URL("file", "", path)` 这种常量协议也一样。
- `URL.handler` 是全局一个字段，`URL.handlers`（Hashtable 缓存）按协议取值也不分键。所有
  `URL.openConnection / openStream` 都会派发到 `jar.Handler.openConnection` → `JarURLConnection` → `JarFile` 校验链。
- 活的 URL 构造调用点（`/tmp/c1dj_callers.py` 列出，见 §22.7）：
  - 常量协议：`ParseUtil.fileToEncodedURL@84`（`"file"`）、`file.Handler.openConnection@119`（`"ftp"`）；
  - spec 有常量前缀：`JavaRuntimeURLConnection.toJrtURL@32`（`"jrt:/" + …`）；
  - 以上下文 URL 解析（protocol / handler 取自 context 对象）：`FileLoader.getResource@13`、`Loader.findResource@13`、
    `Loader.getResource@13`、`JarLoader.*`、`JarURLConnection.parseSpecs@90`；
  - spec 来自属性或字段、前缀推不出：`NativePRNG.getEgdUrl@47`、`SeedGenerator$URLSeedGenerator.init@8`。spec 是安全属性
    `securerandom.source`（缺省 `file:/dev/random`），后者会 `openStream`。乙之后它仍会让「推不出 → 全分支」成立，
    要么对安全属性取值建模，要么 URL 按对象区分后只让这一个对象派发到 jar；这是 −425 之外剩余差距的首要候选。
- 解析式构造器 `URL.<init>(URL,String,URLStreamHandler)` 在 `@386` 调 `getURLStreamHandler(this.protocol)`；protocol
  由 `spec.substring(start, i)` 加 `lowerCaseProtocol` 得到（`@180..@204` 的逐字符扫描），或取 context 的 `protocol`。
  名字求值（`pstrs` / `name_parts`）求不出这个值，需要在构造调用点上从 spec 的前缀常量 / 拼接段求协议。

### 22.4 来源三：`BuiltinClassLoader.ucp` 合流

- `BootLoader.findResources` → `ClassLoaders.bootLoader().findResources` → `ucp.findResources` → `URLClassPath$3.run`。
  真实运行时 boot 的 `ucp` 为 null（`jdk.boot.class.path.append` 不存在，已折叠为 null），这条链是 `ucp` 字段把 boot 和 app
  两个对象的值合在一起造成的。按对象（分配点）的字段精度修掉后，boot 侧 `ucp = null` 可见。
- 注意 a3-L2 的联动：a3-L2 把 `ClassLoader.getResource*` 6 个手写方法改为按字节码翻译（经 `BuiltinClassLoader` → `URLClassPath`），
  合入后 app 加载器的资源查找成为真实路径（FileLoader），本项的正确性论证要随之复核。

### 22.5 批准的方案（协调者 2026-10-03 定，三步，全部终态做法，不加类名特判，事实写进 `runtime/java_runtime` 清单）

1. **乙：URL 协议精度（先做）**
   - 协议字符串按调用点流到 `getURLStreamHandler` 和 DefaultFactory 分支，常量协议只放行对应分支；
   - `URL.handlers` 缓存表按协议键放行，扩展现有按键查找闸门（`engine/keyed.rs`、`[facts.keyed_lookups]`）；
   - `URL(spec)` 等从字符串解析出协议的调用点要能求出协议（前缀常量 / 拼接串），求不出就退回全分支；
   - `URL.handler` 字段按对象精度区分，不再全局合流。
2. **甲：class path 事实（其后）**：先对齐 §22.2 的两处偏差，再由清单声明「启动目录是目录」，分析器推出 URL 以 `/` 结尾。
3. **ucp（单独一步）**：按对象（分配点）的字段精度，见 §22.4。

C1d-a 读代码后的设计备忘（供接手者参考，未实施，不是批准内容）：
- 把 `URL.getURLStreamHandler:(Ljava/lang/String;)` 登记为按键查找入口，在**调用点**设闸门，可以一并覆盖缓存表、
  factory 与 DefaultFactory 三条返回路径。处理器类自身的键没有构造器形参可取，可在清单中声明 JDK 的处理器命名约定
  `sun/net/www/protocol/{协议}/Handler`（DefaultFactory 反射分支与 `lookupViaProperty` 本就按此命名）；不符合约定的处理器类
  键为任意（放行）。`java.base` 不导出这些包，用户 factory 拿不到内建处理器对象，因此该事实成立。
- 现有对象敏感只覆盖「容器形态」类（`engine/classes.rs` `container`）。URL 要按分配点区分，需要一个结构判据，例如
  「实例字段的声明类型是某个按键查找入口的键类」。判据直接取自清单事实，不列类名。`ucp` 的 null / 非 null 需另找判据。
- `@317` 处的协议是 `lowerCaseProtocol(param0)`：常量实参可经常量实参求值折叠（`consteval.rs`）；名字集合需要对每个名字
  做一次常量实参求值。`@386` 处只能按对象取协议，见 §22.3。
- 已有机制：选择子形参按调用点克隆（`engine/selector.rs`，目前只认 int 族静态形参）；分派转发方法按调用点克隆
  （`engine/forward.rs`）；上下文选择集中在 `engine/ctxsel.rs`。

### 22.6 验收口径

- StockTrans、DeepCopy 接近 −425 的上限（≤2870 左右），并报告剩余差距的来源。
- 真实可达场景按真实行为工作，各补一个边界 e2e，expected 取 refjdk：用户 `URLClassLoader(jar)`、`jar:` URL 的 `openStream`、
  `new JarFile` 读条目，最好再加一个签名 jar 校验。
- JCA 四例（TestEcSignVerify、TestMacHmacDigest、TestRsaSignVerify、TestX509ExtensionsParse）与 TestJcaIndirectDigest 的必需类不丢。
- 按步小步提交：乙先、甲后、ucp 单独一步。每步推 origin + github、报完整哈希，由协调者发抽查。本机只编译 / 单测；
  重命令走 `heavy_lock.py`，`CARGO_BUILD_JOBS=2`。

### 22.7 测量脚本（项目结束后删）

- `/tmp/c1dj_cl.sh <Test.java> <tag> [rava closure 额外参数...]`：首次把测试源 javac 到 `/tmp/c1dj_cls_<名>/`，再以本 worktree 的
  `build/analyzer-target/release/rava closure` 加两个 `--image` 目录（jimage `dd9c2c51d6dea9a9/only/java.base`、vmsupport
  `37564758565473ec/java.base`）跑闭包，产物 `/tmp/c1dj_<名>_<tag>.{json,log,err}`，打印类数 / 方法数 / 耗时。示例：
  `python3 /Users/yuwei/dev/workspace/heavy_lock.py /tmp/c1dj_cl.sh tests/e2e/23_algorithms/StockTrans.java e2 --cut "$F@120" --cut "$R@97"`；
  `--flows '@trace:<类>'` 的记录写在 `.err`。同一时间只跑一个闭包。
- `/tmp/c1dj_callers.py <closure.json> <被调正则>`：对闭包内字节码方法 javap，列出不在 `dead_pcs` 内的活调用点。
- `/tmp/c1dj_cls_StockTrans/`：StockTrans 的类文件。

### 22.8 排队项与原 ② 的结论

- **③ Latin-1 语言折叠**（a5-4e：`StringLatin1.toLowerCase` 只在语言为 tr / az / lt 时走 `toLowerCaseEx` → ICU）：排在本项之后。
- **原 ②（容器元素的 Object 方法，即通用 open 值精度）**：结论是它不是 jar 链的独立来源，而是同一类精度问题。
  §22.3 的 `URL.handler` 全局合流、§22.4 的 `ucp` 合流都属于「按对象（分配点）区分字段值」。jar/URL 项先按该判据做 URL
  与 `ucp`；通用的容器元素 Object 方法另行立项，不并入本项。

### 22.9 状态更正（对照 docs/tasks.md，2026-10-04）

- **a2**：已合入（62f46bb2，c1d-p0 b4669206，抽查 9/9）。§20.8「再往下的目标在 a2 实测之后定」不再是待办；
  a2 续项（initPhase2 膨胀定位 → `[[boot_init.phases]]` → boot layer 步骤 2–5）见 tasks.md。
- **a3**：总体进行中，验收仍是审计数 `vm_boundary_methods` 归零（§21.7）。a3-T 由 a3t-vthread 推进：T1（rava_coro）、
  T1b、T1b-2、T2、T3 已合入，T4（删方案 A + 接通 Continuation）、T5（panic 跨栈）与小闭包 E0782 修复待抽查合入，T6 转新代理（交接见 §21.8.5 末「a3-T 现状与交接」）；其余子项状态以 tasks.md 为准。
- **a5**：a5-1～a5-3（OOB 关系推理）未开工。a5-4 已做：s1 构造器查找、s2 instanceof 否定分支、JCA 请求点值流（51a4d8c5）。
  a5-4b 即本节的 jar/URL 来源精度，转新代理；a5-4e 即 ③。

### 22.10 乙 现状（2026-10-04，c1d-p0，按子代理时限停下报告）

**已提交（闭包结果不变）**：
- 清单：`[facts.keyed_lookups]` 新增 `URL.getURLStreamHandler:(Ljava/lang/String;)` 按键查找入口，`class_pattern =
  "sun/net/www/protocol/{}/Handler"`（处理器类的键取自 JDK 命名约定，不符合约定的类键为任意），`scheme_sites` 声明
  解析式构造器 `URL.<init>(URL,String,URLStreamHandler)` 在其中按 spec 形参（下标 1）解析出的协议作键；
  `[facts.string_ops]` 新增 `String.toLowerCase(Locale)` → `to_lower_case`（只折叠不含 `I` 的 ASCII 串，避开语言相关折叠）。
- 引擎：`manifest/keyed.rs`（`class_pattern` / `scheme_sites` / `pattern_key`）；`engine/keyed.rs` 调用点键改走
  `scheme_keys`、处理器类键先查 `pattern_keys`；`engine/keyed_scheme.rs`（新）按 JDK `URL(spec)` 的协议识别规则从拼接段
  候选模式求协议名，判定落进任意串段 / 非 ASCII / 超出组合上限时退回任意键；`engine/pstrs.rs` 抽出 `param_patterns`
  （形参槽的拼接段候选模式）。
- 单元测试：协议常量求值（`keyed_scheme::constant_specs` / `prefixes_before_wild` / `pattern_sets`）、按键放行
  （`keyed::scheme_keys_release_matching_handlers_only`）、推不出时退回全分支（同上 + `pattern_sets`）、
  `consteval::string_ops_on_constants` 的 toLowerCase 断言。
- 边界 e2e：`tests/e2e/53_io_api/TestUrlProtocolOpen.java`（运行期建 jar 的 `jar:` URL `openStream` / 相对解析 / 缺条目、
  `file:` 读取含大写协议与分段构造、未知协议与无协议的 `MalformedURLException`），expected 取 refjdk，两次一致。

**实测**：StockTrans 3286 类 / 19584 方法、DeepCopy 3281 类 / 19545 方法，与基线相同；HelloWorld 468、
TestBuiltinUrlProtocol 3078 不变。闸门本身正确，但 `URL.<init>` / `getURLStreamHandler` 仍按方法汇合全部调用点的协议，
键集为全集，因此不降。

**试过并撤下的两条路（均导致闭包爆炸、300 s 超时）**：
1. 名字集合逐名常量实参求值（`@317` 处 `lowerCaseProtocol(p)` 对每个候选名求值）；
2. URL 按分配点区分（`container` 判据扩成「实例字段声明类型是某按键查找入口的键类」），克隆内常量形参经 absint 折叠。

**根因**：两条路都让协议名从「任意串段（Wild）」变成**精确字面量**，流进 DefaultFactory `@136..@184` 的反射
`Class.forName("sun.net.www.protocol." + p + ".Handler")`。Wild 只匹配闭包内已有类；精确名会把类**命名入闭包**并初始化、
暴露构造器。精确集里出现 `ftp`：来源是 `file.Handler.openConnection@119` 在 host 非空且非 `localhost` 时
`new URL("ftp", host, …)`——JDK 语义下 host 不可知时确实可达。`ftp.Handler` → `FtpURLConnection` → `http` 栈整体进入，
`drain_flows` / hubs 采样显示是闭包规模膨胀而非分析变慢。

**接手方向（终态）**：
- 需要 URL host 精度：证明经 `openConnection` 打开的 file URL 的 host 恒为 `""` / `localhost`（`ParseUtil.fileToEncodedURL`
  等构造点 host 是常量，问题在 host 字段同样全局合流），才能让 `@119` 的 ftp 分支死掉；这与「URL 按对象精度」是同一件事，
  宜和按对象字段精度一起做。
- 或者先定一个原则：按键闸门的键求值与反射类查找的名字求值是否允许不同精度（键求精确、类查找保持 Wild 口径）。这是
  协调者层面的取舍，未擅自实施。
- 剩余 jar 路径（与乙无关）：`URLClassPath.<init>@185` `jarHandler` → `JarLoader` → `URL` 的 P5 构造器显式传 handler，
  `@317` / `@386` 闸门看到的是 JarLoader 的 URL 对象（甲）；`FileLoader` / `Loader` 的 spec 来自 `ParseUtil.encodePath`，
  求不出协议（退回全分支，正确）。
- `securerandom.source`：运行期安全属性（`java.security` 文件 + `Security.setProperty` 可改），分析器无法求值，
  `SeedGenerator$URLSeedGenerator.init` / `NativePRNG.getEgdUrl` 两处保持全分支；要降需对安全属性建模（清单声明缺省值不够，
  用户可在运行期改写），不在乙范围。

## 23. a2 续：initPhase2 膨胀定位与早退检查按分析期事实求值（2026-10-05，分支 c1d-a2c）

### 23.1 实测定位（HelloWorld，`rava closure --root java/lang/System.initPhase2:(ZZ)I`）

基线 469 类 / 1813 方法；加 initPhase2 根后 3283 类 / 19423 方法（+2814，与 StockTrans 的 jar / URL / JCA / stream 区域重合）。
顶层 `--cut` 结果：

| 切点 | 类数 |
|---|---:|
| 无切点（initPhase2 根） | 3283 |
| `initPhase2@0`（`ModuleBootstrap.boot`） | 3106 |
| `initPhase2@16`（`logInitException`） | 3284 |
| `boot@64`（`boot2`） | 3109 |
| `initPhase2@0` + `@16` | 469 |

两道独立闸门，任一单独打开都把 ~2600 类带进来：

- **甲 `logInitException` 打印路径**：`printStackTrace` 实参在 HotSpot 缺省调用里恒为 `false`，分析期不知道。
  链：`logInitException@50 printStackTrace(PrintStream)` → `getOurStackTrace` → `StackTraceElement.of` → `computeFormat` →
  `StackTraceElement$HashedModules.<clinit>` → `Configuration.findModule` → `Collection.stream` → `StreamOpFlag.<clinit>` →
  `EnumMap` → `Class.getEnumConstantsShared` → `Method.invoke` → 注解解析 → `Proxy$Dyn` → `AnnotationInvocationHandler.toStringImpl`
  → `PlatformLogger` / `LoggerFinder` → `SecurityConstants.<clinit>` → `SocketPermission` → `toLowerCase` →
  `ConditionalSpecialCasing` → ICU `Normalizer` → `getResourceAsStream` → `URLClassPath` → `JarFile`。
- **乙 `boot` / `boot2` 早退检查未折叠**：
  1. `ModulePatcher.patchIfNeeded@90` → `JarFile`（← `SystemModuleFinders.toModuleReference` ← `of@100` ← `boot2@240`）；
  2. `ModulePatcher.<init>@90` indy → `Paths` / `FileSystems` / `ReferencePipeline`（← `initModulePatcher@16` ←
     `ModuleBootstrap.<clinit>@28`）；
  3. 解析分支 `boot2@416..872`：`Configuration.resolve` ← `limitFinder@12` ← `boot2@707` → `Resolver` → `ModulePath.readModule`
     → `TempFileHelper` → `SecureRandom` → `sun/security/jca/Providers`。

字节码事实（JDK 21）：`decode(prefix,sep,bool)` 键为 `prefix + 0` 的 StringBuilder 拼接，`@20 getAndRemoveProperty` 为 null 时
返回 `Map.of()`；`addModules()` 键 `"jdk.module.addmods." + 0`，null 时返回 `Set.of()`；`ModulePatcher.<init>` 对空 map 置
`this.map = Map.of()`；`hasPatches = !map.isEmpty()`；`patchIfNeeded = map.get(name)`。即乙的全部早退都落在
「拼接键系统属性读为 null → 空不可变集合 → 空集合查询」这条事实链上。

### 23.2 实施要点（终态，全部由清单 / 字节码给出事实）

- **F2 空不可变集合**：absint 新增对象标记 `Obj::Empty`；`vm_intrinsics.toml [facts.empty_collections]` 声明工厂
  （`List.of()` / `Set.of()` / `Map.of()`）与接收者为 Empty 时的查询结果（`isEmpty`→true、`size`→0、`get`→null、
  `contains*`→false）；标记经 PV 的返回值 / 字段 / 构造摘要 / 静态 final 自然传播，`hasPatches`、`patchIfNeeded`、
  `addModules.isEmpty()` 由此折叠。
- **F1 拼接键系统属性读**：键为拼接值时按 `name_parts` 求候选模式（同 `sysprops_write::removed_keys`），无候选与声明键
  （有值 / 动态 / 不稳定）相交则按缺省值（null）折叠；只读复用 class_lookup / pstrs，不改其实现。
- **F4 initPhase2 实参**：`printStackTrace=false` 由下一步 `[[boot_init.phases]] args` 提供，本步不实施；测量时以
  `--cut java/lang/System.logInitException…@50` 模拟。

### 23.3 下一步接口设想（`[[boot_init.phases]]` / boot layer 步骤 2–5，本步不实施）

- `seeds.toml [[boot_init.phases]]`：`call = "java/lang/System.initPhase2:(ZZ)I"`、`args = [false, false]`、
  `anchors = ["java/lang/System.bootLayer", "java/lang/Class.module"]`。闭包把 phase 当作带常量实参的根：入口帧形参槽
  直接取 `args`（走 `Facts.params` 同一通道，等同调用点常量实参），甲闸门由此死掉，不需要任何 JDK 名特判。
- anchors 声明 phase 产生、运行期读取的 VM 状态字段；闭包对锚字段的读取视为「phase 已运行」，生成器在启动序列里按 phase
  顺序发出调用（步骤 3–5：发射启动调用、bootLayer 落地、`Class.module` 回填）。

### 23.4 实施结果（F1 + F2，本机 `rava closure` 实测）

- F1 实施时推广为「键值来源 = 调用结果站点**或形参**」：`finderFor(String)` 的键是形参，两调用点各传
  `"jdk.module.upgrade.path"` / `"jdk.module.path"`（形参常量格只容一个常量），按 `Gap::Class` 取各调用点流入的名字
  （pstrs）展开为候选模式；各模式结果相同（都不在表中 → null）才折叠。登记表 `Ctx::pkeys`（方法节点, 来源）→ 模式；
  登记变化不在处理该方法中途失效，记入 `pkey_dirty`，由主循环顶部 `pkey_flush` 统一失效重算。
- 新增折叠（HelloWorld initPhase2 根）：`decode@20`→null、死区 34..342；`addModules@24`→null、死区 36..132；
  `finderFor@1`→null、死区 11..83；`ModulePatcher.<init>@5 isEmpty`→true、死区 23..126；`hasPatches@4`→true（`!` 后为 false）；
  `patchIfNeeded@15 get`→null、死区 32..657；`boot2@15/110/120/175/185` 一并折叠。乙闸门的 patcher / finder 两路关闭。

**initPhase2 根下剩余膨胀的前十出口**（新二进制，根 + PST 切；新增类沿首次发现链上溯，归属到第一个「常规闭包已有方法」的
出边，即常规闭包里已有、在根下因事实变宽多出的出边；首次发现口径，非必要性口径）：

| # | 出边（常规闭包已有方法@偏移） | 新增类 | 变宽原因 |
|---:|---|---:|---|
| 1 | `URL.lowerCaseProtocol@42`（折叠丢失） | 1038 | 常规闭包 `URL.<init>` 协议形参恒 `"file"`；根下 `URI.toURL` 传入未知协议 → `toLowerCase(ROOT)` → `ConditionalSpecialCasing` → ICU → `getResourceAsStream` → `URLClassPath$JarLoader` → `JarFile` |
| 2 | `AccessController.executePrivileged@29`（折叠丢失） | 546 | 模块系统新实例化的 `PrivilegedAction` 使 `run` 分派变宽（`SystemModuleFinders$1` / `ModulePath.findAll` …） |
| 3 | `String.valueOf@11`（`toString` 分派） | 411 | 新实例化类型（`ModuleDescriptor$Exports` 等）的 `toString` |
| 4 | `Objects.equals@11`（`equals` 分派） | 306 | 同上，`equals` |
| 5 | 根 `initPhase2` 直接带入 | 72 | `ModuleBootstrap` / `SystemModuleFinders` / `ModuleLayer` 本体 |
| 6 | `ConcurrentHashMap.computeIfAbsent@115`（`Function.apply`） | 34 | `ImageReaderFactory$1` → `ImageReader.open` |
| 7 | `AbstractCollection.toString@1` | 23 | 新集合元素类型 |
| 8 | `ImmutableCollections$AbstractImmutableSet.equals@37` | 18 | 同上 |
| 9 | `Pattern.compile@24`（折叠丢失） | 17 | 正则常量变宽 |
| 10 | `Formatter$FormatSpecifier.print@136/@11`（折叠丢失） | 12 | 格式化实参类型变宽 |

结论：F1/F2 关掉的是「早退分支」，剩余 ~2750 类绝大多数不是 boot2 的独立分支，而是 `SystemModuleFinders.ofSystem`
（`boot2@257`）/ `newConfiguration`（`boot2@935`）真实执行时新增的实例化类型与非常量实参，使常规闭包已有方法的分派 / 形参常量
变宽。分组 `--cut` 实验呈非单调（基线二进制「只开一组」：none 3271、patcher 3277、resolve 3279、cds / arch / post 3271、
finder 3268；新二进制切除全部组反而 4076——切掉 CDS 归档快路径后改走 `ofSystem` / `ModulePath`），故不以切点差值作路径
代价。下一步（boot layer 步骤 2–5）的收敛手段：phase 实参（甲闸门）与锚字段（`bootLayer` / `Class.module` 由 phase 产生，
其余方法读锚而不是重走 `ofSystem`），使分派变宽只发生在 phase 自身帧内；第 1 名的 URL 协议变宽另需 `URI.toURL` 协议来源
事实（`jrt` 常量经 `URI` 字段流）。

**验收对照**（`rava closure`，类数；base = 1fa483a4 构建，new = 本步；种子 0 / 1）：

| 测试 | 常规 base（s0 / s1） | 常规 new（s0 / s1） | initPhase2 根 + PST 切 base（s0 / s1） | 同 new（s0 / s1） |
|---|---|---|---|---|
| HelloWorld | 469 / 469 | 469 / 469 | 3283 / 3283 | 3221 / 3221 |
| StockTrans | 3386 / 3386 | 3384 / 3384 | 3470 / **4230** | 3468 / 3468 |
| DeepCopy | 3388 / 3388 | 3386 / 3386 | 4232 / 4232 | 3469 / 3469 |
| TestModuleLayerDefine | 3262 / **4205** | 3262 / 3262 | 3291 / 3291 | 3280 / 3280 |

- new 在 8 组配置下两种子类集合逐名一致；相对 base 只减不增（常规闭包减 `ModuleLoaderMap` / `$Mapper`；根下减 2–763）。
- base 有种子依赖（StockTrans 根 +760、TestModuleLayerDefine 常规 +943，JCA / jar 区域），new 下消失：属事实变窄后不再经过
  顺序敏感的大门，**不是**顺序问题已修复，引擎顺序线另行跟踪。
- 档案规模（≤ 3609）与 `--stop-after compile` 0 错误、多种子大例对照：服务器作业（见 tasks.md 行）。

**服务器结果（dddf8b49）**：
- 单例编译作业 `c1da-dddf8b49-compile`：HelloWorld（466 JDK + 1 用户类）与 TestModuleLayerDefine（3340 + 3）
  `--stop-after compile` 均通过，0 错误。
- 全量单测作业 `c1da-ut-dddf8b49`：唯一失败 `closure_independent_of_hash_seed`，cargo 停在该测试二进制。
  失败内容（`c1da-ut2-dddf8b49` 单跑）：`TestSerialDefaultSuid` 种子 2 多出 JCA 区域（`com/sun/crypto/provider/AESCipher*` …）。
  本机新二进制下该例与 StockTrans 种子 0 / 1 / 2 集合一致，未复现。
  同一 JCA 大门在基线二进制本机就有种子依赖（StockTrans 根下种子 1 +760、TestModuleLayerDefine 常规种子 1 +943），
  属引擎顺序线的既有问题；F1/F2 改变了事实到达顺序，触发它的用例 / 种子随之变化。
- 其余单测（`--skip` 该测试 + `--no-fail-fast`，含 rava_macros_core）：作业 `c1da-ut2-dddf8b49` 01 全部通过（rc=0）。

## 23.5 种子序阻塞、引导阶段第 1 步与锚点实测（2026-10-05，c1d-a2c b47568ec）

### 种子序（合入阻塞项 `closure_independent_of_hash_seed`）

- 根因两条（本机 `--hash-seed 0/1/2` 二分）：
  - (a) JCA 大门：`Class.forName` / 服务查找的宿主集合在不动点中途按到达顺序放行，种子不同时 JCA 提供者区域时进时不进。
    修法为宿主集合（`instantiation_hosts`）单调求值，本分支 96d677d8 与引擎顺序线 7426e583 文本一致，合并时吸收。
  - (b) 形参窗口内的 `V::Str` 被当作调用点字面量：派生字符串（拼接 / 子串结果）按先到的值进入 `site_lits`，
    后到的值不再撤回。引擎顺序线把 `V::Str` 改为携带来源（`V::Str(Rc<str>, Srcs)`），`site_lits` / `derived_str`
    只收真字面量，合并后本分支适配（sysprops / sysprops_key 改 `V::Str(..)` / `V::lit`）。
- 终态由集成分支 engine-order（V9）给出；b47568ec 合并后本机五例（TestSerialDefaultSuid、StockTrans、
  TestModuleLayerDefine、TestAppClassLoader、HelloWorld）种子 0 / 1 / 2 集合逐名一致。服务器结果见本节末。

### `[[boot_init.phases]]`（6666c19b）

- 清单：`call = "类.方法:描述符"`（静态，形参限 Z/B/C/S/I）、`args`（布尔 / 整数常量，个数与描述符一致）、
  `anchors`（字段键 `类.字段:描述符`）；装载期校验，错误报清单位置。
- 分析器：字段首次 GETSTATIC / GETFIELD 时若命中任一阶段锚点，该阶段作根（`Via::root("boot_phase")`），入口形参经
  `bind_pvs` 绑定 `args` 常量（与调用点常量实参同一通道），返回值回 VM；判定单调，一旦作根不撤回。
- 发射：闭包内已访问的阶段按清单顺序追加为 `vm_boot_init` 条目（main 前执行），非 void 返回值非 0 时 `exit(1)`（同 HotSpot）。
- 登记 initPhase2 `args = [false, false]`，锚点留空——锚点空时闭包与生成树与提交前一致。

### 锚点实测（本地临时 runtime：锚点 = `System.bootLayer`，并移出 `ModuleLayer` 的 vm_boundary 与 `module_layer_impl.rs`）

| 用例 | 常规 类 / 方法 | 锚点 类 / 方法 | 增量（类） | `rava closure` 耗时 |
|---|---|---|---:|---|
| HelloWorld | 469 / 1813 | 469 / 1813 | 0（无锚点读取） | — |
| TestCustomException | 480 / 1867 | 3193 / 18618 | **+2713** | 1 s → 18 s |
| TestStackWalkerFrames | 3108 / 17934 | 3257 / 19283 | +149 | — |
| TestAppClassLoader | 3108 / 17891 | 3257 / 19236 | +149 | — |
| TestModuleLayerDefine | 3265 / 19219 | 3249 / 19184 | −16 | — |
| StockTrans | 3380 / 20800 | 3435 / 21439 | +55 | — |

- phase 实参确实关掉了甲闸门：`logInitException` 在链上，但不再带出 `printStackTrace` 的额外路径。
- TestCustomException 的 +2713 按包：java/util 227、java/util/stream 151、java/lang/invoke 123、sun/nio/cs 97、
  sun/security/provider 86、sun/security/util 80、sun/security/x509 74、java/lang 71、java/security 68、
  java/util/concurrent 66、sun/security/ec 50、sun/nio/fs 44、xml 安全算法 43、java/text 42 ……
- `--cut` 二分（锚点运行时）：切整个 initPhase2 → 480；切 `boot2` → 486；切 `boot2` 全部 ≤@1014 调用点 → 488；
  只切 `ofSystem` 路径（@252/@257）或 resolve 调用点 → 不变（3193）；只保留 0–240 组 → 3202；只保留
  @194 `SystemModuleFinders.systemModules` + @240 `SystemModuleFinders.of` → 3901。没有单一调用点独占增量，
  `URL.lowerCaseProtocol@42`、`StringLatin1.toLowerCase@95` 单切也不变——多路径。
- 真实路径（`--why`）：`systemModules`（`SystemModules$default`）→ `of` → `toModuleReference`（以
  `JavaNetUriAccess.create("jrt", "/"+name)` 建 URI）→ defineModules。新实例化类型（`ModuleDescriptor$Exports` 等）使
  `String.valueOf` / `Objects.equals` / `AccessController.executePrivileged` 的分派变宽；`URL.<init>` 协议形参失去常量后
  `lowerCaseProtocol` → `String.toLowerCase(Locale)` → `StringUTF16.toLowerCaseEx` → `ConditionalSpecialCasing` → ICU
  `getResourceAsStream` → `URLClassPath$JarLoader` → `JarURLConnection` → `Files.createTempFile` → `SecureRandom` → JCA
  （sun/security/* 区域即由此进入）；`Collection.stream` 经 `ModuleDescriptor.toString` 带入 java/util/stream。

**结论**：单例增量 2713 类 > 300，按引导层计划第 1 步规则另立精度项，锚点不启用；引导层第 2–5 步（含
`BootLoader.getSystemPackageLocation`、命名 java.base、强封装、非空 boot layer）以该精度项为前置。精度项两条线：

1. URL / URI 协议事实。真实入口是 `boot2@64 BootLoader.loadModule` → `BuiltinClassLoader$LoadedModule.<init>@57`
   → `createURL(mref.location())` → `URI.toURL` → `URL.of(uri, null)`。`URL.of` 两支：
   - `handler == null && scheme.equals("jrt") && !uri.isOpaque() && uri.getRawFragment() == null` → `@136 new URL("jrt", host, port, file, null)`；
   - 否则 `@251 new URL(null, uri.toString(), handler)`（按规格串解析，`@188 lowerCaseProtocol(子串)` 的协议不可静态求出）。

   `toModuleReference` 以 `JavaNetUriAccess.create("jrt", "/"+name)`（私有构造只写 `scheme` / `path`）建 URI，故
   系统模块的 URI 走第一支；要关掉第二支须**按分配点**的对象字段事实（该 URI 的 `scheme` = `"jrt"`、`path` 非空、
   `fragment` 未写），按字段不分接收者的槽（`PSlot::F`）不够——程序里其他 URI 由解析构造写同名字段。
   第一支之后 `@36 lowerCaseProtocol(protocol)` 的协议形参在 5 参构造上汇合 `"file"`（`ParseUtil.fileToEncodedURL`）
   与 `"jrt"`，单常量格即 Top；需把选择子形参（`selector.rs`，现只认 int 族）推广到「入口值作 `equals` 接收者、
   实参为字面量」的 String 形参，并让构造方法按分配点接收者克隆，常量才能逐调用点到达 `lowerCaseProtocol`。
   `toLowerCase(ROOT)` 内的 `toLowerCaseEx` 依赖字符串内容（σ / 代理对 / İ），不能靠 `Locale.ROOT` 语言事实单独关掉。
   三件（分配点字段事实、String 选择子、构造方法接收者克隆）合起来才关掉第 1 名出口，属独立精度项，工作量不在本线。
2. 分派变宽限制在 phase 帧：锚点启用后其余方法读 `bootLayer` 而不重走 `ofSystem`；新实例化类型的 `toString` /
   `equals` / `PrivilegedAction.run` 分派只在实际有调用者的接收者集合上展开（与 §23.4 第 2–4 名同一机制）。

### 23.6 锚点膨胀的进一步分解（2026-10-05，c1d-a2c 2602f409 同步后，临时锚点 runtime 重建）

- 对照真实 JVM：`java -Xshare:off -Xlog:class+load`（无 CDS、无归档引导层，boot2 全程执行）TestCustomException 共加载
  **588** 类；锚点闭包 3193 类，其中不在「常规闭包 ∪ 真实加载」内的 **2526** 类（真实加载里锚点闭包缺 82 类，均为
  VM 内部 / 引导期类）。即引导层本身真实所需约 +100 类，其余是精度问题。
- 真实 JVM 下 `LoadedModule.<init>` 以 `"jrt".equals(uri.getScheme())` 跳过 `createURL`（系统模块 URI 恒为 jrt），
  不加载 jrt / jar URL 处理器、ICU、JCA、java.util.stream；`URI.scheme` 由 `JavaNetUriAccess.create("jrt", …)` 写入。
- 多路径：单切 `LoadedModule.<init>@48 createURL`、或同时切 `lowerCaseProtocol@42` / `executePrivileged@29` /
  `String.valueOf@11` / `Objects.equals@11` 四点，类数 3193 → 3193 / 3070。切掉 @48 后同一区域改由下列两路进入：
  - `ServiceLoader$ModuleServicesLookupIterator`（引导层非空后服务目录有模块提供者）→ `Class.forName(Module, …)` →
    `BuiltinClassLoader.findClassInModuleOrNull` → `defineClass` → `LoadedModule.codeSourceURL@22`（惰性 `createURL`，
    对 jrt 也建 URL）→ `URL.of`；
  - `ServiceLoader$LazyClassPathLookupIterator` → `BootLoader.findResources` → `findMiscResource` → jar URL →
    `URLJarFile` → `Files.createTempFile` → `SecureRandom` → JCA。
  两路的入口都是 `Charset$ExtendedProviderHolder`（真实 JVM 只在 `Charset.forName` 未知字符集时触发）。
- 首次发现链（`via`）不带上下文：`doPrivileged` / `String.valueOf` 按调用点克隆，但报告中的分派边与方法首达边
  都记在成员上，链里出现的「`AccessibleObject.<clinit>` → `executePrivileged` → `ExtendedProviderHolder$1.run`」、
  「`main@27`（实参 String, int）→ `String.valueOf(Object)` → `ModuleDescriptor$Exports.toString`」都是不同克隆的边拼接，
  不能作为出口归因。精度项开工前需先给 `--why` / `methods.via` 带上上下文（克隆键），否则出口排名不可信。
- 精度项的工作分解（终态，均不按类名）：
  1. 归因工具：`via` / 分派报告带克隆上下文，按上下文给出出口；
  2. 实例字段的确定赋值：类的全部可达构造器在 `this` 逃逸前无条件写入的字段，缺省值不并入 `fvals`（现状为首次抽象分配
     即并入缺省值，`URI.scheme` 恒为 null ∪ "jrt" = Top）；
  3. 字段 / 形参字符串值集（在 `PV` 单常量之外取小集合，同 `V::Ints`），使 `"jrt".equals(uri.getScheme())`、
     `lowerCaseProtocol` 按值集折叠；
  4. 服务目录查找的提供者集合按调用点服务类型收窄（`ExtendedProviderHolder` 只取 `CharsetProvider` 提供者）。

### 23.7 带上下文的出口归因（2026-10-05，ea3e9770；`--why` 逐节点标克隆上下文）

精度项 1 的第一步：`--why` 溯源链每个方法节点按「成员 #上下文」标出（`ctx_label`），链沿节点自身首达边走，同一成员的
不同克隆可辨。用它重看锚点闭包（TestCustomException，临时锚点 runtime）三处出口，归因如下：

| 出口 | 上下文链（摘要） | 真实 JVM | 对应精度项 |
|---|---|---|---|
| `ServiceLoader$ModuleServicesLookupIterator`（及 JCA 路） | `sun/nio/fs/Util.<clinit>@5` → `Charset.forName #@13536:5` → `lookup` → `lookup2@48` → `lookupExtendedCharset@8` → `ExtendedProviderHolder.<clinit>` → `ServiceLoader.iterator` | 字符集名取 `sun.jnu.encoding`（宿主区域 codeset，`[facts.system_properties] dynamic`，运行时 `posix::native_encoding`）；本机为标准字符集故不触发扩展提供者，但宿主为扩展字符集（如 EUC-JP）时真实可达 | 非精度缺陷：开放宿主下该路正确可达；收窄只能来自「扩展字符集提供者」本身按服务类型的选择（已按调用点服务类型选） |
| `SecureRandom` 的方法体 | `boot2@240` → `SystemModuleFinder.<init>` → `Set.of` → `Set12.<init>@9` 分派 `ModuleReferenceImpl.equals` → `Objects.equals #@8078:22` → `ModuleDescriptor.equals` → `Objects.equals #@8082:151` → `Version.equals` → `compareTokens@105` 对 `List<Object>` 元素 `toString` | `Version` 的记号只有 `String` / `Integer` | 容器元素类型：`compareTokens` 读出的元素是 open(Object)，全部活类型的 `toString` 入链（`Objects.equals` 已按调用点克隆，但汇合发生在元素读出处） |
| `java.util.stream` | `FileOutputStream.<clinit>` → `SharedSecrets.ensureClassInitialized #@210:10` → `Lookup.ensureInitialized@19` → `StringBuilder.append(Object)`（实例汇合点，未克隆）→ `String.valueOf(Object) #@515:2` → `ModuleDescriptor$Exports.toString` → `ModuleDescriptor.toString(Set,String)` → `Collection.stream` | `ensureInitialized` 只在访问检查失败时拼异常消息 | G2 实例汇合点（`append(Object)`）：§6.7 实测按调用点克隆不分开；需按实参值集（而非形参汇合）分派 `toString` |

结论：第一处随宿主正确可达；其余两处分属容器元素类型、实例汇合点两类精度项，均与引导层本身无关；锚点启用仍以它们为前置。
## 24. V9：闭包随哈希种子变化——反射调用池去冗余与透传过时边（2026-10-05，分支 v9-rcall）

**症状**：基线 2602f409 上 TestJndiNoProvider 种子 0 = 24492 方法、种子 1/2 = 24470。多出的 20 个方法是两组：
`ObjectStreamClass.getReflector@78` 带入的 Provider / Properties / Hashtable / Collections 各 Map / ImmutableMap /
ReferencedKeyMap 的 `putIfAbsent`，以及 XMLParserImpl 带入的 ArrayDeque / LinkedList / LBQ / LTQ / SynchronousQueue /
DelayedWorkQueue 的 `offer` / `poll`。

**两处依赖求值次序的判定**（终态都与次序无关，取精确一侧，不靠放宽到 open）：

1. 反射调用实参池的去冗余视图 RN（`reflect_call.rs::rcall_absorb`，e57444ae）。值 x 被池中 open 涵盖的条件（非合成、
   已逃逸、属于某 open）随分析只增不减，但原实现在值入池时判定：x 先到就逐个列出、open 先到就略去。两种结果的下游
   效果不等价——未收窄的按偏移写入对逐个列出的对象写其全部引用字段，对 open 目标只写 open 类型自身的字段。
   现在：此刻已涵盖的永久略去，其余挂起，到工作队列排空（单调部分的不动点）时由 `rcall_release` 定夺（与
   `lookup_release` 同口径，排在 `seed_round` 之前）。可覆写成员的接收者同理：是否已退回 VM 枢纽（枢纽展开覆盖
   逐接收者派发）也在排空时判定。实现拆到 `engine/reflect_call_pool.rs`。
2. 透传摘要的过时边（`invoke.rs::edge_ret`，主因）。被调方的透传摘要（返回值只来自形参）随分析推进会在「透传」与
   「经 R 汇合」之间转换，调用方按新摘要重接，但已接的透传边不撤回。原透传边只按调用点返回类型过滤（Object），
   不按被调形参声明类型过滤：种子 0 下 `SocketPermissionCollection.lambda$add$0` 一度是透传，`Map.merge@32`
   （`BiFunction.apply`）的结果因此接上 `@12 Map.get` 的巨集合（反序列化来的 open(Object)），经 `Map.put` →
   `WeakHashMap.put` / `remove` → `ClassValueMap.removeEntry@8` 的 checkcast 注入 open(ClassValue$Entry)，再经
   `FieldReflector.setObjFieldValues` / VarHandle CAS 把巨集合写进 `ClassValue$Entry.value`，`ClassValue.get` 的
   返回随之变宽。种子 1 下该 lambda 从未是透传，结果只经 R（{SocketPermission, open(SocketPermission)}）。
   修法：透传边按被调形参声明类型收窄（与 P → R 汇合路径同一口径；形参类型不是调用点返回类型子类型时取后者）。
   优化性分析下活代码只增不减，早先透传的形参在终态仍经 return 流入 R，过时的透传边因而被 R 涵盖，结果与次序无关。

**结果**（本机 `rava closure`，种子 0/1/2）：TestJndiNoProvider 三种子类 3820 / 方法 24470 / 反射成员 1610，
集合逐项一致（取精确一侧）。`closure_independent_of_hash_seed` 增加 TestJndiNoProvider。

**诊断方法备忘**：`@trace:open:<类型>` 记录首达来源，沿来源反向追链即可定位；本例最后一跳靠
`@edge:<节点>` 看到 `@12 → @32 [Object]` 这条不应存在的边（实参直连结果 = 透传边）。

## 25. a2 续收口：锚点启用的代价口径与前置（2026-10-05，分支 c1d-boot，基于 8bb25e10）

任务 1（早退检查按分析期事实求值，F1 / F2）与任务 2（`[[boot_init.phases]]` 清单化，6666c19b，锚点留空）已在
§23 / §23.5 合入，本节只记 boot layer 步骤 2–5 能否开工的实测。本节未改代码。

### 25.1 两个口径分开看

本地临时 runtime：两个锚点 `System.bootLayer` + `Class.module`；`ModuleLayer` 移出 `[vm_boundary]`；删掉
`module_layer_impl.rs` 与两处 `Class.getModule` 手写。档案并集取服务器作业 c1db-prof-2574ead5（1084 例，Linux）：
**7886 类 / 51370 方法**。任务书写的基线 3609 是更早的口径，下文统一以 7886 为准。

| 口径 | 现状 | 锚点启用 | 说明 |
|---|---|---|---|
| 档案并集（类） | 7886 | 增量约 +20 | TestCustomException / HelloWorld 的锚点闭包不在并集里的类分别只有 36 / 33 个：16 个是 `jdk/internal/module` 引导类（Builder、ArchivedBootLayer、SystemModuleFinders 族等），这是建层本身需要的；8 个 `sun/nio/fs` 和 apple/* 是本机 macOS 与服务器 Linux 的差别；其余是测试自己的类 |
| 单例闭包 HelloWorld | 469 类 / 1.2 s | 3190 类 / 18611 方法 / 17 s，峰值 1.5 GB | 只开 `bootLayer` 锚点时 HelloWorld 不作根。加上 `Class.module` 后，作根路径为 `System.<clinit>` → `FileOutputStream.<clinit>` → `SharedSecrets.getJavaIOFileDescriptorAccess` → `ensureClassInitialized` → `Lookup.ensureInitialized` → `VerifyAccess.isClassAccessible@59` → `getModule`，于是全部 1084 例都作根 |
| 单例闭包 TestCustomException | 480 | 3193 | 同上 |

结论：按档案并集看，锚点几乎不增加规模（+20 类，都是引导层本身必需）。但生产构建按单个项目算闭包，HelloWorld 会变成原来的
**6.8 倍**，二进制体积跟着放大，这违反「正确且最小」。所以第 2–5 步的终态（删手写、锚点启用）在单例膨胀消除之前
**不开工**。

### 25.2 HelloWorld 多出的 2721 类按首次离开模块代码的出口分类

方法：沿每个新增类的首达链（`via`）反向找第一个模块代码帧（`jdk/internal/module`、`java/lang/module`、`Module`、`ModuleLayer`、
`initPhase2`），记下它的下一跳。首达链不带克隆上下文，所以下表只用于排序，不用于逐例归因（§23.6）。

| 类数 | 出口 | 所属精度项 |
|---|---|---|
| 1852 | 首达链上没有模块代码：建层后原本折叠的分支变为可达（`Class.getResourceAsStream@75` 命名模块分支 → `BuiltinClassLoader.findResourceAsStream` → `URLClassPath` jar 加载器 → `Files.createTempFile` → `SecureRandom` → JCA；ICU、ResourceBundle、ForkJoinPool 等） | `Class.classLoader` 接收者汇合：`getResourceAsStream` 的 `this` 不分镜像，boot 类读 classLoader 不能折成 null。与 §23.6 第 4 条服务查找同属一类：汇合点的形参或接收者需要按值集求值 |
| 约 370 | `ModuleDescriptor.toString` → `Collection.stream` / `ReferencePipeline.collect` | G2 实例汇合点（`String.valueOf(Object)` / `StringBuilder.append(Object)`，§23.7 第 3 行） |
| 约 250 | `ModuleDescriptor$Version.compareTokens` 读 `List<Object>` 元素后调 `toString`（`Instant` / `Date` / `X509CRLImpl` / `PKCS7` …） | 容器元素类型（§23.7 第 2 行） |
| 45 | 建层直接需要的类（`LayerInstantiationException` 等） | 不是缺陷 |
| 其余约 200 | `SystemModuleReader.read` → jimage `ImageReader`；`Module.getPackages` → `BootLoader.packages`；`boot2` → `BootLoader.loadModule` 等 | 第 5 步（jimage 嵌入数据）落地后按真实路径重测 |

### 25.3 具体求值（a1 引擎）执行 initPhase2：不可行

用临时探针（未提交）把阶段入口交给 `engine/concrete` 求值，实参为清单常量 `(false, false)`，判断能否「只登记实际执行的指令」。
在进入 `boot2` 之前就失败了，失败原因按顺序如下：
- `System.<clinit>`：`registerNatives` 没有具体语义；
- `ModuleBootstrap$Counters.<clinit>`：读可变静态字段 `System.props`（由 VM 的 initPhase1 填充）；
- `ModuleBootstrap.<clinit>`：读 `SharedSecrets.javaLangAccess`（由 `System.setJavaLangAccess` 引导步骤写入）；
- `Reference.<clinit>`：类初始化写入它类静态字段 `SharedSecrets.javaLangRefAccess`；
- `ArraysSupport.<clinit>`：读 VM 注入的 `UnsafeConstants.BIG_ENDIAN`。

求值器的语义是「无副作用、可撤销的调用点求值」，而引导阶段要求另一套语义：
1. 先有 initPhase1 与 `[boot_init]` 步骤之后的 VM 初始堆（系统属性表、SharedSecrets 各访问器、注入常量）；
2. 阶段的写入是永久的初始状态，不撤销；
3. 结果对象图（Configuration、各个 Module、包表）要作为程序初始状态物化给抽象分析。

这相当于一个构建期引导映像求值器，不属于 a1 引擎的扩展范围。如果要走这条路，需要另立任务。

### 25.4 第 2–5 步的前置（按影响排序）

1. `getResourceAsStream` 一类 `Class` 实例方法：`this.classLoader`、`this.module` 按接收者镜像值集求值（1852 类）。
   同一机制还能把 `VerifyAccess.isModuleAccessible` 的 `refc.getModule() == lookupModule` 在同一模块内折成 true。
2. 容器元素类型（`Version` 记号表只有 String / Integer）。在途：「C1d 剖面并集与容器元素精度」线。
3. G2 实例汇合点按实参值集分派 `toString`。在途：V10「逃逸对象上下文收拢」线。
4. 以上合入后用同一临时 runtime 重测 HelloWorld，判据为单例 ≤ 469 + 100 类（真实 JVM `-Xshare:off` 下 HelloWorld 共加载
   556 类，其中模块相关 38 类）。达标后按 boot-layer.md 第 2–3 步实施。

TestProtectionDomainFaces / TestClassModuleFace / TestSetAccessibleBoundary 在 main 上失败（`getSystemPackageLocation`
native 缺失、`String.class.getModule()` 不是命名的 java.base），三者都依赖引导层，随第 2–3 步一起解决。不单独补手写近似：
`PackageHelper.findModule` 要经 `Modules.findLoadedModule` 取到 java.base 的 `Module` 对象，只返回 `"jrt:/java.base"`
仍然会抛 `InternalError("java.base not loaded")`。

## 26. 档案并集实测：c1db-prof-2574ead5 + c1db-prof16b（2026-10-05，分支 c1d-elem）

**口径**：作业 c1db-prof-2574ead5 共 16 片，第 16 片缺 8 例，由 c1db-prof16b 补齐，合计 1092 例。
用 `scripts/profile_union.py stats` 统计，按类与基线 t1-prof21（4bd826ae，3609 类）逐类对照。

**结论：并集 JDK 类 7972、方法 51767，未达到 ≤ 3609。** 相对基线新增 4698 类、减少 335 类。

| 测试数 | 1 | 10 | 30 | 60 | 120 | 240 | 480 | 960 | 1092 |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 并集类数 | 2096 | 3504 | 3761 | 3775 | 4777 | 5981 | 6732 | 7718 | 7972 |
| 并集方法数 | 11692 | 20554 | 23105 | 23257 | 30477 | 38059 | 43535 | 50060 | 51767 |

- 按域：translate 7952 / boundary 19 / root 1。按最高层级：code 7051、layout 566、type 193、init 91、alloc 71。
- 单例 JDK 类：中位 521、最大 5466。单例方法：中位 2183、最大 34015。
- 单例分两档：533 例在约 3180 类，共享 3144 类的公共核。这一档来自边界域收窄（675 → 19）：open 派发经
  `executePrivileged`、`String.valueOf`、`Objects.equals` 打开了全体活类型的 `run` / `toString` / `equals`。

**新增 4698 类的来源**（逐类沿 via 链从根往下，找第一个不在基线里的节点；明细见
`docs/reports/c1db-prof-union-added.tsv`，减少的 335 类见 `docs/reports/c1db-prof-union-removed.txt`）：

| 来源 | 类数 | 主要闸口（类数） |
|---|---:|---|
| former_boundary：经基线时的边界类展开 | 3944 | java/lang/Class 991、jdk/internal/util/Preconditions 781、xalan TransformerImpl 726、TransformerFactoryImpl 434、XPathImpl 197、HttpClientBuilderImpl 155、HttpClientFacade 144、rmiURLContextFactory 79、HttpServer 68、jline TerminalBuilder 49 |
| new_edge：基线已有的类上新增的边 | 600 | SSLContextImpl$DefaultSSLContext 95、RemoteObjectInvocationHandler 47、AsyncSSLTunnelConnection 15、SignatureParser 15 |
| only_new_tests：只出现在基线之后新增的测试里 | 154 | — |

增量主体（84%）是 C1d 拆除边界后，原边界类（Class、Preconditions、xalan / XPath / HttpClient 等）的方法体转为字节码翻译，
其调用链随之展开。其中 java/lang/Class 与 Preconditions 两个闸口合计 1772 类，主要经反射成员枚举和异常格式化
（`Preconditions.outOfBounds` → `String.format` → Formatter 全家）进入。

精度缺口按影响排序：
1. 容器元素 open(Object)，见 §27。
2. Class 接收者的 classLoader / module 逐类求值（§25.4 第 1 项）。
3. G2 实例汇合点（V10）。

## 26. 前置第 1 项：`Class` 实例方法按接收者镜像求 `classLoader` / `module`（2026-10-05，分支 c1d-clsfact）

提交 69d1c73d（基于集成分支 85a56289）。事实来源只有 `.class` / jmod 与 `runtime/java_runtime` 清单：定义加载器取
`[vm_state.field_hooks]` 的接收者钩子与 `[vm_state.loader_map]`，模块归属取类路径来源（User / Lib = 无名模块）。生成器 crate
不出现类名。

### 26.1 改动

| 层 | 改动 |
|---|---|
| 流图（`bytecode.rs` field） | 类镜像上读接收者钩子字段（`receiver = true`）时，镜像不再计入「其余接收者」，所以不接全局字段节点 `F`。值只来自钩子：应用 / 平台类镜像接钩子值池，引导类镜像恒为 null。此前一个镜像读到的是全部镜像的值并集（`getResourceAsStream@44` 在 boot 类上读到 AppClassLoader / PlatformClassLoader） |
| 抽象解释（`absint.rs`） | 新增 `Oracle::param_mirror_field`。接收者只来自一个 Class 形参（通常是 `this`），且形参镜像值集全是引导类时，`this.classLoader` 折为 null。这是乐观答复，复用 `mirror_watch`，以哨兵 `HOOK_FIELD` 登记；值集新增非引导类镜像、所指未知的 Class 对象或 open 时重分析（`mirror_eq.rs::may_hook`） |
| 引导阶段锚点（`boot_phases.rs`） | 实例字段锚点（`Class.module`）按接收者值集判定：值集只含用户类 / 库类镜像时不作根（boot-layer.md 2.1 的规则此前没有实现） |
| 单测 | `absint::receiver_hook_field_folds_by_param_mirrors`、`mirror_eq::hook_answer_invalidates_on_non_boot_mirror` |

`this.module` 没有做逐镜像的值折叠。现行 runtime 里 `Class.module` 从不写入，`getModule` 是手写；`Module.isNamed` 等也是手写
（`Module` 在 `[vm_boundary]`）。命名 / 无名模块的区分要等 boot-layer 第 2 步加上 `Class.__vm_module` 接收者钩子、
`Module` 回到字节码之后才有可读的值。到那时，流图这一侧已经按接收者钩子字段统一处理，不需要再改：钩子值池按镜像接入，不经 `F`。
同一机制在 `module` 上缺的只有「引导类镜像 → java.base 模块对象」这一条值，需要有按模块名区分的抽象对象。

### 26.2 验收

| 口径 | 前（85a56289） | 后（69d1c73d） | 说明 |
|---|---|---|---|
| HelloWorld，无锚点 | 469 类 / 1813 方法 | 469 / 1813，集合逐项相同 | 只收窄：流图少接 `F` 边，折叠只去分支，锚点只少作根，三处都单调 |
| HelloWorld，§25 临时 runtime（两锚点） | 3190 / 18611 | 3190 / 18611，集合逐项相同 | 折叠确实生效：`Class.getResourceAsStream` 的 `@44` / `@108` 折为 null，死区 `[60,103]`、`[121,127]`（命名模块走 BuiltinClassLoader 的分支和 `cl.getResourceAsStream`） |

**锚点口径没有下降**。§25.2 估的「约 1852 类」是首达链归因，不是必经路径。反事实切除（`--cut`，同一临时 runtime）：

| 切除 | 类数 |
|---|---|
| 无 | 3190 |
| `Class.getResourceAsStream` 整个方法体 / `@75` / `@75 @83 @95 @123` / `ClassLoader.getSystemResourceAsStream` | 均为 3190 |
| `SecureRandom.<init>()V` | 3179 |
| `ICUBinary.getRequiredData` | 3187 |
| `ClassLoaders.<clinit>` | 3164 |
| `ModuleBootstrap.boot2` 整个方法体 | 427 |
| `boot2@352`（`BootLoader.loadModule`）/ `@363`（`defineModule`） | 3249 / 3249（切写入点不单调，见 §7） |
| `boot2@896` / `@747 @815`（流水线） | 3190 / 3190 |
| `boot2@194 @228 @240`（系统模块） | 3893（不单调） |

膨胀全部在 `boot2` 之内，但切除 `boot2` 内任何单个出口都不能消除它，说明这是共享汇点饱和，不是某一条路径。`--flows @merge:300`
的前几项：`System.arraycopy` P0 汇入 2604 类、P2 汇入 1679 类，`StringBuilder.append(Object)` P1 汇入 1210 类，escape 1026 类，
`Formatter.format` 实参数组 871 类，`Unsafe.putReferenceRelease` / `compareAndSetReference` 769 / 725 类，`ComparableTimSort` /
`Arrays.mergeSort` 约 480 类。`@openstat`：`Object` 在 21852 个节点上展开，引入点以 `Reference.get`（WeakHashMap$Entry、
LocaleResources$ResourceReference）与 `HashMap$Node.getKey` 为首。建层代码一旦把模块系统的类型送进这些汇点，任何读出汇点的
虚调用都会对全部类型分派，于是 JCA / ICU / XMLDSig / locale 全部可达。

### 26.3 离判据还差多少

判据：锚点口径 HelloWorld ≤ 569 类（469 + 100；真实 JVM `-Xshare:off` 加载 556 类）。本项完成后仍是 3190 类，**还差 2621 类**。
这一项在锚点口径上没有贡献；它的作用是去掉 `Class` 接收者汇合本身的不精确（汇点饱和消除后，这里就不会再成为新的出口）。
剩余前置按实测重新排序：
1. 共享汇点：`System.arraycopy` 的形参、`append(Object)` / `String.valueOf(Object)`、`Unsafe` 引用 CAS / release 写，以及
   `Reference.get` / `HashMap$Node.getKey` 的 Object 引入。这些属于 V10（逃逸对象上下文收拢）和 c1d-elem（容器元素类型）两条线。
2. 两线合入后，用同一临时 runtime（`seeds.toml` 两锚点、`ModuleLayer` 移出 `[vm_boundary]`、删 `module_layer_impl.rs` 与两处
   `Class.getModule` 手写）重测 HelloWorld，并重复本节的 `boot2` 切除和 `@merge` 测量；若还有残余，再按出口逐项立项。

## 27. 容器元素精度第 1 项：反射数组分配按调用点建模（2026-10-05，分支 c1d-elem，956db0b4 + 35c78422）

**根因**：`Arrays.copyOf(T[], int, Class)@18` 调 `Array.newInstance` → native `Array.newArray`。原模型把它的结果当作
open(Object)，于是 `ArrayList.grow` 之后的 `elementData` 成了任意对象数组，`aaload` 读出 open(Object)，下游的
`toString` / `compareTo` / `equals` 派发到全体活类型。
例：`ModuleDescriptor$Version.compareTokens@100/@105` 原有 392 个派发目标。

**终态做法**：
- **清单登记**：反射数组分配方法在 `vm_intrinsics.toml [facts.reflect.array_allocators]` 中登记，值为元素类型实参序号
  （`Array.newArray:(Ljava/lang/Class;I)Ljava/lang/Object;` = 0）。生成器中不出现类名字面量。
- **返回模型**：调用点的返回模型为 `RetModel::NewArray(i)`，经镜像流边 `MirrorOp::ArrayOf(m, off)` 把元素类型实参的
  类镜像集变换成结果。
- **分配点**：对所指已知的类镜像 X，在调用点 (m, off) 上建数组分配点 `[X@m:off`（与 `anewarray` 同一机制），
  元素只来自其后的写入。
  - 基本类型类镜像给 8 种基本类型数组。
  - 已达 255 维的数组类镜像不产生结果（JVMS §4.4.1）。
- **所指未知时**：元素类型实参出现所指未知的 Class（open、非镜像 Class 值、非字节码类镜像）时，结果为 open(Object)，
  语义与原模型相同。
- **放行判定只在不动点上做**：放行前到达的类镜像只记下。工作队列排空时，实参仍没有所指未知的调用点才放行、逐类型建分配点
  （`reflect.rs::array_of_release`，与 `lookup_release` / `rcall_release` 同一口径）。出现过所指未知的调用点不放行。
  放行后，结果是实参集的单调函数，因此终态与求值次序无关。
  - 首版（956db0b4）在值入点即时判定「先到所指未知即饱和」，饱和前已建的分配点会留存，结果随散列种子变化：
    TestModuleLayerDefine 种子 0/1 为 3084 类，种子 2 为 3264 类。
  - 次版（35c78422）改为排空时判定后，三个种子的结果逐项相同。
- **代价控制**：反射上下文里（`MethodType.fromDescriptor` 的描述符解析、`getClass(open)` 等），元素类型实参约有 1700 个镜像，
  另有 open。在这些上下文里逐类型建点会让分析超过 20 分钟，`Class.arrayType` 还会让维数递归失控。按上一条规则，
  这类调用点在不动点上已有所指未知，不会放行，所以不建分配点。

**结果**（本机 `rava closure`，TestModuleLayerDefine，JDK 21）：

| | 改前 8bb25e10 | 改后 35c78422 种子 0 / 1 / 2 |
|---|---:|---:|
| JDK 类 | 3264 | 3084 / 3084 / 3084（集合逐项相同，均为改前子集，−180） |
| 方法 | 19219 | 18853 ×3（均为改前子集，−366，无新增） |
| compareTokens@100/@105 派发目标 | 392 | 2 |
| 闭包耗时 | — | 25–28 s，RSS ≤ 1.9 GB |

减少的 180 类逐项论证：它们在改前只经两条 open 派发进入。
- `compareTokens@105` 的 `toString` / `compareTo` 派发到全体活类型，例如 `ArrayBlockingQueue.toString`。
- `KeyFactory.nextSpi@64` 等 JCA 取实现处的 `Provider$Service.newInstance` 派发到 open(Provider$Service)，即全部 provider
  的服务类，例如 `XMLDSigRI$ProviderService.newInstance@107`。由此进入 xml-security 实现 111 类（`ApacheCanonicalizer` →
  `Init` …）、org/jcp/xml/dsig 实现 12 类、sasl 工厂、jgss 机制等。provider 类本身（XMLDSigRI 等）仍经 ServiceLoader
  进入，不在减少之列。

这两条派发的接收者都来自 `Array.newArray` 结果的 open(Object) 元素。改后元素只来自实际写入，这两条链都不再成立；
测试本身没有使用这些类。

验证作业：
- 单测 c1de-ut-c146f5c8（新增 `reflect_new_array_element_precision`：TestModuleLayerDefine 三种子一致，且不含上述两条链带入的类）
- 抽查 c1de-sp-61da38bd（TestModuleLayerDefine、TestArrayComponentType、TestReflectArrayDeep、TestVmPlatformNatives、
  SuccessivePrimeDifferences、RankingMethods、ArrayListDemo、ArrayListFull）
- 档案 c1de-prof-c146f5c8（24 片）对照同口径基线 c1de-profb-8bb25e10

结果见 §27.1。

### 27.1 进度与暂停记录（2026-10-05，按协调方要求暂停，分支 c1d-elem @ c146f5c8 + 本文档提交）

**在途作业的处置**（已全部按 PID 停掉，服务器残留已清，无 c1de 运行目录存活）：

| 作业 | 状态 | 结论 |
|---|---|---|
| c1de-ut-c146f5c8 | 作业超时（kr1，60:07，rc=None） | **不算通过**：作业缺 `--job-timeout 7200`，单测未跑完。恢复时须带该参数重发 |
| c1de-sp-61da38bd | 停止时 6 例 PASS：ArrayListDemo、ArrayListFull、TestArrayComponentType、TestVmPlatformNatives、SuccessivePrimeDifferences、RankingMethods | TestReflectArrayDeep、TestModuleLayerDefine 未跑完。TestModuleLayerDefine 运行期 NPE 在集成分支（int-2602f409、c1db-sp-b47568ec）上已存在，不是本改动引入 |
| c1de-prof-c146f5c8 | 停止时 5 / 24 片 rc=0 | 未完成，无并集结论 |
| c1de-profb-8bb25e10（基线） | 停止时 11 / 24 片 rc=0 | 未完成 |

**未解决的回归（阻塞合入）**：TestJndiNoProvider 在 c146f5c8 上种子稳定（3820 类 / 24502 方法），类集与集成分支
781eeec2 相同，但**方法多 32 个**（781eeec2 为 3820 / 24470），违反「集合只减不增」。新增方法与 V9 修过的是同一族：
`Provider.implPutIfAbsent/putIfAbsent`、`ArrayDeque.offer/offerLast`、`Collections$EmptyMap/SingletonMap/SynchronizedMap.putIfAbsent`、
`Collections$SynchronizedCollection.add` 等。

已查明的链路：
1. `XMLParserImpl.repoolDocumentBuilder@10 Queue.offer` 的接收者来自 `getDocumentBuilderQueue@16`，取值路径是
   `SynchronizedMap.get` → `WeakHashMap.get`（上下文 `WeakHashMap@127623:30`）→ `WeakHashMap$Entry.value`（14 个对象）。
   该接收者集合改后为 8499 类加若干 open，改前为 29 类加 open(Object[]) 和 open(SocketPermission)。
   `@path` 显示 `WeakHashMap$Entry.value` 与全局 `R ConcurrentHashMap.get`（8499）同处一个 Object 流强连通代表。
   改后有新值流入这个汇点。
2. 新流入的来源：`Arrays.copyOf(Object[], int, Class)@35 → System.arraycopy` 写入新建的反射数组分配点。
   - 例如 `[Ljava/lang/ClassValue$Entry;@1629:2`，上下文为 `Vector@194:85`、`ArrayList@1088:23`、`ArrayList@606:43`、
     `ImmutableCollections$ListN@494:88`、`TimSort`/`copyOfRange` 等十余个。
   - 共享上下文里，元素类型实参集是众多调用方之和；源数组元素里有 open(Object)。arraycopy 写入时按分量类型收窄，
     open(Object) 变成 open(C)。结果是每个类型 C 的新数组都带上 open(C) 元素。
   - 例如 `@trace:open:java/lang/ClassValue$Entry` 在改后有 101212 个节点。改前这些调用点返回 open(Object)：
     open 数组只按已有的逃逸分配点展开，arraycopy 写进 open 目标时不新增元素（hw_site 只接具体数组）。
   - 于是 open(ClassValue$Entry) 经 `ClassValueMap.finishEntry → WeakHashMap.put` 第 2 实参进入。该实参改后为
     {9 类, open(ClassValue$Entry)}，改前为 {7 类}，并最终汇入上述强连通汇点。
3. 诊断落盘：
   - /tmp/c1de_whp_base2.txt、/tmp/c1de_whp_head.txt：WeakHashMap.put 第 2 实参来源，改前 / 改后。
   - /tmp/c1de_cve.txt：open(ClassValue$Entry) 的 trace。
   - 二进制：/tmp/rava_base2（781eeec2）、/tmp/rava_head（c146f5c8）。

**恢复时的第一步**：
1. 在 c1d-elem 上 `git merge rust-closure-analyzer`。
2. 修第 2 点的终态口径：反射数组分配点的元素只能来自实际写入，而 arraycopy 只搬移元素、不产生新值。
   - 源元素里的 open(o) 写进反射分配点时，不应按分量类型收窄成新的 open(C)。
   - 拟法：hw 写入槽流向反射分配点元素时，open 部分沿用「open 目标只覆盖已逃逸分配点」的同一口径，即只保留源中
     已有的 open(t ⊂ C)，不从 open(Object) 生成 open(C)。
   - 或者在 `array_of_into` 处把「源数组元素含 open(Object)」的共享上下文视同所指未知。
   - 两种方案都必须是不动点上的单调判定。
3. 验收：
   - TestJndiNoProvider 的方法集 ⊆ 781eeec2，且种子 0/1/2 结果一致。
   - TestModuleLayerDefine 仍为 3084 类，且种子无关。
   - `reflect_new_array_element_precision` 通过。
4. 然后推送并重发作业：
   - 单测作业带 `--job-timeout 7200`。
   - 抽查（补 TestReflectArrayDeep、TestJndiNoProvider）。
   - 档案 24 片，与 c1de-profb 同口径基线一起重跑。

### 27.2 恢复：反射数组分配点元素写入不收窄 open（2026-10-05，分支 c1d-elem，合入 300389ce 后）

**根因补充**：§27.1 第 2 点只写了 hw arraycopy。只在 hw 写入上改口径（`hw_site_arrays`）后，TestJndiNoProvider 仍多 32 个方法。
用 `@trace:open:java/lang/ClassValue$Entry` 复查，open(ClassValue$Entry) 还经字节码 aastore 进来：
- 写入点是 `ArrayList.add(int, Object)` 的 P2，写进 `[Ljava/lang/ClassValue$Entry;@…:2` 的元素。
- 机制：共享上下文中，`Arrays.copyOf(T[], int)` 的 newType 取 `original.getClass()`，镜像集是各调用方之和。
  因此 `ArrayList.elementData` 会收到别的调用方所要分量类型的数组。
- 对这些数组的任何元素写入（hw 搬移或字节码 aastore），只要按分量类型过滤，open(Object) 就会被收窄成 open(C)。

**终态口径**（不动点上的单调判定，与写入路径无关）：
- 反射数组分配调用点建出的分配点登记在 `refl_arrays`。
- 写入其元素的流边带 `OPEN_EXACT` 标记。`classes.rs::filter_open_exact` 的规则：
  - 确定类型 ⊂ 分量类型的保留；
  - open(o) 只在 o ⊂ 分量类型时原样保留，不收窄出新的 open(分量)。
- 统一入口是 `elem_filter(x, c)`，用于字节码 aastore（`bytecode.rs`）和 hw 写入（`hw_mem.rs`）。
- 逃逸数组从 `Node::Array` 回灌元素的边仍按分量类型收窄。开放世界里外部代码写入逃逸数组的未知对象，必须读得到。
- 这与改前的语义一致：改前这些调用点返回 open(Object)，元素读出后由 checkcast 收窄；分配点本身不带凭空收窄出的 open。
- 该过滤是集合上的逐元素判定，满足单调性，与工作表顺序无关。
- `OPEN_EXACT` 边不参与 Object 流的 SCC 合并（`flow.rs` 的 objf 判定）。

**本机验收**（二进制为 c1d-elem 工作树 HEAD 加本改动）：

| 项 | 结果 |
|---|---|
| TestJndiNoProvider 种子 0 / 1 / 2 | 均为 3820 类 / 24470 方法，三者相同；与 781eeec2 的类、方法集**完全相同**（+0 / −0），§27.1 的 +32 方法已消除 |
| TestModuleLayerDefine 种子 0 / 1 | 3084 类 / 18853 方法，与 c146f5c8 相同；比 781eeec2（3264 / 19219）少 180 类、366 方法，无新增 |
| `closure_cli::reflect_new_array_element_precision` | 通过（68 s） |
| `cargo test -p closure` | 160 通过 |

注：本机 `CARGO_TARGET_DIR` 指向多个工作树共享的目录。`cargo test` 用的 `CARGO_BIN_EXE_rava` 可能是别的工作树产出的二进制
（首次运行时该用例误报失败）。本机跑 driver 集成测试时须显式设 `CARGO_TARGET_DIR=../build/analyzer-target`。

## 28. 共享汇点精度：逐汇点分解与反事实实测（2026-10-05，分支 c1d-sink，基于 0192bf20）

> 编号说明：c1d-clsfact 线占 §26，c1d-elem 线占 §26（档案并集）/ §27（容器元素精度），本节取 §28，合并时按合入顺序重排。

任务书依据 c1d-clsfact §26 的判断：锚点启用后 HelloWorld 469 → 3190 类，主因是 `ModuleBootstrap.boot2` 链上的共享汇点
（`arraycopy`、`append(Object)`、逃逸汇点、`Formatter.format`、Unsafe CAS / release）把值汇合后再分发。目标 ≤ 569 类，且必须无损。
本节先量化每个汇点，结论是**这个判断不成立**：汇点合计只占 8 类；膨胀主体来自分析期未知运行期值下、按静态语义本来就可达的链。
测点统一为 HelloWorld，JDK 21，本机 macOS。锚点口径 runtime 同 §25.1（3190 类 / 18611 方法）；无锚点口径为 469 类 / 1813 方法。

### 28.1 诊断工具（仅诊断，缺省关闭；`--cut` 条目，结果不健全）

| 条目 / 查询 | 作用 |
|---|---|
| `--cut @node:<标签子串>` | 标签含该子串的类型流节点不接收任何值（流入边不建、直接注入忽略），用于量化单个汇点 |
| `--cut @noopenhub` / `@noreopen` / `@noopenrecv` | 分别关掉 open 接收者枢纽展开、G 增长重跑 open 展开、字节码调用点 open 接收者展开 |
| `--cut @edgeoff` | 触发边转储里的方法源带调用偏移（`M:方法@偏移`），离线按终态死区剔除过期边 |
| `--flows @fopen:<子串>` | 字段不折叠的来源：全局开关、deser、按名 / 按键 / 手写写入中匹配的项 |
| `--flows @in:<节点标签>` | 节点的直接前驱，按标签前缀分组计数（前 60） |
| `--flows @svcunk` | 服务 Class 实参所指未知的查找站点 |

### 28.2 逐汇点切除

每次切掉一个汇点节点（`@node:`，该节点不再接收任何值，下游全部失去经它到达的值）：

| 实验 | 类 | 方法 |
|---|---|---|
| 锚点基线 | 3190 | 18611 |
| 单个汇点切除（arraycopy 目的数组 / `append(Object)` 形参 / 逃逸汇点 / `Formatter.format` 实参数组 / Unsafe CAS·release 值形参，逐个） | 3187–3190 | — |
| 上述汇点全部切除 | 3182 | 18398 |
| 全部汇点 + 三项 open 展开全部关闭 | 2432 | 12592 |
| 无锚点 + 全部汇点切除 | 469 | 1805 |

全局字段节点 `field java/util/ImmutableCollections$Set12.e0` 是最大的汇合点（613 个数组分配点、293 个抽象对象、多种 open
类型），单独切除它同样没有效果。结论：**§26 的汇点归因不成立**，单个或全部汇点精确化至多减 8 类。

切除不单调：部分前缀切除反而增大（例如切掉某个 boot2 内层调用点后得 3881 类，切 `Objects.equals` 形参得 3231 类）。
原因是被切节点值集变空后，下游读者按初值折叠的分支翻转，可达面反而变大（§7 已记）。所以切除数只用于定位，不作逐项相加。

### 28.3 open 派发的反事实

| 实验 | 类 | 方法 |
|---|---|---|
| `@noopenhub` | 2619 | 14349 |
| `@noreopen` | 3190 | — |
| `@noopenrecv` | 3165 | — |
| `@noopenhub` + `@noopenrecv` | 2561 | 13687 |
| 无锚点 + 两者 | 461 | — |

即使完全不展开 open 接收者（不健全上界），仍有 2561 − 469 ≈ 2090 类，约 80% 的增量来自精确类型流本身，不来自 open 派发。

### 28.4 boot2 二分

按 `ModuleBootstrap.boot2` 的存活调用点做前缀切除（切掉偏移 ≥ x 的全部调用）：

- 切 ≥ 240：437 类；切 ≥ 292：3181 类。
- 跳变集中在 `boot2@240` 的 `SystemModuleFinders.of(SystemModules)` 及 264–285 段（建系统模块查找器、`Configuration` 解析）。
  更细的内层切除不单调，不再细分。

即：一旦系统模块图建起来（引导层存在），后续模块描述、读取器、资源定位全部按静态语义可达。

### 28.5 新增 2721 类按家族分解

方法：沿每个新增类的首达链，取最外层的「地标」帧归类（排除 `AccessController.executePrivileged` 这类透传帧）。
首达链不带克隆上下文，只用于排序（同 §23.6）。

| 家族 | 类数 | 典型链 |
|---|---|---|
| Charset 扩展 provider | 719 | `sun/nio/fs/Util.<clinit>` → `Charset.forName(sun.jnu.encoding)` → `lookupExtendedCharset` → `ServiceLoader` → `URLClassPath` → `JarVerifier` → `Signature` → JCA |
| toString 分派 | 445 | `ModuleDescriptor.toString` / `Version` 记号、`StringBuilder.append(Object)` 的多态接收者 |
| 类加载器 | 359 | `BuiltinClassLoader.findResource*` → `URLClassPath` jar 加载器 |
| equals 分派 | 289 | 集合元素 `equals` 的多态接收者 |
| 文件系统 | 222 | `FileSystems` / jrt / zipfs provider |
| 根其余 | 123 | — |
| JCA 补种 | 101 | seeds.toml JCA 段随 `Provider` 入链触发 |
| URL | 94 | `URL` / `URLStreamHandlerProvider` |
| security | 74 | `SecureRandom`、`Policy` |
| regex | 74 | `Pattern` 节点类 |

这些链按静态语义都是**正确**的：`sun.jnu.encoding`、`file.encoding`、`java.security` 配置、模块路径等运行期值在分析期未知，
`Charset.forName(未知名)` 必须覆盖全部扩展字符集，`ServiceLoader` 必须覆盖候选 provider。要无损地剪掉它们，只能让分析期知道这些值。

### 28.6 可无损回收的两项（实测上界）

1. **过期可达（stale reach）**：传播过程中常量 / 折叠事实非单调地先 Top 后收窄（例如字段在写入者入链前按 Top 处理），
   期间被访问的调用点在终态已被判死（如 `ClassFileDumper.<init>@52 → validateDumpDir` 落在终态死区 [51,56]），但可达是单调的、
   不会撤回。离线实测（`--cut @edgeoff --dump-edges` 后按终态 `dead_pcs` / `noreturn_dead_pcs` 剔除过期边，从根重扫）：
   锚点 3189 → 3153 类、18589 → 18327 方法（约 −36 类 / −262 方法）；无锚点 −5 方法、类数不变。
   **（§28.10 更正：上述数字把死区右端点误当闭区间；`dead_pcs` 是半开区间 `[a, b)`，更正后锚点只有 −4 类 / −34 方法）**
   终态做法：引擎在不动点后做一次「终态回收」，按终态折叠剔除死区出边，从全部根重算方法 / 类 / 初始化 / 反射面，
   输出只取重算后的集合。前提是触发边覆盖全部入链原因：目前转储只覆盖方法 / 类 / 分配 / 枢纽四类，无锚点口径下重扫
   只得 1049 / 1813 方法（根种类、补种、VM 规则、反射面未入转储），需先补齐成完整的入链边表。
2. **服务查找的有界未知镜像**：锚点口径下 `seeds.services_unknown = true`，选中目录里已在闭包中的 25 个服务。
   唯一的未知站点是 `ServiceLoader.load(Class,ClassLoader,Module)@7`，调用者是 `ResourceBundle.getServiceLoader@35`，
   服务 Class 来自 `ResourceBundle$3.run` 的 `Class.forName(this.val$providerName, …)`：名字经匿名类捕获字段传入，按名取类解析不到，
   结果为 open `Class`；随后 `ResourceBundleProvider.class.isAssignableFrom(c)` 的成立分支只对已知镜像收窄（`sub_mirrors` 保留 open）。
   精确做法：引入「所指未知但 ⊂ K 的类镜像」值（isAssignableFrom 成立分支把 open / 非镜像 Class 收窄为它），服务查找只选
   ⊂ K 的目录服务。临时探针（只选 `ResourceBundleProvider` 子类型的服务）实测：锚点 3190 → 3171 类（−19），无锚点不变。
   代价：引擎里有十余处按「`x == Class` 即所指未知 / 抽象对象即已知无静态字段」区分的站点（`hw_mem` 静态基址读、
   `hw_offset`、`hw_syntax`、`field_lookup`、`mirror_init`、`mirror_eq` 等），新值种类必须在每处都按「所指未知」处理，
   否则 Unsafe 静态基址读会漏掉 open（不健全）。

两项合计约 −55 类，HelloWorld 锚点口径约 3135，离 569 仍差约 2570。（§28.10 更正：合计约 −23 类）

### 28.7 结论与差距

- **≤ 569 在「无损」约束下不可达，阻塞点不是共享汇点**。增量的主体（§28.5 前五个家族约 2030 类）是分析期未知的运行期值
  （系统属性、安全配置、系统模块图）下按静态语义必须保留的链。
- 把它们剪掉的终态路线只有一条：让这些值在构建期确定——即「构建期引导镜像求值器」（§25.3，已单独立项）：执行
  initPhase1 / 引导步骤 / initPhase2，把系统属性表、SharedSecrets 访问器、引导层模块图物化为程序初始状态，分析从该初始堆出发。
  届时 `Charset.forName(sun.jnu.encoding)` 等按常量折叠，模块图按实际内容求值。
- §25.4 的三项前置（`Class` 接收者 classLoader / module 逐类求值、容器元素类型、G2 汇合点按实参分派）仍然有效，
  它们对应 §28.5 的「类加载器」「toString / equals 分派」家族（约 1090 类），在途于各自分支；合入后用同一 runtime 复测。
- 本分支只提交诊断工具与本节，不改引擎语义；§28.6 两项排在上述前置之后按序实施。

### 28.8 待用户决策

1. 锚点启用（boot layer 第 2–5 步）是否以「构建期引导镜像求值器」为前置：不做求值器，单例 HelloWorld 最好约 3135 − 1090 ≈ 2000 类级别。
2. 生产构建是否允许声明「封闭类路径 / 固定系统属性」（如 `file.encoding`、`sun.jnu.encoding` 取构建机或清单值）：
   这是对运行环境的假设，不是无损精度，但能直接折叠 Charset / 安全配置家族（约 800 类）。
3. §28.6 两项（约 −55 类）是否在前置合入前先做：收益小（约 1.7%），第 1 项需先补全触发边表。

### 28.9 暂停记录与恢复入口（2026-10-05，按协调者要求暂停，c1d-sink 582cab3e 之后）

**已完成**：§28.1–28.8 的分解与结论；诊断工具已提交（582cab3e，不改引擎语义）。本机已验证：无锚点 HelloWorld 仍为 469 / 1813；
closure 单测 159 通过；`@edgeoff` 转储与旧格式排序后逐行一致。

**在途作业（已按 PID 停掉调度进程；服务器上无残留进程，flock 锁随进程释放）**：

| tag | 内容 | 停止时状态 |
|---|---|---|
| sink-ut-582cab3e | 全量单测（generator + rava_macros_core） | 未跑完（服务器被占，反复让出），无结果 |
| sink-prof-582cab3e | 档案作业 24 片（同 c1de-prof-956db0b4 口径） | 第 01 / 02 / 03 / 05 / 09 片完成且 rc=0，其余 19 片未跑 |
| sink-sp-582cab3e | 抽查 6 例 | TestCustomException / Fibonacci / ArrayListDemo 通过；HelloWorld / ComprehensiveTest / BubbleSort 未出结果 |

服务器上留有这三个作业的检出目录（`/data/rava-spot-job-sink-{ut,prof}-582cab3e`、`/data/rava-spot-sink-sp-582cab3e`，
ubuntu 上为 `/mnt/d/workspace/java_rta-spot-job-sink-*`），没有删除。恢复时可以复用，也可以按数据目录规则清掉。

**未完成项（按顺序）**：
1. 以 582cab3e（或恢复时的分支头）重发 sink-ut 全量单测作业。
2. 重发档案作业。基线取 c1de-profb-8bb25e10：8bb25e10 → 0192bf20 没有代码差别，可以直接比较。逐例核对类 / 方法 / 反射集合
   相等（本分支不改语义，判据是完全一致）。
3. 抽查补齐：HelloWorld、ComprehensiveTest、BubbleSort。
4. 按 §28.8 的用户决策推进：先做 §25.4 前置三项的合入复测；§28.6 两项（终态回收需要先补全触发边表；有界未知镜像要逐处处理
   「所指未知」的站点）是否先做，等用户决定。
5. 与并行线的合并风险：
   - c1d-elem：只在本文档末尾追加时冲突，重排节号即可；
   - v11-vn：`flow.rs::flow()` 首行相邻插入（`tau_check_flow` 与 `node_cut`）冲突，两行都保留即可；
   - 其余文件自动合并。

### 28.10 恢复后：同步集成分支与两项回收的复核（2026-10-05，c1d-sink）

**同步**：已把 origin/rust-closure-analyzer（54850400）与 gate/39dc3e02（boot-image-s1）merge 进 c1d-sink，得到 0837196d。
唯一冲突在 `flow.rs::flow()` 开头，`node_cut` 与 V11 的 `tau_check_flow` 两行都保留。合入后复测：无锚点 HelloWorld 469 / 1813，
锚点口径 3190 / 18611，与合入前逐项相同。锚点临时 runtime 按 §25.1 重建在 `build/sink_work/rt_sink`
（原来放在 /tmp，已被巡检清掉）。

**过期可达复核（第 1 项）：实际收益几乎为零**。§28.6 的离线重扫把死区当作闭区间 `[a, b]`。但折叠导出的 `dead_pcs` /
`noreturn_dead_pcs` 是半开区间 `[a, b)`（`fold_noreturn_dead_bytes = Σ(b − a)`），所以落在 `b` 上的边其实是存活的
（例：`NormalizerBase$NFKDMode.getNormalizer2` 死区 `[7, 16)`，16 处的 `<clinit>` 触发边是活的）。按半开区间重扫：

| 口径 | 过期边 | 类 | 方法 |
|---|---|---|---|
| 锚点 | 329 | 3189 → 3185（−4：`Path$1`、`StructureViolationException`、`JrtPath$1`、`ClassFileDumper$2`） | 18589 → 18555（−34） |
| 无锚点 | 4 | 不变 | −4 |

过期边主要来自两类：noreturn 折叠出现在调用点首次处理之后（`WeakHashMap$*Spliterator.tryAdvance`、`ProtectionDomain.toString`），
以及 `ModuleBootstrap.decode` / `addModules` 的系统属性折叠。

无损的终态做法只有一种：不动点之后，把终态死区当作已知死代码，再跑第二遍分析。第二遍的状态是第一遍的子集，第一遍的折叠对
子集依然成立，所以是健全的。代价是分析时间翻倍：锚点约 18 s → 36 s，档案作业每例都要多跑一遍。换来的只是 −4 类，
和「引擎提速优先」相悖，因此**不实施**。如果以后折叠的时机能改成单调的（例如 noreturn 判定确定之前先挂起调用点的入链），
可以零代价拿到这部分收益，记为引擎改进的候选。

**有界未知镜像（第 2 项）：只在锚点口径下有收益，而锚点口径已被引导映像求值器取代**。实测 −19 类（3190 → 3171），
无锚点为 0。生产构建当前 `anchors = []`，这一项对任何实际构建都没有收益。引导映像求值器第 1 步（boot-image-s1，
`docs/plans/2026-10-05-boot-image-evaluator.md` §5）已给出 HelloWorld 闭包上界：JDK 21 ≤ 525，JDK 25 ≤ 465，都在 569 以内。
走映像路线后，`ResourceBundle.getServiceLoader` 这类引导期路径不再入链，这一项的收益随之消失。实施它要改十余处「指向未知
类的 Class」判定（`hw_mem` 静态基址读 ×2、`hw_offset`、`hw_name_write`、`hw_syntax`、`mirror_eq`、`classes::sub`、`ty` 等），
漏改任何一处都是不健全，风险与收益不相称，因此**不实施**。

**结论**：共享汇点线到此收口，引擎语义没有改动。§28.6 原估的 −55 类，更正后为 −23 类：第 1 项 −4，第 2 项 −19，
后者还只在锚点口径下才有。≤ 569 的达成路线是引导映像求值器（在途），不是锚点口径下的精度修补。本分支只保留诊断工具
（`--cut @node:` / `@noopenhub` / `@noreopen` / `@noopenrecv` / `@edgeoff`，`--flows @fopen:` / `@in:` / `@svcunk`）。

### 27.3 同步集成分支（140ef55e，含 V11 类型恒等边 / V12）

- `flow.rs` 冲突：本分支的 objf 判定屏蔽 `OPEN_EXACT`，集成分支（V11）在同代表跳过条件上加了 `ident_edge`。两者都保留：
  `rs == rd && (objf || ident_edge(...))`，objf 仍按 `NOT_SUB | OPEN_EXACT` 屏蔽。
- V11 的 `tau.rs::ident_edge` / `tau_check_flow` 与 `hvn_diag.rs` 原先只识别 `NOT_SUB`，带 `OPEN_EXACT` 位的过滤会被当作类型序号去下标 `names`。
  这里取保守解：`OPEN_EXACT` 边一律不算恒等边；作为入边时，按「非 τ 子类型」使目标失去封闭类型。
  另一种解是去掉标记位后按分量类型判定。`filter_open_exact` 的输出 ⊂ 分量类型，这样做也健全，而且能多合并一些。
  未采用的原因：保守解只少合并，闭包结果不变；而且目前 `OPEN_EXACT` 边只写入 `Node::E`，`E` 节点本来就没有封闭类型。

## 29. a5-4b 归因：JarVerifier / pkcs11 的真实来源与所需能力（2026-10-06，分支 c1d-a54b，基于 140ef55e）

**结论先行**：a5-4b 原设想（§21.5「引导加载器 `ucp` 在未设 `-Xbootclasspath/a` 时为 null」）在 140ef55e 上回收为 0：
`ClassLoaders.<clinit>@66..72` 建 boot `URLClassPath` 的分支已按 `jdk.boot.class.path.append` 缺席折叠为死，活的
`URLClassPath.<init>(String,Z)` 调用点只剩 `@144`（应用类路径）。JarVerifier 与 pkcs11 是三条彼此独立的路线，每条都要一项
本步范围外的新能力才能健全地剪掉；本步**不改引擎语义、无代码提交**，只落归因与反事实实测。DeepCopy 前后均为 3374 类 / 20766 方法。

口径：本机 macOS、`rava closure --jdk 21`，DeepCopy 基线 3374 类 / 20766 方法 / 约 40 s。`--cut` 是反事实切除（不健全，只作归因）。
记号：`R` = `URLClassPath$3.run:()…Loader;`，`J` = `sun/net/www/protocol/jar/Handler.openConnection:(Ljava/net/URL;)…`，
`P` = `ProviderConfig.doLoadProvider:()Ljava/security/Provider;`（方法体整段），`S` = `ResourceBundle.getServiceLoader:(Module,String)@16`
（`ServiceLoader.load(service, loader, module)` 调用点）。

### 29.1 反事实实测

| 切除 | 类数 | 变化 | pkcs11 / smartcardio / ec / XMLDSig(com/sun/org/apache/xml) / org/jcp | JarVerifier |
|---|---:|---:|---|---|
| 无（基线） | 3374 | — | 29 / 8 / 87 / 111 / 16 | 在 |
| `BuiltinClassLoader.findClassOnClassPathOrNull` 方法体 | 3372 | −2 | 不变 | 在 |
| `JarFile.getManifestFromReference@69`（`new JarVerifier`） | 3213 | −161 | 不变 | 去 |
| `R@97` + `R@139`（两处 `new JarLoader`） | 3371 | −3 | 不变 | 在 |
| 只切 `J` | 3353 | −21 | 不变 | 在（经 JarLoader） |
| D：`R@97` + `R@139` + `J` | 3157 | −217 | 不变 | 去 |
| P | 3353 | −21 | 12 / 8 / 87 / 111 / 16 | 在 |
| S | 3355 | −19 | 不变 | 在 |
| P + S | 2990 | −384 | **全部 0** | 在 |
| D + P | 3136 | −238 | 12 / 8 / 73 / 111 / 16 | 去 |
| D + P + S | 2766 | −608 | 全部 0 | 去 |

- JarVerifier 与签名校验链（PKCS7、SignatureFileVerifier、x509 / provider / rsa 等约 217 类）要**同时**堵住类路径来源（甲：
  `$3.run` 的 JarLoader 分支）和协议处理器来源（乙：`URL.handler` 含 jar `Handler`）才出闭包，与 §22.1 的 StockTrans 结论一致。
- **pkcs11 / smartcardio / ec / XMLDSig 不走 jar 链**：D 之后原样保留。它们来自两条 Provider 服务查找，P 与 S 单切各只 −20 左右，
  合切 −384 且五个包全部清零（闭包多连通，单切时另一条照样拉进同一批类）。

### 29.2 路线一（甲 + 乙）：JarVerifier

首次发现链：`ObjectInputStream.<init>` → `ObjectInputFilter$Config.<clinit>` → `System.getLogger` → … → `sun/nio/fs/Util.<clinit>`
→ `Charset.forName` → `ExtendedProviderHolder` → `ServiceLoader` → `loadProvider` → `Class.forName(Module,String)` →
`BuiltinClassLoader.findClassOnClassPathOrNull` → `URLClassPath.getResource` → `getLoader(URL)` → `$3.run@139` `new JarLoader`
→ `JarLoader.getClassPath` → `JarFile.getManifest` → `new JarVerifier`。即**应用**类路径，不是引导类路径。

**按对象 URL 实验（未提交）**：给「容器形态」判据（`engine/classes.rs::container_shape`）加一条结构判据——持有类型为某个
`[facts.keyed_lookups]` 键类（按键查找入口的返回类型，此处 `URLStreamHandler`）的实例字段的类按分配点区分。判据只取清单事实、
不含类名。实测 `URL` 确已按分配点区分（`URL.handler of java/net/URL@<方法>:<偏移>`），`fileToEncodedURL` 产出的类路径 URL
只含 file `Handler`。但闭包：单独 3374（0 变化）；叠加 `R@97` / `R@127` / `R@139` 三处反事实切除也只 3369（−5，对照 D 的 −217）。
原因：

- `URLClassPath$FileLoader.getResource@0` 的 `new URL(getBaseURL(), ParseUtil.encodePath(name, false))` 走解析式构造器，
  spec 推不出（`name` 来自任意类名 / 资源名，`encodePath` 是逐字符变换），`@386` 的协议键闸门退回全集，jar `Handler` 写进该
  对象的 `handler`；该 URL 经 `getResources` 流到 `ServiceLoader$LazyClassPathLookupIterator.parse@9` 的 `openConnection`，派发到 `J`。
- JDK 语义下这条派发不可达：`FileLoader.getResource@20..38` 要求 `url.getFile().startsWith(normalizedBase.getFile())`，否则返回 null；
  `normalizedBase` 的 file 以 `/` 开头（`fileToEncodedURL` 保证），而 jar URL 的 file 是嵌套 URL 串（`<scheme>:…!/…`），
  不可能以 `/` 开头。证明它需要「按处理器区分的 `URL.file` 前缀」这类串域推理，分析器目前没有。
- 甲同理：`$3.run@139` 只在 `file` 不以 `/` 结尾时走 JarLoader；应用类路径 `""` → 当前目录，`fileToEncodedURL` 仅在
  `File.isDirectory()` 为真时补 `/`。要剪掉需要 §22.2 的两处运行期对齐 + 「启动目录是目录」清单事实 + `isDirectory` 调用按
  文件对象区分，单独收益 −3。

所需能力（终态）：URL 按对象字段精度（上面的判据可直接用）+ `URL.file` / spec 的前缀串域推理（含 `encodePath` 这类逐字符
变换的前缀保持）+ 甲的目录事实。三者齐备才兑现 D 的约 −217。

### 29.3 路线二（P）：JCA 提供者逐个装载

`ObjectStreamClass.computeDefaultSUID` → `MessageDigest.getInstance("SHA")` → `ProviderList.getService` 逐个取 provider →
`ProviderConfig.getProvider` → `doLoadProvider` → `ProviderConfig$ProviderLoader.<clinit>` → `ServiceLoader.load(Provider.class, …)`，
引导层全部 Provider 服务的 provider（SunPKCS11、SunPCSC、XMLDSig、SunEC 等）入链。

JDK 语义：`java.security` 的 provider 顺序已嵌入（`runtime/java_runtime/src/jdk_resources/java.security.21.properties`，
SUN 第 1），原生二进制没有 `-Djava.security.properties`；SUN 自带 SHA，`getService` 在第 0 个 provider 就返回，非内建 provider
（走 ServiceLoader 的那些）不会装载。所需能力：**JCA 提供者序求值**——闭包内无 `Security` 改写入口（`setProperty` /
`insertProviderAt` / `removeProvider` 等）可达时，provider 表是构建期事实；对每个被请求的（服务类型, 算法）按表序找到第一个
内建且在 `[jca] providers` 登记里提供该服务的 provider，之后的 provider 不入链；有推不出算法名的请求时退回全表。
限制：jar 链上的 `PKCS7` / `SignatureFileVerifier` 请求推不出算法名，所以 P 的收益以路线一先完成为前提（D + P 才是 −238）。

### 29.4 路线三（S）：ResourceBundle 服务查找的未知 Class

`ResourceBundle.getServiceLoader@16` 的服务 Class 实参为 `open(java/lang/Class)`（来自 `getResourceBundleProviderType` 里按名
`Class.forName`，`ResourceBundleProvider.class.isAssignableFrom(c)` 的收窄对 open 不生效），按 `engine/services.rs` 的规则选中闭包内
全部目录服务，Provider 服务随之入链。所需能力即 §28.10 的「有界未知镜像」；§28.10 已按风险收益判为不实施，结论是这条引导期
路径由引导映像求值器路线消解。本步不改判。

### 29.5 对 a5-4 的影响

- a5-4b 作为独立一步关闭（无独立的健全收窄手段）；它的收益拆到三项能力上：①URL 按对象 + 串前缀推理 + 甲目录事实（约 −217，
  与 tasks.md「jar/URL 来源甲」及乙的余项同一件事）；②JCA 提供者序求值（叠加①后约再 −21）；③引导映像路线消解 S。
  ①②③ 齐备的反事实上界是 DeepCopy 3374 → 2766（−608），pkcs11 / smartcardio / ec / XMLDSig 全部出闭包。
- 达到 ≤1640 仍要 a5-4e / a5-4f 与 §21.5 列的其余来源；2766 只是这三项的上界。
- 测量脚本：`build/a54b/cl.sh`（`rava closure` 包装，经全机锁）与 `build/a54b/fp.py`（按包指纹对比），均在 scratch，不提交。

## 30. 能力①：URL 来源精度——阻塞清单、终态设计与验收（2026-10-06，分支 c1d-url，基于 b60e4f36）

> 状态（2026-10-08）：§30.6–§30.17 待合批验证（c1d-url-b2 3391eb2d，已合入 batch-1008 04600e21）；合批语义取舍见 §30.18。

**结论先行**：§29.2 估计的「URL 按对象 + 串前缀 + 甲目录事实」三项**不够**。实测表明，jar `Handler.openConnection`（下记 `J`）
与 JarVerifier 链至少还有 5 个彼此独立的来源，每个来源各要一项能力才能健全剪掉。本步**不改引擎语义、无代码提交**：
只落阻塞清单、反事实上界和终态设计。按对象 URL 判据的实验补丁记在 §30.4，作为第 1 步的起点。

口径：本机 macOS，`rava closure`，`--java-home tools/refjdk/jdk-21.0.11+10`，冷缓存。`--cut` 是反事实切除（不健全，只作归因）。
记号 `R` = `URLClassPath$3.run:()…Loader;`。基线（b60e4f36）：

| 测试 | 类 | 方法 |
|---|---:|---:|
| HelloWorld | 469 | 1828 |
| StockTrans | 3372 | 20747 |
| DeepCopy | 3374 | 20766 |
| TestSerialDefaultSuid（TSDS） | 3379 | 20760 |

（HelloWorld 本机 469、服务器口径 468，差 1 类是本机与服务器的环境差，与本节无关。）

### 30.1 反事实逐层剥离（DeepCopy）

每一层都在上一层的切除集上追加，每次追加后重查 `J` 的 `--why` 链和 `@opens:java/net/URL` 引入点：

| 层 | 追加切除 | 类 / 方法 | `J` 仍可达的原因 |
|---|---|---:|---|
| 0 | 无 | 3374 / 20766 | — |
| 1 | `R@97` `R@127` `R@139`（甲 + 通用 Loader） | 3371 / 20719 | `ServiceLoader$LazyClassPathLookupIterator.parse@9` 的 `openConnection` |
| 2 | + `BuiltinClassLoader.findResource@113`、`findResources@129`、`URLClassPath.<init>(String,Z)@157`、`ResourceBundle$Control$2.run@8`、`Control.needsReload@65` | 3369 / 20679 | `parse` 的 P1 = {`FileLoader.getResource` 的 URL 对象, open(URL)} |
| 3 | + `URLClassPath$FileLoader.getResource@13`（模拟「该对象 handler 只有 file」） | 3369 / 20679 | open(URL) 引入点：`BuiltinClassLoader$1.hasNext@31`（`Iterator.next` + checkcast）、`URLClassPath.getLoader(I)@36`（`unopenedUrls.pollFirst` + checkcast） |
| 4 | + `BuiltinClassLoader$1.hasNext@26`、`URLClassPath.getLoader(I)@33` | 3369 / 20676 | 新引入点 `nextProviderClass@173`（`Enumeration.nextElement` 在 open(Enumeration) 上的枢纽返回 + checkcast）。JarVerifier 另有独立路线：`ObjectInputFilter$Config` → `BootLoader.getDefinedPackage@33` → `BootLoader$PackageHelper.definePackage@67` → `getManifest` → `new JarInputStream` → `checkManifest@120` |
| 5 | + `nextProviderClass@168`、`BootLoader$PackageHelper.definePackage@67` | **3097 / 18629** | `J` 与 JarVerifier 都不在闭包内 |

第 5 层是能力①在 DeepCopy 上的反事实上界：**−277 类 / −2137 方法**。比 §29 的 D（−217）多出的部分，来自 BootLoader 包定义路线带进的 jar 清单类。
`build/url/` 下的 `cuts1.txt` / `cuts3.txt` / `cuts6.txt` / `cuts7.txt` 依次是第 2–5 层的切除集（scratch，不提交）。

注意：不能用切 `URL.<init>(URL,String,URLStreamHandler)@386`（协议键闸门）的办法模拟「各处 spec 已知」。切构造器内部的事件
不单调，实测运行 20 分钟以上不收敛，已放弃。

### 30.2 阻塞清单（每项都要独立的健全能力）

| # | 来源 | 现状 | 终态能力 |
|---|---|---|---|
| B1 | 甲：`R@139` / `R@97` 的 JarLoader 分支，`R@127` 的通用 Loader | 类路径 URL 的 `file` 是否以 `/` 结尾、`protocol` 是否为 `"file"` 都推不出 | ①「启动目录是目录」清单事实（`[facts]`，原生二进制的应用类路径固定为 `""` → `user.dir`，无 `-cp`）+ `File.isDirectory` 按文件对象求值；② URL 对象的 `protocol` / `file` 按对象串事实（构造器写入的常量与前后缀）；③ `endsWith("/")` / `String.equals` 条件按对象串事实收窄分支 |
| B2 | URL 容器出口的 checkcast：`BuiltinClassLoader.findResource@118`、`findResources@134`、`$1.hasNext@31`、`URLClassPath.<init>@160`、`getLoader(I)@36`、`nextProviderClass@173` | 容器元素未按对象建模时，`Iterator.next` / `pollFirst` / `Enumeration.nextElement` 落到枢纽返回，checkcast 造出 open(URL)；open 接收者走无上下文的 `URL.handler`，后者含全部 handler | 对 `ArrayList` / `ArrayDeque` / 匿名 `Enumeration` 的元素按分配点建模（`container()` 判据延伸到元素出口），使这些 checkcast 的结果只含对象、不含 open。判据只看字节码形态（元素字段 + 出口方法），不写类名 |
| B3 | 手写 `ClassLoader.__impl_getResource`（`class_loader_impl.rs`，`#[jvm_boundary]`）恒返回 null，分析器却把返回建模为 open(URL) | 手写边界的返回值建模过宽 | 随 a3-L2 改为字节码翻译后自然消失；在那之前按清单声明返回值只含 null，不引入 open |
| B4 | 未知 spec 的解析式构造：`FileLoader.getResource@13`（`encodePath(name)`）、`URL.readObject` / `fabricateNewURL`（反序列化）、`SeedGenerator$URLSeedGenerator`（`securerandom.source` 属性）、`JarURLConnection.parseSpecs`（jar 链内部，自循环） | `@386` 协议键闸门在 Any 键下退回全集，jar `Handler` 写进该对象的 `handler` | ① URL 按对象字段精度（§30.4 判据）；② `FileLoader.getResource@20..38` 的 `getFile().startsWith(normalizedBase.getFile())` 守卫：需要「相对 spec 在 file 基 URL 上解析，结果 `file` 以 `/` 开头」+「jar URL 的 `file` 以 `<scheme>:` 开头」这两条按处理器区分的**清单事实**。注意 `new URL("jar", "", "/x")` 这种三参构造的 jar URL，`file` 也以 `/` 开头，所以事实只对解析式路径成立，不对 `URL.file` 全局成立。`URLStreamHandler.parseURL` 里有 `/./`、`/../` 子串循环，纯字节码推前缀不可行；③ 反序列化与安全属性两处的键由引导映像 / 属性求值给出（`securerandom.source` 缺省 `file:/dev/random`，已在嵌入的 `java.security` 内） |
| B5 | `BootLoader$PackageHelper.definePackage@67` → `getManifest` → `JarInputStream` → JarVerifier | `getSystemPackageLocation`（native）返回值未建模，非 `jrt:` 分支活 | 清单事实：原生二进制没有 `-Xbootclasspath/a`，引导包的位置只有 `jrt:/<module>`。把 native 返回值建模为串前缀 `jrt:`，`definePackage` 的 jar 分支按前缀折叠 |
| B6 | `nextProviderClass@168` 的 `configs` 来自 `ClassLoader.getResources` / `getSystemResources`，在 open(ClassLoader) 上派发 | 同 B2 | 同 B2，并随 B3 收窄 |

健全性论证（每项单独成立）：
- B1 的目录事实只在应用类路径固定、无用户可设项时成立。rava 生产构建不接受 `-cp`，类路径资源由构建期打包，这一点要在清单里登记为事实。
- B2 的元素按对象建模只是**精化**：checkcast 结果集合是元素集合的子集，没有丢失可能的值。
- B3、B5 是把 VM / native 的已知行为写进清单，属于 handwritten-boundary §③ 的「VM 注入状态的落地语义」。
- B4 的守卫只在两条串事实都成立时折叠 `J`；任何一条推不出就退回现状（全集），所以不会漏。

### 30.3 终态目标与验收

- 能力①兑现后（B1–B6 齐备）：DeepCopy 3374 → **≤3097**（−277，按 §30.1 第 5 层），jar `Handler` 的分派方法
  （`parseURL` / `parseAbsoluteSpec` / `parseContextSpec` / `canonicalizeString` / `openConnection` / `hashCode` / `sameFile` / `newURL` 等，
  即经 `URLStreamHandler` 虚分派进来的全部方法）与 `JarURLConnection`、`JarVerifier` 及签名校验链不在闭包内。
  jar `Handler` **类本身**可以留在闭包内：`URLClassPath.<init>(String,Z)@185` 无条件 `new` 它并写入 `jarHandler`，是必然实例化的，
  只保留类与构造器（2026-10-06 协调方改写，依据见 §30.6「去掉 B5 一项」）；
  StockTrans / TSDS 走同一组来源，预期同量级（落地时实测，不足 −217 即不达标）。HelloWorld 不走 URL，保持不变（服务器 468）。
- a5-4 总目标沿用 mh-objectify §6.3 的修订值：**DeepCopy ≤2803 / StockTrans ≤2807 / TSDS ≤2809 / HelloWorld 468**，由①②③与格式串求值共同兑现，①的验收口径是「约 −217 且 JarVerifier 链出闭包」（本节实测上界 −277）。
- 验收：服务器单测 rc=0（含 D1 顺序 / 种子无关守护）；抽查 DeepCopy 系、`TestAppClassLoader`、`TestJarFile*`、URL / 类路径相关 e2e 与 HelloWorld 全过；
  动态对照漏覆盖 0、存根命中 0；`--hash-seed` 与 `--flow-batch 1` 两种跑法下闭包集合一致。

### 30.4 第 1 步起点：按对象 URL 判据（实验补丁，未提交）

在 `engine/classes.rs::container_shape` 的 `if !generic { return false; }` 之前加一条判据：类的实例字段类型是某个
`[facts.keyed_lookups]` 键类（按键查找入口的返回类型）时，按分配点区分对象。同时把 `engine/keyed.rs::key_classes`
改为 `pub(super)`。判据只读清单，不含类名：

```rust
let kcs: Vec<String> = self.key_classes().into_iter().map(|k| self.names[k as usize].to_string()).collect();
if chain.iter().any(|cf| inst(cf).iter().any(|f| f.desc.strip_prefix('L').and_then(|d| d.strip_suffix(';')).is_some_and(|c| kcs.iter().any(|k| k == c)))) {
    return true;
}
```

实测：单独加这条，DeepCopy 不变（3374）；叠加第 1 层切除为 3371。它是 B4① 的前提，但单独没有收益，所以不在 C4 冻结期提交。

### 30.5 实施顺序与恢复入口

建议按「收益可独立观测」排序，每步单独分支、小步合入：
1. **B2 + B6**：容器元素出口按对象。这一步消掉 open(URL) 的 6 个引入点，是其余各项能看到收益的前提。可用 `@opens:java/net/URL` 检验：引入点应当归零，只剩 B3。
2. **B3**：手写 `getResource` 的返回值按清单声明。
3. **B4**：§30.4 判据 + 两条处理器串事实 + `startsWith` 守卫收窄。
4. **B1**：目录事实 + 按对象串字段 + `endsWith` / `equals` 收窄。
5. **B5**：`jrt:` 前缀事实。

每步完成后，按 §30.1 的表格把对应切除项从 `cuts7.txt` 中拿掉，重跑并确认仍是 3097。

恢复入口：worktree `../java_rta_c1durl`（分支 `c1d-url`）。测量脚本 `build/url/cl.sh <Test> <tag> [rava closure 参数]`（经全机锁、冷缓存）。
切除集 `build/url/cuts7.txt`；实验补丁 `build/url/per_object_url.patch`。以上文件都在 scratch，补丁正文已抄在 §30.4。

### 30.6 第 1 步实施：B2 + B6 容器元素出口按对象（2026-10-06，分支 `c1d-url-b2`，基于 353d5162）

**结论先行**：B2 的 open(URL) 引入点 `findResource@118`、`findResources@134`、`elements [Ljava/lang/Object;@…` 全部消失；
§30.2 列的 `$1.hasNext@31`、`URLClassPath.<init>@160`、`getLoader(I)@36` 在本基线上已不是引入点。剩余引入点只有三个：
`ServiceLoader$LazyClassPathLookupIterator.nextProviderClass@173`（B6 的残留，来源是 B3）、`ClassLoader.getResource` 的返回（B3），
以及 `URL.equals@1`（`Object` 形参上的 `instanceof`，不属于容器出口）。本步在四例上的独立收益是 −8 类 / −153 方法。
`J` 和 JarVerifier 链仍在闭包内，要等 B3 起各步兑现（见下文「收益归属」）。

#### 根因链（DeepCopy，`@path` / `@in` / `@objs` 逐层查）

1. `findResource@118` 的 checkcast 结果来自 `List.get`。这个 list 是 `findMiscResource@56` 从资源缓存 map（一个
   `ConcurrentHashMap` 分配点）用 `get` 取出的值。
2. `ConcurrentHashMap.get` 经静态辅助 `tabAt` 读表。`tabAt` 自身按 `ctxsel.rs` 规则 1 继承 map 上下文，但它在调用
   `Unsafe.getReferenceAcquire` 这个字节码包装。包装的接收者是单例、不是抽象对象，所以进了方法本体；本体内只有一个
   native `getReferenceVolatile` 调用点，全部 map 的表都在这里汇合。结果是任何一个 map 的 `tabAt` 都读出全部 map 的节点，
   节点的 `val` 也就带上了全部 map 的值。
3. 修好第 2 层后，`tabAt` 只读出本 map 的 9 个节点对象，但 `get@104`（`Node.find` 的结果 `.val`）仍是全集。原因是树箱
   `TreeBin@396:137#@map` 在 `putTreeVal` 里以自身为上下文分配 `TreeNode`，堆上下文链 `@site#@396:137` 截断到 `HEAP_DEPTH`
   后丢掉了属主 map。于是各 map 的树箱共用同一组树节点，`find` 读出全部 map 的值。

#### 实现（两项通用能力，生成器内无类名）

- **内存访问中继方法继承调用方上下文**（`engine/relay.rs`，接在 `invoke.rs` 的非对象接收者分支与 `dispatch_one`）。
  - 中继方法的定义：字节码方法，其引用形参直接或经下游中继方法，作为实参流到清单 `[facts.memory_reads]` 的 `src`，
    或 `[facts.array_writes]` 的 `dst` / `values` / `elements` 槽。
  - 当调用方处在某个上下文中、接收者不是抽象对象时，中继方法继承调用方上下文，与静态辅助方法同一口径。
    具体求值上下文不外传。
  - 判定方式：在字节码调用图上求最小不动点，按「自 key 可达的未定成员集」一次求解并整体缓存。
    解唯一，与查询顺序和成环形态无关（D1）。每个成员的调用边只算一次。
  - 首版逐成员递归、不缓存成环结果，DeepCopy 要 697 s；改成不动点后为 39 s。
- **同巢内部分配沿用属主链**（`engine/classes.rs::obj_at` + `internal_alloc`）。
  - 分配方对象与新对象同类，或同属一个嵌套巢（JVMS §4.7.28 `NestHost`，且分配方是巢成员）时，新对象的链取分配方链
    去掉其自身分配点的部分，即属主链。
  - 巢宿主分配成员对象不算内部分配：宿主就是数据结构的属主，成员对象按宿主分开。
  - 分配方没有属主（链长 1）时，退回原规则：递归结构不延长链，其余以分配方为上下文。
- 诊断：`--flows '@objs:<节点子串>'` 列出匹配节点值集里的抽象对象 / 数组分配点名，并标出逃逸对象。
- 守护测试：`driver/tests/closure_cli.rs::container_elements_per_object`，fixture 是 `ElemTrack.java`。
  - 用例：两个 `ConcurrentHashMap` 各存一种元素，从 m1 取出后 cast 再派发。
  - 断言：`Square.name` 不得入链。改动前它入链，改动后不入链。

单独只加中继一项时，DeepCopy 为 3195 / 20447，与基线相同，`@118` / `@134` 仍是引入点。两项叠加才见效。

#### 实测（本机 macOS，`rava closure`，refjdk 21.0.11+10，冷缓存；基线 353d5162）

| 测试 | 基线 类 / 方法 | 本步 类 / 方法 | 差 | 分析耗时 ms | 方法上下文 |
|---|---:|---:|---:|---:|---:|
| HelloWorld | 469 / 1828 | 469 / 1827 | 0 / −1 | 489 → 515 | 4056 → 4198 |
| StockTrans | 3193 / 20429 | 3185 / 20276 | −8 / −153 | 45971 → 25296 | 95451 → 93454 |
| DeepCopy | 3195 / 20447 | 3187 / 20294 | −8 / −153 | 44550 → 25837 | 95350 → 93331 |
| TestSerialDefaultSuid | 3200 / 20441 | 3192 / 20288 | −8 / −153 | 45768 → 27854 | 95205 → 93171 |

（§30 开头表格的基线是 b60e4f36 上的值。353d5162 之前合入的其他改动已把三例降到约 3195，本节一律以 353d5162 为基线。）

- 四例都是严格子集，新增类 0、新增方法 0。
- 三例去掉的 8 个类相同：`java/time/{MonthDay,OffsetDateTime,Year,YearMonth}$1`（`ChronoField` 的 switch 映射表），以及
  `java/util/stream/Nodes$CollectorTask$OfInt` / `Nodes$SizedCollectorTask$OfInt` / `Nodes$ToArrayTask$OfInt` /
  `Nodes$ToArrayTask$OfPrimitive`。它们原先是靠 map 值汇合造出的 `TemporalAccessor` / `IntStream` 接收者派发进来的。
- HelloWorld 去掉 `ConcurrentHashMap$ReservationNode.find`：`ReservationNode` 只经 `computeIfAbsent` 写进被计算的那个 map 的表，
  读这个 map 的 `get` 才会派发到它的 `find`。
- 分析耗时降了约 40%：值集汇合减少，传播量随之下降。

#### 健全性论证

- **上下文选择不影响健全性。** 两项改动都只改「方法克隆按哪个上下文」和「分配点按哪条链命名」。
  - 每个运行期调用仍落在某个方法克隆上，克隆的形参取自其调用方的实参，全部克隆之并就是原本体的值集。
  - 每个运行期分配仍映射到恰好一个抽象对象。
  - 手写内存访问调用点在克隆内按克隆的实参接入，读出的仍是该实参所指对象的元素 / 字段全集。
  - 所以改动只做细分，不丢值。上下文敏感指针分析对任意上下文抽象都健全，这里没有依赖某种特定选法。
- **判定错误只影响精度，不影响健全。** 中继判定的任一处误差（例如保守分析的方法记为无边）只会让方法退回本体，也就是原行为。
- **引擎不假设克隆的接收者就是上下文对象。** 检查过，`ctx` 只用于堆链命名、上下文选择与具体求值标记；具体求值上下文已排除。
- **结果是严格子集。** 实测四例的类集与方法集都是基线的严格子集（上表）。去掉的成员都能用「元素按 map 分开后，派发接收者不再含该类型」解释。
- **与顺序无关（D1）。** 中继判定是唯一的最小不动点；内部分配规则只读类文件的 `NestHost` 属性。本机已跑
  `closure_independent_of_hash_seed`（StockTrans / DeepCopy / TSDS 等 7 例，换种子后集合一致）与 `container_elements_per_object`，两项均通过。

#### 收益归属（为何本步只有 −8）

open(URL) 退出容器出口以后，`J`（jar `Handler.openConnection`）与 JarVerifier 链仍各有独立来源：

- **B3：** `ClassLoader.getResource` / `getResources` / `getSystemResources` 是手写边界（`class_loader_impl.rs`），返回值被建模为
  open(URL) / open(Enumeration)。`nextProviderClass@106/@48` 的 `configs` 因此只有 open(Enumeration)，`@168` 的 `nextElement`
  落到枢纽返回，`@173` 的 checkcast 造出 open(URL)。这就是 B6 的残留，B6 随 B3 兑现。
- **B4 / B1 / B5：** 不变（§30.2）。

§30.1 的 −277 上界要在 B3 → B4 → B1 → B5 依次落地后兑现。本步是这些步骤看到收益的前提：B3 只要按清单收窄
`getResource*` 的返回，`@118` / `@134` / `@173` 就不会再从容器出口重新引入 open(URL)。

#### 遗留

- `URL.equals@1`：`Object` 形参 `instanceof URL` 产生 open(URL)，属于形参来源精度，不在 B2 范围，留给 B4 一并评估
  （其接收者 handler 是否带进 `J`，需在 B3 后重查）。
- §30.4 按对象 URL 判据仍未提交。它是 B4① 的前提，单独没有收益，随 B4 一起做。
- 内部分配规则对「巢成员分配巢成员」一律沿用属主链。在属主相同、分配方不同的情况下，会少一层分配方区分，可能损失精度。
  四例实测是净收益，健全性不受影响。

#### 验证

- 本机：`cargo build --release -p driver` 通过；`closure_cli` 的 `container_elements_per_object` 与 `closure_independent_of_hash_seed` 通过。
- 服务器单测作业 `c1db2-ut-8f2dc45c`（ref 8f2dc45c，全量 generator 单测 + rava_macros_core）：rc=0，521 通过、0 失败。
- C4 全量冻结期内，本分支不合入、不发起抽查。冻结解除后合入，并按 §30.3 抽查 DeepCopy 系、`TestAppClassLoader`、
  `TestJarFile*`、HelloWorld，同时做动态对照。

### 30.7 第 2 步实施：B3 手写方法返回值来源（2026-10-06，分支 `c1d-url-b2`，提交 b9335ac7）

#### 设计（终态通用，分析器不写类名，清单不新增条目）

手写体本身就是成员的运行期语义，返回值来源直接从手写体语法推出：

- **语法判定**（`handwritten/returns.rs`）：返回点 = fn 体尾表达式（穿过块 / `unsafe` / 带 else 的 `if` / `match` 各分支）与全部 `return`
  （闭包、async 块、嵌套 item 内的不算）。每个返回点只认两种形态：
  - null：`Ok(Default::default())` / `Ok(T::default())`；
  - 路径调用 `T::m(…)` 直接作尾表达式，或 `Ok(T::m(…)?)`。
  - `Err(…)` 是异常路径，不计入；发散宏（`panic!` 等）作尾表达式不产生值。
  - 其余形态（局部变量、方法调用、本文件自由 fn、其他宏、宏内含 `return`、无 else 的 `if`）记为未知。
- **合并**：同名 fn（`merge_fn`）与成员命中的多个 fn（`MemberHw::absorb`）取并，任一未知即未知。返回来源只取本 fn 自己的返回点，
  不沿 `close_transitive` / 手写对象 `absorb_body` 从被调 fn 传递。
- **引擎**（`engine/hw_ret.rs`、`process_handwritten`）：路径类型经 `resolve_tref` 解析，Rust 名按 `methods_by_rust_name` 取同实参数的方法。
  全部命中都是静态方法、可解析，且被调方按返回值节点给出结果（`RetModel::Plain`）时，接 R(被调) → R(本方法)，返回类型不再导出值池；
  否则（含命中 fn 为空）退回 open(返回类型)，即原行为。

#### 健全性

- null 不贡献对象。尾调用交出的就是被调方本次调用的返回值，被调方的 R 节点汇合其全部返回值，是它的上近似。
- 按调用点建模返回值的被调方（类镜像 / 浅拷贝 / 内存读取 / 新数组 / 调用者类），其 R 不一定承载结果，因此一律退回 open。
- 判定只在「全部返回点都认得」时生效，认不出的形态一律退回原行为。
- 返回类型不再导出值池：导出的作用是让 open 返回值能从值池取到对象。返回值改由被调方 R 给出后，值池里的对象不会经返回值交出。

#### 实测（`--stop-after` 级 `rava closure`，冷缓存，B2 = 0c8eb802 → B3）

| 例 | 类（基线 / B2 / B3） | 方法（基线 / B2 / B3） | B3 ⊆ B2 | B3 ⊆ 基线 |
|---|---|---|---|---|
| HelloWorld | 469 / 469 / 469 | 1828 / 1827 / 1827 | 是 | 是 |
| StockTrans | 3193 / 3185 / 3185 | 20429 / 20276 / 20261 | 是 | 是 |
| DeepCopy | 3195 / 3187 / 3187 | 20447 / 20294 / 20279 | 是 | 是 |
| TestSerialDefaultSuid | 3200 / 3192 / 3192 | 20441 / 20288 / 20273 | 是 | 是 |

- `@opens:java/net/URL` 的引入点从 `R ClassLoader.getResource` + `nextProviderClass@173`（4 个上下文）+ `URL.equals@1`
  收窄到只剩 `URL.equals@1`。
- 三个大例各少 15 个方法，内容一致：`URLConnection.getLastModified` / `getHeaderField*`、`FileURLConnection.getLastModified`、
  `MessageHeader.findValue`、`ZipEntry.getTime` 与 `ZipUtils.*DosToJavaTime`、`LocalDateTime.of` / `LocalTime.of` / `ZoneRules.getOffset`、
  `Class.setSigners`、`StringTokenizer.hasMoreElements`、`ConcurrentHashMap$KeyIterator.nextElement`。类数不变。

#### 验证

- 本机：`closure` crate 全部单测 171 通过（新增 `handwritten::returns` 3 例）；`rava` release 构建通过；
  `closure_cli` 的 `closure_independent_of_hash_seed` 与 `container_elements_per_object` 均通过。
- C4 全量冻结期内未发起服务器作业。冻结解除后跑服务器单测，并与 §30.6 一起按 §30.3 抽查。

#### 遗留

- jar `Handler.openConnection` 与 `JarVerifier` 仍在闭包内，引入点只剩 `URL.equals@1`（`Object` 形参 `instanceof URL`）与
  URL 对象 handler 字段的分派，这部分归 B4 / B1 / B5（§30.2）。§30.1 的 −277 上界仍待 B4 → B1 → B5 兑现。
- 返回来源只认尾调用形态。手写体先绑定局部变量再返回（`let x = T::m()?; Ok(x)`）的仍按 open 处理。需要时再扩展到
  局部 `let` 的单赋值传递，判定口径不变。

### 30.8 第 3 步：B4——按对象 URL 落地，`startsWith` 守卫须与 B1② 合并实施（2026-10-06，分支 `c1d-url-b2`，提交 f6752b81）

**结论先行**：
- **B4① 已落地**（f6752b81）：采用 §30.4 判据。单独提交时四例集合不变，它是 B4② 的前提。
- **`URL.equals@1` 复核**：这是通用的 `instanceof` 收窄，不是 URL 特有的漏洞。反事实去掉它，类数和方法数都是 0 变化，所以不处理。
- **`FileLoader.getResource` 的 `startsWith` 守卫在 B4 范围内不能健全折叠。** 下面给出 JDK 能实际走到的反例。要折叠，必须知道
  `normalizedBase.getFile()` 以 `/` 开头。这是基 URL 的按对象串事实，正是 B1② 的能力。终态目标不降：B4②（按键分对象 + 处理器
  `file` 前缀事实 + `startsWith` 收窄）**并入 B1，作为一步实施**，§30.5 的顺序改为 B1+B4② → B5。
- 在 L1 切除下，该守卫的反事实收益已经实测（见下文「守卫」一节）：jar / zip / jrt 连接链与签名校验链全部依赖它。

#### B4①：按对象 URL

`engine/classes.rs::container_shape` 新增一条判据：类链上有实例字段的类型是 `[facts.keyed_lookups]` 键类（按键查找入口的返回类型）时，
对象按分配点区分。判据只读清单键类，不写类名。字段类型没有登记序号的，直接判为非键类，不为判定而新登记类名。
`engine/keyed.rs::key_classes` 改为 `pub(super)`。

健全性：按对象区分只是精化。各对象的字段值集之并，等于原来的无上下文字段值集。

| 例 | 类（基线 / B3 / B4①） | 方法（基线 / B3 / B4①） | B4① = B3 | B4① ⊆ 基线 |
|---|---|---|---|---|
| HelloWorld | 469 / 469 / 469 | 1828 / 1827 / 1827 | 是 | 是 |
| StockTrans | 3193 / 3185 / 3185 | 20429 / 20261 / 20261 | 是 | 是 |
| DeepCopy | 3195 / 3187 / 3187 | 20447 / 20279 / 20279 | 是 | 是 |
| TestSerialDefaultSuid | 3200 / 3192 / 3192 | 20441 / 20273 / 20273 | 是 | 是 |

- 分析耗时在噪声范围内：DeepCopy 22.5 → 23.7 s，TSDS 23.0 → 25.3 s。测量期间机器上有其他作业，没有单独复测。
- 叠加 L1 切除（`R@97` / `R@127` / `R@139`）时，DeepCopy 从 3182 / 20159 变为 3182 / 20157，只少 2 个方法。原因是
  `FileLoader.getResource` 的 URL 对象（`URL@<getResource>:0`）在构造器克隆的 `@386` 闸门上，spec 键未知（`encodePath(name)` 的返回值），
  `handler` 字段仍含 jar `Handler`。

#### `URL.equals@1` 复核

- `--flows '@openorig:java/net/URL|@1 java/net/URL.equals'`（StockTrans）共找到 20 个注入点，都是 open(Object) 经
  `Object.equals` 枢纽实参（`hub 实参0 … on open(Object)`）流到 `URL.equals` 的 P1，再在 `instanceof URL` 处被收窄成 open(URL)。
  open(Object) 的来源是：反射调用实参池、`Objects.equals` / `Arrays.equals` / `ConcurrentHashMap.put` 的实参、反序列化的
  `cloneArray`、注解 `memberValueEquals`。
- 收窄本身是健全的：open(Object) 里确实可能有未经分配点追踪的 URL，例如反序列化出来的 URL。
- 反事实（临时开关，没有提交）：只让 `URL.equals@1` 不产出收窄值，DeepCopy 3187 / 20279 不变。原因是 open(URL) 已经由反序列化路线
  （`URL.readObject` / `readResolve` 等接收者）带入，`URL` 是 final 类，open(URL) 上的派发不会多出目标。
- 结论：不改，也不为 URL 做特判。它属于 open(Object) 来源的通用精度问题，归入 open 来源收窄线。

#### `startsWith` 守卫：为什么 B4 单独做不了

字节码（`URLClassPath$FileLoader.getResource(String,Z)`）：

```
url = new URL(getBaseURL(), ParseUtil.encodePath(name, false));   // @0..@13，两参解析式构造
if (!url.getFile().startsWith(normalizedBase.getFile())) return null;   // @20..@38
file = new File(dir, name.replace('/', File.separatorChar)); if (!file.exists()) return null; ...
```

`normalizedBase = new URL(getBaseURL(), ".")` 在 `FileLoader.<init>` 中赋值。

- **jar URL 的 `file` 形态**：jar `Handler.parseAbsoluteSpec` 要求内层 `spec.substring(0, idx-1)` 能被 `new URL(...)` 解析，
  所以解析式构造出的 jar URL，其 `file` 一定以「内层协议名 + `:`」开头（协议名以字母开头），不可能以 `/` 开头。三参构造
  `new URL("jar", "", "/x")` 的 `file` 可以是 `/x`，所以这条事实只对**解析式构造**成立，不能挂在 `URL.file` 字段上全局成立。
- **反例：`normalizedBase.file` 不一定以 `/` 开头。**
  1. 取基 URL `file:jrt:/`。它的 protocol 是 `file`，`file` 以 `/` 结尾，`getLoader` 会为它造 `FileLoader`。
  2. 得到 `normalizedBase.file = "jrt:/"`。
  3. 取资源名 `jar:jrt:/x!/y`。`encodePath` 不编码 `:`，所以 spec 的键是 `jar`，`url.file = "jrt:/x!/y"`。
  4. 这个 `url.file` 以 `"jrt:/"` 开头，守卫通过。
  5. 之后的 `file.exists()` 是文件系统事实，不能折叠。

  `URLClassLoader(new URL[]{new URL("file:jrt:/")})` 这类用户 URL 就能走到这里，所以 J 确实可达。只看本对象的 handler 与 spec，无法排除这一点。
- **折叠需要的三条事实**（缺任何一条都退回现状）：
  1. **按键分对象**：解析式构造的分配点按 spec 的协议键拆成「每键一对象 + 无协议」。拆分后，构造器克隆里的 `@386` 闸门只放行
     本键的 handler，`handler`、`protocol` 与 `file` 前缀在对象内保持对应。清单为每个键处理器声明「解析式构造结果的 `file` 前缀」，
     例如 jar 为 `<字母>…:`。
  2. **基 URL 的 `file` 以 `/` 开头**：这一条可以从字节码推出。应用类路径的 URL 来自 `ParseUtil.fileToEncodedURL`：
     `if (!path.startsWith("/")) path = "/" + path; … new URL("file", "", path)`。在串前缀域里按 `startsWith` 分支收窄后，两支都以 `/`
     开头，三参构造器把它写进 `file` 字段。还要求：
     - `FileLoader` 的基 URL 只来自这类对象。`URLClassPath(URL[])` 来自用户 URL，必须在对象层面与类路径实例分开，要求
       `URLClassPath` / `Loader` 按对象，`unopenedUrls` 已由 B2 按对象处理。
     - 清单声明 `new URL(base, ".")` 的相对解析保持开头的 `/`。`URLStreamHandler.parseURL` 中的 `/./`、`/../` 循环无法由纯字节码推出前缀，
       这里只声明「上下文 `file` 以 `/` 开头、spec 无协议且无 `//`」时结果仍以 `/` 开头。
  3. **`startsWith` 收窄**：接收者前缀与实参前缀不相容时（`<字母>…:` 与 `/…` 不相容），判定恒为假，jar 键对象到不了 `@39` 之后。

  第 2 条是 B1② 的「URL 对象 `protocol` / `file` 按对象串事实」。B1③ 的 `endsWith("/")` / `equals` 收窄与第 3 条的 `startsWith` 收窄
  属于同一个串前缀判定器。所以 B4② 与 B1 合并实施：同一个按对象串域，同时服务 `R@97/127/139` 的分支折叠和本守卫。
- **守卫的反事实价值**（DeepCopy，L1 切除下）：
  - 切掉该对象的 `openConnection` 节点：3182 / 20157 → 3104 / 18576。
  - 切掉该对象 `@386` 闸门节点（只剩 file handler）：去掉 218 类，包括 jar / zip / jrt 连接链、JarVerifier、PKCS7 / 签名、
    `DisabledAlgorithmConstraints` 等。
  - 切写入点不单调（§7），这两个数只作量级参考。不加 L1 切除时为 0：J 还经 JarLoader 路线可达（B1）。

#### spec 未知的解析式构造点全表（DeepCopy，`@srcs:sun/net/www/protocol/jar/Handler` 的 `@386` 克隆）

| 构造点 | spec 来源 | 归属 |
|---|---|---|
| `URLClassPath$FileLoader.getResource@0` | `encodePath(name)` | 本节守卫，并入 B1 |
| `URLClassPath$3.run@78` | `file.substring(0, len-2)`（`!/` 结尾的 jar 内 jar） | B1（`isDefaultJarHandler` + `endsWith("!/")` 分支） |
| `URL.fabricateNewURL@10`、`URL.readObject@18`、`URL.readResolve@9` | 反序列化流 | 真实未知（流内容由运行期决定），保留 |
| `NativePRNG.getEgdUrl@42` | `securerandom.source` 属性 | 构造后有 `getProtocol().equalsIgnoreCase("file")` 守卫，按键分对象后可由协议串事实折叠，同 B1 串域 |
| `URI.toURL` → `URL.of@238` | URI 串 | 真实未知，保留 |
| `JarURLConnection.parseSpecs@45`，jar `Handler.newURL`（经 `parseAbsoluteSpec` / `sameFile` / `hashCode`） | jar URL 内部 | 只在 J 已可达时出现，是自循环，不构成独立来源 |

「spec 未知时的 handler 事实」的终态答案：键未知时，handler 集合**只能**是全集，不存在更强的健全事实。能做的是第 1 条的按键分对象：
让「若 handler 是 jar，则 `file` 有 jar 形态」这一相关性留在对象内，交给下游守卫折叠。它单独不缩闭包，并入 B1 实施。

#### 验证

- 本机：`closure` crate 单测 171 通过；`rava` release 构建通过；`closure_cli` 的 `closure_independent_of_hash_seed` 与
  `container_elements_per_object` 通过。
- C4 全量冻结期内没有发起服务器作业。

#### 下一步（建议）

B1 + B4② 合并为一步，交付以下内容：
- 按对象串前缀域：字段按接收者对象记前缀，构造器克隆内 `putfield this` 的串值进入对象字段；
- `startsWith` / `endsWith` / `equals` 分支收窄；
- 解析式构造按键分对象，以及清单中的处理器 `file` 前缀与相对解析保前缀事实；
- 类路径目录事实（B1①）。

验收沿用 §30.3：DeepCopy ≤3097（去掉 B5 一项之后的上界再实测），J 出闭包。

### 30.9 第 4 步：B1 串形状域落地 + B1② 的引擎前提实测（2026-10-06，分支 `c1d-url-b2`）

**结论先行**：
- **第 1 小步已落地**：方法内串形状域（前缀 / 后缀 / 不含字符）、清单构建器内容跟踪、`startsWith` / `endsWith` 分支收窄、
  `encodePath` 形状事实。四例集合与 B4① 完全相同（0 增 0 减），折叠逐方法对照只多出健全的死分支。它是 B1③ 与 §30.8 第 3 条的判定器。
- **B1② 不是「再加一层字段记录」能完成的**。逐条核对 `R` 与 `FileLoader.getResource` 的值来源链后，确认还缺 **5 项彼此独立的引擎能力**，
  见下文「B1② 的前提链」。缺任何一项，`R@97/@127/@139` 与 `startsWith` 守卫都折不掉，闭包不变。这 5 项合起来就是
  「按对象的常量传播」（对象敏感的值分析），是闭包引擎的一项大能力，不是 URL 的局部补丁。本步收口时只实现第 1 小步，其余记为设计与恢复入口。
- 「去掉 B5 一项」的反事实上界见下文实测。

#### 第 1 小步：串形状域（提交见本节末）

设计（终态通用，分析器内无类名，清单只新增串操作与一条结果形状事实）：

- **形状格**（`absint/shape.rs`）：`Shape { pre, suf, no: u128, exact }`，是前缀、后缀、ASCII 字符不出现三个经典串抽象域之积。
  - 合流取公共前缀、公共后缀，以及不出现字符的交集。
  - 前后缀上限 `CAP = 256` 字符，超出截短（变弱，仍健全）。格高有限，不动点必收敛。
  - 判定：`starts_with` / `ends_with` / `equals` / `is_empty` / `contains`；`index_of` 只在所找字符确定不出现时折叠为 -1。
  - 收窄：`meet_starts` / `meet_ends`。
- **值上的标签**（`absint/obj.rs`）：
  - `Obj::Str(Shape)` 经 PV 跨方法传递（形参、返回、字段值集），`PV::join` 按形状合流。
  - `Obj::Builder { group, content }` 只在方法内存在，PV 从不保存构建器标签（`of_ret` 把非空构建器引用映射为 `nonnull_ref`）。
- **构建器内容跟踪**（`absint/strs.rs`）：清单 `[facts.string_concat]` 的构造 / 追加 / 取结果三类，按分配点分组。
  - 健全性靠一条不变式：同一状态里同组的各份带标签拷贝指向同一对象，且该对象不可能经无标签的引用被改写。
  - 维持手段：
    - 再次执行分配点，或值交出去（作其它调用的实参、写字段 / 静态字段 / 数组、indy 实参）：撤掉整组标签。
    - 合流时任一入边带某组标签、而结果不带，整组撤掉（`drop_lost_groups`）。
    - 异常处理器入口撤掉全部构建器标签。
  - 追加段按 `String.valueOf` 语义取形状：整数为 `-0123456789` 字符集，布尔为 `truefals` 字符集，引用可能为 null 时并入 `"null"`。
- **分支收窄**：`aload k; ldc L; invokevirtual <starts_with|ends_with>; ifeq/ifne`，后三条须在同一基本块。
  - 成立一侧：局部变量 k 的形状与 L 相交。
  - 两侧：k 都确定非空（接收者已被解引用）。
- **清单**（`vm_intrinsics.toml`）：
  - `[facts.string_ops]` 新增 `starts_with` / `ends_with` / `index_of` ×4 / `last_index_of` ×4 / `contains`。
  - 新表 `[facts.string_shapes]`，只允许 `excludes` 一个键。登记 `ParseUtil.encodePath` 两个重载 `excludes = "#?"`。依据是
    `L_ENCODED` 的第 35 / 63 位，已用 javap 与位运算核对，论证抄在清单注释里。
  - 用途：`fileToEncodedURL` → `new URL("file", "", path)` 里的 `indexOf('#')` / `lastIndexOf('?')` 折叠为 -1。

健全性与反例检查：
- 前后缀判定按 Rust `str` 计算。形状里的串都来自类文件的合法 Unicode 常量，合法串之间按码点与按 UTF-16 码元的前后缀关系一致；
  含孤立代理项的常量不成形状。
- `startsWith` 反例检查：`pre = "/"` 判 `startsWith("/a")` 为「推不出」，判 `startsWith("/x")` 为假，因为 `x` 在两侧都不出现。
  单测 `join_keeps_common_affixes_and_absent_chars` 覆盖这两种情况。
- 超长串（> CAP）不再 `exact`，`equals(全串)` 推不出（单测 `non_ascii_affixes_split_on_char_boundaries`）。
- 构建器别名反例：带标签拷贝在一侧被追加、另一侧没有，合流后整组撤掉（单测 `join_losing_tag_drops_group`）。
  语句式追加（`sb.append(x);` 不用返回值）同步更新同组各拷贝（单测 `statement_appends_update_all_copies`）。
- `encodePath` 事实只声明结果不含 `#` `?`，不声明前缀。`fileToEncodedURL` 里 `startsWith("/")` 两支都以 `/` 开头，
  这是由分支收窄加拼接从字节码推出的（单测 `prefix_after_guard_and_concat` 用同形字节码验证返回值以 `/` 开头）。

实测（本机 `rava closure`，refjdk 21.0.11+10，冷缓存）：

| 例 | 类（B4① / 本步） | 方法（B4① / 本步） | 集合 |
|---|---|---|---|
| HelloWorld | 469 / 469 | 1827 / 1827 | 相同 |
| StockTrans | 3185 / 3185 | 20261 / 20261 | 相同 |
| DeepCopy | 3187 / 3187 | 20279 / 20279 | 相同 |
| TestSerialDefaultSuid | 3192 / 3192 | 20273 / 20273 | 相同 |

- 耗时在噪声范围内（约 24 s）。
- 逐方法折叠对照（`build/url/folds.py`）：多出的只有新的死 null 检查分支，以及 `SocketPermission.getHost` 等处的 `indexOf` 折叠，逐条核对均健全。

验证：closure crate 单测 179 通过（新增 shape 4 / strs 3 / manifest 1）；`closure_cli` 的 `closure_independent_of_hash_seed` 与
`container_elements_per_object` 通过；`cargo check` 无告警。

#### B1② 的前提链（为什么形状域单独不缩闭包）

`R` 的三个分支取决于 `val$url.getProtocol()` 与 `val$url.getFile()`。逐跳核对值来源：

| 跳 | 现状 | 缺的能力 |
|---|---|---|
| ① `URL.<init>(String,String,int,String,Handler)` 写 `this.protocol` / `this.file` | 字段值集 `fvals` 按字段键全局汇合：全部 URL 对象的 `file` 并在一起，含解析式构造、反序列化、用户 URL | **P1 按对象字段值**：`putfield` 按接收者值集里的抽象对象记入 `ovals[(对象, 字段)]`；非对象接收者（open / 非抽象对象 / 手写 / 物化快照）记入 `owild[字段]`；读取在接收者值集全为抽象对象时取「各对象值 ⊔ owild ⊔ 初值」，否则退回全局。写站点已有按接收者增量重跑（`bytecode.rs::field` 的 `recv_delta`），值归属可以挂在同一处 |
| ② `file` / `protocol` 的初值 null | `alloc_defaults` 把初值并入全局值集。按对象值同样要含初值，于是 `R@17 ifnull` 的 null 支永远活着，`"file".equals(protocol)` 也折不掉 | **P2 构造器确定初始化**：分配点之后接的 `<init>` 在把 `this` 交出去之前，在每条路径上都写了字段 f，才去掉该对象 f 的初值。必须是路径敏感的必然分析：放在 absint 状态里，和 `finals` 同构，合流取交集；`this(…)` 委托递归；超类构造器须不交出 `this`。经 lambda 构造器引用、手写分配、序列化分配的对象一律保留初值 |
| ③ `this.file = this.path`（`URL.<init>@303..308`） | 构造器内读自身刚写的字段，取的是全局值集 | P2 的同一状态顺带给出「本方法刚写的值」（同 `finals` 对 static final 的处理） |
| ④ `getFile()` / `getProtocol()` 的返回 | 返回常量格 `rvals` 按成员键汇合（`facts.rs` 的 `rvals: HashMap<MemberRef, PV>`），各接收者对象的克隆节点结果并在一起 | **P3 按接收者对象的返回值**：`orvals[(成员, 对象)]` 在节点记录返回时按 `P(m,0)` 中的抽象对象归属，`P(m,0)` 增长时补归属；调用点接收者值集全为抽象对象时取各对象之并。健全性：调用点派发到的每个目标节点 `P(·,0)` 都含该对象，所以按对象之并覆盖本调用点的全部可能返回 |
| ⑤ `URLClassPath$3.val$url` | `$3` 不是容器形态类（非泛型，字段类型 URL 不是键类），`val$url` 按全局读：所有 `URLClassPath` 对象的全部 URL 汇合（含用户 `URLClassLoader` 的 URL） | **P4 读站点接收者值集**：调用点 / 字段读的接收者来源是本方法站点（`Src::Site`）而非形参时，按站点节点的值集判定。Oracle 目前看不到流图，须给 `Facts` 只读的值集查询，并以形参节点 / 站点节点增长为依赖重分析（同 `mirror_watch`）。另需判定 `$3` 这类「持有按对象类实例的匿名类」是否按对象区分（`container_shape` 新判据，影响面要单独实测） |
| ⑥ 反序列化放开 | DeepCopy / TSDS 里反序列化可达，`URL.file` / `protocol` 可序列化、非 transient，`field_open`（`deser`）对全局读一律不折叠 | **P5 按对象的放开判定**：反序列化只写它自己分配的对象（序列化分配是类 id，不是抽象对象），所以按对象读只看偏移可得（`fopen` / `fopen_names` / `fopen_all`）与手写写入，不看 `deser`（同 `construct.rs::object_field` 的现有论证） |
| ⑦ `path` 以 `/` 结尾 | `fileToEncodedURL@48` 的 `isDirectory()` 推不出 | 「启动目录是目录」清单事实（B1①，见下「待用户决策」） |

P1–P5 齐备、再加 ⑦，`R` 的三个分支才能折叠。`FileLoader.getResource` 的守卫另需 §30.8 的按键分对象与 `"."` 相对解析保前缀事实，
这两条同样建立在 P1–P5 上：`normalizedBase.getFile()` 也要经 P1 + P3。

实施顺序建议（每项单独提交，单独看不到闭包变化，验收看折叠对照与单测）：
1. P1 + P5（按对象字段值 + 按对象放开）；
2. P2（构造器确定初始化，含本方法刚写值）；
3. P3 + P4（按对象返回值 + 站点接收者值集）；
4. ⑦ 目录事实，以及 §30.8 的按键分对象与 `"."` 事实。

#### 第 2 小步：P1 + P5 按对象字段值（`engine/obj_fields.rs`）

**设计**（通用，无类名）：
- **写入**：`bytecode.rs::field` 的 `putfield` 把值并入接收者值集里每个抽象对象的 `ovals[(对象, 字段)]`；其余接收者（open / 非抽象对象 / 无接收者）的值并入 `owild[字段]`。
  - 基本类型字段不拆接收者，写入只进 `owild`；物化快照的写入（`concrete/apply.rs`）也只进 `owild`。
  - 两张表都从初值起算。
- **读取**：`Facts::field` 处理 `getfield` 时，若接收者只来自形参 i（`srcs == [Param(i)]`），且形参值集非空、全由抽象对象组成，就取「初值 ⊔ owild ⊔ 各对象值」。
  - 形参对象集在分析开始时取（`param_obj_sets`），只看声明类型是容器形态类的形参。
  - P5：按对象读不看 `deser`，偏移可得、手写写入、VM 钩子一律不折叠。
- **依赖与共享**：每次按对象读记成 `(形参, 字段, 答复)`；形参无对象集时答复为 None，同样记录。抽象解释是 Oracle 答复的确定函数，所以三处失效都按「答复是否变化」判定：
  - 按对象值变化：读者按 `(对象, 字段)` 登记（`odeps`），`owild` 的读者按字段登记（`owdeps`）。用当前形参对象集复核，答复有变才重分析。
  - 形参值集增长（`obj_watch`，同 `mirror_watch`）：复核答复；不变就只补登新对象的读者。值集由空变成全抽象对象时也会复核，所以不动点与分析顺序无关。
  - 共享摘要：每条记录在新上下文的形参对象集下答复都相同，才复用（`share.rs::shared_analysis`）。开放判定只增不减，仍经全局 `fdeps` 失效。

**健全性**：
- 字节码写入要么记到接收者值集里的每个抽象对象，要么记入 `owild`，读取并入两者。
- open 接收者的写入在流图里流向所有已逃逸对象，这里由 `owild` 覆盖。
- 字节码外写入（手写 / 偏移可得 / VM 钩子）不按对象折叠。
- 反序列化分配是类 id，不是抽象对象（`serial_alloc.rs`），不会落进全由抽象对象组成的值集。
- 手写体分配的容器对象（`hw_obj`）的 Rust 字段写入登记在 `fhw`，按字段不折叠。
- 反例核对：
  - `Properties.enumerate@1` 的 `defaults` 折成 null。Properties 可序列化，但反序列化出来的 Properties 是类 id 接收者，读取时值集不全为抽象对象，退回全局。
  - `BigInteger$RecursiveOp.parallel` 折成 false。可达的分配只有 `multiply` 一侧（`parallel = false`），`parallelMultiply` 不可达。若用户调用了它，写入会按对象记录，折叠自动解除。

**失效判定的迭代**：三版 DeepCopy 实测（基线 23.7s）：
- 直接按字段失效全部读者：203s。`field_put` 重分析 287 万次，其中 286 万次结果不变；共享因形参对象集不同而大量失配。
- 改为按对象登记读者，并按查询答复共享：28.8s。
- 增长与值变化都先复核答复：26.0s。

**实测**（tag `10p`，对照 B4① `4`）：

| 测试 | 类 | 方法 | 折叠常量 / 折叠方法 | 用时（ms，本机负载 6.5，墙钟仅供参考） |
|---|---|---|---|---|
| HelloWorld | 469 → 469 | 1827 → 1827 | 337→340 / 342→345 | 482 → 495 |
| StockTrans | 3185 → 3185 | 20261 → 20259 | 2220→2238 / 1973→1992 | 24841 → 34777 |
| DeepCopy | 3187 → 3187 | 20279 → 20277 | 2218→2236 / 1972→1991 | 23699 → 25967 |
| TestSerialDefaultSuid | 3192 → 3192 | 20273 → 20271 | 2219→2237 / 1972→1991 | 25298 → 31033 |

- 三例各少同样 2 个方法：`ReferencePipeline$Head.opIsStateful`、`BigInteger$RecursiveOp.getParallelForkDepthThreshold`。集合是 B4① 的真子集，没有新增。
- StockTrans / TSDS 的墙钟增幅里，不受本步影响的 setup / seeds 阶段同样慢了约 1.5 倍，属本机负载噪声；分析相关阶段约增 10%–30%。
- 单测：closure 179 过（去掉的过渡测试 `objs_compatible` 不再计入）；`closure_independent_of_hash_seed`、`container_elements_per_object` 过（1494s）。
- 正如前提链所述，本步单独不折 `R`：`getFile()` 的返回仍按成员键汇合（P3），`val$url` 的接收者来自站点而不是形参（P4），初值 null 仍在（P2）。

#### 「去掉 B5 一项」的反事实上界

切除集 `build/url/cutsNoB5.txt` 是 §30.1 第 5 层去掉 `BootLoader$PackageHelper.definePackage@67`；`cutsAll.txt` 是完整第 5 层。均以本步代码实测：

| DeepCopy 切除集 | 类 | 方法 | 与当前（3187 / 20279）之差 |
|---|---|---|---|
| 无切除（本步） | 3187 | 20279 | — |
| `cutsNoB5`（去掉 B5 一项） | 3099 | 18549 | −88 / −1730 |
| `cutsAll`（完整第 5 层） | 3088 | 18445 | −99 / −1834 |

结论：
- 只做 B1（不做 B5）的反事实上界是 3099 类，比目标 ≤3097 多 2 类；完整第 5 层（含 B5）才能到 3088。所以 ≤3097 必须 B1 与 B5 都做。
- **jar Handler 在两种切除下都还在闭包里**。`URLClassPath.<init>(String,Z)@185` 无条件执行 `new sun/net/www/protocol/jar/Handler` 并写入 `jarHandler`，这个类是必然实例化的，任何健全的折叠都去不掉它。可去掉的只有经 `URLStreamHandler` 虚分派进来的方法：`parseURL` / `parseAbsoluteSpec` / `parseContextSpec` / `canonicalizeString` / `hashCode` / `newURL` 等。前提是 §30.8 第 1 项：`URL.handler` 按分析出来的协议键分对象，jar Handler 只流进 `jarHandler` 字段和由它构造的 URL。因此 §30.8 原目标「jar Handler 出闭包」应改为「jar Handler 只保留构造器，经分派进来的方法出闭包」。（2026-10-06 协调方已采纳，§30.3 验收措辞已改写。）

#### 待用户决策

1. **启动目录是目录**（B1①）：原生二进制的应用类路径固定为 `""`，经 `new File("").getCanonicalFile()` 解析为 `user.dir`。
   声明它「是目录」有一个可达反例：进程启动后工作目录被删除，此时 `isDirectory()` 为假，`R@139` 走 JarLoader，命中存根即 panic，
   而 JVM 上是抛 IOException 后跳过该类路径项。是否接受这条启动期事实（或改为构建期把类路径固定为非空的资源目录），需要决定。
2. **B1② 的规模**：P1–P5 是对象敏感的值分析，影响全部字段读 / 返回值，分析耗时与内存需要实测，不是 URL 的局部改动。
   是否按上面的顺序投入，还是先做 B5（独立、清单事实即可）与能力②③，需要决定。
3. `encodePath` 的结果形状用清单事实声明，没有从字节码推出：推出需要数组 / 位运算 / 循环不变式，超出串形状域。
   清单注释附有可核对的位运算论证。
4. 构建器同组、内容不同的合流目前直接撤掉标签（健全但偏粗）；需要时可改为内容形状合流。

#### 恢复入口

- worktree `/Users/yuwei/dev/workspace/java_rta_c1durl2`，分支 `c1d-url-b2`。
- 测量：`build/url/cl.sh <Test> <tag> [参数]`，对照用 `build/url/cmp.py <tag 后缀> [基线后缀]` 与 `build/url/folds.py A.json B.json`。
- P1 + P5 已落地（`engine/obj_fields.rs`，见「第 2 小步」）。下一步是 P2 构造器确定初始化：
  - 在 absint 状态里加 `this` 字段的必然写入集（与 `finals` 同构，合流取交集）；
  - 在 `<init>` 返回点、且 `this` 未被交出时，产出「必然写入字段」摘要；
  - 引擎在分配点后接的构造器全部给出摘要时，`ovals` 不再并入该对象该字段的初值。读取侧的初值并入点是 `obj_fields.rs::obj_field_value`。
- 再往后是 P3 + P4：
  - 按接收者对象的返回值：`rvals` 旁加 `orvals`，归属按节点 `P(m,0)`；
  - Oracle 读站点值集：调用 / 字段读的接收者来自 `Src::Site` 时，查流图站点节点，登记复核方式同 `obj_watch`。
  - 两者都可复用本步的「查询记录 + 答复复核」框架：`ObjQuery`、`obj_queries_same`、`obj_readers_recheck`。

### 30.10 第 5 步：B5 引导包位置 `jrt:/` 前缀（2026-10-06，分支 `c1d-url-b2`，基于 da165172）

协调方决定（2026-10-06）：先做 B5，B1② 的 P2–P4 后做；构建器内容在合流处丢弃可以接受；jar `Handler` 验收口径改写（§30.3 已改）；
「启动目录是目录」待用户决定，本步不实施依赖它的折叠；`encodePath` 的结果形状要改为从字节码推出（见本节末）。

#### 设计

1. **清单事实**（`vm_intrinsics.toml [facts.string_shapes]`）：`BootLoader.getSystemPackageLocation` 的非 null 结果以 `jrt:/` 开头。
   - 依据：JVM 返回命名模块包的模块位置 `jrt:/<模块>`、`-Xbootclasspath/a` 追加项上包的类路径项，或 null。原生二进制没有
     `-Xbootclasspath/a`（无启动参数注入，引导类全部来自构建期模块镜像），所以只剩 null 与 `jrt:/<模块>`。
   - 归类：native 方法（手写类 ①）的落地语义，属 handwritten-boundary §③「VM 注入状态」。清单注释写明手写实现须守此约定。
   - `[facts.string_shapes]` 的表项由 `excludes` 单项扩为 `{ prefix, excludes }` 两项（至少一项；前缀不得含被排除字符，解析期报错）。
     `Shape::prefixed` 取代 `Shape::excluding`。
2. **返回常量格合流 `PV::join_ret`**：两侧都确定非空、值 / 标签无法合流时取「非空引用」，不再落到 Top。
   触发点：`StringLatin1.newString` 一条路径返回常量 `""`、另一条返回带对象标签的 `new String`，合流后 `String.substring` 的返回成了 Top，
   `findModule` 里 `substring` 结果之后的 `ModuleLayer.findModule(mn)` 链因此失去非空性。只用于返回常量格，形参 / 字段常量格不变。
3. **final 实例字段复读收窄**（`absint/narrow.rs`，`State::nnf`）：`aload k; getfield f; ifnull/ifnonnull`（同块，f 为 final 实例字段）
   非 null 一侧记下 (k, f)；此后 `aload k; getfield f` 的结果确定非空。写 k、本帧 `putfield f`、任一调用处撤掉，合流取交集，
   异常处理器入口为空。触发点：`Optional.orElseThrow`（`if (value == null) throw …; return value;`）。
   - 健全性：本线程内，final 字段的字节码写入只在 `<init>`，字节码外写入（反射 / Unsafe / 反序列化 / 手写）都要经调用，已被撤掉规则覆盖；
     他线程的写入与第二次读之间没有 happens-before 边（本线程两次读之间无调用），第二次读取到第一次的值是 JLS §17.4.5 允许的结果，
     final 字段另有 JLS §17.5.3 明文允许缓存（即使构造后被反射改写）。HotSpot C2 对无中间副作用的同字段读取也做公共子表达式合并。
     所以收窄**不看字段的开放判定**（`Oracle::final_field` 只看访问标志，不登记依赖）。
   - 为什么不看开放判定：`Optional.value` 在三处被「按名放开」——`ObjectStreamField("value", char[].class)`（`StringBuffer.<clinit>`）、
     `ObjectInputStream$FieldValues.get` / `ObjectOutputStream$PutFieldImpl.put` 的 `getFieldOffset(name, type)`。它们都走
     `reflect_writes.rs` 的形状兜底（「Class + String 实参 → 按名放开」，Class 实参上找不到该名字段时全局按名放开），
     而实际按名取字段的汇点（`getDeclaredField` / `objectFieldOffset` / `name_resolvers`）另有精确建模。若收窄依赖开放判定，
     这条兜底会让收窄失效；收窄的健全性本就不依赖写入集合（见上），所以与开放判定解耦。
4. **字段配对的基本类型 / 非字节码类镜像**（`lookup_pair.rs::field_wrap_call`）：类值集里的基本类型类镜像（`Class#<primitive>`）与
   非字节码类镜像没有 Java 字段，按名取不到字段，不再算「所指未知」而全局按名放开（与 `field_lookup.rs::class_values` 同一口径）。
   来由：`findVarHandle(Class recv, String name, Class type)` 内部 `MemberName.<init>(Class, String, Class, byte)` 的形状配对把 `type`
   形参也登记成查找类，`AtomicBoolean.<clinit>` 等传入 `int.class`，曾把 `value` 全局放开。

#### 实测（tag `11d`，对照 da165172 的 `10p`）

| 测试 | 类 | 方法 | 折叠常量 / 折叠方法 | 用时 ms（本机有并行作业，仅供参考） |
|---|---|---|---|---|
| HelloWorld | 469 → 469 | 1827 → 1827 | 340→340 / 345→345 | 495 → 523 |
| StockTrans | 3185 → **3150** | 20259 → **19919** | 2238→2212 / 1992→1962 | 34777 → 31700 |
| DeepCopy | 3187 → **3152** | 20277 → **19939** | 2236→2210 / 1991→1961 | 25967 → 29822 |
| TestSerialDefaultSuid | 3192 → **3157** | 20271 → **19931** | 2237→2211 / 1991→1961 | 31033 → 27555 |

- 四例均为 `10p` 的子集（新增类 0、新增方法 0）；三例各 −35 类 / 约 −340 方法。折叠常量 / 方法计数下降是因为被折叠的方法本身出了闭包。
- DeepCopy 折叠：`PackageHelper.definePackage` 死区 `[57, 87]`（`toFileURL` / `getManifest` / `defineOrCheckPackage` 分支）；
  `findModule` 死区 `[24, 79]`（`-Xbootclasspath/a` 的 `file:` 分支）与 `[102, 104]`（`return null`）。
- 出闭包的 35 类：jrtfs（`JrtFileSystem` / `JrtPath` / `SystemImage` / `ExplodedImage` 等）、`ImageReader` 节点类、`FileTreeWalker` 一组、
  `ZipFileSystem`、`JarInputStream` / `ZipInputStream` / `PushbackInputStream`、`AllPermissionCollection`、`Class$Holder`、
  `PackageHelper$1` / `$2`、`UnixUriUtils` 等。
- 分步：只加前缀事实 −29 类（`findModule` 的 `file:` 分支死）；加 `join_ret` 后 `[102,104]` 也死但计数不变；
  final 字段复读收窄解耦开放判定后 `definePackage@57..87` 死，再 −6 类。
- `JarVerifier`、`JarFile`、`JarURLConnection`、jar `Handler` 仍在闭包内：B5 这条路线已切断，剩余来源是 B1 / B4（§30.8、§30.9）。

#### 与引导映像项目的一致性

- `getSystemPackageLocation` 现为 native 手写边界（a3-L1 / 引导映像第 5 步，已知失败 `TestProtectionDomainFaces` 的
  `BootLoader.getSystemPackageLocation` native 缺口即此方法）。落地实现须只返回 null 或 `jrt:/<模块名>`，与本清单事实一致；
  清单注释已写明这一约定。
- `findModule` 的模块分支另需 `ModuleLayer.boot().findModule(mn)` 在运行期能找到该模块（引导层可用），这同样是引导映像第 5 步的范围，
  本步不改变它，只是闭包不再保留 jar 分支作兜底。
- 两边重叠点只有这一个 native 方法；引导映像若改为构建期求值包位置，结果仍须落在 `jrt:/` 前缀内，否则本事实失效（清单解析不检查运行期值）。

#### 单测

- closure crate：新增 `final_field_reread_keeps_nonnull`（final / 非 final、中间调用、`putfield`、改写接收者局部五种情形）、
  `join_ret_keeps_nonnull`，`string_shapes_parse` 增前缀项与两种错误项。
- closure crate 181 过；`closure_independent_of_hash_seed` 过（1320s）、`container_elements_per_object` 过。
  （首轮哈希种子测试与 rava 重建并发，二进制中途被替换，结果作废后单独重跑。）

#### `encodePath` 结果形状：从字节码推出所需的域扩展（本步未实施，清单事实暂留）

`ParseUtil.encodePath(String, Z)` 的「结果不含 `#` `?`」要从字节码推出，需要下列五项能力，每项都是独立的域扩展，
合计超出本步范围（串形状域只描述串本身，不描述数组内容、不跨变量关联）：

| # | 字节码位置 | 需要的能力 |
|---|---|---|
| E1 | `firstEncodeIndex`：`for (i < len) { c = charAt(i); if (快速通道) continue; if (c > 0x7F \|\| match(c, L, H)) return i; } return -1` | **扫描循环摘要**：返回 -1 ⇒ 串中每个字符都不满足谓词 P；返回 i ≥ 0 ⇒ 前缀 [0, i) 不满足 P。P 对 0–127 逐值常量求值（`match` 的两个掩码是 `ldc2_w` 常量，纯算术），得到 ASCII 排除集 |
| E2 | `encodePath(String,Z)@26..40`：`index > -1` 的假侧返回 `path` | **关系事实**：int 值携带「它是串 s 的扫描下标」标签，分支收窄到 `== -1` 时把 s 的形状并上排除集（需要按来源回写局部变量） |
| E3 | `encodePath(String,I,C)`：`retCC[retLen++] = c` 各处 | **路径敏感的字符值集**：比较链（`c == sep`、`a–z` / `A–Z` / `0–9`、`c ≤ 0x7F`、`match`）两侧收窄 char 局部的值集，`castore` 处取值集 |
| E4 | 同上：`newarray char`、`toCharArray`、`System.arraycopy`、扩容拷贝、`new String(char[], int, int)` | **char 数组内容域**：按分配点记「元素排除集」（全部 `castore` 值集与拷贝来源的交），`new String(char[],…)` 结果取之；`arraycopy(pathCC, 0, retCC, 0, index)` 的拷贝段需要 E1 的前缀事实经形参 `index` 传入（两个调用点：常量 0 与扫描下标），即 E2 的关系标签要过形参常量格 |
| E5 | `escape(char[], char, int)`：写 `'%'` 与 `Character.forDigit(…, 16)` | **数组形参写入摘要**（被调方法对实参数组写入的值集）+ `forDigit` 的返回字符集（返回 int 值集求值，`'0'–'9'`、`'a'–'z'`、`'\0'`） |

终态做法：E1–E5 全部落地后删除两条 `excludes` 清单项，由 `encodePath` 的返回值形状直接给出。E3 / E5 的 int 值集已有
（`absint/ints.rs` 的有限值集），缺的是比较链两侧的收窄与「数组形参写入」摘要；E4 是新的对象标签（`Obj::Chars { no }`），
与 §30.9 的构建器标签同构（`castore` 对应 `append`）；E1 / E2 是新的循环摘要与关系标签，工作量最大。
在此之前保留清单事实，注释里的位运算论证可逐位核对（`#` = 0x23、`?` = 0x3F 落在 L_ENCODED 的置位上）。

#### 待用户决策

1. **final 字段复读收窄不看开放判定**（本步实施）：依据 JLS §17.4.5 / §17.5.3（见设计第 3 项）。可观察差异只在「他线程在本线程两次读取之间
   用反射 / Unsafe 改写 final 字段为 null」时出现：JVM 允许第二次读取返回旧值（C2 实际如此），生成的 Rust 按折叠删去 null 分支后
   读到 null 会继续走非 null 路径。若不接受，需改为依赖开放判定，并先收窄 `reflect_writes.rs` 的形状兜底（见下），否则 B5 的
   `definePackage@57..87` 回到可达（−6 类）。
2. **启动目录是目录**（B1①）：仍待决定，本步未实施任何依赖它的折叠。

#### 遗留与恢复入口

- 分支 `c1d-url-b2`，worktree `/Users/yuwei/dev/workspace/java_rta_c1durl2`。测量 `build/url/run4.sh <tag 后缀>`（构建 rava + 四例 + 对照 `10p`）。
- **形状兜底**：`reflect_writes.rs` 中「调用含 Class 形参与 String 实参 → 名字在 Class 实参上找不到字段时全局按名放开」是按名放开的
  最大来源之一（`value` 由 `ObjectStreamField.<init>` / `getFieldOffset` 放开）。终态：按名取字段的汇点全部精确建模
  （`getDeclaredField(s)` / `objectFieldOffset` / `name_resolvers` / `MemberName` 解析），形状兜底删除。需单独立项、服务器全量验证。
- **`encodePath`**：清单 `excludes` 两项暂留，E1–E5（上表）落地后删除。
- 下一步按协调方顺序回到 B1② 的 P2（构造器确定初始化，入口见 §30.9「恢复入口」）。

### 30.11 第 6 步：B1② 的 P2——构造器确定初始化（2026-10-07，分支 `c1d-url-b2`）

#### 用户决策（2026-10-07，协调方转达）

1. **不采纳「启动目录一定是目录」假设**（§30.9「待用户决策」第 1 项、§30.10 第 2 项）：进程启动后工作目录被删时，JVM 抛 IOException、
   跳过该类路径项，转译版走 JarLoader 撞存根 panic，行为不一致。因此：
   - 不实施任何依赖该假设的折叠；不再测「采用该假设」的反事实上界（§30.9 的 `cutsNoB5` / `cutsAll` 上界含该假设，作废为参考值）；
   - 改找不依赖该假设的可靠事实，例如由字节码推出 `ParseUtil.fileToEncodedURL` 产生的 `file` 路径以 `/` 开头（来自绝对路径），
     不要求它是目录。
2. **B5 的「final 字段判非空后同对象再读仍非空」获认可**（JLS §17.5.3），保留（§30.10 设计第 3 项、待决第 1 项结案）。

#### 设计（通用，无类名）

- **构造器摘要**（`absint/init.rs`，`absint::analyze_init`）：抽象解释状态加 `inits`——当前路径上 `this` 必然已写的实例字段
  （解析后的声明键），合流取交集，异常处理器入口为空。逐指令在执行前的栈上判定：
  - `putfield`：接收者恰为 `this`（来源只有形参 0）→ 并入 `inits`；写入值含 `this` → 交出；
  - `getfield`：接收者可能是 `this` 且字段未写 → 记入 `bad`；
  - 委托 / 超类构造器（`invokespecial <init>`，接收者恰为 `this`，其余实参不含 `this`）：按被调摘要组合——交出集取
    「本方已写 ∪ 被调方交出集」，被调方 `bad` 去掉本方已写，返回后并入被调方返回集；无摘要（手写 / 无法分析）按交出；
  - 其余调用的实参 / 接收者、`putstatic` / `aastore` 的值、`athrow`、indy 实参、`checkcast` / `instanceof` / `areturn` 的输入含 `this` → 交出；
  - `return`：返回集取交。摘要 = (返回集, 交出集, bad)，确定集 = 返回集 ∩ 交出集 \ bad。只有第二阶段（不动点入口状态）记录。
- **引擎**（`engine/ctor_init.rs`）：
  - 摘要以辅助分析事实（`Facts { m: None }`）在记忆化帧内求出，记入 `cinits`；输入记录与求值记忆同一套，
    读过的字段转不折叠或属性不折叠集合增长时作废（`ceval_drop` / `sysprops.rs`）。
  - 辅助事实答不出的调用取唯一字节码目标的返回常量 `rvals`（只取 `Const`；缺席 / Top 按未知，不取「不返回」），
    读过的目标记入摘要（嵌套摘要并入外层），该目标返回常量变化时作废（`cinit_ret_changed`，`bytecode.rs::returns` 与
    `concrete/apply.rs::join_rval` 两处写入点）。触发点：辅助分析不对无实参方法常量求值，`System.getSecurityManager()` 不折 null，
    `URL` 构造器在安全管理器分支把 `this` 交出（`checkSpecifyHandler`），确定集只剩 `hashCode` / `port`。
  - 分配点（`Event::New`）登记 `osite`：该方法里全部 `invokespecial cls.<init>`（JVMS §4.10.1.9：`new cls` 只能经 `cls` 自身的
    `<init>` 初始化，初始化前不能使用）。对象确定集 `odef` = 各构造器确定集之交；类链（不含根类）声明了非抽象 `finalize()V` 时为空。
  - 任一摘要作废置 `cinit_drop`，下一次 `invalidate_all` 开头 `obj_defs_dropped`：清空 `odef`，按对象读过的方法按当前答复复核
    （`obj_queries_same`），答复有变才重分析。终态等于按最终事实计算的摘要，与处理顺序无关。
- **读取侧**（`obj_fields.rs::obj_field_value`）：只有字段不在对象确定集时才并入初值；`ovals` / `owild` 改为从 ⊥ 起算。

#### 健全性与反例核对

- 读到对象字段须先持有引用。构造链帧内的直接读（`getfield this`）未写时记 `bad`；引用经交出点流出时只保留交出时已写的字段；
  构造器正常返回后的读取只看返回集；构造器异常退出时对象不可达（交出点已覆盖其它可达途径），唯一例外是终结器，已排除。
- 抽象对象名含（方法键, 偏移）：字节码 `new`、手写分配（`u32::MAX - k`）、lambda 构造引用（indy 偏移）偏移空间互不相交，
  后两者不在 `osite` 中，保留初值。反序列化 / 物化快照分配的是类 id 而非抽象对象；`clone` 复制的是已写入的值。
- 字节码外写入（反射 / Unsafe / 手写 / VM 钩子）不影响「初值是否可见」：这些写入本身仍由 P5 规则（偏移可得 / 手写写入不折叠）覆盖。
- 返回常量依赖：`rvals[t]` 在不动点上覆盖 t 的全部实际返回（与主分析同一事实，主分析调用点即按此取值），增长时摘要作废重算，单调收敛。
- 反例核对（新增折叠逐条，见下）：
  - `LinkedBlockingQueue.capacity = MAX_VALUE`：可达分配只经无参构造器（委托 `this(Integer.MAX_VALUE)`），`capacity` 在交出前写入；
    若用户调用带容量的构造器，写入按对象记录，折叠自动解除。
  - `SliceOps$1` 的 `val$` 字段：合成外部捕获字段在超类构造器调用前写入（javac 形态），确定初始化。
  - `SpinedBuffer.<init>` 的 `initialChunkPower = 4`：无参构造器路径写常量。
  - `SliceOps$SliceTask.doLeaf` / `doTruncate` / `onCompletion` 的死区，及 `cancel` / `completedSize` / `isLeftCompleted` 出闭包：
    依赖的字段在构造器内必写，原先只因初值并入而保留了初值分支。

#### 实测（tag `12b`，对照 `11d`）

| 测试 | 类 | 方法 | 用时 ms（本机负载，仅供参考） |
|---|---|---|---|
| HelloWorld | 469 → 469 | 1827 → 1827 | 523 → 465 |
| StockTrans | 3150 → 3150 | 19919 → **19916** | 31700 → 26135 |
| DeepCopy | 3152 → 3152 | 19939 → **19936** | 29822 → 26326 |
| TestSerialDefaultSuid | 3157 → 3157 | 19931 → **19928** | 27555 → 26272 |

- 四例均为 `11d` 的子集（新增类 0、新增方法 0）；三个大例各 −3 方法（`SliceOps$SliceTask.cancel` / `completedSize` / `isLeftCompleted`）。
  分析耗时无增幅。DeepCopy 摘要作废 91 次（`init_drops`）。
- 加 `rvals` 依赖前（`12a`）集合相同；加入后 `URL.<init>(String,String,String)` 的确定集由 {`hashCode`, `port`} 扩为
  {`file`, `handler`, `hashCode`, `path`, `port`, `protocol`, `ref`}。
  `URL.<init>(URL,String,URLStreamHandler)`（规格串解析）仍只有 {`hashCode`, `port`}：`handler.parseURL(this, …)` 是对可覆盖方法的
  真实交出，属正确结果。
- 本步单独不折 `R`：`getFile()` 的返回仍按成员键汇合（P3），`val$url` 的接收者来自站点（P4）。

#### 单测

- `absint/init_tests.rs` 10 项（全路径写入、交出截断、自存交出、写前读、单侧分支、未知超类、委托组合、无返回、处理器路径、缺省不跟踪）。
- closure crate 191 过；`closure_independent_of_hash_seed`、`container_elements_per_object` 过（1364s）。

### 30.12 第 7 步：B1② 的 P3——按接收者对象的返回值（2026-10-07，分支 `c1d-url-b2`）

#### 设计（通用，无类名；`engine/obj_rets.rs`）

- **归属**：实例方法节点 m 每次分析的返回值并入 `nret[m]`，再并入接收者形参节点 `P(m,0)` 值集里每个抽象对象 o 的
  `orvals[(o, 方法键)]`（方法键 = 节点自身的声明键）；`P(m,0)` 增长时把 `nret[m]` 补归属到新对象（`oret_watch`，经 `flow.rs::node_grown`）。
  具体求值结果（`concrete/apply.rs`）的返回不按对象归属，并入 `orwild[方法键]`。
- **读取**（`Facts::invoke_result` 的 `obj_ret`）：实例调用、非 void、接收者只来自形参 i、该形参值集非空且全为抽象对象时，
  逐对象定出所执行的方法：精确目标（static / special / private / final），否则按对象的类 `Hierarchy::select`（JVMS §5.4.6，
  对象的类在 `param_obj_sets` 时记入 `oclass`）；只取有字节码的方法（手写 / 无体的返回不按对象归属，有此类对象即不折）。
  答复 = 各对象（`orvals[(o,t)]` ⊔ `orwild[t]`）之并：
  - 有值 → 按该值（与按成员的 `rvals` 同一取法：小集合 / 非空引用仍先按常量实参求值）；
  - 无值而有对象的目标未定论（`noreturn.answer_never`：乐观阶段恒真，收尾阶段只对未建节点 / 未分析完 / 等待中的目标）→ 不返回，
    记 `Dep::Never`，与按成员的「尚无返回」同一收尾机制（排空时重算）；
  - 其余（全部对象定论无返回、或有对象选不出字节码目标）→ 退回按成员的 `rvals`。
  调用点没有唯一目标（虚调用多实现）时同样可用：原先直接答未知。
- **依赖**：查询记为 `ObjQuery::Ret(形参, RetSite{指令, 符号引用, 接口}, 答复)`，与按对象字段读同一套复核：`ordeps[(o,t)]` /
  `orwdeps[t]` 登记读者（按各对象选出的 t），归属变化时按答复复核（`obj_readers_recheck`），形参值集增长经 `obj_watch` 复核，
  共享摘要按答复比对（`obj_queries_same`）。

#### 健全性与反例核对

- 接收者为 o 的调用执行方法 t 时，派发把 o 送进所连 t 节点的 `P(·,0)`，该节点的分析覆盖这次执行（入口状态含本调用点实参），
  其返回值归属到 o；节点按旧入口得出的返回值先归属、后因重分析变化时新值继续并入（只增不减）。所以不动点上
  `orvals[(o,t)] ⊔ orwild[t]` 覆盖接收者为 o、执行 t 的全部正常返回值。
- 方法选择只看对象的类（抽象对象的类固定），与分析进度无关；`select` 与派发用同一解析。手写 / native / 无体方法的返回
  不经字节码 `returns`，不按对象归属，这类对象出现即不折。
- 「定论无返回」的对象贡献 ⊥：目标的全部节点都已分析且无一归属到 o，即 o 不在任何 t 节点的接收者值集里（或这些节点无正常返回），
  不动点上此调用点不会以 o 为接收者正常返回 t 的值。之后若 o 流入 t 节点，`oret_grown` 补归属并经 `ordeps` 复核。
- 「不返回」答复只在目标未定论时给出，排空进入收尾后按 `noreturn.rs` 规则重算，同按成员的「尚无返回」。
- 反例核对：同一形参的对象集含两个对象、各自 `tag` 不同 → 答复为两值之并，不折；对象的类覆盖了被调方法 → 按对象的类选出
  覆盖方法，取其归属；接收者经类型转换 / 合流来自多个来源 → `srcs` 不只一个形参，不查。

#### 实测（tag `13b`，对照 `12b`）

| 测试 | 类 | 方法 | 用时 ms（本机负载，仅供参考） |
|---|---|---|---|
| HelloWorld | 469 → 469 | 1827 → 1827 | 465 → 483 |
| StockTrans | 3150 → 3150 | 19916 → 19916 | 26135 → 29304 |
| DeepCopy | 3152 → 3152 | 19936 → 19936 | 26326 → 28980 |
| TestSerialDefaultSuid | 3157 → 3157 | 19928 → 19928 | 26272 → 29317 |

- 四例集合与 `12b` 相同（子集成立）；耗时 +10%–12%，在 20% 线内（同机另有负载，`13a` 只做归属时为 +0%–7%）。
- 夹具 `ObjFacts`：`use(b1)` 的 `b.tag()`（虚调用、无唯一目标）按对象答 `"a"`，`Rare.go` 出闭包；`Holder.run` 的接收者来自
  字段读（站点），留给 P4。
- 中途发现：答复「对象尚无归属 → 退回 `rvals`」时首次分析恒先退回（被调方尚未分析），而调用边不撤回，折叠永远落空；
  改为与按成员「尚无返回」同一乐观机制后生效。
- 本步四例无折叠差异：URL 链上的 `getFile()` 等调用接收者来自站点（字段读 / 调用结果），需 P4。

#### 单测

- closure_cli 新增 `returns_per_receiver_object`（夹具 `tests/fixtures/ObjFacts.java`）。
- 本节 `13b` 是暂存版（⊥ 退回规则为旧口径），未单独提交；最终实现随 P4 一并提交，修订见 §30.13。

### 30.13 第 8 步：B1② 的 P4——站点来源的接收者 + 复核框架定型（2026-10-07，分支 `c1d-url-b2`）

P3 与 P4 共用一套按对象查询 / 复核框架（`obj_fields.rs` / `obj_rets.rs`），P4 实施中对框架的 ⊥ 规则、复核时机和数据结构都做了改动，
P3 的暂存版若单独提交会带着旧的、依赖处理顺序的退回规则。所以 P3 + P4 合成一个提交；另把顺带发现的、与本线无关的
站点求值顺序依赖单独先提交（见下文「站点求值所用分析」）。

#### 设计（通用，无类名）

- **站点来源**（P4）：`Recv::Site(偏移)`——接收者值只来自一个字节码站点（字段读 / 调用结果，`Src::Site`），且该站点结果的
  声明类型是容器形态类（`obj_site_cands`，按方法键缓存）时，对象集取流图站点节点 `S(m, 偏移)` 的值集。字段读与返回值两种
  查询都可用站点来源。站点节点就是派发用的值集，覆盖方法节点 m 各次执行时该站点产生的全部对象。
- **⊥ 规则**（对象集为空，或各对象尚无值 / 尚无归属）：
  - 字段读：定论（`settled`）之前答 `Never`（读取之后暂不可达），定论之后退回全局值集（`bottom_never`）。
  - 返回值：与按成员的「尚无返回」同一口径——收尾（`closing`）前答 `Never`；收尾阶段只在某个对象的目标 t 仍满足
    `noreturn.answer_never(t)`（尚无节点 / 未分析 / 等待中）时答 `Never`，否则退回 `rvals`。
    这样按对象答复不会比按成员更早放弃「不返回」，乐观不返回的定论时机不受按对象查询影响。
  - P3 暂存版的「对象无归属 → 收尾阶段立即退回」会让退回的宽答复先建边，随后到达的归属再把答复收窄，
    边不撤回，结果依赖处理顺序。
- **复核延后、合并**（`obj_flush`）：值变化（`obj_readers_recheck`，原因 = 字段 / 方法键）、来源值集增长（`obj_grown`）、
  构造器摘要作废（原因 = 全部）都只把方法记入 `obj_dirty`（附原因）。复核在两处进行：流传播排空之后，以及主循环每轮开头
  （不动点各项判定之前）。复核时：
  - 只重算来源对象集（仅限查询用到的来源，`obj_sets_of`，与 `obj_sets` 同口径）；
  - 只复核输入可能变了的查询：来源对象集变了的，所读字段在原因里的，同名同描述符方法在原因里的。
    选择保持名字和描述符，JVMS §5.4.6。
  - 答复有变才重分析。不变时，有增长就按上次登记的对象集只补登新增对象（`ObjBound`，同一组查询 `Rc::ptr_eq`）。
- **数据结构**：`ovals` / `odeps` / `orvals` / `ordeps` 由 `(对象, 成员键)` 改为 `成员键 → 对象 → …` 两级表，
  逐对象查表不再克隆、散列成员键；目标选择按（调用点, 对象）缓存（`oret_sel`：只依赖对象的类与类层次）；
  `obj_field_value` 的初值只并一次。
- **站点求值所用分析**（`worklist.rs::site_analysis`，单独提交）：按名查找的名字、键集（`keyed.rs::names_of`）、字段 / 返回串、
  类查找、方法名求值改取 `applied`（其事件正在 / 已经执行的那次分析），未执行过时取当前分析。
  - 问题：方法失效后、重分析前，站点仍可能因接收者 / 键集增长按 `applied` 的事件重跑。原代码取 `analysis`，此时它已被清空，
    于是答「推不出」→ `Keys::Any` / 开放查找，而这种放宽不可撤回。
  - 实例：TestSerialLookupPairing 的 `tryGet@225/@282` 键门放行全部服务提供者，20250 对 19918，随散列种子变化。
  - 这是本线之前就存在的顺序依赖，P3/P4 增加了失效次数，所以暴露出来。

#### 健全性与反例核对

- 站点来源：`S(m,off)` 是该站点结果的流图值集，不动点上覆盖其全部运行期对象（派发同样依赖它）；只取全由抽象对象组成的值集，
  含 open / 非抽象类 / 镜像即不查。按对象字段值 / 返回值的覆盖论证同 §30.12，与来源种类无关。
- 延后复核：读者在复核前用的是旧答复，可能偏窄（偏乐观）。但每次变化都会记入 `obj_dirty`，主循环在不动点各项判定之前
  一定先排空它，所以终态里每个方法的答复都等于按最终对象集 / 最终值算出的答复。原因过滤只略过输入未变的查询：
  - 答复的输入只有四样：来源对象集、`ovals` / `owild[字段]`、`orvals` / `orwild[选出的目标]`、`odef`（作废时原因为全部）；
  - `bottom_never` / `closing` 的变化不在其中，由 `Dep::Never` / 收尾重算覆盖，与原机制相同；
  - 开放判定的变化不在其中，由 `fdeps` 全局读者失效覆盖。
- 补登只增不删：重分析后旧登记残留，只会多复核，不影响结果。
- 反例核对（新增折叠抽查，StockTrans `13b → 14d`）：
  - `ReferenceQueue.poll()` 按对象答 null，于是 `WeakHashMap.expungeStaleEntries`、`ClassCache.processQueue`、
    `LogManager.drainLoggerRefQueueBounded`、`Level$KnownLevel.purge` 的出队分支死。原因是闭包里没有 `ReferenceQueue.enqueue`：
    引用入队由 VM 的引用处理线程驱动，当前闭包与运行时都没有承载它，所以转译产物里队列确实恒空，与该折叠一致。
    将来按手写边界类别 ③ 落地引用类语义（由 `Rc` 释放触发、不引入 GC，见 `2026-10-07-no-gc-memory-model.md`）时，入队路径进入档案，`head` 字段写入按对象记录，折叠自动解除。
    这一项属于既有的建模缺口，不是新的不健全；见「待用户决策」。
    同类还有 `ResourceBundle.findBundle@73`、`FileInputStreamPool.getInputStream@3`、`CleanerImpl.run`、`MemoryCache.emptyQueue`、
    `LocaleObjectCache.cleanStaleEntries`、`ThreadContainers.expungeStaleEntries`、`Bundles.cleanupCache` 等（均为引用队列出队）。
  - 其余新增常量多为 `true`：`Logger.doSetParent@146`、`ProviderList.<init>@358`、`SortedOps$RefSortingSink.accept@5`、
    `URLClassPath.getLoader@164` 等，接收者对象是 `ArrayList` 一类，其 `add` 字节码恒返回 `true`；按对象选出目标后取该目标的返回值。
  - HelloWorld −4 方法：`ConcurrentHashMap.remove(Object,Object)` / `replaceNode` / `TreeBin.removeTreeNode` / `balanceDeletion`。

#### 实测（tag `14d`，对照 `13b`；`14a`–`14c` 为优化过程）

| 测试 | 类 | 方法 | 用时 ms（本机负载，仅供参考） |
|---|---|---|---|
| HelloWorld | 469 → 469 | 1827 → **1823** | 483 → 520（+7.7%） |
| StockTrans | 3150 → 3150 | 19916 → 19916 | 29304 → 32428（+10.7%） |
| DeepCopy | 3152 → 3152 | 19936 → 19936 | 28980 → 32496（+12.1%） |
| TestSerialDefaultSuid | 3157 → 3157 | 19928 → 19928 | 29317 → 31974（+9.1%） |

- 四例均为 `13b` 的子集（新增类 0、新增方法 0）。大例集合不变，只多了一些常量和死区折叠（StockTrans 有 31 个方法的折叠发生变化，均为增加）。
  URL 链上 `R` 所需的 `getFile()` 站点折叠仍未成立：需要 B4 / `fileToEncodedURL` 路径以 `/` 开头的串形状事实（见下）。
- 耗时优化过程（StockTrans）：

  | tag | 做法 | 用时 |
  |---|---|---|
  | `14a` | 每次流增量都即时重算全部来源、全部重登 | 83985 ms（+187%） |
  | `14b` | 延后合并复核 | 70436 ms |
  | `14c` | 两级表 + 目标选择缓存 + 增量补登 | 35701 ms（+22%） |
  | `14d` | 按原因只复核受影响查询 | 32428 ms |

  `14d` 的 `reasons.mirror.analyses` 为 21742（`13b` 为 1516），`field_put` 由 15443 降到 7819；合计重分析增加约 12600 次，
  分析阶段 +0.6 s。复核本身约 7–8 s，剩余优化空间主要在这里。
- 峰值内存（分配器口径 `peak_mem_mb`，`11d` → `12b` → `13b` → `14d`）：
  - HelloWorld 235 → 236 → 238 → 221；
  - StockTrans 2178 → 2173 → 2174 → 2245（相对 B5 +3.1%）；
  - DeepCopy 2200 → 2224 → 2293 → 2304（+4.7%）；
  - TestSerialDefaultSuid 2201 → 2149 → 2249 → 2161（−1.8%）。

  P2–P4 合计增幅在 20% 线内。`peak_rss_mb` 受本机 swap（约 90%）影响波动大：`11d` 为 1352–1520，之后为 1967–2315，
  这只说明页面是否被换出，不作判据。
- 确定性：TestSerialLookupPairing 种子 0 / 1 / 2 的类集合与方法集合一致（3153 / 19918，via 字段可变），修复前为 20250。

#### 单测

- closure crate 191 过；closure_cli `closure_independent_of_hash_seed`、`container_elements_per_object`、`returns_per_receiver_object`、
  `returns_per_site_receiver`（夹具 `ObjFacts.Holder.run`：接收者来自字段读站点，`Rare2.go` 出闭包）4 项全过（1554 s）。

#### 待用户决策

1. 引用队列：P3/P4 让「闭包与运行时都不承载 VM 引用处理线程」这一既有缺口变得可见：`WeakHashMap` 等的出队清理分支被删去，
   转译产物与当前运行时一致，但与 JVM 不同（JVM 下弱引用被回收后会入队）。是否把引用类语义（手写边界类别 ③）列入后续项？
   一旦列入，入队路径进入档案，相关折叠会自动解除，不需要回退本步。
   **已定（2026-10-07）**：列入，按无 GC 内存模型实施——引用类语义由 `Rc` 释放触发，C4 之后（`2026-10-07-no-gc-memory-model.md`）。

#### 遗留与恢复入口

- 下一步（不依赖「启动目录是目录」假设）：由字节码推出 `ParseUtil.fileToEncodedURL` 产生的 `file` 路径以 `/` 开头
  （绝对路径），配合 §30.9 的 B4 `startsWith` 守卫与串形状域，验证 URL 链 `R` 能否折叠。入口：
  - `build/url/cl.sh tests/e2e/23_algorithms/DeepCopy.java <tag> --flows '@trace:sun/net/www/protocol/jar/Handler'`；
  - `build/url/folds.py`。
- 复核耗时：`obj_flush` 约 7–8 s（StockTrans），可再按查询粒度登记脏位（字段读按 (字段, 对象) 精确定位查询）。
- 工具：`build/url/run4.sh <tag>`，`python3 build/url/cmp.py <new> <old>`。

### 30.14 第 9–11 步：facts.rs 拆分、URL 链折叠 R 的可行性、撤除按名放开字段的兜底（2026-10-07，分支 `c1d-url-b2`）

#### 第 9 步：`engine/facts.rs` 按职责拆分（提交 2d4a4bd8）

- 拆为 `facts/{kinds,fields,calls,oracle}.rs`，行为不变。
- `15a` 对 `14d`：四例类 / 方法集合完全一致。
- 用时（ms）：HelloWorld 520 → 510，StockTrans 32428 → 31857，DeepCopy 32496 → 32121，TestSerialDefaultSuid 31974 → 31689。
- closure crate 单测 191 过。

#### 第 10 步：URL 链折叠 R（E1–E5）——阻塞，待用户决策

目标：由字节码推出 `ParseUtil.fileToEncodedURL` 的路径以 `/` 开头，从而折叠 `URLClassPath$3.run`（R）里的 jar 分支，
把 jar `Handler`（J）移出闭包。不假设启动目录是目录。

结论：仅凭 `/` 前缀不能移除 J，**不是推导精度问题，是语义上推不出**。

- R 的 URL 不只来自 `fileToEncodedURL`：`JarLoader` 解析 manifest 的 `Class-Path` 得到相对 URL，
  以 jar 的 URL 为基解析后回流到 R 的同一入口。
- `Class-Path` 内容来自运行期读到的 jar，不受字节码约束，可以写成 `jrt:/…` 等任意形状。
  例如基为 `file:/a/b.jar`、条目为 `jrt:/x` 时，得到的 URL 不以 `file:/` 开头。
  所以 `/` 前缀事实对这一来源不成立，R 的 jar 分支在开放世界下可达。
- 反事实实测（DeepCopy，**不健全的剪枝，只用于量化上限**）：

  | 剪掉的分支 | 类 / 方法 | J |
  |---|---|---|
  | 不剪（`15a`） | 3152 / 19936 | 在 |
  | R@97 + R@127 | 3150 / 19913 | 仍在 |
  | 再加 `getJarFile@63` | 3150 / 19908 | 仍在 |
  | R 的三个分支全剪 | 3147 / 19812 | 仍在 |

  即使把 R 全部剪掉，J 仍经其他路径留在闭包中，收益上限约 5 类 / 124 方法。
  E1–E5 做到底也只能拿到其中健全的一部分，并且依赖下面的产品取舍。

待用户决策（原则 / 产品取舍，本项停在此处）：

- (a) 接受现状：URL 链保留。不改语义，闭包多约 5 类 / 百余方法。
- (b) 构建期固定资源目录：原生二进制的资源与类路径在构建期确定，运行期不再按 jar manifest 扩展类路径。
  这样 `Class-Path` 来源在档案中消失，R 的 jar 分支可健全折叠。
- (c) 原生二进制不设应用类路径：与模块模式一致（`cp = null`），类全部静态链接，`URLClassPath` 只服务于显式构造的
  `URLClassLoader`。R 整条链在档案中只因用户显式使用而进入。
- (d) 重新评估「启动目录是目录」的假设：只能处理 `fileToEncodedURL` 一支，按上面的反例仍不足以移除 J。不建议。

建议 (c)：最贴合「rava 是 Java 的原生编译后端」的定位，类在构建期全部已知，运行期类路径扩展本来就无从加载新字节码。
次选 (b)。选定后，R 的折叠由清单声明运行模型，生成器不需要类名特判。

#### 第 11 步：撤除「按调用形状按名放开字段」的兜底

原状：`reflective_writes` 对任何带 `Class` 形参或 `Class` 接收者、且有 `String` 实参的调用点，按名放开同名字段
（`open_field` / `open_field_name`）。这是不按清单的兜底，与「清单即边界」相悖，也是若干假阳性的来源。

撤除前先实测兜底触发的全部站点。它覆盖、而精确解析器没有覆盖的字段身份入口只有两个：

- `MethodHandles$Lookup.resolveOrFail(byte, Class, String, Class)` 的字段形态；
- `MemberName.<init>(Class, String, Class, byte)`。

其余触发都是方法查找（已由 `method_lookups` 覆盖）或假阳性。

终态设计：

- 按名取字段身份的入口**只有**清单 `[facts.field_writes.name_resolvers]`。新增两条，都带引用种类过滤：
  - `MethodHandles$Lookup.resolveOrFail:(BLjava/lang/Class;Ljava/lang/String;Ljava/lang/Class;)…`
    = `{ kind = 0, class = 1, name = 2, read_kinds = [1, 2] }`；
  - `MemberName.<init>:(BLjava/lang/Class;Ljava/lang/String;Ljava/lang/Object;)V`，过滤同上。
- `kind` / `read_kinds`：`kind` 形参是常量且属于只读种类（JVMS `REF_getField` = 1、`REF_getStatic` = 2）时，
  本站点不算写入。种类非常量时按写入处理（健全）。只给 `read_kinds` 不给 `kind` 时清单解析报错。
- `MemberName(Class, String, Class, byte)` 不入表：java.base 中它的调用方只有 `resolveOrFail` 与
  `DirectMethodHandle.createFunction`（`REF_getStatic`，只读）。前者已经在 `resolveOrFail` 处按种类过滤，
  后者只读。实测把它列为根时，getter 查找会被当成写入（`16a` 多出 `FinalReference` 1 类 + 2 方法）。
- 名字值集（`field_names.rs::name_values`）：
  - 本点字面量。有来源时只取 `site_lits`，即不把常量格派生出的中间态常量当字面量（D1）。
    `16b` 的反例：`findVarHandle` 内的派生值 `V::Str("head", [Param(2)])` 导致 `open_field_name("head")`，
    多出 `FinalReference`。
  - 形参来源：取各调用点在该形参上的字符串常量（`pstr_read`）。已配对、且方法只经字节码调用点进入时不取，交给配对。
    形参被污染（`ptaint`）时为未知。
  - 字段读来源：`String` 字段的字面量写入集（`field_strs`）。字段已放开或有非常量写入时为未知。
  - 辅助方法返回：`callee_consts`。
  - `catch` 来源：未知。
- 未知名字的回退：按类值集放开该类全部字段，类也未知时全放开（`open_class_fields(None)`），
  不再按同名字段在全体类上放开。
- 配对（`lookup_pair.rs::field_wrap_call`）：同样取 `name_values`，名字未知时按配对的类值集放开。
- 非字节码入口（`pstrs.rs::offsite`）：方法经 `bind_pvs`（反射 / 句柄等非字节码调用）进入时登记为 offsite，
  并以 `TRIG_TAINT` 重跑其形参读站点。此后配对不再豁免形参名，避免漏掉不经字节码调用点进入的名字。

健全性审计：被新折叠掉的项逐一查过，都是旧兜底的假阳性。

- 名字取自 `ObjectStreamField` / `FieldValues.getFieldOffset` 等按名 `getField` 读路径，被兜底当成写入：
  - `ObjectOutputStream.protocol`、`CountingWrapper.count`；
  - `KeySetView.value`、`TimeUnit`、`PlatformLogger.isLoggable`；
  - `SunPKCS11.<init>`、`CoderResult`、`ServiceList.tryGet`。
- `ModuleReader.open` / `read`：可达的实现只有 `NullModuleReader`。
- `UnresolvedPermissionCollection` / `Secmod$Module`：闭包中没有其构造器或 `readObject`。

迭代记录：

| tag | 做法 | 相对 `15a` |
|---|---|---|
| `16x` | 直接删兜底 | **不健全**：`BMH_SPECIES` 被折为 null（`resolveOrFail` 字段形态无人放开） |
| `16a` | 加 `resolveOrFail` 与 `MemberName(Class,String,Class,B)` 为解析器 | 多出 `FinalReference` +1 类 +2 方法（getter 查找被当成写入） |
| `16b` | 引用种类过滤，去掉 `MemberName(Class,String,Class,B)` 根 | 多出 `FinalReference`（中间态派生名，D1） |
| `16c` | 有来源时只取 `site_lits` | 四例集合与 `15a` 完全一致 |

四例数据（`15a` → `16c`；类 / 方法集合逐项相等）：

| 用例 | 类 | 方法 | 用时 ms | `peak_mem_mb` |
|---|---|---|---|---|
| HelloWorld | 469 | 1823 | 510 → 548（+7.5%） | 221 → 249（+12.7%） |
| StockTrans | 3150 | 19916 | 31857 → 31147 | 2247 → 2173 |
| DeepCopy | 3152 | 19936 | 32121 → 31048 | 2300 → 2225 |
| TestSerialDefaultSuid | 3157 | 19928 | 31689 → 31641 | 2165 → 2264（+4.6%） |

- 用时与内存增幅都在 20% 线内。HelloWorld 的绝对增量是 38 ms / 28 MB。
- 本步的收益是结构性的：去掉一条不按清单的通道，`name_resolvers` 成为按名取字段身份的唯一入口。
  四例集合没有缩小，因为旧兜底的假阳性在这四例中都已被其他事实覆盖或不可达。

#### 单测

- closure crate 193 过（新增 `reference_kind_filters_read_only_sites`、`read_kinds_require_kind`）。
- closure_cli：`closure_independent_of_hash_seed`、`closure_independent_of_order`、`param_string_constants_fold_switch` 3 项全过（1667 s）。

#### 遗留与恢复入口

- 第 10 步待用户在 (a)–(d) 中选定。选 (b) 或 (c) 后，入口是清单声明运行模型（类路径来源），再以
  `build/url/cl.sh tests/e2e/23_algorithms/DeepCopy.java <tag> --flows '@trace:sun/net/www/protocol/jar/Handler'`
  核对 J 的剩余路径。反事实表说明，剪掉 R 后 J 仍在，需要沿 trace 继续找。
- `MemberName(Class, String, Class, byte)` 不入表靠调用方审计。若将来档案中出现新的调用方，需要复查。
  审计脚本思路：用 javap 列出 java.base 中该构造器的调用点。
- 工具：`build/url/run4.sh <tag> <base>`、`build/url/cnt.py`、`build/url/folds.py`。

### 30.15 第 12 步：原生二进制不设应用类路径（用户决策 (c)，2026-10-07，分支 `c1d-url-b2`）

#### 用户决策（2026-10-07，协调方转达）

选 §30.14 第 10 步的 (c)：生成的原生程序没有运行期应用类路径，等价于模块模式下 `cp = null`。
类全部在构建期编入二进制，运行期不按类路径从磁盘加载 `.class`。用户代码经
`ClassLoader.getResource` / `getResourceAsStream` 读取的类路径资源，在构建期打包进二进制。
机制通用，不为任何库做特判。

#### 字节码事实（JDK 21，javap 核对）

- `ClassLoaders.<clinit>`：`cp = System.getProperty("java.class.path")`。若 cp 为空串或 null，再看
  `jdk.module.main` 是否为 null：为 null 时取 `""`，否则取 null。随后**无条件**执行 `new URLClassPath(cp, false)`。
- 只有 `jdk.module.main != null` 才会得到 cp = null。但 `ModuleBootstrap.boot2`（会追加根模块）和 `LauncherHelper`
  也读这个属性，所以不能用设置它的办法来实现 (c)。
- `URLClassPath(String, boolean)`：逐个把类路径元素交给 `toFileURL(String)`，返回非 null 的才加入 `path`。
  构造器还**无条件**执行 `this.jarHandler = new sun.net.www.protocol.jar.Handler()`，cp = null 时也一样。
- `toFileURL` 的调用方只有两处：上面的 String 构造器，以及 `addFile`
  （`BuiltinClassLoader.appendClassPath` ← Instrumentation 追加类路径）。两处的含义都是「类路径字符串元素 → 运行期加载源」。
- 应用加载器使用 ucp 的地方：`findClassOnClassPathOrNull`（`ucp.getResource` → `defineClass(String, Resource)` →
  `defineClass1`）、`findResourceOnClassPath`、`findResourcesOnClassPath`，以及 `hasClassPath()`（`ucp != null`）。

#### 设计（通用，无类名；清单声明运行模型）

1. **类路径运行模型由清单声明。** `vm_intrinsics.toml` 新增内建类别 `class_path`，含义是：原生二进制没有运行期
   应用类路径，方法体唯一的可观察效果就是「把类路径变成运行期加载源」或「从类路径加载」，由 VM 以构建期结果等价承载。
   登记四个成员，均为手写类 ②（运行模型替换）：
   - `URLClassPath.toFileURL(String)` 恒返回 null。类路径字符串里的元素不形成加载源，于是应用 ucp 的 `path` 恒空，
     与模块模式 cp = null 时「类路径上没有任何东西」的状态一致。`addFile` 自然随之失效。
   - `BuiltinClassLoader.findClassOnClassPathOrNull(String)` 恒返回 null。类路径上没有可定义的字节码；闭包内的类
     已经由 `findLoadedClass0` 在前一步命中。
   - `BuiltinClassLoader.findResourceOnClassPath(String)` 和 `findResourcesOnClassPath(String)`：保留 `hasClassPath()`
     的判定，资源改由构建期嵌入表（见第 2 条）提供。
   
   `[concrete.boot.natives]` 里 toFileURL 的条目由 `defer_call` 改为 `const:null`，引导映像求值与运行期一致。
   闭包分析器把手写体当作精确返回（`hw_ret`：返回 null，或返回 Java 静态方法调用的结果），不需要类名特判。
2. **构建期资源表。**
   - **来源**：用户档案（`Origin::User` 的编译输出目录，含用户项目资源目录）和 `--deps` / `--cp` 给出的库档案
     （`Origin::Lib`），按类路径顺序收录**全部文件**，包括 `.class`，与 JVM 上「类路径资源」的口径一致。
     同名资源按类路径顺序保留多份（对应 `getResources` 的枚举序）。
   - **落点**：用户侧元数据。`UserMeta` 增加 `class_path_resources` 字段，由用户 crate 的
     `rava_user_meta.rs` 用 `include_bytes!` 给出 `(名, 字节)` 表，按名有序，同名保持类路径序。
     运行时声明层与档案侧不随用户变化，跨测试编译复用不受影响。
   - **门控**：表只在读取它的 native 方法位于闭包调用链上时发射，否则为空表。
3. **运行期资源面（VM 支持类，`runtime/java_support/java.base/jdk/internal/loader/EmbeddedClassPath.java`）。**
   - 两个 native 读表：`count(String)` 返回同名资源的份数，`bytes(String, int)` 返回第 i 份的字节。
   - Java 方法 `findResource(String)` / `findResources(String)` / `stream(String)` 构造 URL 与流。
   - URL 形如 `rava-cp:/<ParseUtil.encodePath(名)>`，第 i（≥ 1）份带 `#i`。构造时显式给出本类的 `URLStreamHandler`
     （`Handler`）：`openConnection` 返回读表的 `URLConnection`，`getInputStream` 是 `ByteArrayInputStream`，
     同时给出 `getContentLengthLong`。
   - natives 写在共置手写文件 `embedded_class_path_impl.rs`（`#[jvm_native]`），读 `meta::class_path_resources()`。
4. **java.class.path 属性**保持 `""`（`[facts.system_properties.values]` 与 `[concrete.boot] vm_props` 不变）：
   - JDK 模块模式下该属性同样是 `""`；
   - `TestSystemPropsSpec` 只要求该属性存在；
   - 在 `jdk.module.main` 为 null 时，`""` 经 ClassLoaders 得到的就是 `""`，再经第 1 条得到空 ucp。
   
   语义：「没有运行期类路径」。属性值不列出构建期的类路径，因为这些路径在运行期机器上并不存在。
5. **模块资源的口径收窄。** `input/src/resources.rs::derive` 只在 JDK 档案（`Origin::Jdk` / `Image`）里查找资源。
   用户档案和库档案的资源统一由第 2 条的类路径表承载，不再混入 `jdk_resources::module_resources`
   （它位于声明层，混入会让声明层随用户变化）。
6. **过渡期手写（L2 六个资源方法，`class_loader_impl.rs`）改为经第 3 条取类路径部分。**
   - `__impl_getResource` / `getSystemResource` → `EmbeddedClassPath.findResource`；
   - `__impl_getResources` / `getSystemResources` → `findResources`；
   - `__impl_getResourceAsStream` / `getSystemResourceAsStream` → 先查模块资源，再查 `stream`。
   
   这六个方法的终态仍是撤除：引导映像第 5 步（jimage）落地后，`ClassLoader.getResource` 走字节码，经委派链到达
   第 1 条的 `findResourceOnClassPath` 钩子，由同一张表承载。
7. **嵌入资源视图与系统类加载器所见一致：模块资源在前，类路径在后**（协调方 C4 补充：`TestSystemStableProps`、
   `TestClassResourceStream`）。
   - `EmbeddedClassPath` 的两个 native 读的是「模块资源（至多一份）+ 类路径行」的合并视图，与 JVM 上
     `getSystemResources` 的顺序（父加载器 / 模块先、类路径后）一致。于是 `getResource` / `getResources` /
     `getResourceAsStream` 对 JDK 模块资源也给出 URL 与流，第 6 条的 `resource_stream` 不再另查模块表。
   - 类文件也是模块资源：`<类名>.class` 形态的字符串若指向 JDK 类（`Origin::Jdk` / `Image`），嵌入该类解析胜出的
     类文件字节（镜像改写类优先于 jmod，与类本身同源）。`resources.rs::path_like` 不再排除 `.class`，
     `ClassPath::jdk_resource` 对 `.class` 名按类解析取字节。
   - 资源名的来源分两侧，保证档案侧与具体程序无关：
     - 档案侧（`jdk_resources::module_resources`，声明层）：非用户域调用链上方法体的字符串常量推导，规则不变；
     - 用户侧（`UserMeta.user_module_resources`，用户 crate 的 `USER_MODULE_RESOURCES`）：用户类调用链上方法体
       的字符串常量推导、档案侧未收的 JDK 模块资源，如 `getSystemResourceAsStream("java/lang/String.class")`。
     运行期 `meta::module_resource(name)` 先查档案表、再查用户表；带前导 `/` 的名不命中（`ClassLoader` 资源名语义）。
   - 用户侧两张表落在 `user/src/rava_resources/{class_path,module}/<i>`。
8. **镜像类整体入闭包只限运行期定义的类**（`closure/src/engine/seeds.rs::seed_image`）。
   - 原规则：镜像独有类 / VM 支持类中父类在闭包内的非接口类，整体入闭包（实例化 + 全部方法 + 全量反射面），
     且同直接父类的闭包类（物种族）同取全部方法。它针对物种类、代理类、注入调用器这类由 VM / 运行模型在运行期
     直接定义并实例化、字节码里看不到实例化点的类。
   - `EmbeddedClassPath$Handler`（父类 `URLStreamHandler`）与 `$Connection`（父类 `URLConnection`）是普通支持类，
     却命中了原规则：`Handler` 被整体实例化，`sun.net.www.protocol.jar.Handler` 等同父类的闭包类被拉成物种族、
     全部方法入闭包（`sameFile → hostsEqual → InetAddress.getByName → ServiceLoader → …`），HelloWorld 由 469 类
     涨到 2870 类（tag `17a`）。
   - 新判据（通用，按字节码事实）：**除自身外没有任何镜像类 `new` 它**，才是运行期定义的类。被别的镜像类 `new`
     的类，实例化点在字节码里可见，走常规可达性。实测：JDK 21 镜像独有类与五个 VM 支持类中，`Species_*`、
     `Proxy$Dyn`、`Species_Dyn`、`InjectedInvokerDyn`、`SerializationConstructorAccessorDyn` 只被自身 `new`（或从不被 `new`），
     判定不变；只有 `EmbeddedClassPath$Handler` / `$Connection` 改走常规可达性。判据集首轮补种时算一次
     （`SeedState.image_static_new`）。

#### 影响的 e2e 与处理

| 测试 | 依赖 | 处理 |
|---|---|---|
| `TestClassResourceStream` | 读自身 `.class`（流、URL）、未命中为 null、`getSystemResource(自身)`、`getClassLoader().getResource(自身)`；`getSystemResource("java/lang/String.class")` | 自身 `.class` 由类路径表命中；`jdk-res` 由第 7 条用户侧模块资源（`java/lang/String.class`）命中。预期由运行 NPE 转为通过（C4 全量登记的失败） |
| `TestSystemStableProps` | `getSystemResourceAsStream("java/lang/String.class")`，期望 `sys-stream=true head=CA` | 第 7 条：用户侧模块资源嵌入 `java/lang/String.class` 字节，流首字节 `0xCA`。预期由 `sys-stream-ex=NullPointerException` 转为通过（C4 全量登记的失败，日志 `error_logs/TestSystemStableProps_ubuntu_jdk21.log`） |
| `TestSystemPropsSpec` | `java.class.path` 存在 | 保持 `""`，不受影响 |
| `TestAppClassLoader` | `forName` 不存在的类 → CNFE；自定义 `getResources` 覆盖；ServiceLoader 经上下文加载器 | CNFE 路径：`findClassOnClassPathOrNull` 恒 null，结果不变。覆盖分派不变 |
| `TestServiceLoaderEmpty`、`TestBootContextLoader` 等 ServiceLoader / 上下文加载器用例 | `getResources("META-INF/services/…")` | 用户档案若带 `META-INF/services`，现在能枚举到。e2e 用例没有资源文件，枚举仍为空，输出不变 |
| `TestClassLoaderIdentity`、`TestForNameInit`、`TestClassForNameInit`、`TestForNameComputedName`、`TestModuleLayerDefine`、`TestDynamicProxy` 等 | 加载器层级 / forName | 走 `findLoadedClass0`，不经类路径，不受影响；需抽查确认 |

e2e 语料中没有非 `.java` 的资源文件，所以类路径资源表的内容只有用户 `.class`；用户侧模块资源表只含用户代码
指名的 JDK 资源（`TestClassResourceStream` / `TestSystemStableProps` 为 `java/lang/String.class`）。

#### 产品取舍（待用户决策，本项停在建议上，实现按建议默认）

- **T1 资源表的收录口径**（二进制体积与 JVM 等价之间的取舍）：
  - (a) 收录全部文件，含 `.class`。与 JVM 等价，体积约为类路径归档的大小；只在读表 native 可达时发射。
  - (b) 不收录 `.class`。体积最小，但 `getResource("X.class")` 不再与 JVM 一致（常见于字节码库、版本探测）。
  - (c) 按调用链上的资源名常量推导收窄（同 `module_resources` 的推导法），名字无法静态枚举时退回 (a)。
  
  建议：现在实现 (a)；(c) 作为后续的精度项，单列在体积线上。
  **用户已决策（2026-10-07）：取 (a)，见 §30.16。**
- **T2 URL 字符串往返**：`new URL(url.toString())` 要求 `sun.net.www.protocol.<scheme>.Handler` 这个类存在。
  - (a) 不支持往返。不在 java.base 中新增包；`toString` 后重建 URL 会抛 `MalformedURLException`。
  - (b) 新增 VM 支持类 `sun.net.www.protocol.rava_cp.Handler`，使 URL$DefaultFactory 能按协议名找到它。
    （注：`rava_cp` 不是合法的 URL 协议名，协议只允许字母、数字、`+`、`-`、`.`；实施时改为 `ravacp`。）
  
  建议：先 (a)。若库语料出现往返用法再做 (b)。
  **用户已决策（2026-10-07）：取 (b)，已实施，见 §30.16。**

#### 实测（tag `17b`，对照 `16c`；`17a` 为第 8 条修正前）

| 用例 | 类（16c → 17b） | 方法（16c → 17b） | 耗时 ms | 峰值 RSS MB |
|---|---|---|---|---|
| HelloWorld | 469 → 464（−6 / +1） | 1823 → 1738（−85 / +0） | 548 → 530 | 290 → 225 |
| StockTrans | 3150 → 2798（−355 / +3） | 19916 → 17310（−2621 / +15） | 31147 → 30228 | 2247 → 1941 |
| DeepCopy | 3152 → 2800（−355 / +3） | 19936 → 17323（−2628 / +15） | 31048 → 30203 | 2270 → 1963 |
| TestSerialDefaultSuid | 3157 → 2805（−355 / +3） | 19928 → 17319（−2624 / +15） | 31641 → 30856 | 2308 → 1887 |

- 新增项只有 `EmbeddedClassPath` 本身（HelloWorld 只到类型层，无方法），以及三例大用例里的 `$Handler` / `$Connection`
  与它们覆盖或继承的 `URLConnection.getHeaderField` / `getHeaderFieldDate` / `getLastModified`：这是新运行模型的承载类，
  取代被移除的 `URLClassPath$JarLoader` / `$FileLoader` / `$Loader`、`FileURLMapper`、`HttpURLConnection` 等。除此之外，
  类集与方法集均为 16c 的子集。
- 大用例的 −355 类来自 ucp 的加载器链整体出闭包：jar / file 加载源、`JarFile` 校验、由此牵出的 JCA 提供者
  （`KeychainStore`、`RSACipher` …）与 NIO 缓冲族。
- `17a`（第 8 条修正前）HelloWorld 涨到 2870 类 / 16962 方法，原因见第 8 条。

#### jar `Handler` 能否出闭包（`cl.sh DeepCopy dcjh17b --flows '@trace:sun/net/www/protocol/jar/Handler'`）

不能。理由可靠，两条都是字节码事实：

1. `ClassLoaders.<clinit>` 无条件执行 `new URLClassPath(cp, false)`，构造器无条件执行
   `jarHandler = new sun.net.www.protocol.jar.Handler()`（trace #1–#6）。cp = null 时也一样。所以 `Handler` 必然被实例化。
   HelloWorld 中它只有 `<init>` 在闭包里。
2. DeepCopy 另有一条用户可达路径：`URL.readObject` / `readResolve`（反序列化 URL）→ `URL.getURLStreamHandler(protocol)`
   → `URL$DefaultFactory.createURLStreamHandler`，协议名来自流，于是按名构造 jar `Handler`（trace #16–#55）。
   这是合法语义：反序列化一个 `jar:` URL 本来就需要它。它的 `parseURL` / `sameFile` / `hashCode` 等方法因此可达。

`ucp` 的 jar 加载源（`JarLoader`）已经出闭包；`Handler` 本身只能经由「把 `jarHandler` 字段改成惰性」这类改写 JDK 字节码
的方式拿掉，不符合手写边界规范，不做。

#### 抽查清单（服务器，由用户发起）

- 预期由失败转为通过：`TestClassResourceStream`、`TestSystemStableProps`（C4 全量登记的两例）。
- 类路径 / 加载器 / 资源：`TestSystemPropsSpec`、`TestAppClassLoader`、`TestServiceLoaderEmpty`、`TestBootContextLoader`、
  `TestClassLoaderIdentity`、`TestForNameInit`、`TestClassForNameInit`、`TestForNameComputedName`、`TestModuleLayerDefine`、
  `TestDynamicProxy`，以及其余 ServiceLoader 用例（`--filter ServiceLoader`）。
- 第 8 条判据（物种类 / 代理类整体入闭包不变）：MethodHandle 与 Proxy 相关用例（`--filter MethodHandle`、`--filter Proxy`）。
- 四例回归：`HelloWorld`、`StockTrans`、`DeepCopy`、`TestSerialDefaultSuid`。

#### 单元测试与编译（本机，`build/check-target`）

- 生成器单测：`closure`、`classfile`、`resolve`、`input`、`emit` 全过（11 + 193 + 71 + 2 + 19 + 13 + 8 + 1，0 失败）。
- `closure_cli` 的 `closure_independent_of_hash_seed` 与 `closure_independent_of_order` 都通过（`_large` 按惯例 ignored），
  即与哈希种子和处理顺序无关。
- `rava_meta_tables` 单测全过。
- 对 `TestClassResourceStream` 的 scratch 跑 `cargo check`，rc=0。唯一的警告是既有的 pc_map 警告，与本步无关。
- 提交顺序说明：`34b92dd2`（清单与加载器手写改用 `EmbeddedClassPath`）在 `5351646a`（新增该类）之前。
  因此只有 `34b92dd2` 这个中间提交缺类，`5351646a` 及之后的终态可以编译。

#### 未完成项与续作入口

1. 服务器抽查：按上面的清单，在 C4 冻结解除后由用户发起。合入集成分支也等冻结解除，本分支暂不合入。
2. T1 / T2 产品取舍：用户已决策，T1 取 (a)，T2 取 (b)，见 §30.16。
3. T1 (c) 的收窄（资源名推导）属于体积线的精度项，待 T1 决策后再排。

### 30.16 第 13 步：T1 定案与 T2 (b)——嵌入资源 URL 的字符串往返（2026-10-07，分支 `c1d-url-b2`）

#### 用户决策

- **T1（资源表收录口径）**：构建期收录类路径上的全部资源，包括 `.class`。这是正确性口径的终态。
  按调用链上的资源名收窄属于后续的体积优化：名字能静态枚举时收窄，不能时退回全量。本步只记录结论，不实现收窄。
- **T2（URL 字符串往返）**：取 (b)。由字符串重建的 URL（`new URL(url.toString())`、`URI.create(s).toURL()`）
  能读到构建期嵌入的资源。

#### 设计

1. **协议名 `ravacp`**。URL 协议只允许字母、数字、`+`、`-`、`.`（§30.15 原文写的 `rava_cp` 不合法）。
   `EmbeddedClassPath.PROTOCOL = "ravacp"`。
2. **处理器 `sun.net.www.protocol.ravacp.Handler`**，是 VM 支持类，Java 源在
   `runtime/java_support/java.base/sun/net/www/protocol/ravacp/Handler.java`，由字节码翻译。
   - 它只覆盖 `openConnection(URL)`，转给 `EmbeddedClassPath.openConnection(u)`。
   - 包名遵循 `URL$DefaultFactory` 的约定 `"sun.net.www.protocol." + protocol + ".Handler"`，所以按协议名能找到它。
     `getURLStreamHandler` 的其余查找层（工厂、provider、`java.protocol.handler.pkgs`）不受影响。
   - 嵌套类 `EmbeddedClassPath$Handler` 删除；`EmbeddedClassPath` 改为 public，供该包访问。
3. **URL 形状**：`new URL("ravacp", "", -1, "/<编码后的资源名>#<序号>", HANDLER)`。
   - host 取 `""`，与 `parseURL` 对无 authority 串给出的结果一致，所以 `toString` 往返后 `equals` 和 `hashCode` 成立。
     JVM 的 `file:` 资源 URL 的 host 也是 `""`。
   - 构造时直接传入 `HANDLER` 单例，不经查找，所以 getResource 路径的开销不变。
4. **连接对任意重建 URL 健壮**。`Connection.connect()` 解码路径（必须以 `/` 开头，`ParseUtil.decode` 失败视为非法），
   再解析 ref 中的副本序号（非数字视为非法）。路径或序号非法，或表中查不到该项，一律抛 `FileNotFoundException`，
   与 JVM 打开不存在的 `file:` URL 一致。读取的条目与 `[class_path] resource_readers` 是同一张表。
5. **镜像补充目录的模块归属**（resolve，提交 `7f0bd9ed`）。
   - VM 支持类可以位于 JDK 没有的新包，比如 `sun/net/www/protocol/ravacp`。此前镜像来源的类只能借已有包的归属取得模块，
     新包里的类会没有所属模块：`real_jdk_module_graph` 的无主类检查不过，`reads()` 和导入发射也会出错。
   - 新规则：仅镜像目录 `jimage/<fp>/only/<module>` 与 VM 支持类目录 `vmsupport/<fp>/<module>` 本来就按模块分目录给出，
     目录名即所属模块（`ClassPath::add` 登记 `overlay_modules`）。`ModuleFacts::build` 加一道后处理：
     登记名不是 JDK 模块节点的（比如测试用的临时目录）清掉，退回原来按包借归属的做法。
6. **seed_image 判据（§30.15 第 8 条）不变**：Handler 由 `EmbeddedClassPath.<clinit>` 直接 `new`，按普通可达性入闭包。

#### 接入点（清单声明，生成器不含类名）

- `vm_intrinsics.toml [facts] class_lookups`：`Class.forName` 按名取类。协议名推不出时，
  按闭包内的类名匹配 `"sun.net.www.protocol.*.Handler"`。
- `vm_intrinsics.toml [facts.keyed_lookups]` 中 `URL.getURLStreamHandler` 一条：`class_pattern = "sun/net/www/protocol/{}/Handler"`，
  按此键为 `"ravacp"`。该条注释已补上 ravacp 的说明；`[class_path] resource_readers` 的注释补上 `openConnection` 读同一张表。
- 不新增清单条目，也不新增生成器特判。

#### 分析器修正：按名取类站点的「推不出」不再从上游槽外泄（`engine/pstrs.rs::param_inputs`）

- 实测：`TestEmbeddedUrlRebuild` 闭包（tag `eur1`）中，`URL$DefaultFactory.createURLStreamHandler` 的 `forName` 站点
  （偏移 162）被记为 unsure。
  - 结果接到所指未知的 Class，没有点名任何类，也没有登记类名模式。
  - 反射缺口里因此出现 `getDeclaredConstructor @ ...createURLStreamHandler@169 <- open(Class)`。
  - 运行期的后果：所有 Handler 的构造器都不在反射面上，重建的 ravacp URL 会报 unknown protocol。
    这个问题同样出现在 DeepCopy / StockTrans / TestSerialDefaultSuid（17b）中。
- 根因：形参槽的名字集由 `param_inputs` 在各调用方帧里以 `Gap::Fail` 求值。
  - 内部求值推不出时（`partial_part` / 常量表部分值），只置引擎级标志 `lookup_incomplete` / `lookup_partial`，
    返回值仍算「推得出」。
  - 标志外泄到外层的 `Gap::Class` 求值，把整个站点记为 unsure。本该得到的是「已知名字 | 受约束的任意串」，
    后者按生成范围内的类名匹配。
- 修正：`param_inputs` 对每个调用方实参求值前清零两个标志、求值后取回并恢复外层值；内部推不出收进本槽的
  `complete = false`。外层按自己的 gap 处理推不出的槽：按名取类为「已知名字 | 任意串」，按名查方法为任意串，
  内部求值 `Gap::Fail` 仍置标志，逐层传递。
- 修正后（tag `eur3`）：站点的模式为 `sun.net.www.protocol.*.Handler`，点名 ftp / jrt / ravacp。
  `ravacp/Handler.<init>` 进入反射成员面，`DefaultFactory` 的缺口消失。

#### 实测

| 用例 | 类 | 方法 | 耗时 ms | 反射缺口 |
|---|---|---|---|---|
| HelloWorld（17b → 18a） | 464 → 464 | 1738 → 1738 | 530 → 536 | 0 → 0 |
| StockTrans（17b → 18a） | 2798 → 2798（−1 / +1） | 17310 → 17314 | 30228 → 31064 | 48 → 47 |
| DeepCopy（17b → 18a） | 2800 → 2800（−1 / +1） | 17323 → 17327 | 30203 → 32462 | 48 → 47 |
| TestSerialDefaultSuid（17b → 18a） | 2805 → 2805（−1 / +1） | 17319 → 17323 | 30856 → 35292 | 48 → 47 |
| TestEmbeddedUrlRebuild（eur1 → eur3，eur1 已含 ravacp、未含分析器修正） | 2867 → 3178（+311） | 16892 → 18993 | 16126 → 21028 | 25 → 26 |

- 四例回归用例：类集只是 `EmbeddedClassPath$Handler` 换成 `sun/net/www/protocol/ravacp/Handler`，方法多出
  `openConnection` / `resourceName` / `copyIndex` 等 6 个。`DefaultFactory` 的缺口在三例大用例中都消失，类集不变。
  耗时差在本机跑批波动范围内（同时有其他重进程持锁）。
- TestEmbeddedUrlRebuild 的 +311 类是**正确性所需**，不是膨胀：
  - 用户以运行期字符串 `new URL(String)` 后调用 `openConnection`。协议推不出，`file` Handler 的 `openConnection` 可达。
  - 其字节码对非本机 host 的 `file://host/path` 执行 `new URL("ftp", host, file)` 并按 FTP 打开（JDK 21 `file/Handler.openConnection@62`），
    因此 `"ftp"` 流入协议形参，ftp Handler → `FtpURLConnection` → 代理时的 `HttpURLConnection` → 认证 / JGSS 可达。
  - 修正前这条链因站点 unsure 被截断：闭包看似更小，但实际运行时 ftp Handler 根本无法构造。
  - 新增的两条缺口（`ProviderList.getMechFactoryImpl` 的 `getConstructor`、`Field.copy` 按名查字段）都位于新可达的 JGSS 代码中，
    属于既有的缺口类别。
- 本地 JDK 21 运行 `TestEmbeddedUrlRebuild`：输出与 `tests/expected/TestEmbeddedUrlRebuild.txt` 一致。

#### 生成器 / 宏修正：scratch 编译暴露的两处（2026-10-07）

`TestEmbeddedUrlRebuild` 的闭包新增 ftp → http → 认证链之后，scratch `cargo check` 暴露出两处既有缺陷。
两处都在生成器或宏中修正，不改生成文件。

1. **E0034：`__as_<SimpleName>` 钩子歧义**（宏，提交 `b2a3e67b`）。
   - `sun.net.www.protocol.http.HttpURLConnection` 继承 `java.net.HttpURLConnection`，两者简单名相同，
     所以两个祖先 vtable trait 都有 `__as_HttpURLConnection`。
   - vtable trait 缺省方法体中的 `self.__as_X()` 因此在 decl 层报方法歧义。
   - 修正：`virtual_dispatch/trait_decl.rs` 改为按本类 trait 限定调用，`<VTable>::__as_X(self)`。
2. **E0277：只经 null 存储到达的根类型槽**（生成器，提交 `5cfe2541`）。
   - 涉及 `NegotiateAuthentication.getCache` 中 `aconst_null; astore_0; …; aload_0; areturn` 这条路径。
   - 模拟阶段按根类型 `Object` 绑定该槽，返回点因此生成 `From::from(local_0)`。
   - 变量提升阶段（`slot_type::merged_slot_type`）随后把「null + 单一引用类型」的槽重定为 `HashMap<Object, Object>`，
     读取点于是失配：`HashMap<String, Negotiator>: From<HashMap<Object, Object>>` 不成立。
   - 修正：`sim::Local` 增加「确定为空」标记。
     - 每次存储都会重置这个标记；只有无类型 null 存入根类型绑定时才置位。
     - 汇合点取各前驱的合取。
     - 回边（尚有未处理的前驱）、异常处理器入口和状态机分派入口一律清除。
     - 读取确定为空的槽时直接产出空字面量，由各消费点按目标类型落为默认值。该处现在生成 `return Ok(Default::default())`。

#### 单元测试与编译（本机，`build/check-target`）

- 闭包 / classfile / resolve / input / emit 各 crate：11 + 193 + 71 + 2 + 19 + 13 + 8 + 1 全部通过（`param_inputs` 修正后）。
- instr / sim / method / emit（null 槽修正后）：71、2、20、2、20、2、1、6，全部通过。
- rava_macros_core 16 通过（2 个 ignored）；rava_macros 0 个测试执行（2 个 ignored）。
- 生成器全量 `cargo test --release`：未跑完，有两例失败，均非本分支引入：
  - `reflect_new_array_element_precision`：TestModuleLayerDefine 种子 2 比种子 0 多 `java/nio/file/Path$1`、`jdk/internal/util/ClassFileDumper$2`。原因是种子 2 下 `ClassFileDumper.enabled` 未折叠为 false，`validateDumpDir` 因此可达。基线 91f54a35 自建二进制跑出同样差异，撤掉 `param_inputs` 修正后也一样，属于既有的顺序依赖。
  - `closure_independent_of_hash_seed`：资源巡检停止了 `TestJndiNoProvider` 的闭包进程（RSS 22G 还在涨），所以失败。同一用例在集成分支 84440f29 的 gate 上也膨胀，问题出在 3f59a8e9..84440f29 之间的合并，已另开任务二分定位。本机实测（8G 上限）：基线 91f54a35 二进制 + 基线 runtime 用时 388 s，未触顶；本分支撤掉 `param_inputs` 修正的二进制用时 466 s，未触顶。
- rava release 构建通过。`TestEmbeddedUrlRebuild --stop-after emit` 生成 3175 个 JDK 类 + 2 个用户类，
  scratch `cargo check --keep-going` 结果为 **rc=0、0 个错误**（修正前只有上述 E0034，修后只有上述 E0277）。

#### 抽查清单（服务器，由用户发起；在 §30.15 清单基础上追加）

- 新增：`TestEmbeddedUrlRebuild`，预期通过（自身 / 内部类 `.class` 以及 `java/lang/String.class` 的 URL 经字符串往返后，
  读到的内容与 `getResourceAsStream` 一致）。
- §30.15 的全部项目：
  - `TestClassResourceStream`、`TestSystemStableProps`；
  - 类路径 / 加载器 / ServiceLoader 各例，以及 `--filter ServiceLoader`；
  - `--filter MethodHandle`、`--filter Proxy`；
  - 四例回归：HelloWorld / StockTrans / DeepCopy / TestSerialDefaultSuid。
- URL 相关用例（受协议处理器查找与分析器修正影响）：`--filter Url`、`--filter URL`、`--filter URI`，
  其中包含 `TestUrlParsingFaces`。
- 反射 / 按名取类用例（受 `param_inputs` 修正影响）：`TestForNameComputedName`、`TestForNameInit`、`TestClassForNameInit`、
  `--filter ServiceLoader`（已含）。

#### 提交

- `7f0bd9ed` resolve：镜像补充目录以目录名为所属模块。
- `47b83880` ravacp 协议与独立 Handler（VM 支持类）+ `EmbeddedClassPath` 改动 + 清单注释。
- `6695d716` e2e `TestEmbeddedUrlRebuild` 与期望输出。
- `fdc94f56` 闭包分析：`param_inputs` 中的推不出收进本槽结果，不再外泄为整站点 unsure。
- `b2a3e67b` 宏：vtable trait 缺省方法按本类 trait 限定调用 `__as_` 钩子（E0034）。
- `5cfe2541` 生成器：局部槽「确定为空」跟踪（E0277）。
- 本节文档：本节之后的提交。

### 30.17 第 14 步：SyspropsLambdaLeak 闭包不收敛——四处成因已修、验收达成（2026-10-07～08，分支 `c1d-url-b2`）

#### 现象

- 单测 `sysprops_lambda_return_confined`（`generator/crates/driver/tests/closure_cli.rs`，fixture `SyspropsLambdaLeak.java`：lambda 经
  doPrivileged 返回 `System.getProperties()` 存入静态字段，系统属性表整片逃逸，断言 `leak["all"] == true`）在本分支跑不完。
- 集成分支 f75c32a4 上同一闭包：sg2 用时 10:45、峰值 7.06 GB（复跑 11:28 / 7.13 GB），终态 lcalls 199k、escaped 10.9k、methods 136.7k。
- 本分支 95b2d84e：540 s 时 lcalls 900k、RSS 9.4 GB 仍在涨，10 GB 上限内跑不完。
- 诊断手段：服务器作业（url2fix-*）给引擎临时打补丁，每 120 s 打印规模（方法 / 对象 / 逃逸 / lcalls 按站点与 lambda 实现分布），
  并在逃逸节点增长处按来源计数（事件数 / 对象数 / 数组数）；补丁不提交。配合 `--flows @grow: / @edge:` 与 `--cut` 定位。

#### 成因 1：同巢内部分配一律沿用属主链（提交 `62097efc`）

- B6 对同类 / 同巢分配一律沿用分配方的属主链。集合借出的拆分器（`ArraySpliterator.trySplit`、`IteratorSpliterator.trySplit`）
  按「分配点 × 属主集合」成倍展开；它们之上的方法克隆与 lambda 调用相乘。
- 修正：只在属主与分配方同巢时沿用（例如树箱属于所在 map）。属主由链首段经类表（`seg_cls`）求出，与求值次序无关。
  递归分配给空链。
- 实测（f1，sg2）：6:48 时 rc=134，峰值 9.4 GB。拆分器展开消失，剩余膨胀的源头换成 VarHandle 读-改-写的手写池（见成因 2）。

#### 成因 2：VarHandle 数值读-改-写访问模式未登记，手写池级联（提交 `a33e6f9d`）

- 涉及 `VarHandle.getAndAdd*` 与 `getAndBitwise{Or,And,Xor}*`（含 Acquire / Release，共 12 项）。
  它们是手写方法，体内取引用元素视图（`_field_read` / `_array_get_and_*`），却没有登记在 `[facts.array_writes]` / `[facts.memory_reads]` 中。
- 后果 1：`hw_writes` 的保守口径生效——每个引用形参都可被其它形参、全部实参数组元素和手写产出写入。
- 后果 2：上调已声明内存语义的成员（`Unsafe.getReference*`）时，`subsumed` 不成立。
  调用方值池因此按 `Feed::N(pool)` 汇合为读写实参，读结果又经池回流。
- 实测（c1，`Socket.getAndBitwiseOrState` 截断诊断）：getAndBitwiseOr 池带来 2062 个对象、16883 个数组的逃逸。
- 修正：清单登记这 12 项为无引用写入（`= {}`）。依据 JDK 规范：数值读-改-写只作用于基本类型载体，
  引用载体抛 `UnsupportedOperationException`，不读写引用字段 / 元素。
- 实测（f2，us1）：5:59 时 rc=134，峰值 9.39 GB。lcalls 在 60 / 180 / 300 s 时为 102k / 177k / 690k，
  escaped 为 3.2k / 15.6k / 22.0k。getAndBitwise / getAndAdd 级联消失。
  逃逸来源第一位换成 `UnsafeStaticObjectFieldAccessorImpl.set@47 → Unsafe.putReference` 的写入（1619 对象 / 14181 数组）。

#### 成因 3：静态字段基址按 open(Object) 建模，Field.set 的汇合值经静态访问器整体逃逸（提交 `50da15df`）

- 探针 h1 / h2（a33e6f9d，sg2）与 h2I（f75c32a4，us1）的结果如下。
  - 写入值：`Field.set` 的 P2 来自 `sun/security/jgss/GSSContextImpl.<init>(GSSContextImpl)@131`，即拷贝构造器
    `for (Field f : GSSContextImpl.class.getDeclaredFields()) f.set(this, f.get(src))`。
    `f.get(src)` 是 `FieldAccessor.get` 在 45 个接收者上的汇合，到达 5149 / 4197 / 2692 … 个类以及 `open(Object)`。
  - 写入目标：`UnsafeStaticFieldAccessorImpl.base` 由构造器 `@10` 的 `unsafe.staticFieldBase(field)` 赋值。
    两个提交上它都是 `open(Object)`：手写 native `staticFieldBase0` 按声明返回类型给出 open。
  - 汇合：`hw_site_fields` 遇到 open 写入目标时把写入值接到 `Esc`，于是整个汇合值集逃逸。
  - 集成分支不出现，是因为 GSSContextImpl 链不可达。§30.16 的 `param_inputs` 修正后，按名取协议处理器的站点不再 unsure，
    ftp → http → 认证 / JGSS 链进入可达（同 §30.16 中 TestEmbeddedUrlRebuild 的 +311 类）。
    系统属性表整片逃逸时，`jdk.reflect.useDirectMethodHandle` 等不折叠，Unsafe 字段访问器同样可达。
    缺陷早已存在，只是本分支的正确可达性把它暴露出来。
- 修正（终态，生成器不含类名）：
  - 清单 `[facts.field_writes] static_bases` 登记 `jdk/internal/misc/Unsafe.staticFieldBase0`；
    引擎为它设返回值模型 `RetModel::StaticBase`，按调用点对字段句柄实参（形参 0）做镜像变换 `MirrorOp::Holder`，规则如下：
    - 字段枚举的来源标记（`Field#<enum:C>`）给出口径类 C 及其超类、超接口的类镜像。
      超类、超接口用于覆盖 `getFields` 的继承公开字段与接口常量。
    - 口径推不出、不是枚举标记的句柄（按名取得等）以及 open，给所指未知的类镜像（裸 `Class`）。
  - 按偏移写入时，基址为类镜像的只落到该类按名打开的静态字段（`mirror_write`）；
    所指未知的类镜像落到全部按名打开的静态字段（`poly_write`）。两者都不再流入 `Esc`。
    读侧经 `static_base_read` 按静态偏移取法门控，比原来的 `open(rt)` 更精确。
  - 规则可靠的依据：HotSpot 的 `staticFieldBase` 即声明类的 mirror（手写实现 `Object::from(f.__get_clazz())`）。
    枚举标记在创建时即登记口径，变换对值集逐元素、单调，结果与次序无关。
  - 补充（`c62a0a6f`）：`hw_exports` 的 `modeled` 判定加入 `returns_static_base`。
    这样 `staticFieldBase0` 的手写值池不再按 open 返回类型交给 `Esc`，因为返回值已按 `StaticBase` 建模。
  - `MethodHandleNatives.staticFieldBase(MemberName)` 不在本次登记范围内。DMH 静态访问器走 `Gate::Handle`，
    MemberName 也没有枚举标记；如要改为所指未知的类镜像，另行评估。

#### 实测（SyspropsLambdaLeak，`rava closure`，ulimit -v 10 GB）

| 提交 | 机器 | 结果 | 用时 | 峰值 RSS | 备注 |
|---|---|---|---|---|---|
| f75c32a4（集成分支，基线） | sg2 | 完成 | 10:45（复跑 11:28） | 7.06 GB（7.13 GB） | lcalls 199k、escaped 10.9k、methods 136.7k |
| 95b2d84e（本分支起点） | sg2 | 未完成 | >540 s | 9.4 GB（540 s 时） | lcalls 900k |
| 62097efc（成因 1） | sg2 | rc=134 | 6:48 | 9.4 GB | getAndBitwiseOr 池级联 |
| 62097efc + 截断诊断 | sg2 | rc=134 | 6:17 | 9.38 GB | getAndAdd 池、静态访问器写入 |
| a33e6f9d（成因 2） | us1 | rc=134 | 5:59 | 9.39 GB | escaped 3.2k → 15.6k → 22.0k |
| 50da15df（成因 3） | sg2 | rc=134 | 6:10 | 9.38 GB | 静态访问器写入逃逸消失；lcalls 60/180/300 s：92.6k/174k/621k，escaped 3.2k/13.5k/18.6k |
| c62a0a6f（成因 3 补：hw 池不交出） | sg2 | 探针 rc=124 | — | — | 只用于 grow 来源归因，见成因 4 |
| 485d30b6（成因 4） | sg2 | 完成 rc=0 | 6:18（elapsed_ms 373582） | 4.50 GB | `all=true`、classes 3499；lcalls 230.8k、escaped 6.6k、method_contexts 130.7k；`fa_untrusted=None`，句柄存取站点 6、名字标记 30、枚举标记 2 |

#### 成因 4（已修，提交 `485d30b6`）：`FieldAccessor.get` 接口汇合点把 45 个接收者的返回值并到每个 `Field.get` 调用点

f3 的逃逸首要来源是 `pool MethodHandle.invokeExact`，共 11892 个数组。其后依次是 `Object[]@24628`、`HashMap$Node.key`、
`putReferenceRelease`、`invokeBasic` 池和 `CHM.put` P2。
lcalls 热点 `UnmodifiableEntrySet.lambda$entryConsumer$0@9` 达 404 克隆 × 160 lambda（253k），基线为 604 × 81（93.6k）。

探针 i1（c62a0a6f）的 grow 归因如下：
- `hub 实参0 FieldAccessor.get on 45 个接收者 ==> P1 MethodHandleObjectFieldAccessorImpl.get`：673。
- `@96 HttpConnectSocketImpl.doTunnel ==> P1 Field.get`：672。
- `hub 返回 FieldAccessor.get ==> @33 / @22 Field.get`：332 / 302。
- 接着是 `String.value`、`sun/reflect/generics/tree/*` 各数组字段、`ArrayList.elementData`、`CHM$Node[]` 元素、`Class[]` 元素等。
  这些值经 `Field.get` 的返回值（R）扩散，每项约 35。

结论：`Field.get(obj)` 的取值走 `getFieldAccessor()` → `FieldAccessor.get` 接口派发。45 个访问器实现汇合成一个 hub，
所有被反射读取的字段值并成一个集合。
这个集合经 `Field.get` 的返回值流向 GSSContextImpl 复制构造等调用方，又经
`MethodHandleObjectFieldAccessorImpl.set` → `setter.invokeExact` 进入 `invokeExact` 手写池，最终交给 Esc。
per-object 精度把每个汇合元素都物化为独立对象和上下文，于是这个汇合乘出了 lcalls 爆炸。

修正（终态，生成器不含类名）：
1. 清单 `vm_intrinsics.toml` 的 `[facts.field_writes]` 新增 `handle_getters` / `handle_setters`，登记 `Field.get` / `Field.set` 及
   8 个基本类型变体；引擎给它们设返回值模型 `RetModel::HandleAccess(写?)`（`engine/field_access.rs`）。
2. 调用边到达这些入口时按接收者分类，命中建模就不再接字节码体（不进 `getFieldAccessor` → `FieldAccessor` hub、
   也不进访问器内部的 `MethodHandle.invokeExact` 手写池）：
   - 枚举标记 `Field#<enum:C>`：类粒度上界——C 及其超类的实例引用字段（对象实参中类属于 C 的抽象对象取 `obj_field`，
     其它值取字段汇总 `F` / 写侧 `U`），C 沿 `holder_chain` 的静态引用字段，以及基本类型字段的装箱类（来自 `[boxing]`）。
   - 名字解析的字段句柄（`getDeclaredField(name)` 等，`name_resolvers` 中 `handle` 项）补字段名标记 `Field#<name:owner.name:desc>`，
     精确到单字段；名字或类推不出时标记为 `Field#<name:?>`，该站点退回字节码接边。
   - 接收者是真实 `Field` 对象或手写层产出的确定句柄值时视为「已由标记覆盖」：句柄身份统一由来源标记承载。
3. 可靠性兜底：字段句柄来源（枚举器 / 名字解析器）若经非字节码调用点（反射、手写体、派发外入口）可达，标记可能缺失，
   引擎记 `fa_untrusted` 并把全部已建模站点整体回放为字节码接边（对象 / 值 / 结果节点补接），结论退回原口径、不丢可达性。
   本例 `fa_untrusted=None`。
4. 成因 3 的 `MirrorOp::Holder` 对名字标记取 owner 的类镜像（`holder_set`），与枚举标记同一套口径。
5. 已知取舍：`Field.get(obj)` 的 obj 实参不再进入访问器体，访问器中只依赖 obj 的路径（类型检查失败的异常消息构造）不被分析；
   这些路径只构造 `IllegalArgumentException` 消息，类已由其它路径可达。

实测（f4，sg2，ulimit -v 10 GB）：6:18 完成、峰值 4.50 GB，`all=true`。比基线 10:45 / 7.06 GB 更快更省；
lcalls 热点 `UnmodifiableEntrySet.lambda$entryConsumer$0@9` 为 97.0k（基线 93.6k，同一量级），
逃逸来源首位换成 `Object[]@24628` 元素（440 对象 / 1888 数组）与 `CHM.get` 返回值，`invokeExact` 池不再出现。

#### 残留与后续

- lcalls 热点 `UnmodifiableEntrySet.lambda$entryConsumer$0@9`：f4 为 97.0k，已回到基线（93.6k）量级。
- 字段标记目前是类粒度上界（枚举标记）+ 单字段（名字标记）；枚举结果按名字过滤后（`f.getName().equals(..)`）再存取的站点仍取类粒度，若后续出现热点再加名字收窄。
- 防回归：可加一道清单审计——上调已声明内存语义成员（`subsumed` 一侧）的手写方法本身也必须登记读写事实。
  未登记时 `hw_writes` 的保守口径会与汇合池相乘（成因 2 即是这种情况）。审计放在 `rava audit native` 中。

#### 待验证清单（合批测试，本分支不自跑）

1. `sysprops_lambda_return_confined`：f4 在 `rava closure` 层已验（6:18 / 4.50 GB / `all=true`）；合批时以单测本身复核断言全部通过。
2. 闭包 per-object 精度单测（ElemTrack / ObjFacts 各例）与 `closure_cli` 全套。
3. 生成器 `manifest` 单测新增 `static_bases_parse`、`handle_access_parse`。
4. `closure_independent_of_hash_seed` / `closure_independent_of_order`：待 dev 恢复后跑。
5. 抽查：§30.16 抽查清单全部项目，另加反射字段读写相关用例（`--filter Field`、`--filter Reflect`、`--filter Unsafe`）、
   `TestJcaSasl`（闭包规模样例，非 e2e；e2e 用 JCA 用例 TestAesGcmRound / TestCipherDesModes / TestMacHmacDigest / TestRsaSignVerify 等代替）与 JGSS / HTTP 认证链用例。成因 3 改变了静态字段基址的读写口径，成因 4 改变了 `Field.get/set` 的建模口径
   （重点看反射拷贝构造、`AtomicXxxFieldUpdater`、序列化 `ObjectStreamClass` 字段读写、注解代理等经 `Field` 存取的用例）。

### 30.18 合批 batch-1008 的语义合并（2026-10-08）

batch-1008 依次并入 user-unreach-stubs（108558e0）、c1d-url-b2（3391eb2d，04600e21）、boot-image-s4（c614f840，b52917c9）、
enum-values-direct（59451c29）、closure-composition（c3a06331），合并修复 b558e0c2。c1d-url-b2 与 boot-image-s4 两侧
同改分析器核心，冲突取舍如下（合并提交信息为准）：

- **Facts 两侧字段并存**：`level`（引导求值档位上下文）与 `objs`（c1d 形参抽象对象集）同时在 `Facts` 上。共享分析复用键含
  对象参数集；档位上下文不复用（`engine/worklist.rs`：`closing || level.is_some()` 时不查共享摘要）。
- **facts 拆分**：`engine/facts.rs` 拆为 `facts/{kinds,fields,calls,oracle}.rs`，本批改动移入子模块；`calls.rs` 的
  `CallInfo.nonnull_ret`（含调用者类镜像非空）取代原 `empty`。
- **`invoke_result` 先后**（`facts/oracle.rs`）：`level_queries` 档位折叠 → 清单事实 / 空集合 / `nonnull_ret` 等 →
  偏移判定（`field_offset`）→ 类字面量接收者 `mirrors_call` → 派生结果。`Oracle` 新增 `key_getter`、`string_equality`、
  `param_mirror_call`。
- **absint 拆分**：`absint.rs` 拆为 `value` / `oracle` / `step` / `fixpoint` 子模块（final 字段复读单测拆至
  `final_field_tests.rs`）。定点循环每条指令的次序：`eq_operands` → `tr.pre` → `step` → `final_reread`。条件分支：
  类型收窄 `instanceof_narrow` / `mirror_sub_narrow` 之后 `.or_else(key_test)`，可空性 `null_narrow` 之后
  `.or_else(affix_narrow)`，另算 `final_null_test`。
- **`named_resources`**：分析器按名求出的资源只并入档案侧（模块资源）推导；用户侧推导传空集，并剔除档案已有资源
  （`input/src/build.rs`）。
- **删除「按名开放字段」回退**（取 c1d 撤除兜底，§30.14），本批侧的 `analyzed_exact` 守卫随之删除。
- **`PV::join`**：两侧确定非空 → 非空引用；合并时与 `join_ret` 同时生效，b558e0c2 将 `join_ret` 吸收进 `PV::join` 后删除
  （单测改为 `join_keeps_nonnull`）。
- **映像必需**：`BuildInput.boot_image` 改为必需的 `ImageData`（去 Option）；`BuildInput.system_properties`（`SysPropFacts`）
  与 `input/manifest.rs` 的 `boot_init_classes`（随 `[boot_init]`）删除。closure.json 的 `system_properties` 保留。
- **合并修复 b558e0c2**：引导求值的类路径运行模型与常量格合流——c1d 把 `toFileURL` 的延迟调用改由 `null_returns` 给出，
  s4 起引导求值不取有字节码方法的返回值事实，二者叠加使 initPhase2 执行 `toFileURL` 字节码遇宿主延迟值、映像求值失败；
  `[concrete.boot.natives]` 对 `URLClassPath.toFileURL` / `BuiltinClassLoader.findClassOnClassPathOrNull` 显式 `const:null`。

**验证现状**：首次验证（6934dc93）抽查 41/41 因映像求值失败、单测 closure `--lib` 11 失败、driver 层 32 失败；
b558e0c2 之后映像求值通过，但 HelloWorld emit 内存超限（峰值约 11.9G）。

**待补：根因与修复。**

## 31. C1d 余项：门排名复测与来源归因（2026-10-09，分支 `c1d-rest`，基于 a6dca5c0）

目标口径：DeepCopy ≤ 2803、StockTrans ≤ 2807、TSDS ≤ 2809、HelloWorld 468。本节只做排名、归因与一处通用小步，
不碰在跑线（日志链 logchain3、具体求值缓存组 bootcache、注解签名 annot-sig、转译耗时 perf-regress）。

### 31.1 实测基线（`rava closure`，`closure_composition_job.sh`）

| 作业 | 提交 | 用例 | 类 | 方法 | 备注 |
|---|---|---|---:|---:|---|
| cr-m1-a450ee23 | a450ee23 | DeepCopy | 3757 | — | 单次约 15 min（耗时回归由 perf-regress 修） |
| cr-m1-a450ee23 | a450ee23 | DeepCopy + `logging` 切除集 | 3703 | — | 日志链只占 54 类 |
| cr-m1-a450ee23 | a450ee23 | HelloWorld | 3456 | — | |
| cr-g3-a6dca5c0 | a6dca5c0 | DeepCopy + `logging`（门排名基线） | 3703 | 21355 | 与 a450ee23 相同：31.4 的小步类数 0 变化 |

结论：日志链（logchain3 的范围）不是 DeepCopy 的主体。即使切掉日志链，DeepCopy 离目标仍差约 900 类。

### 31.2 门排名（cr-g3-a6dca5c0，DeepCopy + `logging` 切除，`--gates-verify 3 --gates-timeout 2400`）

| # | 门 | 首达树 | 单切 Δ 模型 | 实测 | 类别 |
|---:|---|---:|---:|---:|---|
| 1 | `AccessController.executePrivileged(PrivilegedAction,…)` 方法体 | 1322 | 210 | 923（→ 2780） | 转发方法：Δ 是全部调用点下游之和，不对应单一机制 |
| 2 | `Charset.lookup`（引导区根 `Charset.isSupported #@level:0`） | 2050 | 290 | 超时 | 构建期可求值；`sun/nio/cs/ext` 279 |
| 3 | `StandardCharsets.charsetForName` | 63 | 101 | — | 同上链 |
| 4 | jar `JarURLConnection.getInputStream` | 609 | 96 | — | 精度缺口；贪心第 4 步再接 `CertificateFactory.generateCertificate` +189 |
| 5 | `executePrivileged(PrivilegedExceptionAction,…)` | 1055 | 50 | — | 转发方法 |
| 6 | `DeepCopy.deepCopy` | 989 | 47 | — | 用户合法语义（序列化） |
| 7–8 | `SPILocaleProviderAdapter.findInstalledProvider` / `$1.run` | 763 | 14 | — | |
| 10 | `Formatter$FormatSpecifier.print` | 1860 | 4 | — | 区域内部互为替补（§16） |

贪心 15 步累计（模型）−1201 类。可归到一个机制、且不与在跑线重叠的来源只有两块：

- **字符集（#2/#3，约 390 类）**：引导区以未知名字调用 `Charset.isSupported`，查找放开到全部扩展字符集。
  要收窄，须先有 U1 的属性取值决定，再加字符串常量上下文（§13、charset-build / charset-ext 线）。本线不做。
- **jar URL（#4 + 证书链，约 285 类）**：见 31.3。

### 31.3 jar URL 的真实来源：`URI.toURL` 慢路径（cr-d1 / cr-d3-a450ee23 诊断）

`ServiceLoader$LazyClassPathLookupIterator.parse(URL)` 的实参是两个抽象对象：`URL@25827:123` 和 `URL@25827:238`
（`URL.of(URI, null)` 在 `URI.toURL@2` 上下文中的两个分配点）。`@path` 反向链如下：

```
SystemModuleFinders$SystemModuleReader.find → JNUA.create("jrt", "/" + module + "/" + name)   // URI(String,String) 构造
  → BuiltinClassLoader.findResource(ModuleReference,String)@64  u.toURL()
  → URL.of(uri, null)
      @123 快路径：handler == null && "jrt".equals(scheme) && !isOpaque && rawAuthority == null && rawFragment == null
           → new URL("jrt", host, port, file, null)              // 协议是常量，键控查找只给 jrt Handler
      @238 慢路径：new URL(null, uri.toString(), null)           // spec 形状未知，键控查找放开 → jar Handler
  → checkURL → BuiltinClassLoader$1.next → CompoundEnumeration → nextProviderClass@168/@173 → parse
```

- 闭包里存活的 `ModuleReader` 只有 `SystemModuleReader` 和 `NullModuleReader`（`instantiated` 实查）。
  所以运行期到达这里的 URI 全是 `jrt` 方案：scheme 常量、path 非空、authority / fragment 从不写入。
  实际执行只走 @123，@238 在这条链上不可达。
- 分析器走到 @238 的原因是 URI **不是抽象对象**：`@objs:P0 java/net/URI.toURL` 为空。
  `container_shape` 只把泛型容器和持有键类字段的类（如 URL 的 `handler`）建成抽象对象，URI 两样都不是，
  于是 `uri.getScheme()` 等读的是 `URI.scheme` 的全局值集（Top），快路径条件判不定。
- 终态能力（记为 ④，归 §30 的 URL 精度线）：
  1. 把「实例字段经读取方法流入键控查找键（`[facts.keyed_lookups]` 的 `key`）」的类也建成按对象的抽象对象，
     使 URI 按分配点分开。判定按字节码，不列类名。
  2. 用 P3（按接收者对象的返回值）求 `getScheme` / `isOpaque` / `getRawAuthority` / `getRawFragment`，
     再用 `string_equality` 判定 `"jrt".equals`。
  3. 解决 `owild` 阻塞：反序列化出的 URI 由 `readObject` → `parse` 经字节码写 `this.authority` 等字段，
     接收者是序列化类 id，不是抽象对象，写入并入 `owild[字段]`。按对象读的答复含 `owild`，所以仍是 Top。
     需要把「接收者只可能是反序列化分配对象」的写入从 `owild` 中分离出来（反序列化对象与抽象对象不相交，
     见 `obj_fields.rs` 健全性第 3 条的论证），否则 ① ② 落地后收益仍为 0。
  - 预期收益：jar `JarURLConnection` / `JarVerifier` / PKCS7 / X500 / 证书工厂链。贪心模型合计约 285 类，
    实测待能力落地后再测。jar `Handler` 本身仍因 `URLClassPath.<init>` 无条件构造而留在闭包里（§30.15）。

### 31.4 小步：确定字符串的引用相等折叠（a450ee23 + 单测 4afa42b6）

`absint.rs::ref_eq`：两个内容不同的确定字符串（`V::Str`）一定是不同对象，`if_acmp` 按「不等」折叠。
内容相同时不断言是同一个对象，因为非字面量来源可能是副本。

- 动机：a5-4e 的 `ConditionalSpecialCasing` / 大小写转换链里有 `locale.getLanguage() == "tr"` 一类的引用比较。
- 实测：DeepCopy（含 `logging` 切除）3703 → 3703，0 变化。原因是 `toUpperCase` 系列按上下文不敏感分析，
  `Locale` 语言值在合流后已经是 Top，到不了两侧都是确定串的形态。
- 规则本身健全且通用，保留。

### 31.5 Formatter 区域（排名 #10，首达 1860）

- 诊断：`Formatter.parse(String)` 在全部上下文中形参值都是 Top（含 `logging` 切除）。
  `FormatSpecifier.print` 读的 `dt` 值集为 `{0, 1}`，所以 `printDateTime` 进入的 Calendar → SPILocaleProviderAdapter
  → ServiceLoader 链保持可达。
- 单靠「`charAt` 结果属于常量串字符集」不够：`%<s` 走正则回退 `FormatSpecifier(String, Matcher)`，
  其中 `dt` 由 `m.start(5)` 决定。收窄必须按常量格式串具体求值 `parse`，并在 `print` 上按接收者对象分开读字段
  （§18.5 / §20.5 的具体求值器路线）。这属于多步设计，本线不展开。

### 31.6 作业清单

- `cr-g3-a6dca5c0`（kr2）：门排名，见 31.2。
- `cr-m1-a450ee23`：类数，见 31.1。
- `cr-d1-a450ee23` / `cr-d3-a450ee23`（jp2）：URL 与 Formatter 诊断，见 31.3 / 31.5。
- `cr-u1-4afa42b6`（jp2）：`cargo test --release -p closure --lib absint`，40 个通过、0 失败（含新增 `distinct_string_constants_ref_ne`）。

## 32. 能力③：常量格式串的具体求值与按对象读说明符字段（2026-10-10，分支 `c1d-fmt`，基于 9a48ca32）

目标：格式串里没有日期转换时，`FormatSpecifier.print` 不再到达 `printDateTime` → Calendar →
SPILocaleProviderAdapter → ServiceLoader 链（§31.5）。只用通用的字节码建模，不对 Formatter 做特判。

### 32.1 现状与阻塞

`Formatter.parse(String)` 已经是具体求值入口（`[concrete] entries`）。要收窄，以下两个条件必须同时成立：

1. **档案里每个 parse 调用点都具体求值成功。** 只要有一个调用点回退抽象调用边，抽象的 `parse` / 说明符构造器
   就会以 Top 格式串执行：`dt = true` 的写入经字节码 `putfield` 并入通配值（基本类型字段不拆接收者），
   每个说明符读到的 `dt` 都是 `{0, 1}`。
2. **`print` 按接收者对象读 `dt`。** 改造前，具体求值的结果对象按**类型**代表（`TypeSet::exact(类)`），
   字段值写进全局值集和通配值。即使各调用点都成功，只要档案里某个常量格式串含日期转换（日志的
   `SimpleFormatter` 缺省格式就有），全局 `dt` 仍是 `{0, 1}`。

### 32.2 机制 B：结果按对象物化（`[concrete] object_results`，第 1 步，已实现）

- **清单**：`object_results` 列出结果按对象物化的入口（须同在 `entries`）。首个入口是 `Formatter.parse`。
- **按对象的类**（`pobj_types`）：分析开始前扫描各入口**自身字节码**里的 `new`，得到这些类。
  对 parse 而言，就是说明符、定长串和列表三个类。列表也因此按对象物化：迭代时按接收者对象读 `elementData`，只取到本次解析出的说明符。这个集合在分析期间固定不变，所以按对象读的门与处理次序无关。
- **物化**（`concrete/apply.rs`）：结果快照里属于这些类的实例（不含数组和 lambda）各成一个抽象对象，
  名字为 `类@concrete:<散列>:<快照序号>`。散列取「入口 + 实参组合 + 冷热」的内容，因此与求值次序无关；
  同一组实参在不同调用点得到同一组对象。
  - 对象登记进 `objs` 与 `obj_chain`。
  - 快照字段值记入 `ovals[(对象, 字段)]`，引用值进对象字段节点 `O(对象, 字段)`。
    这些值不并入 `owild`，也不进 `U`；全局值集 `fvals` 照常并入，供不按对象读的读者使用。
  - 对象记入 `osnap`。快照含全部实例字段，所以按对象读不再并入初值。
- **写入分流**：轨迹另记 `shared_puts`，即接收者不是本次求值新分配对象的写入（静态字段、映像对象、
  此前求值的对象）。按对象物化的入口只把 `shared_puts` 并入 `owild`。
  - 新分配对象上的写入：进了结果的对象由快照按对象记录；没进结果的对象程序看不到。
  - 同一次物化里仍按类型代表的结果对象（不在 `pobj_types` 中的类的实例）不是抽象对象，不会出现在按对象读的对象集里
    （`node_objs` 要求值集全由抽象对象组成），所以它们的写入不并入 `owild` 也是健全的。
  - 其他入口的行为不变。
- **读取**：`obj_param_set` 的门从「声明类型是容器形态类」放宽为「容器形态类，或在 `pobj_types` 中」。
  `print` 按接收者对象克隆（`recv_ctx`），克隆内 `this` 的值集就是该说明符对象，`getfield dt` 读到的是
  「该对象的快照值 ⊔ owild」。

**单调性**：
- `pobj_types` 在分析前确定，门不随分析变化。
- 新增对象只让值集增长，按对象读的复核机制不变（`obj_grown` / `odeps`）。
- 具体求值结果按 (入口, 实参) 记忆。对象名只取决于内容，同一组实参重复应用时幂等（`applied`）。
- `osnap` 只增不减，且只登记快照对象。快照字段集是全部实例字段，所以「不并初值」恒成立，
  不受构造器摘要作废（`odef` 清空）的影响。

**健全性**：
- 对一个物化对象的写入有三种来源，各有覆盖：
  1. 求值期间的写入：由快照最终值覆盖；
  2. 求值之后程序对它的字节码写入：接收者值集里含该抽象对象，照常按对象记录；
  3. 未知接收者的写入：对象逃逸后经 `U` 与 `owild` 覆盖。
- 求值期间对其他既有对象的写入经 `shared_puts` 进入 `owild`。

### 32.3 机制 A：字符串链按调用点克隆（视诊断决定是否需要）

如果诊断显示 parse 调用点回退的原因是「实参不可枚举」（形参被 Top 污染），或「组合数超过 64」（全部调用点的
常量格式串汇合），就需要给「字符串形参原样转给具体求值入口字符串形参」的转发链按调用点克隆，
使污染的调用链与常量调用链分开。形式与选择子形参（`selector.rs`）相同：
- 掩码按字节码判定，并沿 static / special / 直接实例调用递归；
- 上下文按调用点链命名，深度有界。

具体是否需要，见 32.4 的诊断。

### 32.4 诊断：回退调用点的污染来源（`fmt-d1-0dfce4dd` / `fmt-d2-78614c31` / `fmt-d3-78614c31`，sg2）

诊断手段（均为通用诊断，不含类名特判）：
- `--flows @taint:<方法模式>`：列出匹配方法节点上被污染的形参槽，逐跳回溯污染来源，直到第一个不是透传的来源。
  - `taint_site` 处，实参若来自调用方已被污染的形参槽，记为透传，链因此可以继续回溯。
  - 调用点上下文名 `@<方法>:<偏移>` 会解码出方法标签。
- 回退行带上下文标签：`回退（<上下文>）：…`。

**结论**：档案里只有一个 parse 调用点回退，它的格式串是**真实的运行期未知值**，不是建模缺陷。

- 回退点是 `Formatter.format(Locale,String,Object[])@11`，所在上下文的格式串形参被污染。
- 污染链：
  `Formatter.format(Locale,…)` 槽 2 ← 透传 `Formatter.format(String,…)` 槽 1
  ← 透传 `String.format` `#@5006:54` 槽 0 ← 实参 `Site(13)`（`SimpleConsoleLogger.format`）。
  - `@5006:54` 就是 `SimpleConsoleLogger.format`。
  - 格式串来自 `getSimpleFormatString()` → `Formatting.SIMPLE_CONSOLE_LOGGER_FORMAT`，它的值是
    `getSimpleFormat("jdk.system.logger.format", …)`。
  - 也就是说，格式串是运行期系统属性的值。属性缺省时取 `DEFAULT_FORMAT`，即 `"%1$tb %1$td, %1$tY %1$tl:%1$tM:%1$tS %1$Tp …"`，**本身就含日期转换**；实参 1 是 `ZonedDateTime.now()`。
  - 构建期引导映像按 U1 不钉系统属性，所以这个值只能是 Top。
- 这条路径在档案里真实可达（`--why`）：
  `Thread.start0`（引导映像根）→ `Signal$1.run` → `Terminator$1.handle` → `Shutdown.exit` → `Shutdown.logRuntimeExit`
  → `System.Logger.log` → `SimpleConsoleLogger.log/publish/format`。
  - 要不要真的记日志，取决于运行期属性 `jdk.system.logger.level`（`isLoggable(DEBUG)`）和运行期装载的 LoggerFinder，构建期折不掉。
  - 每个程序的引导段都含 `Terminator` 信号处理器，所以**每个档案都有这条链**。
  - DeepCopy 另有一条首次到达的路径：`ObjectInputFilter$Config.<clinit>@114` 的 `System.Logger.log`。反序列化过滤器配置要记日志，级别同样取决于运行期属性。
- 同一条路径还独立带进了 §31.5 的其他目标：
  - **ServiceLoader**：首次到达是 `ZonedDateTime.now` → `Clock.systemDefaultZone` → `ZoneId.systemDefault` → `TimeZone.toZoneId`
    → `ZoneId.of` → `ZoneRegion.ofId` → `ZoneRulesProvider.<clinit>@45`（`ServiceLoader.load`），与 Formatter 无关；
  - **SPILocaleProviderAdapter**：经 `printInteger` → `localizedMagnitude` → `getZero` → `DecimalFormatSymbols.getInstance`
    → `LocaleProviderAdapter.forJRE`，不经 `dt`，任何 `%d` 都会走到；
  - **Calendar**：经 `printDateTime@34`（日志路径必经），也经 `TimeZone.getDisplayName` → CLDR → `MessageFormat`
    → `SimpleDateFormat`（`%Z` 等转换）。

### 32.5 结论与实测

**能力③的收窄目标在 HelloWorld / DeepCopy 上无法健全达成。** 根因是 32.4 中的运行期退出日志链。
只要它在档案上，`printDateTime` → Calendar 链和 ServiceLoader、SPILocaleProviderAdapter 都是真实可达的。
「格式串无日期转换就不到 `printDateTime`」这条在**单个调用点**上成立，但档案按并集计算，其中必有这个日期格式调用点。

实测（`rava closure`，sg2）：

| 用例 | 基线 9a48ca32（类 / 方法） | 机制 B 0dfce4dd（类 / 方法） |
| --- | --- | --- |
| HelloWorld | 3407 / 18453 | 3407 / 18453 |
| DeepCopy | 3727 / 21500 | 3727 / 21500 |

- 其余 parse 调用点都具体求值成功（每个上下文 4–9 组常量）。
- 机制 B 物化出的说明符对象按对象读 `dt` 时，仍会读到 `owild` 里的 `true`。来源是日志调用点上的抽象 parse：
  它的说明符按类 id 代表，不是抽象对象，基本类型字段写入一律并入通配值。
  `--why printDateTime` 首次经由的就是一个 `@concrete` 说明符对象。
  即使把这一处也分开，`printDateTime` 仍会经日志调用点到达，所以数字不会变。

**机制 B 的去留**：保留。理由如下：
- 它是常量格式串按对象读说明符字段的终态形式，健全性论证见 32.2；
- 门在分析前确定，不引入次序依赖；
- 在不含退出日志链的构建单元上，或者将来日志链被健全地排除后，它直接生效。

当前所有档案都含这条链，所以它对闭包规模的实际收益为 0。这一点在这里如实记录。

**余项（需用户决策 / 另立）**：
1. 只有一个办法能真正让 `printDateTime` / Calendar / ServiceLoader 出闭包：判定退出日志链（`Shutdown.logRuntimeExit` → `System.Logger`）不可达，
   或者把它的格式串、级别在构建期钉住。
   - 这要改 U1「属性运行期取宿主值」的口径（例如把 `jdk.system.logger.level`、`jdk.system.logger.format` 划入构建期钉值的属性，同 U14），是语义决定，不属于分析器精度问题。
   - 钉住以后，`isLoggable(DEBUG)` 恒假，整条日志链出闭包。ServiceLoader 的首次到达经 `ZonedDateTime.now`，但可能还有其他路径，是否随之退出需实测。
2. 机制 C（未做，规模收益 0，不建议单独做）：按对象类（`pobj_types`）在抽象执行中的 `new` 也建抽象对象，基本类型字段写入按接收者拆分。
   这样能消除抽象 parse 对具体说明符 `dt` 的污染，但会改 `bytecode.rs` 的字段写入路径（与 c1d-uri 同区），且受第 1 项支配。
3. 机制 A（字符串链按调用点克隆）不需要：回退原因不是污染串扰，而是真实未知值。

### 32.6 次序依赖回归、逐组应用与停用（2026-10-10 续，分支 `c1d-fmt`）

**回归**：机制 B（42e25cf4）令 HelloWorld `closure_independent_of_order` 失败（`fmt-o-78614c31`；基线 9a48ca32 HelloWorld 通过，只有既有的 DeepCopy FindOps 差异）。
`fmt-od-78614c31` 对照缺省次序与 batch 1 / seed 0：缺省次序多出 `ArrayListSpliterator.tryAdvance@48 → DistinctSpliterator.accept` 等流水线派发目标。
根因：parse 调用点先应用了若干常量组合，之后污染到达，调用点不可撤回地回退。回退前已应用的组合留下各自的按对象结果，而哪些组合先到达取决于处理次序。
机制 B 之前，这些结果是类 id 代表，会被回退后的抽象结果吸收；按对象物化后不再被吸收。

**修正（5dbc0bdc、1a64da69）**：对按对象物化的入口，调用点回退后仍逐组应用实参中的已知常量组合（`concrete_known`）。
- 接收者只取所指已知的镜像；引用实参取字面量、形参常量和 null，不看污染。
- 失败的组合，或结果引用映像容器的组合，逐组跳过；每个 (调用点, 实参) 只处理一次（`partial_tried`）。
- 应用集合由此恒为终态的已知常量组合，与次序无关。
- 残留的次序依赖只在原始类型实参由常量变为非常量时出现（回退前应用过的整型组合撤不回），parse 无此形态。

**新问题：内存**。同步 batch-1009d（logchain3：退出日志链 `isLoggable(DEBUG)` 构建期折叠）之后，在 dev 上实测：

| 用例 | 基线 589052eb（类） | 本分支 8850f928 / 1a64da69（机制 B + 逐组应用） |
| --- | --- | --- |
| HelloWorld | 577 | 577 |
| DeepCopy | 3727（561 s，峰值 9.0 GiB） | 超 14 GiB 被 OOM 杀（700–755 s） |

- logchain3 后 HelloWorld 的退出日志链已出闭包（32.5 余项 1 由日志链缺口 ③ 以构建期折叠健全解决，不需改 U1）。HelloWorld 不论是否开机制 B 都是 577，机制 B 在其上无增量。
- DeepCopy 在逐组应用不设组合上限时内存爆。`partial_tried` 去重后依旧，所以内存不在重复处理上，而在组合数，以及每组物化的抽象对象与按对象字段值。
  - 未确认的一点：只开机制 B、不开逐组应用时 DeepCopy 是否也超限。旧基线上机制 B 未超（sg2 无内存上限）。

**处置（0ad9f474）**：`vm_intrinsics.toml [concrete] object_results` 暂置空，机制 B 与逐组应用的代码保留，但不生效。闭包行为回到基线。
dev 实测（`fmt-b2-589052eb` / `fmt-g5-0ad9f474`）：

| 用例 | 基线 589052eb（类 / 方法） | 0ad9f474（类） |
| --- | --- | --- |
| HelloWorld | 577 / 1896 | 577 |
| DeepCopy | 3727 / 21509 | 3727（563 s，峰值 9.0 GiB） |

- 基线 HelloWorld 闭包已不含 Calendar / SPILocaleProviderAdapter / ServiceLoader（logchain3 所致）。DeepCopy 三者仍在。Calendar 的首次到达是 `Preconditions.outOfBoundsMessage@338` → `String.format` → `Formatter.format(Locale,…) #@level:2@89` → `FormatSpecifier.print@11` → `printDateTime@34`。
  这条路径上的格式串都是常量，但到达 parse 时处在 `@level:2` 截断上下文，值已合并。这里正是机制 B 与逐组应用的用武之地，需先解决内存问题再验证。
- 闭包单测 240 通过。

**接手方向**：
1. 先实测 DeepCopy 只开机制 B（逐组应用关掉）时的峰值，定位内存来自组合数还是物化对象数（`--flows @concrete` 的「回退后应用已知常量组合」诊断行数）。
2. 若来自组合数：按入口给逐组应用设终态可判定的上限。超限时的处理要与次序无关，例如该调用点整体不做按对象物化，结果改按类 id 代表，回退前已物化的对象也并入代表（需要撤回机制）。
3. 若来自物化对象：同一说明符类、同一字段值的对象按值合并（对象名改按字段值散列而非组合序号），对象数以不同说明符形态为上界。
4. 恢复 `object_results` 后重跑 HelloWorld / DeepCopy 闭包计数与 `closure_independent_of_order`。
