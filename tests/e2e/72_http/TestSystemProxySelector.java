import java.net.InetSocketAddress;
import java.net.Proxy;
import java.net.ProxySelector;
import java.net.URI;
import java.util.List;

/**
 * DefaultProxySelector 的系统代理 native 边界（java.net.useSystemProxies=true）：
 * 类初始化时 init() 探测平台代理设施（macOS CFNetwork / Linux GIO、GConf），
 * 未设协议代理属性时 select 经 getSystemProxies 取系统配置；无系统代理时为 DIRECT。
 * 系统代理配置随机器而异，系统路径只打印与环境无关的结构判定（非空、每项为 DIRECT 或
 * 未解析的代理地址且端口合法）；协议代理属性优先于系统代理，属性路径打印确切结果。
 */
public class TestSystemProxySelector {

    /** 启动时平台可能把系统代理写进 *.proxyHost 等属性（macOS），先清掉以走 getSystemProxies。 */
    private static final String[] PROXY_PROPS = {
        "http.proxyHost", "http.proxyPort", "https.proxyHost", "https.proxyPort",
        "ftp.proxyHost", "ftp.proxyPort", "socksProxyHost", "socksProxyPort",
        "proxyHost", "proxyPort", "http.nonProxyHosts", "ftp.nonProxyHosts", "socksNonProxyHosts",
    };

    private static void checkSystem(ProxySelector ps, String uri) {
        List<Proxy> list = ps.select(URI.create(uri));
        boolean ok = !list.isEmpty();
        for (Proxy p : list) {
            if (p.type() == Proxy.Type.DIRECT) {
                ok &= p.address() == null;
            } else {
                InetSocketAddress sa = (InetSocketAddress) p.address();
                ok &= sa.isUnresolved() && sa.getPort() > 0 && sa.getPort() <= 65535
                        && !sa.getHostString().isEmpty();
            }
        }
        System.out.println(uri + " system-ok=" + ok);
    }

    private static void show(ProxySelector ps, String uri) {
        List<Proxy> list = ps.select(URI.create(uri));
        StringBuilder sb = new StringBuilder();
        for (Proxy p : list) {
            if (sb.length() > 0) sb.append(", ");
            sb.append(p.type());
            if (p.type() != Proxy.Type.DIRECT) {
                sb.append(" @ ").append(p.address());
            }
        }
        System.out.println(uri + " -> [" + sb + "] size=" + list.size());
    }

    public static void main(String[] args) {
        for (String k : PROXY_PROPS) {
            System.clearProperty(k);
        }
        System.setProperty("java.net.useSystemProxies", "true");
        ProxySelector ps = ProxySelector.getDefault();
        System.out.println("selector=" + ps.getClass().getName());

        // 系统代理路径（getSystemProxies）
        checkSystem(ps, "http://example.invalid/");
        checkSystem(ps, "https://example.invalid/path");
        checkSystem(ps, "ftp://example.invalid/");
        checkSystem(ps, "socket://example.invalid:25");

        // 协议属性优先于系统代理
        System.setProperty("https.proxyHost", "proxy.example.invalid");
        System.setProperty("https.proxyPort", "8443");
        show(ps, "https://example.invalid/");
        System.setProperty("socksProxyHost", "socks.example.invalid");
        System.setProperty("socksProxyPort", "1081");
        show(ps, "socket://example.invalid:25");

        try {
            ps.select(URI.create("noscheme"));
        } catch (IllegalArgumentException e) {
            System.out.println("no-scheme=" + e.getMessage());
        }
    }
}
