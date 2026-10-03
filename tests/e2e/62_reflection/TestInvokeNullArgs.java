import java.lang.reflect.InvocationTargetException;
import java.lang.reflect.Method;

/**
 * Method.invoke 的参数数组语义与 InvocationTargetException 解包（e2e 能力补齐）。
 *
 * null 参数数组 ≡ 空数组、形参收 null 元素、业务异常经 ITE 包装后 getCause 解包、
 * 实参数量/类型不符直抛 IAE——均为 junit m3 阶段修过的语义，此处固化进回归网。
 */
public class TestInvokeNullArgs {

    public String noArg() {
        return "no-arg";
    }

    public static String greet(String who) {
        return "hi:" + who;
    }

    public int explode() {
        throw new IllegalStateException("boom-ihe");
    }

    public int npeBody() {
        String s = null;
        return s.length();
    }

    public static void main(String[] args) throws Exception {
        Method noArg = TestInvokeNullArgs.class.getDeclaredMethod("noArg");
        Method greet = TestInvokeNullArgs.class.getDeclaredMethod("greet", String.class);
        Method explode = TestInvokeNullArgs.class.getDeclaredMethod("explode");
        Method npeBody = TestInvokeNullArgs.class.getDeclaredMethod("npeBody");

        // null 参数数组 ≡ 空数组
        Object r1 = noArg.invoke(new TestInvokeNullArgs(), (Object[]) null);
        Object r2 = noArg.invoke(new TestInvokeNullArgs(), new Object[0]);
        System.out.println("null-arr=" + r1 + " empty-arr=" + r2);

        // 数组内 null 元素 → 形参收到 null；静态方法 null 接收者合法
        System.out.println("null-arg=" + greet.invoke(null, new Object[] { null }));
        System.out.println("static=" + greet.invoke(null, "rava"));

        // ITE 解包：getCause 才是业务异常
        try {
            explode.invoke(new TestInvokeNullArgs());
        } catch (InvocationTargetException e) {
            System.out.println("ite-cause=" + e.getCause().getClass().getSimpleName()
                    + " msg=" + e.getCause().getMessage());
        }
        try {
            npeBody.invoke(new TestInvokeNullArgs());
        } catch (InvocationTargetException e) {
            System.out.println("npe-cause=" + e.getCause().getClass().getSimpleName());
        }

        // 实参数量不符 / 类型不符 → IllegalArgumentException（不包装进 ITE）
        try {
            noArg.invoke(new TestInvokeNullArgs(), "extra");
        } catch (IllegalArgumentException e) {
            System.out.println("argc-iae=" + e.getClass().getSimpleName());
        }
        try {
            greet.invoke(null, Integer.valueOf(1));
        } catch (IllegalArgumentException e) {
            System.out.println("type-iae=" + e.getClass().getSimpleName());
        }
    }
}
