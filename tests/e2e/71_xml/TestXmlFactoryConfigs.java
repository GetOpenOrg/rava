import java.io.StringReader;

import javax.xml.parsers.DocumentBuilder;
import javax.xml.parsers.DocumentBuilderFactory;
import javax.xml.parsers.SAXParserFactory;

import org.w3c.dom.Document;
import org.xml.sax.EntityResolver;
import org.xml.sax.InputSource;
import org.xml.sax.helpers.DefaultHandler;

/**
 * XML 工厂配置面（方法级实测：SAXParserFactory.setValidating 8 jar /
 * DocumentBuilderFactory.setFeature 6 / DocumentBuilder.setEntityResolver 5，
 * 此前零覆盖）：开关往返、特征查询、解析器行为随配置变化。
 */
public class TestXmlFactoryConfigs {

    public static void main(String[] args) throws Exception {
        SAXParserFactory sf = SAXParserFactory.newInstance();
        sf.setValidating(true);
        sf.setNamespaceAware(true);
        System.out.println("sax-validating=" + sf.isValidating()
                + " ns-aware=" + sf.isNamespaceAware()
                + " xinclude=" + sf.isXIncludeAware());
        sf.setFeature("http://xml.org/sax/features/namespaces", true);
        System.out.println("sax-feat-ns=" + sf.getFeature("http://xml.org/sax/features/namespaces"));

        DocumentBuilderFactory df = DocumentBuilderFactory.newInstance();
        df.setCoalescing(true);
        df.setIgnoringComments(true);
        df.setExpandEntityReferences(false);
        System.out.println("dom-coalescing=" + df.isCoalescing()
                + " ignore-comments=" + df.isIgnoringComments()
                + " expand-entities=" + df.isExpandEntityReferences());
        df.setFeature("http://apache.org/xml/features/dom/create-entity-ref-nodes", false);
        System.out.println("dom-feat-query-ok=true");

        // 行为验证：忽略注释后文档无 comment 节点
        DocumentBuilder db = df.newDocumentBuilder();
        final int[] resolved = { 0 };
        db.setEntityResolver((EntityResolver) (publicId, systemId) -> {
            resolved[0]++;
            return null;
        });
        Document doc = db.parse(new InputSource(new StringReader(
                "<root><!-- gone --><a>1<!--x--></a></root>")));
        int comments = 0;
        for (var n = doc.getDocumentElement().getFirstChild(); n != null; n = n.getNextSibling()) {
            if (n.getNodeType() == org.w3c.dom.Node.COMMENT_NODE) {
                comments++;
            }
        }
        System.out.println("comments-ignored=" + (comments == 0));
        System.out.println("resolver-not-needed=" + (resolved[0] == 0));

        // SAX 默认 handler 解析同文档（配置路径完整性）
        SAXParserFactory.newInstance().newSAXParser().parse(
                new InputSource(new StringReader("<ok/>")), new DefaultHandler());
        System.out.println("sax-bare-parse=true");
    }
}
