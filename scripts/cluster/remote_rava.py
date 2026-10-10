"""远端跑单个 rava 子命令（closure / emit / compile）并把结果与错误摘要取回本地：作业模式（--job）的封装。

  uv run --group cluster python scripts/cluster/remote_rava.py closure HelloWorld --ref <sha>            # 闭包：closure.json + 报告
  uv run --group cluster python scripts/cluster/remote_rava.py emit    TestFoo    --ref <sha> [-- --perf --trace-class X]   # 只生成：生成统计
  uv run --group cluster python scripts/cluster/remote_rava.py compile tests/e2e/01_basics/HelloWorld.java --ref <sha>  # 生成 + 编译

阻塞到作业结束；结果在 test_results/remote_rava/<tag>/（缺省 tag = rr-<模式>-<用例>-<sha8>），
最后一行打印摘要文件路径（summary.md）。同一 tag 已成功的作业不重跑（作业模式的续跑语义）；--fresh 强制新 tag。

远端步骤（作业检出目录内）：构建 rava → 取参考 JDK（scripts/fetch_reference_jdk.sh --check）→
  closure：rava closure <T> --java-home JH -o closure.json --report closure.md
  emit：   rava build <T> --stop-after emit    --out <scratch> --clean --java-home JH
  compile：rava build <T> --stop-after emit --out <scratch> --clean --java-home JH && rava compile <scratch>（同 run_tests）
`--` 之后的参数原样追加到 rava 命令行（也可 --args="--perf ..."，须带等号）。产物只取回小文件（rava 输出、*.json / *.md、build.log 截断到 8MB）。
退出码：0 = 作业 rc 0；1 = 作业失败（摘要里有错误块）；2 = 用法 / 解析错误；3 = 作业未完成（infra / 中断）。
"""

import argparse
import fcntl
import json
import os
import re
import shlex
import subprocess
import sys
import time
from pathlib import Path

_HERE = Path(__file__).resolve().parent
sys.path.insert(0, str(_HERE))

import failure_extract as fx   # noqa: E402

SM_ROOT = _HERE.parent
from cluster_config import RESULTS_DIR as RESULTS  # noqa: E402
from cluster_config import REPO_ROOT as JAVA_RTA  # noqa: E402
OUT_REL = "build/remote_rava/out"
SCRATCH_REL = "build/remote_rava/scratch"
MODES = ("closure", "emit", "compile")


def git(*args) -> subprocess.CompletedProcess:
    return subprocess.run(["git", "-C", str(JAVA_RTA), *args], capture_output=True, text=True)


def resolve_sha(ref: str) -> str:
    for attempt in range(2):
        r = git("rev-parse", "--verify", "-q", f"{ref}^{{commit}}")
        if r.returncode == 0:
            return r.stdout.strip()
        if attempt == 0:
            git("fetch", "-q", "origin")
    raise SystemExit(f"--ref {ref} 在 {JAVA_RTA} 中解析不到提交（先推送到 origin）")


def resolve_test(test: str, sha: str) -> str:
    """用例名 / 路径 → 该提交里 tests/e2e 下的相对路径。"""
    files = git("ls-tree", "-r", "--name-only", sha, "tests/e2e").stdout.split()
    if test in files:
        return test
    name = Path(test).name
    if not name.endswith(".java"):
        name += ".java"
    hits = [f for f in files if f.endswith("/" + name)]
    if len(hits) != 1:
        raise SystemExit(f"用例 {test} 在 {sha[:9]} 的 tests/e2e 下{'找不到' if not hits else '不唯一：' + ', '.join(hits)}")
    return hits[0]


