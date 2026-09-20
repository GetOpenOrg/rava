import java.util.Arrays;
import java.util.HexFormat;

public class TestHexFormat {
    public static void main(String[] args) {
        HexFormat hf = HexFormat.of();
        System.out.println("fromByte=" + hf.toHexDigits((byte) 0xAB));
        System.out.println("fromInt=" + hf.toHexDigits(0x1234));
        System.out.println("fromBytes=" + hf.formatHex(new byte[]{0x01, (byte) 0xFF, 0x3C}));
        System.out.println("parse=" + hf.fromHexDigit('F'));
        byte[] b = hf.parseHex("deadbeef");
        System.out.println("parseBytes=" + Arrays.toString(b));
        System.out.println("delimited=" + HexFormat.ofDelimiter(" ").formatHex(new byte[]{0x0A, 0x0B}));
    }
}
