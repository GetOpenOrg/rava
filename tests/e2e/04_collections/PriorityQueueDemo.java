import java.util.PriorityQueue;

public class PriorityQueueDemo {
    public static void main(String[] args) {
        PriorityQueue<Integer> pq = new PriorityQueue<>();
        pq.add(5);
        pq.add(1);
        pq.add(4);
        pq.add(2);
        pq.add(3);
        System.out.println(pq.size()); // 5
        System.out.println(pq.poll()); // 1
        System.out.println(pq.poll()); // 2
        System.out.println(pq.peek()); // 3
        System.out.println(pq.size()); // 3
    }
}
