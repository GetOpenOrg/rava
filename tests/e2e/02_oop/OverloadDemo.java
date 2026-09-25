public class OverloadDemo {
    // 静态重载方法
    static int add(int a, int b) { return a + b; }
    static double add(double a, double b) { return a + b; }

    // 实例重载方法
    String describe(int x) { return "int:" + x; }
    String describe(String s) { return "str:" + s; }

    public static void main(String[] args) {
        // 静态重载
        System.out.println(add(3, 4));       // 7
        System.out.println(add(1.5, 2.5));   // 4.0

        // 实例重载
        OverloadDemo d = new OverloadDemo();
        System.out.println(d.describe(42));        // int:42
        System.out.println(d.describe("hello"));   // str:hello
    }
}
