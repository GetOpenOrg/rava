import org.hamcrest.BaseMatcher;
import org.hamcrest.Description;
import org.hamcrest.MatcherAssert;

import static org.hamcrest.CoreMatchers.allOf;
import static org.hamcrest.CoreMatchers.anyOf;
import static org.hamcrest.CoreMatchers.equalTo;
import static org.hamcrest.CoreMatchers.is;
import static org.hamcrest.CoreMatchers.not;
import static org.hamcrest.CoreMatchers.notNullValue;
import static org.hamcrest.CoreMatchers.nullValue;
import static org.hamcrest.MatcherAssert.assertThat;

/**
 * 63_junit 首批：junit4 crate → hamcrest crate 桥（assertThat + CoreMatchers 链
 * + 自定义 BaseMatcher 的消息拼接）。期望生成等 W0-3。
 */
public class TestJunitHamcrestBridge {

    /** 自定义匹配器：偶数判定（跨 crate 回调面，m5 同族）。 */
    static class IsEven extends BaseMatcher<Integer> {
        @Override
        public boolean matches(Object item) {
            return item instanceof Integer && ((Integer) item) % 2 == 0;
        }

        @Override
        public void describeTo(Description description) {
            description.appendText("an even number");
        }
    }

    static void check(String name, Runnable r) {
        try {
            r.run();
            System.out.println("PASS " + name);
        } catch (AssertionError e) {
            System.out.println("FAIL " + name + " :: " + e.getMessage());
        }
    }

    public static void main(String[] args) {
        check("is-eq", () -> assertThat("abc", is("abc")));
        check("is-eq-fail", () -> assertThat("abc", is("xyz")));
        check("equalTo-fail", () -> assertThat(7, equalTo(8)));
        check("not", () -> assertThat("a", not("b")));
        check("not-fail", () -> assertThat("a", not("a")));
        check("nullValue", () -> assertThat(null, nullValue()));
        check("nullValue-fail", () -> assertThat("x", nullValue()));
        check("notNullValue", () -> assertThat("x", notNullValue()));
        check("allOf", () -> assertThat("hello", allOf(is("hello"), notNullValue())));
        check("allOf-fail", () -> assertThat("hello", allOf(is("hello"), is("world"))));
        check("anyOf", () -> assertThat("hello", anyOf(is("nope"), is("hello"))));
        check("anyOf-fail", () -> assertThat("hello", anyOf(is("nope"), is("nah"))));
        check("custom-even", () -> assertThat(4, new IsEven()));
        check("custom-even-fail", () -> assertThat(5, new IsEven()));
        check("reason-three-arg", () -> assertThat("数值检查", 6, is(7)));
    }
}
