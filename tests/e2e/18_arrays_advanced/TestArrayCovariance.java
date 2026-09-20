import java.util.ArrayList;
import java.util.List;

public class TestArrayCovariance {

    static void inspect(Object[] arr, String tag) {
        System.out.print(tag + " type=" + arr.getClass().getSimpleName() + " values=");
        for (Object o : arr) {
            System.out.print(o + " ");
        }
        System.out.println();
    }

    public static void main(String[] args) {
        // String[] 可以赋给 Object[]（数组协变）
        String[] strings = {"a", "b", "c"};
        Object[] objects = strings;
        inspect(objects, "as-object-array");

        // 通过 Object[] 引用写入错误的元素类型 → ArrayStoreException
        try {
            objects[0] = Integer.valueOf(1);
            System.out.println("store succeeded unexpectedly");
        } catch (ArrayStoreException e) {
            System.out.println("caught ArrayStoreException");
        }

        // 写入正确的类型是可以的
        objects[1] = "z";
        System.out.println("strings[1]=" + strings[1]);

        // 把 Object[] 再向下转回具体数组类型
        String[] back = (String[]) objects;
        System.out.println("back[0]=" + back[0] + " length=" + back.length);

        // 错误的向下转型 → ClassCastException
        Integer[] ints = new Integer[]{1, 2};
        Object[] objs2 = ints;
        try {
            String[] wrong = (String[]) objs2;
            System.out.println("wrong cast ok: " + wrong.length);
        } catch (ClassCastException e) {
            System.out.println("caught ClassCastException");
        }

        // 二维数组也是协变的
        String[][] matrix = {{"x", "y"}, {"z"}};
        Object[][] asObject = matrix;
        System.out.println("matrix rows=" + asObject.length + " row0=" + asObject[0].length);
        Object[] firstRow = asObject[0];
        System.out.println("row0[0]=" + firstRow[0]);

        // 数组作为 Object 传递
        Object asObjRef = strings;
        System.out.println("isArray=" + (asObjRef instanceof String[]));

        // 通过 List<Object> 收集不同数组类型
        List<Object[]> listOfArrays = new ArrayList<>();
        listOfArrays.add(strings);
        listOfArrays.add(ints);
        for (Object[] a : listOfArrays) {
            System.out.println("collected len=" + a.length + " component=" + a.getClass().getSimpleName());
        }

        // System.arraycopy 跨数组类型的存检查（同类型安全）
        String[] dest = new String[3];
        System.arraycopy(strings, 0, dest, 0, 3);
        System.out.println("dest=" + String.join(",", dest));

        System.out.println("done");
    }
}
