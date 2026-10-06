# C1d §29 能力②：JCA 提供者序求值（2026-10-06，分支 c1d-jca，基于 b15b81a3）

上游：`docs/plans/2026-10-01-c1d-closure-bloat.md` §29.3（路线 P）。能力①（URL 按对象 + 前缀串域，分支 c1d-url）与
能力③（ResourceBundle 未知 Class 的服务查找，引导映像线）不在本步范围。

## 1. 问题

`ProviderList` 按 `java.security` 的 `security.provider.N` 序逐项装载 provider：`ProviderConfig.getProvider` 对内建名字
（JDK 21：SUN / SunRsaSign / SunJCE / SunJSSE；JDK 25 另加 SunEC）直接 `new`，其余名字走 `doLoadProvider` →
`ProviderConfig$ProviderLoader` → `ServiceLoader.load(Provider.class)`，引导层全部 Provider 实现（SunPKCS11、SunPCSC、
XMLDSig、SunEC(21)、JdkLDAP …）入链。分析器此前把 `doLoadProvider` 调用点按可达处理，等于假设每次服务查找都可能走完全表。

运行期提供者表是构建期事实：表来自嵌入资源 `<java.home>/conf/security/java.security`（`runtime/java_runtime/src/
jdk_resources/java.security.{21,25}.properties`），原生二进制不接受 `-D` 启动参数（`System` 手写层的宿主属性只有固定键，
不含 `java.security.properties`）。游走按序、命中即停：若每次游走都在首个非内建表项之前停下，装载器永不执行。

提供者表（两版本同）：SUN, SunRsaSign, SunEC, SunJSSE, SunJCE, SunJGSS, …, SunPKCS11。首个非内建表项序号
**JDK 21 = 2（SunEC）**，**JDK 25 = 5（SunJGSS）**。

## 2. 终态设计

### 2.1 清单（`runtime/java_runtime/seeds.toml [jca.order]`，JDK 事实全部在此）

| 键 | 含义 |
|---|---|
| `table` / `table_key` | 提供者表来源文件（`{jdk}` 换参考 JDK 主版本）与表键前缀 |
| `preferred_key` | 偏好序属性；启动表中出现即不启用本机制 |
| `override_property` | 附加属性文件的系统属性 |
| `builtin."21"/"25"` | `ProviderConfig.getProvider` 按名直接构造的 provider（javap 核对） |
| `loader` | 装载器调用点被调成员（`ProviderConfig.doLoadProvider`）：闸门所在 |
| `interior` | 游走内部类（`ProviderList`、`ProviderConfig` 及其嵌套类） |
| `views."21"/"25"` | 惰性视图类与其生产者：JDK 21 `ProviderList$3`←`providers()`、`ProviderList$ServiceList`←三个 `getServices`；JDK 25 `ProviderList$2`、`ProviderList$ServiceIterator`←两个 `getServices` |
| `entries` | 外部入口与游走形态：`GetInstance.getInstance(String,Class,String)`（first，failover）、`GetInstance.getService(String,String)`（first）、`GetInstance.getService(String,String,String)`（按名，实参 2）、`Security.getProvider(String)`（按名，实参 0）、`SecureRandom.getDefaultPRNG`（按名常量 SUN） |
| `mutators` | 改写入口：`Security.setProperty`（键实参 0，键以表键前缀开头 / 等于偏好键才算）、`Providers.setSystemProviderList` / `changeThreadProviderList` / `beginThreadProviderList`（可达即算） |
| `total_types` | 首个服务构造不会失败的服务类型（MessageDigest） |
| `registrations` / `plain` / `aliased` | 内建 provider 的注册体（SunEntries 构造器、SunRsaSignEntries 构造器、SunJCE.putEntries）与注册调用 |

生成器 crate 不含任何 JDK 类名字面量；单元测试用合成名字。

### 2.2 纯计算（`closure/src/seeds/jca_order.rs`）

- `parse_table`：按 N 升序取表；`first_loaded`：首个非内建表项序号 k。
- 内建 provider 必定注册的服务（下近似）：注册体中 `ldc 类型; ldc 算法; ldc 实现类名` 之后的首个调用是登记的注册调用，且该调用
  **无条件执行**（其前无返回 / 抛出、无越过它的前向跳转或分支表目标、无从它之前跳到它之后的异常处理入口）。`aliased`
  注册另登记 `[jca] alias_sources`（KnownOIDs）里该标准名的 OID 与同义名；同一标准名出现在多组时不取同义名。
