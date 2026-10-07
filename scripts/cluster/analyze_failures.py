"""
分析 rava 分布式测试的失败日志，生成包含完整错误内容的报告。

用法：
  uv run --group cluster python scripts/cluster/analyze_failures.py              # 默认 JDK 21
  uv run --group cluster python scripts/cluster/analyze_failures.py --jdk 25
  uv run --group cluster python scripts/cluster/analyze_failures.py --out report.txt
"""

import re
import argparse
from pathlib import Path
from collections import defaultdict

_HERE       = Path(__file__).parent
from cluster_config import RESULTS_DIR  # noqa: E402
ERROR_DIR   = RESULTS_DIR / "error_logs"

# cargo 编译进度行（对错误分析无价值）
_NOISE_PATTERNS = re.compile(
    r"^\s*(Compiling|Downloading|Downloaded|Updating|Locking|Finished|Blocking|Fresh)\s"
)


def _strip_noise(text: str) -> str:
    """去掉 cargo 进度行，保留 warning / error / 其他有意义的内容。"""
    lines = []
    for line in text.splitlines():
        if _NOISE_PATTERNS.match(line):
            continue
        lines.append(line)
    # 去掉首尾多余空行
    return "\n".join(lines).strip()


def _strip_ansi(text: str) -> str:
    return re.sub(r"\x1b\[[0-9;]*[A-Za-z]", "", text)


# ── 错误类型判断 ───────────────────────────────────────────────────────────────

def _classify(build_text: str, run_text: str, out_text: str = "") -> str:
    """返回 category: oom | compile | run | transpile | mismatch | timeout | crash | empty"""
    # OOM：远端 build.log 有 SIGKILL，或本地输出有 signal 9 / OOM 字样
    if "SIGKILL" in build_text or "signal: 9" in build_text:
        return "oom"
    if "killed by signal 9" in out_text or "疑似 OOM" in out_text:
        return "oom"
    # 超时：本地输出有 timeout 字样
    if "run timeout" in out_text or "build timeout" in out_text:
        return "timeout"
    # 脚本崩溃（Python Traceback）
    if "Traceback (most recent call last)" in out_text:
        return "crash"
    # 转译失败
    if run_text and ("transpile" in run_text.lower() or "translation" in run_text.lower()):
        return "transpile"
    if "transpile error" in out_text:
        return "transpile"
    # 编译失败
    if "error: could not compile" in build_text or "error[E" in build_text:
        return "compile"
    if "compile error" in out_text:
        return "compile"
    # 运行失败
    if run_text.strip():
        return "run"
    if "run error" in out_text:
        return "run"
    # 输出不匹配（diff）
    if "output mismatch" in out_text or "— output mismatch" in out_text:
        return "mismatch"
    return "empty"


# ── 加载失败记录 ───────────────────────────────────────────────────────────────

def _split_log(raw: str) -> tuple[str, str, str]:
    """将合并日志按 === build.log === / === run.log === 分段。
    返回 (out_part, build_part, run_part)。"""
    build_part = ""
    run_part   = ""
    out_part   = raw

    if "=== build.log ===" in raw:
        idx = raw.index("=== build.log ===")
        out_part   = raw[:idx].strip()
        remainder  = raw[idx + len("=== build.log ==="):].strip()
        if "=== run.log ===" in remainder:
            idx2       = remainder.index("=== run.log ===")
            build_part = remainder[:idx2].strip()
            run_part   = remainder[idx2 + len("=== run.log ==="):].strip()
        else:
            build_part = remainder
    elif "=== run.log ===" in raw:
        idx        = raw.index("=== run.log ===")
        out_part   = raw[:idx].strip()
        run_part   = raw[idx + len("=== run.log ==="):].strip()

    return out_part, build_part, run_part


def load_failures(jdk: int) -> list[dict]:
    path = RESULTS_DIR / f"failed_tests_jdk{jdk}.txt"
    if not path.exists():
        return []
    failures = []
    seen = set()
    for line in path.read_text(encoding="utf-8").splitlines():
        line = line.strip()
        if not line:
            continue
        parts = line.split("\t")
        test   = parts[0]
        server = parts[1].strip("[]") if len(parts) > 1 else "?"
        ts     = parts[2].strip("[]") if len(parts) > 2 else "?"
        if test in seen:
            continue
        seen.add(test)

        log_path = ERROR_DIR / f"{test}_{server}_jdk{jdk}.log"
        raw = log_path.read_text(errors="replace") if log_path.exists() else ""

        out_raw, build_raw, run_raw = _split_log(_strip_ansi(raw))
        build_text = _strip_noise(build_raw)
        run_text   = run_raw.strip()
        out_text   = out_raw.strip()

        cat = _classify(build_text, run_text, out_text)

        failures.append({
            "test":       test,
            "server":     server,
            "ts":         ts,
            "cat":        cat,
            "build_text": build_text,
            "run_text":   run_text,
            "out_text":   out_text,
            "log_path":   log_path if log_path.exists() else None,
        })
    return failures


# ── 报告生成 ───────────────────────────────────────────────────────────────────

