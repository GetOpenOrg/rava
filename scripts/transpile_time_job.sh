#!/usr/bin/env bash
# 按提交测转译（rava build --stop-after emit）耗时的作业脚本（服务器以作业模式运行；本机不跑）。
# 同一台服务器上依次对若干提交各建一个 detached worktree、构建该提交的 rava、对各用例冷跑转译，
# 便于在同一硬件上对照（二分转译耗时回归）。
#
# 用法：scripts/transpile_time_job.sh [--extra "<rava build 额外参数>"] <用例.java 相对路径,...> <提交>...
#   用例以逗号分隔（如 tests/e2e/23_algorithms/DeepCopy.java,tests/e2e/73_jndi_script/TestJndiNoProvider.java）
#   提交须已推送（脚本先 git fetch origin <提交>）
# 产物：build/ttime/<sha8>/<用例>.{out,err,summary.json,classes}（classes 为闭包类名表，供对照）、closure 类数；汇总 build/ttime/summary.tsv
#       作业取回：--fetch 'build/ttime/**'
# 环境：TTIME_TIMEOUT 单例转译上限秒（缺省 3000）
#       TTIME_MODE=closure 改跑 rava closure（--extra 传 --flows / --site-prof 等诊断参数；输出见 <用例>.err）
set -uo pipefail
REPO="$(cd "$(dirname "$0")/.." && pwd)"
OUT="$REPO/build/ttime"
mkdir -p "$OUT"
EXTRA=""
[[ "${1:-}" == --extra ]] && { EXTRA="$2"; shift 2; }
# 额外参数按 shell 引号规则切分（含空格的 --flows 节点标签用引号括起）
eval "XA=($EXTRA)"
[[ $# -ge 2 ]] || { echo "用法：$0 [--extra ARGS] <用例,...> <提交>..."; exit 2; }
IFS=',' read -r -a CASES <<<"$1"; shift
. "$REPO/scripts/corpus_jdk.sh" "$REPO"
TARGET="$REPO/build/ttime-target"
SUM="$OUT/summary.tsv"
[[ -f "$SUM" ]] || printf 'sha\tcase\trc\twall_s\tmaxrss_mb\tclasses\n' >"$SUM"
rc_all=0
for sha in "$@"; do
    git -C "$REPO" fetch -q origin "$sha" 2>/dev/null || true
    full="$(git -C "$REPO" rev-parse --verify "$sha^{commit}")" || { echo "找不到提交 $sha"; rc_all=1; continue; }
    s8="${full:0:8}"
    wt="$REPO/build/ttime-wt/$s8"
    [[ -d "$wt" ]] || git -C "$REPO" worktree add -q --detach "$wt" "$full" || { rc_all=1; continue; }
    echo "═══ $(date '+%H:%M:%S') 构建 rava @$s8"
    t0=$SECONDS
    cargo build --release -q -p driver --manifest-path "$wt/generator/Cargo.toml" --target-dir "$TARGET" \
        || { echo "构建失败 @$s8"; rc_all=1; continue; }
    mkdir -p "$wt/build/bin"
    cp "$TARGET/release/rava" "$wt/build/bin/rava"
    echo "  构建 $((SECONDS - t0))s"
    mkdir -p "$OUT/$s8"
    for c in "${CASES[@]}"; do
        n="$(basename "$c" .java)"
        scratch="$wt/build/ttime-scratch/$n"
        echo "═══ $(date '+%H:%M:%S') 转译 $n @$s8"
        t0=$SECONDS
        if [[ "${TTIME_MODE:-}" == closure ]]; then
            mkdir -p "$scratch/closure_input"
                (cd "$wt" && timeout "${TTIME_TIMEOUT:-3000}" /usr/bin/time -v "$wt/build/bin/rava" closure "$wt/$c" \
                --java-home "$JAVA_HOME" -o "$scratch/closure_input/closure.json" "${XA[@]}") \
                >"$OUT/$s8/$n.out" 2>"$OUT/$s8/$n.err"
        else
        (cd "$wt" && timeout "${TTIME_TIMEOUT:-3000}" /usr/bin/time -v "$wt/build/bin/rava" build "$wt/$c" \
            --stop-after emit --out "$scratch" --clean --closure-json --perf \
            --closure-cache "$wt/build/ttime-cache-$RANDOM" --java-home "$JAVA_HOME" "${XA[@]}") \
            >"$OUT/$s8/$n.out" 2>"$OUT/$s8/$n.err"
        fi
        rc=$?
        wall=$((SECONDS - t0))
        rss="$(grep -E 'Maximum resident' "$OUT/$s8/$n.err" | awk '{printf "%d", $NF/1024}')"
        cls="$(python3 -c '
import json, sys
c = json.load(open(sys.argv[1]))
json.dump(c.get("summary", {}), open(sys.argv[2], "w"), indent=1)
cs = c["classes"]
names = sorted(x if isinstance(x, str) else x.get("name", str(x)) for x in (cs if isinstance(cs, list) else cs.keys()))
open(sys.argv[3], "w").write("\n".join(names) + "\n")
print(len(cs))' "$scratch/closure_input/closure.json" "$OUT/$s8/$n.summary.json" "$OUT/$s8/$n.classes" 2>/dev/null || echo '?')"
        printf '%s\t%s\t%s\t%s\t%s\t%s\n' "$s8" "$n" "$rc" "$wall" "$rss" "$cls" | tee -a "$SUM"
        grep -E '^\s*\[perf\]' "$OUT/$s8/$n.out" "$OUT/$s8/$n.err" 2>/dev/null | head -40 | sed 's/^/  /'
        [[ $rc == 0 ]] || { rc_all=1; tail -5 "$OUT/$s8/$n.err"; }
        rm -rf "$scratch" "$wt"/build/ttime-cache-*
    done
done
echo "═══ 汇总"; cat "$SUM"
exit $rc_all
