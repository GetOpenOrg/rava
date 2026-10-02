# 真实 native 缺口补齐（2026-10-02，分支 native-gaps）

来源：`docs/reports/gap-scan-api-java.lang-java.util.md` 的 native-missing 35 项、text 等 14 包旧报告（以
`rava audit api` 重跑结果为准）、C6 预检 18 项。归类依据 `docs/reference/handwritten-boundary.md`：
① ACC_NATIVE；② 运行模型替换（运行期类定义点等）；③ VM 驱动行为的落地语义。

## 一、归类与处理

| 类 | 成员 | 类别 | 处理 |
|----|------|------|------|
| jdk/internal/misc/Unsafe | get/put{Boolean,Byte,Short,Char,Float,Double}Volatile（12） | ① | `unsafe__impl.rs`：存储单元与 plain 同一；内存序按 HotSpot `MemoryAccess::get_volatile/put_volatile`：读前 SeqCst 栅栏 + 读后 Acquire，写前 Release + 写后 SeqCst |
| jdk/internal/misc/Unsafe | get/put{Int,Long}Volatile（既有，原只认实例原子单元） | ① | 改走同一通路（覆盖静态字段 / 原生内存） |
| jdk/internal/misc/Unsafe | throwException | ① | 原样抛出（HotSpot `THROW_OOP`，不包装不重填栈）；null → NPE |
| java/lang/invoke/VarHandle | 签名多态入口（get/set/…，既有手写） | ② | 元素族由 3 个扩到 9 个（References/Booleans/Bytes/Shorts/Chars/Ints/Longs/Floats/Doubles），Field 家族按族落到 Unsafe 同族访问器（plain → get/putX，volatile/acquire/release/opaque → get/putXVolatile）；新增 `$FieldStatic*` 形态（基址取 flavor 的 `base` 字段）；数组族统一为存储写锁内读-改-写（`var_handle_ext.rs`） |
| java/lang/Module | defineModule0 / addReads0 / addExports0 / addExportsToAll0 / addExportsToAllUnnamed0 | ① | `module_impl.rs`：VM 模块表（HotSpot `Modules::define_module` / `add_reads_module` / `add_module_exports*` 的检查次序与异常） |
| java/lang/Class | getDeclaredClasses0 / getNestMembers0 / getClassAccessFlagsRaw0 / setSigners（+ getSigners） | ① | `class_impl.rs`；InnerClasses / NestMembers / 原始 access_flags 由生成器发射（`head.rs` 的 `#[nest_members]` / `#[class_access_flags]`），java_meta 造表 |
| java/lang/ClassLoader | defineClass1 / defineClass2 | ② | 原生二进制无运行期类定义（类全集编译期定死）。按 HotSpot `ClassFileParser` 检查次序给出不定义类即可判定的结果（截断 / 魔数 / 主版本 / preview / 常量池标签 / 多余字节，异常类型与消息同 JDK 21）；结构完整的类文件抛 `LinkageError` 说明不支持运行期定义——Java 可捕获，不 panic |
| java/lang/ClassLoader | retrieveDirectives | ① | 断言开关指令：本模型无 `-ea/-esa` 命令行，返回空指令集（与 JDK 未带开关时一致） |
| java/lang/SecurityManager | getClassContext | ③ | 见 §二 |
| java/lang/StackStreamFactory(+$AbstractStackWalker) | checkStackWalkModes / callStackWalk / fetchStackFrames | ③ | 见 §二 |
| java/lang/StackTraceElement | initStackTraceElement | ③ | 见 §二 |
| java/lang/invoke/MethodHandleNatives | expand | ① | 见 §二 |
| jdk/internal/loader/NativeLibraries | `<init>`（有字节码，非 native） | — | ng-07918da9 抽查中，TestClassNestNatives / TestDefineClassRejects 命中存根。`--why` 证实它在调用链上：自定义加载器的构造器 → VM 边界类 ClassLoader.<init>(Void,String,ClassLoader)@54 → NativeLibraries.newInstance → <init>。原因是 `jdk/` 前缀截断：newInstance 判为 handwritten:boundary，不再往下展开，生成器照字节码翻了 newInstance，<init> 却只留存根。处理：在 closure.toml `[release].classes` 放行本类（纯 Java），之后 --why 判为 bytecode，两例本机编译通过。不手写 |
| jdk/internal/access/JavaLangAccess | getEnumConstantsShared（类初始化事实） | ③ | ng-07918da9 中 TestStackWalkerFrames 报 ExceptionInInitializerError ← CCE「StackWalker$Option not an enum」。链路：StackWalker.<clinit> 的 `EnumSet.noneOf(Option.class)` → getUniverse → 手写 getEnumConstantsShared。手写先按名调初始化钩子，再读常量目录；但 Option 尚未初始化（ldc 不触发初始化），JDK 侧又没有登记它的钩子，所以宇宙为空。这与 Class natives、ACC_ENUM 标志都无关。JDK 中 Class.getEnumConstantsShared 反射调用 values()，会初始化目标类。处理：vm_intrinsics.toml 的 `[facts.class_init.initializers]` 登记本方法（形参 0），分析器据此输出 Option / Collector$Characteristics / StreamOpFlag$Type 三个初始化目标，生成器登记它们的钩子。本机编译通过。附带：`EnumSet.of(e)` 经 getDeclaringClass → getSuperclass 得到的 Class 是 open，class_init 因此记为 unknown，生成器退回到对全部带 <clinit> 的类登记钩子（只是登记，闭包不增大）。getSuperclass 返回值精度归 C1d |
| java/lang/ClassLoader 及其嵌套类 | <clinit>（vm_boundary 类的 <clinit> 改按类划分） | ③ 相关 | ng-f9ce2c33 中 ParallelLoaders.loaderTypes 为 null（构造器 isRegistered 上 NPE）；ng-5bcc93ae 中 TestDefineClassRejects 每条拒绝都报 NPE：ClassLoader.checkCerts 读静态 nocerts 为 null，putIfAbsent(null) 抛 NPE。共同根源：`[vm_boundary]` 按外层类覆盖嵌套类，facts.rs 的 boundary_carried 与 input/plan.rs 的 clinit_plan 把 vm_boundary 类的 `<clinit>` 一律当手写承载，不翻译。ClassLoader 的静态状态（nocerts、ParallelLoaders.loaderTypes、assertionLock 等）是纯 Java，不满足手写准入。处理（通用）：closure.toml `[vm_boundary]` 新增 `clinit_carried` 表，只列 <clinit> 确被 VM 模型替换的类（VirtualThread、Module、ModuleLayer、InetAddress、JceSecurity、InvokerBytecodeGenerator、FileSystems；Class、SecurityManager 暂列，待逐类核实后移出）；表外 vm_boundary 类（ClassLoader 及其嵌套类）的 <clinit> 按字节码翻译。撤销 ParallelLoaders 的 `[release]` 放行。ClassLoader.<clinit> 首句 native `registerNatives()` 补手写空实现（HotSpot 仅绑定 JNI 入口）。本机三例编译通过，ClassLoader 生成 __clinit（含 nocerts） |
| （分析器）手写体 `T::__class_init()` | 类初始化建模 | ③ 相关 | ng-462ab7b0 中 TestStackWalkerFrames 转为 panic：JavaLangInvokeAccess.newMemberName 无实现者。链路：StackFrameInfo.<init> → JLIA.newMemberName；JLIA 取自手写 SharedSecrets.getJavaLangInvokeAccess，它在槽位为空时调 `MethodHandleImpl::__class_init()`，由 MethodHandleImpl.<clinit> 登记 MethodHandleImpl$1。分析器不认手写体里的 `__class_init` 调用，MethodHandleImpl 只到 layout 级，<clinit> 不分析，MethodHandleImpl$1 不入闭包。处理（通用）：engine/hw.rs 的 apply_hw 把手写体路径调用 `T::__class_init()` 建模为 T 的类初始化（Rust 名常量 CLASS_INIT_RUST，不含 JDK 类名）。同一机制覆盖 getJavaIOAccess（Console）、getJavaIOFileDescriptorAccess（FileDescriptor）等同形态手写。本机编译通过 |
| （分析器）手写层按名写字段 | StackFrameInfo 填帧 | ③ 相关 | ng-5bcc93ae 中 TestStackWalkerFrames 在 main 抛 NPE。链路：StackFrameInfo.getClassName → declaringClass → MemberName.getDeclaringClass 返回 null → null.getName()。根源：callStackWalk 填帧经辅助函数 `_put_ref(&member, "clazz", …)` 按名写 MemberName.clazz / name / type / flags，字段名是辅助函数形参，分析器看不到写入，按值集（只有 null）把 getDeclaringClass 折叠成返回 null。处理（通用）：object.rs 新增 int 按名写形态 `__unsafe_int_set`；分析器 handwritten/syntax.rs 把首参为字符串字面量的按名协议调用（`__unsafe_ref_set/update`、`__unsafe_int_set` 为写，`__unsafe_ref_get`、`__unsafe_{int,long,bool}_cell` 为读）记为该名字段的读写；填帧改为在调用点以字面量写字段。重生成后 MemberName.getDeclaringClass 为 `Ok(this.__get_clazz())`，本机编译通过 |
| （用例）TestClassNestNatives | 断言映射段（FS-C2 已知失败，另挂任务） | — | ng-5bcc93ae 中 main 抛 NPE：用例对应用类加载器 `TestClassNestNatives.class.getClassLoader()` 设断言状态，本模型下该加载器为 null（Class.classLoader 不设置，属 FS-C2）。ng-893a4c86 复核：重生成的 main 在第 61 行由 `getClassLoader()` 取得 app，第 62 行 `desiredAssertionStatus` 已折叠为常量 false，第 63 行 `app.setPackageAssertionStatus` 是 app 的第一次解引用，NPE 即在此抛出——确认由应用类加载器为 null 引起。7f66279e 曾改用自定义加载器并改 expected，用户决定回退：用例与 expected 恢复原写法（assert(Shape) class on / assert(Inner) default on 为 true，JDK 21 实测）。本例登记为 **FS-C2 已知失败**，另挂任务，不阻塞本分支合入；本分支涉及的其余断言（nest / flags / signers / retrieveDirectives）不受影响。另：`desiredAssertionStatus` 在 vm_intrinsics 中为常量 false，FS-C2 落地后断言映射段还需该常量改为按 ClassLoader 映射求值 |
| （生成器）成员类修饰符 | Class.getModifiers 数据源 | — | ng-810dd426 中 TestClassNestNatives 的 `flags TestClassNestNatives$Hidden` 少 PRIVATE。根源：emit head.rs 把类文件顶层 access 与 InnerClasses 自引用条目按位或，且类修饰符串不含 private / protected。HotSpot `compute_modifier_flags` 对成员类取 inner_class_access_flags 取代顶层 access。处理：有自引用条目时取其 flags；类修饰符串补 private / protected；java_meta 修饰符表在成员类为 private / protected 时不再采信顶层 access 的 public 位（protected 成员类顶层记为 public）。重生成后表项 Hidden=0x060a，其余成员类与 JDK 21 期望一致，本机编译通过 |
| sun/nio/fs/UnixNativeDispatcher | mkdir0 / symlink0 / readlink0 / realpath0 / fstat0 / chmod0 / fchmod0 / chown0 / lchown0 / fchown0 / utimes0 / futimes0 / lutimes0 / futimens0 / statvfs0 / getpwuid | ① | TestModuleLayerDefine 预检报出（`ModuleFinder.of()` → ModulePath.scan → readModule → Files.copy 等，静态在调用链上）。`unix_native_dispatcher_ext.rs`，对标 JDK 21 `UnixNativeDispatcher.c`：路径为 NativeBuffer 地址，RESTARTABLE 重试，失败按 errno 抛 UnixException；readlink 缓冲 PATH_MAX+1、恰好填满截末字节；utimes / futimes / lutimes 微秒、futimens 纳秒（向零截断）；statvfs0 在 macOS 走 statfs（块大小取 f_bsize），其余走 statvfs（f_frsize）；fstat0 借用 fd 不关闭；getpwuid（TestUnixFileNatives 的 getOwner 新入链）走 getpwuid_r，缓冲取 _SC_GETPW_R_SIZE_MAX（缺省 1024），未找到 / 空名抛 ENOENT |
| sun/nio/fs/UnixFileSystem | bufferedCopy0 | ① | `unix_file_system_impl.rs`：直接缓冲循环 read / write（EINTR 重试、短写续写），每批读后查取消字（非 0 抛 ECANCELED），EOF 返回 |
| sun/nio/fs/BsdNativeDispatcher（macOS） | initIDs / setattrlist0 / fsetattrlist0 / getmntonname0 | ① | `bsd_native_dispatcher_impl.rs`：getmntonname0（FileStore 构造入链）取 statfs 的 f_mntonname；commonattr 按 CRTIME / MODTIME / ACCTIME 位序装入纳秒 timespec；initIDs 只缓存挂载表字段 ID，本模型按名访问字段，空实现。Linux 主机的对应类（LinuxNativeDispatcher / LinuxFileSystem.directCopy0 等）是否入链须看服务器预检，本机（macOS JDK）无法生成 |
| （生成器）循环提升的同变量多处首存 | 局部变量声明类型 | — | ng-893a4c86 中 TestModuleLayerDefine 编译报 E0599（Object 上无 add / contains），集成分支 ad9e938d 同样复现。位置：ModuleInfo.readModuleAttribute 的 requires 循环，`Set mods` 在 if 两臂分别首存（Set.of() / new HashSet），循环内 break 后的抛出路径在循环后读取 mods。根源：method/vars/loop_hoist 只把触发提升的第一个 let 降级为赋值，else 臂的同变量 let 留在原处；随后的 if 提升又为它另立声明，Set 与 HashSet 的公共类祖先求不到（接口），回退根类 Object，于是循环头出现 `mods: Set<Object>` 与 `mods: Object` 两个同名声明，后者遮蔽前者。修复：循环提升时把循环体内同一 JVM 变量（LVT 身份 same_jvm_var 不判为另一变量）的全部 let 一并降级；各存入不能对齐到首存类型时按公共祖先汇合并上转（同 if 提升的 widen_into_merged）。新用例 TestLoopJoinedLocal 覆盖接口工厂 + 具体类两臂（汇合为 Set）与两个具体类两臂（ArrayList / LinkedList，汇合为 AbstractList），重生成后两者均正确 |
| （用例）TestModuleLayerDefine | boot layer 模型（已知失败，非 FS-C2） | ③ | ng-6fbcba81 / 6a564816 / 82ba99ba 三轮运行期均为 main 直接抛 NPE（无任何输出，第 67 行前）。按代码定位：第 58 行 `ModuleLayer.boot().configuration()`——`boot()` 是 VM 边界手写（module_layer_impl.rs），返回 `_init_not_null()` 的空层对象，字段 cf / parents / nameToModule 全未设置；`configuration()` 按字节码翻译（`getfield cf; areturn`）得 null，随后 `.resolve(...)` 的空接收者检查（宏 null_receiver_check）抛 NPE。`ClassLoader.getSystemClassLoader()` 不是来源：手写返回已 `_init_not_null` 的 app 加载器（父为 platform），非 null。终态：boot layer 是 VM 引导期（initPhase2）由 ModuleBootstrap.boot 建立并写入 System.bootLayer 的 Java 对象，`boot()` 应按字节码读取该层，其 Configuration 含系统模块（至少 java.base——ModuleDescriptor.Builder.build 会自动加 `requires mandated java.base`，用空配置作父仍会解析失败）。这依赖系统模块描述符（SystemModules / ModuleFinder.ofSystem）的数据承载与引导序列，属 FS-H12（ModuleLayer 整类手写收窄）与引导初始化范围，另立任务；不在 boot() 上补空 Configuration 近似 |
| java/lang/SecurityManager | getClassContext（抽象 / 接口派发帧） | ③ | ng-893a4c86 中 TestSecurityManagerContext 中间多出 `java.lang.Runnable` 帧。根源：vm_stack 按 Rust 符号解析帧，接口 / 抽象方法的派发包装 `X::m` 本身被记为一帧。修复：只有实际执行方法体的符号成帧——abstract 方法不成帧；接口上被派发的实例方法只有 `__default_<m>`（default 方法体）成帧；`java_method_of` 同时剥 `__impl_` 与 `__default_` 前缀。新用例 TestClassContextDispatch 覆盖接口实现、default、覆盖 default、抽象类、模板方法、子接口、lambda 七种形态。ng-6fbcba81 抽查（us1）又暴露两处帧归属错误，按 HotSpot「帧属方法体的声明类（method holder）」统一修正：① 生成器把接口 default 体注入未覆盖的实现类（interface_default_inheritance），java_meta 视之为实现类声明，帧记成 UsesDefault——注入方法改带 `declared_by = "<接口>"` 属性，MethodMeta 增 `declared_by`，帧与行表（line_tables 读同一属性）归声明接口，getDeclaredMethods 不再计入（JDK 为 0）；② 子类覆盖体实际在 `Derived__inner`（vtable impl）与 `Derived__run_base`（base 自由函数）符号上执行，parse_symbol 不识别 `__` 后缀而丢帧——`X__inner` 归 X、`X__m_base` 归 X.m；③ 类上虚方法（`virtual_in` 槽位，MethodMeta 增 `dispatched`）的公开 `X::m` 只是派发入口，上一帧是另一类同签名方法时不成帧，原残留缺口 regress2-b（非 abstract 被覆盖方法多一帧 Base）一并消除。用例补 super 调用、继承未覆盖、经类调用 default、getDeclaredMethods、default 内 Throwable 栈顶、非 abstract 覆盖、委托七种形态。ng-6a564816 复抽仍差「继承未覆盖」一行：用户超类的虚方法在子类未覆盖时，super_inherit 的用户祖先段把祖先方法体按子类重发射（子类 vtable impl 需要真实函数体），同样被当成子类声明——与接口 default 注入同一机制，`declared_by` 推广为「复制进本类的方法体的声明类型」，用户祖先段同样标注；行表源文件改按声明类型自己的 `source` 校正（祖先可在另一源文件）。用例补继承方法内 Throwable 栈顶（类.方法 文件）与子类 getDeclaredMethods（JDK 为 0） |
| jdk/internal/misc/CarrierThreadLocal、TerminatingThreadLocal | `<init>`（闭包放行） | — | ng-6fbcba81 中 TestUnixFileNatives（ubuntu）运行期命中 `stub: jdk/internal/misc/CarrierThreadLocal.<init>:()V`。路径：UnixPath.toRealPath → UnixNativeDispatcher.realpath → copyToNativeBuffer → NativeBuffers.<clinit> → `new NativeBuffers$1`（TerminatingThreadLocal 匿名子类）→ TerminatingThreadLocal.<init> → CarrierThreadLocal.<init>。两类在 `jdk/` 边界前缀下停止展开，`<init>` 不分析即存根；与平台无关（macOS 同样入链），TestFilesApi 不经 toRealPath / NativeBuffers 故未暴露。两类均为纯 Java、无 native，按字节码优先原则加入 closure.toml `[release] classes`。ng-6a564816 复抽（kr1）下一站命中 `stub: jdk/internal/access/JavaLangAccess.getCarrierThreadLocal`：CarrierThreadLocal.get 经 SharedSecrets.getJavaLangAccess()，本分支该入口仍返回过渡手写 SystemJavaLangAccess（无此方法，接口派发落存根）。终态是 JavaLangAccess 实现取自 System$2 字节码（getCarrierThreadLocal → Thread 的 carrier 线程本地表），这要求删除 SharedSecrets / SystemJavaLangAccess 过渡手写并由 System.setJavaLangAccess 引导初始化——正是 C1d-a 1e623cec 的内容；本分支不重复实现（会与之冲突），也不在 SystemJavaLangAccess 上补手写近似。TestUnixFileNatives 记为待 C1d-a 合入的已知失败 |
| com/sun/media/sound/DirectAudioDeviceProvider、PortMixerProvider | nGetNumDevices / nNew*Info | — | 不应在调用链上：移交 C1d（边见 §三） |
| java/lang/invoke/MethodHandleNatives | getMemberVMInfo / getNamedCon | — | 只在 `assert` 体内调用，断言恒关后不可达；不手写（见 §三） |
| java/lang/StackStreamFactory$AbstractStackWalker | setContinuation | — | 只在续体非 null 时调用，本模型续体恒 null；不手写（见 §三） |

