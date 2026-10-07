#!/usr/bin/env bash
# 阶段 API 面作业（dev 上以作业模式运行；本机不跑）：取包 → 样例应用 javac + 真 JVM 运行（记录输出与实载类）
# → rava 闭包两变体（A：仅 main；B：main + JVM 实载的框架类作 --seed-class）→ 面文件与 e2e 分层数据。
#
# 用法：scripts/api_surface_job.sh [阶段=s0]        样例在 tests/lib_pilot/<阶段>_boot/（src/ resources/ jars.txt）
# 产物：build/api_surface/<阶段>/（作业 --fetch 'build/api_surface/<阶段>/**'），面与分层数据另写
#       tests/api_surface/<阶段>.txt、tests/api_surface/tiers_<阶段>.toml（同样取回后入库）
# 口径：docs/plans/2026-10-07-framework-driven-api-coverage.md 步骤 1 / 2
set -uo pipefail
STAGE="${1:-s0}"
REPO="$(cd "$(dirname "$0")/.." && pwd)"
APP="$REPO/tests/lib_pilot/${STAGE}_boot"
OUT="$REPO/build/api_surface/$STAGE"
RAW="$REPO/build/api_surface_raw/$STAGE"
DEPS="$REPO/tests/lib_pilot/deps/target"
mkdir -p "$OUT" "$RAW"
step() { echo "═══ $(date '+%H:%M:%S') $*"; }
# 项目解释器（pyproject requires-python ≥3.12，tomllib 内置）；服务器系统 python3 可能更旧
PY=(uv run -q --project "$REPO" python)

# 1. 取包（maven 本地库落在检出目录内，不写服务器 ~/.m2）
step "取包"
export MAVEN_OPTS="-Dmaven.repo.local=$REPO/build/m2 ${MAVEN_OPTS:-}"
bash "$REPO/scripts/fetch_pilot_deps.sh" --no-scan >"$OUT/fetch.log" 2>&1 || { echo "取包失败"; tail -30 "$OUT/fetch.log"; exit 2; }
grep -E "deps.lock.toml" "$OUT/fetch.log"
. "$REPO/scripts/rava_env.sh" "$REPO"
# 参考 JDK（与 run_tests / 语料脚本同口径；jmods 供 jdk_index 解析），导出 JAVA_HOME 与 CORPUS_JDK_ARGS
. "$REPO/scripts/corpus_jdk.sh" "$REPO"

# 2. 阶段类路径：jars.txt 条目名 → 锁内 jar 路径
NAMES="$(grep -v '^\s*#' "$APP/jars.txt" | grep -v '^\s*$' | paste -sd, -)"
CP="$("${PY[@]}" - "$DEPS/deps.lock.toml" "$NAMES" <<'PY'
import sys
import tomllib
from pathlib import Path
lock = Path(sys.argv[1]); want = sys.argv[2].split(",")
jars = tomllib.loads(lock.read_text())["jar"]
by = {j["coordinate"].split(":")[1] if "coordinate" in j else Path(j["path"]).stem: j for j in jars}
miss = [n for n in want if n not in by]
if miss:
    sys.exit(f"锁中缺少条目：{miss}")
print(":".join(str(lock.parent / by[n]["path"]) for n in want))
PY
)" || exit 2
echo "$CP" | tr ':' '\n' | sed "s|$DEPS/||" >"$OUT/classpath.txt"
echo "阶段 jar $(wc -l <"$OUT/classpath.txt") 个"

# 3. 样例 javac + 真 JVM 运行（实载类日志供种子与对照）
step "javac + JVM"
CLS="$RAW/classes"
rm -rf "$CLS"; mkdir -p "$CLS"
"$JAVA_HOME/bin/javac" -g -parameters -cp "$CP" -d "$CLS" $(find "$APP/src" -name '*.java') || exit 2
cp -R "$APP/resources/." "$CLS/"
timeout 600 "$JAVA_HOME/bin/java" -Xlog:class+load=info:file="$RAW/classload.log" -cp "$CLS:$CP" \
    "$(grep -rl 'static void main' "$APP/src" | head -1 | sed "s|$APP/src/||; s|\.java$||; s|/|.|g")" \
    >"$OUT/jvm_stdout.txt" 2>"$OUT/jvm_stderr.txt"
