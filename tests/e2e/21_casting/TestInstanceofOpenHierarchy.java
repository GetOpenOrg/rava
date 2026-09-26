import java.io.Serializable;

/**
 * instanceof 的开放层次判定（K-JCA 揭出：`MessageDigestSpi 变量 instanceof Cloneable` 被编译期
 * 折叠为 false，MessageDigest.clone 失败）。互不为子类型时：一侧接口、另一侧非 final 类 →
 * 子类可实现该接口，须运行时判定。覆盖：抽象类 / 普通类变量 instanceof 接口（子类实现 / 未实现）、
 * 接口变量 instanceof 非 final 类（实现类是 / 不是其子类、lambda）、null、多层继承。
 */
public class TestInstanceofOpenHierarchy {
    static abstract class Spi {}
    static class Plain extends Spi {}
    static class Impl extends Spi implements Cloneable, Runnable {
        public void run() {}
    }
    static class Deeper extends Impl {}
    static class Base {}
    static class Sub extends Base implements Runnable {
        public void run() {}
    }
    static class Other implements Runnable {
        public void run() {}
    }

    static String checkSpi(Spi s) {
        return (s instanceof Cloneable) + "/" + (s instanceof Runnable) + "/" + (s instanceof Serializable);
    }

    static String checkRunnable(Runnable r) {
        return (r instanceof Base) + "/" + (r instanceof Spi) + "/" + (r instanceof Impl);
    }

    public static void main(String[] args) {
        System.out.println("plain " + checkSpi(new Plain()));
        System.out.println("impl " + checkSpi(new Impl()));
        System.out.println("deeper " + checkSpi(new Deeper()));
        System.out.println("null " + checkSpi((Spi) null));
        System.out.println("sub " + checkRunnable(new Sub()));
        System.out.println("other " + checkRunnable(new Other()));
        System.out.println("implAsRunnable " + checkRunnable(new Impl()));
        Runnable lambda = () -> {};
        System.out.println("lambda " + checkRunnable(lambda));
    }
}
