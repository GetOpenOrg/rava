
import org.slf4j.Logger;
import org.slf4j.LoggerFactory;
import org.slf4j.MDC;
import org.slf4j.Marker;
import org.slf4j.MarkerFactory;

/**
 * slf4j-api pilot A1：日志门面基础 API（W1 #1）。
 * 绑定侧为 NOP/logback(root OFF)，日志调用静默——断言的是 API 语义：
 * 名称/级别开关/MDC/Marker 的返回值与判断，不依赖日志真的输出。
 */
public class Slf4jBasicMain {

    static void check(String name, Runnable r) {
        try {
            r.run();
            System.out.println("PASS " + name);
        } catch (Throwable e) {
            System.out.println("FAIL " + name + " :: " + e);
        }
    }

    public static void main(String[] args) {
        Logger log = LoggerFactory.getLogger(Slf4jBasicMain.class);
        Logger logByName = LoggerFactory.getLogger("pilot.slf4j");

        check("logger-name", () -> {
            if (!Slf4jBasicMain.class.getName().equals(log.getName())) {
                throw new IllegalStateException(log.getName());
            }
        });
        check("logger-by-name", () -> {
            if (!"pilot.slf4j".equals(logByName.getName())) {
                throw new IllegalStateException(logByName.getName());
            }
        });
        check("same-logger-cached", () -> {
            if (LoggerFactory.getLogger("pilot.slf4j") != logByName) {
                throw new IllegalStateException("not cached");
            }
        });
        check("is-level-enabled", () -> {
            boolean d = log.isDebugEnabled();
            boolean i = log.isInfoEnabled();
            boolean w = log.isWarnEnabled();
            boolean e = log.isErrorEnabled();
            System.out.println("  levels: debug=" + d + " info=" + i + " warn=" + w + " error=" + e);
        });
        check("call-silent-ok", () -> {
            log.info("纯文本消息");
            log.info("带参数 {} 和 {}", "甲", Integer.valueOf(2));
            log.warn("带异常", new IllegalStateException("仅用于验证"));
            log.error("错误级别");
        });

        // MDC：map 语义
        MDC.put("traceId", "T-1001");
        MDC.put("userId", "u-42");
        check("mdc-get", () -> {
            if (!"T-1001".equals(MDC.get("traceId"))) {
                throw new IllegalStateException(MDC.get("traceId"));
            }
        });
        check("mdc-keyset-sorted", () -> {
            var keys = new java.util.TreeSet<>(MDC.getCopyOfContextMap().keySet());
            System.out.println("  mdc keys: " + keys);
        });
        MDC.remove("userId");
        check("mdc-remove", () -> {
            if (MDC.get("userId") != null) {
                throw new IllegalStateException("not removed");
            }
        });
        MDC.clear();

        // Marker 层级
        Marker parent = MarkerFactory.getMarker("PARENT");
        Marker child = MarkerFactory.getMarker("CHILD");
        parent.add(child);
        check("marker-contains", () -> {
            if (!parent.contains(child) || !parent.contains("CHILD")) {
                throw new IllegalStateException("contains broken");
            }
        });
        check("marker-name", () -> {
            if (!"CHILD".equals(child.getName()) || child.hasReferences()) {
                throw new IllegalStateException("child state");
            }
        });
        System.out.println("DONE Slf4jBasicMain");
    }
}
