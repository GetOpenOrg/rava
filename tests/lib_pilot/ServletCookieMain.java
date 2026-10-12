
import jakarta.servlet.http.Cookie;

/**
 * servlet-api pilot C5：Cookie 语义面（免容器，W2 #6 无状态件）。
 */
public class ServletCookieMain {

    static void check(String name, Object expected, Object actual) {
        boolean ok = String.valueOf(expected).equals(String.valueOf(actual));
        System.out.println((ok ? "PASS " : "FAIL ") + name
                + (ok ? "" : " :: expected=<" + expected + "> actual=<" + actual + ">"));
    }

    public static void main(String[] args) {
        Cookie c = new Cookie("sid", "A1B2");
        c.setPath("/app");
        c.setDomain("example.com");
        c.setMaxAge(3600);
        c.setSecure(true);
        c.setHttpOnly(true);
        c.setAttribute("SameSite", "Strict");

        check("name", "sid", c.getName());
        check("value", "A1B2", c.getValue());
        check("path", "/app", c.getPath());
        check("domain", "example.com", c.getDomain());
        check("maxage", 3600, c.getMaxAge());
        check("secure", true, c.getSecure());
        check("httponly", true, c.isHttpOnly());
        check("version", 0, c.getVersion());
        check("attribute", "Strict", c.getAttribute("SameSite"));
        check("attribute-absent", null, c.getAttribute("Partitioned"));

        // setValue 之后原对象可变
        c.setValue("C3D4");
        check("mutated-value", "C3D4", c.getValue());

        // 会话 Cookie：maxAge=-1
        Cookie session = new Cookie("jsid", "x");
        check("session-maxage", -1, session.getMaxAge());

        // equals：全属性参与（含 secure/httpOnly/attribute 等）→ 克隆体判等
        check("equals-clone", true, c.equals(c.clone()));
        Cookie diffVal = new Cookie("sid", "ZZZ");
        diffVal.setPath("/app");
        diffVal.setDomain("example.com");
        check("equals-value-matters", false, c.equals(diffVal));

        // clone 面
        Cookie cloned = (Cookie) c.clone();
        check("clone-value", "C3D4", cloned.getValue());
        System.out.println("DONE ServletCookieMain");
    }
}
