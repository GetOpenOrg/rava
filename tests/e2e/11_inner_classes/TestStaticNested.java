public class TestStaticNested {
    static class Node {
        int value;
        Node next;

        Node(int value) {
            this.value = value;
        }
    }

    static class LinkedList {
        Node head;

        void add(int v) {
            Node n = new Node(v);
            n.next = head;
            head = n;
        }

        void print() {
            Node cur = head;
            while (cur != null) {
                System.out.println(cur.value);
                cur = cur.next;
            }
        }
    }

    public static void main(String[] args) {
        LinkedList list = new LinkedList();
        list.add(1);
        list.add(2);
        list.add(3);
        list.print();
    }
}
