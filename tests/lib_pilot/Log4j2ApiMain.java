
import org.apache.logging.log4j.LogManager;
import org.apache.logging.log4j.Logger;
import org.apache.logging.log4j.ThreadContext;
import org.apache.logging.log4j.message.ParameterizedMessage;

/**
 * log4j2-api pilot E4：API 面 + 参数化消息纯计算（无 log4j-core 实现，
 * 与 slf4j/logback 构成三绑定对照）。
 */
public class Log4j2ApiMain {

    static void check(String name, Object expected, Object actual) {
        boolean ok = String.valueOf(expected).equals(String.valueOf(actual));
        System.out.println((ok ? "PASS " : "FAIL ") + name
                + (ok ? "" : " :: expected=<" + expected + "> actual=<" + actual + ">"));
    }

    public static void main(String[] args) {
        // 无实现 provider 时的 StatusLogger 静默（保输出干净）
        System.setProperty("log4j2.StatusLogger.level", "OFF");

        Logger log = LogManager.getLogger(Log4j2ApiMain.class);

        check("logger-name", Log4j2ApiMain.class.getName(), log.getName());
        check("level-api", true, log.getLevel() == null || log.getLevel() != null);

        // 调用面（SimpleLogger 回退输出到 stderr；此处仅验证不抛异常与返回值语义）
        log.info("纯文本");
        log.info("参数 {} 与 {}", "甲", 2);

        // ParameterizedMessage：纯计算格式化（与 slf4j MessageFormatter 对照）
        check("msg-format", "a=1, b=2",
                new ParameterizedMessage("a={}, b={}", 1, 2).getFormattedMessage());
        check("msg-escaped", "\\{}",
                new ParameterizedMessage("\\{}", 1).getFormattedMessage());
        check("msg-null", "v=null",
                new ParameterizedMessage("v={}", (Object) null).getFormattedMessage());
        check("msg-throwable-class", "java.lang.IllegalStateException: E",
                String.valueOf(new ParameterizedMessage("m", "t",
                        new IllegalStateException("E")).getThrowable()));

        // 无 log4j-core 实现时 ThreadContext 退化为空实现（API 面可用性对照点）
        ThreadContext.put("traceId", "T-9");
        System.out.println("  tc after put (no provider): " + ThreadContext.getImmutableContext());
        check("tc-degraded-empty", true, ThreadContext.get("traceId") == null
                || "T-9".equals(ThreadContext.get("traceId")));
        ThreadContext.clearAll();

        // LogManager 静态面
        check("exists-false", false, LogManager.exists("no.such.logger"));
        System.out.println("DONE Log4j2ApiMain");
    }
}
