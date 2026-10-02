/**
 * 类加载器身份与委派：bootstrap null 语义（核心类/基本类型/void）、数组类跟随组件类
 * 加载器、自定义类 getParent 链、同加载器同身份（spring/mybatis 类加载判定的地基）。
 */
public class TestClassLoaderIdentity {

    public static void main(String[] args) throws Exception {
        ClassLoader app = TestClassLoaderIdentity.class.getClassLoader();

        // bootstrap：核心类、基本类型、void 的加载器为 null
        System.out.println("string-null=" + (String.class.getClassLoader() == null));
        System.out.println("int-null=" + (int.class.getClassLoader() == null));
        System.out.println("void-null=" + (void.class.getClassLoader() == null));
        System.out.println("self-app=" + (app != null));

        // 数组类：加载器跟随组件类
        System.out.println("arr-null=" + (String[].class.getClassLoader() == null));
        System.out.println("self-arr-app=" + (TestClassLoaderIdentity[].class.getClassLoader() == app));

        // 平台加载器与委派链
        ClassLoader platform = ClassLoader.getPlatformClassLoader();
        System.out.println("platform-not-null=" + (platform != null));
        System.out.println("app-parent-platform=" + (app.getParent() == platform));
        System.out.println("platform-parent-null=" + (platform.getParent() == null));

        // loadClass 同身份：经名加载与 class 字面量同对象
        Class<?> byName = app.loadClass("TestClassLoaderIdentity");
        System.out.println("same-identity=" + (byName == TestClassLoaderIdentity.class));

        // 自定义加载器：委派给父（不 override loadClass → 委派语义）
        ClassLoader child = new ClassLoader(app) {
        };
        System.out.println("child-parent=" + (child.getParent() == app));
        System.out.println("child-delegate=" + (child.loadClass("java.lang.String") == String.class));
    }
}
