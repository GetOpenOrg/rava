import java.util.LinkedList;
import java.util.Deque;

public class TestLinkedList {
    public static void main(String[] args) {
        LinkedList<Integer> list = new LinkedList<>();
        list.add(1);
        list.add(2);
        list.add(3);
        list.addFirst(0);
        list.addLast(4);

        System.out.println(list.size());
        System.out.println(list.getFirst());
        System.out.println(list.getLast());

        list.removeFirst();
        list.removeLast();
        System.out.println(list);

        // Use as Deque (stack behavior)
        Deque<String> stack = new LinkedList<>();
        stack.push("a");
        stack.push("b");
        stack.push("c");
        System.out.println(stack.pop());
        System.out.println(stack.peek());
    }
}
