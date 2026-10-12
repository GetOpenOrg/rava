
import java.util.LinkedHashMap;
import java.util.List;
import java.util.Map;

import org.yaml.snakeyaml.DumperOptions;
import org.yaml.snakeyaml.LoaderOptions;
import org.yaml.snakeyaml.Yaml;

/**
 * SnakeYAML pilot B5：dump → load 往返等价（W5）。
 */
public class YamlRoundTripMain {

    static void check(String name, Object expected, Object actual) {
        boolean ok = String.valueOf(expected).equals(String.valueOf(actual));
        System.out.println((ok ? "PASS " : "FAIL ") + name
                + (ok ? "" : " :: expected=<" + expected + "> actual=<" + actual + ">"));
    }

    public static void main(String[] args) {
        DumperOptions options = new DumperOptions();
        options.setDefaultFlowStyle(DumperOptions.FlowStyle.BLOCK);
        Yaml yaml = new Yaml(options);

        // 原始结构（LinkedHashMap 保序）
        Map<String, Object> src = new LinkedHashMap<>();
        src.put("name", "pilot");
        src.put("level", 7);
        src.put("ratio", 2.5);
        src.put("enabled", Boolean.TRUE);
        src.put("tags", List.of("a", "b"));
        Map<String, Object> nested = new LinkedHashMap<>();
        nested.put("depth", 2);
        nested.put("list", List.of(1, 2, 3));
        src.put("nested", nested);

        String dumped = yaml.dump(src);
        System.out.println("--- dumped ---");
        System.out.print(dumped);

        // 加载回来逐项比对
        Map<String, Object> back = yaml.load(dumped);
        check("scalar-str", "pilot", back.get("name"));
        check("scalar-int", 7, back.get("level"));
        check("scalar-float", 2.5, back.get("ratio"));
        check("scalar-bool", Boolean.TRUE, back.get("enabled"));
        check("list", "[a, b]", back.get("tags").toString());
        @SuppressWarnings("unchecked")
        Map<String, Object> backNested = (Map<String, Object>) back.get("nested");
        check("nested-depth", 2, backNested.get("depth"));
        check("nested-list", "[1, 2, 3]", backNested.get("list").toString());
        check("keyset", "[enabled, level, name, nested, ratio, tags]",
                new java.util.TreeSet<>(back.keySet()).toString());

        // dump 的锚点/引用：相同对象引用会被写成 &anchor/*ref
        java.util.List<Object> shared = new java.util.ArrayList<>(List.of("x"));
        Map<String, Object> aliasSrc = new LinkedHashMap<>();
        aliasSrc.put("one", shared);
        aliasSrc.put("two", shared);
        String aliasDump = yaml.dump(aliasSrc);
        check("alias-anchor-used", "true",
                String.valueOf(aliasDump.contains("&id") && aliasDump.contains("*id")));

        // load 字符串字面量安全面（无自定义构造器时按默认类型还原）
        Object loaded = yaml.load("just a string");
        check("bare-string", "just a string", loaded);
        System.out.println("DONE YamlRoundTripMain");
    }
}
