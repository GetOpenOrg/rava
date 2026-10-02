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

// 序列化代理形态（同 JUnit Result$SerializedForm）：
// - 私有嵌套代理类声明 private static final serialVersionUID，外层类在静态初始化里
//   ObjectStreamClass.lookup(代理类).getFields() 取 serialPersistentFields（按名读私有静态常量 SUID）
// - writeReplace / readResolve 按名找到的私有 / 继承回调；PutField / GetField 按名读写持久字段
public class TestSerialProxyForm {
    static final class Tally implements Serializable {
        private static final long serialVersionUID = 7L;
        private static final ObjectStreamField[] serialPersistentFields =
                ObjectStreamClass.lookup(Form.class).getFields();

        private int runs;
        private final List<String> failures = new ArrayList<>();

        Tally(int runs, List<String> failures) {
            this.runs = runs;
            this.failures.addAll(failures);
        }

        private Object writeReplace() {
            return new Form(this);
        }

        public String toString() {
            return "Tally(runs=" + runs + ", failures=" + failures + ")";
        }

        private static final class Form implements Serializable {
            private static final long serialVersionUID = 1L;
            private int fRuns;
            private List<String> fFailures;

            Form(Tally t) {
                fRuns = t.runs;
                fFailures = new ArrayList<>(t.failures);
            }

            private void writeObject(ObjectOutputStream s) throws IOException {
                ObjectOutputStream.PutField fields = s.putFields();
                fields.put("fRuns", fRuns);
                fields.put("fFailures", fFailures);
                s.writeFields();
            }

            // 经 readResolve 还原外层对象；readObject 按 GetField 读持久字段（JUnit Result 的形态）
            private Object readResolve() {
                return new Tally(fRuns, fFailures);
            }

            @SuppressWarnings("unchecked")
            private void readObject(ObjectInputStream s) throws IOException, ClassNotFoundException {
                ObjectInputStream.GetField fields = s.readFields();
                fRuns = fields.get("fRuns", 0);
                fFailures = (List<String>) fields.get("fFailures", null);
            }
        }
    }

    // 未声明 SUID、经继承链取得 readResolve 的子类（getInheritableMethod 上溯超类）
    static class Base implements Serializable {
        private static final long serialVersionUID = 3L;
        protected Object readResolve() {
            return "resolved:" + getClass().getSimpleName();
        }
    }

    static class Derived extends Base {
        int v = 9;
    }

    public static void main(String[] args) throws Exception {
        System.out.println("Tally suid=" + ObjectStreamClass.lookup(Tally.class).getSerialVersionUID());
        for (ObjectStreamField f : Tally.serialPersistentFields) {
            System.out.println("field " + f.getName() + " " + f.getTypeCode());
        }
        List<String> fs = new ArrayList<>();
        fs.add("a");
        fs.add("b");
        ByteArrayOutputStream bos = new ByteArrayOutputStream();
        try (ObjectOutputStream oos = new ObjectOutputStream(bos)) {
            oos.writeObject(new Tally(3, fs));
            oos.writeObject(new Derived());
        }
        try (ObjectInputStream ois = new ObjectInputStream(new ByteArrayInputStream(bos.toByteArray()))) {
            System.out.println(ois.readObject());
            System.out.println(ois.readObject());
        }
    }
}
