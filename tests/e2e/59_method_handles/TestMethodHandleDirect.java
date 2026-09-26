import java.lang.invoke.MethodHandle;
import java.lang.invoke.MethodHandles;
import java.lang.invoke.MethodType;
import java.lang.invoke.WrongMethodTypeException;
import java.lang.reflect.Constructor;
import java.lang.reflect.Field;
import java.lang.reflect.Method;

/**
 * MethodHandle 直接句柄（MH-native #40）：Lookup.findStatic / findVirtual（类与接口虚分派）/
 * findConstructor / findGetter / findSetter / findStaticGetter / findStaticSetter、
 * unreflect / unreflectConstructor / unreflectGetter；invokeExact 与 invoke（装箱/拆箱适配）；
 * MethodType 文本；异常穿透、WrongMethodTypeException、NoSuchMethodException、NoSuchFieldException；
 * 同 nest 私有成员访问。
 */
public class TestMethodHandleDirect {
    interface Shape { double area(); }
    static class Sq implements Shape {
        final double s;
        Sq(double s) { this.s = s; }
        public double area() { return s * s; }
    }
    static class Circle implements Shape {
        final double r;
        Circle(double r) { this.r = r; }
        public double area() { return 3 * r * r; }
    }
    static class Point {
        int x;
        private int y;
        static int count = 5;
        Point(int x, int y) { this.x = x; this.y = y; }
        int sum() { return x + y; }
        long scaled(long k) { return (x + y) * k; }
        static String tag(String a, int n) { return a + "#" + n; }
    }
    record Pair(String k, int v) {}

    static void boom() { throw new IllegalStateException("boom"); }

    public static void main(String[] args) throws Throwable {
        MethodHandles.Lookup lookup = MethodHandles.lookup();

        MethodHandle tag = lookup.findStatic(Point.class, "tag",
                MethodType.methodType(String.class, String.class, int.class));
        System.out.println("type " + tag.type());
        System.out.println("exact " + (String) tag.invokeExact("a", 3));
        System.out.println("invoke " + tag.invoke("b", Integer.valueOf(4)));

        MethodHandle area = lookup.findVirtual(Shape.class, "area", MethodType.methodType(double.class));
        for (Shape s : new Shape[] {new Sq(2), new Circle(1)}) {
            System.out.println("area " + (double) area.invokeExact(s));
        }

        MethodHandle ctor = lookup.findConstructor(Point.class,
                MethodType.methodType(void.class, int.class, int.class));
        System.out.println("ctor type " + ctor.type());
        Point p = (Point) ctor.invoke(3, 4);
        MethodHandle sum = lookup.findVirtual(Point.class, "sum", MethodType.methodType(int.class));
        System.out.println("sum " + (int) sum.invokeExact(p));
        MethodHandle scaled = lookup.findVirtual(Point.class, "scaled", MethodType.methodType(long.class, long.class));
        System.out.println("scaled " + (long) scaled.invoke(p, 10));

        MethodHandle gx = lookup.findGetter(Point.class, "x", int.class);
        MethodHandle sy = lookup.findSetter(Point.class, "y", int.class);
        sy.invoke(p, 10);
        System.out.println("field x=" + (int) gx.invoke(p) + " sum=" + (int) sum.invoke(p));
        MethodHandle gy = lookup.findGetter(Point.class, "y", int.class);
        System.out.println("private y=" + (int) gy.invokeExact(p));

        MethodHandle cnt = lookup.findStaticGetter(Point.class, "count", int.class);
        MethodHandle setCnt = lookup.findStaticSetter(Point.class, "count", int.class);
        setCnt.invoke(42);
        System.out.println("static count=" + (int) cnt.invoke() + " direct=" + Point.count);

        Method m = Point.class.getDeclaredMethod("tag", String.class, int.class);
        System.out.println("unreflect " + (String) lookup.unreflect(m).invoke("i", 8));
        Constructor<Pair> pc = Pair.class.getDeclaredConstructor(String.class, int.class);
        Object pair = lookup.unreflectConstructor(pc).invoke("k", 1);
        System.out.println("unreflectConstructor " + pair);
        Field fy = Point.class.getDeclaredField("y");
        System.out.println("unreflectGetter " + (int) lookup.unreflectGetter(fy).invoke(p));

        MethodHandle boom = lookup.findStatic(TestMethodHandleDirect.class, "boom", MethodType.methodType(void.class));
        try {
            boom.invoke();
        } catch (IllegalStateException e) {
            System.out.println("caught " + e.getMessage());
        }
        try {
            String s = (String) tag.invokeExact("x");
            System.out.println("no WMTE " + s);
        } catch (WrongMethodTypeException e) {
            System.out.println("WMTE");
        }
        try {
            lookup.findStatic(Point.class, "nope", MethodType.methodType(void.class));
        } catch (NoSuchMethodException e) {
            System.out.println("NSME");
        }
        try {
            lookup.findGetter(Point.class, "z", int.class);
        } catch (NoSuchFieldException e) {
            System.out.println("NSFE");
        }
    }
}
