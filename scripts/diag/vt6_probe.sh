#!/usr/bin/env bash
# a3-T6 虚拟线程规模剖析（服务器作业用，Linux）：参考 JDK 两次运行 TestVirtualThreadScale 取 expected，
# 转译 tests/perf/VirtualThreadScale 后按各模式采样内存时间线与耗时。产物在 build/vt6/。
# 用法：scripts/diag/vt6_probe.sh <dev|release> [n=100000] [模式...（缺省 sleep park unstarted split）]
# 环境：SKIP_JDK=1 跳过参考 JDK 运行；PERF=1 每模式另跑一次 perf record，报告写 build/vt6/<profile>_<mode>_perf*.txt
set -uo pipefail
cd "$(dirname "$0")/../.."
PROFILE=${1:-dev}; N=${2:-100000}; shift 2 2>/dev/null || shift $#
MODES=${*:-sleep park unstarted split}
OUT=build/vt6; mkdir -p $OUT
JH=$(scripts/fetch_reference_jdk.sh) || exit 1
echo "== host: $(uname -r) $(uname -m) cpus=$(nproc) mem=$(free -m | awk '/Mem:/{print $2}')MiB perf=$(command -v perf || echo none)"

if [ "$PROFILE" = dev ] && [ -z "${SKIP_JDK:-}" ]; then
  for k in 1 2; do
    /usr/bin/time -v "$JH/bin/java" tests/e2e/60_real_threads/TestVirtualThreadScale.java > $OUT/jdk$k.txt 2> $OUT/jdk$k.time
    echo "== jdk run $k: $(grep -E 'Elapsed|Maximum resident' $OUT/jdk$k.time | tr -s ' ' | tr '\n' ' ')"
  done
  cmp $OUT/jdk1.txt $OUT/jdk2.txt && echo "== jdk outputs identical" && cat $OUT/jdk1.txt
fi

REL=(); [ "$PROFILE" = release ] && REL=(--release)
WS=$OUT/ws_$PROFILE
/usr/bin/time -v build/analyzer-target/release/rava build tests/perf/VirtualThreadScale.java --stop-after compile \
  --out $WS --clean --java-home "$JH" "${REL[@]}" > $OUT/build_$PROFILE.log 2>&1
rc=$?
echo "== rava build ($PROFILE) rc=$rc $(grep -E 'Elapsed|Maximum resident' $OUT/build_$PROFILE.log | tr -s ' ' | tr '\n' ' ')"
[ $rc -eq 0 ] || { tail -40 $OUT/build_$PROFILE.log; exit 1; }
DIR=debug; [ "$PROFILE" = release ] && DIR=release
BIN=build/target/$DIR/virtual_thread_scale
[ -x "$BIN" ] || BIN=$(find build -path "*/$DIR/virtual_thread_scale" -type f | head -1)
echo "== bin $BIN $(stat -c %s "$BIN") B"

for mode in $MODES; do
  ms=2000; [ $mode = unstarted ] && ms=3000
  # 计时：不带采样（smaps 读取持 mmap_lock，会拖慢被测进程）
  /usr/bin/time -v "$BIN" $N $ms $mode > $OUT/${PROFILE}_$mode.out 2> $OUT/${PROFILE}_$mode.err
  echo "== $PROFILE $mode n=$N rc=$? (timing)"
  cat $OUT/${PROFILE}_$mode.out
  grep -E '^phase|Elapsed|Maximum resident|context switches|User time|System time|Minor' $OUT/${PROFILE}_$mode.err
  # 内存分布：另跑一次带采样
  python3 scripts/diag/rss_timeline.py $OUT/${PROFILE}_$mode.tsv "$BIN" $N $ms $mode \
    > /dev/null 2> $OUT/${PROFILE}_${mode}_mem.err
  echo "== $PROFILE $mode rc=$? (memory)"
  grep -E '^phase|rss_timeline' $OUT/${PROFILE}_${mode}_mem.err
  if [ -n "${PERF:-}" ]; then
    P=$OUT/${PROFILE}_$mode
    perf record -F 499 -g -o $P.data -- "$BIN" $N $ms $mode > /dev/null 2> $P.perf_rec.log
    echo "== perf $mode rc=$? $(tail -2 $P.perf_rec.log | tr '\n' ' ')"
    perf report -i $P.data --stdio --no-children --sort comm,sym --percent-limit 0.3 -g none 2>/dev/null | head -200 > ${P}_perf_self.txt
    perf report -i $P.data --stdio --children --sort comm,sym --percent-limit 1.5 -g none 2>/dev/null | head -300 > ${P}_perf_incl.txt
    perf report -i $P.data --stdio --no-children --sort sym --percent-limit 2 -g caller,0.5,callee,function,percent --max-stack 40 2>/dev/null | head -1500 > ${P}_perf_calls.txt
    rm -f $P.data
    head -60 ${P}_perf_self.txt
  fi
done
rm -rf $WS
