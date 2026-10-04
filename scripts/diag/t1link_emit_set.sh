#!/bin/bash
# T1 第 2 步实验：建档案并逐个 --profile 发射（只生成）。
# 用法：t1link_emit_set.sh <rava 仓库> <输出目录> <发射的 java（逗号分隔）> <入档案的 java...>
set -u
repo=$1; d=$2; emit=$3; shift 3
cd "$repo"; mkdir -p "$d"
build/analyzer-target/release/rava profile "$@" -o "$d/p.json" --entry-out "$d/entries" > "$d/profile.log" 2>&1 || { tail -3 "$d/profile.log"; exit 1; }
grep '入口' "$d/profile.log" | tail -1
IFS=, read -ra E <<< "$emit"
for j in "${E[@]}"; do n=$(basename "$j" .java)
  build/analyzer-target/release/rava build "$j" --profile "$d/p.json" --stop-after emit --out "$d/$n" --clean > "$d/emit_$n.log" 2>&1 || echo "EMIT FAIL $n"
done
