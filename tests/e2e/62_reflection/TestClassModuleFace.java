/**
 * 模块形态：Class.getModule 的 isNamed/getName（spring 6 的模块判定面，
 * 此前零覆盖）——命名模块（java.base）vs 未命名（类路径自建类）、isExported 边界。
 */
public class TestClassModuleFace {

    public static void main(String[] args) {
        Module javaBase = String.class.getModule();
        System.out.println("jbase-named=" + javaBase.isNamed());
        System.out.println("jbase-name=" + javaBase.getName());

        // 类路径上的自建类：未命名模块
        Module own = TestClassModuleFace.class.getModule();
        System.out.println("own-named=" + own.isNamed());
        System.out.println("own-name=" + own.getName());

        // 各来源类一致归属
        System.out.println("list-same-jbase=" + (java.util.List.class.getModule() == javaBase));
        System.out.println("own-same=" + (thisClassModule() == own));

        // isExported：java.lang 公开导出；jdk.internal.* 不导出
        System.out.println("exported-lang=" + javaBase.isExported("java.lang"));
        System.out.println("exported-internal=" + !javaBase.isExported("jdk.internal.misc"));
        // 未命名模块：getName() 为 null；导出任意包（类路径语义）
        System.out.println("own-name-null=" + (own.getName() == null));
        System.out.println("own-exported-default=" + own.isExported("any.pkg"));

        // canRead 边界
        System.out.println("can-read=" + own.canRead(javaBase));

        // ClassLoader 与模块的对应：未命名模块属于类加载器
        System.out.println("own-loader=" + (own.getClassLoader()
                == TestClassModuleFace.class.getClassLoader()));
    }

    static Module thisClassModule() {
        return TestClassModuleFace.class.getModule();
    }
}
