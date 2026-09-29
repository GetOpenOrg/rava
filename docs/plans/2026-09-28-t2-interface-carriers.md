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

## 三、推进方式

逐批：名单加入该族 → 修手写边界签名 / 生成器 → 定向编译（≤3 例，`scripts/run_bg.sh`）→ 提交。
全部批次落地后置 None、删除两个 txt，跑验收集对账。名单增量期间生成树变化按批记录。

## 四、状态

| 批次 | 状态 |
|---|---|
| 7a | ✅ `409966f` + 名单 8 个访问器接口（TestStreamBasic / TestArrayList PASS） |
| 7b | ✅ `5b67a8f`：Path 进名单；宏 erased_impl_call 与 inherited_gen 转发体对手写 `__impl_` 经 Into 适配（FileIOTest / TestFilesApi PASS） |
| 7c | ⬜ |
| 7d | ⬜ |
| 置 None + 删 txt | ⬜ |
