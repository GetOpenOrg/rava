import java.lang.reflect.Array;

/**
 * 反射数组深水：基本类型数组的 newInstance/Array.get 装箱、getInt 族定点访问、
 * getLength、多维、越界异常（gson/fastjson 数组序列化与 mybatis 集合映射的地基）。
 */
public class TestReflectArrayDeep {

    public static void main(String[] args) throws Exception {
        Object ints = Array.newInstance(int.class, 3);
        Array.set(ints, 0, 41);
        Array.set(ints, 1, Integer.valueOf(42));   // 装箱实参
        Array.setInt(ints, 2, 43);
        System.out.println("len=" + Array.getLength(ints));
        System.out.println("get-boxed=" + Array.get(ints, 1).getClass().getSimpleName());
        System.out.println("getInt=" + Array.getInt(ints, 2));

        // 基本类型数组不接受引用 set（类型不符 → IllegalArgumentException）
        try {
            Array.set(ints, 0, "str");
        } catch (IllegalArgumentException e) {
            System.out.println("bad-set-ex=" + e.getClass().getSimpleName());
        }

        Object strs = Array.newInstance(String.class, 2);
        Array.set(strs, 0, "a");
        System.out.println("ref-elem=" + Array.get(strs, 0)
                + " typed=" + strs.getClass().getSimpleName());

        // 多维：int[2][3]——外层数组元素初始为 null，须先建行再挂回
        Object grid = Array.newInstance(int[].class, 2);
        Object row0 = Array.newInstance(int.class, 3);
        Array.set(grid, 0, row0);
        System.out.println("row-type=" + row0.getClass().getSimpleName());
        Array.setInt(row0, 1, 9);
        System.out.println("grid-deep=" + ((int[][]) grid)[0][1]);

        // 组件类型与维度形态
        System.out.println("comp=" + ints.getClass().getComponentType().getName());
        System.out.println("grid-comp=" + grid.getClass().getComponentType().getSimpleName());

        // 越界
        try {
            Array.get(ints, 3);
        } catch (ArrayIndexOutOfBoundsException e) {
            System.out.println("oob-ex=" + e.getClass().getSimpleName());
        }
        try {
            Array.getLength(new Object());
        } catch (IllegalArgumentException e) {
            System.out.println("not-array-ex=" + e.getClass().getSimpleName());
        }
    }
}
