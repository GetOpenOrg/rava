public class ControlFlowTest {
    public static void main(String[] args) {
        // Test 1: if/else
        int x = 10;
        if (x > 5) {
            System.out.println("x is greater than 5");
        } else {
            System.out.println("x is not greater than 5");
        }

        // Test 2: for loop
        int sum = 0;
        for (int i = 0; i < 5; i++) {
            sum += i;
        }
        System.out.println(sum); // Should print 10

        // Test 3: while loop
        int count = 3;
        while (count > 0) {
            System.out.println(count);
            count = count - 1;
        }

        // Test 4: arithmetic
        int a = 7;
        int b = 3;
        System.out.println(a + b); // 10
        System.out.println(a - b); // 4
        System.out.println(a * b); // 21
        System.out.println(a / b); // 2
        System.out.println(a % b); // 1
    }
}
