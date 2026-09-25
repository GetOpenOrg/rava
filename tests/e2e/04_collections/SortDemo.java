import java.util.ArrayList;
import java.util.Collections;

public class SortDemo {
    public static void main(String[] args) {
        ArrayList<Integer> list = new ArrayList<>();
        list.add(5); list.add(1); list.add(9); list.add(8); list.add(2);
        Collections.sort(list);
        for (int x : list) {
            System.out.println(x); // 1 2 5 8 9
        }

        ArrayList<Integer> list2 = new ArrayList<>();
        list2.add(4); list2.add(3); list2.add(1);
        list2.sort((a, b) -> b - a);
        for (int x : list2) {
            System.out.println(x); // 4 3 1
        }
    }
}
