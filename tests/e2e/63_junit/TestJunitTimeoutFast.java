import org.junit.Test;
import org.junit.runner.JUnitCore;
import org.junit.runner.Result;

import static org.junit.Assert.assertEquals;

/**
 * 63_junit 首批：@Test(timeout=) 限时内完成路径（快速通过/快速失败/sleep 少量）。
 * 真实超时（死循环）是线程模型已知边界（compatibility.md），不在此例。
 * 期望生成等 W0-3。
 */
public class TestJunitTimeoutFast {

    public static class Sample {
        @Test(timeout = 5000)
        public void quickPass() {
            assertEquals(6, 2 * 3);
        }

        @Test(timeout = 5000)
        public void quickFail() {
            assertEquals("expected-fast", "actual-fast");
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

    public static void main(String[] args) {
        Result r = JUnitCore.runClasses(Sample.class);
        System.out.println("run=" + r.getRunCount() + " fail=" + r.getFailureCount()
                + " ok=" + r.wasSuccessful());
        for (org.junit.runner.notification.Failure f : r.getFailures()) {
            System.out.println("failure " + f.getDescription().getMethodName()
                    + " :: " + f.getMessage());
        }
    }
}