echo "JVM rc=$?"; cat "$OUT/jvm_stdout.txt" | head -40
"${PY[@]}" "$REPO/scripts/api_surface.py" seeds --classload "$RAW/classload.log" --out "$OUT/seed_classes.txt" \
    --source "$CLS" $(echo "$CP" | tr ':' '\n' | sed 's/^/--source /')
grep -c ' source: jrt:/\| source: shared objects file' "$RAW/classload.log" | sed 's/^/JVM 实载 JDK 类 /'

# 4. rava 闭包两变体（失败如实记录，不中断后续一跳面）
MAIN="$(grep -rl 'static void main' "$APP/src" | head -1 | sed "s|$APP/src/||; s|\.java$||; s|/|.|g")"
run_closure() {
    local tag="$1"; shift
    step "闭包 $tag"
    local t0=$SECONDS
    local timer=(); [[ -x /usr/bin/time ]] && timer=(/usr/bin/time -v)
    timeout "${API_SURFACE_CLOSURE_TIMEOUT:-2400}" "${timer[@]}" "$RAVA" closure "$CLS" "${CORPUS_JDK_ARGS[@]}" --main "$MAIN" --deps "$DEPS/deps.lock.toml" --cp "$NAMES" \
        -o "$RAW/closure_$tag.json" "$@" >"$OUT/closure_$tag.out" 2>"$OUT/closure_$tag.err"
    local rc=$?
    echo "闭包 $tag rc=$rc 耗时 $((SECONDS - t0))s"
    grep -E "Maximum resident|Elapsed" "$OUT/closure_$tag.err" | sed 's/^\s*/  /'
    grep -vE "^\s+(Maximum|Elapsed|Command|User|System|Percent|Average|Exit|Swaps|File|Socket|Signals|Page|Voluntary|Involuntary|Minor|Major)" \
        "$OUT/closure_$tag.err" | tail -15
    head -c 3000 "$OUT/closure_$tag.out"; echo
    [[ $rc == 0 ]] || rm -f "$RAW/closure_$tag.json"
}
run_closure a
SEEDS=()
while read -r c; do [[ -n "$c" ]] && SEEDS+=(--seed-class "$c"); done <"$OUT/seed_classes.txt"
run_closure b "${SEEDS[@]}"

# 5. 面 + 分层
step "面"
JARS=(); while read -r j; do JARS+=(--jar "$DEPS/$j"); done <"$OUT/classpath.txt"
"${PY[@]}" "$REPO/scripts/api_surface.py" face --closure "a=$RAW/closure_a.json" --closure "b=$RAW/closure_b.json" \
    "${JARS[@]}" --classload "$RAW/classload.log" --out "$REPO/tests/api_surface/$STAGE.txt" --data "$OUT/face.json" || exit 2
cp "$REPO/tests/api_surface/$STAGE.txt" "$OUT/"
step "分层"
JUNIT_CP="$(ls "$DEPS"/pilot-libs/junit-*.jar "$DEPS"/pilot-libs/hamcrest-*.jar 2>/dev/null | paste -sd: -)"
"${PY[@]}" "$REPO/scripts/api_surface.py" tiers --face "$REPO/tests/api_surface/$STAGE.txt" --face-data "$OUT/face.json" \
    --cp "$JUNIT_CP" -j "${API_SURFACE_JOBS:-8}" --out "$REPO/tests/api_surface/tiers_$STAGE.toml" --data "$OUT/tiers.json" || exit 2
cp "$REPO/tests/api_surface/tiers_$STAGE.toml" "$OUT/"
gzip -c "$RAW/classload.log" >"$OUT/classload.log.gz"
step "完成"
