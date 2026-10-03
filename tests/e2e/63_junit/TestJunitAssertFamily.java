import static org.junit.Assert.assertArrayEquals;
import static org.junit.Assert.assertEquals;
import static org.junit.Assert.assertFalse;
import static org.junit.Assert.assertNotEquals;
import static org.junit.Assert.assertNotNull;
import static org.junit.Assert.assertNotSame;
import static org.junit.Assert.assertNull;
import static org.junit.Assert.assertSame;
import static org.junit.Assert.assertTrue;
import static org.junit.Assert.fail;

/**
 * 63_junit 首批（junit 计划步骤 A/B 断言面）：Assert 静态族与失败消息格式。
 * 期望输出生成等 W0-3（form.toml 接线，--pilot-libs classpath）。
 */
public class TestJunitAssertFamily {

    static void check(String name, Runnable r) {
        try {
            r.run();
            System.out.println("PASS " + name);
        } catch (AssertionError e) {
            System.out.println("FAIL " + name + " :: " + e.getMessage());
        }
    }

    public static void main(String[] args) {
        check("assertTrue", () -> assertTrue(true));
        check("assertTrue-fail", () -> assertTrue(false));
        check("assertTrue-msg", () -> assertTrue("真值检查", false));
        check("assertFalse", () -> assertFalse(false));
        check("assertEquals-int", () -> assertEquals(4, 2 + 2));
        check("assertEquals-str-fail", () -> assertEquals("expected-x", "actual-y"));
        check("assertEquals-double-delta", () -> assertEquals(0.3, 0.1 + 0.2, 1e-9));
        check("assertEquals-double-delta-fail", () -> assertEquals(0.3, 0.1 + 0.25, 1e-9));
        check("assertEquals-array", () -> assertArrayEquals(new int[] { 1, 2, 3 }, new int[] { 1, 2, 3 }));
        check("assertEquals-array-fail", () -> assertArrayEquals(new String[] { "a" }, new String[] { "b" }));
        check("assertNull", () -> assertNull(null));
        check("assertNull-fail", () -> assertNull("obj"));
        check("assertNotNull", () -> assertNotNull(new Object()));
        Object o = new Object();
        check("assertSame", () -> assertSame(o, o));
        check("assertSame-fail", () -> assertSame(new Object(), new Object()));
        check("assertNotSame", () -> assertNotSame(new Object(), new Object()));
        check("assertNotEquals", () -> assertNotEquals(1, 2));
        check("assertNotEquals-fail", () -> assertNotEquals(1, 1));
        check("fail", () -> fail());
        check("fail-msg", () -> fail("手动失败"));
    }
}
