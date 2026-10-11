# fix-jdk25 交接（2026-10-11）

分支 `fix-jdk25`（基 c7cfb932），worktree `/data/workspace/rava-wt/fix-jdk25`。按「卡住即停」（同一问题两轮未修好）停止：
JDK 21 两处回归（TestStackWalkerFrames、DeepCopy）根因已定位到本分支栈遍历手写文件与闭包手写扫描器的交互，修法需要动闭包分析器（`generator/crates/closure/src/handwritten/`），交协调者定夺。

## 提交

| 提交 | 内容 |
|---|---|
| efc32d6f | 栈遍历 native 按 JDK 分版本；接口 default 体超接口形参代入（组 2）；VM 支持类按 javac 报错剔除（组 3） |
| cf040742 / 1bdbc731 | 构造器调用点取发射签名（JDK 25 接口 default 体匿名类 ACC_MANDATED 外部实例形参）；仅类模式填帧 JArray 元素界 |
| 387216e7 | JDK 25 VirtualThread 新 native |
| e6390980 | 宏：擦除字段 setter 内层 `.into()` 钉声明类型（E0283）；栈遍历 limit 按 JDK 区分（25 为 bufferSize）；`Class.protectionDomain` 字段钩子；JDK 21 仅类模式帧写入改回 native 体内（未奏效，见下） |
| d869b0a9 | failure_patterns.toml 登记 |

宏改动在 `runtime/rava_macros_core/src/block/rewrite.rs`（`restore_erased_accessors`，非 K2 文件，单测 `erased_setter_pins_inner_into`）。

## 三组原问题（JDK 25 java_base_decl 编译）

1. 栈遍历 native 签名：手写按 JDK 21 描述符写死 → `cfg(jdk_ge_25)` 分版本（efc32d6f）。JDK 25 抽查暴露的后续问题：E0283（宏，e6390980）、「not enough space in buffers」（JDK 25 传 bufferSize，e6390980）。
2. node_of_{double,int,long} `forEach_obj` 形参：`sig/params.rs` 对 this 经超接口声明者按本接口视角代入（efc32d6f）。
3. class_loaders_* 与 BuiltinClassLoader 冲突：JDK 22 起删 SerializationConstructorAccessorImpl，`SerializationConstructorAccessorDyn.java` 不可编译导致整批 VM 支持类缺席 → `resolve/image.rs` 按 javac 报错剔除（efc32d6f；JDK 21 上为空操作）。

## 抽查（dev，ref d869b0a9）

- JDK 25 `fixjdk25-25c-dev`：HelloWorld、AbstractClassTest、TestProtectionDomainFaces PASS；TestStackWalkerFrames / TestStackWalkerLines FAIL（stub `StackFrameInfo.declaringClass`，见下 A）。
  早先 `fixjdk25-25b-dev`（387216e7）：HelloWorld、Kosaraju、GZIPTest、MethodRefDemo、StreamDemo、CsvToTsv、TestLongChainRelease、AdditionChainExponentiation、CatalanNumbers、CompactNumberFormatExample、BaconCipher、DivisorSum PASS；DeepCopy / DataEncryptionStandard 转译 OOM（已知资源类）；TestVirtualThreadScale 已知性能项。
- JDK 21 `fixjdk25-21c-dev`：HelloWorld、AbstractClassTest、TestStackWalkerLines、TestProtectionDomainFaces PASS；**TestStackWalkerFrames FAIL（回归，B）**、**DeepCopy FAIL（回归，C）**。

## 未解问题与根因

### A. JDK 25 StackFrameInfo 反射元素流（闭包缺口，非回归）

JDK 25 `StackFrameBuffer` 以 `type.getDeclaredConstructor(StackWalker).newInstance` 反射分配帧元素。StackFrameInfo 经 [reflect] 入闭包，但 `FrameBuffer.at → ClassFrameInfo.declaringClass` 的分派未解析到 StackFrameInfo，导致 `StackFrameInfo.declaringClass` 生成为存根。这是闭包反射建模（G1 范围），已登记为 `jdk25-stackframeinfo-reflective-element-flow`。

### B. JDK 21 TestStackWalkerFrames null_recv（回归）

现象：`StackStreamFactory.isMethodHandleFrame` 中 `Class.getName` 的接收者被闭包判为恒 null。即 CallerClassFinder 仅类模式帧缓冲的 Class 元素没有写入来源。

- 与基线的结构差异：基线是 `callStackWalk → Self::__fill_in_frames(frames)`。本分支改为 cfg 分版本的 `callStackWalk → self.__call_stack_walk(.., limit, frames) → Self::__fill_in_frames`，同时加入了 JDK 25 自由函数（`_fill_class_only<T>(frames, ..)` 等）。
- 已试两轮：1bdbc731 补 JArray 元素界、e6390980 把 JDK 21 的 `frames.set` 移回 `__fill_in_frames` 体内。两轮均未消除。
- 手写扫描器 `close_transitive` 会传递 `array_access`，按代码看应能覆盖多一跳的情形，因此根因尚未钉死。候选有两个：一是扫描器对 `#[cfg]` 无感知（见 C），`_fill_class_only` 的 JDK 25 体也被并入；二是 `hw_mem::hw_writes` 的「手写体产出」值池在多一跳后断开。

