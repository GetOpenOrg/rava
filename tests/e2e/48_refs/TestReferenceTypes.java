import java.lang.ref.PhantomReference;
import java.lang.ref.Reference;
import java.lang.ref.ReferenceQueue;
import java.lang.ref.SoftReference;
import java.lang.ref.WeakReference;
import java.util.WeakHashMap;

/**
 * equiv 探针③：弱 / 软 / 虚引用。GC 回收时机不确定，只测强可达期间的确定性语义：
 * get / clear / refersTo / enqueue / 队列 poll / PhantomReference.get 恒 null，
 * 以及 WeakHashMap 在键强可达时的映射行为。
 */
public class TestReferenceTypes {
    public static void main(String[] args) {
        String strong = new String("payload");
        ReferenceQueue<String> q = new ReferenceQueue<>();

        // WeakReference：强可达时 get 返回原对象；clear 后 null；refersTo 判定
        WeakReference<String> w = new WeakReference<>(strong, q);
        System.out.println("weak.get=" + w.get() + "," + (w.get() == strong));
        System.out.println("weak.refersTo=" + w.refersTo(strong) + "," + w.refersTo(null));
        System.out.println("weak.poll=" + (q.poll() == null));
        w.clear();
        System.out.println("weak.cleared=" + w.get() + "," + w.refersTo(null));
        // enqueue：首次成功、重复失败；入队后可 poll 出同一引用对象
        WeakReference<String> w2 = new WeakReference<>(strong, q);
        System.out.println("weak.enqueue=" + w2.enqueue() + "," + w2.enqueue());
        Reference<? extends String> polled = q.poll();
        System.out.println("queue.poll=" + (polled == w2) + "," + (q.poll() == null));
        // 无队列构造：enqueue 返回 false
        WeakReference<String> w3 = new WeakReference<>(strong);
        System.out.println("weak.noQueue.enqueue=" + w3.enqueue());
        // null referent
        WeakReference<String> wn = new WeakReference<>(null);
        System.out.println("weak.null=" + wn.get() + "," + wn.refersTo(null));

        // SoftReference：强可达时同样可取回
        SoftReference<String> s = new SoftReference<>(strong);
        System.out.println("soft.get=" + (s.get() == strong));
        s.clear();
        System.out.println("soft.cleared=" + s.get());

        // PhantomReference：get 恒 null（规范），refersTo 可判定
        PhantomReference<String> ph = new PhantomReference<>(strong, q);
        System.out.println("phantom.get=" + ph.get() + ",refersTo=" + ph.refersTo(strong));

        // WeakHashMap：键强可达期间行为与 HashMap 相同
        WeakHashMap<String, Integer> whm = new WeakHashMap<>();
        String k1 = new String("k1"), k2 = new String("k2");
        whm.put(k1, 1); whm.put(k2, 2); whm.put(k1, 10);
        System.out.println("whm=" + whm.size() + "," + whm.get(k1) + "," + whm.get("k2")
                + "," + whm.containsKey("k3"));
        whm.remove(k2);
        System.out.println("whm.remove=" + whm.size() + "," + whm.get("k2"));
        System.out.println("strongAlive=" + strong + k1 + k2);
    }
}
