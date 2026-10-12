#!/bin/zsh
# GraalVM 耗时验证：JVM 运行 vs native-image 构建/运行（2026-10-04 首轮原始版，自 html-to-markdown/graalvm-bench/run_bench.sh 移入）
# 演进版（多 MODES 计时、REPEAT 中位数、PGO、读 tests/graalvm_bench/cases.tsv）见 scripts/graalvm_bench.sh。
#
# 每个用例的流程：
#   1. javac 编译（GraalVM JDK 21）
#   2. JVM 运行计时（超时上限 RUN_TIMEOUT 秒）
#   3. 第二类用例额外跑一次 tracing agent，生成反射/序列化/动态代理配置
#   4. native-image 构建（--no-fallback，计时，超时上限 BUILD_TIMEOUT 秒）
#   5. 原生二进制运行计时
#   6. diff JVM 输出与原生输出，验证正确性
#
# 结果汇总：results.tsv / REPORT.md

set -u
ROOT="$(cd "$(dirname "$0")" && pwd)"
JAVA_HOME_BENCH=${JAVA_HOME_BENCH:-/Library/Java/JavaVirtualMachines/graalvm-21.jdk/Contents/Home}
export PATH="$JAVA_HOME_BENCH/bin:$PATH"
export LANG=en_US.UTF-8

BUILD="$ROOT/build"
RUN_TIMEOUT=${RUN_TIMEOUT:-300}
BUILD_TIMEOUT=${BUILD_TIMEOUT:-900}
TSV="$ROOT/results.tsv"

# ── 用例清单（相对路径 | 类别 1=纯计算 2=反射/序列化/动态代理） ──
CAT1=(
  23_algorithms/LynchBell.java
  23_algorithms/Factorion.java
  23_algorithms/FWord.java
  23_algorithms/FibonacciMatrixExponentiation.java
  23_algorithms/IQPuzzle.java
  23_algorithms/FourIsTheNumberOfLetters.java
  23_algorithms/FractionReduction.java
  23_algorithms/PartitionInteger.java
  23_algorithms/PrimorialNumbers.java
  23_algorithms/RailwayCircuit.java
  23_algorithms/SelfNumbers.java
  23_algorithms/UnprimeableNumbers.java
)
CAT2=(
  72_http/TestHttpLoopbackSync.java
  72_http/TestHttpLoopbackAsync.java
  73_jndi_script/TestJndiNoProvider.java
  23_algorithms/DeepCopy.java
  23_algorithms/StockTrans.java
  35_io/TestSerialDefaultSuid.java
  35_io/TestSerialLookupPairing.java
  35_io/TestSerialProxyForm.java
  62_reflection/TestFieldHandleProvenance.java
  53_io_api/TestUrlProtocolOpen.java
  35_io/RecordsSerializationTest.java
)

mkdir -p "$BUILD"
echo -e "name\tcategory\tjvm_secs\tnative_build_secs\tnative_run_secs\tjvm_exit\tnative_exit\toutput_match" > "$TSV"

# 计时运行：timeout 包裹 /usr/bin/time -p，real 值写入 $timer 文件
timed_run() { # $1=timeout_secs $2=timer_file rest=cmd
  local secs=$1 timer=$2; shift 2
  timeout -k 10 "$secs" /usr/bin/time -p "$@" 2> "$timer"
  local rc=$?
  # /usr/bin/time 的 real 行混在 stderr 里，agent/程序的 stderr 一并写入 timer，
  # 取最后一行 real
  return $rc
}

get_real() { awk '/^real/{v=$2} END{print (v==""?"":v)}' "$1" 2>/dev/null }

# agent 抓不到的配置项，按用例手工补齐（实测发现的缺口）
patch_agent_config() { # $1=name $2=config_dir
  local name=$1 cfgdir=$2
  if [ "$name" = "TestSerialProxyForm" ]; then
    # ObjectStreamClass 在静态初始化阶段走的序列化构造器路径，agent 未捕获；
    # 需在 serialization-config.json 补注册 TestSerialProxyForm$Tally$Form
    python3 - "$cfgdir/serialization-config.json" <<'PYEOF'
import json, sys
p = sys.argv[1]
cfg = json.load(open(p))
if not any(t["name"] == "TestSerialProxyForm$Tally$Form" for t in cfg["types"]):
    cfg["types"].append({"name": "TestSerialProxyForm$Tally$Form"})
json.dump(cfg, open(p, "w"), indent=2)
PYEOF
  fi
}

