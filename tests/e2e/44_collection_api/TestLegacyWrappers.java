import java.util.Collections;
import java.util.Enumeration;
import java.util.HashMap;
import java.util.HashSet;
import java.util.Hashtable;
import java.util.Iterator;
import java.util.Properties;
import java.util.Set;
import java.util.TreeMap;
import java.util.Vector;

/**
 * Collections 包装族第二批 + 遗留容器（方法级实测：newSetFromMap 12 jar /
 * synchronizedSet 9 / emptyIterator 9 / unmodifiableSortedMap 6 / Vector.addElement 9 /
 * trimToSize 9 / Properties.propertyNames 9 / Hashtable.keys 10 / enumeration 10，
 * 此前零覆盖）。
 */
public class TestLegacyWrappers {

    public static void main(String[] args) {
        // newSetFromMap：以 TreeMap 为背衬的 Set
        Set<String> set = Collections.newSetFromMap(new TreeMap<>());
        set.add("b");
        set.add("a");
        System.out.println("backed-set=" + String.join(",", set.stream().sorted().toList()));

        // synchronizedSet 语义
        Set<String> sync = Collections.synchronizedSet(new HashSet<>());
        sync.add("x");
        System.out.println("sync-set=" + sync.contains("x") + sync.size());

        // unmodifiableSortedMap
        TreeMap<String, Integer> tm = new TreeMap<>();
        tm.put("k", 1);
        var usm = Collections.unmodifiableSortedMap(tm);
        System.out.println("usm-first=" + usm.firstKey());
        try {
            usm.put("j", 0);
        } catch (UnsupportedOperationException e) {
            System.out.println("usm-ex=" + e.getClass().getSimpleName());
        }

        // emptyIterator / emptyEnumeration
        Iterator<Object> ei = Collections.emptyIterator();
        System.out.println("empty-iter=" + ei.hasNext());
        Enumeration<Object> ee = Collections.emptyEnumeration();
        System.out.println("empty-enum=" + ee.hasMoreElements());

        // Vector 与 Stack 语义
        Vector<String> v = new Vector<>();
        v.addElement("a");
        v.addElement("b");
        System.out.println("vector=" + v + " cap-grow=" + (v.capacity() >= 2));
        v.trimToSize();
        System.out.println("trimmed-cap=" + v.capacity());
        System.out.println("element-at=" + v.elementAt(0));
        java.util.Stack<String> stack = new java.util.Stack<>();
        stack.push("p");
        System.out.println("stack-peek=" + stack.peek() + " pop=" + stack.pop() + " empty=" + stack.isEmpty());

        // Hashtable：keys / elements / entries
        Hashtable<String, String> ht = new Hashtable<>();
        ht.put("h1", "v1");
        ht.put("h2", "v2");
        int keyCount = 0;
        for (Enumeration<String> e = ht.keys(); e.hasMoreElements();) {
            e.nextElement();
            keyCount++;
        }
        System.out.println("ht-keys=" + keyCount + " entries=" + ht.entrySet().size());
        System.out.println("ht-null-reject=" + rejectNull(ht));

        // Properties 的 propertyNames（枚举形态）与 keys
        Properties p = new Properties();
        p.setProperty("p1", "1");
        p.setProperty("p2", "2");
        int pn = 0;
        for (Enumeration<?> e = p.propertyNames(); e.hasMoreElements();) {
            e.nextElement();
            pn++;
        }
        System.out.println("props-names=" + pn);

        // Collections.enumeration：集合→枚举
        Enumeration<String> ce = Collections.enumeration(java.util.Arrays.asList("m", "n"));
        StringBuilder sb = new StringBuilder();
        while (ce.hasMoreElements()) {
            sb.append(ce.nextElement());
        }
        System.out.println("collected-enum=" + sb);

        // HashMap putIfAbsent 顺手（map compute 族已有）
        HashMap<String, Integer> hm = new HashMap<>();
        hm.put("k", 1);
        System.out.println("absent-keep=" + hm.putIfAbsent("k", 2) + " value=" + hm.get("k"));
    }

    static boolean rejectNull(Hashtable<String, String> ht) {
        try {
            ht.put(null, "x");
            return false;
        } catch (NullPointerException e) {
            return true;
        }
    }
}
