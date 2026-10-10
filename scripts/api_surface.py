#!/usr/bin/env python3
"""框架 API 面与 e2e 分层（api_surface）——docs/plans/2026-10-07-framework-driven-api-coverage.md 步骤 1 / 2 的数据侧。

两个子命令（均在 dev 上经 scripts/api_surface_job.sh 以作业模式运行，本机不跑）：

  face   面 = 闭包调用链面（rava closure 产物里 owner 为 JDK 类、非 missing 的方法，各变体并集）；任一变体都未产出时
         退回近似口径「一跳面（阶段 jar 常量池的 JDK 方法引用，经 jdk_index 解析到声明类），声明类被 JVM 实载即算命中」。
         → 面文件（每行 `owner.name:descriptor`，按「引用 jar 数 × 调用链命中」排序）
         + face.json（规模、口径、按包聚合、排序前列、一跳对照、JVM 实载类对照）。
  tiers  tests/e2e 每例的 JDK 方法集（常量池口径，同 jdk_method_scan / e2e_redundancy_scan，另带描述符并解析到
         声明类）与面求交 → 分层 toml（主力 / 暂缓）+ 补测清单（一跳面上 e2e 零直接引用的公开方法）+ tiers.json。

分层规则（数据决定，脚本不写类名）：
  1. 语义类目录白名单（tests/api_surface/semantic_dirs.toml）→ 主力；
  2. 专属方法 = 该例方法集 − 通用方法（被 ≥ --df 比例的 e2e 用例引用，如打印 / 拼接 / 装箱）；
     专属为空 → 主力（无专属 API，属语义性质）；
  3. 专属方法中面外占比 > --out-ratio → 暂缓（主要覆盖面外 API）；否则 → 主力。
     阈值由数据决定（tiers.json 的 out_ratio_hist 给出分布），与 common_df 一同写入分层 toml。
"""

import argparse
import json
import os
import subprocess
import sys
import tomllib
from collections import Counter, defaultdict
from concurrent.futures import ThreadPoolExecutor
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from jdk_index import JdkIndex  # noqa: E402
from jdk_method_scan import parse_constant_pool  # noqa: E402

ROOT = Path(__file__).resolve().parent.parent
E2E = ROOT / "tests/e2e"
PUBLIC_PREFIXES = ("java/", "javax/", "org/w3c/", "org/xml/", "org/ietf/")


def split_key(key: str):
    head, desc = key.split(":", 1)
    owner, name = head.rsplit(".", 1)
    return owner, name, desc


def pkg_of(owner: str) -> str:
    return owner.rsplit("/", 1)[0].replace("/", ".") if "/" in owner else "(default)"


def class_refs(data: bytes, idx: JdkIndex) -> set:
    """单个 .class 的 JDK 方法引用（Methodref / InterfaceMethodref，含构造器），解析到声明类"""
    _, _, refs = parse_constant_pool(data)
    out = set()
    for tag, owner, name, desc in refs:
        if tag == 9 or owner.startswith("[") or name in ("?", "", "<clinit>") or not idx.has(owner):
            continue
        out.add(f"{idx.resolve(owner, name, desc)}.{name}:{desc}")
    return out


def jar_refs(jar: Path, idx: JdkIndex) -> set:
    import zipfile
    out = set()
    with zipfile.ZipFile(jar) as z:
        for n in z.namelist():
            if n.endswith(".class") and not n.endswith("module-info.class"):
                try:
                    out |= class_refs(z.read(n), idx)
                except Exception:
                    continue
    return out


def closure_jdk(path: Path, idx: JdkIndex):
    """closure.json → (JDK 方法键集合, JDK 类集合, 摘要, 缺失类前列)"""
    v = json.loads(path.read_text(encoding="utf-8"))
    methods = set()
    for m in v.get("methods", []):
        if m.get("kind") == "missing":
            continue
        owner, _, _ = split_key(m["id"])
        if idx.has(owner):
            methods.add(m["id"])
    classes = {c["name"] for c in v.get("classes", []) if idx.has(c["name"])}
    missing = [m["name"] for m in v.get("missing", [])]
    return methods, classes, v.get("summary", {}), missing


