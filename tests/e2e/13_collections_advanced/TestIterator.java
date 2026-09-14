import java.util.ArrayList;
import java.util.Iterator;
import java.util.List;

public class TestIterator {
    public static void main(String[] args) {
        List<String> list = new ArrayList<>();
        list.add("alpha");
        list.add("beta");
        list.add("gamma");

        // Iterator loop
        Iterator<String> it = list.iterator();
        while (it.hasNext()) {
            System.out.println(it.next());
        }

        // Remove via iterator
        List<Integer> nums = new ArrayList<>();
        for (int i = 0; i < 6; i++) {
            nums.add(i);
        }
        Iterator<Integer> numIt = nums.iterator();
        while (numIt.hasNext()) {
            int n = numIt.next();
            if (n % 2 == 0) {
                numIt.remove();
            }
        }
        System.out.println(nums);
    }
}
