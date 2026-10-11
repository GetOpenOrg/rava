#!/usr/bin/env bash
# 直接内存（sun.misc.Unsafe 绝对地址）热循环同机前后计时：当前检出 vs 基线 ref（缺省 96fc1cf4）。
# 用法（服务器作业）：distribute_tests.py --job <tag> --ref <新 sha> --servers <机> --cmd "bash scripts/direct_mem_bench.sh [基线 ref]"
# 输出：[after] / [before] 各一行构建结果、bin_bytes、5 次 real 与 cas_ms / plain_ms / bulk_ms。
set -u
BASE_REF=${1:-96fc1cf4}
ROOT=$(pwd)
write_bench() { # $1=repo dir
  mkdir -p "$1/tests/e2e/99_bench" "$1/tests/expected"
  cat > "$1/tests/e2e/99_bench/DirectMemBench.java" <<'JAVA'
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
  echo "sum=841834958266" > "$1/tests/expected/DirectMemBench.txt"
}
bench() { # $1=label $2=repo dir
  local d=$2
  write_bench "$d"
  (cd "$d" && uv run python3 scripts/run_tests.py -j 1 --release --keep-artifacts --filter /DirectMemBench.java > build/bench_$1.log 2>&1)
  echo "[$1] $(git -C "$d" rev-parse --short HEAD) $(grep -E 'PASS|FAIL|Elapsed' "$d/build/bench_$1.log" | tr '\n' ' ')"
  local bin=$(ls "$d"/build/jdk*/target/release/direct_mem_bench 2>/dev/null | head -1)
  [ -x "$bin" ] || { echo "[$1] no binary"; tail -30 "$d/build/bench_$1.log"; return; }
  echo "[$1] bin_bytes=$(stat -c %s "$bin")"
  for i in 1 2 3 4 5; do
    { TIMEFORMAT='real=%R'; time "$bin" > /dev/null 2> "$d/build/bench_err"; } 2> "$d/build/bench_time"
    echo "[$1] run$i $(cat "$d/build/bench_time") $(cat "$d/build/bench_err")"
  done
}
mkdir -p "$ROOT/build"
bench after "$ROOT"
rm -rf "$ROOT/../bench-base"; git worktree add -f "$ROOT/../bench-base" "$BASE_REF" > /dev/null 2>&1 || git clone -q "$ROOT" "$ROOT/../bench-base"
git -C "$ROOT/../bench-base" checkout -q "$BASE_REF"
mkdir -p "$ROOT/../bench-base/build"
bench before "$ROOT/../bench-base"
git worktree remove --force "$ROOT/../bench-base" 2>/dev/null; rm -rf "$ROOT/../bench-base"
