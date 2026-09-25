public class StaticNestedClass {
    static class Node {
        int value;
        Node next;

        Node(int v) {
            value = v;
            next = null;
        }
    }

    static class LinkedList {
        Node head;

        LinkedList() {
            head = null;
        }

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
        list.add(3);
        list.add(2);
        list.add(1);
        list.print(); // prints 1 2 3
    }
}
