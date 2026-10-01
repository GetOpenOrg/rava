# shellcheck shell=bash
# 各 shell 脚本共用：构建 rava（新鲜时为空操作）并导出 RAVA=<rava 可执行文件>。
# 用法：. "$(dirname "$0")/rava_env.sh" "<仓库根>"
_rava_repo="${1:?rava_env.sh 需要仓库根参数}"
cargo build --release -q -p driver --manifest-path "$_rava_repo/generator/Cargo.toml" \
    --target-dir "$_rava_repo/build/analyzer-target" || { echo "rava 构建失败" >&2; exit 2; }
RAVA="$_rava_repo/build/analyzer-target/release/rava"
export RAVA
unset _rava_repo
