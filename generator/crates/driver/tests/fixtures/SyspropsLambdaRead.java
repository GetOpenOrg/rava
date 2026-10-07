import java.security.AccessController;
import java.security.PrivilegedAction;
import java.util.Properties;

/** lambda 返回系统属性表，经 doPrivileged 原路返回后只读：属性表不逃逸 */
public class SyspropsLambdaRead {
    @SuppressWarnings("removal")
    static String read(String key) {
        PrivilegedAction<Properties> act = () -> System.getProperties();
        Properties p = (Properties) AccessController.doPrivileged(act);
        return p == null ? null : p.getProperty(key);
    }

    public static void main(String[] args) {
        System.out.println(read("rava.lambda.read") == null);
    }
}
