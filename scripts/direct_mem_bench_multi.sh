#!/usr/bin/env bash
# 直接内存基准多 sha 版：对每个给定 sha 独立 worktree 构建 DirectMemBench（release）并跑 5 次，
# 用于回退二分（一次作业串行测多个 sha，每 sha 约 13 分钟构建）。
# 与 direct_mem_bench.sh 相同的 DirectMemBench.java 与口径（cas_ms / plain_ms / bulk_ms）；
# 首个 sha 建议放基线（如 96fc1cf4）作同作业锚点。
# 用法（服务器作业，仅 kr1 同机）：
#   distribute_tests.py --job <tag> --ref <含本脚本的分支> --servers kr1 \
#     --cmd "bash scripts/direct_mem_bench_multi.sh 96fc1cf4 <sha2> <sha3> ..." --job-timeout 14400
set -u
ROOT=$(pwd)
mkdir -p "$ROOT/build"
write_bench() { # $1=repo dir
  mkdir -p "$1/tests/e2e/99_bench" "$1/tests/expected"
  # 期望按 JDK 版本分目录（run_tests 同口径）：检出已有版本目录（jdk*/）则写入之；基线 ref 的
  # 旧检出仍是平铺口径，按其 run_tests 旧版读取位置落位（两布局都兼容，bench 才能跨基线对照）
  local expdir="$1/tests/expected" sub
  sub=$(ls -d "$expdir"/jdk*/ 2>/dev/null | head -1) && expdir="${sub%/}"
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
  echo "sum=841834958266" > "$expdir/DirectMemBench.txt"
}
for SHA in "$@"; do
  D="$ROOT/../dmbm-$SHA"
  git worktree remove --force "$D" > /dev/null 2>&1; rm -rf "$D"
  if ! git worktree add --force "$D" "$SHA" > /dev/null 2>&1; then
    echo "[$SHA] checkout FAIL"
    continue
  fi
  write_bench "$D"
  mkdir -p "$D/build"
  (cd "$D" && uv run python3 scripts/run_tests.py -j 1 --release --keep-artifacts --filter /DirectMemBench.java > "$ROOT/build/dmbm_$SHA.log" 2>&1)
  echo "[$SHA] $(git -C "$D" rev-parse --short HEAD) $(grep -E 'PASS|FAIL|Elapsed' "$ROOT/build/dmbm_$SHA.log" | tr '\n' ' ')"
  BIN=$(ls "$D"/build/jdk*/target/release/direct_mem_bench 2>/dev/null | head -1)
  if [ ! -x "$BIN" ]; then
    echo "[$SHA] no binary"
    tail -30 "$ROOT/build/dmbm_$SHA.log"
    git worktree remove --force "$D" > /dev/null 2>&1
    continue
  fi
  echo "[$SHA] bin_bytes=$(stat -c %s "$BIN")"
  for i in 1 2 3 4 5; do
    { TIMEFORMAT='real=%R'; time "$BIN" > /dev/null 2> "$D/build/bench_err"; } 2> "$D/build/bench_time"
    echo "[$SHA] run$i $(cat "$D/build/bench_time") $(cat "$D/build/bench_err")"
  done
  git worktree remove --force "$D" > /dev/null 2>&1
done
