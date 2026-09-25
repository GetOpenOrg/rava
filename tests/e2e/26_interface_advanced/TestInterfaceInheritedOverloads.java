import java.util.ArrayDeque;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.Collection;
import java.util.Deque;
import java.util.LinkedHashSet;
import java.util.List;
import java.util.Queue;
import java.util.SequencedCollection;
import java.util.SequencedSet;

/**
 * 经**接口接收者**调用声明在超接口上的重载成员（Collection.toArray 三重载、
 * removeIf / stream / forEach 等 default）：接收者接口自身未声明，成员名须按
 * 声明接口的重载态解析（JDK25 ReverseOrderDequeView 的 Deque.toArray(IntFunction)）。
 */
public class TestInterfaceInheritedOverloads {
    interface Shape {
        String name();
        default String describe() { return "shape " + name(); }
        default String describe(int n) { return n + "x " + name(); }
    }
    interface Polygon extends Shape {
        int sides();
    }
    record Square() implements Polygon {
        public String name() { return "square"; }
        public int sides() { return 4; }
    }

    public static void main(String[] args) {
        Deque<String> d = new ArrayDeque<>(List.of("a", "b", "c"));
        // Deque 接收者 → Collection 的 toArray 三重载
        System.out.println("deque.toArray()=" + Arrays.toString(d.toArray()));
        System.out.println("deque.toArray(T[])=" + Arrays.toString(d.toArray(new String[0])));
        System.out.println("deque.toArray(IntFunction)=" + Arrays.toString(d.toArray(String[]::new)));
        // Deque 的逆序视图（ReverseOrderDequeView）
        Deque<String> rev = d.reversed();
        System.out.println("reversed.toArray(IntFunction)=" + Arrays.toString(rev.toArray(String[]::new)));
        System.out.println("reversed.toArray(T[])=" + Arrays.toString(rev.toArray(new String[3])));
        System.out.println("reversed.first=" + rev.getFirst() + ",last=" + rev.getLast());
        // Queue / SequencedCollection / Collection 接收者
        Queue<String> q = new ArrayDeque<>(List.of("x", "y"));
        System.out.println("queue.toArray(IntFunction)=" + Arrays.toString(q.toArray(String[]::new)));
        SequencedCollection<Integer> sc = new ArrayList<>(List.of(3, 1, 2));
        System.out.println("seq.reversed.toArray=" + Arrays.toString(sc.reversed().toArray(Integer[]::new)));
        SequencedSet<String> ss = new LinkedHashSet<>(List.of("p", "q", "r"));
        System.out.println("seqSet.reversed.toArray=" + Arrays.toString(ss.reversed().toArray(String[]::new)));
        Collection<String> c = d;
        System.out.println("coll.removeIf=" + c.removeIf(s -> s.equals("b")) + "," + d);
        StringBuilder sb = new StringBuilder();
        d.forEach(sb::append);
        System.out.println("deque.forEach=" + sb + ",stream=" + d.stream().count());
        // 用户接口：重载 default 声明在超接口，经子接口接收者调用
        Polygon p = new Square();
        System.out.println("user.describe()=" + p.describe() + ",describe(int)=" + p.describe(3)
                + ",sides=" + p.sides());
    }
}
