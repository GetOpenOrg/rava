/**
 * 系统属性的按键删除（Properties.remove）与读取折叠的边界：删除启动时存在的键后读到 null；
 * 删除启动时不存在的键返回 null 且之后仍不存在；删除键经包装方法形参（含两层转交）、拼接（含循环下标）传入；
 * 删除并返回原值经转型后返回（ModuleBootstrap.getAndRemoveProperty 的形态）。
 */
public class TestSysPropsRemove {

    /** 包装方法：删除键来自形参，返回原值 */
    private static String getAndRemove(String key) {
        return (String) System.getProperties().remove(key);
    }

    /** 两层包装：键再经一层形参转交 */
    static String removeVia(String key) {
        return getAndRemove(key);
    }

    public static void main(String[] args) {
        // 启动时不存在的键：删除返回 null，之后仍不存在
        System.out.println("absent-remove=" + getAndRemove("e2e.rm.absent"));
        System.out.println("absent-after=" + System.getProperty("e2e.rm.absent"));

        // 启动时存在的键（常量）：删除返回原值，之后读到 null / 缺省值
        System.out.println("fsep-before=" + System.getProperty("file.separator"));
        System.out.println("fsep-remove=" + getAndRemove("file.separator"));
        System.out.println("fsep-after=" + System.getProperty("file.separator"));
        System.out.println("fsep-default=" + System.getProperty("file.separator", "dflt"));

        // 拼接出的键（编译期不可知的下标）
        String prefix = args.length > 5 ? "nope." : "path.";
        System.out.println("psep-before=" + System.getProperty("path.separator"));
        System.out.println("psep-remove=" + removeVia(prefix + "separator"));
        System.out.println("psep-after=" + System.getProperty("path.separator"));

        // 循环下标拼接的键
        for (int i = 0; i < 3; i++) {
            String v = getAndRemove("e2e.rm.idx." + i);
            System.out.println("idx" + i + "=" + v);
        }
        System.out.println("idx1-after=" + System.getProperty("e2e.rm.idx.1"));

        // 删除后再读带缺省值的形态；从未写入的键读缺省值
        System.out.println("psep-default=" + System.getProperty("path.separator", "dflt"));
        System.out.println("never-default=" + System.getProperty("e2e.rm.never", "gone"));
    }
}