- `first_depth(type, algo)`：表上逐项，遇到必定提供该服务的 provider 即返回序号；先遇到序号 ≥ k 的表项或走完全表即失败。
  名字大小写不敏感（`Provider.getService` 按大写键查）。

### 2.3 引擎（`closure/src/engine/jca_order.rs`）

- **闸门**：`invoke()` 入口处，被调成员为 `loader` 且未放行 → 记入扣住集合，本调用点不处理。
- **入口记账**：`method_ctx` 中受监视成员（内部类方法、入口、改写入口）经非字节码调用点（via 不是来自字节码方法的
  invoke / dispatch / concrete）进入 → 记为有根（调用方不可见）。
- **判定**：工作队列排空、且其余全部放行（rcall / 种子 / 按名取类 / 数组 / 不返回）都不再产生工作后执行；不成立即放行：
  1. 系统属性表全部不折叠或 `override_property` 键可被改写（复用 `sysprops` 的不折叠集合）→ 放行；
  2. 改写入口：无键入口可达 → 放行；有键入口的每个字节码调用点键名推不出、或以表键前缀开头 / 等于偏好键 → 放行；
  3. 从扣住调用点所在方法沿 `callers` 上溯：到入口即记下；视图类方法改从其生产者全部节点继续；非内部方法 → 放行；
     内部方法有根 / 非字节码 / 无调用方 → 放行；
  4. 入口逐调用点按值流求名（`names_of`，与 `[jca]` 服务种子同一套）：推不出 → 放行；调用方非字节码 / 按保守分析 → 放行；
     first：failover 入口的类型不在 `total_types` → 放行；任一（类型, 算法）`first_depth` 失败 → 放行；
     按名：名字不在表中或序号 ≥ k → 放行。
- **放行**：记成因（`summary.jca_order.released`），重跑全部扣住调用点，此后不再扣住。扣住时报告各入口最大游走深度。

### 2.4 顺序无关

扣住期间的分析是「装载器调用点不可达」这一系统的最小不动点（集合结果与处理顺序无关，D1 已保证）；判定只在全局排空点做，
是该不动点的函数；放行一次且永久。故最终结果只有两种：扣住时的最小不动点，或与原分析完全相同（放行后单调补齐）。

## 3. 正确性论证

- **Security 改写入口**：提供者表只能经三条路改变——①`Security.setProperty("security.provider.N" / 偏好键, …)` 在
  `Providers` 初始化前改写属性（键推不出按命中处理）；②`Providers.setSystemProviderList`（`Security.addProvider` /
  `insertProviderAt` / `removeProvider` / `getProviders` 的 `removeInvalid` 收口于此）；③线程局部表
  `changeThreadProviderList` / `beginThreadProviderList`（jar 校验）。三者以私有收口方法登记，可达即放行，不依赖上层 API 列举。
- **附加属性文件**：`Security` 初始化时读 `java.security.properties` 系统属性（`security.overridePropertiesFile=true`）。原生
  二进制启动时该键不存在；程序里任何可能写入它（或属性表整体逃逸 / 键推不出）的活代码使 `sysprops` 不折叠集合含该键或
  `all`，即放行。
- **算法名推不出**：任一入口调用点实参名字集推不出即放行；请求名不在必定注册集合里（含用户故意请求不存在的算法，如
  TestMessageDigestApi 的 `FOO-1`）即视为走完全表，放行。
- **惰性消费者**：`providers()` 列表与 `getServices` 列表 / 迭代器在被遍历时才逐项装载；视图对象只经生产者返回（javap 核对：
  `userList` 只在 `providers()` 读出，视图只在 `getServices` 构造），上溯时改从生产者的调用方继续，消费者落在非入口方法即放行。
  `GetInstance.getInstance` 内的失败转移遍历就地消费视图，由 failover 判据覆盖。
- **failover**：`getInstance` 首个服务构造失败时遍历全表；只对构造不会失败的服务类型（`total_types`）不放行。
- **反射 / 方法句柄进入**：受监视成员有非字节码入口即有根，放行。
- **偏好序**：`jdk.security.provider.preferred` 在启动表里出现则不启用；运行期写入由改写入口判据覆盖。
- **JDK 21/25 差异**：内建集合（25 多 SunEC）、视图类名与生产者签名（List vs Iterator）按版本分表；入口、改写入口描述符两版同。
- **模型外假设**（与既有 `[jca]`、`sysprops` 同口径）：反射直接改写 `Security.props` / `Providers` 私有静态字段、手写层改写
  提供者表不在模型内；内建 provider 构造不抛异常（注册体可执行）。

