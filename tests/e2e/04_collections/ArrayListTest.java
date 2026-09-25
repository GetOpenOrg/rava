import java.util.ArrayList;

public class ArrayListTest {
    public static void main(String[] args) {
        ArrayList<String> list = new ArrayList<>();
        list.add("Hello");
        list.add("World");
        list.add("Java");

        System.out.println(list.size());
        System.out.println(list.get(0));
        System.out.println(list.get(1));
        System.out.println(list.get(2));

        list.set(1, "JNC");
        System.out.println(list.get(1));

        list.remove(2);
        System.out.println(list.size());

        System.out.println(list.isEmpty());
        list.clear();
        System.out.println(list.isEmpty());
    }
}
