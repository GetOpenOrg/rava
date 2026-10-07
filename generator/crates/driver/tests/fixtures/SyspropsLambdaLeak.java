import java.security.AccessController;
import java.security.PrivilegedAction;
import java.util.Properties;

/** lambda 返回系统属性表，经 doPrivileged 返回后存入静态字段：属性表逃逸 */
public class SyspropsLambdaLeak {
    static Properties leaked;

    @SuppressWarnings("removal")
    static void grab() {
        PrivilegedAction<Properties> act = () -> System.getProperties();
        leaked = (Properties) AccessController.doPrivileged(act);
    }

    public static void main(String[] args) {
        grab();
        System.out.println(leaked != null);
    }
}