## 4. 目标与验收

目标（协调方按 mhd 实测修订）：DeepCopy ≤2803 / StockTrans ≤2807 / TSDS ≤2809 / HelloWorld 468；格式串求值与 §29 收益可加。

本能力单独的反事实上界见 §29.1：P 单切 −21（DeepCopy）。预期（实测见 §5）：三例序列化测试的 provider 游走里有推不出算法名的
请求（CodeSource 反序列化 → 证书 / KeyFactory 按未知算法取服务），判定放行，本能力在这三例上为 0；只用 SUN 摘要等
可证请求的测试扣住装载器。

验收：
1. e2e 全过、动态对照漏覆盖 0、无存根命中（服务器 spot）；
2. 结果与哈希种子 / 处理顺序无关（D1 守护单测通过；`--hash-seed` 对比）；
3. 安全 / 加密 e2e 抽查：MessageDigest / Signature / KeyStore / Cipher / 序列化 SUID；
4. 报告叠加 D / S 切除（`--cut`）的反事实。

## 5. 实测

测量脚本 `build/jca/cl.sh`、`build/jca/batch.sh`（scratch 不提交）。`off` = 去掉 `[jca.order]` 的完整 `runtime/` 副本（即改动前，
与 b15b81a3 基线同）；`ds` = 叠加 D+S 切除（`build/jca/cuts_ds.txt`：URLClassPath$3 两点、jar Handler、ResourceBundle
服务查找）；`pds` 另加 P（`doLoadProvider` 方法体）。JDK 21，类 / 方法，均为 1845f494（含 §5.3 修复）。

### 5.1 闭包规模

| 测试 | off | new | off+DS | new+DS | 放行成因（new+DS） |
|---|---|---|---|---|---|
| HelloWorld | 469 / 1828 | 469 / 1828 | 469 / 1828 | 469 / 1828 | 无 JCA |
| DeepCopy | 3195 / 20447 | 3195 / 20447 | 3131 / 18957 | 3131 / 18957 | 改写入口 `setSystemProviderList` 可达 |
| StockTrans | 3193 / 20429 | 3193 / 20429 | 3129 / 18941 | 3129 / 18941 | 同上 |
| TestSerialDefaultSuid | 3200 / 20441 | 3200 / 20441 | 3136 / 18951 | 3136 / 18951 | 同上 |
| Digester | 2912 / 17435 | 2912 / 17435 | 2332 / 14232 | **2263 / 13888** | 扣住（getInstance 深度 0） |
| TestSecureRandomApi | 2914 / 17461 | 2914 / 17461 | 2347 / 14341 | **2278 / 13997** | 扣住（深度 0） |
| SecurityDemo | 2912 / 17439 | 2912 / 17439 | 2349 / 14342 | **2280 / 13998** | 扣住（深度 0） |
| TestMessageDigestApi | 2912 / 17436 | 2912 / 17436 | 2336 / 14271 | 2336 / 14271 | 请求不存在的 `FOO-1`（真实走完全表） |
| TestJcaIndirectDigest | 2941 / 17547 | 2941 / 17547 | 2377 / 14432 | 2377 / 14432 | 改写入口可达 |
| TestMacHmacDigest | 2923 / 17491 | 2923 / 17491 | 2351 / 14322 | 2351 / 14322 | 改写入口可达 |
| TestRsaSignVerify | 2933 / 17680 | 2933 / 17680 | 2602 / 15667 | 2602 / 15667 | 改写入口可达 |
| TestEcSignVerify | 2921 / 17588 | 2921 / 17588 | 2562 / 15441 | 2562 / 15441 | `getServices` 列表在非内部方法消费 |
| TestAesGcmRound | 3011 / 18041 | 3011 / 18041 | 2570 / 15285 | 2570 / 15285 | 同上 |
| TestCipherDesModes | 2998 / 17986 | 2998 / 17986 | 2555 / 15219 | 2555 / 15219 | 同上 |

