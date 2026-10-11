// System.arraycopy 前置检查（HotSpot 顺序与消息）：null / 非数组 / 类型不符 / 负参数 / 越界，越界时目标不被部分写入；
// 引用数组元素存储失败时保留已复制前缀；同数组重叠复制按 memmove 语义。
import java.util.Arrays;

public class TestArrayCopyChecks {
    static void run(String label, Object src, int sp, Object dst, int dp, int len) {
        try {
            System.arraycopy(src, sp, dst, dp, len);
            System.out.println(label + ": ok");
        } catch (RuntimeException e) {
            System.out.println(label + ": " + e.getClass().getName() + ": " + e.getMessage());
        }
    }

    public static void main(String[] args) {
        int[] src = {1, 2, 3};
        int[] d = {10, 20, 30};
        run("neg-len", src, 0, d, 0, -1);
        run("neg-srcpos", src, -1, d, 0, 1);
        run("neg-dstpos", src, 0, d, -1, 1);
        run("src-last", src, 2, d, 0, 2);
        run("dst-last", src, 0, d, 2, 2);
        run("both-over", src, 0, d, 0, 5);
        run("overflow", src, 1, d, 0, Integer.MAX_VALUE);
        run("negpos-len0", src, -1, new int[3], 0, 0);
        run("len0-at-end", src, 3, d, 3, 0);
        System.out.println("d untouched: " + Arrays.toString(d));
        run("null-src", null, 0, d, 0, 1);
        run("null-dst", src, 0, null, 0, 1);
        run("null-src-len0", null, 0, d, 0, 0);
        run("src-not-array", "abc", 0, d, 0, 1);
        run("dst-not-array", src, 0, Integer.valueOf(1), 0, 1);
        run("prim-mismatch", src, 0, new long[3], 0, 1);
        run("prim-to-obj", src, 0, new Object[3], 0, 1);
        run("obj-to-prim", new Object[3], 0, new byte[3], 0, 1);
        run("obj-array-neg", new String[2], 0, new Object[2], 0, -2);

        Object[] mixed = {"a", "b", Integer.valueOf(7), "d"};
        String[] strs = new String[4];
        run("elem-store", mixed, 0, strs, 0, 4);
        System.out.println("strs prefix: " + Arrays.toString(strs));
        Number[] nums = new Number[2];
        run("elem-store-sub", new Integer[]{1, 2}, 0, nums, 0, 2);
        System.out.println("nums: " + Arrays.toString(nums));
        run("type-mismatch-obj", new Integer[]{1}, 0, new String[1], 0, 1);

        int[] ov = {1, 2, 3, 4, 5};
        System.arraycopy(ov, 0, ov, 1, 4);
        System.out.println("overlap fwd: " + Arrays.toString(ov));
        int[] ov2 = {1, 2, 3, 4, 5};
        System.arraycopy(ov2, 1, ov2, 0, 4);
        System.out.println("overlap back: " + Arrays.toString(ov2));
        char[] cs = "hello".toCharArray();
        char[] cd = new char[5];
        System.arraycopy(cs, 1, cd, 0, 4);
        System.out.println("chars: " + new String(cd, 0, 4));
    }
}
