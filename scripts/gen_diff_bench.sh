#!/usr/bin/env bash
# DirectMemBench 生成物对照：对每个给定 sha 独立 worktree 构建 driver（release）并以
# `rava build --stop-after emit` 转译 DirectMemBench.java，打包生成树（java_runtime/user crate 源）
# 供本地 diff。只做转译不做 cargo 编译，不需要 kr1（代码生成形态证据，任意作业服务器可跑）。
# 用法（服务器作业）：
#   distribute_tests.py --job <tag> --ref <含本脚本的分支> --servers <作业服务器…> \
#     --cmd "bash scripts/gen_diff_bench.sh <sha1> <sha2> ..." \
#     --fetch 'build/genbench-*.tgz' --fetch 'build/gdb_*.log' --job-timeout 10800
set -u
ROOT=$(pwd)
mkdir -p "$ROOT/build"
JAVA_HOME=$(scripts/fetch_reference_jdk.sh) || { echo "refjdk FAIL"; exit 1; }
export JAVA_HOME
mkdir -p tests/e2e/99_bench
cat > tests/e2e/99_bench/DirectMemBench.java <<'JAVA'
import java.lang.reflect.Field;
import sun.misc.Unsafe;

public class DirectMemBench {
    public static void main(String[] args) throws Exception {
        Field tf = Unsafe.class.getDeclaredField("theUnsafe");
        tf.setAccessible(true);
        Unsafe u = (Unsafe) tf.get(null);
        long base = u.allocateMemory(4096);
        long dst = u.allocateMemory(4096);
        u.setMemory(base, 4096, (byte) 0);
        long sum = 0;
        long t0 = System.nanoTime();
        for (int i = 0; i < 5_000_000; i++) {
            long a = base + ((i & 255) << 3);
            u.getAndAddInt(null, a, i);
            if (!u.compareAndSwapLong(null, a + 2048, (long) (i - 1), (long) i)) {
                u.putLongVolatile(null, a + 2048, (long) i);
            }
        }
        long t1 = System.nanoTime();
        for (int r = 0; r < 2000; r++) {
            for (int i = 0; i < 1024; i++) {
                u.putInt(base + (i << 2), u.getInt(base + (i << 2)) + r);
            }
        }
        long t2 = System.nanoTime();
        for (int r = 0; r < 20000; r++) {
            u.copyMemory(base, dst, 4096);
            u.setMemory(base + 4000, 96, (byte) r);
            sum += u.getByte(dst + (r & 4095));
        }
        long t3 = System.nanoTime();
        for (int i = 0; i < 1024; i++) {
            sum += u.getInt(base + (i << 2));
            sum += u.getInt(dst + (i << 2));
        }
        System.out.println("sum=" + sum);
        System.err.println("cas_ms=" + (t1 - t0) / 1_000_000 + " plain_ms=" + (t2 - t1) / 1_000_000 + " bulk_ms=" + (t3 - t2) / 1_000_000);
        u.freeMemory(base);
        u.freeMemory(dst);
    }
}
JAVA
for SHA in "$@"; do
  D="$ROOT/../gdb-$SHA"
  git worktree remove --force "$D" > /dev/null 2>&1; rm -rf "$D"
  git worktree add --force "$D" "$SHA" > /dev/null 2>&1 || { echo "[$SHA] checkout FAIL"; continue; }
  mkdir -p "$D/tests/e2e/99_bench"
  cp tests/e2e/99_bench/DirectMemBench.java "$D/tests/e2e/99_bench/"
  if ! (cd "$D" && cargo build --release -q -p driver --manifest-path generator/Cargo.toml --target-dir build/analyzer-target > "$ROOT/build/gdb_drv_$SHA.log" 2>&1); then
    echo "[$SHA] driver build FAIL"; tail -20 "$ROOT/build/gdb_drv_$SHA.log"; continue
  fi
  if ! (cd "$D" && build/analyzer-target/release/rava build tests/e2e/99_bench/DirectMemBench.java --stop-after emit --out build/genbench --java-home "$JAVA_HOME" --train-predefined > "$ROOT/build/gdb_$SHA.log" 2>&1); then
    echo "[$SHA] transpile FAIL"; tail -30 "$ROOT/build/gdb_$SHA.log"; continue
  fi
  echo "[$SHA] $(git -C "$D" rev-parse --short HEAD) transpile ok"
  tar czf "$ROOT/build/genbench-$SHA.tgz" -C "$D/build/genbench" .
  echo "[$SHA] tgz_bytes=$(stat -c %s "$ROOT/build/genbench-$SHA.tgz")"
  git worktree remove --force "$D" > /dev/null 2>&1
done
ls -la "$ROOT/build"/genbench-*.tgz 2>/dev/null
