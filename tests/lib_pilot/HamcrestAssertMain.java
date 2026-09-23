import org.hamcrest.MatcherAssert;
import org.hamcrest.Matcher;
import static org.hamcrest.CoreMatchers.is;
import static org.hamcrest.CoreMatchers.not;
import static org.hamcrest.CoreMatchers.nullValue;
import static org.hamcrest.CoreMatchers.notNullValue;
import static org.hamcrest.CoreMatchers.allOf;
import static org.hamcrest.CoreMatchers.anyOf;
import static org.hamcrest.CoreMatchers.equalTo;

/**
 * M1 golden 用例：hamcrest crate 的 bin 消费面（docs/plans/2026-09-23-junit-crate-pilot.md 件 1）。
 *
 * 用例集 = 反射无关断言族（12 检查，含 5 条失败路径——AssertionError 消息经
 * StringDescription 生成，是与 JVM 侧逐字对账的等价性核心）：
 *   is / equalTo / not / nullValue / notNullValue / allOf / anyOf / 带 reason
 *   的 assertThat 三参形态 / Matcher<T> 链式变量形态。
 *
 * 【下一层定性】TypeSafeMatcher 族（containsInOrder / isEmptyString /
 * greaterThan / closeTo / hasItem / hasEntry / hasKey / hasToString 等）经
 * ReflectiveTypeFinder.findExpectedType 做类层次反射（Class.getSuperclass 查
 * build.rs 元数据表 + getGenericSuperclass），而元数据表只扫 java_runtime 与
 * user 两棵树、不含 lib crate 类——getSuperclass 对缺席项返回 default Class，
 * 发现循环永不终止。属反射 L2 扩表（跨 crate）+ L3 的 M3 前置（主机器域），
 * M1 不越界；instanceOf 用例同因（Class.isInstance native 存根）。
 */
public class HamcrestAssertMain {

    static void check(String name, Runnable r) {
        try {
            r.run();
            System.out.println("PASS " + name);
        } catch (AssertionError e) {
            System.out.println("FAIL " + name + " :: " + e.getMessage());
        }
    }

    public static void main(String[] args) {
        // ── CoreMatchers：反射无关核心族（BaseMatcher / DiagnosingMatcher 载体）──
        check("is-equals-pass", () -> MatcherAssert.assertThat("abc", is("abc")));
        check("is-equals-fail", () -> MatcherAssert.assertThat("actual", is("expected")));
        check("equalTo-pass", () -> MatcherAssert.assertThat(42, equalTo(42)));
        check("not-pass", () -> MatcherAssert.assertThat(1, not(2)));
        check("not-fail", () -> MatcherAssert.assertThat(7, not(7)));
        check("nullValue-pass", () -> MatcherAssert.assertThat(null, nullValue()));
        check("notNullValue-pass", () -> MatcherAssert.assertThat("x", notNullValue()));
        check("allOf-pass", () -> MatcherAssert.assertThat("hello",
                allOf(is("hello"), notNullValue())));
        check("allOf-fail", () -> MatcherAssert.assertThat("hello",
                allOf(is("hello"), is("world"))));
        check("anyOf-fail", () -> MatcherAssert.assertThat("zzz",
                anyOf(is("aaa"), is("bbb"))));
        check("reason-fail", () -> MatcherAssert.assertThat("reason文本", 1, is(2)));

        // ── Matcher<T> 链式变量形态（泛型载体消费）───────────────────────
        Matcher<String> startsHi = is("hi");
        check("matcher-var-pass", () -> MatcherAssert.assertThat("hi", startsHi));
        System.out.println("DONE");
    }
}
