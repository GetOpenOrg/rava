// FS-L11：非 Latin1（UTF16 编码）字符串的 hashCode 与 JVM 一致，决定 HashMap 迭代顺序。
import java.util.*;

public class TestStringHashUtf16 {
    public static void main(String[] args) {
        String[] ss = {"中文", "日本語テキスト", "Ωmega", "emoji😀", "ĀĒĪŌŪ", "mixedé中", "Ā", "￿z", "Latin1é"};
        for (String s : ss) System.out.println(s + " " + s.hashCode() + " len=" + s.length());
        Map<String, Integer> m = new HashMap<>();
        for (int i = 0; i < ss.length; i++) m.put(ss[i], i);
        System.out.println("order " + m.keySet());
        Set<String> hs = new HashSet<>();
        for (int i = 0; i < 40; i++) hs.add("键" + i);
        System.out.println("set " + hs);
        System.out.println("concat hash " + ("中" + "文").hashCode() + " sb " + new StringBuilder("中").append('文').toString().hashCode());
    }
}
