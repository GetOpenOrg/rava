#!/usr/bin/env bash
# 双种子确定性检查：PYTHONHASHSEED=1/2 各转译一次，生成树必须逐字节一致
#（集合迭代序泄漏进生成代码的回归哨兵）。
#
# 用法：scripts/seed_check.sh <path/to/Test.java> [jdk=21]
set -u
SRC="${1:?用法: $0 <Test.java> [jdk]}"; JDKV="${2:-21}"
REPO="$(cd "$(dirname "$0")/.." && pwd)"; cd "$REPO"
n=$(basename "$SRC" .java)
s=$(python3 -c "import re;print(re.sub(r'(?<=[a-z0-9])(?=[A-Z])','_','$n').lower())")
TMP=$(mktemp -d)
for seed in 1 2; do
    PYTHONHASHSEED=$seed python3 scripts/main.py "$SRC" --jdk "$JDKV" --clean --no-run \
        > "$TMP/seed$seed.log" 2>&1 || { echo "TRANSPILE-FAIL seed=$seed（见 $TMP）"; exit 1; }
    cp -r "build/$s" "$TMP/seed$seed"
done
lines=$(diff -r -x Cargo.toml "$TMP/seed1" "$TMP/seed2" | wc -l)
echo "seed-diff-lines=$lines ($n)"
rm -rf "$TMP"
[ "$lines" = 0 ]
