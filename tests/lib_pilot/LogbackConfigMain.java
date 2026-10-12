
import java.util.List;

import ch.qos.logback.classic.Level;
import ch.qos.logback.classic.Logger;
import ch.qos.logback.classic.LoggerContext;
import ch.qos.logback.classic.spi.ILoggingEvent;
import ch.qos.logback.core.read.ListAppender;

/**
 * Logback pilot B12：程序化配置——additivity、logger 层级与 Marker 过滤。
 */
public class LogbackConfigMain {

    static void check(String name, Object expected, Object actual) {
        boolean ok = String.valueOf(expected).equals(String.valueOf(actual));
        System.out.println((ok ? "PASS " : "FAIL ") + name
                + (ok ? "" : " :: expected=<" + expected + "> actual=<" + actual + ">"));
    }

    public static void main(String[] args) {
        LoggerContext ctx = new LoggerContext();
        ctx.setName("pilot-lb-config");

        ListAppender<ILoggingEvent> rootAppender = new ListAppender<>();
        rootAppender.setContext(ctx);
        rootAppender.start();
        Logger root = ctx.getLogger(org.slf4j.Logger.ROOT_LOGGER_NAME);
        root.setLevel(Level.INFO);
        root.addAppender(rootAppender);

        // 子 logger：默认 additivity=true → 事件同时冒泡到 root
        Logger child = ctx.getLogger("pilot.child");
        child.setLevel(Level.DEBUG);

        ListAppender<ILoggingEvent> childAppender = new ListAppender<>();
        childAppender.setContext(ctx);
        childAppender.start();
        child.addAppender(childAppender);

        child.debug("冒泡事件");
        child.info("子级信息");

        // 子 logger 的 appender 收到 2 条；root 因 additivity 也收到 2 条
        check("child-appender-count", 2, childAppender.list.size());
        check("root-bubbled-count", 2, rootAppender.list.size());

        // 关闭 additivity：后续事件不再冒泡
        child.setAdditive(false);
        child.warn("仅子级");
        check("after-additivity-off-child", 3, childAppender.list.size());
        check("after-additivity-off-root", 2, rootAppender.list.size());

        // logger 层级：中间层未定义时按最近祖先级别
        Logger grandChild = ctx.getLogger("pilot.child.grand");
        check("effective-level", Level.DEBUG, grandChild.getEffectiveLevel());

        // 事件元数据面
        ILoggingEvent e = childAppender.list.get(2);
        check("event-meta", "pilot.child|WARN|仅子级",
                e.getLoggerName() + "|" + e.getLevel() + "|" + e.getFormattedMessage());
        check("thread-name", "main", e.getThreadName());

        childAppender.stop();
        rootAppender.stop();
        ctx.stop();
        System.out.println("DONE LogbackConfigMain");
    }
}
