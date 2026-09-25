// 接口静态方法测试：验证 Companion struct 生成与调用路由
public class InterfaceStaticTest {

    interface Greeter {
        // 接口静态方法（有方法体），JNC 生成为 Companion struct 的关联函数
        static String defaultGreeting() {
            return "hello";
        }

        // 普通实例方法
        String greet();
    }

    static class SimpleGreeter implements Greeter {
        // 调用接口静态方法（invokestatic InterfaceStaticTest$Greeter.defaultGreeting）
        public String greet() {
            return Greeter.defaultGreeting() + " world";
        }
    }

    public static void main(String[] args) {
        // 直接调用接口静态方法
        System.out.println(Greeter.defaultGreeting());

        // 通过实现类间接调用接口静态方法
        SimpleGreeter sg = new SimpleGreeter();
        System.out.println(sg.greet());
    }
}
