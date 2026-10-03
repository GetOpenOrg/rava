import javax.xml.parsers.DocumentBuilderFactory;

import org.w3c.dom.Document;
import org.w3c.dom.Element;
import org.w3c.dom.Node;

/**
 * DOM 构造与写侧（方法级实测：newDocument/createElementNS/createTextNode/
 * appendChild/setAttributeNS/getOwnerDocument/getParentNode/getNodeType/
 * getNamespaceURI/getLocalPart/createProcessingInstruction 各 5-7 jar，
 * 此前只读不写零覆盖）：程序化建树 + 遍历验证结构。
 */
public class TestDomBuildTree {

    public static void main(String[] args) throws Exception {
        Document doc = DocumentBuilderFactory.newInstance().newDocumentBuilder().newDocument();

        Element root = doc.createElementNS("urn:e2e", "cfg");
        root.setAttributeNS("urn:e2e", "ver", "1");
        doc.appendChild(root);

        Element name = doc.createElementNS("urn:e2e", "name");
        name.appendChild(doc.createTextNode("rava"));
        root.appendChild(name);

        Element num = doc.createElementNS("urn:e2e", "num");
        num.setAttribute("x", "7");              // 无命名空间属性
        num.setTextContent("42");
        root.appendChild(num);

        doc.appendChild(doc.createProcessingInstruction("e2e-pi", "flag"));

        // 结构验证
        System.out.println("doc-elem=" + doc.getDocumentElement().getLocalName());
        System.out.println("root-owner-doc=" + (root.getOwnerDocument() == doc));
        System.out.println("name-parent=" + name.getParentNode().getLocalName());
        System.out.println("root-nodetype=" + root.getNodeType() + " text-type="
                + name.getFirstChild().getNodeType()
                + " doc-type=" + doc.getNodeType());
        System.out.println("ns-uri=" + root.getNamespaceURI()
                + " ns-attr=" + root.getAttributeNS("urn:e2e", "ver"));
        System.out.println("no-ns-attr=" + num.getAttribute("x"));
        System.out.println("children=" + root.getChildNodes().getLength());
        System.out.println("text=" + name.getTextContent() + " set-text=" + num.getTextContent());

        // PI 节点
        Node pi = doc.getLastChild();
        System.out.println("pi-type=" + pi.getNodeType() + " target=" + pi.getNodeName()
                + " data=" + pi.getNodeValue());

        // removeChild 与 reparent
        root.removeChild(name);
        System.out.println("after-remove=" + root.getChildNodes().getLength());
        num.appendChild(name);
        System.out.println("reparented=" + name.getParentNode().getLocalName());
    }
}
