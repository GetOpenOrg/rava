
import org.jsoup.Jsoup;
import org.jsoup.nodes.Document;
import org.jsoup.nodes.Element;
import org.jsoup.select.Elements;

/**
 * Jsoup pilot B1：HTML 解析树结构断言（W5）。
 */
public class JsoupParseMain {

    static final String HTML = """
            <html><head><title>Pilot 页</title></head>
            <body>
              <div id="main" class="content wide">
                <h1>标题一</h1>
                <p class="intro">第一段<a href="/a">链接A</a>。</p>
                <p>第二段</p>
                <ul><li>甲</li><li>乙</li><li>丙</li></ul>
              </div>
              <div id="footer">页脚</div>
            </body></html>
            """;

    public static void main(String[] args) {
        Document doc = Jsoup.parse(HTML);

        checkEq("title", "Pilot 页", doc.title());
        checkEq("head-meta-empty", "[]",
                doc.head().select("meta").eachAttr("charset").toString());

        Element main = doc.selectFirst("#main");
        checkEq("id-selector", "main", main.id());
        checkEq("class-list", "[content, wide]",
                new java.util.ArrayList<>(main.classNames()).toString());
        checkEq("own-text-nochildren", "页脚", doc.selectFirst("#footer").ownText());

        Elements paragraphs = main.select("p");
        checkEq("p-count", 2, paragraphs.size());
        checkEq("p1-text", "第一段链接A。", paragraphs.get(0).text());
        checkEq("p1-class", "intro", paragraphs.get(0).className());

        Element link = main.selectFirst("a");
        checkEq("abs-href-simplerel", "/a", link.attr("href"));
        checkEq("link-text", "链接A", link.text());

        Elements items = main.select("ul > li");
        checkEq("li-items", "[甲, 乙, 丙]", items.eachText().toString());

        // 属性读写与 data-*
        Element h1 = main.selectFirst("h1");
        h1.attr("data-idx", "1");
        checkEq("data-attr", "1", h1.dataset().get("idx"));

        // 结构遍历：父/兄弟
        checkEq("parent-tag", "div", h1.parent().tagName());
        checkEq("sibling-elements", "h1 p p ul",
                java.util.Arrays.toString(main.children().stream()
                        .map(Element::tagName).toArray()).replace("[", "").replace("]", "")
                        .replace(",", ""));

        // 修改文档后重新序列化片段（确定性输出）
        main.appendElement("p").text("追加段");
        checkEq("after-append", 3, main.select("p").size());
        System.out.println("DONE JsoupParseMain");
    }

    static void checkEq(String name, Object expected, Object actual) {
        boolean ok = String.valueOf(expected).equals(String.valueOf(actual));
        System.out.println((ok ? "PASS " : "FAIL ") + name
                + (ok ? "" : " :: expected=<" + expected + "> actual=<" + actual + ">"));
    }
}
