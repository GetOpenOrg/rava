import java.util.ArrayList;
import java.util.Collections;
import java.util.HashMap;
import java.util.List;
import java.util.Map;

public class TestHashMapOps {

    static void dump(Map<?, ?> m, String tag) {
        List<String> keys = new ArrayList<>();
        for (Object k : m.keySet()) {
            keys.add(String.valueOf(k));
        }
        Collections.sort(keys);
        StringBuilder sb = new StringBuilder(tag).append("{");
        for (String k : keys) {
            sb.append(k).append("=").append(m.get(k)).append(" ");
        }
        sb.append("}");
        System.out.println(sb);
    }

    public static void main(String[] args) {
        Map<String, Integer> scores = new HashMap<>();

        // 基础 put / get
        scores.put("alice", 90);
        scores.put("bob", 75);
        scores.put("carol", 88);
        System.out.println("size=" + scores.size());
        System.out.println("alice=" + scores.get("alice"));
        System.out.println("bob=" + scores.get("bob"));
        System.out.println("missing=" + scores.get("dave"));

        // 覆盖写
        Integer old = scores.put("bob", 80);
        System.out.println("old bob=" + old + " new bob=" + scores.get("bob"));

        // containsKey / containsValue
        System.out.println("hasKey=" + scores.containsKey("alice"));
        System.out.println("hasKey2=" + scores.containsKey("zed"));
        System.out.println("hasValue=" + scores.containsValue(88));

        // remove
        Integer removed = scores.remove("carol");
        System.out.println("removed=" + removed + " size=" + scores.size());
        System.out.println("remove missing=" + scores.remove("nobody"));

        // getOrDefault / putIfAbsent / replace / merge / computeIfAbsent
        System.out.println("getOrDefault=" + scores.getOrDefault("dave", -1));
        System.out.println("putIfAbsent new=" + scores.putIfAbsent("dave", 60));
        System.out.println("putIfAbsent exist=" + scores.putIfAbsent("dave", 99));
        System.out.println("replace=" + scores.replace("dave", 61));
        scores.replace("dave", 62);
        System.out.println("dave=" + scores.get("dave"));

        scores.computeIfAbsent("erin", k -> k.length() * 10);
        System.out.println("erin(computed)=" + scores.get("erin"));
        scores.computeIfAbsent("erin", k -> -1);
        System.out.println("erin(again)=" + scores.get("erin"));

        scores.merge("alice", 5, Integer::sum);
        System.out.println("merged alice=" + scores.get("alice"));

        // null key 与 null value
        Map<String, String> nullable = new HashMap<>();
        nullable.put(null, "null-key");
        nullable.put("k", null);
        System.out.println("nullKey value=" + nullable.get(null));
        System.out.println("k value=" + nullable.get("k"));
        System.out.println("size=" + nullable.size());
        System.out.println("containsKey(null)=" + nullable.containsKey(null));

        // 非 String 键
        Map<Integer, String> byId = new HashMap<>();
        byId.put(1, "one");
        byId.put(2, "two");
        byId.put(Integer.valueOf(1) + 1, "three");
        System.out.println("id2=" + byId.get(2) + " size=" + byId.size());

        dump(scores, "scores");
        System.out.println("isEmpty=" + scores.isEmpty());
        scores.clear();
        System.out.println("after clear isEmpty=" + scores.isEmpty());

        System.out.println("done");
    }
}
