#!/usr/bin/env bash
# 引导映像验收测量（docs/plans/2026-10-05-boot-image-evaluator.md §5.10）：服务器作业用。
#
# 用法：scripts/boot_image_measure.sh <out_dir> <Test.java>...
# 每例：转译（--stop-after emit，计时）→ 留存 boot_image.rs → 逐 crate 峰值编译（crate_mem_profile，缺省档）
# → 运行输出与墙钟（30 次中位数）→ 缺省档 / release 档二进制体积与墙钟。结果汇总到 <out_dir>/summary.txt。
set -uo pipefail

out=${1:?usage: boot_image_measure.sh <out_dir> <Test.java>...}
shift
mkdir -p "$out"
root=$(cd "$(dirname "$0")/.." && pwd)
cd "$root" || exit 1
rava=build/analyzer-target/release/rava
cargo build --release -p driver --manifest-path generator/Cargo.toml --target-dir build/analyzer-target || exit 1
sum="$out/summary.txt"
: > "$sum"

wall() { # <exe> <n>：n 次运行墙钟中位数（ms）
  python3 - "$1" "$2" <<'EOF'
import subprocess, sys, time
exe, n = sys.argv[1], int(sys.argv[2])
ts = []
for _ in range(n):
    t = time.perf_counter()
    subprocess.run([exe], stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
    ts.append((time.perf_counter() - t) * 1000)
ts.sort()
print(f"median {ts[len(ts)//2]:.3f} ms  min {ts[0]:.3f} ms  max {ts[-1]:.3f} ms  (n={n})")
EOF
}

for t in "$@"; do
  name=$(basename "$t" .java)
  t0=$(date +%s)
  /usr/bin/time -v "$rava" build "$t" --stop-after emit --clean > "$out/$name.emit.log" 2>&1
  rc=$?
  t1=$(date +%s)
  # scratch 取 emit 日志里「[emit] … → <路径>（bin …）」的路径（服务器与本机分根方式不同）
  scratch=$(sed -n 's/^\[emit\].* → \([^ （]*\).*/\1/p' "$out/$name.emit.log" | tail -1)
  echo "== $name  scratch=$scratch  emit rc=$rc  transpile $((t1 - t0)) s" >> "$sum"
  grep -E "Maximum resident|Elapsed" "$out/$name.emit.log" | sed 's/^/  emit /' >> "$sum"
  [[ $rc -eq 0 ]] || continue
  bi="$scratch/java_base/src/boot_image.rs"
  if [[ -f "$bi" ]]; then
    printf '  boot_image.rs %d bytes, %d lines\n' "$(stat -c %s "$bi")" "$(wc -l < "$bi")" >> "$sum"
    gzip -c "$bi" > "$out/$name.boot_image.rs.gz"
  fi
  python3 scripts/crate_mem_profile.py run "$scratch" "$out/$name.mem" > "$out/$name.mem.log" 2>&1
  echo "  compile rc=$?" >> "$sum"
  grep -E "java_base[^_]|java_base_decl|总墙钟" "$out/$name.mem/report.md" 2>/dev/null | head -6 | sed 's/^/  /' >> "$sum"
  exe=$(python3 -c 'import json,sys; print(json.load(open(sys.argv[1])).get("exe") or "")' "$scratch/build_status.json" 2>/dev/null)
  if [[ -n "$exe" && -x "$exe" ]]; then
    "$exe" > "$out/$name.run.out" 2>&1
    echo "  run rc=$? lines=$(wc -l < "$out/$name.run.out")" >> "$sum"
    printf '  default bin %d bytes; wall %s\n' "$(stat -c %s "$exe")" "$(wall "$exe" 30)" >> "$sum"
    size -A "$exe" | awk 'NR>2 && $2>100000 { printf "    %-24s %12d\n", $1, $2 }' >> "$sum"
  fi
  "$rava" compile "$scratch" --release > "$out/$name.release.log" 2>&1
  echo "  release compile rc=$?" >> "$sum"
  exe=$(python3 -c 'import json,sys; print(json.load(open(sys.argv[1])).get("exe") or "")' "$scratch/build_status.json" 2>/dev/null)
  if [[ -n "$exe" && -x "$exe" ]]; then
    printf '  release bin %d bytes; wall %s\n' "$(stat -c %s "$exe")" "$(wall "$exe" 30)" >> "$sum"
    size -A "$exe" | awk 'NR>2 && $2>100000 { printf "    %-24s %12d\n", $1, $2 }' >> "$sum"
  fi
done
cat "$sum"
