# main 全量 e2e 失败分诊（2026-09-30）

## 0. 数据口径

- 跑批：分布式全量 `rava/distribute_tests.py --jdk 21`，服务器代码 **git 6099432c**，profile=debug，RUN_TIMEOUT 300 s，TRANSPILE_TIMEOUT 600 s
- 快照：`state_jdk21.json` 22:25，completed 885 / failed 33（全量约 1080，跑批仍在进行，后续失败未计入）
- 证据：`test_results/error_logs/`（`[FAIL]` 行、diff、`[run-classify]`、`[dyn-compare]`、run.log panic 回溯；无 stdout 与 Java 栈）
- 本地复核：worktree `triage-0930`（e1646e6d，含 closure-precision 系列）上的 `rava closure --why`，以及单例 `scripts/main.py --no-run`
- 21:34 之前 a04f03b1 批次中失败的 SubstitutionCipher、TestCommonPool、TestDoubleToStringSpec、IQPuzzle、TestAnnoReflect，在本次 `--reset-failed` 重跑中均已通过

## 1. 总表

| 用例 | 首错 | 归因 | 根因组 |
|---|---|---|---|
| Factorion | 运行超时 >300 s | 测试或环境（debug 重计算） | A |
| FWord | 运行超时 | 测试或环境 | A |
| FibonacciMatrixExponentiation | 运行超时 | 测试或环境 | A |
| FourIsTheNumberOfLetters | 运行超时（10^7 词） | 测试或环境 | A |
| FractionReduction | 运行超时 | 测试或环境 | A |
| LynchBell | 运行超时 | 测试或环境 | A |
| PartitionInteger | 运行超时 | 测试或环境 | A |
| PrimorialNumbers | 运行超时 | 测试或环境 | A |
| RailwayCircuit | 运行超时 | 测试或环境 | A |
| SelfNumbers | 运行超时 | 测试或环境 | A |
| StockTrans | 转译超时 >600 s（Serializable 大闭包） | 闭包分析（性能） | A′ |
| EasterRelatedHolidays | stub `MethodType.toMethodDescriptorString` | 闭包分析 | B |
| TestMessageDigestApi | 同上 | 闭包分析 | B |
| TestMethodHandleCombinators | 同上 | 闭包分析 | B |
| TestMethodHandleDirect | 同上 | 闭包分析 | B |
| TestNetworkInterface | 同上 | 闭包分析 | B |
| TestRecordComponents | 同上 | 闭包分析 | B |
| TestMethodRef | stub `Integer.intValue:()I` | 闭包分析 | C |
| TestMethodRefKinds | stub `Integer.valueOf:(I)` | 闭包分析 | C |
| TestOptional | stub `Integer.valueOf:(I)`（`map(String::length)`） | 闭包分析 | C |
| TestRecordHashCode | stub `Integer.equals:(Object)Z`（record `equals` 比较 Object 分量） | 闭包分析 | K |
| SequenceGenerator | CCE `ArrayList$SubList` → `ArrayList` | 生成器（method/vars） | D |
| RecordPatternTest | 嵌套泛型 record 模式的 instanceof 块被静态折叠掉，输出缺行 | 生成器（instr/sim + method/vars） | D |
| ReflectionGetSource | 栈帧文件名/行号为 Rust 源位置 | 生成器 + 手写层 | E |
| PrintDebugStatement | 同上 | 生成器 + 手写层 | E |
| TestDirectBuffer | stub `UnsafeConstants.UNALIGNED_ACCESS:Z` | 闭包分析（VM 常量清单） | I |
| TestCharsetNamedStreams | `stub: sun/nio/cs/StreamDecoder (charset UTF-16 未消费)` | 闭包分析（种子） | I |
| ChebyshevCoefficients | 末位 ulp 差 | 测试期望依赖 HotSpot 内建 | F |
| DateTest | 期望 `1970年1月1日 00:00:00`，实得 `Jan 1, 1970, 12:00:00 AM` | 环境（locale） | G |
| NoConnection | 运行超时 | 测试缺陷（HotSpot 上同样不终止） | H |
| ScannerTest | 未捕获 NPE | 待定位 | J |
| TestAnnoValues | CCE `Object` → `[Ljava.lang.annotation.Annotation;` | 待定位 | J |
| TestProcessBuilder | 未捕获 NPE | 待定位 | J |

## 2. 根因组说明与修复落点

### A 运行超时（10 例）与 A′ 转译超时（1 例）

- A：均为重计算型 Rosetta 用例（阶乘数位、10^7 级枚举、矩阵快速幂等），debug profile 加上装箱与 `Rc` 开销后超过 300 s。输出没有错误，只是跑不完。
- 落点：
  - 跑批侧：重计算用例用 release profile 跑。按用例放宽超时只是过渡做法，不采纳。
  - 生成器侧（终态）：减少热循环里的装箱与 `Rc` 克隆，属于性能专项，不在本次修复范围。
