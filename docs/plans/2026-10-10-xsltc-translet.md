# XSLTC translet：运行期定义的类（预定义类方案，fix-xsltc2 已实施）

> 分支 fix-xsltc（基线 batch-1010h 873b30f3）。起因：e2e `71_xml/TestXmlTransform` 运行期失败，
> `TransformerConfigurationException: Translet class loaded, but unable to create translet instance.`
> （failure_patterns `xsltc-translet-create`）。本文是分诊结论与终态方案。10-10 用户采纳 S2（§6），fix-xsltc2 按 §5 实施，实施记录见 §8。

## 0. 结论

- **失败类别**：运行期定义类这一能力在 rava 里没有终态承载。不是生成器翻译缺陷、闭包遗漏或反射覆盖问题。
- **失败点**：`TemplatesImpl.defineTransletClasses` → `TransletClassLoader.defineClass(byte[], pd)` →
  `ClassLoader.defineClass1`（native，手写 `runtime/java_runtime/src/java/lang/class_loader_impl.rs`）。
  该手写对格式合法的类文件一律抛 `LinkageError("…: runtime class definition is not supported by the native image …")`。
  `defineTransletClasses` 的 `catch (LinkageError e)` 把它包成 `ErrorMsg.TRANSLET_OBJECT_ERR`，就是用例看到的消息。
- **终态方案（建议）**：建立**预定义类**机制。构建期取得程序运行期会定义的类文件，像用户类一样翻译进用户 crate；
  运行期的类定义 native 按字节内容查登记表，命中就返回静态链接的类，未命中按现状抛 `LinkageError`。
  需要用户定的是构建期字节的来源（§3，决定点 X1）。

## 1. 分诊

### 1.1 cause 链

JDK 21 源码（`TemplatesImpl.java` 458–545 行）：

```java
for (int i = 0; i < classCount; i++) {
    _class[i] = loader.defineClass(_bytecodes[i], pd);   // → ClassLoader.defineClass1（native）
    ...
}
...
catch (ClassFormatError e) { → TRANSLET_CLASS_ERR }
catch (LinkageError e)     { → TRANSLET_OBJECT_ERR }       // "Translet class loaded, but unable to create translet instance."
```

`getTransletInstance` 里反射实例化失败（`InstantiationException` 等）也报同一消息，但走不到那一步：
rava 的 `defineClass1` 对完整类文件无条件抛 `LinkageError`（`_define_class_error` 的 `ClassFileShape::Complete` 分支）。
XSLTC 产出的类文件格式合法（JVM 上能定义），不会落到 `ClassFormatError` 分支。

服务器探针（作业 `xsltc-probe2`，dev，873b30f3）：用例同款代码，捕获异常后按 `getCause()` 打印链。结果见 §1.4。

### 1.2 旁证：run.log 没有 `Caused by`

`TransformerException` 把 cause 存在自己的 `containedException` 字段并覆盖了 `getCause()`，
`Throwable.cause` 字段仍是未设置哨兵（`this`）。rava 的未捕获异常报告（`runtime/java_runtime/src/error.rs`
`report_uncaught_in`）直接读 `cause` 字段，所以不打印 `Caused by`。JVM 的 `printStackTrace` 走虚调用 `getCause()`，会打印。
这是报告器的保真缺口，与本失败无关，记为 §7 遗留项。

### 1.3 XSLTC 产物的性质（本机 JDK 21.0.12 实测）

| 项 | 实测 |
|---|---|
| 本用例样式表产出的类 | 1 个：`die.verwandlung.GregorSamsa extends AbstractTranslet`，3925 字节，major 45 |
| 确定性 | 同一进程两次编译、两个进程各编译一次，SHA-256 相同（`0b8b7f20f2cb8801…`） |
| 类名 | 缺省固定为 `die.verwandlung.GregorSamsa`：**不同样式表产出同名、不同内容的类** |
| 静态字段 | `_sNamesArray` / `_sUrisArray` / `_sTypesArray` / `_sNamespaceArray`，只在 `<clinit>` 写常量 |
| 依赖 | 只引用 java.xml 内部类（`AbstractTranslet`、`BasisLibrary`、DTM 迭代器、`SerializationHandler` 等） |
| 字节码形态 | 无 `jsr` / `ret`；无 StackMapTable（major 45）。rava 的方法翻译不依赖 StackMapTable（`classfile/src/class.rs` 不解析该属性） |

