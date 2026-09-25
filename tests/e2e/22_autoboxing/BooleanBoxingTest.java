public class BooleanBoxingTest {
    public static void main(String[] args) {
        // Boolean.valueOf boxing
        Boolean t = Boolean.valueOf(true);
        Boolean f = Boolean.valueOf(false);
        System.out.println(t);
        System.out.println(f);

        // booleanValue unboxing
        boolean bt = t.booleanValue();
        boolean bf = f.booleanValue();
        System.out.println(bt);
        System.out.println(bf);

        // Conditional usage
        if (t.booleanValue()) {
            System.out.println("t is true");
        }

        // valueOf from variable
        boolean flag = 3 > 2;
        Boolean bFlag = Boolean.valueOf(flag);
        System.out.println(bFlag.booleanValue());
    }
}
