#!/usr/bin/env bash
# S7-4 K6 前置项验收（§9.9）：峰值复测——当前检出（s7-k3）对基线 70dd51f8，三例语料
# 逐 crate 单独重建的 /usr/bin/time 峰值（dev 档；方法同 s7k3-fix2：预热依赖后 touch
# 本 crate 源文件、cargo build -p <crate> 只重建该 crate）。
# 用法：scripts/diag/s7k6_peak.sh [基线提交，缺省 70dd51f8] [用例清单，缺省三例]
# 输出：build/s7k6-peak/summary.txt（crate × 基线/新 峰值与百分比表）；退出 0。
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
BASE_REF="${1:-70dd51f8}"
shift || true
TESTS="${*:-HelloWorld CollectorsDemo DeepCopy}"
OUT="$ROOT/build/s7k6-peak"
mkdir -p "$OUT"

JAVA_HOME="${JAVA_HOME:-$(RAVA_REFJDK_ROOT=${RAVA_REFJDK_ROOT:-} scripts/fetch_reference_jdk.sh --check)}"

build_rava() {  # <目录>
  cargo build --release -p driver --manifest-path "$1/generator/Cargo.toml" \
    --target-dir "$1/build/analyzer-target" > "$OUT/driver-build.$(basename "$1").log" 2>&1
}

# <scratch> <tag>：全量预热构建 → 逐 crate touch + 单独重建（/usr/bin/time -v 记峰值）
measure() {
  local scratch="$1" tag="$2" c src root t
  cd "$scratch"
  CARGO_BUILD_JOBS=2 cargo build > "$OUT/$tag.warm.log" 2>&1
  # (crate 名, manifest 路径) 清单一次取全，避免逐 crate 解析 metadata
  while read -r c root; do
    [ -n "$c" ] || continue
    src=$(dirname "$root")/src
    [ -d "$src" ] && find "$src" -maxdepth 1 -name "*.rs" -exec touch {} +
    t=$(/usr/bin/date +%s)
    CARGO_BUILD_JOBS=2 /usr/bin/time -v cargo build -p "$c" > /dev/null 2> "$OUT/$tag.$c.time" || \
      { echo "crate-build-fail $tag $c" >> "$OUT/failures.txt"; continue; }
    rss=$(awk '/Maximum resident set size/ {print $NF}' "$OUT/$tag.$c.time")
    el=$(( $(/usr/bin/date +%s) - t ))
    echo "$tag $c $rss $el" >> "$OUT/rows.txt"
  done < <(cargo metadata --no-deps --format-version 1 2>/dev/null | uv run python3 -c \
      'import json,sys
for p in json.load(sys.stdin)["packages"]:
    print(p["name"], p["manifest_path"])')
}

: > "$OUT/rows.txt"
build_rava "$ROOT"
for t in $TESTS; do
  f=$(find tests/e2e -name "$t.java" | head -1)
  build/analyzer-target/release/rava build "$f" --java-home "$JAVA_HOME" \
    --runtime runtime/java_runtime --clean --stop-after emit --out "$OUT/new.$t" \
    > "$OUT/new.$t.emit.log" 2>&1
  measure "$OUT/new.$t" "new.$t"
done

BASE_WT="$ROOT/../s7k6-base-wt"
git -C "$ROOT" worktree add "$BASE_WT" "$BASE_REF" > /dev/null 2>&1 || git -C "$ROOT" worktree add --force "$BASE_WT" "$BASE_REF" > /dev/null
build_rava "$BASE_WT"
cd "$BASE_WT"
for t in $TESTS; do
  f=$(find tests/e2e -name "$t.java" | head -1)
  build/analyzer-target/release/rava build "$f" --java-home "$JAVA_HOME" \
    --runtime runtime/java_runtime --clean --stop-after emit --out "$OUT/base.$t" \
    > "$OUT/base.$t.emit.log" 2>&1
  measure "$OUT/base.$t" "base.$t"
done

cd "$ROOT"
uv run python3 - "$OUT" $TESTS <<'EOF'
import sys
out, tests = sys.argv[1], sys.argv[2:]
rows = {}
for line in open(f"{out}/rows.txt"):
    tag, c, rss, el = line.split()
    rows[(tag, c)] = (int(rss), int(el))
lines = ["| 用例 | crate | 基线 KB | 新 KB | 变化 | 基线 s | 新 s |", "|---|---|---:|---:|---:|---:|---:|"]
for t in tests:
    crates = sorted({c for (tag, c) in rows if tag == f"base.{t}"})
    for c in crates:
        b = rows.get((f"base.{t}", c))
        n = rows.get((f"new.{t}", c))
        if not (b and n):
            lines.append(f"| {t} | {c} | ? | ? | 缺测 | | |")
            continue
        pct = (n[0] - b[0]) / b[0] * 100 if b[0] else 0
        lines.append(f"| {t} | {c} | {b[0]:,} | {n[0]:,} | {pct:+.1f}% | {b[1]} | {n[1]} |")
print("\n".join(lines))
open(f"{out}/summary.txt", "a").write("\n".join(lines) + "\n")
EOF
echo "peak: done"
