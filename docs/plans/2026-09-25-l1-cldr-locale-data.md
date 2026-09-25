# L-1：Locale 数据改由 CLDR 资源束字节码翻译供给

> 2026-09-25 · 来源：TestLocaleConstants（fr_FR / it_IT 格式化回退美式）· 关联：tasks.md L-1

## 一、现状与问题

`DecimalFormatSymbols.getInstance(Locale)`（`java/text/decimal_format_symbols_impl.rs`）与
`NumberFormatProvider` 的四族模式（`sun/util/locale/provider/locale_provider_adapter_impl.rs`）
是**手写的两套表**：ROOT/en 与 de。其余 locale 一律回退 ROOT。

- 可观察偏差：`String.format(Locale.FRANCE, "%,.2f", …)` 得 `1,234,567.89`（JVM：`1 234 567,89`，
  分组符 U+202F）；it_IT 同理。
- 原则冲突：**原则 1**——JDK 行为必须来自字节码翻译。CLDR 本地化数据在 JDK 中**就是字节码**，
  手写表是绕行方案，且随 locale 扩展线性增长。

## 二、事实（javap / jmod 实测）

| 项 | 事实 |
|---|---|
| 数据载体 | `sun/text/resources/cldr/FormatData.class`（ROOT，java.base，46 KB）、`FormatData_en.class`（java.base）；其余 414 个 `sun/text/resources/cldr/ext/FormatData_<tag>.class` 在 **jdk.localedata** 模块 |
| 类形态 | `extends java.util.ListResourceBundle`，仅 `<init>` + `protected final Object[][] getContents()`；`getContents` 是纯字面量数组构造（FormatData_fr：~12k 条指令，`ldc` 字符串 + `aastore`） |
| 数据键 | 数字相关：`NumberElements`（及 `<numbering>.NumberElements`）、`NumberPatterns`、`DefaultNumberingSystem`；货币 / 日期等同表 |
| JDK 装载链 | `CLDRLocaleProviderAdapter` → `LocaleResources` → `LocaleData.getBundle(name, locale)` → `ResourceBundle.getBundle` → **按类名反射实例化**（无静态调用边） |
| 语料解析 | `JdkResolver._JMOD_PRIORITY` 不含 `jdk.localedata.jmod`——ext 包的类当前不可解析 |
| 边界 | 全部位于 `sun/` → BFS 截断（内部边界类规则 3b） |

## 三、设计

### 3.1 纯数据资源束类放行（边界规则扩展）

在 `sun/` 截断之外增加一条**结构判定**的放行：类满足以下条件即视为「纯数据类」，照常翻译字节码：

1. 超类是 `java/util/ListResourceBundle`（或其纯数据子类）；
2. 除 `<init>` / `<clinit>` 外只声明 `getContents()`；
3. `getContents` 体内只含常量装载、数组构造与 `aastore`（无 invoke 到非白名单目标）。

按结构而非类名判定 → 满足原则 4（Python 侧无 JDK 类名字面量）。放行只对数据类生效，不影响
`sun/` 下其余类的截断。

### 3.2 种子：包含哪些 locale

反射装载无静态调用边，需显式种子。参照 GraalVM native-image 的 `-H:IncludeLocales`：

- **默认集**：ROOT + en + 用户类常量池中引用的 `Locale` 常量（`getstatic java/util/Locale.FRANCE` 等，
  字节码静态可见）所对应的 locale；
- **显式配置**：`--locales fr,it,…`（scratch 级参数，写入 manifest）；
- 每个选中 locale 连同其**父链**（`fr_FR → fr → ROOT`）一并入闭包——ResourceBundle 回退语义所需。

### 3.3 运行时：以翻译束替换手写表

- 生成侧为入选束生成**静态注册表**（binary name → 构造函数），替代 `ResourceBundle.getBundle`
  的类名反射（与 reflect_dispatch / class_init 钩子同一登记模式）；
- `DecimalFormatSymbols.getInstance(Locale)` 与 NumberFormatProvider 改为按 JDK 语义查束：
  `NumberElements` → 符号集、`NumberPatterns` → 模式；删除手写 en/de 两表；
- 回退链走 ListResourceBundle 的 parent 语义（翻译字节码自带）。

### 3.4 体积控制

`getContents` 翻译后是大型字面量数组初始化（FormatData_fr ≈ 12k 条指令）。控制手段：

- 只翻译入选 locale（默认集通常 1–3 个）；
- 发射层对「连续 `ldc` + `aastore`」形态做数组字面量折叠（`JArray::from(vec![…])`），
  把逐元素赋值压成单个 vec 字面量——同时降低 raw_stmt 计数。

## 四、步骤与验收

| 步 | 内容 | 验收 |
|---|---|---|
| L1-a | `JdkResolver` 纳入 `jdk.localedata.jmod`；纯数据资源束结构判定 + 边界放行 | 单测：结构判定对 FormatData_fr 真、对非数据 `sun/` 类假；现有 e2e 零回归（生成树对照） |
| L1-b | 种子：用户常量池 Locale 常量 → locale 集（含父链）+ `--locales` 参数 | 单测：TestLocaleConstants 的种子集含 fr_FR / it_IT 链 |
| L1-c | 束注册表 + `getBundle` 静态化；DecimalFormatSymbols / NumberFormatProvider 改查束 | **TestLocaleConstants 全绿**（fr_FR / it_IT 格式化与 JVM 逐字一致）；TestFormatLocale、TestStringFormat、TestDateTimeFormat 回归 |
| L1-d | 删除手写 en/de 两表；数组字面量折叠 | 生成树 raw_stmt 不升；二进制体积入档 |

## 五、风险

- **反射链深度**：JDK 的 `LocaleResources` 链经 `ServiceLoader` / `Class.forName`，完整翻译代价高；
  3.3 的静态注册表是在「装载」这一跳截断（与现有手写适配器单例的截断位置相同），其后的
  数据消费（NumberElements 解析）走翻译字节码。
- **JDK 版本差异**：CLDR 版本随 JDK 升级（21：CLDR 42；25：CLDR 47），数据差异由字节码自然携带——
  这正是去掉手写表的收益（BaseLocale 常量表的版本坑不会再现）。
