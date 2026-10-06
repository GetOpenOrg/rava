#!/bin/bash
# 服务器作业用：构建 rava 后逐例 `rava build --stop-after closure --closure-json --clean`（参考 JDK），
# 闭包 JSON 压缩落到 build/cj/<tag>/<Test>.json.gz，摘要行（类 / 有代码 / 方法 / 反射调用统计）写 build/cj/<tag>/<Test>.txt。
# 用法：scripts/diag/closure_job.sh <tag> <Test...>；取回：--fetch 'build/cj/<tag>/*'
set -u
REPO=$(cd "$(dirname "$0")/../.." && pwd)
. "$REPO/scripts/rava_env.sh" "$REPO"
. "$REPO/scripts/corpus_jdk.sh" "$REPO"
tag=$1; shift
O=$REPO/build/cj/$tag; mkdir -p "$O"
for n in "$@"; do
  f=$(find "$REPO/tests/e2e" -name "$n.java" | head -1)
  S=$REPO/build/cj_scratch_$n; rm -rf "$S"
  s=$(date +%s)
  "$RAVA" build "$f" "${CORPUS_JDK_ARGS[@]}" --out "$S" --clean --stop-after closure --closure-json > "$O/$n.log" 2>&1; rc=$?
  j=$(find "$S" -name closure.json | head -1)
  if [ -n "$j" ]; then
    python3 - "$j" "$n" "$rc" "$(( $(date +%s) - s ))" > "$O/$n.txt" <<'PY'
import json, collections, sys
p, n, rc, secs = sys.argv[1:]
d = json.load(open(p)); s = d["summary"]
c = collections.Counter(x["level"] for x in d["classes"])
print(n, f"rc={rc} secs={secs}", "classes", len(d["classes"]), "code", c["code"], "methods", len(d["methods"]),
      "fold_props", s.get("fold_props"))
print("rcall", json.dumps(s.get("rcall"), ensure_ascii=False))
PY
    gzip -c "$j" > "$O/$n.json.gz"
  fi
  cat "$O/$n.txt" 2>/dev/null
  rm -rf "$S"
done
