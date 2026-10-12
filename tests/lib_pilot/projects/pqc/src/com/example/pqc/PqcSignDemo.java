package com.example.pqc;

import java.nio.charset.StandardCharsets;
import java.security.KeyPair;
import java.security.KeyPairGenerator;
import java.security.Security;
import java.security.Signature;

import org.bouncycastle.jce.provider.BouncyCastleProvider;

/**
 * 后量子密码（二）：ML-DSA-65（Dilithium-3，对应文章"文档签名/贷款签名"用例）。
 * 签名 → 验签通过；篡改一个字节 → 验签失败。
 *
 * 输出确定性说明：签名值每次随机（随机化签名），只打印布尔与长度。
 */
public class PqcSignDemo {

    public static void main(String[] args) throws Exception {
        Security.addProvider(new BouncyCastleProvider());

        // ---------- 1. ML-DSA-65 密钥对与签名 ----------
        KeyPairGenerator kpg = KeyPairGenerator.getInstance("ML-DSA-65", "BC");
        KeyPair keyPair = kpg.generateKeyPair();

        String document = "贷款合同：借款人张三，金额 500,000 元，年利率 3.6%";
        byte[] doc = document.getBytes(StandardCharsets.UTF_8);

        Signature signer = Signature.getInstance("ML-DSA-65", "BC");
        signer.initSign(keyPair.getPrivate());
        signer.update(doc);
        byte[] signature = signer.sign();

        System.out.println("[签名] 文档 " + doc.length + " 字节 -> 签名 " + signature.length
                + " 字节（文章参考值约 3300）");

        // ---------- 2. 正常验签 ----------
        Signature verifier = Signature.getInstance("ML-DSA-65", "BC");
        verifier.initVerify(keyPair.getPublic());
        verifier.update(doc);
        System.out.println("[验签] 原文验签 = " + verifier.verify(signature));

        // ---------- 3. 篡改后验签 ----------
        byte[] tampered = doc.clone();
        tampered[10] ^= 0x01; // 篡改一个字节
        Signature verifier2 = Signature.getInstance("ML-DSA-65", "BC");
        verifier2.initVerify(keyPair.getPublic());
        verifier2.update(tampered);
        System.out.println("[验签] 篡改后验签 = " + verifier2.verify(signature));

        // ---------- 4. 公钥尺寸 ----------
        System.out.println("[密钥] ML-DSA-65 公钥 " + keyPair.getPublic().getEncoded().length
                + " 字节（X.509 编码）");
    }
}
