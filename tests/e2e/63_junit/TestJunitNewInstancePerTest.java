import java.util.ArrayList;
import java.util.List;

import org.junit.AfterClass;
import org.junit.Before;
import org.junit.Test;
import org.junit.runner.JUnitCore;
import org.junit.runner.Result;

/**
 * 63_junit 首批：JUnit 4 每 @Test 新实例语义 + @Before 字段写入的存续
 * （lib pilot m5 缺陷的 e2e 形态版：@Before 写 subject → @Test/@After 读到一致值）。
 * 期望生成等 W0-3。
 */
public class TestJunitNewInstancePerTest {

    public static class Sample {
        static int constructed;
        static final List<String> SEEN = new ArrayList<>();

        String subject;

        Sample() {
            constructed++;
        }

        @Before
        public void setUp() {
            subject = "rava-" + constructed;
        }

        @Test
        public void alpha() {
            SEEN.add("alpha:" + subject);
        }

        @Test
        public void beta() {
            subject = subject.toUpperCase();
            SEEN.add("beta:" + subject);
        }

        @Test
        public void gamma() {
            SEEN.add("gamma:" + subject.length());
        }

        @AfterClass
        public static void report() {
            SEEN.add("constructed=" + constructed);
        }
    }

    public static void main(String[] args) {
        Result r = JUnitCore.runClasses(Sample.class);
        System.out.println("run=" + r.getRunCount() + " fail=" + r.getFailureCount());
        for (String s : Sample.SEEN) {
            System.out.println(s);
        }
    }
}
