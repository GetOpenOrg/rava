public class TestTernary {

    static int pick(boolean b, int x, int y) {
        return b ? x : y;
    }

    public static void main(String[] args) {
        int a = 5, b = 10;

        // 基础
        System.out.println(pick(a > b, a, b));

        // 嵌套三元（右结合）
        int n = 0;
        String kind = n > 0 ? "positive" : n < 0 ? "negative" : "zero";
        System.out.println(kind);

        // 分支类型不同 → 数值提升到 double
        double mixed = a > b ? 1 : 2.5;
        System.out.println(mixed);

        // 三元结果直接参与算术
        int x = 3;
        int r = (x % 2 == 0 ? x / 2 : x * 3 + 1) + 100;
        System.out.println(r);

        // 三元中的 char / int 提升
        char c = 'A';
        int ci = true ? c : 'Z';
        System.out.println(ci);

        // 三元嵌套在表达式中间
        String s = "val=" + (a < b ? a : b) + ";max=" + (a > b ? a : b);
        System.out.println(s);

        // 三元作为实参
        System.out.println(pick(true, a, b) + pick(false, a, b));

        // 布尔/引用混合
        Object o = n == 0 ? "zero-obj" : Integer.valueOf(n);
        System.out.println(o);

        // 三元的副作用分支
        int t = 0;
        int before = t;
        int after = before == 0 ? (t = 1) : (t = 2);
        System.out.println(t + "," + after);

        System.out.println("done");
    }
}