fixdomd-b4da5107 抽查的动态对照里，`[miss]` 16 条都出自 `AbstractTranslet.transform` 调到的 DTM / xsltc 迭代器类。
这些类只被 translet 的代码使用，translet 不在闭包里，它们自然也不在。这说明预定义类必须作为闭包的根参与分析（§4.2）。

### 1.4 探针结果

作业 `xsltc-probe2`（dev，873b30f3；探针 `TestXsltcProbe` 只在服务器检出里临时写入，不入库）。转译 3m55s、编译 10m28s、运行 1.6 s，输出：

```
factory=com.sun.org.apache.xalan.internal.xsltc.trax.TransformerFactoryImpl
identity=[<root><a x="1">one</a><b>two</b></root>]
templates=com.sun.org.apache.xalan.internal.xsltc.trax.TemplatesImpl
chain[0]=javax.xml.transform.TransformerConfigurationException: Translet class loaded, but unable to create translet instance.
chain[1]=java.lang.LinkageError: <Unknown>: runtime class definition is not supported by the native image (class universe is fixed at build time)
```

- identity 变换（不经 translet）正确；`newTemplates` 在 rava 里**完整跑完了 XSLTC 编译**（得到 `TemplatesImpl`），说明
  java_cup 语法分析、BCEL 生成整条链的翻译是对的；
- 失败恰在 `defineClass1`，cause 即上述手写抛出的 `LinkageError`（XSLTC 传入的类名为 null，故显示 `<Unknown>`）。

## 2. 现有同类机制为什么不能复用

运行期类定义点（手写准入第 ② 类）现在有三种承载，都假设「被定义类的行为在构建期由 rava 自己知道」：

| 机制 | 被定义的类 | 承载 |
|---|---|---|
| lambda / indy、LambdaForm | 形状由 JDK 规则决定 | `[indy]`、原生 LambdaForm 解释器 |
| `Proxy$Dyn`、`Species_Dyn`、`InjectedInvokerDyn`、`SerializationConstructorAccessorDyn` | 一个通用支持类就能表达全部实例 | `runtime/java_support/` 的 Java 源，`[facts.reflect.defined_classes]` 把定义点映射到支持类 |
| `ClassLoader.defineClass1/2` | 任意字节 | 只做格式检查，合法即 `LinkageError`（TestDefineClassRejects 覆盖） |

translet 的语义全在样式表编译出的字节码里，没有一个通用支持类能表达它，所以不能套 `Proxy$Dyn` 一类的做法。
要执行 translet，只有两条路：构建期拿到字节码并翻译，或者运行期解释字节码。

## 3. 方案比较

公共部分是承载：**按内容寻址的预定义类**（§4）。各方案的区别只在构建期字节从哪里来。
运行期总是重新执行真实的生成代码（XSLTC 照常编译样式表），再用生成出的字节去查表，
所以字节来源的猜测不影响正确性：猜错或漏了，结果只是未命中、抛 `LinkageError`，和现在一样。

### S1：构建期求值（rava 具体引擎在构建期执行生成调用）

做法：分析器在档案调用链上找「输入在构建期可确定」的生成调用（本例 `tf.newTemplates(new StreamSource(new StringReader(常量)))`），
由具体引擎（`engine/concrete`，即引导映像求值器）在构建期执行，在类定义 native 处截获字节。

- 优点：不依赖外部运行；与引导映像同一套引擎；字节来自同一份参考 JDK 字节码，与运行期逐字节相同。
- 缺点：
  1. **选点没有通用规则**。字节由 XSLTC 整条编译链（JAXP 工厂查找 → SAX 解析 → java_cup 语法分析 → BCEL 生成）算出。
     要先认出 `newTemplates(src)` 这一层调用是生成点，再判定接收者（`TransformerFactory.newInstance()`，读系统属性与
     `jaxp.properties`）和实参对象图（`StreamSource` → `StringReader` → 字符串常量）都可在构建期求值。
     现有常量实参求值（`consteval.rs`）的规模上限是 256 条指令、深度 3，量级差得很远。
  2. **引擎能力缺口未知**。XSLTC 编译涉及 ThreadLocal、`ResourceBundle` 错误消息、SAX 工厂服务查找、`jdk.xml.*` 安全限制属性。
     具体引擎至今只跑过 initPhase1–3 与 `<clinit>`（引导映像计划 §5.8，隔离规则严格：跨类静态只读 init_only 字段等），
     能否跑完一次 XSLT 编译要先做探针才知道。
  3. **覆盖面窄**。主流框架的样式表来自资源文件（资源已构建期嵌入，原则上可求值）；但同属这一类的 CGLIB / ByteBuddy
     （Spring AOP、Hibernate 代理）的输入是运行期扫描到的 bean 类，构建期通常算不出。
