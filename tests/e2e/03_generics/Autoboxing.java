import java.util.ArrayList;
public class Autoboxing {
    public static void main(String[] args) {
        // Autoboxing: int → Integer
        Integer a = 42;
        int b = a;                      // unboxing
        System.out.println(b);          // 42

        // Double autoboxing
        Double d = 3.14;
        double e = d;
        System.out.println(e);          // 3.14

        // Boolean autoboxing
        Boolean flag = true;
        boolean f = flag;
        System.out.println(f);          // true

        // Arithmetic with unboxing
        Integer x = 10;
        Integer y = 20;
        int sum = x + y;
        System.out.println(sum);        // 30

        // ArrayList with autoboxing
        ArrayList<Integer> list = new ArrayList<>();
        list.add(1);
        list.add(2);
        list.add(3);
        int total = 0;
        for (int i = 0; i < list.size(); i++) {
            total += list.get(i);  // unboxing from Integer
        }
        System.out.println(total);      // 6

        // Integer methods
        System.out.println(Integer.valueOf(100));  // 100
        System.out.println(Integer.parseInt("123")); // 123
    }
}
