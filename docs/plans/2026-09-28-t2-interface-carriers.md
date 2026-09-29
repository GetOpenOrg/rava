# T-2：接口载体铺开到全部接口（删除 carrier_type_positions.txt）

> 关联：A-4 接口载体（擦除阶段 1：非泛型 `I__VTable` + 载体 struct + `ObjectVTable::__interface`）、
> FS-M2（过渡清单终态删除）、清单整合第 4 项（`d449f77` 决定）。T-1 ✅（`63b4eca`）后启动。

## 一、目标

- `codegen/jvm_type.CARRIER_TYPE_POSITIONS = None`：全部接口进入类型位置时发射擦除载体 `I<Object, ..>`
  （方法经 `__interface` itable 分派），不再回落 `Object`；
- 删除 `runtime/java_runtime/carrier_type_positions.txt`；`signature_erased_interfaces.txt` 的
  CharSequence 已在载体名单内，一并删除（sig_parse 不再特判）。

## 二、实验（2026-09-28，TestStreamBasic）

名单置 None 后 91 个编译错误，按接口族归类：

| 批次 | 接口族 | 错误 | 落点 |
|---|---|---|---|
| 7a | `jdk/internal/access` 的 `*Access`（JavaLangAccess 等）经 SharedSecrets 存取 | ~20 | 手写 `shared_secrets_impl.rs` / `java_lang_access_impl.rs` 仍以 Object 存取 |
| 7b | `java/nio/file/Path` | ~30 | 手写 `sun/nio/fs/*_provider_impl.rs` 以 Object 收发 Path |
| 7c | 流的基本类型特化族（`Int/Long/DoubleConsumer`、`Spliterator$OfPrimitive`、`Node`） | ~20 | 生成器：特化桥接 `forEachRemaining(Object)↔(LongConsumer)` 的名 / 型解析（批次 5 暂缓原因） |
| 7d | 零散：`Runnable`、`InvocationHandler`、`Enumeration`、`System$Logger`、`Temporal`、通道 | ~10 | 各手写边界（thread / proxy / class_loader / virtual_thread） |

## 二之二、7a/7b 之后的全量复测（2026-09-29，TestStreamBasic）

91 → 40：标记接口（Serializable）缺 upcast 9（已修，见标记接口 upcast 提交）；流的基本类型特化族
（Node / SpinedBuffer / Spliterator$OfPrimitive）约 14（7c）；手写边界零散接口约 13（7d：Enumeration、
Runnable、InvocationHandler、通道、JavaLangAccess.layers 的 Stream）；sun/nio/fs trait bound 4（待查）。

## 三、推进方式

逐批：名单加入该族 → 修手写边界签名 / 生成器 → 定向编译（≤3 例，`scripts/run_bg.sh`）→ 提交。
全部批次落地后置 None、删除两个 txt，跑验收集对账。名单增量期间生成树变化按批记录。

## 四、状态

| 批次 | 状态 |
|---|---|
| 7a | ✅ `409966f` + 名单 8 个访问器接口（TestStreamBasic / TestArrayList PASS） |
| 7b | ✅ `5b67a8f`：Path 进名单；宏 erased_impl_call 与 inherited_gen 转发体对手写 `__impl_` 经 Into 适配（FileIOTest / TestFilesApi PASS） |
| 7c | 🔄 `814bc1f`：invoke_sig「this 调用不代入载体」规则收窄到声明者为接口（桥方法 forEach_obj(LongConsumer) 调用侧回落 Object 的根因）；回归 TestStreamBasic / TestFieldEvalOrder PASS；其余特化族错误待名单置 None 后复测 |
| 7d | 🔄 兼容部分 ✅ `ec6a448`（接口字段写入 / 实参 / 返回经 Into，Default::default，LazyLoggers 泛型返回）；只能按终态书写的两处（JavaLangAccess.layers_classloader 返回 Stream、newByteChannel 的 FileAttribute 数组形参）随置 None 一并修改 |
| 标记接口 upcast | ✅ `25c0002`（TestStreamBasic / TestArrayList PASS） |
| 置 None + 删 txt | 🔄 全接口载体下 TestStreamBasic 编译错误 91 → 0（2026-09-29）：①桥方法——接收者类链首个声明是 synthetic 桥时，形参按被桥接的真实方法解析（`tryAdvance(Object)` → `tryAdvance_intconsumer(IntConsumer)`）；②接口签名返回裸类型变量（`T_SPLITR spliterator()`）、描述符为接口载体：经 Object 边界取回（bare Object 接收者接口分派与通用调用结果两处）；③手写终态签名：`layers_classloader` 返回 `Stream<Object>`，`newByteChannel` / `createDirectory` 形参 `JArray<FileAttribute<Object>>`。内存：同闭包（1773 类）rustc 峰值基线 13.83G 通过、全载体 13.93G 在 16G 机器被 OOM 杀——T-2 增量约 0.1G，瓶颈是闭包规模（N14）。`signature_erased_interfaces.txt`（CharSequence）另步删除 |