- 规模：选点规则、引擎补能力、截获字节三部分，估计与引导映像第 6 步（非引导类构建期初始化）相当。

### S2：训练运行记录（建议）

做法：在参考 JDK 上运行程序，用 JVMTI 代理（`ClassFileLoadHook`）记录所有经 `defineClass` 一类入口定义、
且不在类路径 / 模块中的类文件，按内容哈希落盘，作为构建输入。

- 优点：
  1. 对一切运行期字节码生成器通用（XSLTC、CGLIB、ByteBuddy、Javassist、Groovy），不需要按生成器写规则，
     生成器代码里也不出现类名。
  2. 参考 JDK 与 javac / jmods / golden JVM 同源（`tools/refjdk.toml`），记录的字节与 rava 运行期重新生成的字节逐字节相同（§1.3 已验证确定性）。
  3. 语料构建已有现成的 JVM 运行与 JVMTI 代理：`scripts/dyn_compare.py` 的 `load_trace` 代理，`--update-expected` 的 golden 运行。
  4. 业界做法一致：GraalVM native-image 的 `predefined-classes-config.json`（tracing agent 记录、运行期按哈希匹配），
     Spring Boot AOT 在构建期预生成 CGLIB 代理类。
- 缺点：覆盖面取决于训练运行走到哪里，没走到的定义点运行期仍报 `LinkageError`；生产构建要求用户提供训练运行，
  训练产物归用户项目（与「库配置归用户项目、rava 不放第三方库内容」一致）。
- 语料侧落点：训练运行由 `rava build` 在闭包分析发现「类定义 native 可达、且字节不是常量」时按需触发
  （参考 JDK `java -agentpath:<record>`，产物写 scratch `build/<test>/predefined/`）。TestDefineClassRejects 也会触发，
  但记录为空（全部被拒）。golden 运行与训练运行可合并为一次。

### S2b：自举训练（S2 的变体）

训练运行不用参考 JVM，而用 rava 自己编出的二进制：以记录模式运行，未命中的定义把字节落盘，再重建。

- 优点：记录的字节就是 rava 运行期自己生成的字节，不存在「JVM 生成的字节与 rava 重新生成的字节不同」的风险（见 §7 风险 1）；构建不依赖参考 JVM 运行程序。
- 缺点：未命中时定义仍然失败，程序在第一个失败点之后走的路径与 JVM 不同，后续定义点要多轮「运行 → 重建」才能逐个暴露；
  每轮都要完整编译一次（本例约 10 分钟）。

### S3：运行期字节码解释器（不建议）

在运行时里放一个 JVM 字节码解释器，解释执行运行期定义的类。被解释类要继承已编译类（translet 继承 `AbstractTranslet`），
对象模型要支持「编译出的 struct + 解释期追加字段」，虚分派要能跨编译 / 解释两侧。这相当于在原生二进制里再做一个 JVM，
违背「生成的 Rust 是可读中间层」的定位，二进制体积也要承担整个解释器。不建议。

### 比较

| | S1 构建期求值 | S2 训练运行 | S3 运行期解释 |
|---|---|---|---|
| 本用例 | 可（需引擎补能力） | 可 | 可 |
| 资源文件样式表 | 原则上可 | 可 | 可 |
| CGLIB / ByteBuddy（输入运行期才知道） | 一般不可 | 可（训练覆盖到的） | 可 |
| 构建依赖 | 无新增 | 参考 JDK 运行一次程序 | 无 |
| 生成器 / 引擎工作量 | 大（选点规则 + 引擎能力） | 中（代理 + 输入通道） | 极大 |
| 可读性 / 体积 | 不变 | 不变 | 显著变差 |

## 4. 承载：按内容寻址的预定义类（S1 / S2 共用）

### 4.1 输入与翻译

