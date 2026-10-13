
import java.lang.annotation.Retention;
import java.lang.annotation.RetentionPolicy;
import java.lang.reflect.Method;

import jakarta.servlet.annotation.WebServlet;
import jakarta.servlet.annotation.WebFilter;
import jakarta.servlet.annotation.WebInitParam;
import jakarta.servlet.annotation.WebListener;

/**
 * servlet-api pilot C6：注解元数据（RUNTIME 保留、免容器，W2 #6 无状态件）。
 */
public class ServletAnnotationMain {

    @WebServlet(name = "hello", urlPatterns = {"/hi", "/hello"},
            initParams = @WebInitParam(name = "enc", value = "utf-8"),
            loadOnStartup = 1, asyncSupported = true)
    static class HelloServlet {
    }

    @WebFilter(filterName = "logf", urlPatterns = "/*",
            servletNames = {"hello"})
    static class LogFilter {
    }

    @WebListener
    static class CtxListener {
    }

    static void check(String name, Object expected, Object actual) {
        boolean ok = String.valueOf(expected).equals(String.valueOf(actual));
        System.out.println((ok ? "PASS " : "FAIL ") + name
                + (ok ? "" : " :: expected=<" + expected + "> actual=<" + actual + ">"));
    }

    public static void main(String[] args) {
        // @WebServlet 全属性读取
        WebServlet servlet = HelloServlet.class.getAnnotation(WebServlet.class);
        check("servlet-name", "hello", servlet.name());
        check("url-patterns", "[/hi, /hello]", java.util.Arrays.toString(servlet.urlPatterns()));
        check("init-param-name", "enc", servlet.initParams()[0].name());
        check("init-param-value", "utf-8", servlet.initParams()[0].value());
        check("load-on-startup", 1, servlet.loadOnStartup());
        check("async-supported", true, servlet.asyncSupported());

        // @WebFilter
        WebFilter filter = LogFilter.class.getAnnotation(WebFilter.class);
        check("filter-name", "logf", filter.filterName());
        check("filter-patterns", "[/*]", java.util.Arrays.toString(filter.urlPatterns()));
        check("filter-servlet-names", "[hello]", java.util.Arrays.toString(filter.servletNames()));

        // @WebListener（无属性的标记注解）
        check("listener-present", true,
                CtxListener.class.isAnnotationPresent(WebListener.class));

        // 保留策略：容器运行期可见
        check("runtime-retention", RetentionPolicy.RUNTIME,
                WebServlet.class.getAnnotation(Retention.class).value());

        // 注解的注解面：@WebServlet 自身元信息
        check("annotation-target-type", true,
            WebServlet.class.isAnnotation() && !WebServlet.class.isInterface()
                || WebServlet.class.isAnnotation());
        System.out.println("DONE ServletAnnotationMain");
    }
}
