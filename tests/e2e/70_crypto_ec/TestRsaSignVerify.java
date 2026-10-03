import java.security.KeyFactory;
import java.security.KeyPair;
import java.security.KeyPairGenerator;
import java.security.SecureRandom;
import java.security.Signature;
import java.security.interfaces.RSAPrivateKey;
import java.security.interfaces.RSAPublicKey;
import java.security.spec.PKCS8EncodedKeySpec;
import java.security.spec.X509EncodedKeySpec;

/**
 * RSA 签名验签与密钥编码往返（JWT / commons-codec / BC pilot 的前奏，此前零覆盖）：
 * 固定种子密钥对、SHA256withRSA 签验签真假、X509/PKCS8 编码经 KeyFactory 还原。
 */
public class TestRsaSignVerify {

    static SecureRandom fixed() throws Exception {
        SecureRandom r = SecureRandom.getInstance("SHA1PRNG");
        r.setSeed(1234L);
        return r;
    }

    public static void main(String[] args) throws Exception {
        KeyPairGenerator kpg = KeyPairGenerator.getInstance("RSA");
        kpg.initialize(2048, fixed());
        KeyPair kp = kpg.generateKeyPair();

        RSAPublicKey pub = (RSAPublicKey) kp.getPublic();
        RSAPrivateKey priv = (RSAPrivateKey) kp.getPrivate();
        System.out.println("algo=" + pub.getAlgorithm() + " bits=" + pub.getModulus().bitLength());
        System.out.println("format=" + pub.getFormat() + "/" + priv.getFormat());

        byte[] data = "rsa-e2e-payload".getBytes("UTF-8");

        Signature signer = Signature.getInstance("SHA256withRSA");
        signer.initSign(priv, fixed());
        signer.update(data);
        byte[] sig = signer.sign();
        System.out.println("sig-len=" + sig.length);

        Signature verifier = Signature.getInstance("SHA256withRSA");
        verifier.initVerify(pub);
        verifier.update(data);
        System.out.println("verify-true=" + verifier.verify(sig));

        byte[] tampered = sig.clone();
        tampered[10] ^= 0x01;
        verifier.initVerify(pub);
        verifier.update(data);
        System.out.println("verify-tampered=" + verifier.verify(tampered));

        // 编码往返：X509 / PKCS8
        KeyFactory kf = KeyFactory.getInstance("RSA");
        RSAPublicKey pub2 = (RSAPublicKey) kf.generatePublic(
                new X509EncodedKeySpec(pub.getEncoded()));
        RSAPrivateKey priv2 = (RSAPrivateKey) kf.generatePrivate(
                new PKCS8EncodedKeySpec(priv.getEncoded()));
        System.out.println("pub-roundtrip=" + pub2.getModulus().equals(pub.getModulus()));

        Signature again = Signature.getInstance("SHA256withRSA");
        again.initSign(priv2, fixed());
        again.update(data);
        System.out.println("deterministic=" + java.util.Arrays.equals(sig, again.sign()));
    }
}
