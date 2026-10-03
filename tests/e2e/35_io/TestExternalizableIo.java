import java.io.Externalizable;
import java.io.IOException;
import java.io.ObjectInput;
import java.io.ObjectInputStream;
import java.io.ObjectOutput;
import java.io.ObjectOutputStream;
import java.io.Serializable;
import java.io.ByteArrayInputStream;
import java.io.ByteArrayOutputStream;

/**
 * Externalizable 序列化通道（序列化回调族的最后一块：writeExternal/readExternal
 * 显式布局、public 无参构造要求、与 Serializable 混合字段无关性）。
 */
public class TestExternalizableIo {

    public static class Conf implements Externalizable {
        public String name;
        public int level;

        public Conf() {
        }   // 反序列化要求的 public 无参构造

        public Conf(String name, int level) {
            this.name = name;
            this.level = level;
        }

        @Override
        public void writeExternal(ObjectOutput out) throws IOException {
            out.writeUTF(name);
            out.writeInt(level);
        }

        @Override
        public void readExternal(ObjectInput in) throws IOException {
            name = in.readUTF();
            level = in.readInt();
        }

        @Override
        public String toString() {
            return name + ":" + level;
        }
    }

    public static class Nested implements Serializable {
        Conf conf = new Conf("inner", 7);
        String extra = "e";
    }

    public static void main(String[] args) throws Exception {
        // 纯 Externalizable 往返
        ByteArrayOutputStream bos = new ByteArrayOutputStream();
        try (ObjectOutputStream oos = new ObjectOutputStream(bos)) {
            oos.writeObject(new Conf("rava", 42));
        }
        Object back;
        try (ObjectInputStream ois = new ObjectInputStream(
                new ByteArrayInputStream(bos.toByteArray()))) {
            back = ois.readObject();
        }
        System.out.println("roundtrip=" + back);
        System.out.println("typed=" + (back instanceof Conf));

        // 嵌进 Serializable 图中
        ByteArrayOutputStream bos2 = new ByteArrayOutputStream();
        try (ObjectOutputStream oos = new ObjectOutputStream(bos2)) {
            oos.writeObject(new Nested());
        }
        Nested n2;
        try (ObjectInputStream ois = new ObjectInputStream(
                new ByteArrayInputStream(bos2.toByteArray()))) {
            n2 = (Nested) ois.readObject();
        }
        System.out.println("nested=" + n2.conf + " extra=" + n2.extra);

        // 修改字段值不影响布局协议（显式读写， serialVersionUID 相同即可）
        ByteArrayOutputStream bos3 = new ByteArrayOutputStream();
        try (ObjectOutputStream oos = new ObjectOutputStream(bos3)) {
            oos.writeObject(new Conf("中文", -1));
        }
        try (ObjectInputStream ois = new ObjectInputStream(
                new ByteArrayInputStream(bos3.toByteArray()))) {
            System.out.println("unicode=" + ois.readObject());
        }
    }
}
