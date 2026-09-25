public class ExceptionVarReuseTest {
    // Test: JVM reuses same variable slot for array and exception across try/catch
    public static int processArray(int[] data) {
        int result = 0;
        try {
            for (int i = 0; i < data.length; i++) {
                result += data[i];
                if (data[i] < 0) {
                    throw new IllegalArgumentException("negative: " + data[i]);
                }
            }
        } catch (Exception e) {
            System.out.println("Caught: " + e.getMessage());
            return -1;
        }
        return result;
    }

    // Test: try/catch with string processing (different type in slot)
    public static String readAndProcess(String input) {
        StringBuilder sb = new StringBuilder();
        try {
            String[] parts = input.split(",");
            for (String part : parts) {
                sb.append(part.trim().toUpperCase());
                sb.append(";");
            }
        } catch (Exception e) {
            return "ERROR: " + e.getMessage();
        }
        return sb.toString();
    }

    // Test: nested try-catch with ArithmeticException
    public static int nestedExceptions(int a, int b) {
        int result = 0;
        try {
            result = a / b;
            try {
                int[] arr = {a, b, result};
                result = arr[0] + arr[1] + arr[2];
            } catch (Exception e) {
                System.out.println("Inner: " + e.getMessage());
                result = -2;
            }
        } catch (ArithmeticException e) {
            System.out.println("ArithDiv: " + e.getMessage());
            result = -1;
        }
        return result;
    }

    public static void main(String[] args) {
        // Test 1: Normal path
        int[] data1 = {10, 20, 30};
        System.out.println("Normal sum: " + processArray(data1));

        // Test 2: Exception path
        int[] data2 = {1, -5, 3};
        System.out.println("Exception result: " + processArray(data2));

        // Test 3: String processing
        System.out.println("Processed: " + readAndProcess("hello, world, test"));

        // Test 4: Nested normal
        System.out.println("Nested normal: " + nestedExceptions(10, 2));

        // Test 5: Nested outer exception (div by zero)
        System.out.println("Nested div0: " + nestedExceptions(10, 0));
    }
}
