import java.lang.invoke.MethodHandle;
import java.lang.invoke.MethodHandles;
import java.lang.invoke.MethodType;
import java.lang.invoke.VarHandle;

/**
 * 子字字段（boolean / byte / short / char）的 CAS 族：JDK 的 Unsafe.compareAndSetBoolean / Byte /
 * Short 按 `offset & ~3` 取所在 int 字、经 getIntVolatile + weakCompareAndSetInt 完成——
 * 偏移模型须让字地址落在字段自身，且相邻子字字段互不别名。
 * 覆盖：VarHandle 实例字段 CAS / compareAndExchange / getAndSet / getAndAdd（boolean、byte、short、char），
 * 相邻子字字段写入互不影响；未初始化类的静态方法句柄首次调用（DirectMethodHandle.ensureInitialized →
 * MethodHandle.updateForm → Unsafe.compareAndSetBoolean 字节码路径）；同一句柄反复调用。
 */
public class TestSubwordFieldCas {
    static class Flags {
        boolean a;
        byte b;
        boolean c;
        byte d;
        short s;
        char ch;
        int tail;
    }

    static class Lazy {
        static int seed;
        static {
            seed = 40;
            System.out.println("Lazy.<clinit>");
        }
        static int bump(int x) { return seed + x; }
    }

    static void show(String tag, Flags f) {
        System.out.println(tag + ": a=" + f.a + " b=" + f.b + " c=" + f.c + " d=" + f.d
                + " s=" + f.s + " ch=" + (int) f.ch + " tail=" + f.tail);
    }

    public static void main(String[] args) throws Throwable {
        MethodHandles.Lookup lk = MethodHandles.lookup();
        VarHandle A = lk.findVarHandle(Flags.class, "a", boolean.class);
        VarHandle B = lk.findVarHandle(Flags.class, "b", byte.class);
        VarHandle C = lk.findVarHandle(Flags.class, "c", boolean.class);
        VarHandle D = lk.findVarHandle(Flags.class, "d", byte.class);
        VarHandle S = lk.findVarHandle(Flags.class, "s", short.class);
        VarHandle CH = lk.findVarHandle(Flags.class, "ch", char.class);

        Flags f = new Flags();
        f.d = 7;
        f.tail = -1;
        show("init", f);

        // boolean：CAS 成功 / 失败、见证值、交换
        System.out.println("A.cas(false,true)=" + A.compareAndSet(f, false, true));
        System.out.println("A.cas(false,true)=" + A.compareAndSet(f, false, true));
        System.out.println("C.cae(true,true)=" + (boolean) C.compareAndExchange(f, true, true));
        System.out.println("C.getAndSet(true)=" + (boolean) C.getAndSet(f, true));
        System.out.println("A.cas(true,false)=" + A.compareAndSet(f, true, false));
        show("bool", f);

        // byte：负值、见证值、回绕加法；相邻字段不受影响
        System.out.println("B.cas(0,-1)=" + B.compareAndSet(f, (byte) 0, (byte) -1));
        System.out.println("B.cas(0,5)=" + B.compareAndSet(f, (byte) 0, (byte) 5));
        System.out.println("B.cae(-1,127)=" + (byte) B.compareAndExchange(f, (byte) -1, (byte) 127));
        System.out.println("B.getAndAdd(1)=" + (byte) B.getAndAdd(f, (byte) 1));
        System.out.println("D.cae(0,1)=" + (byte) D.compareAndExchange(f, (byte) 0, (byte) 1));
        System.out.println("D.getAndSet(-128)=" + (byte) D.getAndSet(f, (byte) -128));
        boolean done;
        do {
            byte cur = (byte) D.getVolatile(f);
            done = D.weakCompareAndSet(f, cur, (byte) (cur + 3));
        } while (!done);
        show("byte", f);

        // short / char
        System.out.println("S.cas(0,-30000)=" + S.compareAndSet(f, (short) 0, (short) -30000));
        System.out.println("S.cae(1,2)=" + (short) S.compareAndExchange(f, (short) 1, (short) 2));
        System.out.println("CH.cas(0,0xFFFF)=" + CH.compareAndSet(f, (char) 0, (char) 0xFFFF));
        System.out.println("CH.getAndSet('z')=" + (int) (char) CH.getAndSet(f, 'z'));
        show("short/char", f);

        // 普通写入与 VarHandle CAS 同一存储
        f.a = true;
        f.b = 9;
        System.out.println("A.cas(true,false)=" + A.compareAndSet(f, true, false));
        System.out.println("B.cas(9,10)=" + B.compareAndSet(f, (byte) 9, (byte) 10));
        show("mixed", f);

        // 未初始化类的静态方法句柄：首次调用触发类初始化（经 MethodHandle.updateForm 的 boolean CAS）
        MethodHandle bump = lk.findStatic(Lazy.class, "bump", MethodType.methodType(int.class, int.class));
        System.out.println("before invoke");
        System.out.println("bump(2)=" + (int) bump.invokeExact(2));
        System.out.println("bump(3)=" + (int) bump.invokeExact(3));
        int sum = 0;
        for (int i = 0; i < 300; i++) {
            sum += (int) bump.invokeExact(i);
        }
        System.out.println("sum=" + sum);
    }
}
