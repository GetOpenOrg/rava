
import java.util.Arrays;
import java.util.TreeSet;

import jakarta.servlet.DispatcherType;
import jakarta.servlet.ServletContext;
import jakarta.servlet.SessionTrackingMode;
import jakarta.servlet.http.MappingMatch;
import jakarta.servlet.http.HttpServletMapping;

/**
 * servlet-api pilot C8：枚举与常量面（免容器，纯元数据比对）。
 */
public class ServletEnumMetaMain {

    static void check(String name, Object expected, Object actual) {
        boolean ok = String.valueOf(expected).equals(String.valueOf(actual));
        System.out.println((ok ? "PASS " : "FAIL ") + name
                + (ok ? "" : " :: expected=<" + expected + "> actual=<" + actual + ">"));
    }

    public static void main(String[] args) {
        // DispatcherType 全集与 valueOf
        check("dispatcher-values", "[FORWARD, INCLUDE, REQUEST, ASYNC, ERROR]",
                Arrays.toString(DispatcherType.values()));
        check("dispatcher-valueof", DispatcherType.ASYNC, DispatcherType.valueOf("ASYNC"));
        check("dispatcher-ordinal-REQUEST", 2, DispatcherType.REQUEST.ordinal());

        // SessionTrackingMode
        check("tracking-values", "[COOKIE, URL, SSL]",
                Arrays.toString(SessionTrackingMode.values()));
        check("tracking-tostring", "COOKIE", SessionTrackingMode.COOKIE.toString());

        // MappingMatch（servlet 4+ 的映射语义）
        check("mapping-values", "[CONTEXT_ROOT, DEFAULT, EXACT, EXTENSION, PATH]",
                Arrays.toString(MappingMatch.values()));
        check("mapping-exact", MappingMatch.EXACT, MappingMatch.valueOf("EXACT"));

        // ServletContext 常量（规范名）
        check("order-attr", "jakarta.servlet.context.orderedLibs",
                ServletContext.ORDERED_LIBS);
        check("tempdir-attr", "jakarta.servlet.context.tempdir",
                ServletContext.TEMPDIR);

        // 树排序的全枚举名串联（字节面稳定）
        TreeSet<String> all = new TreeSet<>();
        for (DispatcherType d : DispatcherType.values()) {
            all.add(d.name());
        }
        for (SessionTrackingMode m : SessionTrackingMode.values()) {
            all.add(m.name());
        }
        check("combined-sorted", "[ASYNC, COOKIE, ERROR, FORWARD, INCLUDE, REQUEST, SSL, URL]",
                all.toString());
        System.out.println("DONE ServletEnumMetaMain");
    }
}
