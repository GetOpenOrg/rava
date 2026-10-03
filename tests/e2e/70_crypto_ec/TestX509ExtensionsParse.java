import java.io.ByteArrayInputStream;
import java.nio.charset.StandardCharsets;
import java.security.cert.CRLReason;
import java.security.cert.CertificateFactory;
import java.security.cert.X509CRL;
import java.security.cert.X509CRLEntry;
import java.security.cert.X509Certificate;
import java.util.ArrayList;
import java.util.Collection;
import java.util.List;
import java.util.Set;
import java.util.TreeSet;

/**
 * X.509 扩展解析（闭包边界）：证书 / CRL 的扩展按 OID 经 OIDMap 反射构造扩展类，
 * 覆盖证书策略限定符（CPS / UserNotice → PolicyQualifierInfo）、策略约束 / 映射、名称约束、
 * AKI / SKI、CRL 分发点、AIA、Netscape 证书类型、Freshest CRL，以及 CRL 条目吊销原因（CRLReason）。
 * PEM 由 openssl 预先生成（EC P-256），输出不依赖当前时间。
 */
public class TestX509ExtensionsParse {

    static final String CA = """
        -----BEGIN CERTIFICATE-----
        MIICYDCCAgagAwIBAgICEAEwCgYIKoZIzj0EAwIwMzEVMBMGA1UEAwwMUmF2YSBU
        ZXN0IENBMQ0wCwYDVQQKDARSYXZhMQswCQYDVQQGEwJDTjAgFw0yNjAxMDEwMDAw
        MDBaGA8yMTI2MDEwMTAwMDAwMFowMzEVMBMGA1UEAwwMUmF2YSBUZXN0IENBMQ0w
        CwYDVQQKDARSYXZhMQswCQYDVQQGEwJDTjBZMBMGByqGSM49AgEGCCqGSM49AwEH
        A0IABJ+C0+mhyXe5Z4DROQynIBuoGG+tMKqq9SCCRvudvDPgWvUzsPQR+ZUEKT0A
        3qKiZlqMQBw4RhD6BSlipcOaf9mjggEGMIIBAjASBgNVHRMBAf8ECDAGAQH/AgEB
        MA4GA1UdDwEB/wQEAwIBBjAdBgNVHQ4EFgQUX6WluBp+kgjncMAxGVhOgqK3CDUw
        UwYDVR0gBEwwSjBIBgQqAwQFMEAwIwYIKwYBBQUHAgEWF2h0dHBzOi8vZXhhbXBs
        ZS5jb20vY3BzMBkGCCsGAQUFBwICMA0aC1JhdmEgbm90aWNlMA8GA1UdJAQIMAaA
        AQCBAQEwCgYDVR02BAMCAQIwFwYDVR0hBBAwDjAMBgQqAwQFBgQqAwQGMDIGA1Ud
        HgEB/wQoMCagDzANggtleGFtcGxlLmNvbaETMBGCD2JhZC5leGFtcGxlLmNvbTAK
        BggqhkjOPQQDAgNIADBFAiEAggUdW2MzFPhmrpzkC5TZ3SZhX6s4t9AYKuwPaK7d
        cqwCICjLqPaHvJmdxVnivPXDyPR7vKdiDPGy/44STrE2Vv6z
        -----END CERTIFICATE-----
        """;