### C. JDK 21 DeepCopy 闭包翻倍（回归）

本机 `--stop-after emit` 二分（同一 rava 二进制，只换 runtime 文件）：

| runtime | JDK 类 |
|---|---|
| 基线 c7cfb932 | 3076 |
| 基线 + 本分支栈遍历与 STE 文件 | 6479 |
| 基线 + 仅本分支栈遍历文件 | 6479 |

扩张由 `stack_stream_factory_abstract_stack_walker_impl.rs` 单独引起。闭包经 `MessageDigest.getInstance → GetInstance → SunProvider$ProviderService.newInstance` 纳入 jgss 等 20+ 模块。扩张后编出 E0599：`GSSNameSpi.equals_obj` 缺失。这是潜伏的生成器缺陷，接口重声明 Object 方法时调用点带重载后缀，但接口块不发射该声明；随闭包扩张暴露。

> **E0599 已修（fix-iface-obj-overload，2026-10-11，已并入 batch-1011i）**：5b6aacf0 让调用点对接口接收者上的根类公开方法取根类名，判定只在 `owner::is_iface_root_member` 一处，与规划器 root_keyed 同口径。d4cb7a55 修了同源的类侧缺陷：类内 equals 有重载时，ObjectVTable 桥接用 `#[equals_vtable = "Owner::equals_obj"]` 指向带后缀的槽名，此前退回身份比较。测例 TestIfaceObjectOverload，抽查 iobj-d4cb7a55 6/6，单测 iobj-ut-d4cb7a55 三作业全过。
> 剩余项：① 闭包翻倍本身（扫描器不识别 `#[cfg]`）仍归本节根因，未处理；② 未在 jgss 已纳入的大闭包下实测 GSSNameSpi 编译，需等扫描器修复后，或在 JDK 21 DeepCopy 复现分支上复验；③ 本报告文件此前只在 fix-jdk25 分支上，本段随 fix-iface-obj-overload 带入，与 fix-jdk25 合并时 add/add 冲突仅限本段。

根因：闭包手写扫描器（`closure/src/handwritten/scan.rs` 等）**不识别 `#[cfg]`**，generator/crates 内没有 cfg 处理。JDK 21 构建时，JDK 25 分支的手写体也被建模，其中包括 `_fill_frame_info`（分配 ResolvedMethodName）以及对 `info` 的按名写 `name` / `type_` / `classOrMemberName`。这些按名写的接收者不是按名读的来源（recv_src 为 None），扫描器「只知字段名」。按名写入同名字段的全体候选使值流大幅放宽。

## 候选方案（终态）

1. **手写扫描器按参考 JDK 求值 `cfg(jdk_ge_25)`**（推荐）。
   - 做法：`Handwritten::new` 增加参考 JDK 主版本，由 driver 的 build / closure / profile 三处传入。`load` / `pub_item_names` / `file_uses` 在 `scan_file` 前剔除 cfg 不成立的 item / impl item / stmt / expr，只认 `jdk_ge_25` 与 `not(jdk_ge_25)` 两种形式，其他 cfg 原样保留。
   - 理由：扫描器所见与编译器所编同源，同时消除 C 及 B 中 cfg 的贡献。
   - 风险：涉及 `closure/src/handwritten/`，需确认不与 G1 冲突。档案键已含 jdk_major，缓存无须另改。
2. 方案 1 之后若 B 仍在，用 `--trace-class java/lang/Class` 看 CallerClassFinder 帧数组的元素来源，查 `hw_mem::hw_writes` 对多一跳手写体的 produced 值池。手写侧的退路是把 `__call_stack_walk` 拆回两个 cfg native 各自直接调 `Self::__fill_in_frames`，但这属于回退本分支早先的结构，按规则先报。
3. `GSSNameSpi.equals_obj`：接口重声明 Object 方法（equals / hashCode / toString）且有同名重载时，调用点命名应与接口块发射口径一致。另登记为生成器缺陷，独立于本分支。

## 其余失败分类（JDK 25 全量 c7cfb932）

- 本分支已修：栈遍历编译错误（396）、classfile compound_element（25）、Method_Direct 存根（6）、PhantomReference.clear0（4）、VirtualThread 新 native（2）、TestProtectionDomainFaces。
- JDK 版本输出差异（期望输出按 JDK 21 录制）：TestObjectWaitFrames、TestParseDoubleEdge、TestNativeFrameTrace。
- 闭包 fold / dead_pcs：TestCollectionsFactories、TestListOf。
- 指令缺口（condy switch 标签）：TestPatternSlotReuse。
- 运行超时：TestArraysSetAll。
- 资源类：DeepCopy / DataEncryptionStandard JDK 25 转译 OOM（并入 `transpile-oom-14g`）。
- 已知性能：TestVirtualThreadScale（两 JDK 均失败，`debug-profile-too-slow`）。
