import java.util.ArrayDeque;
import java.util.Deque;
import java.util.LinkedList;
import java.util.Queue;
import java.util.Stack;
import java.util.ArrayList;
import java.util.List;

public class QueueStackTest {
    public static void main(String[] args) {
        // 1. Queue (LinkedList as Queue)
        Queue<String> queue = new LinkedList<>();
        queue.add("first");
        queue.add("second");
        queue.add("third");
        System.out.println("queue size: " + queue.size());
        System.out.println("peek: " + queue.peek());
        System.out.println("poll: " + queue.poll());
        System.out.println("after poll size: " + queue.size());

        // 2. ArrayDeque as Queue
        Deque<Integer> deque = new ArrayDeque<>();
        deque.addLast(10);
        deque.addLast(20);
        deque.addLast(30);
        deque.addFirst(5);
        System.out.println("deque size: " + deque.size());
        System.out.println("first: " + deque.peekFirst());
        System.out.println("last: " + deque.peekLast());
        deque.pollFirst();
        deque.pollLast();
        System.out.println("after poll size: " + deque.size());

        // 3. Stack
        Stack<String> stack = new Stack<>();
        stack.push("a");
        stack.push("b");
        stack.push("c");
        System.out.println("stack size: " + stack.size());
        System.out.println("peek: " + stack.peek());
        System.out.println("pop: " + stack.pop());
        System.out.println("pop: " + stack.pop());
        System.out.println("after pop size: " + stack.size());
        System.out.println("search a: " + stack.search("a"));
        System.out.println("stack empty: " + stack.empty());

        // 4. LinkedList as Deque
        LinkedList<Integer> ll = new LinkedList<>();
        ll.add(1);
        ll.add(2);
        ll.add(3);
        ll.addFirst(0);
        ll.addLast(4);
        System.out.println("ll: " + ll);
        System.out.println("ll first: " + ll.getFirst());
        System.out.println("ll last: " + ll.getLast());
        ll.removeFirst();
        ll.removeLast();
        System.out.println("ll after remove: " + ll);

        System.out.println("Done.");
    }
}
