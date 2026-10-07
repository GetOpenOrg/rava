#!/usr/bin/env python3
"""闭包构成分析：把 closure.json 中的类 / 方法按机制拆分（规则在 scripts/closure_composition.toml）。

三种视角（每个用例各一张表）：
  own     自身包归属：类按最长前缀落入的块
  entry   入口机制：沿 via 首达链从根走到该类，第一个落入 mechanism 块的节点（含根与类自身）决定归属；
          链上无机制节点的类按自身包（base / user）计。各块记录「入口边」（前驱 → 首个机制节点）的类数
  through 经过计数：via 链上任意位置出现该机制的类数（首达树上「整块去掉」的上界；真实可减数以 --diff 反事实为准）
另：根种类（main / boot_image / boot_region / seed …）分布；--classload 读 JVM -Xlog:class+load 日志，只按自身包计（无链）；
--diff 对照基线与反事实切除的 closure.json，按基线入口机制统计消失的类；--face 统计各块在 API 面文件中的方法数。

用法：
  uv run python scripts/closure_composition.py --closure hello=build/ccomp/hello.json.gz ... \\
      [--classload s0boot-jvm=classload.log.gz] [--diff label=base.json.gz:cut.json.gz] \\
      [--face tests/api_surface/s0.txt] [--md out.md] [--json out.json] [--top 3]
口径：docs/reports/2026-10-07-closure-composition.md
"""
from __future__ import annotations

import argparse
import gzip
import json
import re
import sys
import tomllib
from collections import Counter, defaultdict
from pathlib import Path

RULES = Path(__file__).with_name("closure_composition.toml")


def load_json(p: str):
    with (gzip.open(p, "rt") if p.endswith(".gz") else open(p)) as f:
        return json.load(f)


def open_text(p: str):
    return gzip.open(p, "rt", errors="replace") if p.endswith(".gz") else open(p, errors="replace")


class Rules:
    def __init__(self, path: Path):
        d = tomllib.loads(path.read_text())
        self.cats = d["category"]
        self.title = {c["id"]: c["title"] for c in self.cats}
        self.kind = {c["id"]: c["kind"] for c in self.cats}
        self.order = [c["id"] for c in self.cats] + ["other"]
        self.title["other"] = "其他 JDK（未命中规则）"
        self.kind["other"] = "base"
        # (类型, 串, 块, 声明序)：最长匹配，同长取先声明
        self.entries = []
        for i, c in enumerate(self.cats):
            for p in c["prefixes"]:
                self.entries.append((p, c["id"], i))
        self.user = next((c["id"] for c in self.cats if c["kind"] == "user"), "user")
        self.memo: dict[str, str] = {}

    @staticmethod
    def _hit(pat: str, name: str) -> int:
        if pat.endswith("/"):
            return len(pat) if name.startswith(pat) else -1
        if pat.endswith("*"):
            p = pat[:-1]
            return len(p) if name.startswith(p) else -1
        return len(pat) if name == pat or name.startswith(pat + "$") else -1

    def of(self, name: str) -> str:
        r = self.memo.get(name)
        if r is None:
            best = (-1, 1 << 30, "other")
            for pat, cid, i in self.entries:
                n = self._hit(pat, name)
                if n > best[0] or (n == best[0] and n >= 0 and i < best[1]):
                    best = (n, i, cid)
            r = best[2] if best[0] >= 0 else "other"
            self.memo[name] = r
        return r

    def is_mech(self, cid: str) -> bool:
        return self.kind.get(cid) == "mechanism"


def owner_of(label: str) -> str:
    """方法标签 / 根串 → 所属类（内部名无 '.'，方法标签为 类.名:描述符）"""
    return label.split(".", 1)[0].split(":", 1)[0].split(" ", 1)[0]


