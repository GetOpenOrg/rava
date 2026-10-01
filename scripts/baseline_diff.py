#!/usr/bin/env python3
"""全量结果与冻结基线对照：Python 生成器删除判据（docs/plans/2026-10-01-python-generator-deletion.md §一）。

判定：语料全部有结论（无未跑、无「无结果」），基线通过集 ⊆ 本次通过集，且本次无转译失败。基线里已不在
语料中的用例（删除 / 改名）单列为「语料已移除」，不计回归，但须在 §五 逐例说明。

通过集按跨提交累计口径（distribute_tests 续跑时不清零，每例记录其运行提交）；报告列出提交分布，
并单列「通过时的提交早于最新提交」的例数，供判断是否需要 --reset 在最新代码上全量复跑。

输入（二选一）：
  --results    分布式跑批结果目录（server_maintenance/rava/test_results）：读 state_jdk{N}.json 的
               passed / failed（每例带运行提交），转译失败取 error_logs/*_jdk{N}.log
  --passed     通过清单，可多个（run_tests.py --record-passed 的 passed_tests_jdk21.txt，或
               distribute_tests 的 master_passed_jdk21.txt）；每行一个用例路径，# 开头为注释
  --log        跑批日志，可多个；从中提取转译失败（「— transpile error」行）
  --commit     提交号，写入报告（--results 下取提交分布，可省略）
  --baseline   冻结基线清单（缺省 docs/plans/2026-10-01-python-baseline-jdk21.txt）
  --jdk        JDK 版本（缺省 21，用于 --results 下的文件名）

输出：Markdown 报告（可直接贴入 §五）；退出码 0 = 满足删除条件，1 = 不满足，2 = 输入错误。

用法：
    python3 scripts/baseline_diff.py --results ../server_maintenance/rava/test_results
    python3 scripts/baseline_diff.py --passed build/passed_tests_jdk21.txt --log build/logs/bg/full21.log --commit <sha>
"""
from __future__ import annotations

import argparse
import json
import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
E2E_PREFIX = "tests/e2e/"
DEFAULT_BASELINE = ROOT / "docs/plans/2026-10-01-python-baseline-jdk21.txt"
# run_tests.py 结果行：`[时间] [FAIL ] 01_basics/X.java ... — transpile error`
TRANSPILE_FAIL_RE = re.compile(r"\[\s*FAIL\s*\]\s+(\S+\.java)\b.*—\s*transpile error")
# distribute_tests 对「输出既无 PASS 也无 FAIL 行」的失败原因前缀
NO_RESULT = "no result"


class InputError(Exception):
    """输入错误（结果目录缺件），退出码 2。"""


def normalize(name: str) -> str:
    """用例名统一为 tests/e2e/ 相对路径（日志行里是 E2E 目录相对路径）。"""
    name = name.strip()
    return name if name.startswith(E2E_PREFIX) else E2E_PREFIX + name


def load_list(path: Path) -> set[str]:
    return {normalize(ln) for ln in path.read_text(encoding="utf-8").splitlines()
            if ln.strip() and not ln.lstrip().startswith("#")}


def load_results(results: Path, jdk: int, by_stem: dict[str, str]) -> dict:
    """读取分布式结果目录：通过集、提交分布、转译失败、未跑、无结果。"""
    state_path = results / f"state_jdk{jdk}.json"
    if not state_path.exists():
        raise InputError(f"缺少 {state_path}")
    state = json.loads(state_path.read_text(encoding="utf-8"))
    if "passed" in state:
        passed_map = {t: (v or {}).get("commit") for t, v in state["passed"].items()}
    else:  # 旧格式：completed 列表 + 可选全局 commit
        passed_map = {t: state.get("commit") for t in state.get("completed", [])}
    failed = state.get("failed", {})
    dist: dict[str, int] = {}
    for c in passed_map.values():
        key = (c or "unknown")[:9]
        dist[key] = dist.get(key, 0) + 1
    no_result = {by_stem.get(n, normalize(n)) for n, v in failed.items()
                 if str(v.get("error", "")).startswith(NO_RESULT)}
    # 转译失败只取当前仍在失败集里的用例（之后已通过的旧日志不计）
    failed_paths = {by_stem.get(n, normalize(n)) for n in failed}
    tfail: set[str] = set()
    err_dir = results / "error_logs"
    for log in sorted(err_dir.glob(f"*_jdk{jdk}.log")) if err_dir.exists() else ():
        tfail |= transpile_failures(log.read_text(encoding="utf-8", errors="replace"))
    ran = set(passed_map) | set(failed)
    return {
        "passed": {by_stem.get(n, normalize(n)) for n in passed_map},
        "commits": dist,
        "tfail": tfail & failed_paths,
        "not_run": sorted(p for s, p in by_stem.items() if s not in ran),
        "no_result": sorted(no_result),
    }


