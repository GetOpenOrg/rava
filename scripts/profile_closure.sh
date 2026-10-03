#!/usr/bin/env bash
# JDK 档案规模测量（docs/plans/2026-10-01-cross-test-compile-reuse.md §五）：
# 对 e2e 全集按发现序分片，逐例 `rava build --stop-after closure --closure-json`，
# 把 closure.json 压缩存为 <out>/<Test>.json.gz，逐例状态 / 耗时记入 <out>/shard_K.tsv。
#
# 用法：scripts/profile_closure.sh K/N <out_dir>
#   分片与 run_tests.py --batch 同口径（tests/e2e 下 *.java 排序后均分，余量摊给前几片）
#   环境变量：JDK（缺省 21）
# 每例 scratch 用后即删，只留 json.gz；镜像目录（--image）由 rava build 缺省派生，与 rava closure 显式传参同源。
set -u
SHARD="${1:?用法: $0 K/N <out_dir>}"; OUT="${2:?用法: $0 K/N <out_dir>}"
REPO="$(cd "$(dirname "$0")/.." && pwd)"; cd "$REPO"
. "$REPO/scripts/rava_env.sh" "$REPO"
JDKV="${JDK:-21}"
K="${SHARD%/*}"; N="${SHARD#*/}"
mkdir -p "$OUT"; OUT="$(cd "$OUT" && pwd)"
SCR="$REPO/build/profile_scratch_$K"
TSV="$OUT/shard_$K.tsv"
: > "$TSV"
ALL=()
while IFS= read -r f; do ALL+=("$f"); done < <(find tests/e2e -name '*.java' | LC_ALL=C sort)
TOTAL=${#ALL[@]}; Q=$((TOTAL / N)); R=$((TOTAL % N))
START=$(( (K - 1) * Q + (K - 1 < R ? K - 1 : R) ))
LEN=$(( Q + (K <= R ? 1 : 0) ))
echo "[profile] JDK $JDKV 第 $K/$N 片：$LEN / $TOTAL 例"
for f in "${ALL[@]:$START:$LEN}"; do
    n=$(basename "$f" .java)
    rm -rf "$SCR"
    t0=$(python3 -c 'import time;print(time.time())')
    "$RAVA" build "$f" --jdk "$JDKV" --out "$SCR" --clean --stop-after closure --closure-json \
        > "$OUT/$n.log" 2>&1
    rc=$?
    dt=$(python3 -c "import time;print(f'{time.time()-$t0:.2f}')")
    j=$(find "$SCR" -name closure.json 2>/dev/null | head -1)
    if [ "$rc" = 0 ] && [ -n "$j" ]; then
        gzip -c "$j" > "$OUT/$n.json.gz"; rm -f "$OUT/$n.log"; st=ok
    else
        st=fail
    fi
    printf '%s\t%s\t%s\t%s\n' "$n" "$f" "$st" "$dt" >> "$TSV"
done
rm -rf "$SCR"
echo "[profile] 完成：ok=$(grep -c $'\tok\t' "$TSV") fail=$(grep -c $'\tfail\t' "$TSV")"
