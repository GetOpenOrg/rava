import java.util.logging.Filter;
import java.util.logging.Formatter;
import java.util.logging.Handler;
import java.util.logging.Level;
import java.util.logging.LogRecord;
import java.util.logging.Logger;

/**
 * java.logging Formatter / Filter / LogRecord 参数替换
 * （jmod 覆盖计划 A 档；自定义 Handler 捕获后统一打印，无时间戳成分）。
 */
public class TestLogHandlerFormat {

    static final StringBuilder OUT = new StringBuilder();

    static class Capture extends Handler {
        @Override
        public void publish(LogRecord r) {
            if (getFormatter() == null) {
                setFormatter(new Simple());
            }
            OUT.append(getFormatter().format(r));
        }

        @Override
        public void flush() {
        }

        @Override
        public void close() {
        }
    }

    static class Simple extends Formatter {
        @Override
        public String format(LogRecord r) {
            String base = r.getLevel() + " [" + formatMessage(r) + "]";
            if (r.getThrown() != null) {
                base += " thrown=" + r.getThrown().getClass().getSimpleName();
            }
            return base;
        }
    }

    public static void main(String[] args) {
        Logger log = Logger.getLogger("e2e.format");
        log.setUseParentHandlers(false);
        Capture h = new Capture();
        h.setLevel(Level.ALL);
        log.addHandler(h);
        log.setLevel(Level.ALL);

        // formatMessage 的 {0}/{1} 参数替换
        log.log(Level.INFO, "pair {0}-{1}", new Object[] { "a", "b" });

        // LogRecord 携带异常
        log.log(Level.WARNING, "with-cause", new IllegalStateException("boom-log"));

        // Filter 在 handler 层否决（logger 层放行）
        h.setFilter(new Filter() {
            @Override
            public boolean isLoggable(LogRecord r) {
                return r.getLevel().intValue() >= Level.WARNING.intValue();
            }
        });
        log.info("filtered-out");
        log.severe("passes-filter");

        System.out.print(OUT);
        System.out.println("filter-set=" + (h.getFilter() != null));
        System.out.println("level-parse=" + Level.parse("WARNING").intValue());
    }
}
