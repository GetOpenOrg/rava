import java.lang.reflect.Method;

// 运行时热路径微基准（docs/reports/2026-10-11-code-review-triage.md「性能实测」）：
// 字符串常量 ldc、类字面量、getClass、setAccessible 后的 Method.invoke 各在热循环中重复执行；
// 输出只含确定性校验和，耗时由外部计时
public class TestReflectLdcHotLoop {
    static class Base {
        int step(int x) { return x + 1; }
    }

    static class Derived extends Base {
        @Override
        int step(int x) { return x + 3; }
    }

    private static int twice(int x) { return x * 2; }

    public static void main(String[] args) throws Exception {
        final int n = 1_000_000;

        // ldc：同一字面量恒为同一驻留对象
        String first = "hot-literal";
        long ldcSum = 0;
        int same = 0;
        for (int i = 0; i < n; i++) {
            String s = "hot-literal";
            if (s == first) same++;
            ldcSum += s.length() + (i & 7);
        }
        System.out.println("ldc " + same + " " + ldcSum);

        // 类字面量与 getClass：同一类恒为同一镜像
        Object o = new Derived();
        int classHits = 0;
        long nameLen = 0;
        for (int i = 0; i < n; i++) {
            if (o.getClass() == Derived.class) classHits++;
            if (Base.class.isInstance(o)) classHits++;
            if ((i & 1023) == 0) nameLen += o.getClass().getName().length();
        }
        System.out.println("class " + classHits + " " + nameLen);

        // Method.invoke：虚方法（经接收者运行时类上溯）与私有静态方法
        Method step = Base.class.getDeclaredMethod("step", int.class);
        step.setAccessible(true);
        Method tw = TestReflectLdcHotLoop.class.getDeclaredMethod("twice", int.class);
        tw.setAccessible(true);
        long invSum = 0;
        for (int i = 0; i < n; i++) {
            // 返回值经 hashCode 取值（Integer.hashCode 即其值），不做 checkcast
            invSum += step.invoke(o, i & 255).hashCode();
            invSum += tw.invoke(null, i & 127).hashCode();
        }
        System.out.println("invoke " + invSum);
    }
}