## 二、栈遍历组（类 ③）

帧源 `runtime/java_runtime/src/vm_stack.rs`：真实 Rust 栈 → Java 帧（符号 → 声明类 → java_meta 方法表对位，
同一 Java 方法的派发包装 / `__impl_` 体归并为一帧，闭包帧不成帧）。Reflection.getCallerClass 的帧解析
同迁于此。

- **SecurityManager.getClassContext**（`security_manager_impl.rs`）＝ `JVM_GetClassContext`：最上方的 Java 帧须是
  getClassContext 本身，否则抛 InternalError（消息同 HotSpot）；然后逐帧取持有类，排除 native 帧和安全栈遍历
  要忽略的帧，即 `Method::is_ignored_by_security_stack_walk`：Method.invoke、MethodAccessorImpl 的子类、
  `@LambdaForm$Compiled`。
- **AbstractStackWalker.callStackWalk / fetchStackFrames**（`stack_stream_factory_abstract_stack_walker_impl.rs`），
  对应 `StackWalk::walk` / `fetchNextBatch` / `fill_in_frames`：
  - 锚点：线程本地的锚点表，在 callStackWalk 帧处取快照；doStackWalk 返回（含异常）后锚点失效。失效锚点再取帧时抛
    InternalError "doStackWalk: corrupted buffers on stack"。
  - 起始跳帧：先跳过 StackWalker 实现帧（持有类为 StackWalker、AbstractStackWalker 或其直接子类），再跳 skipframes 帧。
  - 隐藏帧：mode 不含 SHOW_HIDDEN_FRAMES，或为 GET_CALLER_CLASS 模式时，跳过 `@Hidden` 方法。
  - 调用者敏感检查：GET_CALLER_CLASS 模式下，若批首帧是 `@CallerSensitive` 方法，抛 UOE（消息同 HotSpot，含
    `Method::external_name`）。
  - 缓冲区与解码检查：缓冲不足抛 IAE "not enough space in buffers"；首批解码不到帧抛 InternalError
    "stack walk: decode failed"；续批同样情况抛 "doStackWalk: decode frames failed"。
  - StackFrameInfo：经按名协议写 memberName 的 clazz / name / type（描述符串）/ flags。flags 按
    `init_method_MemberName` 计算：修饰位、类别位、ref kind、CALLER_SENSITIVE。原生帧没有字节码下标，bci 记 0。
  - FILL_LIVE_STACK_FRAMES（内部 API LiveStackFrame 的局部变量 / 操作数快照）依赖解释器帧布局，抛 UOE。
