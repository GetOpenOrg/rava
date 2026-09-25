/**
 * 兄弟分支给同名局部变量存入不同引用类型时的合并类型（JVM 合并点语义：公共祖先；
 * JDK InetAddress.createBuiltinInetAddressResolver 揭出：声明为接口、两分支存入不同实现类，
 * 变量按首分支类型声明 → E0308）。覆盖：接口两实现（if/else）/ 三实现（switch）、
 * 兄弟类公共父类、父子类、同型、基本类型、合并后接口方法 / 覆盖方法 / toString 虚分派。
 */
public class TestBranchLocalMerge {
    interface Resolver { String resolve(String h); }
    static final class HostsFile implements Resolver {
        public String resolve(String h) { return "hosts:" + h; }
        public String toString() { return "HostsFile"; }
    }
    static final class Platform implements Resolver {
        public String resolve(String h) { return "platform:" + h; }
        public String toString() { return "Platform"; }
    }
    static final class Cached implements Resolver {
        public String resolve(String h) { return "cached:" + h; }
    }
    static abstract class Shape { abstract double area(); public String toString() { return getClass().getSimpleName() + "(" + area() + ")"; } }
    static class Square extends Shape { double s; Square(double s) { this.s = s; } double area() { return s * s; } }
    static class Circle extends Shape { double r; Circle(double r) { this.r = r; } double area() { return 3 * r * r; } }
    static class BigSquare extends Square { BigSquare() { super(10); } }

    static Resolver two(boolean hosts) {
        Resolver r;
        if (hosts) {
            r = new HostsFile();
        } else {
            r = new Platform();
        }
        return r;
    }

    static String three(int k) {
        Resolver r;
        switch (k) {
            case 0 -> r = new HostsFile();
            case 1 -> r = new Platform();
            default -> r = new Cached();
        }
        return r.resolve("h" + k);
    }

    static double siblings(boolean sq) {
        Shape s;
        if (sq) {
            s = new Square(2);
        } else {
            s = new Circle(1);
        }
        return s.area();
    }

    static String parentChild(boolean big) {
        Square q;
        if (big) {
            q = new BigSquare();
        } else {
            q = new Square(3);
        }
        return q.toString();
    }

    static String sameType(boolean a) {
        String s;
        if (a) {
            s = "left";
        } else {
            s = "right";
        }
        return s.toUpperCase();
    }

    static long primitive(boolean a) {
        long v;
        if (a) {
            v = 1L << 40;
        } else {
            v = -7;
        }
        return v + 1;
    }

    public static void main(String[] args) {
        System.out.println(two(true).resolve("a") + " " + two(false).resolve("b"));
        System.out.println(two(true) + " " + two(false));
        for (int k = 0; k < 3; k++) System.out.println(three(k));
        System.out.println(siblings(true) + " " + siblings(false));
        System.out.println(parentChild(true) + " " + parentChild(false));
        System.out.println(sameType(true) + " " + sameType(false));
        System.out.println(primitive(true) + " " + primitive(false));
    }
}
