import java.util.ArrayList;
import java.util.List;

/**
 * 声明为 Object 的局部变量：静态类型以声明为准，不随首个存入值收窄
 * （JDK JceSecurity$2 `Object result; try { result = X; } catch (Exception e) { result = e; }`
 * 揭出：变量按首值类型声明，catch 分支存入异常 → E0308）。
 * 覆盖：首绑定 String / 装箱 Integer / 数组 / null，catch 分支再赋值，循环内换类型再赋值，
 * 之后 instanceof 分派与 toString/equals/hashCode 虚分派；对照组：声明为接口（List）/
 * 父类（Number）的局部保持既有行为。
 */
public class TestObjectLocalWidening {
    static Object fromTry(boolean fail) {
        Object result;
        try {
            if (fail) throw new IllegalStateException("boom");
            result = "ok";
        } catch (Exception e) {
            result = e;
        }
        return result;
    }

    static String kind(Object o) {
        if (o instanceof String s) return "String:" + s;
        if (o instanceof Integer i) return "Integer:" + (i + 1);
        if (o instanceof int[] a) return "int[]:" + a.length;
        if (o instanceof Exception e) return "Exception:" + e.getMessage();
        return o == null ? "null" : "Other:" + o;
    }

    public static void main(String[] args) {
        System.out.println(kind(fromTry(false)));
        System.out.println(kind(fromTry(true)));

        Object a = "text";
        System.out.println(kind(a) + " " + a.hashCode() + " " + a.equals("text"));
        a = 41;
        System.out.println(kind(a));
        a = new int[]{1, 2, 3};
        System.out.println(kind(a));
        Object n = null;
        System.out.println(kind(n));
        n = "late";
        System.out.println(kind(n));

        Object cur = 0;
        StringBuilder trace = new StringBuilder();
        for (int i = 0; i < 4; i++) {
            trace.append(kind(cur)).append(';');
            cur = (i % 2 == 0) ? (Object) ("s" + i) : (Object) Integer.valueOf(i * 10);
        }
        System.out.println(trace);

        List<String> list = new ArrayList<>();   // 对照：声明为接口
        list.add("x");
        System.out.println(list + " " + list.size());
        Number num = Integer.valueOf(7);          // 对照：声明为父类
        System.out.println(num.intValue() + num.doubleValue());
    }
}
