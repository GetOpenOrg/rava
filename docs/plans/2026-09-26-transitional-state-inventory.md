# 过渡态清单：未按最终态实现的全部内容（2026-09-26）

> **来源**：用户要求列出所有「只实现了过渡状态、没按最终态实现」的内容，形成记录。
>
> **口径**：满足以下任一条即列入。
> - 文档或代码注释标明为「过渡 / 临时 / 近似 / 档位 / 不建模 / 恒真 / 占位 / no-op」。
> - 违反 CLAUDE.md 原则 0、原则 1 或命名原则 3。
> - 行为与 JVM 不等价（compatibility.md 中的非「等价」行）。
>
> **编号 FS-xx**（Final-State）。各列含义：
> - 「可观察差异」：与 JVM 的行为差别；无行为差别时写「仅架构」。
> - 「既有任务」：文档里已有的编号，没有则写「新立」。
>
> **统计**：共 117 项。其中既有记录 66 项（含部分覆盖），**无记录 51 项，本文件起立项**。
>
> 抽查已核实三条（2026-09-26）：`Math.random()` 恒为 0.5（math_impl.rs:26）、`Character.digit` 只认 ASCII、`_nf_covered` 覆盖机制没有「仅 native」约束。
>
> **推进口径**
> - 每项完成后从本表划掉，并在 `docs/tasks.md` 归档。
> - 验收按原则 6：写 e2e 测试，期望输出由 JVM 生成。

## 〇、根因项（优先堵住）

| # | 现状 | 最终态 | 可观察差异 | 既有任务 |
|---|---|---|---|---|
| **FS-H0** | 公开 API 类的 `_impl.rs` 覆盖机制没有「只许 ACC_NATIVE」的约束。`codegen/emitter/class_writer.py` 的 `_nf_covered`：只要同名 fn 存在就跳过字节码翻译，非 native 方法被悄悄替换，且没有审计计数 | 公开包 `_impl.rs` 只许 native，违例计数为 0 且进入 `[raw-audit]` | FS-N1..N5、FS-H1..H9 都源于此 | 新立 |

### FS-H0 进度与余项明细（`[override-audit]`，2026-09-26）

审计线（fb4141a）：`class_writer.py` 登记每处非 native 方法的手写覆盖，`[raw-audit] non_native_overrides=N` 汇总、`[override-audit]` 逐位点输出（`scripts/main.py`）。VM 内建函数经 `runtime/java_runtime/intrinsics.txt` 准入，单独计 `intrinsics=`，不算违例。

| 批次 | 内容 | 计数 |
|---|---|---|
| fb4141a | 删除 Math / StrictMath 全部越界覆盖（FS-N1..N5）；StrictMath.sqrt 入内建清单（1c7c5e1） | → 65 |
| f675869 | 删除 Character.digit、Thread.interrupt / isTerminated / getThreadGroup；AtomicInteger(int) 改为调用点 upcalls 声明 | 65 → 60 |

余 60 处（ComprehensiveTest / TestMathSpec 调用链口径），按根因分组：

