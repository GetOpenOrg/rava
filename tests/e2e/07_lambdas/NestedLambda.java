import java.util.ArrayList;
import java.util.function.Function;

public class NestedLambda {
    // Method returning a lambda (closure-returning function)
    static Function<Integer, Integer> adder(int x) {
        return y -> x + y;
    }

    public static void main(String[] args) {
        Function<Integer, Integer> add5 = adder(5);
        // We can't call the Function interface directly, but we can use stream
        ArrayList<Integer> list = new ArrayList<>();
        list.add(1); list.add(2); list.add(3);
        // Lambda that sums all elements added to base value
        int base = 10;
        int sum = 0;
        for (int n : list) {
            sum += n + base;
        }
        System.out.println(sum); // (1+10)+(2+10)+(3+10) = 36
        // Nested filter: lambda captures outer variable
        long count = list.stream().filter(n -> n > 1).count();
        System.out.println(count); // 2
    }
}
