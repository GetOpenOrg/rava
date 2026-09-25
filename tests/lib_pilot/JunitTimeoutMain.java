import org.junit.Test;
import org.junit.runner.JUnitCore;
import org.junit.runner.Result;
import org.junit.runner.notification.Failure;

import static org.junit.Assert.assertEquals;

/**
 * M4 golden 用例：@Test(timeout=) 路径（docs/plans/2026-09-23-junit-crate-pilot.md M4）。
 *
 * JUnit 4 对带 timeout 的测试包一层 FailOnTimeout：新建线程（ThreadGroup + daemon）
 * 运行测试体，主线程以 FutureTask.get(timeout, unit) 限时等待结果。
 *
 * 覆盖面：限时内正常完成、限时内断言失败（异常经 FutureTask/ExecutionException
 * 传回主线程）、测试体内 Thread.sleep、同类混合带/不带 timeout 的方法。
 *
 * 不覆盖（线程模型方案 A 的档位边界，见 docs/compatibility.md）：测试体真实超时
 * （死循环 / sleep 超过限时）——单线程协作调度无抢占，模拟线程运行至完成，
 * 「test timed out after N milliseconds」不可达。
 */
public class JunitTimeoutMain {

    public static class TimedPass {
        @Test(timeout = 5000)
        public void quick() {
            assertEquals(6, 2 * 3);
        }

        @Test(timeout = 5000)
        public void sleepsBriefly() throws InterruptedException {
            Thread.sleep(1);
            assertEquals("ab", "a" + "b");
        }

        @Test
        public void untimed() {
            assertEquals(1, 1);
        }
    }

    public static class TimedFail {
        @Test(timeout = 5000)
        public void failsInTime() {
            assertEquals(7, 3 + 3);
        }
    }

    private static void report(String label, Result r) {
        System.out.println(label + " run=" + r.getRunCount()
                + " fail=" + r.getFailureCount()
                + " ok=" + r.wasSuccessful());
        for (Failure f : r.getFailures()) {
            System.out.println(label + " failure: " + f.getMessage());
        }
    }

    public static void main(String[] args) {
        report("pass", JUnitCore.runClasses(TimedPass.class));
        report("fail", JUnitCore.runClasses(TimedFail.class));
    }
}
