import java.lang.reflect.Method;

public class ListMethods {
    public int examplePublicInstanceMethod(char c, double d) {
        return 42;
    }

    private boolean examplePrivateInstanceMethod(String s) {
        return true;
    }

    public static void main(String[] args) {
        Class clazz = ListMethods.class;

        // getMethods()/getDeclaredMethods() 的返回顺序 JVM 不作保证——排序后打印，
        // 集合内容不变、输出确定（golden 双跑一致的先决条件）
        System.out.println("All public methods (including inherited):");
        Method[] publics = clazz.getMethods();
        java.util.Arrays.sort(publics, java.util.Comparator.comparing(Method::toString));
        for (Method m : publics) {
            System.out.println(m);
        }
        System.out.println();
        System.out.println("All declared methods (excluding inherited):");
        Method[] declared = clazz.getDeclaredMethods();
        java.util.Arrays.sort(declared, java.util.Comparator.comparing(Method::toString));
        for (Method m : declared) {
            System.out.println(m);
        }
    }
}