- **StackStreamFactory.checkStackWalkModes**：模式位编码与数据面一致，恒返回 true。
- **StackTraceElement.initStackTraceElement**（`stack_trace_element_impl.rs`）＝ `fill_in`：
  - 由 memberName 填 declaringClassObject / declaringClass / methodName。
  - fileName 取声明类的 SourceFile。为此新增 java_meta 表 `CLASS_SOURCE_FILE`，由 head.rs 已发射的 `#[source]` 造表。
  - lineNumber：native 方法记 -2，其余记 -1（无 bci，无法查行号表）。
  - 形参以 `Into<Object>` 接收，不以类型名引用 StackFrameInfo。否则 getCallerClass 路径会被拉入生成范围。
- **生成器修正（继承的手写非槽位方法）**：
  - 问题：手写 native 等非槽位方法在声明类中只以 `// [meta]` 注释行存在，原先不进 `ClassEmission.methods`。子类接收者调用继承来的这类方法（如 `Probe extends SecurityManager` 调用 `getClassContext()`）时，生成器找不到声明者，因此不发继承成员，编译报 E0599。
  - 修正：`class_writer::methods` 的 Handwritten 判定，在发 `[meta]` 注释段时同时给出声明记录，并打上 `meta` 标记（`EmittedMethod.meta`）。继承成员解析时用 `find_declared` 把它当作声明者，按宏的「非虚继承：上转后直调」路径发射。
  - 不受影响的部分：槽位、桥接、接口实现的定位（`find` / `slotted`）仍然排除 `[meta]` 声明。
  - 连带修正：这次修正暴露出手写 `ClassLoader.getResources` / `getSystemResources` 的返回类型（Object）与声明（`Enumeration<Object>`）不一致，已对齐。
