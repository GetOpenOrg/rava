import java.lang.reflect.Field;

/**
 * Field.get / Field.set 的 boolean 实例字段分支（ListFields 揭出）：读 true/false、
 * 写后读回、非 Boolean 值 → IllegalArgumentException、null 接收者 → NPE、
 * 同类私有字段无需 setAccessible。
 */
public class TestFieldBoolean {
    private boolean flagA = true;
    public boolean flagB;

    public static void main(String[] args) throws Exception {
        TestFieldBoolean o = new TestFieldBoolean();
        Field a = TestFieldBoolean.class.getDeclaredField("flagA");
        Field b = TestFieldBoolean.class.getDeclaredField("flagB");
        System.out.println("a=" + a.get(o) + " b=" + b.get(o));

        a.set(o, false);
        b.set(o, Boolean.TRUE);
        System.out.println("after set a=" + a.get(o) + " b=" + b.get(o) + " direct=" + o.flagA + "," + o.flagB);

        try {
            b.set(o, 1);
            System.out.println("no exception?");
        } catch (IllegalArgumentException e) {
            System.out.println("IAE on int value");
        }
        try {
            a.get(null);
            System.out.println("no exception?");
        } catch (NullPointerException e) {
            System.out.println("NPE on null receiver");
        }
        System.out.println("final a=" + a.get(o) + " b=" + b.get(o));
    }
}
