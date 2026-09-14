public class TestInterfaceStatic {

    interface MathUtils {
        static int square(int x) { return x * x; }
        static int cube(int x) { return x * x * x; }
        static boolean isPrime(int n) {
            if (n < 2) return false;
            for (int i = 2; i * i <= n; i++) {
                if (n % i == 0) return false;
            }
            return true;
        }
        static int clamp(int val, int min, int max) {
            return Math.max(min, Math.min(max, val));
        }
    }

    interface StringUtils {
        static String repeat(String s, int n) {
            StringBuilder sb = new StringBuilder();
            for (int i = 0; i < n; i++) sb.append(s);
            return sb.toString();
        }
        static boolean isPalindrome(String s) {
            int l = 0, r = s.length() - 1;
            while (l < r) {
                if (s.charAt(l) != s.charAt(r)) return false;
                l++;
                r--;
            }
            return true;
        }
        static String capitalize(String s) {
            if (s == null || s.isEmpty()) return s;
            return Character.toUpperCase(s.charAt(0)) + s.substring(1).toLowerCase();
        }
    }

    public static void main(String[] args) {
        System.out.println(MathUtils.square(5));   // 25
        System.out.println(MathUtils.cube(3));     // 27
        System.out.println(MathUtils.isPrime(7));  // true
        System.out.println(MathUtils.isPrime(9));  // false
        System.out.println(MathUtils.isPrime(2));  // true
        System.out.println(MathUtils.clamp(15, 0, 10)); // 10
        System.out.println(MathUtils.clamp(-5, 0, 10)); // 0
        System.out.println(MathUtils.clamp(5, 0, 10));  // 5

        System.out.println(StringUtils.repeat("ab", 3));       // ababab
        System.out.println(StringUtils.isPalindrome("racecar")); // true
        System.out.println(StringUtils.isPalindrome("hello"));   // false
        System.out.println(StringUtils.capitalize("hELLO"));    // Hello
        System.out.println(StringUtils.capitalize(""));         // (empty)
    }
}
