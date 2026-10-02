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
| com/sun/media/sound/DirectAudioDeviceProvider、PortMixerProvider | nGetNumDevices / nNew*Info | — | 不应在调用链上：移交 C1d（边见 §三） |

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
  - 处理：这是闭包精度问题，不在调用链上，不手写，**移交 C1d**。
  - 证实方法：在服务器对 audit 入口跑 `--why com/sun/media/sound/DirectAudioDeviceProvider`。

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
| tests/e2e/62_reflection/TestDefineClassRejects.java | ClassLoader.defineClass1 / defineClass2 |
| tests/e2e/62_reflection/TestStackWalkerFrames.java | StackStreamFactory.checkStackWalkModes / callStackWalk / fetchStackFrames；StackTraceElement.initStackTraceElement；MethodHandleNatives.expand |
| tests/e2e/62_reflection/TestSecurityManagerContext.java | SecurityManager.getClassContext |
