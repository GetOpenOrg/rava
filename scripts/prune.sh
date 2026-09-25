#!/usr/bin/env bash
# 清理共享编译缓存中的过期产物（跑批间调用，防磁盘写满）。
#
# 每个测试在共享 CARGO_TARGET_DIR 下各留一份 java_runtime 库（0.5–1G，包版本
# 按 scratch 唯一化，互不复用）与测试二进制（50–500M）；scratch 语义下源码每轮
# 重生成，这些产物下一轮必然重建，删除无损。宏 crate 与第三方依赖缓存保留。
#
# 用法：scripts/prune.sh          # 清 build/jdk*/target 与 build/target
set -u
BUILD="$(cd "$(dirname "$0")/.." && pwd)/build"
[ -d "$BUILD" ] || exit 0
for t in "$BUILD"/jdk*/target "$BUILD"/target; do
    [ -d "$t/debug" ] || continue
    rm -f "$t"/debug/deps/libjava_runtime-* "$t"/debug/deps/java_runtime-* 2>/dev/null
    find "$t/debug" -maxdepth 2 -type f -size +50M -delete 2>/dev/null
done
exit 0
