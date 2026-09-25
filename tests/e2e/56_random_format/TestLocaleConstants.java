import java.util.Locale;

/**
 * Locale 常量全集（BaseLocale 常量表下标随 JDK 版本变化的回归哨兵）：
 * 每个 Locale 常量的 toString / language / country，以及同值 Locale.of 取回的
 * 等值性、按常量格式化的千分位 / 小数点。
 */
public class TestLocaleConstants {
    static void show(String name, Locale l) {
        System.out.println(name + "=" + l + "|" + l.getLanguage() + "|" + l.getCountry()
                + "|" + l.equals(Locale.of(l.getLanguage(), l.getCountry())));
    }

    public static void main(String[] args) {
        show("ROOT", Locale.ROOT);
        show("ENGLISH", Locale.ENGLISH);
        show("FRENCH", Locale.FRENCH);
        show("GERMAN", Locale.GERMAN);
        show("ITALIAN", Locale.ITALIAN);
        show("JAPANESE", Locale.JAPANESE);
        show("KOREAN", Locale.KOREAN);
        show("CHINESE", Locale.CHINESE);
        show("SIMPLIFIED_CHINESE", Locale.SIMPLIFIED_CHINESE);
        show("TRADITIONAL_CHINESE", Locale.TRADITIONAL_CHINESE);
        show("FRANCE", Locale.FRANCE);
        show("GERMANY", Locale.GERMANY);
        show("ITALY", Locale.ITALY);
        show("JAPAN", Locale.JAPAN);
        show("KOREA", Locale.KOREA);
        show("UK", Locale.UK);
        show("US", Locale.US);
        show("CANADA", Locale.CANADA);
        show("CANADA_FRENCH", Locale.CANADA_FRENCH);
        // 格式化随 Locale 常量走（分组 / 小数分隔符）
        for (Locale l : new Locale[]{Locale.US, Locale.GERMANY, Locale.FRANCE, Locale.UK, Locale.ITALY}) {
            System.out.println("fmt." + l + "=" + String.format(l, "%,.2f", 1234567.891));
        }
    }
}
