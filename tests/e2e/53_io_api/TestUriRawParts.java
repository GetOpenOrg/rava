import java.net.URI;
import java.net.IDN;

/**
 * URI raw 形态与国际化域名（方法级实测：URI.getRawQuery/getFragment/isOpaque/
 * toASCIIString 各 5 jar、IDN.toASCII/toUnicode 5-7 jar，此前零覆盖）：
 * 编码态与解码态的区别、opaque URI（mailto:）、IDN 转写。
 */
public class TestUriRawParts {

    public static void main(String[] args) throws Exception {
        URI u = URI.create("https://example.com/p%20ath?q=a%20b#f%20g");

        // 编码态（raw）vs 解码态
        System.out.println("raw-path=" + u.getRawPath());
        System.out.println("path=" + u.getPath());
        System.out.println("raw-query=" + u.getRawQuery());
        System.out.println("query=" + u.getQuery());
        System.out.println("raw-frag=" + u.getRawFragment());
        System.out.println("frag=" + u.getFragment());

        // isOpaque：非层级 URI（scheme:opaque，无 path 起点 /）
        URI mailto = URI.create("mailto:someone@example.com");
        System.out.println("opaque=" + mailto.isOpaque() + " hier=" + !u.isOpaque());
        System.out.println("mailto-scheme=" + mailto.getScheme()
                + " ssp=" + mailto.getSchemeSpecificPart());

        // toASCIIString：全 ASCII 形态（非 ASCII 输入时转写）
        URI ascii = URI.create("https://example.com/%E4%B8%AD");
        System.out.println("ascii-str=" + ascii.toASCIIString());

        // resolve 相对与绝对化
        URI base = URI.create("https://h/base/x");
        System.out.println("resolve=" + base.resolve("y"));
        System.out.println("relativize=" + base.relativize(URI.create("https://h/base/y")));

        // normalize
        URI messy = URI.create("https://h/a/../b/./c");
        System.out.println("normalize=" + messy.normalize());

        // 多参构造形态
        URI built = new URI("https", "example.com", "/path", "k=v", "ref");
        System.out.println("built=" + built + " query=" + built.getQuery());

        // IDN：国际化域名转写（punycode）
        System.out.println("idn-ascii=" + IDN.toASCII("bücher.example"));
        System.out.println("idn-back=" + IDN.toUnicode("xn--bcher-kva.example"));
        try {
            IDN.toASCII("bad..name..");
        } catch (IllegalArgumentException e) {
            System.out.println("idn-ex=" + e.getClass().getSimpleName());
        }
    }
}
