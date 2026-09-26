// FS-H6：Properties.getProperty 改走字节码翻译后的 defaults 链与非 String 值语义。
import java.util.Properties;

public class TestPropertiesDefaults {
    public static void main(String[] args) {
        Properties base = new Properties();
        base.setProperty("color", "blue");
        base.setProperty("size", "M");
        Properties mid = new Properties(base);
        mid.setProperty("size", "L");
        Properties top = new Properties(mid);
        top.put("count", Integer.valueOf(3));        // 非 String 值：getProperty 视为缺席
        top.setProperty("name", "top");

        String[] keys = {"color", "size", "name", "count", "missing"};
        for (String k : keys) {
            System.out.println(k + " top=" + top.getProperty(k) + " mid=" + mid.getProperty(k)
                + " dflt=" + top.getProperty(k, "none") + " containsKey=" + top.containsKey(k));
        }
        mid.remove("size");
        System.out.println("after remove size=" + top.getProperty("size"));
        base.setProperty("count", "7");
        System.out.println("count via defaults=" + top.getProperty("count") + " raw=" + top.get("count"));
        System.out.println("names=" + new java.util.TreeSet<>(top.stringPropertyNames()));
        System.out.println("sys file.separator=" + System.getProperty("file.separator")
            + " absent=" + System.getProperty("no.such.prop") + " dflt=" + System.getProperty("no.such.prop", "d"));
    }
}
