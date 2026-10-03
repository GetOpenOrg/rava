import java.io.StringReader;
import java.util.ListResourceBundle;
import java.util.MissingResourceException;
import java.util.PropertyResourceBundle;
import java.util.ResourceBundle;

/**
 * ResourceBundle 资源束族（方法级实测：getBundle/getString 各 16 jar 引用，
 * spring message / logback bundle 的地基，此前零覆盖）：PropertyResourceBundle
 * 构造、getString/getObject/keySet、未命中 MissingResourceException、parent 链
 * 回落、类路径无束的 getBundle 边界。
 */
public class TestResourceBundleFaces {

    static class FixedBundle extends ListResourceBundle {
        @Override
        protected Object[][] getContents() {
            return new Object[][] {
                { "app.name", "rava" },
                { "app.desc", "fallback-desc" },
                { "app.count", 42 },
            };
        }
    }

    static class ChildBundle extends ListResourceBundle {
        ChildBundle(ResourceBundle parent) {
            setParent(parent);   // protected 只能在自身实例（继承链内）调用
        }

        @Override
        protected Object[][] getContents() {
            return new Object[][] {
                { "app.name", "child-override" },
            };
        }
    }

    public static void main(String[] args) throws Exception {
        PropertyResourceBundle prb = new PropertyResourceBundle(new StringReader(
                "greet=hello\nzh=你好\n"));
        System.out.println("keys=" + prb.keySet().size());
        System.out.println("greet=" + prb.getString("greet"));
        System.out.println("zh=" + prb.getString("zh"));
        System.out.println("contains=" + prb.containsKey("greet") + prb.containsKey("nope"));

        try {
            prb.getString("missing");
        } catch (MissingResourceException e) {
            System.out.println("miss-ex=" + e.getClass().getSimpleName()
                    + " key=[" + e.getKey() + "]");
        }

        // getObject 取非字符串值
        ListResourceBundle fixed = new FixedBundle();
        System.out.println("object-type=" + fixed.getObject("app.count").getClass().getSimpleName());
        try {
            fixed.getString("app.count");   // 非字符串按 getString 取 → ClassCastException
        } catch (ClassCastException e) {
            System.out.println("cast-ex=" + e.getClass().getSimpleName());
        }

        // parent 链：子未命中回落父、子覆盖优先（字符串键走 getString，非字符串走 getObject）
        ChildBundle child = new ChildBundle(fixed);
        System.out.println("override=" + child.getString("app.name"));
        System.out.println("fallback=" + child.getString("app.desc"));
        System.out.println("fallback-type=" + child.getObject("app.count").getClass().getSimpleName());
        System.out.println("merged-keys=" + child.keySet().size());

        // 类路径无束 → MissingResourceException（getBundle 的空路径边界）
        try {
            ResourceBundle.getBundle("no.such.bundle.name");
        } catch (MissingResourceException e) {
            System.out.println("bundle-ex=" + e.getClass().getSimpleName());
        }
    }
}
