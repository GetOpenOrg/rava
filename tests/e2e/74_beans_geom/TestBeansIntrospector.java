import java.beans.BeanInfo;
import java.beans.Introspector;
import java.beans.PropertyDescriptor;
import java.util.Arrays;
import java.util.Comparator;

/**
 * java.beans：Introspector 属性发现与读写方法反射（jmod 覆盖计划第 4 步，
 * java.desktop 纯 Java 子集；getter/setter 反射标准件 = mybatis POJO 映射的地基）。
 */
public class TestBeansIntrospector {

    public static class Sample {
        private String name;

        public String getName() {
            return name;
        }

        public void setName(String name) {
            this.name = name;
        }

        public int getReadOnly() {
            return 7;
        }
    }

    public static void main(String[] args) throws Exception {
        BeanInfo bi = Introspector.getBeanInfo(Sample.class, Object.class);
        PropertyDescriptor[] pds = bi.getPropertyDescriptors();
        Arrays.sort(pds, Comparator.comparing(PropertyDescriptor::getName));
        for (PropertyDescriptor p : pds) {
            System.out.println("prop=" + p.getName()
                    + " read=" + (p.getReadMethod() != null)
                    + " write=" + (p.getWriteMethod() != null)
                    + " type=" + p.getPropertyType().getSimpleName());
        }

        Sample s = new Sample();
        for (PropertyDescriptor p : pds) {
            if (p.getName().equals("name") && p.getWriteMethod() != null) {
                p.getWriteMethod().invoke(s, "bean-value");
                System.out.println("via-read=" + p.getReadMethod().invoke(s));
            }
        }
        System.out.println("direct=" + s.getName());

        // 缓存语义：同类两次 getBeanInfo 返回同一实例
        BeanInfo bi2 = Introspector.getBeanInfo(Sample.class);
        System.out.println("cached=" + (bi2 == Introspector.getBeanInfo(Sample.class)));
    }
}
