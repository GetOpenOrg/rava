import java.util.Iterator;
import java.util.ArrayList;

public class UserIterable {
    static class NumberList implements Iterable<Integer> {
        private ArrayList<Integer> data;

        NumberList() { this.data = new ArrayList<>(); }

        void add(int v) { data.add(v); }

        @Override
        public Iterator<Integer> iterator() {
            return data.iterator();
        }
    }

    public static void main(String[] args) {
        NumberList list = new NumberList();
        list.add(10);
        list.add(20);
        list.add(30);

        int sum = 0;
        for (int v : list) {
            sum += v;
        }
        System.out.println(sum);   // 60

        Iterator<Integer> it = list.iterator();
        int count = 0;
        while (it.hasNext()) {
            it.next();
            count++;
        }
        System.out.println(count); // 3
    }
}
