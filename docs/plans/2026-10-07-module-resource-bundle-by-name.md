# 模块资源束按名装载（c4-resbundle，基于 rust-closure-analyzer 1fe9e5ea）

> 状态（2026-10-08）：已完成（c4-resbundle 110e1602，合入集成分支 74837098，2026-10-07）。TestXmlSaxEvents 的已知失败条目随 batch-1008 删除（6934dc93，待合批验证）；TestRowSetProvider 按框架 API 面暂缓。

> 根因：`ResourceBundle.getBundle(基名, …)` 按名装载的资源束（`.properties` 文件或 `ResourceBundle` 子类）没有建模。
> 涉及：TestRowSetProvider（`Can't find bundle for base name com.sun.rowset.RowSetResourceBundle`）、
> TestXmlSaxEvents（`Could not load any resource bundle by com.sun.org.apache.xerces.internal.impl.msg.XMLMessages`）。

## 一、现状

- 两个束在 jmod 里都是 `.properties` 文件（`com/sun/rowset/RowSetResourceBundle*.properties`、
  `com/sun/org/apache/xerces/internal/impl/msg/XMLMessages*.properties`），不是类。
- 运行时这一路已经能走通：`ResourceBundleProviderHelper.loadPropertyResourceBundle` → `Module.getResourceAsStream` →
  `BootLoader.findResourceAsStream`（手写，查生成的模块资源表 `jdk_resources::module_resources`）。
- 缺的是文件没有嵌入。模块资源只由 `input/src/resources.rs` 从调用链上的 ldc 路径形字符串推导，
  点分的束基名还要加 locale 后缀和 `.properties` 才是文件路径，推导覆盖不到。
- 束是类（`ListResourceBundle` 子类）时同理：`Class.forName` + `newInstance` 反射构造，没有静态调用边。
  按名取类的通用建模只能看到 `getBundle` 内部拼出的名字，那个名字推不出。

### 进度（2026-10-07 收尾，排期调整后暂缓）

TestRowSetProvider（S2）按用户排期暂缓（known_failures `deferred`），本线到此收尾。

- **已完成并抽查验证**（合成分支 c4-resbundle-chk = 本分支 + c1d-url-b2，抽查 resbchk-ee998acd / resbchk-cf858e27）：
  TestXmlSaxEvents 通过；HelloWorld、CollectorsDemo、TestPropertiesXmlRoundTrip、TestSaxLocatorAttributes 不回归。
  通用部分：资源束基名按键值求值与字面量兜底（2.1–2.2）、按名读取的资源（2.3）、lambda 捕获值对齐（2.4）、
  推不全站点与 String 字段非常量写入的槽输入（2.5）。`ResourceBundle.getBundle` / 资源读取入口都由清单登记，
  框架侧（如 Spring `ResourceBundleMessageSource` 经 `getBundle` 按名装载）同样适用。
- **已查实的结论（TestRowSetProvider）**：
  - `com.sun.rowset.RowSetResourceBundle` 束已能按字面量站点求出并嵌入，原报错不再出现；
  - 第二个根因是 `SyncFactory` 经 `Module.getResourceAsStream(ROWSET_PROPERTIES)` 读 `javax/sql/rowset/rowset.properties`，
    名字是拼接后写进静态字段的。resb-diag8 确认闭包现已求出
    `{"javax/sql/rowset/rowset.properties", "rowset.properties"}`。
- **未完成**：resbchk-cf858e27 中 TestRowSetProvider 仍报 `Resource javax/sql/rowset/rowset.properties not found`。
  名字已求出，所以问题在其后两段之一，尚未区分：
  - ① 事实链路没把该资源嵌进模块资源表：`named_resources` → profile / compose → `input/build.rs` `module_resources`；
  - ② 运行时 `Module.getResourceAsStream` 对具名模块 `java.sql.rowset` 的查找（模块名 / 包可见性 / 资源表键）没命中。
- **恢复时从这里开始**：在 scratch 生成树里查 `jdk_resources::module_resources` 是否含 `javax/sql/rowset/rowset.properties`
  （属于哪个模块键）。
  - 含：追运行时 `Module.getResourceAsStream` → `BootLoader.findResourceAsStream` 的查找路径；
  - 不含：追 `[closure] 按名读取的资源` 日志行，再看 profile 合并或 compose 的过滤。
