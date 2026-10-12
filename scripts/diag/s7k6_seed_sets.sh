#!/usr/bin/env bash
# S7-4 K6 前置项验收（§9.9）：语料闭包集合对照——当前检出（s7-k3）对基线 70dd51f8，
# 四用例 × 种子 0/1/2 的类 / 方法 / 反射成员集合逐项比较。
# 用法：scripts/diag/s7k6_seed_sets.sh [基线提交，缺省 70dd51f8] [用例清单，缺省四例]
# 输出：build/s7k6-seed/summary.txt（对照表 + 差异明细）；集合全同退出 0，否则 1。
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
BASE_REF="${1:-70dd51f8}"
shift || true
TESTS="${*:-HelloWorld CollectorsDemo DeepCopy TestSerialLookupPairing}"
OUT="$ROOT/build/s7k6-seed"
mkdir -p "$OUT"

# 参考 JDK（作业环境经 RAVA_REFJDK_ROOT 给根目录；本地主检出缺省走 tools/refjdk）
JAVA_HOME="${JAVA_HOME:-$(RAVA_REFJDK_ROOT=${RAVA_REFJDK_ROOT:-} scripts/fetch_reference_jdk.sh --check)}"

build_rava() {  # <目录>
  cargo build --release -p driver --manifest-path "$1/generator/Cargo.toml" \
    --target-dir "$1/build/analyzer-target" > "$OUT/driver-build.$(basename "$1").log" 2>&1
}

run_sets() {  # <检出目录> <rava> <前缀>
  local dir="$1" rava="$2" tag="$3" t f s
  cd "$dir"
  for t in $TESTS; do
    f=$(find tests/e2e -name "$t.java" | head -1)
    for s in 0 1 2; do
      echo "[closure] $tag $t seed $s"
      "$rava" closure "$f" --java-home "$JAVA_HOME" --runtime "$dir/runtime/java_runtime" \
        --hash-seed "$s" -o "$OUT/$tag.$t.s$s.json" > "$OUT/$tag.$t.s$s.log" 2>&1
    done
  done
}

build_rava "$ROOT"
run_sets "$ROOT" "$ROOT/build/analyzer-target/release/rava" new

BASE_WT="$ROOT/../s7k6-base-wt-$(basename "$ROOT")"  # 每检出目录独立：同服务器多 tag 并行不共用
git -C "$ROOT" worktree add "$BASE_WT" "$BASE_REF" > /dev/null 2>&1 || git -C "$ROOT" worktree add --force "$BASE_WT" "$BASE_REF" > /dev/null
build_rava "$BASE_WT"
run_sets "$BASE_WT" "$BASE_WT/build/analyzer-target/release/rava" base

uv run python3 - "$OUT" $TESTS <<'EOF'
import json, sys
out, tests = sys.argv[1], sys.argv[2:]
def sets(tag, t, s):
    d = json.load(open(f"{out}/{tag}.{t}.s{s}.json"))
    cls = {c if isinstance(c, str) else c["name"] for c in d["classes"]}
    ms = {m["id"] for m in d["methods"]}
    rm = {m["member"] for m in d["reflect"]["members"]}
    return cls, ms, rm
ok = True
lines = ["| 用例 | seed | 基线类/方法/反射 | 新类/方法/反射 | 一致 |", "|---|---|---|---|---|"]
for t in tests:
    for s in "012":
        b, n = sets("base", t, s), sets("new", t, s)
        same = b == n
        ok &= same
        lines.append(f"| {t} | {s} | {len(b[0])}/{len(b[1])}/{len(b[2])} | {len(n[0])}/{len(n[1])}/{len(n[2])} | {'✓' if same else '✗'} |")
        if not same:
            for i, what in enumerate(["类", "方法", "反射成员"]):
                a, c = b[i] - n[i], n[i] - b[i]
                if a or c:
                    lines.append(f"  {t} s{s} {what}差异：仅基线 {sorted(a)[:5]}，仅新 {sorted(c)[:5]}")
    # 跨种子一致性（新侧自身）
    s0 = sets("new", t, "0")
    for s in "12":
        if sets("new", t, s) != s0:
            ok = False
            lines.append(f"  {t}：新侧 seed {s} 与 seed 0 集合不同")
print("\n".join(lines))
open(f"{out}/summary.txt", "w").write("\n".join(lines) + "\n")
sys.exit(0 if ok else 1)
EOF
echo "seed-sets: ALL-IDENTICAL"
