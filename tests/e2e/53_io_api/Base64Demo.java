import java.util.Base64;

public class Base64Demo {
    public static void main(String[] args) {
        // Basic encode / decode
        Base64.Encoder enc = Base64.getEncoder();
        Base64.Decoder dec = Base64.getDecoder();

        String original = "Hello, World!";
        String encoded = enc.encodeToString(original.getBytes());
        System.out.println(encoded);

        byte[] decodedBytes = dec.decode(encoded);
        System.out.println(new String(decodedBytes));

        // encode byte array
        byte[] data = {72, 101, 108, 108, 111}; // "Hello"
        System.out.println(enc.encodeToString(data));

        // Empty input
        System.out.println(enc.encodeToString(new byte[0]));
        System.out.println(dec.decode("").length == 0);

        // URL encoder (no padding issues with standard)
        Base64.Encoder urlEnc = Base64.getUrlEncoder();
        String urlEncoded = urlEnc.encodeToString("hello/world+test".getBytes());
        System.out.println(urlEncoded.contains("+") || urlEncoded.contains("/") || urlEncoded.length() > 0);

        // Round-trip various strings
        String[] tests = {"a", "ab", "abc", "abcd", "ruva test 123"};
        for (String t : tests) {
            byte[] e = enc.encode(t.getBytes());
            byte[] d = dec.decode(e);
            System.out.println(new String(d).equals(t));
        }

        System.out.println("done");
    }
}
