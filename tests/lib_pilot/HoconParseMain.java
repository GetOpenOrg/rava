
import com.typesafe.config.Config;
import com.typesafe.config.ConfigFactory;

/**
 * typesafe config pilot B7：HOCON 固定文本解析（W5）。
 */
public class HoconParseMain {

    static final String HOCON = """
            app {
              name = "pilot-app"
              version = "1.2.3"
            }
            db {
              host = "127.0.0.1"
              port = 5432
              timeout = 30s
              replicas = [ "r1", "r2" ]
            }
            # 覆盖语法：同键后者生效
            db.host = "10.0.0.1"
            score.high = 9.5
            score.low = 3
            """;

    static void check(String name, Object expected, Object actual) {
        boolean ok = String.valueOf(expected).equals(String.valueOf(actual));
        System.out.println((ok ? "PASS " : "FAIL ") + name
                + (ok ? "" : " :: expected=<" + expected + "> actual=<" + actual + ">"));
    }

    public static void main(String[] args) {
        Config c = ConfigFactory.parseString(HOCON);

        check("string", "pilot-app", c.getString("app.name"));
        check("version", "1.2.3", c.getString("app.version"));
        check("int", 5432, c.getInt("db.port"));
        check("override", "10.0.0.1", c.getString("db.host"));
        check("duration-millis", 30000, c.getDuration("db.timeout").toMillis());
        check("double", 9.5, c.getDouble("score.high"));
        check("path-list", "[r1, r2]", c.getStringList("db.replicas").toString());

        // 根层级与键集
        check("root-entries", "[app, db, score]",
                new java.util.TreeSet<>(c.root().keySet()).toString());

        // hasPath / 缺省
        check("haspath", true, c.hasPath("db.port"));
        check("missing", false, c.hasPath("db.user"));
        check("with-default-hit", 5432,
                c.withFallback(ConfigFactory.parseString("db.port=9999")).getInt("db.port"));

        // getObject 面板
        var db = c.getObject("db");
        check("object-keys", "[host, port, replicas, timeout]",
                new java.util.TreeSet<>(db.keySet()).toString());

        // 解析 boolean 与 null
        Config extra = ConfigFactory.parseString("flag=true, none=null");
        check("boolean", true, extra.getBoolean("flag"));
        check("null-value", true, extra.getIsNull("none"));
        System.out.println("DONE HoconParseMain");
    }
}
