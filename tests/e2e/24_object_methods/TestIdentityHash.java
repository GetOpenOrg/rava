import java.util.HashMap;
import java.util.HashSet;
import java.util.IdentityHashMap;
import java.util.Map;
import java.util.Set;

/**
 * equiv 探针①：identity hash / 默认 Object.hashCode（compatibility.md「近似等价」行实测）。
 * 只断言 JVM 规范保证的性质（稳定性、null=0、默认 hashCode 即 identity、覆盖不影响
 * identity、身份容器语义），不打印具体哈希值（实现相关）。
 */
public class TestIdentityHash {
    static class Plain { }

    static class ValueKey {
        final int v;
        ValueKey(int v) { this.v = v; }
        @Override public boolean equals(Object o) { return o instanceof ValueKey k && k.v == v; }
        @Override public int hashCode() { return v * 31; }
    }

    public static void main(String[] args) {
        Plain a = new Plain();
        Plain b = new Plain();
        // 稳定性：同一对象多次调用结果相同
        System.out.println("stable=" + (System.identityHashCode(a) == System.identityHashCode(a)));
        System.out.println("stableDefault=" + (a.hashCode() == a.hashCode()));
        // 默认 hashCode（未覆盖）与 identityHashCode 一致
        System.out.println("defaultIsIdentity=" + (a.hashCode() == System.identityHashCode(a)));
        Object o = new Object();
        System.out.println("objectDefault=" + (o.hashCode() == System.identityHashCode(o)));
        // null → 0
        System.out.println("null=" + System.identityHashCode(null));
        // 覆盖 hashCode 不影响 identity：值相等的两个对象 hashCode 相同
        ValueKey k1 = new ValueKey(7), k2 = new ValueKey(7);
        System.out.println("overrideEq=" + (k1.hashCode() == k2.hashCode()) + "," + k1.hashCode());
        System.out.println("overrideIdentityStable=" + (System.identityHashCode(k1) == System.identityHashCode(k1)));
        // 字符串：值哈希确定，identity 与值哈希无关（不断言相等性，只断言值哈希）
        String s1 = new String("abc"), s2 = new String("abc");
        System.out.println("strHash=" + s1.hashCode() + "," + (s1.hashCode() == s2.hashCode()));
        System.out.println("strSameRef=" + (s1 == s2));
        // 默认 equals = 身份：两个不同 Plain 进 HashSet 各占一格；同一对象重复插入不增长
        Set<Plain> set = new HashSet<>();
        set.add(a); set.add(b); set.add(a);
        System.out.println("hashSetIdentity=" + set.size() + "," + set.contains(a) + "," + set.contains(new Plain()));
        // 值 equals：HashMap 按值合并
        Map<ValueKey, String> hm = new HashMap<>();
        hm.put(k1, "first"); hm.put(k2, "second");
        System.out.println("hashMapValue=" + hm.size() + "," + hm.get(new ValueKey(7)));
        // IdentityHashMap：按引用区分值相等的键
        Map<Object, String> ihm = new IdentityHashMap<>();
        ihm.put(k1, "first"); ihm.put(k2, "second"); ihm.put(k1, "again");
        System.out.println("identityMap=" + ihm.size() + "," + ihm.get(k1) + "," + ihm.get(k2)
                + "," + ihm.containsKey(new ValueKey(7)));
        Map<String, Integer> ihs = new IdentityHashMap<>();
        ihs.put(s1, 1); ihs.put(s2, 2);
        System.out.println("identityMapStr=" + ihs.size());
    }
}
