import java.util.List;
import java.util.stream.Stream;

// SharedSecrets 惰性访问器：getJavaUtilCollectionAccess 首次取到 null 时经
// Class.forName("java.util.ImmutableCollections$Access", true, null) 触发其 <clinit> 注册；
// 用户代码只经 Stream.toList 间接到达（不直接触及 ImmutableCollections）
public class TestSharedSecretsLazyAccess {
    public static void main(String[] args) {
        List<Integer> a = Stream.of(3, 1, 2).toList();
        System.out.println(a);
        List<String> b = Stream.of("x", null, "z").toList();
        System.out.println(b + " size=" + b.size() + " get1=" + b.get(1));
        try {
            a.add(4);
            System.out.println("mutable");
        } catch (UnsupportedOperationException e) {
            System.out.println("immutable");
        }
        System.out.println(Stream.<Integer>empty().toList().isEmpty());
    }
}
