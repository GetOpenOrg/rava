import java.io.InputStream;
import java.lang.module.Configuration;
import java.lang.module.ModuleDescriptor;
import java.nio.charset.Charset;
import java.util.Optional;

/**
 * 引导模块层与引导阶段的结束状态（构建期引导映像，计划 2026-10-05-boot-image-evaluator 第 3 步）：
 * System.initPhase1–3 在构建期求值，ModuleLayer.boot()、各 Module 的导出 / 可读关系、内建加载器、
 * 系统属性与标准流都来自映像，运行期只重放宿主相关部分。本例只读取这些状态，不定义新层。
 */
public class TestBootLayer {
    static String name(ClassLoader l) {
        return l == null ? "null" : l.getName();
    }

    public static void main(String[] args) throws Exception {
        ModuleLayer boot = ModuleLayer.boot();
        System.out.println("boot != null: " + (boot != null));
        System.out.println("boot same: " + (ModuleLayer.boot() == boot));
        System.out.println("parents = " + boot.parents().size()
                + ", empty parent: " + (boot.parents().get(0) == ModuleLayer.empty()));

        Optional<Module> baseOpt = boot.findModule("java.base");
        System.out.println("java.base present: " + baseOpt.isPresent());
        Module base = baseOpt.get();
        System.out.println("String module same: " + (String.class.getModule() == base));
        System.out.println("base named: " + base.isNamed() + ", layer same: " + (base.getLayer() == boot));
        System.out.println("base loader: " + name(base.getClassLoader()));
        System.out.println("base exports java.lang: " + base.isExported("java.lang"));
        System.out.println("base exports jdk.internal.misc: " + base.isExported("jdk.internal.misc"));
        System.out.println("base opens java.lang: " + base.isOpen("java.lang"));
        ModuleDescriptor bd = base.getDescriptor();
        System.out.println("descriptor name: " + bd.name() + ", open: " + bd.isOpen()
                + ", has java.util: " + bd.packages().contains("java.util"));
        System.out.println("in boot modules: " + boot.modules().contains(base));

        Module sql = boot.findModule("java.sql").get();
        System.out.println("java.sql loader: " + name(sql.getClassLoader()));
        System.out.println("findLoader(java.sql) platform: "
                + (boot.findLoader("java.sql") == ClassLoader.getPlatformClassLoader()));
        System.out.println("java.sql reads base: " + sql.canRead(base));
        System.out.println("java.sql exports java.sql: " + sql.isExported("java.sql"));
        System.out.println("Time module same: " + (java.sql.Time.class.getModule() == sql));
        System.out.println("missing module: " + boot.findModule("no.such.module").isPresent());

        Configuration cf = boot.configuration();
        System.out.println("cf has java.base: " + cf.findModule("java.base").isPresent());
        System.out.println("cf parent empty: " + (cf.parents().get(0) == Configuration.empty()));

        Module unnamed = TestBootLayer.class.getModule();
        System.out.println("app module named: " + unnamed.isNamed() + ", layer: " + unnamed.getLayer());
        System.out.println("app module is scl unnamed: "
                + (unnamed == ClassLoader.getSystemClassLoader().getUnnamedModule()));
        System.out.println("app reads base: " + unnamed.canRead(base));

        try (InputStream in = base.getResourceAsStream("java/lang/Object.class")) {
            byte[] magic = in.readNBytes(4);
            System.out.printf("Object.class magic: %02x%02x%02x%02x%n", magic[0], magic[1], magic[2], magic[3]);
        }

        System.out.println("scl = " + name(ClassLoader.getSystemClassLoader())
                + ", parent = " + name(ClassLoader.getSystemClassLoader().getParent()));
        Thread main = Thread.currentThread();
        System.out.println("thread = " + main.getName() + ", group = " + main.getThreadGroup().getName()
                + ", parent group = " + main.getThreadGroup().getParent().getName());
        System.out.println("ctx == scl: " + (main.getContextClassLoader() == ClassLoader.getSystemClassLoader()));

        System.out.println("file.separator = " + System.getProperty("file.separator"));
        System.out.println("path.separator = " + System.getProperty("path.separator"));
        System.out.println("lineSeparator is \\n: " + System.lineSeparator().equals("\n"));
        System.out.println("line.separator same: " + System.lineSeparator().equals(System.getProperty("line.separator")));
        System.out.println("user.dir set: " + (System.getProperty("user.dir") != null));
        System.out.println("java.home set: " + (System.getProperty("java.home") != null));
        System.out.println("default charset = " + Charset.defaultCharset().name());
        System.out.println("out same: " + (System.out == System.out) + ", err != out: " + (System.err != System.out));
        System.out.println("processors > 0: " + (Runtime.getRuntime().availableProcessors() > 0));
        System.out.println("maxMemory > 0: " + (Runtime.getRuntime().maxMemory() > 0));
    }
}
