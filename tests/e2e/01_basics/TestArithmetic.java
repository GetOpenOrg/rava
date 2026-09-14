public class TestArithmetic {

    public static int add(int a, int b) {
        return a + b;
    }

    public static int multiply(int a, int b) {
        return a * b;
    }

    public static int factorial(int n) {
        int result = 1;
        for (int i = 2; i <= n; i++) {
            result = result * i;
        }
        return result;
    }

    public static void main(String[] args) {
        int x = 5;
        int y = 10;
        int sum = add(x, y);
        System.out.println(sum);

        int prod = multiply(3, 7);
        System.out.println(prod);

        int fact = factorial(6);
        System.out.println(fact);

        int rem = 17 % 5;
        System.out.println(rem);
    }
}
