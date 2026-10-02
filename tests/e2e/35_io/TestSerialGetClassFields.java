import java.io.ByteArrayInputStream;
import java.io.ByteArrayOutputStream;
import java.io.IOException;
import java.io.ObjectInputStream;
import java.io.ObjectOutputStream;
import java.io.Serializable;
import java.lang.reflect.Field;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;

// getClass 作用于只按接口 / Object 类型可知的值：类镜像取已实例化子类集合，
// 字段枚举（序列化描述符、getDeclaredFields + Field.get）按接收者类镜像逐类进行；
// lambda 合成类的类镜像：超类是 Object、无捕获时无字段
public class TestSerialGetClassFields {
    interface Shape extends Serializable {
        double area();
    }

    static class Base implements Serializable {
        private static final long serialVersionUID = 1L;
        protected String tag = "base";
    }

    static class Circle extends Base implements Shape {
        private static final long serialVersionUID = 2L;
        double r;
        Circle(double r) { this.r = r; this.tag = "circle"; }
        public double area() { return Math.round(Math.PI * r * r * 100) / 100.0; }
        public String toString() { return tag + "(r=" + r + ")"; }
    }

    static class Rect extends Base implements Shape {
        private static final long serialVersionUID = 3L;
        int w, h;
        transient int cache = 99;
        Rect(int w, int h) { this.w = w; this.h = h; this.tag = "rect"; }
        public double area() { return w * h; }
        public String toString() { return tag + "(" + w + "x" + h + ",cache=" + cache + ")"; }
    }

    static class Logged extends Base implements Shape {
        private static final long serialVersionUID = 4L;
        int side;
        transient String note;
        Logged(int side) { this.side = side; this.tag = "logged"; }
        public double area() { return side * side; }
        private void writeObject(ObjectOutputStream out) throws IOException {
            out.defaultWriteObject();
            out.writeUTF("note-" + side);
        }
        private void readObject(ObjectInputStream in) throws IOException, ClassNotFoundException {
            in.defaultReadObject();
            note = in.readUTF();
        }
        public String toString() { return tag + "(side=" + side + ",note=" + note + ")"; }
    }

    static Shape pick(int i) {
        switch (i % 3) {
            case 0: return new Circle(1.5);
            case 1: return new Rect(2, 3);
            default: return new Logged(4);
        }
    }

    static Object roundTrip(Object o) throws Exception {
        ByteArrayOutputStream bos = new ByteArrayOutputStream();
        try (ObjectOutputStream out = new ObjectOutputStream(bos)) {
            out.writeObject(o);
        }
        try (ObjectInputStream in = new ObjectInputStream(new ByteArrayInputStream(bos.toByteArray()))) {
            return in.readObject();
        }
    }

    static String fieldNames(Class<?> c) {
        List<String> names = new ArrayList<>();
        for (Field f : c.getDeclaredFields()) {
            names.add(f.getName());
        }
        names.sort(null);
        return c.getSimpleName() + names;
    }

    public static void main(String[] args) throws Exception {
        List<Shape> shapes = new ArrayList<>();
        for (int i = 0; i < 3; i++) {
            shapes.add(pick(i + args.length));
        }
        // 序列化：描述符按运行期类（getClass）逐类建，超类字段经 getSuperclass 上溯
        for (Shape s : shapes) {
            Object back = roundTrip(s);
            System.out.println("serial " + back + " area=" + ((Shape) back).area());
        }
        // 字段枚举：接收者 Class 来自接口类型值的 getClass
        for (Shape s : shapes) {
            System.out.println("fields " + fieldNames(s.getClass()) + " super=" + fieldNames(s.getClass().getSuperclass()));
        }
        // 按枚举得到的字段读取：tag 声明在超类，经 getSuperclass 的字段面读出
        for (Shape s : shapes) {
            StringBuilder sb = new StringBuilder();
            for (Field f : s.getClass().getSuperclass().getDeclaredFields()) {
                if (f.getName().equals("tag")) {
                    f.setAccessible(true);
                    sb.append("tag=").append(f.get(s));
                }
            }
            List<String> ints = new ArrayList<>();
            for (Field f : s.getClass().getDeclaredFields()) {
                if (f.getType() == int.class) {
                    f.setAccessible(true);
                    ints.add(f.getName() + "=" + f.getInt(s));
                }
            }
            ints.sort(null);
            System.out.println("read " + sb + " ints=" + ints);
        }
        // lambda 合成类：超类是 Object，无捕获时无字段
        Runnable r = () -> System.out.println("run");
        Class<?> lc = r.getClass();
        System.out.println("lambda super=" + lc.getSuperclass().getName() + " fields=" + lc.getDeclaredFields().length);
        r.run();
        // 数组值的 getClass
        Object arr = new Shape[] { shapes.get(0) };
        System.out.println("array " + arr.getClass().getSimpleName() + " super=" + arr.getClass().getSuperclass().getName()
                + " fields=" + Arrays.toString(arr.getClass().getDeclaredFields()));
    }
}
