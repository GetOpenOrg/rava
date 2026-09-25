public class ArrayTest {
    // 数组作为方法参数和返回值
    public static int[] createIntArray(int size) {
        int[] arr = new int[size];
        for (int i = 0; i < size; i++) {
            arr[i] = i * 10;
        }
        return arr;
    }

    public static void printIntArray(int[] arr) {
        for (int i = 0; i < arr.length; i++) {
            System.out.println(arr[i]);
        }
    }

    public static void main(String[] args) {
        // int[] 创建、赋值、读取、.length
        int[] nums = new int[5];
        nums[0] = 10;
        nums[1] = 20;
        nums[2] = 30;
        nums[3] = 40;
        nums[4] = 50;
        System.out.println("int array length: " + nums.length);
        System.out.println("nums[0] = " + nums[0]);
        System.out.println("nums[4] = " + nums[4]);

        // double[] 操作
        double[] doubles = new double[3];
        doubles[0] = 1.5;
        doubles[1] = 2.5;
        doubles[2] = 3.5;
        System.out.println("double array length: " + doubles.length);
        System.out.println("doubles[1] = " + doubles[1]);

        // boolean[] 操作
        boolean[] bools = new boolean[2];
        bools[0] = true;
        bools[1] = false;
        System.out.println("bools[0] = " + bools[0]);
        System.out.println("bools[1] = " + bools[1]);

        // String[] 操作
        String[] names = new String[3];
        names[0] = "Alice";
        names[1] = "Bob";
        names[2] = "Charlie";
        System.out.println("names length: " + names.length);
        System.out.println("names[2] = " + names[2]);

        // 数组作为方法参数和返回值
        int[] created = createIntArray(4);
        System.out.println("created length: " + created.length);
        printIntArray(created);

        // 数组修改（store 后 load 验证）
        nums[2] = 999;
        System.out.println("nums[2] after modify = " + nums[2]);

        // 二维数组 (multianewarray)
        int[][] matrix = new int[2][3];
        matrix[0][0] = 1;
        matrix[0][1] = 2;
        matrix[0][2] = 3;
        matrix[1][0] = 4;
        matrix[1][1] = 5;
        matrix[1][2] = 6;
        System.out.println("matrix[0][1] = " + matrix[0][1]);
        System.out.println("matrix[1][2] = " + matrix[1][2]);
        System.out.println("matrix length: " + matrix.length);
        System.out.println("matrix[0] length: " + matrix[0].length);

        // System.arraycopy
        int[] src = {10, 20, 30, 40, 50};
        int[] dest = new int[5];
        System.arraycopy(src, 1, dest, 0, 3);
        System.out.println("arraycopy dest[0] = " + dest[0]);
        System.out.println("arraycopy dest[1] = " + dest[1]);
        System.out.println("arraycopy dest[2] = " + dest[2]);
    }
}
