import java.io.UnsupportedEncodingException;
import java.net.URLDecoder;
import java.net.URLEncoder;
import java.nio.charset.StandardCharsets;

/**
 * URLEncoder / URLDecoder 全形态：表单编码（空格→+、保留字百分号）、UTF-8 中文、
 * 非法序列异常（servlet 表单解析与 Retrofit 表单编码的地基；现存 UrlEncoderExample
 * 实为 Base64，此面此前零覆盖）。
 */
public class TestUrlEncoderDecoder {

    public static void main(String[] args) throws Exception {
        String raw = "a b+c&=中";
        String enc = URLEncoder.encode(raw, StandardCharsets.UTF_8);
        System.out.println("enc=" + enc);
        System.out.println("roundtrip=" + raw.equals(URLDecoder.decode(enc, StandardCharsets.UTF_8)));

        // 指定字符串名形态
        String enc2 = URLEncoder.encode("梦", "UTF-8");
        System.out.println("enc-named=" + enc2);
        System.out.println("dec-named=" + URLDecoder.decode(enc2, "UTF-8"));

        // decode 的 + → 空格（表单语义，与 URI 解码不同）
        System.out.println("plus=" + URLDecoder.decode("a+b", "UTF-8"));

        // 逐保留字形态
        System.out.println("amp=" + URLEncoder.encode("&", "UTF-8")
                + " eq=" + URLEncoder.encode("=", "UTF-8")
                + " pct=" + URLEncoder.encode("%", "UTF-8"));
        // 字母数字与 .*-_ 保留不转
        System.out.println("keep=" + URLEncoder.encode("aZ09.*-_", "UTF-8"));

        // 非法序列
        try {
            URLDecoder.decode("%4", "UTF-8");
        } catch (IllegalArgumentException e) {
            System.out.println("trunc-ex=" + e.getClass().getSimpleName());
        }
        try {
            URLDecoder.decode("%ZZ", "UTF-8");
        } catch (IllegalArgumentException e) {
            System.out.println("badhex-ex=" + e.getClass().getSimpleName());
        }
        try {
            URLEncoder.encode("x", "NO-SUCH-CHARSET");
        } catch (UnsupportedEncodingException e) {
            System.out.println("charset-ex=" + e.getClass().getSimpleName());
        }
    }
}
