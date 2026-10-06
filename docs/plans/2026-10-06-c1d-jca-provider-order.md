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

（见下文，测量脚本 `build/jca/cl.sh`、`build/jca/batch.sh`，scratch 不提交；`off` = 去掉 `[jca.order]` 的 runtime 副本，即改动前。）

## 6. 恢复入口

- worktree `../java_rta_c1djca`，分支 c1d-jca；
- 量类：`build/jca/batch.sh <Test...>`（每例 off / new / offds / newds 四次），结果 `build/jca/<tag>_<Test>.json` 的
  `summary.{classes,methods,jca_order}`。
