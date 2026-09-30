import java.io.IOException;
import java.net.URL;
import java.security.AccessController;
import java.security.AllPermission;
import java.security.CodeSource;
import java.security.Permission;
import java.security.Permissions;
import java.security.PrivilegedActionException;
import java.security.PrivilegedExceptionAction;
import java.security.SecurityPermission;
import java.security.cert.Certificate;
import java.util.ArrayList;
import java.util.Collections;
import java.util.Enumeration;
import java.util.List;
import java.util.PropertyPermission;

/**
 * gap 7e：java.security 的纯 Java 小类按字节码翻译。覆盖：Permissions.elements（PermissionsEnumerator
 * 跨权限类集合枚举）与 implies、AllPermission 蕴含一切、SecurityPermission 名字通配、CodeSource 的
 * equals / hashCode 一致性 / getLocation / toString、doPrivileged(PrivilegedExceptionAction) 把受检异常
 * 包成 PrivilegedActionException（getException / getCause / toString），非受检异常原样穿透。
 */
public class TestSecurityPermissions {
    @SuppressWarnings("removal")
    public static void main(String[] args) throws Exception {
        Permissions perms = new Permissions();
        perms.add(new PropertyPermission("user.*", "read"));
        perms.add(new SecurityPermission("getProperty.*"));
        perms.add(new RuntimePermission("exitVM.0"));
        List<String> names = new ArrayList<>();
        for (Enumeration<Permission> e = perms.elements(); e.hasMoreElements(); ) {
            Permission p = e.nextElement();
            names.add(p.getClass().getSimpleName() + ":" + p.getName() + ":" + p.getActions());
        }
        Collections.sort(names);
        System.out.println("elements=" + names);
        System.out.println("implies user.home read=" + perms.implies(new PropertyPermission("user.home", "read")));
        System.out.println("implies user.home write=" + perms.implies(new PropertyPermission("user.home", "write")));
        System.out.println("implies getProperty.x=" + perms.implies(new SecurityPermission("getProperty.securerandom.source")));
        System.out.println("implies setProperty.x=" + perms.implies(new SecurityPermission("setProperty.x")));
        System.out.println("implies exitVM.1=" + perms.implies(new RuntimePermission("exitVM.1")));

        AllPermission all = new AllPermission();
        System.out.println("all name=" + all.getName() + " actions=" + all.getActions()
                + " implies=" + all.implies(new RuntimePermission("anything"))
                + " eq=" + all.equals(new AllPermission()) + " hash=" + all.hashCode());
        Permissions withAll = new Permissions();
        withAll.add(all);
        System.out.println("withAll implies exitVM.9=" + withAll.implies(new RuntimePermission("exitVM.9")));

        URL loc = new URL("file:/opt/app/lib/app.jar");
        CodeSource cs1 = new CodeSource(loc, (Certificate[]) null);
        CodeSource cs2 = new CodeSource(new URL("file:/opt/app/lib/app.jar"), (Certificate[]) null);
        CodeSource cs3 = new CodeSource(new URL("file:/opt/app/lib/other.jar"), (Certificate[]) null);
        System.out.println("cs eq=" + cs1.equals(cs2) + " hashEq=" + (cs1.hashCode() == cs2.hashCode())
                + " ne=" + cs1.equals(cs3) + " loc=" + cs1.getLocation()
                + " certs=" + cs1.getCertificates());
        System.out.println("cs toString=" + cs1);

        try {
            AccessController.doPrivileged((PrivilegedExceptionAction<String>) () -> {
                throw new IOException("disk gone");
            });
        } catch (PrivilegedActionException e) {
            System.out.println("pae exception=" + e.getException());
            System.out.println("pae cause same=" + (e.getCause() == e.getException()));
            System.out.println("pae toString=" + e);
        }
        String ok = AccessController.doPrivileged((PrivilegedExceptionAction<String>) () -> "fine");
        System.out.println("ok=" + ok);
        try {
            AccessController.doPrivileged((PrivilegedExceptionAction<String>) () -> {
                throw new IllegalStateException("unchecked");
            });
        } catch (IllegalStateException e) {
            System.out.println("unchecked passes through: " + e.getMessage());
        }
    }
}
