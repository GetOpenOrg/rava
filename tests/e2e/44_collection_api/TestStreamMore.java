import java.util.*;
import java.util.stream.*;

public class TestStreamMore {
    public static void main(String[] args) {
        List<Integer> nums = List.of(1, 2, 3, 4, 5, 6);
        System.out.println("takeWhile=" + nums.stream().takeWhile(x -> x < 4).toList());
        System.out.println("dropWhile=" + nums.stream().dropWhile(x -> x < 3).toList());
        System.out.println("ofNullable=" + Stream.ofNullable("x").count() + "," + Stream.ofNullable(null).count());
        System.out.println("iterate=" + Stream.iterate(1, n -> n < 10, n -> n + 2).toList());
        System.out.println("toList=" + nums.stream().map(x -> x * 2).toList());
        System.out.println("mapMulti=" + Stream.of(List.of(1, 2), List.of(3)).<Integer>mapMulti((l, c) -> l.forEach(c)).toList());
        List<Integer> out = new ArrayList<>();
        nums.stream().collect(() -> out, (acc, x) -> acc.add(x * 10), (a, b) -> a.addAll(b));
        System.out.println("mutableReduce=" + out);
        System.out.println("flatMap=" + Stream.of(List.of(1, 2), List.of(3, 4)).flatMap(List::stream).toList());
    }
}
