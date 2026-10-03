import java.io.ByteArrayInputStream;
import java.io.ByteArrayOutputStream;
import java.io.IOException;
import java.io.ObjectInputStream;
import java.io.ObjectOutputStream;
import java.io.ObjectStreamClass;
import java.io.ObjectStreamField;
import java.io.Serializable;
import java.util.ArrayList;
import java.util.List;

// 序列化的字段枚举（ObjectStreamClass.getDefaultSerialFields / computeDefaultSUID）只用字段的名字、修饰符与
// 非 static 字段的偏移，枚举本身不初始化类；默认 SUID 计算经 native hasStaticInitializer 查 <clinit>，HotSpot 查找时
// 初始化该可序列化类（JNI GetStaticMethodID），所以写出类对象会跑 Lazy 的静态初始化器，而只取描述符、枚举接口
// Shape 的字段不初始化 Shape（接口常量 ORIGIN 在首次读取时才求值）。另覆盖多种可序列化字段、
// serialPersistentFields 自定义、transient / static 字段，以及不可序列化类的 lookup / 写出
public class TestSerialEnumNoInit {
    static final StringBuilder LOG = new StringBuilder();

    static void log(String s) {
        LOG.append(s).append(';');
    }

    // 只经类字面量写出、取描述符：静态初始化器有可观察的副作用，其运行时机须与 JDK 一致
    static class Lazy implements Serializable {
        static final List<String> NAMES = initNames();
        static int counter = 3;
        String tag = "lazy";

        static List<String> initNames() {
            log("Lazy.<clinit>");
            List<String> l = new ArrayList<>();
            l.add("a");
            return l;
        }
    }

    interface Shape extends Serializable {
        Object ORIGIN = Origin.make("shape");
    }

    static class Origin implements Serializable {
        final String who;

        Origin(String who) { this.who = who; }

        static Origin make(String who) {
            log("Origin.make(" + who + ")");
            return new Origin(who);
        }
    }

    // 未声明 serialVersionUID：写出时按默认算法计算 SUID（枚举全部声明字段，含 static / transient）
    static class Mixed implements Shape {
        static int instances;
        int i = 7;
        long l = 1L << 40;
        double d = 2.5;
        char c = 'x';
        boolean b = true;
        String s = "str";
        int[] arr = {1, 2, 3};
        Integer boxed = 42;
        List<String> list = new ArrayList<>();
        Origin origin = new Origin("mixed");
        transient String skipped = "transient";

        Mixed() {
            instances++;
            list.add("one");
            list.add("two");
        }

        public String toString() {
            return "Mixed(" + i + "," + l + "," + d + "," + c + "," + b + "," + s + "," + arr.length + "," + arr[2] + ","
                    + boxed + "," + list + "," + origin.who + "," + skipped + ")";
        }
    }

    // serialPersistentFields 自定义：只序列化 name 与 score（score 经 PutField 写为 long），count 不进流
    static class Custom implements Serializable {
        private static final long serialVersionUID = 11L;
        private static final ObjectStreamField[] serialPersistentFields = {
            new ObjectStreamField("name", String.class),
            new ObjectStreamField("score", long.class),
        };
        String name;
        int score;
        int count;

        Custom(String name, int score, int count) {
            this.name = name;
            this.score = score;
            this.count = count;
        }

        private void writeObject(ObjectOutputStream out) throws IOException {
            ObjectOutputStream.PutField pf = out.putFields();
            pf.put("name", name);
            pf.put("score", (long) score * 10);
            out.writeFields();
        }

        private void readObject(ObjectInputStream in) throws IOException, ClassNotFoundException {
            ObjectInputStream.GetField gf = in.readFields();
            name = (String) gf.get("name", "?");
            score = (int) gf.get("score", -1L);
            count = -1;
        }

        public String toString() {
            return "Custom(" + name + "," + score + "," + count + ")";
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

    public static void main(String[] args) throws Exception {
        // 1. 写出类对象：描述符含默认 SUID（computeDefaultSUID → hasStaticInitializer 初始化 Lazy）与字段表
        Object back = roundTrip(Lazy.class);
        System.out.println("class back: " + (back == Lazy.class) + " log=[" + LOG + "]");

        // 2. 取描述符与字段表；接口 Shape 的描述符不初始化 Shape
        ObjectStreamClass lazyDesc = ObjectStreamClass.lookup(Lazy.class);
        StringBuilder names = new StringBuilder();
        for (ObjectStreamField f : lazyDesc.getFields()) {
            names.append(f.getName()).append(':').append(f.getTypeCode()).append(' ');
        }
        System.out.println("lazy fields: " + names.toString().trim() + " suid0=" + (lazyDesc.getSerialVersionUID() != 0)
                + " log=[" + LOG + "]");
        ObjectStreamClass shapeDesc = ObjectStreamClass.lookup(Shape.class);
        System.out.println("shape desc: " + (shapeDesc == null) + " log=[" + LOG + "]");

        // 3. 主动使用：已初始化，静态初始化器不重跑
        System.out.println("lazy names: " + Lazy.NAMES + " counter=" + Lazy.counter + " log=[" + LOG + "]");

        // 4. 多种可序列化字段（含 transient、static、接口常量）往返
        Mixed m = new Mixed();
        Mixed m2 = (Mixed) roundTrip(m);
        System.out.println("mixed: " + m2 + " instances=" + Mixed.instances);
        System.out.println("mixed fields: " + ObjectStreamClass.lookup(Mixed.class).getFields().length
                + " suid0=" + (ObjectStreamClass.lookup(Mixed.class).getSerialVersionUID() != 0));
        System.out.println("shape origin: " + ((Origin) Shape.ORIGIN).who + " log=[" + LOG + "]");

        // 5. serialPersistentFields 自定义
        Custom c = (Custom) roundTrip(new Custom("bob", 9, 4));
        System.out.println("custom: " + c);
        StringBuilder cf = new StringBuilder();
        for (ObjectStreamField f : ObjectStreamClass.lookup(Custom.class).getFields()) {
            cf.append(f.getName()).append(':').append(f.getTypeCode()).append(' ');
        }
        System.out.println("custom fields: " + cf.toString().trim() + " suid=" + ObjectStreamClass.lookup(Custom.class).getSerialVersionUID());

        // 6. 不可序列化的类：lookup 返回 null，写出抛 NotSerializableException
        System.out.println("thread desc: " + ObjectStreamClass.lookup(Thread.class));
        try {
            roundTrip(new Object());
            System.out.println("no exception");
        } catch (java.io.NotSerializableException e) {
            System.out.println("not serializable: " + e.getMessage());
        }
    }
}
