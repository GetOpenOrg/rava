import java.lang.invoke.MethodHandle;
import java.lang.invoke.MethodHandles;
import java.lang.invoke.MethodType;

// 方法句柄首次调用触发目标类静态初始化（JVMS §5.5 / DirectMethodHandle.checkInitialized）：
// findStatic / findStaticGetter / findStaticSetter / findConstructor，
// 覆盖查找时未初始化（调用时才 <clinit>）与查找前已初始化两种情况
public class TestMethodHandleStaticInit {
    static final StringBuilder LOG = new StringBuilder();

    static void log(String s) { LOG.append(s).append(';'); }

    static class StaticTarget {
        static { log("StaticTarget.<clinit>"); }
        static int twice(int x) { return x * 2; }
    }

    static class GetterTarget {
        static int value;
        static { value = 41; log("GetterTarget.<clinit>"); }
    }

    static class SetterTarget {
        static String label = "init";
        static { log("SetterTarget.<clinit>"); }
    }

    static class CtorTarget {
        static { log("CtorTarget.<clinit>"); }
        final int n;
        CtorTarget(int n) { this.n = n; }
    }

    static class Warm {
        static int hits;
        static { log("Warm.<clinit>"); }
        static int bump() { return ++hits; }
        Warm() { hits += 10; }
    }

    static void flush(String tag) {
        System.out.println(tag + ": " + LOG);
        LOG.setLength(0);
    }

    public static void main(String[] args) throws Throwable {
        MethodHandles.Lookup lk = MethodHandles.lookup();

        // 未初始化：查找不触发，首次调用触发
        MethodHandle twice = lk.findStatic(StaticTarget.class, "twice", MethodType.methodType(int.class, int.class));
        flush("after findStatic");
        int r = (int) twice.invokeExact(21);
        flush("after invoke static=" + r);
        r = (int) twice.invokeExact(5);
        flush("second invoke static=" + r);

        MethodHandle getter = lk.findStaticGetter(GetterTarget.class, "value", int.class);
        flush("after findStaticGetter");
        int v = (int) getter.invokeExact();
        flush("after get value=" + v);

        MethodHandle setter = lk.findStaticSetter(SetterTarget.class, "label", String.class);
        flush("after findStaticSetter");
        setter.invokeExact("set");
        flush("after set label=" + SetterTarget.label);

        MethodHandle ctor = lk.findConstructor(CtorTarget.class, MethodType.methodType(void.class, int.class));
        flush("after findConstructor");
        CtorTarget c = (CtorTarget) ctor.invokeExact(7);
        flush("after new n=" + c.n);

        // 已初始化：查找前已触发 <clinit>，句柄调用不再重复
        int h = Warm.bump();
        flush("warm direct=" + h);
        MethodHandle bump = lk.findStatic(Warm.class, "bump", MethodType.methodType(int.class));
        MethodHandle wget = lk.findStaticGetter(Warm.class, "hits", int.class);
        MethodHandle wset = lk.findStaticSetter(Warm.class, "hits", int.class);
        MethodHandle wnew = lk.findConstructor(Warm.class, MethodType.methodType(void.class));
        h = (int) bump.invokeExact();
        wset.invokeExact(h + 100);
        Warm w = (Warm) wnew.invokeExact();
        int got = (int) wget.invokeExact();
        flush("warm handles hits=" + got + " obj=" + (w != null));
    }
}