| 组 | 覆盖点 | 根因 / 去除路径 | 对应项 |
|---|---|---|---|
| 反射元数据 | `Class.` cast / descriptorString / getAnnotation(s) / isAnnotationPresent / getClassLoader / getConstructor(s) / getDeclaredConstructor(s) / getDeclaredField(s) / getDeclaredMethod(s) / getFields / getMethod / getModule / getPackageName / isMemberClass / isRecord（21）；`Constructor.` getAnnotation / newInstance，`Field.` getAnnotation / getDeclaredAnnotations，`Method.` getAnnotation / getDeclaredAnnotations / invoke（7） | JDK 字节码经 ReflectionFactory / 注解解析器读 class 文件字节；原生二进制无常量池——需「反射元数据表」生成后以 native（getDeclaredFields0 等）承载，公开方法回到字节码 | FS-R 组 |
| 类加载器 / 模块 | `ClassLoader.` getParent / getResource(s) / getSystemClassLoader / getSystemResource(s)（6）；`Module.` canUse / isNamed（2） | 需内建类加载器对象图（BuiltinClassLoader 边界类）+ 静态资源表 | FS-C 组 |
| 运行时字节码生成 | `InvokerBytecodeGenerator.` generateCustomizedCode / generateLambdaFormInterpreterEntryPoint / isStaticallyInvocable×3 / lookupPregenerated（6） | 原生二进制不能在运行时定义类；最终态为「LambdaForm 解释执行」路径（等价于 HotSpot 关闭编译时的解释入口），届时这组改为内建清单并注明理由 | FS-M 组 |
| 安全框架 | `AccessController.` doPrivileged / getContext，`Permission.<init>`、`BasicPermission.<init>`，`SecureRandom.<init>`×2（6） | 前四个可直接回到字节码（SecurityManager 恒 null）；SecureRandom 需 Provider 服务表（FS-C4） | FS-C4 |
| I/O | `FileCleanable.` register / unregister，`FileSystems.getDefault`（3） | Cleaner 线程（FS-G 组）与 DefaultFileSystemProvider 边界类 | FS-G / FS-I |
| 数值 | `Double.toString`（FS-N6，DoubleToDecimal）、`Integer.valueOf`（IntegerCache.<clinit> 链） | 回到字节码 | FS-N6 |
| 集合 | `ArrayList.` add(E,Object[],int) / elementData / get，`Arrays.copyOf(T[],int)`，`ConcurrentHashMap.<init>()` | ArrayList：泛型 E 载体与 checkcast；copyOf：组件类型（FS-R6）；CHM：transfer 的变量提升缺口（codegen/method/vars.py hoist） | FS-R6 / 新立 |
| 其他 | `Enum.valueOf`（enumConstantDirectory 反射链）、`VirtualThread.<init>`（FS-T4）、`DecimalFormatSymbols.initializeCurrency`（Currency 数据文件） | 各随所属项 | FS-T4 等 |

## 一、线程 / 并发

| # | 现状 | 最终态 | 可观察差异 | 既有任务 |
|---|---|---|---|---|
| FS-T1 | 第一档 OS 线程 + GIL：同一时刻只有一个线程执行 Java，线程交错只落在安全点 | 第二档：Arc + 原子单元，去掉 GIL，真并行 | 没有并行加速；交错是 JMM 合法执行的一个子集 | #42 第二档（**进行中**，4fa1d48 起） |
| FS-T2 | 对象模型原语是单线程后端的类型别名；`__GilStatic` 靠 `unsafe impl Sync` | feature `mt`：Arc / 原子 / 读写锁 / OnceLock | 仅架构 | #42（进行中） |
| FS-T3 | `Runtime.availableProcessors()` 恒返回 1（runtime_impl.rs，注释仍写「协作调度」） | 返回真实核数，ForkJoinPool commonPool 按并行度运行 | 返回值是 1；CompletableFuture 走 ThreadPerTaskExecutor；并行流退化 | 新立（随 #42 第二档） |
| FS-T4 | 虚拟线程 = OS 线程，Continuation / 载体线程 / 容器登记都不建模 | Continuation 建模 | 无法创建海量虚拟线程（每条保留 256MiB 栈）；toString / 载体信息不同 | #42 / compat 虚拟线程行 |
| FS-T5 | InternalLock 所有实例共用一把全局可重入锁，unlock 不核对实例 | 按实例加锁（翻译 JDK 的 ReentrantLock 包装） | 不同流之间伪互斥（性能问题；理论上可能死锁） | S-11 |
| FS-T6 | Thread 的 `setPriority0`、`setNativeName`、`getThreads`、`dumpThreads`、`getStackTrace0`、`scopedValueCache` 等 native 缺失 | 实现 | `setPriority` / `getAllStackTraces` 等命中 panic 存根 | 新立（按需） |
| FS-T7 | Unsafe / VarHandle 的 CAS 族以「GIL 下读-比-写不可分割」的普通单元承载 | 原子单元 / 引用槽写锁内的读-比-写 | 仅架构（GIL 下等价） | #42（**4fa1d48 已改为原子**，待验证） |