- 预定义类是**程序私有**的：进用户 crate（生产构建静态链接；语料构建同样进各测试自己的 user crate），不进档案。
- 输入：一组类文件，每个带内容哈希（SHA-256）与原始 binary name。翻译走用户类同一路径（字节码 → Rust），
  手写准入不变：它们是有字节码的方法，全部按字节码翻译。
- **同名不同内容**（XSLTC 缺省类名固定）：内部键在原名后加内容哈希后缀，形如隐藏类 `die/verwandlung/GregorSamsa/0x<哈希前 16 位>`，
  沿用 `Class.for_class` 对隐藏类名的处理。`getName()` 返回原名：由元数据表给出（预定义类的名字条目），不从内部键推导。
  只有一个内容时不加后缀。

### 4.2 闭包分析

- 预定义类的根：类定义 native（`ClassLoader.defineClass0/1/2`）的返回值 = 预定义类镜像集合（多个精确镜像的并）。
  沿用 `[facts.reflect.defined_classes]` 的建模方式（`engine/hw.rs` 把定义点返回值置为支持类的精确镜像），
  清单项从「定义点 → 单个支持类」扩成「定义点 → 预定义类集合」，集合由输入给出，清单里只登记哪些 native 是类定义点。
- 随后 `getConstructor().newInstance()`、`getSuperclass()` 等在这些镜像上按已有反射分析解析，
  translet 的方法与它调用的 DTM / 迭代器类进入闭包。
- 没有预定义类输入时，定义点返回值与现在相同（不新增任何类）。HelloWorld / DeepCopy 不经过类定义 native，闭包不变。
- 语料档案：档案是全体入口调用链的并集，预定义类调用到的 JDK 方法要进档案，所以预定义类作为其所属测试的入口种子参与档案计算。

### 4.3 运行期

`defineClass0/1/2` 的手写（类 1 native，语义登记为类 2「运行期类定义点」）：

1. 现有参数检查与格式检查不变（TestDefineClassRejects 的输出不变）；
2. 格式合法时对区间字节求 SHA-256，查生成的预定义类表（java_meta，键为哈希）：
   - 命中：校验调用方给的 `name` 与类文件中的名字一致（不一致按 JVM 抛 `NoClassDefFoundError: … (wrong name: …)`）；
     同一加载器重复定义同名类按 JVM 抛 `LinkageError: … attempted duplicate class definition`；
     记录 (加载器, 类) 为定义关系，返回该类镜像；
   - 未命中：维持现在的 `LinkageError`。
3. 定义加载器：`Class.__vm_defining_loader`（`[vm_state.field_hooks]`）先查运行期定义记录，再查生成器给出的静态表。
   `TemplatesImpl` 在定义前经 `ModuleLayer.defineModules` 建了模块 `jdk.translet`，`Class.module` 钩子按「定义加载器 + 包」查 VM 模块表，
   有了正确的定义加载器就能查到该模块。

### 4.4 与 JVM 的语义差异（需登记）

同一份字节被两个加载器各定义一次（例如对同一样式表调两次 `newTemplates`）时，JVM 得到两个不同的类，静态字段各一份；
rava 只有一个静态链接的类，两次定义返回同一镜像，静态字段共用，`getClassLoader()` 返回首个定义者。
XSLTC translet 的静态字段只在 `<clinit>` 写常量，共用不可观察。这一差异写进 `java-rust-translation-reference.md` 与清单注释。
（决定点 X3。）

## 5. 实施步骤（按 S2，10-10 已批准）

1. **记录代理**：在 `load_trace` 代理旁加记录模式：`ClassFileLoadHook` 中 `loader != null`、类不来自类路径 / jrt 的，
   按 SHA-256 写 `<哈希>.class` 与清单（原名、定义次数）。
2. **构建输入通道**：`rava build` 读 scratch `predefined/`（语料由按需训练运行生成；生产由用户项目目录提供），
   `input` crate 把这些类登记为用户域的预定义类。
3. **闭包**：类定义 native 的返回值建模（§4.2），清单 `vm_intrinsics.toml` 登记 `defineClass0/1/2` 为类定义点。
4. **发射与元数据**：内部键与原名（§4.1），java_meta 生成哈希 → 类表与名字条目。
5. **运行期**：`class_loader_impl.rs` 三个 native 查表（§4.3），`class_impl.rs` 定义加载器钩子查运行期定义记录。
6. **按需训练**：闭包发现类定义 native 可达且有非常量字节时，以参考 JDK 跑一次训练运行，结果并入输入后重算闭包。

