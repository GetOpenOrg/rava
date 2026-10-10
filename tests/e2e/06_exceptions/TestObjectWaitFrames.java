/**
 * Object.wait 族成帧：wait()/wait(J)/wait(JI) 是字节码方法，最终经 native wait0 阻塞。
 * 帧形态与 JDK 一致：wait0 (Native Method) 帧 + wait 各重载的字节码行号帧；
 * 接收者静态类型为 Object 与用户类两种调用形态；不持锁、负超时、越界纳秒、中断四种异常来源。
 */
public class TestObjectWaitFrames {
    static class Lock {
    }

    public static void main(String[] args) throws Exception {
        Object o = new Object();
        try {
            o.wait();
        } catch (IllegalMonitorStateException e) {
            print(e);
        }
        try {
            o.wait(10L);
        } catch (IllegalMonitorStateException e) {
            print(e);
        }
        try {
            o.wait(10L, 5);
        } catch (IllegalMonitorStateException e) {
            print(e);
        }
        Lock l = new Lock();
        try {
            l.wait();
        } catch (IllegalMonitorStateException e) {
            print(e);
        }
        try {
            l.wait(10L);
        } catch (IllegalMonitorStateException e) {
            print(e);
        }
        try {
            l.wait(10L, 5);
        } catch (IllegalMonitorStateException e) {
            print(e);
        }
        try {
            synchronized (o) {
                o.wait(-1L);
            }
        } catch (IllegalArgumentException e) {
            print(e);
        }
        try {
            synchronized (l) {
                l.wait(1L, -1);
            }
        } catch (IllegalArgumentException e) {
            print(e);
        }
        Thread.currentThread().interrupt();
        try {
            synchronized (l) {
                l.wait();
            }
        } catch (InterruptedException e) {
            print(e);
        }
        synchronized (o) {
            o.wait(1L);
        }
        System.out.println("done");
    }

    static void print(Throwable e) {
        System.out.println(e);
        for (StackTraceElement s : e.getStackTrace()) {
            System.out.println("  " + s.getClassName() + "." + s.getMethodName() + " " + s.getFileName()
                    + ":" + s.getLineNumber() + " native=" + s.isNativeMethod());
        }
    }
}
