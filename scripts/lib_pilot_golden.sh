#!/usr/bin/env bash
# lib pilot golden 对账（M1/M2，docs/plans/2026-09-23-junit-crate-pilot.md）
#
# 用法：
#   scripts/lib_pilot_golden.sh m1     # hamcrest crate：JVM 真 hamcrest vs 翻译 crate
#   scripts/lib_pilot_golden.sh m2     # junit4 crate（Assert 子集）：同上
#   scripts/lib_pilot_golden.sh m2 --no-transpile   # 只重跑对账（复用已生成 scratch）
#
# 前置：--jdk 21 的 JAVA_HOME（默认 /opt/homebrew/opt/openjdk@21）；jar 资产在
# pilot-deps/target/pilot-libs/（可经 PILOT_LIBS 覆盖）。
# 流程：javac（-cp jars）→ java 真 jar 侧 golden → main.py --lib 转译 →
# cargo run 翻译侧输出 → diff 逐字对账。golden 文本随仓库存档于
# tests/lib_pilot/golden/（跑批可复现的对账凭据）。
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
LIBS="${PILOT_LIBS:-/Users/yuwei/dev/workspace/pilot-deps/target/pilot-libs}"
export JAVA_HOME="${JAVA_HOME:-/opt/homebrew/opt/openjdk@21}"
JAVAC="$JAVA_HOME/bin/javac"; JAVA="$JAVA_HOME/bin/java"
MODE="${1:?用法: $0 m1|m2 [--no-transpile]}"
TRANSPILE=1
[[ "${2:-}" == "--no-transpile" ]] && TRANSPILE=0

cd "$REPO_ROOT/tests/lib_pilot"
mkdir -p golden

case "$MODE" in
m1)
    MAIN=HamcrestAssertMain
    CP="$LIBS/hamcrest-3.0.jar"
    LIB_ARGS=(--lib "hamcrest=$LIBS/hamcrest-3.0.jar")
    ;;
m2)
    MAIN=JunitAssertMain
    CP="$LIBS/junit-4.13.2.jar:$LIBS/hamcrest-3.0.jar"
    LIB_ARGS=(--lib "hamcrest=$LIBS/hamcrest-3.0.jar"
              --lib "junit4=$LIBS/junit-4.13.2.jar:seed=org.junit.Assert")
    ;;
*) echo "未知模式: $MODE" >&2; exit 2;;
esac

echo "== [1/3] JVM 侧 golden（真 jar）=="
rm -rf classes && "$JAVAC" -d classes -cp "$CP" "$MAIN.java"
"$JAVA" -cp "classes:$CP" "$MAIN" > "golden/${MODE}_jvm.txt"
echo "JVM 侧 $(wc -l < "golden/${MODE}_jvm.txt" | tr -d ' ') 行"

echo "== [2/3] 转译 + cargo run（翻译 crate）=="
cd "$REPO_ROOT"
if [[ "$TRANSPILE" == 1 ]]; then
    python3 scripts/main.py "tests/lib_pilot/$MAIN.java" --jdk 21 --clean \
        "${LIB_ARGS[@]}" --no-run > "/tmp/${MODE}_transpile.log" 2>&1
fi
SCRATCH="build/$(python3 - "$MAIN" <<'PY'
import re, sys
print(re.sub(r'(?<=[a-z0-9])(?=[A-Z])', '_', sys.argv[1]).lower())
PY
)"
(cd "$SCRATCH" && CARGO_TARGET_DIR="$REPO_ROOT/build/target" \
    cargo run --quiet --bin "$(basename "$SCRATCH")" \
    > "$REPO_ROOT/tests/lib_pilot/golden/${MODE}_rs.txt" 2>"/tmp/${MODE}_cargo.err") \
    || { echo "cargo run 失败，见 /tmp/${MODE}_cargo.err" >&2; exit 1; }

echo "== [3/3] 逐字对账 =="
if diff -u "tests/lib_pilot/golden/${MODE}_jvm.txt" "tests/lib_pilot/golden/${MODE}_rs.txt"; then
    echo "GOLDEN OK（${MODE}）"
else
    echo "GOLDEN DIFF（${MODE}）" >&2
    exit 1
fi
