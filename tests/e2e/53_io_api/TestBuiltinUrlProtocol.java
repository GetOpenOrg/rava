import java.net.MalformedURLException;
import java.net.URL;

// 内建协议名常量上的分支折叠边界：URL.isOverrideable 的逐字符比较、DefaultFactory 的 hashCode 分派、
// 非 ASCII 字符的小写映射与越界 charAt 不得被错误折叠
public class TestBuiltinUrlProtocol {
    static String kind(String p) {
        switch (p) {
            case "file": return "F";
            case "jar": return "J";
            case "jrt": return "R";
            default: return "?";
        }
    }

    static void make(String protocol, String file) {
        try {
            URL u = new URL(protocol, "", file);
            System.out.println(protocol + " -> " + u.getProtocol() + " " + u);
        } catch (MalformedURLException e) {
            System.out.println(protocol + " -> MalformedURLException: " + e.getMessage());
        }
    }

    public static void main(String[] args) {
        make("file", "/tmp/a.txt");
        make("FILE", "/tmp/b.txt");
        make("jar", "file:/tmp/c.jar!/d");
        make("jrt", "/java.base");
        make("nosuchproto", "/x");
        System.out.println(kind("file") + kind("jar") + kind("jrt") + kind("http"));
        System.out.println("file".hashCode() + " " + "".hashCode() + " " + "Aa".hashCode() + " " + "BB".hashCode());
        System.out.println((int) Character.toLowerCase('F') + " " + (int) Character.toLowerCase('É') + " " + (int) Character.toLowerCase('İ'));
        System.out.println("jér".charAt(1) == 'é');
        try {
            System.out.println("ab".charAt(2));
        } catch (StringIndexOutOfBoundsException e) {
            System.out.println("SIOOBE " + e.getMessage());
        }
    }
}
