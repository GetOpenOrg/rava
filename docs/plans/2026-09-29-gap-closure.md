# 缺口成批补全（gap_scan 驱动）

> 关联：`scripts/gap_scan.py`（编译前缺口扫描）、每次转译日志的 `[precheck]` 行、
> `docs/reports/gap-scan-api-*.md`（API 模式报告）。

## 一、动机

此前缺口靠「编译 + 运行 → panic 命中一个修一个」发现，每轮约 13 分钟只暴露一个。
gap_scan 在转译后（cargo 之前）按生成产物列出调用链上**全部**落 panic 的成员：

- `native-missing`：调用链上的 native 方法，生成体为 `panic!("native: …")`；
- `boundary-stub`：调用链上 / 触达的边界成员，生成体为 `panic!("stub: …")`。

过滤：签名多态方法（调用点改写承载）、抽象方法、从未实例化类的实例方法（RTA）。
口径仍是调用链可达（BFS 过近似）——HelloWorld 这类通过用例同样有 17 native + 122 存根的
潜在缺口（运行期不执行），因此**不作编译门槛**，用于成批补全。

## 二、扫描面与结果（2026-09-29）

| 报告 | 入口 | 闭包 | native | 存根 |
|---|---|---|---:|---:|
| `gap-scan-api-java.lang-java.util.md` | java.lang + java.util 的 261 个 public 类 / 3630 个方法 | 2475 类 / 40517 方法 | 49 | 207 |
| `gap-scan-api-java.text-…-java.nio.charset.md` | text / time / io / math / stream / concurrent / regex / zip / function / nio / nio.file / nio.charset 的 415 类 / 5477 方法 | — | 60 | 186 |
| 合并去重 | | | 78（26 类） | 235（87 类） |
| 复扫 `gap-scan-api-java.lang-java.util.md`（批次 1–3 后） | 同上 | — | 23 | 193 |

## 三、分类与批次

| 批次 | 内容 | 做法 | 状态 |
|---|---|---|---|
| 1 | ArraysSupport.mismatch 全基本类型族（13）、RandomAccessFile（10） | 手写 native / 边界方法 + e2e（TestArraysMismatchTypes、TestRandomAccessFile） | ✅ `c7a29df` |
| 2 | 语义明确的 VM / 平台 native：VM.getNanoTimeAdjustment、Runtime.max/total/freeMemory / gc、System.mapLibraryName、Class.getProtectionDomain0 / getSigners、BootLoader.getSystemPackageNames、Finalizer.isFinalizationEnabled / reportComplete、CDS.logLambdaFormInvoker、VirtualThread.registerNatives、Array.multiNewArray、ProcessHandleImpl（initNative / getCurrentPid0 / isAlive0 / parent0 / destroy0） | 手写 native | ✅ |
| 3 | zip 压缩：Inflater（11）/ Deflater（10） | FFI 到系统 zlib（JDK 自身即链接 zlib） | ✅（Buffer 族随直接缓冲区实现，保持存根） |
| 4 | 纯 Java 内部类：RandomSupport 族（35）、sun/util/locale（LanguageTag / LocaleMatcher / LocaleUtils / LocaleObjectCache / InternalLocaleBuilder / LocaleExtensions / ParseStatus，约 22）、ICU 规范化 / 双向（NormalizerBase / BidiBase / NormalizerImpl，约 16）、sun/text（约 6）、sun/invoke/util（约 5）、jdk/internal/module Checks / Resources（4） | 优先 closure.toml [release] 放行翻译（「能生成的都走生成器」）；逐包评估闭包增量（N14 内存上限） | 🔄 RandomSupport 族 + sun/util/locale（LanguageTag / InternalLocaleBuilder / LocaleExtensions / Extension / UnicodeLocaleExtension / LocaleSyntaxException / ParseStatus / StringTokenIterator / LocaleMatcher / LocaleEquivalentMaps / LocaleObjectCache / LocaleUtils）已放行（RandomSupport.initialSeed 与 BaseLocale 保持手写），e2e TestRandomStreams / TestLocaleLanguageTag；ICU / sun/text / sun/invoke/util / module Checks 待续 |
| 5 | VM 耦合手写：ClassLoader（16）、BootLoader（6）、Modules（5）、InnocuousThread / VirtualThreads（8）、sun/security/jca/GetInstance（6）、TimeZoneNameUtility（5）、StreamDecoder / StreamEncoder（6，指定字符集名的 Reader / Writer）、System.setOut0 / setErr0 / setIn0 | 手写边界方法 | 🔄 StreamDecoder / StreamEncoder 按字符集名构造 + getEncoding（历史名）、GetPropertyAction 构造器 / run ✅（e2e TestCharsetNamedStreams）；System.setOut0 / setErr0 / setIn0 ✅（批次 2）；其余待续 |
| 6a | 进程派生：ProcessImpl.forkAndExec（fork + execve，PATH 搜索、管道 / 继承 / 文件重定向、合并错误流、错误管道回传 errno，JDK 21 posix_spawn 模式的异常文案）、ProcessHandleImpl.waitForProcessExit0（waitpid / waitid WNOWAIT）、InnocuousThread.newSystemThread / newThread（进程回收线程工厂）、ProcessStartEvent.<init>（JFR 占位） | 手写 native / 边界方法 + e2e（TestProcessBuilder） | ✅（cargo check 零错误；运行期待用户机器验证） |
| 6b | VM 注入的类元数据：ObjectStreamClass.hasStaticInitializer（默认 serialVersionUID 计算）、Class.getPermittedSubclasses0（sealed 反射）——生成器从 classfile 读 `<clinit>` 在场 / PermittedSubclasses 属性写入类级属性 `has_clinit` / `permitted_subclasses`，build.rs 汇总为 class_meta_table；VirtualThread.notifyJvmti*（无 JVMTI → no-op）、Reference.hasReferencePendingList（无 GC → false） | 生成器 + build.rs + 手写 native，e2e TestSerialDefaultSuid | ✅（cargo check 零错误；运行期待用户验证） |
| 6 | 运行期结构：StackWalker（StackStreamFactory 4 + StackTraceElement 1）、ProcessImpl.forkAndExec + waitForProcessExit0 / getProcessPids0 / Info.info0、NetworkInterface（4）、反射本地访问器 invoke0 / newInstance0、MethodHandleNatives expand / getMemberVMInfo / getNamedCon、Class.getGenericSignature0（需类泛型签名元数据表）、ObjectStreamClass.hasStaticInitializer（需 `<clinit>` 元数据） | 各自方案 | ⬜ |
| — | 字节码生成链（asm ClassWriter / Type / Label、InvokerBytecodeGenerator）、安全管理器链（Policy、FilePermCompat、ReflectUtil.checkProxyPackageAccess）、JFR 事件 | 生成链截断 / SecurityManager 恒 null——设计上不执行，维持存根 | — |

