import java.nio.file.Files;
import java.nio.file.Path;
import java.util.Map;

/**
 * 类自有方法 `m(String[])` 与接口继承的抽象 `m()` 同名：类内 `this.m()` 必须解析到
 * 无参的接口成员（经子类实现分派），不得落到自有的一参方法。覆盖直接接口与超接口传递两种形态，
 * 以及 JDK 中的同构实例（AbstractBasicFileAttributeView.readAttributes，经 Files.getAttribute 触发）。
 */
public class TestOwnOverloadVsInterfaceAbstract {
    interface View {
        String read();
    }

    interface NamedView extends View {
        String name();
    }

    // 直接实现接口：自有 read(String[]) 与接口抽象 read() 同名
    abstract static class AbstractView implements View {
        String read(String[] keys) {
            StringBuilder sb = new StringBuilder(read());
            for (String k : keys) {
                sb.append('+').append(k);
            }
            return sb.toString();
        }
    }

    static final class PlainView extends AbstractView {
        public String read() {
            return "plain";
        }
    }

    // 经超接口传递：read() 声明在 NamedView 的超接口 View 上
    abstract static class AbstractNamedView implements NamedView {
        String read(String[] keys) {
            return name() + ":" + read() + "#" + keys.length;
        }
    }

    static final class FancyView extends AbstractNamedView {
        public String read() {
            return "fancy";
        }

        public String name() {
            return "fv";
        }
    }

    public static void main(String[] args) throws Exception {
        AbstractView a = new PlainView();
        System.out.println(a.read(new String[] {"x", "y"}));
        System.out.println(a.read());
        View v = a;
        System.out.println(v.read());

        AbstractNamedView b = new FancyView();
        System.out.println(b.read(new String[] {"k"}));
        NamedView nv = b;
        System.out.println(nv.read() + "/" + nv.name());

        Path p = Files.createTempFile("rava-ovl", ".txt");
        try {
            Files.writeString(p, "hello");
            Object size = Files.getAttribute(p, "basic:size");
            System.out.println("size=" + size);
            Map<String, Object> attrs = Files.readAttributes(p, "basic:size,isRegularFile");
            System.out.println("attrs.size=" + attrs.get("size") + ",regular=" + attrs.get("isRegularFile"));
            Object nlink = Files.getAttribute(p, "unix:nlink");
            System.out.println("nlink=" + nlink);
        } finally {
            Files.delete(p);
        }
    }
}
