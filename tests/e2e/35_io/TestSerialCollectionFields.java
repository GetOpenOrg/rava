import java.io.*;
import java.util.*;

// JDK 集合作为可序列化对象的字段：ArrayList / HashMap / LinkedList 各自的私有 writeObject / readObject
// 回调经反射调用（ObjectStreamClass 按名查找），覆盖空集合、嵌套集合、null 字段、同一集合被两个字段共享、
// 集合元素是用户可序列化对象、反序列化后可继续修改
public class TestSerialCollectionFields {
    static class Item implements Serializable {
        private static final long serialVersionUID = 1L;
        final String name;
        final int qty;

        Item(String name, int qty) {
            this.name = name;
            this.qty = qty;
        }

        @Override
        public String toString() {
            return name + "x" + qty;
        }
    }

    static class Holder implements Serializable {
        private static final long serialVersionUID = 2L;
        ArrayList<Item> items = new ArrayList<>();
        HashMap<String, Integer> counts = new HashMap<>();
        LinkedList<String> log = new LinkedList<>();
        ArrayList<String> empty = new ArrayList<>();
        HashMap<Integer, List<String>> nested = new HashMap<>();
        LinkedList<Integer> missing;
        List<String> sharedA;
        List<String> sharedB;
    }

    static Object roundTrip(Object o) throws Exception {
        ByteArrayOutputStream bos = new ByteArrayOutputStream();
        try (ObjectOutputStream out = new ObjectOutputStream(bos)) {
            out.writeObject(o);
        }
        try (ObjectInputStream in = new ObjectInputStream(new ByteArrayInputStream(bos.toByteArray()))) {
            return in.readObject();
        }
    }

    public static void main(String[] args) throws Exception {
        Holder h = new Holder();
        h.items.add(new Item("apple", 3));
        h.items.add(new Item("pear", 5));
        h.items.add(null);
        h.counts.put("alpha", 1);
        h.counts.put("beta", 2);
        h.counts.put("gamma", 3);
        h.counts.put(null, 0);
        h.log.add("start");
        h.log.addFirst("boot");
        h.log.addLast("stop");
        h.nested.put(1, new ArrayList<>(List.of("a", "b")));
        h.nested.put(2, new LinkedList<>(List.of("c")));
        List<String> shared = new ArrayList<>(List.of("x", "y"));
        h.sharedA = shared;
        h.sharedB = shared;

        Holder r = (Holder) roundTrip(h);
        System.out.println("items=" + r.items + " size=" + r.items.size() + " " + r.items.getClass().getSimpleName());
        System.out.println("counts=" + new TreeMap<>(nonNullKeys(r.counts)) + " null->" + r.counts.get(null) + " size=" + r.counts.size());
        System.out.println("log=" + r.log + " first=" + r.log.getFirst() + " last=" + r.log.getLast());
        System.out.println("empty=" + r.empty + " isEmpty=" + r.empty.isEmpty());
        System.out.println("nested=" + r.nested.get(1) + "/" + r.nested.get(2).getClass().getSimpleName() + r.nested.get(2));
        System.out.println("missing=" + r.missing);
        System.out.println("shared same=" + (r.sharedA == r.sharedB) + " distinct from original=" + (r.sharedA != shared));

        // 反序列化得到的集合可继续使用（内部数组 / 表 / 链表头尾已按回调重建）
        r.items.add(new Item("plum", 7));
        r.items.remove(0);
        r.counts.merge("alpha", 10, Integer::sum);
        r.log.removeFirst();
        r.log.add("again");
        r.sharedA.add("z");
        System.out.println("items2=" + r.items);
        System.out.println("alpha=" + r.counts.get("alpha"));
        System.out.println("log2=" + r.log);
        System.out.println("sharedB=" + r.sharedB);

        // 集合本身作为顶层对象
        ArrayList<Integer> top = new ArrayList<>();
        for (int i = 0; i < 20; i++) {
            top.add(i * i);
        }
        @SuppressWarnings("unchecked")
        ArrayList<Integer> top2 = (ArrayList<Integer>) roundTrip(top);
        System.out.println("top=" + top2.subList(15, 20) + " sum=" + top2.stream().mapToInt(Integer::intValue).sum());
        HashMap<String, Item> byName = new HashMap<>();
        for (Item it : h.items) {
            if (it != null) {
                byName.put(it.name, it);
            }
        }
        @SuppressWarnings("unchecked")
        HashMap<String, Item> byName2 = (HashMap<String, Item>) roundTrip(byName);
        System.out.println("byName=" + new TreeMap<>(byName2));
    }

    static Map<String, Integer> nonNullKeys(Map<String, Integer> m) {
        Map<String, Integer> out = new HashMap<>();
        for (Map.Entry<String, Integer> e : m.entrySet()) {
            if (e.getKey() != null) {
                out.put(e.getKey(), e.getValue());
            }
        }
        return out;
    }
}