## 二、对象模型与内存

| # | 现状 | 最终态 | 可观察差异 | 既有任务 |
|---|---|---|---|---|
| FS-M1 | 可读层的禁用调用还没清零：`from_any` 残余、`Into::<I>::into` 约 1.7 万处、闭包 `Rc::new`；translation-reference §16.3 状态表过期 | 全部为 0 | 仅架构（可读性） | A-2 / A-4 |
| FS-M2 | 接口载体化按名单铺设（Spliterator 族暂缓）；`signature_erased_interfaces.txt` 让 CharSequence 仍擦成 Object | 名单置 None，删除两个文件 | 仅架构 | A-4 / T-2 |
| FS-M3 | 抽象类、枚举、手写类没有 `__interface` | 全部由宏生成 | 接口查询抛 AbstractMethodError | A-6 |
| FS-M4 | 原生值盒进 Object，`is_instance_of` 只认精确包装类；`JvmRef` 是 Arch-1 之前的过渡物 | 只保留翻译出的包装类对象，删除 JvmRef | 原生盒 `instanceof Number / Comparable` 为 false；不走 IntegerCache | S-3 / T-4（instanceof 缺口新立） |
| FS-M5 | identity hash 取实例地址截断成 i32 | 31 位非负伪随机 hash（HotSpot 语义） | `hashCode()` 可能为负；低位恒为 0，分布差 | 新立 |
| FS-M6 | `Object.equals` 保留 String 内容比较的捷径 | 纯引用比较 | 仅架构 | S-6 / P-2 |
| FS-M7 | 对 null 接收者 getfield / putfield 不抛 NPE（宏访问器没有 null 检查） | 抛可捕获的 NPE | 读 null 对象字段得到默认值 | S-9 |
| FS-M8 | `_is_jnull` 对非 Object 载体恒为 false；codegen 用类型名启发式；手写层多处踩坑 | 统一经 `is_jvm_null` 钩子 | 类型变量实例化后 null 判定为假，走错分支 | 新立 |

## 三、反射 / MethodHandle / Lambda

| # | 现状 | 最终态 | 可观察差异 | 既有任务 |
|---|---|---|---|---|
| FS-R1 | `java/lang/Class` 整类手写（vm_boundary，962 行，含非 native 方法） | 翻译字节码 + 反射数据注册表，只手写 native | 各方法语义都是手写近似 | 新立 |
| FS-R2 | `Class.forName` 没有加载器层级；`initialize=true` 延迟初始化 | 行为等价 | 见 compat | S-66 |
| FS-R3 | `getFields` 中接口常量按超类型闭包顺序枚举 | 按直接超接口递归 | 返回顺序不同 | S-66 |
| FS-R4 | 反射访问检查拿不到调用方，近似为「用户类成员一律可达」（925bd5b 已让 getCallerClass 能识别用户帧，可以接上了） | 按调用方判定（同类 / 同包 / nestmate） | 跨类访问 private 不抛 IllegalAccessException | 新立 |
| FS-R5 | `Field.get/set` 对 S/B/C/F/D 实例字段与无 ConstantValue 的静态字段是存根；非表构造的 Method / Constructor 是存根 | 全形态实现 | panic | 部分（remaining-issues 反射族） |
| FS-R6 | `reflect.Array` 只有 `newArray`，引用数组恒为 `Object[]`，基本类型分支与 get / set / getLength / multiNewArray 缺失 | 按运行时组件类型建数组 | `Array.newInstance(String.class,n).getClass()` 为 `Object[]` | 新立 |
| FS-R7 | MethodHandle 靠 LambdaForm 解释器执行；字段句柄与 Unsafe 成员有缺口时 panic | 组合子全集 + record 序列化 | 预生成物种外不可达 | N11 / #40 |
| FS-R8 | Lambda 对象的 `__class_name` 报接口 binary name | `Outer$$Lambda/...` 这类隐藏类身份 | `getClass().getName()` / toString 不同 | 新立 |
| FS-R9 | 注解实例是最小合成物；Proxy 不支持 | — | Proxy 不可用 | compat §1（声明为不可等价） |
| FS-R10 | 非数组类字面量 / getClass 的同一性未实测 | 每类唯一的 Class | — | S-5 |

