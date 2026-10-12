
import com.alibaba.fastjson2.JSON;
import com.alibaba.fastjson2.JSONArray;
import com.alibaba.fastjson2.JSONObject;
import com.alibaba.fastjson2.JSONPath;

/**
 * Fastjson2 pilot B10：JSONPath 命中矩阵（W5）。
 */
public class FastjsonPathMain {

    static void check(String name, Object expected, Object actual) {
        boolean ok = String.valueOf(expected).equals(String.valueOf(actual));
        System.out.println((ok ? "PASS " : "FAIL ") + name
                + (ok ? "" : " :: expected=<" + expected + "> actual=<" + actual + ">"));
    }

    static final String JSON_TEXT = """
            {
              "store": {
                "book": [
                  {"category":"ref","author":"Nigel Rees","title":"Sayings","price":8.95},
                  {"category":"fic","author":"Evelyn Waugh","title":"Sword","price":12.99},
                  {"category":"fic","author":"Herman Melville","title":"Moby Dick","price":8.99}
                ],
                "bicycle": {"color":"red","price":19.95}
              }
            }
            """;

    public static void main(String[] args) {
        Object root = JSON.parse(JSON_TEXT);

        // 基本路径
        check("root-prop", "red", JSONPath.eval(root, "$.store.bicycle.color"));
        check("array-index", "Sword", JSONPath.eval(root, "$.store.book[1].title"));

        // 通配与投影
        check("wildcard-prices", "[8.95,12.99,8.99]",
                JSONPath.eval(root, "$.store.book[*].price").toString());
        check("wildcard-titles", "[\"Sayings\",\"Sword\",\"Moby Dick\"]",
                JSONPath.eval(root, "$.store.book[*].title").toString());

        // 过滤器
        Object cheapTitles = JSONPath.eval(root,
                "$.store.book[?(@.price < 10)].title");
        check("filter-cheap", "[\"Sayings\",\"Moby Dick\"]", cheapTitles.toString());

        Object ficAuthors = JSONPath.eval(root,
                "$.store.book[?(@.category == 'fic')].author");
        check("filter-category", "[\"Evelyn Waugh\",\"Herman Melville\"]", ficAuthors.toString());

        // 深扫描
        Object allPrices = JSONPath.eval(root, "$..price");
        check("deep-scan", "[8.95,12.99,8.99,19.95]", allPrices.toString());

        // length
        check("length", 3, JSONPath.eval(root, "$.store.book.length()"));

        // JSONPath.set：写回（对解析副本）
        JSONObject copy = JSON.parseObject(JSON_TEXT);
        JSONPath.set(copy, "$.store.bicycle.color", "blue");
        check("path-set", "blue", JSONPath.eval(copy, "$.store.bicycle.color"));

        // contains 面板（路径存在性）
        check("path-contains", true,
                JSONPath.contains(copy, "$.store.bicycle.color"));
        check("path-contains-miss", false,
                JSONPath.contains(copy, "$.store.bicycle.serial"));
        System.out.println("DONE FastjsonPathMain");
    }
}