## 四、验证

- 每批：新增 / 相关 e2e（期望输出由 JVM 生成）；手写层编译以 `cargo check` 在本机验证
  （16G 本机完整构建贴 OOM 线，见 N8 / N14），运行期回归由用户机器批量验证。
- 每批合入后重跑 `gap_scan.py api …`，缺口数只降不升。

## 五、VM 耦合边界类按方法划分（Python 生成器侧，2026-09-29）

与 Rust 闭包分析器（C1c 第 2 步）及 CLAUDE.md 新口径对齐：`closure.toml [vm_boundary]` 类
（Class / ClassLoader / Module / ModuleLayer / SecurityManager / VirtualThread）不再整类截断——

- native / VM 内建 / 共置手写体提供的方法取手写；其余被调用到的方法按字节码翻译（BFS 照常展开）；
- `<clinit>` 不翻译（类初始化入链跳过，发射层不生成 `__clinit`，`__class_init()` 为 no-op，
  与整类手写时一致）；手写成员照旧计入 `vm_boundary_methods`。
- `[vm_boundary].whole_class`：规模驱动的策略截断（FileSystems / InetAddress / JceSecurity /
  InvokerBytecodeGenerator）——Python BFS 是过近似口径，按方法划分会重新展开名字服务 / JCE 策略等
  子系统，仍整类截断；Python 改为消费 closure.json 后随 C1d 删除。Rust 分析器不读此键。
- 随之暴露并补齐：`ClassLoader.findBootstrapClass` / `findLoadedClass0`（native）、
  `PerfCounter` 计数器取值族（jvmstat 占位）；迟至静态边补扫的门 3 增加字段臂（读写不在闭包内的
  其它边界类字段的方法保持存根）。
- 规模（JDK 21）：HelloWorld 2145 → 2198 个生成文件（+2.5%），TestProcessBuilder 2210 → 2263；
  `ClassLoader.loadClass` 族、`Class.isEnum` / `isAnnotation`、注解数据链等由存根变为字节码翻译。

## 六、剩余缺口的去向（2026-09-29 复扫后）

复扫：java.lang + java.util native 49 → 23、存根 207 → 193；java.text 等 14 包 native 60 → 33、存根 186 → 155。

