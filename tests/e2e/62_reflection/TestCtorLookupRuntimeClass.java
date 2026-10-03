import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.Map;
import java.util.TreeMap;
import java.util.function.Supplier;

/**
 * 构造器查找的 Class 值来自运行期对象（getClass），对象先经容器、Object 返回值、lambda 流转：
 *   - List<Object> 元素、Map 值、Supplier 结果上的 getClass().getDeclaredConstructor().newInstance()；
 *   - 目标含 JDK 类（ArrayList / TreeMap / StringBuilder）与用户类（含嵌套静态类）；
 *   - 序列化之外的「按运行期类复制」：同一查找点上值集为多个类。
 */
public class TestCtorLookupRuntimeClass {
    static class Box {
        final String tag;
        public Box() { this.tag = "fresh-box"; }
        Box(String tag) { this.tag = tag; }
        @Override public String toString() { return "Box(" + tag + ")"; }
    }

    static Object pick(int i) {
        switch (i) {
            case 0: return new ArrayList<String>(List.of("x"));
            case 1: return new TreeMap<String, Integer>(Map.of("k", 1));
            case 2: return new StringBuilder("sb");
            default: return new Box("old");
        }
    }

    static Object freshOf(Object o) throws ReflectiveOperationException {
        return o.getClass().getDeclaredConstructor().newInstance();
    }

    public static void main(String[] args) throws Exception {
        List<Object> items = new ArrayList<>();
        for (int i = 0; i < 4; i++) {
            items.add(pick(i));
        }
        for (Object o : items) {
            Object f = freshOf(o);
            System.out.println(o.getClass().getSimpleName() + " " + o + " -> " + f.getClass().getSimpleName() + " [" + f + "]");
        }

        Map<String, Object> byName = new HashMap<>();
        byName.put("deque", new java.util.ArrayDeque<Integer>(List.of(7)));
        byName.put("box", new Box("mapped"));
        for (String k : new String[] {"deque", "box"}) {
            Object f = freshOf(byName.get(k));
            System.out.println("map " + k + " -> " + f.getClass().getSimpleName() + " [" + f + "]");
        }

        Supplier<Object> s = () -> new java.util.LinkedList<String>(List.of("q"));
        Object f = freshOf(s.get());
        System.out.println("supplier -> " + f.getClass().getSimpleName() + " [" + f + "] empty=" + ((java.util.LinkedList<?>) f).isEmpty());
    }
}
