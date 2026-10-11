import java.util.ArrayList;
import java.util.Arrays;
import java.util.LinkedHashMap;
import java.util.List;
import java.util.Map;

import org.springframework.beans.BeanWrapper;
import org.springframework.beans.BeanWrapperImpl;
import org.springframework.beans.BeansException;
import org.springframework.beans.MutablePropertyValues;
import org.springframework.beans.PropertyAccessorUtils;
import org.springframework.beans.PropertyBatchUpdateException;
import org.springframework.beans.PropertyValue;

/**
 * spring-beans 切片 sb2（矩阵 #12）：BeanWrapperImpl 属性读写与嵌套路径。
 *
 * 覆盖面：getPropertyValue / setPropertyValue（简单属性、String → 基本类型 / 包装类 / 枚举转换）、
 * 嵌套路径 a.b.c、autoGrowNestedPaths 自动建中间对象与集合元素、索引路径 list[i] / array[i] /
 * map[key]、isReadableProperty / isWritableProperty / getPropertyType、setPropertyValues
 * （MutablePropertyValues / Map / ignoreUnknown / 批量异常 PropertyBatchUpdateException）、
 * NotReadable / NotWritable / NullValueInNestedPath / TypeMismatch 异常、
 * PropertyAccessorUtils 路径工具、setWrappedInstance 换实例。无 cglib、无容器。
 */
public class SpringBeanWrapperMain {

    public enum Level { LOW, MEDIUM, HIGH }

    public static class Address {
        private String city;
        private String zip;
        private Geo geo;

        public String getCity() { return city; }
        public void setCity(String city) { this.city = city; }
        public String getZip() { return zip; }
        public void setZip(String zip) { this.zip = zip; }
        public Geo getGeo() { return geo; }
        public void setGeo(Geo geo) { this.geo = geo; }

        @Override
        public String toString() { return "Address{city=" + city + ", zip=" + zip + ", geo=" + geo + "}"; }
    }

    public static class Geo {
        private double lat;
        private double lng;

        public double getLat() { return lat; }
        public void setLat(double lat) { this.lat = lat; }
        public double getLng() { return lng; }
        public void setLng(double lng) { this.lng = lng; }

        @Override
        public String toString() { return "Geo(" + lat + "," + lng + ")"; }
    }

    public static class Customer {
        private String name;
        private int age;
        private Integer score;
        private long id;
        private boolean vip;
        private Level level;
        private Address address;
        private List<Address> history = new ArrayList<>();
        private List<String> tags;
        private Map<String, Integer> counters = new LinkedHashMap<>();
        private int[] lucky = new int[3];
        private String[] aliases;
        private String readOnly = "fixed";

        public String getName() { return name; }
        public void setName(String name) { this.name = name; }
        public int getAge() { return age; }
        public void setAge(int age) { this.age = age; }
        public Integer getScore() { return score; }
        public void setScore(Integer score) { this.score = score; }
        public long getId() { return id; }
        public void setId(long id) { this.id = id; }
        public boolean isVip() { return vip; }
        public void setVip(boolean vip) { this.vip = vip; }
        public Level getLevel() { return level; }
        public void setLevel(Level level) { this.level = level; }
        public Address getAddress() { return address; }
        public void setAddress(Address address) { this.address = address; }
        public List<Address> getHistory() { return history; }
        public void setHistory(List<Address> history) { this.history = history; }
        public List<String> getTags() { return tags; }
        public void setTags(List<String> tags) { this.tags = tags; }
        public Map<String, Integer> getCounters() { return counters; }
        public void setCounters(Map<String, Integer> counters) { this.counters = counters; }
        public int[] getLucky() { return lucky; }
        public void setLucky(int[] lucky) { this.lucky = lucky; }
        public String[] getAliases() { return aliases; }
        public void setAliases(String[] aliases) { this.aliases = aliases; }
        public String getReadOnly() { return readOnly; }
        public void setWriteOnly(String v) { this.name = "w:" + v; }
    }

    static void show(BeanWrapper bw, String path) {
        try {
            Object v = bw.getPropertyValue(path);
            String text = v instanceof int[] ? Arrays.toString((int[]) v)
                    : v instanceof Object[] ? Arrays.toString((Object[]) v) : String.valueOf(v);
            System.out.println("get " + path + " = " + text);
        } catch (BeansException e) {
            System.out.println("get " + path + " -> " + e.getClass().getSimpleName() + ": " + e.getMessage());
        }
    }

    static void set(BeanWrapper bw, String path, Object value) {
        try {
            bw.setPropertyValue(path, value);
            System.out.println("set " + path + " <- " + value + " ok");
        } catch (BeansException e) {
            System.out.println("set " + path + " <- " + value + " -> " + e.getClass().getSimpleName() + ": " + e.getMessage());
        }
    }

