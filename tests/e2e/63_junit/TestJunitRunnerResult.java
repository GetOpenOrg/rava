import org.junit.Test;
import org.junit.runner.JUnitCore;
import org.junit.runner.Result;
import org.junit.runner.notification.Failure;

import static org.junit.Assert.assertEquals;

/**
 * 63_junit 首批：JUnitCore 多类运行与 Result/Failure 语义（计数、ComparisonFailure
 * 消息格式、wasSuccessful）。期望生成等 W0-3。
 */
public class TestJunitRunnerResult {

    public static class Passing {
        @Test
        public void twoPlusTwo() {
            assertEquals(4, 2 + 2);
        }

        @Test
        public void stringCheck() {
            assertEquals("ab", "a" + "b");
        }
    }

    public static class Failing {
        @Test
        public void mismatch() {
            assertEquals("expected-x", "actual-y");
        }

        @Test
        public void boom() {
            throw new IllegalStateException("runner-boom");
        }
    }

    public static class Mixed {
        @Test
        public void pass() {
            assertEquals(1, 1);
        }

        @Test
        public void fail() {
            assertEquals("e", "f");
        }
    }

    public static void main(String[] args) {
        Result pass = JUnitCore.runClasses(Passing.class);
        System.out.println("pass-run=" + pass.getRunCount() + " fail=" + pass.getFailureCount()
                + " ok=" + pass.wasSuccessful());

        Result fail = JUnitCore.runClasses(Failing.class);
        System.out.println("fail-run=" + fail.getRunCount() + " fail=" + fail.getFailureCount()
                + " ok=" + fail.wasSuccessful());
        for (Failure f : fail.getFailures()) {
            System.out.println("failure " + f.getDescription().getMethodName()
                    + " :: " + f.getMessage());
        }

        Result multi = JUnitCore.runClasses(Passing.class, Mixed.class);
        System.out.println("multi-run=" + multi.getRunCount() + " fail=" + multi.getFailureCount());
        System.out.println("multi-first-failure=" + (multi.getFailures().isEmpty()
                ? "none" : multi.getFailures().get(0).getDescription().getMethodName()));
    }
}