## 四、I/O 与文件系统

| # | 现状 | 最终态 | 可观察差异 | 既有任务 |
|---|---|---|---|---|
| FS-IO1 | POSIX 档 A：`access` 的 R/W/X 近似为 F_OK | 档 B：`faccessat` | isReadable 等会误报 | #15（按需） |
| FS-IO2 | `checkAccess0` 宽容近似；`getSpace0` 恒 0；`getNameMax0` 恒 255 | 真实的 access / statvfs / pathconf | `canWrite` 误报；`getFreeSpace` 恒为 0 | 部分（#15） |
| FS-IO3 | UnixPath 的 macOS NFD 用恒等处理；relativize / toUri / 迭代器是存根 | 完整实现 | macOS 非 ASCII 路径比较不同；调用即 panic | #15 |
| FS-IO4 | `FileSystems` 整类手写 | 翻译字节码 | 自定义 FileSystemProvider / zipfs 不可用 | 新立 |
| FS-IO5 | FileChannel 直连 std；Unsafe 没有 allocateMemory | lock / map / off-heap | `allocateDirect` / `map` / `lock` 命中存根 | 新立 |
| FS-IO6 | 伪 `java.home="/java-runtime"`，只嵌入 tzdb.dat 等少数文件 | — | `java.home` 不同；读取其他 JDK 资源失败 | 新立 |

## 五、字符集 / Locale / 格式化

| # | 现状 | 最终态 | 可观察差异 | 既有任务 |
|---|---|---|---|---|
| FS-L1 | 只有静态可见的 locale 入选 | — | 运行期拼出的 locale 回落 ROOT | L-1 |
| FS-L2 | Currency 数据层没建模，手写地区表 | 翻译 CLDR 货币数据 | 其余地区给 `XXX` / `¤`；`getCurrency()` 为 null | L-1 |
| FS-L3 | CalendarDataUtility 是手写数据表，非 en 语言回落 ROOT | 翻译 FormatData 束 | de/fr/ja/zh 的月份、星期名为英文 | 新立 |
| FS-L4 | LocaleProviderAdapter 单适配器 | — | `java.locale.providers` 无效 | 新立 |
| FS-L5 | 只有 9 个标准 charset，全部手写 | 翻译 `sun.nio.cs.*` | windows-1252 等抛 UnsupportedCharsetException | 新立 |
| FS-L6 | StreamEncoder / Decoder 手写，出错动作固定为 REPLACE；`newStringNoRepl` 遇非 UTF-8 输入 panic | 翻译编解码器 | REPORT 语义丢失；panic | 新立 |
| FS-L7 | System.out / err 编码固定 UTF-8 | 跟随 `stdout.encoding` | C/POSIX locale 下输出字节不同 | 新立 |
| FS-L8 | FloatingDecimal 用近似判定，`digitsRoundedUp` 恒为 false | 精确语义 | DecimalFormat 舍入边界可能不同 | 新立 |
| FS-L9 | JDK25 下 `DoubleToDecimal.split` / FloatToDecimal 是存根 | 实现 | JDK25 下 `%f/%e/%g` panic | tasks P1 |
| FS-L10 | `hashCodeOfUTF16` 未实现 | — | JDK25 路径 | #18 |
| FS-L11 | String.hashCode 的 UTF16 分支硬编码大端 | 按平台字节序 | 非 Latin1 字符串 hash 不同 | S-21 |

