#!/usr/bin/env bash
# 闭包分析器基准（docs/plans/2026-09-30-closure-analyzer-performance.md P0）：
# 基准集串行跑 `rava closure`，记录墙钟 / 峰值 RSS / 上下文数 / 重分析次数，产出对照表。
#
# 用法：
#   scripts/closure_bench.sh <out_dir> [Test ...]        跑基准（缺省 = 四例 + 验收集 27 例）
#   scripts/closure_bench.sh --quick <out_dir>           只跑 HelloWorld Digester CollectorsDemo
#   scripts/closure_bench.sh --diff <base_dir> <new_dir> 逐例集合对照 closure.json（剔除 via、elapsed_ms、perf；
#                                                      列表按内容排序，顺序不计；差异逐节报告）
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

# closure.json 集合对照（不变量：计划 §二——集合一致，`via` 与条目顺序可变）：
# 剔除 summary.elapsed_ms / summary.perf 与全部 `via`，列表按规范文本排序后逐节比较，报告差异明细
setcmp() {
    python3 - "$1" "$2" <<'EOF2'
import json, sys
def canon(x):
    if isinstance(x, dict):
        return {k: canon(v) for k, v in x.items() if k != 'via'}
    if isinstance(x, list):
        return sorted((canon(v) for v in x), key=lambda v: json.dumps(v, sort_keys=True, ensure_ascii=False))
    return x
def load(p):
    d = json.load(open(p, encoding='utf-8'))
    d.get('summary', {}).pop('elapsed_ms', None)
    d.get('summary', {}).pop('perf', None)
    return canon(d)
def key(v):
    if isinstance(v, dict):
        for k in ('name', 'id', 'site', 'method', 'member'):
            if k in v:
                return v[k]
    return json.dumps(v, sort_keys=True, ensure_ascii=False)
def walk(path, a, b, out):
    if isinstance(a, dict) and isinstance(b, dict):
        for k in sorted(set(a) | set(b)):
            if k not in a or k not in b:
                out.append(f"  {path}.{k}: {'仅新' if k not in a else '仅基线'}")
            elif a[k] != b[k]:
                walk(f"{path}.{k}", a[k], b[k], out)
    elif isinstance(a, list) and isinstance(b, list):
        ka = {json.dumps(v, sort_keys=True, ensure_ascii=False): v for v in a}
        kb = {json.dumps(v, sort_keys=True, ensure_ascii=False): v for v in b}
        ia = {key(v): v for v in a}; ib = {key(v): v for v in b}
        rm = [k for k in ka if k not in kb]; ad = [k for k in kb if k not in ka]
        out.append(f"  {path}: 基线 {len(a)} / 新 {len(b)}，仅基线 {len(rm)}，仅新 {len(ad)}")
        shown = 0
        for k in sorted(set(key(ka[x]) for x in rm) | set(key(kb[x]) for x in ad)):
            if shown >= 8:
                out.append("    …"); break
            va, vb = ia.get(k), ib.get(k)
            tag = '-' if vb is None else '+' if va is None else '~'
            s = k if isinstance(k, str) else json.dumps(k)
            out.append(f"    {tag} {s[:200]}")
            if tag == '~':
                out.append(f"      基线 {json.dumps(va, ensure_ascii=False)[:300]}")
                out.append(f"      新   {json.dumps(vb, ensure_ascii=False)[:300]}")
            shown += 1
    else:
        out.append(f"  {path}: {json.dumps(a)[:120]} → {json.dumps(b)[:120]}")
a, b = load(sys.argv[1]), load(sys.argv[2])
out = []
walk('', a, b, out)
print('\n'.join(out))
sys.exit(1 if out else 0)
EOF2
}

if [ "${1:-}" = "--diff" ]; then
    BASE="${2:?}"; NEW="${3:?}"; bad=0
    for f in "$BASE"/*.json; do
        n=$(basename "$f")
        if [ ! -f "$NEW/$n" ]; then echo "MISSING $n"; bad=1; continue; fi
        if detail=$(setcmp "$f" "$NEW/$n"); then echo "SAME    $n"; else echo "DIFF    $n"; echo "$detail"; bad=1; fi
    done
    exit $bad
fi

TESTS=""
if [ "${1:-}" = "--quick" ]; then shift; TESTS="HelloWorld Digester CollectorsDemo"; fi
OUT="${1:?用法: $0 <out_dir> [tests...]}"; shift
[ -n "$TESTS" ] || TESTS="${*:-$FOUR $ACCEPT}"
mkdir -p "$OUT"; OUT="$(cd "$OUT" && pwd)"
HOME_J="$(/usr/libexec/java_home -v "$JDKV" 2>/dev/null || echo "${JAVA_HOME:?}")"
# 镜像独有类 / VM 支持类目录（与 rava build 缺省派生同一实现：resolve::image）
IMAGES=$("$RAVA" image-dirs --java-home "$HOME_J" --runtime runtime/java_runtime | sed 's/^/--image /' | tr '\n' ' ')

TABLE="$OUT/bench.md"
echo "| 用例 | 墙钟 s | user s | sys s | 峰值 RSS MB | 峰值 footprint MB | 类 | 方法 | 上下文 | 分析次数 |" > "$TABLE"
echo "|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|" >> "$TABLE"
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
# 内存压力下 RSS 会因页压缩 / 换出偏低；footprint 含压缩页，作内存目标的口径
fp = re.search(r'(\d+)\s+peak memory footprint', t)
fp = f"{int(fp.group(1)) / 2**20:.0f}" if fp else '—'
s = json.load(open(sys.argv[2], encoding='utf-8'))['summary']
an = s.get('perf', {}).get('analyses', '—')
print(f"| {sys.argv[3]} | {real:.2f} | {user:.2f} | {sys_:.2f} | {rss:.0f} | {fp} | {s['classes']} | {s['methods']} | {s['method_contexts']} | {an} |")
EOF
)
    echo "$row" | tee -a "$TABLE"
done
echo "表：$TABLE"
