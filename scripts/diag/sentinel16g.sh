#!/usr/bin/env bash
# 16G 哨兵回归集与闭包病理档（T11）——G1 每轮优化后的水位重扫统一入口。
#
# 来源：docs/reports/2026-10-13-scan32g-authoritative-list.md §八（T6 权威清单建议）。
# 两档：
#   sentinel  常规哨兵档（top-10 非病理峰值例）：逐例转译 + 16G tripwire（越线即杀记 FAIL）
#             + 峰值记录。绿转红警报口径：任一例 rc≠0、或峰值超其基线 +25%、或越 16G 线
#             → 汇总行打 `SENTINEL ALERT`；全绿打 `SENTINEL ALL-GREEN`。
#   pathology 闭包病理档（六库 12 例，全部已知转译不收敛）：固定窗口采样——每例 240s /
#             16G 上限的 RSS 曲线（10s 粒度落 TSV），不跑死；输出速率（MiB/s，最小二乘斜率）
#             与曲线哈希，与基线对比：速率差 >±20% → `PATHOLOGY DRIFT`。
#             终局验收（G1 修复后）加 --full：上限 16G→32G、窗口取消（跑完或越线杀，
#             发射完成即认证不超限）。
#
# 用法：
#   scripts/diag/sentinel16g.sh                 # = all（两档顺序跑）
#   scripts/diag/sentinel16g.sh sentinel        # 只跑哨兵档
#   scripts/diag/sentinel16g.sh pathology       # 只跑病理档
#   scripts/diag/sentinel16g.sh all --full      # 32G 认证档（终局验收用）
#   scripts/diag/sentinel16g.sh promote [日期]  # 把 build/sentinel/<日期>/ 的结果提升为
#                                                # docs/sentinel-baselines/ 仓内存档基线
#
# 输出：build/sentinel/<日期>/{sentinel.tsv,pathology.tsv,summary.txt,curves/,logs/}；
#       基线（docs/sentinel-baselines/）随仓库存档，对比规则见该目录 baseline.md。
# 断点续跑（长任务纪律）：两档 TSV 均逐例 append 落盘——中断后重跑同一命令，TSV 里
#       已有行的例自动跳过（汇总行从全表聚合）；env SENTINEL_OUT 可覆盖输出目录（自测用）。
# 内存纪律：顺序执行、一次一例（哨兵档实测单例 ≤8G 量级、病理档窗口 ≤16G）；
#       重进程（run_tests / lib_pilot_golden）一律经全机重进程锁 heavy_lock。
# 断言口径（gatefix 教训）：set -euo pipefail；结论只从落盘文本行判（RESULT JSON /
#       汇总行 grep），rc 仅作辅助。
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
HEAVY="${HEAVY_LOCK:-python3 /data/workspace/heavy_lock.py}"   # 全机重进程锁（缺省开）
SAMPLER="$ROOT/scripts/diag/sentinel_sampler.py"
BASEDIR="$ROOT/docs/sentinel-baselines"
DATE="$(date +%F)"
OUT="${SENTINEL_OUT:-$ROOT/build/sentinel/$DATE}"   # env 覆盖（自测 / 定向续跑）
LIMIT_GIB=16          # tripwire / 病理窗口上限
WINDOW_S=240          # 病理档固定窗口
MODE_ARG="${1:-all}"
shift || true
FULL=0
for a in "$@"; do
    [[ "$a" == "--full" ]] && FULL=1
done
if [[ "$FULL" == 1 ]]; then
    LIMIT_GIB=32
    WINDOW_S=0        # 认证档不设窗口：跑完（认证不超限）或越 32G 杀
fi

# ── 名单（T6 权威清单 §八.2 / §四）───────────────────────────────────────────
# 哨兵档 top-10 非病理峰值例：9 e2e（双基线 top 峰值）+ 1 pilot（HoconParseMain）
SENTINEL_E2E=(
    "52_lang_features/TestSystemStableProps"
    "71_xml/TestXmlTransform"
    "71_xml/TestSaxNamespaceCallbacks"
    "72_http/TestSystemProxySelector"
    "23_algorithms/DataEncryptionStandard"
    "23_algorithms/TestAesGcmRound"
    "23_algorithms/TestCipherDesModes"
    "23_algorithms/TestCipherAlgorithmParameters"
    "73_jndi_script/TestJndiNoProvider"
)
SENTINEL_PILOT=(HoconParseMain)
# 闭包病理档 12 例（六库，转译不收敛，速率基线对比不判 rc）
PATHOLOGY=(
    OgnlCollectionMain OgnlContextMain OgnlEvalMain
    YamlRoundTripMain YamlTypesMain
    FastjsonPathMain FastjsonStreamMain
    DatabindAnnotationMain DatabindDeserMain DatabindSerMain
    SpringProxyJdkMain
)
PATHOLOGY_ESPER="esper/com.example.esper.EsperLengthWindowDemo"   # 项目面 FQCN 入口

