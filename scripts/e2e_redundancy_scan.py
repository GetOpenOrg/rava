#!/usr/bin/env python3
"""e2e 冗余透视（e2e_redundancy_scan）——哪些测试的 JDK 调用面被其他测试完全盖住。

目的：控制语料数量、压缩全量测试时间。与 jdk_method_scan 互补：那个找「jar 用了
但 e2e 没盖」的缺口，这个找「e2e 内部互相盖住」的重复。

方法：
  1. 逐测试 javac 到 build/redundancy-classes/<名>/（带 mtime 缓存，二次跑秒级）；
  2. 解析 .class 常量池 → 该测试**实际调用**的 JDK (类, 方法) 集合（复用
     jdk_method_scan 的解析器；口径同为符号引用，不含反射字符串）；
  3. 冗余判定（两条，均只出候选、人工裁决）：
     a. 同目录子集：M(t) ⊆ ∪ M(同目录其他)——目录是语义域，同域内被完全
        盖住 = 强冗余候选（跨目录不做子集判定：算法族 JDK 面同 tiny 但逻辑互异）；
     b. 相似对：Jaccard(M(a), M(b)) ≥ 阈值（同目录 0.75 / 跨目录 0.9）。

注意：
  - 名字级覆盖 ≠ 分支语义等价（同一方法的不同异常分支可能分属两测试）——输出
    是**候选清单**，删除需人工过目；
  - JDK 面为空/极小的测试（纯算法题）不参与子集判定，仅计数说明；
  - 编译失败的测试原样列出（通常是被 --skip-failed 基线管理的已知失败）。

用法：
    python3 scripts/e2e_redundancy_scan.py                 # 全量分析（首跑含编译）
    python3 scripts/e2e_redundancy_scan.py --dir 62_reflection   # 单目录深看
    python3 scripts/e2e_redundancy_scan.py --jaccard 0.8   # 相似阈值
    python3 scripts/e2e_redundancy_scan.py --report docs/reports/e2e-redundancy-scan.md
"""

import argparse
import subprocess
import sys
from collections import defaultdict
from itertools import combinations
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from jdk_method_scan import JDK_PREFIXES, parse_constant_pool  # noqa: E402

ROOT = Path(__file__).resolve().parent.parent
E2E = ROOT / "tests/e2e"
OUT = ROOT / "build/redundancy-classes"


def compile_one(java: Path, cls_dir: Path, javac: str) -> bool:
    """javac 单测试（mtime 缓存）；返回是否可用。"""
    marker = cls_dir / ".ok"
    if marker.exists() and marker.stat().st_mtime >= java.stat().st_mtime:
        return True
    if not any(cls_dir.glob("*.class")):
        cls_dir.mkdir(parents=True, exist_ok=True)
    r = subprocess.run([javac, "-g", "-d", str(cls_dir), str(java)],
                       capture_output=True, text=True, cwd=ROOT)
    if r.returncode != 0:
        return False
    marker.write_text("ok")
    return True


def method_set(cls_dir: Path) -> set:
    """目录内全部 .class 的 JDK 方法引用集合。"""
    out = set()
    for cf in cls_dir.rglob("*.class"):
        try:
            data = cf.read_bytes()
        except OSError:
            continue
        try:
            _, _, refs = parse_constant_pool(data)
        except Exception:
            continue
        for tag, cls, name, _ in refs:
            if tag != 9 and cls.startswith(JDK_PREFIXES) and name not in ("?", "", "<init>", "<clinit>"):
                out.add((cls, name))
    return out


def jaccard(a: set, b: set) -> float:
    if not a and not b:
        return 1.0
    return len(a & b) / len(a | b)


