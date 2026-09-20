import java.util.List;
import java.util.Map;

public class TestUnnamedVar {
    public static void main(String[] args) {
        int sum = 0;
        for (int _ : new int[]{1, 2, 3, 4, 5}) {
            sum++;
        }
        System.out.println("count=" + sum);

        Map.Entry<String, Integer> pair = new java.util.AbstractMap.SimpleEntry<>("k", 42);
        if (pair instanceof Map.Entry<?, Integer> _) {
            System.out.println("entry");
        }

        List<Integer> nums = List.of(1, 2, 3);
        int total = 0;
        for (var _ : nums) {
            total += 1;
        }
        System.out.println("total=" + total);

        Runnable r = () -> {
            var _ = 1;
            System.out.println("lambda");
        };
        r.run();

        System.out.println("done");
    }
}
