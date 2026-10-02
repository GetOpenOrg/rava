import java.nio.ByteBuffer;
import java.nio.charset.CharacterCodingException;
import java.nio.charset.Charset;
import java.nio.charset.CharsetDecoder;
import java.nio.charset.CodingErrorAction;

/**
 * jdk.charsets 扩展字符集：GBK 编解码往返与不可映射输入的三种处置
 * （jmod 覆盖计划 A 档，64_charsets_ext；预审模式：期望先行、进基线等 jmod 第 1 步）。
 */
public class TestCharsetGbk {

    public static void main(String[] args) throws Exception {
        Charset gbk = Charset.forName("GBK");
        byte[] enc = "中文测试".getBytes(gbk);
        StringBuilder hex = new StringBuilder();
        for (byte b : enc) {
            hex.append(String.format("%02X", b));
        }
        System.out.println("enc-len=" + enc.length + " hex=" + hex);
        System.out.println("roundtrip=" + "中文测试".equals(new String(enc, gbk)));
        System.out.println("alias-cp936=" + gbk.aliases().contains("CP936"));

        byte[] bad = { (byte) 0xFF, 0x61 };
        CharsetDecoder rep = gbk.newDecoder()
                .onMalformedInput(CodingErrorAction.REPLACE);
        String s1 = rep.decode(ByteBuffer.wrap(bad)).toString();
        System.out.println("replace-len=" + s1.length() + " head=0x"
                + Integer.toHexString(s1.charAt(0)).toUpperCase());

        CharsetDecoder ign = gbk.newDecoder()
                .onMalformedInput(CodingErrorAction.IGNORE);
        String s2 = ign.decode(ByteBuffer.wrap(bad)).toString();
        System.out.println("ignore=[" + s2 + "]");

        try {
            gbk.newDecoder().onMalformedInput(CodingErrorAction.REPORT)
                    .decode(ByteBuffer.wrap(bad));
        } catch (CharacterCodingException e) {
            System.out.println("report-ex=" + e.getClass().getSimpleName());
        }
    }
}
