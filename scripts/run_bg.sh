#!/usr/bin/env bash
# 后台跑批包装：低内存编译环境 + 先清缓存 + 输出落盘 + 完成标记。
#
# 用法：scripts/run_bg.sh <tag> <command...>
#   输出 → build/logs/bg/<tag>.log，结束写 build/logs/bg/<tag>.done（exit=<码>）
# 例：  scripts/run_bg.sh j21 python3 scripts/run_tests.py --jdk 21 --filter TestAtomics
#
# 低内存环境（16G 机器上大闭包 rustc 峰值 ~14G 会被 OOM 杀）：
#   CARGO_INCREMENTAL=0                      增量元数据双份内存是 OOM 压垮点
#   CARGO_PROFILE_DEV_DEBUG=line-tables-only 调试信息减量（二进制约减半，行为不变）
set -u
tag="${1:?用法: $0 <tag> <command...>}"; shift
REPO="$(cd "$(dirname "$0")/.." && pwd)"
LOGS="$REPO/build/logs/bg"; mkdir -p "$LOGS"
rm -f "$LOGS/$tag.done"
export LANG=C.UTF-8 LC_ALL=C.UTF-8
export CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=line-tables-only
"$REPO/scripts/prune.sh"
cd "$REPO"
"$@" > "$LOGS/$tag.log" 2>&1
echo "exit=$?" > "$LOGS/$tag.done"
