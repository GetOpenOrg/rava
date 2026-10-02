#!/bin/bash
# C1d 闭包实测：构建 rava 后逐例 `rava build --stop-after <阶段> --closure-json --clean`，报类数 / 方法数 / fold_props / 耗时。
# 用法：scripts/diag/c1d_measure.sh <tag> <closure|emit> <Test...>
# 重命令，须经全机锁：python3 <workspace>/heavy_lock.py scripts/diag/c1d_measure.sh ...
# 输出：/tmp/c1d_<tag>.<Test>.{log,json}；不跑 e2e、不编译生成物
set -u
export CARGO_BUILD_JOBS=2
REPO=$(cd "$(dirname "$0")/../.." && pwd); TD=$REPO/build/analyzer-target
tag=$1; stage=$2; shift 2
(cd "$REPO/generator" && cargo build --release -p driver --target-dir "$TD" 2>&1 | grep -E "^error|^warning|Finished" -A6)
cd "$REPO"
for n in "$@"; do
  f=$(find tests/e2e -name "$n.java" | head -1)
  s=$(date +%s)
  timeout 1500 "$TD/release/rava" build "$f" --stop-after "$stage" --closure-json --clean > "/tmp/c1d_$tag.$n.log" 2>&1; rc=$?
  j=$(ls -t build/*/closure_input/closure.json | head -1); cp "$j" "/tmp/c1d_$tag.$n.json"
  python3 - "$tag" "$n" "$rc" "$(( $(date +%s) - s ))" <<'PY'
import json, collections, sys
tag, n, rc, secs = sys.argv[1:]
d = json.load(open(f"/tmp/c1d_{tag}.{n}.json")); s = d["summary"]
c = collections.Counter(x["level"] for x in d["classes"])
print(tag, n, f"rc={rc} secs={secs}", "classes", len(d["classes"]), "code", c["code"], "methods", len(d["methods"]),
      "fold_props", s.get("fold_props"), "all", (s.get("sysprops_unstable") or {}).get("all"))
PY
done
