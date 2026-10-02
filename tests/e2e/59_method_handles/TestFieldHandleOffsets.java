import java.lang.invoke.MethodHandle;
import java.lang.invoke.MethodHandles;
import java.lang.invoke.VarHandle;
import java.util.ArrayList;
import java.util.List;
import java.util.concurrent.atomic.AtomicReferenceFieldUpdater;

/**
 * 字段偏移可得的边界（C1d-b b1 / S2）：只有按名取得（findGetter / findSetter / findVarHandle /
 * 字段更新器）的字段可经偏移读写；手写体写入的字段（Thread.name）与边界类字段（ClassLoader）
 * 不因此成为偏移读写目标。用例覆盖：方法句柄字段访问器（解释器口径）写入引用字段后经字节码读出、
 * VarHandle 实例 / 静态引用字段 CAS 与 getAndSet、AtomicReferenceFieldUpdater、
 * 同时使用手写写入字段与类加载器的普通读取不受影响。
 */
public class TestFieldHandleOffsets {
    interface Shape { String name(); }
    static final class Sq implements Shape { public String name() { return "sq"; } }
    static final class Tri implements Shape { public String name() { return "tri"; } }

    static class Box {
        Object payload;
        volatile Shape shape;
        List<String> tags = new ArrayList<>();
        static Object shared;
    }

    static final AtomicReferenceFieldUpdater<Box, Shape> SHAPE =
            AtomicReferenceFieldUpdater.newUpdater(Box.class, Shape.class, "shape");

    public static void main(String[] args) throws Throwable {
        MethodHandles.Lookup lk = MethodHandles.lookup();
        Box b = new Box();

        // 方法句柄字段访问器：写入值只落在按名取得的字段上
        MethodHandle setP = lk.findSetter(Box.class, "payload", Object.class);
        MethodHandle getP = lk.findGetter(Box.class, "payload", Object.class);
        setP.invoke(b, new Sq());
        Object p = getP.invoke(b);
        System.out.println("mh payload: " + ((Shape) p).name());
        System.out.println("bytecode payload: " + ((Shape) b.payload).name());

        // VarHandle 实例字段 CAS / getAndSet
        VarHandle vh = lk.findVarHandle(Box.class, "shape", Shape.class);
        System.out.println("cas null->sq: " + vh.compareAndSet(b, (Shape) null, (Shape) new Sq()));
        Shape old = (Shape) vh.getAndSet(b, (Shape) new Tri());
        System.out.println("old: " + old.name() + ", now: " + ((Shape) vh.get(b)).name());

        // VarHandle 静态字段
        VarHandle sh = lk.findStaticVarHandle(Box.class, "shared", Object.class);
        sh.set("hello");
        System.out.println("static: " + sh.get() + " / " + Box.shared);
        System.out.println("static cas: " + sh.compareAndSet((Object) "hello", (Object) new Tri()));
        System.out.println("static now: " + ((Shape) Box.shared).name());

        // 字段更新器
        System.out.println("updater cas: " + SHAPE.compareAndSet(b, b.shape, new Sq()));
        System.out.println("updater get: " + SHAPE.get(b).name());

        // 句柄写入的列表经字节码使用
        MethodHandle setT = lk.findSetter(Box.class, "tags", List.class);
        List<String> l = new ArrayList<>();
        l.add("a");
        setT.invoke(b, l);
        b.tags.add("b");
        System.out.println("tags: " + b.tags);

        // 手写体写入的字段（线程名）与边界类（类加载器）照常读取
        Thread t = new Thread(() -> {}, "worker-1");
        t.setName("worker-2");
        System.out.println("thread: " + t.getName());
        ClassLoader cl = TestFieldHandleOffsets.class.getClassLoader();
        System.out.println("loader: " + (cl != null) + " parent: " + (cl.getParent() != null));
    }
}
