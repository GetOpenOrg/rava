import java.util.function.IntBinaryOperator;
import java.util.function.IntUnaryOperator;

/**
 * 大写开头的参数 / 局部变量名（Rosetta 语料 WordWrap 揭出：签名与方法体引用命名不一致 E0425）。
 * 覆盖：实例方法 / 静态方法 / 构造器参数、单字母与缩写前缀（IOPort → ioPort）、
 * ALL_CAPS 原样、wide 类型（long / double 占两槽后的下一参数）、lambda 形参与捕获、
 * catch 变量、参数重赋值、与 camelCase 化结果同名的另一个局部（shadow 区分）。
 */
public class TestUpperCaseLocalNames {
    private final int Width;
    private final String Label;

    TestUpperCaseLocalNames(int Width, String Label) {
        this.Width = Width;
        this.Label = Label;
    }

    int area(int Height) {
        int Result = Width * Height;
        return Result;
    }

    static long mix(long Big, int Small, double Ratio, int After) {
        long Acc = Big + Small;
        Acc += (long) (Ratio * 10);
        return Acc + After;
    }

    static int acronym(int IOPort, int URLCount, int X) {
        return IOPort * 100 + URLCount * 10 + X;
    }

    static int constants(int MAX_VALUE, int ARG_BASE) {
        return MAX_VALUE - ARG_BASE;
    }

    static int reassign(int Count) {
        Count = Count * 2;
        Count += 1;
        return Count;
    }

    static int lambdas(int Base) {
        IntUnaryOperator AddBase = V -> V + Base;
        IntBinaryOperator Mul = (A, B) -> A * B;
        return AddBase.applyAsInt(5) + Mul.applyAsInt(Base, 3);
    }

    static String catches(String Input) {
        try {
            return "ok:" + Integer.parseInt(Input);
        } catch (NumberFormatException NFE) {
            return "bad:" + NFE.getMessage();
        }
    }

    static int loops(int[] Values) {
        int Sum = 0;
        for (int I = 0; I < Values.length; I++) {
            int Item = Values[I];
            Sum += Item;
        }
        for (int V : Values) {
            Sum += V;
        }
        return Sum;
    }

    String describe(String Prefix) {
        String S = Prefix + Label;
        return S + "/" + Width;
    }

    public static void main(String[] args) {
        TestUpperCaseLocalNames T = new TestUpperCaseLocalNames(7, "box");
        System.out.println("area=" + T.area(6));
        System.out.println("mix=" + mix(1_000_000_000_000L, 3, 2.5, 4));
        System.out.println("acronym=" + acronym(1, 2, 3));
        System.out.println("constants=" + constants(10, 4));
        System.out.println("reassign=" + reassign(20));
        System.out.println("lambdas=" + lambdas(4));
        System.out.println("catch1=" + catches("42"));
        System.out.println("catch2=" + catches("x"));
        System.out.println("loops=" + loops(new int[]{1, 2, 3}));
        System.out.println("describe=" + T.describe("my-"));
    }
}
