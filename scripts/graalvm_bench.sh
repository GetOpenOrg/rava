#!/usr/bin/env bash
# GraalVM / rava 运行性能对照：同一批用例在多种参数条件下计时（R1 运行性能与产品对标的外部参照）
#
# 用法：scripts/graalvm_bench.sh [用例名...]        # 缺省跑 tests/graalvm_bench/cases.tsv 全部
#   环境变量：
#     JAVA_HOME_BENCH  GraalVM JDK 根目录（缺省 macOS 安装位置）
#     MODES            逗号分隔的计时条件（缺省 jvm-serial,jvm-g1,ni-default,ni-o3,ni-pgo）
#                        jvm-serial / jvm-g1   JVM 运行（-XX:+UseSerialGC / 缺省 G1）
#                        ni-default            native-image 缺省优化（-O2）
#                        ni-o3                 native-image -O3 -march=native
#                        ni-pgo                native-image PGO（--pgo-instrument 采样 → --pgo）
#                        rava-debug / rava-release  scripts/run_tests.py 转译 + 编译 + 运行
#     REPEAT           每个运行计时重复次数，取中位数（缺省 3）
#     RUN_TIMEOUT / BUILD_TIMEOUT  运行 / 构建超时秒数（缺省 300 / 900）
#     OUT              输出目录（缺省 build/graalvm_bench）
#     HEAVY_LOCK       rava 模式外包的全机锁命令前缀（如 "python3 ../heavy_lock.py"；缺省不包）
#   输出：$OUT/<用例>/（产物与日志）、$OUT/results.tsv（长表：一行一个 用例 × 条件；
#         bin_bytes 为可执行文件字节数，JVM 条件记 -）
#
# 每例流程：javac → JVM 输出作为对照基准 → 逐条件构建计时 + 运行计时（中位数）+ 二进制大小 → 与 JVM 输出逐字对照
# 报告与结论：docs/reports/2026-10-04-graalvm-baseline.md

set -u
REPO="$(cd "$(dirname "$0")/.." && pwd)"
JAVA_HOME_BENCH=${JAVA_HOME_BENCH:-/Library/Java/JavaVirtualMachines/graalvm-21.jdk/Contents/Home}
export PATH="$JAVA_HOME_BENCH/bin:$PATH"
export LANG=en_US.UTF-8

CASES="$REPO/tests/graalvm_bench/cases.tsv"
OUT=${OUT:-$REPO/build/graalvm_bench}
RUN_TIMEOUT=${RUN_TIMEOUT:-300}
BUILD_TIMEOUT=${BUILD_TIMEOUT:-900}
REPEAT=${REPEAT:-3}
MODES=${MODES:-jvm-serial,jvm-g1,ni-default,ni-o3,ni-pgo}
HEAVY_LOCK=${HEAVY_LOCK:-}
TSV="$OUT/results.tsv"
HOST="$(uname -s)-$(uname -m)-$(hostname -s 2>/dev/null || hostname)"

mkdir -p "$OUT"
[ -f "$TSV" ] || printf 'host\tname\tcategory\tmode\tbuild_secs\trun_secs\texit\toutput_match\tbin_bytes\n' > "$TSV"

# 单次计时：bash 内建 time（不依赖 /usr/bin/time），real 秒数写入 $2
timed_run() { # $1=timeout_secs $2=timer_file rest=cmd
  local secs=$1 timer=$2 rc; shift 2
  local TIMEFORMAT=%R
  { time timeout -k 10 "$secs" "$@" 2>> "$timer.stderr"; rc=$?; } 2> "$timer"
  return $rc
}

# 重复 REPEAT 次取 real 中位数；stdout 只保留第一次（$2），退出码取第一次，写入全局 LAST_RC
LAST_RC=0
median_run() { # $1=timer_prefix $2=stdout_file rest=cmd
  local prefix=$1 outf=$2 i rc vals=(); shift 2
  for i in $(seq 1 "$REPEAT"); do
    if [ "$i" = 1 ]; then
      timed_run "$RUN_TIMEOUT" "$prefix.$i" "$@" > "$outf"; rc=$?; LAST_RC=$rc
    else
      timed_run "$RUN_TIMEOUT" "$prefix.$i" "$@" > /dev/null; rc=$?
    fi
    vals+=("$(cat "$prefix.$i")")
    [ "$rc" = 124 ] && break      # 超时不再重复
  done
  printf '%s\n' "${vals[@]}" | sort -n | awk '{a[NR]=$1} END{print (NR?a[int((NR+1)/2)]:"")}'
}