    static final String LEAF = """
        -----BEGIN CERTIFICATE-----
        MIIDEDCCAragAwIBAgICIAIwCgYIKoZIzj0EAwIwMzEVMBMGA1UEAwwMUmF2YSBU
        ZXN0IENBMQ0wCwYDVQQKDARSYXZhMQswCQYDVQQGEwJDTjAgFw0yNjAxMDEwMDAw
        MDBaGA8yMTI2MDEwMTAwMDAwMFowNjEYMBYGA1UEAwwPd3d3LmV4YW1wbGUuY29t
        MQ0wCwYDVQQKDARSYXZhMQswCQYDVQQGEwJDTjBZMBMGByqGSM49AgEGCCqGSM49
        AwEHA0IABOGoB3koWNkR9yhRxO0SwnMdehDx0xt4BUS1R/7t8rNC9YFrX47CvYi/
        V3PyyNNGtlJHgz9cZ99r8U8yXHSjWRqjggGzMIIBrzAJBgNVHRMEAjAAMA4GA1Ud
        DwEB/wQEAwIHgDAdBgNVHSUEFjAUBggrBgEFBQcDAQYIKwYBBQUHAwIwHQYDVR0O
        BBYEFGJzZdnbFfAWcIMWkWGmicqa7lfKMB8GA1UdIwQYMBaAFF+lpbgafpII53DA
        MRlYToKitwg1MC8GA1UdEQQoMCaCD3d3dy5leGFtcGxlLmNvbYcECgAAAYENYUBl
        eGFtcGxlLmNvbTAhBgNVHRIEGjAYhhZodHRwczovL2V4YW1wbGUuY29tL2NhMCsG
        A1UdHwQkMCIwIKAeoByGGmh0dHBzOi8vZXhhbXBsZS5jb20vY2EuY3JsMFwGCCsG
        AQUFBwEBBFAwTjAkBggrBgEFBQcwAYYYaHR0cHM6Ly9vY3NwLmV4YW1wbGUuY29t
        MCYGCCsGAQUFBzAChhpodHRwczovL2V4YW1wbGUuY29tL2NhLmNlcjARBgNVHSAE
        CjAIMAYGBCoDBAYwEQYJYIZIAYb4QgEBBAQDAgZAMC4GA1UdLgQnMCUwI6AhoB+G
        HWh0dHBzOi8vZXhhbXBsZS5jb20vZGVsdGEuY3JsMAoGCCqGSM49BAMCA0gAMEUC
        IGuYZctA4SWt7alaTJ+CIekBfIVNt5lJaPKotBLF2is3AiEA3jp2l4Vb+UK5pgVV
        jphF/W/WVjAK5b3R9fF5udyOIVM=
        -----END CERTIFICATE-----
        """;

    static final String CRL = """
        -----BEGIN X509 CRL-----
        MIIBTDCB8gIBATAKBggqhkjOPQQDAjAzMRUwEwYDVQQDDAxSYXZhIFRlc3QgQ0Ex
        DTALBgNVBAoMBFJhdmExCzAJBgNVBAYTAkNOFw0yNjEwMDMxMjIyNTdaGA8yMTI2
        MDkwOTEyMjI1N1owWzAhAgIwAxcNMjYwMjAxMDAwMDAwWjAMMAoGA1UdFQQDCgEB
        MCECAjAEFw0yNjAyMDIwMDAwMDBaMAwwCgYDVR0VBAMKAQQwEwICMAUXDTI2MDIw
        MzAwMDAwMFqgLzAtMB8GA1UdIwQYMBaAFF+lpbgafpII53DAMRlYToKitwg1MAoG
        A1UdFAQDAgEFMAoGCCqGSM49BAMCA0kAMEYCIQCo5/J8DfCiPkrWCZ9JJWzSwYZb
        oGAxJNEcnNJKYzfvcAIhAIiwDn5ldAY6pf31ya6dlGcJuyBnTnTsQ3T9pU54KvK9
        -----END X509 CRL-----
        """;

    static byte[] bytes(String s) {
        return s.getBytes(StandardCharsets.US_ASCII);
    }

    static String sorted(Set<String> s) {
        return s == null ? "null" : new TreeSet<>(s).toString();
    }

    static void exts(String tag, X509Certificate c) {
        System.out.println(tag + " critical=" + sorted(c.getCriticalExtensionOIDs()));
        System.out.println(tag + " noncritical=" + sorted(c.getNonCriticalExtensionOIDs()));
        TreeSet<String> all = new TreeSet<>(c.getCriticalExtensionOIDs());
        all.addAll(c.getNonCriticalExtensionOIDs());
        for (String oid : all) {
            System.out.println(tag + " ext " + oid + " len=" + c.getExtensionValue(oid).length);
        }
    }

