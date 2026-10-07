# 模块资源束按名装载（c4-resbundle，基于 rust-closure-analyzer 1fe9e5ea）

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
   - 属性文件 `B.properties` / `B_<后缀>.properties`：类路径上存在的，进输出事实 `bundle_resources`。
4. 集合只增不减，站点值集增长时由补种轮重求（外层不动点）。

### 2.3 事实链路

`SeedState.bundles.resources` → 闭包 JSON `seeds.bundle_resources` → 档案 profile（集合并）→
`input::facts::SeedFacts.bundle_resources`（`from_closure` / `parse_seeds` 两个入口一致）→ `compose`（档案非用户侧 ∪ 单测用户侧）
→ `input/build.rs` 并入 `module_resources`（按路径读字节嵌入）。运行时零改动。

## 三、边界与已知限制

- **lambda 捕获实参**：`SecuritySupport.getResourceBundle` 把基名捕获进 lambda 再调用 `getBundle`，
  而 pstr 不追踪捕获实参，所以基名求值是 Any，XMLMessages 靠字面量回退入选。
  捕获实参感知的名字追踪属于名字求值的通用增强，另行实施；做完后这类站点会自动收窄到精确名字。
- **locale 覆盖**沿用 locale 种子的口径。宿主 locale 不在集合里（例如 zh_CN）时回退到根束，
  与 GraalVM `-H:IncludeLocales` 同一取舍。
- 调用链上从未作为字面量出现的基名（外部配置读入的名字）覆盖不到；这类站点记为未知站点，统计进日志行。

## 四、验收

- 抽查集：TestXmlSaxEvents、TestRowSetProvider（目标通过），HelloWorld、CollectorsDemo、TestXmlTransform、
  TestSaxLocatorAttributes、TestPropertiesXmlRoundTrip（不回归）。
- 闭包规模：对照基线（HelloWorld 469、CollectorsDemo 2914、TestRowSetProvider 2958 类），增量只能来自束类。
- 两例通过后删除 `docs/known_failures.toml` 中对应的条目。
