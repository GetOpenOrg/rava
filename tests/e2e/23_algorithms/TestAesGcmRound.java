import java.nio.charset.StandardCharsets;

import javax.crypto.Cipher;
import javax.crypto.spec.GCMParameterSpec;
import javax.crypto.spec.SecretKeySpec;

/**
 * AES/GCM 认证加密往返（H2 加密存储 / commons-compress 加密 zip / JWT AES 通道的地基，
 * GCMParameterSpec 此前零覆盖）：固定 key/iv 的密文逐字可比、tag 篡改边界
 * AEADBadTagException、ECB 模式对照。
 */
public class TestAesGcmRound {

    static String hex(byte[] b) {
        StringBuilder sb = new StringBuilder();
        for (byte x : b) {
            sb.append(String.format("%02x", x));
        }
        return sb.toString();
    }

    public static void main(String[] args) throws Exception {
        byte[] key = "0123456789abcdef".getBytes(StandardCharsets.UTF_8);   // 128-bit
        byte[] iv = { 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11 };
        byte[] plain = "gcm-e2e-payload".getBytes(StandardCharsets.UTF_8);

        Cipher enc = Cipher.getInstance("AES/GCM/NoPadding");
        enc.init(Cipher.ENCRYPT_MODE, new SecretKeySpec(key, "AES"),
                new GCMParameterSpec(128, iv));
        byte[] ct = enc.doFinal(plain);
        System.out.println("ct-len=" + ct.length);             // 明文长 + 16 字节 tag
        System.out.println("ct=" + hex(ct));

        Cipher dec = Cipher.getInstance("AES/GCM/NoPadding");
        dec.init(Cipher.DECRYPT_MODE, new SecretKeySpec(key, "AES"),
                new GCMParameterSpec(128, iv));
        System.out.println("roundtrip=" + new String(dec.doFinal(ct), StandardCharsets.UTF_8));

        // tag 篡改 → AEADBadTagException
        byte[] tampered = ct.clone();
        tampered[tampered.length - 1] ^= 0x01;
        try {
            dec.doFinal(tampered);
            System.out.println("tampered=unexpected");
        } catch (Exception e) {
            System.out.println("tampered-ex=" + e.getClass().getSimpleName());
        }

        // 密文段篡改同样被认证拒绝
        byte[] midTweak = ct.clone();
        midTweak[0] ^= 0x80;
        try {
            dec.doFinal(midTweak);
        } catch (Exception e) {
            System.out.println("midtweak-ex=" + e.getClass().getSimpleName());
        }

        // AAD：附加认证数据（不进密文，参与认证）
        Cipher aadEnc = Cipher.getInstance("AES/GCM/NoPadding");
        aadEnc.init(Cipher.ENCRYPT_MODE, new SecretKeySpec(key, "AES"),
                new GCMParameterSpec(128, iv));
        aadEnc.updateAAD("header".getBytes(StandardCharsets.UTF_8));
        byte[] aadCt = aadEnc.doFinal(plain);

        Cipher aadDec = Cipher.getInstance("AES/GCM/NoPadding");
        aadDec.init(Cipher.DECRYPT_MODE, new SecretKeySpec(key, "AES"),
                new GCMParameterSpec(128, iv));
        aadDec.updateAAD("header".getBytes(StandardCharsets.UTF_8));
        System.out.println("aad-roundtrip=" + new String(aadDec.doFinal(aadCt), StandardCharsets.UTF_8));
        Cipher noAad = Cipher.getInstance("AES/GCM/NoPadding");
        noAad.init(Cipher.DECRYPT_MODE, new SecretKeySpec(key, "AES"),
                new GCMParameterSpec(128, iv));
        try {
            noAad.doFinal(aadCt);
        } catch (Exception e) {
            System.out.println("aad-miss-ex=" + e.getClass().getSimpleName());
        }

        // ECB 对照（无 IV 无认证）
        Cipher ecb = Cipher.getInstance("AES/ECB/PKCS5Padding");
        ecb.init(Cipher.ENCRYPT_MODE, new SecretKeySpec(key, "AES"));
        byte[] ecbCt = ecb.doFinal(plain);
        Cipher ecbDec = Cipher.getInstance("AES/ECB/PKCS5Padding");
        ecbDec.init(Cipher.DECRYPT_MODE, new SecretKeySpec(key, "AES"));
        System.out.println("ecb-roundtrip=" + new String(ecbDec.doFinal(ecbCt), StandardCharsets.UTF_8));
    }
}
