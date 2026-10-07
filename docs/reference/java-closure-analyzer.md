> **参考资料（非计划）**：通用的 Java 依赖闭包分析知识整理，2026-09-30 收入。rava 的执行依据是
> `docs/plans/2026-09-29-rust-closure-analyzer.md` 与 `docs/plans/2026-09-29-boundary-narrowing.md`，本文与之冲突时以计划为准。
>
> 与 rava 的对照：
>
> | 本文章节 | rava 现状 |
> |---|---|
> | 6.1 / 14.3 类型分层 | 已实现：`Level::Type / Init / Alloc / Code` |
> | 6.4 双向补边 | 已实现：差分传播 + 站点重跑 |
> | 7.1 `jsr` / `ret` | 已实现：进入保守模式（`absint.rs`），不静默跳过 |
> | 10.4 值来源标签、10.5 选择性上下文敏感 | 已实现：C1c 第 2 步（容器对象敏感） |
> | 5.3 VM 填充字段、5.4 上调 | 由 `runtime/java_runtime/{closure,seeds,vm_intrinsics}.toml` 清单承载 |
> | 第 9 节 JDK 运行时代码生成 | lambda / indy / MethodHandle 由 rava 自有运行模型替换（VM 契约第 3 类） |
> | 15.3 按层级对照、排除清单 | 已实现：translate 域动态漏覆盖 = 0 口径 |
> | 16.3 确定性 | 已实现：双种子检查 |
> | 17.4 双算过渡 | 对应 C3 消费切换 + `scripts/compare_trees.sh` |
> | 14.4 每站点目标数直方图 / top-K 汇点 | 待补，C1d 放行实测时用于定位精度退化 |
> | 18.4 读取 GraalVM reachability-metadata | 远期，lib pilot 接入反射重的库时作为 `seeds.toml` 补种来源 |
> | 16.7 超时降级、上下文数上限回退 | **不采纳**：按时间降级破坏确定性；主动回退精度违背终态原则，性能靠算法解决 |
>
> 本文未覆盖、rava 特有的内容：手写层的 VM 契约清单与按方法划分、逃逸模型（open 值只代表已逃逸对象）、
> 乐观常量阶段与失效重算、边界收窄（C1d）。

# Java 依赖闭包分析器：完整分析过程与实现步骤

> 本文讨论的"闭包分析"指**依赖闭包（可达性闭包）**：给定一组根，求出它们传递依赖的全部类、方法、字段（以及资源、native 符号）的最小可靠集合。ProGuard/R8 的 shrink、GraalVM native-image 的 points-to 分析、jlink 的模块裁剪做的都是这件事。
>
> 文末附录 C 简述另一种含义——lambda / 匿名类的**捕获变量分析**。

---

## 目录

