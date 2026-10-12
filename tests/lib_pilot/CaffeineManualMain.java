
import com.github.benmanes.caffeine.cache.Cache;
import com.github.benmanes.caffeine.cache.Caffeine;
import com.github.benmanes.caffeine.cache.LoadingCache;

/**
 * Caffeine pilot B3：手动加载面 + 命中统计（单线程确定性，W5）。
 */
public class CaffeineManualMain {

    static void check(String name, Object expected, Object actual) {
        boolean ok = String.valueOf(expected).equals(String.valueOf(actual));
        System.out.println((ok ? "PASS " : "FAIL ") + name
                + (ok ? "" : " :: expected=<" + expected + "> actual=<" + actual + ">"));
    }

    public static void main(String[] args) {
        java.util.concurrent.atomic.AtomicInteger loads = new java.util.concurrent.atomic.AtomicInteger();

        LoadingCache<String, String> cache = Caffeine.newBuilder()
                .maximumSize(10)
                .recordStats()
                .build(key -> {
                    loads.incrementAndGet();
                    return "v-" + key;
                });

        // get：首次加载、再次命中
        check("first-get", "v-a", cache.get("a"));
        check("second-get", "v-a", cache.get("a"));
        check("other-key", "v-b", cache.get("b"));
        check("load-count", 2, loads.get());

        // getAll：批量（缺的合并加载）
        var all = cache.getAll(java.util.List.of("a", "c"));
        check("getall", "{a=v-a, c=v-c}",
                new java.util.TreeMap<>(all).toString());
        check("load-count-after", 3, loads.get());

        // invalidate 与命中统计
        cache.invalidate("a");
        check("after-invalidate", "v-a", cache.get("a")); // 重新加载
        check("load-count-final", 4, loads.get());

        var stats = cache.stats();
        check("hit-count", 2, stats.hitCount());
        check("miss-count", 4, stats.missCount());
        check("size", 3, cache.estimatedSize());

        // 手动 Cache 面：put/getIfPresent
        Cache<String, String> manual = Caffeine.newBuilder().build();
        manual.put("x", "1");
        check("manual-present", "1", manual.getIfPresent("x"));
        check("manual-absent", null, manual.getIfPresent("y"));
        System.out.println("DONE CaffeineManualMain");
    }
}
