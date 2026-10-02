/**
 * Class.cast / asSubclass / isInstance / isAssignableFrom 全形态
 * （含数组协变与通配边界——T2 正在细化 isAssignableFrom 语义，本例即其回归网）。
 */
public class TestClassCastSubclass {

    public static void main(String[] args) {
        Number n = Integer.valueOf(5);
        System.out.println("cast-hit=" + Number.class.cast(n));
        try {
            String.class.cast(n);
        } catch (ClassCastException e) {
            System.out.println("cast-ex=" + e.getClass().getSimpleName());
        }

        Class<? extends Number> sub = Integer.class.asSubclass(Number.class);
        System.out.println("asSubclass=" + sub.getSimpleName());
        try {
            String.class.asSubclass(Number.class);
        } catch (ClassCastException e) {
            System.out.println("asSubclass-ex=" + e.getClass().getSimpleName());
        }

        // isAssignableFrom 全形态
        System.out.println("num<-int=" + Number.class.isAssignableFrom(Integer.class));
        System.out.println("int<-num=" + Integer.class.isAssignableFrom(Number.class));
        System.out.println("prim-no-box=" + (int.class.isAssignableFrom(Integer.class) == false));
        System.out.println("arr-covariant=" + Object[].class.isAssignableFrom(Integer[].class));
        System.out.println("arr-elem=" + (Integer[].class.isAssignableFrom(Number[].class) == false));
        System.out.println("iface=" + java.util.List.class.isAssignableFrom(java.util.ArrayList.class));
        System.out.println("byname=" + forName2());
        System.out.println("identity=" + String.class.isAssignableFrom(String.class));

        // isInstance：实例视角（null 恒 false）
        System.out.println("isInstance=" + java.util.List.class.isInstance(new java.util.ArrayList<>()));
        System.out.println("isInstance-null=" + (java.util.List.class.isInstance(null) == false));
        System.out.println("isInstance-arr=" + Object[].class.isInstance(new Integer[0]));
        System.out.println("isInstance-prim=" + (int.class.isInstance(5) == false));
    }

    static boolean forName2() {
        try {
            return java.util.List.class
                    .isAssignableFrom(Class.forName("java.util.LinkedList"));
        } catch (ClassNotFoundException e) {
            return false;
        }
    }
}