bench_one() { # $1=src_rel $2=category
  local src=$1 cat=$2
  local name="${src:t:r}"
  local dir="$BUILD/$name"
  mkdir -p "$dir/classes" "$dir/bin" "$dir/config"

  echo "==== [$name] (cat$cat) ===="

  # 1. 编译
  if ! javac -encoding UTF-8 -d "$dir/classes" "$ROOT/tests/$src" 2> "$dir/javac.log"; then
    echo "[$name] javac FAILED"; head -5 "$dir/javac.log"
    echo -e "$name\t$cat\tCOMPILE_FAIL\t-\t-\t-\t-\t-" >> "$TSV"; return
  fi

  # 2. JVM 运行计时
  local jvm_rc jvm_real
  timed_run "$RUN_TIMEOUT" "$dir/jvm.time" java -XX:+UseSerialGC -cp "$dir/classes" "$name" > "$dir/jvm.out"
  jvm_rc=$?
  jvm_real=$(get_real "$dir/jvm.time")
  echo "[$name] JVM: ${jvm_real}s (exit=$jvm_rc)"
  if [ $jvm_rc -ne 0 ] && [ $jvm_rc -ne 124 ]; then
    echo "[$name] JVM run failed:"; head -5 "$dir/jvm.out"
  fi

  # 3. tracing agent（仅二类）
  if [ "$cat" = "2" ]; then
    timeout -k 10 "$RUN_TIMEOUT" java -agentlib:native-image-agent=config-output-dir="$dir/config" \
      -cp "$dir/classes" "$name" > "$dir/agent.out" 2> "$dir/agent.err"
    echo "[$name] agent run exit=$?"
    patch_agent_config "$name" "$dir/config"
  fi

  # 4. native-image 构建
  local ni_rc build_real ni_args
  ni_args=(--no-fallback -cp "$dir/classes" -o "$dir/bin/$name" "$name")
  if [ "$cat" = "2" ] && [ -d "$dir/config" ] && ls "$dir/config"/*.json >/dev/null 2>&1; then
    ni_args=(-H:ConfigurationFileDirectories="$dir/config" "${ni_args[@]}")
  fi
  # 实测补充项：jar 协议需显式启用（agent 不会生成该开关）
  if [ "$name" = "TestUrlProtocolOpen" ]; then
    ni_args=(--enable-url-protocols=jar "${ni_args[@]}")
  fi
  timed_run "$BUILD_TIMEOUT" "$dir/ni.time" native-image "${ni_args[@]}" > "$dir/ni.log" 2>&1
  ni_rc=$?
  build_real=$(get_real "$dir/ni.time")
  if [ $ni_rc -ne 0 ]; then
    echo "[$name] native-image BUILD FAILED (rc=$ni_rc, ${build_real}s)"; tail -5 "$dir/ni.log"
    echo -e "$name\t$cat\t${jvm_real:-?}\tBUILD_FAIL\t-\t$jvm_rc\t-\t-" >> "$TSV"; return
  fi
  echo "[$name] native-image build: ${build_real}s"

  # 5. 原生运行计时
  local nat_rc nat_real
  timed_run "$RUN_TIMEOUT" "$dir/native.time" "$dir/bin/$name" > "$dir/native.out"
  nat_rc=$?
  nat_real=$(get_real "$dir/native.time")
  echo "[$name] native: ${nat_real}s (exit=$nat_rc)"

  # 6. 输出一致性
  local match
  if diff -q "$dir/jvm.out" "$dir/native.out" > /dev/null 2>&1; then
    match="SAME"
  else
    match="DIFF"
    echo "[$name] output DIFF:"; diff "$dir/jvm.out" "$dir/native.out" | head -5
  fi

  echo -e "$name\t$cat\t${jvm_real:-?}\t$build_real\t${nat_real:-?}\t$jvm_rc\t$nat_rc\t$match" >> "$TSV"
}

echo "java: $(java -version 2>&1 | head -1)"
echo "native-image: $(native-image --version 2>&1 | head -1)"
echo "start: $(date '+%F %T')"

for src in "${CAT1[@]}"; do bench_one "$src" 1; done
for src in "${CAT2[@]}"; do bench_one "$src" 2; done

echo "end: $(date '+%F %T')"
echo "DONE, results at $TSV"
