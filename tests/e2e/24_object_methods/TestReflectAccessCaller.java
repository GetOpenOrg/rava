// FS-R4：反射访问检查按调用方类判定（Reflection.verifyMemberAccess）：不同顶层类之间的
// private 成员 → IllegalAccessException；嵌套类（nestmate）/ 同包非 private 可达；
// setAccessible(true) 后可达。
import java.lang.reflect.*;

class Vault {
    private int secret = 1;
    int pkgField = 2;
    protected int protField = 3;
    public int pubField = 4;
    private static String hide() { return "hidden"; }
    private Vault() { }
    Vault(int x) { secret = x; }
}

public class TestReflectAccessCaller {
    static class Nested { private int inner = 5; private String tag() { return "nested-tag"; } }

    static String tryGet(Field f, Object o) {
        try { return String.valueOf(f.get(o)); }
        catch (IllegalAccessException e) { return "IAE"; }
    }

    public static void main(String[] args) throws Exception {
        Vault v = new Vault(1);
        for (String n : new String[]{"secret", "pkgField", "protField", "pubField"}) {
            System.out.println(n + " -> " + tryGet(Vault.class.getDeclaredField(n), v));
        }
        Method hide = Vault.class.getDeclaredMethod("hide");
        try {
            System.out.println("hide " + hide.invoke(null));
        } catch (IllegalAccessException e) {
            System.out.println("hide -> IAE");
        }
        hide.setAccessible(true);
        System.out.println("hide accessible " + hide.invoke(null));
        Field s = Vault.class.getDeclaredField("secret");
        s.setAccessible(true);
        s.set(v, 42);
        System.out.println("secret after setAccessible " + s.get(v));

        Nested nd = new Nested();
        System.out.println("nestmate field " + tryGet(Nested.class.getDeclaredField("inner"), nd));
        System.out.println("nestmate method " + Nested.class.getDeclaredMethod("tag").invoke(nd));
        try {
            Field w = Vault.class.getDeclaredField("secret");
            w.set(v, 7);
            System.out.println("set private no IAE");
        } catch (IllegalAccessException e) {
            System.out.println("set private -> IAE");
        }
    }
}
