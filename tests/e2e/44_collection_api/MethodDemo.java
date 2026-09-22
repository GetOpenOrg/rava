import java.lang.reflect.Method;

public class MethodDemo {
    public static void main(String[] args) throws Exception {
        Class<?> c = Integer.class;

        // 命中：valueOf(String)（public static，返回 Integer）
        Method v1 = c.getDeclaredMethod("valueOf", String.class);
        System.out.println("n1=" + v1.getName() + ",mods=" + v1.getModifiers()
                + ",ret=" + v1.getReturnType().getSimpleName());

        // 重载消歧：parseInt(String) 与 parseInt(String, int) 同名不同参
        //（JDK 查询键不含返回类型）
        Method p1 = c.getDeclaredMethod("parseInt", String.class);
        Method p2 = c.getDeclaredMethod("parseInt", String.class, int.class);
        System.out.println("p1params=" + p1.getParameterTypes().length
                + ",ret=" + p1.getReturnType().getSimpleName());
        System.out.println("p2params=" + p2.getParameterTypes().length
                + ",ret=" + p2.getReturnType().getSimpleName());

        // 未命中 → NoSuchMethodException（消息为 JDK 形态的完整描述）
        try {
            c.getDeclaredMethod("noSuchMethodHere");
            System.out.println("unexpected");
        } catch (NoSuchMethodException e) {
            System.out.println("caught=" + e.getMessage());
        }

        // 复数形态：声明方法序列里能按名找到 parseInt
        boolean hasParseInt = false;
        for (Method m : c.getDeclaredMethods()) {
            if (m.getName().equals("parseInt")) {
                hasParseInt = true;
            }
        }
        System.out.println("hasParseInt=" + hasParseInt);
    }
}
