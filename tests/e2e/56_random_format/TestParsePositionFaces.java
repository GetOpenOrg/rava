import java.text.NumberFormat;
import java.text.ParsePosition;
import java.text.SimpleDateFormat;
import java.util.Locale;
import java.util.TimeZone;

/**
 * ParsePosition 通道（方法级实测：setIndex 7 / getErrorIndex 6，此前零覆盖）：
 * Format.parse(String, ParsePosition) 的部分解析、失败 errorIndex、索引推进。
 */
public class TestParsePositionFaces {

    public static void main(String[] args) throws Exception {
        ParsePosition pp = new ParsePosition(0);
        System.out.println("initial-index=" + pp.getIndex()
                + " initial-error=" + pp.getErrorIndex());
        pp.setIndex(2);
        System.out.println("after-set=" + pp.getIndex());

        // 数字部分解析：前缀数字后停止（不抛异常，索引停在 5）
        NumberFormat nf = NumberFormat.getInstance(Locale.US);
        pp.setIndex(0);
        Number n = nf.parse("123abc", pp);
        System.out.println("partial=" + n + " stopped-at=" + pp.getIndex()
                + " no-error=" + (pp.getErrorIndex() == -1));

        // 从中间起解析
        pp.setIndex(4);
        Number n2 = nf.parse("xx 42 yy", pp);
        System.out.println("from-offset=" + n2 + " index=" + pp.getIndex());

        // 失败：null 结果 + errorIndex 指位
        pp.setIndex(0);
        Number bad = nf.parse("not-a-num", pp);
        System.out.println("fail-null=" + (bad == null) + " error-at=" + pp.getErrorIndex());

        // 日期部分解析：长度不足失败
        SimpleDateFormat df = new SimpleDateFormat("yyyy-MM-dd", Locale.ROOT);
        df.setTimeZone(TimeZone.getTimeZone("GMT"));
        ParsePosition dp = new ParsePosition(0);
        var d1 = df.parse("2026-10-03tail", dp);
        System.out.println("date-partial=" + (d1 != null) + " stopped=" + dp.getIndex());
        ParsePosition dp2 = new ParsePosition(0);
        var d2 = df.parse("2026/10/03", dp2);
        System.out.println("date-fail=" + (d2 == null) + " err=" + dp2.getErrorIndex());

        // toString 形态（含两索引）
        System.out.println("pp-tostring=" + new ParsePosition(3).toString().contains("index"));
    }
}
