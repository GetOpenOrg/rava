
import ognl.Ognl;
import ognl.OgnlContext;

/**
 * OGNL pilot A19：表达式求值核心面（W1 插队件，反射重灾区）。
 */
public class OgnlEvalMain {

    public static class Root {
        public String name = "root";
        public int level = 3;

        public String greet(String who) {
            return "hi " + who;
        }

        public static String statik() {
            return "static-value";
        }
    }

    static void check(String name, Object expected, Object actual) {
        boolean ok = String.valueOf(expected).equals(String.valueOf(actual));
        System.out.println((ok ? "PASS " : "FAIL ") + name
                + (ok ? "" : " :: expected=<" + expected + "> actual=<" + actual + ">"));
    }

    static Object eval(String expr, Object root) throws Exception {
        OgnlContext ctx = (OgnlContext) Ognl.createDefaultContext(root);
        return Ognl.getValue(expr, ctx, root);
    }

    public static void main(String[] args) throws Exception {
        Root root = new Root();

        // 属性访问（含继承自 Object 的方法）
        check("property", "root", eval("name", root));
        check("field-num", 3, eval("level", root));

        // 方法调用
        check("method-call", "hi ognl", eval("greet('ognl')", root));
        check("static-method", "static-value", eval("@OgnlEvalMain$Root@statik()", root));

        // 运算与条件
        check("arith", 7, eval("level + 4", root));
        check("ternary", "high", eval("level > 2 ? 'high' : 'low'", root));
        check("concat", "root-3", eval("name + '-' + level", root));

        // 赋值写回（反射写路径；#var 集合面见 OgnlCollectionMain）
        OgnlContext ctx = (OgnlContext) Ognl.createDefaultContext(root);
        check("assign", "changed", Ognl.getValue("name = 'changed'", ctx, root));
        check("assign-side-effect", "changed", root.name);

        // 常量与字面量
        check("literal", "x", eval("\"x\"", root));
        check("bool-true", Boolean.TRUE, eval("true", root));
        System.out.println("DONE OgnlEvalMain");
    }
}
