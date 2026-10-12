
import java.util.List;

import ch.qos.logback.classic.Level;
import ch.qos.logback.classic.Logger;
import ch.qos.logback.classic.LoggerContext;
import ch.qos.logback.classic.PatternLayout;
import ch.qos.logback.classic.spi.ILoggingEvent;
import ch.qos.logback.core.read.ListAppender;

/**
 * Logback pilot B11：编程式 pattern 布局 + ListAppender 捕获
 * （排 slf4j green 之后；固定上下文，输出逐字断言）。
 */
public class LogbackPatternMain {

    static void eq(String name, String expected, String actual) {
        System.out.println((expected.equals(actual) ? "PASS " : "FAIL ") + name
                + (expected.equals(actual) ? "" : " :: expected=<" + expected + "> actual=<" + actual + ">"));
    }

    public static void main(String[] args) {
        LoggerContext ctx = new LoggerContext();
        ctx.setName("pilot-lb");
        ctx.putProperty("appEnv", "test");

        PatternLayout layout = new PatternLayout();
        layout.setContext(ctx);
        layout.setPattern("%level|%logger|%msg|%property{appEnv}%nopex");
        layout.start();

        Logger log = ctx.getLogger("pilot.Pattern");
        log.setLevel(Level.INFO);
        ListAppender<ILoggingEvent> appender = new ListAppender<>();
        appender.setContext(ctx);
        appender.start();
        log.addAppender(appender);

        // 发射固定事件（含参数与异常）
        log.info("纯文本");
        log.warn("参数 {} 与 {}", "甲", 2);
        log.error("带异常", new IllegalStateException("E"));
        log.debug("被级别过滤"); // INFO 之上，不进 appender

        List<ILoggingEvent> events = appender.list;
        eq("events-count", "3", String.valueOf(events.size()));
        eq("event1", "INFO|pilot.Pattern|纯文本|test",
                layout.doLayout(events.get(0)).strip());
        eq("event2", "WARN|pilot.Pattern|参数 甲 与 2|test",
                layout.doLayout(events.get(1)).strip());
        String third = layout.doLayout(events.get(2)).strip();
        eq("event3-head", "ERROR|pilot.Pattern|带异常|test", third);

        // throwable 的 pattern 变体（%ex 短栈首行）
        PatternLayout exLayout = new PatternLayout();
        exLayout.setContext(ctx);
        exLayout.setPattern("%msg => %ex{1}");
        exLayout.start();
        String exLine = exLayout.doLayout(events.get(2)).strip();
        eq("event3-throwable", "带异常 => java.lang.IllegalStateException: E",
                exLine.split("\\n")[0]); // %ex{1} 之后是栈帧行

        // 级别过滤语义
        eq("level-filter", "true",
                String.valueOf(events.stream().noneMatch(e -> e.getLevel() == Level.DEBUG)));
        layout.stop();
        exLayout.stop();
        appender.stop();
        ctx.stop();
        System.out.println("DONE LogbackPatternMain");
    }
}
