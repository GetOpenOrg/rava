#!/usr/bin/env bash
# 语料参考 JDK：按 tools/refjdk.toml 取包（下载 + sha256 校验 + 解压）并定位 JAVA_HOME。
# 方案见 docs/plans/2026-10-03-reference-jdk-21.md。参考构建的定位只有这一处实现：
# run_tests.py 与语料 shell 脚本（经 scripts/corpus_jdk.sh）都调用本脚本 --check。
#
# 用法：
#   scripts/fetch_reference_jdk.sh                 # 确保当前平台的参考 JDK 就位（幂等），stdout 打印 JAVA_HOME
#   scripts/fetch_reference_jdk.sh --check         # 只检查不下载：就位则打印 JAVA_HOME，否则退出码 1 并提示取包命令
#   scripts/fetch_reference_jdk.sh --platform linux-x64   # 指定平台（缺省按 uname 判定；清单段名）
#   scripts/fetch_reference_jdk.sh --root DIR      # 指定 JDK 根目录
#   scripts/fetch_reference_jdk.sh --tag           # 只打印清单 tag
#
# JDK 根目录：--root > 环境变量 RAVA_REFJDK_ROOT > 主检出的 tools/refjdk/（git worktree 共用主检出那一份；
# gitignore）。落位 <根>/<tag>/ 即 JAVA_HOME（macOS 包的 Contents/Home 提升为该目录），其中
# .rava-refjdk 记录 tag / 平台 / sha256，--check 以它与清单比对（清单换版本或换包即视为未就位）。
# 并发安全：同一根目录下多个进程同时取包时以 <根>/.lock-<tag> 互斥，安装经临时目录整体改名落位。
# stdout 只输出 JAVA_HOME（供 $(...) 捕获），其余信息走 stderr。
set -euo pipefail

REPO="$(cd "$(dirname "$0")/.." && pwd)"
MANIFEST="$REPO/tools/refjdk.toml"
MARKER=".rava-refjdk"

MODE=ensure
PLATFORM=""
ROOT="${RAVA_REFJDK_ROOT:-}"
ROOT_FLAG=""   # 提示取包命令时原样带回显式 --root
while [ $# -gt 0 ]; do
    case "$1" in
        --check) MODE=check ;;
        --tag) MODE=tag ;;
        --platform) PLATFORM="${2:?--platform 需要参数}"; shift ;;
        --root) ROOT="${2:?--root 需要参数}"; ROOT_FLAG=" --root $2"; shift ;;
        -h|--help) sed -n '2,20p' "$0" | sed 's/^# \{0,1\}//'; exit 0 ;;
        *) echo "未知参数：$1" >&2; exit 2 ;;
    esac
    shift
done

log() { echo "[refjdk] $*" >&2; }

[ -f "$MANIFEST" ] || { log "缺清单 $MANIFEST"; exit 2; }

# 清单取值：mf <key> [段名]（段名缺省 = 顶层）
mf() {
    awk -v want_sec="${2:-}" -v key="$1" '
        /^[[:space:]]*#/ || /^[[:space:]]*$/ { next }
        /^\[/ { sec = $0; gsub(/[][[:space:]]/, "", sec); next }
        sec == want_sec {
            k = $0; sub(/[[:space:]]*=.*/, "", k); gsub(/[[:space:]]/, "", k)
            if (k == key) { v = $0; sub(/^[^=]*=[[:space:]]*"/, "", v); sub(/"[[:space:]]*$/, "", v); print v; exit }
        }' "$MANIFEST"
}

TAG="$(mf tag)"; VERSION="$(mf version)"
[ -n "$TAG" ] && [ -n "$VERSION" ] || { log "清单缺 tag / version：$MANIFEST"; exit 2; }
if [ "$MODE" = tag ]; then echo "$TAG"; exit 0; fi

if [ -z "$PLATFORM" ]; then
    case "$(uname -s)" in Linux) os=linux ;; Darwin) os=macos ;; *) log "不支持的系统 $(uname -s)"; exit 2 ;; esac
    case "$(uname -m)" in x86_64|amd64) arch=x64 ;; arm64|aarch64) arch=aarch64 ;; *) log "不支持的架构 $(uname -m)"; exit 2 ;; esac
    PLATFORM="$os-$arch"
fi
URL="$(mf url "$PLATFORM")"; SHA="$(mf sha256 "$PLATFORM")"
[ -n "$URL" ] && [ -n "$SHA" ] || { log "清单无平台 [$PLATFORM]：$MANIFEST"; exit 2; }

