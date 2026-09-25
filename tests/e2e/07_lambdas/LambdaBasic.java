import java.util.ArrayList;

public class LambdaBasic {
    public static void main(String[] args) {
        ArrayList<Integer> list = new ArrayList<>();
        list.add(3);
        list.add(1);
        list.add(2);
        list.sort((a, b) -> a - b);
        Integer first = list.get(0);
        System.out.println(first.intValue()); // 1
    }
}
