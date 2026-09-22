import java.lang.reflect.Field;
import java.io.InterruptedIOException;

public class FieldDemo {
    public static void main(String[] args) throws Exception {
        // public 实例 int 字段：get/set 直接可达（不经 setAccessible 的私有链）
        InterruptedIOException iioe = new InterruptedIOException();
        Field bf = InterruptedIOException.class.getDeclaredField("bytesTransferred");
        System.out.println("getB=" + bf.get(iioe));
        bf.set(iioe, 512);
        System.out.println("afterSetB=" + iioe.bytesTransferred);
        System.out.println("modsB=" + bf.getModifiers());
    }
}
