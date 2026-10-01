#!/usr/bin/env python3
"""`-Z dump-mono-stats` 单态化实例归类（docs/plans/2026-10-01-rustc-memory-and-crate-split.md §六）。

用法：python3 scripts/mono_stats.py <mono_items.json> [--top N]
输出：各类别的实例数 / size_est（total_estimate）及占比，另列 size 前 N 的条目。
类别按名称启发式划分（先匹配者优先）：
  std/core   —— std / core / alloc / parking_lot 的泛型实例；
  macro      —— 宏 / runtime 的 `__` 样板方法（末段以 `__` 起始，`__clinit` 类初始化体除外）；
  gil        —— runtime gil / sync_model 泛型；
  vtable     —— ObjectVTable 缺省方法；
  object_ext —— object_ext 泛型辅助；
  gen-generic—— 其余多份实例的条目（泛型 Java 类方法等）；
  gen        —— 其余单份实例（生成的非泛型方法体、存根、From 等）。
"""
import json
import sys


def category(name: str, count: int) -> str:
    head = name.lstrip("<")
    if head.startswith(("std::", "core::", "alloc::", "parking_lot", "lock_api", "hashbrown")):
        return "std/core"
    last = name.rsplit("::", 1)[-1]
    if last.startswith("__") and last != "__clinit":
        return "macro"
    if "gil::" in name or "sync_model::" in name:
        return "gil"
    if "ObjectVTable" in name:
        return "vtable"
    if "object_ext::" in name:
        return "object_ext"
    return "gen-generic" if count > 1 else "gen"


def main() -> None:
    path = sys.argv[1]
    top = int(sys.argv[sys.argv.index("--top") + 1]) if "--top" in sys.argv else 15
    items = json.load(open(path))
    agg: dict[str, list[int]] = {}
    for it in items:
        c = category(it["name"], it["instantiation_count"])
        a = agg.setdefault(c, [0, 0, 0])
        a[0] += 1
        a[1] += it["instantiation_count"]
        a[2] += it["total_estimate"]
    n_inst = sum(a[1] for a in agg.values())
    n_size = sum(a[2] for a in agg.values())
    print(f"合计：条目 {len(items)}，实例 {n_inst}，size_est {n_size}")
    print("| 类别 | 条目 | 实例 | 实例占比 | size_est | size 占比 |")
    print("|---|---:|---:|---:|---:|---:|")
    for c, (e, i, s) in sorted(agg.items(), key=lambda kv: -kv[1][2]):
        print(f"| {c} | {e} | {i} | {100 * i / n_inst:.1f}% | {s} | {100 * s / n_size:.1f}% |")
    print(f"\nsize_est 前 {top}：")
    for it in sorted(items, key=lambda x: -x["total_estimate"])[:top]:
        print(f"  {it['total_estimate']:>8} {it['instantiation_count']:>6}×  {it['name'][:140]}")


if __name__ == "__main__":
    main()
