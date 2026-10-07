// lambda 目标为子接口、子接口覆盖超接口 default：经超接口类型调用须落到子接口的 default（极大声明）
public class TestLambdaSubIfaceDefault {
    interface Pred {
        boolean is(int c);
        default String tag() { return "Pred"; }
        default Pred union(Pred p) { return c -> is(c) || p.is(c); }
    }

    interface BmpPred extends Pred {
        default String tag() { return "BmpPred"; }
        default Pred union(Pred p) {
            mark("BmpPred.union");
            return c -> is(c) || p.is(c);
        }
    }

    interface Mid extends BmpPred {}

    interface Left extends Pred { default String tag() { return "Left"; } }

    interface Diamond extends Left, Mid { default String tag() { return "Diamond"; } }

    interface Reabstract extends Pred { String tag(); }

    static void mark(String s) { System.out.println(s); }

    static String viaPred(Pred p) { return p.tag(); }

    public static void main(String[] args) {
        BmpPred b = c -> c == 'a';
        Pred asPred = b;
        System.out.println(asPred.tag());
        Pred u = asPred.union(c -> c == 'b');
        System.out.println(u.is('a') + " " + u.is('b') + " " + u.is('c'));

        Mid m = c -> c == 'm';
        System.out.println(viaPred(m));
        System.out.println(((Pred) m).union(c -> false).is('m'));

        Diamond d = c -> true;
        System.out.println(viaPred(d) + " " + ((Left) d).tag() + " " + ((BmpPred) d).tag());

        Pred plain = c -> c > 0;
        System.out.println(plain.tag() + " " + plain.union(c -> false).is(1));

        Reabstract r = new Reabstract() {
            public boolean is(int c) { return false; }
            public String tag() { return "Reabstract"; }
        };
        System.out.println(viaPred(r));
    }
}