- A′ StockTrans：Serializable 相关闭包过大，转译超过 600 s。落点是闭包分析器性能计划（e1646e6d，终态 ≤ 10 s / ≤ 1 GB）。

### B 手写上行调用未入闭包（6 例）

- 现象：运行时命中 `MethodType.toMethodDescriptorString` 存根。回溯为 `MemberName$Factory.resolveOrFail` → `MethodHandleNatives.resolve` → 手写 `resolve_method`（`runtime/java_runtime/src/java/lang/invoke/method_handle_natives_impl.rs:180-183`）。
- 根因：这个调用点写在 `let Ok(mt) = ….try_cast::<MethodType>(…) else {…}` 里。6099432c 的手写扫描不认 let-else 绑定里 turbofish 形式的接收者类型，上行调用边因此丢失。
- 修复状态：已经修好，main 待重跑。修复提交 8078f2b1（按目标类型推断 turbofish，支持 let-else / if let / match 绑定）和 3c3c3129（try_cast 推断），均不在 6099432c 的祖先中，但当前 main f1379345 已包含。在 e1646e6d 上执行 `rava closure --why`，该方法为 bytecode，经 `MethodHandleNatives.resolve` 分派入链。
- 落点：无需新改动。用当前 main 重跑这 6 例即可验证。

### C 方法引用的装箱/拆箱适配边缺失（3 例）

- 现象：`Integer::valueOf` 或 `Integer.intValue` 存根被命中。场景是方法引用或 lambda 的实现方法签名为原始类型，而 SAM 签名为引用类型（`map(String::length)`、`TestMethodRef::doubleIt` 等）。
- 根因：LambdaMetafactory 在 SAM 与实现方法之间做装箱/拆箱适配，这些隐式调用由 VM 生成，没有出现在任何字节码里。在 e1646e6d 上复核，三例的 `Integer.intValue` 以及 TestOptional、TestMethodRefKinds 的 `Integer.valueOf` 仍不在闭包内。
- 落点：闭包分析器 `generator/crates/closure/src/engine/lambda.rs`。在 `invoke_lambda`/`lambda_step` 接入实现方法时，逐位比较 SAM 描述符（instantiatedMethodType）与实现方法描述符：原始类型对引用类型的位置加 `<Box>.valueOf` 边，引用类型对原始类型的位置加 `<Box>.<prim>Value` 边。返回值按同样规则处理。

### K record `ObjectMethods` 未派发分量的 equals/hashCode/toString（1 例）

- 现象：TestRecordHashCode 中 `record Named(String name, Object extra)` 的 `equals` 对 `extra`（Integer）做虚派发，命中 `Integer.equals` 存根。在 e1646e6d 上复核，该方法同样不在闭包内。
- 根因：`vm_intrinsics.toml [indy] native` 把 `ObjectMethods.bootstrap` 当作普通原生引导方法处理。分析器（`engine/lambda.rs` 的 `indy` 中 `kind` 分支）只沿静态实参里的 getter 句柄读字段，没有对各引用分量的值集派发 `Object.equals/hashCode/toString`。`Concat` 分支对 `toString` 有这种派发，这里缺少同样的处理。
- 落点：
  - 在 `[indy]` 中新增 `object_methods` 类别。
  - 分析器按调用点名（`equals` / `hashCode` / `toString`），对每个引用类型 getter 句柄读出的字段值集，派发对应的 `Object` 方法。做法与 `Concat` 分支一致。

### D 局部槽位定型错误（2 例）

- SequenceGenerator：`s = s.subList(…)` 与 `new ArrayList` 共用一个槽位。生成代码把槽位定型为 `ArrayList<Object>`（`let mut s = ArrayList::<Object>::new()?`），再对 `subList` 的结果做 `From<Object>` 转换，运行时抛出 CCE。
- RecordPatternTest：`genericInferenceTest` 中 slot 10 被 `t():Object`、Point、String、int 复用。槽位定型过窄后，instanceof 被静态折叠为 false（cfg-audit `instanceof_fold=1`），整个模式匹配块消失。
- 根因：没有 LVT 时，槽位取首次存入的类型，而不是所有存入类型的公共上界。instanceof 的静态否证又依赖了这个推断出来的类型，而不是声明类型。
- 落点（P5 切换后在 Rust 侧修）：
  - `generator/crates/method/src/vars/slot_type.rs`：槽位类型取所有存入点的最小公共上界。原始类型和引用类型复用同一槽位时拆成不同变量。
  - instanceof 折叠（Python 侧 `codegen/instr/sim/control.py` 的 `record_instanceof_fold`，以及对应的 Rust sim）：只依据字节码可证的精确类型做折叠，槽位推断类型不作为依据。

