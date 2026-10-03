import java.io.ByteArrayOutputStream;
import java.util.logging.Formatter;
import java.util.logging.Level;
import java.util.logging.LogRecord;
import java.util.logging.Logger;
import java.util.logging.StreamHandler;

/**
 * java.logging 父子 Logger 语义：级别继承、Handler 继承传播、useParentHandlers
 * （jmod 覆盖计划 A 档；输出经自定义 Formatter 捕获后统一打印，规避 stderr/时间戳，
 * LogManager 初始化路径（conf/logging.properties / 系统属性）仍被完整走一遍）。
 */
public class TestLoggerHierarchy {

    static class Fixed extends Formatter {
        @Override
        public String format(LogRecord r) {
            return r.getLevel() + " " + r.getLoggerName() + " " + formatMessage(r);
        }
    }

    public static void main(String[] args) {
        ByteArrayOutputStream sink = new ByteArrayOutputStream();
        StreamHandler h = new StreamHandler(sink, new Fixed());
        h.setLevel(Level.ALL);

        Logger parent = Logger.getLogger("e2e.hierarchy");
        parent.setUseParentHandlers(false);
        parent.addHandler(h);
        parent.setLevel(Level.INFO);

        Logger child = Logger.getLogger("e2e.hierarchy.child");
        // child 未设级别也未设 handler：记录经 useParentHandlers=true 传播到 parent 的 handler，
        // 有效级别继承 parent 的 INFO
        child.info("child-info");
        child.fine("child-fine");           // 继承级别之下 → 过滤
        parent.warning("parent-warning");

        parent.setLevel(Level.WARNING);
        child.info("child-info-2");         // 继承级别收紧后被滤
        child.severe("child-severe");

        parent.setLevel(Level.INFO);
        child.setUseParentHandlers(false);
        child.info("child-isolated");       // 关闭传播 → 无 handler 接收

        h.flush();
        System.out.print(sink);
        System.out.println("child-null-level=" + (child.getLevel() == null));
        System.out.println("child-loggable-severe=" + child.isLoggable(Level.SEVERE));
        System.out.println("child-loggable-fine=" + child.isLoggable(Level.FINE));
        System.out.println("anon-name-null=" + (Logger.getAnonymousLogger().getName() == null));
    }
}
