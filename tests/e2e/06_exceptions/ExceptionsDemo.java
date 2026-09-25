public class ExceptionsDemo {
    static int parseOrDefault(String s) {
        try {
            return Integer.parseInt(s);
        } catch (NumberFormatException e) {
            System.out.println("NFE: " + e.getMessage());
            return -1;
        }
    }

    static void checkIndex(int[] arr, int i) {
        try {
            System.out.println(arr[i]);
        } catch (ArrayIndexOutOfBoundsException e) {
            System.out.println("OOB caught");
        }
    }

    static void checkNull(String s) {
        try {
            if (s == null) throw new NullPointerException("null string");
            System.out.println(s.length());
        } catch (NullPointerException e) {
            System.out.println("NPE: " + e.getMessage());
        }
    }

    static void checkArg(int n) {
        try {
            if (n < 0) throw new IllegalArgumentException("negative: " + n);
            System.out.println("ok: " + n);
        } catch (IllegalArgumentException e) {
            System.out.println("IAE: " + e.getMessage());
        }
    }

    public static void main(String[] args) {
        System.out.println(parseOrDefault("42"));    // 42
        System.out.println(parseOrDefault("abc"));   // NFE: For input string: "abc" / -1
        int[] arr = {10, 20, 30};
        checkIndex(arr, 1);   // 20
        checkNull("hello");   // 5
        checkArg(5);          // ok: 5
        checkArg(-3);         // IAE: negative: -3
    }
}
