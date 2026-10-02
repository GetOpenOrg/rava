import java.io.StringReader;
import java.io.StringWriter;

import javax.xml.transform.OutputKeys;
import javax.xml.transform.Templates;
import javax.xml.transform.Transformer;
import javax.xml.transform.TransformerFactory;
import javax.xml.transform.stream.StreamResult;
import javax.xml.transform.stream.StreamSource;

/**
 * java.xml Transformer 序列化与 XSLT 模板（71_xml 先行件，预审模式不进基线）。
 *
 * identity 转换的逐字序列化（显式输出属性避免缩进差异）、XML 声明形态、
 * Templates 复用的一致性——闭包含 XSLT 编译器（javax.xml.transform.* 整栈）。
 */
public class TestXmlTransform {

    public static void main(String[] args) throws Exception {
        String xml = "<root><a x=\"1\">one</a><b>two</b></root>";
        TransformerFactory tf = TransformerFactory.newInstance();

        Transformer t1 = tf.newTransformer();
        t1.setOutputProperty(OutputKeys.OMIT_XML_DECLARATION, "yes");
        StringWriter sw1 = new StringWriter();
        t1.transform(new StreamSource(new StringReader(xml)), new StreamResult(sw1));
        System.out.println("identity=[" + sw1 + "]");

        Transformer t2 = tf.newTransformer();
        t2.setOutputProperty(OutputKeys.ENCODING, "UTF-8");
        StringWriter sw2 = new StringWriter();
        t2.transform(new StreamSource(new StringReader(xml)), new StreamResult(sw2));
        String out2 = sw2.toString();
        System.out.println("decl-head=" + out2.substring(0, out2.indexOf("?>") + 2));
        System.out.println("decl-newline-body="
                + (out2.charAt(out2.indexOf("?>") + 2) == '\n'));

        Templates tpl = tf.newTemplates(new StreamSource(new StringReader(
                "<xsl:stylesheet version=\"1.0\" xmlns:xsl=\"http://www.w3.org/1999/XSL/Transform\">"
                        + "<xsl:template match=\"/\"><out><xsl:value-of select=\"count(//*)\"/>"
                        + "</out></xsl:template></xsl:stylesheet>")));
        StringWriter sa = new StringWriter();
        StringWriter sb = new StringWriter();
        Transformer a = tpl.newTransformer();
        Transformer b = tpl.newTransformer();
        a.transform(new StreamSource(new StringReader(xml)), new StreamResult(sa));
        b.transform(new StreamSource(new StringReader(xml)), new StreamResult(sb));
        System.out.println("xsl-count-out=[" + sa + "]");
        System.out.println("template-reuse-eq=" + sa.toString().equals(sb.toString()));
    }
}