# ── 小工具 ──────────────────────────────────────────────────────────────────
log()  { printf '%s %s\n' "$(date +%H:%M:%S)" "$*"; }
tagify() { printf '%s' "$1" | tr '/' '_'; }

# 读采样器 RESULT JSON 的一个字段（断言判文本：从落盘行 grep，不靠退出码）
json_field() { # <file> <key>
    python3 -c 'import json,sys
for ln in open(sys.argv[1], encoding="utf-8"):
    if ln.startswith("RESULT "):
        print(json.loads(ln[7:])[sys.argv[2]]); break
else: print("")' "$1" "$2"
}

# 基线 → 关联数组（文件缺行即无该例基线：记 BASELINE-SET，不作警报）
declare -A SENT_BASE=() PATH_BASE=()
load_baselines() {
    [[ -f "$BASEDIR/sentinel-baseline.tsv" ]] || return 0
    while IFS=$'\t' read -r face case peak _rc _out _ver _date; do
        [[ "$face" == "face" ]] && continue
        SENT_BASE["$face|$case"]="$peak"
    done < "$BASEDIR/sentinel-baseline.tsv"
    [[ -f "$BASEDIR/pathology-baseline.tsv" ]] || return 0
    while IFS=$'\t' read -r case rate _fin _out _sha _ver _date _win; do
        [[ "$case" == "case" ]] && continue
        PATH_BASE["$case"]="$rate"
    done < "$BASEDIR/pathology-baseline.tsv"
}

# 后台起采样器；转译命令结束后收结果。
#   run_sample <case-tag> <window_s> <interval_s> [grace_s] → $OUT/logs/<tag>.sampler（写 RESULT 行）
#   grace 缺省 600：须覆盖 heavy_lock 排队 + JVM golden 前置段——过短会在命令尚未进转译段时
#   误报 no-run 并使该例全程脱离 tripwire（采样器已退出）。
run_sample() {
    local tag="$1" window="$2" interval="$3" grace="${4:-600}"
    : > "$OUT/logs/$tag.sampler"
    python3 "$SAMPLER" --worktree "$ROOT" --limit-gib "$LIMIT_GIB" \
        --interval "$interval" --window-s "$window" --quiet-grace "$grace" \
        --tsv "$OUT/curves/$tag.tsv" >> "$OUT/logs/$tag.sampler" 2>&1 &
    SAMPLE_PID=$!
}
join_sample() { # <case-tag>（阻塞收 RESULT；采样器自身异常 → verdict 判 NO-SAMPLE）
    local tag="$1"
    wait "$SAMPLE_PID" || true
}

# 跑一条重命令（heavy_lock 包装），rc 落 $RUN_RC（不触发 set -e）
run_heavy() {
    if $HEAVY "$@" > "$OUT/logs/$CUR_TAG.log" 2>&1; then RUN_RC=0; else RUN_RC=$?; fi
}

# ── 逐例落盘 TSV（长任务纪律：每完成一例立即 append，中断可再续）──────────────
# 用法：init_tsv <file> <header>；append_row <file> <row>；
#       tsv_done <file> <键列>（输出已落盘键，供续跑跳过）；tsv_stat <file> <verdict列>
#       → "<fail数> <set数> <总行数>"（fail = verdict 以 FAIL/DRIFT 开头）
SENT_TSV="$OUT/sentinel.tsv"
PATH_TSV="$OUT/pathology.tsv"
init_tsv()  { [[ -s "$1" ]] || printf '%s\n' "$2" > "$1"; }
append_row() { printf '%s\n' "$2" >> "$1"; }
tsv_done()  { local f="$1" expr="$2"
              [[ -s "$f" ]] && awk -F'\t' "NR>1 && \$0 !~ /^#/{print $expr}" "$f" || true; }
