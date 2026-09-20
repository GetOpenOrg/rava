public class TestEnumConstantBody {
    enum Op {
        ADD { int f(int a, int b) { return a + b; } },
        SUB { int f(int a, int b) { return a - b; } },
        MUL { int f(int a, int b) { return a * b; } };
        abstract int f(int a, int b);
    }

    public static void main(String[] args) {
        System.out.println("add=" + Op.ADD.f(3, 4));
        System.out.println("sub=" + Op.SUB.f(10, 2));
        System.out.println("mul=" + Op.MUL.f(5, 6));
        for (Op o : Op.values()) {
            System.out.println("op=" + o + ":" + o.f(8, 2));
        }
    }
}
