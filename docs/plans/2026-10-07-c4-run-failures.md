# C4 全量运行期失败分诊与修复（c4-runfix，基于 rust-closure-analyzer 3f59a8e9）

> 范围：C4 全量中不在 Python 基线、属合法 Java 测试的运行期失败。分支 `c4-runfix`（不合入集成分支，C4 冻结）。
> 抽查一律在服务器上跑（`distribute_tests.py --spot`），本机只做 cargo check / 相关单测 / `--stop-after emit` / `--why` / `--flows`。

## 一、逐例结论

| 测试 | 根因 | 修复 / 登记 | 抽查 |
|---|---|---|---|
| TestSaxLocatorAttributes / TestSaxNamespaceCallbacks / TestDomBuildTree / TestXmlDomParse / TestXmlFactoryConfigs / TestXmlXPath / TestXmlStax | FactoryFinder 按名取类站点的名字含推不出的支（系统属性 / 服务文件读出的名字）时整个站点作废，缺省实现类常量名（`…jaxp.SAXParserFactoryImpl` 等）也不放行 | a5de7eee：按名取类站点名字含推不出的支时，已知名字照常在不动点上放行；JCA 实例化宿主（`[jca] instantiation_hosts`）的类名实参按推不出处理（由 JCA 规则按请求算法补种），防注册表全部算法实现类经被调方站点入链 | c4run-xml-a5de7eee：7 例全过 |
| TestXmlSaxEvents | 工厂缺省实现同上已修；随后暴露下一层：`MissingResourceException: Could not load any resource bundle by com.sun.org.apache.xerces.internal.impl.msg.XMLMessages`（解析错误消息按名装载资源束类） | 资源束按名装载未建模，与 TestRowSetProvider（已登记）同根因，登记已知 | c4run-xml-a5de7eee：ubuntu 失败（签名如左） |
| TestPropertiesXmlRoundTrip | ① CJK 键 String.hashCode NPE（c4-regress 1b096880 已修，TestStringHashUtf16 同根）；② `Properties.store0` 局部 `entries` 的 LVT 声明为 `Collection`，首值 `entrySet()` 为子接口 `Set`、后值为 `ArrayList`；生成器的「再赋值接口声明变量取载体」规则跳过接口值，按 `Set` 定型，ArrayList 被包成 Set 视图，`Collection.iterator` 派发落空 → AbstractMethodError | a927148c：再赋值的接口声明局部首值为子接口时同样按声明接口载体定型（`sim/src/store/align.rs`） | c4run-props-4981c057（c4-runfix + c4-regress 合成抽查分支 `c4-runfix-chk`）：本例与 CollectorsDemo / DeepCopy / TestStringHashUtf16 全过 |
| TestSetAccessibleBoundary | Class.getModule 手写近似使 java.base 类落无名模块，checkCanSetAccessible 放行 | b3c2bf8c 登记已知，归引导映像第 4 步 | — |
| TestBeansPropertyEditor | 见第二节 | 方案 A 实施（分支 `c4-beans-precision`：3005a63d 引用选择子按 null 分调用点上下文 + 清单 `Class.newInstance` 入 `constructor_lookups`；第二步按名取类名字求值补合流 / 取后缀 / 基本类型镜像取名），已知失败登记已删 | 待抽查 |
| TestUrlParsingFaces | 见第三节（产品取舍，停在该项） | — | c4run-url-3f59a8e9（us1）同样失败，非 ubuntu 脏工作区所致 |

## 二、TestBeansPropertyEditor：构造器查找值集不齐全（10-07 用户定方案 A，已实施）

**根因**：`PropertyEditorManager.findEditor(int.class)` 返回 null，用户随后 `intEd.setAsText` NPE。
`PropertyEditorFinder.<init>` 以 ldc 把预置编辑器（`com/sun/beans/editors/IntegerEditor` 等 8 个）登记进 `WeakCache`，
`find` 先 `instantiate(predefined, null)`，未命中再按名 `instantiate(type, name)`（`ClassFinder.findClass`）。
两次调用在同一接收者对象上下文里合流（`--flows`）：`InstanceFinder.instantiate@29` 的 `Class.newInstance` 接收者 =
{8 个编辑器镜像 + String / Level / 基本类型镜像} ∪ `open(Class)`（按名取类结果，名字含推不出的支）。
`Class.newInstance` 只是成员枚举（`getDeclaredConstructors0`，open 接收者记反射缺口），构造器查找（`constructor_lookups`）
又只在值集齐全时点名 JDK 类——编辑器镜像所指类停在 type 级，从未实例化。`LevelEditor` 能工作是因为它经按名取类站点点名。

