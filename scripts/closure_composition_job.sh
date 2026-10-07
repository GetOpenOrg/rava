#!/usr/bin/env bash
# 闭包构成分析作业（服务器以作业模式运行；本机不跑）：对选定用例跑 rava closure，产出 closure.json（gzip）
# 供 scripts/closure_composition.py 按机制拆分；可带反事实切除（--cut-file）做「整块去掉能减多少类」的归因。
#
# 用法：scripts/closure_composition_job.sh [--cut-file F --tag T | --cut-sets "名1 名2 …"] <用例>...
#   用例：hello | collectors | deepcopy | jcasasl | s0boot（s0boot 需 dev 级内存，见报告）
#   --cut-file F：反事实切除条目文件（同 rava closure --cut-file；不健全，只作归因）；--tag T 为产物名后缀
#   --cut-sets：依次取 scripts/closure_composition_cuts/<名>.txt 作切除、名作 tag，对每个用例各跑一遍
# 产物：build/ccomp/<用例>[.<tag>].json.gz、.out（stdout 摘要）、.err（stderr 尾与 /usr/bin/time）
#       作业取回：--fetch 'build/ccomp/**'
# 口径：docs/reports/2026-10-07-closure-composition.md
set -uo pipefail
REPO="$(cd "$(dirname "$0")/.." && pwd)"
OUT="$REPO/build/ccomp"
mkdir -p "$OUT"
CUT=(); TAG=""; SETS=""
while [[ $# -gt 0 ]]; do
    case "$1" in
        --cut-file) CUT=(--cut-file "$(cd "$(dirname "$2")" && pwd)/$(basename "$2")"); shift 2 ;;
        --tag) TAG=".$2"; shift 2 ;;
        --cut-sets) SETS="$2"; shift 2 ;;
        *) break ;;
    esac
done
[[ $# -gt 0 ]] || { echo "用法：$0 [--cut-file F --tag T | --cut-sets 名单] <用例>..."; exit 2; }
if [[ -n "$SETS" ]]; then
    rc_sets=0
    for s in $SETS; do
        bash "$0" --cut-file "$(cd "$(dirname "$0")" && pwd)/closure_composition_cuts/$s.txt" --tag "$s" "$@" || rc_sets=1
    done
    exit $rc_sets
fi
step() { echo "═══ $(date '+%H:%M:%S') $*"; }
. "$REPO/scripts/rava_env.sh" "$REPO"
. "$REPO/scripts/corpus_jdk.sh" "$REPO"

# TestJcaSasl：JCA provider 顺序线的合成例（docs/plans/2026-10-06-c1d-jca-provider-order.md §7.4），语料中无此文件
jcasasl_src() {
    local d="$OUT/src/jcasasl"; mkdir -p "$d"
    cat >"$d/TestJcaSasl.java" <<'JEOF'
import java.security.*;
import java.util.*;
import javax.security.sasl.*;
public class TestJcaSasl {
    public static void main(String[] args) throws Exception {
        System.out.println(Security.getProviders().length > 0);
        System.out.println(MessageDigest.getInstance("SHA").getDigestLength());
        Enumeration<SaslClientFactory> e = Sasl.getSaslClientFactories();
        System.out.println(e.hasMoreElements());
        SaslClient c = Sasl.createSaslClient(new String[] {"PLAIN"}, null, "ldap", "h", null, null);
        System.out.println(c == null);
    }
}
JEOF
    echo "$d/TestJcaSasl.java"
}

# S0 最小 Boot 样例：取包 → javac → 类目录 + --deps/--cp（与 scripts/api_surface_job.sh 同口径）
S0_ARGS=()
s0boot_prepare() {
    local app="$REPO/tests/lib_pilot/s0_boot" deps="$REPO/tests/lib_pilot/deps/target" cls="$OUT/s0boot_classes"
    export MAVEN_OPTS="-Dmaven.repo.local=$REPO/build/m2 ${MAVEN_OPTS:-}"
    bash "$REPO/scripts/fetch_pilot_deps.sh" --no-scan >"$OUT/s0boot_fetch.log" 2>&1 || { tail -30 "$OUT/s0boot_fetch.log"; return 1; }
    local names cp main
    names="$(grep -v '^\s*#' "$app/jars.txt" | grep -v '^\s*$' | paste -sd, -)"
    cp="$(uv run -q --project "$REPO" python - "$deps/deps.lock.toml" "$names" <<'PY'
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
)" || return 1
    rm -rf "$cls"; mkdir -p "$cls"
    "$JAVA_HOME/bin/javac" -g -parameters -cp "$cp" -d "$cls" $(find "$app/src" -name '*.java') || return 1
    cp -R "$app/resources/." "$cls/"
    main="$(grep -rl 'static void main' "$app/src" | head -1 | sed "s|$app/src/||; s|\.java$||; s|/|.|g")"
    S0_ARGS=("$cls" --main "$main" --deps "$deps/deps.lock.toml" --cp "$names")
}

input_of() {
    case "$1" in
        hello) echo "$REPO/tests/e2e/01_basics/HelloWorld.java" ;;
        collectors) echo "$REPO/tests/e2e/04_collections/CollectorsDemo.java" ;;
        deepcopy) echo "$REPO/tests/e2e/23_algorithms/DeepCopy.java" ;;
        jcasasl) jcasasl_src ;;
        *) return 1 ;;
    esac
}

rc_all=0
for c in "$@"; do
    step "闭包 $c${TAG}"
    if [[ "$c" == s0boot ]]; then
        s0boot_prepare || { echo "s0boot 准备失败"; rc_all=1; continue; }
        IN=("${S0_ARGS[@]}")
    else
        f="$(input_of "$c")" || { echo "未知用例 $c"; rc_all=1; continue; }
        IN=("$f")
    fi
    base="$OUT/$c$TAG"
    timer=(); [[ -x /usr/bin/time ]] && timer=(/usr/bin/time -v)
    t0=$SECONDS
    timeout "${CCOMP_TIMEOUT:-3000}" "${timer[@]}" "$RAVA" closure "${IN[@]}" "${CORPUS_JDK_ARGS[@]}" "${CUT[@]}" \
        -o "$base.json" >"$base.out" 2>"$base.err"
    rc=$?
    echo "闭包 $c$TAG rc=$rc 耗时 $((SECONDS - t0))s"
    grep -E "Maximum resident|Elapsed" "$base.err" | sed 's/^\s*/  /'
    grep -E '^\s*"(classes|methods)":' "$base.out" | head -2
    tail -c 2000 "$base.err" | grep -vE "^\s+(Maximum|Elapsed|Command|User|System|Percent|Average|Exit|Swaps|File|Socket|Signals|Page|Voluntary|Involuntary|Minor|Major)" | tail -8
    if [[ $rc == 0 && -s "$base.json" ]]; then gzip -f "$base.json"; else rc_all=1; rm -f "$base.json"; fi
    # stdout 只留 summary 段之后的前 200 行（--why / --flows 不在本作业使用）
    head -c 200000 "$base.out" >"$base.out.tmp" && mv "$base.out.tmp" "$base.out"
done
step "完成"
exit $rc_all
