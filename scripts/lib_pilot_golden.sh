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
#   scripts/lib_pilot_golden.sh sb1    # spring-beans 切片（矩阵 #12）：BeanUtils
#   scripts/lib_pilot_golden.sh sb2    # spring-beans 切片：BeanWrapperImpl 属性读写与嵌套路径
#   scripts/lib_pilot_golden.sh sb3    # spring-beans 切片：PropertyEditor 族与类型转换
#   scripts/lib_pilot_golden.sh jc1    # jackson-core 切片（矩阵 #3）：JsonParser 流式解析与 token 状态机（含吞吐基准）
#   scripts/lib_pilot_golden.sh jc2    # jackson-core 切片：JsonGenerator 往返
#   scripts/lib_pilot_golden.sh jc3    # jackson-core 切片：数字边角与编码（含数字吞吐基准）
#   scripts/lib_pilot_golden.sh jc4    # jackson-core 切片：JsonFactory 配置与特性开关
#   scripts/lib_pilot_golden.sh m2 --no-transpile   # 只重跑对账（复用已生成 scratch）
#   PILOT_PROFILE=release scripts/lib_pilot_golden.sh jc1   # 编译档（dev 缺省 / dev-opt / release / release-small）；性能测点用 release
#   scripts/lib_pilot_golden.sh Lang3StringUtilsMain        # 单文件 pilot：文件名即模式，cp 由脚本内 CP 表解析（2026-10-12 迁移批）
#   scripts/lib_pilot_golden.sh guava/GuavaCollectionsDemo  # 多文件项目：projects/<库>/src 全部源，入口 FQCN，
#                                                           # cp 取 projects/<库>/form.toml；golden 文件名 <库>_<入口简名>
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
MODE="${1:?用法: $0 m1|m2|m3|m4|m5|sc1..sc4|sb1..sb3|jc1..jc4|<MainName>|<库>/<入口> [--no-transpile|--emit-only]}"
TRANSPILE=1
EMIT_ONLY=0
[[ "${2:-}" == "--no-transpile" ]] && TRANSPILE=0
# --emit-only：只做转译（--stop-after emit）即退，scratch 供 scripts/compare_trees.sh 对照；
# scratch 根可用 SCRATCH_ROOT 覆盖（缺省 $REPO_ROOT/build），双提交树对比时两轮各给不同根
[[ "${2:-}" == "--emit-only" ]] && EMIT_ONLY=1

cd "$REPO_ROOT/tests/lib_pilot"
mkdir -p golden

# ---- 2026-10-12 迁移批：单文件 pilot 与多文件项目两种直达模式 ----
# 单文件 pilot 的 cp 表（条目名 = 依赖锁 artifactId；jar 路径按锁序解析，与 e2e_form 同口径）
# 2026-10-13 放量迁入（docs/plans/2026-10-13-e2e-libpilot-rollout-draft.md §二）：22 绿例 + AsmTransformMain
# + Slf4jBasicMain 已 git mv 入 tests/e2e/{75_slf4j,78_lang3,79_io,80_servlet,81_jpa,85_spring_core,
# 88_log4j2,89_asm}（目录级 form.toml 承接 cp），本表同步删去已迁入口的死行；表内只剩未进管道的
# pilot（其 golden 直达复现入口保留）。
pilot_cp_table() {
    case "$1" in
    JodaBasicMain|JodaChronologyMain|JodaPeriodMain) echo joda-time ;;
    OgnlCollectionMain|OgnlContextMain|OgnlEvalMain) echo ognl,javassist ;;
    JsoupParseMain|JsoupSelectorMain) echo jsoup ;;
    CaffeineManualMain|CaffeineWeigherMain) echo caffeine ;;
    YamlRoundTripMain|YamlTypesMain) echo snakeyaml ;;
    HoconParseMain) echo config ;;
    AssertJChainMain) echo assertj-core ;;
    FastjsonPathMain|FastjsonStreamMain) echo fastjson2 ;;
    LogbackConfigMain|LogbackPatternMain) echo logback-classic,logback-core,slf4j-api ;;
    DatabindAnnotationMain|DatabindDeserMain|DatabindSerMain) echo jackson-databind,jackson-core,jackson-annotations ;;
    SpringProxyJdkMain) echo spring-aop,spring-core,spring-jcl ;;
    KotlinStdlibPilotMain) echo kotlin-stdlib ;;
    *) return 1 ;;
    esac
}

