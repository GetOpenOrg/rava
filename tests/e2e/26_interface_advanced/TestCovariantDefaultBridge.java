import java.util.ArrayList;
import java.util.List;
import java.util.ListIterator;

// 边界：接口默认方法协变收窄超接口方法的返回类型（编译器在接口里生成桥方法），实现类继承的
// 另一个默认方法（Iterable.forEach）经 `this.iterator()` 走到协变的默认方法（同 jline DefaultHistory）
public class TestCovariantDefaultBridge {
    interface History extends Iterable<String> {
        ListIterator<String> iterator(int index);

        default ListIterator<String> iterator() {
            return iterator(0);
        }
    }

    static class Hist implements History {
        final List<String> items = new ArrayList<>();

        public ListIterator<String> iterator(int index) {
            return items.listIterator(index);
        }
    }

    public static void main(String[] args) {
        Hist h = new Hist();
        h.items.add("one");
        h.items.add("two");
        h.items.add("three");
        StringBuilder sb = new StringBuilder();
        h.forEach(s -> sb.append(s).append(';'));
        System.out.println(sb);
        Iterable<String> it = h;
        int n = 0;
        for (String s : it) {
            n += s.length();
        }
        System.out.println(n);
        ListIterator<String> li = h.iterator();
        li.next();
        System.out.println(li.nextIndex());
    }
}
