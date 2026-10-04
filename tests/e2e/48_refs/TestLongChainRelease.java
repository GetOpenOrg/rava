// 长引用链的释放：单链表 / 泛型擦除字段链 / 数组链在主线程与虚拟线程上各建一条并整体丢弃。
// JVM 由 GC 回收，与链长无关；转译产物的对象释放必须同样不随链长加深调用栈。
public class TestLongChainRelease {
    static final class Node {
        final int value;
        Node next;
        Node(int value, Node next) { this.value = value; this.next = next; }
    }

    static final class Holder<T> {
        final T item;
        Holder(T item) { this.item = item; }
    }

    static long nodeChain(int n) {
        Node head = null;
        for (int i = 0; i < n; i++) {
            head = new Node(i, head);
        }
        long sum = 0;
        int len = 0;
        for (Node p = head; p != null; p = p.next) {
            sum += p.value;
            len++;
        }
        head = null;
        return sum * 31 + len;
    }

    static int holderChain(int n) {
        Object cur = "tail";
        for (int i = 0; i < n; i++) {
            cur = new Holder<Object>(cur);
        }
        int depth = 0;
        while (cur instanceof Holder<?> h) {
            cur = h.item;
            depth++;
        }
        return depth;
    }

    static int arrayChain(int n) {
        Object[] cur = null;
        for (int i = 0; i < n; i++) {
            cur = new Object[] { cur, i };
        }
        int depth = 0;
        while (cur != null) {
            cur = (Object[]) cur[0];
            depth++;
        }
        return depth;
    }

    static String run(String where) {
        StringBuilder sb = new StringBuilder();
        sb.append(where).append(" nodes=").append(nodeChain(1_000_000));
        sb.append(" holders=").append(holderChain(200_000));
        sb.append(" arrays=").append(arrayChain(200_000));
        return sb.toString();
    }

    public static void main(String[] args) throws Exception {
        System.out.println(run("main"));
        String[] out = new String[1];
        Thread vt = Thread.ofVirtual().start(() -> out[0] = run("virtual"));
        vt.join();
        System.out.println(out[0]);
        // 链在丢弃之前保持可达，丢弃发生在被调方返回时
        Node keep = null;
        for (int i = 0; i < 1_000_000; i++) {
            keep = new Node(i, keep);
        }
        System.out.println("kept head=" + keep.value);
        keep = null;
        System.out.println("done");
    }
}
