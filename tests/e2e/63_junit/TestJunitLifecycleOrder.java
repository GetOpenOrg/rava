import java.util.ArrayList;
import java.util.List;

import org.junit.After;
import org.junit.AfterClass;
import org.junit.Before;
import org.junit.BeforeClass;
import org.junit.Test;
import org.junit.runner.JUnitCore;
import org.junit.runner.Result;

/**
 * 63_junit 首批（生命周期注解族）：@BeforeClass → (@Before→@Test→@After)×N → @AfterClass
 * 的调用次序与静态/实例状态语义。期望生成等 W0-3。
 */
public class TestJunitLifecycleOrder {

    static final List<String> EVENTS = new ArrayList<>();
    static int beforeCount;
    static int afterCount;

    public static class Sample {
        int instanceValue;

        @BeforeClass
        public static void beforeAll() {
            EVENTS.add("beforeClass");
        }

        @AfterClass
        public static void afterAll() {
            EVENTS.add("afterClass");
        }

        @Before
        public void setUp() {
            beforeCount++;
            instanceValue = 42;
            EVENTS.add("before:" + beforeCount);
        }

        @After
        public void tearDown() {
            afterCount++;
            EVENTS.add("after:" + afterCount + " value=" + instanceValue);
        }

        @Test
        public void first() {
            instanceValue += 1;
            EVENTS.add("test:first value=" + instanceValue);
        }

        @Test
        public void second() {
            EVENTS.add("test:second value=" + instanceValue);
        }
    }

    public static void main(String[] args) {
        Result r = JUnitCore.runClasses(Sample.class);
        System.out.println("run=" + r.getRunCount() + " fail=" + r.getFailureCount()
                + " ok=" + r.wasSuccessful());
        for (String e : EVENTS) {
            System.out.println(e);
        }
    }
}
