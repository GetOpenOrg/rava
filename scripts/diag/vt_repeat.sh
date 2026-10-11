#!/usr/bin/env bash
# 虚拟线程偶发失败统计（服务器作业用）：转译一个 e2e 用例，顺序 + 8 进程并行重复运行，逐次与 expected 对照，
# 按差异行归类计数；可选参考 JDK 同口径对照。产物在 build/vtrep/<Test>/。
# 用法：scripts/diag/vt_repeat.sh <TestName> [顺序次数=100] [并行轮数=20]
# 环境：JDK=1 另跑参考 JDK 同口径；PROFILE=release / dev-opt 换构建档位
set -uo pipefail
cd "$(dirname "$0")/../.."
T=${1:?TestName}; SEQ=${2:-100}; PAR=${3:-20}
SRC=$(find tests/e2e -name "$T.java" | head -1)
EXP=tests/expected/$T.txt
OUT=build/vtrep/$T; rm -rf $OUT; mkdir -p $OUT/runs
JH=$(scripts/fetch_reference_jdk.sh) || exit 1
echo "== host: $(uname -m) cpus=$(nproc) ref=$(git rev-parse --short HEAD)"

tally() {  # $1=标签 $2=运行目录
  local tag=$1 dir=$2 fail=0 total=0
  : > $OUT/${tag}_diffs.txt
  for f in $dir/*.out; do
    total=$((total+1))
    if ! diff -q "$EXP" "$f" > /dev/null; then
      fail=$((fail+1))
      diff "$EXP" "$f" | grep -E '^[<>]' >> $OUT/${tag}_diffs.txt
      echo "-- $(basename $f) rc=$(cat ${f%.out}.rc 2>/dev/null)" >> $OUT/${tag}_fail_list.txt
    fi
  done
  echo "== $tag: fail $fail / $total"
  sort $OUT/${tag}_diffs.txt | uniq -c | sort -rn | head -20
}

run_set() {  # $1=标签 其余=命令
  local tag=$1; shift
  local d=$OUT/runs/$tag; mkdir -p $d
  for i in $(seq 1 $SEQ); do
    /usr/bin/time -f "time %e s rss %M KB" timeout 120 "$@" > $d/s$i.out 2> $d/s$i.err; echo $? > $d/s$i.rc
  done
  for r in $(seq 1 $PAR); do
    for j in 1 2 3 4 5 6 7 8; do
      ( timeout 120 "$@" > $d/p${r}_$j.out 2> $d/p${r}_$j.err; echo $? > $d/p${r}_$j.rc ) &
    done
    wait
  done
  tally $tag $d
  [ $SEQ -le 10 ] && grep -h "^time" $d/s*.err
}

if [ -n "${JDK:-}" ]; then
  mkdir -p $OUT/jc && "$JH/bin/javac" -d $OUT/jc "$SRC" || exit 1
  run_set jdk "$JH/bin/java" -cp $OUT/jc $T
fi

REL=(); DIR=debug
case "${PROFILE:-dev}" in release) REL=(--release); DIR=release;; dev-opt) REL=(--dev-opt); DIR=dev-opt;; esac
cargo build --release -q -p driver --manifest-path generator/Cargo.toml --target-dir build/analyzer-target || exit 1
touch $OUT/start.stamp
build/analyzer-target/release/rava build "$SRC" --stop-after compile --out $OUT/ws --clean \
  --java-home "$JH" "${REL[@]}" > $OUT/build.log 2>&1 || { tail -40 $OUT/build.log; exit 1; }
SNAKE=$(echo "$T" | sed -E 's/([a-z0-9])([A-Z])/\1_\2/g; s/([A-Z])([A-Z][a-z])/\1_\2/g' | tr 'A-Z' 'a-z')
BIN=$(find build -path "*/$DIR/$SNAKE" -type f -newer $OUT/start.stamp | head -1)
echo "== bin $BIN"
[ -x "$BIN" ] || { echo "no binary"; exit 1; }
run_set rava "$BIN"
