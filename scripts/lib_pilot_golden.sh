#!/usr/bin/env bash
# lib pilot golden 对账（M1–M5，docs/plans/2026-09-23-junit-crate-pilot.md）
#
# 用法：
#   scripts/lib_pilot_golden.sh m1     # hamcrest crate：JVM 真 hamcrest vs 翻译 crate
#   scripts/lib_pilot_golden.sh m2     # junit4 crate（Assert 子集）：同上
#   scripts/lib_pilot_golden.sh m3     # junit4 crate（Runner 路径）：JUnitCore.runClasses
#   scripts/lib_pilot_golden.sh m4     # junit4 crate（@Test(timeout=) 路径）：FailOnTimeout
#   scripts/lib_pilot_golden.sh m5     # 跨 crate 分派链：user 类实现 lib 类型并被 lib 回调
#   scripts/lib_pilot_golden.sh sc1    # spring-core 切片（矩阵 #11）：ResolvableType / MethodParameter 泛型元数据
#   scripts/lib_pilot_golden.sh sc2    # spring-core 切片：AntPathMatcher
#   scripts/lib_pilot_golden.sh sc3    # spring-core 切片：PropertyPlaceholderHelper + StringUtils
#   scripts/lib_pilot_golden.sh sc4    # spring-core 切片：StreamUtils + MultiValueMap
#   scripts/lib_pilot_golden.sh m2 --no-transpile   # 只重跑对账（复用已生成 scratch）
#
# 前置：参考 JDK（tools/refjdk.toml，scripts/fetch_reference_jdk.sh 取包；JDK=N 改用本机 JDK，仅供实验）；jar 资产在 tests/lib_pilot/deps/target/pilot-libs/
#（scripts/fetch_pilot_deps.sh --no-scan 导出，同时生成依赖锁 deps.lock.toml；可经 PILOT_LIBS 覆盖）。
# 库输入走 V12-1 终态入口：--deps deps.lock.toml + --cp 锁条目名（crate 名 = 模块 crate 名：
# hamcrest → org_hamcrest、junit → junit）；整包翻译以 --seed-class 显式给出（jar 全类清单）。
# 流程：javac（-cp jars）→ java 真 jar 侧 golden → rava build --deps/--cp 转译 + 编译 →
# 运行翻译侧产物 → diff 逐字对账。golden 文本随仓库存档于
# tests/lib_pilot/golden/（跑批可复现的对账凭据）。
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
# jar 资产默认取仓库内导出位（依赖清单 tests/lib_pilot/deps/pom.xml）
LIBS="${PILOT_LIBS:-$REPO_ROOT/tests/lib_pilot/deps/target/pilot-libs}"
[ -f "$LIBS/junit-4.13.2.jar" ] || { echo "缺 jar：先跑 scripts/fetch_pilot_deps.sh --no-scan（或设 PILOT_LIBS）" >&2; exit 2; }
LOCK="${DEPS_LOCK:-$(dirname "$LIBS")/deps.lock.toml}"
[ -f "$LOCK" ] || { echo "缺依赖锁 $LOCK：先跑 scripts/fetch_pilot_deps.sh --no-scan" >&2; exit 2; }
# hamcrest 全类清单（m1 整包翻译的显式种子；m2–m5 在此之上加 junit 种子——与原 :seed= 序一致）
HAMCREST_ALL="$(python3 - "$LIBS/hamcrest-3.0.jar" <<'PY'
import sys, zipfile
print(",".join(sorted(n[:-6] for n in zipfile.ZipFile(sys.argv[1]).namelist()
                     if n.endswith(".class") and not n.endswith("module-info.class"))))
PY
)"
. "$REPO_ROOT/scripts/rava_env.sh" "$REPO_ROOT"
# 语料 JDK 与 run_tests.py 同口径：golden JVM 与转译语料同源于参考构建
. "$REPO_ROOT/scripts/corpus_jdk.sh" "$REPO_ROOT"
JAVAC="$JAVA_HOME/bin/javac"; JAVA="$JAVA_HOME/bin/java"
MODE="${1:?用法: $0 m1|m2|m3|m4|m5|sc1|sc2|sc3|sc4 [--no-transpile|--emit-only]}"
TRANSPILE=1
EMIT_ONLY=0
[[ "${2:-}" == "--no-transpile" ]] && TRANSPILE=0
# --emit-only：只做转译（--stop-after emit）即退，scratch 供 scripts/compare_trees.sh 对照；
# scratch 根可用 SCRATCH_ROOT 覆盖（缺省 $REPO_ROOT/build），双提交树对比时两轮各给不同根
[[ "${2:-}" == "--emit-only" ]] && EMIT_ONLY=1

cd "$REPO_ROOT/tests/lib_pilot"
mkdir -p golden

case "$MODE" in
m1)
    MAIN=HamcrestAssertMain
    CP="$LIBS/hamcrest-3.0.jar"
    LIB_ARGS=(--deps "$LOCK" --cp hamcrest --seed-class "$HAMCREST_ALL")
    ;;
m2)
    MAIN=JunitAssertMain
    CP="$LIBS/junit-4.13.2.jar:$LIBS/hamcrest-3.0.jar"
    LIB_ARGS=(--deps "$LOCK" --cp hamcrest,junit --seed-class "$HAMCREST_ALL,org.junit.Assert")
    ;;
