public class IntegerMethodsDemo {
    public static void main(String[] args) {
        int a = 7;
        int b = 3;
        System.out.println(a);                           // 7
        System.out.println(b);                           // 3
        System.out.println(a + b);                       // 10
        System.out.println(Integer.toBinaryString(10));  // 1010
        System.out.println(Integer.toHexString(255));    // ff
        System.out.println(Integer.min(3, 7));           // 3
        System.out.println(Integer.MAX_VALUE > 0);       // true
        System.out.println(Integer.compare(3, 7));       // -1
        System.out.println(Double.parseDouble("2.5"));   // 2.5
        System.out.println(Integer.sum(100, 200));       // 300
    }
}
