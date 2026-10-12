
import java.util.List;

import com.alibaba.fastjson2.JSON;
import com.alibaba.fastjson2.JSONArray;
import com.alibaba.fastjson2.JSONObject;
import com.alibaba.fastjson2.JSONReader;

/**
 * Fastjson2 pilot B9：流式 JSONReader 读取（W5）。
 */
public class FastjsonStreamMain {

    static void check(String name, Object expected, Object actual) {
        boolean ok = String.valueOf(expected).equals(String.valueOf(actual));
        System.out.println((ok ? "PASS " : "FAIL ") + name
                + (ok ? "" : " :: expected=<" + expected + "> actual=<" + actual + ">"));
    }

    public static void main(String[] args) {
        String json = """
                {"name":"pilot","version":7,"ratio":2.5,
                 "tags":["a","b"],
                 "nested":{"ok":true,"none":null}}
                """;

        // 1. JSONReader.Any 流式消费（读一个对象）
        JSONObject obj;
        try (JSONReader reader = JSONReader.of(json.toCharArray())) {
            obj = (JSONObject) reader.readAny();
        }
        check("string", "pilot", obj.getString("name"));
        check("int", 7, obj.getIntValue("version"));
        check("double", 2.5, obj.getDoubleValue("ratio"));
        check("tags", "[\"a\",\"b\"]", obj.getJSONArray("tags").toString());
        check("nested-ok", true, obj.getJSONObject("nested").getBoolean("ok"));

        // 2. 数组流式遍历：逐元素 readObject（不全量加载语义）
        StringBuilder items = new StringBuilder();
        try (JSONReader reader = JSONReader.of("[1,\"two\",3.5,true,null]".toCharArray())) {
            for (Object v : reader.readArray()) {
                items.append(v).append(';');
            }
        }
        check("array-stream", "1;two;3.5;true;null;", items.toString());

        // 3. JSONObject 取值类型面
        JSONObject typed = JSON.parseObject("{\"n\":5,\"b\":true,\"big\":9876543210}");
        check("get-long", 9876543210L, typed.getLong("big"));
        check("get-boolean", true, typed.getBoolean("b"));
        check("get-intvalue", 5, typed.getIntValue("n"));

        // 4. JSONArray/JSONObject 的可变操作面
        JSONArray arr = new JSONArray();
        arr.add("x");
        arr.addAll(List.of("y", "z"));
        check("jsonarray-mutate", "[\"x\",\"y\",\"z\"]", arr.toString());
        arr.remove(0);
        check("jsonarray-remove", "[\"y\",\"z\"]", arr.toString());

        // 5. pretty format（格式化输出）
        JSONObject pretty = new JSONObject();
        pretty.put("k", "v");
        System.out.println("  pretty: " + JSON.toJSONString(pretty,
                com.alibaba.fastjson2.JSONWriter.Feature.PrettyFormat).replace("\n", "|"));
        System.out.println("DONE FastjsonStreamMain");
    }
}
