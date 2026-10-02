// StackWalker 帧的行号与 Throwable 栈同源（行表 + LineNumberTable）：普通调用、lambda、接口 default、
// 继承未覆盖的超类方法、构造器与静态初始化；StackFrame.getLineNumber 与 toStackTraceElement 一致。
import java.util.List;
import java.util.function.Supplier;
import java.util.stream.Collectors;

public class TestStackWalkerLines {
    static List<String> walk() {
        return StackWalker.getInstance().walk(s -> s
                .filter(f -> f.getClassName().startsWith("TestStackWalkerLines"))
                .map(f -> f.getClassName() + "." + f.getMethodName() + ":" + f.getLineNumber()
                        + (f.toStackTraceElement().getLineNumber() == f.getLineNumber() ? "" : " MISMATCH")
                        + " " + f.getFileName())
                .collect(Collectors.toList()));
    }

    static void show(String title, List<String> frames) {
        System.out.println(title);
        for (String f : frames) {
            System.out.println("  " + f);
        }
    }

    interface Walker {
        default List<String> viaDefault() {
            return walk();
        }
    }

    static class Base implements Walker {
        List<String> inherited() {
            return walk();
        }
    }

    static class Derived extends Base {
        final List<String> atInit;

        Derived() {
            atInit = walk();
        }
    }

    static class Holder {
        static final List<String> AT_CLINIT = walk();
    }

    static List<String> nested(int depth) {
        if (depth == 0) {
            return walk();
        }
        return nested(depth - 1);
    }

    public static void main(String[] args) {
        show("direct:", walk());
        show("recursive:", nested(2));
        Supplier<List<String>> s = () -> walk();
        show("lambda:", s.get());
        Derived d = new Derived();
        show("default:", d.viaDefault());
        show("inherited:", d.inherited());
        show("constructor:", d.atInit);
        show("clinit:", Holder.AT_CLINIT);
    }
}
