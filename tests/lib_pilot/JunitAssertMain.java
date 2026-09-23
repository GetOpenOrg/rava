import static org.junit.Assert.assertTrue;
import static org.junit.Assert.assertFalse;
import static org.junit.Assert.assertEquals;
import static org.junit.Assert.assertArrayEquals;
import static org.junit.Assert.assertNull;
import static org.junit.Assert.assertNotNull;
import static org.junit.Assert.assertSame;
import static org.junit.Assert.assertNotSame;
import static org.junit.Assert.assertNotEquals;
import static org.junit.Assert.assertThat;
import static org.junit.Assert.fail;
import static org.hamcrest.CoreMatchers.is;

/**
 * M2 golden 用例：junit4 crate（org.junit.Assert 子集）的 bin 消费面
 * （docs/plans/2026-09-23-junit-crate-pilot.md 件 2）。
 *
 * Assert.* 全族：布尔 / 相等（含 double delta / 数组）/ 空 / 同一性 /
 * 异或（assertNotEquals）/ hamcrest assertThat 桥（junit4 crate → hamcrest
 * crate 的跨 crate 引用面）/ fail。
 * 失败路径的 AssertionError 消息（含 ComparisonFailure 的
 * "expected:<x> but was:<y>" 格式）是与 JVM 侧逐字对账的等价性核心。
 */
public class JunitAssertMain {

    static void check(String name, Runnable r) {
        try {
            r.run();
            System.out.println("PASS " + name);
        } catch (AssertionError e) {
            System.out.println("FAIL " + name + " :: " + e.getMessage());
        }
    }

    public static void main(String[] args) {
        // ── 布尔族 ───────────────────────────────────────────────────────
        check("assertTrue-pass", () -> assertTrue(true));
        check("assertTrue-fail", () -> assertTrue(false));
        check("assertTrue-msg-fail", () -> assertTrue("真值检查", false));
        check("assertFalse-pass", () -> assertFalse(false));
        check("assertFalse-fail", () -> assertFalse(true));

        // ── 相等族 ───────────────────────────────────────────────────────
        check("assertEquals-long-pass", () -> assertEquals(5L, 5L));
        check("assertEquals-long-fail", () -> assertEquals(5L, 6L));
        check("assertEquals-obj-pass", () -> assertEquals("abc", "abc"));
        check("assertEquals-obj-fail", () -> assertEquals("expected字符串", "actual字符串"));
        check("assertEquals-msg-fail", () -> assertEquals("带 reason", 1, 2));
        check("assertEquals-double-pass", () -> assertEquals(0.3, 0.1 + 0.2, 0.0001));
        check("assertEquals-double-fail", () -> assertEquals(0.5, 0.1 + 0.2, 0.0001));
        check("assertEquals-object-pass", () -> assertEquals(Integer.valueOf(7), Integer.valueOf(7)));
        check("assertNotEquals-pass", () -> assertNotEquals(1, 2));
        check("assertNotEquals-fail", () -> assertNotEquals(3, 3));

        // ── 数组族（ExactComparisonCriteria）────────────────────────────
        // assertArrayEquals 需 java/lang/reflect/Array.getLength（native 存根，
        // 反射 L1/L2——与 Class.isInstance 同边界，M3 前置）——如实归类下一层，
        // 不入本 golden。

        // ── 空值 / 同一性族 ─────────────────────────────────────────────
        check("assertNull-pass", () -> assertNull(null));
        check("assertNull-fail", () -> assertNull("not-null"));
        check("assertNotNull-pass", () -> assertNotNull(new Object()));
        Object o1 = new Object();
        Object o2 = new Object();
        check("assertSame-pass", () -> assertSame(o1, o1));
        // assertSame 失败消息含 System.identityHashCode（JVM 与翻译侧均非确定），
        // 不入 golden；同一性语义由 assertSame-pass / assertNotSame-pass 覆盖
        check("assertNotSame-pass", () -> assertNotSame(o1, o2));

        // ── assertThat 桥（junit4 → hamcrest 跨 crate 引用）────────────
        // 桥的等价性由反射无关的 is matcher 覆盖；startsWith 等 TypeSafeMatcher
        // 族与 M1 同因（ReflectiveTypeFinder 反射元数据，M3 前置）不入 golden。
        check("assertThat-pass", () -> assertThat("hello", is("hello")));
        check("assertThat-fail", () -> assertThat("hello world", is("bye")));
        check("assertThat-msg-fail", () -> assertThat("桥接失败", 1, is(2)));

        // ── fail（assertThrows 需 Class.isInstance——M1 instanceOf 用例同因，
        //    反射 L1/L2，M3 前置，如实归类下一层）───────────────────────
        check("fail-plain", () -> fail());
        check("fail-msg", () -> fail("主动失败"));

        System.out.println("DONE");
    }
}
