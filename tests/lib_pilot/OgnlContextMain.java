
import java.lang.reflect.Member;
import java.util.Map;

import ognl.Ognl;
import ognl.OgnlContext;
import ognl.TypeConverter;

/**
 * OGNL pilot A20：OgnlContext 变量绑定与自定义类型转换。
 */
public class OgnlContextMain {

    static void check(String name, Object expected, Object actual) {
        boolean ok = String.valueOf(expected).equals(String.valueOf(actual));
        System.out.println((ok ? "PASS " : "FAIL ") + name
                + (ok ? "" : " :: expected=<" + expected + "> actual=<" + actual + ">"));
    }

    /** 转换演示宿主：echo(int) 的参数触发 String→Integer 转换。 */
    public static class Echo {
        public String echo(int n) {
            return String.valueOf(n);
        }
    }

    public static class Tagged {
        public String getTag() {
            return "ctx-main";
        }
    }

    public static void main(String[] args) throws Exception {
        Object root = new Tagged();
        OgnlContext ctx = (OgnlContext) Ognl.createDefaultContext(root);

        // #var 变量
        ctx.put("threshold", Integer.valueOf(10));
        check("var-read", 10, Ognl.getValue("#threshold", ctx, root));
        check("var-compare", Boolean.TRUE, Ognl.getValue("#threshold >= 5", ctx, root));

        // #root 引用
        check("root-ref", "ctx-main", Ognl.getValue("#root.tag", ctx, root));

        // 内联 Map / List 构造
        check("map-get", "v", Ognl.getValue("#m = #{'k' : 'v'}, #m['k']", ctx, root));
        check("set-collect", "[1, 2]", String.valueOf(
                Ognl.getValue("#s = {1, 2}, #s", ctx, root)));

        // 方法调用参数的自动类型转换（String → int）
        check("type-convert", "21", Ognl.getValue("echo('21')", ctx, new Echo()));

        // context 的 Map 面（TreeSet 排序保证顺序）
        ctx.put("k2", "v2");
        check("context-keys", "[k2, threshold]",
                new java.util.TreeSet<>(ctx.keySet()).toString());
        System.out.println("DONE OgnlContextMain");
    }
}
