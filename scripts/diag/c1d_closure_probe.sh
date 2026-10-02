#!/bin/bash
# C1d 闭包诊断：直接跑 `rava closure`（不构建、不发射），可带 --flows 查询与 C1DR_* 探针环境变量。
# 用法：scripts/diag/c1d_closure_probe.sh <out> <Test> <超时秒> [rava closure 额外参数...]
#   例：C1DR_PROG=1 scripts/diag/c1d_closure_probe.sh w16 StockTrans 900 --flows '@path:<节点>|<类>'
# --flows 查询形式：'@path:<节点>|<类>'（值从哪条路径流入）、'@openorig:<类型>|<节点>'（open 值来源）、
#   '@openinj:<类型>'（open 注入点）、'@array'（未知数组写入）、'elem:<数组>'（数组元素值集）
# C1DR_* 探针（PROG / NOENUM / NOMHARR / GROW=<节点子串> / TRACECLS=<类或分配名> / WATCH）只在 WIP 分支
#   c1d-pick-wip（63d90e86）上存在，见 docs/plans/2026-10-02-c1d-reflect-narrow.md §4.5
# 输出：/tmp/c1dr_<out>.{json,txt}（txt 末行 rc=）
set -u
REPO=$(cd "$(dirname "$0")/../.." && pwd)
out=$1; t=$2; secs=$3; shift 3
cd "$REPO"
f=$(find tests/e2e -name "$t.java" | head -1)
timeout "$secs" build/analyzer-target/release/rava closure "$f" "$@" -o "/tmp/c1dr_$out.json" > "/tmp/c1dr_$out.txt" 2>&1
echo "rc=$?" >> "/tmp/c1dr_$out.txt"
