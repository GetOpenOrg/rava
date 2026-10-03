import org.junit.Test;
import org.junit.runner.JUnitCore;
import org.junit.runner.Result;
import org.junit.runner.notification.Failure;

import static org.junit.Assert.assertArrayEquals;
import static org.junit.Assert.assertEquals;
import static org.junit.Assert.assertNull;
import static org.junit.Assert.assertSame;

/**
 * 63_junit 首批：Runner 路径下各类断言失败的 Failure.getMessage() 形态
 * （经 InvocationTargetException 解包后的消息文本）。期望生成等 W0-3。
 */
public class TestJunitFailureMessages {

    public static class Sample {
        @Test
        public void intMismatch() {
            assertEquals(1, 2);
        }

        @Test
        public void stringMismatch() {
            assertEquals("left", "right");
        }

        @Test
        public void doubleMismatch() {
            assertEquals(1.5, 2.5, 1e-9);
        }

        @Test
        public void arrayMismatch() {
            assertArrayEquals(new long[] { 1L, 2L }, new long[] { 1L, 3L });
        }

        @Test
        public void nullMismatch() {
            assertNull("not-null");
        }

        @Test
        public void sameMismatch() {
            assertSame(new Object(), new Object());
        }

        @Test
        public void runtimeException() {
            throw new UnsupportedOperationException("not-implemented");
        }
    }

    public static void main(String[] args) {
        Result r = JUnitCore.runClasses(Sample.class);
        System.out.println("fail=" + r.getFailureCount());
        for (Failure f : r.getFailures()) {
            System.out.println(f.getDescription().getMethodName() + " :: " + f.getMessage());
        }
    }
}
