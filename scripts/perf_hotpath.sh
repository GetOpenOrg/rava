#!/usr/bin/env bash
# 运行时热路径三测点计时（docs/reports/2026-10-11-code-review-triage.md「性能实测」）：
# 每例记 构建耗时（转译 + 编译，秒）/ 运行耗时（REPEAT 次中位数，秒）/ 二进制字节数，写 TSV。
#
# 用法：scripts/perf_hotpath.sh <out.tsv> [用例 ...]
#   用例：tests/e2e 下的类名（如 DeepCopy）或 spring-core 切片 sc1..sc4（经 scripts/lib_pilot_golden.sh）
#   缺省用例：DeepCopy StreamDemo TestAnnoReflect TestDynamicProxy TestReflectLdcHotLoop sc1 sc2 sc3 sc4
# 环境：REPEAT（缺省 5）、PILOT_LIBS（sc 切片的 jar 目录，见 lib_pilot_golden.sh）
# 仅服务器执行（经 distribute_tests.py --job 下发）；首例含档案 crate 冷编译，需要时先跑一遍热身用例。
set -uo pipefail

REPO="$(cd "$(dirname "$0")/.." && pwd)"
OUT="${1:?用法: $0 <out.tsv> [用例 ...]}"; shift
CASES=("$@")
[ ${#CASES[@]} -eq 0 ] && CASES=(DeepCopy StreamDemo TestAnnoReflect TestDynamicProxy TestReflectLdcHotLoop sc1 sc2 sc3 sc4)
REPEAT="${REPEAT:-5}"
. "$REPO/scripts/rava_env.sh" "$REPO"
. "$REPO/scripts/corpus_jdk.sh" "$REPO"
SROOT="$REPO/build/perf_hotpath"
mkdir -p "$SROOT"
printf 'case\tbuild_secs\trun_median_secs\tbin_bytes\tstatus\n' > "$OUT"

now() { date +%s.%N; }
secs() { python3 -c "import sys; print(f'{float(sys.argv[2]) - float(sys.argv[1]):.2f}')" "$1" "$2"; }
exe_of() { python3 -c 'import json,sys; print(json.load(open(sys.argv[1])).get("exe") or "")' "$1/build_status.json" 2>/dev/null; }
snake() { python3 -c 'import re,sys; print(re.sub(r"(?<=[a-z0-9])(?=[A-Z])", "_", sys.argv[1]).lower())' "$1"; }

# 运行 REPEAT 次取墙钟中位数（输出丢弃；任一次非零退出 → FAIL）
median_run() {
    python3 - "$1" "$REPEAT" <<'PY'
import statistics, subprocess, sys, time
exe, n = sys.argv[1], int(sys.argv[2])
ts = []
for _ in range(n):
    t = time.perf_counter()
    r = subprocess.run([exe], stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
    ts.append(time.perf_counter() - t)
    if r.returncode != 0:
        print("FAIL"); sys.exit(0)
print(f"{statistics.median(ts):.3f}")
PY
}

for c in "${CASES[@]}"; do
    status=ok
    t0=$(now)
    if [[ "$c" == sc[0-9] ]]; then
        main=$(grep -oP "^\s*$c\) MAIN=\K\w+" "$REPO/scripts/lib_pilot_golden.sh")
        SCRATCH_ROOT="$SROOT" bash "$REPO/scripts/lib_pilot_golden.sh" "$c" > "$SROOT/$c.log" 2>&1 || status=FAIL
        scratch="$SROOT/$(snake "$main")"
    else
        src=$(find "$REPO/tests/e2e" -name "$c.java" | head -1)
        scratch="$SROOT/$(snake "$c")"
        { "$RAVA" build "$src" "${CORPUS_JDK_ARGS[@]}" --clean --out "$scratch" --stop-after emit \
            && "$RAVA" compile "$scratch"; } > "$SROOT/$c.log" 2>&1 || status=FAIL
    fi
    build=$(secs "$t0" "$(now)")
    exe=$(exe_of "$scratch")
    run=- bytes=-
    if [ "$status" = ok ] && [ -x "$exe" ]; then
        run=$(median_run "$exe")
        [ "$run" = FAIL ] && status=FAIL
        bytes=$(stat -c %s "$exe")
    else
        status=FAIL
    fi
    printf '%s\t%s\t%s\t%s\t%s\n' "$c" "$build" "$run" "$bytes" "$status" | tee -a "$OUT"
done
