import java.beans.PropertyEditor;
import java.beans.PropertyEditorSupport;
import java.nio.charset.Charset;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.Collection;
import java.util.List;
import java.util.Locale;
import java.util.Map;
import java.util.Properties;
import java.util.Set;
import java.util.TreeMap;
import java.util.TreeSet;
import java.util.UUID;

import org.springframework.beans.BeanWrapperImpl;
import org.springframework.beans.SimpleTypeConverter;
import org.springframework.beans.TypeMismatchException;
import org.springframework.beans.propertyeditors.CharacterEditor;
import org.springframework.beans.propertyeditors.CharsetEditor;
import org.springframework.beans.propertyeditors.ClassEditor;
import org.springframework.beans.propertyeditors.CustomBooleanEditor;
import org.springframework.beans.propertyeditors.CustomCollectionEditor;
import org.springframework.beans.propertyeditors.CustomMapEditor;
import org.springframework.beans.propertyeditors.CustomNumberEditor;
import org.springframework.beans.propertyeditors.LocaleEditor;
import org.springframework.beans.propertyeditors.PatternEditor;
import org.springframework.beans.propertyeditors.PropertiesEditor;
import org.springframework.beans.propertyeditors.StringArrayPropertyEditor;
import org.springframework.beans.propertyeditors.StringTrimmerEditor;
import org.springframework.beans.propertyeditors.UUIDEditor;

/**
 * spring-beans 切片 sb3（矩阵 #12）：PropertyEditor 族与类型转换。
 *
 * 覆盖面：java.beans.PropertyEditorSupport 子类（CustomNumberEditor / CustomBooleanEditor /
 * StringTrimmerEditor / CharacterEditor / ClassEditor / CharsetEditor / LocaleEditor /
 * PatternEditor / PropertiesEditor / StringArrayPropertyEditor / UUIDEditor /
 * CustomCollectionEditor / CustomMapEditor）的 setAsText / getAsText / setValue；
 * SimpleTypeConverter.convertIfNecessary（缺省编辑器、数组 / 集合 / 枚举、TypeMismatchException）；
 * registerCustomEditor（按类型 / 按属性路径）与 findCustomEditor；用户自定义 PropertyEditorSupport
 * 子类经 BeanWrapperImpl 生效。无 cglib、无容器。
 */
public class SpringPropertyEditorMain {

    public enum Mode { FAST, SAFE }

    /** 用户自定义编辑器："x,y" → Point。 */
    public static class Point {
        final int x;
        final int y;

        public Point(int x, int y) {
            this.x = x;
            this.y = y;
        }

        @Override
        public String toString() { return "Point(" + x + "," + y + ")"; }
    }

    public static class PointEditor extends PropertyEditorSupport {
        @Override
        public void setAsText(String text) {
            String[] parts = text.split(",");
            setValue(new Point(Integer.parseInt(parts[0].trim()), Integer.parseInt(parts[1].trim())));
        }

        @Override
        public String getAsText() {
            Point p = (Point) getValue();
            return p == null ? "" : p.x + "," + p.y;
        }
    }

    public static class Settings {
        private Point origin;
        private String title;
        private String note;
        private Integer limit;
        private Mode mode;
        private Set<Integer> ports;

        public Point getOrigin() { return origin; }
        public void setOrigin(Point origin) { this.origin = origin; }
        public String getTitle() { return title; }
        public void setTitle(String title) { this.title = title; }
        public String getNote() { return note; }
        public void setNote(String note) { this.note = note; }
        public Integer getLimit() { return limit; }
        public void setLimit(Integer limit) { this.limit = limit; }
        public Mode getMode() { return mode; }
        public void setMode(Mode mode) { this.mode = mode; }
        public Set<Integer> getPorts() { return ports; }
        public void setPorts(Set<Integer> ports) { this.ports = ports; }
    }