**已实测否决的方案**：把 `Class.newInstance` 列入 `constructor_lookups` 并让不齐全值集里的已知镜像照常点名（同 a5de7eee 对按名取类的口径）——
本例闭包 +10 类（编辑器入链），但全局代价不可接受：DeepCopy / TestSerialUserGenericCallbacks +1062 类（序列化构造器查找把流不敏感池里
全部可序列化镜像——含 xerces DOM、JCA 实现类——点名），其余用例普遍 +10（KeyStore 条目类）。这正是 `constructor_lookup` 文档所述
「不齐全时已知部分只是合流带进来的镜像」的情形，口径保持不变（改动已回退，未提交）。

**选项**：

- **A. 调用点敏感 + null 分支剪枝**：`instantiate(predefined, null)` 调用点上 `name` 为常量 null，`if (name != null)` 分支不可达，
  该调用点上下文里 @29 接收者只有注册表镜像（齐全）→ 照常点名。需要分析器对「形参为 null 常量」的调用点分化上下文并剪枝，
  通用、精度不降，工作量中等（上下文选择策略 + 折叠联动）。
- **B. 只对实例化入口（`instantiators`，`Class.newInstance`）放宽齐全要求**：实现最小，但与 `getDeclaredConstructor().newInstance()`
  语义等价却口径不同，属特判，且 JDK 内其他 `newInstance` 站点的代价未量。
- **C. 维持现状登记已知**（已登记 `docs/known_failures.toml`）。

**建议**：A（终态口径：靠精度区分真实目标，不放宽齐全判定）。

**实施（10-07 用户选定 A，分支 `c4-beans-precision`，基于 rust-closure-analyzer 84440f29）**：

1. 分析器（3005a63d）：选择子形参（`selector.rs`）由「int 族形参作 switch / 条件跳转键」扩到「可空性未知、直接作
   ifnull / ifnonnull 操作数的引用形参」，判定扩到实例方法，静态转发链同时认引用形参原样转发。字节码调用点在
   被调方引用选择子形参上传 null 时，被调方按调用点克隆（虚分派 `dispatch_one` / 非虚 `edge_recv` 经
   `recv_call_ctx`，静态经 `selector_ctx`）。克隆用新的「形参常量上下文」（`ctxsel.rs` `const_ctx`）：只分开方法节点，
   堆上下文取外层上下文（`ctx_heap`），克隆体内分配与不克隆时同一抽象对象；按（堆上下文, 调用点）命名，递归链上有界。
   `instantiate(predefined, null)` 克隆里 `name` 形参常量为 null，`if (name != null)` 支剪去，@29 接收者只剩注册表镜像。
2. 清单：`Class.newInstance:()Ljava/lang/Object;` 入 `[facts.reflect] constructor_lookups`（即 `getDeclaredConstructor().newInstance()`，
   同口径；齐全判定不放宽）。克隆里 @29 值集齐全，预置编辑器构造器按已知集合点名。

**过程中否掉的两版**（均未提交）：
- 克隆沿用 `site_ctx_in`（调用点为堆上下文链首）：类集不变，但方法集 +15～+41（克隆体内分配按调用点而非接收者命名，
  不同接收者经同一调用点的容器对象合流，`List.forEach` / `LoggerWrapper.log` 等多出目标）。改为堆上下文取外层后方法集逐项一致。
- 形参常量上下文按外层上下文全名命名：外层本身是克隆时名字逐层延长，递归调用链上上下文无界——本机 beans 闭包 22 分钟、
  内存 30GB（协调者已停）。改为按堆上下文命名后有界。

**闭包对照**（本机 `rava build <T> --stop-after emit --closure-json --clean`，单例、8GB 内存上限；基线 = 84440f29 构建）：

