// FS-R6：反射创建数组（Array.newInstance / Arrays.copyOf / toArray(T[])）的运行时组件类型。
import java.lang.reflect.Array;
import java.util.*;

public class TestArrayComponentType {
    static String name(Object o) { return o.getClass().getName(); }

    public static void main(String[] args) {
        String[] a = {"x", "y"};
        String[] b = Arrays.copyOf(a, 4);
        System.out.println("copyOf " + name(b) + " len=" + b.length + " tail=" + b[3] + " same=" + (b.getClass() == String[].class));
        Object[] o = Arrays.copyOf(a, 3, Object[].class);
        System.out.println("copyOf Object[] " + name(o) + " " + (o.getClass() == Object[].class));
        CharSequence[] cs = Arrays.copyOf(a, 1, CharSequence[].class);
        System.out.println("copyOf CharSequence[] " + name(cs) + " " + cs[0]);

        List<Integer> list = new ArrayList<>(List.of(3, 1, 2));
        Integer[] ints = list.toArray(new Integer[0]);
        System.out.println("toArray " + name(ints) + " " + Arrays.toString(ints));
        String[] big = new String[5];
        Arrays.fill(big, "z");
        String[] filled = new ArrayList<>(List.of("p", "q")).toArray(big);
        System.out.println("toArray fill same=" + (filled == big) + " " + Arrays.toString(filled));

        Object arr = Array.newInstance(String.class, 2);
        System.out.println("newInstance " + name(arr) + " String[]=" + (arr instanceof String[])
            + " Object[]=" + (arr instanceof Object[]) + " CharSequence[]=" + (arr instanceof CharSequence[])
            + " Integer[]=" + (arr instanceof Integer[]) + " Cloneable=" + (arr instanceof Cloneable));
        Object[] objs = (Object[]) arr;
        objs[0] = "ok";
        try {
            objs[1] = Integer.valueOf(1);
            System.out.println("no ASE");
        } catch (ArrayStoreException e) {
            System.out.println("ASE " + e.getMessage());
        }
        try {
            Integer[] bad = (Integer[]) arr;
            System.out.println("no CCE " + bad.length);
        } catch (ClassCastException e) {
            System.out.println("CCE caught");
        }
        String[] back = (String[]) arr;
        System.out.println("back " + back[0] + " " + back[1] + " identity=" + (back == arr));
        Object nested = Array.newInstance(int[].class, 2);
        System.out.println("nested " + name(nested) + " " + (nested instanceof int[][]) + " " + (nested instanceof Object[]));
        Number[] nums = (Number[]) Array.newInstance(Integer.class, 1);
        try {
            nums[0] = Double.valueOf(1.5);
            System.out.println("no ASE");
        } catch (ArrayStoreException e) {
            System.out.println("ASE " + e.getMessage());
        }
        Object[] cl = objs.clone();
        System.out.println("clone " + name(cl) + " " + cl[0]);
        System.out.println("Object[] newInstance " + name(Array.newInstance(Object.class, 0)));
        System.out.println("int newInstance " + name(Array.newInstance(int.class, 3)) + " " + Array.getLength(Array.newInstance(long.class, 4)));
        String[] sorted = new TreeSet<>(Set.of("c", "a", "b")).toArray(new String[0]);
        System.out.println("TreeSet toArray " + name(sorted) + " " + String.join(",", sorted));
    }
}
