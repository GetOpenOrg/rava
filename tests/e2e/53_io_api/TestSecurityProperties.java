import java.security.Security;

/**
 * java.security.Security 属性（Rosetta SerializableDemo 揭出：ObjectInputFilter$Config.<clinit>
 * 读 jdk.serialFilter，Security.getProperty 为存根）。覆盖：java.security 基线键（JDK 21/25 同值者）、
 * 未定义键 → null、setProperty 覆盖 / 新增后读回、值两端空白 trim、null 键 → NPE。
 */
public class TestSecurityProperties {
    public static void main(String[] args) {
        for (String k : new String[]{"security.provider.1", "securerandom.source", "keystore.type",
                "networkaddress.cache.negative.ttl", "login.configuration.provider",
                "securerandom.strongAlgorithms", "jdk.serialFilter", "no.such.key"}) {
            System.out.println(k + " = " + Security.getProperty(k));
        }
        Security.setProperty("keystore.type", "jks");
        System.out.println("override keystore.type = " + Security.getProperty("keystore.type"));
        Security.setProperty("my.custom.key", "  padded value  ");
        System.out.println("custom = [" + Security.getProperty("my.custom.key") + "]");
        try {
            Security.getProperty(null);
            System.out.println("no NPE?");
        } catch (NullPointerException e) {
            System.out.println("null key -> NPE");
        }
    }
}
