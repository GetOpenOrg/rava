import java.security.MessageDigest;
import javax.crypto.Mac;
import javax.crypto.SecretKeyFactory;
import javax.crypto.spec.PBEKeySpec;
import javax.crypto.spec.SecretKeySpec;

/**
 * JCA 间接请求的边界：用户代码只写外层算法名，内层服务由 JDK 实现类按构造器形参名再请求。
 * - HmacSHA224 / HmacSHA384 / HmacSHA512 / HmacSHA512/256：HmacCore 子类把摘要算法名经构造器形参传给
 *   MessageDigest.getInstance（用户串里不出现 "SHA-384" 等摘要名）；
 * - PBKDF2WithHmacSHA256 / PBKDF2WithHmacSHA1：PBKDF2 实现按形参名请求 Mac，Mac 实现再按形参名请求摘要（两层间接）；
 * - 算法名来自局部字符串数组的元素。
 * 输出均为确定性摘要，逐字节可比。
 */
public class TestJcaIndirectDigest {

    static String hex(byte[] b) {
        StringBuilder sb = new StringBuilder();
        for (byte x : b) {
            sb.append(String.format("%02x", x));
        }
        return sb.toString();
    }

    public static void main(String[] args) throws Exception {
        byte[] key = "indirect-key".getBytes("UTF-8");
        byte[] msg = "indirect-message".getBytes("UTF-8");
        for (String algo : new String[] {"HmacSHA224", "HmacSHA384", "HmacSHA512", "HmacSHA512/256"}) {
            Mac m = Mac.getInstance(algo);
            m.init(new SecretKeySpec(key, algo));
            System.out.println(algo + " len=" + m.getMacLength() + " " + hex(m.doFinal(msg)));
        }

        for (String prf : new String[] {"PBKDF2WithHmacSHA256", "PBKDF2WithHmacSHA1"}) {
            SecretKeyFactory f = SecretKeyFactory.getInstance(prf);
            PBEKeySpec spec = new PBEKeySpec("password".toCharArray(), "salt-1234".getBytes("UTF-8"), 1000, 256);
            System.out.println(prf + " " + hex(f.generateSecret(spec).getEncoded()));
        }

        // 局部数组元素作算法名：直接请求摘要
        for (String d : new String[] {"SHA-224", "SHA-512/224"}) {
            System.out.println(d + " " + hex(MessageDigest.getInstance(d).digest(msg)));
        }
    }
}
