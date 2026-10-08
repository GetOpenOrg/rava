import java.nio.charset.Charset;
import java.util.SortedMap;

/**
 * jdk.charsets 扩展提供者的按名取类：x-IBM930 / IBM037 只在扩展提供者（sun.nio.cs.ext.ExtendedCharsets）中，
 * 运行期经 AbstractCharsetProvider.lookup 的 Class.forName + newInstance 反射实例化，类名取自 classMap
 * （构造器经 charset(名, 类名, 别名) 以常量实参写入的实例字段映射）。
 * forName 编解码往返（含别名查找）+ isSupported + availableCharsets 存在性；不打印全集，集合顺序无关。
 */
public class TestCharsetExtLookup {

    static String hex(byte[] b) {
        StringBuilder sb = new StringBuilder();
        for (byte x : b) {
            sb.append(Character.forDigit((x >> 4) & 0xf, 16)).append(Character.forDigit(x & 0xf, 16));
        }
        return sb.toString();
    }

    static void roundTrip(String name, String text) {
        Charset cs = Charset.forName(name);
        byte[] enc = text.getBytes(cs);
        String dec = new String(enc, cs);
        System.out.println(name + " -> " + cs.name() + " bytes=" + hex(enc) + " roundtrip=" + text.equals(dec));
    }

    public static void main(String[] args) {
        roundTrip("x-IBM930", "ABC日本");
        roundTrip("IBM037", "Hello, World!");
        roundTrip("cp037", "EBCDIC 037");
        System.out.println("supported x-IBM930=" + Charset.isSupported("x-IBM930"));
        System.out.println("supported x-IBM939=" + Charset.isSupported("x-IBM939"));
        SortedMap<String, Charset> all = Charset.availableCharsets();
        System.out.println("available x-IBM930=" + all.containsKey("x-IBM930"));
        System.out.println("available IBM037=" + all.containsKey("IBM037"));
        System.out.println("available x-MacRoman=" + all.containsKey("x-MacRoman"));
        Charset mac = all.get("x-MacRoman");
        System.out.println("x-MacRoman via map=" + (mac == null ? "null" : hex("café".getBytes(mac))));
    }
}
