import java.io.*;
import java.lang.reflect.Method;
import java.util.ArrayList;
import java.util.List;

/**
 * 按名查方法的包装方法与调用点配对：查找类与名字都经包装方法形参传入，各调用点的（类, 名字）
 * 组合各自解析；序列化回调（ArrayList.writeObject / readObject 与用户类的私有回调）经同一种
 * 「类形参 × 名字形参」的辅助方法查找。transient 字段不经默认序列化写出。
 */
public class TestSerialLookupPairing {
    static class Account implements Serializable {
        private static final long serialVersionUID = 1L;
        String owner;
        int balance;
        transient String cache = "warm";

        Account(String owner, int balance) {
            this.owner = owner;
            this.balance = balance;
        }

        private void writeObject(ObjectOutputStream out) throws IOException {
            out.defaultWriteObject();
            out.writeUTF("tag:" + owner);
        }

        private void readObject(ObjectInputStream in) throws IOException, ClassNotFoundException {
            in.defaultReadObject();
            cache = in.readUTF();
        }

        @Override
        public String toString() {
            return owner + "=" + balance + "(" + cache + ")";
        }
    }

    static class Greeter {
        private String hello(String who) {
            return "hello " + who;
        }

        private String bye(String who) {
            return "bye " + who;
        }
    }

    static class Counter {
        private int count;

        private void bump(Integer by) {
            count += by;
        }

        private int total() {
            return count;
        }
    }

    // 包装方法：查找类与名字都来自形参
    static Method find(Class<?> cl, String name, Class<?>... params) throws NoSuchMethodException {
        Method m = cl.getDeclaredMethod(name, params);
        m.setAccessible(true);
        return m;
    }

    // 再包一层：类与名字仍来自形参（逐层上推）
    static Object call(Class<?> cl, String name, Object target, Class<?> pt, Object arg) throws Exception {
        Method m = pt == null ? find(cl, name) : find(cl, name, pt);
        return pt == null ? m.invoke(target) : m.invoke(target, arg);
    }

    public static void main(String[] args) throws Exception {
        Greeter g = new Greeter();
        System.out.println(call(Greeter.class, "hello", g, String.class, "rava"));
        System.out.println(call(Greeter.class, "bye", g, String.class, "jvm"));
        Counter c = new Counter();
        call(Counter.class, "bump", c, Integer.class, 5);
        call(Counter.class, "bump", c, Integer.class, 7);
        System.out.println("total=" + call(Counter.class, "total", c, null, null));

        List<Account> accounts = new ArrayList<>();
        accounts.add(new Account("alice", 10));
        accounts.add(new Account("bob", 20));
        ByteArrayOutputStream bos = new ByteArrayOutputStream();
        try (ObjectOutputStream oos = new ObjectOutputStream(bos)) {
            oos.writeObject(accounts);
        }
        System.out.println("bytes>0=" + (bos.size() > 0));
        try (ObjectInputStream ois = new ObjectInputStream(new ByteArrayInputStream(bos.toByteArray()))) {
            @SuppressWarnings("unchecked")
            List<Account> back = (List<Account>) ois.readObject();
            System.out.println(back.getClass().getSimpleName() + " " + back.size());
            for (Account a : back) {
                System.out.println(a);
            }
        }
    }
}
