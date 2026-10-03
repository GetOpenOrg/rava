import java.io.StringReader;
import java.io.StringWriter;

import javax.xml.transform.Transformer;
import javax.xml.transform.TransformerFactory;
import javax.xml.transform.dom.DOMResult;
import javax.xml.transform.dom.DOMSource;
import javax.xml.transform.stream.StreamResult;
import javax.xml.transform.stream.StreamSource;

import org.w3c.dom.Document;
import org.w3c.dom.NamedNodeMap;
import org.w3c.dom.Node;

/**
 * Transformer 的 DOM 通道与节点属性（方法级实测：DOMResult.getNode 6 jar /
 * Node.getAttributes 5 jar，此前零覆盖）：源→DOM 树、DOM→串、NamedNodeMap 读取。
 */
public class TestDomResultNode {

    public static void main(String[] args) throws Exception {
        TransformerFactory tf = TransformerFactory.newInstance();

        // Stream → DOMResult：getNode 取回构建的树
        Transformer t = tf.newTransformer();
        DOMResult dr = new DOMResult();
        t.transform(new StreamSource(new StringReader(
                "<cfg ver=\"2\"><k>v</k><empty/></cfg>")), dr);
        Node node = dr.getNode();
        System.out.println("node-type-doc=" + (node instanceof Document));
        Document doc = (Document) node;
        System.out.println("root=" + doc.getDocumentElement().getTagName());

        // Node.getAttributes：元素上有、文本节点无
        NamedNodeMap attrs = doc.getDocumentElement().getAttributes();
        System.out.println("attr-len=" + attrs.getLength()
                + " ver=" + attrs.getNamedItem("ver").getNodeValue());
        System.out.println("attr-item=" + attrs.item(0).getNodeName());
        Node text = doc.getDocumentElement().getFirstChild().getFirstChild();
        System.out.println("text-attrs-null=" + (text.getAttributes() == null));

        // DOM → 串（DOMSource 通道）
        Transformer back = tf.newTransformer();
        back.setOutputProperty(javax.xml.transform.OutputKeys.OMIT_XML_DECLARATION, "yes");
        StringWriter sw = new StringWriter();
        back.transform(new DOMSource(doc), new StreamResult(sw));
        System.out.println("roundtrip=" + sw.toString());

        // 树编辑后再次序列化
        doc.getDocumentElement().setAttribute("added", "yes");
        StringWriter sw2 = new StringWriter();
        back.transform(new DOMSource(doc), new StreamResult(sw2));
        System.out.println("edited-contains=" + sw2.toString().contains("added=\"yes\""));
    }
}
