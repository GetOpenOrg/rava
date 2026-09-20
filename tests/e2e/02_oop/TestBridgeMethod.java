import java.util.ArrayList;
import java.util.List;

// 泛型父类：子类用具体类型重写 → javac 生成 bridge 方法
abstract class Store<T> {
    abstract void put(T item);
    abstract T get();

    void showAll() {
        System.out.println("store value=" + get());
    }
}

class StringStore extends Store<String> {
    private String v;

    @Override
    public void put(String item) {
        this.v = item;
        System.out.println("StringStore.put " + item);
    }

    @Override
    public String get() {
        return v;
    }
}

class IntStore extends Store<Integer> {
    private Integer v = 0;

    @Override
    public void put(Integer item) {
        this.v = item;
        System.out.println("IntStore.put " + item);
    }

    @Override
    public Integer get() {
        return v;
    }
}

// 协变返回类型：子类返回更具体的类型 → 生成 bridge
class CovBase {
    Number value() { return 1; }
    Object ref() { return "base"; }
}

class CovDerived extends CovBase {
    @Override
    Integer value() { return 42; }

    @Override
    String ref() { return "derived"; }
}

// 泛型接口 + 具体类型实现
interface Crate<T> {
    void set(T t);
    T peek();
}

class StringCrate implements Crate<String> {
    private String s = "";

    @Override
    public void set(String t) {
        s = t;
    }

    @Override
    public String peek() {
        return s.toUpperCase();
    }
}

// 通过父类/接口引用调用 → 走 bridge 方法
public class TestBridgeMethod {

    static void useBridge() {
        Store<String> s = new StringStore();
        s.put("hello");
        System.out.println("get=" + s.get());
        s.showAll();

        Store<Integer> i = new IntStore();
        i.put(7);
        System.out.println("get=" + i.get());
        i.showAll();

        // 原生（raw）调用，直接打到 bridge
        Store raw = new StringStore();
        raw.put("raw-value");
        System.out.println("raw get=" + raw.get());
    }

    static void useCovariant() {
        CovBase b = new CovDerived();
        System.out.println("value=" + b.value());
        System.out.println("ref=" + b.ref());

        CovDerived d = new CovDerived();
        System.out.println("direct value=" + d.value());
        System.out.println("direct ref=" + d.ref());
    }

    static void useGenericInterface() {
        List<Crate<String>> boxes = new ArrayList<>();
        boxes.add(new StringCrate());
        for (Crate<String> box : boxes) {
            box.set("abc");
            System.out.println("peek=" + box.peek());
        }
    }

    public static void main(String[] args) {
        useBridge();
        useCovariant();
        useGenericInterface();

        // 把实现放进 List<Store<String>> 再统一调用：全部走 bridge
        List<Store<String>> stores = new ArrayList<>();
        stores.add(new StringStore());
        for (Store<String> st : stores) {
            st.put("via-list");
            System.out.println("list get=" + st.get());
        }

        System.out.println("done");
    }
}
