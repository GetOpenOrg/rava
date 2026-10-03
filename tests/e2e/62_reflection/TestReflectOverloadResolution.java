import java.lang.reflect.InvocationTargetException;
import java.lang.reflect.Method;

/**
 * 反射取重载各形态与 invoke 的装箱适配：同名字方法按形参类型分别取得、
 * 装箱实参调基本类型形参、varargs 三形态、getGenericParameterTypes。
 * （OGNL 方法分派与 spring 方法解析的地基。）
 */
public class TestReflectOverloadResolution {

    public static class Calc {
        public String f(int x) {
            return "int:" + x;
        }

        public String f(Integer x) {
            return "Integer:" + x;
        }

        public String f(Object x) {
            return "Object:" + x;
        }

        public String f(String tag, int... nums) {
            return "var:" + tag + "/" + nums.length;
        }

        private String secret(int x) {
            return "secret:" + x;
        }

        public <T extends java.util.List<?>> String generic(T item) {
            return "generic";
        }
    }

    public static void main(String[] args) throws Exception {
        Class<Calc> c = Calc.class;
        Method fInt = c.getMethod("f", int.class);
        Method fBox = c.getMethod("f", Integer.class);
        Method fObj = c.getMethod("f", Object.class);
        Method fVar = c.getMethod("f", String.class, int[].class);

        Calc calc = new Calc();
        // 精确签名各自可取，互不混淆
        System.out.println("int=" + fInt.invoke(calc, 1));
        System.out.println("box=" + fBox.invoke(calc, Integer.valueOf(1)));
        System.out.println("obj=" + fObj.invoke(calc, "s"));
        // 装箱适配：int 形参方法吃 Integer 实参
        System.out.println("autobox=" + fInt.invoke(calc, Integer.valueOf(7)));

        // varargs 形态：数组整体 / 空数组；散参逐个装箱对 int... 不适配（组装 int[] 不拆箱）
        System.out.println("varargs-array=" + fVar.invoke(calc, "b", new int[] { 3 }));
        System.out.println("varargs-empty=" + fVar.invoke(calc, "c", new int[0]));
        try {
            fVar.invoke(calc, "a", 1, 2);
            System.out.println("varargs-spread=ok");
        } catch (IllegalArgumentException e) {
            System.out.println("varargs-spread-ex=" + e.getClass().getSimpleName());
        }
        System.out.println("isVarArgs=" + fVar.isVarArgs());

        // 私有方法：setAccessible 后可调
        Method s = c.getDeclaredMethod("secret", int.class);
        s.setAccessible(true);
        System.out.println("private=" + s.invoke(calc, 5));

        // 泛型方法的形参元数据
        Method g = c.getMethod("generic", java.util.List.class);
        System.out.println("generic-param=" + g.getGenericParameterTypes()[0].getTypeName());
        System.out.println("generic-varargs=" + g.getTypeParameters().length);

        // invoke 实参类型不匹配 → IllegalArgumentException（不包装 ITE）
        try {
            fInt.invoke(calc, "not-an-int");
        } catch (IllegalArgumentException e) {
            System.out.println("type-mismatch-ex=" + e.getClass().getSimpleName());
        } catch (InvocationTargetException e) {
            System.out.println("type-mismatch-ite");
        }
    }
}
