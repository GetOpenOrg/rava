import java.util.ArrayList;
import java.util.List;
import java.util.HashMap;
import java.util.Map;

public class TestNestedGeneric {

    static class Pair<A, B> {
        A first;
        B second;

        Pair(A first, B second) {
            this.first = first;
            this.second = second;
        }

        @Override
        public String toString() { return "(" + first + ", " + second + ")"; }

        Pair<B, A> swap() { return new Pair<>(second, first); }
    }

    static class Box<T> {
        T value;
        Box(T value) { this.value = value; }
        <R> Box<R> map(java.util.function.Function<T, R> f) {
            return new Box<>(f.apply(value));
        }
        @Override
        public String toString() { return "Box[" + value + "]"; }
    }

    // nested generic: list of pairs
    static <K, V> List<Pair<K, V>> fromMap(Map<K, V> map) {
        List<Pair<K, V>> result = new ArrayList<>();
        for (Map.Entry<K, V> e : map.entrySet()) {
            result.add(new Pair<>(e.getKey(), e.getValue()));
        }
        return result;
    }

    static <T> List<List<T>> chunk(List<T> list, int size) {
        List<List<T>> result = new ArrayList<>();
        for (int i = 0; i < list.size(); i += size) {
            int end = Math.min(i + size, list.size());
            result.add(new ArrayList<>(list.subList(i, end)));
        }
        return result;
    }

    public static void main(String[] args) {
        Pair<String, Integer> p1 = new Pair<>("hello", 42);
        System.out.println(p1);          // (hello, 42)
        System.out.println(p1.swap());   // (42, hello)

        Pair<List<Integer>, String> nested = new Pair<>(new ArrayList<>(), "empty");
        nested.first.add(1);
        nested.first.add(2);
        System.out.println(nested.first.size()); // 2
        System.out.println(nested.second);       // empty

        // Box with map
        Box<Integer> box = new Box<>(21);
        Box<Integer> doubled = box.map(n -> n * 2);
        Box<String> strBox = doubled.map(Object::toString);
        System.out.println(box);     // Box[21]
        System.out.println(doubled); // Box[42]
        System.out.println(strBox);  // Box[42]

        // nested list
        List<Integer> nums = new ArrayList<>();
        for (int i = 1; i <= 9; i++) nums.add(i);

        List<List<Integer>> chunks = chunk(nums, 3);
        System.out.println(chunks.size()); // 3
        for (List<Integer> c : chunks) {
            System.out.println(c);
        }
        // [1, 2, 3], [4, 5, 6], [7, 8, 9]

        // Pair of pairs
        Pair<Pair<Integer, Integer>, String> coord = new Pair<>(new Pair<>(3, 4), "origin");
        System.out.println(coord.first);  // (3, 4)
        System.out.println(coord.second); // origin
    }
}
