import java.util.List;

/**
 * synchronized 块内的模式 switch（记录模式隐含 MatchException 处理区与 monitor 退出处理区
 * 交错）：CFG 结构化报「try 区域与控制流不成嵌套结构」，方法体落 panic 存根（S-67）。
 * 覆盖：循环内 synchronized + 模式 switch 表达式、synchronized 内直接 return 模式 switch。
 */public class TestSyncPatternSwitch {
    record Point(int x, int y) {}
    static int f(List<Object> items) {
        int t = 0;
        Object lock = new Object();
        for (Object it : items) {
            synchronized (lock) {
                t += switch (it) { case Point(int x, int y) -> x + y; default -> 0; };
            }
        }
        return t;
    }
    static int g(Object it) {
        synchronized (TestSyncPatternSwitch.class) {
            return switch (it) { case Point(int x, int y) -> x * y; default -> -1; };
        }
    }
    public static void main(String[] a) { System.out.println(f(List.of(new Point(1, 2), "x"))); System.out.println(g(new Point(3, 4))); }
}
