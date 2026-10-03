import java.io.StringReader;

import javax.xml.parsers.DocumentBuilderFactory;
import javax.xml.xpath.XPath;
import javax.xml.xpath.XPathConstants;
import javax.xml.xpath.XPathFactory;

import org.w3c.dom.Document;
import org.w3c.dom.NodeList;
import org.xml.sax.InputSource;

/**
 * java.xml XPath 求值（71_xml 先行件，预审模式不进基线）。
 *
 * 数值/布尔/字符串/节点集四种求值类型、谓词（位置与属性）、name()/sum() 函数
 * ——mybatis 动态 SQL 与配置解析共同压着的面。
 */
public class TestXmlXPath {

    public static void main(String[] args) throws Exception {
        String xml = "<team><dev id=\"1\" lang=\"en\"/><dev id=\"2\" lang=\"zh\"/>"
                + "<dev id=\"3\" lang=\"zh\"/><qa id=\"4\"/></team>";
        Document doc = DocumentBuilderFactory.newInstance()
                .newDocumentBuilder().parse(new InputSource(new StringReader(xml)));
        XPath xp = XPathFactory.newInstance().newXPath();

        System.out.println("count=" + xp.evaluate("count(/team/dev)", doc, XPathConstants.NUMBER));
        System.out.println("sum=" + xp.evaluate("sum(/team/dev/@id)", doc, XPathConstants.NUMBER));

        NodeList ns = (NodeList) xp.evaluate("//dev[@lang='zh']/@id", doc, XPathConstants.NODESET);
        for (int i = 0; i < ns.getLength(); i++) {
            System.out.println("zh-id=" + ns.item(i).getNodeValue());
        }

        System.out.println("first=" + xp.evaluate("/team/dev[1]/@id", doc, XPathConstants.STRING));
        System.out.println("last-lang="
                + xp.evaluate("string(/team/dev[last()]/@lang)", doc, XPathConstants.STRING));
        System.out.println("bool=" + xp.evaluate("boolean(/team/qa)", doc, XPathConstants.BOOLEAN));
        System.out.println("bool-miss="
                + xp.evaluate("boolean(/team/pm)", doc, XPathConstants.BOOLEAN));
        System.out.println("name=" + xp.evaluate("name(/team/*[4])", doc, XPathConstants.STRING));
    }
}
