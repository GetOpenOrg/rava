import java.util.ArrayList;
import java.util.Collections;

public class GenericMethodTest {

    // Generic static method with Comparable bound
    static <T extends Comparable<T>> T max(T a, T b) {
        return a.compareTo(b) >= 0 ? a : b;
    }

    static <T extends Comparable<T>> T min(T a, T b) {
        return a.compareTo(b) <= 0 ? a : b;
    }

    // Generic container class
    static class Box<T> {
        private T value;
        Box(T value) { this.value = value; }
        T get() { return value; }
        void set(T value) { this.value = value; }

        @Override
        public String toString() {
            return "Box[" + value + "]";
        }
    }

    // Generic swap
    static <T> void swap(ArrayList<T> list, int i, int j) {
        T tmp = list.get(i);
        list.set(i, list.get(j));
        list.set(j, tmp);
    }

    // Bounded generic method: count elements matching condition
    static <T extends Comparable<T>> int countGreaterThan(ArrayList<T> list, T threshold) {
        int count = 0;
        for (int i = 0; i < list.size(); i++) {
            T item = list.get(i);
            if (item.compareTo(threshold) > 0) {
                count++;
            }
        }
        return count;
    }

    // Generic pair
    static class Pair<A, B> {
        A first;
        B second;
        Pair(A first, B second) {
            this.first = first;
            this.second = second;
        }
        @Override
        public String toString() {
            return "(" + first + ", " + second + ")";
        }
    }

    public static void main(String[] args) {
        // Test 1: Generic static methods
        System.out.println(max("apple", "banana"));   // banana
        System.out.println(max("zoo", "ant"));         // zoo
        System.out.println(min(10, 20));               // 10
        System.out.println(min(99, 1));                // 1

        // Test 2: Generic class
        Box<String> strBox = new Box<>("hello");
        System.out.println(strBox.get());              // hello
        System.out.println(strBox);                    // Box[hello]
        strBox.set("world");
        System.out.println(strBox.get());              // world

        Box<Integer> intBox = new Box<>(42);
        System.out.println(intBox.get());              // 42

        // Test 3: Generic swap
        ArrayList<String> list = new ArrayList<>();
        list.add("a");
        list.add("b");
        list.add("c");
        swap(list, 0, 2);
        System.out.println(list.get(0) + list.get(1) + list.get(2)); // cba

        // Test 4: Bounded generic method
        ArrayList<Integer> numbers = new ArrayList<>();
        numbers.add(1);
        numbers.add(5);
        numbers.add(3);
        numbers.add(8);
        numbers.add(2);
        numbers.add(7);
        System.out.println(countGreaterThan(numbers, 4)); // 3 (5, 8, 7)

        // Test 5: Generic pair
        Pair<String, Integer> pair = new Pair<>("age", 25);
        System.out.println(pair);                      // (age, 25)
        System.out.println(pair.first);                // age
        System.out.println(pair.second);               // 25

        // Test 6: Generic class with sorting
        ArrayList<String> words = new ArrayList<>();
        words.add("cherry");
        words.add("apple");
        words.add("banana");
        Collections.sort(words);
        for (int i = 0; i < words.size(); i++) {
            System.out.println(words.get(i));
        }
        // apple, banana, cherry

        // Test 7: Nested generics
        Box<Box<String>> nested = new Box<>(new Box<>("inner"));
        System.out.println(nested.get().get());        // inner
        System.out.println(nested);                    // Box[Box[inner]]
    }
}
