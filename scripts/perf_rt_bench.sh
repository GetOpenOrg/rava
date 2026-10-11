#!/usr/bin/env bash
# 运行时性能同机对照微基准（docs/tasks.md「10-11 晚收拢」第 4 条）：rava 与参考 JVM 同机同口径。
#   1) 反射实参 marshalling（BenchReflectArg）：idInt(int)（拆箱+装箱）vs idRef(Integer)（引用直传）
#      vs idInt2(int,int)——三者走同一分派协议，差值即 unbox/box 边界开销（rd::unbox_* 的
#      __obj_str 文本路径在 rava 侧的占比可直接推得）；
#   2) TestReflectLdcHotLoop 三段（ldc / 类字面量 / Method.invoke）逐段墙钟（计时副本 RldcSegments）；
#   3) stream 管线 vs 手写循环（BenchStreamLoop，每元素 ns）——PartitionInteger 46 s 的归因旁证。
# 用法（仅服务器作业）：scripts/perf_rt_bench.sh <out.tsv> [全例运行次数，缺省 5]
#   环境变量 CASES="A B C" 只跑子集（类名空格分隔，缺省全部三个）
# 测试源写在 tests/e2e/99_rtbench/（作业检出内自建，作业后即弃，不入库）。
set -uo pipefail

REPO="$(cd "$(dirname "$0")/.." && pwd)"
OUT="${1:?用法: $0 <out.tsv> [运行次数]}"
REPS="${2:-5}"
CASES="${CASES:-BenchReflectArg RldcSegments BenchStreamLoop}"
mkdir -p "$(dirname "$OUT")"
BENCH="$REPO/tests/e2e/99_rtbench"
CLS="$REPO/build/rtpp-classes"
mkdir -p "$BENCH" "$CLS"
. "$REPO/scripts/rava_env.sh" "$REPO"
. "$REPO/scripts/corpus_jdk.sh" "$REPO"
snake() { python3 -c 'import re,sys; print(re.sub(r"(?<=[a-z0-9])(?=[A-Z])", "_", sys.argv[1]).lower())' "$1"; }

# ── 测试源（与 TestReflectLdcHotLoop 同构；stdout 确定性校验和，stderr 计时行 key=value）──
cat > "$BENCH/BenchReflectArg.java" <<'JAVA'
import java.lang.reflect.Method;

public class BenchReflectArg {
    static int intSink;
    static Object objSink;
    static int idInt(int x) { return x; }
    static Object idRef(Integer x) { return x; }
    static int idInt2(int a, int b) { return a + b; }

    public static void main(String[] args) throws Exception {
        final int n = 1_000_000;
        Method mInt = BenchReflectArg.class.getDeclaredMethod("idInt", int.class);
        mInt.setAccessible(true);
        Method mRef = BenchReflectArg.class.getDeclaredMethod("idRef", Integer.class);
        mRef.setAccessible(true);
        Method mInt2 = BenchReflectArg.class.getDeclaredMethod("idInt2", int.class, int.class);
        mInt2.setAccessible(true);
        Integer boxed = 12345;
        for (int i = 0; i < 50_000; i++) { intSink += (Integer) mInt.invoke(null, i & 65535); }
        long t0 = System.nanoTime();
        for (int i = 0; i < n; i++) intSink += (Integer) mInt.invoke(null, i & 65535);
        long t1 = System.nanoTime();
        for (int i = 0; i < n; i++) objSink = mRef.invoke(null, boxed);
        long t2 = System.nanoTime();
        for (int i = 0; i < n; i++) intSink += (Integer) mInt2.invoke(null, i & 65535, i & 65535);
        long t3 = System.nanoTime();
        System.out.println("sink=" + intSink + " " + objSink);
        System.err.println("NSPER idInt=" + (t1 - t0) / n + " idRef=" + (t2 - t1) / n + " idInt2=" + (t3 - t2) / n);
    }
}
JAVA

cat > "$BENCH/RldcSegments.java" <<'JAVA'
import java.lang.reflect.Method;

// TestReflectLdcHotLoop 的计时副本：三段各 n=1_000_000，stderr 输出逐段纳秒
public class RldcSegments {
    static class Base { int step(int x) { return x + 1; } }
    static class Derived extends Base { @Override int step(int x) { return x + 3; } }
    private static int twice(int x) { return x * 2; }