class Closure:
    """closure.json 的 via 首达链：节点 ('C', 类) / ('M', 方法) / ('R', 根串)"""

    def __init__(self, d: dict, rules: Rules):
        self.rules = rules
        self.classes = {c["name"]: c for c in d["classes"]}
        self.methods = {m["id"]: m for m in d["methods"]}
        self.summary = d.get("summary", {})
        self._first: dict = {}
        self._near: dict = {}
        self._thru: dict = {}
        self._root: dict = {}
        self._depth: dict = {}
        self._rep: dict = {}
        self.by_owner: dict[str, list[str]] = defaultdict(list)
        for mid in self.methods:
            self.by_owner[owner_of(mid)].append(mid)

    def cat_of_class(self, name: str) -> str:
        c = self.classes.get(name)
        if c is not None and c.get("domain") in ("user", "lib"):
            return self.rules.user
        return self.rules.of(name)

    def node_cat(self, node) -> str:
        return self.cat_of_class(node[1] if node[0] == "C" else owner_of(node[1]))

    def raw_parent(self, node):
        if node[0] == "R":
            return None, None
        rec = self.classes.get(node[1]) if node[0] == "C" else self.methods.get(node[1])
        via = rec.get("via") if rec else None
        if not via:
            return None, None
        f = via["from"]
        if "root" in f:
            return ("R", f["root"]), via["kind"]
        if "method" in f:
            return ("M", f["method"]), via["kind"]
        return ("C", f["class"]), via["kind"]

    def parent(self, node):
        """链上的上一节点。落到类节点时改取该类的代表方法：类节点的 via 多为类型层边（超类型 / 字段 / 成员），
        方法经 clinit 边挂在类上时真正的触发者是该类首个被用到的代码（closure.json 不含 clinit 触发 via，取近似）"""
        p, k = self.raw_parent(node)
        if p is not None and p[0] == "C":
            p = self.rep(p[1], skip_clinit=True)
        return p, k

    def _raw_depth(self, node) -> int:
        if node in self._depth:
            return self._depth[node]
        seq, seen, cur = [], set(), node
        while cur is not None and cur not in seen and cur not in self._depth:
            seen.add(cur)
            seq.append(cur)
            cur, _ = self.raw_parent(cur)
        base = self._depth.get(cur, 0) if cur is not None else 0
        for i, n in enumerate(reversed(seq)):
            self._depth[n] = base + i + 1
        return self._depth[node]

    def _raw_owners(self, node) -> set:
        """原始 via 链（不做类节点替换）上出现的全部所属类"""
        out, seen, cur = set(), set(), node
        while cur is not None and cur not in seen:
            seen.add(cur)
            out.add(cur[1] if cur[0] == "C" else owner_of(cur[1]))
            cur, _ = self.raw_parent(cur)
        return out

    def rep(self, cname: str, skip_clinit: bool = False):
        """类的代表节点：类有方法入闭包时取首达链最短（原始 via 深度）的方法节点（代码层的入链原因），否则取类节点。
        skip_clinit（链中途的类节点替换）：只取原始链不经过该类自身的方法——否则 <clinit> 内 new 自身等形成回环"""
        key = (cname, skip_clinit)
        if key in self._rep:
            return self._rep[key]
        ms = self.by_owner.get(cname, ())
        if skip_clinit:
            ms = [m for m in ms if cname not in (self._raw_owners(self.raw_parent(("M", m))[0]) if self.raw_parent(("M", m))[0] else set())]
        r = min((("M", m) for m in ms), key=lambda n: (self._raw_depth(n), n[1])) if ms else ("C", cname)
        self._rep[key] = r
        return r

    def _chain(self, node):
        """根 → node 的节点序列（环保护）"""
        seq, seen, cur = [], set(), node
        while cur is not None and cur not in seen:
            seen.add(cur)
            seq.append(cur)
            cur, _ = self.parent(cur)
        seq.reverse()
        return seq

    def first(self, node):
        """(入口机制, 前驱节点, 首个机制节点)；链上无机制节点时为 (None, None, None)"""
        if node in self._first:
            return self._first[node]
        seq = self._chain(node)
        # 自根向下逐点填 memo，后续查询直接命中
        prev_res, prev = (None, None, None), None
        for n in seq:
            if n in self._first:
                prev_res = self._first[n]
            else:
                if prev_res[0] is None and self.rules.is_mech(self.node_cat(n)):
                    prev_res = (self.node_cat(n), prev, n)
                self._first[n] = prev_res
            prev = n
        return self._first[node]

    def nearest(self, node):
        if node in self._near:
            return self._near[node]
        seq = self._chain(node)
        cur = None
        for n in seq:
            if n in self._near:
                cur = self._near[n]
                continue
            c = self.node_cat(n)
            if self.rules.is_mech(c):
                cur = c
            self._near[n] = cur
        return self._near[node]

    def through(self, node):
        if node in self._thru:
            return self._thru[node]
        seq = self._chain(node)
        acc = frozenset()
        for n in seq:
            if n in self._thru:
                acc = self._thru[n]
                continue
            c = self.node_cat(n)
            if self.rules.is_mech(c):
                acc = acc | {c}
            self._thru[n] = acc
        return self._thru[node]

    def root_kind(self, node):
        if node in self._root:
            return self._root[node]
        seq = self._chain(node)
        r = None
        if seq and seq[0][0] == "R" and len(seq) > 1:
            _, r = self.parent(seq[1])
        elif seq:
            _, r = self.parent(seq[0])
        r = r or "断链（手写伪方法 / 无 via）"
        for n in seq:
            self._root[n] = r
        return r


