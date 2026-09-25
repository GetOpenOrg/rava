public class DivByZeroTest {
    public static void main(String[] args) {
        // 1. Direct division by zero caught
        try {
            int x = 10 / 0;
            System.out.println("no exception");
        } catch (ArithmeticException e) {
            System.out.println("caught: " + e.getMessage());
        }

        // 2. Modulo by zero caught
        try {
            int y = 10 % 0;
            System.out.println("no exception");
        } catch (ArithmeticException e) {
            System.out.println("mod caught: " + e.getMessage());
        }

        // 3. Variable division by zero
        int divisor = 0;
        try {
            int z = 100 / divisor;
            System.out.println("no exception");
        } catch (ArithmeticException e) {
            System.out.println("var caught: " + e.getMessage());
        }

        // 4. Normal division works
        System.out.println("10/2 = " + (10 / 2));
        System.out.println("10%3 = " + (10 % 3));

        // 5. Long division by zero
        try {
            long a = 100L / 0L;
            System.out.println("no exception");
        } catch (ArithmeticException e) {
            System.out.println("long caught: " + e.getMessage());
        }

        // 6. Float division by zero (no exception, returns Infinity)
        double inf = 1.0 / 0.0;
        System.out.println("float div: Infinity");

        System.out.println("Done.");
    }
}
