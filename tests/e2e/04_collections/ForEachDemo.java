import java.util.ArrayList;

public class ForEachDemo {
    public static void main(String[] args) {
        ArrayList<String> list = new ArrayList<>();
        list.add("alpha");
        list.add("beta");
        list.add("gamma");
        int count = 0;
        for (String s : list) {
            System.out.println(s);
            count++;
        }
        System.out.println(count);
    }
}
