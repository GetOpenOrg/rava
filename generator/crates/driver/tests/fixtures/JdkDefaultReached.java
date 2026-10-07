import java.util.ArrayList;
import java.util.Arrays;
import java.util.Comparator;
import java.util.List;

// 用户匿名类继承的 JDK default 在调用链上（thenComparing 经用户接收者调用）：翻译 default 体，
// 体内 lambda 经 samtype 的 SAM 合成对象装箱
public class JdkDefaultReached {
    public static void main(String[] args) {
        Comparator<String> byLen = new Comparator<String>() {
            @Override
            public int compare(String x, String y) {
                return x.length() - y.length();
            }
        };
        List<String> l = new ArrayList<>(Arrays.asList("bb", "a", "ab"));
        l.sort(byLen.thenComparing(Comparator.naturalOrder()));
        System.out.println(l);
    }
}
