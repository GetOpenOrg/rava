#!/usr/bin/env bash
# 闭包处理顺序无关性矩阵（D1）：同一用例在 --flow-batch × --hash-seed 各组合下跑冷闭包，全键对照（去 via）。
# 用法：scripts/diag/order_matrix.sh <Test.java> [--batches "1 7 64 512 4096"] [--seeds "0 1 2 12345"]
#        [--out <目录>] [--base <基准 json>] [-- <rava closure 额外参数，如 --java-home P / --jdk 21>]
# 输出：<out>/<Test>.b<B>.s<S>.json.gz、<out>/<Test>.times.txt（每次墙钟秒数）与 <out>/<Test>.cmp.txt；
# 集合全同退出 0，否则 1。基准缺省取首个组合。
# 本机经 heavy_lock 跑（设 RAVA_NO_LOCK=1 跳过，服务器用）。
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
RAVA="${RAVA:-$ROOT/build/analyzer-target/release/rava}"
java="$1"; shift
batches="1 7 64 512 4096"
seeds="0 1 2 12345"
out="$ROOT/build/logs/order_matrix"
base=""
extra=()
while [ $# -gt 0 ]; do
  case "$1" in
    --batches) batches="$2"; shift 2;;
    --seeds) seeds="$2"; shift 2;;
    --out) out="$2"; shift 2;;
    --base) base="$2"; shift 2;;
    --) shift; extra=("$@"); break;;
    *) echo "未知参数 $1" >&2; exit 2;;
  esac
done
mkdir -p "$out"
name="$(basename "$java" .java)"
lock=(python3 /Users/yuwei/dev/workspace/heavy_lock.py)
[ "${RAVA_NO_LOCK:-}" = 1 ] && lock=()
files=()
for b in $batches; do
  for s in $seeds; do
    f="$out/$name.b$b.s$s.json"
    start=$(date +%s)
    ${lock[@]+"${lock[@]}"} "$RAVA" closure "$java" --flow-batch "$b" --hash-seed "$s" -o "$f" \
      ${extra[@]+"${extra[@]}"} > "$out/$name.b$b.s$s.log" 2>&1 || echo "[b$b s$s] rava 失败" >&2
    echo "b$b s$s $(( $(date +%s) - start ))" | tee -a "$out/$name.times.txt"
    gzip -f "$f"
    files+=("$f.gz")
  done
done
[ -n "$base" ] || base="${files[0]}"
python3 "$ROOT/scripts/diag/cj_cmp.py" "$base" "${files[@]}" | tee "$out/$name.cmp.txt"
exit "${PIPESTATUS[0]}"