| 用例 | 基线类数 | 仅分析器（3005a63d） | 分析器 + 清单 | 方法数 基线 → 终 | 方法上下文 基线 → 终 |
|---|---|---|---|---|---|
| TestBeansPropertyEditor | 2940 | 2940（类 / 方法集一致） | 2950（+10） | 17522 → 17601 | 70648 → 78446 |
| HelloWorld | 469 | 469（一致） | 469（一致） | 1828 → 1828 | 4056 → 4405 |
| CollectorsDemo | 2914 | 2914（一致） | 2924（+10） | 17465 → 17513 | 70841 → 78381 |
| DeepCopy | 3195 | 3195（一致） | 3205（+10） | 20460 → 20507 | 95359 → 104694 |
| TestSerialUserGenericCallbacks | 3198 | 3198（一致） | 3208（+10） | 20451 → 20498 | 95520 → 104838 |

- 预置编辑器入链：`com/sun/beans/editors/{Boolean,Byte,Double,Float,Integer,Long,Short}Editor` 全部实例化，`<init>` / `setAsText` 等进闭包
  （beans 方法 +79 / -0：`com/sun/beans/editors` 18 个、keystore 相关 45 个、其余为解析路径）。
- 四个对照用例 +10 类全部相同：`java/security/KeyStore$Entry$Attribute`、`java/security/PKCS12Attribute`、
  `sun/security/pkcs12/PKCS12KeyStore$`{`CertEntry`,`Entry`,`KeyEntry`,`PrivateKeyEntry`,`SecretKeyEntry`}、
  `sun/security/provider/JavaKeyStore$`{`KeyEntry`,`TrustedCertEntry`}、`sun/security/tools/KeyStoreUtil`。来源是清单一项，不是分析器改动：
  `sun/security/util/KeyStoreDelegator.engineLoad` 等 4 处以 `Class.newInstance` 实例化其 `primaryKeyStore` / `secondaryKeyStore`
  字段（值集齐全：`JavaKeyStore$JKS` / `PKCS12KeyStore` 镜像，`DualFormatJKS.<init>` 的 ldc）。原先这两个 keystore 类停在 type 级、
  构造器不在闭包——`KeyStore.getInstance("JKS").load(...)`（`AnchorCertificates` 读 cacerts 即走此路）运行期会落存根，属漏闭包，
  补上的是 keystore 加载体（条目类 / PKCS12 属性）。与第二节「已实测否决的方案」的 +1062 不同：那是放宽齐全判定所致，本方案不放宽。
- 闭包耗时（`summary.elapsed_ms`，本机同期有其他重进程，仅供参考）：beans 21.3 s → 25.4 s、DeepCopy 47.9 s → 64.5 s；
  方法上下文数稳定在 +9%～+11%。

**第二步：按名取类的名字求值（beans-ff651aca 抽查 11/12，beans 仍 NPE）**

抽查与 dev 诊断作业（`beansdiag-ff651aca`）定位：`int` 编辑器已正常，NPE 在 `findEditor(String.class)` 返回 null 后的
`strEd.setAsText`（用户第 50 行）。`com/sun/beans/editors/StringEditor` 不在 `PropertyEditorFinder` 预置表里，只经
`InstanceFinder.find` 的按名支取得：`name = type.getName() + suffix`；`idx = name.lastIndexOf('.') + 1`；
`if (idx > 0) name = name.substring(idx)`；再对 `packages` 各前缀 `instantiate(type, prefix, name)` →
`prefix + "." + name` → `ClassFinder.findClass` → `Class.forName`。原名字求值在三处推不出，`ClassFinder` 站点名字集为空、
只剩「任意串」支（只匹配闭包里已有的类），`StringEditor` 永不入链：

1. 合流值（`name` 在 `if` 后是拼接结果与截取结果的合流）：原先多来源值整体推不出。现各来源各成一支（`Part::Alt`），
   字面量来源成字面量；拆支嵌套至多 2 层（值经循环回到自身必经合流，保证终止）——`engine/name_ops.rs` `phi_parts`。
2. 取后缀：清单 `[facts.string_concat] suffixes` 新增 `String.substring:(I)Ljava/lang/String;`。接收者候选能拍平时，
   起点为 int 常量取该后缀，否则取各候选的全部后缀（超集；候选名最终只保留类路径上存在的类）——`name_ops.rs` `suffix_part`。
