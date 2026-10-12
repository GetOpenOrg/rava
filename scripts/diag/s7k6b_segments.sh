#!/usr/bin/env bash
# S7-4 K6 验收（§9.9）：切段逐类一致——当前检出（s7-k6）对基线的 D8 分段，三例语料各段
# crate 的类文件集合（相对路径）逐项比较；新侧忽略本轮新增的标记文件 `<stem>_m.rs`
#（标记文件是加法，不参与切段；K6 的 D8 分段输入与基线逐字节一致，本脚本实证）。
# 用法：scripts/diag/s7k6b_segments.sh [基线提交，缺省 31e4762d] [用例清单，缺省三例]
# 输出：build/s7k6b-seg/summary.txt（逐段类数对照 + 差异明细 + 新侧标记文件统计）；一致退出 0。
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
BASE_REF="${1:-31e4762d}"
shift || true
TESTS="${*:-HelloWorld CollectorsDemo DeepCopy}"
OUT="$ROOT/build/s7k6b-seg"
mkdir -p "$OUT"

JAVA_HOME="${JAVA_HOME:-$(RAVA_REFJDK_ROOT=${RAVA_REFJDK_ROOT:-} scripts/fetch_reference_jdk.sh --check)}"

build_rava() {  # <目录>
  cargo build --release -p driver --manifest-path "$1/generator/Cargo.toml" \
    --target-dir "$1/build/analyzer-target" > "$OUT/driver-build.$(basename "$1").log" 2>&1
}

# <scratch>：各声明段 crate 的类文件相对路径清单（`<根>_decl*`；排除标记文件 / mod.rs /
# 手写真源——手写文件两侧相同，计入也不影响判定，排除以聚焦类文件）。子壳执行：不改变调用方 cwd
seg_listing() {
  local scratch="$1" c d
  (
    cd "$scratch" || exit 1
    for d in java_base_decl*/src; do
      [ -d "$d" ] || continue
      c=$(basename "$(dirname "$d")")
      find "$d" -name "*.rs" ! -name "mod.rs" ! -name "lib.rs" ! -name "*_m.rs" \
        | sed "s|^$d/||" | sort | sed "s|^|$c |"
    done
  )
}

emit_side() {  # <检出目录> <rava> <tag>
  local dir="$1" rava="$2" tag="$3" t f
  for t in $TESTS; do
    cd "$dir"  # 每用例回到检出目录（seg_listing / rava build 均不改变本 shell cwd，保险起见）
    f=$(find tests/e2e -name "$t.java" | head -1)
    echo "[emit] $tag $t"
    "$rava" build "$f" --java-home "$JAVA_HOME" --runtime "$dir/runtime/java_runtime" \
      --clean --stop-after emit --out "$OUT/$tag.$t" > "$OUT/$tag.$t.emit.log" 2>&1
    seg_listing "$OUT/$tag.$t" > "$OUT/$tag.$t.segs.txt"
    # 新侧标记文件统计（体量数据，供 K6 验收记录；不参与一致性判定；只数声明段树——
    # 用户类的文件名可以 `_m` 结尾（Java 名 FooM），不计入）
    find "$OUT/$tag.$t"/java_base_decl*/src -name "*_m.rs" 2>/dev/null | wc -l \
      > "$OUT/$tag.$t.marker-count.txt"
    find "$OUT/$tag.$t"/java_base_decl*/src -name "*_m.rs" 2>/dev/null | xargs -r du -cb \
      | tail -1 | awk '{print $1}' > "$OUT/$tag.$t.marker-bytes.txt"
  done
}

build_rava "$ROOT"
emit_side "$ROOT" "$ROOT/build/analyzer-target/release/rava" "new"

BASE_WT="$ROOT/../s7k6b-base-wt-$(basename "$ROOT")"  # 每检出目录独立：同服务器多 tag 并行不共用
# 首选新建；路径已有登记的 worktree（作业重试 / 本机复跑）则就地 checkout 到基线提交
if ! git -C "$ROOT" worktree add "$BASE_WT" "$BASE_REF" > /dev/null 2>&1; then
  git -C "$BASE_WT" checkout -q "$BASE_REF" || { echo "base worktree 不可用：$BASE_WT"; exit 1; }
fi
build_rava "$BASE_WT"
emit_side "$BASE_WT" "$BASE_WT/build/analyzer-target/release/rava" "base"

rc=0
{
  echo "| 用例 | 段内类文件 | 基线 | 新 | 一致 |"
  echo "|---|---:|---:|---:|---|"
  for t in $TESTS; do
    if diff -q "$OUT/base.$t.segs.txt" "$OUT/new.$t.segs.txt" > /dev/null; then
      echo "| $t | $(wc -l < "$OUT/base.$t.segs.txt") | — | — | ✓ 逐类一致 |"
    else
      rc=1
      echo "| $t | $(wc -l < "$OUT/base.$t.segs.txt") | $(wc -l < "$OUT/new.$t.segs.txt") | — | ✗ |"
      echo '```diff'
      diff "$OUT/base.$t.segs.txt" "$OUT/new.$t.segs.txt" | head -40
      echo '```'
    fi
    echo "  - 标记文件：$(cat "$OUT/new.$t.marker-count.txt") 个，$(cat "$OUT/new.$t.marker-bytes.txt") 字节"
  done
} | tee "$OUT/summary.txt"
exit $rc
