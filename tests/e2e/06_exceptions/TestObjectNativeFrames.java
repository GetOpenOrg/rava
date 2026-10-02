/** 手写根类（Object）方法成帧：native 方法帧 (Native Method) 的类 / 方法 / 文件 / 行号 -2。 */
public class TestObjectNativeFrames {
    static class Plain {
        Object copy() throws CloneNotSupportedException {
            return super.clone();
        }
    }

    static class Lock {
    }

    public static void main(String[] args) {
        try {
            new Plain().copy();
        } catch (CloneNotSupportedException e) {
            print(e);
        }
        Object o = new Object();
        try {
            o.notify();
        } catch (IllegalMonitorStateException e) {
            print(e);
        }
        Lock l = new Lock();
        try {
            l.notifyAll();
        } catch (IllegalMonitorStateException e) {
            print(e);
        }
        try {
            l.notify();
        } catch (IllegalMonitorStateException e) {
            print(e);
        }
    }

    static void print(Throwable e) {
        System.out.println(e);
        for (StackTraceElement s : e.getStackTrace()) {
            System.out.println("  " + s.getClassName() + "." + s.getMethodName() + " " + s.getFileName()
                    + ":" + s.getLineNumber() + " native=" + s.isNativeMethod());
        }
    }
}
