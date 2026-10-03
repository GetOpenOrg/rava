import java.util.StringTokenizer;

/**
 * StringTokenizer 老分词器（方法级实测：countTokens 11 jar / nextToken，
 * mybatis/lang3 的历史通道，此前零覆盖）：三参构造、逐 token、剩余计数、
 * delimiter 变体 nextToken(String)。
 */
public class TestStringTokenizer {

    public static void main(String[] args) {
        StringTokenizer st = new StringTokenizer("a,b,,c", ",");
        System.out.println("count=" + st.countTokens());
        StringBuilder sb = new StringBuilder();
        while (st.hasMoreTokens()) {
            sb.append(st.nextToken()).append("|");
        }
        System.out.println("tokens=" + sb);
        System.out.println("empty-after=" + !st.hasMoreTokens());
        try {
            st.nextToken();
        } catch (java.util.NoSuchElementException e) {
            System.out.println("exhaust-ex=" + e.getClass().getSimpleName());
        }

        // 连续分隔符合并、首尾分隔符忽略
        StringTokenizer st2 = new StringTokenizer(",x,,y,", ",");
        System.out.println("merged=" + st2.countTokens());
        System.out.println("first=" + st2.nextToken() + " second=" + st2.nextToken());

        // 默认分隔符集（空格族）
        StringTokenizer st3 = new StringTokenizer("a  b\tc\nd");
        System.out.println("default-delim=" + st3.countTokens());

        // 双参返回分隔符形态（returnDelims=true）
        StringTokenizer st4 = new StringTokenizer("a,b", ",", true);
        StringBuilder with = new StringBuilder();
        while (st4.hasMoreTokens()) {
            with.append(st4.nextToken()).append("/");
        }
        System.out.println("with-delims=" + with);

        // 变分隔符 nextToken(String)
        StringTokenizer st5 = new StringTokenizer("k=v;x=y", "=");
        System.out.println("pair-k=" + st5.nextToken("="));
        System.out.println("pair-rest=" + st5.nextToken(";x=y"));
    }
}
