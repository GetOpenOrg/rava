import java.security.ProtectionDomain;

/**
 * ProtectionDomain 与 Package 元信息（方法级实测：Class.getProtectionDomain 8 jar /
 * Package.getImplementationVersion 7，此前零覆盖）：只打印布尔与规范版本号
   （不含机器路径与 JDK build 串）。
 */
public class TestProtectionDomainFaces {

    public static void main(String[] args) {
        ProtectionDomain pd = TestProtectionDomainFaces.class.getProtectionDomain();
        System.out.println("pd-nonnull=" + (pd != null));
        System.out.println("codesource-nonnull=" + (pd.getCodeSource() != null));
        System.out.println("principals-empty=" + (pd.getPrincipals().length == 0));
        System.out.println("permissions-nonnull=" + (pd.getPermissions() != null));

        // 类路径类的权限域类型
        System.out.println("pd-class=" + pd.getClass().getName().endsWith("ProtectionDomain"));

        // Package 元信息：java.lang 规范版本稳定（21）；自建包无实现元数据
        Package lang = String.class.getPackage();
        System.out.println("lang-spec=" + lang.getSpecificationVersion());
        System.out.println("lang-name=" + lang.getName());
        Package own = TestProtectionDomainFaces.class.getPackage();
        System.out.println("own-impl-null=" + (own.getImplementationVersion() == null)
                + " own-title-null=" + (own.getImplementationTitle() == null));

        // 同类同域
        System.out.println("same-pd=" + (String.class.getProtectionDomain() != null));
    }
}
