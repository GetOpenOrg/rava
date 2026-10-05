#!/usr/bin/env bash
# dev-opt 用户 crate 优化档对照（R1-next SelfNumbers 复核）：同一 scratch 先按 package.user opt-level 0
# 编译运行，再改 1 重编（只重编用户 crate）运行。输出每档的编译耗时、运行耗时 / 峰值内存、二进制大小。
# 用法：scripts/runprof/user_opt_cmp.sh <Test.java>...
set -uo pipefail
cd "$(dirname "$0")/../.."
RAVA=build/analyzer-target/release/rava
mkdir -p build/runprof
for T in "$@"; do
  N=$(basename "$T" .java)
  WS=$("$RAVA" build "$T" --stop-after emit 2>&1 | sed -n 's/.*→ \(.*\)（bin.*/\1/p' | tail -1)
  [ -n "$WS" ] || { echo "$N emit 失败"; continue; }
  for lvl in 0 1; do
    sed -i "/\[profile.dev-opt.package.user\]/{n;s/opt-level = .*/opt-level = $lvl/}" "$WS/Cargo.toml"
    grep -A1 "package.user" "$WS/Cargo.toml" | tr '\n' ' '; echo
    s=$(date +%s.%N)
    "$RAVA" compile "$WS" --dev-opt > build/runprof/${N}_u$lvl.compile.log 2>&1 || tail -20 build/runprof/${N}_u$lvl.compile.log
    e=$(date +%s.%N)
    BIN=$(python3 -c "import json,sys;print(json.load(open(sys.argv[1]))['exe'])" "$WS/build_status.json")
    /usr/bin/time -f "%e s %M KB" -o build/runprof/${N}_u$lvl.time timeout 1200 "$BIN" > build/runprof/${N}_u$lvl.out
    rc=$?
    echo "$N user-opt=$lvl compile $(python3 -c "print(round($e-$s,1))")s run $(cat build/runprof/${N}_u$lvl.time) rc=$rc bin $(stat -c %s "$BIN") out-md5 $(md5sum < build/runprof/${N}_u$lvl.out | cut -c1-8)"
  done
done
