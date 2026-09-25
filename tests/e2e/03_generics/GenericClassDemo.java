import java.util.ArrayList;
import java.util.List;

public class GenericClassDemo {
    // Generic container class
    static class Box<T> {
        private T value;

        public Box(T value) {
            this.value = value;
        }

        public T get() {
            return value;
        }

        public void set(T value) {
            this.value = value;
        }
    }

    // Generic pair class
    static class Pair<A, B> {
        private A first;
        private B second;

        public Pair(A first, B second) {
            this.first = first;
            this.second = second;
        }

        public A getFirst() { return first; }
        public B getSecond() { return second; }

        @Override
        public String toString() {
            return "(" + first + ", " + second + ")";
        }
    }

    // Generic stack using ArrayList
    static class Stack<T> {
        private List<T> items = new ArrayList<>();

        public void push(T item) {
            items.add(item);
        }

        public T pop() {
            return items.remove(items.size() - 1);
        }

        public T peek() {
            return items.get(items.size() - 1);
        }

        public int size() {
            return items.size();
        }

        public boolean isEmpty() {
            return items.isEmpty();
        }
    }

    public static void main(String[] args) {
        // Box<Integer>
        Box<Integer> intBox = new Box<>(42);
        System.out.println(intBox.get());
        intBox.set(100);
        System.out.println(intBox.get());

        // Box<String>
        Box<String> strBox = new Box<>("hello");
        System.out.println(strBox.get());

        // Pair<String, Integer>
        Pair<String, Integer> pair = new Pair<>("age", 25);
        System.out.println(pair.getFirst());
        System.out.println(pair.getSecond());

        // Generic Stack
        Stack<Integer> stack = new Stack<>();
        stack.push(1);
        stack.push(2);
        stack.push(3);
        System.out.println(stack.size());
        System.out.println(stack.pop());
        System.out.println(stack.peek());
        System.out.println(stack.isEmpty());
    }
}
