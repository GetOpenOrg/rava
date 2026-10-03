import java.io.*;
import java.util.ArrayList;
import java.util.List;

/**
 * 反序列化经序列化构造器创建对象：不运行目标类自身的构造器，只在新分配的实例上运行首个不可序列化超类的
 * 无参构造器。目标含 JDK 类（Integer / Long / ArrayList，首个不可序列化超类为 Object / AbstractList）与用户类
 * （不可序列化的基类无参构造器重设基类字段，可序列化子类的构造器不运行）。
 */
public class TestSerialAllocTargets {
    static int baseInits = 0;
    static int childInits = 0;

    static class Base {
        String origin;
        int baseValue;

        Base() {
            baseInits++;
            origin = "base-ctor";
            baseValue = -1;
        }
    }

    static class Child extends Base implements Serializable {
        private static final long serialVersionUID = 1L;
        String name;
        Integer score;

        Child(String name, Integer score) {
            childInits++;
            this.name = name;
            this.score = score;
            this.origin = "child-ctor";
            this.baseValue = 42;
        }

        @Override
        public String toString() {
            return name + ":" + score + " origin=" + origin + " baseValue=" + baseValue;
        }
    }

    public static void main(String[] args) throws Exception {
        List<Object> items = new ArrayList<>();
        items.add(Integer.valueOf(1234567));
        items.add(Long.valueOf(-9876543210L));
        items.add(new Child("ann", 7));
        List<String> inner = new ArrayList<>();
        inner.add("x");
        inner.add("y");
        items.add(inner);
        System.out.println("before: baseInits=" + baseInits + " childInits=" + childInits);

        ByteArrayOutputStream bos = new ByteArrayOutputStream();
        try (ObjectOutputStream oos = new ObjectOutputStream(bos)) {
            oos.writeObject(items);
        }
        Object back;
        try (ObjectInputStream ois = new ObjectInputStream(new ByteArrayInputStream(bos.toByteArray()))) {
            back = ois.readObject();
        }
        List<?> list = (List<?>) back;
        System.out.println("size=" + list.size() + " type=" + list.getClass().getSimpleName());
        for (Object o : list) {
            System.out.println(o.getClass().getSimpleName() + " -> " + o);
        }
        Integer i = (Integer) list.get(0);
        System.out.println("int+1=" + (i + 1) + " equals=" + i.equals(1234567));
        System.out.println("after: baseInits=" + baseInits + " childInits=" + childInits);
    }
}
