import java.lang.reflect.Field;

/**
 * setAccessible 边界：自家类私有可开、JDK 内部（java.base 未开放）抛
 * InaccessibleObjectException、static final 写拒绝——fastjson/gson/hibernate
 * 撞 JDK 强封装的同一位置，翻译侧须复现同款形态。
 */
public class TestSetAccessibleBoundary {

    private String mine = "ok";
    private static final int LOCKED = 1;

    public static void main(String[] args) throws Exception {
        // 自家私有：setAccessible 后可读
        TestSetAccessibleBoundary t = new TestSetAccessibleBoundary();
        Field f = TestSetAccessibleBoundary.class.getDeclaredField("mine");
        f.setAccessible(true);
        System.out.println("mine=" + f.get(t));
        f.set(t, "changed");
        System.out.println("mine-set=" + f.get(t));
        System.out.println("accessible-flag=" + f.trySetAccessible());

        // JDK 内部私有（java.base 未开放）→ InaccessibleObjectException
        try {
            Field value = String.class.getDeclaredField("value");
            value.setAccessible(true);
            System.out.println("jdk-open=unexpected");
        } catch (java.lang.reflect.InaccessibleObjectException e) {
            System.out.println("jdk-open-ex=" + e.getClass().getSimpleName());
        }

        // static final：setAccessible 成功但 Field.set 拒绝
        Field locked = TestSetAccessibleBoundary.class.getDeclaredField("LOCKED");
        locked.setAccessible(true);
        System.out.println("locked-read=" + locked.get(null));
        try {
            locked.set(null, 2);
        } catch (IllegalAccessException e) {
            System.out.println("locked-set-ex=" + e.getClass().getSimpleName());
        }
        System.out.println("locked-still=" + LOCKED);

        // setAccessible(false) 后再读私有 → IllegalAccessException（回归关闭语义）
        f.setAccessible(false);
        try {
            f.get(t);
        } catch (IllegalAccessException e) {
            System.out.println("revoke-ex=" + e.getClass().getSimpleName());
        }
    }
}
