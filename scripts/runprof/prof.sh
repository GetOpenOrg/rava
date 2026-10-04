#!/usr/bin/env bash
# R1 运行性能剖析（服务器无 perf 权限时用）：发射 + 编译一例，LD_PRELOAD 采样器跑 N 秒并符号化。
# 用法：scripts/runprof/prof.sh <Test.java> [release|debug] [秒数]
# 产物：build/runprof/<snake>_<profile>.txt（报告）与 .time（无采样全程计时，可选 RUNPROF_FULL=1）
set -euo pipefail
cd "$(dirname "$0")/../.."
T=$1; P=${2:-release}; S=${3:-30}
RAVA=build/analyzer-target/release/rava
SN=$(python3 -c "import re,sys;n=sys.argv[1];n=re.sub(r'([A-Z]+)([A-Z][a-z])',r'\1_\2',n);print(re.sub(r'([a-z\d])([A-Z])',r'\1_\2',n).lower())" "$(basename "$T" .java)")
mkdir -p build/runprof
WS=$("$RAVA" build "$T" --stop-after emit 2>&1 | sed -n 's/.*→ \(.*\)（bin.*/\1/p' | tail -1)
[ -n "$WS" ] || { echo "emit 失败"; exit 1; }
REL=(); [ "$P" = release ] && REL=(--release)
"$RAVA" compile "$WS" --target-dir "$PWD/build/runprof-target" "${REL[@]}" > build/runprof/${SN}_${P}_compile.log 2>&1 || { tail -30 build/runprof/${SN}_${P}_compile.log; exit 1; }
BIN=$(python3 -c "import json,sys;print(json.load(open(sys.argv[1]))['exe'])" "$WS/build_status.json")
echo "bin $BIN"
cc -O2 -shared -fPIC -o build/runprof/sampler.so scripts/runprof/sampler.c
set +e
RUNPROF_OUT=build/runprof/${SN}_${P}.raw LD_PRELOAD=$PWD/build/runprof/sampler.so RUNPROF_MAX=$((S*1000)) timeout $((S*3)) "$BIN" > build/runprof/${SN}_${P}.out 2> build/runprof/${SN}_${P}.err
echo "rc=$?" >> build/runprof/${SN}_${P}.out
if [ "${RUNPROF_FULL:-0}" = 1 ]; then
  /usr/bin/time -f "%e s %M KB" -o build/runprof/${SN}_${P}.time timeout 600 "$BIN" > build/runprof/${SN}_${P}.full.out
fi
set -e
touch build/runprof/${SN}_${P}.raw
python3 scripts/runprof/report.py build/runprof/${SN}_${P}.raw "$BIN" > build/runprof/${SN}_${P}.txt
{ echo "--- raw: $(wc -l < build/runprof/${SN}_${P}.raw) 行"; cat build/runprof/${SN}_${P}.err; } >> build/runprof/${SN}_${P}.txt
rm -f build/runprof/${SN}_${P}.raw
head -40 build/runprof/${SN}_${P}.txt
