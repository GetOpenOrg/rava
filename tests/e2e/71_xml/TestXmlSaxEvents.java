import java.io.StringReader;

import javax.xml.parsers.SAXParserFactory;

import org.xml.sax.Attributes;
import org.xml.sax.InputSource;
import org.xml.sax.SAXParseException;
import org.xml.sax.helpers.DefaultHandler;

/**
 * java.xml SAX 事件流（71_xml 先行件，预审模式不进基线）。
 *
 * 事件序（start/characters/end）、属性序、文本跨块累积在 endElement 处统一输出、
 * 格式错误时的行列号定位。
 */
public class TestXmlSaxEvents {

    static class Recorder extends DefaultHandler {
        final StringBuilder text = new StringBuilder();

        @Override
        public void startElement(String uri, String localName, String qName, Attributes attrs) {
            System.out.println("start " + qName + " uri=[" + uri + "]");
            for (int i = 0; i < attrs.getLength(); i++) {
                System.out.println("  attr " + attrs.getQName(i) + "=" + attrs.getValue(i));
            }
        }

        @Override
        public void characters(char[] ch, int start, int length) {
            text.append(ch, start, length);
        }

        @Override
        public void endElement(String uri, String localName, String qName) {
            if (text.length() > 0) {
                System.out.println("text " + qName + "=[" + text + "]");
                text.setLength(0);
            }
            System.out.println("end " + qName);
        }
    }

    public static void main(String[] args) throws Exception {
        String xml = "<log><entry level=\"INFO\">boot</entry><entry level=\"WARN\">retry</entry></log>";
        SAXParserFactory f = SAXParserFactory.newInstance();
        f.setNamespaceAware(true);
        f.newSAXParser().parse(new InputSource(new StringReader(xml)), new Recorder());

        // 格式错误：未闭合元素的行列号（只打印定位，不打印可变的错误消息文本）
        try {
            f.newSAXParser().parse(new InputSource(new StringReader("<a><b></a>")), new Recorder());
        } catch (SAXParseException e) {
            System.out.println("parse-ex line=" + e.getLineNumber()
                    + " col=" + e.getColumnNumber());
        }
    }
}
