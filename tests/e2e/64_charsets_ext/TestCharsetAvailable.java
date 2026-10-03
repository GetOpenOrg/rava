import java.nio.charset.Charset;
import java.util.SortedMap;

/**
 * jdk.charsets 提供者注册面：availableCharsets 中扩展字符集的存在性
 * （jmod 覆盖计划 A 档；ExtendedProviderHolder 经 ServiceLoader 跨模块装载——
 * 只打印存在性布尔，不打印全集，集合顺序无关）。
 */
public class TestCharsetAvailable {

    public static void main(String[] args) {
        SortedMap<String, Charset> all = Charset.availableCharsets();
        System.out.println("gbk=" + all.containsKey("GBK"));
        System.out.println("gb18030=" + all.containsKey("GB18030"));
        System.out.println("big5=" + all.containsKey("Big5"));
        System.out.println("shift_jis=" + all.containsKey("Shift_JIS"));
        System.out.println("euc-jp=" + all.containsKey("EUC-JP"));
        System.out.println("euc-kr=" + all.containsKey("EUC-KR"));
        System.out.println("base-utf8=" + all.containsKey("UTF-8"));
    }
}
