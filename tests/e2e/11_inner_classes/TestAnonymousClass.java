public class TestAnonymousClass {
    interface Greeter {
        String greet(String name);
    }

    interface Transformer {
        int transform(int x);
    }

    public static void main(String[] args) {
        Greeter formal = new Greeter() {
            @Override
            public String greet(String name) {
                return "Hello, " + name + "!";
            }
        };

        Greeter casual = new Greeter() {
            @Override
            public String greet(String name) {
                return "Hey " + name;
            }
        };

        System.out.println(formal.greet("Alice"));
        System.out.println(casual.greet("Bob"));

        Transformer doubler = new Transformer() {
            @Override
            public int transform(int x) {
                return x * 2;
            }
        };

        System.out.println(doubler.transform(21));
    }
}
