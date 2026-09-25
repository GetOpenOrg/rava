// 接口 default 方法测试
interface Greeter {
    String name();

    // default 方法调用同接口的 abstract 方法
    default String greet() {
        return "Hello, " + name() + "!";
    }
}

interface Logger {
    default String log(String message) {
        return "[LOG] " + message;
    }
}

// 使用默认实现（不覆盖 default 方法）
class SimpleGreeter implements Greeter {
    public String name() {
        return "World";
    }
}

// 覆盖 default 方法
class FormalGreeter implements Greeter {
    public String name() {
        return "Sir";
    }

    public String greet() {
        return "Good day, " + name() + ".";
    }
}

// 多接口实现均含 default 方法
class LoggingGreeter implements Greeter, Logger {
    public String name() {
        return "Alice";
    }
}

public class DefaultMethodTest {
    public static void main(String[] args) {
        // 1. 使用默认实现
        SimpleGreeter simple = new SimpleGreeter();
        System.out.println(simple.greet());

        // 2. default 方法中调用 abstract 方法（多态）
        System.out.println(simple.name());

        // 3. 覆盖 default 方法
        FormalGreeter formal = new FormalGreeter();
        System.out.println(formal.greet());

        // 4. 多接口 default 方法
        LoggingGreeter logger = new LoggingGreeter();
        System.out.println(logger.greet());
        System.out.println(logger.log("test message"));
    }
}
