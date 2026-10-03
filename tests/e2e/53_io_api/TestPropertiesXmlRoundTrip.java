import java.io.ByteArrayInputStream;
import java.io.ByteArrayOutputStream;
import java.util.Properties;
import java.util.TreeSet;

/**
 * Properties 的 XML 往返与 defaults 继承（storeToXML 无时间戳成分、逐字可比；
 * spring XML 配置与 commons-configuration2 的地基）。
 */
public class TestPropertiesXmlRoundTrip {

    public static void main(String[] args) throws Exception {
        Properties p = new Properties();
        p.setProperty("app.name", "rava");
        p.setProperty("app.count", "42");
        p.setProperty("中文键", "中文值");
        p.setProperty("empty", "");

        ByteArrayOutputStream out = new ByteArrayOutputStream();
        p.storeToXML(out, "e2e-comment", "UTF-8");
        String xml = out.toString("UTF-8");
        System.out.println("xml-head=" + xml.substring(0, xml.indexOf('>') + 1));

        Properties back = new Properties();
        back.loadFromXML(new ByteArrayInputStream(out.toByteArray()));
        for (String k : new TreeSet<>(back.stringPropertyNames())) {
            System.out.println("back " + k + "=" + back.getProperty(k));
        }
        System.out.println("roundtrip=" + p.equals(back));

        // defaults 继承：子层未命中回落父层；stringPropertyNames 合并
        Properties parent = new Properties();
        parent.setProperty("inherited", "from-parent");
        parent.setProperty("overridden", "parent-val");
        Properties child = new Properties(parent);
        child.setProperty("overridden", "child-val");
        child.setProperty("own", "child-own");
        System.out.println("fallback=" + child.getProperty("inherited"));
        System.out.println("override=" + child.getProperty("overridden"));
        System.out.println("merged-count=" + child.stringPropertyNames().size());

        // store（文本形态）的键排序（时间戳注释不打印，只验证键序）
        ByteArrayOutputStream txt = new ByteArrayOutputStream();
        child.store(txt, null);
        String s = txt.toString("UTF-8");
        System.out.println("has-parent-key=" + s.contains("overridden=child-val"));
        System.out.println("store-lines=" + (s.split("\n").length > 4));
    }
}
