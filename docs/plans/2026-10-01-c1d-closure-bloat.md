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

