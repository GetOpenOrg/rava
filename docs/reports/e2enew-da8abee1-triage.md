# e2enew-da8abee1 失败归因（69 例）＋ 6 例输出不符 expected 复核

> 日期：2026-10-03（任务 #2 + #6 合一交付）
> 数据源：`server_maintenance/rava/test_results/spot/e2enew-da8abee1/`（8 服务器 spot 跑批，
> git da8abee1、JDK 21、117 新增例中失败 69、通过 48）
> 本地复核环境：worktree `feat/e2e-failure-triage`（main @ 6c0adc44）、JDK 21.0.11（macOS）、
> golden 环境（DisableIntrinsic + en_US + UTC）复刻 run_tests 口径。

---

## 一、总览：失败 × 类型 × 模块档

| 失败类型 | 例数 | A 档 | B 档 | B4(beans) | 存量目录 |
|---|---:|---:|---:|---:|---:|
| 运行错误（panic） | 54 | 11 | 12 | 3 | 28 |
| 编译错误（rustc） | 8 | 3 | 1 | — | 4 |
| 输出不符 | 6 | 1 | — | — | 5 |
| 转译错误 | 1 | — | 1 | — | — |

模块分布（新增例）：62_reflection 17、71_xml 10、70_crypto_ec 4、74_beans_geom 3、
64_charsets_ext 3、72_http 3、66_logging 3、65_locale_data 2、69_zipfs 2、
67_sql 1、73_jndi_script 1；其余 20 例落在存量目录（52/44/30/35/17/53 等，多为第八轮方法级补齐件）。
闭包规模：新增例 extra 集中在 800–1100 类（logging 族最大 ~1600），未见失控扇出
（jmod 计划 §六「超过 2000 类说明来源」红线未触）。

## 二、任务 #6：六例输出不符的 expected 复核结论

**expected 纠正数 = 0**。六例在本机 JDK 21 golden 环境双跑全部稳定且与 expected 逐字一致——
错不在期望文件；1 例为环境依赖，其余 5 例为生成器侧缺陷（差异片段摘自服务器日志）：

| 用例 | 结论 | 差异/依据 |
|---|---|---|
| TestClassCastSubclass | **生成器缺陷** | `arr-covariant` expected=true 实际=false——`Object[].class.isAssignableFrom(Integer[].class)` 未实现**数组协变** |
| TestClassModuleFace | **生成器缺陷（语义未建模）** | `jbase-named=false / jbase-name=null / exported-internal=false`——模块系统元数据（isNamed/getName/isExported）缺失，java.base 呈未命名空模块形态 |
| TestInvokeNullArgs | **生成器缺陷** | `argc-iae / type-iae` 两行缺失——`Method.invoke` 对实参数量/类型不符**未抛 IllegalArgumentException**（catch 未触发） |
| TestSetAccessibleBoundary | **生成器缺陷（语义未建模）** | `jdk-open=unexpected`——对 java.base 私有字段 `setAccessible(true)` 不抛 InaccessibleObjectException（JDK 强封装边界未建模；与模块元数据同根） |
| TestLocaleCurrency | **依赖环境** | `¥→CN¥`、`¥9,999.50→CN¥ 9,999.50`——**CLDR 数据随 JDK 小版本漂移**（本机 21.0.11=¥，服务器 OpenJDK build=CN¥）。expected 单点生成不可移植；生成器无责（两侧数据源各自的 jmod 即如此）。建议后续把 CNY 符号断言从源码弱化或按 JDK build 分层（超出本任务范围，附于 §五 建议） |
| TestSystemStableProps | **生成器缺陷** | `sys-stream-ex=NullPointerException`——`ClassLoader.getSystemResourceAsStream("java/lang/String.class")` 返回 null（系统资源流对 JDK 自身类不可用，与 ClassResourceStream 同族） |

**本地当前 HEAD（6c0adc44）复现验证：5/5 全部复现，失败形态与服务器逐例一致**
（diff 行数相同：ClassCastSubclass 2 行 / ModuleFace 8 行 / InvokeNullArgs 2 行 /
SetAccessibleBoundary 2 行 / SystemStableProps 2 行）——五个生成器缺陷在 da8abee1
之后的提交中均未修复，根因族定级有效。

