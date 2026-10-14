
import org.springframework.util.Assert;

/**
 * spring-core pilot C3：Assert 前置校验（切片 3/4）。
 * 异常类型与消息文本是断言库的输出契约。
 */
public class SpringUtilAssertMain {

    static String capture(Runnable r) {
        try {
            r.run();
            return "<no-failure>";
        } catch (RuntimeException e) {
            return e.getClass().getSimpleName() + ": " + e.getMessage();
        }
    }

    public static void main(String[] args) {
        // 通过路径静默
        System.out.println("PASS notnull-ok "
                + "<no-failure>".equals(capture(() -> Assert.notNull("x", "must not be null"))));
        System.out.println("PASS isTrue-ok "
                + "<no-failure>".equals(capture(() -> Assert.isTrue(1 > 0, "must be true"))));
        System.out.println("PASS hasText-ok "
                + "<no-failure>".equals(capture(() -> Assert.hasText("a", "must have text"))));
        System.out.println("PASS notEmpty-ok "
                + "<no-failure>".equals(capture(() ->
                Assert.notEmpty(java.util.List.of(1), "must not be empty"))));
        System.out.println("PASS noNullElements-ok "
                + "<no-failure>".equals(capture(() ->
                Assert.noNullElements(java.util.List.of("a"), "must have no nulls"))));
        System.out.println("PASS isInstanceOf-ok "
                + "<no-failure>".equals(capture(() ->
                Assert.isInstanceOf(String.class, "s", "must be String"))));
        System.out.println("PASS isAssignable-ok "
                + "<no-failure>".equals(capture(() ->
                Assert.isAssignable(Number.class, Integer.class, "must be assignable"))));
        System.out.println("PASS state-ok "
                + "<no-failure>".equals(capture(() -> Assert.state(true, "bad state"))));

        // 失败路径：异常类型 + 消息逐字
        System.out.println("msg-notnull: " + capture(() -> Assert.notNull(null, "对象不能为空")));
        System.out.println("msg-istrue: " + capture(() -> Assert.isTrue(false, "条件不满足")));
        System.out.println("msg-hastext: " + capture(() -> Assert.hasText("  ", "文本为空")));
        System.out.println("msg-notempty: " + capture(() ->
                Assert.notEmpty(java.util.List.of(), "列表为空")));
        System.out.println("msg-nonull: " + capture(() ->
                Assert.noNullElements(java.util.Arrays.asList("a", null), "含 null")));
        System.out.println("msg-isinstance: " + capture(() ->
                Assert.isInstanceOf(String.class, 42, "类型不符")));
        System.out.println("msg-state: " + capture(() -> Assert.state(false, "状态非法")));
        // Supplier 消息变体（惰性求值）
        System.out.println("msg-supplier: " + capture(() ->
                Assert.notNull(null, () -> "supplier 消息")));
        System.out.println("DONE SpringUtilAssertMain");
    }
}
