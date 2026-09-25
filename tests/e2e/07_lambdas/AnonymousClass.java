public class AnonymousClass {
    interface Transformer {
        int transform(int x);
    }

    interface Formatter {
        String format(String s);
    }

    static int apply(Transformer t, int x) {
        return t.transform(x);
    }

    public static void main(String[] args) {
        // Anonymous class implementing an interface
        Transformer doubler = new Transformer() {
            @Override
            public int transform(int x) {
                return x * 2;
            }
        };

        Transformer squarer = new Transformer() {
            @Override
            public int transform(int x) {
                return x * x;
            }
        };

        System.out.println(doubler.transform(5));   // 10
        System.out.println(squarer.transform(4));   // 16
        System.out.println(apply(doubler, 7));      // 14

        // Anonymous class with a captured value
        final int base = 100;
        Transformer adder = new Transformer() {
            @Override
            public int transform(int x) {
                return x + base;
            }
        };

        System.out.println(adder.transform(42));    // 142

        // Anonymous class for Formatter
        Formatter upper = new Formatter() {
            @Override
            public String format(String s) {
                return s.toUpperCase();
            }
        };

        System.out.println(upper.format("hello"));  // HELLO
    }
}