def remote_cmd(mode: str, test_path: str, extra: str) -> str:
    """作业命令（在检出目录下以 bash 执行）。rava 自身失败不让整个脚本提前退出：先收集产物再返回 rava 的退出码。"""
    rava = "build/analyzer-target/release/rava"
    if mode == "closure":
        run = f"{rava} closure {shlex.quote(test_path)} --java-home \"$JH\" -o $O/closure.json --report $O/closure.md {extra}"
    elif mode == "emit":
        run = (f"{rava} build {shlex.quote(test_path)} --stop-after emit --out {SCRATCH_REL} --clean "
               f"--java-home \"$JH\" {extra}")
    else:
        # 与 run_tests 同构：先 emit（进程退出释放闭包内存），再 rava compile；单进程 --stop-after compile
        # 会让闭包分析的常驻内存与 rustc 同处一个内存上限（StockTrans 试运行 OOM，tools-rr-compile-5）
        run = (f"{rava} build {shlex.quote(test_path)} --stop-after emit --out {SCRATCH_REL} --clean "
               f"--java-home \"$JH\" {extra} && {rava} compile {SCRATCH_REL}")
    collect = (
        f"S={SCRATCH_REL}; "
        "if [ -d $S ]; then "
        "  for f in $S/*.json $S/*.toml $S/*.md; do [ -f \"$f\" ] && cp \"$f\" $O/; done; "
        "  mkdir -p $O/logs; for f in $S/logs/*; do [ -f \"$f\" ] && head -c 8388608 \"$f\" > $O/logs/$(basename \"$f\"); done; "
        "  find $S -name '*.rs' | wc -l | sed 's/^ */rs_files=/' > $O/scratch_stats.txt; "
        "  du -sk $S | sed 's/\\t.*//;s/^/scratch_kb=/' >> $O/scratch_stats.txt; "
        "fi; "
    )
    return (
        f"O={OUT_REL}; rm -rf build/remote_rava; mkdir -p $O; "
        "cargo build --release -q -p driver --manifest-path generator/Cargo.toml --target-dir build/analyzer-target "
        "> $O/driver_build.log 2>&1 || { tail -60 $O/driver_build.log; echo '[remote_rava] rava 构建失败'; exit 90; }; "
        "JH=$(scripts/fetch_reference_jdk.sh --check) || { echo '[remote_rava] 参考 JDK 未就位'; exit 91; }; "
        f"echo \"[remote_rava] {mode} {test_path} JH=$JH\"; "
        # 作业模式导出了 CARGO_BUILD_JOBS（核数一半）；rava 只在调用方未设置时才对重型 crate 自动单作业编译，
        # 不撤掉会让 StockTrans 这类大闭包在内存上限内被杀（试运行 tools-rr-compile-4 实测）
        f"( unset CARGO_BUILD_JOBS; {run} ) > $O/rava.log 2>&1; rc=$?; "
        + ("" if mode == "closure" else collect)
        + "head -c 4096 $O/rava.log; echo; echo '…'; tail -c 16384 $O/rava.log; "
        "echo \"[remote_rava] rava rc=$rc\"; exit $rc"
    )


def closure_stats(path: Path) -> list[str]:
    try:
        data = json.loads(path.read_text(encoding="utf-8"))
    except Exception as e:
        return [f"closure.json 读取失败：{e}"]
    out = [f"closure.json {path.stat().st_size} 字节"]
    if isinstance(data, dict):
        for k, v in data.items():
            if isinstance(v, (list, dict)):
                out.append(f"  {k}: {len(v)} 项")
            elif isinstance(v, (int, float, str, bool)) and len(str(v)) < 120:
                out.append(f"  {k}: {v}")
    return out


def summarize(mode: str, tag: str, sha: str, test_path: str, job_dir: Path, out: Path) -> tuple[Path, int]:
    summ = json.loads((job_dir / "summary.json").read_text(encoding="utf-8")) if (job_dir / "summary.json").exists() else {}
    job = summ.get("01", {})
    rc, server = job.get("rc"), job.get("server", "?")
    art = job_dir / "01" / OUT_REL
    logs = sorted(job_dir.glob("01_*.log"))
    job_log = logs[-1].read_text(encoding="utf-8", errors="replace") if logs else ""
    lines = [f"# remote_rava {mode} {test_path}", "",
             f"- ref: {sha}", f"- 服务器: {server}，rc={rc}，耗时 {job.get('seconds', '?')}s"
             + (f"，**OOM（{job['oom']}，作业内存上限内被杀，错误块可能缺失）**" if job.get("oom") else "") + ("，超时" if job.get("timeout") else ""),
             f"- 作业目录: {job_dir}", f"- 产物目录: {art}", ""]
    if rc is None or job.get("status") != "done":
        lines += ["作业未完成（infra / 中断）：重跑同一命令即续跑。", "", "```", fx.head_tail(job_log, 8000), "```"]
        code = 3
    else:
        code = 0 if rc == 0 else 1
        if rc in (90, 91):
            lines.append("rava 构建失败" if rc == 90 else "参考 JDK 未就位")
        if mode == "closure" and (art / "closure.json").exists():
            lines += ["## 闭包", "", *closure_stats(art / "closure.json"), f"报告：{art / 'closure.md'}", ""]
        st = art / "scratch_stats.txt"
        if st.exists():
            lines += ["## 生成规模", "", *st.read_text().split(), ""]
        bs = art / "build_status.json"
        if bs.exists():
            lines += ["## build_status.json", "", "```json", bs.read_text(encoding="utf-8")[:6000], "```", ""]
        rlog = (art / "rava.log").read_text(encoding="utf-8", errors="replace") if (art / "rava.log").exists() else ""
        blog = (art / "logs" / "build.log")
        btext = blog.read_text(encoding="utf-8", errors="replace") if blog.exists() else ""
        blocks, bsum = fx.build_error_blocks(rlog + "\n" + btext)
        if blocks:
            lines += [f"## 错误块（{len(blocks)} 个）", "", *bsum, "", "```", fx._cap(blocks, 120_000), "```", ""]
        panics = fx.panic_sections(rlog)
        if panics:
            lines += ["## panic", "", "```", fx._cap(panics, 30_000), "```", ""]
        lines += ["## rava 输出（首尾）", "", "```", fx.head_tail(fx.strip_ansi(rlog), 20_000), "```"]
    out.mkdir(parents=True, exist_ok=True)
    p = out / "summary.md"
    p.write_text("\n".join(lines) + "\n", encoding="utf-8")
    return p, code


