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
| 6 | 运行期结构：StackWalker（StackStreamFactory 4 + StackTraceElement 1）、ProcessImpl.forkAndExec + waitForProcessExit0 / getProcessPids0 / Info.info0、NetworkInterface（4）、反射本地访问器 invoke0 / newInstance0、MethodHandleNatives expand / getMemberVMInfo / getNamedCon、Class.getGenericSignature0（需类泛型签名元数据表）、ObjectStreamClass.hasStaticInitializer（需 `<clinit>` 元数据） | 各自方案 | ⬜ |
| — | 字节码生成链（asm ClassWriter / Type / Label、InvokerBytecodeGenerator）、安全管理器链（Policy、FilePermCompat、ReflectUtil.checkProxyPackageAccess）、JFR 事件 | 生成链截断 / SecurityManager 恒 null——设计上不执行，维持存根 | — |

## 四、验证

- 每批：新增 / 相关 e2e（期望输出由 JVM 生成）；手写层编译以 `cargo check` 在本机验证
  （16G 本机完整构建贴 OOM 线，见 N8 / N14），运行期回归由用户机器批量验证。
- 每批合入后重跑 `gap_scan.py api …`，缺口数只降不升。
