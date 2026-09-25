import java.lang.ref.WeakReference;
import java.lang.ref.SoftReference;

public class WeakRefDemo {
    public static void main(String[] args) {
        // WeakReference — referent accessible before GC
        String str = new String("hello weak");
        WeakReference<String> wref = new WeakReference<>(str);
        System.out.println(wref.get() != null);
        System.out.println(wref.get().equals("hello weak"));

        // After dropping the strong reference, get() MAY return null
        // We cannot guarantee GC runs, so only test that the API works
        str = null;
        // wref.get() might still be non-null (GC not forced); just test no crash
        Object got = wref.get();
        System.out.println(got == null || got instanceof String);

        // SoftReference
        String str2 = new String("hello soft");
        SoftReference<String> sref = new SoftReference<>(str2);
        System.out.println(sref.get() != null);
        System.out.println(sref.get().equals("hello soft"));
        str2 = null;
        Object got2 = sref.get();
        System.out.println(got2 == null || got2 instanceof String);

        // Multiple WeakReferences to same object
        String shared = "shared";
        WeakReference<String> w1 = new WeakReference<>(shared);
        WeakReference<String> w2 = new WeakReference<>(shared);
        System.out.println(w1.get() == w2.get());

        // WeakReference with null
        WeakReference<Object> nullRef = new WeakReference<>(null);
        System.out.println(nullRef.get() == null);

        System.out.println("done");
    }
}
