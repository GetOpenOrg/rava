#!/usr/bin/env bash
# golden 类闸门统一入口：fetch 前导自取 + 逐 golden 断言（2026-10-13 T9 落地）。
#
# 背景（10-13 libgates3/4 事故链）：gate 作业裸跑 lib_pilot_golden.sh，缺 jar 早退（exit 2）但
# 作业命令 `… | tail -4` 管道掩码 rc → 假绿；救急又以 PILOT_LIBS 外指 rava-wt/jvmgold 的 jar
# 目录 → 该 worktree 被冻结、六分支清理被阻。
#
# 规则（本脚本固化，gate 面一律遵守）：
#   1) gate 一律自取依赖，禁止 PILOT_LIBS 外指其他检出 / worktree 的 jar 目录——环境里已设
#      PILOT_LIBS / DEPS_LOCK 时在此显式丢弃并打日志，落回本检出导出位；
#   2) 前导 scripts/fetch_pilot_deps.sh --no-scan（幂等：~/.m2 缓存命中秒级完成 mvn 解析 +
#      jar 导出 + 依赖锁生成，与 e2e 面 form.toml 的 fetch 同一命令、同一基础设施假设）；
#   3) 逐 golden 断言：输出必须含 "GOLDEN OK" 文样且 rc 为零（显式判文本不信 rc——早退 /
#      管道掩码都凑不出该文样；rc 单看也不够——见事故链）。任一不满足即 FAIL，汇总后 exit 1。
#      作业命令可放心 `2>&1 | tail -N`：断言在脚本内完成，tail 只截显示。
#
# 用法：scripts/diag/gate_lib_golden.sh m1 jc1 guava ...
#   参数 = lib_pilot_golden.sh 的模式（m1..m5 / sc1..sc4 / sb1..sb3 / jc1..jc4 / <MainName> /
#   <库>/<入口>），另支持 <库> 裸简写：展开为该库已点亮（golden/<库>_*_rs.txt 在档）的全部入口。
#   例：gate_lib_golden.sh m1 jc1 guava   # m1 + jc1 + guava/GuavaCollectionsDemo
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
GOLDEN_SH="$REPO_ROOT/scripts/lib_pilot_golden.sh"
FETCH_SH="$REPO_ROOT/scripts/fetch_pilot_deps.sh"
[ -f "$GOLDEN_SH" ] || { echo "缺 $GOLDEN_SH" >&2; exit 2; }
[ -f "$FETCH_SH" ] || { echo "缺 $FETCH_SH" >&2; exit 2; }
[ $# -ge 1 ] || { echo "用法: $0 m1|m2|..|jc1|..|<MainName>|<库>/<入口>|<库> ..." >&2; exit 2; }

# ── 规则 1：丢弃外部 PILOT_LIBS / DEPS_LOCK 外指（gate 一律自取） ──
for _v in PILOT_LIBS DEPS_LOCK; do
    if [ -n "${!_v:-}" ]; then
        echo "[gate] ${_v}=${!_v} 已忽略：gate 一律自取依赖，禁止外指其他检出的 jar / 锁目录" >&2
    fi
    unset "$_v"
done

# ── 模式展开：<库> 裸简写 → 该库已点亮的入口（golden/<库>_*_rs.txt 在档） ──
MODES=()
for arg in "$@"; do
    if [[ "$arg" == */* || ! -d "$REPO_ROOT/tests/lib_pilot/projects/$arg" ]]; then
        MODES+=("$arg")
        continue
    fi
    # <库> 简写：项目目录存在才走展开（含 / 的项目模式不在此列）；兼容 bash 3.2 不用 mapfile
    _entries=()
    while IFS= read -r e; do
        [ -n "$e" ] && _entries+=("$e")
    done < <(cd "$REPO_ROOT/tests/lib_pilot/golden" 2>/dev/null \
             && ls -- "${arg}_"*"_rs.txt" 2>/dev/null | sed -n "s/^${arg}_\(.*\)_rs\.txt$/\1/p" | sort -u)
    if [ "${#_entries[@]}" -eq 0 ]; then
        echo "项目 $arg 无已点亮 golden（golden/${arg}_*_rs.txt 不在档）；入口见 projects/README.md，逐个显式给出" >&2
        exit 2
    fi
    for e in "${_entries[@]}"; do MODES+=("$arg/$e"); done
done

# ── 规则 2：fetch 前导自取（幂等；~/.m2 缓存命中秒级） ──
echo "== [0] fetch 前导：$FETCH_SH --no-scan =="
_t0=$EPOCHREALTIME
bash "$FETCH_SH" --no-scan
echo "[gate] 依赖就绪（$FETCH_SH --no-scan，$(python3 -c "print(f'{$EPOCHREALTIME-$_t0:.1f}')")s）"

# ── 规则 3：逐 golden 断言（rc == 0 且输出含 GOLDEN OK 文样） ──
LOGDIR="$(mktemp -d /tmp/gate_lib_golden.XXXXXX)"
PASS=(); FAIL=()
for mode in "${MODES[@]}"; do
    log="$LOGDIR/$(echo "$mode" | tr '/' '_').log"
    echo "== [gate] $mode =="
    rc=0
    bash "$GOLDEN_SH" "$mode" >"$log" 2>&1 || rc=$?
    tail -6 "$log" | sed 's/^/    /'
    if [ "$rc" -eq 0 ] && grep -q "GOLDEN OK" "$log"; then
        echo "[gate] PASS $mode"
        PASS+=("$mode")
    else
        echo "[gate] FAIL $mode（rc=$rc，完整日志 $log）" >&2
        FAIL+=("$mode")
    fi
done

echo "GATE_LIB_GOLDEN ${#PASS[@]}/${#MODES[@]} PASS${FAIL[*]:+；FAIL: ${FAIL[*]}}"
[ "${#FAIL[@]}" -eq 0 ]
