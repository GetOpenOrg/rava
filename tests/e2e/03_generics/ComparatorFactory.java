import java.util.*;

public class ComparatorFactory {
    public static void main(String[] args) {
        List<String> words = new ArrayList<>();
        words.add("banana");
        words.add("apple");
        words.add("cherry");

        // comparing() with key extractor
        words.sort(Comparator.comparing(s -> s));
        System.out.println(words.get(0)); // apple

        // reversed()
        Comparator<String> byName = Comparator.comparing(s -> s);
        Comparator<String> rev = byName.reversed();
        words.sort(rev);
        System.out.println(words.get(0)); // cherry

        // thenComparing() for tie-breaking
        List<String> words2 = new ArrayList<>();
        words2.add("bb");
        words2.add("aa");
        words2.add("ba");
        Comparator<String> byLen = Comparator.comparing(s -> String.valueOf(s.length()));
        Comparator<String> byLenThenName = byLen.thenComparing(Comparator.comparing(s -> s));
        words2.sort(byLenThenName);
        System.out.println(words2.get(0)); // aa
        System.out.println(words2.get(1)); // ba
        System.out.println(words2.get(2)); // bb
    }
}
