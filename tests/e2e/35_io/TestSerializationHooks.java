import java.io.ByteArrayInputStream;
import java.io.ByteArrayOutputStream;
import java.io.IOException;
import java.io.ObjectInputStream;
import java.io.ObjectOutputStream;
import java.io.ObjectStreamClass;
import java.io.Serializable;
import java.util.ArrayList;
import java.util.List;

/**
 * 序列化定制挂钩：ObjectInputStream.resolveClass 重写（方法级实测：6 jar，
 * 各序列化框架的类解析定制点，此前零覆盖）——解析名记录、等价返回、
 * 未知类 → ClassNotFoundException 通道。
 */
public class TestSerializationHooks {

    static class Payload implements Serializable {
        String name = "p";
        int level = 3;
    }

    static class Recording extends ObjectInputStream {
        final List<String> resolved = new ArrayList<>();

        Recording(byte[] data) throws IOException {
            super(new ByteArrayInputStream(data));
        }

        @Override
        protected Class<?> resolveClass(ObjectStreamClass desc)
                throws IOException, ClassNotFoundException {
            resolved.add(desc.getName());
            return super.resolveClass(desc);
        }
    }

    public static void main(String[] args) throws Exception {
        ByteArrayOutputStream bos = new ByteArrayOutputStream();
        try (ObjectOutputStream oos = new ObjectOutputStream(bos)) {
            oos.writeObject(new Payload());
        }
        byte[] bytes = bos.toByteArray();

        // 重写点被调到，且解析出的类与常规通道等价
        Recording rec = new Recording(bytes);
        Object back = rec.readObject();
        System.out.println("resolved-contains=" + rec.resolved.contains("TestSerializationHooks$Payload"));
        System.out.println("same-class=" + (back.getClass() == Payload.class));
        System.out.println("fields=" + ((Payload) back).name + "/" + ((Payload) back).level);

        // 未知类名 → ClassNotFoundException（resolveClass 的异常通道）
        Recording ghost = new Recording(bytes) {
            @Override
            protected Class<?> resolveClass(ObjectStreamClass desc) throws ClassNotFoundException {
                if (desc.getName().endsWith("Payload")) {
                    throw new ClassNotFoundException("blocked:" + desc.getName());
                }
                return super.resolveClass(desc);
            }
        };
        try {
            ghost.readObject();
        } catch (ClassNotFoundException e) {
            System.out.println("blocked-ex=" + e.getClass().getSimpleName()
                    + " msg-head=" + e.getMessage().startsWith("blocked"));
        }

        // 替身解析：把 Payload 解析为另一个 serialVersionUID 兼容类不可行（类身份校验），
        // 但解析到自身合法；此处验证 resolveClass 返回值直接生效
        Recording alias = new Recording(bytes) {
            @Override
            protected Class<?> resolveClass(ObjectStreamClass desc) throws ClassNotFoundException {
                if (desc.getName().endsWith("Payload")) {
                    return Payload.class;   // 显式重定向到同一类
                }
                return super.resolveClass(desc);
            }
        };
        System.out.println("alias-ok=" + (alias.readObject() instanceof Payload));
    }
}
