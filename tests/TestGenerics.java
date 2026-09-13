import java.util.ArrayList;

public class TestGenerics {
    public static void main(String[] args) {
        ArrayList<String> list = new ArrayList<>();
        list.add("Alice");
        list.add("Bob");
        list.add("Charlie");

        // 每个 get() 返回 Object，赋值给 String 变量时字节码插入 checkcast
        String first = list.get(0);
        System.out.println(first);        // Alice

        String second = list.get(1);
        System.out.println(second);       // Bob

        String third = list.get(2);
        System.out.println(third);        // Charlie

        System.out.println(list.size());  // 3
    }
}
