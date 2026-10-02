import java.lang.StackWalker.Option;
import java.lang.StackWalker.StackFrame;
import java.util.List;
import java.util.Set;
import java.util.stream.Collectors;

/**
 * StackWalker 的 VM 栈遍历（StackStreamFactory.callStackWalk / fetchStackFrames 与
 * StackFrameInfo 的 MemberName 展开）：帧序、类名 / 方法名、调用者类、limit 与 skip、分批取帧。
 * 只打印本类帧（类名 + 方法名），行号不进输出。
 */
public class TestStackWalkerFrames {
    static final StackWalker PLAIN = StackWalker.getInstance();
    static final StackWalker RETAIN = StackWalker.getInstance(Set.of(Option.RETAIN_CLASS_REFERENCE));

    static List<String> ownFrames(StackWalker w) {
        return w.walk(s -> s.filter(f -> f.getClassName().startsWith("TestStackWalkerFrames"))
                .map(f -> f.getClassName() + "." + f.getMethodName())
                .collect(Collectors.toList()));
    }

    static List<String> level3() { return ownFrames(PLAIN); }
    static List<String> level2() { return level3(); }
    static List<String> level1() { return level2(); }

    static int recurse(int n) {
        if (n == 0) {
            return PLAIN.walk(s -> s.filter(f -> f.getMethodName().equals("recurse")).count()).intValue();
        }
        return recurse(n - 1);
    }

    static class Helper {
        Class<?> whoCalls() { return RETAIN.getCallerClass(); }
        String topMethod() { return PLAIN.walk(s -> s.findFirst().map(StackFrame::getMethodName).orElse("?")); }
    }

    static String callerFromHelper() { return new Helper().whoCalls().getName(); }

    public static void main(String[] args) {
        System.out.println("chain = " + level1());
        System.out.println("recurse depth = " + recurse(40));
        System.out.println("caller = " + callerFromHelper());
        System.out.println("top = " + new Helper().topMethod());
        System.out.println("first frame class = "
                + RETAIN.walk(s -> s.findFirst().map(f -> f.getDeclaringClass().getSimpleName()).orElse("?")));
        System.out.println("limit 2 = " + PLAIN.walk(s -> s.limit(2).map(StackFrame::getMethodName).collect(Collectors.toList())));
        System.out.println("skip 1 first = " + level2skip());
        try {
            PLAIN.walk(s -> s.findFirst().map(StackFrame::getDeclaringClass).orElse(null));
        } catch (UnsupportedOperationException e) {
            System.out.println("no retain: " + e.getMessage());
        }
        try {
            PLAIN.getCallerClass();
        } catch (UnsupportedOperationException e) {
            System.out.println("plain getCallerClass: " + e.getMessage());
        }
        Runnable r = () -> System.out.println("in lambda top = "
                + PLAIN.walk(s -> s.findFirst().map(StackFrame::getMethodName).orElse("?")));
        r.run();
        System.out.println("ste = " + PLAIN.walk(s -> s.findFirst().map(f -> {
            StackTraceElement e = f.toStackTraceElement();
            return e.getClassName() + "#" + e.getMethodName() + " native=" + e.isNativeMethod();
        }).orElse("?")));
        System.out.println("native? = " + PLAIN.walk(s -> s.findFirst().map(StackFrame::isNativeMethod).orElse(null)));
    }

    static String level2skip() {
        return PLAIN.walk(s -> s.skip(1).findFirst().map(f -> f.getMethodName()).orElse("?"));
    }
}