# 清单里登记的 serialization-config 补注册（agent 盲区）
patch_serialization() { # $1=config_dir $2=逗号分隔类型名
  python3 - "$1/serialization-config.json" "$2" <<'PYEOF'
import json, sys
p, names = sys.argv[1], sys.argv[2].split(",")
cfg = json.load(open(p))
have = {t["name"] for t in cfg.get("types", [])}
cfg.setdefault("types", []).extend({"name": n} for n in names if n not in have)
json.dump(cfg, open(p, "w"), indent=2)
PYEOF
}

row() { # name cat mode build run exit match bin_bytes
  printf '%s\t%s\t%s\t%s\t%s\t%s\t%s\t%s\t%s\n' "$HOST" "$@" >> "$TSV"
}

bytes_of() { [ -f "$1" ] && wc -c < "$1" | tr -d ' ' || echo -; }

match_of() { diff -q "$1" "$2" > /dev/null 2>&1 && echo SAME || echo DIFF; }

# run_tests.py 的二进制命名（_to_bin_name）
bin_name_of() { python3 -c 'import re,sys; s=re.sub(r"([A-Z]+)([A-Z][a-z])",r"\1_\2",sys.argv[1]); print(re.sub(r"([a-z\d])([A-Z])",r"\1_\2",s).lower())' "$1"; }

bench_jvm() { # name cat dir mode gcflag
  local name=$1 cat=$2 dir=$3 mode=$4 gc=$5 real
  real=$(median_run "$dir/$mode.time" "$dir/$mode.out" java $gc -cp "$dir/classes" "$name")
  echo "[$name] $mode: run ${real}s (exit=$LAST_RC)"
  row "$name" "$cat" "$mode" - "$real" "$LAST_RC" "$(match_of "$dir/ref.out" "$dir/$mode.out")" -
}

bench_ni() { # name cat dir mode ni_args...
  local name=$1 cat=$2 dir=$3 mode=$4 build real rc; shift 4
  local args=("$@") bin="$dir/bin/${name}_$mode"
  case "$mode" in
    ni-o3) args+=(-O3 -march=native) ;;
    ni-pgo)
      if ! native-image "${args[@]}" --pgo-instrument -o "$dir/bin/${name}_inst" "$name" > "$dir/$mode.inst.log" 2>&1 \
         || ! (cd "$dir" && rm -f default.iprof && timeout -k 10 "$RUN_TIMEOUT" "bin/${name}_inst" > /dev/null 2>&1); then
        echo "[$name] $mode: instrument FAILED"; row "$name" "$cat" "$mode" INSTRUMENT_FAIL - - - -; return
      fi
      args+=(--pgo="$dir/default.iprof") ;;
  esac
  timed_run "$BUILD_TIMEOUT" "$dir/$mode.build.time" native-image "${args[@]}" -o "$bin" "$name" > "$dir/$mode.build.log" 2>&1
  rc=$?; build=$(cat "$dir/$mode.build.time")
  if [ $rc -ne 0 ]; then
    echo "[$name] $mode: BUILD FAILED (rc=$rc)"; tail -3 "$dir/$mode.build.log"
    row "$name" "$cat" "$mode" BUILD_FAIL - "$rc" - -; return
  fi
  real=$(median_run "$dir/$mode.time" "$dir/$mode.out" "$bin")
  echo "[$name] $mode: build ${build}s, run ${real}s, bin $(bytes_of "$bin") B (exit=$LAST_RC)"
  row "$name" "$cat" "$mode" "$build" "$real" "$LAST_RC" "$(match_of "$dir/ref.out" "$dir/$mode.out")" "$(bytes_of "$bin")"
}

