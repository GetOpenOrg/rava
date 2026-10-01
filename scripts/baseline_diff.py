#!/usr/bin/env python3
"""全量结果与冻结基线对照：Python 生成器删除判据（docs/plans/2026-10-01-python-generator-deletion.md §一）。

判定：基线通过集 ⊆ 本次通过集，且本次无转译失败。基线里已不在语料中的用例（删除 / 改名）
单列为「语料已移除」，不计回归，但须在 §五 逐例说明。

输入：
  --baseline   冻结基线清单（缺省 docs/plans/2026-10-01-python-baseline-jdk21.txt）
  --passed     本次通过清单，可多个（run_tests.py --record-passed 的 passed_tests_jdk21.txt，
               或分布式跑批各节点的通过清单）；清单格式：每行一个用例路径，# 开头为注释
  --log        本次跑批日志，可多个；从中提取转译失败（「— transpile error」行）
  --commit     本次跑批的提交号，写入报告

输出：Markdown 报告（可直接贴入 §五）；退出码 0 = 满足删除条件，1 = 不满足。

用法：
    python3 scripts/baseline_diff.py --passed build/passed_tests_jdk21.txt --log build/logs/bg/full21.log --commit <sha>
"""
from __future__ import annotations

import argparse
import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
E2E_PREFIX = "tests/e2e/"
DEFAULT_BASELINE = ROOT / "docs/plans/2026-10-01-python-baseline-jdk21.txt"
# run_tests.py 结果行：`[时间] [FAIL ] 01_basics/X.java ... — transpile error`
TRANSPILE_FAIL_RE = re.compile(r"\[\s*FAIL\s*\]\s+(\S+\.java)\b.*—\s*transpile error")


def normalize(name: str) -> str:
    """用例名统一为 tests/e2e/ 相对路径（日志行里是 E2E 目录相对路径）。"""
    name = name.strip()
    return name if name.startswith(E2E_PREFIX) else E2E_PREFIX + name


def load_list(path: Path) -> set[str]:
    return {normalize(ln) for ln in path.read_text(encoding="utf-8").splitlines()
            if ln.strip() and not ln.lstrip().startswith("#")}


def transpile_failures(log_text: str) -> set[str]:
    return {normalize(m.group(1)) for m in TRANSPILE_FAIL_RE.finditer(log_text)}


def corpus(root: Path) -> set[str]:
    return {str(p.relative_to(root)) for p in (root / E2E_PREFIX).rglob("*.java")}


def compare(baseline: set[str], passed: set[str], tfail: set[str],
            present: set[str]) -> dict[str, list[str]]:
    missing = baseline - passed
    return {
        "regressions": sorted(missing & present),
        "removed": sorted(missing - present),
        "transpile_fail": sorted(tfail),
        "new_passes": sorted(passed - baseline),
    }


def report(res: dict[str, list[str]], n_base: int, n_passed: int, commit: str) -> str:
    ok = not res["regressions"] and not res["transpile_fail"]
    lines = [
        f"- 提交：`{commit or '（未填）'}`",
        f"- 基线通过 {n_base} 例；本次通过 {n_passed} 例（其中基线外新增 {len(res['new_passes'])} 例）",
        f"- 回归（基线通过、本次未通过且仍在语料中）：{len(res['regressions'])}",
        f"- 语料已移除（基线有、语料无，须逐例说明）：{len(res['removed'])}",
        f"- 转译失败：{len(res['transpile_fail'])}",
        f"- 结论：{'✅ 满足删除条件' if ok else '❌ 不满足删除条件'}",
    ]
    for key, title in (("regressions", "回归"), ("removed", "语料已移除"),
                       ("transpile_fail", "转译失败")):
        if res[key]:
            lines.append(f"\n{title}：")
            lines.extend(f"- `{n}`" for n in res[key])
    return "\n".join(lines) + "\n"


def main(argv: list[str] | None = None) -> int:
    ap = argparse.ArgumentParser(description=__doc__.split("\n")[0])
    ap.add_argument("--baseline", type=Path, default=DEFAULT_BASELINE)
    ap.add_argument("--passed", type=Path, nargs="+", required=True)
    ap.add_argument("--log", type=Path, nargs="*", default=[])
    ap.add_argument("--commit", default="")
    args = ap.parse_args(argv)

    baseline = load_list(args.baseline)
    passed: set[str] = set()
    for p in args.passed:
        passed |= load_list(p)
    tfail: set[str] = set()
    for p in args.log:
        tfail |= transpile_failures(p.read_text(encoding="utf-8", errors="replace"))
    res = compare(baseline, passed, tfail, corpus(ROOT))
    sys.stdout.write(report(res, len(baseline), len(passed), args.commit))
    return 0 if not res["regressions"] and not res["transpile_fail"] else 1


if __name__ == "__main__":
    sys.exit(main())