    public static void main(String[] args) {
        Customer c = new Customer();
        BeanWrapperImpl bw = new BeanWrapperImpl(c);
        System.out.println("wrappedClass = " + bw.getWrappedClass().getSimpleName());

        // ── 简单属性与 String 转换 ──
        set(bw, "name", "Dora");
        set(bw, "age", "37");
        set(bw, "score", "  12 ");
        set(bw, "score", 99);
        set(bw, "id", "9000000000");
        set(bw, "vip", "yes");
        set(bw, "level", "HIGH");
        set(bw, "tags", "red,green,blue");
        set(bw, "aliases", "x,y");
        for (String p : Arrays.asList("name", "age", "score", "id", "vip", "level", "tags", "aliases")) {
            show(bw, p);
        }

        // ── 类型不匹配 / 不可读写 ──
        set(bw, "age", "not-a-number");
        set(bw, "level", "EXTREME");
        set(bw, "readOnly", "x");
        set(bw, "missing", "x");
        show(bw, "missing");
        set(bw, "writeOnly", "v");
        show(bw, "name");
        show(bw, "writeOnly");

        // ── 嵌套路径 ──
        show(bw, "address.city");
        set(bw, "address.city", "Paris");
        Address a = new Address();
        a.setCity("Lyon");
        set(bw, "address", a);
        set(bw, "address.zip", "69001");
        show(bw, "address.city");
        show(bw, "address.zip");
        set(bw, "address.geo.lat", "45.75");
        // 嵌套访问器按路径缓存且继承创建时的 autoGrow 标志：换新包装器验证自动建中间对象
        BeanWrapperImpl grow = new BeanWrapperImpl(new Customer());
        grow.setAutoGrowNestedPaths(true);
        set(grow, "address.geo.lat", "45.75");
        set(grow, "address.geo.lng", 4.85);
        set(grow, "address.city", "Nice");
        show(grow, "address");
        bw.setAutoGrowNestedPaths(true);

        // ── 索引路径（list / array / map） ──
        set(bw, "history[0].city", "Berlin");
        set(bw, "history[2].city", "Rome");
        show(bw, "history[0].city");
        show(bw, "history[1]");
        show(bw, "history[2].city");
        System.out.println("history.size = " + c.getHistory().size());
        set(bw, "lucky[1]", "7");
        show(bw, "lucky");
        show(bw, "lucky[1]");
        set(bw, "counters[apples]", "5");
        set(bw, "counters['pears']", 8);
        show(bw, "counters[apples]");
        show(bw, "counters");
        set(bw, "tags[1]", "GREEN");
        show(bw, "tags");

        // ── 元信息 ──
        for (String p : Arrays.asList("name", "readOnly", "writeOnly", "address.geo.lat", "history[0].zip",
                "counters[apples]", "nope", "address.nope")) {
            System.out.println("meta " + p + " readable=" + bw.isReadableProperty(p) + " writable=" + bw.isWritableProperty(p));
        }
        for (String p : Arrays.asList("age", "score", "level", "address.geo", "history[0]", "lucky[0]", "tags")) {
            Class<?> t = bw.getPropertyType(p);
            System.out.println("type " + p + " = " + (t == null ? "null" : t.getSimpleName()));
        }

        // ── 批量设置 ──
        MutablePropertyValues pvs = new MutablePropertyValues();
        pvs.add("name", "Eve");
        pvs.add("age", "29");
        pvs.addPropertyValue(new PropertyValue("address.city", "Oslo"));
        pvs.add("unknown", "ignored");
        bw.setPropertyValues(pvs, true);
        show(bw, "name");
        show(bw, "age");
        show(bw, "address.city");
        System.out.println("pvs contains age = " + pvs.contains("age") + ", size = " + pvs.size());

        Map<String, Object> map = new LinkedHashMap<>();
        map.put("vip", "false");
        map.put("level", "LOW");
        bw.setPropertyValues(map);
        show(bw, "vip");
        show(bw, "level");

        MutablePropertyValues bad = new MutablePropertyValues();
        bad.add("age", "x1");
        bad.add("score", "x2");
        bad.add("name", "Fay");
        try {
            bw.setPropertyValues(bad);
        } catch (PropertyBatchUpdateException e) {
            System.out.println("batch errors = " + e.getExceptionCount());
            for (var pae : e.getPropertyAccessExceptions()) {
                System.out.println("  " + pae.getPropertyName() + " code=" + pae.getErrorCode() + " value=" + pae.getValue());
            }
            show(bw, "name");
        }

        // ── 换实例 / 路径工具 ──
        Customer other = new Customer();
        other.setName("Gus");
        bw.setWrappedInstance(other);
        show(bw, "name");
        show(bw, "address");
        System.out.println("canonical = " + PropertyAccessorUtils.canonicalPropertyName("map['key'].list[\"0\"]"));
        System.out.println("nested = " + PropertyAccessorUtils.isNestedOrIndexedProperty("a.b") + ","
                + PropertyAccessorUtils.isNestedOrIndexedProperty("a"));
        System.out.println("firstSep = " + PropertyAccessorUtils.getFirstNestedPropertySeparatorIndex("m[a.b].c")
                + " lastSep = " + PropertyAccessorUtils.getLastNestedPropertySeparatorIndex("m[a.b].c.d"));
        System.out.println("propertyName = " + PropertyAccessorUtils.getPropertyName("list[3]"));
        System.out.println("matches = " + PropertyAccessorUtils.matchesProperty("list[3]", "list"));
    }
}
