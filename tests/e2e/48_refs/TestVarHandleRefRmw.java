import java.lang.invoke.MethodHandles;
import java.lang.invoke.VarHandle;
import java.util.concurrent.atomic.AtomicReferenceFieldUpdater;
import java.util.function.UnaryOperator;

/**
 * 经字段偏移的引用读-改-写：VarHandle getAndSet / compareAndExchange / compareAndSet 与 CAS 循环式 getAndUpdate，
 * AtomicReferenceFieldUpdater（Unsafe 引用 CAS 族）。
 * 覆盖：实例引用字段（含与目标语言关键字同名的 in / type / match、含 $ 的字段名）、子类遮蔽父类同名字段
 * （两字段互不影响）、静态引用字段（含关键字名 box）、引用数组元素（Object[] / String[]）、
 * 同名关键字基本类型字段（loop:int / yield:long）、两线程并发 CAS 计数。
 */
public class TestVarHandleRefRmw {
    static class Holder {
        volatile Object in;
        volatile String type;
        volatile Object match;
        volatile Object a$b;
        volatile Object plain;
        volatile int loop;
        volatile long yield;
    }

    static class Base {
        volatile Object x = "base-x";
    }

    static class Derived extends Base {
        volatile Object x = "derived-x";
    }

    static volatile Object box;
    static volatile Object counter = 0;

    static final AtomicReferenceFieldUpdater<Holder, Object> IN_UPDATER =
            AtomicReferenceFieldUpdater.newUpdater(Holder.class, Object.class, "in");

    /** CAS 循环式 getAndUpdate（VarHandle 无 getAndUpdate，按 JDK AtomicReference 的写法）。 */
    static Object getAndUpdate(VarHandle vh, Object holder, UnaryOperator<Object> f) {
        Object prev = vh.getVolatile(holder);
        while (true) {
            Object next = f.apply(prev);
            Object witness = vh.compareAndExchange(holder, prev, next);
            if (witness == prev) {
                return prev;
            }
            prev = witness;
        }
    }

    static Object getAndUpdateStatic(VarHandle vh, UnaryOperator<Object> f) {
        Object prev = vh.getVolatile();
        while (true) {
            Object next = f.apply(prev);
            if (vh.weakCompareAndSet(prev, next)) {
                return prev;
            }
            prev = vh.getVolatile();
        }
    }

    public static void main(String[] args) throws Throwable {
        MethodHandles.Lookup lk = MethodHandles.lookup();
        instanceFields(lk);
        shadowedFields(lk);
        staticFields(lk);
        arrayElements();
        primitiveKeywordFields(lk);
        updater();
        concurrent(lk);
    }

    static void instanceFields(MethodHandles.Lookup lk) throws Throwable {
        VarHandle IN = lk.findVarHandle(Holder.class, "in", Object.class);
        VarHandle TYPE = lk.findVarHandle(Holder.class, "type", String.class);
        VarHandle MATCH = lk.findVarHandle(Holder.class, "match", Object.class);
        VarHandle AB = lk.findVarHandle(Holder.class, "a$b", Object.class);
        VarHandle PLAIN = lk.findVarHandle(Holder.class, "plain", Object.class);
        Holder h = new Holder();

        System.out.println("in cas(null->s1): " + IN.compareAndSet(h, null, "s1") + " in=" + h.in);
        System.out.println("in cas(null->s2): " + IN.compareAndSet(h, null, "s2") + " in=" + h.in);
        System.out.println("in getAndSet: " + IN.getAndSet(h, "s3") + " in=" + h.in);
        System.out.println("in get: " + IN.get(h));
        IN.set(h, "s4");
        System.out.println("in set: " + h.in);

        System.out.println("type cae(null->t): " + TYPE.compareAndExchange(h, null, "t") + " type=" + h.type);
        System.out.println("type cae(x->u): " + TYPE.compareAndExchange(h, "x", "u") + " type=" + h.type);
        System.out.println("type getAndSetAcquire: " + TYPE.getAndSetAcquire(h, "v") + " type=" + h.type);

        h.match = 10;
        Object old = getAndUpdate(MATCH, h, v -> (Integer) v * 3);
        System.out.println("match getAndUpdate: " + old + " -> " + h.match);
        System.out.println("match cae release: " + MATCH.compareAndExchangeRelease(h, h.match, "m") + " match=" + h.match);

        System.out.println("a$b getAndSetRelease: " + AB.getAndSetRelease(h, "dollar") + " a$b=" + h.a$b);
        System.out.println("a$b weakCas: " + AB.weakCompareAndSetPlain(h, "dollar", "d2") + " a$b=" + h.a$b);

        Object o = new Object();
        System.out.println("plain getAndSet: " + PLAIN.getAndSet(h, o) + " same=" + (h.plain == o));
        System.out.println("fields: in=" + h.in + " type=" + h.type + " match=" + h.match + " a$b=" + h.a$b);
    }

