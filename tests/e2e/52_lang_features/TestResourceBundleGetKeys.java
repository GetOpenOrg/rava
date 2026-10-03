import java.io.StringReader;
import java.util.Enumeration;
import java.util.ListResourceBundle;
import java.util.PropertyResourceBundle;
import java.util.ResourceBundle;

/**
 * ResourceBundle.getKeys 枚举通道（方法级实测：6 jar——keySet 已有覆盖、
 * 枚举形态零命中，此前零覆盖）：Property/List 两形态、parent 链合并枚举。
 */
public class TestResourceBundleGetKeys {

    static class Parent extends ListResourceBundle {
        @Override
        protected Object[][] getContents() {
            return new Object[][] {
                { "p.only", "P" },
                { "shared", "from-parent" },
            };
        }
    }

    static class Child extends ListResourceBundle {
        Child(ResourceBundle parent) {
            setParent(parent);   // protected 仅限自身实例（继承链内）调用
        }

        @Override
        protected Object[][] getContents() {
            return new Object[][] {
                { "c.only", "C" },
                { "shared", "from-child" },
            };
        }
    }

    static String enumSorted(ResourceBundle b) {
        java.util.TreeSet<String> s = new java.util.TreeSet<>();
        for (Enumeration<String> e = b.getKeys(); e.hasMoreElements();) {
            s.add(e.nextElement());
        }
        return String.join(",", s);
    }

    public static void main(String[] args) throws Exception {
        PropertyResourceBundle prb = new PropertyResourceBundle(new StringReader("a=1\nb=2\n"));
        System.out.println("props-keys=" + enumSorted(prb));

        // getKeys 与 keySet 一致
        System.out.println("keys-eq-keyset=" + prb.keySet().containsAll(
                java.util.Collections.list(prb.getKeys())));

        // parent 链：子键 + 父键合并去重
        Child child = new Child(new Parent());
        System.out.println("merged=" + enumSorted(child));
        System.out.println("override-wins=" + child.getString("shared"));
    }
}
