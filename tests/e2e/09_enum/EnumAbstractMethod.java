public class EnumAbstractMethod {
    enum Operation {
        PLUS("+")  { public int apply(int x, int y) { return x + y; } },
        MINUS("-") { public int apply(int x, int y) { return x - y; } },
        TIMES("*") { public int apply(int x, int y) { return x * y; } };

        private final String symbol;
        Operation(String symbol) { this.symbol = symbol; }

        public abstract int apply(int x, int y);

        @Override
        public String toString() { return symbol; }
    }

    public static void main(String[] args) {
        int x = 10, y = 3;
        for (Operation op : Operation.values()) {
            System.out.printf("%d %s %d = %d%n", x, op, y, op.apply(x, y));
        }
    }
}
