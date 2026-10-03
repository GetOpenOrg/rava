import java.nio.charset.Charset;

/**
 * jdk.charsets CJK 家族字符集往返：GB18030 / Big5 / Shift_JIS / EUC-JP / EUC-KR
 * （jmod 覆盖计划 A 档；表驱动编解码与不可映射字符处置）。
 */
public class TestCharsetCjkFamily {

    public static void main(String[] args) throws Exception {
        String[][] cases = {
            { "GB18030", "中文测试" },
            { "Big5", "繁體中文測試" },
            { "Shift_JIS", "日本語テスト" },
            { "EUC-JP", "日本語テスト" },
            { "EUC-KR", "한국어 시험" },
        };
        for (String[] c : cases) {
            Charset cs = Charset.forName(c[0]);
            byte[] b = c[1].getBytes(cs);
            String back = new String(b, cs);
            System.out.println(c[0] + " bytes=" + b.length + " roundtrip=" + c[1].equals(back));
        }

        // GB18030 四字节区：欧元符号
        byte[] euro = "€".getBytes(Charset.forName("GB18030"));
        System.out.println("gb18030-euro-bytes=" + euro.length);
        System.out.println("gb18030-euro-back=[" + new String(euro, "GB18030") + "]");
    }
}
