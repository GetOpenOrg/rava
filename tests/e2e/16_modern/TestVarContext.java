import java.util.ArrayList;
import java.util.Arrays;
import java.util.HashMap;
import java.util.List;
import java.util.Map;

class DataHolder implements AutoCloseable {
    final String name;

    DataHolder(String name) {
        this.name = name;
    }

    String read() {
        return "data:" + name;
    }

    @Override
    public void close() {
        System.out.println("closed " + name);
    }
}

public class TestVarContext {

    public static void main(String[] args) {
        // 局部变量初始化器
        var count = 10;
        var text = "abc";
        var ratio = 1.5;
        var flag = true;
        var ch = 'x';
        System.out.println(count + " " + text + " " + ratio + " " + flag + " " + ch);

        // 方法调用结果
        var list = new ArrayList<String>();
        list.add("a");
        var size = list.size();
        System.out.println("size=" + size);

        // 泛型推断（菱形 + var）
        var map = new HashMap<String, Integer>();
        map.put("k", 1);
        var entry = map.entrySet().iterator().next();
        System.out.println("entry=" + entry.getKey() + "=" + entry.getValue());

        // 数组（推断为具体元素类型的数组）
        var nums = new int[]{1, 2, 3};
        var names = new String[]{"a", "b"};
        System.out.println(nums[0] + " " + names[1]);

        // 增强 for 中的 var
        StringBuilder sb = new StringBuilder();
        for (var n : nums) {
            sb.append(n).append(",");
        }
        for (var s : names) {
            sb.append(s).append(";");
        }
        System.out.println(sb);

        // 传统 for 中的 var
        for (var i = 0; i < 3; i++) {
            System.out.print(i);
        }
        System.out.println();

        // lambda 参数既可用显式类型，也可用推断类型
        List<String> words = Arrays.asList("aa", "b", "ccc");
        words.forEach((var w) -> System.out.print(w.length() + " "));
        System.out.println();

        // try-with-resources 中的 var
        try (var holder = new DataHolder("h1")) {
            System.out.println(holder.read());
        }

        // 多资源
        try (var h2 = new DataHolder("h2"); var h3 = new DataHolder("h3")) {
            System.out.println(h2.read() + " " + h3.read());
        }

        // 三元表达式的类型推断
        var result = flag ? "yes" : "no";
        System.out.println("ternary=" + result);

        var number = flag ? Integer.valueOf(1) : Double.valueOf(2.0);
        System.out.println("ternary number=" + number);

        // var 在静态方法的返回结果上接收
        System.out.println(makeMessage());

        // var 保存匿名类实例
        var runnable = new Runnable() {
            @Override
            public void run() {
                System.out.println("anonymous ran");
            }
        };
        runnable.run();

        // var 与 switch 表达式 / 表达式结果
        String key = "b";
        var chosen = switch (key) {
            case "a" -> 1;
            case "b" -> 2;
            default -> 0;
        };
        System.out.println("switch result=" + chosen);

        System.out.println("done");
    }

    static String makeMessage() {
        return "static-return";
    }
}
