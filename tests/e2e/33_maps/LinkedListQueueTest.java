import java.util.LinkedList;
import java.util.ArrayDeque;
import java.util.Stack;

public class LinkedListQueueTest {
    public static void main(String[] args) {
        // Test 1: LinkedList as List
        LinkedList<String> list = new LinkedList<>();
        list.add("A");
        list.add("B");
        list.add("C");
        System.out.println(list.size());
        System.out.println(list.get(0));
        System.out.println(list.get(2));

        // Test 2: LinkedList addFirst/addLast
        list.addFirst("Z");
        list.addLast("W");
        System.out.println(list.getFirst());
        System.out.println(list.getLast());
        System.out.println(list.size());

        // Test 3: LinkedList as Queue
        LinkedList<Integer> queue = new LinkedList<>();
        queue.offer(10);
        queue.offer(20);
        queue.offer(30);
        System.out.println(queue.peek());
        System.out.println(queue.poll());
        System.out.println(queue.poll());
        System.out.println(queue.size());

        // Test 4: Contains and indexOf
        System.out.println(list.contains("B"));
        System.out.println(list.contains("X"));
        System.out.println(list.indexOf("B"));

        // Test 5: Remove by index (list is: Z,A,B,C,W)
        list.remove(2);  // removes "B"
        System.out.println(list.size());
        System.out.println(list.contains("B"));

        // Test 6: Iteration
        LinkedList<String> items = new LinkedList<>();
        items.add("one");
        items.add("two");
        items.add("three");
        StringBuilder sb = new StringBuilder();
        for (String item : items) {
            if (sb.length() > 0) sb.append(",");
            sb.append(item);
        }
        System.out.println(sb.toString());

        // Test 7: ArrayDeque
        ArrayDeque<String> deque = new ArrayDeque<>();
        deque.add("X");
        deque.add("Y");
        deque.add("Z");
        System.out.println(deque.size());
        System.out.println(deque.peek());
        System.out.println(deque.poll());
        System.out.println(deque.size());

        // Test 8: Stack
        Stack<Integer> stack = new Stack<>();
        stack.push(1);
        stack.push(2);
        stack.push(3);
        System.out.println(stack.peek());
        System.out.println(stack.pop());
        System.out.println(stack.pop());
        System.out.println(stack.size());
        System.out.println(stack.empty());
        stack.pop();
        System.out.println(stack.empty());

        // Test 9: Queue FIFO vs Stack LIFO
        LinkedList<Integer> fifo = new LinkedList<>();
        fifo.offer(1);
        fifo.offer(2);
        fifo.offer(3);
        System.out.print("FIFO: ");
        while (!fifo.isEmpty()) {
            System.out.print(fifo.poll() + " ");
        }
        System.out.println();

        Stack<Integer> lifo = new Stack<>();
        lifo.push(1);
        lifo.push(2);
        lifo.push(3);
        System.out.print("LIFO: ");
        while (!lifo.empty()) {
            System.out.print(lifo.pop() + " ");
        }
        System.out.println();
    }
}
