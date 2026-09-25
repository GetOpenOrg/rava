import java.util.ArrayList;
public class ArrayListFull {
    public static void main(String[] args) {
        ArrayList<Integer> list = new ArrayList<>();
        list.add(10); list.add(20); list.add(30);
        for (int i = 0; i < list.size(); i++) {
            Integer val = (Integer) list.get(i);
            System.out.println(val.intValue());
        }
    }
}
