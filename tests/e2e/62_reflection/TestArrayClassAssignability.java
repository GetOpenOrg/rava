import java.io.Serializable;

/** Class.isAssignableFrom 边界：数组协变、多维、基本元素、数组到 Object/Cloneable/Serializable、基本类型类、null。 */
public class TestArrayClassAssignability {
    static void check(String label, Class<?> target, Class<?> source) {
        System.out.println(label + "=" + target.isAssignableFrom(source));
    }

    public static void main(String[] args) {
        check("Object[]<-Integer[]", Object[].class, Integer[].class);
        check("Integer[]<-Object[]", Integer[].class, Object[].class);
        check("Number[]<-Integer[]", Number[].class, Integer[].class);
        check("Comparable[]<-String[]", Comparable[].class, String[].class);
        check("Number[][]<-Integer[][]", Number[][].class, Integer[][].class);
        check("Object[]<-int[][]", Object[].class, int[][].class);
        check("Object[]<-int[]", Object[].class, int[].class);
        check("Object<-int[]", Object.class, int[].class);
        check("Cloneable<-String[]", Cloneable.class, String[].class);
        check("Serializable<-long[]", Serializable.class, long[].class);
        check("Serializable[]<-String[][]", Serializable[].class, String[][].class);
        check("Runnable<-int[]", Runnable.class, int[].class);
        check("int[]<-int[]", int[].class, int[].class);
        check("long[]<-int[]", long[].class, int[].class);
        check("Integer[]<-int[]", Integer[].class, int[].class);
        check("int<-int", int.class, int.class);
        check("long<-int", long.class, int.class);
        check("Object<-int", Object.class, int.class);
        check("Integer<-int", Integer.class, int.class);
        check("Number<-Integer", Number.class, Integer.class);
        check("Integer<-Number", Integer.class, Number.class);
        Object[] holder = new Integer[] {1, 2};
        System.out.println("runtime=" + holder.getClass().getName() + "," + Object[].class.isInstance(holder));
        try {
            Object.class.isAssignableFrom(null);
            System.out.println("null=no-throw");
        } catch (NullPointerException e) {
            System.out.println("null=NPE");
        }
    }
}
