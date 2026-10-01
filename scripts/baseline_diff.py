#!/usr/bin/env python3
"""全量结果与冻结基线对照：Python 生成器删除判据（docs/plans/2026-10-01-python-generator-deletion.md §一）。

判定：本次是**单一提交**上的全量结果，语料全部有结论（无未跑、无「无结果」），基线通过集 ⊆ 本次
通过集，且本次无转译失败。基线里已不在语料中的用例（删除 / 改名）单列为「语料已移除」，不计回归，
但须在 §五 逐例说明。

输入（二选一）：
  --results    分布式跑批结果目录（server_maintenance/rava/test_results）：读 state_jdk{N}.json 的
               commit / completed / failed，通过清单取 passed_jdk{N}_<commit>.txt，转译失败取
               error_logs/*_jdk{N}.log；未跑完与「no result」都判不满足
  --passed     本次通过清单，可多个（run_tests.py --record-passed 的 passed_tests_jdk21.txt，或
               distribute_tests 的 passed_jdk21_<commit>.txt）；清单格式：每行一个用例路径，# 开头为
               注释；头部带 `commit <sha>` 的清单须同属一个提交，混入多个提交即报错（不接受跨提交并集）
  --log        本次跑批日志，可多个；从中提取转译失败（「— transpile error」行）
  --commit     本次跑批的提交号（清单头部带提交号时可省略，给出则须一致）
  --baseline   冻结基线清单（缺省 docs/plans/2026-10-01-python-baseline-jdk21.txt）
  --jdk        JDK 版本（缺省 21，用于 --results 下的文件名）

输出：Markdown 报告（可直接贴入 §五）；退出码 0 = 满足删除条件，1 = 不满足，2 = 输入口径错误。

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
# distribute_tests 通过清单头部：`# rava 分布式测试通过清单  JDK 21  commit <sha>  <时间>`
COMMIT_RE = re.compile(r"^#.*\bcommit\s+([0-9a-f]{7,40})\b", re.M)
# distribute_tests 对「输出既无 PASS 也无 FAIL 行」的失败原因前缀
NO_RESULT = "no result"


class InputError(Exception):
    """输入口径错误（跨提交、提交号不符、结果目录缺件），退出码 2。"""


def normalize(name: str) -> str:
    """用例名统一为 tests/e2e/ 相对路径（日志行里是 E2E 目录相对路径）。"""
    name = name.strip()
    return name if name.startswith(E2E_PREFIX) else E2E_PREFIX + name


def load_list(path: Path) -> set[str]:
    return {normalize(ln) for ln in path.read_text(encoding="utf-8").splitlines()
            if ln.strip() and not ln.lstrip().startswith("#")}


def list_commit(path: Path) -> str | None:
    m = COMMIT_RE.search(path.read_text(encoding="utf-8"))
    return m.group(1) if m else None


def single_commit(commits: list[str | None], given: str) -> str:
    """清单头部提交号与 --commit 须指向同一提交（前缀匹配），返回完整者。"""
    known = {c for c in commits if c} | ({given} if given else set())
    if not known:
        return ""
    full = max(known, key=len)
    stray = sorted(c for c in known if not full.startswith(c))
    if stray:
        raise InputError(f"通过清单 / --commit 混入多个提交：{sorted(known)}（只接受单一提交的结果）")
    return full


def load_results(results: Path, jdk: int, by_stem: dict[str, str]) -> dict:
    """读取分布式结果目录：通过集、转译失败、未跑、无结果、提交号。"""
    state_path = results / f"state_jdk{jdk}.json"
    if not state_path.exists():
        raise InputError(f"缺少 {state_path}")
    state = json.loads(state_path.read_text(encoding="utf-8"))
    commit = state.get("commit")
    if not commit:
        raise InputError(f"{state_path} 没有 commit 字段：旧版跑批结果是跨提交口径，不能作删除依据")
    passed_file = results / f"passed_jdk{jdk}_{commit[:12]}.txt"
    if not passed_file.exists():
        raise InputError(f"缺少 {passed_file}（可先运行 distribute_tests.py --merge 重写）")
    if list_commit(passed_file) not in (None, commit):
        raise InputError(f"{passed_file.name} 头部提交号与 state 不符")
    passed = load_list(passed_file)
    named = {by_stem.get(n, normalize(n)) for n in state.get("completed", [])}
    if named != passed:
        raise InputError(f"{passed_file.name} 与 state completed 不一致（相差 {len(named ^ passed)} 例），"
                         "先运行 distribute_tests.py --merge 重写")
    failed = state.get("failed", {})
    no_result = {by_stem.get(n, normalize(n)) for n, v in failed.items()
                 if str(v.get("error", "")).startswith(NO_RESULT)}
    ran = set(state.get("completed", [])) | set(failed)
    tfail: set[str] = set()
    err_dir = results / "error_logs"
    for log in sorted(err_dir.glob(f"*_jdk{jdk}.log")) if err_dir.exists() else ():
        text = log.read_text(encoding="utf-8", errors="replace")
        head = text.split("\n", 1)[0]
        m = re.search(r"\bgit:\s*([0-9a-f]+)", head)
        if m and not commit.startswith(m.group(1)):
            continue  # 旧提交遗留日志不计入
        tfail |= transpile_failures(text)
    return {
        "commit": commit,
        "passed": passed,
        "tfail": tfail,
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
        "regressions": sorted(missing & present),
        "removed": sorted(missing - present),
        "transpile_fail": sorted(tfail),
        "not_run": sorted(not_run),
        "no_result": sorted(no_result),
        "new_passes": sorted(passed - baseline),
    }


def satisfied(res: dict[str, list[str]]) -> bool:
    return not any(res[k] for k in ("regressions", "transpile_fail", "not_run", "no_result"))


def report(res: dict[str, list[str]], n_base: int, n_passed: int, commit: str) -> str:
    lines = [
        f"- 提交：`{commit or '（未填）'}`",
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
    try:
        if args.results:
            by_stem = {Path(p).stem: p for p in present}
            r = load_results(args.results, args.jdk, by_stem)
            commit = single_commit([r["commit"]], args.commit)
            passed, tfail = r["passed"], r["tfail"]
            not_run, no_result = r["not_run"], r["no_result"]
        else:
            commit = single_commit([list_commit(p) for p in args.passed], args.commit)
            passed = set()
            for p in args.passed:
                passed |= load_list(p)
            tfail = set()
        for p in args.log:
            tfail |= transpile_failures(p.read_text(encoding="utf-8", errors="replace"))
    except InputError as e:
        sys.stderr.write(f"baseline_diff: {e}\n")
        return 2
    res = compare(baseline, passed, tfail, present, not_run, no_result)
    sys.stdout.write(report(res, len(baseline), len(passed), commit))
    return 0 if satisfied(res) else 1


if __name__ == "__main__":
    sys.exit(main())
