import java.util.List;
import java.util.Locale;

// BCP 47 语言标签：Locale.forLanguageTag / toLanguageTag / Builder / lookup（sun/util/locale 族）
public class TestLocaleLanguageTag {
    static String show(Locale l) {
        return "[" + l.getLanguage() + "|" + l.getScript() + "|" + l.getCountry() + "|" + l.getVariant() + "] " + l;
    }

    public static void main(String[] args) {
        System.out.println(Locale.US.toLanguageTag());
        System.out.println(Locale.ROOT.toLanguageTag());
        System.out.println(Locale.CHINA.toLanguageTag());
        System.out.println(new Locale("en", "US", "POSIX").toLanguageTag());
        System.out.println(new Locale("iw", "IL").toLanguageTag());
        System.out.println(new Locale("ja", "JP", "JP").toLanguageTag());
        for (String tag : new String[]{"zh-Hant-TW", "en-US", "sr-Latn-RS", "de-CH-1996", "EN-gb",
                "x-private", "en-US-u-ca-japanese", "und", "i-klingon", "abc-def-ghijklmnop"}) {
            Locale l = Locale.forLanguageTag(tag);
            System.out.println(tag + " -> " + show(l) + " -> " + l.toLanguageTag());
        }
        Locale b = new Locale.Builder().setLanguage("fr").setRegion("CA").setScript("Latn")
                .setVariant("POSIX").build();
        System.out.println("builder " + show(b) + " " + b.toLanguageTag());
        Locale ext = new Locale.Builder().setLanguageTag("th-TH").setUnicodeLocaleKeyword("nu", "thai").build();
        System.out.println("ext " + ext.toLanguageTag() + " nu=" + ext.getUnicodeLocaleType("nu")
                + " keys=" + ext.getUnicodeLocaleKeys() + " hasExt=" + ext.hasExtensions());
        try {
            new Locale.Builder().setLanguage("toolonglanguage").build();
        } catch (Exception e) {
            System.out.println(e.getClass().getSimpleName() + ": " + e.getMessage());
        }
        List<Locale.LanguageRange> ranges = Locale.LanguageRange.parse("en-GB;q=0.9,fr;q=0.8,de");
        for (Locale.LanguageRange lr : ranges) {
            System.out.println("range " + lr.getRange() + " w=" + lr.getWeight());
        }
        List<Locale> cands = List.of(Locale.forLanguageTag("fr-FR"), Locale.forLanguageTag("en-US"),
                Locale.forLanguageTag("de-DE"));
        System.out.println("lookup=" + Locale.lookup(ranges, cands));
        System.out.println("filter=" + Locale.filter(ranges, cands));
        System.out.println("stripExt=" + ext.stripExtensions().toLanguageTag());
    }
}
