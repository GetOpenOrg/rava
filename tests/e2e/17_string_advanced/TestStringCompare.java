public class TestStringCompare {

    public static void main(String[] args) {
        String a = "Hello";
        String b = "world";
        String lowerA = "hello";
        String paddedA = "  Hello  ";

        // equals / equalsIgnoreCase
        System.out.println("equals=" + a.equals(lowerA));
        System.out.println("equalsIgnoreCase=" + a.equalsIgnoreCase(lowerA));
        System.out.println("equals self=" + a.equals("Hello"));

        // compareTo / compareToIgnoreCase（返回差值）
        System.out.println("compareTo=" + a.compareTo(b));
        System.out.println("compareTo lower=" + a.compareTo(lowerA));
        System.out.println("compareToIgnoreCase=" + a.compareToIgnoreCase(lowerA));
        System.out.println("cmp aaa vs aab=" + "aaa".compareTo("aab"));
        System.out.println("cmp a vs ab=" + "a".compareTo("ab"));

        // startsWith / endsWith / contains
        System.out.println("startsWith He=" + a.startsWith("He"));
        System.out.println("startsWith offset=" + a.startsWith("llo", 2));
        System.out.println("endsWith lo=" + a.endsWith("lo"));
        System.out.println("contains ell=" + a.contains("ell"));
        System.out.println("contains zz=" + a.contains("zz"));

        // regionMatches（大小写敏感/不敏感）
        String ref = "HelloWorld";
        System.out.println("region(0,Hello)=" + ref.regionMatches(0, "Hello", 0, 5));
        System.out.println("region ignore case=" + ref.regionMatches(true, 5, "WORLD", 0, 5));

        // isEmpty / isBlank / length
        System.out.println("empty isEmpty=" + "".isEmpty() + " length=" + "".length());
        System.out.println("blank isEmpty=" + "   ".isEmpty() + " isBlank=" + "   ".isBlank());
        System.out.println("tab isBlank=" + "\t\n".isBlank());
        System.out.println("length=" + a.length());

        // trim / strip 系列（Java 11+）
        System.out.println("trim=[" + paddedA.trim() + "]");
        System.out.println("strip=[" + paddedA.strip() + "]");
        System.out.println("stripLeading=[" + paddedA.stripLeading() + "]");
        System.out.println("stripTrailing=[" + paddedA.stripTrailing() + "]");

        // 大小写转换
        System.out.println("toUpperCase=" + lowerA.toUpperCase());
        System.out.println("toLowerCase=" + a.toLowerCase());

        // repeat（Java 11+）
        System.out.println("repeat=" + "ab".repeat(3));

        // contentEquals 与 StringBuilder 比较
        StringBuilder sb = new StringBuilder("Hello");
        System.out.println("contentEquals sb=" + a.contentEquals(sb));

        // 字面量 intern 与 new String 的差异
        String lit = "abc";
        String heap = new String("abc");
        System.out.println("lit==heap=" + (lit == heap));
        System.out.println("lit.equals(heap)=" + lit.equals(heap));
        System.out.println("interned==lit=" + (heap.intern() == lit));

        System.out.println("done");
    }
}
