import java.util.regex.Matcher;
import java.util.regex.Pattern;

/**
 * regex 锚定语义（方法级实测：lookingAt 5 jar，此前零覆盖）：
 * lookingAt 前缀锚定 vs matches 全串锚定、命中后 group 边界、reset 复用。
 */
public class TestRegexAnchors {

    public static void main(String[] args) {
        Pattern p = Pattern.compile("\\d+");

        // lookingAt：前缀匹配即真（不要求全串）
        Matcher m = p.matcher("123abc");
        System.out.println("looking=" + m.lookingAt() + " group=" + m.group()
                + " end=" + m.end());

        // matches：全串锚定
        Matcher m2 = p.matcher("123abc");
        System.out.println("full-miss=" + !m2.matches());

        // 前缀不匹配 → lookingAt false
        Matcher m3 = p.matcher("abc123");
        System.out.println("no-prefix=" + !m3.lookingAt());

        // 复用同一 Matcher：lookingAt 后 reset 再 matches
        Matcher m4 = p.matcher("42");
        System.out.println("first-look=" + m4.lookingAt());
        m4.reset();
        System.out.println("after-reset-matches=" + m4.matches());

        // 带锚点模式：^$ 与 region 无关的语义
        Pattern anchored = Pattern.compile("^[a-z]+$");
        System.out.println("anchored-hit=" + anchored.matcher("abc").matches());
        System.out.println("anchored-miss=" + !anchored.matcher(" abc").matches());

        // find 多命中迭代（既有面回顾 + 与锚定对照）
        Matcher f = p.matcher("a1b22c333");
        StringBuilder hits = new StringBuilder();
        while (f.find()) {
            hits.append(f.group()).append(";");
        }
        System.out.println("find-all=" + hits);
    }
}