def label(node) -> str:
    if node is None:
        return "—"
    return {"C": "类 ", "M": "", "R": "根 "}[node[0]] + node[1]


def analyze_closure(name: str, d: dict, rules: Rules, top: int) -> dict:
    cl = Closure(d, rules)
    own_c, own_m = Counter(), Counter()
    ent_c, ent_m, near_c, thru_c, roots = Counter(), Counter(), Counter(), Counter(), Counter()
    edges: dict[str, Counter] = defaultdict(Counter)
    matrix: dict[str, Counter] = defaultdict(Counter)
    for cname in cl.classes:
        node = cl.rep(cname)
        own = cl.cat_of_class(cname)
        own_c[own] += 1
        mech, pred, first_node = cl.first(node)
        ent = mech or own
        ent_c[ent] += 1
        matrix[ent][own] += 1
        if mech:
            edges[mech][f"{label(pred)} → {label(first_node)}"] += 1
        near_c[cl.nearest(node) or own] += 1
        for m in cl.through(node):
            thru_c[m] += 1
        roots[cl.root_kind(node)] += 1
    for mid in cl.methods:
        node = ("M", mid)
        own = cl.cat_of_class(owner_of(mid))
        own_m[own] += 1
        mech, _, _ = cl.first(node)
        ent_m[mech or own] += 1
    return {
        "name": name,
        "classes": len(cl.classes),
        "methods": len(cl.methods),
        "own_classes": dict(own_c),
        "own_methods": dict(own_m),
        "entry_classes": dict(ent_c),
        "entry_methods": dict(ent_m),
        "nearest_classes": dict(near_c),
        "through_classes": dict(thru_c),
        "root_kinds": dict(roots),
        "entry_edges": {k: v.most_common(top) for k, v in edges.items()},
        "entry_by_own": {k: dict(v) for k, v in matrix.items()},
        "_closure": cl,
    }


CL_RE = re.compile(r"\[class,load\]\s+(\S+)\s+source:\s*(.*)$")


def analyze_classload(name: str, path: str, rules: Rules) -> dict:
    own = Counter()
    hidden = Counter()
    total = 0
    with open_text(path) as f:
        for line in f:
            m = CL_RE.search(line)
            if not m:
                continue
            cname, src = m.group(1), m.group(2).strip()
            jdk = src.startswith("jrt:/") or src.startswith("shared objects file")
            internal = cname.replace(".", "/")
            if "/0x" in internal:
                # 运行期生成的隐藏类（lambda / LambdaForm / 代理）：按生成者包计入「运行期生成」
                base = internal.split("/0x", 1)[0]
                hidden[rules.of(base) if not src.startswith("file:") else rules.user] += 1
                continue
            if src.startswith("__"):
                hidden["proxy"] += 1
                continue
            if not jdk:
                continue
            total += 1
            own[rules.of(internal)] += 1
    return {"name": name, "classes": total, "own_classes": dict(own), "hidden": dict(hidden)}