## 三、根因族归并（13 族，按影响面排序）

| # | 根因族 | 典型 stub / 错误 | 影响例数 | 域 |
|---|---|---|---:|---|
| R1 | **xml 内部 lambda 存根** | `jdk/xml/internal/SecuritySupport.lambda$getSystemProperty$0` | **10**（71_xml 全部） | 闭包分析器（lambda 体可达性） |
| R2 | **泛型反射 scope 构造存根** | `sun/reflect/generics/scope/AbstractScope.<init>` + E0432 | 6（GenericSuperclass/TypeVariables/TypesDeep/NestingFamily/MemberModifiers/OwnerType） | 手写/边界清单（构造器准入？） |

> **R2 实施要点（refl-fix A 族，2026-10-05）**：scope 存根已随边界收窄消解，现症为 `Class.getGenericSignature0` 手写恒返回 null（旧「已知偏差」）——类级泛型签名缺席，getGenericSuperclass/Interfaces 退回 Class、getTypeParameters 为空（AIOOBE / Mismatch of count / NPE 同根）。
> 修法：元数据新增类级表 `CLASS_SIGNATURE`（扫描已有的 `#[generic_signature]` 块属性，档案侧 + 用户侧同形），native 按表返回真实签名；消费方 sun/reflect/generics 走字节码翻译。
> MemberModifiers 余项：`TypeVariableImpl.getBounds` 的 `value instanceof FieldTypeSignature[]`（静态类型 `Object[]`）被生成器编译期折叠为假——数组源未考虑协变，bounds 不被具化、clone 后转 `Type[]` 抛 CCE（toGenericString 吞异常成 `<CCE>`）。修法：数组源对引用元素数组 / 非数组目标改按运行时类判定，仅基本元素数组互斥才折叠（全档案 instanceof_fold 3→0）。

| R3 | **JCA 服务查找存根** | `sun/security/jca/ProviderList.getServices` + `GetInstance.getServices` | 5（crypto_ec 4 + RsaSignVerify） | 手写/边界（ProviderList 是 VM 驱动域） |
| R4 | **logging 模块 import 断链** | rustc E0433（找不到类型） | 4（logging 3 + ResourceBundleFaces） | 生成器 import（跨模块 jmod 类名解析） |
| R5 | **反射 invoke 参数校验缺失** | 无 IAE 抛出（见 §二） | 1（InvokeNullArgs；属 R5 语义族） | 反射 L3 校验 |

> **R5 实施要点（refl-fix B 族，2026-10-05）**：native invoke（Method / Constructor 两条 NativeAccessor）直通 `reflect_invoke`，缺 HotSpot `Reflection::invoke_method` 的前置校验——实例方法空接收者 NPE、实参个数 IAE、引用实参类型 IAE 均未抛，类型错的实参落进分派臂的 `From<Object>` 被静默转换或 panic（OverloadResolution 同根）。
> 修法：`reflect_dispatch::native_invoke` 按描述符先校验（失败置 bad_arg，不包 InvocationTargetException），再进分派；四个 native accessor 统一改走它。

| R6 | **模块元数据/强封装未建模** | 见 §二 ModuleFace/SetAccessible | 2 | 模块系统建模（boot layer 相邻） |
| R7 | **系统资源装载** | getSystemResourceAsStream null / findBootstrapClassOrNull / ClassResourceStream | 3 | 资源装载通道（K10 前奏） |
| R8 | **beans finder 构造存根** | `com/sun/beans/finder/InstanceFinder.<init>` | 3（beans 全部） | 边界/手写清单 |
| R9 | **charset/zipfs 提供者构造存根** | `StandardCharsets$Classes.<init>` / `java/nio/file/FileSystem.<init>` | 5（charsets 3 + zipfs 2） | SPI 装载路径（A 档第 1 步依赖） |
| R10 | **数组反射 native** | `java/lang/reflect/Array.set` native 存根 | 1（ReflectArrayDeep） | native 准入表 |
| R11 | **L3 桥分派闭包缺席** | `Comparable.compareTo:(Object)` 分派未覆盖 | 1（InterfaceMethodReflect） | 闭包分析器分派 |