tsv_stat()  { awk -F'\t' -v v="$2" 'NR>1 && $0 !~ /^#/{
                  n++; if ($v ~ /^(FAIL|DRIFT)/) f++; else if ($v == "BASELINE-SET") s++
              } END{printf "%d %d %d", f+0, s+0, n+0}' "$1"; }

# ── 哨兵档 ──────────────────────────────────────────────────────────────────
run_sentinel() {
    log "== 哨兵档：${#SENTINEL_E2E[@]} e2e + ${#SENTINEL_PILOT[@]} pilot，tripwire ${LIMIT_GIB}G =="
    local -A DONE=()
    local k
    while IFS= read -r k; do [[ -n "$k" ]] && DONE["$k"]=1; done < <(tsv_done "$SENT_TSV" '$1"|"$2')
    init_tsv "$SENT_TSV" $'face\tcase\trc\toutcome\tpeak_gib\tbaseline_gib\tverdict\tdur_s'
    local face case tag peak base verdict dur sampler
    for entry in "${SENTINEL_E2E[@]}" ; do
        face=e2e; case="$entry"; tag="$(tagify "$case")"; CUR_TAG="sent-$tag"
        if [[ -n "${DONE[$face|$case]:-}" ]]; then log "跳过 e2e $case（TSV 已落盘，续跑）"; continue; fi
        log "哨兵例 e2e $case"
        run_sample "$CUR_TAG" 0 5 1200
        run_heavy python3 "$ROOT/scripts/run_tests.py" --no-run -j 1 \
            --transpile-timeout 2400 --out-dir "$OUT/e2e-out" \
            --filter "/${case##*/}.java"
        join_sample "$CUR_TAG"
        sampler="$OUT/logs/$CUR_TAG.sampler"
        peak="$(json_field "$sampler" peak_gib)"
        outcome="$(json_field "$sampler" outcome)"
        dur="$(json_field "$sampler" dur_s)"
        base="${SENT_BASE[$face|$case]:-}"
        verdict="$(sentinel_verdict "$RUN_RC" "$outcome" "$peak" "$base")"
        append_row "$SENT_TSV" "$(printf '%s\t%s\t%s\t%s\t%s\t%s\t%s\t%s' \
"$face" "$case" "$RUN_RC" "$outcome" "$peak" "${base:-}" "$verdict" "$dur")"
        log "哨兵例 e2e $case → $verdict（峰值 ${peak}G / dur ${dur}s）——已落盘"
    done
    for case in "${SENTINEL_PILOT[@]}"; do
        face=pilot; tag="sent-$case"; CUR_TAG="$tag"
        if [[ -n "${DONE[$face|$case]:-}" ]]; then log "跳过 pilot $case（TSV 已落盘，续跑）"; continue; fi
        log "哨兵例 pilot $case"
        run_sample "$CUR_TAG" 0 5 900
        SCRATCH_ROOT="$OUT/pilot-scratch" \
            run_heavy bash "$ROOT/scripts/lib_pilot_golden.sh" "$case" --emit-only
        join_sample "$CUR_TAG"
        sampler="$OUT/logs/$CUR_TAG.sampler"
        peak="$(json_field "$sampler" peak_gib)"
        outcome="$(json_field "$sampler" outcome)"
        dur="$(json_field "$sampler" dur_s)"
        base="${SENT_BASE[$face|$case]:-}"
        verdict="$(sentinel_verdict "$RUN_RC" "$outcome" "$peak" "$base")"
        append_row "$SENT_TSV" "$(printf '%s\t%s\t%s\t%s\t%s\t%s\t%s\t%s' \
"$face" "$case" "$RUN_RC" "$outcome" "$peak" "${base:-}" "$verdict" "$dur")"
        log "哨兵例 pilot $case → $verdict（峰值 ${peak}G / dur ${dur}s）——已落盘"
    done
    local fails sets total
    read -r fails sets total <<< "$(tsv_stat "$SENT_TSV" 7)"
    if [[ "$fails" -gt 0 ]]; then
        printf 'SENTINEL ALERT %d/%d 例红（rc≠0 / 越线 / 基线+25%%）\n' "$fails" "$total" | tee -a "$SUMMARY"
    else
        printf 'SENTINEL ALL-GREEN (%d/%d)%s\n' \
            "$total" "$total" \
            "$([[ $sets -gt 0 ]] && printf ' 其中 %d 例新记基线' "$sets" || true)" | tee -a "$SUMMARY"
    fi
    SENT_RC=$(( fails > 0 ? 1 : 0 ))
}

