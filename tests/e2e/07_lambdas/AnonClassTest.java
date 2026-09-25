public class AnonClassTest {

    interface Greeter {
        String greet(String name);
    }

    interface Calculator {
        int compute(int a, int b);
    }

    public static void main(String[] args) {
        // Anonymous class implementing user interface
        Greeter hello = new Greeter() {
            public String greet(String name) {
                return "Hello, " + name + "!";
            }
        };
        System.out.println(hello.greet("World"));

        // Anonymous class with captured variable
        String prefix = "Hi";
        Greeter hi = new Greeter() {
            public String greet(String name) {
                return prefix + ", " + name + "!";
            }
        };
        System.out.println(hi.greet("Java"));

        // Anonymous class implementing another interface
        Calculator adder = new Calculator() {
            public int compute(int a, int b) {
                return a + b;
            }
        };
        System.out.println("3 + 4 = " + adder.compute(3, 4));

        Calculator multiplier = new Calculator() {
            public int compute(int a, int b) {
                return a * b;
            }
        };
        System.out.println("3 * 4 = " + multiplier.compute(3, 4));
    }
}