def jvm_loaded(log: Path):
    """-Xlog:class+load 日志 → (JDK 类集合, 非 JDK 类 → 来源)"""
    jdk, other = set(), {}
    for ln in log.read_text(encoding="utf-8", errors="replace").splitlines():
        if "source:" not in ln:
            continue
        body = ln.split("]")[-1].strip()
        name, _, src = body.partition(" source: ")
        name = name.strip().replace(".", "/")
        if src.startswith("jrt:/") or src.startswith("shared objects file"):
            jdk.add(name)
        else:
            other[name] = src.strip()
    return jdk, other


def cmd_seeds(a):
    """JVM 实载类日志 → 闭包 --seed-class 清单（来源为阶段 jar 或样例类目录的类；运行期生成类不在其中）"""
    _, other = jvm_loaded(Path(a.classload))
    roots = [os.path.abspath(p) for p in a.source]
    seeds = sorted(n for n, src in other.items()
                   if src.startswith("file:") and any(src[5:].startswith(r) for r in roots))
    Path(a.out).write_text("\n".join(seeds) + "\n", encoding="utf-8")
    print(f"[api-surface] 种子类 {len(seeds)} 个（实载非 JDK 类 {len(other)}）→ {a.out}")


def cmd_face(a):
    idx = JdkIndex.load(Path(a.java_home))
    variants, chain, chain_classes, missing_all = {}, set(), set(), Counter()
    for spec in a.closure:
        name, _, path = spec.partition("=")
        if not Path(path).exists():
            variants[name] = {"status": "缺失（闭包未产出）"}
            continue
        ms, cs, summary, missing = closure_jdk(Path(path), idx)
        variants[name] = {"status": "ok", "jdk_methods": len(ms), "jdk_classes": len(cs),
                          "summary": {k: summary.get(k) for k in ("classes", "methods", "elapsed_ms") if k in summary},
                          "missing_classes": len(missing)}
        missing_all.update(missing)
        chain |= ms
        chain_classes |= cs
    jar_count = Counter()
    per_jar = {}
    for jar in map(Path, a.jar):
        refs = jar_refs(jar, idx)
        per_jar[jar.name] = len(refs)
        jar_count.update(refs)
    onehop = set(jar_count)
    loaded = other = None
    if a.classload and Path(a.classload).exists():
        loaded, other = jvm_loaded(Path(a.classload))
    # 闭包全部未产出时的近似：一跳方法的声明类被真 JVM 实载 = 调用链命中（报告须标「闭包待复算」）
    chain_mode = "closure" if any(v.get("status") == "ok" for v in variants.values()) else "jvm-loaded"
    if chain_mode == "jvm-loaded":
        face = onehop
        hit = {k for k in onehop if split_key(k)[0] in loaded} if loaded is not None else set()
    else:
        face, hit = chain, chain

    def rank(k):
        j, c = jar_count.get(k, 0), 1 if k in hit else 0
        return (-(j * c), -j, -c, k)

    ordered = sorted(face, key=rank)
    Path(a.out).parent.mkdir(parents=True, exist_ok=True)
    Path(a.out).write_text("\n".join(ordered) + "\n", encoding="utf-8")

    def public(k):
        o, n, d = split_key(k)
        return o.startswith(PUBLIC_PREFIXES) and idx.is_public_api(o, n, d)

    by_pkg = defaultdict(lambda: [0, 0, 0, 0])
    for k in face:
        row = by_pkg[pkg_of(split_key(k)[0])]
        row[0] += 1
        row[1] += k in hit
        row[2] += k in onehop
        row[3] += public(k)
    # 一跳对照：一跳面中落在调用链外的部分（闭包剪掉的可选特性引用），按包聚合
    onehop_out = onehop - chain if chain_mode == "closure" else set()
    out_pkg = Counter(pkg_of(split_key(k)[0]) for k in onehop_out)
    data = {
        "face": len(face), "chain": len(chain), "onehop": len(onehop), "both": len(chain & onehop),
        "onehop_only": len(onehop - chain), "chain_only": len(chain - onehop),
        "public": sum(1 for k in face if public(k)), "chain_classes": len(chain_classes),
        "chain_public": sum(1 for k in chain if public(k)), "onehop_public": sum(1 for k in onehop if public(k)),
        "onehop_out_by_pkg": out_pkg.most_common(40),
        "onehop_out_top": sorted(onehop_out, key=lambda k: (-jar_count[k], k))[:120],
        "onehop_out_jars": Counter(jar_count[k] for k in onehop_out).most_common(),
        "chain_by_pkg": Counter(pkg_of(split_key(k)[0]) for k in chain).most_common(60),
        "chain_mode": chain_mode, "hit": len(hit), "public_hit": sum(1 for k in hit if public(k)),
        "variants": variants, "per_jar_onehop": per_jar,
        "by_pkg": sorted(([p, *r] for p, r in by_pkg.items()), key=lambda r: -r[1]),
        "top": [[k, jar_count.get(k, 0), k in hit] for k in ordered[:a.top]],
        "onehop_jars": {k: jar_count[k] for k in onehop},
        "onehop_all": sorted(onehop),
        "missing_lib_classes_top": [n for n, _ in missing_all.most_common(60)],
    }
    if loaded is not None:
        lost = sorted(c for c in loaded - chain_classes if "$$" not in c and "/$Proxy" not in c)
        lost_pkg = Counter(pkg_of(c) for c in lost)
        data["jvm"] = {"loaded_jdk": len(loaded), "loaded_other": len(other),
                       "loaded_jdk_in_closure": len(loaded & chain_classes),
                       "lost_by_pkg": lost_pkg.most_common(40), "lost_sample": lost[:200]}
    Path(a.data).write_text(json.dumps(data, ensure_ascii=False, indent=1), encoding="utf-8")
    print(f"[api-surface] 面 {len(face)}（{chain_mode}，命中 {len(hit)}，调用链 {len(chain)}，一跳 {len(onehop)}，交 {len(chain & onehop)}）→ {a.out}")
    for n, v in variants.items():
        print(f"  变体 {n}: {v}")


