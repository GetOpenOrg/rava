# fix-jdk25 交接（2026-10-11）

分支 `fix-jdk25`（基 c7cfb932），worktree `/data/workspace/rava-wt/fix-jdk25`。
第一轮按「卡住即停」交回后，协调者采纳候选方案 1（扫描器求值 cfg）并授权手写退路。第二轮结果：

- C（DeepCopy 闭包翻倍）由 d279b821 消除，闭包类集与方法集同集成头。
- B（TestStackWalkerFrames null_recv）独立于 cfg。根因是 doStackWalk 上调经 `&self` 辅助 fn 转手后，扫描器丢失 self 接收者。由 725249ef 修复。

## 提交

| 提交 | 内容 |
|---|---|
| efc32d6f | 栈遍历 native 按 JDK 分版本；接口 default 体超接口形参代入（组 2）；VM 支持类按 javac 报错剔除（组 3） |
| cf040742 / 1bdbc731 | 构造器调用点取发射签名（JDK 25 接口 default 体匿名类 ACC_MANDATED 外部实例形参）；仅类模式填帧 JArray 元素界 |
| 387216e7 | JDK 25 VirtualThread 新 native |
| e6390980 | 宏：擦除字段 setter 内层 `.into()` 钉声明类型（E0283）；栈遍历 limit 按 JDK 区分（25 为 bufferSize）；`Class.protectionDomain` 字段钩子；JDK 21 仅类模式帧写入改回 native 体内（未奏效，见下） |
| d869b0a9 | failure_patterns.toml 登记 |
| d279b821 | 闭包手写扫描器按参考 JDK 求值 `#[cfg]`（`handwritten/cfg.rs`，修 C） |
| 708cf86a | 栈遍历：两版本 callStackWalk 直接调 `__fill_in_frames`（授权退路第一步，未消 B） |
| c55c7b32 | 并入 origin/rust-closure-analyzer（d3c7fa5d） |
| 36df97e6 | cfg 单测夹具修正（保留字段改 u8） |
| 725249ef | 栈遍历：doStackWalk 上调留在各版本 native 体内，锚点登记抽为无 self 辅助（修 B） |

宏改动在 `runtime/rava_macros_core/src/block/rewrite.rs`（`restore_erased_accessors`，非 K2 文件，单测 `erased_setter_pins_inner_into`）。

## 三组原问题（JDK 25 java_base_decl 编译）

1. 栈遍历 native 签名：手写按 JDK 21 描述符写死 → `cfg(jdk_ge_25)` 分版本（efc32d6f）。JDK 25 抽查暴露的后续问题：E0283（宏，e6390980）、「not enough space in buffers」（JDK 25 传 bufferSize，e6390980）。
2. node_of_{double,int,long} `forEach_obj` 形参：`sig/params.rs` 对 this 经超接口声明者按本接口视角代入（efc32d6f）。
3. class_loaders_* 与 BuiltinClassLoader 冲突：JDK 22 起删 SerializationConstructorAccessorImpl，`SerializationConstructorAccessorDyn.java` 不可编译导致整批 VM 支持类缺席 → `resolve/image.rs` 按 javac 报错剔除（efc32d6f；JDK 21 上为空操作）。

## 第一轮抽查（dev，ref d869b0a9）

- JDK 25 `fixjdk25-25c-dev`：HelloWorld、AbstractClassTest、TestProtectionDomainFaces PASS；TestStackWalkerFrames / TestStackWalkerLines FAIL（stub `StackFrameInfo.declaringClass`，见下 A）。
  早先 `fixjdk25-25b-dev`（387216e7）：HelloWorld、Kosaraju、GZIPTest、MethodRefDemo、StreamDemo、CsvToTsv、TestLongChainRelease、AdditionChainExponentiation、CatalanNumbers、CompactNumberFormatExample、BaconCipher、DivisorSum PASS；DeepCopy / DataEncryptionStandard 转译 OOM（已知资源类）；TestVirtualThreadScale 已知性能项。
