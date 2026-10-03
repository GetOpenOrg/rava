import java.util.concurrent.ConcurrentHashMap;

/**
 * ConcurrentHashMap 键集族（方法级实测：newKeySet 8 jar，此前零覆盖）：
 * newKeySet 可增删、map.keySet() 视图只删不增（add 拒绝）。
 */
public class TestConcurrentKeySetFaces {

    public static void main(String[] args) {
        ConcurrentHashMap.KeySetView<String, Boolean> ks = ConcurrentHashMap.newKeySet();
        ks.add("a");
        ks.add("b");
        ks.add("a");                          // 集合语义去重
        System.out.println("size=" + ks.size() + " contains=" + ks.contains("a"));
        ks.remove("b");
        System.out.println("after-remove=" + ks.size());
        ks.addAll(java.util.Arrays.asList("c", "d"));
        System.out.println("after-addall=" + ks.size());

        // map.keySet() 视图：remove 生效、add 拒绝
        ConcurrentHashMap<String, Integer> m = new ConcurrentHashMap<>();
        m.put("k1", 1);
        m.put("k2", 2);
        var view = m.keySet();
        view.remove("k1");
        System.out.println("view-remove-works=" + (m.size() == 1));
        try {
            view.add("k3");
            System.out.println("view-add=unexpected");
        } catch (UnsupportedOperationException e) {
            System.out.println("view-add-ex=" + e.getClass().getSimpleName());
        }

        // 值原子族回顾点（computeIfAbsent 已有他例）：putIfAbsent 返回旧值
        System.out.println("absent-old-null=" + (m.putIfAbsent("k9", 9) == null)
                + " present-old=" + m.putIfAbsent("k9", 99));
        System.out.println("mapping-count=" + m.mappingCount());

        // keySet 的流与遍历（弱一致迭代包含全部元素）
        int seen = 0;
        for (String k : m.keySet()) {
            seen++;
        }
        System.out.println("iter-count=" + seen);
    }
}