    public static void main(String[] args) throws Exception {
        CertificateFactory cf = CertificateFactory.getInstance("X.509");
        X509Certificate ca = (X509Certificate) cf.generateCertificate(new ByteArrayInputStream(bytes(CA)));
        X509Certificate leaf = (X509Certificate) cf.generateCertificate(new ByteArrayInputStream(bytes(LEAF)));

        System.out.println("ca subject=" + ca.getSubjectX500Principal().getName());
        System.out.println("ca version=" + ca.getVersion() + " serial=" + ca.getSerialNumber().toString(16)
                + " sigalg=" + ca.getSigAlgName() + " pathlen=" + ca.getBasicConstraints());
        exts("ca", ca);
        ca.verify(ca.getPublicKey());
        System.out.println("ca self-verify ok");

        System.out.println("leaf subject=" + leaf.getSubjectX500Principal().getName());
        System.out.println("leaf issuer=" + leaf.getIssuerX500Principal().getName());
        System.out.println("leaf pathlen=" + leaf.getBasicConstraints());
        boolean[] ku = leaf.getKeyUsage();
        StringBuilder kb = new StringBuilder();
        for (boolean b : ku) {
            kb.append(b ? '1' : '0');
        }
        System.out.println("leaf keyUsage=" + kb);
        System.out.println("leaf eku=" + leaf.getExtendedKeyUsage());
        Collection<List<?>> san = leaf.getSubjectAlternativeNames();
        for (List<?> e : san) {
            System.out.println("leaf san " + e.get(0) + " " + e.get(1));
        }
        for (List<?> e : leaf.getIssuerAlternativeNames()) {
            System.out.println("leaf ian " + e.get(0) + " " + e.get(1));
        }
        exts("leaf", leaf);
        leaf.verify(ca.getPublicKey());
        System.out.println("leaf verify ok");

        X509CRL crl = (X509CRL) cf.generateCRL(new ByteArrayInputStream(bytes(CRL)));
        System.out.println("crl issuer=" + crl.getIssuerX500Principal().getName() + " version=" + crl.getVersion());
        System.out.println("crl noncritical=" + sorted(crl.getNonCriticalExtensionOIDs()));
        crl.verify(ca.getPublicKey());
        System.out.println("crl verify ok");
        List<X509CRLEntry> entries = new ArrayList<>(crl.getRevokedCertificates());
        entries.sort((a, b) -> a.getSerialNumber().compareTo(b.getSerialNumber()));
        for (X509CRLEntry e : entries) {
            CRLReason r = e.getRevocationReason();
            System.out.println("crl entry " + e.getSerialNumber().toString(16) + " reason=" + r
                    + " ordinal=" + (r == null ? -1 : r.ordinal()) + " hasExt=" + e.hasExtensions());
        }
        System.out.println("leaf revoked=" + crl.isRevoked(leaf));

        // 扩展的完整文本表示：逐扩展类 toString（含策略限定符、名称约束、策略映射、CRL 原因）
        for (String line : ca.toString().split("\n")) {
            String t = line.trim();
            if (t.startsWith("[") || t.contains("Policy") || t.contains("qualifier") || t.contains("Explicit")
                    || t.contains("Inhibit") || t.contains("Constraints") || t.contains("example")) {
                System.out.println("ca| " + t);
            }
        }
        for (String line : leaf.toString().split("\n")) {
            String t = line.trim();
            if (t.contains("Extension") || t.contains("example") || t.contains("Usage") || t.contains("CertType")) {
                System.out.println("leaf| " + t);
            }
        }
        for (String line : crl.toString().split("\n")) {
            String t = line.trim();
            if (t.contains("Reason") || t.contains("CRL Number") || t.contains("Extension")) {
                System.out.println("crl| " + t);
            }
        }
    }
}
