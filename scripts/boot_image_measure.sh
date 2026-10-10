#!/usr/bin/env bash
# 引导映像验收测量（docs/plans/2026-10-05-boot-image-evaluator.md §5.10）：服务器作业用。
#
# 用法：scripts/boot_image_measure.sh <out_dir> <Test.java | 已构建可执行文件>...（可执行文件只做 boot_probe）
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

# `__boot_image_start` 本身的耗时（§5.10 门槛 ≤ 1 ms）：uprobe / uretprobe 成对计时（tracefs uprobe_events，
# 需免密 sudo；探针用后即删）。权限或内核能力不具备时写明原因（不改服务器配置、不装软件）
boot_probe() { # <exe> <out_prefix>：tracefs uprobe / uretprobe 计 __boot_image_start 入口到返回（只依赖 sudo -n）
  local exe=$1 pre=$2 line sym vaddr off tr ev i
  exe=$(readlink -f "$exe")
  line=$(nm "$exe" 2>/dev/null | awk '$3 ~ /__boot_image_start/ { print $1, $3; exit }')
  if [[ -z $line ]]; then echo "  boot_probe: 符号 __boot_image_start 不在二进制中"; return; fi
  read -r vaddr sym <<< "$line"
  if ! sudo -n true 2> /dev/null; then echo "  boot_probe: uprobe 需 root，sudo -n 不可用"; return; fi
  # 虚拟地址 → 文件偏移（所在 LOAD 段：off = vaddr - p_vaddr + p_offset）
  off=$(readelf -lW "$exe" | python3 -c '
import sys
a = int(sys.argv[1], 16)
for l in sys.stdin:
    f = l.split()
    if f[:1] == ["LOAD"]:
        o, v, m = int(f[1], 16), int(f[2], 16), int(f[5], 16)
        if v <= a < v + m: print(hex(a - v + o)); break
' "$vaddr")
  if [[ -z $off ]]; then echo "  boot_probe: 无法定位 $sym 的文件偏移"; return; fi
  tr=/sys/kernel/tracing; sudo -n test -d $tr/events || tr=/sys/kernel/debug/tracing
  ev=$tr/uprobe_events
  sudo -n sh -c "echo '-:rava_boot/bis' >> $ev; echo '-:rava_boot/bie' >> $ev" 2> /dev/null
  if ! sudo -n sh -c "echo 'p:rava_boot/bis $exe:$off' >> $ev && echo 'r:rava_boot/bie $exe:$off' >> $ev"; then
    echo "  boot_probe: 写 $ev 失败（内核未启用 CONFIG_UPROBE_EVENTS？）"; return
  fi
  sudo -n sh -c "echo > $tr/trace; echo 1 > $tr/events/rava_boot/enable; echo 1 > $tr/tracing_on"
  for i in $(seq 1 10); do "$exe" > /dev/null 2>&1; done
  sudo -n cat $tr/trace > "$pre.boot_probe.txt"
  sudo -n sh -c "echo 0 > $tr/events/rava_boot/enable; echo '-:rava_boot/bis' >> $ev; echo '-:rava_boot/bie' >> $ev"
  python3 - "$pre.boot_probe.txt" "$sym" "$off" <<'EOF2'
import re, sys
start, v = {}, []
for l in open(sys.argv[1]):
    m = re.search(r'-(\d+)\s+\[\d+\].*?\s(\d+\.\d+):\s+(bis|bie):', l)
    if not m: continue
    pid, t, e = m.group(1), float(m.group(2)), m.group(3)
    if e == "bis": start[pid] = t
    elif pid in start: v.append((t - start.pop(pid)) * 1e6)
v.sort()
print(f"  boot_probe: {sys.argv[2]} @ {sys.argv[3]}  median {v[len(v)//2]:.0f} us  min {v[0]:.0f} us  max {v[-1]:.0f} us  (n={len(v)})" if v else "  boot_probe: 无采样")
EOF2
}

for t in "$@"; do
  name=$(basename "$t" .java)
  if [[ $t != *.java && -x $t ]]; then # 只探测已构建的二进制
    echo "== $name  boot_probe only" >> "$sum"; boot_probe "$t" "$out/$name" >> "$sum"; continue
  fi
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