    static void text(String label, PropertyEditor ed, String in) {
        try {
            ed.setAsText(in);
            Object v = ed.getValue();
            String shown = v instanceof Object[] ? Arrays.toString((Object[]) v) : String.valueOf(v);
            System.out.println(label + " setAsText(" + in + ") -> " + shown + " [" + (v == null ? "null" : v.getClass().getSimpleName())
                    + "] asText=" + ed.getAsText());
        } catch (IllegalArgumentException e) {
            System.out.println(label + " setAsText(" + in + ") -> " + e.getClass().getSimpleName() + ": " + e.getMessage());
        }
    }

    static void convert(SimpleTypeConverter tc, Object value, Class<?> type) {
        try {
            Object r = tc.convertIfNecessary(value, type);
            String shown = r instanceof int[] ? Arrays.toString((int[]) r)
                    : r instanceof Object[] ? Arrays.toString((Object[]) r) : String.valueOf(r);
            System.out.println("convert " + value + " -> " + type.getSimpleName() + " = " + shown
                    + " [" + (r == null ? "null" : r.getClass().getSimpleName()) + "]");
        } catch (TypeMismatchException e) {
            System.out.println("convert " + value + " -> " + type.getSimpleName() + " : TypeMismatchException code="
                    + e.getErrorCode() + " required=" + e.getRequiredType().getSimpleName());
        }
    }