扣住时少掉的 69 类：SunPKCS11 / SunPCSC(smartcardio) / SunSASL / SunJGSS / JdkLDAP / SunEC / XMLDSig、
`ProviderConfig$ProviderLoader` 与 EC 参数 / 工具类。

### 5.2 结论与放行归因

- **不叠加切除时，本能力在全部测试上为 0**：`Security.getProviders` → `Providers.getFullProviderList` → `removeInvalid` →
  `setSystemProviderList` 是真实的全表装载（`getProviders` 逐项实例化），经 `AlgorithmId.getName` → `aliasOidsTable` →
  `collectOIDAliases` 入链。入链有两条：①未知接收者 `Object.toString` 派发到 `AlgorithmId.toString`；②`AlgorithmId.<init>`
  → `decodeParams` → `getName`（证书解析，序列化里 `CodeSource.readObject` → `CertificateFactory.generateCertificate`）。
- 三例序列化测试：在 D+S 上再切 ①（`AlgorithmId.toString`，`build/jca/cuts_dst.txt`）仍放行，成因转为 ②：DeepCopy off/new
  均 3131 / 18956，StockTrans 均 3129 / 18940，TSDS 均 3136 / 18950。P+D+S+切 ① = DeepCopy 2766 / 17943、StockTrans 2764 / 17931、TSDS 2771 / 17937，
  与 P+D+S 相同——**序列化三例上 P 的收益只能由「CodeSource 反序列化证书路径」收窄取得，提供者序求值本身取不到**
  （该路径在运行期确会走完全表，本能力放行是正确的）。目标 DeepCopy ≤2803 / StockTrans ≤2807 / TSDS ≤2809 不靠本能力达成。
- Cipher / Mac / EC：JDK 21 上 SunJCE（序号 4）、SunEC（序号 2）在首个非内建表项 SunEC 处或其后，运行期确经装载器，放行正确。
- 本能力的实际收益在「只用 SUN 提供的服务（摘要 / SecureRandom）」的程序上：在 URL（D）与 ResourceBundle（S）收窄落地后
  每例 −69 类 / −344 方法。

### 5.3 顺手修复：`[jca]` 服务种子的求值序依赖

测量中发现 TestRsaSignVerify new+DS 比 off+DS 多 4 类（2606 vs 2602），不单调。根因在既有 `seed_jca`：请求点算法名并入一个与类型
无关的集合，`RSAUtil.getParams` 的 AlgorithmParameters 请求名（RSASSA-PSS 等）若在该类型变为「推不出」之前被求值，就会选中
Signature / KeyFactory / KeyPairGenerator 的同名服务；求值先后随调度变化。修复（1845f494）：请求名按（服务类型, 算法键）登记，
`jca::select` 只按同类型命中（含同义名）。修复后两侧均为 2602 / 15667，其余测试数字不变。

### 5.4 验收

- **顺序无关**：Digester / SecurityDemo（扣住）与 TestRsaSignVerify（放行）在 D+S 上以 `--hash-seed 7`、`--flow-batch 1` 各重跑，
  类集合、方法集合与缺省运行逐项相同（via 可变）。
- **单测**：本机 closure crate 171 通过（含 `jca_order` 5 例、`jca::select` 按类型登记新用例）；服务器全量单测
  `c1dj-ut-1845f494`（sg1，生成器 + rava_macros_core）523 通过 0 失败，rc=0。
- **服务器 spot**（`c1dj-spot-1845f494`，JDK 21）：TestMessageDigestApi、Digester、TestSecureRandomApi、SecurityDemo、
  TestJcaIndirectDigest、TestMacHmacDigest、TestRsaSignVerify、TestEcSignVerify、TestAesGcmRound、TestCipherDesModes、
  DeepCopy、TestSerialDefaultSuid、StockTrans、HelloWorld 14/14 通过（输出与 JDK 一致，无存根命中）。
- **局限**：不叠切除时本能力在全部测试上放行，e2e 闭包与改动前相同；扣住分支的运行期正确性只能在 D（能力①）与 S（能力③）落地后
  由 e2e 覆盖——届时 Digester / TestSecureRandomApi / SecurityDemo 是首批验证例。

## 6. 恢复入口

- worktree `../java_rta_c1djca`，分支 c1d-jca；
- 量类：`build/jca/batch.sh <Test...>`（每例 off / new / offds / newds 四次），结果 `build/jca/<tag>_<Test>.json` 的
  `summary.{classes,methods,jca_order}`。
