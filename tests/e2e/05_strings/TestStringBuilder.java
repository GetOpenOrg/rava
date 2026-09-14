public class TestStringBuilder {
    public static void main(String[] args) {
        // 字符串拼接（+ 运算符，编译为 StringBuilder）
        String first = "Hello";
        String last  = "World";
        String full  = first + ", " + last + "!";
        System.out.println(full);

        // length / charAt
        System.out.println(full.length());
        System.out.println(full.charAt(0));

        // 常用 String 方法
        System.out.println("hello".toUpperCase());
        System.out.println("HELLO".toLowerCase());
        System.out.println("  hello  ".trim());
        System.out.println("hello".contains("ell"));
        System.out.println("hello world".replace("world", "Java"));
        System.out.println("hello".startsWith("he"));
        System.out.println("hello".endsWith("lo"));
        System.out.println("hello world".substring(6));
        System.out.println("a,b,c".split(",").length);
        System.out.println(String.valueOf(42));
        System.out.println(String.valueOf(true));
        System.out.println("hello".equals("hello"));
        System.out.println("hello".equals("world"));
        System.out.println("hello".indexOf("ll"));

        // StringBuilder 直接操作
        StringBuilder sb = new StringBuilder();
        sb.append("foo");
        sb.append("bar");
        sb.append(42);
        System.out.println(sb.toString());
        System.out.println(sb.length());

        // 链式调用
        String chained = new StringBuilder()
            .append("a")
            .append("b")
            .append("c")
            .toString();
        System.out.println(chained);

        // int / double 拼接
        int age = 25;
        System.out.println("age=" + age);
        double pi = 3.14;
        System.out.println("pi=" + pi);
    }
}