> **R11 实施要点（refl-fix C 族，2026-10-05）**：根因不在闭包——桥方法（ACC_BRIDGE）不发射为 Rust 方法，方法元数据与分派臂里都没有它，`getMethod("compareTo", Object)` 落到 Comparable、`isBridge` 计数为 0。
> 修法：类头发 `#[bridge_method(.., bridge_to = 真实描述符)]` 行（宏忽略），元数据扫描收为方法行（修饰符 bridge = 0x40，与已发射的协变桥 wrapper 同签名去重），分派按桥描述符键入真实方法臂。

| R12 | **E0308 编译错**（三处不同面） | ComparatorNullsFirst / JndiNoProvider / StreamTerminalEdges | 3 | 生成器类型发射（record/泛型推断待抽查） |
| R13 | **http async 转译错** | TestHttpLoopbackAsync transpile fail | 1 | 转译器（CompletableFuture 链待抽查） |

未单列：AnnoDeepAccess / ProtectionDomainFaces（`Class$Holder.allPermDomain` 存根）/ 
OverloadResolution 等 4 例杂项 run error，归入 R2/R3 邻域待逐例细看；
LocaleDateCjk（run error）与 R-CLDR 环境相邻，待日志细读。

> **ProtectionDomainFaces 实施要点（refl-fix C 族）**：`Class$Holder` 随外层 VM 边界类 Class 截断，allPermDomain 存根——纯 Java 静态状态，入 `translate_nested`；应用类的 `getProtectionDomain0` 按内建加载器定义类路径落地（ucp 首条目 CodeSource → `SecureClassLoader.getProtectionDomain`），codesource 非空与 JDK 一致。


## 四、jmod A/B 档结论（对应计划 §七 实测列）

- **A 档 17 例：全灭**（charsets 3 / locale 2 / logging 3 / sql 1 / random 0 / zipfs 2 / crypto 4 + sql 的 RowSet 1）。
  主根因 R8/R9（提供者构造存根）+ R3（JCA）+ R4（import 断链）——与 jmod 计划预判一致
  （第 1 步依赖 boot layer 与 SPI 装载路径），**用例本身无需修改**，失败即预审登记。
- **B 档 16 例：xml 10 全灭于 R1 单一根因**；http 3（含转译错 1）；jndi 1（E0308）；
  rowset 1。xml 修复 R1 后有望整档解锁。
- **B4（beans）3 例**：R8 单根因。
- §七 实测列已按本报告回填（闭包类数 = dyn-compare extra，native-missing/boundary-stub
  见 R1–R11 对应族）。

## 五、建议的 tasks.md 条目（由用户合入）

```
- [ ] R1 xml lambda 存根：jdk/xml/internal/SecuritySupport.lambda$getSystemProperty$0 可达性（10 例整档，71_xml）
- [ ] R2 泛型反射 scope：sun/reflect/generics/scope 构造器存根与 E0432（6 例，62_reflection）
- [ ] R3 JCA 服务查找：ProviderList/GetInstance 存根处理（5 例，crypto_ec+RSA）
- [ ] R4 跨模块 import 断链：java.logging 模块类 E0433（4 例）
- [ ] R5 反射 invoke 实参校验：数量/类型不符须抛 IAE（InvokeNullArgs 2 行）
- [ ] R6 模块元数据建模：isNamed/getName/isExported + 强封装边界（2 例）
- [ ] R7 系统资源装载：getSystemResourceAsStream/findBootstrapClassOrNull/Class 资源（3 例）
- [ ] R8 beans finder 构造存根（3 例）；R9 charset/zipfs 提供者构造（5 例）——A 档第 1 步前置
- [ ] R10 Array.set native 准入；R11 compareTo 桥分派闭包；R12 E0308 三例抽查；R13 http async 转译错
- [ ] 环境：CLDR 随 JDK build 漂移（TestLocaleCurrency）——语料断言弱化或按 build 分层
```

R1–R4 合计 **25/69（36%）**，四个根因各为单点修复可整族解锁；与 T2/from_any 在途域
重叠的部分（R2/R5/R11）建议并入其队列，其余为 jmod 第 1 步（A 档）的入场排期依据。
