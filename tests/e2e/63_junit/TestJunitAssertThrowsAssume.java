import org.junit.Test;
import org.junit.runner.JUnitCore;
import org.junit.runner.Result;
import org.junit.runner.notification.Failure;

import static org.junit.Assert.assertThrows;
import static org.junit.Assert.assertTrue;
import static org.junit.Assume.assumeFalse;
import static org.junit.Assume.assumeNoException;
import static org.junit.Assume.assumeTrue;

/**
 * 63_junit 首批：assertThrows（4.13+）与 Assume 族的前置不满足语义
 * （assumption violated → 计 run 不计 failure）。期望生成等 W0-3。
 */
public class TestJunitAssertThrowsAssume {

    public static class Sample {
        @Test
        public void throwsOk() {
            IllegalStateException e = assertThrows(IllegalStateException.class,
                    () -> { throw new IllegalStateException("thrown-msg"); });
            assertTrue("thrown-msg".equals(e.getMessage()));
        }

        @Test
        public void throwsWrongType() {
            assertThrows(IllegalStateException.class,
                    () -> { throw new IllegalArgumentException("x"); });
        }

        @Test
        public void throwsNothing() {
            assertThrows(IllegalStateException.class, () -> { });
        }

        @Test
        public void assumePass() {
            assumeTrue("前提成立", 1 + 1 == 2);
            assumeFalse(1 + 1 == 3);
        }

        @Test
        public void assumeViolated() {
            assumeTrue("前提不成立", 1 + 1 == 3);
        }

        @Test
        public void assumeNoExceptionPass() {
            assumeNoException(new IllegalStateException("不应出现"));
        }
    }

    public static void main(String[] args) {
        Result r = JUnitCore.runClasses(Sample.class);
        System.out.println("run=" + r.getRunCount() + " fail=" + r.getFailureCount()
                + " ignore=" + r.getIgnoreCount());
        for (Failure f : r.getFailures()) {
            System.out.println("failure " + f.getDescription().getMethodName()
                    + " :: " + f.getMessage());
        }
    }
}
