import java.util.Arrays;

/**
 * Arrays 深水族：deepHashCode 一致性、deepEquals 嵌套与 null 边界、deepToString
 * 固定形态（guava Objects.hash 场景与 assertj 数组断言的地基）。
 */
public class TestArraysDeepOps {

    public static void main(String[] args) {
        Object[] a = { 1, new int[] { 2, 3 }, new String[] { "x" } };
        Object[] b = { 1, new int[] { 2, 3 }, new String[] { "x" } };

        // 浅层 hashCode 对嵌套数组不等（数组身份 hash），deep 相等
        System.out.println("shallow-ne=" + (Arrays.hashCode(a) != Arrays.hashCode(b)));
        System.out.println("deep-hash-eq=" + (Arrays.deepHashCode(a) == Arrays.deepHashCode(b)));
        System.out.println("deep-eq=" + Arrays.deepEquals(a, b));

        b[1] = new int[] { 2, 4 };
        System.out.println("deep-ne=" + !Arrays.deepEquals(a, b));

        // null 与自引用边界
        System.out.println("null-eq=" + Arrays.deepEquals(null, null));
        System.out.println("null-vs=" + !Arrays.deepEquals(null, a));
        System.out.println("one-null=" + !Arrays.deepEquals(new Object[] { null }, new Object[] { 1 }));
        // 自引用：同一实例 == 快捷命中；不同实例互指会 StackOverflowError（deepEquals 无环保护）
        Object[] self = new Object[1];
        self[0] = self;
        System.out.println("self-same=" + Arrays.deepEquals(self, self));
        try {
            Object[] other = new Object[1];
            other[0] = other;
            Arrays.deepEquals(self, other);
            System.out.println("self-cross=ok");
        } catch (StackOverflowError e) {
            System.out.println("self-cross-ex=" + e.getClass().getSimpleName());
        }
        // deepToString 有环保护：自引用打印 [...]
        System.out.println("self-str=" + Arrays.deepToString(self));

        // deepToString 固定形态
        System.out.println("str=" + Arrays.deepToString(a));
        System.out.println("str-nested=" + Arrays.deepToString(
                new Object[] { new long[][] { { 1L }, { 2L, 3L } } }));
        System.out.println("str-null=" + Arrays.deepToString(new Object[] { null }));
    }
}
