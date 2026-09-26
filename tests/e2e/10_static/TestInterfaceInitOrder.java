/**
 * 类初始化触发点（S-10，JVMS §5.5）：类初始化时先初始化**声明了 default 方法**的超接口
 * （含间接超接口，按递归枚举序）；无 default 方法的接口不随实现类初始化，只在访问其
 * 非常量静态字段时初始化；子接口初始化不触发父接口初始化。
 */
public class TestInterfaceInitOrder {
    static int log(String s) {
        System.out.println(s);
        return 1;
    }

    interface WithDefault {
        int A = log("WithDefault init");
        default String hello() { return "hello"; }
    }
    interface NoDefault {
        int B = log("NoDefault init");
        void f();
    }
    interface SubDefault extends WithDefault {
        int C = log("SubDefault init");
    }
    interface OwnDefault {
        int D = log("OwnDefault init");
        default int four() { return 4; }
    }
    static class Base implements OwnDefault {
        static { log("Base init"); }
    }
    static class Impl extends Base implements NoDefault, SubDefault {
        static { log("Impl init"); }
        public void f() {}
    }

    public static void main(String[] args) {
        log("main start");
        Impl i = new Impl();
        log("after new");
        System.out.println(i.hello() + " " + i.four());
        System.out.println("B=" + NoDefault.B);
        System.out.println("C=" + SubDefault.C);
    }
}
