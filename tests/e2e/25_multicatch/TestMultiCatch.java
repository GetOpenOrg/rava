public class TestMultiCatch {

    static int divide(int a, int b) {
        return a / b;
    }

    static int parseAndDivide(String a, String b) {
        return Integer.parseInt(a) / Integer.parseInt(b);
    }

    static String accessArray(int[] arr, int idx) {
        return String.valueOf(arr[idx]);
    }

    static void tryCatch(int a, int b) {
        try {
            int result = divide(a, b);
            System.out.println("result: " + result);
        } catch (ArithmeticException e) {
            System.out.println("ArithmeticException: " + e.getMessage());
        }
    }

    static void multiCatch(String a, String b) {
        try {
            int result = parseAndDivide(a, b);
            System.out.println("result: " + result);
        } catch (NumberFormatException | ArithmeticException e) {
            System.out.println("caught: " + e.getClass().getSimpleName());
        }
    }

    static void nestedTry(String[] data) {
        try {
            for (int i = 0; i < data.length; i++) {
                try {
                    int val = Integer.parseInt(data[i]);
                    System.out.println("parsed: " + val);
                } catch (NumberFormatException e) {
                    System.out.println("bad number: " + data[i]);
                }
            }
        } catch (NullPointerException e) {
            System.out.println("null data");
        }
    }

    public static void main(String[] args) {
        tryCatch(10, 2);   // result: 5
        tryCatch(10, 0);   // ArithmeticException: / by zero

        multiCatch("10", "2");   // result: 5
        multiCatch("10", "0");   // caught: ArithmeticException
        multiCatch("abc", "2");  // caught: NumberFormatException

        nestedTry(new String[]{"1", "abc", "3", "bad", "5"});
        // parsed: 1, bad number: abc, parsed: 3, bad number: bad, parsed: 5

        // finally block
        for (int i = 0; i < 3; i++) {
            try {
                if (i == 1) throw new RuntimeException("fail at 1");
                System.out.println("try: " + i);
            } catch (RuntimeException e) {
                System.out.println("caught: " + e.getMessage());
            } finally {
                System.out.println("finally: " + i);
            }
        }
        // try:0, finally:0, caught:fail at 1, finally:1, try:2, finally:2
    }
}
