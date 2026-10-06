#!/bin/bash
# 服务器作业用：构建 rava 后对单例跑 `rava closure`（参考 JDK），附带任意诊断参数（--flows / --why / --cut …）。
# 产物落到 build/cj/<tag>/<Test>.{txt,err}.gz（stdout = 查询结果与 summary，stderr = 记录型查询实时记录）、
# <Test>.json.gz（closure.json）、<Test>.sum（类 / 有代码 / 方法 / fold_props 摘要行）。
# 用法：scripts/diag/closure_probe_job.sh <tag> <Test> [rava closure 参数...]；取回：--fetch 'build/cj/<tag>/*'
set -u
REPO=$(cd "$(dirname "$0")/../.." && pwd)
. "$REPO/scripts/rava_env.sh" "$REPO"
. "$REPO/scripts/corpus_jdk.sh" "$REPO"
tag=$1; n=$2; shift 2
O=$REPO/build/cj/$tag; mkdir -p "$O"
f=$(find "$REPO/tests/e2e" -name "$n.java" | head -1)
C=$(mktemp -d)
s=$(date +%s)
"$RAVA" closure "$f" "${CORPUS_JDK_ARGS[@]}" --closure-cache "$C" -o "$O/$n.json" "$@" > "$O/$n.txt" 2> "$O/$n.err"; rc=$?
rm -rf "$C"
if [ -f "$O/$n.json" ]; then
  python3 - "$O/$n.json" "$n" "$rc" "$(( $(date +%s) - s ))" > "$O/$n.sum" <<'PY'
import json, collections, sys
p, n, rc, secs = sys.argv[1:]
d = json.load(open(p)); s = d["summary"]
c = collections.Counter(x["level"] for x in d["classes"])
print(n, f"rc={rc} secs={secs}", "classes", len(d["classes"]), "code", c["code"], "methods", len(d["methods"]),
      "fold_props", s.get("fold_props"))
PY
  gzip -f "$O/$n.json"
else
  echo "$n rc=$rc no closure.json" > "$O/$n.sum"
fi
gzip -f "$O/$n.txt" "$O/$n.err"
cat "$O/$n.sum"
