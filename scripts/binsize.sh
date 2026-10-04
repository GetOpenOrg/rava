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
for f in "$ci/meta_tables.rs" "$ci/line_tables.rs" "$ci/closure_tables.rs" "$scratch/user/src/rava_user_meta.rs"; do
  [[ -f "$f" ]] && printf '%-24s %12d\n' "$(basename "$f")" "$(wc -c < "$f")"
done

echo "== per-table source bytes (meta_tables.rs / line_tables.rs)"
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

echo "== row counts"
meta="$ci/meta_tables.rs"; lines="$ci/line_tables.rs"
stats=$(grep -h '^// \[meta-stats\]' "$meta" "$lines" 2>/dev/null || true)
if [[ -n "$stats" ]]; then
  echo "$stats" | sed 's#^// \[meta-stats\] ##'
elif [[ -f "$meta" && -f "$lines" ]]; then
  # 字面量形态的表（B1 之前）：按行数 × 结构体大小 + 去重字符串字节估算二进制中的元数据量
  python3 - "$meta" "$lines" <<'EOF2'
import re, sys
meta, lines = open(sys.argv[1]).read(), open(sys.argv[2]).read()
def section(text, name):
    i = text.index(' ' + name + ':'); j = text.find('#[export_name', i + 10)
    return text[i: j if j > 0 else len(text)]
lits = lambda s: set(re.findall(r'"((?:[^"\\]|\\.)*)"', s))
cm = section(meta, 'CLASS_METHODS'); cf = section(meta, 'CLASS_FIELDS')
lt = section(lines, 'LINE_TABLES'); ln = section(lines, 'LINE_NUMBERS')
mrows, frows = cm.count('MethodMeta {'), cf.count('FieldMeta {')
blob = sum(len(re.findall(r'\d+', m)) for m in re.findall(r'annotations: &\[([^\]]*)\]|annotation_default: &\[([^\]]*)\]', cm) for m in m)
lmeth = len(re.findall(r'\("[^"]*", "[^"]*", "[^"]*", "[^"]*", "[^"]*"\)', lt))
lrows = len(re.findall(r'\(\d+, \d+, \d+\)', lt))
lents, pairs = ln.count('\n    ("'), len(re.findall(r'\(\d+, \d+\)', ln))
# MethodMeta 136 B、FieldMeta 88 B、行表方法项 5×&str 80 B、行 12 B、LineNumbers 项 64 B + 每对 4 B、外层行 32 / 48 B
struct = (mrows * 136 + cm.count('", &[\n') * 32 + blob + frows * 88 + cf.count('", &[\n') * 32
          + lt.count('\n    ("') * 48 + lmeth * 80 + lrows * 12 + lents * 64 + pairs * 4)
strings = sum(len(x) for x in lits(cm) | lits(cf) | lits(lt) | lits(ln))
print(f"class_methods_rows {mrows}\nclass_fields_rows {frows}\nline_methods {lmeth}\nline_rows {lrows}")
print(f"line_numbers_entries {lents}\nline_number_pairs {pairs}")
print(f"meta_bytes_est {struct + strings}  (struct {struct} + unique strings {strings})")
EOF2
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