- JDK 21 `fixjdk25-21c-dev`：HelloWorld、AbstractClassTest、TestStackWalkerLines、TestProtectionDomainFaces PASS；**TestStackWalkerFrames FAIL（回归，B）**、**DeepCopy FAIL（回归，C）**。

## 问题与根因

### A. JDK 25 StackFrameInfo 反射元素流（闭包缺口，非回归）

JDK 25 `StackFrameBuffer` 以 `type.getDeclaredConstructor(StackWalker).newInstance` 反射分配帧元素。StackFrameInfo 经 [reflect] 入闭包，但 `FrameBuffer.at → ClassFrameInfo.declaringClass` 的分派未解析到 StackFrameInfo，导致 `StackFrameInfo.declaringClass` 生成为存根。这是闭包反射建模（G1 范围），已登记为 `jdk25-stackframeinfo-reflective-element-flow`。

### B. JDK 21 TestStackWalkerFrames null_recv（回归，已修 725249ef）

现象：`StackStreamFactory.isMethodHandleFrame` 中 `Class.getName` 的接收者被闭包判为恒 null。即 CallerClassFinder 仅类模式帧缓冲的 Class 元素没有写入来源。

d279b821 之后 B 仍在，说明与 cfg 无关。708cf86a 让两版本 callStackWalk 直接调 `__fill_in_frames`，也没有消除。随后按文件回退做抽查二分（JDK 21 TestStackWalkerFrames）：

| 诊断 ref | 内容 | 结果 |
|---|---|---|
| d3c7fa5d | 集成头 | PASS |
| b1 8f1532cd | 回退栈遍历与 STE 手写 | PASS |
| b2 e74cd901 | 回退生成器、宏、attrs、vm_intrinsics 改动 | FAIL |
| b3 f4edd1da | 恢复 let 绑定写法 | FAIL |
| b4 c377b01c | 只回退 STE 手写 | FAIL |
| b5 3cefc854 | doStackWalk 上调留在 native 体内 | PASS（TestStackWalkerLines 也 PASS） |

根因：本分支把 `self.doStackWalk(..)` 上调移进了 `&self` 辅助 fn `__walk_batches`。扫描器并入被调 fn 体时（`handwritten` 的 absorb_body / close_transitive），把其中调用记为 `on_self: false`，上调的接收者因此与 native 的 `this` 断开。doStackWalk 之后的 CallerClassFinder 帧流不再连到 native 写入的 Class 元素。

修法（725249ef）：两版本 callStackWalk 体内直接 `__fill_in_frames` → `__register_anchor`（无 self，只做 end_index 校验与锚点登记）→ `self.doStackWalk(..)` → 移除锚点。

手写规则：对 this 的 Java 上调必须写在 native 体内，辅助 fn 只做无 self 的准备工作。已登记为 `hw-upcall-self-lost-in-helper`。扫描器对 `&self` 辅助内 `self.X()` 的 on_self 传递属于闭包分析器（engine / handwritten）改进项，留给后续，本分支不动。

### C. JDK 21 DeepCopy 闭包翻倍（回归，已修 d279b821）

本机 `--stop-after emit` 二分（同一 rava 二进制，只换 runtime 文件）：

| runtime | JDK 类 |
|---|---|
| 基线 c7cfb932 | 3076 |
| 基线 + 本分支栈遍历与 STE 文件 | 6479 |
| 基线 + 仅本分支栈遍历文件 | 6479 |
| d279b821（扫描器求值 cfg） | 3029 |

扩张由 `stack_stream_factory_abstract_stack_walker_impl.rs` 单独引起，经 `MessageDigest.getInstance → GetInstance → SunProvider$ProviderService.newInstance` 纳入 jgss 等 20+ 模块。

根因：闭包手写扫描器不识别 `#[cfg]`。JDK 21 下也建模了 JDK 25 分支里对 `info` 的按名写 `name` / `type_` / `classOrMemberName`，值流因此放宽。

修法（d279b821）：
- `handwritten/cfg.rs` 按参考 JDK 三值求值 cfg（`jdk_ge_N` / `not` / `all` / `any`，其余不定、保留）。
- 剔除为假的 item、impl / trait 成员、语句、match 分支、字段与变体。
- driver build / closure / profile 三处传入 `ClassPath::release()`。
- 无类名特判，有单测。

