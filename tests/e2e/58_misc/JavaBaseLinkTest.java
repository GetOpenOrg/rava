public class JavaBaseLinkTest {
    public static void main(String[] args) {
        // 测试 Long（java-base 中的包装类）
        long x = Long.parseLong("42");
        long y = Long.parseLong("-7");
        System.out.println(x + y);           // 35

        // 测试 Math（java-base 中的数学类）
        long absY = (long) Math.abs((double) y);
        System.out.println(absY);             // 7

        // 测试 Integer.MAX_VALUE
        System.out.println(Integer.MAX_VALUE); // 2147483647

        // 测试 ArrayList
        java.util.ArrayList<String> list = new java.util.ArrayList<>();
        list.add("hello");
        list.add("world");
        System.out.println(list.size());      // 2
    }
}
