#!/usr/bin/env bash
# 确定性检查：同一输入转译两次，两次生成逐字节一致
#（集合迭代序等非确定性泄漏进生成代码的回归哨兵）。
#
# 用法：scripts/seed_check.sh <path/to/Test.java> [jdk=21]
set -u
SRC="${1:?用法: $0 <Test.java> [jdk]}"; JDKV="${2:-21}"
REPO="$(cd "$(dirname "$0")/.." && pwd)"; cd "$REPO"
. "$REPO/scripts/rava_env.sh" "$REPO"
n=$(basename "$SRC" .java)
s=$(python3 -c "import re;print(re.sub(r'(?<=[a-z0-9])(?=[A-Z])','_','$n').lower())")
TMP=$(mktemp -d)
for run in 1 2; do
    "$RAVA" build "$SRC" --jdk "$JDKV" --clean --stop-after emit \
        > "$TMP/run$run.log" 2>&1 || { echo "TRANSPILE-FAIL run=$run（见 $TMP）"; exit 1; }
    cp -r "build/$s" "$TMP/run$run"
done
lines=$(diff -r -x Cargo.toml "$TMP/run1" "$TMP/run2" | wc -l)
echo "seed-diff-lines=$lines ($n)"
rm -rf "$TMP"
[ "$lines" = 0 ]
