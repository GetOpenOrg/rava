# shellcheck shell=bash
# 语料 shell 脚本共用的 JDK 选择，与 run_tests.py 同口径（docs/plans/2026-10-03-reference-jdk-21.md）：
#   缺省 = 参考构建（tools/refjdk.toml，scripts/fetch_reference_jdk.sh --check 定位；未就位即退出，不回退系统 JDK）；
#   环境变量 JDK=N 显式覆盖为本机 JDK N（实验，标记非参考构建；须先设好 RAVA）。
# 导出 JAVA_HOME，并设数组 CORPUS_JDK_ARGS=(--java-home <home>) 供 rava build / emit / image-dirs 使用。
# 用法：. "$REPO/scripts/corpus_jdk.sh" "$REPO"
_cj_repo="${1:?corpus_jdk.sh 需要仓库根参数}"
if [ -n "${JDK:-}" ]; then
    JAVA_HOME="$("${RAVA:?corpus_jdk.sh：JDK=N 覆盖需要先设 RAVA}" jdk --jdk "$JDK" --home-only)" || exit 2
    echo "[jdk] JAVA_HOME → ${JAVA_HOME}（JDK=${JDK}：非参考构建，仅供实验）" >&2
else
    JAVA_HOME="$("$_cj_repo/scripts/fetch_reference_jdk.sh" --check)" || {
        echo "[jdk] 语料缺省使用参考 JDK，当前未就位（见上）；实验可设 JDK=N 改用本机 JDK" >&2; exit 2; }
    echo "[jdk] JAVA_HOME → ${JAVA_HOME}（参考构建 $("$_cj_repo/scripts/fetch_reference_jdk.sh" --tag)）" >&2
fi
export JAVA_HOME
CORPUS_JDK_ARGS=(--java-home "$JAVA_HOME")
unset _cj_repo
