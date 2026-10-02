import java.lang.invoke.MethodHandle;
import java.lang.invoke.MethodHandles;
import java.lang.reflect.Method;
import java.util.ArrayList;
import java.util.List;
import java.util.function.Supplier;

/**
 * 反射调用按接收者派发（C1d-b T3）：Method.invoke / unreflect 句柄的接收者与实参取自调用点实参池，
 * 实例方法按池中每个接收者选中实现（覆写 / 继承 / 接口默认方法），容器对象按接收者分上下文——
 * 两个装不同元素的容器经同一反射方法调用，各自只处理自己的元素；lambda 接收者经接口方法反射调用；
 * Object[] 实参数组的元素进入实参池；声明在父类的方法经子类接收者调用取子类覆写。
 */
public class TestReflectInvokePerReceiver {
    interface Shape {
        String name();
        default String describe() { return "shape " + name(); }
    }

    static class Circle implements Shape {
        public String name() { return "circle"; }
    }

    static class Square implements Shape {
        public String name() { return "square"; }
        public String describe() { return "four sides " + name(); }
    }

    static class Box {
        private final List<Object> items = new ArrayList<>();
        Box add(Object o) { items.add(o); return this; }
        // 私有容器回调：同一反射方法经两个接收者调用，各自遍历自己的元素
        private String dump(String prefix) {
            StringBuilder sb = new StringBuilder(prefix);
            for (Object o : items) sb.append(' ').append(o);
            return sb.toString();
        }
    }

    static class Base {
        public String who(Object tag) { return "base:" + tag; }
    }

    static class Derived extends Base {
        @Override
        public String who(Object tag) { return "derived:" + tag; }
    }

    public static void main(String[] args) throws Throwable {
        Method describe = Shape.class.getMethod("describe");
        Shape[] shapes = { new Circle(), new Square() };
        for (Shape s : shapes) {
            System.out.println("describe: " + describe.invoke(s));
        }

        Method dump = Box.class.getDeclaredMethod("dump", String.class);
        dump.setAccessible(true);
        Box ints = new Box().add(1).add(2).add(3);
        Box words = new Box().add("alpha").add(new StringBuilder("beta"));
        System.out.println(dump.invoke(ints, "ints:"));
        System.out.println(dump.invoke(words, "words:"));

        Method who = Base.class.getMethod("who", Object.class);
        Object[] callArgs = { Integer.valueOf(7) };
        System.out.println(who.invoke(new Base(), callArgs));
        System.out.println(who.invoke(new Derived(), callArgs));

        Method get = Supplier.class.getMethod("get");
        Supplier<String> lazy = () -> "from lambda";
        System.out.println("supplier: " + get.invoke(lazy));

        MethodHandle h = MethodHandles.lookup().unreflect(Shape.class.getMethod("name"));
        for (Shape s : shapes) {
            System.out.println("handle: " + (String) h.invoke(s));
        }

        try {
            dump.invoke(new Object(), "bad");
        } catch (IllegalArgumentException e) {
            System.out.println("IAE for wrong receiver");
        }
    }
}
