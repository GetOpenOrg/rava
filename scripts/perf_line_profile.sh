#!/usr/bin/env bash
# 行级热点剖析（docs/tasks.md「10-11 晚收拢」第 4 条；前置 nogc-perf3 只到符号级）：
#   release 档带行号符号构建（CARGO_PROFILE_RELEASE_DEBUG=line-tables-only，release 默认 strip 关），
#   每例两次采样——F997 无调用栈（自采样按符号 <case>.self.txt 与按源码行 <case>.lines.txt）、
#   F300 dwarf 调用链（children 视图 <case>.children.txt）。产物为文本，perf.data 即删。
#
# 用法（仅服务器作业，经 distribute_tests.py --job 下发；机器需 perf）：
#   scripts/perf_line_profile.sh <out_dir> <java 文件 ...>
# 产物：<out_dir>/<case>.{build.log,self.txt,lines.txt,children.txt}；二进制在 build/target/release/。
set -uo pipefail

REPO="$(cd "$(dirname "$0")/.." && pwd)"
OUT="${1:?用法: $0 <out_dir> <java ...>}"; shift
[ $# -ge 1 ] || { echo "缺 java 文件" >&2; exit 2; }
mkdir -p "$OUT"
. "$REPO/scripts/rava_env.sh" "$REPO"
snake() { python3 -c 'import re,sys; print(re.sub(r"(?<=[a-z0-9])(?=[A-Z])", "_", sys.argv[1]).lower())' "$1"; }

perf --version 2>&1 | head -1
for src in "$@"; do
    c="$(basename "$src" .java)"
    echo "=== $c build（release 带行号符号）"
    CARGO_PROFILE_RELEASE_STRIP=none CARGO_PROFILE_RELEASE_DEBUG=line-tables-only \
        "$RAVA" build "$src" --release --keep-artifacts --stop-after compile > "$OUT/$c.build.log" 2>&1 \
        || { echo "BUILD FAIL $c"; tail -5 "$OUT/$c.build.log"; continue; }
    B="build/target/release/$(snake "$c")"
    [ -x "$B" ] || { echo "NO BIN $c"; tail -5 "$OUT/$c.build.log"; continue; }

    echo "=== $c perf record（F997 自采样）"
    perf record -F 997 -o "$OUT/$c.self.perf" -- "$B" > /dev/null 2>"$OUT/$c.perf.err" \
        || { echo "RECORD FAIL $c"; cat "$OUT/$c.perf.err"; continue; }
    perf report -i "$OUT/$c.self.perf" --no-children --stdio --percent-limit 0.3 -g none 2>/dev/null \
        | grep -v "^#" | grep -v "^$" | head -70 > "$OUT/$c.self.txt" || true
    # 行级：-l 追加源码行（函数名:行号）；line-tables-only 足够行号归属，内联点落在调用行（注记）
    perf report -i "$OUT/$c.self.perf" --no-children --stdio --percent-limit 0.4 -l 2>/dev/null \
        | grep -v "^#" | grep -v "^$" | head -110 > "$OUT/$c.lines.txt" || true

    echo "=== $c perf record（F300 dwarf 调用链）"
    perf record -F 300 --call-graph dwarf,32768 -o "$OUT/$c.cg.perf" -- "$B" > /dev/null 2>>"$OUT/$c.perf.err" || true
    perf report -i "$OUT/$c.cg.perf" --children --stdio --percent-limit 2 -g none 2>/dev/null \
        | grep -v "^#" | grep -v "^$" | head -70 > "$OUT/$c.children.txt" || true
    rm -f "$OUT/$c.self.perf" "$OUT/$c.cg.perf"
    echo "=== $c done"
done
