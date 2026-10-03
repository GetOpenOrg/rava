import java.lang.invoke.MethodHandle;
import java.lang.invoke.MethodHandles;
import java.lang.reflect.Field;
import java.lang.reflect.Modifier;
import java.util.ArrayList;
import java.util.List;

/**
 * 字段句柄的来源：按名字常量取到的句柄只写该字段；字段枚举 / 名字不可知的按名取句柄得到的句柄
 * 经集合、辅助方法转手后再由 Field.set* / Lookup.unreflectSetter 写入。被写的字段读取时须是写入后的值，
 * 只被枚举、只读名字与值的类的字段保持构造时的值。
 */
public class TestFieldHandleProvenance {
    static class Named {
        private int level = 1;
        private String tag = "named";
    }

    // 只经枚举得到的句柄写入（句柄先存进列表，再由另一方法写）
    static class Listed {
        private int retries = 3;
        private String mode = "fast";
        private long budget = 10L;
    }

    // 枚举得到的句柄经 Lookup.unreflectSetter 写入
    static class ViaSetter {
        private String label = "before";
    }

    // 名字在运行期拼出：按名取句柄，名字不可知
    static class Computed {
        private int alpha = 7;
        private int beta = 8;
    }

    // 只读枚举：取名字、修饰符与值，不写入
    static class ReadOnly {
        private int width = 640;
        private int height = 480;
    }

    static List<Field> collect(Class<?> c) {
        List<Field> out = new ArrayList<>();
        for (Field f : c.getDeclaredFields()) {
            if (!Modifier.isStatic(f.getModifiers()) && !f.isSynthetic()) {
                out.add(f);
            }
        }
        return out;
    }

    static void overwrite(List<Field> fs, Object target) throws IllegalAccessException {
        for (Field f : fs) {
            f.setAccessible(true);
            Class<?> t = f.getType();
            if (t == int.class) {
                f.setInt(target, 99);
            } else if (t == long.class) {
                f.setLong(target, 12345L);
            } else if (t == String.class) {
                f.set(target, "patched-" + f.getName());
            }
        }
    }

    static String suffix(int i) {
        return i == 0 ? "lpha" : "eta";
    }

    public static void main(String[] args) throws Throwable {
        Named n = new Named();
        Field level = Named.class.getDeclaredField("level");
        level.setAccessible(true);
        level.setInt(n, 2);
        System.out.println("named: level=" + n.level + " tag=" + n.tag);

        Listed l = new Listed();
        overwrite(collect(Listed.class), l);
        System.out.println("listed: retries=" + l.retries + " mode=" + l.mode + " budget=" + l.budget);

        ViaSetter v = new ViaSetter();
        for (Field f : ViaSetter.class.getDeclaredFields()) {
            if (f.getType() == String.class) {
                MethodHandle mh = MethodHandles.lookup().unreflectSetter(f);
                mh.invoke(v, "after");
            }
        }
        System.out.println("setter: label=" + v.label);

        Computed c = new Computed();
        String[] heads = {"a", "b"};
        for (int i = 0; i < heads.length; i++) {
            Field f = Computed.class.getDeclaredField(heads[i] + suffix(i));
            f.setAccessible(true);
            f.setInt(c, f.getInt(c) * 10);
        }
        System.out.println("computed: alpha=" + c.alpha + " beta=" + c.beta);

        ReadOnly r = new ReadOnly();
        StringBuilder sb = new StringBuilder();
        for (Field f : collect(ReadOnly.class)) {
            f.setAccessible(true);
            sb.append(f.getName()).append(':').append(Modifier.toString(f.getModifiers())).append('=').append(f.get(r)).append(' ');
        }
        System.out.println("readonly: " + sb.toString().trim());
        System.out.println("readonly direct: " + r.width + "x" + r.height);
    }
}
