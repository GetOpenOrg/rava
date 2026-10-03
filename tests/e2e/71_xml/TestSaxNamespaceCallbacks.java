import java.io.StringReader;

import javax.xml.parsers.SAXParserFactory;

import org.xml.sax.Attributes;
import org.xml.sax.InputSource;
import org.xml.sax.helpers.AttributesImpl;
import org.xml.sax.helpers.DefaultHandler;

/**
 * SAX 命名空间与全回调序（方法级实测：startDocument/endDocument/startPrefixMapping/
 * endPrefixMapping/processingInstruction/setDocumentLocator/setEntityResolver/getXMLReader/
 * AttributesImpl.addAttribute 各 5-10 jar，此前零覆盖）：前缀映射回调、PI 回调、
 * AttributesImpl 写侧、getXMLReader 通道。
 */
public class TestSaxNamespaceCallbacks {

    static class Full extends DefaultHandler {
        @Override
        public void startDocument() {
            System.out.println("startDocument");
        }

        @Override
        public void endDocument() {
            System.out.println("endDocument");
        }

        @Override
        public void startPrefixMapping(String prefix, String uri) {
            System.out.println("prefix+ " + prefix + "=" + uri);
        }

        @Override
        public void endPrefixMapping(String prefix) {
            System.out.println("prefix- " + prefix);
        }

        @Override
        public void processingInstruction(String target, String data) {
            System.out.println("pi " + target + " [" + data + "]");
        }

        @Override
        public void startElement(String uri, String local, String qName, Attributes attrs) {
            StringBuilder sb = new StringBuilder("start " + local + " uri=[" + uri + "]");
            for (int i = 0; i < attrs.getLength(); i++) {
                sb.append(" ").append(attrs.getLocalName(i)).append("=").append(attrs.getValue(i));
            }
            System.out.println(sb);
        }

        @Override
        public void endElement(String uri, String local, String qName) {
            System.out.println("end " + local);
        }
    }

    public static void main(String[] args) throws Exception {
        String xml = "<?e2e flag='on'?><root xmlns:a=\"urn:a\" xmlns=\"urn:default\">"
                + "<a:item a:k=\"v\">x</a:item><plain/></root>";
        SAXParserFactory f = SAXParserFactory.newInstance();
        f.setNamespaceAware(true);
        org.xml.sax.XMLReader reader = f.newSAXParser().getXMLReader();
        Full handler = new Full();
        reader.setContentHandler(handler);
        reader.setErrorHandler(handler);
        reader.parse(new InputSource(new StringReader(xml)));

        // InputSource 的 systemId/publicId 形态
        InputSource src = new InputSource(new StringReader("<z/>"));
        src.setSystemId("mem://e2e");
        src.setPublicId("pub-e2e");
        System.out.println("sysid=" + src.getSystemId() + " pubid=" + src.getPublicId());

        // AttributesImpl 写侧（SAX 过滤器/构造器的地基）
        AttributesImpl ai = new AttributesImpl();
        ai.addAttribute("urn:a", "k", "a:k", "CDATA", "v1");
        ai.addAttribute("", "plain", "plain", "NMTOKEN", "v2");
        System.out.println("ai-len=" + ai.getLength() + " first=" + ai.getValue(0)
                + " by-q=" + ai.getValue("a:k"));
        ai.setAttribute(0, "urn:a", "k", "a:k", "CDATA", "v1b");
        System.out.println("ai-set=" + ai.getValue(0));
        ai.removeAttribute(1);
        System.out.println("ai-after-remove=" + ai.getLength());
    }
}