- [阶段 0：规格定义](#阶段-0规格定义)
- [阶段 1：理论基础](#阶段-1理论基础)
- [阶段 2：输入收集与类路径建模](#阶段-2输入收集与类路径建模)
- [阶段 3：解析与符号表](#阶段-3解析与符号表)
- [阶段 4：类层次与 JVMS 解析 / 选择 / 初始化语义](#阶段-4类层次与-jvms-解析--选择--初始化语义)
- [阶段 5：根集合](#阶段-5根集合)
- [阶段 6：不动点求解](#阶段-6不动点求解)
- [阶段 7：逐指令与逐属性规则](#阶段-7逐指令与逐属性规则)
- [阶段 8：语言特性与动态特性建模](#阶段-8语言特性与动态特性建模)
- [阶段 9：JDK 自身的运行时代码生成](#阶段-9jdk-自身的运行时代码生成)
- [阶段 10：精度提升](#阶段-10精度提升)
- [阶段 11：开放世界建模](#阶段-11开放世界建模)
- [阶段 12：结构一致性补全与桩化](#阶段-12结构一致性补全与桩化)
- [阶段 13：配置规则语言](#阶段-13配置规则语言)
- [阶段 14：输出与可解释性](#阶段-14输出与可解释性)
- [阶段 15：验证](#阶段-15验证)
- [阶段 16：工程化](#阶段-16工程化)
- [阶段 17：迭代工作流与已知局限](#阶段-17迭代工作流与已知局限)
- [阶段 18：工业级要求](#阶段-18工业级要求)
- [附录 A：完整性核对清单](#附录-a完整性核对清单)
- [附录 B：参考实现与文献](#附录-b参考实现与文献)
- [附录 C：另一种"闭包分析"——捕获变量分析](#附录-c另一种闭包分析捕获变量分析)

---

## 阶段 0：规格定义

以下决策决定后面每一步怎么做，必须先明确并写进设计文档。

| 决策项 | 选项 | 影响 |
|---|---|---|
| 分析粒度 | 类级 / 成员级（方法、字段） | 成员级裁剪效果好得多，复杂度也更高 |
| 世界假设 | 封闭世界（应用） / 开放世界（库） | 开放世界需要建模外部调用者与外部子类（阶段 11） |
| 可靠性等级 | sound / soundy（显式声明不支持的特性） / best-effort | 决定反射、native、动态类生成的处理方式 |
| 精度等级 | CHA / RTA / XTA / VTA / 指针分析 | 决定闭包大小与分析代价 |
| 目标平台 | 目标 JDK 版本、classfile 版本范围、目标 VM | 决定需要支持的指令、属性、上调清单 |
| 输出消费者 | 重写 classfile 在真实 JVM 运行 / 喂给代码生成器或转译器 / 仅做报告 | 决定是否需要桩化、结构补全、校验器兼容、JLI 运行时策略（阶段 9） |
| invokedynamic 策略 | 保留 `java.lang.invoke` 运行时 / 分析前静态 desugar | 直接影响闭包规模与阶段 9 的工作量 |
| 未知处理策略 | 缺失类、无法推断的反射、未知引导方法、**动态类定义**（`defineClass` / `defineHiddenClass`，代码本身是运行时数据） | 报错 / 保守保留 / 按配置处理（动态类定义可以"额外字节码输入"的方式交给配置） |
| 资源处理 | 是否同时计算资源闭包 | 见阶段 8.7 |
| 性能 / 规模预算 | 图规模（类、方法、调用点数）、目标耗时、内存上限 | 决定精度上限与阶段 16 的溢出策略 |
| 分层结果 → 下游形态契约 | 每个层级（Referenced / Initialized / Instantiated / Dispatch-relevant）在下游各生成什么 | 下游若对 Referenced 级类型仍全量展开，闭包再准也无收益（阶段 14.3） |
| 入口组织 | 单入口 / 多入口（多测试、多产品） | 决定 per-entry 闭包与共享底座的分解与缓存（阶段 17.3） |

选择**成员级粒度**的同时，必须一并设计**结构反射**（`getDeclaredMethods` 等成员枚举 API）的处理策略（阶段 8.3.2），否则成员级裁剪在这些类上不可靠。

---

## 阶段 1：理论基础

### 1.1 形式化

分析状态是若干有限集合的乘积格（幂集格）：

| 集合 | 含义 |
|---|---|
| R | 可达方法 |
| I | 已实例化类型 |
| C | 已初始化类型 |
| T | 被引用类型 |
| F_r / F_w | 被读 / 被写字段 |
| V | 已登记的虚调用点 |
| N | 可达 native 方法 |
| Res | 可达资源 |

- 每条推导规则是单调函数，目标是**最小不动点**。集合有限且只增不减，迭代必然终止（Kleene / Tarski 不动点定理）。
- 规则可等价写成集合约束或 **Datalog**。RTA 的核心两条：

```prolog
Reachable(tgt) :- Reachable(m), VirtCall(m, T, sig),
                  Instantiated(C), SubtypeOf(C, T), Select(C, sig, tgt).
Instantiated(C) :- Reachable(m), New(m, C).
```

### 1.2 调用图算法谱系（由粗到细）

| 算法 | 虚调用 `x.m()` 的目标 | 精度 | 代价 |
|---|---|---|---|
| CHA（类层次分析） | 声明类型的所有子类型中的 `m` | 粗 | 极低 |
| RTA（快速类型分析） | CHA 结果 ∩ 全局已实例化类型 | 中 | 低 |
| XTA / VTA | 按方法、字段或变量细分的类型集合 | 较细 | 中 |
| 指针分析（Andersen、Steensgaard、k-CFA、k-obj、类型敏感） | `x` 可能指向的对象的类型 | 细 | 高 |

对 JDK 规模（java.base 约 7,500 类）RTA 是性价比最高的起点，秒级完成；闭包过大再升级到 XTA。

> "秒级"仅对类级 / RTA 成立。成员级闭包一旦引入上下文敏感（阶段 10.5），在数千方法的图上会到百秒量级甚至超时，需要阶段 16.7 的溢出策略。

### 1.3 可靠性理论

- 静态闭包必须是真实运行时依赖的**上近似**：漏一个可达成员就是编译失败或运行时缺失，宁多勿少。
- 反射、native、动态类生成会打破静态可知性。实践中采用"soundy"做法：**显式列出不支持的特性**，由配置补足，并用动态预言机（阶段 15）兜底。

---

## 阶段 2：输入收集与类路径建模

1. **来源**：classpath 目录、JAR、jmod、`lib/modules`（jimage）。
2. **多版本 JAR**：`META-INF/versions/N/` 按目标版本覆盖基础版本。
3. **模块系统**：
   - 解析 `module-info`：`requires`（含 `transitive`）、`exports`（含限定导出 `to`）、`opens`、`uses`、`provides`。
   - 处理包归属和 split package。
   - 限定导出与 `opens` 影响开放世界的根（阶段 11）。
4. **类名冲突与类加载器**：多来源同名类按加载顺序决定遮蔽。分析器通常假设单一命名空间；若目标环境存在多个类加载器（同名不同类），需要建模为 `(loader, name)` 二元组或明确声明不支持。
5. **缺失类（phantom class）**：被引用但找不到定义的类，记录下来，按阶段 0 的策略处理（报错 / 保守保留 / 视为外部类型）。
6. **资源文件**：`META-INF/services/*`、`*.properties`、`ResourceBundle` 文件。它们含有类名字符串，是反射依赖的来源，也是资源闭包的输入。

---

## 阶段 3：解析与符号表

1. 解析 classfile 全部结构：常量池、字段、方法、`Code`、异常表。
2. **类级属性**（全部都要读）：

   | 属性 | 用途 |
   |---|---|
   | `Signature` | 泛型类型引用 |
   | `InnerClasses`、`EnclosingMethod` | 嵌套关系；`Class.getSimpleName` / `getDeclaredClasses` 依赖它 |
   | `NestHost` / `NestMembers` | nestmate 私有访问 |
   | `PermittedSubclasses` | sealed 层级，可用于收窄分派 |
   | `Record` | record 分量，反射与序列化用 |
   | `BootstrapMethods` | indy / condy 引导方法 |
   | `RuntimeVisible*Annotations`、`RuntimeInvisible*Annotations`、`*TypeAnnotations` | 注解类型引用 |
   | `Module*` | 模块信息 |
   | `SourceFile` | 编译单元归属；输出按编译单元组织时是闭包规则的输入（阶段 12.11） |
   | `SourceDebugExtension` | 仅调试 |
3. **方法级属性**：`Exceptions`、`Signature`、`MethodParameters`、参数注解、`AnnotationDefault`（注解接口方法的默认值，引用枚举 / 类字面量 / 嵌套注解）、`StackMapTable`、`LocalVariableTable`、`LocalVariableTypeTable`。访问标志：`ACC_BRIDGE`、`ACC_SYNTHETIC`、`ACC_VARARGS`、`ACC_NATIVE`、`ACC_ABSTRACT`、`ACC_SYNCHRONIZED`、`ACC_STATIC`。
4. **字段级属性**：`ConstantValue`——编译期常量会被 javac 内联，读取它不触发类初始化。
5. **工程约定**：
   - 所有类名、方法签名、字段签名驻留为整数 ID。
   - **方法体惰性解析**：只有方法变为可达时才解码字节码。

---

## 阶段 4：类层次与 JVMS 解析 / 选择 / 初始化语义

### 4.1 层次结构

- 超类链、超接口、**反向子类型表**。
- 每个类型的全部超类型集合预计算为位集。
- **数组类型**：超类型为 `Object`、`Cloneable`、`Serializable`；数组间的协变子类型关系；数组的成员是 `Object` 的成员（`arr.clone()`、`arr.hashCode()`）。

### 4.2 符号解析（resolution，JVMS §5.4.3）

- 字段解析（§5.4.3.2）：先本类，再超接口，最后超类。
- 方法解析（§5.4.3.3）：本类 → 超类链 → 超接口中的"最具体者"。
- **接口方法解析（§5.4.3.4）**：若接口方法引用命名了 `Object` 的 public 非静态方法（如 `toString`、`hashCode`），解析到 `Object` 的方法——这一条常被漏掉。
- 解析失败对应的链接错误：`NoSuchMethodError`、`NoSuchFieldError`、`IllegalAccessError`、`IncompatibleClassChangeError`、`AbstractMethodError`。

### 4.3 方法选择（selection，JVMS §5.4.6）

- 从运行时接收者类开始沿超类链找覆盖方法；找不到时在超接口中找**最具体的非抽象默认方法**。
- 存在多个最具体默认方法时是歧义，运行时抛 `IncompatibleClassChangeError`。
- 覆盖关系需满足访问性约束（§5.4.5）：`private` 和跨包的包私有方法不参与覆盖。
- 所有解析和选择结果都要缓存（memoize）。

### 4.4 `invokespecial` 的三种语义

1. 构造器调用 `<init>`。
2. 私有方法调用。
3. `super.m()`：受 `ACC_SUPER` 影响，从当前类的直接超类开始查找；接口中的 `X.super.m()` 要求 `X` 是直接超接口。

### 4.5 nestmate 私有调用（Java 11+）

私有方法可通过 `invokevirtual` / `invokeinterface` 调用，按直接调用处理，不登记为虚调用。

### 4.6 签名多态方法

`MethodHandle.invoke` / `invokeExact` / `invokeBasic` / `linkTo*`、`VarHandle` 全部访问模式方法。调用点描述符与声明不一致，必须特判，不能按描述符匹配。

### 4.7 桥方法与协变返回

桥方法委托到真实实现，两者一并保留。

### 4.8 类初始化触发规则（JVMS §5.5）

| 触发源 | 说明 |
|---|---|
| `new` | 实例化前初始化 |
| `getstatic` / `putstatic` / `invokestatic` | 声明该成员的类；**编译期常量字段除外**（已内联） |
| 子类初始化 | 触发超类初始化；**超接口仅当声明了非抽象非静态方法（默认方法）时才初始化** |
| 接口初始化 | **不**传递到超接口 |
| 反射 | `Class.forName(name)`（单参数默认初始化）、`Class.forName(name, true, loader)`、`Method.invoke`（静态方法）、`Constructor.newInstance`、`Field.get/set`（静态字段） |
| 方法句柄 | `REF_getStatic` / `putStatic` / `invokeStatic` / `newInvokeSpecial` 种类的句柄**首次调用**时初始化 |
| 显式 | `MethodHandles.Lookup.ensureInitialized`、`Unsafe.ensureClassInitialized`、`jdk.internal.reflect.Reflection.ensureClassInitialized`（JDK 内部广泛使用，尤其是序列化与 `VarHandle` 路径） |
| 启动 | 含 `main` 的类 |

`ldc` Class 常量、`instanceof`、`checkcast`、数组创建**不**触发初始化。

初始化失败语义：`<clinit>` 抛异常 → `ExceptionInInitializerError`；之后再访问该类 → `NoClassDefFoundError`。这两个错误类型必须在闭包内。

---

## 阶段 5：根集合

### 5.1 显式入口

`main`、测试入口；开放世界下的导出 API（阶段 11）。

### 5.2 JVM 隐式必需类

以目标 VM 源码为准（HotSpot：`vmClassMacros.hpp` / `vmClasses.hpp`、`vmSymbols.hpp`、`javaClasses.cpp`）。典型包括：

- `Object`、`String`、`Class`、`ClassLoader`、`Module`、`System`、`Thread`、`ThreadGroup`、`Runtime`
- `Cloneable`、`Serializable`（数组超类型）、`Record`
- `Throwable`、`Error`、`Exception`、`RuntimeException`、`LinkageError` 家族、`VirtualMachineError` 家族、`StackTraceElement`
- 基本类型包装类、`Boolean.TYPE` 等
- `java.lang.ref.*`：`Reference`、`SoftReference`、`WeakReference`、`FinalReference`、`PhantomReference`、`Finalizer`
- `java.lang.invoke.*`：`MethodHandle`、`VarHandle`、`MemberName`、`ResolvedMethodName`、`MethodHandleNatives`、`LambdaForm`、`CallSite` 家族
- 反射：`AccessibleObject`、`Field`、`Method`、`Constructor`、`Parameter`
- `StackWalker`、`StackFrameInfo`
- 虚拟线程：`Continuation`、`ContinuationScope`、`StackChunk`、`VirtualThread`
- `jdk.internal.misc.Unsafe`、`UnsafeConstants`

### 5.3 VM 直接访问的字段（字段级裁剪必须视为根）

VM 通过 `javaClasses.cpp` 计算偏移量直接读写以下字段，静态字节码中可能看不到任何访问：

- `String.value` / `coder` / `hash` / `hashIsZero`
- `Class.classLoader` / `module` / `componentType` / `classData` / `reflectionData`
- `Thread.holder`（`Thread$FieldHolder`）/ `eetop` / `name` / `contextClassLoader` / `interrupted`
- `Throwable.backtrace` / `stackTrace` / `depth` / `detailMessage`
- `Reference.referent` / `queue` / `next` / `discovered`
- `ClassLoader.parent` / `name` / `unnamedModule` / `nameAndId`
- `Module.loader` / `name`
- `MemberName.*`、`LambdaForm.vmentry`、`CallSite.target`
- 包装类的 `value` 字段
- **反射对象由 VM 填充的字段**：`Class.getRecordComponents0` 构造的 `RecordComponent.clazz / name / type / accessor / signature / annotations`，`Method` / `Field` / `Constructor` 的 `clazz / slot / name / type / modifiers` 等。这些字段没有任何 Java 端 `putfield`，但 `RecordComponent.getAccessor` → `Method.invoke` 整条链依赖它们；漏掉即漏类。
- 对 `<init>` 不可见的 `@Stable`、`@Contended` 语义也要保留

### 5.4 JVM 上调（upcall）

即 VM 主动调用的 Java 方法，**最容易遗漏的根**。典型清单（以目标 VM 版本源码为准）：

| 场景 | 方法 |
|---|---|
| 启动 | `System.initPhase1/2/3`、`ModuleBootstrap.boot`、`LauncherHelper.checkAndLoadMain` |
| 关闭 | `Shutdown.shutdown`、`Runtime.exit` 路径、关闭钩子 |
| 线程 | `Thread.run`、`Thread.exit`、`Thread.dispatchUncaughtException`、`ThreadGroup.uncaughtException` |
| 虚拟线程 | `Continuation.enter` / `enterSpecial`、`VirtualThread.run`、`VirtualThread.runContinuation` |
| 类加载 | `ClassLoader.loadClass`、`ClassLoader.findNative`、`BootLoader` 相关 |
| 引用处理 | `Reference.processPendingReferences`、`Finalizer.register`、`Finalizer.runFinalizer`、`Cleaner` |
| 方法句柄链接 | `MethodHandleNatives.linkCallSite`、`linkMethod`、`linkDynamicConstant`、`linkMethodHandleConstant`、`findMethodHandleType` |
| 栈遍历 | `StackWalker.doStackWalk`、`Reflection.getCallerClass`（`@CallerSensitive`） |
| 信号 | `jdk.internal.misc.Signal.dispatch` |
| 异常 | `Throwable.<init>`、`Throwable.fillInStackTrace`、`StackTraceElement.of`、`Throwable.printStackTrace`（默认未捕获异常处理） |
| 反射实现 | `Class.getName`、`Class.getConstantPool` 等 VM 缓存路径 |

### 5.5 native 层与外部代码回调的 Java 方法

- JNI：`FindClass`、`GetMethodID`、`GetFieldID`、`CallXxxMethod`，需要从 native 源码人工或工具提取。
- JNI `ThrowNew` / `Throw`：native 抛出的异常是**实例化来源**。
- **库加载回调**：`System.load` / `loadLibrary` 触发 `JNI_OnLoad`，卸载触发 `JNI_OnUnload`；`RegisterNatives` 动态绑定的 native 方法及其在 native 侧回调的 Java 方法都是根。JDK 自身大量类在 `<clinit>` 中调用 `registerNatives()`，这条边由字节码可见，但绑定后 native 侧的回调不可见。
- FFM API：`Linker.upcallStub` 的目标方法句柄，native 通过它回调 Java。
- 目标环境自己手写的 native 实现层引用的 Java 方法。

### 5.6 配置规则

补足反射、服务加载、序列化、代理等静态推断不到的依赖（阶段 13）。

---

## 阶段 6：不动点求解

### 6.1 类型分层（不能只有"可达 / 不可达"）

| 层级 | 含义 | 需要保留的内容 |
|---|---|---|
| Referenced | 类型名被引用（checkcast、签名、catch、StackMapTable） | 类型本身及其超类型 |
| Initialized | 会执行 `<clinit>` | 加上 `<clinit>` 及其依赖 |
| Instantiated | 存在该类型的实例 | 加上参与虚分派 |
| Dispatch-relevant | 需要 vtable / itable / trait 槽位 | 加上被覆盖的声明 |

### 6.2 字段和方法分层

- 字段：只读、只写、读写、VM 访问。只写字段可删，但要保留写入表达式的副作用。
- 方法：直接调用、分派目标、仅结构需要（抽象声明或槽位）、native。

### 6.3 实例化的全部来源

除 `new` 外：`Object.clone`、`Unsafe.allocateInstance`、`ReflectionFactory.newConstructorForSerialization`、`Constructor.newInstance` / `Class.newInstance`、`Array.newInstance`、native `AllocObject` / `NewObject`、lambda 与代理的合成类、反序列化、`Enum.valueOf` 路径、条件常量 `ConstantBootstraps.invoke`。

### 6.4 主循环

```
worklist 元素 = Reachable(m) | Instantiate(C) | Initialize(C) | FieldAccess(f) | VirtCall(T,sig)

process(Reachable m):
    逐条扫描 m 的指令、异常表和属性（阶段 7），产生新事件

process(Instantiate C):
    Initialize(C)
    for (T, sig) in virtual_sites where C <: T:
        Reachable(select(C, sig))              # 回放已登记的虚调用

process(Initialize C):
    Initialize(super(C))
    for I in superinterfaces(C) where I 声明了默认方法: Initialize(I)
    Reachable(C.<clinit>)

process(VirtCall T, sig):
    virtual_sites[(T,sig)] += site
    for C in instantiated where C <: T:
        Reachable(select(C, sig))              # 对已实例化类型求解
```

**关键是双向补边**：新调用点要对已实例化类型求解，新实例化类型要回放已登记的调用点。任一方向漏掉，闭包都不可靠。

---

## 阶段 7：逐指令与逐属性规则

### 7.1 字节码指令

| 指令 | 规则 |
|---|---|
| `new C` | Instantiate(C)，隐含 Initialize(C) |
| `newarray` / `anewarray` / `multianewarray` | 数组类型可达；元素类型 Referenced；可能抛 `NegativeArraySizeException` |
| `invokestatic` | 直接边；Initialize(owner) |
| `invokespecial` | 直接边，按阶段 4.4 三种语义解析 |
| `invokevirtual` / `invokeinterface` | 登记虚调用；特判 nestmate 私有调用、签名多态方法、数组接收者、接口引用命中 `Object` 方法 |
| `invokedynamic` | 引导方法可达并 Initialize 其所属类；按阶段 8.1 建模；可能抛 `BootstrapMethodError` |
| `getstatic` / `putstatic` | 字段读 / 写；Initialize(声明类)，编译期常量除外 |
| `getfield` / `putfield` | 字段读 / 写 |
| `checkcast` / `instanceof` | 类型 Referenced；`checkcast` 可抛 `ClassCastException` |
| `ldc` Class | 类型 Referenced，不初始化 |
| `ldc` String | `String` 类型；`String` 需已初始化（intern） |
| `ldc` MethodType | 其中的参数和返回类型 Referenced |
| `ldc` MethodHandle | 按引用种类（`REF_getField` … `REF_invokeInterface`、`REF_newInvokeSpecial`）产生对应边；首次调用可能触发初始化 |
| `ldc` Dynamic（condy） | 引导方法可达；静态参数递归处理；`ConstantBootstraps.*` 按阶段 8.1 建模 |
| `athrow` | 异常类型已由创建点实例化；保留可能的处理器路径 |
| `monitorenter` / `monitorexit` | 隐式异常 |
| `aastore` | `ArrayStoreException` |
| `arraylength`、`xaload` / `xastore` | 隐式异常 |
| `idiv` / `irem` / `ldiv` / `lrem` | `ArithmeticException` |
| `jsr` / `ret`（classfile ≤ 49） | 子程序：要么先做子程序内联再分析，要么明确报错 / 进入保守模式。**不能静默跳过**，否则子程序体内的所有边全部丢失 |

### 7.2 隐式异常与错误完整映射

| 异常 / 错误 | 来源 |
|---|---|
| `NullPointerException` | `getfield`、`putfield`、`invoke*`、`arraylength`、`athrow`、`monitor*`、数组读写、拆箱 |
| `ArrayIndexOutOfBoundsException` | `xaload`、`xastore` |
| `ArrayStoreException` | `aastore` |
| `NegativeArraySizeException` | 数组创建 |
| `ArithmeticException` | 整数除法 / 取余 |
| `ClassCastException` | `checkcast` |
| `IllegalMonitorStateException` | `monitorexit`、`wait` / `notify` |
| `BootstrapMethodError` | `invokedynamic`、condy |
| `ExceptionInInitializerError`、`NoClassDefFoundError` | 类初始化失败及其后续访问 |
| `AbstractMethodError`、`IncompatibleClassChangeError`、`IllegalAccessError`、`NoSuchMethodError`、`NoSuchFieldError`、`UnsatisfiedLinkError`、`VerifyError`、`ClassFormatError` | 链接与 native 绑定 |
| `OutOfMemoryError`、`StackOverflowError`、`InternalError` | 全局 |

这些类型要 Instantiate，并连带其构造器（含 `Throwable.<init>` → `fillInStackTrace`、`StackTraceElement`）的依赖。

### 7.3 方法与类的附属依赖

- 异常表 catch 类型：Referenced。
- `Exceptions` 属性：需保留 API 签名时为 Referenced。
- 方法描述符、字段描述符、record 分量描述符、`Signature` 中出现的全部类型：Referenced。
- **`StackMapTable` 中的类型：Referenced。** 若输出在真实 JVM 上运行，校验器会加载这些类做可赋值性判断，删掉会抛 `NoClassDefFoundError`。
- `LocalVariableTypeTable`（保留调试信息时）：Referenced。
- 注解类型与 `AnnotationDefault` 引用：见阶段 8.4。
- `InnerClasses` / `EnclosingMethod` / `NestHost` 引用的类型：结构需要，见阶段 12。

---

## 阶段 8：语言特性与动态特性建模

### 8.1 `invokedynamic` 与 condy，按引导方法分类

| 引导方法 | 规则 |
|---|---|
| `LambdaMetafactory.metafactory` / `altMetafactory` | 实现方法可达（构造器引用则 Instantiate 对应类型）；视为存在一个合成类实现函数式接口，其 SAM 分派到实现方法；`altMetafactory` 还要处理桥方法、标记接口、可序列化 lambda（`$deserializeLambda$`） |
| `StringConcatFactory.makeConcat*` | 每个对象参数隐含 `String.valueOf` → 对 `toString()` 的虚调用；基本类型参数隐含对应 `toString` 静态方法 |
| `ObjectMethods.bootstrap`（record） | 全部分量访问器、`equals` / `hashCode` / `toString` 依赖，分量类型的 `equals` / `hashCode` |
| `SwitchBootstraps.typeSwitch` / `enumSwitch` | 标签中的类型和枚举常量；record 模式 → 分量访问器 |
| `ConstantBootstraps.primitiveClass` / `enumConstant` / `getStaticFinal` / `invoke` / `nullConstant` / `fieldVarHandle` / `staticFieldVarHandle` / `arrayVarHandle` / `explicitCast` | 按参数建模：枚举常量、静态字段读、目标方法句柄可达 |
| 其他 / 未知引导方法 | 保守处理或按配置处理，并在报告中列出 |

### 8.2 编译器生成的结构

- **枚举**：`values()`、`valueOf()`、`$VALUES`。`Enum.valueOf`、`Class.getEnumConstants`、`EnumSet`、`EnumMap`、序列化会**反射调用 `values()`**，枚举类型可达则 `values()` 必须保留。
- **枚举 switch**：`$SwitchMap$` 合成类，调用 `ordinal()`。
- **字符串 switch**：`hashCode()` 与 `equals()`。
- **断言**：`$assertionsDisabled`、`Class.desiredAssertionStatus`。
- **内部类**：外部类引用、合成访问器 `access$NNN`（Java 11 前）、`this$0` 字段。
- **自动装箱 / 拆箱**：显式 `valueOf` / `xxxValue` 调用，按普通指令处理。
- **try-with-resources**：`close()` 虚调用、`Throwable.addSuppressed`。
- **sealed**：`PermittedSubclasses` 可收窄分派目标（与阶段 10.3 去虚化配合）。前提是该属性在语料中真实存在且完整；经反射 / 动态代理生成的类型不受 sealed 约束，不可依赖。
- **record**：规范构造器，反射和序列化用。
- **桥方法**：见阶段 4.7。

### 8.3 反射与字符串驱动的依赖

先做**过程内常量传播 / 字符串分析**，尽量推断具体目标：

- `Class.forName("字面量")`、`X.class.getMethod` / `getDeclaredMethod("名", 参数类型)`、`getField` / `getDeclaredField`、`getConstructor(...).newInstance`、`Class.newInstance`
- `Array.newInstance`
- **枚举式反射（结构反射）**：见 8.3.2，语义是**整类成员保留**，与按名查找不同。
- 字段名字符串：`AtomicInteger/Long/ReferenceFieldUpdater.newUpdater(类, "字段")`、`Unsafe.objectFieldOffset(类, "字段")`、`MethodHandles.Lookup.findVarHandle` / `findStaticVarHandle`
- `MethodHandles.Lookup.findVirtual` / `findStatic` / `findSpecial` / `findConstructor` / `findGetter` / `findSetter` / `unreflect*`
- `ServiceLoader.load(X.class)`：结合 `META-INF/services` 与 `provides`，所有提供者的公共无参构造器（或静态 `provider()`）可达。
- `ResourceBundle.getBundle(name)`：按 `name_语言_地区_变体` 模式加载类和 `.properties`。已实施（seeds.toml `[bundles]`，`engine/bundles.rs`）：调用点基名按键值求值（lambda 捕获的基名经捕获值对齐追到创建点，`engine/lambda_vals.rs`），推不出时回退到调用链上的束形字面量；按入选 locale 父链展开，束类反射构造补种、属性文件入模块资源表。
- 系统属性 / 配置文件中的类名：各类 SPI 默认实现类名。
- `Class.getName()` → `Class.forName()` 的字符串回流。

推断失败的调用点统一**报告**，由配置规则补足。

#### 8.3.1 反射结果的回流建模

上面解决的是"找到目标"。另一半问题是**反射调用的返回值如何流回图**：`Method.invoke`、`Constructor.newInstance`、`Field.get`、`MethodHandle.invoke` 的静态返回类型都是 `Object`。若直接按 `Object` 处理（open），下游所有汇点（`toString`、`equals`、`hashCode`、集合的 `put` / `get`）都会被污染，这是成员级闭包收敛不下去的头号原因。

做法是一层"常量反射数据流"：

1. 在同一方法内把 `(Class 常量, 名字常量, 参数类型常量)` 与后续的 `invoke` / `newInstance` / `get` 汇点配对（沿局部变量与栈的 def-use 链）。
2. 配对成功 → 用解析出的目标方法 / 构造器 / 字段的**声明类型**作为结果的静态类型，参与后续分派收窄。
3. 配对失败 → 结果类型 open 为 `Object`，同时把该调用点列入报告，供配置规则补足或人工审查。
4. `MethodHandle` 链（`asType`、`bindTo`、`filterReturnValue`）按同样思路做过程内类型传播；跨方法传播的句柄默认 open。

这条规则同时影响可靠性（反射目标可达）和精度（结果类型是否收窄），两者都要在阶段 15 验证。

#### 8.3.2 结构反射（成员枚举 API）

`getDeclaredMethods` / `getMethods` / `getDeclaredFields` / `getFields` / `getDeclaredConstructors` / `getConstructors` / `getDeclaredClasses` / `getClasses` / `getRecordComponents` / `getPermittedSubclasses` / `getNestMembers` / `getEnumConstants` / `getInterfaces` / `getAnnotations`。这类 API 枚举一个类的全部成员或关系，是成员级裁剪的最大盲区：

- 接收者类可静态确定（`X.class`、常量传播结果）→ 该类的对应成员全部保留（成员级裁剪在这个类上退化为类级），并记录为报告项，便于评估其对闭包大小的影响。
- 接收者类不能确定 → 报告，由配置规则决定；默认策略按阶段 0 的未知处理策略。
- 枚举出的 `Method` / `Field` / `Constructor` 对象后续的 `invoke` / `get` / `newInstance` 按 8.3.1 处理：结果类型 open，目标为该类的全部对应成员。

#### 8.3.3 动态类定义

`ClassLoader.defineClass`、`Unsafe.defineClass`、`MethodHandles.Lookup.defineClass` / `defineHiddenClass`、`Instrumentation.redefineClasses`。类字节在运行时产生，静态分析原则上不可见。策略：

- 默认列为不支持并报告调用点。
- 若配置提供了"额外字节码输入"（例如已知生成器的产物），把它们当作普通输入类加入阶段 2。
- JDK 内部的动态类生成单独按阶段 9 处理。

### 8.4 代理与注解

- **`Proxy.newProxyInstance` / `Proxy.getProxyClass`**：所列接口的所有方法都可被调用，进入 `InvocationHandler.invoke`；`Object` 的 `equals` / `hashCode` / `toString` 同样经代理转发。
- **运行时注解**：`getAnnotation` 会生成注解接口的代理，注解元素方法、`AnnotationDefault` 引用的类型（枚举、类字面量、嵌套注解）要保留；`@Inherited` 使查找沿超类链进行；`@Repeatable` 的容器注解。

### 8.5 序列化

- **`Serializable`**：魔法方法 `writeObject`、`readObject`、`readObjectNoData`、`writeReplace`、`readResolve`；字段 `serialVersionUID`、`serialPersistentFields`；第一个不可序列化超类的无参构造器（经 `ReflectionFactory.newConstructorForSerialization`，不调用本类构造器）。
- **`Externalizable`**：`writeExternal`、`readExternal`、公共无参构造器。
- **record**：通过规范构造器反序列化。
- **枚举**：按名称经 `Enum.valueOf` 反序列化。
- **代理类**：`Proxy` 序列化描述符。
- **可序列化 lambda**：`SerializedLambda` 与 `$deserializeLambda$`。

### 8.6 其他隐式调用

- `Thread.start` → `run`（经 native）
- `AccessController.doPrivileged` → `run`
- `finalize`：覆盖了 `Object.finalize` 的已实例化类型 → `Finalizer.register` 上调
- `Cleaner` / `Reference` 回调、`ReferenceQueue`
- 关闭钩子（`Runtime.addShutdownHook`）
- `Object.clone` 对数组和 `Cloneable` 的语义（不经构造器实例化）
- `Unsafe.allocateInstance`：Instantiate(C) 但**不加**构造器边（绕过构造器；`<clinit>` 仍触发）
- `System.load` / `loadLibrary` → `JNI_OnLoad`；`RegisterNatives` 绑定的方法（阶段 5.5）
- `Object.wait` / `notify` 的 `IllegalMonitorStateException`
- `ThreadLocal.initialValue`、`InheritableThreadLocal.childValue`

### 8.7 资源闭包

与类闭包并行计算的第二个闭包：

- `Class.getResource*` / `ClassLoader.getResource*` / `Module.getResourceAsStream` 的字面量参数；名字经拼接 / 字段得出的由 seeds.toml `[resource_lookups]` 入口按名求值（`engine/res_lookups.rs`，只取全为已知段的候选），种子事实 `named_resources`
- `ResourceBundle` 文件（`.properties` 束由 8.3 的 getBundle 建模给出，种子事实 `named_resources`）
- `META-INF/services/*`
- 输出时保留被引用的资源；无法推断的资源访问点进入报告。

---

## 阶段 9：JDK 自身的运行时代码生成

JDK 内部会在运行时生成类，静态 classfile 中不存在，是闭包分析的盲区：

| 机制 | 生成器 |
|---|---|
| lambda 实现类（隐藏类） | `InnerClassLambdaMetafactory` |
| 动态代理 | `ProxyGenerator` |
| `LambdaForm` 编译 | `InvokerBytecodeGenerator` |
| `BoundMethodHandle` species | `ClassSpecializer` |
| 反射访问器（JDK ≤ 17） | `MethodAccessorGenerator`；JDK 18+ 改为方法句柄（JEP 416） |
| 用户代码 | `Lookup.defineHiddenClass`、`ClassLoader.defineClass`、`Instrumentation` |

两种策略（阶段 0 必须选定）：

1. **保留 JLI 运行时**：把 `java.lang.invoke` 整套实现（`MethodHandleImpl`、`LambdaForm`、`DirectMethodHandle`、`Invokers`、字节码生成器及其对 ASM/ClassFile API 的依赖）作为根纳入闭包。闭包会显著变大，但语义完整。
2. **静态 desugar**：分析前把 lambda、字符串拼接、record 方法、switch 等 indy 调用点降级为普通类和调用（类似 R8 / retrolambda 的做法），从而不需要 JLI 运行时。对转译器类消费者通常是更好的选择，但要覆盖全部引导方法种类，并对未知引导方法报错。

无论哪种策略，用户代码的 `defineClass` / `defineHiddenClass` 都属于"不支持的动态特性"，需在 soundy 声明中列出。

---

## 阶段 10：精度提升（可选层级）

1. **过程内精化**
   - 常量折叠并剪掉死分支（例如 `static final boolean` 开关）。
     - 注意：只能折叠 `ConstantValue` 属性中的编译期常量；运行时计算的 `static final` 需要 `<clinit>` 模拟或纯度分析才能折叠，否则不可靠。
   - 利用局部分配点的精确类型收窄接收者类型。
   - 空值分析，减少隐式 NPE 路径。
2. **`<clinit>` 纯度分析**：无副作用的初始化不作为强制依赖；可进一步做类初始化模拟（GraalVM 的做法）。
3. **sealed / final 收窄**：接收者类型 final 或 sealed 层级已知时直接去虚化。
4. **算法升级**：RTA → XTA（每方法 / 每字段类型集合）→ VTA（每变量）→ 上下文敏感 → 指针分析。
   - **值带来源标签**：XTA / VTA 落地时，让分派真正收窄的不是裸类型集合，而是每个值携带**来源**——来自哪个分配点、哪个形参、哪个 catch、哪个字符串 / 类常量、哪个反射汇点（阶段 8.3.1）。类型集合是来源集合的投影；没有来源，汇点的类型集合只会单调膨胀，无法按调用点区分。
5. **上下文敏感性（成员级闭包的精度天花板）**：
   - 问题：`toString` / `hashCode` / `equals` / `compareTo` / `Comparator.compare` 这类**公共汇点**被全部代码共享，XTA 在它们身上仍会汇出上百个分派目标。原因是无上下文分析把所有调用者的实参并合到同一个形参节点。
   - 三档解法，按代价递增：
     | 上下文 | 区分依据 | 适用汇点 |
     |---|---|---|
     | 调用点敏感（k-CFA） | 调用链最近 k 个调用点 | 工具方法、静态帮助函数 |
     | 对象敏感（k-obj） | 接收者的分配点 | 实例方法、`this` 流 |
     | 容器 / 堆上下文敏感 | 容器对象的分配点 + 元素字段的堆上下文 | `HashMap.put/get`、`ArrayList` 元素回调用户对象的 `hashCode/equals` |
   - 实践上通常**混合**：默认无上下文，只对被仪表（阶段 14.4）识别出的巨型汇点启用对象敏感或容器敏感；这是精度 / 代价比最好的做法。
   - 上下文敏感的规模问题见阶段 16.7。
6. **权衡**：每升一级都用阶段 15 的指标（尤其是每站点目标数分布）衡量收益。

---

## 阶段 11：开放世界建模（仅库场景）

1. **外部调用者**：导出包中的 public / protected 方法、构造器、字段都是根。限定导出（`exports … to`）与 `opens` 单独处理。
2. **外部子类**：
   - 对每个**非 final、非 sealed 的导出类型 T**，引入虚拟类型 ⊤_T，视为已实例化。
   - 它不贡献方法体，但可覆盖 T 的所有可覆盖方法，因此库中对 T 的虚调用必须保留分派机制。
   - T 的可访问构造器、protected 成员要保留。
3. **外部对象流入**：API 参数、返回值、可写字段中出现的类型，视为可能是外部实现。典型例子是集合回调用户对象的 `hashCode` / `equals` / `compareTo`、`Comparator.compare`。
4. **外部异常**：外部子类可能抛出任意 `Throwable`，库中 catch 类型只需 Referenced。
5. **外部反射**：`opens` 的包允许深度反射，私有成员也要保留。
6. **外部 native**：库导出的 native 方法可能被外部实现。

---

## 阶段 12：结构一致性补全与桩化

不动点求出后，做一次闭合检查，保证结果本身合法：

1. 类型被保留 → 其全部超类型保留（Referenced 层级）。
2. 保留的虚方法在超类型中覆盖的声明要保留（vtable / itable 槽位）。
3. 已实例化的具体类必须对所有可达的抽象声明提供实现；实现被裁掉时生成**桩**。
4. 保留成员的描述符、签名、`StackMapTable` 中出现的类型要保留。
5. 构造器链完整：子类构造器调用的父类构造器要保留，最终到 `Object.<init>`。
6. 嵌套关系：`NestHost`、`InnerClasses`、`EnclosingMethod` 引用的类型要保留，否则 `getSimpleName` / nestmate 访问失败。
7. 枚举：`values()` / `valueOf()` / `$VALUES` 与所有常量一起保留。
8. record：`Record` 属性、访问器、规范构造器一致。
9. **桩化策略**：结构上需要、语义上不可达的成员，替换为抛异常的方法体（`AbstractMethodError` 或自定义错误），不能直接删除。
10. **字段布局**：若输出为 classfile 且 VM 直接访问字段（阶段 5.3），这些字段的存在与顺序不可变动。
11. **编译单元闭包（按 `SourceFile` 而非按引用边）**：当输出按编译单元组织（一个源文件生成一个目标文件 / 模块），或下游需要 `InnerClasses` / `EnclosingMethod` 在同一单元内一致时，同一 `SourceFile` 的全部类——嵌套类、匿名类、同文件的兄弟顶层类——必须一起进入闭包（至少 Referenced 级）。`access$NNN` 合成访问器规则只覆盖内部类与外部类之间有调用边的情况，覆盖不了"同文件但无引用边"的兄弟类。这条规则缺失的典型症状是：主类生成了，辅助类文件根本没有生成，下游整批编译失败。
12. **资源与描述符一致性**：`META-INF/services/*` 与 `module-info` 的 `provides` / `uses` / `opens` / `exports` 中引用的类被裁掉时，资源文件与模块描述必须同步更新（重写场景见阶段 14.8）。

补全可能引入新的可达元素，**回到阶段 6 再迭代**，直到稳定。

---

## 阶段 13：配置规则语言

参考 ProGuard / R8 keep rules 与 GraalVM reachability-metadata 的设计：

1. **规则种类**：保留类 / 保留成员 / 保留类及成员 / 仅保留名字（不裁剪但允许优化）。
2. **匹配**：通配符（`*`、`**`、`***`）、按修饰符、按注解、按超类型。
3. **条件保留**：`-if` 语义——仅当某类可达时才保留另一组成员。
4. **分类**：反射、JNI、序列化、代理、资源、服务加载、上调，每类独立文件。
5. **失效检测**：未命中任何元素的规则要报告，避免规则腐化。
6. **来源标注**：每条规则记录来源（人工、动态预言机、工具生成），便于审计。
7. **优先级与冲突**：规则只增不减，不需要冲突消解，但需要去重。

---

## 阶段 14：输出与可解释性

1. **分层集合**：类型（4 层）、方法（4 类）、字段（读 / 写 / VM 访问）、native 方法清单、资源清单。
2. **原因图**：每个元素记录第一条使它可达的前驱边。支持：
   - "为什么可达"：最短原因链
   - "为什么不可达"：没有任何规则命中的说明
3. **分层结果 → 下游形态契约**（阶段 0 已约定，此处是输出格式的体现）：

   | 层级 | 下游应生成的内容 | 下游**不应**生成的内容 |
   |---|---|---|
   | Referenced | 类型名 / 类型声明、超类型关系 | 方法体、字段访问器、分派表、元数据 |
   | Initialized | 加 `<clinit>` 与静态字段 | 实例方法体 |
   | Instantiated | 加构造器、实例字段、可达实例方法 | 未可达方法 |
   | Dispatch-relevant | 加 vtable / itable / trait 槽位与覆盖声明 | 无关槽位 |

   输出中每个类型必须**显式标注层级**，下游必须按层级瘦身。若下游对 Referenced 级类型仍无条件展开全部产物（分派表、祖先转换链、访问器、元数据），这类类型的体量与全量类无异，闭包分析的收益会被完全抵消。Referenced 级类型在实际语料里通常占每个入口闭包的相当比例，这一条决定了分析器是否"有用"。
4. **精度仪表：每站点目标数分布**：输出每个虚调用点的分派目标数，给出直方图与 top-K 巨型汇点清单（调用点、声明类型、目标数、目标来源）。这是决定"往哪里加精度"（阶段 10.5）的唯一直接依据；只看闭包总大小无法定位。
5. **问题报告**：缺失类、无法推断的反射 / 资源访问点、结构反射与动态类定义的命中点、不支持特性的使用位置、未命中的配置规则、未知引导方法、反射结果回流失败点（阶段 8.3.1）。
6. **统计**：各层级数量、各规则贡献量、与上次运行的差异（diff）。
7. **格式**：机器可读（JSON / 二进制）+ 人类可读报告；输出顺序**确定**。
8. **重写制品的后处理**（输出用途为裁剪后的 classfile / JAR / 模块时）：
   - 方法体变更后**重算 `StackMapTable`**（或用 COMPUTE_FRAMES 类的汇编器），并重新通过字节码校验；重算本身会触发类层次查询（LUB 计算），需要闭包中的类型齐全。
   - 常量池重建：清理失效条目；`BootstrapMethods` 与 indy / condy 引用保持一致。
   - 类属性修正：`NestMembers`、`InnerClasses`、`EnclosingMethod`、`PermittedSubclasses`、`Record`、`Exceptions`、`Signature` 指向被删成员或类型时同步修正或删除。
   - `module-info` 与保留的类同步（`exports` / `opens` 的包必须非空，`provides` 的实现必须存在）。
   - `META-INF/services` 与保留的提供者同步；多版本 JAR 的 `META-INF/versions/*` 各层一致裁剪。
   - 桩方法体（阶段 12.9）写入，并保证桩本身引用的错误类型在闭包内。

---

## 阶段 15：验证

1. **自洽验证**：闭包中每段代码引用的每个符号都在闭包中存在（或是桩）。
2. **下游验证**：
   - 重写 classfile：字节码校验器（`-Xverify:all`、ASM `CheckClassAdapter`）；在真实 JVM 上启动并跑测试。
   - 代码生成：跑下游编译。
3. **动态预言机（最关键）**：
   - 用 Java agent、JVMTI、JFR 记录测试套件实际加载的类、执行的方法、访问的字段；类级可用 `-Xlog:class+load`。
   - 检查"观测集 ⊆ 静态闭包"。每个违例都是可靠性漏洞，通常出在反射、上调、invokedynamic、VM 字段访问上。
   - **这是发现"你不知道自己遗漏了什么"的唯一系统方法。**
   - **覆盖偏差**：预言机只能发现测试执行到的路径。要配合 JDK 自身的 jtreg 套件、第三方库测试、以及基于规则的模糊测试扩大覆盖。
   - **对照必须按层级进行**：JVM 的"加载" ≠ "链接" ≠ "初始化" ≠ "执行"。`-Xlog:class+load` 观测到的类只对应 Referenced 级；`-Xlog:class+init` 对应 Initialized 级；方法执行观测（JVMTI `MethodEntry`、JFR、agent 插桩）才对应可达方法。把"加载集"直接对"方法体集合"会产生大量假违例，反而误导精度调整方向。
   - **对照域要排除被替代的运行时基础设施**：若目标运行时用自有实现替代了 JDK 的某些机制（字符集选择、`ServiceLoader` 表驱动初始化、`java.lang.invoke` 的类生成、CDS / 模块系统启动），HotSpot 的加载集中会有大量永远不需要进闭包的类。一个几行 `println` 的程序 HotSpot 也会加载数百个类，其中大部分属于此类。必须先建立"排除清单"（可从阶段 9 的策略与阶段 5.2 的隐式类清单推导），否则预言机会被假违例淹没。
   - **违例归因**：每个真违例归因到"缺一条配置规则"还是"缺一类规则"（分析器本身的漏洞）；后者要改分析器，不能只加配置。
4. **差分验证**：相同根和配置下，与 jdeps（类级）、R8 / ProGuard（`-printusage`）、GraalVM native-image（`-H:+PrintAnalysisCallTree`）的结果对比，逐项分析差异。
5. **精度评估**：静态闭包 / 动态观测集的比值（按层级分别计算）；各算法层级下闭包大小；每站点目标数分布与 top-K 巨型汇点的变化（阶段 14.4）；"删一个成员看是否编译失败"的抽样测试检验闭包是否偏大。
6. **回归测试**：每条规则一个最小用例程序，加黄金文件比对。

---

## 阶段 16：工程化

1. **性能**：ID 驻留、位集、CSR 邻接表；并行解析；缓存解析与选择结果；惰性解析方法体；worklist 去重。
2. **增量分析**：按类内容哈希缓存解析结果；输入变化时只让受影响部分重新进入 worklist。
3. **确定性**：遍历顺序稳定，同样输入每次输出完全一致。
4. **可配置性**：规则 DSL、算法层级开关、每类特性开关、soundy 声明。
5. **可观测性**：迭代次数、worklist 峰值、每条规则耗时、各阶段耗时。
6. **实现路线**：Datalog（Soufflé）便于扩展维护；原生实现（位集 + 增量 worklist）对 RTA 级的 JDK 规模足够。
7. **上下文敏感的溢出策略**：位集 / CSR / 惰性解析只解决 RTA 规模的问题；上下文敏感分析在数千方法的图上上下文数会爆炸。需要：
   - **上下文数上限**：每个方法 / 每个堆对象的上下文数设上限，超限回退到共享（无上下文）节点，并在报告中标注"此处精度已降级"。
   - **选择性启用**：仅对阶段 14.4 识别出的巨型汇点及其上游启用上下文，其余保持无上下文。
   - **增量重算粒度**：以调用点 / 字段节点为单位重算，而不是整图重跑；上下文节点的失效传播要有界。
   - **超时与降级**：分析有全局时间预算，超时后按已求得的上近似输出（仍可靠，只是不最小），并记录未收敛的节点。

---

## 阶段 17：迭代工作流与已知局限

### 17.1 工作流

```
定义规格 → 建根集合 → 求闭包 → 结构补全 → 下游验证
        ↑                                    │
        └──── 补配置规则 ←── 动态预言机 / 差分 ←┘
```

每一轮预言机违例都要归因到"缺规则"还是"缺规则种类"（分析器本身的漏洞），后者要修改分析器而不是加配置。

### 17.2 施工顺序：两级里程碑

前面的阶段是**知识组织顺序**，不是施工顺序。实际施工分两级，且不可颠倒：

1. **里程碑 ①：RTA 正确性闭环**
   - 目标：动态预言机违例 = 0（按层级对照，排除清单已建立），先不追求闭包小。
   - 范围：阶段 2–8、11、12 的全部规则，精度停留在 RTA。
   - 验收：全部测试入口在闭包产物上通过下游编译与运行。
2. **里程碑 ②：精度逐级升级**
   - 每升一级（XTA → 值来源 → 选择性上下文敏感）都用阶段 14.4 的每站点目标数与阶段 15.5 的比值验收，收益不足以覆盖代价的一级不上线。
   - 每次升级后重跑里程碑 ① 的全部验收，精度升级不得引入可靠性回退。

### 17.3 多入口闭包的组织

多测试 / 多产品场景下，闭包不是一个而是一组：

- 先求所有入口的**共享底座**（隐式类、上调、JLI 运行时或 desugar 产物、公共基础类）——它对所有入口相同，只算一次并缓存。
- 再对每个入口求 **per-entry 增量闭包** = 该入口闭包 − 共享底座。
- 构建编排按"底座一次 + 增量并行"组织；底座变化才使全部增量失效，入口变化只重算自己的增量。
- 输出中标注每个元素属于底座还是哪些入口，便于差分与缓存命中分析。

### 17.4 替换既有实现：双算过渡期

若分析器要替换项目中已有的闭包逻辑（常见的是旧的 BFS / CHA 实现），不能直接切换：

1. **双算并行**：新旧两套在相同输入上同时运行。
2. **逐类 / 逐成员 diff 报告**：每个差异标注方向（新多 / 旧多）和原因链。
3. **人工审差异**："新少旧多"的每一项必须证明旧实现是过近似；"新多旧少"的每一项必须证明旧实现是漏项。
4. 差异清零或全部有结论后再切换；切换后旧实现保留一段时间作为回归对照。

### 17.5 必须写进文档的局限

- 不支持的动态特性：用户 `defineClass` / `defineHiddenClass`、`Instrumentation`、无法推断的反射。
- 依赖目标 VM 版本的上调与字段清单，版本升级需重新核对。
- 多类加载器命名空间是否支持。
- 预言机的覆盖偏差。

---

## 阶段 18：工业级要求

前 17 个阶段回答的是"分析什么、怎么分析"。工业级工具与设计规格的差距在于：规则是否可验证、可版本化、可扩展、可互操作，以及对畸形输入是否健壮。本阶段列出这些质量属性，以及为满足它们仍需补充的分析内容。

### 18.1 补充的分析内容

1. **异常流分析**：默认把所有 catch 块视为可达是过近似。做过程内 + 简单过程间的"可能抛出类型"传播（声明的 `Exceptions`、`athrow` 的静态类型、隐式异常表），只有能到达 handler 的异常类型才使该 handler 的代码可达。属于精度项，与阶段 10 并列。
2. **类初始化策略与堆快照**：若下游支持"构建期初始化"（分析期执行 `<clinit>`，把结果对象快照进产物），快照堆中的每个对象都使其类 Instantiated、其引用的类可达——这是**独立于代码的根来源**。需要：可安全构建期初始化的判定（`<clinit>` 纯度分析的扩展，阶段 10.2）、堆快照的遍历规则、运行期初始化类的显式清单。
3. **替换机制（substitution）**：在分析期用替代实现覆盖指定方法体（GraalVM 的 `@TargetClass` / `@Substitute`、`@Delete`），用于切断反射重、平台相关或不需要的依赖路径。替换后的方法体按普通方法参与分析。这是工业工具缩小 JDK 闭包的核心手段，与阶段 12.9 的桩化不同：桩是结构占位，替换是语义等价的重实现。
4. **假设规则**：`-assumenosideeffects`（调用可删除）、`-assumevalues`（返回值 / 字段值范围已知，用于死分支折叠）。它们直接改变闭包，属于配置语言（阶段 13）的一部分，并且必须在报告中标注"此处依赖假设"。
5. **模块层与运行时可访问性变更**：`ModuleLayer.defineModules*`、`Module.addOpens` / `addExports` / `addReads`、`Instrumentation.redefineModule`。默认视为不支持并报告；配置可声明额外的 `opens`。
6. **字符串分析边界**：阶段 8.3 的常量传播是过程内的；跨方法的字符串流（`getName()` → 拼接 → `forName`）默认 open 并报告。可选接入专门的字符串分析（JSA 类），但要在 soundy 声明中写明边界。

### 18.2 规则目录与一致性测试

- **规则目录**：每条可达性规则一个 ID（如 `R-INSN-NEW`、`R-INDY-LMF`、`R-UPCALL-THREAD-RUN`），记录规范依据（JVMS 章节 / JDK 源码位置 / 编译器约定）、触发条件、产生的事件、所属层级、引入版本。
- **每条规则一个最小用例**，用例名与规则 ID 对应，黄金文件比对；统计规则覆盖率（哪些规则从未被语料触发）。
- **外部一致性套件**：CATS（OPAL 项目的调用图测试套件，专门覆盖 Java 调用图不可靠来源）、JDK jtreg、DaCapo / Renaissance 基准（作为动态预言机语料）。
- **规范追溯**：每条规则 ↔ 用例 ↔ 本文档章节三向可追溯。

### 18.3 JDK 版本兼容矩阵

规则随 JDK 版本变化，分析器必须按目标版本切换规则集。至少维护以下矩阵（示例，需按目标版本逐条核实）：

| 版本 | 影响闭包的变化 |
|---|---|
| 8 | 无模块系统；`jsr/ret` 仍可能出现在旧依赖；lambda 用 ASM 生成匿名类 |
| 9 | 模块系统、`StringConcatFactory`、多版本 JAR、jimage |
| 11 | nestmates、condy、`Unsafe.defineAnonymousClass` 仍在 |
| 15–17 | 隐藏类替代 `defineAnonymousClass`；record、sealed 定型；`SecurityManager` 弃用 |
| 18 | 反射改为方法句柄实现（JEP 416）：`MethodAccessorGenerator` 消失，`java.lang.invoke` 依赖上升；`finalize` 弃用（JEP 421） |
| 19–21 | 虚拟线程（`Continuation` 上调）、record 模式与 `SwitchBootstraps.typeSwitch` 定型、`SecurityManager` 不可启用 |
| 22–24 | FFM 定型（upcall stub）、`ClassFile` API、`Unsafe` 内存访问方法弃用、字符串拼接策略变化 |
| 25+ | 以发行说明为准 |

每个版本的**上调清单、VM 访问字段清单、隐式类清单**都要单独核对（阶段 5），不能跨版本复用。

### 18.4 配置生态互操作

- **读取既有格式**：ProGuard / R8 keep rules（含 `META-INF/proguard/`、`META-INF/com.android.tools/`）、GraalVM `reachability-metadata.json`（`META-INF/native-image/<group>/<artifact>/`）、社区维护的 GraalVM Reachability Metadata Repository。工业工具不能要求用户为它重写所有配置。
- **追踪代理作为配置生成器**：阶段 15.3 的动态预言机反过来用——运行测试，记录反射 / 资源 / 代理 / 序列化的实际使用，**自动生成配置**（GraalVM `native-image-agent` 的做法），再由人审。同一套观测基础设施同时服务验证与配置生成。
- **配置合并与来源追踪**：多来源配置合并时记录每条规则来自哪个文件，冲突（如一方 `-assumenosideeffects` 另一方 keep）要报告。

### 18.5 扩展 API

- **Feature / 插件机制**：在分析生命周期的固定点（分析前、每轮迭代后、分析后）暴露回调，允许外部代码查询可达性并注册额外根、替换、初始化策略。GraalVM 的 `Feature` 接口是参考设计。
- **框架适配器**：Jackson、Gson、Spring、JPA、gRPC 等反射 / 注解驱动的框架，各自的可达性约定由适配器封装（"被 `@JsonProperty` 标注的成员可达"）。没有扩展 API，反射重的库无法接入。
- **稳定的内部 API**：类层次查询、可达性查询、原因图查询对插件开放，并有版本承诺。

### 18.6 输入健壮性

分析器会处理来自任意来源的 classfile 与 JAR，必须按不可信输入对待：

- **畸形 classfile**：常量池索引越界或成环、属性长度溢出、非法描述符、重复方法、超深继承链、`StackMapTable` 与代码不一致。解析器要在每个字段做边界检查，出错时对该类降级（报告 + 视为 phantom 或保守保留），不能崩溃或死循环。
- **JAR / ZIP**：zip-slip 路径穿越、zip bomb（压缩比 / 解压后总大小上限）、重复条目、超长文件名。
- **资源限制**：单类大小上限、总内存预算、分析时间预算（阶段 16.7）；超限时确定性地失败并给出原因。
- **模糊测试**：解析器与规则引擎都要进 fuzz 流水线（结构感知的 classfile fuzzer + 变异 JDK 语料）。

### 18.7 输出契约版本化

- 输出 schema 带版本号；新增字段向后兼容，删除或语义变更升主版本。
- 下游生成器声明它依赖的 schema 版本；分析器在版本不匹配时拒绝而不是静默输出。
- 原因图、报告、统计的格式同样版本化。

### 18.8 量化验收标准

"衡量收益"必须落成数字门槛，否则无法判断一次改动是否可发布：

| 指标 | 门槛（示例，按项目设定） |
|---|---|
| 可靠性 | 指定语料（jtreg 子集 + 项目测试）上动态预言机违例 = 0，按层级分别统计 |
| 精度 | 相同根与配置下，闭包大小相对 native-image / R8 的偏差 ≤ X%；top-K 巨型汇点目标数不高于基线 |
| 性能 | 指定规模语料 RTA ≤ T1 秒，含选择性上下文敏感 ≤ T2 秒，峰值内存 ≤ M |
| 确定性 | 同一输入 N 次运行输出字节级一致 |
| 规则覆盖 | 规则目录中每条规则至少被一个用例与一个真实语料触发 |
| 健壮性 | fuzz 流水线连续 N 小时无崩溃 / 无超时 |

发布前所有门槛必须通过；任一门槛回退即阻断。

### 18.9 诊断与错误处理

- **分级**：错误（必须处理，如缺失类导致不可靠）、警告（可能不可靠，如无法推断的反射）、提示（精度损失，如上下文溢出）。
- **错误码目录**：每类问题固定错误码，文档中有解释与处理建议。
- **抑制机制**：按错误码 + 位置模式抑制（等价于 `-dontwarn`），抑制本身进入报告，避免静默。
- **退出码**：区分"分析失败""分析成功但有未处理错误""完全成功"，便于 CI 集成。
- **可复现的问题报告**：每个诊断附最小定位信息（类、方法、字节码偏移、原因链），并能导出为独立复现输入。

### 18.10 工业级核对

| 属性 | 工业级要求 |
|---|---|
| 可验证 | 规则目录 + 一致性套件 + 动态预言机 + 量化门槛 |
| 可版本化 | JDK 兼容矩阵、输出 schema 版本、规则引入版本 |
| 可扩展 | Feature API、框架适配器、替换机制、假设规则 |
| 可互操作 | 读取既有配置格式、追踪代理生成配置、元数据仓库 |
| 健壮 | 畸形输入降级、资源限制、fuzz 流水线 |
| 可运维 | 分级诊断、错误码、抑制、退出码、可观测性（阶段 16.5） |
| 可解释 | 原因图、每站点目标数、假设与替换的显式标注 |

---

## 附录 A：完整性核对清单

按 JVM 规范结构逐项对照：

| 范围 | 核对内容 |
|---|---|
| classfile 结构（JVMS 第 4 章） | 所有属性、常量池类型、访问标志 |
| 加载、链接、初始化（JVMS 第 5 章） | 解析、选择、初始化触发、访问控制、链接错误 |
| 指令集（JVMS 第 6 章） | 全部指令的直接依赖与隐式异常；`jsr` / `ret` 内联或报错 |
| 数组类型 | 数组超类型、数组接收者的方法选择（`clone` / `getClass` / `Object` 方法）、多维数组 `checkcast` |
| 校验器 | `StackMapTable` 引起的类加载 |
| JVM 隐式必需类、VM 访问字段、上调 | 以目标 VM 源码清单为准；含反射对象（`RecordComponent`、`Method` 等）由 VM 填充的字段 |
| 类初始化触发 | 指令、反射（`forName`、`Method.invoke`）、显式（`ensureInitialized` 三处）、方法句柄首次调用 |
| 编译器约定 | lambda、字符串拼接、record、enum、switch、断言、内部类、桥方法、**try-with-resources（`close` + `addSuppressed`）**、装箱 |
| 编译单元 | 同一 `SourceFile` 的类一并保留（输出按编译单元组织时） |
| condy | `ConstantBootstraps` 各引导方法分类；未知引导方法保守 |
| 动态特性 | 反射（按名 / 结构反射 / **结果回流**）、动态类定义、方法句柄、代理、注解、服务加载、资源、序列化、native 回调与库加载（`JNI_OnLoad` / `RegisterNatives`）、FFM upcall |
| 不经构造器的实例化 | `clone`、`Unsafe`、反序列化、native |
| JDK 运行时代码生成 | JLI 运行时保留或静态 desugar |
| 新版本特性 | 虚拟线程、FFM、sealed、record 模式 |
| 世界边界 | 开放世界、模块导出、`opens`、外部子类 / 对象 / 异常 |
| 结构补全 | 超类型、槽位、构造器链、嵌套、桩化、字段布局、编译单元 |
| 精度设计 | 值来源标签、选择性上下文敏感（调用点 / 对象 / 容器）、每站点目标数仪表、溢出策略 |
| 下游契约 | 分层结果 → 生成形态映射（Referenced 级只声明不展开） |
| 重写后处理 | `StackMapTable` 重算、常量池 / `BootstrapMethods` 清理、类属性修正、`module-info` 与 `META-INF/services` 同步、多版本 JAR 一致裁剪 |
| 配置 | 规则语言、失效检测 |
| 动态预言机（与 JVMS 第 5 章同级的一级条目） | 按层级对照、排除清单、违例归因、覆盖扩展 |
| 差分与过渡 | 与 jdeps / R8 / native-image 差分；替换旧实现的双算过渡 |
| 施工顺序 | 里程碑 ①（正确性闭环）→ 里程碑 ②（精度升级） |
| 精度补充 | 异常流分析、类初始化策略与堆快照、替换机制、假设规则、模块层 |
| 工业级属性 | 规则目录与一致性套件、JDK 版本矩阵、配置互操作与追踪代理、扩展 API、输入健壮性、输出契约版本化、量化门槛、分级诊断 |

JVM 规范本身是有限的，逐章对照可以覆盖全部静态语义；但目标 VM 的上调、VM 访问字段和第三方库的反射约定会随版本变化，没有任何静态清单能永久保证完整。动态预言机是**必备环节**，不是可选项。

---

## 附录 B：参考实现与文献

**实现**

- SootUp / WALA / OPAL：CHA、RTA、VTA 的教科书级实现。
- Doop：Datalog 指针分析。
- GraalVM native-image：`PointsToAnalysis`、开放 / 封闭世界处理、invokedynamic 建模、类初始化模拟、reachability-metadata。
- R8 / ProGuard：keep rules 语义、indy desugar。
- HotSpot 源码：`vmClassMacros.hpp`、`vmSymbols.hpp`、`javaClasses.cpp`、`JavaCalls` 的调用点。

**文献**

- Bacon & Sweeney, *Fast Static Analysis of C++ Virtual Function Calls*, OOPSLA 1996（RTA）。
- Tip & Palsberg, *Scalable Propagation-Based Call Graph Construction Algorithms*, OOPSLA 2000（XTA / VTA）。
- Smaragdakis & Balatsouras, *Pointer Analysis*, Foundations and Trends in PL, 2015。
- Livshits et al., *In Defense of Soundiness: A Manifesto*, CACM 2015。
- Reif et al., *Judge: Identifying, Understanding, and Evaluating Sources of Unsoundness in Call Graphs*, ISSTA 2019（Java 调用图不可靠来源的系统清单；配套 CATS 测试套件）。
- Sui et al., *On the Recall of Static Call Graph Construction in Practice*, ICSE 2020。
- Christensen, Møller & Schwartzbach, *Precise Analysis of String Expressions*, SAS 2003（JSA，字符串分析）。
- Wimmer et al., *Initialize Once, Start Fast: Application Initialization at Build Time*, OOPSLA 2019（构建期初始化与堆快照）。

---

## 附录 C：另一种"闭包分析"——捕获变量分析

若目标是分析 lambda / 匿名类**捕获了哪些局部变量**、是否 effectively final、是否需要转为 `move` 或共享所有权：

- 在字节码层，捕获的变量就是 `invokedynamic` 调用点压栈的实参（对应 `LambdaMetafactory` 的 `invokedType` 参数），外加合成方法 `lambda$...` 的前 N 个参数。
- 匿名类的捕获变量对应构造器参数与 `val$xxx` 合成字段。
- 只需分析 indy 静态参数、`invokedType` 描述符与合成方法签名即可，比依赖闭包简单得多；若要判断"是否被修改"，再加一趟对合成方法体的过程内分析。
