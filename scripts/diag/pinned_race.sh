#!/usr/bin/env bash
# a3-T pinned 诊断（服务器作业用）：参考 JDK 与 rava 各跑 PinnedRace（顺序 + 8 进程并行），对照 1d 提前返回率。
# 用法：scripts/diag/pinned_race.sh [rounds=300]
set -uo pipefail
cd "$(dirname "$0")/../.."
R=${1:-300}
OUT=build/vt6; mkdir -p $OUT
JH=$(scripts/fetch_reference_jdk.sh) || exit 1
run_all() {  # $1=标签 其余=命令
  local tag=$1; shift
  for m in intr nointr; do
    echo "== $tag seq $("$@" $R $m)"
    for j in 1 2 3 4 5 6 7 8; do ( "$@" $R $m > $OUT/pr_$j.out 2>&1 ) & done; wait
    for j in 1 2 3 4 5 6 7 8; do echo "== $tag par$j $(cat $OUT/pr_$j.out)"; done
  done
}
mkdir -p $OUT/prc && "$JH/bin/javac" -d $OUT/prc scripts/diag/PinnedRace.java
run_all jdk "$JH/bin/java" -cp $OUT/prc PinnedRace
build/analyzer-target/release/rava build scripts/diag/PinnedRace.java --stop-after compile --out $OUT/pr --clean \
  --java-home "$JH" > $OUT/pr_build.log 2>&1 || { tail -30 $OUT/pr_build.log; exit 1; }
B=$(find build -path "*/debug/pinned_race" -type f | head -1)
run_all rava "$B"
