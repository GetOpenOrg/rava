import java.nio.charset.StandardCharsets;
import java.security.MessageDigest;
import java.security.NoSuchAlgorithmException;

/**
 * K-JCA：MessageDigest 服务查找 + 翻译字节码的摘要实现（sun.security.provider.MD5 / SHA / SHA2）。
 * 覆盖：MD5 / SHA-1 / SHA-256 已知向量、算法名大小写不敏感、分段 update（单字节 / 区间）与一次性
 * 等价、reset、digest 写入外部缓冲、clone 后分叉、isEqual、getAlgorithm / getDigestLength /
 * getProvider、toString、未知算法 → NoSuchAlgorithmException 消息。
 */
public class TestMessageDigestApi {
    static String hex(byte[] b) {
        StringBuilder sb = new StringBuilder();
        for (byte x : b) sb.append(String.format("%02x", x & 0xff));
        return sb.toString();
    }

    public static void main(String[] args) throws Exception {
        byte[] abc = "abc".getBytes(StandardCharsets.UTF_8);
        byte[] empty = new byte[0];
        for (String alg : new String[] {"MD5", "SHA-1", "SHA-256"}) {
            MessageDigest md = MessageDigest.getInstance(alg);
            System.out.println(md.getAlgorithm() + " len=" + md.getDigestLength()
                    + " provider=" + md.getProvider().getName());
            System.out.println("  abc   " + hex(md.digest(abc)));
            System.out.println("  empty " + hex(md.digest(empty)));
        }

        // 大小写不敏感：返回对象的 getAlgorithm 保留调用方写法（JDK 语义）
        MessageDigest lower = MessageDigest.getInstance("sha-256");
        System.out.println(lower.getAlgorithm() + " " + hex(lower.digest(abc)).substring(0, 16));

        // 分段 update 与一次性等价（跨 64 字节块边界）
        byte[] data = new byte[200];
        for (int i = 0; i < data.length; i++) data[i] = (byte) (i * 7 + 3);
        MessageDigest whole = MessageDigest.getInstance("SHA-256");
        String expect = hex(whole.digest(data));
        MessageDigest parts = MessageDigest.getInstance("SHA-256");
        parts.update(data[0]);
        parts.update(data, 1, 63);
        parts.update(data, 64, 100);
        for (int i = 164; i < 200; i++) parts.update(data[i]);
        System.out.println("segmented equal=" + expect.equals(hex(parts.digest())));

        // reset 丢弃已输入数据
        MessageDigest r = MessageDigest.getInstance("MD5");
        r.update("garbage".getBytes(StandardCharsets.UTF_8));
        r.reset();
        System.out.println("reset " + hex(r.digest(abc)));

        // digest 写入外部缓冲（偏移处）
        MessageDigest into = MessageDigest.getInstance("SHA-1");
        into.update(abc);
        byte[] buf = new byte[24];
        int n = into.digest(buf, 2, 20);
        System.out.println("into n=" + n + " head=" + buf[0] + "," + buf[1] + " " + hex(buf).substring(4, 12));

        // clone 后分叉
        MessageDigest base = MessageDigest.getInstance("SHA-256");
        base.update("prefix-".getBytes(StandardCharsets.UTF_8));
        MessageDigest fork = (MessageDigest) base.clone();
        base.update("a".getBytes(StandardCharsets.UTF_8));
        fork.update("b".getBytes(StandardCharsets.UTF_8));
        String ha = hex(base.digest());
        String hb = hex(fork.digest());
        MessageDigest direct = MessageDigest.getInstance("SHA-256");
        System.out.println("clone differs=" + !ha.equals(hb)
                + " forkOk=" + hb.equals(hex(direct.digest("prefix-b".getBytes(StandardCharsets.UTF_8)))));

        // isEqual
        byte[] d1 = MessageDigest.getInstance("MD5").digest(abc);
        byte[] d2 = MessageDigest.getInstance("MD5").digest(abc);
        byte[] d3 = MessageDigest.getInstance("MD5").digest(empty);
        System.out.println("isEqual " + MessageDigest.isEqual(d1, d2) + " " + MessageDigest.isEqual(d1, d3));

        System.out.println(MessageDigest.getInstance("MD5"));

        try {
            MessageDigest.getInstance("FOO-1");
        } catch (NoSuchAlgorithmException e) {
            System.out.println("NSAE: " + e.getMessage());
        }
    }
}
