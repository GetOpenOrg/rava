import java.lang.invoke.MethodHandle;
import java.lang.invoke.MethodHandles;
import java.lang.invoke.MethodType;

/**
 * BoundMethodHandle 动态物种（N11）：jlink 只预生成 L…/LJ/LLJ/LLLJ/I/IL/D/DL 共 15 种物种，
 * 其余绑定形状（LI、LII、LJL、LF、LD、LLI …）在 JVM 上由 ASM 现场生成物种类。
 * 覆盖：insertArguments 绑定 int / long / float / double / 混合、链式追加绑定（copyWithExtend）、
 * bindTo 后再 insertArguments、对绑定结果再 asType。
 */
public class TestBmhDynamicSpecies {
    static String f(Object a, int b) { return "f(" + a + "," + b + ")"; }
    static String g(Object a, int b, int c) { return "g(" + a + "," + b + "," + c + ")"; }
    static String h(Object a, long b, Object c) { return "h(" + a + "," + b + "," + c + ")"; }
    static String k(Object a, float b, double c) { return "k(" + a + "," + b + "," + c + ")"; }
    static String m(String self, int x, long y, double z) { return self + ":" + x + ":" + y + ":" + z; }

    public static void main(String[] args) throws Throwable {
        MethodHandles.Lookup l = MethodHandles.lookup();
        MethodHandle F = l.findStatic(TestBmhDynamicSpecies.class, "f", MethodType.methodType(String.class, Object.class, int.class));
        MethodHandle G = l.findStatic(TestBmhDynamicSpecies.class, "g", MethodType.methodType(String.class, Object.class, int.class, int.class));
        MethodHandle H = l.findStatic(TestBmhDynamicSpecies.class, "h", MethodType.methodType(String.class, Object.class, long.class, Object.class));
        MethodHandle K = l.findStatic(TestBmhDynamicSpecies.class, "k", MethodType.methodType(String.class, Object.class, float.class, double.class));
        MethodHandle M = l.findStatic(TestBmhDynamicSpecies.class, "m", MethodType.methodType(String.class, String.class, int.class, long.class, double.class));

        // 物种 LI：Object + int
        System.out.println("LI  " + MethodHandles.insertArguments(F, 0, "a", 1).invoke());
        // 物种 LII：Object + int + int（record (int,int) 反序列化同形）
        System.out.println("LII " + MethodHandles.insertArguments(G, 0, "b", 2, 3).invoke());
        // 物种 LJL
        System.out.println("LJL " + MethodHandles.insertArguments(H, 0, "c", 4L, "d").invoke());
        // 物种 LFD
        System.out.println("LFD " + MethodHandles.insertArguments(K, 0, "e", 1.5f, 2.25).invoke());
        // 链式追加：先绑一个（L），再逐个追加 I → J → D（copyWithExtend 链）
        MethodHandle step = MethodHandles.insertArguments(M, 0, "s");
        step = MethodHandles.insertArguments(step, 0, 7);
        step = MethodHandles.insertArguments(step, 0, 8L);
        System.out.println("chain " + step.invoke(9.5));
        step = MethodHandles.insertArguments(step, 0, 10.5);
        System.out.println("chain " + step.invoke());
        // 同一动态物种重复使用（登记表复用）
        for (int i = 0; i < 3; i++) {
            System.out.println("again " + MethodHandles.insertArguments(G, 0, "r" + i, i, i * i).invoke());
        }
        // 绑定结果再 asType
        MethodHandle half = MethodHandles.insertArguments(G, 0, "t", 5);
        MethodHandle asObj = half.asType(MethodType.methodType(Object.class, Integer.class));
        System.out.println("asType " + asObj.invoke(Integer.valueOf(6)));
        System.out.println("type " + half.type());
    }
}