### E Java 栈帧信息缺失（2 例）

- 现象：`StackTraceElement` 的文件名和行号是 Rust 源的位置，期望的是 `Xxx.java:行号`。
- 落点：生成器按 LineNumberTable 为每个方法输出「Rust 位置 → (类, 方法, Java 行号)」映射表；手写层 `throwable_impl` 的栈帧捕获查这张表还原 Java 帧。属于 VM 驱动行为（栈遍历）的手写，符合准入第 ③ 类。

### I 清单/种子遗漏（2 例）

- TestDirectBuffer：`Bits.<clinit>` → `Unsafe.unalignedAccess` 读取 `UnsafeConstants.UNALIGNED_ACCESS`。这是 VM 注入的常量字段，`vm_intrinsics.toml` 没有声明，结果成了存根。落点：在 `vm_intrinsics.toml` 的 VM 常量里声明 `UnsafeConstants` 的全部字段（`ADDRESS_SIZE0` / `PAGE_SIZE` / `BIG_ENDIAN` / `UNALIGNED_ACCESS` / `DATA_CACHE_*`）。
- TestCharsetNamedStreams：UTF-16 charset 没有补种，`StreamDecoder` 走到存根。落点：`seeds.toml` 的 charset 补种覆盖 UTF-16 系列（UTF-16 / UTF-16BE / UTF-16LE），或者由分析器从 `Charset.forName` 的调用点常量推导。

### F 数学内建 ulp 差（1 例）

- ChebyshevCoefficients：rava 忠实翻译了 `StrictMath`/FdLibm，本机核对输出一致。HotSpot 的 `Math.cos` 内建与 StrictMath 相差不超过 1 ulp，期望文件是按 HotSpot 结果生成的。
- 落点：测试侧。期望输出不应依赖 VM 内建精度，可以把打印精度截断到有效位内。rava 不应为迎合 HotSpot 内建而偏离字节码语义。

### G 环境 locale（1 例）

- DateTest：期望文件按 zh 环境生成，服务器默认是 en。落点：跑批固定 `LANG`/`user.language`，期望文件与运行环境使用同一 locale；或者测试显式指定 `Locale`。

### H 测试缺陷（1 例）

- NoConnection：每轮都 `new Random(42)`，加上同一个置换，迭代成环，HotSpot 上也不终止。落点：修测试，或从全量中移除。

### J 待定位（3 例）

- ScannerTest：`new Scanner(System.in); sc.close()` 抛未捕获 NPE。ScannerInputTest 能通过，怀疑是 close 路径（`System.in` 的 close 手写/VM 流）。
- TestAnnoValues：CCE `Object` → `[Ljava.lang.annotation.Annotation;`，怀疑 `getParameterAnnotations` 构造的数组组件类型是 Object 而不是 Annotation（build.rs 反射/注解元数据表）。
- TestProcessBuilder：未捕获 NPE，没有 Java 栈；dyn miss 只有良性的 LambdaMetafactory。
- 建议：服务器 error_logs 附上转译产物的 stdout，以及 `JvmError` 的 Java 栈（`report_uncaught_in` 打出 cause 链）。这 3 例都需要在本地串行复现并运行后才能定位。

## 3. 优先级（按影响用例数）

| 序 | 组 | 用例数 | 落点 | 备注 |
|---|---|---|---|---|
| 1 | A / A′ | 10 + 1 | 跑批 profile；分析器性能计划 | 不是正确性缺陷，调整跑批参数即可消掉 10 例 |
| 2 | B | 6 | 已修（8078f2b1 / 3c3c3129，在当前 main 中） | 用当前 main 重跑验证 |
| 3 | C | 3 | 闭包分析器 `engine/lambda.rs` 装箱适配边 | 方法引用是常见形态，潜在影响面大 |
| 4 | J | 3 | 待定位 | 先补服务器日志 |
| 5 | D | 2 | Rust 生成器 `method/src/vars/slot_type.rs` + instanceof 折叠 | P5 切换后在 Rust 侧修；静默错误输出，风险高 |
| 6 | E | 2 | 生成器行号映射 + `throwable_impl` | |
| 7 | I | 2 | `vm_intrinsics.toml` / `seeds.toml` | 清单级改动，成本低 |
| 8 | K | 1 | `[indy] object_methods` + 分析器分派 | record 的 equals/hashCode/toString 普遍依赖，潜在影响面大 |
| 9 | F / G / H | 各 1 | 测试与环境 | |

C、K、D 三组的实际影响面比这里的用例数大：方法引用装箱、record 分量派发、槽位复用都是常见形态，这批只是首先暴露出来的用例。建议排在 A 的跑批参数调整之后优先修。
