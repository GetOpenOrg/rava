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
| TestBeansPropertyEditor | 见第二节 | 登记已知（原则取舍，停在该项） | — |
| TestUrlParsingFaces | 见第三节（产品取舍，停在该项） | — | c4run-url-3f59a8e9（us1）同样失败，非 ubuntu 脏工作区所致 |

## 二、TestBeansPropertyEditor：构造器查找值集不齐全（待决）

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

**建议**：A（终态口径：靠精度区分真实目标，不放宽齐全判定）。需协调者确认后另立任务，本项停在此处。

## 三、TestUrlParsingFaces：URL 协议处理器按包前缀装载（待决）

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
- TestBeansPropertyEditor：待决（第二节，建议 A）。
- TestXmlSaxEvents / TestRowSetProvider：模块资源束按名装载的分析器建模，另立任务。
- TestPropertiesXmlRoundTrip 依赖 c4-regress（1b096880）先合入集成分支；a927148c 与之合并后该例通过。
- a927148c 改动生成器存储对齐，影响面只在「接口声明、区间内再赋值、首值为子接口」的局部；抽查四例通过，C4 解冻合入前随全量再验。
