import javax.xml.namespace.QName;

/**
 * javax.xml.namespace.QName（方法级实测：getLocalPart 8 jar，此前整族零覆盖——
 * SOAP/JAXB/WS 路径的类型名载体）：三构造形态、prefix 不参与 equals、
 * toString/valueOf 往返。
 */
public class TestQNameFaces {

    public static void main(String[] args) {
        QName localOnly = new QName("root");
        QName full = new QName("urn:e2e", "child");
        QName prefixed = new QName("urn:e2e", "child", "ns1");

        System.out.println("local=" + localOnly.getLocalPart()
                + " ns-empty=" + localOnly.getNamespaceURI().isEmpty()
                + " prefix-default=" + prefixed.getPrefix());

        // equals：namespace+localpart 决定，prefix 无关
        System.out.println("eq-ignores-prefix=" + full.equals(prefixed));
        System.out.println("hash-eq=" + (full.hashCode() == prefixed.hashCode()));
        System.out.println("neq=" + !full.equals(localOnly));

        // toString / valueOf 往返（带 ns 的规范形态 {ns}local）
        String s = full.toString();
        System.out.println("to-string=" + s);
        System.out.println("value-of-roundtrip=" + QName.valueOf(s).equals(full));
        System.out.println("local-tostring=" + localOnly.toString());

        // 非法构造：空 localpart → IllegalArgumentException
        try {
            new QName("");
        } catch (IllegalArgumentException e) {
            System.out.println("empty-local-ex=" + e.getClass().getSimpleName());
        }
        try {
            new QName("urn", "");
        } catch (IllegalArgumentException e) {
            System.out.println("empty-with-ns-ex=" + e.getClass().getSimpleName());
        }
    }
}
