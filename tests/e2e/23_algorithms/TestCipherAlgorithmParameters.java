import java.nio.charset.StandardCharsets;
import java.security.AlgorithmParameters;
import javax.crypto.Cipher;
import javax.crypto.spec.IvParameterSpec;
import javax.crypto.spec.SecretKeySpec;

/**
 * gap：JCA 参数对象（java.security.AlgorithmParameters / AlgorithmParametersSpi，纯 Java）。
 * 覆盖：CBC 加密后 Cipher.getParameters → getAlgorithm / getParameterSpec(IvParameterSpec) /
 * getEncoded（DER OCTET STRING）；以参数对象初始化解密（init(mode, key, AlgorithmParameters)）；
 * AlgorithmParameters.getInstance + init(spec) 独立构造并按编码重建（init(byte[])）；ECB 无参数 → null。
 */
public class TestCipherAlgorithmParameters {
    static String hex(byte[] b) {
        StringBuilder sb = new StringBuilder();
        for (byte x : b) sb.append(String.format("%02x", x & 0xff));
        return sb.toString();
    }

    static void roundTrip(String transformation, SecretKeySpec key, byte[] ivBytes, byte[] msg) throws Exception {
        Cipher enc = Cipher.getInstance(transformation);
        enc.init(Cipher.ENCRYPT_MODE, key, new IvParameterSpec(ivBytes));
        byte[] ct = enc.doFinal(msg);
        AlgorithmParameters params = enc.getParameters();
        IvParameterSpec spec = params.getParameterSpec(IvParameterSpec.class);
        System.out.println(transformation + " alg=" + params.getAlgorithm()
                + " provider=" + params.getProvider().getName()
                + " iv=" + hex(spec.getIV()) + " der=" + hex(params.getEncoded()));
        System.out.println("  ct=" + hex(ct));

        Cipher dec = Cipher.getInstance(transformation);
        dec.init(Cipher.DECRYPT_MODE, key, params);
        System.out.println("  plain=" + new String(dec.doFinal(ct), StandardCharsets.UTF_8)
                + " iv=" + hex(dec.getIV()));
    }

    public static void main(String[] args) throws Exception {
        byte[] msg = "Attack at dawn, retreat at dusk".getBytes(StandardCharsets.UTF_8);

        SecretKeySpec des = new SecretKeySpec(new byte[] {1, 35, 69, 103, -119, -85, -51, -17}, "DES");
        roundTrip("DES/CBC/PKCS5Padding", des, new byte[] {9, 8, 7, 6, 5, 4, 3, 2}, msg);

        byte[] k16 = new byte[16];
        byte[] iv16 = new byte[16];
        for (int i = 0; i < 16; i++) {
            k16[i] = (byte) (i * 17 + 3);
            iv16[i] = (byte) (0xF0 - i * 7);
        }
        SecretKeySpec aes = new SecretKeySpec(k16, "AES");
        roundTrip("AES/CBC/PKCS5Padding", aes, iv16, msg);

        // 独立构造参数对象，按 DER 编码重建
        AlgorithmParameters ap = AlgorithmParameters.getInstance("AES");
        ap.init(new IvParameterSpec(iv16));
        byte[] der = ap.getEncoded();
        AlgorithmParameters ap2 = AlgorithmParameters.getInstance("AES");
        ap2.init(der);
        System.out.println("rebuilt iv=" + hex(ap2.getParameterSpec(IvParameterSpec.class).getIV())
                + " same=" + hex(der).equals(hex(ap2.getEncoded())));

        Cipher dec = Cipher.getInstance("AES/CBC/PKCS5Padding");
        dec.init(Cipher.DECRYPT_MODE, aes, ap2);
        Cipher enc = Cipher.getInstance("AES/CBC/PKCS5Padding");
        enc.init(Cipher.ENCRYPT_MODE, aes, ap);
        System.out.println("via params: " + new String(dec.doFinal(enc.doFinal(msg)), StandardCharsets.UTF_8));

        // ECB 无 IV：getParameters 返回 null
        Cipher ecb = Cipher.getInstance("AES/ECB/PKCS5Padding");
        ecb.init(Cipher.ENCRYPT_MODE, aes);
        System.out.println("ecb params=" + ecb.getParameters());
    }
}
