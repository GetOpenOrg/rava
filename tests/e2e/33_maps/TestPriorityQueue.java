import java.util.ArrayList;
import java.util.Comparator;
import java.util.List;
import java.util.PriorityQueue;

class Job implements Comparable<Job> {
    final String name;
    final int priority;

    Job(String name, int priority) {
        this.name = name;
        this.priority = priority;
    }

    @Override
    public int compareTo(Job other) {
        return Integer.compare(other.priority, this.priority); // 高优先级先出
    }

    @Override
    public String toString() {
        return name + "(" + priority + ")";
    }
}

public class TestPriorityQueue {

    public static void main(String[] args) {
        // 自然序（小顶堆）
        PriorityQueue<Integer> pq = new PriorityQueue<>();
        int[] raw = {5, 1, 9, 3, 7, 1};
        for (int v : raw) {
            pq.offer(v);
        }
        System.out.println("size=" + pq.size());
        System.out.println("peek=" + pq.peek());

        List<String> drained = new ArrayList<>();
        while (!pq.isEmpty()) {
            drained.add(String.valueOf(pq.poll()));
        }
        System.out.println("drained=" + drained);

        // poll 空队列返回 null，peek 也是；element() 抛异常
        System.out.println("poll on empty=" + pq.poll());
        System.out.println("peek on empty=" + pq.peek());
        try {
            pq.element();
        } catch (RuntimeException e) {
            System.out.println("element on empty -> " + e.getClass().getSimpleName());
        }

        // 自定义 Comparator（大顶堆）
        PriorityQueue<Integer> maxHeap = new PriorityQueue<>(Comparator.reverseOrder());
        for (int v : raw) {
            maxHeap.add(v);
        }
        List<String> desc = new ArrayList<>();
        while (!maxHeap.isEmpty()) {
            desc.add(String.valueOf(maxHeap.remove()));
        }
        System.out.println("descending=" + desc);

        // remove(Object) 移除中间元素
        PriorityQueue<String> words = new PriorityQueue<>();
        words.offer("delta");
        words.offer("alpha");
        words.offer("charlie");
        System.out.println("removed charlie=" + words.remove("charlie"));
        System.out.println("words size=" + words.size());
        List<String> ordered = new ArrayList<>();
        while (!words.isEmpty()) {
            ordered.add(words.poll());
        }
        System.out.println("ordered=" + ordered);

        // Comparable 元素
        PriorityQueue<Job> jobs = new PriorityQueue<>();
        jobs.offer(new Job("low", 1));
        jobs.offer(new Job("high", 9));
        jobs.offer(new Job("mid", 5));
        List<String> executed = new ArrayList<>();
        while (!jobs.isEmpty()) {
            executed.add(jobs.poll().toString());
        }
        System.out.println("jobs=" + executed);

        // 迭代顺序不保证有序（排序后打印）
        PriorityQueue<Integer> mixed = new PriorityQueue<>();
        mixed.addAll(List.of(8, 2, 6, 4));
        List<Integer> snapshot = new ArrayList<>(mixed);
        java.util.Collections.sort(snapshot);
        System.out.println("sorted snapshot=" + snapshot);
        System.out.println("head=" + mixed.poll());

        System.out.println("done");
    }
}
