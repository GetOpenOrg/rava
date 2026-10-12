
import java.util.List;

import static org.assertj.core.api.Assertions.assertThat;

/**
 * AssertJ pilot B8：fluent 断言链与失败消息文本（W5）。
 * 失败消息是断言库的核心输出——捕获后逐字断言。
 */
public class AssertJChainMain {

    static void check(String name, Runnable r) {
        try {
            r.run();
            System.out.println("PASS " + name);
        } catch (AssertionError e) {
            System.out.println("FAIL " + name + " :: " + singleLine(e.getMessage()));
        }
    }

    static String singleLine(String msg) {
        return msg == null ? "null" : msg.replace("\n", " | ");
    }

    /** 捕获断言失败消息的辅助。 */
    static String capture(Runnable r) {
        try {
            r.run();
            return "<no-failure>";
        } catch (AssertionError e) {
            return singleLine(e.getMessage());
        }
    }

    public static void main(String[] args) {
        check("string-chain", () -> assertThat("Hello AssertJ")
                .startsWith("Hello")
                .endsWith("AssertJ")
                .hasSize(13)
                .contains("Assert")
                .doesNotContain("xyz")
                .matches("[A-Za-z ]+"));

        check("number-chain", () -> assertThat(42)
                .isPositive()
                .isEven()
                .isBetween(40, 44)
                .isLessThan(100)
                .isNotEqualTo(41));

        check("list-chain", () -> assertThat(List.of("a", "b", "c"))
                .hasSize(3)
                .containsExactly("a", "b", "c")
                .contains("b")
                .doesNotContain("z")
                .first().isEqualTo("a"));

        check("map-chain", () -> assertThat(java.util.Map.of("k1", 1, "k2", 2))
                .hasSize(2)
                .containsKeys("k1", "k2")
                .containsEntry("k1", 1)
                .doesNotContainKey("k3"));

        check("extracting", () ->
                assertThat(List.of(new P("甲", 18), new P("乙", 20)))
                        .extracting(P::name)
                        .containsExactly("甲", "乙"));

        // 失败消息逐字断言（描述断言库的输出契约）
        System.out.println("  msg-eq: " + capture(() -> assertThat(1).isEqualTo(2)));
        System.out.println("  msg-size: " + capture(() -> assertThat("abc").hasSize(5)));
        System.out.println("  msg-contains: " + capture(() -> assertThat("abc").contains("z")));
        System.out.println("DONE AssertJChainMain");
    }

    record P(String name, int age) {
    }
}
