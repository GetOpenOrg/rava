#!/usr/bin/env bash
# 二进制体积剖析：段 / 节大小 + 元数据表源码规模 + 行数对照（docs/plans/2026-10-04-binary-size.md B0）
#
# 用法：scripts/binsize.sh <scratch> [binary]
#   <scratch>  rava 生成的 scratch 目录（如 build/hello_world）
#   [binary]   可选，缺省取 build_status.json 的 exe，再退到 <target>/release/<bin>
# 只读：不触发编译。release 二进制需先经
#   heavy_lock.py build/analyzer-target/release/rava compile <scratch> --release --keep-artifacts
set -euo pipefail

scratch=${1:?usage: binsize.sh <scratch> [binary]}
scratch=${scratch%/}
root=$(cd "$(dirname "$0")/.." && pwd)
status="$scratch/build_status.json"
ci="$scratch/closure_input"

bin=${2:-}
if [[ -z "$bin" && -f "$status" ]]; then
  bin=$(python3 -c 'import json,sys; d=json.load(open(sys.argv[1])); print(d.get("exe") or "")' "$status")
fi
if [[ -z "$bin" && -f "$status" ]]; then
  name=$(python3 -c 'import json,sys; print(json.load(open(sys.argv[1]))["emit"]["bin"])' "$status")
  bin="$root/build/target/release/$name"   # rava 缺省 target（--target-dir 时请显式传 binary）
fi

echo "== binary: ${bin:-<none>}"
if [[ -n "$bin" && -f "$bin" ]]; then
  printf 'file_bytes %d\n' "$(stat -f %z "$bin" 2>/dev/null || stat -c %s "$bin")"
  if [[ "$(uname)" == Darwin ]]; then
    # 段总量 + 关键节（__text / __const / __data_const 下各节）
    size -m "$bin" | awk '
      /^Segment /      { seg=$2; sub(":","",seg); printf "segment %-16s %12d\n", seg, $3; next }
      /^\tSection /    { sec=$2; sub(":","",sec); printf "  section %-12s %-16s %12d\n", seg, sec, $3 }'
  else
    size -A "$bin" | awk 'NR>2 && $2>0 { printf "section %-24s %12d\n", $1, $2 }'
  fi
else
  echo "(binary missing: run release compile first)"
fi

echo "== metadata sources (bytes)"
for f in "$ci/meta_tables.rs" "$ci/line_tables.rs" "$scratch/user/src/rava_user_meta.rs"; do
  [[ -f "$f" ]] && printf '%-24s %12d\n' "$(basename "$f")" "$(wc -c < "$f")"
done

echo "== per-table source bytes (meta_tables.rs / line_tables.rs, escaped literal text)"
for f in "$ci/meta_tables.rs" "$ci/line_tables.rs"; do
  [[ -f "$f" ]] || continue
  # 以顶层 static / const 声明为界累计字节
  awk '
    match($0, /(static|const) [A-Z_][A-Z_0-9]*:/) {
      if (name!="") printf "  %-28s %12d\n", name, bytes
      name=substr($0, RSTART, RLENGTH-1); sub(/^(static|const) /,"",name); bytes=0 }
    { bytes += length($0) + 1 }
    END { if (name!="") printf "  %-28s %12d\n", name, bytes }' "$f" | sort -k2 -n -r | head -12
done

echo "== metadata binary bytes"
meta="$ci/meta_tables.rs"; lines="$ci/line_tables.rs"; user="$scratch/user/src/rava_user_meta.rs"
stats=$(grep -h '^// \[meta-stats\]' "$meta" "$lines" 2>/dev/null || true)
if [[ -n "$stats" ]]; then
  # 表是字符串池 + 字节流：发射层写出各表字节流 / 池的确切字节数（档案侧三文件 + 用户侧行）
  for f in "$meta" "$lines" "$user"; do
    [[ -f "$f" ]] || continue
    grep -h '^// \[meta-stats\]' "$f" | awk -v file="$(basename "$f")" '{ printf "  %-22s %-24s %10d\n", file, $3, $4; t += $4 }
      END { printf "  %-22s %-24s %10d\n", file, "(total)", t }'
  done
  cat "$meta" "$lines" "$user" 2>/dev/null | grep -h '^// \[meta-stats\]' \
    | awk '{ t += $4 } END { printf "meta_bytes %d  (exact: pools + streams)\n", t }'
else
  echo "(no [meta-stats]: emit first)"
fi
if [[ -f "$ci/closure.json" ]]; then
  python3 - "$ci/closure.json" <<'EOF'
import json, sys
d = json.load(open(sys.argv[1]))
print("reachable_methods", len(d.get("methods", [])))
print("closure_classes", len(d.get("classes", [])))
r = d.get("reflect", {})
print("reflect", " ".join(f"{k}={len(v)}" for k, v in sorted(r.items()) if isinstance(v, (list, dict))))
EOF
fi