- c4-resbundle-chk 上的临时诊断提交（476fb384、084e9c5f）只在 chk 分支，不并入。

## 二、终态设计

### 2.1 清单（seeds.toml `[bundles]`）

```toml
[bundles]
root = "java/util/ResourceBundle"          # 束类候选须是它的子类
[bundles.lookups]                          # 按名装载入口 → 基名实参序号（不含接收者，0 起）
"java/util/ResourceBundle.getBundle:(Ljava/lang/String;)Ljava/util/ResourceBundle;" = 0
...                                        # 公开 getBundle 重载全部登记
```

生成器 crate 里不写任何 JDK 类名，入口与根类都由清单给出。

### 2.2 闭包引擎（`engine/bundles.rs`，补种轮 `seed_round` 驱动）

1. **登记站点**：新可达方法只扫描一次，记录两样东西：
   - `lookups` 成员的调用点（方法、偏移、基名实参序号）；
   - 束形 ldc 字面量，即由标识符段组成的点分名，且不含 `/`。
2. **求基名**：每轮在站点所在方法的当前分析里，用 `names_of`（与 JCA 请求点同一套键值求值，`cur_site` 置位）
   重新求基名实参。
   - 结果是 `Keys::Set`，就取这些名字。
   - 结果是 `Keys::Any`（名字推不出），就把该站点记为未知，回退到「调用链上的束形字面量」。
     这和服务查找的回退一个道理：原生程序里能装载的束，名字只能来自闭包内的代码。
     回退名只有在类路径上真有对应的束类或 `.properties` 时才入选。
3. **展开候选**：基名用 `locale::collect`（ROOT + en + 用户代码可见 locale + `--locale`）及其父链展开成后缀：
   - 束类 `B` / `B_<后缀>`：在类路径上、是 `root` 的子类、有 `<init>()V` 的，按 locale 束的方式补种
     （实例化 + 类初始化 + 无参构造器作种子方法 + 反射分派面登记 `<init>`）；
   - 属性文件 `B.properties` / `B_<后缀>.properties`：类路径上存在的，进输出事实 `named_resources`。
4. 集合只增不减，站点值集增长时由补种轮重求（外层不动点）。

### 2.3 按名读取的资源（seeds.toml `[resource_lookups]`，`engine/res_lookups.rs`）

实测发现 TestRowSetProvider 还有第二个同类根因：`SyncFactory` 读 `javax/sql/rowset/rowset.properties`，
资源名是 `"javax" + strFileSep + "sql" + … + "rowset.properties"` 的拼接结果，存进静态字段 `ROWSET_PROPERTIES`
（另一支取系统属性），再在 lambda 里 `getModule().getResourceAsStream(ROWSET_PROPERTIES)`。
生成器只认 ldc 字面量与相邻字面量模板，这个名字认不出来。

- 清单登记资源读取入口（`Module.getResourceAsStream`，`Class.getResource[AsStream]`（relative），
  `ClassLoader.getResource[s|AsStream]` / `getSystemResource[s|AsStream]`），值为 `{ arg, relative }`。
  键按声明处成员匹配：扫描时先按方法名筛，命中再解析声明类。
- 调用点和资源束站点在同一次新可达方法扫描里登记。每个补种轮用 `Gap::Class` 求资源名实参，推不出的段
  记为任意串；展开后只取全部段已知的候选，类路径上存在的进 `named_resources`。
  系统属性这类外部给出的名字不在闭包内，原生程序里也只能读到嵌入的资源。
- relative 入口的相对名按调用点所在类的包解析，与生成器字面量推导同一口径。

### 2.4 lambda 实现方法的形参值（`engine/lambda_vals.rs`）

`SecuritySupport.getResourceBundle(bundle, locale)` 把基名捕获进 lambda（`doPrivileged(() -> getBundle(bundle, locale))`）。
原来 lambda 接边时，直接拿 SAM 调用点（`run()`，无实参）的实参值对位实现方法的形参。结果是捕获值缺失，
有捕获时 SAM 实参还会错位：形参常量、污染、字符串槽三处都受影响。基名槽于是为空集，既不是 Any，也选不中名字。

终态做法：
- lambda 登记时记下创建点 indy 的实参值（创建方法帧）。
- 接边时实现方法的实参 = 捕获值 ++ SAM 实参（虚 / 接口 / 特殊实现跳过首个，即接收者）。捕获段按创建方法帧、
  SAM 段按调用方帧，分别并入形参常量、污染与字符串槽（子集边指向各自帧的形参槽）。
