import org.junit.Test;
import org.junit.runner.JUnitCore;
import org.junit.runner.Result;
import org.junit.runner.notification.Failure;

import static org.junit.Assert.assertEquals;
import static org.junit.Assert.assertTrue;

/**
 * M3 golden 用例：JUnitCore.runClasses 的 Runner 路径（docs/plans/
 * 2026-09-23-junit-crate-pilot.md M3——注解元数据 + 反射 L3 分派）。
 *
 * 覆盖面：@Test 注解发现（getDeclaredMethods + getAnnotation 的注解元数据
 * 表）、方法调用（Method.invoke 的 L3 分派）、测试类实例化（Constructor.
 * newInstance 的 <init> 分派臂）、断言失败路径（AssertionError 消息经
 * InvocationTargetException 包装/解包后的 Failure 语义）。
 *
 * 输出确定性：只打印 Result 的计数字段与 Failure.getMessage()（无栈迹、
 * 无身份哈希）——JVM 侧与翻译侧逐字对账。
 */
public class JunitRunnerMain {

    public static class SamplePass {
        @Test
        public void twoPlusTwo() {
            assertEquals(4, 2 + 2);
        }

        @Test
        public void stringCheck() {
            assertTrue("abc".startsWith("ab"));
        }
    }

    public static class SampleFail {
        @Test
        public void boom() {
            assertEquals("expected-x", "actual-y");
        }
    }

    public static void main(String[] args) {
        Result r1 = JUnitCore.runClasses(SamplePass.class);
        System.out.println("run=" + r1.getRunCount()
                + " fail=" + r1.getFailureCount()
                + " ok=" + r1.wasSuccessful());
        for (Failure f : r1.getFailures()) {
            System.out.println("unexpected: " + f.getMessage());
        }

        Result r2 = JUnitCore.runClasses(SampleFail.class);
        System.out.println("run=" + r2.getRunCount()
                + " fail=" + r2.getFailureCount()
                + " ok=" + r2.wasSuccessful());
        for (Failure f : r2.getFailures()) {
            System.out.println("failure: " + f.getMessage());
        }
    }
}
