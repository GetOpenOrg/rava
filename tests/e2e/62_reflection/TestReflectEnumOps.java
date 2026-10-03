import java.lang.reflect.Method;

/**
 * 枚举反射全形态：getEnumConstants、Enum.valueOf 命中/未命中、常量子类体的 isEnum
 * 边界、values/valueOf 合成方法的反射调用（mybatis EnumTypeHandler 与各序列化框架
 * 的枚举通道）。
 */
public class TestReflectEnumOps {

    enum Op {
        ALPHA,
        BETA {
            // 带方法体 → BETA 的运行时类是匿名子类（isEnum=false 的经典边界）
        },
        GAMMA
    }

    public static void main(String[] args) throws Exception {
        System.out.println("isEnum=" + Op.class.isEnum());
        Object[] consts = Op.class.getEnumConstants();
        for (Object c : consts) {
            System.out.println("const=" + c + " ord=" + ((Enum<?>) c).ordinal());
        }

        // 常量子类体边界：BETA 实例的运行时类不再 isEnum，但 getSuperclass 回到 Op
        Op b = Op.BETA;
        System.out.println("beta-class=" + b.getClass().getSimpleName()
                + " isEnum=" + b.getClass().isEnum()
                + " super-is-op=" + (b.getClass().getSuperclass() == Op.class));
        System.out.println("alpha-same=" + (Op.ALPHA.getClass() == Op.class));

        System.out.println("valueOf-hit=" + Enum.valueOf(Op.class, "GAMMA"));
        try {
            Enum.valueOf(Op.class, "NOPE");
        } catch (IllegalArgumentException e) {
            System.out.println("valueOf-ex=" + e.getClass().getSimpleName());
        }

        // 合成方法反射：values() / valueOf(String)
        Method values = Op.class.getMethod("values");
        Object[] all = (Object[]) values.invoke(null);
        System.out.println("reflect-values=" + all.length);
        Method valueOf = Op.class.getDeclaredMethod("valueOf", String.class);
        System.out.println("reflect-valueOf=" + valueOf.invoke(null, "ALPHA"));
        System.out.println("valueOf-varargs=" + valueOf.isVarArgs());

        // getMethod 取不到 values 的形参版（不存在）→ NoSuchMethodException 边界
        try {
            Op.class.getMethod("valueOf");
        } catch (NoSuchMethodException e) {
            System.out.println("no-method-ex=" + e.getClass().getSimpleName());
        }
    }
}
