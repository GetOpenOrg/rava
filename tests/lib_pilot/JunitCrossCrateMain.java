import org.hamcrest.BaseMatcher;
import org.hamcrest.Description;
import org.hamcrest.Matcher;
import org.hamcrest.MatcherAssert;
import org.hamcrest.TypeSafeMatcher;
import org.junit.Before;
import org.junit.Test;
import org.junit.runner.JUnitCore;
import org.junit.runner.Result;
import org.junit.runner.notification.Failure;

/**
 * M5 golden 用例：跨 crate 分派链（docs/plans/2026-09-23-junit-crate-pilot.md M5）。
 *
 * 用户 crate 的类继承 / 实现 lib crate 的抽象类型，由 lib crate 代码回调：
 *   - BaseMatcher 子类（matches / describeTo 覆盖）→ hamcrest 的 MatcherAssert
 *     经 Matcher 接口虚分派回调用户实现，失败消息由 lib 的 StringDescription
 *     拼接用户 describeTo 输出；
 *   - TypeSafeMatcher 子类（matchesSafely / describeMismatchSafely 覆盖）→
 *     lib 基类的模板方法（matches → matchesSafely）跨 crate 回调；
 *   - 匿名 Matcher 实现（匿名类位于 user crate）；
 *   - JUnit Runner（junit4 crate）→ 用户测试类（@Before + @Test）→ hamcrest
 *     断言（hamcrest crate）→ 用户 Matcher（user crate）：三 crate 往返链。
 */
public class JunitCrossCrateMain {

    /** BaseMatcher 子类：偶数判定。 */
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

    /** TypeSafeMatcher 子类：前缀判定（模板方法跨 crate 回调）。 */
    static class HasPrefix extends TypeSafeMatcher<String> {
        private final String prefix;

        HasPrefix(String prefix) {
            this.prefix = prefix;
        }

        @Override
        protected boolean matchesSafely(String item) {
            return item.startsWith(prefix);
        }

        @Override
        public void describeTo(Description description) {
            description.appendText("a string starting with ").appendValue(prefix);
        }

        @Override
        protected void describeMismatchSafely(String item, Description mismatch) {
            mismatch.appendText("started with ").appendValue(item.substring(0, 1));
        }
    }

    static Matcher<String> ofLength(final int n) {
        return new BaseMatcher<String>() {
            @Override
            public boolean matches(Object item) {
                return item instanceof String && ((String) item).length() == n;
            }

            @Override
            public void describeTo(Description description) {
                description.appendText("length ").appendValue(n);
            }
        };
    }

    static void check(String name, Runnable r) {
        try {
            r.run();
            System.out.println("PASS " + name);
        } catch (AssertionError e) {
            System.out.println("FAIL " + name + " :: " + e.getMessage());
        }
    }

    public static class RunnerSample {
        private String subject;

        @Before
        public void setUp() {
            subject = "java-rta";
        }

        @Test
        public void prefixOk() {
            MatcherAssert.assertThat(subject, new HasPrefix("java"));
        }

        @Test
        public void lengthBad() {
            MatcherAssert.assertThat(subject, ofLength(3));
        }
    }

    public static void main(String[] args) {
        check("even-pass", () -> MatcherAssert.assertThat(4, new IsEven()));
        check("even-fail", () -> MatcherAssert.assertThat(5, new IsEven()));
        check("prefix-pass", () -> MatcherAssert.assertThat("rust", new HasPrefix("ru")));
        check("prefix-fail", () -> MatcherAssert.assertThat("java", new HasPrefix("ru")));
        check("length-pass", () -> MatcherAssert.assertThat("abc", ofLength(3)));
        check("length-fail", () -> MatcherAssert.assertThat("abcd", ofLength(3)));

        Result r = JUnitCore.runClasses(RunnerSample.class);
        System.out.println("runner run=" + r.getRunCount()
                + " fail=" + r.getFailureCount()
                + " ok=" + r.wasSuccessful());
        for (Failure f : r.getFailures()) {
            System.out.println("runner failure: " + f.getMessage());
        }
    }
}