CAT_LABELS = {
    "oom":       "💥 OOM（编译器被 SIGKILL）",
    "compile":   "🔴 Rust 编译失败",
    "transpile": "🟠 转译失败",
    "run":       "🟡 运行时失败",
    "mismatch":  "🟣 输出不匹配",
    "timeout":   "⏱️  超时",
    "crash":     "🔥 脚本崩溃（Python Traceback）",
    "empty":     "⬜ 日志为空（连接失败/未执行）",
}


def _tail_out(out_text: str, keyword: str, context: int = 30) -> list[str]:
    """从本地完整输出中提取含 keyword 附近的行。"""
    lines = out_text.splitlines()
    for i, l in enumerate(lines):
        if keyword in l:
            start = max(0, i - 5)
            return lines[start: i + context]
    return lines[-context:] if lines else []


def _format_entry(f: dict) -> str:
    sep   = "─" * 72
    lines = []
    lines.append(f"\n{'═'*72}")
    lines.append(f"  {f['test']}  [{f['server']}]  {f['ts']}  {CAT_LABELS.get(f['cat'], f['cat'])}")
    lines.append(sep)

    build = f["build_text"]
    run   = f["run_text"]
    out   = f.get("out_text", "")

    if f["cat"] == "empty":
        lines.append("  （无日志内容，可能测试从未执行或采集阶段出错）")
        if out:
            lines.append("  run_tests.py 输出（末尾 20 行）：")
            for l in out.splitlines()[-20:]:
                lines.append(f"    {l}")

    elif f["cat"] == "oom":
        if build:
            lines.append("  build.log（关键部分）：")
            relevant = [l for l in build.splitlines()
                        if any(k in l for k in ("error", "signal", "SIGKILL", "Caused", "process"))]
            for l in relevant[-10:]:
                if len(l) > 200 and "rustc" in l:
                    l = l[:200] + "  …（省略）"
                lines.append(f"    {l}")
        else:
            lines.append("  run_tests.py 输出（OOM 相关行）：")
            for l in _tail_out(out, "signal 9"):
                lines.append(f"    {l}")

    elif f["cat"] == "timeout":
        lines.append("  run_tests.py 输出（超时信息）：")
        for l in _tail_out(out, "timeout"):
            lines.append(f"    {l}")

    elif f["cat"] == "crash":
        lines.append("  run_tests.py 崩溃 Traceback：")
        for l in _tail_out(out, "Traceback", context=25):
            lines.append(f"    {l}")

    elif f["cat"] == "transpile":
        if run:
            lines.append("  run.log：")
            for l in run.splitlines():
                lines.append(f"    {l}")
        else:
            lines.append("  run_tests.py 输出（转译失败部分）：")
            for l in _tail_out(out, "transpile error"):
                lines.append(f"    {l}")

    elif f["cat"] == "mismatch":
        lines.append("  run_tests.py 输出（diff 部分）：")
        for l in _tail_out(out, "output mismatch", context=50):
            lines.append(f"    {l}")

    else:  # compile / run
        if build:
            lines.append("  build.log：")
            for l in build.splitlines():
                lines.append(f"    {l}")
        if run:
            lines.append("  run.log：")
            for l in run.splitlines():
                lines.append(f"    {l}")
        if not build and not run and out:
            lines.append("  run_tests.py 输出（失败相关行）：")
            keyword = "compile error" if f["cat"] == "compile" else "run error"
            for l in _tail_out(out, keyword):
                lines.append(f"    {l}")

    return "\n".join(lines)


def generate_report(failures: list[dict], jdk: int) -> str:
    groups: dict[str, list[dict]] = defaultdict(list)
    for f in failures:
        groups[f["cat"]].append(f)

    out = []
    out.append(f"rava 失败测试完整报告  JDK {jdk}")
    out.append(f"共 {len(failures)} 个失败测试")
    out.append("=" * 72)

    # 分类汇总
    out.append("\n分类汇总：")
    for cat in ["oom", "compile", "transpile", "run", "mismatch", "timeout", "crash", "empty"]:
        n = len(groups.get(cat, []))
        if n:
            out.append(f"  {CAT_LABELS[cat]}: {n} 个")

    # 按类别展示详情
    for cat in ["compile", "run", "transpile", "mismatch", "timeout", "crash", "oom", "empty"]:
        items = groups.get(cat, [])
        if not items:
            continue
        out.append(f"\n\n{'#'*72}")
        out.append(f"# {CAT_LABELS[cat]}  ({len(items)} 个)")
        out.append(f"{'#'*72}")
        for f in sorted(items, key=lambda x: x["test"]):
            out.append(_format_entry(f))

    return "\n".join(out)


# ── 命令行入口 ─────────────────────────────────────────────────────────────────

def main():
    ap = argparse.ArgumentParser(description="分析 rava 测试失败日志")
    ap.add_argument("--jdk", type=int, default=21)
    ap.add_argument("--out", metavar="FILE", help="输出到文件（默认打印到终端）")
    args = ap.parse_args()

    failures = load_failures(args.jdk)
    if not failures:
        print(f"没有找到 JDK {args.jdk} 的失败记录")
        return

    report = generate_report(failures, args.jdk)
    if args.out:
        Path(args.out).write_text(report, encoding="utf-8")
        print(f"报告已写入 {args.out}（{len(failures)} 个失败）")
    else:
        print(report)


if __name__ == "__main__":
    main()
