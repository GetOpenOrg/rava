import java.io.InputStream;

/**
 * System 属性稳定子集与属性读通道（方法级实测：getProperties 12 jar /
 * getBoolean 11 / getInteger 6 / getSystemResourceAsStream 5，此前零覆盖；
 * 只读跨 mac/linux 稳定的键，os.name/user.dir 等机器相关键只验存在性）。
 */
public class TestSystemStableProps {

    public static void main(String[] args) {
        // 稳定键的值（mac/linux 两平台一致）
        System.out.println("file-sep=" + System.getProperty("file.separator"));
        System.out.println("path-sep=" + System.getProperty("path.separator"));
        System.out.println("line-sep-lf=" + "\n".equals(System.getProperty("line.separator")));
        System.out.println("tmpdir-defined=" + (System.getProperty("java.io.tmpdir") != null));

        // 版本键：语料 JDK 钉 21（21/25 双目标下startsWith 不锁死）
        System.out.println("spec-v=" + System.getProperty("java.specification.version"));

        // 机器相关键：只验存在不打印值
        System.out.println("os-defined=" + (System.getProperty("os.name") != null));
        System.out.println("user-dir-defined=" + (System.getProperty("user.dir") != null));
        System.out.println("props-size=" + (System.getProperties().size() > 10));

        // getBoolean / getInteger（带默认值形态）
        System.setProperty("e2e.flag", "true");
        System.setProperty("e2e.num", "42");
        System.out.println("bool-hit=" + Boolean.getBoolean("e2e.flag")
                + " miss=" + Boolean.getBoolean("e2e.nope"));
        System.out.println("int-hit=" + Integer.getInteger("e2e.num")
                + " int-default=" + Integer.getInteger("e2e.nope", 7)
                + " int-alt=" + Integer.getInteger("e2e.num", 0));

        // 系统资源流：JDK 自身类（与 Class 资源流互补的通道）
        try (InputStream in = ClassLoader.getSystemResourceAsStream("java/lang/String.class")) {
            System.out.println("sys-stream=" + (in != null)
                    + " head=" + String.format("%02X", in.read()));
        } catch (Exception e) {
            System.out.println("sys-stream-ex=" + e.getClass().getSimpleName());
        }

        // setProperty/removeProperty 形态
        System.setProperty("e2e.temp", "x");
        System.out.println("set-get=" + System.getProperty("e2e.temp"));
        System.clearProperty("e2e.temp");
        System.out.println("cleared=" + (System.getProperty("e2e.temp") == null));
    }
}
