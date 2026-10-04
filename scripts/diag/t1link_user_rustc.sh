#!/bin/bash
# T1 第 2 步实验（2026-10-04）：不经 cargo，直接 rustc 编译用户 crate 并链接档案产物。
# 用法：t1link_user_rustc.sh <scratch 目录> <bin 名> <档案 deps 目录> <输出目录> <链接方式> [额外 rustc 参数...]
#   链接方式：rlib —— 逐个 --extern 档案 rlib（静态）；
#             dylib —— --extern 档案门面 dylib（java_profile）+ 各 crate rmeta，rpath 指向 deps。
# 输出：<输出目录>/<bin>、time.txt（/usr/bin/time -l）、run.txt（运行输出）。
set -u
scratch=$1; bin=$2; deps=$3; out=$4; mode=$5; shift 5
mkdir -p "$out"
pick() { ls "$deps"/lib$1-*.$2 2>/dev/null | head -1; }
crates() { echo java_runtime java_meta; ls "$deps" | sed -n "s/^lib\(java_body_[0-9]*\)-.*\.rlib$/\1/p" | sort -u; }
macros=$(pick rava_macros dylib)
args=(--crate-name "$bin" --edition=2021 "$scratch/user/src/${MAIN:-main.rs}" --crate-type bin --emit=link
      -C panic=abort -C embed-bitcode=no -C debuginfo=line-tables-only -C split-debuginfo=unpacked
      --cap-lints allow --out-dir "$out" -L "dependency=$deps" --extern "rava_macros=$macros")
case $mode in
  rlib)
    for c in $(crates); do
      f=$(pick $c rlib); [ -n "$f" ] && args+=(--extern "$c=$f")
    done ;;
  dylib)
    # 门面 dylib 静态吸收全部档案 crate 与 std（panic=abort 与 sysroot 的 std dylib 不兼容，不用 prefer-dynamic）；
    # 用户 crate 以 rmeta 解析路径，链接只走门面；入口须 `use java_profile as _;`（MAIN=main_dy.rs）
    for c in $(crates); do
      f=$(pick $c rmeta); [ -n "$f" ] && args+=(--extern "$c=$f")
    done
    args+=(--extern "java_profile=$(pick java_profile dylib)"
           -C "link-arg=-Wl,-rpath,$deps" -C "link-arg=-Wl,-rpath,$(rustc --print target-libdir)") ;;
esac
/usr/bin/time -l rustc "${args[@]}" "$@" > "$out/rustc.txt" 2> "$out/time.txt"
rc=$?
grep -E 'real|maximum resident' "$out/time.txt"
[ $rc -ne 0 ] && { grep -E '^error' -A6 "$out/time.txt" | head -40; exit $rc; }
ls -la "$out/$bin" | awk '{print "bin bytes", $5}'
"$out/$bin" > "$out/run.txt" 2>&1; echo "exit $?"
