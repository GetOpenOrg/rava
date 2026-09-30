#!/usr/bin/env bash
# 闭包分析器基准（docs/plans/2026-09-30-closure-analyzer-performance.md P0）：
# 基准集串行跑 `rava closure`，记录墙钟 / 峰值 RSS / 上下文数 / 重分析次数，产出对照表。
#
# 用法：
#   scripts/closure_bench.sh <out_dir> [Test ...]        跑基准（缺省 = 四例 + 验收集 27 例）
#   scripts/closure_bench.sh --quick <out_dir>           只跑 HelloWorld Digester CollectorsDemo
#   scripts/closure_bench.sh --diff <base_dir> <new_dir> 逐例对照 closure.json（去掉 summary.elapsed_ms / summary.perf）
# 环境变量：RAVA（分析器二进制，缺省 build/analyzer-target/release/rava）、JDK（缺省 21）
# 同一时间只跑一个分析进程（串行）；输入类目录缓存在 build/perf_in/<Test>/classes。
set -u
REPO="$(cd "$(dirname "$0")/.." && pwd)"; cd "$REPO"
RAVA="${RAVA:-$REPO/build/analyzer-target/release/rava}"
JDKV="${JDK:-21}"
FOUR="HelloWorld Digester DeepCopy CollectorsDemo"
ACCEPT="TestTernary TestShortCircuit TestControlFlow TestLabeledBreak TestDoWhile
TestSwitchExpression TestSwitchString TestSwitchFallthrough TestNestedTry TestTryInLoop
TestCasting TestHashMapOps TestChmTransfer TestBitwise TestAtomics TestHoistShadow
TestZonedDateTime TestFilesApi TestDateTimeFormat TestOptionalFull TestSuppressed
TestSynchronized TestArrayList TestStreamBasic TestStreamAdvanced TestStreamCollectors
TestCompletableFuture"

# closure.json 去掉计时 / 性能观测字段后的规范文本
norm() {
    python3 - "$1" <<'EOF'
import json, sys
d = json.load(open(sys.argv[1], encoding='utf-8'))
d.get('summary', {}).pop('elapsed_ms', None)
d.get('summary', {}).pop('perf', None)
sys.stdout.write(json.dumps(d, indent=1, ensure_ascii=False))
EOF
}

if [ "${1:-}" = "--diff" ]; then
    BASE="${2:?}"; NEW="${3:?}"; bad=0
    for f in "$BASE"/*.json; do
        n=$(basename "$f")
        if [ ! -f "$NEW/$n" ]; then echo "MISSING $n"; bad=1; continue; fi
        if cmp -s <(norm "$f") <(norm "$NEW/$n"); then echo "SAME    $n"; else echo "DIFF    $n"; bad=1; fi
    done
    exit $bad
fi

TESTS=""
if [ "${1:-}" = "--quick" ]; then shift; TESTS="HelloWorld Digester CollectorsDemo"; fi
OUT="${1:?用法: $0 <out_dir> [tests...]}"; shift
[ -n "$TESTS" ] || TESTS="${*:-$FOUR $ACCEPT}"
mkdir -p "$OUT"; OUT="$(cd "$OUT" && pwd)"
HOME_J="$(/usr/libexec/java_home -v "$JDKV" 2>/dev/null || echo "${JAVA_HOME:?}")"
# 镜像独有类 / VM 支持类目录（与 codegen/closure_input.py 同源）
IMAGES=$(python3 -c "
import sys; sys.path.insert(0, '.')
from codegen.jdk_resolver import JdkResolver
print(' '.join('--image ' + d for d in JdkResolver(prefer_major=$JDKV).image_class_dirs()))")

TABLE="$OUT/bench.md"
echo "| 用例 | 墙钟 s | user s | sys s | 峰值 RSS MB | 类 | 方法 | 上下文 | 分析次数 |" > "$TABLE"
echo "|---|---:|---:|---:|---:|---:|---:|---:|---:|" >> "$TABLE"
for n in $TESTS; do
    f=$(find tests/e2e -name "$n.java" | head -1)
    [ -n "$f" ] || { echo "NOT-FOUND $n"; continue; }
    pkg=$(sed -n 's/^package \([a-zA-Z0-9_.]*\);.*/\1/p' "$f" | head -1)
    main="${pkg:+$pkg.}$n"
    cls="build/perf_in/$n/classes"
    if [ ! -d "$cls" ]; then
        mkdir -p "$cls"
        "$HOME_J/bin/javac" -g -d "$cls" "$f" 2>"$OUT/$n.javac" || { echo "JAVAC-FAIL $n"; continue; }
    fi
    # shellcheck disable=SC2086
    /usr/bin/time -l "$RAVA" closure "$cls" --java-home "$HOME_J" --runtime runtime/java_runtime \
        --main "$main" $IMAGES -o "$OUT/$n.json" > "$OUT/$n.summary" 2> "$OUT/$n.time" \
        || { echo "FAIL $n（见 $OUT/$n.time）"; continue; }
    row=$(python3 - "$OUT/$n.time" "$OUT/$n.json" "$n" <<'EOF'
import json, re, sys
t = open(sys.argv[1], encoding='utf-8', errors='replace').read()
g = lambda p: float(re.search(p, t).group(1))
real, user, sys_ = g(r'([\d.]+) real'), g(r'([\d.]+) user'), g(r'([\d.]+) sys')
rss = g(r'(\d+)\s+maximum resident set size') / 2**20
s = json.load(open(sys.argv[2], encoding='utf-8'))['summary']
an = s.get('perf', {}).get('analyses', '—')
print(f"| {sys.argv[3]} | {real:.2f} | {user:.2f} | {sys_:.2f} | {rss:.0f} | {s['classes']} | {s['methods']} | {s['method_contexts']} | {an} |")
EOF
)
    echo "$row" | tee -a "$TABLE"
done
echo "表：$TABLE"
