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