    public static void main(String[] args) throws Exception {
        final int n = 1_000_000;
        String first = "hot-literal";
        long ldcSum = 0; int same = 0;
        long t0 = System.nanoTime();
        for (int i = 0; i < n; i++) {
            String s = "hot-literal";
            if (s == first) same++;
            ldcSum += s.length() + (i & 7);
        }
        long t1 = System.nanoTime();
        Object o = new Derived();
        int classHits = 0; long nameLen = 0;
        for (int i = 0; i < n; i++) {
            if (o.getClass() == Derived.class) classHits++;
            if (Base.class.isInstance(o)) classHits++;
            if ((i & 1023) == 0) nameLen += o.getClass().getName().length();
        }
        long t2 = System.nanoTime();
        Method step = Base.class.getDeclaredMethod("step", int.class);
        step.setAccessible(true);
        Method tw = RldcSegments.class.getDeclaredMethod("twice", int.class);
        tw.setAccessible(true);
        long invSum = 0;
        for (int i = 0; i < n; i++) {
            invSum += (Integer) step.invoke(o, i & 255);
            invSum += (Integer) tw.invoke(null, i & 127);
        }
        long t3 = System.nanoTime();
        System.out.println("ldc " + same + " " + ldcSum);
        System.out.println("class " + classHits + " " + nameLen);
        System.out.println("invoke " + invSum);
        System.err.println("SEGNANO ldc=" + (t1 - t0) + " class=" + (t2 - t1) + " invoke=" + (t3 - t2));
    }
}
JAVA

cat > "$BENCH/BenchStreamLoop.java" <<'JAVA'
import java.util.Arrays;

public class BenchStreamLoop {
    static final int[] primes = new int[64];
    static {
        int i = 0;
        for (int c = 2; i < 64; c++) {
            boolean p = true;
            for (int d = 2; d * d <= c && p; d++) if (c % d == 0) p = false;
            if (p) primes[i++] = c;
        }
    }
    static int sumStream(int[] combo) { return Arrays.stream(combo).map(i -> primes[i]).sum(); }
    static int sumLoop(int[] combo) { int s = 0; for (int i : combo) s += primes[i]; return s; }

    public static void main(String[] args) {
        final int warm = 200_000, n = 2_000_000, m = 24;
        int[] combo = new int[m];
        for (int i = 0; i < m; i++) combo[i] = i;
        long sink = 0;
        for (int i = 0; i < warm; i++) sink += sumStream(combo);
        long t0 = System.nanoTime();
        for (int i = 0; i < n; i++) sink += sumStream(combo);
        long t1 = System.nanoTime();
        for (int i = 0; i < warm; i++) sink += sumLoop(combo);
        long t2 = System.nanoTime();
        for (int i = 0; i < n; i++) sink += sumLoop(combo);
        long t3 = System.nanoTime();
        System.out.println("sink=" + (sink & 65535));
        System.err.println("NSPER stream=" + (t1 - t0) / n + " loop=" + (t3 - t2) / n
            + " stream_elem=" + (t1 - t0) / n / m + " loop_elem=" + (t3 - t2) / n / m);
    }
}
JAVA

: > "$OUT.raw"
run_one() { # $1=side $2=类名 $3=完整运行命令（bash -c 执行）
    local side="$1" c="$2" exe="$3" r
    for r in $(seq "$REPS"); do
        bash -c "$exe" > /dev/null 2> "$CLS/$c.$side.$r.err" || { echo "RUN FAIL $side $c run$r" >&2; return 1; }
        grep -hoE '(NSPER|SEGNANO) .*' "$CLS/$c.$side.$r.err" \
            | tr ' ' '\n' | grep '=' | sed "s/^\([a-zA-Z0-9_]*\)=/\1 /" \
            | awk -v c="$c" -v side="$side" -v r="$r" '{ print c "\t" side "\t" r "\t" $1 "\t" $2 }' >> "$OUT.raw"
    done
}

for c in $CASES; do
    echo "=== $c rava build"
    "$RAVA" build "$BENCH/$c.java" --release --keep-artifacts --stop-after compile > "$CLS/$c.rava.log" 2>&1 \
        || { echo "RAVA BUILD FAIL $c" >&2; tail -5 "$CLS/$c.rava.log" >&2; continue; }
    echo "=== $c javac"
    "$JAVA_HOME/bin/javac" -d "$CLS" "$BENCH/$c.java" > "$CLS/$c.javac.log" 2>&1 || { echo "JAVAC FAIL $c" >&2; continue; }
    run_one rava "$c" "build/target/release/$(snake "$c")"
    run_one jvm "$c" "$JAVA_HOME/bin/java -XX:+UseSerialGC -cp $CLS $c"
done

echo "=== 汇总（各测点中位数 / 最小 / 最大）"
python3 - "$OUT.raw" <<'PY'
import statistics, sys, collections
by = collections.defaultdict(list)
for l in open(sys.argv[1]):
    c, side, r, metric, v = l.rstrip('\n').split('\t')
    by[(c, side, metric)].append(float(v))
print('case\tside\tmetric\tmedian\tmin\tmax')
for (c, side, metric), vs in sorted(by.items()):
    print(f"{c}\t{side}\t{metric}\t{statistics.median(vs):.0f}\t{min(vs):.0f}\t{max(vs):.0f}")
PY
