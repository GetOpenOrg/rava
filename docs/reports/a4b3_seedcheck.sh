#!/bin/zsh
# A-4 批次 3+ 双种子生成树一致性检查（口径对齐证据文档 §6：双种子 diff 归零）。
# 用法: zsh docs/reports/a4b3_seedcheck.sh TestIterator [更多测试...]
set -e
cd "$(dirname "$0")/../.."
if (( $# )); then
  TESTS=("$@")
else
  TESTS=(TestIterator)
fi
for t in $TESTS; do
  java=$(find tests/e2e -name "$t.java" | head -1)
  [ -z "$java" ] && { echo "未找到 $t"; exit 1 }
  for seed in 1 2; do
    PYTHONHASHSEED=$seed python3 scripts/main.py "$java" --no-run --clean >/dev/null 2>&1
    rm -rf "/tmp/a4b3_seed_$seed"
    cp -R "build/$(python3 -c "from codegen.emitter import to_snake;print(to_snake('$t'))")/java_runtime/src" "/tmp/a4b3_seed_$seed"
  done
  if diff -r /tmp/a4b3_seed_1 /tmp/a4b3_seed_2 >/dev/null 2>&1; then
    echo "[seedcheck] $t: 一致"
  else
    echo "[seedcheck] $t: 不一致"
    diff -rq /tmp/a4b3_seed_1 /tmp/a4b3_seed_2 | head -10
    exit 1
  fi
done
