public class TestShortCircuit {

    static boolean side(boolean v, String tag) {
        System.out.println("eval " + tag);
        return v;
    }

    public static void main(String[] args) {
        // && 短路：右侧不执行
        System.out.println("A: " + (side(false, "A-left") && side(true, "A-right")));
        // && 不短路
        System.out.println("B: " + (side(true, "B-left") && side(true, "B-right")));
        // || 短路
        System.out.println("C: " + (side(true, "C-left") || side(false, "C-right")));
        // || 不短路
        System.out.println("D: " + (side(false, "D-left") || side(true, "D-right")));

        // 非短路 & 与 |：两侧都求值
        System.out.println("E: " + (side(false, "E-left") & side(true, "E-right")));
        System.out.println("F: " + (side(true, "F-left") | side(false, "F-right")));

        // 短路保护：右侧会抛异常但不执行
        String s = null;
        boolean guarded = (s != null) && (s.length() > 0);
        System.out.println("guarded=" + guarded);

        // 混合 && / || 的优先级
        System.out.println("G: " + (side(true, "G1") || side(false, "G2") && side(true, "G3")));

        // 短路条件控制循环
        int i = 0;
        while (i < 10 && side(i < 3, "loop" + i)) {
            i++;
        }
        System.out.println("loop i=" + i);

        System.out.println("done");
    }
}
