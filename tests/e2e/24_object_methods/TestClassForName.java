/**
 * Class.forName（native forName0；序列化 ObjectInputStream.resolveClass 经三参数形式按名解析）：
 * 用户类 / 嵌套类（$ 名）/ JDK 类 / 基本数组与对象数组名、三参数形式（initialize=false、loader=null 取 JDK 类；null loader 取用户类的
 * JDK 行为是 CNFE——原生单镜像偏差，见 compatibility.md）、
 * 同名多次取回同一 Class、未知类名与基本类型名 → ClassNotFoundException（消息为原名）。
 */
public class TestClassForName {
    static class Inner {}

    public static void main(String[] args) throws Exception {
        String[] names = {"TestClassForName", "TestClassForName$Inner", "java.util.ArrayList",
                "java.lang.String", "[I", "[Ljava.lang.String;"};
        for (String n : names) {
            Class<?> c = Class.forName(n);
            System.out.println(n + " -> " + c.getName() + " array=" + c.isArray());
        }
        Class<?> a = Class.forName("java.util.ArrayList", false, null);
        System.out.println("3-arg bootstrap=" + a.getName() + " same=" + (Class.forName("java.lang.String") == String.class)
                + " " + (Class.forName("TestClassForName$Inner") == Inner.class));
        for (String bad : new String[] {"no.such.Clazz", "int", "TestClassForName.Inner"}) {
            try {
                Class.forName(bad);
                System.out.println("no exception? " + bad);
            } catch (ClassNotFoundException e) {
                System.out.println("CNFE " + e.getMessage());
            }
        }
    }
}
