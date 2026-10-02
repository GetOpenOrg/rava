import java.security.KeyPair;
import java.security.KeyPairGenerator;
import java.security.SecureRandom;
import java.security.interfaces.ECPublicKey;
import java.security.spec.ECGenParameterSpec;
import java.util.Arrays;

import javax.crypto.KeyAgreement;

/**
 * jdk.crypto.ec（SunEC）：固定种子下的 EC 密钥对生成与 ECDH 共享密钥
 * （jmod 覆盖计划 A 档；只打印相等性与长度，不打印密钥值；JCA 提供者跨模块注册）。
 */
public class TestEcKeyAgreement {

    static SecureRandom fixed() throws Exception {
        SecureRandom r = SecureRandom.getInstance("SHA1PRNG");
        r.setSeed(42L);
        return r;
    }

    public static void main(String[] args) throws Exception {
        KeyPairGenerator kpg = KeyPairGenerator.getInstance("EC");
        kpg.initialize(new ECGenParameterSpec("secp256r1"), fixed());
        KeyPair a = kpg.generateKeyPair();
        KeyPair b = kpg.generateKeyPair();

        System.out.println("algo=" + a.getPublic().getAlgorithm());
        System.out.println("format=" + a.getPublic().getFormat());
        System.out.println("field-size="
                + ((ECPublicKey) a.getPublic()).getParams().getCurve().getField().getFieldSize());

        KeyAgreement ka = KeyAgreement.getInstance("ECDH");
        ka.init(a.getPrivate());
        ka.doPhase(b.getPublic(), true);
        byte[] s1 = ka.generateSecret();

        KeyAgreement kb = KeyAgreement.getInstance("ECDH");
        kb.init(b.getPrivate());
        kb.doPhase(a.getPublic(), true);
        byte[] s2 = kb.generateSecret();

        System.out.println("secret-len=" + s1.length);
        System.out.println("both-eq=" + Arrays.equals(s1, s2));
    }
}
