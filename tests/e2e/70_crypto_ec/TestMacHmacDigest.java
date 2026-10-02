import javax.crypto.Mac;
import javax.crypto.spec.SecretKeySpec;

/**
 * Mac HmacSHA256/HmacMD5 固定密钥摘要（commons-codec HmacUtils 与 JWT 类库的地基，
 * 此前零覆盖；确定性输出逐字节可比）。
 */
public class TestMacHmacDigest {

    static String hex(byte[] b) {
        StringBuilder sb = new StringBuilder();
        for (byte x : b) {
            sb.append(String.format("%02x", x));
        }
        return sb.toString();
    }

    static Mac mac(String algo, String key) throws Exception {
        Mac m = Mac.getInstance(algo);
        m.init(new SecretKeySpec(key.getBytes("UTF-8"), algo));
        return m;
    }

    public static void main(String[] args) throws Exception {
        Mac sha = mac("HmacSHA256", "rava-secret");
        System.out.println("hmac-sha256=" + hex(sha.doFinal("payload".getBytes("UTF-8"))));

        Mac md5 = mac("HmacMD5", "rava-secret");
        System.out.println("hmac-md5=" + hex(md5.doFinal("payload".getBytes("UTF-8"))));

        // 分段 update 等价一次性 doFinal
        Mac split = mac("HmacSHA256", "rava-secret");
        split.update("pay".getBytes("UTF-8"));
        split.update("load".getBytes("UTF-8"));
        System.out.println("split-eq=" + hex(split.doFinal()).equals(
                hex(mac("HmacSHA256", "rava-secret").doFinal("payload".getBytes("UTF-8")))));

        // 同消息同密钥 → 确定性
        System.out.println("deterministic=" + hex(mac("HmacSHA256", "k").doFinal("m".getBytes()))
                .equals(hex(mac("HmacSHA256", "k").doFinal("m".getBytes()))));

        // 换密钥 → 不同
        System.out.println("key-sensitive=" + !hex(mac("HmacSHA256", "k1").doFinal("m".getBytes()))
                .equals(hex(mac("HmacSHA256", "k2").doFinal("m".getBytes()))));

        // getMacLength/getAlgorithm
        System.out.println("len=" + mac("HmacSHA256", "k").getMacLength()
                + " algo=" + mac("HmacSHA256", "k").getAlgorithm());
    }
}
