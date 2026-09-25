#!/usr/bin/env bash
# 两组转译树逐字节对照（Cargo.toml 除外——包版本按 scratch 唯一化）+ raw-audit 计数对照。
#
# 用法：scripts/compare_trees.sh <base_dir> <new_dir>
#   输出每测试 diff 行数；全部为 0 且审计行一致 → exit 0，否则 exit 1。
set -u
BASE="${1:?用法: $0 <base_dir> <new_dir>}"; NEW="${2:?用法: $0 <base_dir> <new_dir>}"
rc=0
for d in "$BASE"/*/; do
    n=$(basename "$d")
    if [ ! -d "$NEW/$n" ]; then echo -n "$n=MISSING "; rc=1; continue; fi
    c=$(diff -r -x Cargo.toml "$BASE/$n" "$NEW/$n" | wc -l)
    echo -n "$n=$c "; [ "$c" = 0 ] || rc=1
done
echo
if diff <(grep -h raw-audit "$BASE"/*.log) <(grep -h raw-audit "$NEW"/*.log) > /dev/null; then
    echo "raw-audit identical"
else
    echo "raw-audit DIFFERS"; rc=1
fi
exit $rc
