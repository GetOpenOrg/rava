import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.Stack;

public class RecursiveAlgoTest {
    public static void main(String[] args) {
        // 1. Fibonacci with memoization
        Map<Integer, Long> memo = new HashMap<>();
        System.out.println("fib(10): " + fib(10, memo));
        System.out.println("fib(20): " + fib(20, memo));

        // 2. Tower of Hanoi
        List<String> moves = new ArrayList<>();
        hanoi(3, "A", "C", "B", moves);
        System.out.println("hanoi moves: " + moves.size());
        System.out.println("first: " + moves.get(0));
        System.out.println("last: " + moves.get(moves.size() - 1));

        // 3. Permutations count
        List<String> perms = new ArrayList<>();
        permute("ABC".toCharArray(), 0, perms);
        System.out.println("perms of ABC: " + perms.size());

        // 4. Binary search (iterative)
        int[] arr = {1, 3, 5, 7, 9, 11, 13, 15, 17, 19};
        System.out.println("search 7: " + binarySearch(arr, 7));
        System.out.println("search 10: " + binarySearch(arr, 10));

        // 5. Stack-based expression evaluation
        System.out.println("eval: " + evalPostfix("3 4 + 2 *"));

        // 6. GCD / LCM
        System.out.println("gcd(12,8): " + gcd(12, 8));
        System.out.println("lcm(12,8): " + lcm(12, 8));

        // 7. Power function
        System.out.println("pow(2,10): " + power(2, 10));
        System.out.println("pow(3,5): " + power(3, 5));

        System.out.println("Done.");
    }

    static long fib(int n, Map<Integer, Long> memo) {
        if (n <= 1) return n;
        if (memo.containsKey(n)) return memo.get(n);
        long result = fib(n - 1, memo) + fib(n - 2, memo);
        memo.put(n, result);
        return result;
    }

    static void hanoi(int n, String from, String to, String aux, List<String> moves) {
        if (n == 1) {
            moves.add(from + "->" + to);
            return;
        }
        hanoi(n - 1, from, aux, to, moves);
        moves.add(from + "->" + to);
        hanoi(n - 1, aux, to, from, moves);
    }

    static void permute(char[] arr, int idx, List<String> result) {
        if (idx == arr.length - 1) {
            result.add(new String(arr));
            return;
        }
        for (int i = idx; i < arr.length; i++) {
            char tmp = arr[idx];
            arr[idx] = arr[i];
            arr[i] = tmp;
            permute(arr, idx + 1, result);
            tmp = arr[idx];
            arr[idx] = arr[i];
            arr[i] = tmp;
        }
    }

    static int binarySearch(int[] arr, int target) {
        int lo = 0, hi = arr.length - 1;
        while (lo <= hi) {
            int mid = (lo + hi) / 2;
            if (arr[mid] == target) return mid;
            if (arr[mid] < target) lo = mid + 1;
            else hi = mid - 1;
        }
        return -1;
    }

    static int evalPostfix(String expr) {
        Stack<Integer> stack = new Stack<>();
        String[] tokens = expr.split(" ");
        for (String token : tokens) {
            if (token.equals("+") || token.equals("-") || token.equals("*")) {
                int b = stack.pop();
                int a = stack.pop();
                if (token.equals("+")) stack.push(a + b);
                else if (token.equals("-")) stack.push(a - b);
                else stack.push(a * b);
            } else {
                stack.push(Integer.parseInt(token));
            }
        }
        return stack.pop();
    }

    static int gcd(int a, int b) {
        while (b != 0) {
            int t = b;
            b = a % b;
            a = t;
        }
        return a;
    }

    static int lcm(int a, int b) {
        return a / gcd(a, b) * b;
    }

    static long power(long base, int exp) {
        if (exp == 0) return 1;
        if (exp % 2 == 0) {
            long half = power(base, exp / 2);
            return half * half;
        }
        return base * power(base, exp - 1);
    }
}
