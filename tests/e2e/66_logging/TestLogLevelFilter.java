import java.util.logging.Level;
import java.util.logging.LogRecord;
import java.util.logging.Logger;

/**
 * java.logging Level 解析与 logger 层过滤
 * （jmod 覆盖计划 A 档；Level.parse 数值/名称两形态、isLoggable 边界）。
 */
public class TestLogLevelFilter {

    public static void main(String[] args) {
        System.out.println("info-int=" + Level.INFO.intValue());
        System.out.println("parse-name=" + Level.parse("CONFIG").intValue());
        System.out.println("parse-num=" + Level.parse("900").intValue());
        System.out.println("parse-off=" + Level.parse("OFF").intValue());

        // 无此名也无此值 → IllegalArgumentException
        try {
            Level.parse("NO_SUCH_LEVEL");
        } catch (IllegalArgumentException e) {
            System.out.println("parse-ex=" + e.getClass().getSimpleName());
        }

        Logger log = Logger.getLogger("e2e.levels");
        log.setLevel(Level.CONFIG);
        System.out.println("loggable-config=" + log.isLoggable(Level.CONFIG));
        System.out.println("loggable-fine=" + log.isLoggable(Level.FINE));
        System.out.println("loggable-severe=" + log.isLoggable(Level.SEVERE));

        // LogRecord 层构造（不经 logger，直接验证记录语义）
        LogRecord r = new LogRecord(Level.INFO, "rec");
        r.setSequenceNumber(7L);
        r.setThreadID(3);
        System.out.println("rec-level=" + r.getLevel().getName()
                + " seq=" + r.getSequenceNumber() + " tid=" + r.getThreadID()
                + " msg=[" + r.getMessage() + "]");
    }
}
