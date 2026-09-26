// FS-P1：initPhase1 系统属性全集——版本族来自 VersionProps，VM / 平台 / 编码族。
// 只打印跨 JDK 版本、跨宿主确定的事实（版本号本身以关系断言）。
public class TestSystemPropsSpec {
    static String p(String k) { return System.getProperty(k); }

    public static void main(String[] args) {
        int feature = Runtime.version().feature();
        System.out.println("spec==feature " + (Integer.parseInt(p("java.specification.version")) == feature));
        System.out.println("vm spec==feature " + p("java.vm.specification.version").equals(p("java.specification.version")));
        System.out.println("class.version " + p("java.class.version").equals((feature + 44) + ".0"));
        System.out.println("version prefix " + p("java.version").startsWith(String.valueOf(feature)));
        System.out.println("runtime.version prefix " + p("java.runtime.version").startsWith(p("java.version")));
        System.out.println("version.date " + p("java.version.date").matches("\\d{4}-\\d{2}-\\d{2}"));
        System.out.println("spec name " + p("java.specification.name") + " / " + p("java.specification.vendor"));
        System.out.println("vm spec " + p("java.vm.specification.name") + " / " + p("java.vm.specification.vendor"));
        System.out.println("runtime.name " + p("java.runtime.name"));
        String[] present = {"java.vendor", "java.vendor.url", "java.vm.name", "java.vm.vendor", "java.vm.version",
            "java.vm.info", "os.version", "os.name", "os.arch", "java.home", "user.dir", "java.io.tmpdir",
            "native.encoding", "sun.jnu.encoding", "java.class.path", "java.library.path"};
        StringBuilder sb = new StringBuilder();
        for (String k : present) sb.append(p(k) != null ? "" : k + " missing; ");
        System.out.println("present " + (sb.length() == 0 ? "all" : sb.toString()));
        System.out.println("file.encoding " + p("file.encoding") + " model " + p("sun.arch.data.model")
            + " endian " + p("sun.cpu.endian") + " jdk.debug " + p("jdk.debug"));
        System.out.println("separators " + p("file.separator") + " " + p("path.separator")
            + " nl=" + p("line.separator").equals("\n"));
        System.out.println("os.version nonempty " + !p("os.version").isEmpty());
        System.out.println("default charset " + java.nio.charset.Charset.defaultCharset());
        System.out.println("absent " + p("no.such.key") + " " + System.getProperty("no.such.key", "dflt"));
        System.setProperty("my.key", "v1");
        System.out.println("set " + p("my.key") + " clear " + System.clearProperty("my.key") + " after " + p("my.key"));
    }
}