闭包对照（remote_rava closure，JDK 21 DeepCopy）：c55c7b32 为 **3001 类 / 16425 方法**，与集成头 d3c7fa5d 类名集、方法名集逐项一致。

扩张时暴露的 `GSSNameSpi.equals_obj`（E0599）是独立的潜伏生成器缺陷，闭包回落后不再触发，仍作遗留项。

## 抽查（dev，ref 725249ef）

| JDK | spot | 结果 |
|---|---|---|
| 21 | fixjdk25-21g-dev | **8/8 PASS**：HelloWorld、DeepCopy、StockTrans、TestStackWalkerFrames、TestStackWalkerLines、TestProtectionDomainFaces、AbstractClassTest、StreamDemo |
| 25 | fixjdk25-25g-dev | **验收 7/7 PASS**：HelloWorld、AbstractClassTest、TestProtectionDomainFaces、StreamDemo、GZIPTest、Kosaraju、CompactNumberFormatExample；TestStackWalkerFrames / TestStackWalkerLines FAIL（stub `StackFrameInfo.declaringClass`，问题 A，不阻塞） |

单测（dev 作业，ref 36df97e6；725249ef 只改 runtime 手写，不影响生成器单测）：全绿。fj25g-utA closure_cli 11/11（2068 s）；fj25g-utB 其余 workspace、driver 各 CLI 与 rava_macros_core rc=0；fj25g-utS 两条确定性单测 2/2（3688 s）。上一轮 fj25f-utA / utS 是作业超时（缺省 3600 s），不是失败；本轮把 `--job-timeout` 放宽到 10800 后通过。

## 遗留

- A：JDK 25 StackFrameInfo 反射元素流（`jdk25-stackframeinfo-reflective-element-flow`，G1 / 反射建模范围），两个 StackWalker 测试在 JDK 25 上仍为存根失败。
- JDK 25 DeepCopy / DataEncryptionStandard 转译 OOM（`transpile-oom-14g`，资源类）。
- `GSSNameSpi.equals_obj`：接口重声明 Object 方法且有同名重载时，调用点命名与接口块发射口径不一致（潜伏生成器缺陷）。
- 扫描器 `&self` 辅助内 `self.X()` 上调丢 on_self（见 B）：手写侧已规避，分析器侧未改。

## 下一步

1. 合批：fix-jdk25（725249ef 及文档提交）并入集成分支。改动在 `handwritten/cfg.rs` 与 driver 三处传参，不涉及 engine / hub / reflect.rs。
2. 问题 A：闭包反射建模，让 `FrameBuffer.at → ClassFrameInfo.declaringClass` 的分派看到经反射构造的 StackFrameInfo 元素。修好后复测 JDK 25 TestStackWalkerFrames / TestStackWalkerLines。
3. 分析器侧：absorb_body 对 `&self` 辅助 fn 内的 `self.X()` 保留 on_self（接收者回连到调用方的 self），之后手写不再受该规则约束。
4. `GSSNameSpi.equals_obj`：统一接口重声明 Object 方法的调用点命名与接口块发射口径。
5. JDK 25 全量复跑（合批后），按「其余失败分类」逐项处理。

## 其余失败分类（JDK 25 全量 c7cfb932）

- 本分支已修：栈遍历编译错误（396）、classfile compound_element（25）、Method_Direct 存根（6）、PhantomReference.clear0（4）、VirtualThread 新 native（2）、TestProtectionDomainFaces。
- JDK 版本输出差异（期望输出按 JDK 21 录制）：TestObjectWaitFrames、TestParseDoubleEdge、TestNativeFrameTrace。
- 闭包 fold / dead_pcs：TestCollectionsFactories、TestListOf。
- 指令缺口（condy switch 标签）：TestPatternSlotReuse。
- 运行超时：TestArraysSetAll。
- 资源类：DeepCopy / DataEncryptionStandard JDK 25 转译 OOM（并入 `transpile-oom-14g`）。
- 已知性能：TestVirtualThreadScale（两 JDK 均失败，`debug-profile-too-slow`）。
