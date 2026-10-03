import java.sql.Connection;
import java.sql.Driver;
import java.sql.DriverPropertyInfo;
import java.sql.DriverManager;
import java.sql.SQLException;
import java.sql.SQLFeatureNotSupportedException;
import java.util.Properties;

/**
 * java.sql DriverManager：自定义驱动的注册/查找/注销与无匹配路径
 * （jmod 覆盖计划 A 档；DriverManager 内部取调用者类接 CallerSensitive 工作，
 * 驱动服务查找为空路径——初始 drivers 计数为 0 的前提）。
 */
public class TestSqlDriverManager {

    static class FakeDriver implements Driver {
        @Override
        public Connection connect(String url, Properties info) {
            return null; // 返回 null → DriverManager 继续找 → 无匹配 SQLException
        }

        @Override
        public boolean acceptsURL(String url) {
            return url != null && url.startsWith("jdbc:fake:");
        }

        @Override
        public DriverPropertyInfo[] getPropertyInfo(String url, Properties info) {
            return new DriverPropertyInfo[0];
        }

        @Override
        public int getMajorVersion() {
            return 1;
        }

        @Override
        public int getMinorVersion() {
            return 0;
        }

        @Override
        public boolean jdbcCompliant() {
            return false;
        }

        @Override
        public java.util.logging.Logger getParentLogger() throws SQLFeatureNotSupportedException {
            throw new SQLFeatureNotSupportedException("no-parent-logger");
        }
    }

    public static void main(String[] args) throws Exception {
        System.out.println("initial-drivers=" + DriverManager.drivers().count());

        Driver d = new FakeDriver();
        DriverManager.registerDriver(d);
        System.out.println("after-register=" + DriverManager.drivers().count());
        System.out.println("driver-for=" + (DriverManager.getDriver("jdbc:fake:demo") == d));
        System.out.println("accepts=" + d.acceptsURL("jdbc:fake:x") + "/" + d.acceptsURL("jdbc:other:x"));

        try {
            DriverManager.getConnection("jdbc:fake:test");
        } catch (SQLException e) {
            System.out.println("conn-ex=" + e.getClass().getSimpleName());
        }
        try {
            DriverManager.getDriver("jdbc:unknown:x");
        } catch (SQLException e) {
            System.out.println("no-suitable=" + e.getClass().getSimpleName());
        }

        DriverManager.deregisterDriver(d);
        System.out.println("after-deregister=" + DriverManager.drivers().count());

        try {
            d.getParentLogger();
        } catch (SQLFeatureNotSupportedException e) {
            System.out.println("parent-logger-ex=" + e.getClass().getSimpleName());
        }
    }
}
