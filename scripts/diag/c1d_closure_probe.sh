#!/bin/bash
# C1d 闭包诊断：直接跑 `rava closure`（不构建、不发射），配合 --flows 查询。
# 用法：scripts/diag/c1d_closure_probe.sh <out> <Test> <超时秒> [rava closure 额外参数...]
#   例：scripts/diag/c1d_closure_probe.sh w16 StockTrans 900 --flows '@trace:java/util/concurrent/CopyOnWriteArrayList'
# --flows 查询形式（完整说明见 docs/environment-variables.md「rava closure」）：
#   分析后求值：'@path:<节点>|<类>'（值从哪条路径流入）、'@openorig:<类型>|<节点>'（open 值来源）、
#     '@openinj:<类型>'（open 注入点）、'@array'（未知数组写入）、'elem:<数组>'（数组元素值集）
#   记录型（分析前登记、传播中逐条记录）：'@grow:<节点>'（节点每次增长及来源）、'@trace:<类或分配名>' /
#     '@trace:open:<类型>'（获得该值的节点及来源，按到达先后）、'@edge:<节点>'（以该节点为目标新建的流边）
# 输出：/tmp/c1d_probe_<out>.json（closure.json）、.txt（stdout：查询结果与 summary，末行 rc=）、
#   .err（stderr：记录型查询的实时记录 `[flows <查询>] #序号 …`，超时被杀时仍保留已记录部分）
set -u
REPO=$(cd "$(dirname "$0")/../.." && pwd)
out=$1; t=$2; secs=$3; shift 3
cd "$REPO"
f=$(find tests/e2e -name "$t.java" | head -1)
p=/tmp/c1d_probe_$out
timeout "$secs" build/analyzer-target/release/rava closure "$f" "$@" -o "$p.json" > "$p.txt" 2> "$p.err"
echo "rc=$?" >> "$p.txt"
