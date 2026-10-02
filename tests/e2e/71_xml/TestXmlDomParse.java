import java.io.StringReader;

import javax.xml.parsers.DocumentBuilderFactory;

import org.w3c.dom.Document;
import org.w3c.dom.Element;
import org.w3c.dom.NodeList;
import org.xml.sax.InputSource;

/**
 * java.xml DOM 解析与遍历（71_xml 先行件）。
 *
 * 预审模式（docs/plans/2026-10-03-jmod-coverage.md 第 0 步）：源码 + 期望先行，
 * 不进全量基线，进基线等 jmod 第 2 步。本目录 5 例同时是 mybatis XML 面
 * （框架矩阵 #10）的先行压力面。
 */
public class TestXmlDomParse {

    public static void main(String[] args) throws Exception {
        String xml = "<root xmlns:a=\"urn:a\"><item a:id=\"1\" lang=\"en\">first</item>"
                + "<item a:id=\"2\" lang=\"zh\">second</item><tail>end</tail></root>";
        DocumentBuilderFactory f = DocumentBuilderFactory.newInstance();
        f.setNamespaceAware(true);
        Document doc = f.newDocumentBuilder().parse(new InputSource(new StringReader(xml)));

        Element root = doc.getDocumentElement();
        System.out.println("root=" + root.getNodeName() + " local=" + root.getLocalName());
        System.out.println("children=" + root.getChildNodes().getLength());

        NodeList items = doc.getElementsByTagNameNS("urn:a", "item");
        System.out.println("ns-items=" + items.getLength());
        for (int i = 0; i < items.getLength(); i++) {
            Element e = (Element) items.item(i);
            System.out.println("item id=" + e.getAttributeNS("urn:a", "id")
                    + " lang=" + e.getAttribute("lang")
                    + " text=" + e.getTextContent().trim());
        }

        System.out.println("tail=" + doc.getElementsByTagName("tail").item(0).getTextContent());
        // 缺失属性归一化：空串而非 null
        System.out.println("missing-attr=[" + root.getAttribute("nope") + "]");
    }
}