def compile_test(java: Path, out: Path, javac: str, cp: str) -> bool:
    marker = out / ".ok"
    if marker.exists() and marker.stat().st_mtime >= java.stat().st_mtime:
        return True
    out.mkdir(parents=True, exist_ok=True)
    cmd = [javac, "-g", "-nowarn", "-encoding", "UTF-8", "-d", str(out)]
    if cp:
        cmd += ["-cp", cp]
    r = subprocess.run(cmd + [str(java)], capture_output=True, text=True, cwd=ROOT)
    if r.returncode != 0:
        return False
    marker.write_text("ok")
    return True


def test_methods(cls_dir: Path, idx: JdkIndex) -> set:
    out = set()
    for cf in cls_dir.rglob("*.class"):
        try:
            out |= class_refs(cf.read_bytes(), idx)
        except Exception:
            continue
    return out


def toml_list(name, items, per_line=1):
    lines = [f"{name} = ["]
    lines += [f'  "{x}",' for x in items]
    lines.append("]")
    return lines


def cmd_tiers(a):
    idx = JdkIndex.load(Path(a.java_home))
    face = [l.strip() for l in Path(a.face).read_text(encoding="utf-8").splitlines() if l.strip()]
    face_set = set(face)
    fdata = json.loads(Path(a.face_data).read_text(encoding="utf-8"))
    onehop_jars = fdata["onehop_jars"]
    semantic = set(tomllib.loads(Path(a.semantic).read_text(encoding="utf-8"))["dirs"])
    javac = str(Path(a.java_home) / "bin/javac")
    files = sorted(E2E.rglob("*.java"))
    out_root = ROOT / "build/api_surface/e2e-classes"

    def work(java):
        d = out_root / java.relative_to(E2E).with_suffix("").as_posix().replace("/", "__")
        ok = compile_test(java, d, javac, a.cp or "")
        return java, (test_methods(d, idx) if ok else None)

    with ThreadPoolExecutor(max_workers=a.jobs) as ex:
        results = list(ex.map(work, files))
    msets = {}
    failed = []
    for java, ms in results:
        tid = java.relative_to(E2E).with_suffix("").as_posix()
        if ms is None:
            failed.append(tid)
        else:
            msets[tid] = ms
    df = Counter()
    for ms in msets.values():
        df.update(ms)
    cut = max(2, int(a.df * len(msets)))
    common = {k for k, c in df.items() if c >= cut}

    tiers, detail = {"main": [], "deferred": []}, {}
    for tid in sorted(msets):
        ms = msets[tid]
        d = tid.split("/")[0]
        spec = ms - common
        inf = sorted(spec & face_set)
        outf = sorted(spec - face_set)
        ratio = len(outf) / len(spec) if spec else 0.0
        if d in semantic:
            tier, why = "main", "语义类目录"
        elif not spec:
            tier, why = "main", "无专属 API"
        elif ratio > a.out_ratio:
            tier, why = "deferred", f"专属 {len(spec)}，面外 {len(outf)}（{ratio:.0%} > {a.out_ratio:.0%}）"
        else:
            tier, why = "main", f"专属 {len(spec)}，面外 {len(outf)}（{ratio:.0%} ≤ {a.out_ratio:.0%}）"
        tiers[tier].append(tid)
        detail[tid] = {"tier": tier, "why": why, "methods": len(ms), "specific": len(spec), "out_ratio": round(ratio, 4),
                       "semantic": d in semantic, "in_face": inf, "out_face": outf, "n_in": len(inf), "n_out": len(outf)}
    for tid in failed:
        tiers["main"].append(tid)
        detail[tid] = {"tier": "main", "why": "javac 失败，缺省主力", "methods": 0, "specific": 0, "out_ratio": 0.0,
                       "semantic": tid.split("/")[0] in semantic, "in_face": [], "out_face": [], "n_in": 0, "n_out": 0}

    covered = set(df)
    gap = []
    for k in face:
        if k in covered or k not in onehop_jars:
            continue
        o, n, dsc = split_key(k)
        if n == "<init>" and not idx.is_public_api(o, n, dsc):
            continue
        if o.startswith(PUBLIC_PREFIXES) and idx.is_public_api(o, n, dsc):
            gap.append(k)
    chain_pub_uncovered = sum(1 for k in face if k not in covered and k not in onehop_jars
                              and split_key(k)[0].startswith(PUBLIC_PREFIXES) and idx.is_public_api(*split_key(k)))

    kf = tomllib.loads((ROOT / "docs/known_failures.toml").read_text(encoding="utf-8"))
    by_stem = {tid.split("/")[-1]: tid for tid in detail}
    review = {}
    for e in kf.get("known", []):
        tid = by_stem.get(e["test"])
        if tid:
            review[e["test"]] = {"tid": tid, "deferred": e.get("deferred"), **detail[tid]}

    lines = [
        "# e2e 分层数据（S0）——scripts/api_surface.py tiers 生成，勿手改；口径见",
        "# docs/plans/2026-10-07-framework-driven-api-coverage.md §一 与 docs/reports/api-surface-s0.md",
        'stage = "S0"',
        'face = "tests/api_surface/s0.txt"',
        f"common_df = {a.df}  # 通用方法门槛：被 ≥ 该比例 e2e 用例引用（本轮 ≥ {cut} 例，{len(common)} 个方法）",
        f"out_ratio = {a.out_ratio}  # 暂缓门槛：专属方法中面外占比 > 该值（非语义目录、专属非空的用例）",
        "",
        "[summary]",
        f"tests = {len(detail)}",
        f"main = {len(tiers['main'])}",
        f"deferred = {len(tiers['deferred'])}",
        f"gap = {len(gap)}",
        f"javac_failed = {len(failed)}",
        "",
        "[semantic]",
        *toml_list("dirs", sorted(semantic)),
        "",
        "[tiers]",
        *toml_list("main", sorted(tiers["main"])),
        *toml_list("deferred", sorted(tiers["deferred"])),
        "",
        "# 补测：一跳面（S0 jar 直接调用）上的公开 JDK 方法，e2e 零直接引用；按面排序（引用 jar 数 × 调用链命中）",
        "[gap]",
        *toml_list("methods", gap),
    ]
    Path(a.out).write_text("\n".join(lines) + "\n", encoding="utf-8")
    data = {"tests": len(detail), "main": len(tiers["main"]), "deferred": len(tiers["deferred"]),
            "gap": len(gap), "gap_chain_public_uncovered": chain_pub_uncovered, "failed": failed,
            "common_cut": cut, "common": sorted(common), "deferred_list": sorted(tiers["deferred"]),
            "by_dir": {}, "review": review, "gap_top": [[k, onehop_jars.get(k, 0)] for k in gap[:150]],
            "e2e_face_cover": len(covered & face_set)}
    by_dir = defaultdict(Counter)
    for tid, d in detail.items():
        by_dir[tid.split("/")[0]][d["tier"]] += 1
    data["by_dir"] = {k: dict(v) for k, v in sorted(by_dir.items())}
    data["deferred_detail"] = {t: detail[t] for t in tiers["deferred"]}
    # 面外占比分布（非语义目录、专属非空）：定 --out-ratio 的数据依据；detail 全量供离线换阈值重分
    rated = [d["out_ratio"] for d in detail.values() if not d["semantic"] and d["specific"]]
    data["out_ratio"] = a.out_ratio
    data["out_ratio_hist"] = {f"{i / 10:.1f}": sum(1 for r in rated if i / 10 <= r < (i + 1) / 10 or (i == 9 and r == 1.0))
                              for i in range(10)}
    data["out_ratio_full"] = sum(1 for r in rated if r == 1.0)
    data["detail"] = detail
    Path(a.data).write_text(json.dumps(data, ensure_ascii=False, indent=1), encoding="utf-8")
    print(f"[api-surface] 分层：主力 {len(tiers['main'])}，暂缓 {len(tiers['deferred'])}，补测 {len(gap)}"
          f"（javac 失败 {len(failed)}）→ {a.out}")


