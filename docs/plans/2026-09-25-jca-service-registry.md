# K-JCA：JCA 服务注册——算法实现类字节码翻译 + 静态服务表

> 2026-09-25 · 关联：tasks.md S-66 同批语料（DataEncryptionStandard、Digester、SecurityDemo）、
> L-1 纯数据资源束（`docs/plans/2026-09-25-l1-cldr-locale-data.md`，同构的「边界放行 + 种子 + 注册表」三段式）

## 一、问题

`Cipher.getInstance("DES")` / `MessageDigest.getInstance("MD5")` 的 JDK 路径：

```
engine 类（javax/crypto/Cipher、java/security/MessageDigest）
  → sun/security/jca/GetInstance.getInstance / getServices(List<ServiceId>)   ← 内部边界
  → ProviderList → Provider.getService(type, algo) → Provider$Service
  → Service.newInstance → 反射实例化 className（com.sun.crypto.provider.DESCipher、sun.security.provider.MD5）
```

- 实现类在 `com/sun/`、`sun/` 前缀下，按现行边界规则需整体手写——DES / MD5 / SHA 的算法体是纯 Java 计算，
  手写违背「字节码翻译优先」（CLAUDE.md 规则 0/1）。
- 实现类只经 `Class.forName(字符串)` + 反射构造可达，BFS 看不到调用边（与 L-1 资源束同一形态）。
- DataEncryptionStandard 实测：编译已通过，运行命中 `stub: sun/security/jca/ServiceId.<init>`。

## 二、方案（三段式，与 L-1 同构）

| 段 | 内容 | 落点 |
|---|---|---|
| K-JCA-a 服务表 | 从 provider 注册方法字节码抽取 `(type, algorithm, implClass)` 三元组：连续三条 String 常量 ldc、第三条是可加载类名（点形态）。provider 清单在 `runtime/java_runtime/jca_providers.txt`（Python 不出现 JDK 类名，规则 4） | `codegen/jca_services.py` |
| K-JCA-b 种子 | BFS 不动点处：服务类型 ∈ 已达方法的 String 常量（engine 类 `getInstance` 自带 ldc "Cipher"/"MessageDigest"）且 算法名 ∈ 用户类 String 常量（大小写不敏感；transformation `DES/ECB/PKCS5Padding` 取首段）→ 实现类 `<init>` 入队、记为已实例化；触发条件：`GetInstance` 成员被触达（清单 trigger 行） | `callchain._seed_jca_services` |
| K-JCA-c 放行 | 被种子选中的实现类所在**包**（`com/sun/crypto/provider/`、`sun/security/provider/`）内的类，以及 `boundary_release.txt` 列出的 engine/SPI 类（`java/security/MessageDigest`、`MessageDigestSpi`）从边界前缀放行，按字节码翻译；它们对其余内部类（`sun/security/util/*`、`sun/security/jca/*`）的调用仍在边界截断 | `callchain._is_boundary_class` |
| K-JCA-d 注册表 | codegen 发射 `register_jca_services(&[("Cipher", "DES", "SunJCE", ctor), ..])`；手写边界 `sun/security/jca/{GetInstance, ServiceId, ProviderList, JCAUtil}`、`java/security/Provider$Service` 查表构造 | `runtime/java_runtime/src/jca.rs` + 边界手写 |

## 三、不做 / 偏差

- provider 优先级、`Security.addProvider`、第三方 provider：只有 JDK 内建 provider 的静态表。
- 算法别名（OID、`SHA256` 等 KnownOIDs 派生名）：首版只认规范名（大小写不敏感），偏差入 compatibility.md。
- 运行期拼接的算法名（非常量）：静态闭包无法预知 → `NoSuchAlgorithmException`（与未知算法同形态），
  可用 CLI 覆写（后续按需加 `--jca`）。
- JCE 管辖策略（`JceSecurity`）：恒无限制（JDK 9+ 默认 unlimited）。

## 四、验收

- e2e：DataEncryptionStandard（DES 加解密）、Digester（MD5）、SecurityDemo（SHA-256/MD5/SHA-1 + SecureRandom）
- 新增 e2e：未知算法 → NoSuchAlgorithmException 消息与 JDK 一致；同类型多算法；transformation 字符串；
  digest 分段 update / reset / clone
- 单测：服务表抽取、种子匹配（大小写、transformation 首段、类型过滤）
- 验收集 27 例生成树：不触达 JCA 的测试逐字节一致
