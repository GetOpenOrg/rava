import java.util.ArrayList;
import java.util.Arrays;
import java.util.Collections;
import java.util.List;

public class CollectionsUtilTest {
    public static void main(String[] args) {
        // 1. Collections.sort + reverse
        List<Integer> list = new ArrayList<>(Arrays.asList(3, 1, 4, 1, 5, 9, 2, 6));
        Collections.sort(list);
        System.out.println("sorted: " + list);
        Collections.reverse(list);
        System.out.println("reversed: " + list);

        // 2. Collections.min / max
        System.out.println("min: " + Collections.min(list));
        System.out.println("max: " + Collections.max(list));

        // 3. Collections.frequency
        List<Integer> freq = new ArrayList<>(Arrays.asList(1, 2, 2, 3, 3, 3));
        System.out.println("freq(3): " + Collections.frequency(freq, 3));

        // 4. Collections.singletonList
        List<String> single = Collections.singletonList("only");
        System.out.println("singleton: " + single);
        System.out.println("singleton size: " + single.size());

        // 5. Collections.emptyList
        List<Object> empty = Collections.emptyList();
        System.out.println("emptyList size: " + empty.size());

        // 6. Collections.swap
        List<String> swapList = new ArrayList<>(Arrays.asList("a", "b", "c", "d"));
        Collections.swap(swapList, 0, 3);
        System.out.println("swap: " + swapList);

        // 7. Collections.fill
        List<String> fillList = new ArrayList<>(Arrays.asList("x", "y", "z"));
        Collections.fill(fillList, "filled");
        System.out.println("fill: " + fillList);

        // 8. Collections.binarySearch (list must be sorted)
        List<Integer> sorted = new ArrayList<>(Arrays.asList(1, 3, 5, 7, 9));
        int idx = Collections.binarySearch(sorted, 5);
        System.out.println("binarySearch(5): " + idx);
        int notFound = Collections.binarySearch(sorted, 4);
        System.out.println("binarySearch(4) negative: " + (notFound < 0));

        // 9. Collections.nCopies
        List<String> copies = Collections.nCopies(3, "repeat");
        System.out.println("nCopies: " + copies);

        // 10. Arrays.equals
        int[] a1 = {1, 2, 3};
        int[] a2 = {1, 2, 3};
        int[] a3 = {1, 2, 4};
        System.out.println("Arrays.equals same: " + Arrays.equals(a1, a2));
        System.out.println("Arrays.equals diff: " + Arrays.equals(a1, a3));

        // 11. Arrays.binarySearch
        int[] sortedArr = {1, 3, 5, 7, 9};
        System.out.println("Arrays.binarySearch(5): " + Arrays.binarySearch(sortedArr, 5));
        System.out.println("Arrays.binarySearch(4) negative: " + (Arrays.binarySearch(sortedArr, 4) < 0));

        System.out.println("Done.");
    }
}
