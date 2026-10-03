import java.lang.invoke.MethodHandle;
import java.lang.invoke.MethodHandles;
import java.lang.invoke.MethodType;
import java.lang.reflect.Constructor;

// 边界用例：抽象类经构造器句柄 / 构造器反射实例化——JDK 语义为分配前抛 InstantiationException
// （DirectMethodHandle 分配器 → Unsafe.allocateInstance 按 ACC_ABSTRACT 拒绝），不得落到分配存根
public class TestAbstractCtorInstantiation {
    public static abstract class Shape {
        public Shape() {
            System.out.println("Shape ctor ran");
        }
        abstract double area();
    }

    public static class Square extends Shape {
        public Square() {
            super();
        }
        double area() {
            return 4.0;
        }
    }

    public static void main(String[] args) throws Throwable {
        MethodHandles.Lookup lookup = MethodHandles.lookup();
        MethodType ctorType = MethodType.methodType(void.class);

        MethodHandle number = MethodHandles.publicLookup().findConstructor(Number.class, ctorType);
        try {
            Object o = number.invoke();
            System.out.println("Number handle -> " + o);
        } catch (InstantiationException e) {
            System.out.println("Number handle -> InstantiationException: " + e.getMessage());
        }

        MethodHandle shape = lookup.findConstructor(Shape.class, ctorType);
        try {
            Object o = shape.invoke();
            System.out.println("Shape handle -> " + o);
        } catch (InstantiationException e) {
            System.out.println("Shape handle -> InstantiationException: " + e.getMessage());
        }

        Constructor<Shape> reflective = Shape.class.getConstructor();
        try {
            Shape s = reflective.newInstance();
            System.out.println("Shape reflect -> " + s);
        } catch (InstantiationException e) {
            System.out.println("Shape reflect -> InstantiationException: " + e.getMessage());
        }

        MethodHandle square = lookup.findConstructor(Square.class, ctorType);
        Shape sq = (Shape) square.invoke();
        System.out.println("Square handle -> area " + sq.area());
        Square sq2 = Square.class.getConstructor().newInstance();
        System.out.println("Square reflect -> area " + sq2.area());
    }
}