## 六、安全 / JCA

| # | 现状 | 最终态 | 可观察差异 | 既有任务 |
|---|---|---|---|---|
| FS-K1..K6 | 算法按静态可见入选；别名 / 服务属性未登记；JCE 策略恒 unlimited；`SecureRandom.getInstance(算法)` 是存根；不支持第三方 provider | — | 见 compat JCA 行 | K-JCA |
| FS-K7 | `java/security/` 整包是边界，Permission 不建模 | — | 仅架构（JDK24+ 已删除 SecurityManager） | 新立 |
| FS-K8 | 没有 `java.security` 配置层：Debug 恒 null，`CryptoAlgorithmConstraints.permits` 恒 true | 读取配置 | `disabledAlgorithms` 等不生效 | 新立 |

## 七、类加载 / 模块 / 服务

| # | 现状 | 最终态 | 可观察差异 | 既有任务 |
|---|---|---|---|---|
| FS-C1 | 自定义 ClassLoader / Agent / 运行期生成类不支持 | 声明为不可等价 | — | compat §1 |
| FS-C2 | ClassLoader 是恒等对象，资源查询恒缺席；`getClassLoader` 恒返回 null | 分层加载器 + classpath 资源 | `getResource` / `getResourceAsStream` 恒为 null | 部分 |
| FS-C3 | 全局只有一个无名模块（JDK 类也在其中）；模块访问检查恒真 | — | `String.class.getModule().getName()` 为 null（JVM 给 "java.base"） | 新立 |
| FS-C4 | 服务目录恒为空 | 静态服务表（含用户 `META-INF/services`） | ServiceLoader 找不到任何 provider | 新立 |
| FS-C5 | `forName(..., true)` 不立即初始化 | — | 初始化时机不同 | S-66 |

## 八、异常与栈回溯

| # | 现状 | 最终态 | 可观察差异 | 既有任务 |
|---|---|---|---|---|
| FS-E1 | 栈帧内容是 Rust 符号 | 真实的 Java 帧元数据 | printStackTrace / StackTraceElement 内容不同 | S-19 #5（声明为边界） |
| FS-E2 | `getExtendedNPEMessage` 返回空 | JEP 358 helpful 消息 | `NPE.getMessage()` 为 null | 新立 |
| FS-E3 | 不可捕获的 panic 路径：downcast / checkcast / JArray 的 NPE·CCE / wrapper `From<Object>` 失败 / `__obj_str` 中 toString 抛异常 | 全部以 `Err(JvmError)` 传播 | CCE / NPE / toString 中的异常无法 catch，进程 panic | 部分（S-1 / S-9；toString 路径新立） |

## 九、数值 / 浮点（均源于 FS-H0）

| # | 现状 | 最终态 | 可观察差异 | 既有任务 |
|---|---|---|---|---|
| **FS-N1** | **`Math.random()` 恒返回 0.5**（math_impl.rs:26；Math 没有 native，这是覆盖字节码） | 删除手写，走翻译链 | 所有随机逻辑的结果恒定 | 新立 |
| FS-N2 | `IEEEremainder` 用 `round`（应为 rint） | 删除手写 | `IEEEremainder(5,2)` 得 -1.0（JVM 为 1.0） | 新立 |
| FS-N3 | `pow` 走 Rust 的 `powf` | 翻译 StrictMath / FdLibm | `pow(1,NaN)`、`pow(-1,±∞)` 得 1.0（Java 为 NaN） | 新立 |
| FS-N4 | sin/cos/tan/exp/log/… 走 Rust libm | 走 FdLibm 字节码链（S-19 已按此处理 rint / expm1） | 末位可能差 1ulp | 新立 |
| FS-N5 | 双下划线命名的重载（`round__f`、`nextUp__d` 等）与 mangle 规则不符，是死代码且本身有误 | 删除 | 仅架构 | 新立 |
| FS-N6 | `Double/Float.toString` 手写重排 Rust 格式化结果 | 翻译 DoubleToDecimal | 仅架构（已对拍） | 部分（S-19 #4 / T-4） |

