public class DeepRecursion {
    // Test deep recursion with fibonacci
    static int fibonacci(int n) {
        if (n <= 1) return n;
        return fibonacci(n - 1) + fibonacci(n - 2);
    }

    // Test mutual recursion
    static boolean isEven(int n) {
        if (n == 0) return true;
        return isOdd(n - 1);
    }

    static boolean isOdd(int n) {
        if (n == 0) return false;
        return isEven(n - 1);
    }

    public static void main(String[] args) {
        // Fibonacci (not too deep to avoid stack overflow)
        System.out.println(fibonacci(20));  // 6765
        System.out.println(fibonacci(30));  // 832040

        // Mutual recursion
        System.out.println(isEven(100));  // true
        System.out.println(isOdd(99));    // true
        System.out.println(isEven(7));    // false
    }
}