def main():
    ap = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    sub = ap.add_subparsers(dest="cmd", required=True)
    s = sub.add_parser("seeds")
    s.add_argument("--classload", required=True)
    s.add_argument("--source", action="append", default=[], help="种子来源前缀（jar 路径或类目录）")
    s.add_argument("--out", required=True)
    f = sub.add_parser("face")
    f.add_argument("--closure", action="append", default=[], metavar="名=closure.json")
    f.add_argument("--jar", action="append", default=[])
    f.add_argument("--java-home", default=os.environ.get("JAVA_HOME", ""))
    f.add_argument("--classload")
    f.add_argument("--top", type=int, default=200)
    f.add_argument("--out", required=True)
    f.add_argument("--data", required=True)
    t = sub.add_parser("tiers")
    t.add_argument("--face", required=True)
    t.add_argument("--face-data", required=True)
    t.add_argument("--semantic", default=str(ROOT / "tests/api_surface/semantic_dirs.toml"))
    t.add_argument("--java-home", default=os.environ.get("JAVA_HOME", ""))
    t.add_argument("--cp", help="e2e javac 类路径（63_junit 需 junit / hamcrest）")
    t.add_argument("--df", type=float, default=0.10)
    t.add_argument("--out-ratio", type=float, default=0.5, help="暂缓门槛：专属方法中面外占比 > 该值")
    t.add_argument("-j", "--jobs", type=int, default=8)
    t.add_argument("--out", required=True)
    t.add_argument("--data", required=True)
    a = ap.parse_args()
    {"seeds": cmd_seeds, "face": cmd_face, "tiers": cmd_tiers}[a.cmd](a)


if __name__ == "__main__":
    main()
