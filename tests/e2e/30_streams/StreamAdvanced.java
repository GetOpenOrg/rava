import java.util.ArrayList;

public class StreamAdvanced {
    public static void main(String[] args) {
        ArrayList<Integer> numbers = new ArrayList<>();
        numbers.add(1); numbers.add(2); numbers.add(3); numbers.add(4); numbers.add(5);

        System.out.println(numbers.stream().mapToInt(n -> n).sum()); // 15

        System.out.println(numbers.stream().filter(n -> n % 2 == 0).count()); // 2

        int max = 0;
        for (int n : numbers) {
            if (n > max) max = n;
        }
        System.out.println(max); // 5

        System.out.println(numbers.stream().filter(n -> n < 3).count()); // 2

        System.out.println(numbers.stream().filter(n -> n > 1).count()); // 4

        System.out.println(numbers.stream().mapToInt(n -> n * 2).sum()); // 30
    }
}
