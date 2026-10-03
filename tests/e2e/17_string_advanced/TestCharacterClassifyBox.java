/**
 * Character 分类器全集与拆箱链（方法级实测：isWhitespace 28 jar / charValue 26 /
 * shortValue 22 / byteValue 21 / isLetterOrDigit 20 / charCount 19 / isLowerCase 15 /
 * isJavaIdentifierPart 14，此前零覆盖）：分类布尔面、代理对 charCount、
 * Number 拆箱链（包装→窄化）。
 */
public class TestCharacterClassifyBox {

    public static void main(String[] args) {
        System.out.println("ws-space=" + Character.isWhitespace(' ')
                + " ws-nbsp=" + Character.isWhitespace('\u00A0')
                + " ws-tab=" + Character.isWhitespace('\t'));
        System.out.println("lod=" + Character.isLetterOrDigit('a')
                + Character.isLetterOrDigit('中')
                + Character.isLetterOrDigit('!'));
        System.out.println("lower=" + Character.isLowerCase('a')
                + " upper=" + Character.isLowerCase('A'));
        System.out.println("ident-start=" + Character.isJavaIdentifierStart('$')
                + Character.isJavaIdentifierStart('9') + Character.isJavaIdentifierStart('Z'));
        System.out.println("ident-part=" + Character.isJavaIdentifierPart('5')
                + Character.isJavaIdentifierPart('-') + Character.isJavaIdentifierPart('_'));

        // 代理对：charCount 与增补字符（U+1D11E 五线谱谱号）
        int cp = 0x1D11E;
        System.out.println("charCount=" + Character.charCount(cp)
                + " bmp=" + Character.charCount('A'));
        String supp = new String(Character.toChars(cp));
        System.out.println("supp-len=" + supp.length()
                + " codePointAt=" + Integer.toHexString(supp.codePointAt(0)));

        // Number 拆箱链：Integer → 全窄化
        Integer boxed = 300;
        System.out.println("int=" + (boxed.intValue() == 300)
                + " long=" + boxed.longValue()
                + " double=" + boxed.doubleValue());
        Character c = 'Z';
        System.out.println("char-value=" + c.charValue()
                + " numeric=" + Character.getNumericValue('Z'));
        Short s = 700;
        System.out.println("short-value=" + s.shortValue()
                + " int-from-short=" + s.intValue());
        Byte b = 40;
        System.out.println("byte-value=" + b.byteValue() + " double-from-byte=" + b.doubleValue());
        Long l = 5L;
        System.out.println("float-from-long=" + l.floatValue());
    }
}
