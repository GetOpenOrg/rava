/**
 * Character 大小写的码点重载族（方法级实测：isUpperCase(int) 22 jar——char 重载已有、
 * 码点重载零命中，此前零覆盖）：增补平面字母的大小写与多字符映射计数。
 */
public class TestCharacterCaseCodePoint {

    public static void main(String[] args) {
        // 基本面：码点重载与 char 重载一致
        System.out.println("upper-basic=" + Character.isUpperCase('A')
                + Character.isUpperCase((int) 'A'));
        System.out.println("lower-basic=" + Character.isLowerCase('z')
                + Character.isLowerCase((int) 'z'));
        System.out.println("mixed=" + Character.isUpperCase('a')
                + Character.isLowerCase('Q'));

        // 增补平面：数学字母 DESERET CAPITAL（U+10400 区）
        int suppUpper = 0x10400;   // 𐐀
        int suppLower = 0x10428;   // 𐐨
        System.out.println("supp-upper=" + Character.isUpperCase(suppUpper));
        System.out.println("supp-lower=" + Character.isLowerCase(suppLower));

        // 大小写转换的码点形态
        System.out.println("to-upper-supp=" + Integer.toHexString(Character.toUpperCase(suppLower)));
        System.out.println("to-lower-supp=" + Integer.toHexString(Character.toLowerCase(suppUpper)));

        // 多字符大小写映射（ß → SS）
        java.lang.String mapped = java.lang.String.valueOf('ß').toUpperCase(java.util.Locale.ROOT);
        System.out.println("sharp-s-len=" + mapped.length() + " value=" + mapped);
    }
}
