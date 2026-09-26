// FS-M7：null 接收者的 getfield / putfield 抛可捕获的 NullPointerException（JVMS §6.5）。
public class TestFieldNullReceiver {
    static class Node { int v; long l; double d; String s; Node next; }
    static class Box<T> { T val; }
    static int calls;
    static int compute() { calls++; System.out.println("compute ran"); return 7; }

    int own = 1;
    class Inner { int read() { return own + 1; } }

    static String npe(Runnable r) {
        try { r.run(); return "no exception"; }
        catch (NullPointerException e) { return "NPE"; }
    }

    public static void main(String[] args) {
        Node n = null;
        Node a = new Node();
        Box<String> box = null;
        Node[] arr = new Node[2];
        System.out.println("read int " + npe(() -> System.out.println(n.v)));
        System.out.println("read long " + npe(() -> System.out.println(n.l)));
        System.out.println("read double " + npe(() -> System.out.println(n.d)));
        System.out.println("read ref " + npe(() -> System.out.println(n.s)));
        System.out.println("write int " + npe(() -> n.v = 5));
        System.out.println("write ref " + npe(() -> n.s = "x"));
        System.out.println("write order " + npe(() -> n.v = compute()) + " calls=" + calls);
        System.out.println("compound " + npe(() -> n.l += 3));
        System.out.println("chain " + npe(() -> System.out.println(a.next.v)));
        System.out.println("chain write " + npe(() -> a.next.next = a));
        System.out.println("generic " + npe(() -> System.out.println(box.val)));
        System.out.println("array elem " + npe(() -> System.out.println(arr[1].s)));
        a.next = new Node();
        a.next.v = 41;
        a.next.v++;
        System.out.println("ok chain " + a.next.v + " " + npe(() -> a.next.s.length()));
        TestFieldNullReceiver outer = new TestFieldNullReceiver();
        System.out.println("inner " + outer.new Inner().read());
        int got;
        try {
            got = n.v;
        } catch (NullPointerException e) {
            got = -1;
        } finally {
            System.out.println("finally ran");
        }
        System.out.println("got " + got);
    }
}
