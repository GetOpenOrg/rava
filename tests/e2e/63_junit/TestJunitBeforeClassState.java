import java.util.ArrayList;
import java.util.List;

import org.junit.AfterClass;
import org.junit.BeforeClass;
import org.junit.Test;
import org.junit.runner.JUnitCore;
import org.junit.runner.Result;

import static org.junit.Assert.assertEquals;

/**
 * 63_junit 首批：@BeforeClass 静态状态在全部 @Test 间的共享语义
 * （与每测试新实例相对——类级一次初始化）。期望生成等 W0-3。
 */
public class TestJunitBeforeClassState {

    public static class Sample {
        static List<String> log;
        static int initOrder;

        @BeforeClass
        public static void initAll() {
            log = new ArrayList<>();
            log.add("beforeClass");
            initOrder = 1;
        }

        @Test
        public void readsShared() {
            assertEquals(1, initOrder);
            log.add("reads:" + ++initOrder);
        }

        @Test
        public void seesEarlierWrite() {
            log.add("sees:" + initOrder);
        }

        @AfterClass
        public static void report() {
            for (String s : log) {
                System.out.println(s);
            }
            System.out.println("final=" + initOrder);
        }
    }

    public static void main(String[] args) {
        Result r = JUnitCore.runClasses(Sample.class);
        System.out.println("run=" + r.getRunCount() + " fail=" + r.getFailureCount()
                + " ok=" + r.wasSuccessful());
    }
}
