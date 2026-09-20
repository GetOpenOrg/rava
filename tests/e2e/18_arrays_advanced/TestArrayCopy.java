import java.util.Arrays;

public class TestArrayCopy {

    static void show(int[] a, String tag) {
        System.out.println(tag + Arrays.toString(a));
    }

    static void showObjs(Object[] a, String tag) {
        System.out.println(tag + Arrays.toString(a));
    }

    public static void main(String[] args) {
        // Arrays.copyOf / copyOfRange
        int[] src = {1, 2, 3, 4, 5};
        int[] copy = Arrays.copyOf(src, 3);
        show(copy, "copyOf(3)=");
        int[] grown = Arrays.copyOf(src, 7);
        show(grown, "copyOf(7)=");
        int[] range = Arrays.copyOfRange(src, 1, 4);
        show(range, "copyOfRange(1,4)=");

        // System.arraycopy 基本类型
        int[] dst = new int[6];
        System.arraycopy(src, 0, dst, 0, 5);
        show(dst, "arraycopy=");

        int[] offset = new int[6];
        System.arraycopy(src, 1, offset, 2, 3);
        show(offset, "arraycopy offset=");

        // 自复制且区间重叠（向前/向后）
        int[] overlapForward = {1, 2, 3, 4, 5};
        System.arraycopy(overlapForward, 0, overlapForward, 1, 4);
        show(overlapForward, "self copy forward=");

        int[] overlapBackward = {1, 2, 3, 4, 5};
        System.arraycopy(overlapBackward, 1, overlapBackward, 0, 4);
        show(overlapBackward, "self copy backward=");

        // 引用类型数组复制（浅拷贝语义）
        StringBuilder sb = new StringBuilder("shared");
        StringBuilder[] refs = {sb, new StringBuilder("b")};
        StringBuilder[] refCopy = Arrays.copyOf(refs, 3);
        refCopy[0].append("!");
        System.out.println("identity shared=" + (refCopy[0] == refs[0]));
        showObjs(refCopy, "refCopy=");
        System.out.println("original mutated=" + refs[0]);

        // 二维数组整体 / 行级复制
        int[][] matrix = {{1, 2}, {3, 4}, {5, 6}};
        int[][] matrixCopy = Arrays.copyOf(matrix, 2);
        System.out.println("rows=" + matrixCopy.length + " row0=" + Arrays.toString(matrixCopy[0]));
        int[][] deepRow = {Arrays.copyOf(matrix[0], 3), Arrays.copyOf(matrix[1], 3)};
        deepRow[0][2] = 99;
        System.out.println("deep row0=" + Arrays.toString(deepRow[0]) + " original row0=" + Arrays.toString(matrix[0]));

        // 边界校验
        try {
            System.arraycopy(src, 0, new int[2], 0, 5);
            System.out.println("overflow accepted");
        } catch (ArrayIndexOutOfBoundsException e) {
            System.out.println("arraycopy oob -> " + e.getClass().getSimpleName());
        }
        try {
            System.arraycopy(src, -1, new int[5], 0, 2);
            System.out.println("negative accepted");
        } catch (IndexOutOfBoundsException e) {
            System.out.println("negative srcPos -> " + e.getClass().getSimpleName());
        }
        try {
            System.arraycopy("not an array", 0, new int[5], 0, 1);
            System.out.println("non-array accepted");
        } catch (RuntimeException e) {
            System.out.println("non-array src -> " + e.getClass().getSimpleName());
        }

        // clone
        int[] cloned = src.clone();
        cloned[0] = 100;
        show(cloned, "cloned=");
        show(src, "source unchanged=");

        System.out.println("done");
    }
}
