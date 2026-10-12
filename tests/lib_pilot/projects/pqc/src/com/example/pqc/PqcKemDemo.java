package com.example.pqc;

import java.nio.charset.StandardCharsets;
import java.security.KeyPair;
import java.security.KeyPairGenerator;
import java.security.Security;
import java.util.Arrays;

import javax.crypto.Cipher;
import javax.crypto.KeyGenerator;
import javax.crypto.SecretKey;
import javax.crypto.spec.GCMParameterSpec;

import org.bouncycastle.jcajce.SecretKeyWithEncapsulation;
import org.bouncycastle.jcajce.spec.KEMExtractSpec;
import org.bouncycastle.jcajce.spec.KEMGenerateSpec;
import org.bouncycastle.jce.provider.BouncyCastleProvider;

/**
 * 后量子密码（一）：ML-KEM-768（Kyber-768，对应文章"服务间载荷加密"用例）。
 * KEM 封装/解封装得到共享密钥 → 用它做 AES-256-GCM 混合加密。
 *
 * 说明：文章的 PqcStarterLib 为虚构库；JDK 21 无原生 ML-KEM（JDK 24+ 才有，JEP 496），
 * 此处用 Bouncy Castle 1.80 的 JCE KeyGenerator 路线（BC 未注册 javax.crypto.KEM
 * 服务，算法名用聚合名 "ML-KEM"，默认参数即 ML-KEM-768）。
 *
 * 输出确定性说明：封装密钥/密文每次随机，只打印布尔结论与长度。
 */
public class PqcKemDemo {

    public static void main(String[] args) throws Exception {
        Security.addProvider(new BouncyCastleProvider());

        // ---------- 1. ML-KEM 密钥对（默认 ML-KEM-768 参数集） ----------
        KeyPairGenerator kpg = KeyPairGenerator.getInstance("ML-KEM", "BC");
        KeyPair keyPair = kpg.generateKeyPair();
        System.out.println("[密钥] ML-KEM-768 公钥 X.509 编码 " + keyPair.getPublic().getEncoded().length
                + " 字节（裸公钥约 1184，文章参考值）");

        // ---------- 2. 发送方：封装生成共享密钥（密钥+封装密钥一起生成） ----------
        KeyGenerator kemGen = KeyGenerator.getInstance("ML-KEM", "BC");
        kemGen.init(new KEMGenerateSpec(keyPair.getPublic(), "AES"));
        SecretKeyWithEncapsulation senderKey =
                (SecretKeyWithEncapsulation) kemGen.generateKey();
        byte[] encapsulation = senderKey.getEncapsulation(); // 随密文一起发送
        System.out.println("[封装] 封装密钥 " + encapsulation.length + " 字节");

        // ---------- 3. 接收方：解封装还原同一共享密钥 ----------
        KeyGenerator kemDec = KeyGenerator.getInstance("ML-KEM", "BC");
        kemDec.init(new KEMExtractSpec(keyPair.getPrivate(), encapsulation, "AES"));
        SecretKey receiverKey = kemDec.generateKey();
        System.out.println("[协商] 双方共享密钥一致 = "
                + Arrays.equals(senderKey.getEncoded(), receiverKey.getEncoded()));

        // ---------- 4. 混合加密：AES-GCM 加密固定明文 ----------
        String plaintext = "转账 100.00 元至账户 6222****1234";
        byte[] plain = plaintext.getBytes(StandardCharsets.UTF_8);
        byte[] iv = new byte[12];

        Cipher encrypt = Cipher.getInstance("AES/GCM/NoPadding");
        encrypt.init(Cipher.ENCRYPT_MODE, senderKey, new GCMParameterSpec(128, iv));
        byte[] ciphertext = encrypt.doFinal(plain);

        Cipher decrypt = Cipher.getInstance("AES/GCM/NoPadding");
        decrypt.init(Cipher.DECRYPT_MODE, receiverKey, new GCMParameterSpec(128, iv));
        String recovered = new String(decrypt.doFinal(ciphertext), StandardCharsets.UTF_8);

        System.out.println("[混合加密] 明文 " + plain.length + " 字节 -> 密文 " + ciphertext.length + " 字节");
        System.out.println("[混合加密] 解密还原一致 = " + plaintext.equals(recovered));
    }
}
