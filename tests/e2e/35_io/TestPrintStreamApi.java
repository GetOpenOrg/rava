public class TestPrintStreamApi {

    public static void main(String[] args) {
        // print 各重载（不换行）
        System.out.print("a");
        System.out.print(1);
        System.out.print(2.5);
        System.out.print(true);
        System.out.print('Z');
        System.out.println();

        // println 各重载
        System.out.println("str");
        System.out.println(42);
        System.out.println(-7L);
        System.out.println(3.14);
        System.out.println(3.14f);
        System.out.println(false);
        System.out.println('c');
        System.out.println(new StringBuilder("sb"));
        System.out.println();

        // println 引用对象（走 String.valueOf）
        Object obj = Integer.valueOf(99);
        System.out.println(obj);
        System.out.println((Object) null);

        // println 字符数组
        char[] chars = {'H', 'i'};
        System.out.println(chars);

        // printf 基本格式
        System.out.printf("%d%n", 100);
        System.out.printf("%s-%s%n", "a", "b");
        System.out.printf("%.2f%n", 3.14159);
        System.out.printf("%x %o%n", 255, 8);
        System.out.printf("%c%n", 'A');
        System.out.printf("%b %b%n", true, null);

        // printf 宽度 / 对齐 / 补零
        System.out.printf("[%5d]%n", 42);
        System.out.printf("[%-5d]%n", 42);
        System.out.printf("[%05d]%n", 42);
        System.out.printf("[%8.3f]%n", 2.0);
        System.out.printf("[%,d]%n", 1234567);

        // printf 多参数顺序
        System.out.printf("%2$s %1$d%n", 7, "x");

        // String.format 与 printf 等价
        String formatted = String.format("%d/%o/%x", 15, 15, 15);
        System.out.println(formatted);
        String padded = String.format("%-6s|%6s", "l", "r");
        System.out.println(padded);

        // printf 特殊浮点
        System.out.printf("%.0f %.3f%n", 2.5, 1.0 / 3.0);
        System.out.printf("%e%n", 12345.678);

        // 拼接 SimplifiedFormatter 常见组合
        StringBuilder sb = new StringBuilder();
        sb.append(String.format("%04d", 9)).append("-").append(String.format("%+.1f", 2.25));
        System.out.println(sb);

        System.out.println("done");
    }
}
