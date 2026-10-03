import javax.sql.rowset.CachedRowSet;
import javax.sql.rowset.RowSetProvider;

/**
 * java.sql.rowset：RowSetProvider 工厂构造与空 CachedRowSet 导航
 * （jmod 覆盖计划 B 档空提供者组；工厂经反射构造 com.sun.rowset 内部实现）。
 */
public class TestRowSetProvider {

    public static void main(String[] args) throws Exception {
        CachedRowSet crs = RowSetProvider.newFactory().createCachedRowSet();
        System.out.println("created=" + (crs != null));
        System.out.println("factory-same=" + (RowSetProvider.newFactory().getClass()
                == RowSetProvider.newFactory().getClass()));

        crs.setCommand("SELECT 1");
        System.out.println("command=" + crs.getCommand());
        crs.setTableName("t");
        System.out.println("table=" + crs.getTableName());

        // 空行集导航：无行的游标语义
        System.out.println("next=" + crs.next());
        System.out.println("prev=" + crs.previous());
        System.out.println("before-first=" + crs.isBeforeFirst());
        System.out.println("rows=" + crs.size());
    }
}
