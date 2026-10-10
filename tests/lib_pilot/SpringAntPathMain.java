import java.util.ArrayList;
import java.util.Arrays;
import java.util.Comparator;
import java.util.List;
import java.util.Map;
import java.util.TreeMap;

import org.springframework.util.AntPathMatcher;

/**
 * spring-core 切片 sc2（矩阵 #11）：AntPathMatcher。
 *
 * 覆盖面：match / matchStart（? * ** 与 {var} / {var:regex} 模板，经 AntPathStringMatcher 的
 * java.util.regex 编译）、isPattern、extractPathWithinPattern、extractUriTemplateVariables、
 * combine、getPatternComparator 排序、自定义分隔符与大小写不敏感、模式缓存。
 */
public class SpringAntPathMain {

    static final AntPathMatcher MATCHER = new AntPathMatcher();

    static void match(String pattern, String path) {
        System.out.println("match " + pattern + " " + path + " -> " + MATCHER.match(pattern, path));
    }

    public static void main(String[] args) {
        // ── 基本通配 ──
        match("/test", "/test");
        match("test", "/test");
        match("/t?st", "/test");
        match("/t?st", "/tst");
        match("*.jsp", "index.jsp");
        match("/*.jsp", "/a/index.jsp");
        match("/bla/**/bla", "/bla/testing/testing/bla");
        match("/bla/**/bla", "/bla/bla");
        match("/**", "/x/y/z");
        match("/**/*.html", "/a/b/c.html");
        match("/**/*.html", "/a/b/c.htm");
        match("/x/**/y/*", "/x/a/b/y/z");
        match("/{name}.{ext}", "/report.pdf");
        match("/users/{id:\\d+}", "/users/42");
        match("/users/{id:\\d+}", "/users/abc");
        match("/*bla*/**/bla/*", "/XXXblaXXXX/testing/testing/bla/testing");

        // ── matchStart / isPattern ──
        System.out.println("matchStart /x/**/y /x/a -> " + MATCHER.matchStart("/x/**/y", "/x/a"));
        System.out.println("matchStart /x/y /x/z -> " + MATCHER.matchStart("/x/y", "/x/z"));
        for (String p : Arrays.asList("/a/b", "/a/*", "/a/{id}", "/a/?", "/a/**")) {
            System.out.println("isPattern " + p + " -> " + MATCHER.isPattern(p));
        }

        // ── extractPathWithinPattern ──
        String[][] within = {
                {"/docs/cvs/commit.html", "/docs/cvs/commit.html"},
                {"/docs/*", "/docs/cvs/commit"},
                {"/docs/cvs/*.html", "/docs/cvs/commit.html"},
                {"/docs/**", "/docs/cvs/commit"},
                {"/*.html", "/commit.html"},
                {"/docs/**/*.html", "/docs/a/b/commit.html"},
        };
        for (String[] w : within) {
            System.out.println("within " + w[0] + " " + w[1] + " -> '"
                    + MATCHER.extractPathWithinPattern(w[0], w[1]) + "'");
        }

        // ── extractUriTemplateVariables ──
        String[][] vars = {
                {"/hotels/{hotel}", "/hotels/1"},
                {"/hotels/{hotel}/bookings/{booking}", "/hotels/1/bookings/2"},
                {"/{page}.{ext}", "/report.pdf"},
                {"/users/{id:\\d+}-{slug}", "/users/42-hello"},
                {"/**/{name}.html", "/a/b/index.html"},
        };
        for (String[] v : vars) {
            Map<String, String> m = new TreeMap<>(MATCHER.extractUriTemplateVariables(v[0], v[1]));
            System.out.println("vars " + v[0] + " " + v[1] + " -> " + m);
        }
        try {
            MATCHER.extractUriTemplateVariables("/users/{id}", "/groups/1");
            System.out.println("vars mismatch -> no exception");
        } catch (IllegalStateException e) {
            System.out.println("vars mismatch -> " + e.getMessage());
        }

        // ── combine ──
        String[][] combos = {
                {"/hotels", "/bookings"},
                {"/hotels/*", "booking"},
                {"/hotels/**", "booking"},
                {"/hotels/**", "/hotels/booking"},
                {"/*.html", "/hotels"},
                {"/hotels", "/*.html"},
                {"/{foo}", "/bar"},
                {"", "/hotels"},
                {"/hotels/*", "{hotel}"},
        };
        for (String[] c : combos) {
            System.out.println("combine " + c[0] + " + " + c[1] + " -> " + MATCHER.combine(c[0], c[1]));
        }
        try {
            MATCHER.combine("/*.html", "/*.txt");
        } catch (IllegalArgumentException e) {
            System.out.println("combine conflict -> " + e.getMessage());
        }

        // ── 模式比较器排序 ──
        List<String> patterns = new ArrayList<>(Arrays.asList(
                "/hotels/**", "/hotels/{hotel}", "/hotels/new", "/**", "/hotels/*", "/hotels/{hotel}/*"));
        Comparator<String> cmp = MATCHER.getPatternComparator("/hotels/new");
        patterns.sort(cmp);
        System.out.println("sorted " + patterns);

        // ── 自定义分隔符 / 大小写 ──
        AntPathMatcher dots = new AntPathMatcher(".");
        System.out.println("dots com.example.*.Service com.example.user.Service -> "
                + dots.match("com.example.*.Service", "com.example.user.Service"));
        System.out.println("dots com.**.Service com.a.b.c.Service -> "
                + dots.match("com.**.Service", "com.a.b.c.Service"));
        AntPathMatcher insensitive = new AntPathMatcher();
        insensitive.setCaseSensitive(false);
        System.out.println("insensitive /Group/{groupName}/Members /group/sales/members -> "
                + insensitive.match("/Group/{groupName}/Members", "/group/sales/members"));
        insensitive.setTrimTokens(true);
        System.out.println("trim /a/b / a / b -> " + insensitive.match("/a/b", "/ a / b"));

        // 大量不同模式：越过缓存阈值（65536）前的正常路径，重复匹配命中缓存
        int hits = 0;
        for (int i = 0; i < 200; i++) {
            if (MATCHER.match("/items/{id}/part" + (i % 10), "/items/" + i + "/part" + (i % 10))) {
                hits++;
            }
        }
        System.out.println("cache loop hits=" + hits);
        System.out.println("done");
    }
}
