/**
 * 字符串拼接模板（invokedynamic makeConcatWithConstants）的常量段含换行 / 制表 /
 * 首尾空白 / 花括号 / 反斜杠 / 控制字符（Rosetta 语料 WordWrap 揭出：模板含换行时常量段整段
 * 丢失；BWT 揭出：控制字符转义 （Rust 形态 u{..}） 被当作占位花括号双写；字面量含 \\u0001 /
 * \\u0002 时 javac 把它移为配方常量实参，常量值须原样回填而非参与实参位切分）。
 * 每行输出用 [] 包裹，空白差异可见。
 */
public class TestConcatTemplateWhitespace {
    static String show(String s) {
        return "[" + s.replace("\n", "\\n").replace("\r", "\\r").replace("\t", "\\t") + "]";
    }

    static String show2(String s) {
        StringBuilder sb = new StringBuilder("[");
        for (char ch : s.toCharArray()) {
            if (ch < 0x20 || ch == 0x7f) sb.append("<").append((int) ch).append(">");
            else sb.append(ch);
        }
        return sb.append("]").toString();
    }

    public static void main(String[] args) {
        String w = "word";
        int n = 42;
        char c = 'x';
        double d = 1.5;
        boolean b = true;
        Object o = null;

        System.out.println(show("\n" + w + " "));              // 前导换行 + 尾空格
        System.out.println(show(w + "\n"));                     // 尾换行
        System.out.println(show("\n" + w));                     // 仅前导换行
        System.out.println(show("a\nb" + w + "c\nd"));          // 中段换行
        System.out.println(show("\n\n" + n + "\n\n"));          // 连续换行 + int
        System.out.println(show("\r\n" + c + "\t"));            // CRLF + char + 制表
        System.out.println(show("  " + d + "  "));              // 首尾多空格 + double
        System.out.println(show("\t" + b + "\n" + o + "\n"));   // 多参数间换行 + boolean + null
        System.out.println(show("{" + w + "}\n"));              // 花括号 + 换行
        System.out.println(show("\\n" + w + "\\"));             // 字面反斜杠（非换行）
        System.out.println(show(w + "\n" + n + "\n" + c));      // 三参数，常量段只有换行
        System.out.println(show2("\u0002" + w + "\u0003"));     // BWT 原形态：STX/ETX 控制字符
        System.out.println(show2("{\u0001}" + n + "\u001f{"));  // 控制字符紧邻花括号
        System.out.println(show2("\u007f" + c + "\u0000"));    // DEL + NUL
        System.out.println(show2("a\u0002b" + n + "\u0001\u0002" + w + "{\u0001" + c));  // 多常量位：字面量含 \u0001 与 \u0002
        System.out.print("\n" + w + " ");                       // WordWrap 原形态（直接输出）
        System.out.print(w + "\n");
        System.out.println("end");
    }
}
