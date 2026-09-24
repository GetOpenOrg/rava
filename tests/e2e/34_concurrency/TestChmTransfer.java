import java.util.concurrent.ConcurrentHashMap;

public class TestChmTransfer {

    public static void main(String[] args) {
        // 无参构造：JDK 默认 16 槽 / 阈值 12，装载 >12 条目即触发 transfer 扩容搬移
        ConcurrentHashMap<String, Integer> m = new ConcurrentHashMap<>();
        for (int i = 0; i < 40; i++) {
            m.put("key" + i, i);
        }
        System.out.println("size=" + m.size());

        int sum = 0;
        int missing = 0;
        for (int i = 0; i < 40; i++) {
            Integer v = m.get("key" + i);
            if (v == null) {
                missing++;
            } else {
                sum += v;
            }
        }
        System.out.println("sum=" + sum);
        System.out.println("missing=" + missing);
        System.out.println("get17=" + m.get("key17"));
        System.out.println("absent=" + m.get("nope"));
        System.out.println("contains=" + m.containsKey("key17") + "," + m.containsKey("key40"));

        // 覆盖写与删除在扩容后的正确性
        Integer old = m.put("key3", 300);
        System.out.println("old3=" + old + " new3=" + m.get("key3"));
        Integer rm = m.remove("key5");
        System.out.println("rm5=" + rm + " size=" + m.size());
        System.out.println("done");
    }
}
