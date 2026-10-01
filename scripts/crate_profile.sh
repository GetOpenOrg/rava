#!/usr/bin/env bash
# 逐 crate rustc 峰值 RSS / 墙钟（拆 crate 验收，docs/plans/2026-10-01-rustc-memory-and-crate-split.md §7.6）：
# 对已生成的 scratch 工作区，预编依赖后强制重编 workspace 自有 crate（java_runtime / java_meta /
# java_body_* / lib crate / user），经 RUSTC_WRAPPER 以 `/usr/bin/time -l` 包裹每个 rustc，
# 输出每 crate 的 wall / peak_rss 与整次 cargo build 墙钟。
#
# 用法：scripts/crate_profile.sh <out_dir> <scratch>...   例：build/cp build/hello_world
# 环境变量：PROFILE_TOUCH=user（只强制重编 user crate，测「仅用户类变化」的重编集合）
# 重命令：须经 heavy_lock 调用；CARGO_BUILD_JOBS=2。
set -u
OUT="${1:?用法: $0 <out_dir> <scratch>...}"; shift
mkdir -p "$OUT"; OUT="$(cd "$OUT" && pwd)"
export CARGO_BUILD_JOBS=2 CARGO_INCREMENTAL=0
WRAP="$OUT/rustc_wrap.sh"
cat > "$WRAP" <<'EOF'
#!/usr/bin/env bash
# RUSTC_WRAPPER：$1 = rustc，其余为参数；只计 --crate-name 属本工作区的编译
name=""; prev=""
for a in "$@"; do [ "$prev" = "--crate-name" ] && name="$a"; prev="$a"; done
case "$name" in
  java_runtime|java_meta|java_body_*|user) ;;
  *) case " ${PROFILE_LIBS:-} " in *" $name "*) ;; *) exec "$@";; esac ;;
esac
t="$(mktemp)"
/usr/bin/time -l -o "$t" "$@"; rc=$?
peak=$(awk '/maximum resident set size/ {printf "%.0f", $1/1048576}' "$t")
wall=$(awk '/real/ {print $1}' "$t")
echo "$name wall=${wall}s peak_rss=${peak}MB rc=$rc" >> "$PROFILE_LOG"
rm -f "$t"
exit $rc
EOF
chmod +x "$WRAP"
for ws in "$@"; do
    ws="$(cd "$ws" && pwd)"; name="$(basename "$ws")"
    export CARGO_TARGET_DIR="$OUT/target"
    export RUSTC_WRAPPER="$WRAP" PROFILE_LOG=/dev/null PROFILE_LIBS=""
    members=$(sed -n 's/^members = \[\(.*\)\]/\1/p' "$ws/Cargo.toml" | tr -d '",')
    # 先完整编一次（依赖就绪），再按 PROFILE_TOUCH 失效指纹后测量
    (cd "$ws" && cargo build -q) || { echo "BUILD-FAIL $name"; continue; }
    if [ "${PROFILE_TOUCH:-all}" = user ]; then
        touch "$ws"/user/src/main.rs
    else
        for m in $members; do
            rm -rf "$CARGO_TARGET_DIR"/debug/.fingerprint/"$m"-* "$CARGO_TARGET_DIR"/debug/deps/lib"$m"-*
        done
        rm -rf "$CARGO_TARGET_DIR"/debug/.fingerprint/user-*
    fi
    bin=$(sed -n 's/^name = "\(.*\)"/\1/p' "$ws/user/Cargo.toml" | tail -1)
    export PROFILE_LOG="$OUT/$name.crates.log" PROFILE_LIBS="$members $bin"
    : > "$PROFILE_LOG"
    s=$(python3 -c 'import time;print(time.time())')
    (cd "$ws" && cargo build -q) ; rc=$?
    e=$(python3 -c 'import time;print(time.time())')
    echo "== $name rc=$rc touch=${PROFILE_TOUCH:-all} cargo_wall=$(python3 -c "print(round($e-$s,1))")s"
    cat "$PROFILE_LOG"
done
