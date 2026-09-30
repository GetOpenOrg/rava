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
