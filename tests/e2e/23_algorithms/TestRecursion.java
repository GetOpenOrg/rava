public class TestRecursion {

    static long fibonacci(int n) {
        if (n <= 1) return n;
        return fibonacci(n - 1) + fibonacci(n - 2);
    }

    static long factorial(int n) {
        if (n <= 1) return 1;
        return n * factorial(n - 1);
    }

    static int gcd(int a, int b) {
        if (b == 0) return a;
        return gcd(b, a % b);
    }

    static int power(int base, int exp) {
        if (exp == 0) return 1;
        if (exp % 2 == 0) {
            int half = power(base, exp / 2);
            return half * half;
        }
        return base * power(base, exp - 1);
    }

    static int sumDigits(int n) {
        if (n < 10) return n;
        return n % 10 + sumDigits(n / 10);
    }

    static String reverse(String s) {
        if (s.isEmpty()) return s;
        return reverse(s.substring(1)) + s.charAt(0);
    }

    public static void main(String[] args) {
        // fibonacci
        for (int i = 0; i <= 10; i++) {
            System.out.print(fibonacci(i));
            if (i < 10) System.out.print(" ");
        }
        System.out.println();  // 0 1 1 2 3 5 8 13 21 34 55

        // factorial
        System.out.println(factorial(0));   // 1
        System.out.println(factorial(5));   // 120
        System.out.println(factorial(10));  // 3628800

        // gcd
        System.out.println(gcd(48, 18));  // 6
        System.out.println(gcd(100, 75)); // 25
        System.out.println(gcd(7, 13));   // 1

        // power
        System.out.println(power(2, 10));  // 1024
        System.out.println(power(3, 5));   // 243
        System.out.println(power(5, 0));   // 1

        // sum digits
        System.out.println(sumDigits(12345)); // 15
        System.out.println(sumDigits(999));   // 27

        // reverse
        System.out.println(reverse("hello")); // olleh
        System.out.println(reverse("abcde")); // edcba
    }
}
