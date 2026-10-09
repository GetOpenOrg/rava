#!/usr/bin/env bash
# 引导映像验收测量（docs/plans/2026-10-05-boot-image-evaluator.md §5.10）：服务器作业用。
#
# 用法：scripts/boot_image_measure.sh <out_dir> <Test.java>...
# 每例：转译（--stop-after emit，计时）→ 留存 boot_image.rs → 逐 crate 峰值编译（crate_mem_profile，缺省档）
# → 运行输出与墙钟（30 次中位数）→ 缺省档二进制体积、`__boot_image_start` uprobe 计时 → release 档体积与墙钟
# （MEASURE_RELEASE=0 跳过）。结果汇总到 <out_dir>/summary.txt。
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

# `__boot_image_start` 本身的耗时（§5.10 门槛 ≤ 1 ms）：uprobe / uretprobe 成对计时（bpftrace，需免密 sudo）。
# 工具或权限不具备时写明原因（不改服务器配置、不装软件）
boot_probe() { # <exe> <out_prefix>
  local exe=$1 pre=$2 sym
  sym=$(nm "$exe" 2>/dev/null | awk '$3 ~ /__boot_image_start/ { print $3; exit }')
  if [[ -z $sym ]]; then echo "  boot_probe: 符号 __boot_image_start 不在二进制中"; return; fi
  if ! command -v bpftrace > /dev/null; then
    echo "  boot_probe: 无 bpftrace（perf: $(command -v perf || echo 无)；perf_event_paranoid=$(cat /proc/sys/kernel/perf_event_paranoid 2>/dev/null)；sudo -n: $(sudo -n true 2>/dev/null && echo 可用 || echo 不可用)）"
    return
  fi
  if ! sudo -n true 2> /dev/null; then echo "  boot_probe: bpftrace 需 root，sudo -n 不可用"; return; fi
  local i
  : > "$pre.boot_probe.txt"
  for i in $(seq 1 10); do
    sudo -n bpftrace -q -e "uprobe:$exe:$sym { @s = nsecs; } uretprobe:$exe:$sym /@s/ { printf(\"boot_us %d\\n\", (nsecs - @s) / 1000); clear(@s); }" -c "$exe" 2>&1 | grep '^boot_us' >> "$pre.boot_probe.txt"
  done
  python3 - "$pre.boot_probe.txt" <<'EOF2'
import sys
v = sorted(int(l.split()[1]) for l in open(sys.argv[1]) if l.startswith("boot_us"))
print(f"  boot_probe: __boot_image_start median {v[len(v)//2]} us  min {v[0]} us  max {v[-1]} us  (n={len(v)})" if v else "  boot_probe: 无采样")
EOF2
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
    boot_probe "$exe" "$out/$name" >> "$sum"
  fi
  [[ ${MEASURE_RELEASE:-1} == 1 ]] || continue
  "$rava" compile "$scratch" --release > "$out/$name.release.log" 2>&1
  echo "  release compile rc=$?" >> "$sum"
  exe=$(python3 -c 'import json,sys; print(json.load(open(sys.argv[1])).get("exe") or "")' "$scratch/build_status.json" 2>/dev/null)
  if [[ -n "$exe" && -x "$exe" ]]; then
    printf '  release bin %d bytes; wall %s\n' "$(stat -c %s "$exe")" "$(wall "$exe" 30)" >> "$sum"
    size -A "$exe" | awk 'NR>2 && $2>100000 { printf "    %-24s %12d\n", $1, $2 }' >> "$sum"
  fi
done
cat "$sum"
