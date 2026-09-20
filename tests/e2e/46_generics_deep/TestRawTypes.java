import java.util.*;

public class TestRawTypes {
    public static void main(String[] args) {
        List raw = new ArrayList();
        raw.add("hello");
        raw.add(42);
        System.out.println("raw0=" + raw.get(0));
        System.out.println("raw1=" + raw.get(1));
        System.out.println("size=" + raw.size());
        List<String> typed = raw;
        System.out.println("typed0=" + typed.get(0));
        Map m = new HashMap();
        m.put("k", 1);
        System.out.println("mapGet=" + m.get("k"));
        System.out.println("mapSize=" + m.size());
        System.out.println("listClass=" + raw.getClass().getSimpleName());
    }
}
