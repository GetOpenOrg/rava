// FS-M5：身份哈希为 31 位非负、稳定、低位分布均匀；未覆盖 hashCode 的类 hashCode == identityHashCode。
import java.util.*;

public class TestIdentityHashSpec {
    static class Plain {}
    static class Custom { public int hashCode() { return 42; } }

    public static void main(String[] args) {
        List<Object> objs = new ArrayList<>();
        for (int i = 0; i < 2000; i++) objs.add(i % 2 == 0 ? new Object() : new Plain());
        boolean nonNeg = true, stable = true, same = true, nonZero = true;
        Set<Integer> low4 = new HashSet<>(), distinct = new HashSet<>();
        for (Object o : objs) {
            int h = o.hashCode();
            nonNeg &= h >= 0;
            nonZero &= h != 0;
            stable &= h == o.hashCode();
            same &= h == System.identityHashCode(o);
            low4.add(h & 0xF);
            distinct.add(h);
        }
        System.out.println("nonNeg=" + nonNeg + " nonZero=" + nonZero + " stable=" + stable + " same=" + same);
        System.out.println("low4 buckets=" + low4.size() + " distinct>1990=" + (distinct.size() > 1990));
        Custom c = new Custom();
        System.out.println("custom " + c.hashCode() + " identity nonNeg=" + (System.identityHashCode(c) >= 0));
        Object o = new Object();
        System.out.println("toString suffix " + o.toString().equals("java.lang.Object@" + Integer.toHexString(o.hashCode())));
        System.out.println("null identity " + System.identityHashCode(null));
        Map<Object, Integer> ihm = new IdentityHashMap<>();
        for (int i = 0; i < 100; i++) ihm.put(objs.get(i), i);
        System.out.println("identity map " + ihm.size() + " get " + ihm.get(objs.get(57)));
        Set<Object> hs = new HashSet<>(objs);
        System.out.println("hash set " + hs.size() + " contains " + hs.contains(objs.get(1999)) + " " + hs.contains(new Object()));
    }
}
