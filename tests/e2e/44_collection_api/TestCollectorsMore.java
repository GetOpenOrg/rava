import java.util.*;
import java.util.stream.*;
import java.util.Comparator;

public class TestCollectorsMore {
    public static void main(String[] args) {
        List<String> words = List.of("a", "bb", "cc", "ddd");
        Map<Integer, List<String>> byLen = words.stream()
                .collect(Collectors.groupingBy(String::length, TreeMap::new, Collectors.toList()));
        System.out.println("byLen=" + byLen);
        Map<Boolean, List<String>> part = words.stream()
                .collect(Collectors.partitioningBy(w -> w.length() > 1, Collectors.toList()));
        System.out.println("part=" + part);
        Map<Integer, String> toMap = words.stream()
                .collect(Collectors.toMap(String::length, w -> w, (a, b) -> a + b, TreeMap::new));
        System.out.println("toMap=" + toMap);
        Set<String> toSet = words.stream().collect(Collectors.toCollection(TreeSet::new));
        System.out.println("toSet=" + toSet);
        Double avg = words.stream().collect(Collectors.averagingInt(String::length));
        System.out.println("avg=" + avg);
        IntSummaryStatistics stat = words.stream().collect(Collectors.summarizingInt(String::length));
        System.out.println("sum=" + stat.getSum() + ",max=" + stat.getMax() + ",avg=" + stat.getAverage());
        String join = words.stream().collect(Collectors.joining("-"));
        System.out.println("join=" + join);
        Map<Integer, Integer> summing = words.stream()
                .collect(Collectors.groupingBy(String::length, TreeMap::new, Collectors.summingInt(String::length)));
        System.out.println("summing=" + summing);
        Map<Integer, String> mapping = words.stream()
                .collect(Collectors.groupingBy(String::length, TreeMap::new,
                        Collectors.mapping(String::toUpperCase, Collectors.joining(","))));
        System.out.println("mapping=" + mapping);
        Map.Entry<Integer, Integer> tee = words.stream().collect(Collectors.teeing(
                Collectors.summingInt(String::length),
                Collectors.maxBy(Comparator.comparingInt(String::length)),
                (sum, max) -> Map.entry(sum, max.map(String::length).orElse(0))));
        System.out.println("teeing=" + tee);
    }
}
