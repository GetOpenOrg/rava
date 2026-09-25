import java.security.*;

public class SecurityDemo {
    public static void main(String[] args) throws Exception {
        // MessageDigest SHA-256
        MessageDigest sha256 = MessageDigest.getInstance("SHA-256");
        byte[] hash = sha256.digest("hello".getBytes());
        System.out.println(hash.length); // 32 bytes

        // Hex string conversion
        StringBuilder hex = new StringBuilder();
        for (byte b : hash) {
            hex.append(String.format("%02x", b & 0xff));
        }
        String hashStr = hex.toString();
        System.out.println(hashStr.length() == 64);
        // SHA-256("hello") is deterministic
        System.out.println(hashStr);

        // MD5
        MessageDigest md5 = MessageDigest.getInstance("MD5");
        byte[] md5hash = md5.digest("hello".getBytes());
        System.out.println(md5hash.length); // 16 bytes

        // Same input -> same hash
        MessageDigest sha256b = MessageDigest.getInstance("SHA-256");
        byte[] hash2 = sha256b.digest("hello".getBytes());
        System.out.println(java.util.Arrays.equals(hash, hash2));

        // SHA-1
        MessageDigest sha1 = MessageDigest.getInstance("SHA-1");
        byte[] sha1hash = sha1.digest("test".getBytes());
        System.out.println(sha1hash.length); // 20 bytes

        // SecureRandom — just test it doesn't crash
        SecureRandom sr = new SecureRandom();
        byte[] randomBytes = new byte[16];
        sr.nextBytes(randomBytes);
        System.out.println(randomBytes.length == 16);
        System.out.println(sr.nextInt() >= Integer.MIN_VALUE);

        System.out.println("done");
    }
}