## 十、GC / 弱引用 / Finalization

| # | 现状 | 最终态 | 可观察差异 | 既有任务 |
|---|---|---|---|---|
| FS-G1 | 没有 GC（Rc 循环永不释放，驻留表只增不减）；`Runtime.gc/freeMemory/totalMemory/maxMemory` 缺失 | 回收机制 + 内存查询 | 内存持续增长；调用即 panic | 部分（compat「无 GC」） |
| FS-G2 | 弱 / 软 / 虚引用不清除、不入队 | — | WeakHashMap 条目不消失 | compat 近似行 |
| FS-G3 | finalize 不触发 | — | finalize 不执行 | compat |
| FS-G4 | Cleaner / `FileCleanable.register` 是 no-op | — | 未 close 的 fd 泄漏；Cleaner 动作不执行 | 新立 |
| FS-G5 | 监视器 / park 侧表条目不回收，地址键可能被复用 | 对象头或弱键 | 内存增长；理论上身份冲突 | 新立 |

## 十一、生成器质量

| # | 现状 | 最终态 | 可观察差异 | 既有任务 |
|---|---|---|---|---|
| FS-Q1 | RawExpr/RawStmt 逃生舱：静态构造点 193 处 | 0，全部类型化 IR | 仅架构 | P2 IR 结构化收敛 |
| FS-Q2 | Python 侧 JVM→Rust 类型映射 | 宏自行决策 | 仅架构 | M-3 |
| FS-Q3 | 泛型 bounds 由 Python 计算 | 进宏 | 仅架构 | T-1 |
| FS-Q4 | 方法级类型变量擦成上界或 Object；turbofish 推断失败以 Object 兜底（违反命名原则 3） | 生成 Rust 泛型参数 | 仅架构 | N4 G2 |
| FS-Q5 | TypeIR 的 G1 / G3 / G4 / G5 能力缺口 | 完全体 | 仅架构 | N4 |
| FS-Q6 | fallback B 组 15 处静默降级；A 组 CfgError 转 panic 存根 | 默认 STRICT，零兜底 | 丢类后运行期命中存根 | fallback-audit 报告 |
| FS-Q7 | 不可归约 CFG 的状态机兜底从未实战 | 补单元测试 | — | S-13 |
| FS-Q8 | 无调试信息时 try 区域布局偏差 | — | 仅代码形状 | S-12 |
| FS-Q9 | 调用链外的 toString / hashCode 存根返回默认值（类名 / 0）而非 panic（违反原则 2） | 统一 panic 存根 | 静默输出类名或 0 | 部分（S-21） |
| FS-Q10 | 未翻译接收者的残余分派发射 `Default::default()` | AbstractMethodError 或正确分派 | 静默得到默认值 | 新立 |
| FS-Q11 | 不支持的 invokedynamic（enumSwitch / condy / EnumDesc 标签 / 缺 impl 的方法引用）发射 `/* TODO */` + `Object::default()` | 翻译这些 bootstrap | 静默 null | 部分（S-17） |
| FS-Q12 | record 的 toString/hashCode/equals 靠查找文本标记替换；makeConcat 直接发射 `format!` | 翻译 ObjectMethods / StringConcatFactory | 仅架构 | 新立 |
| FS-Q13 | 可见性映射：private / package 一律为 `pub(crate)` | 精确映射 | 仅架构 | 部分 |
| FS-Q14 | 遗留死代码与死分支（宏 `let _ = (...)`、未用导入、装箱类擦除死分支） | 删除 | 仅架构 | G-5 / G-6（部分新立） |
| FS-Q15 | `native_upcalls.provides` 按名前缀近似匹配 | 按 mangle 精确匹配 | 仅架构 | 新立 |
| FS-Q16 | 翻译质量小项：mut 标注递归 / 括号优化 / 布尔压缩 / 语义桩计数 / 类级并行 | — | 仅架构 | T54 / T60 / T70 / T72 / T65 |
| FS-Q17 | Debug 由宏生成 | 改用 `#[derive(Debug)]` | 仅架构 | R-3 |

