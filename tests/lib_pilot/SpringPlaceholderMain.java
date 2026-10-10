import java.util.Properties;

import org.springframework.util.PropertyPlaceholderHelper;
import org.springframework.util.StringUtils;

/**
 * spring-core 切片 sc3（矩阵 #11）：PropertyPlaceholderHelper（6.2 起委托 PlaceholderParser）。
 *
 * 覆盖面：${} 替换、默认值分隔符、嵌套占位符、键内占位符、转义字符、ignoreUnresolvable
 * 两种模式、循环引用检测、自定义前后缀、PlaceholderResolver 回调；附 StringUtils 常用面。
 */
public class SpringPlaceholderMain {

    static void run(String label, PropertyPlaceholderHelper helper, String text, Properties props) {
        try {
            System.out.println(label + " -> " + helper.replacePlaceholders(text, props));
        } catch (IllegalArgumentException e) {
            System.out.println(label + " -> " + e.getClass().getSimpleName() + ": " + e.getMessage());
        }
    }

    public static void main(String[] args) {
        Properties props = new Properties();
        props.setProperty("app.name", "rava");
        props.setProperty("app.version", "1.0");
        props.setProperty("greeting", "hello ${app.name}");
        props.setProperty("env", "prod");
        props.setProperty("db.prod.url", "jdbc:h2:mem:prod");
        props.setProperty("loop.a", "${loop.b}");
        props.setProperty("loop.b", "${loop.a}");
        props.setProperty("empty", "");

        PropertyPlaceholderHelper strict = new PropertyPlaceholderHelper("${", "}", ":", '\\', false);
        PropertyPlaceholderHelper lenient = new PropertyPlaceholderHelper("${", "}", ":", '\\', true);
        PropertyPlaceholderHelper simple = new PropertyPlaceholderHelper("${", "}");

        run("plain", strict, "no placeholders", props);
        run("single", strict, "${app.name}", props);
        run("multi", strict, "${app.name}-${app.version}", props);
        run("nested-value", strict, "say: ${greeting}", props);
        run("nested-key", strict, "${db.${env}.url}", props);
        run("default-used", strict, "${missing:fallback}", props);
        run("default-unused", strict, "${app.name:fallback}", props);
        run("default-placeholder", strict, "${missing:${app.name}}", props);
        run("default-empty", strict, "[${missing:}]", props);
        run("empty-value", strict, "[${empty}]", props);
        run("escaped", strict, "\\${app.name} = ${app.name}", props);
        run("unresolved-strict", strict, "x ${nope} y", props);
        run("unresolved-lenient", lenient, "x ${nope} y ${app.name}", props);
        run("circular", strict, "${loop.a}", props);
        run("simple-no-default", simple, "${app.name:ignored}", props);

        PropertyPlaceholderHelper custom = new PropertyPlaceholderHelper("#{", "}#", "?", null, false);
        run("custom-syntax", custom, "#{app.name}# v#{app.version}# #{x?dflt}#", props);

        String resolved = strict.replacePlaceholders("${a}+${b}=${c:3}",
                name -> name.equals("a") ? "1" : name.equals("b") ? "2" : null);
        System.out.println("resolver -> " + resolved);

        // ── StringUtils 常用面 ──
        System.out.println("hasText -> " + StringUtils.hasText("  ") + " " + StringUtils.hasText(" x "));
        System.out.println("tokenize -> " + String.join("|",
                StringUtils.tokenizeToStringArray("a, b ,,c ;d", ",;")));
        System.out.println("commaDelimited -> " + StringUtils.commaDelimitedListToSet("x,y,x,z"));
        System.out.println("cleanPath -> " + StringUtils.cleanPath("/a/b/../c/./d//e"));
        System.out.println("capitalize -> " + StringUtils.capitalize("spring") + " "
                + StringUtils.uncapitalize("Spring"));
        System.out.println("filenameExt -> " + StringUtils.getFilenameExtension("dir/app.yml") + " "
                + StringUtils.stripFilenameExtension("dir/app.yml"));
        System.out.println("replace -> " + StringUtils.replace("a.b.c", ".", "::"));
        System.out.println("countOccurrences -> " + StringUtils.countOccurrencesOf("banana", "an"));
        System.out.println("delete -> " + StringUtils.deleteAny("h-e-l_l-o", "-_"));
        System.out.println("quote -> " + StringUtils.quote("x"));
        System.out.println("done");
    }
}
