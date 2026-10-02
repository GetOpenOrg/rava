/**
 * Class.forName 的初始化边界：三参 initialize true/false 与 <clinit> 触发计数、
 * 数组/内部类名形态、基本类型简名不可寻（框架按名加载的完整通道）。
 */
public class TestClassForNameInit {

    static class InitCounter {
        static int count;

        static {
            count++;
        }
    }

    static class Nested {
    }

    public static void main(String[] args) throws Exception {
        String name = "TestClassForNameInit$InitCounter";
        ClassLoader cl = TestClassForNameInit.class.getClassLoader();

        Class<?> c1 = Class.forName(name, false, cl);
        System.out.println("lazy-count=" + InitCounter.count);
        System.out.println("same-class=" + (c1 == InitCounter.class));

        Class<?> c2 = Class.forName(name, true, cl);
        System.out.println("init-count=" + InitCounter.count);
        System.out.println("cached=" + (c1 == c2));

        // 数组类名形态
        Class<?> arr = Class.forName("[Ljava.lang.String;");
        System.out.println("arr=" + arr.getName() + " isArray=" + arr.isArray());
        Class<?> primArr = Class.forName("[[I");
        System.out.println("prim-arr=" + primArr.getName());

        // 内部类 $ 名 ✓，点形态 ✗
        System.out.println("dollar=" + Class.forName(name).getSimpleName());
        try {
            Class.forName("TestClassForNameInit.Nested");
        } catch (ClassNotFoundException e) {
            System.out.println("dot-ex=" + e.getClass().getSimpleName());
        }

        // 基本类型简名不可寻（int.class 反射镜像存在但 forName 拒绝）
        try {
            Class.forName("int");
        } catch (ClassNotFoundException e) {
            System.out.println("prim-ex=" + e.getClass().getSimpleName());
        }
        System.out.println("prim-mirror=" + int.class.getName());

        // 未命中类名
        try {
            Class.forName("no.such.Clazz");
        } catch (ClassNotFoundException e) {
            System.out.println("miss-ex=" + e.getClass().getSimpleName()
                    + " name=" + e.getMessage());
        }
    }
}
