#!/usr/bin/env python3
"""JDK 档案测量（docs/plans/2026-10-01-cross-test-compile-reuse.md §五）。

输入是逐测试的 closure.json（或 scripts/profile_closure.sh 产出的 <Test>.json.gz）。

子命令：
  stats <dir>...              并集规模：类 / 方法（按域、层级、种类）、单例分布、并集随测试数增长曲线
  merge <out.json> --user T <json>...
                              合成并集 closure.json，供 `rava emit` 生成近似档案树（测声明层峰值、单例
                              只编用户 crate 的耗时）。只保留用户 T 的用户类；JDK 部分取并集。
                              折叠按方法取参与测试中折叠点最少的一份（与该测试闭包自洽，可编译；
                              只用于体量测量，不代表档案的折叠语义）
  folds <dir>... --jdk-home H 折叠精度代价：逐例折叠点、并集（同一方法在全部到达测试中折叠一致才保留）、
                              开放世界（null_recv 的静态接收者类型可被用户扩展时不折叠）
"""
import argparse
import gzip
import json
import random
import re
import subprocess
import sys
from collections import Counter, defaultdict
from pathlib import Path

LEVELS = ["type", "layout", "init", "alloc", "code"]


def load(p: Path) -> dict:
    op = gzip.open if p.suffix == ".gz" else open
    with op(p, "rt", encoding="utf-8") as f:
        return json.load(f)


def inputs(paths) -> list[tuple[str, Path]]:
    out = []
    for p in map(Path, paths):
        if p.is_dir():
            for f in sorted(p.rglob("*.json.gz")) + sorted(p.rglob("closure.json")):
                n = f.name[:-8] if f.name.endswith(".json.gz") else f.parent.parent.name
                out.append((n, f))
        else:
            out.append((p.name.split(".")[0], p))
    seen, uniq = set(), []
    for n, f in out:
        if n not in seen:
            seen.add(n)
            uniq.append((n, f))
    return uniq


def owner(member_id: str) -> str:
    return member_id.split(".", 1)[0] if "." in member_id.split(":", 1)[0] else member_id


def jdk_sets(d: dict) -> tuple[set, set]:
    users = {c["name"] for c in d["classes"] if c["domain"] == "user"}
    cls = {c["name"] for c in d["classes"] if c["domain"] != "user"}
    ms = {m["id"] for m in d["methods"] if owner(m["id"]) not in users}
    return cls, ms


# ── stats ─────────────────────────────────────────────────────────────────────

