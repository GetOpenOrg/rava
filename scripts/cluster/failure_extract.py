"""失败日志保全：从服务器拷回的 build.log / run.log / out.log 中摘取定位所需的关键部分。

服务器上的 scratch（含 rustc 全文 build.log）在用例收尾时即被清理，本地 error_logs 若只有
run_tests 打印的首个错误摘要，就无法离线定位。这里在收集阶段把关键内容摘进本地日志：

- build.log：全部 `error[...]` / `error:` 诊断块（含 `-->` 位置、源码上下文行、note / help），
  按出现顺序，去重；
- run.log：全部 panic 消息（`thread '...' panicked at` 及其后的消息行、`stub:` 行）与回溯头部；
- out.log（run_tests 输出）：过长时保留首尾。

每例总量有上限（MAX_TOTAL_BYTES），超出按节截断并注明。
"""

import re

MAX_TOTAL_BYTES = 200 * 1024        # 单例 error_logs 文件上限
MAX_OUT_BYTES = 60 * 1024           # run_tests 输出部分上限（首尾各半）
MAX_BLOCK_LINES = 120               # 单个诊断块行数上限
MAX_BACKTRACE_LINES = 60            # 每个 panic 的回溯头部行数
RAW_KEEP_BYTES = 48 * 1024          # 小于此大小的 run.log 整体保留

_ANSI = re.compile(r"\x1b\[[0-9;]*[A-Za-z]")
# 诊断块起点：error[E0782]: … / error: …（不含 cargo 收尾的 could not compile，那条单独收）
_ERR_START = re.compile(r"^error(\[[A-Z]?\d+\])?: ")
# 块终点：下一个顶格诊断 / cargo 进度行 / rustc 汇总行
_BLOCK_END = re.compile(
    r"^(error|warning)(\[[A-Z]?\d+\])?: |^\s*(Compiling|Checking|Finished|Building|Running|Fresh) "
    r"|^For more information about|^Some errors have detailed|^\{")
_SUMMARY = re.compile(r"^(error: could not compile|error: aborting|For more information about|"
                      r"Some errors have detailed|warning: build failed)")
_PANIC = re.compile(r"^thread '.*' (\(\d+\) )?panicked at ")


def strip_ansi(text: str) -> str:
    return _ANSI.sub("", text)


def build_error_blocks(text: str) -> tuple[list[str], list[str]]:
    """返回 (诊断块列表, 汇总行列表)。块内保留原始缩进与空行（rustc 块以空行结束）。"""
    lines = strip_ansi(text).splitlines()
    blocks: list[str] = []
    summary: list[str] = []
    seen: set[str] = set()
    i = 0
    while i < len(lines):
        ln = lines[i]
        if _SUMMARY.match(ln):
            if ln not in summary:
                summary.append(ln)
            i += 1
            continue
        if not _ERR_START.match(ln):
            i += 1
            continue
        j = i + 1
        while j < len(lines) and not _BLOCK_END.match(lines[j]):
            j += 1
        body = lines[i:j]
        while body and not body[-1].strip():
            body.pop()
        if len(body) > MAX_BLOCK_LINES:
            body = body[:MAX_BLOCK_LINES] + [f"... （本块共 {j - i} 行，截断）"]
        blk = "\n".join(body)
        if blk not in seen:
            seen.add(blk)
            blocks.append(blk)
        i = j
    return blocks, summary


def panic_sections(text: str) -> list[str]:
    """每个 panic：panicked 行 + 消息行（直到回溯或空行）+ 回溯头部 MAX_BACKTRACE_LINES 行。
    另收不在 panic 消息内的 `stub:` 行。"""
    lines = strip_ansi(text).splitlines()
    out: list[str] = []
    covered: set[int] = set()
    i = 0
    while i < len(lines):
        if not _PANIC.match(lines[i]):
            i += 1
            continue
        j = i + 1
        # 消息行：直到 note:/stack backtrace:/下一个 panic
        while j < len(lines) and not (lines[j].startswith("stack backtrace:") or lines[j].startswith("note: ")
                                      or _PANIC.match(lines[j])):
            j += 1
        sec = lines[i:j]
        k = j
        if k < len(lines) and lines[k].startswith("stack backtrace:"):
            end = k + 1
            while end < len(lines) and end - k <= MAX_BACKTRACE_LINES and not _PANIC.match(lines[end]):
                end += 1
            sec += lines[k:end]
            if end < len(lines) and not _PANIC.match(lines[end]):
                sec.append("   ... （回溯其余部分省略）")
            k = end
        elif k < len(lines) and lines[k].startswith("note: "):
            sec.append(lines[k])
            k += 1
        covered.update(range(i, k))
        out.append("\n".join(sec))
        i = max(k, i + 1)
    stray = [ln for n, ln in enumerate(lines) if n not in covered and ln.lstrip().startswith("stub:")]
    if stray:
        out.append("\n".join(dict.fromkeys(stray)))
    return out


def head_tail(text: str, cap: int) -> str:
    data = text.encode()
    if len(data) <= cap:
        return text
    half = cap // 2
    head = data[:half].decode(errors="ignore")
    tail = data[-half:].decode(errors="ignore")
    return f"{head}\n\n... （中间 {len(data) - 2 * half} 字节省略）...\n\n{tail}"


def _cap(parts: list[str], limit: int) -> str:
    """按节累加，超过上限的节截断并注明，其后的节只留标题。"""
    out: list[str] = []
    used = 0
    for p in parts:
        b = len(p.encode())
        if used + b <= limit:
            out.append(p)
            used += b
            continue
        room = max(0, limit - used - 200)
        if room > 1024:
            out.append(p.encode()[:room].decode(errors="ignore") + "\n... （超出单例上限，以下截断）\n")
            used = limit
        else:
            title = p.split("\n", 2)[:2]
            out.append("\n".join(title) + "\n... （超出单例上限，本节省略）\n")
    return "".join(out)


def compose(header: str, out_log: str, build_log: str | None, run_log: str | None,
            build_status: str | None = None, limit: int = MAX_TOTAL_BYTES) -> str:
    """拼出本地 error_logs 文件内容。节顺序：头 → run_tests 输出 → 编译诊断 → panic → build_status。"""
    parts = [header, head_tail(strip_ansi(out_log), MAX_OUT_BYTES)]
    if build_log:
        blocks, summary = build_error_blocks(build_log)
        if blocks or summary:
            parts.append(f"\n\n=== build.log 诊断（error 块 {len(blocks)} 个，全文 {len(build_log.encode())} 字节）===\n"
                         + "\n\n".join(blocks) + ("\n\n" + "\n".join(summary) if summary else "") + "\n")
        else:
            parts.append(f"\n\n=== build.log 尾部（未识别到 error 块）===\n"
                         + head_tail(strip_ansi(build_log)[-16384:], 16384) + "\n")
    if run_log:
        if len(run_log.encode()) <= RAW_KEEP_BYTES:
            parts.append("\n\n=== run.log ===\n" + strip_ansi(run_log) + "\n")
        else:
            secs = panic_sections(run_log)
            body = "\n\n".join(secs) if secs else head_tail(strip_ansi(run_log), 16384)
            parts.append(f"\n\n=== run.log 摘要（panic {len(secs)} 处，全文 {len(run_log.encode())} 字节）===\n"
                         + body + "\n")
    if build_status:
        parts.append("\n\n=== build_status.json ===\n" + build_status[:4096] + "\n")
    return _cap(parts, limit)
