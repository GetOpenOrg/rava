#!/bin/bash
# T1 第 2 步实验：把已编好的档案 rlib 打成一个门面 dylib（java_profile），
# 声明层 / 实现层 / java_meta 之间的 export_name 环在 dylib 内闭合。
# 用法：t1link_facade.sh <档案 deps 目录> [额外 rustc 参数...]（语料模式：-C panic=unwind -C prefer-dynamic）
set -u
deps=$1; shift
pick() { ls "$deps"/lib$1-*.$2 2>/dev/null | head -1; }
crates() { echo java_runtime java_meta; ls "$deps" | sed -n "s/^lib\(java_body_[0-9]*\)-.*\.rlib$/\1/p" | sort -u; }
src="$deps/../java_profile_facade.rs"
: > "$src"
args=(--crate-name java_profile --edition=2021 "$src" --crate-type dylib --emit=link
      -C panic=abort -C embed-bitcode=no -C debuginfo=line-tables-only -C split-debuginfo=unpacked
      -C extra-filename=-t1link --out-dir "$deps" -L "dependency=$deps"
      -C "link-arg=-Wl,-install_name,@rpath/libjava_profile-t1link.dylib")
for c in $(crates); do
  echo "pub extern crate $c;" >> "$src"
  args+=(--extern "$c=$(pick $c rlib)")
done
/usr/bin/time -l rustc "${args[@]}" "$@" 2>&1 | grep -E '^error|real|maximum resident|^ *= ' | head -40
ls -la "$deps"/libjava_profile-t1link.dylib 2>/dev/null | awk '{print "dylib bytes", $5}'