def cmd_stats(a):
    tests = inputs(a.dirs)
    per = {}
    by_dom, by_lvl, by_kind = {}, {}, {}
    for n, f in tests:
        d = load(f)
        users = {c["name"] for c in d["classes"] if c["domain"] == "user"}
        cls, ms = jdk_sets(d)
        per[n] = (cls, ms)
        for c in d["classes"]:
            if c["domain"] == "user":
                continue
            by_dom[c["name"]] = c["domain"]
            lv = by_lvl.get(c["name"])
            if lv is None or LEVELS.index(c["level"]) > LEVELS.index(lv):
                by_lvl[c["name"]] = c["level"]
        for m in d["methods"]:
            if owner(m["id"]) not in users:
                by_kind.setdefault(m["id"], m["kind"])
    U_c = set().union(*(c for c, _ in per.values()))
    U_m = set().union(*(m for _, m in per.values()))
    cs = sorted(len(c) for c, _ in per.values())
    msz = sorted(len(m) for _, m in per.values())
    med = lambda xs: xs[len(xs) // 2]
    print(f"测试数 {len(per)}")
    print(f"并集 JDK 类 {len(U_c)}（按域 {dict(Counter(by_dom.values()))}；按最高层级 {dict(Counter(by_lvl.values()))}）")
    print(f"并集 JDK 方法 {len(U_m)}（按种类 {dict(Counter(by_kind.values()))}）")
    print(f"单例 JDK 类 中位 {med(cs)} / 最大 {cs[-1]}；方法 中位 {med(msz)} / 最大 {msz[-1]}")
    print(f"逐例相加 类次 {sum(cs)}（去重 {len(U_c)}，平均每类 {sum(cs) / len(U_c):.1f} 次）；方法次 {sum(msz)}")
    code = {c for c, l in by_lvl.items() if l == "code"}
    print(f"并集 code 级类 {len(code)}；单例 code 级类次 {sum(len(per[n][0] & code) for n in per)}")
    # 增长曲线（各 10 次随机抽样平均）
    names = list(per)
    rng = random.Random(20261003)
    pts = [k for k in (1, 10, 30, 60, 120, 240, 480, 960) if k < len(names)] + [len(names)]
    row_c, row_m = [], []
    for k in pts:
        sc = sm = 0
        reps = 1 if k == len(names) else 10
        for _ in range(reps):
            s = rng.sample(names, k)
            sc += len(set().union(*(per[x][0] for x in s)))
            sm += len(set().union(*(per[x][1] for x in s)))
        row_c.append(sc // reps)
        row_m.append(sm // reps)
    print("| 测试数 | " + " | ".join(map(str, pts)) + " |")
    print("|---|" + "---:|" * len(pts))
    print("| 并集类数 | " + " | ".join(map(str, row_c)) + " |")
    print("| 并集方法数 | " + " | ".join(map(str, row_m)) + " |")
    if a.top:
        # 贡献独有类最多的测试（只在该测试出现的类）
        cnt = Counter(c for cls, _ in per.values() for c in cls)
        own = sorted(((sum(1 for c in per[n][0] if cnt[c] == 1), n) for n in per), reverse=True)[: a.top]
        print("独有类最多：" + "，".join(f"{n} {k}" for k, n in own))


# ── merge ─────────────────────────────────────────────────────────────────────

def fold_weight(f: dict) -> int:
    return sum(len(v) for k, v in f.items() if isinstance(v, list))


def cmd_merge(a):
    tests = inputs(a.jsons)
    docs = [(n, load(f)) for n, f in tests]
    keep_user = None
    drop = set()
    for n, d in docs:
        users = {c["name"] for c in d["classes"] if c["domain"] == "user"}
        if n == a.user:
            keep_user = users
        else:
            drop |= users
    if keep_user is None:
        sys.exit(f"--user {a.user} 不在输入中")
    drop -= keep_user
    ok = lambda mid: owner(mid) not in drop

    classes, methods, folds, fold_part = {}, {}, {}, defaultdict(list)
    sets = defaultdict(set)
    reflect = {"members": {}, "fields": {}, "field_names": set(), "gaps": set(), "field_enum_gaps": []}
    seeds = {"data_bundles": set(), "annotation_enums": set(), "jca": {}, "reflect_names": defaultdict(set),
             "reflect_all": set(), "services": defaultdict(dict), "services_unknown": False}
    ci = {"targets": set(), "unknown": False, "sites": [], "unknown_sites": []}
    sp_vals, sp_dyn = {}, set()
    kind_rank = {"missing": 0, "abstract": 1}
    for n, d in docs:
        for c in d["classes"]:
            if c["name"] in drop:
                continue
            o = classes.get(c["name"])
            if o is None or LEVELS.index(c["level"]) > LEVELS.index(o["level"]):
                classes[c["name"]] = {"name": c["name"], "domain": c["domain"], "level": c["level"]}
        for m in d["methods"]:
            if not ok(m["id"]):
                continue
            o = methods.get(m["id"])
            if o is None or kind_rank.get(m["kind"], 2) > kind_rank.get(o["kind"], 2):
                methods[m["id"]] = {"id": m["id"], "kind": m["kind"]}
            if m["kind"] == "bytecode":
                fold_part[m["id"]].append(n)
        fm = {f["method"]: f for f in d.get("folds", [])}
        for mid in fm:
            if ok(mid):
                folds.setdefault(mid, []).append(fm[mid])
        for k in ("clinit", "refs", "unresolved", "dispatched", "instantiated", "hw_inherited"):
            sets[k] |= {x for x in d.get(k, []) if ok(x)}
        for x in d.get("missing", []):
            sets["missing_names"].add(x["name"])
        r = d["reflect"]
        for x in r.get("members", []):
            if ok(x["member"]):
                reflect["members"][x["member"]] = x
        for x in r.get("fields", []):
            reflect["fields"][(x["owner"], x["name"])] = x
        reflect["field_names"] |= set(r.get("field_names", []))
        reflect["gaps"] |= set(r.get("gaps", []))
        s = d.get("seeds", {})
        seeds["data_bundles"] |= set(s.get("data_bundles", []))
        seeds["annotation_enums"] |= set(s.get("annotation_enums", []))
        for j in s.get("jca", []):
            seeds["jca"][json.dumps(j, sort_keys=True)] = j
        for o, ns in (s.get("reflect_names") or {}).items():
            seeds["reflect_names"][o] |= set(ns)
        seeds["reflect_all"] |= set(s.get("reflect_all", []))
        for svc in s.get("services", []):
            for p in svc.get("providers", []):
                seeds["services"][svc["service"]][p["class"]] = p
        x = d.get("class_init") or {}
        ci["targets"] |= set(x.get("targets", []))
        ci["unknown"] |= bool(x.get("unknown"))
        sp = d["system_properties"]
        sp_dyn |= set(sp.get("dynamic", []))
        for k, v in (sp.get("values") or {}).items():
            if k in sp_vals and sp_vals[k] != v:
                sp_vals[k] = None
            else:
                sp_vals.setdefault(k, v)
    # 折叠：参与测试中有任何一例无折叠 → 不折叠；否则取折叠点最少的一份（与该例闭包自洽）
    out_folds = []
    for mid, fs in folds.items():
        if len(fs) < len(fold_part.get(mid, [])):
            continue
        out_folds.append(min(fs, key=fold_weight))
    sp_dyn |= {k for k, v in sp_vals.items() if v is None}
    out = {
        "classes": list(classes.values()),
        "methods": list(methods.values()),
        "clinit": sorted(sets["clinit"]),
        "refs": sorted(sets["refs"]),
        "missing": [{"name": x} for x in sorted(sets["missing_names"])],
        "unresolved": sorted(sets["unresolved"]),
        "folds_version": docs[0][1].get("folds_version"),
        "folds": out_folds,
        "reflect": {"members": list(reflect["members"].values()), "fields": list(reflect["fields"].values()),
                    "field_names": sorted(reflect["field_names"]), "gaps": sorted(reflect["gaps"]),
                    "field_enum_gaps": []},
        "seeds": {"data_bundles": sorted(seeds["data_bundles"]), "annotation_enums": sorted(seeds["annotation_enums"]),
                  "jca": list(seeds["jca"].values()),
                  "reflect_names": {k: sorted(v) for k, v in seeds["reflect_names"].items()},
                  "reflect_all": sorted(seeds["reflect_all"]),
                  "services": [{"service": k, "providers": list(v.values())} for k, v in seeds["services"].items()],
                  "services_unknown": False},
        "class_init": {"targets": sorted(ci["targets"]), "unknown": ci["unknown"], "sites": [], "unknown_sites": []},
        "dispatched": sorted(sets["dispatched"]),
        "instantiated": sorted(sets["instantiated"]),
        "hw_inherited": sorted(sets["hw_inherited"]),
        "system_properties": {"values": {k: v for k, v in sp_vals.items() if v is not None}, "dynamic": sorted(sp_dyn)},
        "summary": {"classes": len(classes), "methods": len(methods)},
    }
    Path(a.out).write_text(json.dumps(out, ensure_ascii=False), encoding="utf-8")
    print(f"并集：类 {len(classes)}，方法 {len(methods)}，折叠方法 {len(out_folds)}（来自 {len(docs)} 例，用户 {a.user}）")


# ── folds ─────────────────────────────────────────────────────────────────────

_INSN = re.compile(r"^\s*(\d+): (\w+)\s+#\d+(?:,\s*\d+)?\s+// (InterfaceMethod|Method|class|Field) (\S+)")


class Javap:
    """按需 javap -c -p 取 JDK 方法字节码，解析 pc → 虚 / 接口调用的静态属主；类修饰符判定可扩展性"""

    def __init__(self, home: str):
        self.home = home
        self.code: dict[str, dict[str, dict[int, str]]] = {}
        # 方法 → [(pc, 操作码, 引用)]：invoke* 的被调成员 `属主.名:描述符`、new 的类
        self.insns: dict[str, list[tuple[int, str, str]]] = {}
        self.mods: dict[str, str] = {}

    def _run(self, cls: str) -> str:
        r = subprocess.run([f"{self.home}/bin/javap", "-c", "-p", "-s", cls.replace("/", ".")],
                           capture_output=True, text=True)
        return r.stdout

    def load(self, cls: str):
        if cls in self.code:
            return
        txt = self._run(cls)
        methods: dict[str, dict[int, str]] = {}
        cur = None
        pending = ""
        for line in txt.splitlines():
            if line.startswith("    descriptor: "):
                cur = (pending, line.split(": ", 1)[1].strip())
                methods[f"{cur[0]}:{cur[1]}"] = {}
                continue
            if line.startswith("  ") and not line.startswith("    ") and "(" in line and line.rstrip().endswith(";"):
                head = line.split("(", 1)[0].split()
                pending = head[-1] if head else ""
                cls_simple = cls.rsplit("/", 1)[-1]
                if pending in (cls.replace("/", "."), cls_simple):
                    pending = "<init>"
                continue
            if line.strip() == "static {};":
                pending = "<clinit>"
                continue
            m = _INSN.match(line)
            if m and cur:
                key = f"{cur[0]}:{cur[1]}"
                pc, op, kind, ref = int(m.group(1)), m.group(2), m.group(3), m.group(4)
                if kind in ("Method", "InterfaceMethod") and "." not in ref.split(":", 1)[0]:
                    ref = f"{cls}.{ref}"
                if kind in ("Method", "InterfaceMethod"):
                    o = ref.split(".", 1)[0]
                    if op in ("invokevirtual", "invokeinterface"):
                        methods[key][pc] = o
                    self.insns.setdefault(f"{cls}.{key}", []).append((pc, op, ref))
                elif op == "new":
                    self.insns.setdefault(f"{cls}.{key}", []).append((pc, op, ref))
        self.code[cls] = methods
        head = next((l for l in txt.splitlines() if re.search(r"\b(class|interface)\b", l) and "{" in l), "")
        self.mods[cls] = head

    def recv(self, mid: str, pc: int) -> str | None:
        cls, rest = mid.split(".", 1)
        self.load(cls)
        return self.code[cls].get(rest, {}).get(pc)

    def extensible(self, cls: str) -> bool:
        """用户代码可扩展：导出包（java/ javax/）中 public、非 final、非 sealed 的类或接口"""
        if not cls.startswith(("java/", "javax/")):
            return False
        if cls not in self.mods:
            self.mods[cls] = next((l for l in self._run_head(cls)), "")
        h = self.mods[cls]
        if not h:
            return True
        return "public" in h and " final " not in f" {h} " and "sealed" not in h.split("permits")[0].split()

    def _run_head(self, cls: str):
        r = subprocess.run([f"{self.home}/bin/javap", "-p", cls.replace("/", ".")], capture_output=True, text=True)
        return [l for l in r.stdout.splitlines() if re.search(r"\b(class|interface)\b", l) and "{" in l]


def cmd_folds(a):
    tests = inputs(a.dirs)
    per_test = Counter()
    per_kind = Counter()
    fold_of = defaultdict(dict)          # method → test → null_recv pcs
    reach = defaultdict(set)             # method → tests reaching it as bytecode
    ndead = defaultdict(list)            # method → 各测试的 noreturn_dead_pcs 区间
    U_c, U_m = set(), set()
    for n, f in tests:
        d = load(f)
        users = {c["name"] for c in d["classes"] if c["domain"] == "user"}
        cs, ms = jdk_sets(d)
        U_c |= cs
        U_m |= ms
        for m in d["methods"]:
            if m["kind"] == "bytecode" and owner(m["id"]) not in users:
                reach[m["id"]].add(n)
        for x in d.get("folds", []):
            if owner(x["method"]) in users:
                continue
            fold_of[x["method"]][n] = tuple(sorted(x.get("null_recv", [])))
            ndead[x["method"]].extend(tuple(r) for r in x.get("noreturn_dead_pcs", []))
            per_kind["consts"] += len(x.get("consts", []))
            per_kind["null_recv"] += len(x.get("null_recv", []))
            per_kind["noreturn_calls"] += len(x.get("noreturn_calls", []))
        per_test[n] = sum(len(x.get("null_recv", [])) for x in d.get("folds", []) if owner(x["method"]) not in users)
    # 并集语义：方法在全部到达测试中都有同一 null_recv 点才保留
    union_pts, single_pts, lost_union = set(), set(), set()
    for mid, by in fold_of.items():
        pcs_all = [set(by.get(t, ())) for t in reach.get(mid, by.keys())]
        inter = set.intersection(*pcs_all) if pcs_all else set()
        anyp = set().union(*(set(v) for v in by.values()))
        union_pts |= {(mid, p) for p in inter}
        single_pts |= {(mid, p) for p in anyp}
        lost_union |= {(mid, p) for p in anyp - inter}
    jp = Javap(a.jdk_home)
    open_lost, kept, unknown = [], [], []
    recv_cnt = Counter()
    for mid, pc in sorted(union_pts):
        r = jp.recv(mid, pc)
        if r is None:
            unknown.append((mid, pc))
            continue
        if jp.extensible(r):
            open_lost.append((mid, pc, r))
            recv_cnt[r] += 1
        else:
            kept.append((mid, pc, r))
    print(f"测试数 {len(tests)}；逐例 JDK 折叠点合计 {dict(per_kind)}；逐例 null_recv 中位 "
          f"{sorted(per_test.values())[len(per_test) // 2]} / 最大 {max(per_test.values())}")
    print(f"null_recv 不同折叠点（方法+pc，任一测试） {len(single_pts)}")
    print(f"  并集语义保留 {len(union_pts)}，因测试间不一致丢失 {len(lost_union)}")
    print(f"  并集保留中：接收者类型可被用户扩展（开放世界不折叠） {len(open_lost)}，不可扩展照常折叠 {len(kept)}，"
          f"未解析 {len(unknown)}")
    print("  开放世界丢失点的接收者类型 Top：" + "，".join(f"{k.rsplit('/', 1)[-1]} {v}" for k, v in recv_cnt.most_common(15)))
    # 一阶闭包增量：不折叠后调用点之后的截断区间（noreturn_dead_pcs 中起点在调用点之后者）恢复可达，
    # 区间内 invoke / new 引用的、并集闭包外的方法与类；加被调声明本身。只算一层，不含传递展开
    add_m, add_c = set(), set()
    for mid, pc, r in open_lost:
        ins = jp.insns.get(mid, [])
        call = next((ref for p, op, ref in ins if p == pc), None)
        if call and call not in U_m:
            add_m.add(call)
        for lo, hi in set(ndead.get(mid, [])):
            if lo <= pc:
                continue
            for p, op, ref in ins:
                if lo <= p < hi:
                    if op == "new":
                        if ref not in U_c:
                            add_c.add(ref)
                    elif ref not in U_m:
                        add_m.add(ref)
                        if ref.split(".", 1)[0] not in U_c:
                            add_c.add(ref.split(".", 1)[0])
    print(f"  开放世界一阶闭包增量：方法 +{len(add_m)}（并集 {len(U_m)}，+{100 * len(add_m) / max(1, len(U_m)):.2f}%），"
          f"类 +{len(add_c)}（并集 {len(U_c)}，+{100 * len(add_c) / max(1, len(U_c)):.2f}%）")
    if a.dump:
        Path(a.dump).write_text("\n".join(f"{m}@{p}\t{r}" for m, p, r in open_lost), encoding="utf-8")


def main():
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    sub = ap.add_subparsers(dest="cmd", required=True)
    s = sub.add_parser("stats")
    s.add_argument("dirs", nargs="+")
    s.add_argument("--top", type=int, default=0)
    s.set_defaults(fn=cmd_stats)
    m = sub.add_parser("merge")
    m.add_argument("out")
    m.add_argument("--user", required=True)
    m.add_argument("jsons", nargs="+")
    m.set_defaults(fn=cmd_merge)
    f = sub.add_parser("folds")
    f.add_argument("dirs", nargs="+")
    f.add_argument("--jdk-home", required=True)
    f.add_argument("--dump")
    f.set_defaults(fn=cmd_folds)
    a = ap.parse_args()
    a.fn(a)


if __name__ == "__main__":
    main()
