import java.lang.invoke.MethodHandle;
import java.lang.invoke.MethodHandles;
import java.lang.invoke.MethodType;
import java.util.Arrays;
import java.util.List;

/**
 * MethodHandle 组合子（MH-native #40）：bindTo、insertArguments、dropArguments、
 * filterReturnValue、filterArguments、asType、constant、identity、guardWithTest、
 * asSpreader、asCollector、变参句柄（asList）、invokeWithArguments（数组与 List 形态）、
 * 组合链上的异常穿透。
 */
public class TestMethodHandleCombinators {
    static String tag(String a, int n) { return a + "#" + n; }
    static boolean isPos(int x) { return x > 0; }
    static int twice(int x) { return x * 2; }
    static String join(String... parts) { return String.join("-", parts); }
    static int fail(int x) { throw new ArithmeticException("fail " + x); }

    int base = 100;
    int add(int d) { return base + d; }

    public static void main(String[] args) throws Throwable {
        MethodHandles.Lookup lookup = MethodHandles.lookup();
        MethodHandle tag = lookup.findStatic(TestMethodHandleCombinators.class, "tag",
                MethodType.methodType(String.class, String.class, int.class));

        MethodHandle add = lookup.findVirtual(TestMethodHandleCombinators.class, "add",
                MethodType.methodType(int.class, int.class));
        MethodHandle bound = add.bindTo(new TestMethodHandleCombinators());
        System.out.println("bindTo " + (int) bound.invokeExact(5) + " " + bound.type());

        MethodHandle ins = MethodHandles.insertArguments(tag, 1, 9);
        System.out.println("insert " + (String) ins.invokeExact("c") + " " + ins.type());

        MethodHandle drop = MethodHandles.dropArguments(tag, 0, Object.class, long.class);
        System.out.println("drop " + (String) drop.invoke(new Object(), 1L, "d", 1) + " " + drop.type());

        MethodHandle len = lookup.findVirtual(String.class, "length", MethodType.methodType(int.class));
        MethodHandle filtRet = MethodHandles.filterReturnValue(tag, len);
        System.out.println("filterReturn " + (int) filtRet.invokeExact("eeee", 12));

        MethodHandle twice = lookup.findStatic(TestMethodHandleCombinators.class, "twice",
                MethodType.methodType(int.class, int.class));
        MethodHandle filtArg = MethodHandles.filterArguments(tag, 1, twice);
        System.out.println("filterArgs " + (String) filtArg.invokeExact("f", 21));

        MethodHandle asT = tag.asType(MethodType.methodType(Object.class, Object.class, Integer.class));
        System.out.println("asType " + asT.invoke("g", 5) + " " + asT.type());

        System.out.println("constant " + MethodHandles.constant(String.class, "K").invoke());
        System.out.println("identity " + (int) MethodHandles.identity(int.class).invokeExact(7));

        MethodHandle test = lookup.findStatic(TestMethodHandleCombinators.class, "isPos",
                MethodType.methodType(boolean.class, int.class));
        MethodHandle gwt = MethodHandles.guardWithTest(test,
                MethodHandles.dropArguments(MethodHandles.constant(String.class, "pos"), 0, int.class),
                MethodHandles.dropArguments(MethodHandles.constant(String.class, "neg"), 0, int.class));
        System.out.println("guard " + gwt.invoke(1) + " " + gwt.invoke(-1));

        MethodHandle spread = tag.asSpreader(Object[].class, 2);
        System.out.println("spreader " + spread.invoke(new Object[] {"s", 2}));
        MethodHandle joinH = lookup.findStatic(TestMethodHandleCombinators.class, "join",
                MethodType.methodType(String.class, String[].class));
        System.out.println("varargs " + joinH.isVarargsCollector() + " " + joinH.invoke("a", "b", "c"));
        MethodHandle collect = joinH.asFixedArity().asCollector(String[].class, 2);
        System.out.println("collector " + (String) collect.invokeExact("x", "y"));
        MethodHandle asList = lookup.findStatic(Arrays.class, "asList",
                MethodType.methodType(List.class, Object[].class));
        System.out.println("asList " + asList.invoke(1, 2, 3));

        System.out.println("withArgs " + tag.invokeWithArguments("h", 6));
        System.out.println("withList " + tag.invokeWithArguments(List.of("i", 7)));

        MethodHandle fail = lookup.findStatic(TestMethodHandleCombinators.class, "fail",
                MethodType.methodType(int.class, int.class));
        MethodHandle chain = MethodHandles.filterArguments(fail, 0, twice);
        try {
            chain.invoke(4);
        } catch (ArithmeticException e) {
            System.out.println("chain caught " + e.getMessage());
        }
    }
}