- **MethodHandleNatives.expand**（`method_handle_natives_impl.rs`）＝ `expand_MemberName(suppress=0)`：
  - mname 为 null 时抛 InternalError "mname is null"。
  - clazz / name / type 三项齐全时直接返回。
  - 方法 / 构造器：本模型没有 vmtarget，抛 IAE "nothing to expand"。
  - 字段：clazz 为 null 时抛 IAE "nothing to expand (as field)"；有名字时按字段表补 type；其余情况抛 IAE
    "unrecognized MemberName format"。

## 三、移交 C1d

- `java/lang/invoke/MethodHandle.linkToNative`（不在 35 项清单内，TestSecurityManagerContext 预检报出）：
  - `rava closure --why` 实测的入链路径：
    ```
    main@68 → [dispatch] Method.invoke@90 → [invoke] DirectMethodHandleAccessor$NativeAccessor.invoke@91
      → [signature] NativeAccessor.methodAccessorInvoker:()Ljava/lang/invoke/MethodHandle;
      → [reflect] 类 java/lang/invoke/MethodHandle → linkToNative
    ```
  - 结论：MethodHandle 只是作为返回类型出现在方法签名里，反射模型却因此把该类的全部成员都当作反射可达。用户的反射目标（getMethod 的 Class 实参）只有 TestSecurityManagerContext 本类，没有任何调用经 FFM 下调桩到达 linkToNative。
  - 处理：属闭包过近似（反射成员集），**移交 C1d**，不手写。
  - 备注：即使将来有真实调用，linkToNative 的目标也是运行期生成的下调桩（`NativeEntryPoint.makeDowncallStub`），归第②类。
