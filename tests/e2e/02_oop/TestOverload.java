public class TestOverload {

    // 重载候选：int / long / Integer / double / Object / varargs
    static String f(int x) { return "int"; }
    static String f(long x) { return "long"; }
    static String f(Integer x) { return "Integer"; }
    static String f(double x) { return "double"; }
    static String f(Object x) { return "Object"; }
    static String f(int... xs) { return "int..."; }

    // 两级父类形参的“最具体”选择
    static String g(Object o) { return "Object"; }
    static String g(Number n) { return "Number"; }
    static String g(Integer i) { return "Integer"; }

    // 单参 vs varargs：精确优先
    static String h(int a, int b) { return "2args"; }
    static String h(int... xs) { return "varargs"; }

    // null 消歧：最具体的引用类型
    static String nul(String s) { return "String"; }
    static String nul(Object o) { return "Object"; }

    // char / short / byte 走提升到 int
    static String up(int x) { return "int"; }
    static String up(long x) { return "long"; }

    // 返回类型不参与重载，但参数类型组合不同可以
    static int sum(int a, int b) { return a + b; }
    static long sum(long a, long b) { return a + b; }
    static double sum(double a, double b) { return a + b; }

    public static void main(String[] args) {
        System.out.println("f(1)=" + f(1));
        System.out.println("f(1L)=" + f(1L));
        System.out.println("f(Integer.valueOf(1))=" + f(Integer.valueOf(1)));
        System.out.println("f(1.5f)=" + f(1.5f));
        System.out.println("f(1.5)=" + f(1.5));
        System.out.println("f(\"s\")=" + f("s"));
        System.out.println("f(1,2)=" + f(1, 2));
        System.out.println("f()=" + f());

        System.out.println("g(Object)=" + g(new Object()));
        System.out.println("g(Number via Double)=" + g(Double.valueOf(2.5)));
        System.out.println("g(Integer)=" + g(Integer.valueOf(3)));

        System.out.println("h(1,2)=" + h(1, 2));
        System.out.println("h(1,2,3)=" + h(1, 2, 3));

        System.out.println("nul(null)=" + nul(null));

        char c = 'A';
        short s = 7;
        byte b = 3;
        System.out.println("up(char)=" + up(c));
        System.out.println("up(short)=" + up(s));
        System.out.println("up(byte)=" + up(b));

        System.out.println("sum(int,int)=" + sum(1, 2));
        System.out.println("sum(long,long)=" + sum(1L, 2L));
        System.out.println("sum(double,double)=" + sum(1.5, 2.5));
        System.out.println("sum(int,long)=" + sum(1, 2L));

        // 编译期常量折叠后仍是 int → 选 int 版本
        System.out.println("f(2+3)=" + f(2 + 3));

        System.out.println("done");
    }
}
