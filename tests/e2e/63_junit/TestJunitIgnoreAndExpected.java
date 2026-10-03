import org.junit.Ignore;
import org.junit.Test;
import org.junit.runner.JUnitCore;
import org.junit.runner.Result;
import org.junit.runner.notification.Failure;

import static org.junit.Assert.assertTrue;

/**
 * 63_junit 首批：@Ignore 计数语义 + @Test(expected=…) 三态（命中/子类/不命中→失败）。
 * 期望生成等 W0-3。
 */
public class TestJunitIgnoreAndExpected {

    public static class Sample {
        @Test
        public void normal() {
            assertTrue(true);
        }

        @Ignore("未实现")
        @Test
        public void skippedWithReason() {
        }

        @Ignore
        @Test
        public void skippedNoReason() {
        }

        @Test(expected = IllegalStateException.class)
        public void exactHit() {
            throw new IllegalStateException("expected-hit");
        }

        @Test(expected = RuntimeException.class)
        public void subclassHit() {
            throw new IllegalStateException("subclass");
        }

        @Test(expected = IllegalStateException.class)
        public void notThrown() {
            // 应抛未抛 → 失败
        }

        @Test(expected = IllegalStateException.class)
        public void wrongType() {
            throw new IllegalArgumentException("wrong");
        }
    }

    public static void main(String[] args) {
        Result r = JUnitCore.runClasses(Sample.class);
        System.out.println("run=" + r.getRunCount() + " fail=" + r.getFailureCount()
                + " ignore=" + r.getIgnoreCount() + " ok=" + r.wasSuccessful());
        for (Failure f : r.getFailures()) {
            System.out.println("failure " + f.getDescription().getMethodName()
                    + " :: " + f.getMessage());
        }
    }
}
