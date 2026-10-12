
import java.util.List;

import kotlin.collections.CollectionsKt;
import kotlin.text.StringsKt;

/**
 * kotlin-stdlib pilot E5：非 javac 字节码的第一块试金石（E 批中期件）。
 * Java 侧调用 Kotlin 编译产物：扩展函数 → XxxKt 静态方法、Unit 返回 null。
 */
public class KotlinStdlibPilotMain {

    static void check(String name, Object expected, Object actual) {
        boolean ok = String.valueOf(expected).equals(String.valueOf(actual));
        System.out.println((ok ? "PASS " : "FAIL ") + name
                + (ok ? "" : " :: expected=<" + expected + "> actual=<" + actual + ">"));
    }

    public static void main(String[] args) {
        List<Integer> list = List.of(3, 1, 2);

        // 集合扩展函数（CollectionsKt 静态方法，Unit 需显式 return null）
        check("max-or-null", 3, CollectionsKt.maxOrNull(list));
        check("sorted", "[1, 2, 3]", CollectionsKt.sorted(list).toString());
        check("reversed", "[2, 1, 3]", CollectionsKt.reversed(list).toString());
        check("first", 3, CollectionsKt.first(list));
        check("contains", true, CollectionsKt.contains(list, 2));

        // joinToString：默认参数以完整签名暴露（互操作痛点的对照面）
        String joined = CollectionsKt.joinToString(list, ", ", "[", "]", -1, "...", null);
        check("join-tostring", "[3, 1, 2]", joined);

        // 字符串扩展
        check("repeat", "ababab", StringsKt.repeat("ab", 3));
        check("is-blank", true, StringsKt.isBlank("   "));
        check("lines", "[a, b]", StringsKt.lines("a\nb").toString());

        // Kotlin 集合的互操作面（CollectionsKt.toList 返回 kotlin List，java 侧即 java.util.List）
        java.util.List<Integer> kList = CollectionsKt.toList(list);
        check("kotlin-list-size", 3, kList.size());

        // Pair / Triple 值类型（Kotlin data class 的字节码形态）
        kotlin.Pair<String, Integer> pair = new kotlin.Pair<>("k", 7);
        check("pair", "(k, 7)", pair.toString());
        check("pair-first", "k", pair.getFirst());
        check("pair-second", 7, pair.getSecond());
        System.out.println("DONE KotlinStdlibPilotMain");
    }
}
