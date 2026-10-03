import java.text.BreakIterator;
import java.text.Normalizer;
import java.util.Locale;

// 模块资源推导边界：ICU 归一化数据（nfc.nrm / nfkc.nrm）由「目录前缀 + 名 + .nrm」拼接得出；
// 断句数据（Word / Line / Sentence / CharacterBreakIteratorData）为无扩展名资源名。
public class TestNormalizerResourceForms {
    static String hex(String s) {
        StringBuilder sb = new StringBuilder();
        for (char c : s.toCharArray()) sb.append(String.format("%04X ", (int) c));
        return sb.toString().trim();
    }

    static int count(BreakIterator it, String text) {
        it.setText(text);
        int n = 0;
        for (int e = it.next(); e != BreakIterator.DONE; e = it.next()) n++;
        return n;
    }

    public static void main(String[] args) {
        String composed = "é";
        String decomposed = "é";
        String ligature = "ﬁ";
        for (Normalizer.Form f : Normalizer.Form.values()) {
            System.out.println(f + ": " + hex(Normalizer.normalize(composed, f)) + " | "
                + hex(Normalizer.normalize(decomposed, f)) + " | " + hex(Normalizer.normalize(ligature, f))
                + " | normalized=" + Normalizer.isNormalized(decomposed, f));
        }
        String text = "Hello world. How are you? Fine, thanks!";
        System.out.println("words=" + count(BreakIterator.getWordInstance(Locale.US), text));
        System.out.println("lines=" + count(BreakIterator.getLineInstance(Locale.US), text));
        System.out.println("sentences=" + count(BreakIterator.getSentenceInstance(Locale.US), text));
        System.out.println("chars=" + count(BreakIterator.getCharacterInstance(Locale.US), "éa"));
    }
}
