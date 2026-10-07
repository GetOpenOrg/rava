import java.util.Arrays;
import java.util.Comparator;

// 用户匿名类实现 JDK 函数式接口，继承的 default（thenComparing 等，体内含同接口 lambda）不在调用链上：
// 展开为存根，不翻译其 JDK 字节码（回归：FileExt / IntConcat「samtype 无 SAM 合成对象」）
public class JdkDefaultUnreached {
    public static void main(String[] args) {
        String[] s = {"bb", "a", "ab"};
        Arrays.sort(s, new Comparator<String>() {
            @Override
            public int compare(String x, String y) {
                return x.compareToIgnoreCase(y);
            }
        });
        System.out.println(Arrays.toString(s));
    }
}
