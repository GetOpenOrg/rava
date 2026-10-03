/**
 * setStackTrace 与栈元素构造（方法级实测：5 jar，此前零覆盖）：
 * 覆盖后 getStackTrace 返回注入元素、空数组重置为无栈、栈元素的 equals 形态。
 */
public class TestStackTraceOps {

    public static void main(String[] args) {
        RuntimeException e = new RuntimeException("st");
        System.out.println("natural-nonempty=" + (e.getStackTrace().length > 0));

        // 注入自定义栈：完全替换
        StackTraceElement[] fake = {
            new StackTraceElement("FakeClass", "fakeMethod", "Fake.java", 42),
            new StackTraceElement("Outer$Inner", "run", "Outer.java", -1),
        };
        e.setStackTrace(fake);
        StackTraceElement[] got = e.getStackTrace();
        System.out.println("len=" + got.length);
        System.out.println("top=" + got[0].getClassName() + "." + got[0].getMethodName()
                + ":" + got[0].getLineNumber());
        System.out.println("file=" + got[0].getFileName()
                + " native=" + got[1].isNativeMethod());

        // 注入元素是拷贝（改 got 不影响再取）
        got[0] = null;
        System.out.println("defensive-copy=" + (e.getStackTrace()[0] != null));

        // 空数组：无栈形态
        e.setStackTrace(new StackTraceElement[0]);
        System.out.println("empty-len=" + e.getStackTrace().length);

        // 栈元素 equals/hashCode/toString
        StackTraceElement a = new StackTraceElement("C", "m", "F.java", 1);
        StackTraceElement b = new StackTraceElement("C", "m", "F.java", 1);
        System.out.println("eq=" + a.equals(b) + " hash-eq=" + (a.hashCode() == b.hashCode()));
        System.out.println("to-string=" + a.toString());
    }
}