3. 基本类型镜像取名：`type` 值集含基本类型镜像（`findEditor(int.class)`，各基本类型共用一个抽象镜像）时
   `getName` / `getSimpleName` 原先整体推不出；现取全部基本类型关键字（超集）——`name_eval.rs` `mirror_name`。

`ClassFinder.findClass@11/26/35` 的名字集由空集变为含 `com.sun.beans.editors.StringEditor`（及 `TestBeansPropertyEditor$LevelEditor`、
`intEditor` 等不存在的候选，解析时丢弃）。分析器不含类名，三项都是通用名字求值能力。

**第二步闭包对照**（同上口径，基线 = ff651aca）：

| 用例 | 类数 ff651aca → 第二步 | 方法数 | 方法上下文 |
|---|---|---|---|
| TestBeansPropertyEditor | 2950 → 2967（+17） | 17601 → 17702 | 78446 → 80798 |
| HelloWorld | 469 → 469（类 / 方法集一致） | 1828 → 1828 | 4405 → 4405 |
| CollectorsDemo | 2924 → 2940（+16） | 17513 → 17612 | 78381 → 80721 |
| DeepCopy | 3205 → 3225（+20） | 20507 → 20637 | 104694 → 107764 |
| TestSerialUserGenericCallbacks | 3208 → 3228（+20） | 20498 → 20628 | 104838 → 107944 |

- beans 的 +17 中 `com/sun/beans/editors/StringEditor` 是目标；其余 16 类四个用例共有，来自同一个此前推不出的按名取类站点
  `SPILocaleProviderAdapter$1.run@77`：`Class.forName(SPILocaleProviderAdapter.class.getCanonicalName() + "$" + c.getSimpleName() + "Delegate")`。
  `getCanonicalName` 的返回值是合流（数组 / 局部类 / 顶层类各支），原先整体推不出，名字只剩「任意串 + `$` + 简单名 + `Delegate`」，
  只匹配闭包已有类（无）；现顶层类支解析出 `sun.util.locale.provider.SPILocaleProviderAdapter`，点名 12 个
  `SPILocaleProviderAdapter$*ProviderDelegate`，及其 `addImpl` 带入的 `JRELocaleProviderAdapter$AvailableJRELocales`、
  `java/util/stream/DistinctOps`（+3 个内部类）；DeepCopy / Serial 另有 `Nodes$CollectionNode`、`ReduceOps$4`（+1）、
  `StreamSpliterators$DistinctSpliterator`。这是可达站点由「推不出」变为「解析出」的漏闭包补全：用户经 ServiceLoader 装了
  `LocaleServiceProvider` 实现时，原闭包运行期落 ClassNotFoundException。若要收回，正途是 ServiceLoader 遍历按档案内
  服务提供者集折叠（无提供者时循环体不可达），属另一项精度改进，不在本修复内。

## 三、TestUrlParsingFaces：URL 协议处理器按包前缀装载（10-10 已按 A 实施，c4-url）

**根因**：`URL$DefaultFactory.createURLStreamHandler@162` 以 `"sun.net.www.protocol." + protocol + ".Handler"` 按名取类，
`protocol` 来自 URL 字符串的字符循环解析，推不出；分析器对该站点生成受约束模式 `[Lit("sun.net.www.protocol."), Wild, Lit(".Handler")]`，
按现行口径只匹配**已在闭包中**的类（`class_patterns`），`https.Handler` 无其他入链路径，故不入闭包，运行期 `unknown protocol: https`。

**选项**：

- **A. 模式匹配类路径（开放世界）**：受约束模式对档案类路径匹配，命中的类全部入链。与「档案在开放世界下计算」一致，用户无需配置；
  代价是凡触达 `new URL(String)` 的程序都带上 http / https / ftp / mailto / jmod / jrt 等全部协议处理器及其依赖（https 牵出 SSL / JSSE 栈），
  闭包与二进制体积上升（未实测，需按三测点量）。
- **B. 构建配置声明协议集**（同 GraalVM `--enable-url-protocols`）：缺省只含 file / jar / jrt，用户项目配置追加；
  闭包最小，但违背「开发者继续写 Java、无需额外配置」的产品定位，配置缺失时运行期才失败。
