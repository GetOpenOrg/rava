#!/usr/bin/env bash
# rustc 分阶段测量（docs/plans/2026-10-01-rustc-memory-and-crate-split.md §五第 1 步）：
# 对已生成的 scratch 工作区，用 nightly 单独编译 java_runtime 一次，记录
#   - `-Z time-passes`：各阶段耗时与阶段结束时 RSS（stderr → <out>/<name>.passes.log）；
#   - `-Z dump-mono-stats`：单态化实例（<out>/<name>.mono/）；
#   - `/usr/bin/time -l`：峰值 RSS、指令数（<out>/<name>.time.log）；rustc 诊断在 <out>/<name>.stderr.log；
#   - 每 2 s 采样 rustc RSS（<out>/<name>.rss.log）。
# 依赖（parking_lot / libc / rava_macros）先单独编好，不计入测量。
#
# 用法：scripts/rustc_profile.sh <out_dir> <scratch>...   例：build/rp build/hello_world build/deep_copy
# 环境变量：TOOLCHAIN（缺省 nightly）、SELF_PROFILE=1（另加 -Z self-profile，产出 .mm_profdata）
# 重进程：调用前确认没有别的 rustc 在跑；本脚本串行编译，CARGO_BUILD_JOBS=2。
set -u
OUT="${1:?用法: $0 <out_dir> <scratch>...}"; shift
mkdir -p "$OUT"; OUT="$(cd "$OUT" && pwd)"
TC="${TOOLCHAIN:-nightly}"
export CARGO_BUILD_JOBS=2 CARGO_INCREMENTAL=0
for ws in "$@"; do
    ws="$(cd "$ws" && pwd)"; name="$(basename "$ws")"
    export CARGO_TARGET_DIR="$OUT/target"
    (cd "$ws" && cargo +"$TC" build -q -p parking_lot -p libc -p rava_macros) || { echo "DEPS-FAIL $name"; continue; }
    rm -rf "$OUT/$name.mono"; mkdir -p "$OUT/$name.mono"
    extra=""
    if [ "${SELF_PROFILE:-0}" = 1 ]; then
        rm -rf "$OUT/$name.prof"; mkdir -p "$OUT/$name.prof"; extra="-Z self-profile=$OUT/$name.prof"
    fi
    # 强制重编 java_runtime：删掉其指纹
    rm -rf "$CARGO_TARGET_DIR"/debug/.fingerprint/java_runtime-* "$CARGO_TARGET_DIR"/debug/deps/libjava_runtime-*
    (cd "$ws" && exec /usr/bin/time -l -o "$OUT/$name.time.log" cargo +"$TC" rustc -q -p java_runtime --lib -- \
        -Z time-passes -Z dump-mono-stats="$OUT/$name.mono" -Z dump-mono-stats-format=json $extra \
        > /dev/null 2> "$OUT/$name.stderr.log") &
    pid=$!
    : > "$OUT/$name.rss.log"
    while kill -0 "$pid" 2>/dev/null; do
        ps -Ao pid,rss,command | awk -v t="$SECONDS" '/rustc/ && /crate-name java_runtime/ && !/awk/ {print t, int($2/1024)}' >> "$OUT/$name.rss.log"
        sleep 2
    done
    wait "$pid"; rc=$?
    # time-passes 在 stderr；与 rustc 诊断分开存
    grep -E '^time:' "$OUT/$name.stderr.log" > "$OUT/$name.passes.log"
    peak=$(awk '/maximum resident set size/ {printf "%.2f", $1/1073741824}' "$OUT/$name.time.log")
    wall=$(awk '/real/ {print $1}' "$OUT/$name.time.log")
    echo "$name rc=$rc wall=${wall}s peak_rss=${peak}GB"
done
