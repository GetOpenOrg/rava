import java.util.HashMap;
import java.util.LinkedHashMap;
import java.util.LinkedHashSet;
import java.util.List;
import java.util.Map;

public class TestLinkedHash {

    public static void main(String[] args) {
        // LinkedHashMap 保持插入顺序
        LinkedHashMap<String, Integer> lhm = new LinkedHashMap<>();
        lhm.put("delta", 4);
        lhm.put("alpha", 1);
        lhm.put("charlie", 3);
        lhm.put("bravo", 2);
        System.out.println("insertion order keys=" + lhm.keySet());
        System.out.println("insertion order values=" + lhm.values());

        // 重复 put 不改变顺序
        lhm.put("alpha", 100);
        System.out.println("after update keys=" + lhm.keySet());
        System.out.println("alpha=" + lhm.get("alpha"));

        // remove 再 put → 排到末尾
        lhm.remove("alpha");
        lhm.put("alpha", 7);
        System.out.println("after remove+reinsert keys=" + lhm.keySet());

        // LinkedHashSet 保持插入顺序
        LinkedHashSet<String> lhs = new LinkedHashSet<>();
        lhs.add("z");
        lhs.add("a");
        lhs.add("m");
        lhs.add("a");
        System.out.println("linked set=" + lhs + " size=" + lhs.size());

        // 与 HashMap 对照：LinkedHashMap 明确保序
        HashMap<String, Integer> hm = new HashMap<>(lhm);
        System.out.println("linked first=" + lhm.keySet().iterator().next());
        System.out.println("hashmap size=" + hm.size());

        // LinkedHashMap.getOrDefault / containsKey
        System.out.println("getOrDefault=" + lhm.getOrDefault("nope", -1));
        System.out.println("containsKey delta=" + lhm.containsKey("delta"));

        // 遍历 entrySet 也是插入序
        StringBuilder sb = new StringBuilder();
        for (Map.Entry<String, Integer> e : lhm.entrySet()) {
            sb.append(e.getKey()).append(":").append(e.getValue()).append(" ");
        }
        System.out.println("entries=" + sb.toString().trim());

        // 由 List 构造
        LinkedHashMap<Integer, String> fromList = new LinkedHashMap<>();
        List<String> names = List.of("x", "y", "z");
        int idx = 0;
        for (String n : names) {
            fromList.put(idx++, n);
        }
        System.out.println("fromList keys=" + fromList.keySet() + " values=" + fromList.values());

        // clear 后再插入
        lhm.clear();
        System.out.println("cleared isEmpty=" + lhm.isEmpty());
        lhm.put("new", 1);
        System.out.println("keys=" + lhm.keySet());

        System.out.println("done");
    }
}
