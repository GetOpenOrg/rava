/**
 * 窄整型解析族（方法级实测：parseByte 9 jar / parseShort 9，此前零覆盖）：
 * 边界值、基数形态、溢出 NumberFormatException、valueOf 缓存恒等。
 */
public class TestNumericParsersByteShort {

    public static void main(String[] args) {
        System.out.println("byte-max=" + Byte.parseByte("127"));
        System.out.println("byte-min=" + Byte.parseByte("-128"));
        System.out.println("byte-hex=" + Byte.parseByte("7f", 16));
        System.out.println("byte-octal=" + Byte.parseByte("10", 8));
        try {
            Byte.parseByte("128");
        } catch (NumberFormatException e) {
            System.out.println("byte-overflow-ex=" + e.getClass().getSimpleName());
        }
        try {
            Byte.parseByte("");
        } catch (NumberFormatException e) {
            System.out.println("byte-empty-ex=" + e.getClass().getSimpleName());
        }

        System.out.println("short-max=" + Short.parseShort("32767"));
        System.out.println("short-min=" + Short.parseShort("-32768"));
        System.out.println("short-hex=" + Short.parseShort("FF", 16));
        try {
            Short.parseShort("32768");
        } catch (NumberFormatException e) {
            System.out.println("short-overflow-ex=" + e.getClass().getSimpleName());
        }

        // valueOf 缓存恒等（-128..127 同实例）
        System.out.println("byte-cached=" + (Byte.valueOf((byte) 1) == Byte.valueOf((byte) 1)));
        System.out.println("short-cached=" + (Short.valueOf((short) 100) == Short.valueOf((short) 100)));
        System.out.println("short-beyond=" + (Short.valueOf((short) 200) == Short.valueOf((short) 200)));

        // toString 与反向
        byte b = -5;
        short s = 300;
        System.out.println("roundtrip=" + Byte.parseByte(Byte.toString(b))
                + Short.parseShort(Short.toString(s)));
        System.out.println("decode-hex=" + Byte.decode("0x7F")
                + Short.decode("0x7FFF"));
    }
}
