/**
 * Java 10 var 局部变量类型推断测试
 */
public class VarTest {
    public static void main(String[] args) {
        var x = 42;
        var msg = "Hello var";
        var list = new java.util.ArrayList<Integer>();
        list.add(1);
        list.add(2);
        list.add(3);
        var sum = 0;
        for (var item : list) {
            sum += item;
        }
        System.out.println(msg + ": x=" + x + ", sum=" + sum);
    }
}
