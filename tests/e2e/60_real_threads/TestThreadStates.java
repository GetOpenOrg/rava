import java.util.concurrent.locks.LockSupport;

/** 阻塞中的 Thread.getState：sleep / wait(ms) → TIMED_WAITING，wait() / park() / join() → WAITING，竞争 synchronized → BLOCKED。 */
public class TestThreadStates {
    static final Object lock = new Object();

    static Thread.State settle(Thread t) throws InterruptedException {
        Thread.State s = t.getState();
        for (int i = 0; i < 400 && (s == Thread.State.RUNNABLE || s == Thread.State.NEW); i++) {
            Thread.sleep(5);
            s = t.getState();
        }
        return s;
    }

    public static void main(String[] args) throws Exception {
        Thread sleeper = new Thread(() -> {
            try {
                Thread.sleep(2000);
            } catch (InterruptedException e) {
            }
        });
        System.out.println("new: " + sleeper.getState());
        sleeper.start();
        System.out.println("sleep: " + settle(sleeper));
        sleeper.interrupt();
        sleeper.join();
        System.out.println("after: " + sleeper.getState());

        Thread waiter = new Thread(() -> {
            synchronized (lock) {
                try {
                    lock.wait();
                } catch (InterruptedException e) {
                }
            }
        });
        waiter.start();
        System.out.println("wait(): " + settle(waiter));
        synchronized (lock) {
            lock.notifyAll();
        }
        waiter.join();

        Thread timed = new Thread(() -> {
            synchronized (lock) {
                try {
                    lock.wait(2000);
                } catch (InterruptedException e) {
                }
            }
        });
        timed.start();
        System.out.println("wait(ms): " + settle(timed));
        timed.interrupt();
        timed.join();

        Thread parked = new Thread(LockSupport::park);
        parked.start();
        System.out.println("park: " + settle(parked));
        LockSupport.unpark(parked);
        parked.join();

        Thread blocked;
        synchronized (lock) {
            blocked = new Thread(() -> {
                synchronized (lock) {
                    lock.hashCode();
                }
            });
            blocked.start();
            System.out.println("monitor enter: " + settle(blocked));
        }
        blocked.join();

        Thread target = new Thread(() -> {
            try {
                Thread.sleep(300);
            } catch (InterruptedException e) {
            }
        });
        target.start();
        Thread joiner = new Thread(() -> {
            try {
                target.join();
            } catch (InterruptedException e) {
            }
        });
        joiner.start();
        System.out.println("join: " + settle(joiner));
        joiner.join();
        System.out.println("done: " + joiner.getState());
    }
}
