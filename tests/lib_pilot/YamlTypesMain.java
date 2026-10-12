
import java.util.LinkedHashMap;
import java.util.List;

import org.yaml.snakeyaml.Yaml;

/**
 * SnakeYAML pilot B6：标量/集合/多文档的类型还原（W5）。
 */
public class YamlTypesMain {

    static void check(String name, Object expected, Object actual) {
        boolean ok = String.valueOf(expected).equals(String.valueOf(actual));
        System.out.println((ok ? "PASS " : "FAIL ") + name
                + (ok ? "" : " :: expected=<" + expected + "> actual=<" + actual + ">"));
    }

    static Object t(String yaml) {
        return new Yaml().load(yaml);
    }

    public static void main(String[] args) {
        // 标量类型推断
        check("int", 42, t("42"));
        check("long", 9876543210L, t("9876543210"));
        check("float", 2.5, t("2.5"));
        check("bool-yes", Boolean.TRUE, t("true"));
        check("null-tilde", null, t("~"));
        java.util.TimeZone.setDefault(java.util.TimeZone.getTimeZone("GMT+08:00"));
        check("date", "Mon Oct 05 08:00:00 GMT+08:00 2026",
                String.valueOf(t("2026-10-05"))); // Date.toString（UTC 零点 → 本地 08:00）

        // 集合语法
        check("flow-list", "[a, b]", t("[a, b]").toString());
        check("block-list", "[1, 2, 3]", t("- 1\n- 2\n- 3\n").toString());
        check("flow-map", "{k1=v1, k2=v2}", new java.util.TreeMap<>(
                (java.util.Map<?, ?>) t("{k1: v1, k2: v2}")).toString());

        // 嵌套块结构
        Object nested = t("""
                server:
                  host: 127.0.0.1
                  port: 8080
                auth:
                  enabled: true
                  roles: [admin, user]
                """);
        @SuppressWarnings("unchecked")
        java.util.Map<String, Object> m = (java.util.Map<String, Object>) nested;
        check("nested-keys", "[auth, server]", new java.util.TreeSet<>(m.keySet()).toString());
        @SuppressWarnings("unchecked")
        java.util.Map<String, Object> server = (java.util.Map<String, Object>) m.get("server");
        check("server-port", 8080, server.get("port"));
        check("server-host", "127.0.0.1", server.get("host"));

        // 多文档（--- 分隔）
        var docs = new java.util.ArrayList<>();
        for (Object doc : new Yaml().loadAll("a: 1\n---\nb: 2\n")) {
            docs.add(doc);
        }
        check("multi-doc-size", 2, docs.size());
        check("multi-doc-2", "{b=2}", docs.get(1).toString());

        // 空文档与空串
        check("empty-doc-null", null, t(""));
        System.out.println("DONE YamlTypesMain");
    }
}