def analyze_diff(label_: str, base: dict, cut: dict, rules: Rules) -> dict:
    cl = Closure(base, rules)
    after = {c["name"] for c in cut["classes"]}
    gone = [n for n in cl.classes if n not in after]
    added = [c["name"] for c in cut["classes"] if c["name"] not in cl.classes]
    by_ent, by_own = Counter(), Counter()
    for n in gone:
        mech, _, _ = cl.first(cl.rep(n))
        by_ent[mech or cl.cat_of_class(n)] += 1
        by_own[cl.cat_of_class(n)] += 1
    return {
        "label": label_,
        "base_classes": len(cl.classes),
        "cut_classes": len(after),
        "base_methods": len(base["methods"]),
        "cut_methods": len(cut["methods"]),
        "removed": len(gone),
        "added": len(added),
        "removed_by_entry": dict(by_ent),
        "removed_by_own": dict(by_own),
    }


def face_counts(path: str, rules: Rules) -> dict:
    meth, cls = Counter(), defaultdict(set)
    for line in Path(path).read_text().splitlines():
        line = line.strip()
        if not line or line.startswith("#"):
            continue
        o = owner_of(line)
        c = rules.of(o)
        meth[c] += 1
        cls[c].add(o)
    return {c: {"methods": meth[c], "classes": len(cls[c])} for c in meth}


def pct(n: int, t: int) -> str:
    return f"{100.0 * n / t:.1f}%" if t else "—"


