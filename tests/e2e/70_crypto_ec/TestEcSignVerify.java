import java.security.KeyPair;
import java.security.KeyPairGenerator;
import java.security.SecureRandom;
import java.security.Signature;
import java.security.spec.ECGenParameterSpec;

/**
 * jdk.crypto.ec（SunEC）：SHA256withECDSA 签名/验签（真假两路）与 ASN.1 签名结构
 * （jmod 覆盖计划 A 档；固定种子确定性签名，DER 长度逐字可比）。
 */
public class TestEcSignVerify {

    static SecureRandom fixed() throws Exception {
        SecureRandom r = SecureRandom.getInstance("SHA1PRNG");
        r.setSeed(99L);
        return r;
    }

    public static void main(String[] args) throws Exception {
        KeyPairGenerator kpg = KeyPairGenerator.getInstance("EC");
        kpg.initialize(new ECGenParameterSpec("secp256r1"), fixed());
        KeyPair kp = kpg.generateKeyPair();

        byte[] data = "ecdsa-e2e-payload".getBytes("UTF-8");

        Signature signer = Signature.getInstance("SHA256withECDSA");
        signer.initSign(kp.getPrivate(), fixed());
        signer.update(data);
        byte[] sig = signer.sign();
        System.out.println("sig-len=" + sig.length);
        System.out.println("sig-head=" + String.format("%02X", sig[0])
                + " " + String.format("%02X", sig[1]));

        Signature verifier = Signature.getInstance("SHA256withECDSA");
        verifier.initVerify(kp.getPublic());
        verifier.update(data);
        System.out.println("verify-true=" + verifier.verify(sig));

        byte[] tampered = sig.clone();
        tampered[tampered.length - 1] ^= 0x01;
        verifier.initVerify(kp.getPublic());
        verifier.update(data);
        System.out.println("verify-tampered=" + verifier.verify(tampered));

        byte[] wrongData = "other".getBytes("UTF-8");
        verifier.initVerify(kp.getPublic());
        verifier.update(wrongData);
        System.out.println("verify-wrong-data=" + verifier.verify(sig));

        // 同种子重签 → 逐字节一致（DER 结构无随机化成分的确定性验证）
        Signature again = Signature.getInstance("SHA256withECDSA");
        again.initSign(kp.getPrivate(), fixed());
        again.update(data);
        byte[] sig2 = again.sign();
        System.out.println("deterministic=" + java.util.Arrays.equals(sig, sig2));
    }
}
