"""
将 java_rta 指定分支的最新代码合并到本地 main，并推送到所有远端。

流程：
  1. 只 fetch 源分支（不拉 main）
  2. fast-forward 合并到本地 main
  3. 推送到 origin/main 和 github/main

代理：自动读取 HTTPS_PROXY / HTTP_PROXY / ALL_PROXY 环境变量，
      只作用于 fetch / push 等网络操作。

用法：
  uv run --group cluster python scripts/cluster/merge_to_main.py
  uv run --group cluster python scripts/cluster/merge_to_main.py --branch claude/jolly-dijkstra-diftum
  uv run --group cluster python scripts/cluster/merge_to_main.py --dry-run
"""

import os
import sys
import argparse
import subprocess
from pathlib import Path

from cluster_config import REPO_ROOT as JAVA_RTA_DIR  # noqa: E402
DEFAULT_BRANCH = "claude/jolly-dijkstra-diftum"
FETCH_REMOTE   = "github"
PUSH_REMOTES   = ["origin", "github"]


def _detect_proxy() -> str | None:
    for var in ("HTTPS_PROXY", "https_proxy", "HTTP_PROXY", "http_proxy", "ALL_PROXY", "all_proxy"):
        val = os.environ.get(var, "").strip()
        if val:
            return val
    return None


def run(cmd: list[str], check: bool = True, dry: bool = False,
        proxy: str | None = None) -> subprocess.CompletedProcess:
    """执行命令；proxy 非空时在 git 命令中注入 -c http.proxy=。"""
    if proxy and cmd[0] == "git":
        cmd = ["git", "-c", f"http.proxy={proxy}", *cmd[1:]]
    print(f"  $ {' '.join(cmd)}")
    if dry:
        return subprocess.CompletedProcess(cmd, 0, stdout="", stderr="")
    result = subprocess.run(cmd, cwd=JAVA_RTA_DIR, capture_output=True, text=True)
    if result.stdout.strip():
        print(result.stdout.rstrip())
    if result.stderr.strip():
        print(result.stderr.rstrip())
    if check and result.returncode != 0:
        print(f"\n❌ 命令失败 (exit={result.returncode})")
        sys.exit(result.returncode)
    return result


def main():
    ap = argparse.ArgumentParser(description="合并最新分支到 main 并推送")
    ap.add_argument("--branch", default=DEFAULT_BRANCH,
                    help=f"要合并的源分支（默认 {DEFAULT_BRANCH}）")
    ap.add_argument("--dry-run", action="store_true",
                    help="只打印命令，不实际执行")
    args = ap.parse_args()

    dry    = args.dry_run
    branch = args.branch
    proxy  = _detect_proxy()

    if not JAVA_RTA_DIR.exists():
        print(f"❌ 找不到项目目录: {JAVA_RTA_DIR}")
        sys.exit(1)

    if dry:
        print("⚠️  dry-run 模式，以下命令不会实际执行\n")
    if proxy:
        print(f"🌐 检测到代理: {proxy}\n")

    # ── 检查工作区 ────────────────────────────────────────────────────────────
    print("── 检查工作区状态 ──")
    r = run(["git", "status", "--porcelain"], dry=dry)
    if not dry and r.stdout.strip():
        print(f"❌ 工作区有未提交的修改，请先 stash 或 commit：\n{r.stdout}")
        sys.exit(1)
    print("  ✅ 工作区干净")

    # ── 只拉取源分支 ──────────────────────────────────────────────────────────
    print(f"\n── fetch {FETCH_REMOTE}/{branch} ──")
    run(["git", "fetch", FETCH_REMOTE, branch], proxy=proxy, dry=dry)

    # ── 切换到 main 并合并 ────────────────────────────────────────────────────
    print("\n── 切换到 main ──")
    run(["git", "checkout", "main"], dry=dry)

    src_ref = f"{FETCH_REMOTE}/{branch}"
    print(f"\n── 合并 {src_ref} → main ──")
    r = run(["git", "merge", "--ff-only", src_ref], check=False, dry=dry)
    if not dry and r.returncode != 0:
        print("\n❌ fast-forward 合并失败，分支历史存在分叉，需手动处理")
        sys.exit(r.returncode)
    print("  ✅ 合并完成")

    # ── 推送到两个远端 ────────────────────────────────────────────────────────
    print("\n── 推送 main ──")
    for remote in PUSH_REMOTES:
        print(f"\n  → {remote}/main")
        run(["git", "push", remote, "main"], proxy=proxy, dry=dry)
        print(f"  ✅ {remote}/main 推送完成")

    # ── 完成 ──────────────────────────────────────────────────────────────────
    print("\n✅ 全部完成")
    if not dry:
        run(["git", "log", "--oneline", "-5"], dry=False)


if __name__ == "__main__":
    main()
