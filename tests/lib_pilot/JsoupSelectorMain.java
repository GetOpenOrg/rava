
import org.jsoup.Jsoup;
import org.jsoup.nodes.Document;

/**
 * Jsoup pilot B2：CSS 选择器命中矩阵（W5）。
 */
public class JsoupSelectorMain {

    static final String HTML = """
            <div>
              <a class="btn primary" href="/p1">P1</a>
              <a class="btn" href="/p2">P2</a>
              <a href="/plain">Plain</a>
              <input type="text" value="q" disabled>
              <table><tr><td>1</td><td>2</td></tr><tr><td>3</td><td>4</td></tr></table>
              <span data-state="on">S1</span><span data-state="off">S2</span>
            </div>
            """;

    static void eq(String name, String expected, String actual) {
        System.out.println((expected.equals(actual) ? "PASS " : "FAIL ") + name
                + (expected.equals(actual) ? "" : " :: expected=<" + expected + "> actual=<" + actual + ">"));
    }

    public static void main(String[] args) {
        Document doc = Jsoup.parse(HTML);

        eq("class-and", "[P1]",
                doc.select("a.btn.primary").eachText().toString());
        eq("class-only", "[P1, P2]",
                doc.select("a.btn").eachText().toString());
        eq("attr-exists", "[q]",
                doc.select("input[disabled]").eachAttr("value").toString());
        eq("attr-eq", "[/p1]",
                doc.select("a[href=/p1]").eachAttr("href").toString());
        eq("attr-prefix", "[P1, P2, Plain]",
                doc.select("a[href^=/]").eachText().toString());
        eq("nth-child", "[3]",
                doc.select("table tr:nth-child(2) td:first-child").eachText().toString());
        eq("td-last", "4",
                doc.select("td:last-child").eachText().get(1));
        eq("data-attr", "[S1]",
                doc.select("span[data-state=on]").eachText().toString());
        eq("not-selector", "[P2]",
                doc.select("a.btn:not(.primary)").eachText().toString());
        eq("contains-text", "[Plain]",
                doc.select("a:containsOwn(lain)").eachText().toString());
        eq("element-count", "7",
                String.valueOf(doc.select("div > *").size()));
        eq("union-selector", "[P1, P2]",
                doc.select("a.btn:not(.primary), a.btn.primary").eachText()
                        .stream().sorted().toList().toString());
        eq("empty-input-value", "q",
                doc.select("input[type=text]").attr("value"));
        System.out.println("DONE JsoupSelectorMain");
    }
}
