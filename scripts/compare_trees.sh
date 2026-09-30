#!/usr/bin/env bash
# 两组转译树逐字节对照（Cargo.toml 除外——包版本按 scratch 唯一化）+ raw-audit 计数对照。
# closure.json 去掉计时 / 内存采样键（键名以 _ms / _mb 结尾：elapsed_ms、perf.phases_ms、
# perf.peak_rss_mb、perf.rss_marks_mb）后按结构对照——这些值每次运行都不同，不是生成结果。
#
# 用法：scripts/compare_trees.sh <base_dir> <new_dir>
#   输出每测试 diff 行数；全部为 0 且审计行一致 → exit 0，否则 exit 1。
set -u
BASE="${1:?用法: $0 <base_dir> <new_dir>}"; NEW="${2:?用法: $0 <base_dir> <new_dir>}"
rc=0
_strip_timing() {
    python3 - "$1" <<'PY'
import json, sys
def strip(v):
    if isinstance(v, dict):
        return {k: strip(x) for k, x in v.items() if not k.endswith(('_ms', '_mb'))}
    if isinstance(v, list):
        return [strip(x) for x in v]
    return v
print(json.dumps(strip(json.load(open(sys.argv[1]))), indent=1, sort_keys=True))
PY
}
for d in "$BASE"/*/; do
    n=$(basename "$d")
    if [ ! -d "$NEW/$n" ]; then echo -n "$n=MISSING "; rc=1; continue; fi
    c=$(( $(diff -r -x Cargo.toml -x closure.json "$BASE/$n" "$NEW/$n" | wc -l) ))
    for j in $(cd "$BASE/$n" && find . -name closure.json); do
        if [ ! -f "$NEW/$n/$j" ]; then c=$((c + 1)); continue; fi
        c=$(( c + $(diff <(_strip_timing "$BASE/$n/$j") <(_strip_timing "$NEW/$n/$j") | wc -l) ))
    done
    echo -n "$n=$c "; [ "$c" = 0 ] || rc=1
done
echo
if diff <(grep -h raw-audit "$BASE"/*.log) <(grep -h raw-audit "$NEW"/*.log) > /dev/null; then
    echo "raw-audit identical"
else
    echo "raw-audit DIFFERS"; rc=1
fi
exit $rc
