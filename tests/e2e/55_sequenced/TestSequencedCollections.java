import java.util.*;

public class TestSequencedCollections {
    public static void main(String[] args) {
        SequencedSet<String> ss = new LinkedHashSet<>();
        ss.add("a");
        ss.add("b");
        ss.add("c");
        System.out.println("first=" + ss.getFirst());
        System.out.println("last=" + ss.getLast());
        System.out.println("reversed=" + ss.reversed());
        ss.addFirst("x");
        ss.addLast("y");
        System.out.println("after=" + ss);

        SequencedMap<String, Integer> sm = new LinkedHashMap<>();
        sm.put("p", 1);
        sm.put("q", 2);
        System.out.println("mfirst=" + sm.firstEntry());
        System.out.println("mlast=" + sm.lastEntry());
        System.out.println("mreversed=" + sm.reversed());
    }
}