- `com/sun/media/sound/DirectAudioDeviceProvider`、`PortMixerProvider` 的 nGetNumDevices / nNew*Info：
  - 来由（推断，`rava closure --why` 证实前不作定论）：audit api 的入口是包内全部 public 方法。
    `ServiceLoader.<init>` 的服务 Class 形参值集不精确（[services.lookups]）。因此模块服务目录中
    java.desktop 的 `javax.sound.sampled.spi.MixerProvider` provider 全部入选；provider 的无参构造器调用
    nGetNumDevices 等 native。
  - 处理：这是闭包精度问题，不在调用链上，不手写，**已移交 C1d-b**。

- 14 包旧报告（2026-09-29）里另外 3 项不手写：
  - `MethodHandleNatives.getNamedCon:(I[Ljava/lang/Object;)I`：唯一调用者是 `verifyConstants`，只在
    `MethodHandleNatives.<clinit>` 的 `assert(verifyConstants())` 里调用。
  - `MethodHandleNatives.getMemberVMInfo:(Ljava/lang/invoke/MemberName;)Ljava/lang/Object;`：唯一调用者是
    `MemberName.vminfoIsConsistent`，只在 `MemberName$Factory.resolve` 的 `assert(m.vminfoIsConsistent())` 里调用。
    HotSpot 的 product 构建中，getNamedCon 恒返回 0（`#ifndef PRODUCT`）。两者都是调试设施。
  - `AbstractStackWalker.setContinuation:(J[Ljava/lang/Object;Ljdk/internal/vm/Continuation;)V`：只在
    `getNextBatch` 的 `hasMoreContinuations()` 为真时调用，前提是 `continuation != null`。非 null 续体只来自
    `Continuation.stackWalker`（`JLA.newStackWalkerInstance(.., innermost())`）。本模型把虚拟线程映射为 OS 线程，
    续体从不挂载（`VirtualThread` 是 VM 边界类），所以 `continuation` 恒为 null。
  - 实测：`vm_intrinsics.toml` 把 `Class.desiredAssertionStatus(0)` 定为常量 false（41dc1d6d 起），因此
    `$assertionsDisabled` 折叠为真，assert 体整片不可达。当前分析器下：
    - TestMethodHandleCombinators：`--why verifyConstants`、`--why vminfoIsConsistent` 都报「不在闭包内」；
    - TestStackWalkerFrames：`--why` 三项都报「不在闭包内」。
  - 定稿（单例 `--stop-after closure --closure-json` 核对 closure.json 的 methods）：TestMethodHandleCombinators
    与 TestStackWalkerFrames 中 getNamedCon、verifyConstants、getMemberVMInfo、vminfoIsConsistent、setContinuation
    均不在闭包内（swf 中 hasMoreContinuations 在闭包内，setContinuation 不在）。三项不在调用链上，不手写。
    旧报告出自折叠落地前的旧分析器；若全包 audit 恢复后仍出现，归闭包精度问题移交 C1d。

