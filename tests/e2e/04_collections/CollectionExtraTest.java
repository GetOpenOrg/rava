import java.util.LinkedList;
import java.util.ArrayDeque;
import java.util.Stack;

public class CollectionExtraTest {
    public static void main(String[] args) {
        // LinkedList as List
        LinkedList<String> list = new LinkedList<>();
        list.add("A");
        list.add("B");
        list.add("C");
        System.out.println("LinkedList: " + list);
        System.out.println("size: " + list.size());
        System.out.println("get(1): " + list.get(1));
        System.out.println("contains B: " + list.contains("B"));
        System.out.println("isEmpty: " + list.isEmpty());

        // LinkedList as Deque
        list.addFirst("Z");
        list.addLast("D");
        System.out.println("After addFirst/addLast: " + list);
        System.out.println("getFirst: " + list.getFirst());
        System.out.println("getLast: " + list.getLast());
        list.removeFirst();
        list.removeLast();
        System.out.println("After remove first/last: " + list);

        // LinkedList as Queue
        System.out.println("peek: " + list.peek());
        list.offer("E");
        System.out.println("After offer E: " + list);
        list.poll();
        System.out.println("After poll: " + list);

        // ArrayDeque
        ArrayDeque<Integer> deque = new ArrayDeque<>();
        deque.add(1);
        deque.add(2);
        deque.add(3);
        System.out.println("ArrayDeque: " + deque);
        deque.addFirst(0);
        deque.addLast(4);
        System.out.println("After addFirst/addLast: " + deque);
        System.out.println("peekFirst: " + deque.peekFirst());
        System.out.println("peekLast: " + deque.peekLast());
        deque.pollFirst();
        deque.pollLast();
        System.out.println("After poll first/last: " + deque);
        System.out.println("size: " + deque.size());

        // Stack
        Stack<String> stack = new Stack<>();
        stack.push("X");
        stack.push("Y");
        stack.push("Z");
        System.out.println("Stack: " + stack);
        System.out.println("peek: " + stack.peek());
        System.out.println("pop: " + stack.pop());
        System.out.println("After pop: " + stack);
        System.out.println("search X: " + stack.search("X"));
        System.out.println("search Y: " + stack.search("Y"));
        System.out.println("empty: " + stack.empty());
        System.out.println("size: " + stack.size());
    }
}
