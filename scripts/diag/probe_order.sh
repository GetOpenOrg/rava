#!/usr/bin/env bash
# 顺序无关性探针：当前运行期 + a8c386e4 的 meta.rs（合并表访问器宏形态，宏展开出的 fn 对闭包分析器不可见，
# 手写层 rt-fn 边丢失后流值断开，曾使 TestSerialLookupPairing 闭包随哈希种子在两个口径间摆动）。
# 引擎的判定若都单调，探针下各种子集合仍须一致。
# 用法：scripts/diag/probe_order.sh [<Test.java>] [--seeds "0 1 2 3 4"]（其余参数透传 seed_sets.sh）
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
java="${1:-$ROOT/tests/e2e/35_io/TestSerialLookupPairing.java}"
[ $# -gt 0 ] && shift
probe="$ROOT/build/probe_order/java_runtime"
rm -rf "$probe"
mkdir -p "$(dirname "$probe")"
cp -R "$ROOT/runtime/java_runtime" "$probe"
git -C "$ROOT" show a8c386e4:runtime/java_runtime/src/meta.rs > "$probe/src/meta.rs"
seeds="0 1 2 3 4"
if [ "${1:-}" = "--seeds" ]; then seeds="$2"; shift 2; fi
exec "$ROOT/scripts/diag/seed_sets.sh" "$java" --runtime "$probe" --seeds "$seeds" --out "$ROOT/build/logs/probe_order" "$@"