def render(results, loads, diffs, face, rules: Rules) -> str:
    out = ["# 闭包构成（按机制）\n", f"规则：`{RULES.name}`（最长前缀匹配）；视角说明见脚本文档串。\n"]
    if results:
        out.append("\n## 总览：入口机制归属（类数 / 占比）\n")
        hdr = "| 块 | 种类 | " + " | ".join(r["name"] for r in results) + (" | S0 面方法 / 类 |" if face else " |")
        out.append(hdr)
        out.append("|" + "---|" * (2 + len(results) + (1 if face else 0)))
        for cid in rules.order:
            vals = [r["entry_classes"].get(cid, 0) for r in results]
            f = face.get(cid) if face else None
            if not any(vals) and not f:
                continue
            cells = [f"{v} ({pct(v, r['classes'])})" if v else "0" for v, r in zip(vals, results)]
            row = f"| {rules.title[cid]} | {rules.kind[cid]} | " + " | ".join(cells)
            if face:
                row += f" | {f['methods']} / {f['classes']}" if f else " | 0"
            out.append(row + " |")
        out.append("| **合计** | | " + " | ".join(f"{r['classes']} 类 / {r['methods']} 方法" for r in results) + (" | |" if face else " |"))
    for r in results:
        t = r["classes"]
        out.append(f"\n## {r['name']}：{t} 类 / {r['methods']} 方法\n")
        out.append("| 块 | 自身包 类 | 自身包 方法 | 入口归属 类 | 入口归属 方法 | 最近机制 类 | 经过 类 |")
        out.append("|---|---:|---:|---:|---:|---:|---:|")
        for cid in rules.order:
            row = [r["own_classes"].get(cid, 0), r["own_methods"].get(cid, 0), r["entry_classes"].get(cid, 0),
                   r["entry_methods"].get(cid, 0), r["nearest_classes"].get(cid, 0), r["through_classes"].get(cid, 0)]
            if not any(row):
                continue
            out.append(f"| {rules.title[cid]} | {row[0]} ({pct(row[0], t)}) | {row[1]} | {row[2]} ({pct(row[2], t)}) | {row[3]} | {row[4]} | {row[5]} |")
        out.append("\n根种类：" + "、".join(f"{k} {v}" for k, v in sorted(r["root_kinds"].items(), key=lambda x: -x[1])))
        out.append("\n入口边（前驱 → 首个机制节点，类数）：\n")
        for cid in rules.order:
            es = r["entry_edges"].get(cid)
            if not es:
                continue
            out.append(f"- {rules.title[cid]}：" + "；".join(f"`{e}` {n}" for e, n in es))
    for l in loads:
        t = l["classes"]
        out.append(f"\n## {l['name']}（JVM 实载 JDK 类，仅自身包归属，无入链信息）：{t} 类\n")
        out.append("| 块 | 类 | 占比 |\n|---|---:|---:|")
        for cid in rules.order:
            v = l["own_classes"].get(cid, 0)
            if v:
                out.append(f"| {rules.title[cid]} | {v} | {pct(v, t)} |")
        if l["hidden"]:
            out.append("\n运行期生成的隐藏类 / 代理（按生成者包）：" + "、".join(
                f"{rules.title.get(k, k)} {v}" for k, v in sorted(l["hidden"].items(), key=lambda x: -x[1])))
    if diffs:
        out.append("\n## 反事实切除（不健全，只作归因）\n")
        out.append("| 切除 | 基线类 | 切后类 | 消失 | 新增 | 方法 基线→切后 | 消失类按基线入口机制 |")
        out.append("|---|---:|---:|---:|---:|---|---|")
        for x in diffs:
            top = "、".join(f"{rules.title.get(k, k)} {v}" for k, v in sorted(x["removed_by_entry"].items(), key=lambda y: -y[1])[:6])
            out.append(f"| {x['label']} | {x['base_classes']} | {x['cut_classes']} | {x['removed']} | {x['added']} | "
                       f"{x['base_methods']}→{x['cut_methods']} | {top} |")
    return "\n".join(out) + "\n"


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--closure", action="append", default=[], metavar="NAME=PATH")
    ap.add_argument("--classload", action="append", default=[], metavar="NAME=PATH")
    ap.add_argument("--diff", action="append", default=[], metavar="LABEL=BASE:CUT")
    ap.add_argument("--face", metavar="PATH")
    ap.add_argument("--rules", default=str(RULES))
    ap.add_argument("--top", type=int, default=3)
    ap.add_argument("--md")
    ap.add_argument("--json")
    ap.add_argument("--unmatched", action="store_true", help="列出归入 other 的类（调规则用）")
    a = ap.parse_args()
    rules = Rules(Path(a.rules))
    results, loads, diffs = [], [], []
    cache: dict[str, dict] = {}

    def get(p):
        if p not in cache:
            cache[p] = load_json(p)
        return cache[p]

    for spec in a.closure:
        n, p = spec.split("=", 1)
        results.append(analyze_closure(n, get(p), rules, a.top))
    for spec in a.classload:
        n, p = spec.split("=", 1)
        loads.append(analyze_classload(n, p, rules))
    for spec in a.diff:
        n, rest = spec.split("=", 1)
        b, c = rest.split(":", 1)
        diffs.append(analyze_diff(n, get(b), get(c), rules))
    face = face_counts(a.face, rules) if a.face else None
    if a.unmatched:
        for r in results:
            for c in sorted(r["_closure"].classes):
                if r["_closure"].cat_of_class(c) == "other":
                    print(f"{r['name']}\t{c}")
        return 0
    md = render(results, loads, diffs, face, rules)
    if a.md:
        Path(a.md).write_text(md)
    else:
        sys.stdout.write(md)
    if a.json:
        for r in results:
            r.pop("_closure", None)
        Path(a.json).write_text(json.dumps({"closures": results, "classload": loads, "diffs": diffs, "face": face},
                                           ensure_ascii=False, indent=1))
    return 0


if __name__ == "__main__":
    sys.exit(main())
