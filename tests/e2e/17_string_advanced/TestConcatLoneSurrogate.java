import java.lang.annotation.Retention;
import java.lang.annotation.RetentionPolicy;

/**
 * 字符串拼接（invokedynamic makeConcatWithConstants）保住孤立代理项：配方常量段、常量位值
 * （字面量含 \\u0001 时 javac 移为配方常量实参）、char 实参、String 实参中的孤立代理项
 * 必须原样进入结果，不得变成 U+FFFD；两个半代理拼接后须合成合法代理对。
 * 另含注解字符串常量中的孤立代理项（经稀疏常量池 → AnnotationParser）。
 * 输出一律按 UTF-16 码元十六进制呈现，与标准输出编码无关。
 */
public class TestConcatLoneSurrogate {
    @Retention(RetentionPolicy.RUNTIME)
    @interface Tag {
        String value();
    }

    @Tag("t\uDBFFz")
    static class Tagged {
    }

    static String hex(String s) {
        StringBuilder sb = new StringBuilder("[");
        for (int i = 0; i < s.length(); i++) {
            if (i > 0) sb.append(' ');
            sb.append(Integer.toHexString(s.charAt(i)));
        }
        return sb.append("] len=").append(s.length()).toString();
    }

    public static void main(String[] args) {
        int n = 7;
        char hi = '\uD83D';
        char lo = '\uDE00';
        String loneHi = String.valueOf(hi);
        String loneLo = new String(new char[] {lo});

        // 配方常量段含孤立代理项
        System.out.println(hex("\uD800=" + n));
        System.out.println(hex(n + "\uDFFF"));
        // 常量位值含孤立代理项（字面量带 \u0001，javac 把它移为常量实参）
        System.out.println(hex("\u0001\uDC01" + n));
        // char 实参
        System.out.println(hex("a" + lo + "b"));
        System.out.println(hex(hi + "|" + n));
        // String 实参
        System.out.println(hex("<" + loneHi + ">"));
        System.out.println(hex(loneLo + n + loneHi));
        // 两个半代理拼接成合法代理对
        String pair = "" + hi + lo;
        System.out.println(hex(pair) + " cp=" + Integer.toHexString(pair.codePointAt(0)));
        String pair2 = loneHi + loneLo;
        System.out.println(pair2.equals("😀") + " " + pair2.codePointCount(0, pair2.length()));
        // 拼接结果不含替换字符
        System.out.println(("x" + loneHi + lo + n).indexOf('�'));
        // 注解字符串常量
        Tag tag = Tagged.class.getAnnotation(Tag.class);
        System.out.println(hex(tag.value()));
    }
}
