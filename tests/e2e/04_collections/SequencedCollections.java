import java.util.ArrayList;

public class SequencedCollections {
    public static void main(String[] args) {
        ArrayList<String> list = new ArrayList<>();
        list.add("b");
        list.add("c");
        list.addFirst("a");
        list.addLast("d");
        System.out.println(list.getFirst());
        System.out.println(list.getLast());
        System.out.println(list.size());
    }
}