    static void shadowedFields(MethodHandles.Lookup lk) throws Throwable {
        VarHandle BX = lk.findVarHandle(Base.class, "x", Object.class);
        VarHandle DX = lk.findVarHandle(Derived.class, "x", Object.class);
        Derived d = new Derived();
        Base asBase = d;
        System.out.println("shadow getAndSet base: " + BX.getAndSet(d, "B1") + " base.x=" + asBase.x + " derived.x=" + d.x);
        System.out.println("shadow getAndSet derived: " + DX.getAndSet(d, "D1") + " base.x=" + asBase.x + " derived.x=" + d.x);
        System.out.println("shadow cae base(D1->B2): " + BX.compareAndExchange(d, "D1", "B2") + " base.x=" + asBase.x);
        System.out.println("shadow cae derived(D1->D2): " + DX.compareAndExchange(d, "D1", "D2") + " derived.x=" + d.x);
        System.out.println("shadow get: base=" + BX.get(d) + " derived=" + DX.get(d));
    }

    static void staticFields(MethodHandles.Lookup lk) throws Throwable {
        VarHandle BOX = lk.findStaticVarHandle(TestVarHandleRefRmw.class, "box", Object.class);
        VarHandle COUNTER = lk.findStaticVarHandle(TestVarHandleRefRmw.class, "counter", Object.class);
        System.out.println("box getAndSet: " + BOX.getAndSet("b1") + " box=" + box);
        System.out.println("box cae(b1->b2): " + BOX.compareAndExchange("b1", "b2") + " box=" + box);
        System.out.println("box cae(zz->b3): " + BOX.compareAndExchange("zz", "b3") + " box=" + box);
        System.out.println("box cas(b2->null): " + BOX.compareAndSet("b2", null) + " box=" + box);
        Object prev = getAndUpdateStatic(COUNTER, v -> (Integer) v + 5);
        System.out.println("counter getAndUpdate: " + prev + " -> " + counter);
        counter = 0;
    }

    static void arrayElements() {
        VarHandle OA = MethodHandles.arrayElementVarHandle(Object[].class);
        VarHandle SA = MethodHandles.arrayElementVarHandle(String[].class);
        Object[] objs = new Object[3];
        String[] strs = {"a", "b", "c"};
        System.out.println("objs getAndSet[1]: " + OA.getAndSet(objs, 1, "o1") + " objs[1]=" + objs[1]);
        System.out.println("objs cae[1](o1->o2): " + OA.compareAndExchange(objs, 1, "o1", "o2") + " objs[1]=" + objs[1]);
        System.out.println("objs cas[2](x->y): " + OA.compareAndSet(objs, 2, "x", "y") + " objs[2]=" + objs[2]);
        objs[0] = 1;
        Object prev = OA.getAndSet(objs, 0, (Integer) objs[0] + 41);
        System.out.println("objs update[0]: " + prev + " -> " + objs[0]);
        System.out.println("strs getAndSet[0]: " + SA.getAndSet(strs, 0, "A") + " strs[0]=" + strs[0]);
        System.out.println("strs cae[2](c->C): " + SA.compareAndExchange(strs, 2, "c", "C") + " strs=" + String.join(",", strs));
        try {
            SA.getAndSet(strs, 3, "z");
        } catch (ArrayIndexOutOfBoundsException e) {
            System.out.println("strs out of bounds: " + e.getClass().getSimpleName());
        }
    }

    static void primitiveKeywordFields(MethodHandles.Lookup lk) throws Throwable {
        VarHandle LOOP = lk.findVarHandle(Holder.class, "loop", int.class);
        VarHandle YIELD = lk.findVarHandle(Holder.class, "yield", long.class);
        Holder h = new Holder();
        System.out.println("loop getAndAdd: " + (int) LOOP.getAndAdd(h, 7) + " loop=" + h.loop);
        System.out.println("loop cas(7->9): " + LOOP.compareAndSet(h, 7, 9) + " loop=" + h.loop);
        System.out.println("yield cae(0->123456789012): " + (long) YIELD.compareAndExchange(h, 0L, 123456789012L) + " yield=" + h.yield);
        System.out.println("yield getAndSet: " + (long) YIELD.getAndSet(h, -1L) + " yield=" + h.yield);
    }

    static void updater() {
        Holder h = new Holder();
        System.out.println("updater cas(null->u1): " + IN_UPDATER.compareAndSet(h, null, "u1") + " in=" + h.in);
        System.out.println("updater getAndSet: " + IN_UPDATER.getAndSet(h, "u2") + " in=" + h.in);
        System.out.println("updater getAndUpdate: " + IN_UPDATER.getAndUpdate(h, v -> v + "!") + " in=" + h.in);
        System.out.println("updater accumulateAndGet: " + IN_UPDATER.accumulateAndGet(h, "?", (a, b) -> a + "" + b));
    }

    static void concurrent(MethodHandles.Lookup lk) throws Throwable {
        VarHandle IN = lk.findVarHandle(Holder.class, "in", Object.class);
        Holder h = new Holder();
        h.in = 0;
        Runnable task = () -> {
            for (int i = 0; i < 500; i++) {
                getAndUpdate(IN, h, v -> (Integer) v + 1);
            }
        };
        Thread t1 = new Thread(task);
        Thread t2 = new Thread(task);
        t1.start();
        t2.start();
        t1.join();
        t2.join();
        System.out.println("concurrent in=" + h.in);
    }
}
