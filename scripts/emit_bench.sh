#!/usr/bin/env bash
# 发射层基准（docs/plans/2026-09-30-emitter-performance.md P0）：基准集串行跑
#   ① `rava build --no-run --perf --clean`（javac + 闭包 + 发射，整条生成管线）；
#   ② `rava emit <closure.json> --perf` 进空目录（只发射，冷写出）；
#   ③ 同一 ② 目录再 emit 一次（热写出：内容不变文件不重写）。
# 每次记录 /usr/bin/time -l 的墙钟 / user / sys / 峰值 RSS 与 `[perf]` 分阶段行，产出对照表。
#
# 用法：scripts/emit_bench.sh <out_dir> [Test ...]   缺省 HelloWorld Digester DeepCopy CollectorsDemo TestCompletableFuture
# 环境变量：RAVA（二进制，缺省 build/analyzer-target/release/rava）、JDK（缺省参考构建 tools/refjdk.toml；JDK=N 改用本机 JDK N，仅供实验）、
#           SKIP_BUILD=1（跳过 ①，复用 <out_dir>/<Test>/closure_input）、
#           EMIT_ARGS（追加给 build / emit 的参数，如 `--emit-jobs 1` 测串行发射）
# 同一时间只跑一个 rava 进程（串行），不跑 cargo。
set -u
REPO="$(cd "$(dirname "$0")/.." && pwd)"; cd "$REPO"
RAVA="${RAVA:-$REPO/build/analyzer-target/release/rava}"
OUT="${1:?用法: $0 <out_dir> [tests...]}"; shift
TESTS="${*:-HelloWorld Digester DeepCopy CollectorsDemo TestCompletableFuture}"
mkdir -p "$OUT"; OUT="$(cd "$OUT" && pwd)"
. "$REPO/scripts/corpus_jdk.sh" "$REPO"
HOME_J="$JAVA_HOME"
# 镜像独有类 / VM 支持类目录由 rava 在未给 --image 时自行派生（resolve::image）
COMMON="--java-home $HOME_J --runtime $REPO/runtime/java_runtime --perf ${EMIT_ARGS:-}"

# /usr/bin/time -l 输出 → "墙钟 user sys RSS_MB 指令G 周期G"（指令数不受机器负载影响，作 A/B 主指标）
tm() {
    awk '/real/ && /user/ {w=$1; u=$3; s=$5} /maximum resident set size/ {r=int($1/1048576)}
         /instructions retired/ {i=$1/1e9} /cycles elapsed/ {c=$1/1e9}
         END {printf "%s %s %s %s %.1f %.1f", w, u, s, r, i, c}' "$1"
}
# [perf] 阶段 name ms
ph() { grep "\[perf\] 阶段 $2 " "$1" | awk '{printf "%.0f", $4}'; }
# 阶段及其全部子项（`name` 与 `name.*`）之和
phs() { grep -E "\[perf\] 阶段 $2(\.[^ ]+)? " "$1" | awk '{s += $4} END {if (NR) printf "%.0f", s}'; }

TABLE="$OUT/bench.md"
{
echo "| 用例 | 模式 | 墙钟 s | user s | sys s | 峰值 RSS MB | 指令 G | 周期 G | closure | input | names+ctx | overlay | classes.prep | classes.imports | classes | p2.impls | p2.inherited | p2.sam | p2.dispatch | write | mod_tree | entry |"
echo "|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|"
} > "$TABLE"
row() { # 用例 模式 log timefile
    read -r w u s r i c <<< "$(tm "$4")"
    echo "| $1 | $2 | $w | $u | $s | $r | $i | $c | $(ph "$3" closure) | $(phs "$3" input) | $(ph "$3" names+ctx) | $(ph "$3" overlay) | $(ph "$3" classes.prep) | $(ph "$3" classes.imports) | $(ph "$3" classes) | $(ph "$3" phase2.impls) | $(ph "$3" phase2.inherited) | $(ph "$3" phase2.sam) | $(ph "$3" phase2.dispatch) | $(ph "$3" write) | $(ph "$3" mod_tree) | $(ph "$3" entry) |" >> "$TABLE"
}
for n in $TESTS; do
    f=$(find tests/e2e -name "$n.java" | head -1)
    [ -n "$f" ] || { echo "NOT-FOUND $n"; continue; }
    f="$REPO/$f"
    S="$OUT/$n"
    if [ -z "${SKIP_BUILD:-}" ]; then
        /usr/bin/time -l "$RAVA" build "$f" $COMMON --out "$S" --clean --no-run --closure-json > "$OUT/$n.build.log" 2> "$OUT/$n.build.time" \
            || { echo "BUILD-FAIL $n"; continue; }
        row "$n" build "$OUT/$n.build.log" "$OUT/$n.build.time"
    fi
    E="$OUT/$n.emit"
    rm -rf "$E"
    for mode in cold warm; do
        /usr/bin/time -l "$RAVA" emit "$S/closure_input/closure.json" --java "$f" $COMMON --out "$E" > "$OUT/$n.$mode.log" 2> "$OUT/$n.$mode.time" \
            || { echo "EMIT-FAIL $n $mode"; continue; }
        row "$n" "emit-$mode" "$OUT/$n.$mode.log" "$OUT/$n.$mode.time"
    done
    echo "done $n"
done
cat "$TABLE"
