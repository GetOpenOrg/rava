import java.lang.reflect.Method;

/**
 * 反射方法调用的实例字段写回语义（e2e 能力补齐，docs/plans/2026-10-03-e2e-capability-coverage.md）。
 *
 * JUnit Runner 的 @Before 模式零依赖同构：实例方法经 Method.invoke 写实例字段后，
 * 直接字段读与反射 getter 读必须一致。lib pilot m5（JunitCrossCrateMain）翻译侧缺陷
 * 正是「@Before 写入未生效（was null）」，本例是该语义在 java.base 上的回归网。
 */
public class TestReflectStateWriteBack {

    private String subject;
    private int count;

    public void setUp(String subject) {
        this.subject = subject;
    }

    public String getSubject() {
        return subject;
    }

    public void bump(int delta) {
        this.count += delta;
    }

    public int getCount() {
        return count;
    }

    static String statLabel(String in) {
        return "label:" + in;
    }

    public static void main(String[] args) throws Exception {
        Class<?> cls = TestReflectStateWriteBack.class;
        Method setUp = cls.getDeclaredMethod("setUp", String.class);
        Method getSubject = cls.getDeclaredMethod("getSubject");
        Method bump = cls.getDeclaredMethod("bump", int.class);
        Method getCount = cls.getDeclaredMethod("getCount");

        TestReflectStateWriteBack t = new TestReflectStateWriteBack();
        System.out.println("initial=" + getSubject.invoke(t));
        setUp.invoke(t, "rava");
        System.out.println("direct-subject=" + t.subject);
        System.out.println("getter-subject=" + getSubject.invoke(t));

        // 装箱参数经 invoke 写 int 字段；void 返回值通道为 null
        Object r1 = bump.invoke(t, Integer.valueOf(40));
        Object r2 = bump.invoke(t, 2);
        System.out.println("count=" + t.count + " via-get=" + getCount.invoke(t)
                + " void-null=" + (r1 == null) + "," + (r2 == null));

        // 双实例隔离：JUnit 每 @Test 新实例的语义（写互不串扰）
        TestReflectStateWriteBack a = new TestReflectStateWriteBack();
        TestReflectStateWriteBack b = new TestReflectStateWriteBack();
        setUp.invoke(a, "one");
        setUp.invoke(b, "two");
        bump.invoke(a, 1);
        System.out.println("a=" + getSubject.invoke(a) + "/" + getCount.invoke(a));
        System.out.println("b=" + getSubject.invoke(b) + "/" + getCount.invoke(b));

        // 静态方法：null 接收者合法
        Method stat = cls.getDeclaredMethod("statLabel", String.class);
        System.out.println("static=" + stat.invoke(null, "ok"));
    }
}
