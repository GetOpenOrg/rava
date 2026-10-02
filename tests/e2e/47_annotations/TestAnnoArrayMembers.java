import java.lang.annotation.Retention;
import java.lang.annotation.RetentionPolicy;
import java.util.Arrays;

/**
 * 注解数组成员边界：三层嵌套注解（每层取值后再下钻）、各层空数组、基本类型数组（8 种）、
 * 字符串数组（含转义）、枚举数组、Class 数组（用户类 / JDK 类 / 基本类型 / 数组类型），
 * 以及只出现在注解属性体里的嵌套注解类型。各注解只有单个成员，toString 文本与成员迭代序无关。
 */
public class TestAnnoArrayMembers {
    enum Color { RED, GREEN, BLUE }

    @Retention(RetentionPolicy.RUNTIME)
    @interface L3 { int[] value(); }

    @Retention(RetentionPolicy.RUNTIME)
    @interface L2 { L3[] value(); }

    @Retention(RetentionPolicy.RUNTIME)
    @interface L1 { L2[] value(); }

    @Retention(RetentionPolicy.RUNTIME)
    @interface Ints { int[] value(); }

    @Retention(RetentionPolicy.RUNTIME)
    @interface Longs { long[] value(); }

    @Retention(RetentionPolicy.RUNTIME)
    @interface Doubles { double[] value(); }

    @Retention(RetentionPolicy.RUNTIME)
    @interface Floats { float[] value(); }

    @Retention(RetentionPolicy.RUNTIME)
    @interface Bools { boolean[] value(); }

    @Retention(RetentionPolicy.RUNTIME)
    @interface Chars { char[] value(); }

    @Retention(RetentionPolicy.RUNTIME)
    @interface Bytes { byte[] value(); }

    @Retention(RetentionPolicy.RUNTIME)
    @interface Shorts { short[] value(); }

    @Retention(RetentionPolicy.RUNTIME)
    @interface Strs { String[] value(); }

    @Retention(RetentionPolicy.RUNTIME)
    @interface Colors { Color[] value(); }

    @Retention(RetentionPolicy.RUNTIME)
    @interface Types { Class<?>[] value(); }

    static class Marker {
    }

    @L1({@L2({@L3({1, 2}), @L3({})}), @L2({}), @L2(@L3(9))})
    @Ints({3, -4, Integer.MAX_VALUE})
    @Longs({5L, Long.MIN_VALUE})
    @Doubles({1.5, -0.0, Double.NaN})
    @Floats({2.5f, Float.POSITIVE_INFINITY})
    @Bools({true, false})
    @Chars({'a', '\'', '\n'})
    @Bytes({(byte) 1, (byte) -128})
    @Shorts({(short) 7, (short) -32768})
    @Strs({"plain", "q\"t", "tab\t", ""})
    @Colors({Color.BLUE, Color.RED})
    @Types({Marker.class, String.class, int.class, String[].class, Color.class})
    static class Full {
    }

    @Ints({})
    @Strs({})
    @Colors({})
    @Types({})
    @L1({})
    static class Empty {
    }

    public static void main(String[] args) {
        L1 l1 = Full.class.getAnnotation(L1.class);
        System.out.println(l1);
        L2[] l2s = l1.value();
        System.out.println("l2 count=" + l2s.length);
        for (L2 l2 : l2s) {
            System.out.println("  " + l2 + " l3 count=" + l2.value().length);
            for (L3 l3 : l2.value()) {
                int sum = 0;
                for (int v : l3.value()) {
                    sum += v;
                }
                System.out.println("    " + l3 + " sum=" + sum);
            }
        }
        System.out.println(Full.class.getAnnotation(Ints.class));
        System.out.println(Full.class.getAnnotation(Longs.class));
        System.out.println(Full.class.getAnnotation(Doubles.class));
        System.out.println(Full.class.getAnnotation(Floats.class));
        System.out.println(Full.class.getAnnotation(Bools.class));
        System.out.println(Full.class.getAnnotation(Chars.class));
        System.out.println(Full.class.getAnnotation(Bytes.class));
        System.out.println(Full.class.getAnnotation(Shorts.class));
        System.out.println(Full.class.getAnnotation(Strs.class));
        Colors colors = Full.class.getAnnotation(Colors.class);
        System.out.println(colors + " first=" + colors.value()[0].ordinal());
        Types types = Full.class.getAnnotation(Types.class);
        System.out.println(types);
        System.out.println("types=" + Arrays.toString(types.value()));
        System.out.println(Empty.class.getAnnotation(Ints.class));
        System.out.println(Empty.class.getAnnotation(Strs.class));
        System.out.println(Empty.class.getAnnotation(Colors.class));
        System.out.println(Empty.class.getAnnotation(Types.class));
        System.out.println(Empty.class.getAnnotation(L1.class));
        System.out.println("equal=" + l1.equals(Full.class.getAnnotation(L1.class))
                + " differ=" + l1.equals(Empty.class.getAnnotation(L1.class)));
    }
}
