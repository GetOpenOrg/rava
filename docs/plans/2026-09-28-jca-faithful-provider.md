# JCA provider 回到字节码（FS-H0 JCA 组）

> 前置：K-JCA 静态服务表（`2026-09-25-jca-service-registry.md`）、FS-R R2 反射访问器管线
>（`2026-09-27-reflection-metadata-table.md`）。

## 一、问题

FS-H0 审计线余项里 JCA 组 17 处越界覆盖：

| 类 | 覆盖 | 原因 |
|----|------|------|
| `Provider` | `getName` 1 | provider 对象是手写「身份」对象（只设 name 字段） |
| `Provider$Service` | 7（getter ×4、`getAttribute`、`supportsParameter`、`newInstance`） | 服务描述由注册表直构；`newInstance` 查注册表构造闭包替代按类名反射 |
| `SecureRandom` | 9 | 缺省 PRNG 直读 `/dev/urandom`，不经 provider 列表 |

副作用：服务属性（SupportedModes / SupportedPaddings / SupportedKeyFormats）恒 null（FS-K2），
`getAttribute` / `supportsParameter` 与 JDK 可观测不一致。

## 二、方案

**provider 对象用真实 Provider 子类构造**（JDK `ProviderConfig` 对内建 provider 同样直接
`new SunJCE()` / `new Sun()`），服务描述 / 属性 / 实现类构造全部走翻译字节码：

1. 清单 `seeds.toml [jca]`：providers 增 `provider = Provider 子类`；放行 `java/security/Provider`
  （含 `$Service` / `$ServiceKey` / `$EngineDescription` / `$UString`）与 `InvalidParameterException`。
2. BFS 种子（`callchain._seed_jca_services`）：入选服务的实现类无参构造器入链并登记按名
   分派面（`REFLECT_CONSTS[impl] ∋ <init>`——`Service.newInstance` 经 `Class.forName` →
   `getConstructor().newInstance()` 落到 dispatch 臂）；其 provider 子类构造器入链。
3. 生成 main：`jca::register_services(&[(类型, 算法, 实现类, provider)])` 只作 provider 选择
   依据；`jca::register_providers(&[(名, 构造闭包)])`。
4. 手写边界 `sun/security/jca/GetInstance`（内部包，合规手写）：provider 列表 = 服务表中登记
   了该 (类型, 算法) 的 provider（按需构造一次并缓存，ProviderList 同样缓存实例）；其后与
   JDK 逐步同构——`Provider.getService`、`Service.newInstance(null)`、`new Instance(..)`，
   首个失败时依次尝试其余服务；`getServices(List<ServiceId>)` 按 JDK `ServiceList` 的
   provider 外层 × id 内层次序收集。
5. 删除 `provider_service_impl.rs` 与 `Provider.getName` 覆盖。

闭包仍按「engine 类在链上 × 用户算法名」只翻译入选实现类；provider 注册方法（`putEntries`
等）里其余服务只是字符串常量，未入选实现类的 `Class.forName` → ClassNotFoundException →
`NoSuchAlgorithmException("Error constructing implementation ..")`，与「类不可加载」的 JDK
行为同形。

## 三、分步

| 步 | 内容 | 覆盖 | 状态 |
|----|------|------|------|
| J1 | Provider / Service 回到字节码（本文二）；补 `SecurityConstants.PROVIDER_VER`、放行 `SecurityProviderConstants` / `KnownOIDs` / file 协议 `Handler` / `IPAddressUtil`；反射构造面补发 `<alloc>` 臂 | −8 | ✅ TestCipherDesModes / Digester PASS |
| J2 | SecureRandom 回到字节码：`getDefaultPRNG` 经手写边界 `Providers` / `ProviderList`（登记序 = 优先序，SUN 在先）取 SUN 的 NativePRNG；删 `secure_random_impl.rs` / `provider_impl.rs`；`GetInstance` 按 provider 名重载 | −9 | ✅ SecurityDemo / TestSecureRandomApi PASS |
| J3 | `JceSecurity`：VM 耦合边界类（closure.toml [vm_boundary]），手写方法改入 `vm_boundary_methods` 单独计数 | 移出越界计数 | ✅ |

J2 的种子补充（`codegen/jca_services.py`）：

- `defaults`：算法名由 JDK 运行期配置得出的缺省服务（`SecureRandom.getDefaultPRNG` → `SecureRandom.NativePRNG`）；
- `alias_sources`：`KnownOIDs.<clinit>` 每个构造调用的字符串实参组 = 一组同义名（"SHA" / "SHA1" → "SHA-1"）；
- 链上 JDK 方法里 `<engine>.getInstance` 调用前的字符串实参（`sun.security.provider.SecureRandom.init` 的
  `MessageDigest.getInstance("SHA", "SUN")`）；
- 入选实现类的**全部**构造器入链：`newInstance` 按服务类型的 `EngineDescription.constructorParameterClass`
  选构造器（SecureRandom 为 `(SecureRandomParameters)`）。

代价：`Cipher.init(opmode, key)` 经 `JCAUtil.getDefSecureRandom()` 真实构造 SecureRandom（JDK 同），
Cipher 类测试的闭包随之含 SUN provider 与 NativePRNG；SecurityDemo 等需 `CARGO_BUILD_JOBS=1` 编译。

## 四、验收

- e2e：TestCipherDesModes、DataEncryptionStandard、Digester、TestMessageDigestApi、SecurityDemo、
  TestSecureRandomApi 全 PASS（期望输出不变）。
- `[raw-audit] non_native_overrides` 按步下降。
