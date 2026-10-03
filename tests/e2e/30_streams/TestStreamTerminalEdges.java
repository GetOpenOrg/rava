import java.util.List;
import java.util.function.BooleanSupplier;
import java.util.stream.Collectors;

/**
 * Stream 终止/组装边角（方法级实测：findAny 7 / mapToLong 5 / collectingAndThen 5 /
 * BooleanSupplier.getAsBoolean 5，此前零覆盖）：顺序流 findAny 稳定取首、
 * 原始特化管道、收集后加工。
 */
public class TestStreamTerminalEdges {

    public static void main(String[] args) {
        // findAny：顺序流上稳定返回首个元素
        List<String> data = List.of("a", "b", "c");
        System.out.println("findany=" + data.stream().findAny().orElse("none"));
        System.out.println("findany-empty=" + data.stream().filter(x -> x.equals("z"))
                .findAny().isEmpty());

        // mapToLong：原始特化 + 汇总
        long sum = data.stream().mapToLong(String::length).sum();
        System.out.println("sum=" + sum);
        System.out.println("summary-max=" + data.stream().mapToLong(String::length).max().orElse(-1));

        // collectingAndThen：toList 后包不可变
        var frozen = data.stream()
                .collect(Collectors.collectingAndThen(Collectors.toList(),
                        java.util.Collections::unmodifiableList));
        System.out.println("frozen=" + frozen.size());
        try {
            frozen.add("x");
        } catch (UnsupportedOperationException e) {
            System.out.println("frozen-ex=" + e.getClass().getSimpleName());
        }

        // BooleanSupplier
        BooleanSupplier flag = () -> 1 + 1 == 2;
        System.out.println("supplier=" + flag.getAsBoolean());
        int[] hits = { 0 };
        BooleanSupplier counted = () -> {
            hits[0]++;
            return hits[0] > 0;
        };
        System.out.println("supplier-counted=" + counted.getAsBoolean() + hits[0]);

        // 双管道：mapToInt 与 boxed 回流
        System.out.println("boxed=" + data.stream().mapToInt(String::length)
                .boxed().collect(Collectors.toList()));
    }
}