if [ -z "$ROOT" ]; then
    # git worktree 共用主检出的 tools/refjdk（每个 worktree 各下一份约 200MB 无意义）
    common="$(git -C "$REPO" rev-parse --path-format=absolute --git-common-dir 2>/dev/null || true)"
    main="$REPO"
    case "$common" in */.git) main="${common%/.git}" ;; esac
    ROOT="$main/tools/refjdk"
fi
HOME_DIR="$ROOT/$TAG"

installed() {
    [ -f "$HOME_DIR/$MARKER" ] && [ -x "$HOME_DIR/bin/java" ] && [ -x "$HOME_DIR/bin/javac" ] \
        && [ -d "$HOME_DIR/jmods" ] \
        && grep -qx "sha256=$SHA" "$HOME_DIR/$MARKER" && grep -qx "platform=$PLATFORM" "$HOME_DIR/$MARKER"
}

if installed; then echo "$HOME_DIR"; exit 0; fi
if [ "$MODE" = check ]; then
    log "参考 JDK ${TAG}（${PLATFORM}）未就位：$HOME_DIR"
    log "取包：$REPO/scripts/fetch_reference_jdk.sh${ROOT_FLAG}${RAVA_REFJDK_ROOT:+（RAVA_REFJDK_ROOT=$RAVA_REFJDK_ROOT）}"
    exit 1
fi

sha256_of() {
    if command -v sha256sum >/dev/null 2>&1; then sha256sum "$1" | awk '{print $1}'
    else shasum -a 256 "$1" | awk '{print $1}'; fi
}

mkdir -p "$ROOT"
# 根目录自带 gitignore：落在任何检出（含尚未登记 /tools/refjdk/ 的旧分支）里都不进 git 状态
[ -f "$ROOT/.gitignore" ] || [ -n "${RAVA_REFJDK_ROOT:-}" ] || echo '*' > "$ROOT/.gitignore"
LOCK="$ROOT/.lock-$TAG"
TMP=""
cleanup() { [ -n "$TMP" ] && rm -rf "$TMP"; rmdir "$LOCK" 2>/dev/null || true; }
# 互斥：拿不到锁就等持锁方装完（超过 30 分钟的锁视为上次中断遗留）
waited=0
until mkdir "$LOCK" 2>/dev/null; do
    if installed; then echo "$HOME_DIR"; exit 0; fi
    if [ -n "$(find "$LOCK" -maxdepth 0 -mmin +30 2>/dev/null)" ]; then
        log "清理遗留锁 $LOCK"; rmdir "$LOCK" 2>/dev/null || true; continue
    fi
    [ $waited -eq 0 ] && log "另一进程正在取包，等待（${LOCK}）"
    waited=$((waited + 5)); sleep 5
    [ $waited -lt 1800 ] || { log "等待取包锁超时：$LOCK"; exit 1; }
done
trap cleanup EXIT
if installed; then echo "$HOME_DIR"; exit 0; fi

TMP="$ROOT/.tmp-$TAG-$$"
rm -rf "$TMP"; mkdir -p "$TMP/x"
ARCHIVE="$TMP/jdk.tar.gz"
log "下载 ${TAG}（${PLATFORM}）：$URL"
if command -v curl >/dev/null 2>&1; then
    curl -fL --retry 3 --connect-timeout 30 -sS -o "$ARCHIVE" "$URL"
else
    wget -q -O "$ARCHIVE" "$URL"
fi
got="$(sha256_of "$ARCHIVE")"
[ "$got" = "$SHA" ] || { log "sha256 不符：期望 ${SHA}，实得 $got"; exit 1; }
tar -xzf "$ARCHIVE" -C "$TMP/x"
rm -f "$ARCHIVE"
java_bin="$(find "$TMP/x" -maxdepth 5 -path '*/bin/java' -type f | head -1)"
[ -n "$java_bin" ] || { log "包内找不到 bin/java"; exit 1; }
home_src="$(dirname "$(dirname "$java_bin")")"
rel_ver="$(sed -n 's/^JAVA_VERSION="\(.*\)"$/\1/p' "$home_src/release" 2>/dev/null)"
[ "$rel_ver" = "$VERSION" ] || { log "release 文件版本 $rel_ver ≠ 清单 $VERSION"; exit 1; }
printf 'tag=%s\nplatform=%s\nsha256=%s\nurl=%s\n' "$TAG" "$PLATFORM" "$SHA" "$URL" > "$home_src/$MARKER"
# 旧的无效落位（清单换包 / 残缺）整体替换
rm -rf "$HOME_DIR"
mv "$home_src" "$HOME_DIR"
installed || { log "落位后校验失败：$HOME_DIR"; exit 1; }
log "就位：${HOME_DIR}（$("$HOME_DIR/bin/java" -version 2>&1 | head -1)）"
echo "$HOME_DIR"
