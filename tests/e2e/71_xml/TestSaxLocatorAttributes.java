import java.io.StringReader;

import javax.xml.parsers.SAXParserFactory;

import org.xml.sax.Attributes;
import org.xml.sax.InputSource;
import org.xml.sax.Locator;
import org.xml.sax.helpers.DefaultHandler;

/**
 * SAX Locator 与 Attributes 元数据（方法级实测：setDocumentLocator 5 jar /
 * Attributes.getURI 5 / Node.getAttributes 5，此前零覆盖）：定位器行列号、
 * 属性的 URI/类型查询。
 */
public class TestSaxLocatorAttributes {

    static Locator loc;

    static class H extends DefaultHandler {
        @Override
        public void setDocumentLocator(Locator locator) {
            loc = locator;
        }

        @Override
        public void startElement(String uri, String localName, String qName, Attributes attrs) {
            System.out.println("at " + localName + " line=" + loc.getLineNumber()
                    + " col=" + loc.getColumnNumber()
                    + " pub-null=" + (loc.getPublicId() == null));
            for (int i = 0; i < attrs.getLength(); i++) {
                System.out.println("  attr " + attrs.getLocalName(i)
                        + " uri=[" + attrs.getURI(i) + "]"
                        + " q=" + attrs.getQName(i)
                        + " type=" + attrs.getType(i));
            }
            // 按 URI+local 定位取值
            System.out.println("  by-uri=" + attrs.getValue(uri, attrs.getLocalName(0)));
        }
    }

    public static void main(String[] args) throws Exception {
        String xml = "<root xmlns:a=\"urn:a\">\n  <item a:k=\"v\" plain=\"p\">t</item>\n</root>";
        SAXParserFactory f = SAXParserFactory.newInstance();
        f.setNamespaceAware(true);
        f.newSAXParser().parse(new InputSource(new StringReader(xml)), new H());
    }
}
