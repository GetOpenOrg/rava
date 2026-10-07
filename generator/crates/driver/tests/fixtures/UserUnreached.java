import java.util.function.Supplier;

// 用户类与 JDK 类同一可达性判据（原则 §2）：调用链外的用户方法发 `stub:` 存根，不翻译方法体；
// 经 lambda / 虚派发 / 接口 default / 静态字段（类初始化）可达的照常翻译
public class UserUnreached {
    interface Greeter {
        String greet();

        default String loud() { return greet().toUpperCase(); }

        default String quiet() { return greet().toLowerCase(); }
    }

    static class Hello implements Greeter {
        public String greet() { return "Hello"; }
    }

    // 无 <clinit>、无链上方法，只经静态字段读写触发初始化：不得退化为类型存根（字段访问器不得是存根）。
    // 实例方法放在从未实例化的类上：已实例化的用户类会经 open 接收者 getClass 的反射枚举全成员入链
    // （分析器反射建模口径），链外实例方法只能在未实例化类上稳定观测
    static class Counter {
        static int count;

        String unusedInstance() { return new java.util.TreeMap<String, String>().toString(); }
    }

    static String unusedStatic() { return new java.util.ArrayDeque<String>().toString(); }

    public static void main(String[] args) {
        Supplier<String> s = () -> new Hello().loud();
        System.out.println(s.get());
        Counter.count += args.length + 1;
        System.out.println(Counter.count);
    }
}