- 名字求值的非常量实参也支持 indy 创建点。
- 任一段的值未知（非字节码调用方、具体求值物化的 lambda）时，形参整体按未知处理。

做完后 `SecuritySupport` 的站点能沿 `getResourceBundle` 调用点求出捕获的基名；上游推不全的部分见 2.5。

### 2.5 名字推不全与具体求值写入

抽查诊断（resbchk-62b4be4a、resbchk-ee998acd）补出三处：

- **推不全的站点**：`names_of` 用 `Gap::Fail` 求值，槽推不全时（未定字段、非常量上游）只给出已知部分并置
  `lookup_partial` / `lookup_incomplete`，结果仍是 `Keys::Set`。资源束站点此前忽略这两个标记，把「只知道一部分」
  当成「全部名字」，XMLMessages 等基名于是漏选。现在 `seed_bundles` 在求值前后取走这两个标记：推不全时
  已知名字照收，站点同时记为推不出，由字面量兜底覆盖其余基名。
- **String 字段的非常量写入**：`SyncFactory.initMapIfNecessary` 把拼接结果 `"javax" + strFileSep + … + "rowset.properties"`
  写进 `ROWSET_PROPERTIES`。字段字符串槽 `PSlot::F` 此前只收字面量写入与形参透传，拼接 / 调用结果写入只把槽记为推不出；
  而读者直接读字段时名字段按内部求值（`Gap::Fail`）处理，推不全只置引擎标志，按名读取的资源把 `<clinit>` 里的
  `rowset.properties` 当成全部名字。现在非常量写入与非常量实参同一口径，登记为槽的输入
  （写入方, putfield / putstatic 偏移, `FIELD_VALUE`）：读者在写入方帧里按拼接段求出名字（`strFileSep` 本身又经字段槽
  取得 `/`），求不出的写入（如系统属性一支）使槽推不全。值未知的写入仍使槽推不出。
  按名读取的资源只取已知名字，求值前后保存引擎的推不全标志，不外泄给其他站点。
- **具体求值的字段写入**：具体求值器（`engine/concrete/`）执行的写入此前只投影为常量格（整数 / 长整数 / null / 其他），
  String 字段的常量集 `field_strs` 与字段槽都收不到。现在投影新增 `Put::Str`（字符串内容可读时），物化时
  （`concrete_str_puts`）与字节码写入同一口径并入：内容可读为字面量，不可读则字段槽推不出。

### 2.6 事实链路

`SeedState.named_resources`（资源束属性文件 ∪ 按名读取的资源）→ 闭包 JSON `seeds.named_resources` → 档案 profile（集合并）→
`input::facts::SeedFacts.named_resources`（`from_closure` / `parse_seeds` 两个入口一致）→ `compose`（档案非用户侧 ∪ 单测用户侧）
→ `input/build.rs` 并入 `module_resources`（按路径读字节嵌入）。运行时零改动。

## 三、边界与已知限制

- **运行时依赖**：`.properties` 束在 JDK 类所属的无名模块下走 `Control.newBundle` → `ClassLoader.getResource` →
  URL 读入，依赖 c1d-url-b2（T2 内嵌资源 URL Handler）让 `getResource` 返回内嵌资源的 URL；
  抽查在合成分支 c4-resbundle-chk（c1d-url-b2 + 本分支）上进行。
- **locale 覆盖**沿用 locale 种子的口径。宿主 locale 不在集合里（例如 zh_CN）时回退到根束，
  与 GraalVM `-H:IncludeLocales` 同一取舍。
- 调用链上从未作为字面量出现的基名（外部配置读入的名字）覆盖不到；这类站点记为未知站点，统计进日志行。

## 四、验收

- 抽查集：TestXmlSaxEvents、TestRowSetProvider（目标通过），HelloWorld、CollectorsDemo、TestXmlTransform、
  TestSaxLocatorAttributes、TestPropertiesXmlRoundTrip（不回归）。
- 闭包规模：对照基线（HelloWorld 469、CollectorsDemo 2914、TestRowSetProvider 2958 类），增量只能来自束类。
- 两例通过后删除 `docs/known_failures.toml` 中对应的条目。