# 绿转红判定：rc≠0 → FAIL-RC；越线被杀 → FAIL-LIMIT；峰值超基线 +25% → FAIL-REGRESS；
# 无基线 → BASELINE-SET（首轮采集口径，不作警报）
sentinel_verdict() { # <rc> <outcome> <peak> <base>
    local rc="$1" outcome="$2" peak="$3" base="$4"
    [[ "$outcome" == "limit-kill" ]] && { echo "FAIL-LIMIT(>${LIMIT_GIB}G)"; return; }
    [[ "$outcome" == "no-run" ]]       && { echo "FAIL-NO-RUN"; return; }
    [[ "$rc" != 0 ]]                   && { echo "FAIL-RC($rc)"; return; }
    if [[ -n "$base" ]]; then
        python3 -c 'import sys
peak, base = float(sys.argv[1]), float(sys.argv[2])
print("FAIL-REGRESS(+{:.0f}%)".format((peak/base-1)*100) if peak > base*1.25 else "PASS")' \
            "$peak" "$base"
    else
        echo "BASELINE-SET"
    fi
}

# ── 病理档 ──────────────────────────────────────────────────────────────────
run_pathology() {
    log "== 病理档：12 例固定窗口 ${WINDOW_S}s / 上限 ${LIMIT_GIB}G，10s 粒度 =="
    local -A DONE=()
    local k
    while IFS= read -r k; do [[ -n "$k" ]] && DONE["$k"]=1; done < <(tsv_done "$PATH_TSV" '$1')
    init_tsv "$PATH_TSV" $'case\toutcome\tsamples\tdur_s\tfinal_gib\trate_mib_s\tbaseline_rate\tdrift_pct\tverdict'
    local case tag rate base verdict outcome pct sampler samples dur final
    local all="${PATHOLOGY[*]} $PATHOLOGY_ESPER"
    for case in $all; do
        tag="$(tagify "$case")"; CUR_TAG="path-$tag"
        if [[ -n "${DONE[$case]:-}" ]]; then log "跳过 $case（TSV 已落盘，续跑）"; continue; fi
        log "病理例 $case"
        run_sample "$CUR_TAG" "$WINDOW_S" 10 900
        SCRATCH_ROOT="$OUT/pilot-scratch" \
            run_heavy bash "$ROOT/scripts/lib_pilot_golden.sh" "$case" --emit-only
        join_sample "$CUR_TAG"
        sampler="$OUT/logs/$CUR_TAG.sampler"
        rate="$(json_field "$sampler" rate_mib_s)"
        outcome="$(json_field "$sampler" outcome)"
        samples="$(json_field "$sampler" samples)"
        dur="$(json_field "$sampler" dur_s)"
        final="$(json_field "$sampler" final_gib)"
        base="${PATH_BASE[$case]:-}"
        read -r verdict pct <<< "$(pathology_verdict "$outcome" "$rate" "$base")"
        append_row "$PATH_TSV" "$(printf '%s\t%s\t%s\t%s\t%s\t%s\t%s\t%s\t%s' \
"$case" "$outcome" "$samples" "$dur" "$final" "$rate" "${base:-}" "${pct:-}" "$verdict")"
        log "病理例 $case → $verdict（速率 ${rate} MiB/s / ${samples} 样本）——已落盘"
    done
    local drift sets total
    read -r drift sets total <<< "$(tsv_stat "$PATH_TSV" 9)"
    if [[ "$drift" -gt 0 ]]; then
        printf 'PATHOLOGY DRIFT %d/%d 例速率偏移超 ±20%%（含行为改变：completed 即修复信号）\n' "$drift" "$total" | tee -a "$SUMMARY"
    elif [[ "$sets" -gt 0 ]]; then
        printf 'PATHOLOGY BASELINE-SET (%d/%d)（无既有基线可比，本轮全部记基线）\n' "$total" "$total" | tee -a "$SUMMARY"
    else
        printf 'PATHOLOGY STABLE (%d/%d)\n' "$total" "$total" | tee -a "$SUMMARY"
    fi
    PATH_RC=$(( drift > 0 ? 1 : 0 ))
}