- TestReflectProbe（ng-893a4c86，`Unsafe 静态引用读：java/lang/Integer.MAX_VALUE 无字段闭包`）：
  本分支与集成分支 ad9e938d 分别只生成并比对：两边 `java/lang/integer.rs` 的 `__reflect_field` 都只有
  serialVersionUID 一臂，closure.json 的 reflect.members 两边都没有 Integer.MAX_VALUE。不是本分支引入，
  属反射字段闭包缺口，**移交 C1d-b**。

## 四、行数说明

`unsafe__impl.rs`、`class_impl.rs` 超过 600 行：共置机制按「类 → `<stem>_impl.rs` / `<stem>_ext.rs`」
两个伴生文件挂载，VarHandle 已用 `_ext` 拆出元素族与数组族；Unsafe / Class 的拆分需要生成器支持多于两个
伴生文件，另立任务。

## 五、e2e 用例清单（expected 均为 JDK 21 实测）

| 用例 | 覆盖的手写方法 |
|------|----------------|
| tests/e2e/48_refs/TestVolatilePrimitiveAccess.java | Unsafe get/put{Boolean,Byte,Short,Char,Float,Double}Volatile、get/put{Int,Long}Volatile 静态臂；VarHandle 九元素族的实例 / 静态 / 数组形态 |
| tests/e2e/62_reflection/TestModuleLayerDefine.java | Module.defineModule0 / addReads0 / addExports0（经 ModuleLayer.Controller） |
| tests/e2e/62_reflection/TestClassNestNatives.java | Class.getDeclaredClasses0 / getNestMembers0 / getClassAccessFlagsRaw0 / setSigners / getSigners；ClassLoader.retrieveDirectives |
| tests/e2e/62_reflection/TestDefineClassRejects.java | ClassLoader.defineClass1 / defineClass2；ClassLoader.registerNatives（<clinit>） |
| tests/e2e/62_reflection/TestStackWalkerFrames.java | StackStreamFactory.checkStackWalkModes / callStackWalk / fetchStackFrames；StackTraceElement.initStackTraceElement；MethodHandleNatives.expand |
| tests/e2e/62_reflection/TestSecurityManagerContext.java | SecurityManager.getClassContext |
| tests/e2e/62_reflection/TestClassContextDispatch.java | SecurityManager.getClassContext 经接口 / default / 抽象 / 模板 / super 调用 / 继承未覆盖 / 非 abstract 覆盖 / 委托 / 子接口 / lambda 派发时的帧；注入 default 不计入实现类 getDeclaredMethods；default 体内 Throwable 栈顶类 |
| tests/e2e/53_io_api/TestUnixFileNatives.java | UnixNativeDispatcher 属性 / 目录 / 链接族 native、UnixFileSystem.bufferedCopy0、（macOS）BsdNativeDispatcher.setattrlist0 / fsetattrlist0 / getmntonname0、getpwuid（属主名 = user.name）；含已存在 / 父目录缺失 / 非链接 / 路径缺失 / 悬空链接等 errno 边界 |
| tests/e2e/01_basics/TestLoopJoinedLocal.java | （生成器）循环内 if 两臂首存的局部在循环后读取：接口工厂 + 具体类、两个具体类 |
