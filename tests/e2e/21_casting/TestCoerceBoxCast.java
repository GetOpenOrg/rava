/**
 * checkcast / coerce 路径的装箱强转见证（fix-invoke-box-cast 的同类风险面核查，
 * docs/tasks.md「剩余工作」第 10 条）。
 *
 * 基本类型表达式 / 返回值经 (Object) / (Integer) / (Double) 等强转与隐式上转：
 * 包装类按值的静态种类选定（int→Integer、char→Character、boolean→Boolean），
 * byte/short 拓宽后装箱仍按拓宽后的种类；三元 / 比较产物装箱；错误包装类强转
 * 抛 ClassCastException（拒收面）。
 */
public class TestCoerceBoxCast {
    static int intVal() { return 7; }
    static long longVal() { return 99L; }
    static char charVal() { return 'Z'; }
    static byte byteVal() { return -3; }
    static short shortVal() { return 1234; }
    static boolean boolVal() { return true; }
    static float floatVal() { return 1.25f; }
    static double doubleVal() { return 0.5; }
    static <T> T pass(T v) { return v; }

    public static void main(String[] args) {
        // ── 基本类型表达式直接强转（源级装箱 + checkcast）──
        Object oi = (Object) intVal();
        Integer ii = (Integer) intVal();
        Number ni = (Number) intVal();
        Object od = (Object) doubleVal();
        Double dd = (Double) doubleVal();
        Object ol = (Object) longVal();
        Long ll = (Long) longVal();
        Object oz = (Object) boolVal();
        Boolean bb = (Boolean) boolVal();
        Object oc = (Object) charVal();
        Character cc = (Character) charVal();
        System.out.println("direct " + oi + " " + ii + " " + ni.intValue() + " " + od
                + " " + dd + " " + ol + " " + ll + " " + oz + " " + bb + " " + oc + " " + cc);

        // ── byte / short 拓宽后装箱：种类随拓宽结果（int → Integer）──
        int widenedB = byteVal();
        int widenedS = shortVal();
        int widenedC = charVal();
        Object owb = (Object) widenedB;
        Integer iwb = (Integer) widenedB;
        Object ows = (Object) widenedS;
        Integer iws = (Integer) widenedS;
        Object owc = (Object) widenedC;
        Integer iwc = (Integer) widenedC;
        System.out.println("widen " + owb + " " + iwb + " " + ows + " " + iws + " " + owc + " " + iwc
                + " " + (byteVal() == (byte) -3));

        // ── 原种类装箱（byte→Byte / short→Short / char→Character，不经拓宽）──
        Byte rb = (Byte) (Object) byteVal();
        Short rs = (Short) (Object) shortVal();
        Character rc = (Character) (Object) charVal();
        Number nb = (Number) (Object) byteVal();
        Number ns = (Number) (Object) shortVal();
        System.out.println("narrow-box " + rb + " " + rs + " " + rc + " " + nb.intValue()
                + " " + ns.shortValue());

        // ── 三元 / 比较产物装箱 ──
        boolean flag = intVal() > 3;
        Object tern1 = flag ? intVal() : 2.5;
        Object tern2 = !flag ? "s" : longVal();
        Object cmp1 = (Object) (intVal() == 7);
        Object cmp2 = (Object) (doubleVal() < 1.0);
        Number tn = flag ? 1 : 2;
        System.out.println("tern " + tern1 + " " + tern2 + " " + cmp1 + " " + cmp2 + " " + tn);

        // ── instanceof 与 getClass（装箱对象身份与种类）──
        System.out.println("inst " + (oi instanceof Integer) + " " + (oi instanceof Number)
                + " " + (oi instanceof Comparable) + " " + (od instanceof Double)
                + " " + (od instanceof Number) + " " + (oc instanceof Character)
                + " " + (oc instanceof Comparable));
        System.out.println("cls " + oi.getClass().getName() + " " + od.getClass().getName()
                + " " + oc.getClass().getName() + " " + oz.getClass().getName());

        // ── 泛型边界：类型形参承载装箱值，取回时 checkcast ──
        Integer pg = TestCoerceBoxCast.<Integer>pass(41);
        Object po = TestCoerceBoxCast.<Object>pass(42);
        Number pn = TestCoerceBoxCast.<Number>pass(43);
        System.out.println("generic " + pg + " " + po + " " + pn.intValue());

        // ── Object... 变参与数组元素装箱 ──
        Object[] arr = {intVal(), longVal(), charVal(), byteVal(), shortVal(), boolVal(), floatVal(), doubleVal()};
        Integer a0 = (Integer) arr[0];
        Long a1 = (Long) arr[1];
        Character a2 = (Character) arr[2];
        Byte a3 = (Byte) arr[3];
        Short a4 = (Short) arr[4];
        Boolean a5 = (Boolean) arr[5];
        Float a6 = (Float) arr[6];
        Double a7 = (Double) arr[7];
        System.out.println("array " + a0 + " " + a1 + " " + a2 + " " + a3 + " " + a4
                + " " + a5 + " " + a6 + " " + a7);

        // ── 装箱值的超类型视图与 Comparable 应答 ──
        Comparable<Integer> ci = (Comparable<Integer>) (Object) intVal();
        Comparable<Character> ch = (Comparable<Character>) (Object) charVal();
        System.out.println("cmp " + ci.compareTo(7) + " " + ci.compareTo(8) + " " + ch.compareTo('A'));

        // ── 拒收面：错误包装类强转 → ClassCastException ──
        try {
            Long bad = (Long) (Object) intVal();
            System.out.println("no-cce " + bad);
        } catch (ClassCastException e) {
            System.out.println("cce-int-to-long " + e.getClass().getSimpleName());
        }
        try {
            Double bad2 = (Double) (Object) longVal();
            System.out.println("no-cce " + bad2);
        } catch (ClassCastException e) {
            System.out.println("cce-long-to-double " + e.getClass().getSimpleName());
        }
        try {
            Integer bad3 = (Integer) (Object) charVal();
            System.out.println("no-cce " + bad3);
        } catch (ClassCastException e) {
            System.out.println("cce-char-to-int " + e.getClass().getSimpleName());
        }
        try {
            Boolean bad4 = (Boolean) (Object) intVal();
            System.out.println("no-cce " + bad4);
        } catch (ClassCastException e) {
            System.out.println("cce-int-to-bool " + e.getClass().getSimpleName());
        }
    }
}
