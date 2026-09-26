// FS-C5：Class.forName(name) 立即初始化（JLS §12.4.1），forName(name, false, loader) 不初始化；
// 初始化失败 → ExceptionInInitializerError，其后 NoClassDefFoundError。
public class TestForNameInit {
    static class Eager { static { System.out.println("Eager <clinit>"); } static int v = 1; }
    static class Passive { static { System.out.println("Passive <clinit>"); } static int v = 2; }
    static class Child extends Parent { static { System.out.println("Child <clinit>"); } }
    static class Parent { static { System.out.println("Parent <clinit>"); } }
    static class Broken { static int v; static { if (true) throw new IllegalStateException("bad init"); } }

    // 类字面量使类进入闭包（ldc 不触发初始化）
    static final Class<?>[] KEEP = {Eager.class, Passive.class, Child.class, Broken.class};

    public static void main(String[] args) throws Exception {
        System.out.println("start");
        Class<?> e = Class.forName("TestForNameInit$Eager");
        System.out.println("after forName Eager: " + e.getSimpleName());
        Class<?> p = Class.forName("TestForNameInit$Passive", false, TestForNameInit.class.getClassLoader());
        System.out.println("after passive forName: " + p.getSimpleName());
        System.out.println("Passive.v=" + Passive.v);
        Class.forName("TestForNameInit$Child");
        System.out.println("after Child");
        Class.forName("TestForNameInit$Eager");
        System.out.println("second forName no re-init");
        try {
            Class.forName("TestForNameInit$Broken");
        } catch (ExceptionInInitializerError err) {
            System.out.println("EIIE cause " + err.getCause().getClass().getName() + ": " + err.getCause().getMessage());
        }
        try {
            Class.forName("TestForNameInit$Broken");
        } catch (NoClassDefFoundError err) {
            System.out.println("NCDFE");
        }
        try {
            Class.forName("TestForNameInit$Missing");
        } catch (ClassNotFoundException ex) {
            System.out.println("CNFE " + ex.getMessage());
        }
        System.out.println("array " + Class.forName("[I").getName());
    }
}
