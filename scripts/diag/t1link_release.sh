#!/bin/bash
# T1 第 2 步实验：生产模式静态链接的首构与「只改用户 crate」重构耗时（cargo release）。
# 用法：t1link_release.sh <scratch> <bin> <target 目录> <lto: true|false|thin>
set -u
cd "$1"; bin=$2; export CARGO_TARGET_DIR=$3 CARGO_PROFILE_RELEASE_LTO=$4 CARGO_BUILD_JOBS=2
echo "== 首构 lto=$4"
/usr/bin/time -l cargo build --release --bin "$bin" 2>&1 | grep -E 'Finished|^error|real|maximum resident'
touch user/src/main.rs
echo "== 只改用户 crate"
/usr/bin/time -l cargo build --release --bin "$bin" 2>&1 | grep -E 'Finished|^error|real|maximum resident'
ls -la "$3/release/$bin" | awk '{print "bin bytes", $5}'
"$3/release/$bin" > "$3/run_$bin.txt" 2>&1; echo "exit $?"
