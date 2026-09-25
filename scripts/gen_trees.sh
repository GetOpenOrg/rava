#!/usr/bin/env bash
# 生成转译树（--no-run）到 <out_dir>/<Test>/，日志 <out_dir>/<Test>.log。
# 用于「改动前后生成树逐字节一致」验收（配合 scripts/compare_trees.sh）。
#
# 用法：scripts/gen_trees.sh <out_dir> [TestA TestB ...]
#   不给测试名 → 默认验收集（窗口 3 的 23 例 + 流三例 + CompletableFuture）
#   环境变量：REPO（转译用的仓库/worktree，默认本仓库）、JDK（默认 21）
set -u
OUT="${1:?用法: $0 <out_dir> [tests...]}"; shift
DEFAULT_SET="TestTernary TestShortCircuit TestControlFlow TestLabeledBreak TestDoWhile
TestSwitchExpression TestSwitchString TestSwitchFallthrough TestNestedTry TestTryInLoop
TestCasting TestHashMapOps TestChmTransfer TestBitwise TestAtomics TestHoistShadow
TestZonedDateTime TestFilesApi TestDateTimeFormat TestOptionalFull TestSuppressed
TestSynchronized TestArrayList TestStreamBasic TestStreamAdvanced TestStreamCollectors
TestCompletableFuture"
TESTS="${*:-$DEFAULT_SET}"
REPO="${REPO:-$(cd "$(dirname "$0")/.." && pwd)}"
mkdir -p "$OUT"; OUT="$(cd "$OUT" && pwd)"
cd "$REPO"
for n in $TESTS; do
    f=$(find tests/e2e -name "$n.java" | head -1)
    [ -n "$f" ] || { echo "NOT-FOUND $n"; continue; }
    s=$(python3 -c "import re;print(re.sub(r'(?<=[a-z0-9])(?=[A-Z])','_','$n').lower())")
    python3 scripts/main.py "$f" --jdk "${JDK:-21}" --clean --no-run > "$OUT/$n.log" 2>&1 \
        || echo "TRANSPILE-FAIL $n"
    rm -rf "${OUT:?}/$n" && cp -r "build/$s" "$OUT/$n"
    echo "$(grep -h 'raw-audit\|fallback-audit' "$OUT/$n.log" | tr '\n' ' ') $n"
done
