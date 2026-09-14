import java.util.ArrayDeque;
import java.util.Deque;

public class TestArrayDeque {
    public static void main(String[] args) {
        // Use as queue (FIFO)
        Deque<Integer> queue = new ArrayDeque<>();
        queue.offer(1);
        queue.offer(2);
        queue.offer(3);
        System.out.println(queue.poll());
        System.out.println(queue.peek());
        System.out.println(queue.size());

        // Use as stack (LIFO)
        Deque<String> stack = new ArrayDeque<>();
        stack.push("first");
        stack.push("second");
        stack.push("third");
        System.out.println(stack.pop());
        System.out.println(stack.pop());
        System.out.println(stack.size());
    }
}