## 十二、手写替代字节码翻译的类（违反原则 0 / 1）

| # | 现状 | 最终态 | 可观察差异 | 既有任务 |
|---|---|---|---|---|
| FS-H1 | `Character.digit` 只认 ASCII | 翻译 CharacterData | `Character.digit('٣',10)` 得 -1；`parseInt("１２３")` 抛 NumberFormatException | 新立 |
| FS-H2 | `Integer.valueOf(int)` / toString 手写 | 走生成 | 仅架构 | T-4 |
| FS-H3 | ArrayList 的私有助手手写 | 翻译 | 仅架构 | 新立 |
| FS-H4 | `Arrays.copyOf(T[],int)` 手写，结果恒为 `Object[]` | 翻译 | 结果 `getClass()` 为 `Object[]`；存异类元素不抛 ArrayStoreException | 新立 |
| FS-H5 | `ConcurrentHashMap()` 初始容量设为 1024，用来避开扩容路径的生成缺口 | 修好 vars 提升缺口，删除伴生 | 条目超过 768 后进入有缺陷的扩容路径 | 新立 |
| FS-H6 | `Properties.getProperty` 手写，忽略 defaults 链 | 翻译 | `new Properties(defaults).getProperty(k)` 不回落 | 新立 |
| FS-H7 | `AtomicInteger(int)` 构造器手写 | 翻译 | 仅架构 | 新立 |
| FS-H8 | `Enum.valueOf` 靠常量目录手写 | — | 泛型上下文可能抛 CCE | S-15 / A-1 |
| FS-H9 | Thread 的 getThreadGroup / interrupt / isTerminated 这些非 native 方法手写 | 翻译 | 仅架构 | 新立 |
| FS-H10 | Object / ObjectVTable 整类手写 | vtable 从 `Object.class` 翻译 | 仅架构 | Arch-4 |
| FS-H11 | `string_ext.rs`、驻留表手写 | 由字节码承载 | 仅架构 | P-2 |
| FS-H12 | vm_boundary 里的公开包类整类手写：Class / ClassLoader / Module / ModuleLayer / VirtualThread / SecurityManager / FileSystems / JceSecurity / InvokerBytecodeGenerator | 缩小到只手写 native | 见 FS-R1 / C2 / C3 / IO4 | 新立 |
| FS-H13 | jdk/internal 纯 Java 类整类重写：DoubleToDecimal / FloatToDecimal / FloatingDecimal / DecimalDigits / Preconditions 等 | 经 `boundary_release.txt` 放行翻译 | 基本仅架构 | P-3 |

## 十三、进程 / 环境 / 系统属性

| # | 现状 | 最终态 | 可观察差异 | 既有任务 |
|---|---|---|---|---|
| FS-P1 | 系统属性只有约 10 个键，不能用 `-D` 注入 | 完整的 initPhase1 属性集 | `java.version` / `os.version` / `java.class.path` 等返回 null | 新立 |
| FS-P2 | `Shutdown.halt0` 等未实现 | 实现 | `System.exit` / `Runtime.halt` 命中存根 | 新立 |
| FS-P3 | `ProcessEnvironment.environ` 未实现 | 实现 | `System.getenv` 命中存根 | 新立 |
| FS-P4 | `forkAndExec` 未实现 | 实现 | 无法启动子进程 | 部分 |
| FS-P5 | `desiredAssertionStatus` 恒为 false，没有 `-ea` | 支持断言开关 | assert 永不执行 | 新立 |

## 十四、其他（文档 / 宏遗留 / 治理）

