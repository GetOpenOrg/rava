import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;

public class LambdaCaptureTest {
    public static void main(String[] args) {
        // Test 1: forEach 修改捕获的 ArrayList
        ArrayList<Integer> list = new ArrayList<>();
        List<Integer> numbers = Arrays.asList(1, 2, 3, 4, 5);
        numbers.forEach(n -> list.add(n));
        System.out.println("forEach capture: " + list.size());

        // Test 2: stream().forEach 修改捕获的 ArrayList
        ArrayList<String> result = new ArrayList<>();
        List<String> words = Arrays.asList("a", "b", "c");
        words.stream().forEach(w -> result.add(w.toUpperCase()));
        System.out.println("stream forEach capture: " + result.size());

        // Test 3: map + collect 捕获
        List<Integer> doubled = numbers.stream()
            .map(n -> n * 2)
            .collect(java.util.stream.Collectors.toList());
        System.out.println("map collect: " + doubled.size());

        // Test 4: int[] 数组捕获（引用语义）
        int[] count = {0};
        numbers.forEach(n -> count[0] += n);
        System.out.println("counter capture: " + count[0]);
    }
}
