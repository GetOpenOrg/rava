public class TestHoistShadow {

    static class Node {
        int val;
        Node next;
        Node(int v, Node n) { val = v; next = n; }
    }

    static class Tree {
        int val;
        Tree next;
        Tree(int v, Tree n) { val = v; next = n; }
    }

    // CHM.transfer 形态：外层循环内 if/else 分支首次赋值两个变量 + 内层循环
    // 分支再赋值 + 循环后读取。分支内的赋值不得被遮蔽 let 吞失。
    static int[] splitCount(Node head) {
        int loCount = 0;
        int hiCount = 0;
        for (int round = 0; round < 2; round++) {
            Node ln, hn;
            if ((round & 1) == 0) {
                ln = head;
                hn = null;
            } else {
                hn = head;
                ln = null;
            }
            for (Node p = head; p != null; p = p.next) {
                if ((p.val & 1) == 0) {
                    ln = new Node(p.val, ln);
                } else {
                    hn = new Node(p.val, hn);
                }
            }
            for (Node q = ln; q != null; q = q.next) {
                loCount += q.val;
            }
            for (Node q = hn; q != null; q = q.next) {
                hiCount += q.val;
            }
        }
        return new int[] { loCount, hiCount };
    }

    // 同名异型守卫：兄弟作用域里 Node p / Tree p 复用同一名字（javac 槽位
    // 复用），是两个变量，不得并入同一绑定
    static int sameNameKinds(Node head, Tree root) {
        int total = 0;
        for (Node p = head; p != null; p = p.next) {
            total += p.val;
        }
        for (Tree p = root; p != null; p = p.next) {
            total += p.val * 10;
        }
        return total;
    }

    // 同名异宽守卫：int i / long i 循环变量复用
    static int sameNameWidths(int n) {
        int s = 0;
        for (int i = 0; i < n; i++) {
            s += i;
        }
        long t = 0;
        for (long i = 1; i <= n; i++) {
            t += i * 2;
        }
        return s + (int) t;
    }

    public static void main(String[] args) {
        Node head = null;
        for (int i = 7; i >= 0; i--) {
            head = new Node(i, head);
        }
        int[] counts = splitCount(head);
        System.out.println("lo=" + counts[0]);
        System.out.println("hi=" + counts[1]);

        Tree root = new Tree(1, new Tree(2, new Tree(3, null)));
        System.out.println("kinds=" + sameNameKinds(head, root));
        System.out.println("widths=" + sameNameWidths(5));
    }
}
