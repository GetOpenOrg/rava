import java.nio.charset.StandardCharsets;
import java.security.NoSuchAlgorithmException;
import javax.crypto.BadPaddingException;
import javax.crypto.Cipher;
import javax.crypto.IllegalBlockSizeException;
import javax.crypto.spec.IvParameterSpec;
import javax.crypto.spec.SecretKeySpec;

/**
 * K-JCA：Cipher 服务查找（transformation 候选 → ServiceId → 服务表）+ 翻译字节码的 DES 实现
 * （com.sun.crypto.provider.DESCipher / CipherCore / DESCrypt）。覆盖：缺省 transformation 与
 * 显式 ECB/PKCS5Padding 等价、CBC + 显式 IV 往返与 getIV、ECB/NoPadding 整块、NoPadding 非整块
 * → IllegalBlockSizeException、错误密钥解密 → BadPaddingException、update + doFinal 分段、
 * getAlgorithm / getBlockSize / getProvider、未知 transformation → NoSuchAlgorithmException。
 */
public class TestCipherDesModes {
    static String hex(byte[] b) {
        StringBuilder sb = new StringBuilder();
        for (byte x : b) sb.append(String.format("%02x", x & 0xff));
        return sb.toString();
    }

    public static void main(String[] args) throws Exception {
        SecretKeySpec key = new SecretKeySpec(new byte[] {1, 35, 69, 103, -119, -85, -51, -17}, "DES");
        SecretKeySpec wrong = new SecretKeySpec(new byte[] {8, 7, 6, 5, 4, 3, 2, 1}, "DES");
        byte[] msg = "The quick brown fox".getBytes(StandardCharsets.UTF_8);

        Cipher dflt = Cipher.getInstance("DES");
        dflt.init(Cipher.ENCRYPT_MODE, key);
        byte[] c1 = dflt.doFinal(msg);
        System.out.println(dflt.getAlgorithm() + " block=" + dflt.getBlockSize()
                + " provider=" + dflt.getProvider().getName() + " " + hex(c1));

        Cipher ecb = Cipher.getInstance("DES/ECB/PKCS5Padding");
        ecb.init(Cipher.ENCRYPT_MODE, key);
        System.out.println("ecb same=" + hex(c1).equals(hex(ecb.doFinal(msg))));
        ecb.init(Cipher.DECRYPT_MODE, key);
        System.out.println("ecb plain=" + new String(ecb.doFinal(c1), StandardCharsets.UTF_8));

        IvParameterSpec iv = new IvParameterSpec(new byte[] {9, 8, 7, 6, 5, 4, 3, 2});
        Cipher cbc = Cipher.getInstance("DES/CBC/PKCS5Padding");
        cbc.init(Cipher.ENCRYPT_MODE, key, iv);
        byte[] c2 = cbc.doFinal(msg);
        System.out.println("cbc " + hex(c2) + " iv=" + hex(cbc.getIV()));
        cbc.init(Cipher.DECRYPT_MODE, key, iv);
        System.out.println("cbc plain=" + new String(cbc.doFinal(c2), StandardCharsets.UTF_8));

        // 分段：update 输出已完成块，doFinal 收尾
        cbc.init(Cipher.ENCRYPT_MODE, key, iv);
        byte[] p1 = cbc.update(msg, 0, 5);
        byte[] p2 = cbc.update(msg, 5, 9);
        byte[] p3 = cbc.doFinal(msg, 14, msg.length - 14);
        System.out.println("parts " + (p1 == null ? 0 : p1.length) + "+" + (p2 == null ? 0 : p2.length)
                + "+" + p3.length + " same=" + hex(c2).equals(
                        (p1 == null ? "" : hex(p1)) + (p2 == null ? "" : hex(p2)) + hex(p3)));

        Cipher raw = Cipher.getInstance("DES/ECB/NoPadding");
        raw.init(Cipher.ENCRYPT_MODE, key);
        byte[] block16 = "0123456789abcdef".getBytes(StandardCharsets.UTF_8);
        System.out.println("nopad " + hex(raw.doFinal(block16)));
        try {
            raw.doFinal("short".getBytes(StandardCharsets.UTF_8));
        } catch (IllegalBlockSizeException e) {
            System.out.println("IBSE: " + e.getMessage());
        }

        Cipher bad = Cipher.getInstance("DES/ECB/PKCS5Padding");
        bad.init(Cipher.DECRYPT_MODE, wrong);
        try {
            bad.doFinal(c1);
        } catch (BadPaddingException e) {
            System.out.println("BPE: " + e.getMessage());
        }

        try {
            Cipher.getInstance("Blowfish2/ECB/NoPadding");
        } catch (NoSuchAlgorithmException e) {
            System.out.println("NSAE: " + e.getMessage());
        }
    }
}
