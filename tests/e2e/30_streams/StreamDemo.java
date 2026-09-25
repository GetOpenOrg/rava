import java.util.Arrays;
import java.util.List;

public class StreamDemo {
    public static void main(String[] args) {
        List<Integer> list = Arrays.asList(1, 2, 3, 4, 5);
        System.out.println(list.size()); // 5
        long count = list.stream().filter(x -> x > 1).count();
        System.out.println(count); // 4
    }
}