def main(argv=None) -> int:
    ap = argparse.ArgumentParser(description="远端跑 rava closure / emit / compile 并取回结果与错误摘要")
    ap.add_argument("mode", choices=MODES)
    ap.add_argument("test", help="用例名（HelloWorld）或 tests/e2e 下相对路径")
    ap.add_argument("--ref", required=True, help="rava 提交（须已推送 origin，服务器从内部仓库取）")
    ap.add_argument("--args", default="", help="追加给 rava 的参数（一个字符串，须写成 --args=\"--perf\"；或放在 -- 之后）")
    ap.add_argument("--tag", help="作业 tag（缺省 rr-<模式>-<用例>-<sha8>）")
    ap.add_argument("--fresh", action="store_true", help="tag 加时间戳，不复用同 tag 的已成功结果")
    ap.add_argument("--servers", nargs="+")
    ap.add_argument("--timeout", type=int, default=3600)
    argv = list(sys.argv[1:] if argv is None else argv)
    passthrough = []
    if "--" in argv:
        i = argv.index("--")
        argv, passthrough = argv[:i], argv[i + 1:]
    a = ap.parse_args(argv)
    if passthrough:
        a.args = " ".join(([a.args] if a.args else []) + [shlex.quote(x) for x in passthrough])
    sha = resolve_sha(a.ref)
    test_path = resolve_test(a.test, sha)
    stem = re.sub(r"[^A-Za-z0-9]", "", Path(test_path).stem)[:24]
    tag = a.tag or f"rr-{a.mode}-{stem}-{sha[:8]}"
    if a.fresh:
        tag += time.strftime("-%m%d%H%M%S")
    cmd = remote_cmd(a.mode, test_path, a.args)
    job_dir = RESULTS / "job" / tag
    job_dir.mkdir(parents=True, exist_ok=True)
    dist_log = job_dir / "remote_rava.out"
    argv_dist = ["uv", "run", "python", "cluster/distribute_tests.py", "--job", tag, "--ref", sha,
                 "--cmd", cmd, "--fetch", f"{OUT_REL}/**", "--job-timeout", str(a.timeout)]
    if a.servers:
        argv_dist += ["--servers", *a.servers]
    # 同 tag 单实例：两个本地实例会争同一份 summary.json 与远端租约（试运行实测：误把第二个实例接到同一租约上）。
    # 已有实例在跑时阻塞等它结束，随后的 distribute_tests 会因作业已成功而跳过，直接出摘要
    lock_fd = os.open(job_dir / "remote_rava.lock", os.O_RDWR | os.O_CREAT, 0o644)
    try:
        fcntl.flock(lock_fd, fcntl.LOCK_EX | fcntl.LOCK_NB)
    except BlockingIOError:
        print(f"[remote_rava] tag {tag} 已有实例在跑，等待其结束后出摘要", file=sys.stderr, flush=True)
        fcntl.flock(lock_fd, fcntl.LOCK_EX)
    print(f"[remote_rava] {a.mode} {test_path} @ {sha[:9]}，tag {tag}，作业日志 {dist_log}", file=sys.stderr, flush=True)
    with dist_log.open("a", encoding="utf-8") as fh:
        p = subprocess.run(argv_dist, cwd=SM_ROOT, stdout=fh, stderr=subprocess.STDOUT)
    if p.returncode != 0:
        print(f"[remote_rava] distribute_tests 退出码 {p.returncode}（见 {dist_log}）", file=sys.stderr)
    path, code = summarize(a.mode, tag, sha, test_path, job_dir, RESULTS / "remote_rava" / tag)
    print(path)
    return code


if __name__ == "__main__":
    sys.exit(main())
