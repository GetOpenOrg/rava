import java.io.ByteArrayInputStream;
import java.io.ByteArrayOutputStream;
import java.io.IOException;
import java.io.ObjectInputStream;
import java.io.ObjectOutputStream;
import java.io.Serializable;

/**
 * 边界用例：序列化按名反射回调用户泛型类的私有 writeObject / readObject，沿父类上溯取到的泛型父类
 * writeReplace，以及代理形态的 readResolve（泛型类的反射分派臂挂在擦除视图上）。
 */
public class TestSerialUserGenericCallbacks {
    /** 用户泛型类：私有回调读写额外的校验标记 */
    static class Box<T> implements Serializable {
        private static final long serialVersionUID = 1L;
        private transient T value;
        private transient int marker;

        Box(T value) {
            this.value = value;
        }

        private void writeObject(ObjectOutputStream out) throws IOException {
            out.defaultWriteObject();
            out.writeObject(value);
            out.writeInt(42);
        }

        @SuppressWarnings("unchecked")
        private void readObject(ObjectInputStream in) throws IOException, ClassNotFoundException {
            in.defaultReadObject();
            value = (T) in.readObject();
            marker = in.readInt();
        }

        @Override
        public String toString() {
            return "Box(" + value + ", marker=" + marker + ")";
        }
    }

    /** 两个类型形参的泛型类：回调里交换写出顺序 */
    static final class Pair<A, B> implements Serializable {
        private static final long serialVersionUID = 1L;
        private transient A first;
        private transient B second;

        Pair(A first, B second) {
            this.first = first;
            this.second = second;
        }

        private void writeObject(ObjectOutputStream out) throws IOException {
            out.writeObject(second);
            out.writeObject(first);
        }

        @SuppressWarnings("unchecked")
        private void readObject(ObjectInputStream in) throws IOException, ClassNotFoundException {
            second = (B) in.readObject();
            first = (A) in.readObject();
        }

        @Override
        public String toString() {
            return "Pair(" + first + ", " + second + ")";
        }
    }

    /** 泛型父类声明 writeReplace：子类序列化时经父类链取到 */
    abstract static class Shape<T> implements Serializable {
        private static final long serialVersionUID = 1L;
        final T tag;

        Shape(T tag) {
            this.tag = tag;
        }

        abstract int size();

        protected Object writeReplace() {
            return new Form(getClass().getSimpleName(), String.valueOf(tag), size());
        }
    }

    static final class Square extends Shape<String> {
        private static final long serialVersionUID = 1L;
        final int side;

        Square(String tag, int side) {
            super(tag);
            this.side = side;
        }

        @Override
        int size() {
            return side * side;
        }

        @Override
        public String toString() {
            return "Square(" + String.valueOf(tag) + ", " + side + ")";
        }
    }

    /** 代理形态：反序列化后经 readResolve 还原 */
    static final class Form implements Serializable {
        private static final long serialVersionUID = 1L;
        final String kind;
        final String tag;
        final int area;

        Form(String kind, String tag, int area) {
            this.kind = kind;
            this.tag = tag;
            this.area = area;
        }

        private Object readResolve() {
            int side = (int) Math.round(Math.sqrt(area));
            return new Square(tag + "@" + kind, side);
        }
    }

    static Object roundTrip(Object o) throws IOException, ClassNotFoundException {
        ByteArrayOutputStream bos = new ByteArrayOutputStream();
        try (ObjectOutputStream out = new ObjectOutputStream(bos)) {
            out.writeObject(o);
        }
        try (ObjectInputStream in = new ObjectInputStream(new ByteArrayInputStream(bos.toByteArray()))) {
            return in.readObject();
        }
    }

    public static void main(String[] args) throws Exception {
        System.out.println("box: " + roundTrip(new Box<>("alpha")));
        System.out.println("ibox: " + roundTrip(new Box<>(7)));
        System.out.println("nested: " + roundTrip(new Box<>(new Box<>("inner"))));
        System.out.println("pair: " + roundTrip(new Pair<>("left", 3)));
        Object sq = roundTrip(new Square("red", 3));
        System.out.println("shape: " + sq + " " + sq.getClass().getSimpleName());
        System.out.println("boxed shape: " + roundTrip(new Box<>(new Square("blue", 2))));
    }
}