验收：TestXmlTransform 通过；TestDefineClassRejects 输出不变；71_xml 其余用例、HelloWorld、DeepCopy、CollectorsDemo 无新增失败；
HelloWorld / DeepCopy 闭包类数、方法数不变；闭包单测 A、B 组无新增失败。

## 6. 决定（2026-10-10 用户采纳建议）与备选路线

### 6.1 决定

- **X1 字节来源 = S2 训练运行**：在参考 JDK 上用 JVMTI 代理记录运行期定义的类字节，按 §5 实施。
- **X2 触发方式**：
  - 语料构建由 `rava build` 按需自动触发训练运行；
  - 生产构建要求用户显式执行训练命令（如 `rava trace`），产物归用户项目，rava 不自动运行用户程序。
- **X3 同字节多次定义 = 共用一个类**：语义差异按 §4.4 登记到 `java-rust-translation-reference.md` 与清单注释。

### 6.2 备选路线（主线跑不通时按此切换）

§4 的承载（按内容寻址的预定义类）与字节来源无关，下列路线都复用它，切换时只换「字节从哪里来」。

| 路线 | 何时切换 | 做法 | 代价 / 限制 |
|---|---|---|---|
| **B1：S2b 自举训练** | 风险 1 实测成立：rava 运行期重新生成的字节与 JVM 记录的字节不同，且原因（身份哈希排序等）无法在生成器侧消除 | 用 rava 自己的二进制以记录模式运行，未命中的定义把字节落盘后重建；可与 S2 并用（S2 给初始集合，S2b 只补哈希不符的类） | 每补一轮要完整编译一次（本例约 10 分钟）；首个失败点之后的路径要多轮才能逐个暴露 |
| **B2：S1 构建期求值** | 训练运行不可得，例如参考 JDK 跑不起程序、程序依赖外部环境；或要求生产构建不运行用户程序 | 由具体引擎（引导映像求值器）在构建期执行「输入可确定」的生成调用，在类定义 native 处截获字节 | 选点没有通用规则，要先补引擎能力（先做探针：能否跑完一次 XSLTC 编译）；CGLIB / ByteBuddy 这类输入运行期才知道的一般覆盖不了；身份哈希同样有风险 1 |
| **B3：S1 + S2 并用** | S2 已落地，但有些用户不愿或不能提供训练运行，同时其样式表 / 模板来自构建期可求值的资源 | 构建期可求值的输入走 S1，其余走 S2；两路产物并入同一张预定义类表 | 两套来源都要维护；要定义两路结果冲突时以谁为准（按内容哈希去重即可） |
| **B4：X3 改为每次定义一个新类** | 发现共用静态字段可观察的真实用例（生成类的静态字段在 `<clinit>` 之外被写、或用例比较两次定义的 Class 身份） | 每次定义得到独立镜像，静态字段按定义实例分开存，对象携带类指针 | 对象模型改动大，影响全部生成类的静态字段访问路径；只在有用例需要时做 |
| **B5：S3 运行期字节码解释器** | 以上路线都无法覆盖，且确有必须支持的场景（例如字节由运行期外部输入决定，如用户上传的样式表） | 运行时内置 JVM 字节码解释器，被解释类可继承已编译类，虚分派跨编译 / 解释两侧 | 违背「生成的 Rust 是可读中间层」的定位，二进制要带整个解释器；列为最后手段，启用前须用户再次决定 |

S2 的实施中，以下情况视为「跑不通」，应停下并回报，按上表切换：

- 构建后自检发现训练字节与 rava 运行期字节不符，且原因不可消除：切 B1。
- 训练运行无法在语料环境中自动完成（参考 JDK 缺模块、程序需要网络或交互）：切 B2，或对该用例登记暂缓。
- 共用语义导致输出与 JVM 不一致：切 B4。

## 7. 遗留与风险

- **风险 1：两侧字节不同**。按内容寻址要求 rava 运行期重新生成的字节与构建期取得的字节逐字节相同。XSLTC 本身是确定的（§1.3），
  但若生成器内部按身份哈希排序（以无 `hashCode` 覆盖的对象为 `HashMap` 键再遍历），JVM 与 rava 的身份哈希不同会使字节不同。
  XSLTC 的表以字符串为键，预计不受影响；实施时以「训练字节 = rava 运行期字节」作为每个预定义类的构建后自检（运行期未命中时
  报出期望哈希与实际哈希，便于定位）。S2b 没有这一风险；S1 的具体引擎身份哈希按自己的规则取（`fnv32`，引导映像计划 §5.8.1），与运行期不同，同样受影响。