bench_rava() { # name cat dir mode src_rel
  local name=$1 cat=$2 dir=$3 mode=$4 src=$5 prof=debug flag=() log elapsed build real bin snake
  [ "$mode" = rava-release ] && { prof=release; flag=(--release); }
  log="$dir/$mode.log"
  # --keep-artifacts：保住可执行文件供重复计时与量体积；取走后删本例 scratch
  (cd "$REPO" && $HEAVY_LOCK python3 scripts/run_tests.py -j 1 --keep-artifacts "${flag[@]}" --filter "/$name.java") > "$log" 2>&1
  # Elapsed: 13m59.7s  (transpile 6m51.4s, build 7m07.2s, run 0.59s)
  elapsed=$(grep -E '^Elapsed: .*\(transpile' "$log" | tail -1)
  build=$(python3 - "$elapsed" <<'PYEOF'
import re, sys
d = re.findall(r"(?:(\d+)m)?([\d.]+)s", sys.argv[1])
secs = [int(m or 0) * 60 + float(s) for m, s in d]
print(f"{secs[1] + secs[2]:.2f}" if len(secs) >= 4 else "-")
PYEOF
)
  snake=$(bin_name_of "$name")
  bin="$dir/bin/${name}_$mode"
  # run_tests.py 的输出根按 JDK 主版本分层：build/jdk<N>/{<snake>/, target/<prof>/<snake>}
  local root; for root in "$REPO"/build/jdk*/; do
    [ -f "$root/target/$prof/$snake" ] || continue
    cp "$root/target/$prof/$snake" "$bin"
    rm -rf "$root/$snake" "$root/target/$prof/$snake" "$root/target/$prof/$snake.d"
  done
  if [ ! -x "$bin" ] || ! grep -q "PASS" "$log"; then
    echo "[$name] $mode: FAILED（见 ${log}）"
    row "$name" "$cat" "$mode" "$build" - - FAIL -; return
  fi
  real=$(median_run "$dir/$mode.time" "$dir/$mode.out" "$bin")
  echo "[$name] $mode: transpile+build ${build}s, run ${real}s, bin $(bytes_of "$bin") B (exit=$LAST_RC)"
  row "$name" "$cat" "$mode" "$build" "$real" "$LAST_RC" "$(match_of "$dir/ref.out" "$dir/$mode.out")" "$(bytes_of "$bin")"
}

bench_one() { # $1=src_rel $2=category $3=extra_ni_args $4=serialization_extra
  local src=$1 cat=$2 extra=$3 ser=$4
  local name; name=$(basename "$src" .java)
  local dir="$OUT/$name"
  rm -rf "$dir"; mkdir -p "$dir/classes" "$dir/bin" "$dir/config"
  echo "==== [$name] (cat$cat) ===="

  if ! javac -encoding UTF-8 -d "$dir/classes" "$REPO/tests/e2e/$src" 2> "$dir/javac.log"; then
    echo "[$name] javac FAILED"; head -5 "$dir/javac.log"
    row "$name" "$cat" javac COMPILE_FAIL - - - -; return
  fi
  # 对照基准：JVM 一次运行的输出
  timeout -k 10 "$RUN_TIMEOUT" java -cp "$dir/classes" "$name" > "$dir/ref.out" 2> /dev/null

  local ni_args=(--no-fallback -cp "$dir/classes")
  if [[ ",$MODES," == *",ni-"* ]]; then
    [ "$extra" != "-" ] && read -r -a extra_arr <<< "$extra" && ni_args+=("${extra_arr[@]}")
    if [ "$cat" = "2" ]; then
      timeout -k 10 "$RUN_TIMEOUT" java -agentlib:native-image-agent=config-output-dir="$dir/config" \
        -cp "$dir/classes" "$name" > "$dir/agent.out" 2> "$dir/agent.err"
      [ "$ser" != "-" ] && patch_serialization "$dir/config" "$ser"
      ls "$dir/config"/*.json > /dev/null 2>&1 && ni_args+=(-H:ConfigurationFileDirectories="$dir/config")
    fi
  fi

  local mode
  for mode in ${MODES//,/ }; do
    case "$mode" in
      jvm-serial) bench_jvm "$name" "$cat" "$dir" "$mode" -XX:+UseSerialGC ;;
      jvm-g1)     bench_jvm "$name" "$cat" "$dir" "$mode" -XX:+UseG1GC ;;
      ni-*)       bench_ni "$name" "$cat" "$dir" "$mode" "${ni_args[@]}" ;;
      rava-*)     bench_rava "$name" "$cat" "$dir" "$mode" "$src" ;;
      *)          echo "未知条件 $mode"; exit 2 ;;
    esac
  done
}

echo "host: $HOST  nproc: $(getconf _NPROCESSORS_ONLN)"
echo "java: $(java -version 2>&1 | head -1)"
[[ ",$MODES," == *",ni-"* ]] && echo "native-image: $(native-image --version 2>&1 | head -1)"
echo "start: $(date '+%F %T')  MODES=$MODES REPEAT=$REPEAT"

grep -v '^#' "$CASES" | while IFS=$'\t' read -r src cat extra ser; do
  [ -z "$src" ] && continue
  if [ $# -gt 0 ] && [[ " $* " != *" $(basename "$src" .java) "* ]]; then continue; fi
  bench_one "$src" "$cat" "$extra" "$ser" < /dev/null
done

echo "end: $(date '+%F %T')"
echo "DONE, results at $TSV"