# 依赖锁条目名（逗号分隔）→ 冒号连接的 jar 绝对路径（锁序；缺条目报错）
resolve_cp() {
    [[ -z "$1" ]] && { echo ""; return 0; }
    python3 - "$LOCK" "$1" "$REPO_ROOT/scripts" <<'PY'
import sys
from pathlib import Path
sys.path.insert(0, sys.argv[3])
import e2e_form
want = set(x for x in sys.argv[2].split(",") if x)
entries = e2e_form.lock_entries(Path(sys.argv[1]))
have = {n for n, _ in entries}
miss = [n for n in sorted(want) if n not in have]
if miss:
    sys.stderr.write("依赖锁缺条目: " + ",".join(miss) + "（先跑 scripts/fetch_pilot_deps.sh --no-scan）\n")
    sys.exit(2)
print(":".join(str(p) for n, p in entries if n in want))
PY
}

# 项目 form.toml 的 cp 清单 → 逗号分隔条目名
form_cp() {
    python3 -c '
import sys, tomllib
print(",".join(tomllib.loads(open(sys.argv[1], encoding="utf-8").read()).get("cp", [])))' "$1"
}

# 项目 form.toml 的可选 deps 键 → 该项目依赖锁的仓库相对路径（空 = 主模块锁）。
# 双版本并存（docs/plans/2026-10-13-lib-pilot-dual-version-mechanism.md）：冲突项目把 deps 指到
# deps-extra/<模块>/target/deps.lock.toml；锁内 jar 路径（pilot-libs/…）相对锁所在目录解析，
# 与 e2e_form.lock_entries 同口径
form_deps() {
    python3 -c '
import sys, tomllib
print(tomllib.loads(open(sys.argv[1], encoding="utf-8").read()).get("deps", ""))' "$1"
}

GOLDEN_ID="$MODE"          # golden 文件名（既有 id 模式 = id；项目模式 = <库>_<入口简名>）
MODE_TAG="$(echo "$MODE" | tr "/" "_")"   # /tmp 日志文件名（项目模式 MODE 含 /，须打平）
SRCDIR="."                 # 源目录（"." = tests/lib_pilot 平铺单文件）
MAIN_ARGS=()               # rava build 额外参数（多文件项目须 --main 声明入口）

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
sb1|sb2|sb3)
    # spring-beans 切片（矩阵 #12）：同 sc 口径，用户 main 为唯一入口；spring-beans 依赖 spring-core / spring-jcl
    case "$MODE" in
        sb1) MAIN=SpringBeanUtilsMain ;;
        sb2) MAIN=SpringBeanWrapperMain ;;
        sb3) MAIN=SpringPropertyEditorMain ;;
    esac
    CP="$LIBS/spring-beans-6.2.19.jar:$LIBS/spring-core-6.2.19.jar:$LIBS/spring-jcl-6.2.19.jar"
    LIB_ARGS=(--deps "$LOCK" --cp spring-beans,spring-core,spring-jcl)
    ;;
jc1|jc2|jc3|jc4)
    # jackson-core 切片（矩阵 #3）：同 sc 口径，用户 main 为唯一入口，库类按档案调用链翻译
    case "$MODE" in
        jc1) MAIN=JacksonParserMain ;;
        jc2) MAIN=JacksonGeneratorMain ;;
        jc3) MAIN=JacksonNumbersEncodingMain ;;
        jc4) MAIN=JacksonFactoryConfigMain ;;
    esac
    CP="$LIBS/jackson-core-2.22.3.jar"
    LIB_ARGS=(--deps "$LOCK" --cp jackson-core)
    ;;
