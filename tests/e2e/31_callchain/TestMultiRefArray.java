/**
 * 调用链缺口 A3：multianewarray 创建的「多维引用类型数组」。
 *
 * `new String[2][3]` 编译为 multianewarray，常量池类条目形如 `[[Ljava/lang/String;`。
 * 该字符串含前导 `[`，会被引用收集逻辑的 `'[' not in ...` 守卫过滤掉，
 * 导致数组元素类型不进闭包。
 *
 * 已有的 TestMultiArray 只覆盖基本类型多维数组（int[][]），不触发此路径。
 */
public class TestMultiRefArray {
    public static void main(String[] args) {
        String[][] grid = new String[2][3];
        grid[0][0] = "a";
        grid[0][1] = "b";
        grid[0][2] = "c";
        grid[1][0] = "d";
        grid[1][1] = "e";
        grid[1][2] = "f";

        System.out.println(grid[0][0]);
        System.out.println(grid[1][2]);
        System.out.println(grid.length);
        System.out.println(grid[0].length);

        for (String[] row : grid) {
            for (String v : row) {
                System.out.print(v + " ");
            }
        }
        System.out.println();

        // 三维引用数组
        String[][][] cube = new String[2][2][2];
        cube[0][0][0] = "x";
        cube[1][1][1] = "y";
        System.out.println(cube[0][0][0]);
        System.out.println(cube[1][1][1]);

        // Object 多维数组（元素类型在编译期是 Object）
        Object[][] objs = new Object[2][2];
        objs[0][0] = "hello";
        objs[0][1] = 42;
        System.out.println(objs[0][0]);
        System.out.println(objs[0][1]);

        // 引用数组的默认值
        String[][] empty = new String[2][2];
        System.out.println(empty[0][0] == null);
    }
}
