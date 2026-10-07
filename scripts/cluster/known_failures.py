"""已知失败判定：把抽查（spot）结果与 java_rta docs/known_failures.toml 对照。

清单格式见 java_rta docs/known_failures.toml：[[known]] 条目含 test / signature / owner / since / note。
判定：失败用例的本地失败日志包含某条同名条目的 signature 子串 → 已知；否则（含同名但签名不符）→ 新失败。
OOM / 超时 / 无结果（日志里没有签名）自然落入新失败；infra 失败与未跑完的用例单列（未完成）。

清单来源（按序取第一个可用的）：
  1. 显式 --known PATH
  2. java_rta 仓库集成分支上的版本：git -C <java_rta> show rust-closure-analyzer:docs/known_failures.toml
  3. java_rta 主检出工作区里的 docs/known_failures.toml

命令行（在仓库根目录下）：
  uv run --group cluster python scripts/cluster/known_failures.py <spot_tag> [--tests A B ...] [--known PATH] [--json]
  退出码：0 = 完成且无新失败；1 = 有新失败；2 = 未完成（有用例未出结果 / infra 失败）；3 = 用法 / 读取错误
"""

import argparse
import json
import re
import subprocess
import sys
import tomllib
from dataclasses import dataclass, field, asdict
from pathlib import Path

_HERE = Path(__file__).resolve().parent
from cluster_config import RESULTS_DIR  # noqa: E402
from cluster_config import REPO_ROOT as JAVA_RTA  # noqa: E402
INTEG_BRANCH = "rust-closure-analyzer"
KNOWN_REL = "docs/known_failures.toml"
DEFAULT_JDK = 21


@dataclass
class KnownEntry:
    test: str
    signature: str
    owner: str = ""
    since: str = ""
    note: str = ""


def parse_known(text: str) -> list[KnownEntry]:
    data = tomllib.loads(text)
    out = []
    for i, e in enumerate(data.get("known", [])):
        if not e.get("test") or not e.get("signature"):
            raise ValueError(f"known_failures 第 {i + 1} 条缺 test 或 signature")
        out.append(KnownEntry(e["test"], e["signature"], e.get("owner", ""), e.get("since", ""), e.get("note", "")))
    return out


def load_known(path: str | Path | None = None, repo: Path = JAVA_RTA,
               branch: str = INTEG_BRANCH) -> tuple[list[KnownEntry], str]:
    """返回 (条目, 来源描述)。找不到清单时返回空清单（所有失败都算新失败）。"""
    if path:
        p = Path(path)
        return parse_known(p.read_text(encoding="utf-8")), str(p)
    r = subprocess.run(["git", "-C", str(repo), "show", f"{branch}:{KNOWN_REL}"],
                       capture_output=True, text=True)
    if r.returncode == 0:
        return parse_known(r.stdout), f"{repo}@{branch}:{KNOWN_REL}"
    p = repo / KNOWN_REL
    if p.exists():
        return parse_known(p.read_text(encoding="utf-8")), str(p)
    return [], "（未找到 known_failures.toml，全部失败按新失败计）"


# ── 失败摘要 ────────────────────────────────────────────────────────────────────

_FAIL_LINE = re.compile(r"\[FAIL \]\s+\S+\s+—\s+(.*?)(?:\s+\(transpile|\s+\||$)")
_KEY_LINE = re.compile(r"^(error(\[[A-Z]?\d+\])?: |stub: |thread '.*' (\(\d+\) )?panicked at|OOM: |\[panic\] |timeout|[-+][^-+])")


def summarize(log_text: str, max_lines: int = 6) -> str:
    """失败日志的短摘要：FAIL 行的原因 + 首批关键行（error / stub / panic / diff）。"""
    out = []
    m = _FAIL_LINE.search(log_text)
    if m:
        out.append(m.group(1).strip()[:300])
    for ln in log_text.splitlines():
        if _KEY_LINE.match(ln) and ln.strip() not in out:
            out.append(ln.strip()[:300])
        if len(out) >= max_lines:
            break
    return " | ".join(out) if out else "（失败日志无可识别的摘要行）"


def error_logs_for(spot_dir: Path, test: str, jdk: int = DEFAULT_JDK) -> list[Path]:
    d = spot_dir / "error_logs"
    if not d.exists():
        return []
    return sorted(d.glob(f"{test}_*_jdk{jdk}.log"), key=lambda p: p.stat().st_mtime, reverse=True)


# ── 判定 ────────────────────────────────────────────────────────────────────────

