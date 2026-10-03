import java.util.Iterator;
import java.util.ServiceLoader;

/**
 * ServiceLoader 按类加载器查找服务时经 ModuleLayer.layers(loader) 读 ModuleLayer.CLV（类加载器值表），
 * 空层 EMPTY_LAYER 与 CLV 都由 ModuleLayer.<clinit> 建立；<clinit> 不翻译时 CLV 为 null，查找即 NPE。
 * 覆盖：系统 / 平台 / 线程上下文类加载器、null（引导）加载器、空层的模块与父层、对同一加载器的重复查找。
 */
public class TestServiceLoaderLayers {
    public interface Greeter {
        String greet();
    }

    static int count(ServiceLoader<?> sl) {
        int n = 0;
        Iterator<?> it = sl.iterator();
        while (it.hasNext()) {
            it.next();
            n++;
        }
        return n;
    }

    public static void main(String[] args) {
        ClassLoader scl = ClassLoader.getSystemClassLoader();
        System.out.println("scl: " + count(ServiceLoader.load(Greeter.class, scl)));
        // 同一加载器第二次查找：命中 CLV 中已登记的层列表
        System.out.println("scl again: " + count(ServiceLoader.load(Greeter.class, scl)));
        System.out.println("tccl: " + count(ServiceLoader.load(Greeter.class)));
        System.out.println("platform: "
                + count(ServiceLoader.load(Greeter.class, ClassLoader.getPlatformClassLoader())));
        System.out.println("bootstrap: " + count(ServiceLoader.load(Greeter.class, null)));
        System.out.println("installed: " + count(ServiceLoader.loadInstalled(Greeter.class)));
        System.out.println("findFirst: " + ServiceLoader.load(Greeter.class, scl).findFirst().isPresent());
        System.out.println("stream: " + ServiceLoader.load(Greeter.class, scl).stream().count());

        ModuleLayer empty = ModuleLayer.empty();
        System.out.println("empty modules: " + empty.modules().size());
        System.out.println("empty parents: " + empty.parents().size());
        System.out.println("empty same: " + (empty == ModuleLayer.empty()));
        System.out.println("empty find: " + empty.findModule("java.base").isPresent());
        System.out.println("empty loader lookup: " + count(ServiceLoader.load(empty, Greeter.class)));
    }
}
