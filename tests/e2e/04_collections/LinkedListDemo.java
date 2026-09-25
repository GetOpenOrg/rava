import java.util.LinkedList;

public class LinkedListDemo {
    public static void main(String[] args) {
        LinkedList<String> queue = new LinkedList<>();
        queue.add("first");
        queue.add("second");
        queue.add("third");
        System.out.println(queue.size());
        System.out.println(queue.poll());
        System.out.println(queue.peekLast());
        queue.addFirst("zero");
        System.out.println(queue.getFirst());
    }
}
