# L-2：货币数据层回到 JDK（FS-L2 / FS-H0 末项）

> 关联：L-1 CLDR 资源束（`2026-09-25-l1-cldr-locale-data.md`）、JDK 资源嵌入（`runtime/java_runtime/src/jdk_resources/`）、
> FS-H0（`DecimalFormatSymbols.initializeCurrency` 是最后一处越界手写覆盖）。

## 一、现状

`decimal_format_symbols_impl.rs` 的 `initializeCurrency(Locale)` 用手写地区表给出货币代码 / 本地符号
（US/CA/GB/欧元区/JP/CN/KR/TW，其余 `XXX` / `¤`），`currency` 字段不建模（`getCurrency()` 为 null）。

## 二、JDK 路径

```
DecimalFormatSymbols.initializeCurrency(locale)
  → Currency.getInstance(locale)            // 国家码 → 货币代码：currency.data（java.base 模块资源）
      → Currency.<clinit> → Class.getResourceAsStream("/java/util/currency.data") → DataInputStream 解析
  → Currency.getSymbol(locale)              // 本地符号：CLDR CurrencyNames 资源束
      → LocaleServiceProviderPool.getLocalizedObject(CurrencyNameGetter, ..)   ← 内部边界
      → CurrencyNameProviderImpl → LocaleResources.getCurrencyName(key)        ← 内部边界（已手写）
```

## 三、方案

| 步 | 内容 |
|----|------|
| C1 | seeds.toml `[module_resources]` 登记 `java/util/currency.data`；生成器从**本轮所用 JDK 的 jmod** 提取（数据与 JDK 版本同源，生成物不提交），写入 scratch 并生成 `jdk_resources/module_resources.rs`（include_bytes! 表）；`Class.getResourceAsStream` 走字节码（无名模块 → `ClassLoader.getSystemResourceAsStream`），VM 边界类 ClassLoader 的资源查询对模块资源返回 ByteArrayInputStream |
| C2 | `Currency` 走翻译字节码（`java/util/` 公开包，本就在 BFS 内）；`getInstance(Locale)` / `getCurrencyCode` / `getDefaultFractionDigits` 全部字节码 |
| C3 | 符号：seeds.toml `[locale] bundles` 增 CLDR `CurrencyNames` 族（按入选 locale 补种，L-1 同机制）；内部边界 `LocaleServiceProviderPool.getLocalizedObject` 对 CurrencyNameProvider 转 `LocaleResources.getCurrencyName`（资源束查表，回落链同 L-1） |
| C4 | 删除 `initializeCurrency` 手写覆盖；`getCurrency()` 回到 JDK 语义 |

## 四、状态

✅ 2026-09-28 完成：C1–C4 全部落地；TestCurrencyApi / TestFormatLocale PASS，`non_native_overrides=0`（FS-H0 收尾）。
实施中追加：资源束族支持独立触发成员（seeds.toml `[locale] bundles` 表形态）、`NumberFormatProvider` 货币样式按
`adjustForCurrencyDefaultFractionDigits` 调整小数位、`[upcall-audit]` 手写回调目标未解析告警。

## 五、验收

- e2e：现有 DecimalFormat 货币格式用例（`NumberFormat.getCurrencyInstance`）期望输出不变；新增 `TestCurrencyApi`
  （`Currency.getInstance(Locale.US/JAPAN/GERMANY)`、`getSymbol`、`getDefaultFractionDigits`、`DecimalFormatSymbols.getCurrency`），
  期望输出由 JVM 生成。
- `[raw-audit] non_native_overrides` → 0（FS-H0 收尾）。
