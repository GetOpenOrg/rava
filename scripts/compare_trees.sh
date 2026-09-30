#!/usr/bin/env bash
# 两组转译树逐字节对照（Cargo.toml 除外——包版本按 scratch 唯一化；closure_input/closure.json
# 去掉闭包计时字段 summary.elapsed_ms / summary.perf 后对照）+ raw-audit 计数对照。
#
# 用法：scripts/compare_trees.sh <base_dir> <new_dir>
#   输出每测试 diff 行数；全部为 0 且审计行一致 → exit 0，否则 exit 1。
set -u
BASE="${1:?用法: $0 <base_dir> <new_dir>}"; NEW="${2:?用法: $0 <base_dir> <new_dir>}"
rc=0
# closure.json 去计时字段后规范化输出（缺文件输出 MISSING）
strip_timing() {
    python3 -c "
import json, sys
try:
    j = json.load(open(sys.argv[1]))
except OSError:
    print('MISSING'); sys.exit()
s = j.get('summary', {})
s.pop('elapsed_ms', None); s.pop('perf', None)
print(json.dumps(j, indent=1, sort_keys=True))" "$1"
}
for d in "$BASE"/*/; do
    n=$(basename "$d")
    if [ ! -d "$NEW/$n" ]; then echo -n "$n=MISSING "; rc=1; continue; fi
    c=$(diff -r -x Cargo.toml -x closure.json "$BASE/$n" "$NEW/$n" | wc -l)
    cj="closure_input/closure.json"
    if [ -f "$BASE/$n/$cj" ] || [ -f "$NEW/$n/$cj" ]; then
        c=$((c + $(diff <(strip_timing "$BASE/$n/$cj") <(strip_timing "$NEW/$n/$cj") | wc -l)))
    fi
    echo -n "$n=$c "; [ "$c" = 0 ] || rc=1
done
echo
if diff <(grep -h raw-audit "$BASE"/*.log) <(grep -h raw-audit "$NEW"/*.log) > /dev/null; then
    echo "raw-audit identical"
else
    echo "raw-audit DIFFERS"; rc=1
fi
exit $rc
