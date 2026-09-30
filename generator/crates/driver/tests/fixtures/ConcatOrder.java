// 字符串拼接求值顺序（JLS §15.18.1 / §15.7.1）：实参从左到右求值，toString 转换次序与 JDK 21 一致
// （javac 21 在求值处即以 String.valueOf 转换；旧版 javac 的 Object 实参由 StringConcatFactory 按实参序转换）。
// 覆盖：两个实参的 toString 有副作用；静态字段读取夹在两个有副作用的调用之间。
public class ConcatOrder {
    static final class Loud {
        final String tag;
        Loud(String tag) { this.tag = tag; }
        @Override public String toString() {
            System.out.println("toString " + tag);
            return tag;
        }
    }

    static int step = 0;

    static Loud make(String tag) {
        System.out.println("eval " + tag + " #" + (++step));
        return new Loud(tag);
    }

    public static void main(String[] args) {
        Loud a = new Loud("A");
        Loud b = new Loud("B");
        String s = "[" + a + "|" + b + "]";
        System.out.println(s);
        String t = make("X") + "-" + step + "-" + make("Y");
        System.out.println(t);
    }
}