@dataclass
class Verdict:
    tag: str
    tests: list[str]
    passed: list[str] = field(default_factory=list)
    known: list[dict] = field(default_factory=list)      # {test, signature, owner, log}
    new: list[dict] = field(default_factory=list)        # {test, reason, summary, log}
    pending: list[str] = field(default_factory=list)     # 未出结果（含租约中）
    infra: list[str] = field(default_factory=list)       # infra 失败（网络 / 执行异常）
    known_source: str = ""

    @property
    def complete(self) -> bool:
        return not self.pending and not self.infra

    @property
    def clean(self) -> bool:
        """完成且失败全部已知。"""
        return self.complete and not self.new

    def line(self) -> str:
        """一行结果：通过 x/y，已知失败 …，新失败 …"""
        s = f"{len(self.passed)}/{len(self.tests)} 通过"
        if self.known:
            s += "，已知失败 " + "、".join(sorted({k["test"] for k in self.known}))
        if self.new:
            s += "，新失败 " + "、".join(n["test"] for n in self.new)
        if self.pending:
            s += f"，未出结果 {len(self.pending)}"
        if self.infra:
            s += f"，infra 失败 {len(self.infra)}"
        return s

    def to_dict(self) -> dict:
        d = asdict(self)
        d.update(complete=self.complete, clean=self.clean, line=self.line())
        return d


def classify(tag: str, tests: list[str] | None, known: list[KnownEntry], known_source: str = "",
             results_dir: Path = RESULTS_DIR, jdk: int = DEFAULT_JDK) -> Verdict:
    spot_dir = results_dir / "spot" / tag
    state_path = spot_dir / f"state_jdk{jdk}.json"
    state = json.loads(state_path.read_text(encoding="utf-8")) if state_path.exists() else {}
    passed, failed = state.get("passed", {}), state.get("failed", {})
    infra = state.get("infra_failed", {})
    if tests is None:
        tests = sorted(set(passed) | set(failed) | set(infra)
                       | {v.get("test") for v in state.get("leases", {}).values() if v.get("test")})
    v = Verdict(tag, sorted(tests), known_source=known_source)
    by_test: dict[str, list[KnownEntry]] = {}
    for e in known:
        by_test.setdefault(e.test, []).append(e)
    for t in v.tests:
        if t in passed:
            v.passed.append(t)
        elif t in failed:
            logs = error_logs_for(spot_dir, t, jdk)
            text = logs[0].read_text(encoding="utf-8", errors="replace") if logs else ""
            hit = next((e for e in by_test.get(t, []) if e.signature in text), None)
            if hit:
                v.known.append({"test": t, "signature": hit.signature, "owner": hit.owner,
                                "log": str(logs[0]) if logs else ""})
            else:
                reason = failed[t].get("error", "")
                summ = summarize(text) if text else "（无失败日志）"
                if by_test.get(t):
                    summ = "同名已知条目签名不符 | " + summ
                v.new.append({"test": t, "reason": reason, "summary": summ,
                              "log": str(logs[0]) if logs else ""})
        elif t in infra:
            v.infra.append(t)
        else:
            v.pending.append(t)
    return v


def main(argv=None) -> int:
    ap = argparse.ArgumentParser(description="抽查结果的已知失败判定")
    ap.add_argument("tag")
    ap.add_argument("--tests", nargs="+", help="期望的用例名单（缺省取 state 里出现过的）")
    ap.add_argument("--known", help="known_failures.toml 路径（缺省读 java_rta 集成分支）")
    ap.add_argument("--jdk", type=int, default=DEFAULT_JDK)
    ap.add_argument("--json", action="store_true")
    ap.add_argument("--results-dir", default=str(RESULTS_DIR), help="test_results 目录（缺省本检出的）")
    a = ap.parse_args(argv)
    try:
        known, src = load_known(a.known)
    except Exception as e:
        print(f"读取已知失败清单失败: {e}", file=sys.stderr)
        return 3
    v = classify(a.tag, a.tests, known, src, Path(a.results_dir), a.jdk)
    if not v.tests:
        # 未给 --tests 且 state 里一个用例都没有（抽查未发起 / tag 写错）：不能当作「无新失败」
        print(f"[{a.tag}] 无抽查结果（{Path(a.results_dir) / 'spot' / a.tag} 下没有任何用例记录）；"
              f"抽查未发起或 tag 有误，请带 --tests 指定名单", file=sys.stderr)
        return 2
    if a.json:
        print(json.dumps(v.to_dict(), ensure_ascii=False, indent=2))
    else:
        print(f"[{a.tag}] {v.line()}   （清单：{src}）")
        for k in v.known:
            print(f"  已知  {k['test']}: {k['signature']}  → {k['owner']}")
        for n in v.new:
            print(f"  新失败 {n['test']}: {n['summary']}\n         {n['log']}")
        for t in v.pending:
            print(f"  未出结果 {t}")
        for t in v.infra:
            print(f"  infra 失败 {t}")
    return 2 if not v.complete else (1 if v.new else 0)


if __name__ == "__main__":
    sys.exit(main())
