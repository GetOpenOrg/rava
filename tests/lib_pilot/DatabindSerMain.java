
import java.util.List;
import java.util.Map;

import com.fasterxml.jackson.core.type.TypeReference;
import com.fasterxml.jackson.databind.JsonNode;
import com.fasterxml.jackson.databind.ObjectMapper;

/**
 * jackson-databind pilot E2：基础读写 + 树模型 + 泛型集合（E 批）。
 */
public class DatabindSerMain {

    static class Order {
        public String orderId = "SO-001";
        public long amountFen = 12500;
        public List<String> items = List.of("键盘", "鼠标");
        public Map<String, Integer> stock = newStock();
    }

    static Map<String, Integer> newStock() {
        Map<String, Integer> m = new java.util.LinkedHashMap<>(); // 固定插入序
        m.put("键盘", 3);
        m.put("鼠标", 5);
        return m;
    }

    static void check(String name, Object expected, Object actual) {
        boolean ok = String.valueOf(expected).equals(String.valueOf(actual));
        System.out.println((ok ? "PASS " : "FAIL ") + name
                + (ok ? "" : " :: expected=<" + expected + "> actual=<" + actual + ">"));
    }

    public static void main(String[] args) throws Exception {
        ObjectMapper mapper = new ObjectMapper();

        // POJO 序列化（字段顺序：public 声明序）
        String json = mapper.writeValueAsString(new Order());
        System.out.println("  order json: " + json);
        check("contains-id", true, json.contains("\"orderId\":\"SO-001\""));
        check("contains-amount", true, json.contains("\"amountFen\":12500"));

        // 反序列化往返
        Order back = mapper.readValue(json, Order.class);
        check("roundtrip-id", "SO-001", back.orderId);
        check("roundtrip-amount", 12500L, back.amountFen);
        check("roundtrip-items", "[键盘, 鼠标]", back.items.toString());

        // 树模型 JsonNode（读取与路径）
        JsonNode root = mapper.readTree(json);
        check("tree-path", "SO-001", root.path("orderId").asText());
        check("tree-items-size", 2, root.path("items").size());
        check("tree-items-0", "键盘", root.path("items").get(0).asText());
        check("tree-long", 12500L, root.path("amountFen").asLong());
        check("tree-missing-default", "fallback", root.path("none").asText("fallback"));

        // 泛型集合：TypeReference
        List<Order> orders = mapper.readValue("[" + json + "]",
                new TypeReference<List<Order>>() {
                });
        check("generic-list-size", 1, orders.size());
        check("generic-list-elem", "SO-001", orders.get(0).orderId);

        // Map 泛型
        Map<String, Integer> counts = mapper.readValue("{\"a\":1,\"b\":2}",
                new TypeReference<Map<String, Integer>>() {
                });
        check("generic-map", "{a=1, b=2}", new java.util.TreeMap<>(counts).toString());

        // writeValueAsString 的 null 语义（字段为 null 时输出 null）
        Map<String, Object> withNull = new java.util.LinkedHashMap<>();
        withNull.put("a", null);
        withNull.put("b", 1);
        check("null-field", "{\"a\":null,\"b\":1}", mapper.writeValueAsString(withNull));
        System.out.println("DONE DatabindSerMain");
    }
}
