import java.util.ServiceLoader;

/**
 * ServiceLoader 空提供者边界（多 provider 绑定需 META-INF/services 资源、
 * 单文件 e2e 无法携带——由 #1 slf4j pilot 承担真绑定；此处固化空态与 reload 语义，
 * jdk.random/zipfs/charsets 的跨模块装载已另有专项覆盖）。
 */
public class TestServiceLoaderEmpty {

    /** 全语料无任何 provider 的接口。 */
    interface NoProvider {
        String tag();
    }

    public static void main(String[] args) {
        ServiceLoader<NoProvider> sl = ServiceLoader.load(NoProvider.class);

        System.out.println("hasNext=" + sl.iterator().hasNext());
        System.out.println("findFirst-empty=" + sl.findFirst().isEmpty());
        System.out.println("stream-count=" + sl.stream().count());
        sl.reload();
        boolean reloadEmpty = ServiceLoader.load(NoProvider.class).stream().count() == 0;
        System.out.println("reload-ok=" + reloadEmpty);

        // 指定类加载器形态
        ServiceLoader<NoProvider> byLoader =
                ServiceLoader.load(NoProvider.class, TestServiceLoaderEmpty.class.getClassLoader());
        System.out.println("by-loader=" + byLoader.findFirst().isEmpty());

        // 类型参数（provider 类型安全）
        System.out.println("typed-empty=" + sl.findFirst().isPresent());
    }
}