- **C. 从流入 URL / URI 构造的字符串字面量推导 scheme**：通用性差——协议解析是字符循环，需在分析器里对 URL 解析做语义建模，等于为单一 JDK 类写特判，违反「生成器不写类名特判」。

**建议**：A（正确性优先、与档案开放世界口径一致），同时以档案规模衡量：协议处理器只在 `URL(String)` 解析可达时入链，
体积代价按三测点实测后决定是否对 https 的 SSL 栈做按需拆分。需用户 / 协调者拍板，本项停在此处。

**实施（2026-10-10，分支 c4-url）**：按 A 实施，但以候选模式形状限定范围——首段是非空字面量（固定包前缀）的
任意串候选按类路径匹配、命中全部入链；首段即任意串的候选仍只匹配闭包内类名（不限定命名空间，按类路径匹配会膨胀）。
实现 `class_lookup.rs::anchored` / `classpath_matches`（同一模式只扫描一次类路径）。闭包对照（main 583edccd → 2122af05）：
HelloWorld 580 → 580；DeepCopy 3028 → 3033（ftp / http / https / jmod / mailto 五个 Handler）；TestUrlParsingFaces
3404 → 5123（+1719，新增站点只有 DefaultFactory@162，点名 https / jmod / mailto；其余为 https.Handler 经
`URL.openConnection` 分派可达后带入的 SSL 默认上下文（JCA，约 835）、HttpsURLConnectionImpl（约 210）、
certpath LDAP → JNDI → RMI 注册表服务（约 310）与序列化 / Proxy（约 106））。体积代价属 https 支持本身；收窄途径是
`URL.handler` 按对象区分处理器（字段按分配点敏感），另立精度项。

## 四、提交与抽查

| 提交 | 内容 |
|---|---|
| b3c2bf8c | 已知失败登记：TestSetAccessibleBoundary |
| a5de7eee | 闭包：按名取类站点名字含推不出的支时已知名字照常在不动点上放行；JCA 宿主类名实参按推不出处理；`closure_cli::unsure_lookup_releases_known_names` |
| a927148c | 生成器：再赋值的接口声明局部首值为子接口时按声明接口载体定型 |
| ce7bda6c | 已知失败：删除 TestDomBuildTree |
| （本文提交） | 已知失败登记 TestXmlSaxEvents / TestBeansPropertyEditor；本文档 |

抽查：
- `c4run-xml-a5de7eee`：TestSaxLocatorAttributes / TestSaxNamespaceCallbacks / TestDomBuildTree / TestXmlDomParse / TestXmlFactoryConfigs / TestXmlXPath / TestXmlStax 通过；TestXmlSaxEvents 失败（资源束，已登记）。
- `c4run-props-4981c057`（`c4-runfix-chk` = c4-runfix a927148c + c4-regress 6ad12e1d 的合成提交，仅供抽查，不合入）：TestPropertiesXmlRoundTrip / CollectorsDemo / DeepCopy / TestStringHashUtf16 通过。
- `c4run-url-3f59a8e9`（us1）：TestUrlParsingFaces 同样失败（非 ubuntu 环境问题）。

本地精度对照（a5de7eee 前后闭包类数）：TestSaxLocatorAttributes 2929→3501、TestSaxNamespaceCallbacks 2930→3502、TestDomBuildTree 2924→3144（新入的为 Xerces 实现类）；
HelloWorld 469、CollectorsDemo 2914、DeepCopy 3195、TestModuleLayerDefine 3076、TestRowSetProvider 2958、TestSerialUserGenericCallbacks 3198 不变。

## 五、未完成项

- TestUrlParsingFaces：待决（第三节，建议 A，需实测体积）。
- TestBeansPropertyEditor：方案 A 与第二步已实施（第二节），待服务器抽查运行通过；+10 KeyStore 类、+16～20 SPI 委托 / 流类均为可达站点的漏闭包补全，请协调者确认是否接受。
- TestXmlSaxEvents / TestRowSetProvider：模块资源束按名装载的分析器建模，另立任务。
- TestPropertiesXmlRoundTrip 依赖 c4-regress（1b096880）先合入集成分支；a927148c 与之合并后该例通过。
- a927148c 改动生成器存储对齐，影响面只在「接口声明、区间内再赋值、首值为子接口」的局部；抽查四例通过，C4 解冻合入前随全量再验。
