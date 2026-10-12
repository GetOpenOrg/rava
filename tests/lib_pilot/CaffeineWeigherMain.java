
import com.github.benmanes.caffeine.cache.Cache;
import com.github.benmanes.caffeine.cache.Caffeine;
import com.github.benmanes.caffeine.cache.RemovalCause;

/**
 * Caffeine pilot B4：maximumWeight 淘汰与 removal 监听（单线程确定性，W5）。
 */
public class CaffeineWeigherMain {

    static final java.util.List<String> EVICTED =
            java.util.Collections.synchronizedList(new java.util.ArrayList<>());

    static void check(String name, Object expected, Object actual) {
        boolean ok = String.valueOf(expected).equals(String.valueOf(actual));
        System.out.println((ok ? "PASS " : "FAIL ") + name
                + (ok ? "" : " :: expected=<" + expected + "> actual=<" + actual + ">"));
    }

    public static void main(String[] args) {
        // 每条按 value 长度计权，总权重 6；超过即淘汰（LRU 面）
        Cache<String, String> cache = Caffeine.newBuilder()
                .maximumWeight(6)
                .weigher((String k, String v) -> v.length())
                .removalListener((String k, String v, RemovalCause cause) -> {
                    if (cause == RemovalCause.SIZE) {
                        EVICTED.add(k);
                    }
                })
                .build();

        cache.put("a", "111");  // 权 3
        cache.put("b", "22");   // 权 2（累计 5）
        check("both-present", "true",
                String.valueOf(cache.getIfPresent("a") != null
                        && cache.getIfPresent("b") != null));

        cache.put("c", "9999"); // 权 4 → 超 6，触发按权重淘汰
        // 淘汰发生后（可能异步，手动触发维护确保同步可见）
        cache.cleanUp();
        var present = new java.util.TreeSet<String>();
        cache.asMap().keySet().forEach(present::add);
        System.out.println("  present after evict: " + present + " evicted(size): " + EVICTED);

        check("evicted-new-entry", "[c]", EVICTED.toString());

        // invalidate 的 removal cause 是 EXPLICIT，不进 EVICTED（SIZE 专属集合）
        cache.invalidate("a");
        cache.cleanUp();
        check("explicit-not-size", "true",
                String.valueOf(EVICTED.equals(java.util.List.of("c"))));

        // policy 面：权重描述
        var policy = cache.policy().eviction().orElseThrow();
        check("maximum-weight", 6L, policy.getMaximum());
        System.out.println("DONE CaffeineWeigherMain");
    }
}
