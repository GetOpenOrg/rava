import java.util.Collections;
import java.util.HashMap;
import java.util.Map;
import java.util.Set;

/**
 * Collections 工厂族（方法级实测：emptyMap 34 jar / emptySet 31 / singleton 20 /
 * unmodifiableCollection 21 / synchronizedMap 18 / singletonMap 13，此前零覆盖）：
 * 空集不可变边界、singleton 语义、视图只读异常、synchronized 包装。
 */
public class TestCollectionsFactories {

    public static void main(String[] args) {
        // 空集工厂：单例性 + 不可变
        Map<Object, Object> em = Collections.emptyMap();
        System.out.println("empty-same=" + (em == Collections.emptyMap()));
        System.out.println("empty-eq=" + em.equals(new HashMap<>()));
        System.out.println("empty-size=" + em.size() + set().size() + list().size());
        try {
            em.put("k", "v");
        } catch (UnsupportedOperationException e) {
            System.out.println("empty-map-ex=" + e.getClass().getSimpleName());
        }
        try {
            set().add("x");
        } catch (UnsupportedOperationException e) {
            System.out.println("empty-set-ex=" + e.getClass().getSimpleName());
        }

        // singleton / singletonMap
        Set<String> one = Collections.singleton("only");
        System.out.println("singleton-size=" + one.size() + " contains=" + one.contains("only")
                + " miss=" + one.contains("other"));
        Map<String, Integer> sm = Collections.singletonMap("k", 1);
        System.out.println("singletonMap=" + sm);
        try {
            one.remove("only");
        } catch (UnsupportedOperationException e) {
            System.out.println("singleton-remove-ex=" + e.getClass().getSimpleName());
        }

        // unmodifiableCollection 视图：读通写拒
        java.util.Collection<String> view = Collections.unmodifiableCollection(
                java.util.Arrays.asList("a", "b"));
        System.out.println("view=" + view.size() + " iter-has-a=" + view.contains("a"));
        try {
            view.add("c");
        } catch (UnsupportedOperationException e) {
            System.out.println("unmod-ex=" + e.getClass().getSimpleName());
        }

        // synchronizedMap：语义等价（含 null 键值形态）
        Map<String, String> sync = Collections.synchronizedMap(new HashMap<>());
        sync.put("a", "1");
        sync.put(null, "n");
        System.out.println("sync=" + sync.get("a") + "/" + sync.get(null) + " size=" + sync.size());
        sync.remove(null);
        System.out.println("after-remove=" + sync.size());

        // addAll 便捷族
        java.util.List<String> dst = new java.util.ArrayList<>();
        Collections.addAll(dst, "x", "y", "z");
        System.out.println("addall=" + dst);
    }

    static Set<Object> set() {
        return Collections.emptySet();
    }

    static java.util.List<Object> list() {
        return Collections.emptyList();
    }
}