| 缺口 | 去向 | 理由 |
|---|---|---|
| TimeZoneNameUtility（Date.toString / 格式串 `z` 的时区名）、ICU 规范化 / 双向（Normalizer / Bidi / Collator 链的 sun/text） | C1d 放行 `sun/util/locale/provider`、`jdk/internal/icu`、`sun/text` | 纯 Java 逻辑（CLDR metazone 映射、ICU 数据表）；按「手写只留 VM 契约层」不手写复刻，待精确闭包下实测放行增量 |
| zip 直接缓冲区族（Adler32 / CRC32 / Deflater / Inflater 的 ByteBuffer 版，10 个） | 随直接内存（Unsafe.allocateMemory）一并实现 | rava 尚无直接内存，这些 native 运行期不可达 |
| NativeMethodAccessorImpl.invoke0 / NativeConstructorAccessorImpl.newInstance0 | 不做 | JDK 21 反射调用 native 方法走 DirectMethodHandleAccessor$NativeAccessor（已手写）；旧式访问器仅在关闭 useDirectMethodHandle 时使用 |
| StackWalker（StackStreamFactory 4 + StackTraceElement.initStackTraceElement） | 暂缓 | 需要真实 Java 帧（方法名 / 调用方类）；rava 栈回溯为 Rust 栈近似，输出难与 JVM 一致 |
| Class.getGenericSignature0 | 随 C1d 放行 sun/reflect/generics | native 本身简单（类级 generic_signature 属性已在），但消费方泛型 visitor 体系未放行（曾使 java_runtime 编译峰值越过 15G） |
| MethodHandleNatives expand / getMemberVMInfo / getNamedCon | 不做 | MH-native 模型替代；getNamedCon 只在断言校验路径 |
| NetworkInterface | 暂缓 | 语料无网络接口枚举 |


## 七、native 补全（语料扫描驱动，2026-09-30）

扫描口径改为 Rust 闭包分析器：`scripts/native_gap_scan.py` 对 tests/e2e 全语料逐例 `rava closure`
（1080 例，每例约 1 秒），与 runtime/ 手写 fn 交叉比对，报告 `docs/reports/native-gap-scan.md`。
（gap_scan.py 的 API 模式全量种子在 16G 本机 OOM，不再作为主口径。）

| 批次 | 内容 | 提交 |
|---|---|---|
| 7a | ProcessHandleImpl.getProcessPids0 / Info.info0 / Info.initIDs（/proc），isAlive0 / parent0 返回与校验真实启动时刻；e2e TestProcessHandleInfo | d0ee300 |
| 7b | 直接内存：`native_memory` 基础模块 + Unsafe allocateMemory0 / reallocateMemory0 / freeMemory0 / setMemory0 / copyMemory0 / copySwapMemory0 及包装、get/put(Object,long) 直接内存分支、ScopedMemoryAccess 批量操作；zip 直接缓冲区族（CRC32 / Adler32 / Inflater / Deflater 的 Buffer 变体）；e2e TestDirectBuffer | d0ee300 |
| 7c | NetworkInterface 全部 native（getifaddrs + /sys/class/net）、Inet4Address / Inet6Address.init；NativeConstructorAccessorImpl.newInstance0 / NativeMethodAccessorImpl.invoke0；VarHandle.getAndAdd 族；StaticProperty 公开访问器；e2e TestNetworkInterface | a8f7698 |
| 7d | JCA 参数对象：`java/security/AlgorithmParameters` / `AlgorithmParametersSpi` / `Security` / `spec/InvalidParameterSpecException` 放行按字节码翻译（seeds.toml [jca]）。Security.<clinit> 读 `<java.home>/conf/security/java.security`：jdk_resources 以嵌入的生效属性文本应答（`lookup` + `getBooleanAttributes0` 视为存在的普通文件），Properties.load 按字节码解析；`getImpl` / `getSpiClass`（Class.forName 按名取 SPI）走字节码。随放行删除 `security_impl.rs`（手写 getProperty / setProperty 成为非 native 覆盖，`non_native_overrides` 回到 0）。补手写：GetInstance.getInstance(Service, Class) / (…, Provider)、GetInstance$Instance.toArray、SharedSecrets.setJavaSecurityPropertiesAccess、SecurityPropertyModificationEvent.<init>、EventHelper.isLoggingSecurity（恒 false）；e2e TestCipherAlgorithmParameters（cargo check 零错误；运行期待用户验证）。闭包增量：TestCipherDesModes 1170 → 1177、TestSecureRandomApi 1111 → 1115、SecurityDemo 1115 → 1119 | 本提交 |

native 缺口 16 → 6。剩余 6 个均为静态可达、运行期不执行：

| native | 触达路径 | 结论 |
|---|---|---|
| StackStreamFactory.callStackWalk / checkStackWalkModes、StackTraceElement.initStackTraceElement | ThreadLocal.dumpStackIfVirtualThread（仅 `-Djdk.traceVirtualThreadLocals`） | 暂缓（需真实 Java 帧） |
| MethodHandleNatives.expand / getMemberVMInfo / getNamedCon | MemberName.expandFromVM（type 已由手写 resolve 填好时直接返回）/ 断言校验路径 | 不做（MH-native 模型替代） |

另：6 例（DeepCopy / StockTrans / RecordsSerializationTest / SerializableDemo / TestSerialDefaultSuid /
TestSerializationPrimitives）的 `rava closure` 超过 120 秒，未纳入统计——序列化闭包的分析耗时问题，转闭包分析器。
