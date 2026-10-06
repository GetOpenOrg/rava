#!/usr/bin/env python3
"""闭包 JSON 全键对照（去掉 via 与 summary，列表排序）：顺序无关性验收用。

用法：cj_cmp.py <基准.json[.gz]> <其他.json[.gz]>...
输出每个文件相对基准的差异键与示例；全部一致退出 0，否则 1。
类 / 方法（含 kind 与 fns 等全部字段）之外的键（dispatch / reflect / folds / clinit …）同样逐项比较。
"""
import gzip
import json
import sys


def load(p):
    op = gzip.open if p.endswith(".gz") else open
    with op(p, "rt") as f:
        return json.load(f)


def norm(v):
    """去 via，列表按规范 JSON 文本排序"""
    if isinstance(v, dict):
        return {k: norm(x) for k, x in v.items() if k != "via"}
    if isinstance(v, list):
        xs = [norm(x) for x in v]
        return sorted(xs, key=lambda x: json.dumps(x, sort_keys=True, ensure_ascii=False))
    return v


def flat(d, pre=""):
    """顶层（reflect / seeds 等字典再下一层）键 → 规范元素集合"""
    out = {}
    for k, v in d.items():
        if k == "summary":
            continue
        key = pre + k
        if isinstance(v, dict) and not pre:
            out.update(flat(v, key + "."))
        elif isinstance(v, list):
            out[key] = {json.dumps(x, sort_keys=True, ensure_ascii=False) for x in norm(v)}
        else:
            out[key] = {json.dumps(norm(v), sort_keys=True, ensure_ascii=False)}
    return out


def counts(d):
    return f"类 {len(d['classes'])} / 方法 {len(d['methods'])} / 反射成员 {len(d['reflect']['members'])}"


def main():
    base_p, others = sys.argv[1], sys.argv[2:]
    bd = load(base_p)
    base = flat(bd)
    print(f"{base_p}: {counts(bd)}")
    ok = True
    for p in others:
        od = load(p)
        cur = flat(od)
        diffs = []
        for k in sorted(set(base) | set(cur)):
            a, b = base.get(k, set()), cur.get(k, set())
            if a != b:
                diffs.append((k, sorted(a - b), sorted(b - a)))
        print(f"{p}: {counts(od)} {'一致' if not diffs else '不一致'}")
        for k, a, b in diffs:
            ok = False
            print(f"  {k}: 仅基准 {len(a)}，仅本例 {len(b)}")
            for x in a[:5]:
                print(f"    - {x[:200]}")
            for x in b[:5]:
                print(f"    + {x[:200]}")
    sys.exit(0 if ok else 1)


if __name__ == "__main__":
    main()
