import java.text.BreakIterator;
import java.text.Collator;
import java.text.DateFormatSymbols;
import java.text.DecimalFormatSymbols;
import java.time.DayOfWeek;
import java.time.LocalDate;
import java.time.Month;
import java.time.format.DateTimeFormatter;
import java.time.format.FormatStyle;
import java.time.format.TextStyle;
import java.util.Arrays;
import java.util.Calendar;
import java.util.Currency;
import java.util.Locale;
import java.util.TimeZone;

/**
 * 本地化资源束族：LocaleData 的每个访问器按适配器类型取 CLDR 包或 JRE（FALLBACK）包下的束，
 * 束经类名反射装载。覆盖 FormatData（两族）、JavaTimeSupplementary、CurrencyNames、TimeZoneNames、
 * LocaleNames、CalendarData、BreakIteratorInfo / BreakIteratorResources、CollationData。
 */
public class TestLocaleBundleFamilies {
    public static void main(String[] args) {
        Locale en = Locale.US;
        LocalDate d = LocalDate.of(2024, 3, 9);
        System.out.println("pattern = " + DateTimeFormatter.ofPattern("EEEE, MMMM d, uuuu", en).format(d));
        System.out.println("localized = " + DateTimeFormatter.ofLocalizedDate(FormatStyle.FULL).withLocale(en).format(d));
        System.out.println("month = " + Month.MARCH.getDisplayName(TextStyle.SHORT, en) + " / " + Month.MARCH.getDisplayName(TextStyle.NARROW, en));
        System.out.println("dow = " + DayOfWeek.SATURDAY.getDisplayName(TextStyle.FULL, en));

        DateFormatSymbols dfs = DateFormatSymbols.getInstance(en);
        System.out.println("symbols = " + dfs.getMonths()[0] + " " + dfs.getShortWeekdays()[1] + " " + Arrays.toString(dfs.getEras()) + " " + Arrays.toString(dfs.getAmPmStrings()));
        DecimalFormatSymbols dec = DecimalFormatSymbols.getInstance(en);
        System.out.println("decimal = " + dec.getDecimalSeparator() + " " + dec.getGroupingSeparator() + " " + dec.getPercent());

        System.out.println("currency = " + Currency.getInstance("USD").getSymbol(en) + " " + Currency.getInstance("EUR").getDisplayName(en));
        System.out.println("timezone = " + TimeZone.getTimeZone("UTC").getDisplayName(false, TimeZone.SHORT, en));
        System.out.println("language = " + Locale.FRANCE.getDisplayLanguage(en) + " / " + Locale.FRANCE.getDisplayCountry(en));

        Calendar cal = Calendar.getInstance(en);
        System.out.println("calendar = first " + cal.getFirstDayOfWeek() + " minimal " + cal.getMinimalDaysInFirstWeek());

        BreakIterator words = BreakIterator.getWordInstance(en);
        words.setText("Hello brave world");
        int n = 0;
        for (int end = words.next(); end != BreakIterator.DONE; end = words.next()) n++;
        System.out.println("break segments = " + n);

        Collator c = Collator.getInstance(en);
        String[] s = {"banana", "Apple", "cherry", "apple"};
        Arrays.sort(s, c);
        System.out.println("collated = " + Arrays.toString(s));
    }
}
