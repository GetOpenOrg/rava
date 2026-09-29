import java.io.ByteArrayInputStream;
import java.io.ByteArrayOutputStream;
import java.io.ObjectInputStream;
import java.io.ObjectOutputStream;
import java.io.ObjectStreamClass;
import java.io.Serializable;
import java.util.ArrayList;
import java.util.List;

// 未声明 serialVersionUID 的类：ObjectStreamClass 按 JDK 算法计算默认 SUID
// （类名 / 修饰符 / 接口 / 字段 / 是否有静态初始化器 / 构造器 / 方法 → SHA-1）
public class TestSerialDefaultSuid {
    static class Point implements Serializable {
        int x;
        int y;
        Point(int x, int y) { this.x = x; this.y = y; }
        public String toString() { return "Point(" + x + "," + y + ")"; }
    }

    static class WithStatic implements Serializable {
        static int counter = initCounter();
        String name = "w";
        transient int skipped = 7;
        private long id;
        static int initCounter() { return 5; }
        protected void touch() { id++; }
        public String toString() { return "WithStatic(" + name + "," + skipped + "," + id + ")"; }
    }

    static final class Holder implements Serializable {
        final List<String> items = new ArrayList<>();
        public Holder add(String s) { items.add(s); return this; }
        public String toString() { return "Holder" + items; }
    }

    record R(int a, String b) implements Serializable {}

    enum E { A, B }

    sealed interface Shape permits Circle, Square {}
    record Circle(double r) implements Shape {}
    record Square(double s) implements Shape {}

    public static void main(String[] args) throws Exception {
        for (Class<?> c : new Class<?>[]{Point.class, WithStatic.class, Holder.class, R.class, E.class,
                String.class, Integer.class, ArrayList.class}) {
            System.out.println(c.getName() + " suid=" + ObjectStreamClass.lookup(c).getSerialVersionUID());
        }
        ByteArrayOutputStream bos = new ByteArrayOutputStream();
        try (ObjectOutputStream oos = new ObjectOutputStream(bos)) {
            oos.writeObject(new Point(1, 2));
            oos.writeObject(new WithStatic());
            oos.writeObject(new Holder().add("a").add("b"));
        }
        try (ObjectInputStream ois = new ObjectInputStream(new ByteArrayInputStream(bos.toByteArray()))) {
            System.out.println(ois.readObject());
            System.out.println(ois.readObject());
            System.out.println(ois.readObject());
        }

        System.out.println("sealed=" + Shape.class.isSealed() + " " + Circle.class.isSealed() + " " + Point.class.isSealed());
        Class<?>[] permitted = Shape.class.getPermittedSubclasses();
        StringBuilder sb = new StringBuilder();
        for (Class<?> p : permitted) sb.append(p.getSimpleName()).append(' ');
        System.out.println("permitted=" + sb.toString().trim() + " nonSealed=" + (Point.class.getPermittedSubclasses() == null));
    }
}
