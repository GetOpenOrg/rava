#!/usr/bin/env bash
# 档案模式顺序无关性（D1）：同一入口集在多组 --flow-batch × --hash-seed 下跑 rava profile，对照档案键与内容摘要。
# 用法：scripts/diag/profile_matrix.sh [--combos "64:0 1:1 7:2 512:12345 4096:0"] [--out 目录] [--tests "A B …"] [-- rava 额外参数]
# 缺省入口集 = scripts/gen_trees.sh 的验收集 27 例。首个组合为基准；各组合的档案写 <out>/profile.bB.sS.json，
# 摘要逐行打印「组合 耗时 key content_digest」，全部相同退出 0。不一致时另用 cj_cmp.py 对照档案 JSON。
# 重命令经 heavy_lock（RAVA_NO_LOCK=1 时直接跑，供服务器作业用）。
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
RAVA="${RAVA:-$ROOT/build/analyzer-target/release/rava}"
combos="64:0 1:1 7:2 512:12345 4096:0"
out="$ROOT/build/d1/profile"
tests="$(sed -n '/^DEFAULT_SET="/,/"$/p' "$ROOT/scripts/gen_trees.sh" | tr -d '"' | sed 's/^DEFAULT_SET=//')"
extra=()
while [ $# -gt 0 ]; do
  case "$1" in
    --combos) combos="$2"; shift 2;;
    --out) out="$2"; shift 2;;
    --tests) tests="$2"; shift 2;;
    --) shift; extra=("$@"); break;;
    *) echo "未知参数 $1" >&2; exit 2;;
  esac
done
mkdir -p "$out"
files=()
for n in $tests; do
  f=$(find "$ROOT/tests/e2e" -name "$n.java" | head -1)
  [ -n "$f" ] || { echo "NOT-FOUND $n" >&2; exit 2; }
  files+=("$f")
done
run() {
  if [ "${RAVA_NO_LOCK:-}" = 1 ]; then "$@"; else python3 /Users/yuwei/dev/workspace/heavy_lock.py "$@"; fi
}
base=""
ok=0
for c in $combos; do
  b="${c%%:*}"; s="${c##*:}"
  f="$out/profile.b$b.s$s.json"
  start=$(date +%s)
  run "$RAVA" profile "${files[@]}" --flow-batch "$b" --hash-seed "$s" -o "$f" ${extra[@]+"${extra[@]}"} > "$out/profile.b$b.s$s.log" 2>&1 || { echo "b$b s$s 失败"; ok=1; continue; }
  sig=$(python3 -c "import json,sys; p=json.load(open(sys.argv[1]))['profile']; print(p['key'], p['content_digest'])" "$f")
  echo "b$b s$s $(( $(date +%s) - start ))s $sig"
  if [ -z "$base" ]; then base="$sig"; elif [ "$sig" != "$base" ]; then ok=1; fi
done
[ $ok = 0 ] && echo "档案一致" || echo "档案不一致"
exit $ok