def main():
    ap = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    ap.add_argument("--dir", help="只分析单个目录名（如 62_reflection）")
    ap.add_argument("--jaccard", type=float, default=0.75, help="同目录相似阈值（默认 0.75）")
    ap.add_argument("--report", metavar="PATH", help="报告落盘（markdown）")
    args = ap.parse_args()

    import shutil
    jdk = subprocess.run([sys.executable, str(ROOT / "scripts/jdk_select.py")],
                         capture_output=True, text=True)
    home = Path(jdk.stdout.strip() or "")
    javac_bin = str(home / "bin/javac") if (home / "bin/javac").exists() else shutil.which("javac") or "javac"

    files = sorted(E2E.rglob("*.java"))
    if args.dir:
        files = [f for f in files if f.parent.name == args.dir]
    print(f"[redundancy] 分析 {len(files)} 个测试（编译缓存 {OUT}）…")

    msets = {}
    failed = []
    tiny = 0
    for i, java in enumerate(files, 1):
        cls_dir = OUT / java.stem
        if not compile_one(java, cls_dir, javac_bin):
            failed.append(java.relative_to(E2E).as_posix())
            continue
        ms = method_set(cls_dir)
        msets[java] = ms
        if len(ms) < 3:
            tiny += 1
        if i % 150 == 0:
            print(f"  … {i}/{len(files)}")

    by_dir = defaultdict(list)
    for java, ms in msets.items():
        by_dir[java.parent.name].append((java, ms))

    lines = [f"# e2e 冗余透视（{len(files)} 测试，JDK 方法集 = 常量池符号引用）", ""]
    lines.append(f"- 编译失败（跳过，多属已知失败基线）：{len(failed)}")
    lines.append(f"- JDK 面过小（<3 方法，纯算法族，不参与子集判定）：{tiny}")
    lines.append("")

    # ── 同目录子集判定 ──────────────────────────────────────────────
    lines.append("## 一、同目录内被其余测试完全覆盖（强冗余候选）")
    lines.append("")
    candidates = []
    for d, items in sorted(by_dir.items()):
        union_other = {}
        for java, ms in items:
            for (cls, name) in ms:
                union_other.setdefault((cls, name), []).append(java.stem)
        dir_union = set(union_other)
        for java, ms in items:
            if len(ms) < 3:
                continue
            cover_count = defaultdict(int)
            for key in ms:
                for t in union_other[key]:
                    cover_count[t] += 1
            cover_count.pop(java.stem, None)
            # 真正的缺失 = 只被本测试引用到的键
            missing = [k for k in ms if set(union_other[k]) == {java.stem}]
            if not missing and cover_count:
                top = sorted(cover_count.items(), key=lambda kv: -kv[1])[:3]
                top_s = ", ".join(f"{t}({c}/{len(ms)})" for t, c in top)
                candidates.append((d, java.stem, len(ms), top_s))
    if candidates:
        lines.append(f"共 {len(candidates)} 个候选（目录 | 测试 | JDK 面大小 | 主要覆盖者）：")
        lines.append("")
        for d, name, size, top in candidates:
            lines.append(f"- {d} / **{name}**（{size}）← {top}")
    else:
        lines.append("- （无）")
    lines.append("")

    # ── 相似对 ─────────────────────────────────────────────────────
    lines.append(f"## 二、高相似对（同目录 Jaccard ≥ {args.jaccard}）")
    lines.append("")
    pairs = []
    for d, items in sorted(by_dir.items()):
        for (a, ma), (b, mb) in combinations(items, 2):
            j = jaccard(ma, mb)
            if j >= args.jaccard and len(ma) >= 3 and len(mb) >= 3:
                pairs.append((d, a.stem, b.stem, j, len(ma), len(mb)))
    for d, a, b, j, la, lb in sorted(pairs, key=lambda x: -x[3])[:60]:
        lines.append(f"- {d}: {a} ↔ {b} — J={j:.2f}（{la}/{lb}）")
    if not pairs:
        lines.append("- （无）")
    lines.append("")

    lines.append("## 三、编译失败清单（供基线核对）")
    lines.append("")
    lines.extend(f"- {f}" for f in failed[:20] if failed) or lines.append("- （无）")
    lines.append("")

    total_face = set()
    for ms in msets.values():
        total_face |= ms
    lines.append(f"> 全库 JDK 方法面合计 {len(total_face)} 个 (类,方法) 对；"
                 f"候选删除不影响该面（子集判定保证），但分支语义需人工复核。")

    print("\n".join(lines))
    if args.report:
        out = Path(args.report)
        out.parent.mkdir(parents=True, exist_ok=True)
        out.write_text("\n".join(lines) + "\n", encoding="utf-8")
        print(f"\n[redundancy] 报告已落盘：{out}")


if __name__ == "__main__":
    main()
