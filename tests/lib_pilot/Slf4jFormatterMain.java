
import org.slf4j.helpers.MessageFormatter;

/**
 * slf4j-api pilot A2：MessageFormatter 占位符语义（纯计算，无绑定参与）。
 * 覆盖 {} 替换、转义、参数不足/过剩、null、数组与 Throwable 尾参。
 */
public class Slf4jFormatterMain {

    static String fmt(String pattern, Object... args) {
        return MessageFormatter.arrayFormat(pattern, args).getMessage();
    }

    static void eq(String name, String expected, String actual) {
        System.out.println((expected.equals(actual) ? "PASS " : "FAIL ") + name
                + (expected.equals(actual) ? "" : " :: expected=<" + expected + "> actual=<" + actual + ">"));
    }

    public static void main(String[] args) {
        eq("basic", "a=1, b=2", fmt("a={}, b={}", 1, 2));
        eq("short-args", "x=1, y={}", fmt("x={}, y={}", 1));            // 参数不足：占位符原样保留
        eq("escaped", "\\{1}/\\{2}", fmt("\\{{}}/\\{{}}", 1, 2));        // \{ 转义后仍消费参数（实测语义）
        eq("null-arg", "value=null", fmt("value={}", new Object[]{null}));
        eq("array-as-single", "arr=[A, B]", fmt("arr={}", (Object) new String[]{"A", "B"}));
        eq("no-placeholder", "plain text", fmt("plain text", 1, 2));
        eq("many-args", "1 2 3 tail", fmt("{} {} {} tail", 1, "2", 3, "多余1", "多余2"));

        // Throwable 尾参：不进消息，由 getThrowable 返回
        var tuple = MessageFormatter.arrayFormat("boom",
                new Object[]{"x", new IllegalStateException("E")});
        eq("throwable-appended", "boom", tuple.getMessage());
        eq("throwable-returned", "java.lang.IllegalStateException: E",
                String.valueOf(tuple.getThrowable()));
        System.out.println("DONE Slf4jFormatterMain");
    }
}