- 未捕获异常报告不打印经 `getCause()` 覆盖给出的 cause（§1.2）：`report_uncaught_in` 应按 JVM `printStackTrace` 走虚调用 `getCause()`。
- `ClassLoader.defineClass0`（`Lookup.defineClass` / `defineHiddenClass` 的入口）在 rava 里尚无手写；预定义类方案实施时一并补上，
  隐藏类的名字后缀规则与 §4.1 一致。

## 8. 实施记录（fix-xsltc2，2026-10-10）

分支 fix-xsltc2（基线 d401e742；10-10 合入集成分支 6a668ac6 后为 3b65f6cf 起）。§5 第 1–6 步全部实施，未触发 §6.2 切换条件。

### 8.1 各步落点

| 步 | 提交 | 落点 |
|---|---|---|
| 1 记录代理 | f0cc4f7c | `scripts/dyn_agent/define_record.c`（JVMTI `ClassFileLoadHook`，非引导加载器的定义按序原样落盘 `<序号>.class`；调用栈含清单另行承载的类定义点（`kind = "class_definition"`，如动态代理）的不记录；类路径已有 / 非法字节由构建端 `normalize` 剔除）；`classfile::sha256`（生成器侧 SHA-256，无外部依赖） |
| 2 输入通道 | 290bd5ce | 预定义类目录 = `classes/<binary name>.class` + `predefined.toml`；`resolve` 新增来源 `Origin::Predefined`（用户域，定义加载器 `"defined"`）；`rava build --predefined <目录>` / `--train-predefined`、`rava trace`（生产构建显式训练，产物写用户指定目录） |
| 3 闭包 | 290bd5ce | `vm_intrinsics.toml` 新增 `predefined_definers`（`defineClass0/1/2`，附 X3 语义注释）；`engine/hw.rs` 把定义点返回值置为全体预定义类精确镜像之并，无预定义类时不变 |
| 4 发射与元数据 | 2ae5dabe | `input` 对预定义来源的用户类求内容哈希；user meta 发射 `PREDEFINED_CLASSES: &[(哈希, 原名)]`（按哈希排序），`rava_meta_tables` 汇总进 `UserMeta.predefined_classes` |
| 5 运行期 | 57465630 | `predefined.rs`（SHA-256、二分查表、定义记录：类 / 加载器 / ProtectionDomain）；`class_loader_impl.rs` 的 `defineClass0/1/2` 共用 `_define`：格式检查不变 → 求哈希查表 → 未命中抛 `LinkageError`（报实际哈希与同名预定义类的期望哈希，即 §7 风险 1 的构建后自检）→ 名字不符 `NoClassDefFoundError (wrong name: …)` → 同加载器重复定义 `LinkageError attempted duplicate class definition` → 记录并返回；`findLoadedClass0` / `__vm_defining_loader` / `getProtectionDomain0` / `__is_known_class` 查定义记录（定义前不可见）；`ClassLoader.defineClass0` 补上（`Lookup.defineClass` 入口）；`report_uncaught_in` 改为虚调 `getCause()` |
| 6 按需训练 | 290bd5ce、cd818df0 | `build_stages` 首轮闭包后若类定义 native 可达且允许训练，以参考 JDK 跑一次训练运行（`-Xshare:off -agentpath:…`，超时 600 s），产物并入类路径后重算闭包（缓存键随输入变）；新鲜度按用户类 / jar / 代理源 / java home / main 的戳判断，不新鲜即删旧产物重训；`run_tests.py` 的语料转译带 `--train-predefined`，生产构建缺省不训练 |

训练产物只在 scratch（`build/<test>/predefined/`）与用户项目目录，不提交。

### 8.2 实测

- TestXmlTransform 训练：`训练运行 exit 0（0.3s）：记录 29 个定义 → 预定义类 1 个，同名不同内容 0 个，类路径已有 28，非法字节 0`；
  预定义类 `die/verwandlung/GregorSamsa`（sha256 `0b8b7f20…7044f`，定义 1 次），user meta 生成表项；第二次构建判为新鲜、不重训（作业 xsltc2-ev-new）。
  记录到的另 28 个是测试自己的类与 XSLTC 运行期辅助类，类路径上已有，正确过滤。
