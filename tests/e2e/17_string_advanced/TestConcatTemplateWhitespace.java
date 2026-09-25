/**
 * 字符串拼接模板（invokedynamic makeConcatWithConstants）的常量段含换行 / 制表 /
 * 首尾空白 / 花括号 / 反斜杠（Rosetta 语料 WordWrap 揭出：模板含换行时常量段整段丢失）。
 * 每行输出用 [] 包裹，空白差异可见。
 */
public class TestConcatTemplateWhitespace {
    static String show(String s) {
        return "[" + s.replace("\n", "\\n").replace("\r", "\\r").replace("\t", "\\t") + "]";
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
        System.out.print("\n" + w + " ");                       // WordWrap 原形态（直接输出）
        System.out.print(w + "\n");
        System.out.println("end");
    }
}
