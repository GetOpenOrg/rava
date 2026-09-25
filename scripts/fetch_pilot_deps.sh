#!/usr/bin/env bash
# lib pilot 语料取包 + 透视：mvn 解析依赖闭包 → jar 导出 → 逐 jar 跑 dep_scan。
#
# 依赖清单（版本钉死）：tests/lib_pilot/deps/pom.xml
# jar 导出位：tests/lib_pilot/deps/target/pilot-libs/（gitignore；scripts/lib_pilot_golden.sh 默认从此取）
#
# 用法：scripts/fetch_pilot_deps.sh            # 取包 + 逐 jar 透视
#       scripts/fetch_pilot_deps.sh --no-scan  # 只取包（golden 对账前置）
# 依赖树全貌：mvn -B -f tests/lib_pilot/deps/pom.xml dependency:tree
# 前置：mvn 在 PATH；JDK 经 scripts/jdk_select.py 选择（与 main.py / run_tests.py 同一入口）。
set -euo pipefail
REPO="$(cd "$(dirname "$0")/.." && pwd)"
DEPS="$REPO/tests/lib_pilot/deps"
SCAN=1; [[ "${1:-}" == "--no-scan" ]] && SCAN=0

JAVA_HOME="$(python3 "$REPO/scripts/jdk_select.py")" || { echo "未找到可用 JDK" >&2; exit 2; }
export JAVA_HOME

# 清残留：copy-dependencies 不清目录，升版后旧 jar 会并存污染语料口径
rm -rf "$DEPS/target/pilot-libs"
mvn -B -q -f "$DEPS/pom.xml" package
echo "─── jar 已导出：$DEPS/target/pilot-libs/ ───"
ls -lh "$DEPS/target/pilot-libs/"

[[ "$SCAN" == 1 ]] || exit 0
for jar in "$DEPS"/target/pilot-libs/*.jar; do
    echo
    echo "════ $(basename "$jar") ════"
    python3 "$REPO/scripts/dep_scan.py" "$jar"
done