- 两侧字节一致：rava 运行期 XSLTC 生成的 translet 字节命中训练表（TestXmlTransform 通过，未触发未命中报错），§7 风险 1 未发生，不需要切 B1。
- 抽查 xsltc2-s1（cd818df0，dev）：TestXmlTransform、TestDefineClassRejects、HelloWorld、DeepCopy 全过；TestXmlTransform 转译 338.7 s、编译 486.8 s。
- 抽查 xsltc2-s2（3b65f6cf，dev）：15 例全过——TestXmlTransform、TestDefineClassRejects、71_xml 其余 11 例（TestDomBuildTree、TestDomResultNode、TestQNameFaces、TestSaxLocatorAttributes、TestSaxNamespaceCallbacks、TestXmlDomParse、TestXmlFactoryConfigs、TestXmlSaxEvents、TestXmlStax、TestXmlXPath）、CollectorsDemo、HelloWorld、DeepCopy；TestXmlTransform 转译 469.8 s、编译 510.7 s、运行 0.9 s。
- 闭包规模（`rava closure`）：集成基线 6a668ac6 → 本分支 3b65f6cf（作业 xsltc2-ev3-base / -new）：

  | 用例 | 类数 | 方法数 | 方法上下文 |
  |---|---|---|---|
  | HelloWorld | 577 → 577 | 1896 → 1897 | 5784 → 5787 |
  | DeepCopy | 3116 → 3116 | 17270 → 17270 | 154552 → 154560 |

  HelloWorld 多出的唯一方法是 `java/lang/Throwable.getCause:()Ljava/lang/Throwable;`（闭包 JSON 中基线 0 处、本分支 2 处），
  来自 `report_uncaught_in` 改虚调 `getCause()`（任务要求的修复，手写调用经 `uncaught` VM 规则入闭包）；预定义类机制本身不改变两例闭包。
  DeepCopy 已含 `getCause` 与 `defineClass1/2`，方法数不变；它可达类定义 native，语料构建会跑一次训练（预定义类 0 个，闭包不变）。
  （d401e742 → cd818df0 同样是 HelloWorld 577 / 1896 → 1897，作业 xsltc2-ev-base / -new。）
- 单测：见 §8.4。

### 8.3 未完成与遗留

- **同名不同内容**（§4.1 内部键加哈希后缀）未实现：训练中同名多内容的类按冲突整体排除（训练摘要计数「同名不同内容」），运行期定义时走未命中 `LinkageError`。
  XSLTC 同名多内容只在同一进程编译多份不同样式表时出现，现有语料无此用例；实现时按 §4.1 走隐藏类名路径。
- **语料档案模式**（§4.2 末条）未接：预定义类目前只在 per-test 构建的用户 crate 里，`rava profile` / 档案计算还不把它作为所属测试的入口种子；
  档案模式下预定义类调用到、而档案外的 JDK 方法会落存根。
- **隐藏类**（`defineClass0` 带 `HIDDEN_CLASS` 标志）：JVMTI 规范规定隐藏类不触发 `ClassFileLoadHook`，训练记录不到，
  运行期 `Lookup.defineHiddenClass` 定义用户字节仍走未命中 `LinkageError`（JDK 内部的隐藏类——lambda、LambdaForm——另有 VM 承载，不受影响）。
  需要时改用字节码插桩记录 `defineClass0` 实参（S2 的记录手段扩展），不改承载。
- 训练运行不注入 rava 构建期钉值的属性配置（引导映像 U14），程序若按平台属性分支生成不同字节会未命中；运行期报期望 / 实际哈希便于定位。
- `defineClass0` 的 `classData` 未保存（`MethodHandles.classData` 取不到）；`Class.forName` 对预定义类的可见性只看「是否已定义」，不区分加载器；
  重复定义报错消息不含 JVM 给出的加载器描述。
- `report_uncaught_in` 虚调 `getCause()` 使 `Throwable.getCause` 经 `uncaught` VM 规则进入每个程序的闭包：HelloWorld 方法数 +1，属正确依赖（JVM 的未捕获报告同样经 `printStackTrace` 虚调 `getCause`）。
