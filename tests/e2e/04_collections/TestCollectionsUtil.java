import java.util.ArrayList;
import java.util.Arrays;
import java.util.Collections;
import java.util.Comparator;
import java.util.List;

public class TestCollectionsUtil {

    static void show(List<?> list, String tag) {
        System.out.println(tag + list);
    }

    public static void main(String[] args) {
        List<Integer> nums = new ArrayList<>(Arrays.asList(5, 2, 9, 1, 5, 6));
        show(nums, "origin=");

        // sort 升序 / 降序
        Collections.sort(nums);
        show(nums, "sorted=");
        Collections.sort(nums, Collections.reverseOrder());
        show(nums, "reverseSorted=");
        Collections.sort(nums, (x, y) -> Integer.compare(x, y));
        show(nums, "lambdaSorted=");

        // reverse / rotate / shuffle（shuffle 用固定 seed 保证可复现）
        Collections.reverse(nums);
        show(nums, "reversed=");
        Collections.rotate(nums, 2);
        show(nums, "rotate(2)=");
        Collections.rotate(nums, -2);
        show(nums, "rotate(-2)=");

        java.util.Random seeded = new java.util.Random(42);
        List<Integer> toShuffle = new ArrayList<>(Arrays.asList(1, 2, 3, 4, 5));
        Collections.shuffle(toShuffle, seeded);
        show(toShuffle, "shuffled=");

        // min / max / frequency
        System.out.println("min=" + Collections.min(nums));
        System.out.println("max=" + Collections.max(nums));
        System.out.println("frequency(5)=" + Collections.frequency(nums, 5));
        System.out.println("frequency(99)=" + Collections.frequency(nums, 99));

        // binarySearch（必须先排序）
        List<Integer> sortedCopy = new ArrayList<>(nums);
        Collections.sort(sortedCopy);
        show(sortedCopy, "forBinarySearch=");
        System.out.println("binarySearch(5)=" + Collections.binarySearch(sortedCopy, 5));
        System.out.println("binarySearch(3)=" + Collections.binarySearch(sortedCopy, 3));

        // replaceAll / fill / copy / swap
        Collections.swap(sortedCopy, 0, sortedCopy.size() - 1);
        show(sortedCopy, "after swap=");
        List<Integer> filled = new ArrayList<>(Arrays.asList(0, 0, 0));
        Collections.fill(filled, 7);
        show(filled, "filled=");
        List<Integer> dst = new ArrayList<>(Arrays.asList(0, 0, 0, 0));
        Collections.copy(dst, new ArrayList<>(Arrays.asList(1, 2, 3)));
        show(dst, "copied=");
        System.out.println("replaceAll 5->0 count=" + Collections.replaceAll(nums, 5, 0));
        show(nums, "after replaceAll=");

        // addAll / disjoint
        List<String> base = new ArrayList<>();
        Collections.addAll(base, "a", "b", "c");
        show(base, "addAll=");
        System.out.println("disjoint=" + Collections.disjoint(base, Arrays.asList("x", "y")));
        System.out.println("disjoint shared=" + Collections.disjoint(base, Arrays.asList("a", "z")));

        // nCopies / frequency with strings
        List<String> ten = Collections.nCopies(3, "R");
        show(ten, "nCopies=");
        List<String> emptyList = Collections.emptyList();
        System.out.println("emptyList size=" + emptyList.size());
        List<String> singleton = Collections.singletonList("only");
        show(singleton, "singleton=");

        // unmodifiable / unmodifiableSorted 行为
        List<String> guard = Collections.unmodifiableList(base);
        System.out.println("guard get(0)=" + guard.get(0) + " size=" + guard.size());
        try {
            guard.add("nope");
            System.out.println("modify accepted");
        } catch (UnsupportedOperationException e) {
            System.out.println("unmodifiable -> " + e.getClass().getSimpleName());
        }

        // Comparator 工具：comparing / nullsLast
        List<String> words = new ArrayList<>(Arrays.asList("pear", "fig", "apple"));
        words.sort(Comparator.comparingInt(String::length));
        show(words, "byLength=");
        words.sort(Comparator.comparing(String::toString));
        show(words, "natural=");

        // 稳定性：多次排序结果一致
        List<Integer> twice = new ArrayList<>(Arrays.asList(3, 1, 2));
        Collections.sort(twice);
        Collections.sort(twice);
        show(twice, "idempotent=");

        System.out.println("done");
    }
}
