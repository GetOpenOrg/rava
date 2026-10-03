import java.beans.BeanInfo;
import java.beans.IndexedPropertyDescriptor;
import java.beans.Introspector;
import java.beans.MethodDescriptor;
import java.beans.PropertyDescriptor;
import java.util.Arrays;
import java.util.Comparator;

/**
 * beans 索引属性与方法描述符（方法级实测：getIndexedReadMethod 6 /
 * Introspector.decapitalize 6 / getMethodDescriptors 5 / getIndexedWriteMethod 5，
 * 此前零覆盖）：数组属性的索引访问器发现与反射调用、命名缩写规则。
 */
public class TestBeansIndexedProps {

    public static class TagBag {
        private String[] tags = new String[4];

        public String getTags(int i) {
            return tags[i];
        }

        public void setTags(int i, String v) {
            tags[i] = v;
        }

        public String[] getTags() {
            return tags;
        }

        public void setTags(String[] all) {
            this.tags = all;
        }

        public int total() {
            return tags.length;
        }
    }

    public static void main(String[] args) throws Exception {
        BeanInfo bi = Introspector.getBeanInfo(TagBag.class, Object.class);
        for (PropertyDescriptor p : bi.getPropertyDescriptors()) {
            if (p instanceof IndexedPropertyDescriptor ip) {
                System.out.println("indexed=" + p.getName()
                        + " idx-read=" + (ip.getIndexedReadMethod() != null)
                        + " idx-write=" + (ip.getIndexedWriteMethod() != null)
                        + " array-read=" + (ip.getReadMethod() != null));
            }
        }

        // 索引访问器的反射调用
        IndexedPropertyDescriptor ipd = (IndexedPropertyDescriptor) Arrays.stream(
                bi.getPropertyDescriptors())
                .filter(p -> p.getName().equals("tags"))
                .findFirst().orElseThrow();
        TagBag bag = new TagBag();
        ipd.getIndexedWriteMethod().invoke(bag, 1, "idx-value");
        System.out.println("via-indexed=" + ipd.getIndexedReadMethod().invoke(bag, 1));

        // 方法描述符（排序打印名字）
        MethodDescriptor[] mds = bi.getMethodDescriptors();
        Arrays.stream(mds).map(d -> d.getMethod().getName())
                .filter(n -> n.contains("Tags") || n.equals("total"))
                .sorted()
                .forEach(n -> System.out.println("md=" + n));

        // decapitalize：两连大写保留、单大写缩写
        System.out.println("decap-UserName=" + Introspector.decapitalize("UserName"));
        System.out.println("decap-URL=" + Introspector.decapitalize("URL"));
        System.out.println("decap-A=" + Introspector.decapitalize("A"));
        System.out.println("decap-lower=" + Introspector.decapitalize("already"));
    }
}
