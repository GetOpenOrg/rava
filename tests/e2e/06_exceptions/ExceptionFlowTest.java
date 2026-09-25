import java.util.ArrayList;
import java.util.List;

public class ExceptionFlowTest {
    public static void main(String[] args) {
        // 1. Cross-method exception in loop
        List<String> results = new ArrayList<>();
        for (int i = -2; i <= 2; i++) {
            try {
                results.add(divide(100, i));
            } catch (ArithmeticException e) {
                results.add("ERR");
            }
        }
        System.out.println("div: " + results);

        // 2. Chained cross-method exceptions
        try {
            String s = outer(0);
            System.out.println("outer: " + s);
        } catch (IllegalArgumentException e) {
            System.out.println("chained: " + e.getMessage());
        }

        // 3. Exception in recursive method
        System.out.println("fib(-1): " + safeFib(-1));
        System.out.println("fib(7): " + safeFib(7));

        // 4. Finally with cross-method exception
        StringBuilder sb = new StringBuilder();
        try {
            sb.append("try ");
            String val = divide(1, 0);
            sb.append(val);
        } catch (ArithmeticException e) {
            sb.append("catch ");
        } finally {
            sb.append("finally");
        }
        System.out.println("flow: " + sb.toString());

        // 5. Multiple exceptions from different methods
        for (int i = 0; i < 3; i++) {
            try {
                System.out.println("op" + i + ": " + riskyOp(i));
            } catch (ArithmeticException e) {
                System.out.println("op" + i + ": arith=" + e.getMessage());
            } catch (IllegalArgumentException e) {
                System.out.println("op" + i + ": arg=" + e.getMessage());
            }
        }

        System.out.println("Done.");
    }

    static String divide(int a, int b) {
        return String.valueOf(a / b);
    }

    static String outer(int x) {
        return inner(x);
    }

    static String inner(int x) {
        if (x <= 0) {
            throw new IllegalArgumentException("negative: " + x);
        }
        return String.valueOf(x * 2);
    }

    static String safeFib(int n) {
        try {
            return String.valueOf(fib(n));
        } catch (IllegalArgumentException e) {
            return "error: " + e.getMessage();
        }
    }

    static int fib(int n) {
        if (n < 0) throw new IllegalArgumentException("n<0");
        if (n <= 1) return n;
        return fib(n - 1) + fib(n - 2);
    }

    static String riskyOp(int op) {
        if (op == 0) return String.valueOf(10 / 0);
        if (op == 1) throw new IllegalArgumentException("bad op");
        return "ok";
    }
}
