#!/usr/bin/env bash
# 闭包顺序无关性验收：同一用例换 --hash-seed 跑冷闭包，对照类 / 方法 / 反射成员集合。
# 用法：scripts/diag/seed_sets.sh <Test.java> [--runtime <java_runtime 目录>] [--seeds "0 1 2"] [--out <目录>] [-- <rava closure 额外参数>]
# 输出：<out>/<Test>.s<N>.json 与集合摘要；集合全同退出 0，否则 1。
# 重命令：内部经 heavy_lock 运行 rava closure（单个进程，串行）。
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
RAVA="${RAVA:-$ROOT/build/analyzer-target/release/rava}"
java="$1"; shift
runtime="$ROOT/runtime/java_runtime"
seeds="0 1 2"
out="$ROOT/build/logs/seed_sets"
extra=()
while [ $# -gt 0 ]; do
  case "$1" in
    --runtime) runtime="$2"; shift 2;;
    --seeds) seeds="$2"; shift 2;;
    --out) out="$2"; shift 2;;
    --) shift; extra=("$@"); break;;
    *) echo "未知参数 $1" >&2; exit 2;;
  esac
done
mkdir -p "$out"
name="$(basename "$java" .java)"
for s in $seeds; do
  f="$out/$name.s$s.json"
  start=$(date +%s)
  python3 /Users/yuwei/dev/workspace/heavy_lock.py "$RAVA" closure "$java" --jdk 21 --runtime "$runtime" \
    --hash-seed "$s" -o "$f" ${extra[@]+"${extra[@]}"} > "$out/$name.s$s.log" 2>&1
  echo "[seed $s] $(( $(date +%s) - start )) s"
done
python3 - "$out" "$name" $seeds <<'EOF'
import json, sys
out, name, seeds = sys.argv[1], sys.argv[2], sys.argv[3:]
def sets(s):
    d = json.load(open(f"{out}/{name}.s{s}.json"))
    cls = {c if isinstance(c, str) else c["name"] for c in d["classes"]}
    ms = {m["id"] for m in d["methods"]}
    rm = {m["member"] for m in d["reflect"]["members"]}
    return cls, ms, rm
base = sets(seeds[0])
ok = True
for s in seeds:
    cur = sets(s)
    print(f"seed {s}: 类 {len(cur[0])} / 方法 {len(cur[1])} / 反射成员 {len(cur[2])}")
    for i, what in enumerate(["类", "方法", "反射成员"]):
        a, b = base[i] - cur[i], cur[i] - base[i]
        if a or b:
            ok = False
            print(f"  与 seed {seeds[0]} 的{what}差异：仅 {seeds[0]} {len(a)}，仅 {s} {len(b)}；例 {sorted(a)[:3]} / {sorted(b)[:3]}")
print("集合一致" if ok else "集合不一致")
sys.exit(0 if ok else 1)
EOF
