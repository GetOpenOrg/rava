import javax.naming.CompositeName;
import javax.naming.InitialContext;
import javax.naming.NoInitialContextException;

/**
 * java.naming：无提供者路径（jmod 覆盖计划 B 档空提供者组；
 * InitialContext 无工厂配置 → NoInitialContextException，服务查找为空）。
 */
public class TestJndiNoProvider {

    public static void main(String[] args) throws Exception {
        try {
            new InitialContext();
        } catch (NoInitialContextException e) {
            System.out.println("nic=" + e.getClass().getSimpleName());
        }

        CompositeName n = new CompositeName("a/b/c");
        System.out.println("size=" + n.size());
        System.out.println("get1=" + n.get(1));
        System.out.println("str=" + n.toString());
        System.out.println("empty=" + new CompositeName().isEmpty());
    }
}
