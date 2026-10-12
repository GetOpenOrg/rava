#!/usr/bin/env bash
# vendor 搬移保留 mtime 复核：HelloWorld emit 后首编，把整个 scratch 经 cp -a / mv 搬到 /tmp 再编，
# 期望 vendor 宏 crate（rava_macros / rava_macros_core / rava_coro）全部 Fresh（cargo 指纹不失效）。
# 背景：docs/reports/2026-10-11-code-review-triage.md「生成器修复」节的未完成项
# （rg-vmove-e11a3191 在 sg1 三次无 done 被归还；原脚本随 worktree 丢失，本脚本按报告口径重建）。
# 判定：两段再编的输出里不得出现 "Compiling rava_macros"，出现即指纹失效（根因方向见报告：workspace 绝对路径）。
set -euo pipefail
REPO="${REPO:-$(pwd)}"
f=$(find "$REPO/tests/e2e" -name HelloWorld.java | head -1)
. "$REPO/scripts/rava_env.sh" "$REPO"
. "$REPO/scripts/corpus_jdk.sh" "$REPO"
"$RAVA" build "$f" "${CORPUS_JDK_ARGS[@]}" --clean --stop-after emit > /tmp/vmove_emit.log 2>&1
S="$REPO/build/hello_world"
[ -d "$S" ] || { echo "EMIT-FAIL：无 $S"; tail -5 /tmp/vmove_emit.log; exit 1; }
cd "$S"
echo "== 首编"
cargo build -q 2>&1 | tail -3 || true

rm -rf /tmp/vmove_a && cp -a "$S" /tmp/vmove_a
cd /tmp/vmove_a
echo "== cp -a 后再编（期望 vendor 宏 crate Fresh）"
cargo build -v 2>&1 | tee /tmp/vmove_a.log | grep -E "Compiling rava|Fresh rava" | head -12 || true
grep -q "Compiling rava" /tmp/vmove_a.log && echo "CP-A 结果：宏 crate 重编（不达预期）" || echo "CP-A 结果：Fresh 达标"

mv /tmp/vmove_a /tmp/vmove_b
cd /tmp/vmove_b
echo "== mv 后再编（期望 vendor 宏 crate Fresh）"
cargo build -v 2>&1 | tee /tmp/vmove_b.log | grep -E "Compiling rava|Fresh rava" | head -12 || true
grep -q "Compiling rava" /tmp/vmove_b.log && echo "MV 结果：宏 crate 重编（不达预期）" || echo "MV 结果：Fresh 达标"
echo VMOVE-DONE