def transpile_failures(log_text: str) -> set[str]:
    return {normalize(m.group(1)) for m in TRANSPILE_FAIL_RE.finditer(log_text)}


def corpus(root: Path) -> set[str]:
    return {str(p.relative_to(root)) for p in (root / E2E_PREFIX).rglob("*.java")}


def compare(baseline: set[str], passed: set[str], tfail: set[str], present: set[str],
            not_run: list[str] = (), no_result: list[str] = ()) -> dict[str, list[str]]:
    missing = baseline - passed
    return {
        "regressions": sorted((missing & present) - set(not_run)),  # 未跑的单列，不算回归
        "removed": sorted(missing - present),
        "transpile_fail": sorted(tfail),
        "not_run": sorted(not_run),
        "no_result": sorted(no_result),
        "new_passes": sorted(passed - baseline),
    }


def satisfied(res: dict[str, list[str]]) -> bool:
    return not any(res[k] for k in ("regressions", "transpile_fail", "not_run", "no_result"))


def report(res: dict[str, list[str]], n_base: int, n_passed: int, commit: str,
           commits: dict[str, int] | None = None) -> str:
    dist = ", ".join(f"`{c}`×{n}" for c, n in sorted((commits or {}).items(), key=lambda x: -x[1]))
    lines = [
        f"- 提交：{dist or f'`{commit}`' if (dist or commit) else '（未填）'}",
        f"- 基线通过 {n_base} 例；本次通过 {n_passed} 例（其中基线外新增 {len(res['new_passes'])} 例）",
        f"- 回归（基线通过、本次未通过且仍在语料中）：{len(res['regressions'])}",
        f"- 语料已移除（基线有、语料无，须逐例说明）：{len(res['removed'])}",
        f"- 转译失败：{len(res['transpile_fail'])}",
        f"- 未跑（语料中尚无结论）：{len(res['not_run'])}",
        f"- 无结果（输出无 PASS/FAIL 行，须重跑）：{len(res['no_result'])}",
        f"- 结论：{'✅ 满足删除条件' if satisfied(res) else '❌ 不满足删除条件'}",
    ]
    for key, title in (("regressions", "回归"), ("removed", "语料已移除"),
                       ("transpile_fail", "转译失败"), ("no_result", "无结果"),
                       ("not_run", "未跑")):
        if res[key]:
            lines.append(f"\n{title}：")
            lines.extend(f"- `{n}`" for n in res[key])
    return "\n".join(lines) + "\n"


def main(argv: list[str] | None = None) -> int:
    ap = argparse.ArgumentParser(description=__doc__.split("\n")[0])
    ap.add_argument("--baseline", type=Path, default=DEFAULT_BASELINE)
    src = ap.add_mutually_exclusive_group(required=True)
    src.add_argument("--results", type=Path)
    src.add_argument("--passed", type=Path, nargs="+")
    ap.add_argument("--log", type=Path, nargs="*", default=[])
    ap.add_argument("--commit", default="")
    ap.add_argument("--jdk", type=int, default=21)
    args = ap.parse_args(argv)

    baseline = load_list(args.baseline)
    present = corpus(ROOT)
    not_run: list[str] = []
    no_result: list[str] = []
    commits: dict[str, int] = {}
    if args.results:
        try:
            r = load_results(args.results, args.jdk, {Path(p).stem: p for p in present})
        except InputError as e:
            sys.stderr.write(f"baseline_diff: {e}\n")
            return 2
        passed, tfail, commits = r["passed"], r["tfail"], r["commits"]
        not_run, no_result = r["not_run"], r["no_result"]
    else:
        passed = set()
        for p in args.passed:
            passed |= load_list(p)
        tfail = set()
    for p in args.log:
        tfail |= transpile_failures(p.read_text(encoding="utf-8", errors="replace"))
    res = compare(baseline, passed, tfail, present, not_run, no_result)
    sys.stdout.write(report(res, len(baseline), len(passed), args.commit, commits))
    return 0 if satisfied(res) else 1


if __name__ == "__main__":
    sys.exit(main())
