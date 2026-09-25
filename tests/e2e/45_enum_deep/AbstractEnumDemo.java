public class AbstractEnumDemo {
    enum Operation {
        PLUS {
            public int apply(int x, int y) { return x + y; }
        },
        MINUS {
            public int apply(int x, int y) { return x - y; }
        },
        TIMES {
            public int apply(int x, int y) { return x * y; }
        },
        DIVIDE {
            public int apply(int x, int y) { return x / y; }
        };

        public abstract int apply(int x, int y);
    }

    public static void main(String[] args) {
        int x = 6;
        int y = 2;
        for (Operation op : Operation.values()) {
            System.out.println(op.name() + ": " + op.apply(x, y));
        }
    }
}
