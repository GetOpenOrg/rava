// FS-E3：toString 抛出的异常经字符串拼接 / println / StringBuilder / 集合 toString 传播，可被 catch。
public class TestToStringThrows {
    static class Bad { public String toString() { throw new IllegalStateException("boom"); } }
    public static void main(String[] args) {
        Object b = new Bad();
        try {
            String s = "v=" + b;
            System.out.println(s);
        } catch (IllegalStateException e) {
            System.out.println("caught " + e.getMessage());
        }
        try {
            System.out.println(b);
        } catch (IllegalStateException e) {
            System.out.println("caught2 " + e.getMessage());
        }
        try {
            StringBuilder sb = new StringBuilder().append(b);
            System.out.println(sb);
        } catch (IllegalStateException e) {
            System.out.println("caught3 " + e.getMessage());
        }
        try {
            System.out.println(String.valueOf(b) + java.util.List.of(b));
        } catch (IllegalStateException e) {
            System.out.println("caught4 " + e.getMessage());
        }
    }
}
