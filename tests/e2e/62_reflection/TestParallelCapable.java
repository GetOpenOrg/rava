/**
 * 并行加载器登记（FS-C2 边界）：ClassLoader.registerAsParallelCapable() 按调用者类（CallerSensitive）登记，
 * 只有父类已登记时才成功；内建加载器（BuiltinClassLoader 族）在各自 static 块登记，父类 SecureClassLoader
 * 须先完成登记。覆盖：直接继承 ClassLoader 的子类登记成功、父类未登记的子类登记失败、已登记类的子类
 * 登记成功、未调用登记的类、系统 / 平台加载器的登记状态。
 */
public class TestParallelCapable {
    static class Capable extends ClassLoader {
        static final boolean REG = registerAsParallelCapable();
        Capable() { super(null); }
    }

    static class Plain extends ClassLoader {
        Plain() { super(null); }
    }

    static class ChildOfPlain extends Plain {
        static final boolean REG = registerAsParallelCapable();
    }

    static class ChildOfCapable extends Capable {
        static final boolean REG = registerAsParallelCapable();
    }

    public static void main(String[] args) {
        System.out.println("Capable register = " + Capable.REG);
        System.out.println("Capable registered = " + new Capable().isRegisteredAsParallelCapable());
        System.out.println("ChildOfPlain register = " + ChildOfPlain.REG);
        System.out.println("ChildOfPlain registered = " + new ChildOfPlain().isRegisteredAsParallelCapable());
        System.out.println("Plain registered = " + new Plain().isRegisteredAsParallelCapable());
        System.out.println("ChildOfCapable register = " + ChildOfCapable.REG);
        System.out.println("ChildOfCapable registered = " + new ChildOfCapable().isRegisteredAsParallelCapable());
        System.out.println("system registered = " + ClassLoader.getSystemClassLoader().isRegisteredAsParallelCapable());
        System.out.println("platform registered = " + ClassLoader.getPlatformClassLoader().isRegisteredAsParallelCapable());
    }
}