    public static void main(String[] args) {
        // ── 单个编辑器 ──
        CustomNumberEditor intEd = new CustomNumberEditor(Integer.class, true);
        text("int", intEd, "42");
        text("int", intEd, "0x1F");
        text("int", intEd, "");
        text("int", intEd, "4x");
        CustomNumberEditor longEd = new CustomNumberEditor(Long.class, false);
        text("long", longEd, "-9000000000");
        text("long", longEd, "");
        intEd.setValue(7L);
        System.out.println("int setValue(7L) -> " + intEd.getValue() + " [" + intEd.getValue().getClass().getSimpleName() + "]");
        text("double", new CustomNumberEditor(Double.class, false), "2.5");

        CustomBooleanEditor boolEd = new CustomBooleanEditor(true);
        for (String s : Arrays.asList("true", "on", "YES", "1", "off", "no", "0", "")) {
            text("bool", boolEd, s);
        }
        text("bool", boolEd, "maybe");
        text("bool-custom", new CustomBooleanEditor("ja", "nein", false), "JA");

        text("trim", new StringTrimmerEditor(true), "  hello  ");
        text("trim", new StringTrimmerEditor(true), "   ");
        text("trim-del", new StringTrimmerEditor("-", false), " a-b-c ");
        text("char", new CharacterEditor(false), "z");
        text("char", new CharacterEditor(false), "\\u0041");
        text("char", new CharacterEditor(false), "ab");
        text("class", new ClassEditor(), "java.lang.String");
        text("class", new ClassEditor(), "int[]");
        text("class", new ClassEditor(), "no.such.Clazz");
        text("charset", new CharsetEditor(), "UTF-8");
        text("locale", new LocaleEditor(), "fr_CA");
        text("locale", new LocaleEditor(), "en-US");
        text("pattern", new PatternEditor(), "a+b*");
        text("uuid", new UUIDEditor(), "123e4567-e89b-12d3-a456-426614174000");
        text("strarr", new StringArrayPropertyEditor(), "a, b ,c");
        text("strarr-semi", new StringArrayPropertyEditor(";", true, true), " a ; ; b ");

        PropertiesEditor propsEd = new PropertiesEditor();
        propsEd.setAsText("a=1\n# comment\nb = two\nc:3");
        System.out.println("props -> " + new TreeMap<>((Properties) propsEd.getValue()));

        CustomCollectionEditor collEd = new CustomCollectionEditor(TreeSet.class);
        collEd.setValue(Arrays.asList("b", "a", "b"));
        System.out.println("coll(TreeSet) setValue -> " + collEd.getValue() + " [" + collEd.getValue().getClass().getSimpleName() + "]");
        collEd.setAsText("solo");
        System.out.println("coll(TreeSet) setAsText -> " + collEd.getValue());
        CustomCollectionEditor listEd = new CustomCollectionEditor(List.class, true);
        listEd.setValue(new String[] {"x", "y"});
        System.out.println("coll(List) array -> " + listEd.getValue() + " [" + listEd.getValue().getClass().getSimpleName() + "]");
        listEd.setValue(null);
        System.out.println("coll(List,nullAsEmpty) null -> " + listEd.getValue());
        CustomMapEditor mapEd = new CustomMapEditor(TreeMap.class);
        mapEd.setValue(Map.of("k2", 2, "k1", 1));
        System.out.println("map(TreeMap) -> " + mapEd.getValue() + " [" + mapEd.getValue().getClass().getSimpleName() + "]");

        // ── SimpleTypeConverter（缺省编辑器 + 转换规则） ──
        SimpleTypeConverter tc = new SimpleTypeConverter();
        convert(tc, "123", int.class);
        convert(tc, "123", Integer.class);
        convert(tc, "3.75", double.class);
        convert(tc, "1e3", java.math.BigDecimal.class);
        convert(tc, "123456789012345678901234567890", java.math.BigInteger.class);
        convert(tc, "true", boolean.class);
        convert(tc, "SAFE", Mode.class);
        convert(tc, "q", char.class);
        convert(tc, "java.util.ArrayList", Class.class);
        convert(tc, "de_DE", Locale.class);
        convert(tc, "ISO-8859-1", Charset.class);
        convert(tc, "123e4567-e89b-12d3-a456-426614174000", UUID.class);
        convert(tc, "1,2,3", int[].class);
        convert(tc, "a,b", String[].class);
        convert(tc, 5, String.class);
        convert(tc, 5L, Integer.class);
        convert(tc, "abc", int.class);
        convert(tc, "NOPE", Mode.class);
        convert(tc, "x=1", Properties.class);

        @SuppressWarnings("unchecked")
        Collection<Object> listIn = new ArrayList<>(Arrays.asList("4", "5"));
        Object arr = tc.convertIfNecessary(listIn, Integer[].class);
        System.out.println("convert list -> Integer[] = " + Arrays.toString((Integer[]) arr));

        tc.registerCustomEditor(Point.class, new PointEditor());
        convert(tc, "3, 4", Point.class);
        System.out.println("findCustomEditor(Point) = " + tc.findCustomEditor(Point.class, null).getClass().getSimpleName());
        System.out.println("findCustomEditor(String) = " + tc.findCustomEditor(String.class, null));
        System.out.println("hasCustomEditorForElement = " + tc.hasCustomEditorForElement(Point.class, null));
        PropertyEditor def = tc.getDefaultEditor(Locale.class);
        System.out.println("getDefaultEditor(Locale) = " + (def == null ? "null" : def.getClass().getSimpleName()));

        // ── BeanWrapper 上注册（按类型 / 按路径） ──
        Settings s = new Settings();
        BeanWrapperImpl bw = new BeanWrapperImpl(s);
        bw.registerCustomEditor(Point.class, new PointEditor());
        bw.registerCustomEditor(String.class, "title", new StringTrimmerEditor(true));
        bw.registerCustomEditor(Integer.class, "limit", new CustomNumberEditor(Integer.class, true));
        bw.setPropertyValue("origin", "10,20");
        bw.setPropertyValue("title", "   ");
        bw.setPropertyValue("note", "   ");
        bw.setPropertyValue("limit", "");
        bw.setPropertyValue("mode", "FAST");
        bw.setPropertyValue("ports", new String[] {"8080", "80", "8080"});
        System.out.println("settings origin=" + s.getOrigin() + " title=" + s.getTitle() + " note='" + s.getNote()
                + "' limit=" + s.getLimit() + " mode=" + s.getMode() + " ports=" + new TreeSet<>(s.getPorts())
                + " [" + s.getPorts().getClass().getSimpleName() + "]");
        System.out.println("bw.findCustomEditor(String,title) = " + bw.findCustomEditor(String.class, "title").getClass().getSimpleName());
        System.out.println("bw.findCustomEditor(String,note) = " + bw.findCustomEditor(String.class, "note"));
        try {
            bw.setPropertyValue("origin", "bad");
        } catch (RuntimeException e) {
            System.out.println("origin bad -> " + e.getClass().getSimpleName() + " cause=" + e.getCause().getClass().getSimpleName());
        }
    }
}
