# 参考 JDK 21：语料两侧（expected 生成与转译）同源化

> 日期：2026-10-03。来源：e2enew-da8abee1 归因报告中 TestLocaleCurrency 的 CLDR 漂移
> （本机 21.0.11 = ¥，服务器 OpenJDK build = CN¥），用户裁定：**合法测试不改**，
> 终态为「expected 与转译使用同一个 JDK 构建」，方案采纳与否由本助手定夺。
> **决定：采纳，按本方案落地。**

## 一、问题与终态

CLDR/localedata 等随 JDK 数据随小版本（update release）漂移。expected 由本机 JDK 生成、
转译语料 jmod 取自服务器 JDK，两个构建不同即产生伪差异（本次 1 例，未来 jmod A 档
locale/charset 批量用例会放大此类噪声）。终态：**单一参考 JDK 21 构建**同时供两侧使用。

## 二、机制（零 rava 改动、零系统配置改动）

不动 `resolve::jdk`、不改系统安装的 JDK——**在编排层用 JAVA_HOME 指向参考 JDK**：

1. **选型**：Eclipse Temurin 21 最新 GA（macOS aarch64 + Linux x64 双平台皆有产物；
     落地时钉死精确 tag 与 SHA256，例如 `jdk-21.0.X+Y`，记录在数据目录 README）。
   同版本同 vendor 的双平台构建携带同一份 CLDR/localedata——漂移源消除。
2. **落位**（gitignore 数据目录，与 pilot-libs 同模式）：
   - 服务器：各机数据目录（如 `/data/rava-jdk/<tag>/`），由分发脚本侧统一解压；
   - 本机：仓库根 `tools/refjdk/`（gitignore），`scripts/fetch_reference_jdk.sh`
     一次性下载+校验 SHA256+解压（含平台选择，脚本随仓库走）。
3. **消费**：
   - 服务器抽查：`distribute_tests.py` 发作业前 `export JAVA_HOME=/data/rava-jdk/<tag>`；
   - 本机 expected 生成：`run_tests.py --update-expected` 前同样 export（或脚本内置
     优先取 `tools/refjdk`，存在即用，否则回落既有发现链——回落链保持现状不动）。
4. **一次性切换**：切换后对 locale/charset 敏感用例批量再生成期望
   （`--update-expected --filter 65_locale_data 64_charsets_ext` + 双跑确定性闸门），
   CLDR 版本差导致的 expected 变更一次入库；此后此类漂移归零。

## 三、边界

- 双 JDK 目标（21/25）不受影响：参考 JDK 只钉死 21 基线的语料同源；25 适配轮
  另按其基线处理。
- `.jdk-version` 语义不变（仍表达"21"）；参考 JDK 是**构建级**钉死，在
  `tools/refjdk/VERSION` 里记录 tag 与 SHA256，供人工核对两侧一致。

## 四、执行拆分

| 步骤 | 内容 | 归属 |
|---|---|---|
| 1 | `scripts/fetch_reference_jdk.sh`（下载/校验/解压 + VERSION 登记） | 本助手可做 |
| 2 | 本机切换 expected 生成 + 敏感用例再生成入库 | 本助手可做（仅 JVM 侧轻量作业） |
| 3 | 服务器数据目录解压 + distribute 侧 JAVA_HOME 注入 | 用户侧（运维域） |