| # | 现状 | 最终态 | 可观察差异 | 既有任务 |
|---|---|---|---|---|
| ~~FS-O1~~ ✅ | compatibility.md 多行状态过期（浮点、Object.wait、null 数组、record hashCode、NegativeArraySize、类初始化、泛型擦除身份） | 刷新 | 仅文档 | 新立 |
| ~~FS-O2~~ ✅ | 注释失真（class_init「thread_local」、多处「协作档位」、monitor 中断「未实现」、thread_impl getState） | 刷新 | 仅文档 | 新立 |
| ~~FS-O3~~ ✅ | `#[java_synchronized]`（函数级静态 Mutex）、`java_switch!`（equals 链）自称过渡且 codegen 不调用，但 reference 仍列为主力 | 删除或修正文档 | 死代码 | 新立 |
| FS-O4 | 等价审计只计数，结构化 NearApprox 告警与每类等价等级报告未做 | 完成 compat §2 / §4 契约 | 近似场景静默编译通过 | compat §2 / §4 |
| FS-O5 | System.Logger 返回静默 Logger | 默认 SimpleConsoleLogger | INFO 及以上日志不输出 | 新立 |
| FS-O6 | JFR / PerfCounter / CDS / ClassFileDumper 是 no-op 占位 | — | JFR 不可用 | 新立 |
| FS-O7 | JUnit crate 的 workspace path 依赖 | 版本化的 `java-runtime-core` | 仅架构 | M5 / R9 |
| FS-O8 | 生成器仍是 Python | 单二进制 `java-rta` | 仅架构 | #10 / R0 |
| FS-O9 | 手写 `_impl` 分配的对象不进 RTA | — | 覆盖方法可能是存根 | N6（待推送） |
| FS-O10 | 对象序列化：record 路径 / 反序列化实例化 | — | — | S-66 / N2 |
| FS-O11 | 跨包同简单名类撞名 | — | — | S-14 |

## 汇总

| 主题 | 条目数 | 已有记录（含部分） | 无记录（本文件立项） |
|---|---|---|---|
| 线程 / 并发 | 7 | 5 | 2 |
| 对象模型与内存 | 8 | 6 | 2 |
| 反射 / MethodHandle / Lambda | 10 | 6 | 4 |
| I/O 与文件系统 | 6 | 3 | 3 |
| 字符集 / Locale / 格式化 | 11 | 5 | 6 |
| 安全 / JCA | 8 | 6 | 2 |
| 类加载 / 模块 / 服务 | 5 | 3 | 2 |
| 异常与栈回溯 | 3 | 2 | 1 |
| 数值 / 浮点 | 6 | 1 | 5 |
| GC / 弱引用 / Finalization | 5 | 3 | 2 |
| 生成器质量 | 17 | 14 | 3 |
| 手写替代字节码翻译的类 | 14 | 5 | 9 |
| 进程 / 环境 / 系统属性 | 6 | 1 | 5 |
| 其他 | 11 | 6 | 5 |
| **合计** | **117** | **66** | **51** |

## 建议推进顺序

1. **FS-H0 + FS-N1..N5 + FS-H1 / H4 / H6**：先给覆盖机制加「仅 native」约束和审计线，再删除越界的手写覆盖，让这些方法改走字节码翻译。这一步同时消除 `Math.random` 恒为 0.5、`pow` / `IEEEremainder` 规格偏差、`Character.digit` 非 ASCII、`Arrays.copyOf` 组件类型、`Properties` defaults 等问题。
2. **FS-T1..T3**：#42 第二档真并行（进行中），并让 `availableProcessors` 返回真实核数。
3. **FS-E3 / FS-M7 / FS-M8**：让所有异常都能被 catch（checkcast / NPE / toString 路径）；统一判空。
4. **FS-P1..P3 / FS-C4**：系统属性全集、exit、getenv、ServiceLoader 静态服务表。
5. **FS-O1..O3**：文档与注释的真实性。
6. 其余按用例触达程度排序。