*)
    # 直达模式一：单文件 pilot（tests/lib_pilot/<MainName>.java 存在即命中）
    if [[ -f "$MODE.java" ]]; then
        MAIN="$MODE"
        CPN="$(pilot_cp_table "$MODE")" || { echo "cp 表无此 pilot：$MODE" >&2; exit 2; }
        CP="$(resolve_cp "$CPN")" || exit 2
        LIB_ARGS=(--deps "$LOCK" --cp "$CPN")
    # 直达模式二：多文件项目 projects/<库>/src（入口可 FQCN 或简单名）
    elif [[ "$MODE" == */* && -d "projects/${MODE%%/*}/src" ]]; then
        LIB="${MODE%%/*}"
        MAIN="${MODE#*/}"
        SRCDIR="projects/$LIB/src"
        # 入口给简单名时推导 FQCN（README：入口可 FQCN 或简单名；java 须带 package 全名）：
        # SRCDIR 下找文件名 == <MAIN>.java 的源文件，读其 ^package 声明拼 FQCN；无包声明保持原值
        if [[ "$MAIN" != *.* ]]; then
            _src="$(find "$SRCDIR" -name "$MAIN.java" | sort | head -n 1)"
            if [[ -n "$_src" ]]; then
                _pkg="$(sed -n 's/^package[[:space:]]\+\([A-Za-z0-9_.]*\).*/\1/p' "$_src" | head -n 1)"
                [[ -n "$_pkg" ]] && MAIN="$_pkg.$MAIN"
            fi
        fi
        GOLDEN_ID="${LIB}_${MAIN##*.}"
        # form.toml 可选 deps 键：该项目依赖锁（仓库相对路径）。读到即 cp 解析与 --deps 都改用该锁
        # （deps-extra 双版本模块，锁内 path 相对位指向该模块 pilot-libs）；无键沿用主模块锁，行为不变
        FORM_DEPS="$(form_deps "projects/$LIB/form.toml")"
        if [[ -n "$FORM_DEPS" ]]; then
            LOCK="$REPO_ROOT/$FORM_DEPS"
            [ -f "$LOCK" ] || { echo "缺依赖锁 $LOCK：先跑 scripts/fetch_pilot_deps.sh --no-scan" >&2; exit 2; }
        fi
        CPN="$(form_cp "projects/$LIB/form.toml")"
        CP="$(resolve_cp "$CPN")" || exit 2
        if [[ -n "$CPN" ]]; then
            LIB_ARGS=(--deps "$LOCK" --cp "$CPN")
        else
            LIB_ARGS=()   # 零依赖项目（纯 JDK 面）
        fi
        MAIN_ARGS=(--main "$MAIN")
    else
        echo "未知模式: $MODE（既有 id / <MainName> / <库>/<入口>）" >&2; exit 2
    fi
    ;;
esac

# 源集合：平铺单文件 = "$MAIN.java"；项目 = SRCDIR 下全部 .java（排序稳定）
SOURCES=()
if [[ "$SRCDIR" == "." ]]; then
    SOURCES=("$MAIN.java")
else
    while IFS= read -r f; do SOURCES+=("$f"); done < <(find "$SRCDIR" -name '*.java' | sort)
    [[ ${#SOURCES[@]} -gt 0 ]] || { echo "$SRCDIR 下无 .java" >&2; exit 2; }
fi
CP_ARGS=()
[[ -n "$CP" ]] && CP_ARGS=(-cp "$CP")

echo "== [1/3] JVM 侧 golden（真 jar）=="
rm -rf classes && "$JAVAC" -d classes "${CP_ARGS[@]}" "${SOURCES[@]}"
# 三测点（运行 / 构建耗时、二进制大小）与 JVM 运行耗时对照，结尾汇总一行 METRICS
_t0=$EPOCHREALTIME
"$JAVA" -Dstdout.encoding=UTF-8 -cp "classes${CP:+:$CP}" "$MAIN" > "golden/${GOLDEN_ID}_jvm.txt"
JVM_S=$(python3 -c "print(f'{$EPOCHREALTIME-$_t0:.3f}')")
echo "JVM 侧 $(wc -l < "golden/${GOLDEN_ID}_jvm.txt" | tr -d ' ') 行"

echo "== [2/3] 转译 + 编译 + 运行（翻译 crate）=="
cd "$REPO_ROOT"
SCRATCH="${SCRATCH_ROOT:-$REPO_ROOT/build}/$(python3 - "$MAIN" <<'PY'
import re, sys
print(re.sub(r'(?<=[a-z0-9])(?=[A-Z])', '_', sys.argv[1]).lower())
PY
)"
TRANSPILE_S=-
# rava build 在 cd "$REPO_ROOT" 后调用：源列表须锚仓库根前缀（main 既有口径是
# tests/lib_pilot/$MAIN.java 硬编码；SOURCES 相对 tests/lib_pilot，此处统一补前缀）
RAVA_SOURCES=()
for _s in "${SOURCES[@]}"; do RAVA_SOURCES+=("tests/lib_pilot/$_s"); done
if [[ "$TRANSPILE" == 1 ]]; then
    _t0=$EPOCHREALTIME
    # shellcheck disable=SC2046  # 源列表与既有 id 模式同一入口；项目模式多源 + --main 声明入口
    "$RAVA" build "${RAVA_SOURCES[@]}" "${MAIN_ARGS[@]}" "${CORPUS_JDK_ARGS[@]}" --clean "${LIB_ARGS[@]}" \
        --out "$SCRATCH" --stop-after emit > "/tmp/${MODE_TAG}_transpile.log" 2>&1 \
        || { echo "转译失败，见 /tmp/${MODE_TAG}_transpile.log" >&2; exit 1; }
    TRANSPILE_S=$(python3 -c "print(f'{$EPOCHREALTIME-$_t0:.1f}')")
fi
if [[ "$EMIT_ONLY" == 1 ]]; then
    echo "emit-only：$SCRATCH"
    exit 0
fi
_t0=$EPOCHREALTIME
PROFILE_ARGS=()
[[ -n "${PILOT_PROFILE:-}" && "$PILOT_PROFILE" != dev ]] && PROFILE_ARGS=(--"$PILOT_PROFILE")
"$RAVA" compile "$SCRATCH" "${PROFILE_ARGS[@]}" > "/tmp/${MODE_TAG}_cargo.err" 2>&1 \
    || { echo "编译失败，见 /tmp/${MODE}_cargo.err 与 $SCRATCH/build_status.json" >&2; exit 1; }
COMPILE_S=$(python3 -c "print(f'{$EPOCHREALTIME-$_t0:.1f}')")
EXE="$(python3 -c 'import json,sys;print(json.load(open(sys.argv[1]))["exe"])' "$SCRATCH/build_status.json")"
_t0=$EPOCHREALTIME
"$EXE" > "$REPO_ROOT/tests/lib_pilot/golden/${GOLDEN_ID}_rs.txt" 2>>"/tmp/${MODE_TAG}_cargo.err" \
    || { echo "运行失败，见 /tmp/${MODE_TAG}_cargo.err" >&2; exit 1; }
RUN_S=$(python3 -c "print(f'{$EPOCHREALTIME-$_t0:.3f}')")
# 程序自报的吞吐基准行（stderr 的 JC_BENCH；JVM 侧直接打在本日志）
grep -h "^JC_BENCH" "/tmp/${MODE_TAG}_cargo.err" | sed "s/^/rs /" || true
echo "METRICS ${MODE} profile=${PILOT_PROFILE:-dev} run_s=${RUN_S} jvm_run_s=${JVM_S} transpile_s=${TRANSPILE_S} compile_s=${COMPILE_S} bin_bytes=$(stat -c %s "$EXE")"

echo "== [3/3] 逐字对账 =="
if diff -u "tests/lib_pilot/golden/${GOLDEN_ID}_jvm.txt" "tests/lib_pilot/golden/${GOLDEN_ID}_rs.txt"; then
    echo "GOLDEN OK（${MODE}）"
else
    echo "GOLDEN DIFF（${MODE}）" >&2
    exit 1
fi