m3)
    # Runner 路径：种子 = JUnitCore（runClasses 入口）+ Assert（用例断言面）+
    # Test 注解（@Test 发现）；Runner 族其余成员由可达性通道 BFS 拉入
    MAIN=JunitRunnerMain
    CP="$LIBS/junit-4.13.2.jar:$LIBS/hamcrest-3.0.jar"
    LIB_ARGS=(--deps "$LOCK" --cp hamcrest,junit
              --seed-class "$HAMCREST_ALL,org.junit.runner.JUnitCore,org.junit.Assert,org.junit.Test")
    ;;
m4)
    # timeout 路径：@Test(timeout=) 经 FailOnTimeout（线程 + FutureTask.get 限时等待）；
    # 种子同 m3，FailOnTimeout 族由可达性通道拉入
    MAIN=JunitTimeoutMain
    CP="$LIBS/junit-4.13.2.jar:$LIBS/hamcrest-3.0.jar"
    LIB_ARGS=(--deps "$LOCK" --cp hamcrest,junit
              --seed-class "$HAMCREST_ALL,org.junit.runner.JUnitCore,org.junit.Assert,org.junit.Test")
    ;;
m5)
    # 跨 crate 分派链：user 类继承/实现 lib 类型（BaseMatcher / TypeSafeMatcher /
    # 匿名 Matcher），由 lib 代码回调；Runner→用户测试→hamcrest→用户 Matcher 三 crate 往返
    MAIN=JunitCrossCrateMain
    CP="$LIBS/junit-4.13.2.jar:$LIBS/hamcrest-3.0.jar"
    LIB_ARGS=(--deps "$LOCK" --cp hamcrest,junit
              --seed-class "$HAMCREST_ALL,org.junit.runner.JUnitCore,org.junit.Assert,org.junit.Test,org.junit.Before")
    ;;
sc1|sc2|sc3|sc4)
    # spring-core 切片（矩阵 #11）：用户 main 为唯一入口，库类按档案调用链翻译，不设整包种子；
    # spring-jcl 是 spring-core 的日志门面依赖（LogFactory）
    case "$MODE" in
        sc1) MAIN=SpringResolvableTypeMain ;;
        sc2) MAIN=SpringAntPathMain ;;
        sc3) MAIN=SpringPlaceholderMain ;;
        sc4) MAIN=SpringStreamMultiValueMain ;;
    esac
    CP="$LIBS/spring-core-6.2.19.jar:$LIBS/spring-jcl-6.2.19.jar"
    LIB_ARGS=(--deps "$LOCK" --cp spring-core,spring-jcl)
    ;;
*) echo "未知模式: $MODE" >&2; exit 2;;
esac

echo "== [1/3] JVM 侧 golden（真 jar）=="
rm -rf classes && "$JAVAC" -d classes -cp "$CP" "$MAIN.java"
"$JAVA" -Dstdout.encoding=UTF-8 -cp "classes:$CP" "$MAIN" > "golden/${MODE}_jvm.txt"
echo "JVM 侧 $(wc -l < "golden/${MODE}_jvm.txt" | tr -d ' ') 行"

echo "== [2/3] 转译 + 编译 + 运行（翻译 crate）=="
cd "$REPO_ROOT"
SCRATCH="${SCRATCH_ROOT:-$REPO_ROOT/build}/$(python3 - "$MAIN" <<'PY'
import re, sys
print(re.sub(r'(?<=[a-z0-9])(?=[A-Z])', '_', sys.argv[1]).lower())
PY
)"
if [[ "$TRANSPILE" == 1 ]]; then
    "$RAVA" build "tests/lib_pilot/$MAIN.java" "${CORPUS_JDK_ARGS[@]}" --clean "${LIB_ARGS[@]}" \
        --out "$SCRATCH" --stop-after emit > "/tmp/${MODE}_transpile.log" 2>&1 \
        || { echo "转译失败，见 /tmp/${MODE}_transpile.log" >&2; exit 1; }
fi
if [[ "$EMIT_ONLY" == 1 ]]; then
    echo "emit-only：$SCRATCH"
    exit 0
fi
"$RAVA" compile "$SCRATCH" > "/tmp/${MODE}_cargo.err" 2>&1 \
    || { echo "编译失败，见 /tmp/${MODE}_cargo.err 与 $SCRATCH/build_status.json" >&2; exit 1; }
EXE="$(python3 -c 'import json,sys;print(json.load(open(sys.argv[1]))["exe"])' "$SCRATCH/build_status.json")"
"$EXE" > "$REPO_ROOT/tests/lib_pilot/golden/${MODE}_rs.txt" 2>>"/tmp/${MODE}_cargo.err" \
    || { echo "运行失败，见 /tmp/${MODE}_cargo.err" >&2; exit 1; }

echo "== [3/3] 逐字对账 =="
if diff -u "tests/lib_pilot/golden/${MODE}_jvm.txt" "tests/lib_pilot/golden/${MODE}_rs.txt"; then
    echo "GOLDEN OK（${MODE}）"
else
    echo "GOLDEN DIFF（${MODE}）" >&2
    exit 1
fi
