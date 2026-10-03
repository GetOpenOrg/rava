import java.io.StringReader;
import java.io.StringWriter;

import javax.xml.stream.XMLOutputFactory;
import javax.xml.stream.XMLInputFactory;
import javax.xml.stream.XMLStreamConstants;
import javax.xml.stream.XMLStreamReader;
import javax.xml.stream.XMLStreamWriter;

/**
 * java.xml StAX 拉式解析与写侧（71_xml 先行件，预审模式不进基线）。
 *
 * 读侧：XMLStreamReader 事件序（START_ELEMENT/CHARACTERS/END_ELEMENT）与属性访问；
 * 写侧：XMLStreamWriter 产出的序列化文本逐字可比。
 */
public class TestXmlStax {

    public static void main(String[] args) throws Exception {
        String xml = "<cfg debug=\"on\"><name>rava</name><nums><n>1</n><n>2</n></nums></cfg>";
        XMLInputFactory in = XMLInputFactory.newInstance();
        XMLStreamReader r = in.createXMLStreamReader(new StringReader(xml));
        while (r.hasNext()) {
            int ev = r.next();
            switch (ev) {
                case XMLStreamConstants.START_ELEMENT: {
                    StringBuilder line = new StringBuilder("start " + r.getLocalName());
                    for (int i = 0; i < r.getAttributeCount(); i++) {
                        line.append(" ").append(r.getAttributeLocalName(i))
                                .append("=").append(r.getAttributeValue(i));
                    }
                    System.out.println(line);
                    break;
                }
                case XMLStreamConstants.CHARACTERS: {
                    if (!r.isWhiteSpace()) {
                        System.out.println("text=" + r.getText());
                    }
                    break;
                }
                case XMLStreamConstants.END_ELEMENT: {
                    System.out.println("end " + r.getLocalName());
                    break;
                }
                default:
                    break;
            }
        }
        r.close();

        StringWriter sw = new StringWriter();
        XMLStreamWriter w = XMLOutputFactory.newInstance().createXMLStreamWriter(sw);
        w.writeStartDocument();
        w.writeStartElement("out");
        w.writeAttribute("k", "v");
        w.writeStartElement("inner");
        w.writeCharacters("text");
        w.writeEndElement();
        w.writeEndElement();
        w.writeEndDocument();
        w.close();
        System.out.println("written=[" + sw + "]");
    }
}