# 速率判定：NO-RUN（没见到转译进程）按运行事故计 drift；无基线 → BASELINE-SET；
# |rate/base−1| > 0.20 → DRIFT；否则 STABLE。输出 "<verdict> <pct>"
pathology_verdict() { # <outcome> <rate> <base>
    local outcome="$1" rate="$2" base="$3"
    if [[ "$outcome" == "no-run" ]]; then echo "DRIFT(NO-RUN) no-run"; return; fi
    if [[ -z "$base" ]]; then echo "BASELINE-SET -"; return; fi
    python3 -c 'import sys
rate, base = float(sys.argv[1]), float(sys.argv[2])
pct = (rate/base - 1) * 100
print(("DRIFT" if abs(pct) > 20 else "STABLE") + " " + f"{pct:+.1f}%")' "$rate" "$base"
}

# ── 基线提升（build/sentinel/<日期>/ → docs/sentinel-baselines/，随仓库存档）──
promote() {
    local src="$OUT" ver date
    if [[ -n "${1:-}" && "$1" != "latest" ]]; then src="$ROOT/build/sentinel/$1"; fi
    [[ -f "$src/sentinel.tsv" && -f "$src/pathology.tsv" ]] || {
        echo "promote：$src 下缺 sentinel.tsv / pathology.tsv（先跑一轮）" >&2; exit 2; }
    ver="$(git -C "$ROOT" rev-parse --short HEAD)"
    date="$(basename "$src")"
    mkdir -p "$BASEDIR/sentinel-curves" "$BASEDIR/pathology-curves"
    awk -F'\t' -v v="$ver" -v d="$date" 'NR>1{print $1"\t"$2"\t"$5"\t"$3"\t"$4"\t"v"\t"d}' \
        "$src/sentinel.tsv" > "$BASEDIR/sentinel-baseline.tsv"
    awk -F'\t' -v v="$ver" -v d="$date" -v w="$WINDOW_S" \
        'NR>1{print $1"\t"$6"\t"$5"\t"$2"\t"v"\t"d"\t"w}' \
        "$src/pathology.tsv" > "$BASEDIR/pathology-baseline.tsv"
    local tag
    grep -v '^#' "$src/sentinel.tsv" | tail -n +2 | cut -f2 | while IFS= read -r case; do
        tag="sent-$(tagify "$case")"
        if [[ -f "$src/curves/$tag.tsv" ]]; then
            cp "$src/curves/$tag.tsv" "$BASEDIR/sentinel-curves/$(tagify "$case").tsv"
        fi
    done || true
    for case in ${PATHOLOGY[*]} "$PATHOLOGY_ESPER"; do
        tag="path-$(tagify "$case")"
        if [[ -f "$src/curves/$tag.tsv" ]]; then
            cp "$src/curves/$tag.tsv" "$BASEDIR/pathology-curves/$(tagify "$case").tsv"
        fi
    done
    printf '基线已存档 %s（版本 %s，%s）：sentinel 10 例峰值 + pathology 12 例速率/曲线\n' \
        "$BASEDIR" "$ver" "$date"
}

# ── 主流程 ──────────────────────────────────────────────────────────────────
mkdir -p "$OUT/curves" "$OUT/logs" "$OUT/e2e-out" "$OUT/pilot-scratch"
load_baselines
SUMMARY="$OUT/summary.txt"
: > "$SUMMARY"
SENT_RC=0 PATH_RC=0
{
    printf '# sentinel16g %s mode=%s limit=%sG window=%ss worktree=%s\n' \
        "$DATE" "$MODE_ARG" "$LIMIT_GIB" "$WINDOW_S" "$ROOT"
} >> "$SUMMARY"

case "$MODE_ARG" in
sentinel)    run_sentinel ;;
pathology)   run_pathology ;;
all)         run_sentinel; run_pathology ;;
promote)     promote "${1:-latest}"; exit 0 ;;
*) echo "用法: $0 [sentinel|pathology|all|promote [日期]] [--full]" >&2; exit 2 ;;
esac

{
    printf '\n== sentinel ==\n'; cat "$OUT/sentinel.tsv" 2>/dev/null || true
    printf '\n== pathology ==\n'; cat "$OUT/pathology.tsv" 2>/dev/null || true
} >> "$SUMMARY"
ALERT_LINE="$(grep -h '^SENTINEL ' "$SUMMARY" | tail -1 || true)"
log "完成：$OUT（rc sentinel=$SENT_RC pathology=$PATH_RC）"
[[ "$MODE_ARG" != pathology ]] && [[ -n "$ALERT_LINE" ]] && log "$ALERT_LINE"
exit $(( SENT_RC || PATH_RC ))
