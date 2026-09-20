import java.util.ArrayList;
import java.util.Collections;
import java.util.HashMap;
import java.util.List;
import java.util.Map;

public class TestMapIteration {

    public static void main(String[] args) {
        Map<String, Integer> m = new HashMap<>();
        m.put("delta", 4);
        m.put("alpha", 1);
        m.put("charlie", 3);
        m.put("bravo", 2);

        // entrySet 迭代（排序后打印，保证输出稳定）
        List<String> lines = new ArrayList<>();
        for (Map.Entry<String, Integer> e : m.entrySet()) {
            lines.add(e.getKey() + "->" + e.getValue());
        }
        Collections.sort(lines);
        System.out.println("entries=" + lines);

        // keySet / values
        List<String> keys = new ArrayList<>(m.keySet());
        Collections.sort(keys);
        System.out.println("keys=" + keys);

        List<Integer> vals = new ArrayList<>(m.values());
        Collections.sort(vals);
        System.out.println("values=" + vals);

        // Map.Entry.setValue
        for (Map.Entry<String, Integer> e : m.entrySet()) {
            if (e.getKey().equals("alpha")) {
                Integer prev = e.setValue(100);
                System.out.println("alpha prev=" + prev + " now=" + e.getValue());
            }
        }

        // forEach + lambda
        List<String> viaForEach = new ArrayList<>();
        m.forEach((k, v) -> viaForEach.add(k + "=" + v));
        Collections.sort(viaForEach);
        System.out.println("forEach=" + viaForEach);

        // keySet / values / entrySet 的 remove 会反映到 Map
        java.util.Iterator<String> kit = m.keySet().iterator();
        while (kit.hasNext()) {
            if (kit.next().equals("bravo")) {
                kit.remove();
            }
        }
        System.out.println("after iterator remove size=" + m.size() + " hasBravo=" + m.containsKey("bravo"));

        // values 的 removeIf
        boolean changed = m.values().removeIf(v -> v > 3);
        System.out.println("removeIf changed=" + changed + " size=" + m.size());

        // entrySet 迭代中抛 ConcurrentModificationException
        try {
            for (Map.Entry<String, Integer> e : m.entrySet()) {
                m.put("extra" + e.getKey(), 0);
            }
            System.out.println("structural modify accepted");
        } catch (RuntimeException ex) {
            System.out.println("modify during iterate -> " + ex.getClass().getSimpleName());
        }

        System.out.println("done");
    }
}
