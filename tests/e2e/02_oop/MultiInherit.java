public class MultiInherit {
    static class A {
        public int foo() { return 1; }
        public String bar() { return "A_bar"; }
    }

    static class B extends A {
        // Overrides A.foo
        public int foo() { return 2; }
        // Inherits A.bar
    }

    static class C extends B {
        // Inherits B.foo (B's override of A.foo)
        // Inherits A.bar via B
    }

    public static void main(String[] args) {
        C c = new C();
        System.out.println(c.foo());   // 2 — B's override via C delegation
        System.out.println(c.bar());   // A_bar — A's method via B→C delegation chain

        A a = c;
        System.out.println(a.foo());   // 2 — virtual dispatch: C→B.foo
        System.out.println(a.bar());   // A_bar — virtual dispatch: C→B→A.bar
    }
}
