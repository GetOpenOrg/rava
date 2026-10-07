# 无 GC 内存模型：编译期逃逸分析 + 所有权推断

> 日期：2026-10-07（用户定，本文仅登记决定，详细设计在 C4 之后另行展开）
> 关系：[transitional-state-inventory](2026-09-26-transitional-state-inventory.md) FS-G1（Rc 环永不释放）/ FS-G4（Cleaner no-op）的终态口径；[product-vision](2026-09-18-product-vision.md)「无 GC」定位。

## 一、决定

1. **坚持无 GC**：不做追踪式 GC，也不做运行期环回收（试探删除等）。
2. 回收只靠两项编译期机制：
   - **逃逸分析 + 作用域 / 区域整组释放**：方法内不逃逸的临时对象（含成环的，如 stream 管线）在作用域或区域结束时整组释放。判据用例：PartitionInteger（C4 全量运行超时 900 s，非死循环，引用环泄漏）。
   - **所有权推断生成弱引用**：可证明安全的回指字段生成 `Weak`，从源头不成强环。
3. 两者都覆盖不到的逃逸环登记为已知限制，不以运行期机制兜底。
4. **引用类语义**（`WeakReference` / `SoftReference` 清空、`ReferenceQueue` 入队，手写边界第 ③ 类）随本机制设计，由对象释放（`Rc` drop）触发，不引入任何 GC。对外表述不写「GC 引用处理」，避免误读。

## 二、排期

C4 收官之后实施。入队路径进闭包后，`ReferenceQueue.poll` 当前的折叠随之撤销。
