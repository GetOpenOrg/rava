#!/usr/bin/env bash
# 同机配对计时（D1 等闭包改造验收）：基准提交与当前工作树的 rava 在同一台机器上交替跑冷闭包，逐轮记墙钟秒数。
# 用法：scripts/diag/pair_time.sh <基准提交> <Test.java> [--rounds N] [--out 目录] [-- rava closure 额外参数]
# 基准源码用 git archive 解到 <out>/base_src（带自己的 runtime/ 与 analyzer-target），不动当前工作树。
# 输出逐行「轮次 base|new 秒数 类数」，末行打印两侧中位数与相对变化。重命令经 heavy_lock（RAVA_NO_LOCK=1 跳过）。
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
base_rev="$1"; java="$(cd "$(dirname "$2")" && pwd)/$(basename "$2")"; shift 2
rounds=3
out="$ROOT/build/d1/pair"
extra=()
while [ $# -gt 0 ]; do
  case "$1" in
    --rounds) rounds="$2"; shift 2;;
    --out) out="$2"; shift 2;;
    --) shift; extra=("$@"); break;;
    *) echo "未知参数 $1" >&2; exit 2;;
  esac
done
lock=(python3 /Users/yuwei/dev/workspace/heavy_lock.py)
[ "${RAVA_NO_LOCK:-}" = 1 ] && lock=()
mkdir -p "$out"
src="$out/base_src"
if [ ! -x "$src/build/analyzer-target/release/rava" ]; then
  rm -rf "$src"; mkdir -p "$src"
  git -C "$ROOT" archive "$base_rev" | tar -x -C "$src"
  ${lock[@]+"${lock[@]}"} cargo build --release -q -p driver --manifest-path "$src/generator/Cargo.toml" --target-dir "$src/build/analyzer-target"
fi
new="$ROOT/build/analyzer-target/release/rava"
old="$src/build/analyzer-target/release/rava"
name="$(basename "$java" .java)"
log="$out/$name.pair.txt"
: > "$log"
for r in $(seq 1 "$rounds"); do
  for side in base new; do
    bin="$new"; [ "$side" = base ] && bin="$old"
    f="$out/$name.$side.json"
    start=$(python3 -c 'import time; print(time.time())')
    ${lock[@]+"${lock[@]}"} "$bin" closure "$java" -o "$f" ${extra[@]+"${extra[@]}"} > "$out/$name.$side.log" 2>&1 || echo "[$side r$r] rava 失败" >&2
    secs=$(python3 -c "import time; print(round(time.time()-$start, 1))")
    n=$(python3 -c "import json,sys; print(len(json.load(open(sys.argv[1]))['classes']))" "$f" 2>/dev/null || echo "?")
    echo "r$r $side $secs $n" | tee -a "$log"
  done
done
python3 - "$log" <<'PY'
import sys, statistics
rows = [l.split() for l in open(sys.argv[1]) if l.strip()]
med = {s: statistics.median(float(r[2]) for r in rows if r[1] == s) for s in ("base", "new")}
print(f"中位数 base {med['base']:.1f}s new {med['new']:.1f}s 变化 {100*(med['new']/med['base']-1):+.1f}%")
PY
