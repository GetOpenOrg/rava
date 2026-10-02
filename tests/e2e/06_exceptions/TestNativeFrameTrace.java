// 栈帧来源统一的边界：手写 native 方法成帧（行号 -2 / "Native Method"）、延迟 lambda 不在创建方法上多出帧、
// 原位闭包（异常处理区段）帧行号与外层方法一致。
import java.util.function.Supplier;

public class TestNativeFrameTrace {
    static void show(String title, Throwable t) {
        System.out.println(title);
        for (StackTraceElement e : t.getStackTrace()) {
            String cls = e.getClassName();
            String line = cls.startsWith("TestNativeFrameTrace") ? " line=" + e.getLineNumber() : "";
            System.out.println("  " + cls + "." + e.getMethodName() + " native=" + e.isNativeMethod() + line);
        }
    }

    static Throwable fromLambda() {
        Supplier<Throwable> s = () -> new Throwable("lambda");
        return s.get();
    }

    static Throwable fromCatch() {
        try {
            Object o = null;
            return new Throwable(o.toString());
        } catch (NullPointerException npe) {
            return new Throwable("catch");
        }
    }

    public static void main(String[] args) {
        Thread.currentThread().interrupt();
        try {
            Thread.sleep(10);
            System.out.println("not interrupted");
        } catch (InterruptedException ie) {
            show("sleep: " + ie.getMessage(), ie);
        }
        show("lambda:", fromLambda());
        show("catch:", fromCatch());
    }
}
