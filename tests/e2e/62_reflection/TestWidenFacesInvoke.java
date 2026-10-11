import java.lang.reflect.Method;

// Method.invoke 基本类型拓宽逐面见证（JLS §5.1.2 拆箱+拓宽）：int 形参分别接受
// Integer（恒等）/ Short / Byte / Character（拓宽）与拒收 Long（long 不窄化）。
// 每面独立 try，翻译侧失败面打 IAE 标记不中断；类名行用于确认装箱形态。
public class TestWidenFacesInvoke {
    static int takeInt(int x) { return x + 1; }

    static void face(String tag, Method mi, Object arg) {
        try {
            System.out.println(tag + " " + (Integer) mi.invoke(null, arg));
        } catch (IllegalArgumentException e) {
            System.out.println(tag + " IAE");
        } catch (Exception e) {
            System.out.println(tag + " " + e.getClass().getSimpleName());
        }
    }

    public static void main(String[] args) throws Exception {
        Method mi = TestWidenFacesInvoke.class.getDeclaredMethod("takeInt", int.class);
        mi.setAccessible(true);
        System.out.println("cls-short " + Short.valueOf((short) -300).getClass().getName());
        System.out.println("cls-byte " + Byte.valueOf((byte) 100).getClass().getName());
        System.out.println("cls-char " + Character.valueOf('A').getClass().getName());
        face("exact-int", mi, Integer.valueOf(7));
        face("widen-short", mi, Short.valueOf((short) -300));
        face("widen-byte", mi, Byte.valueOf((byte) 100));
        face("widen-char", mi, Character.valueOf('A'));
        Short sv = (short) -300;
        face("widen-local-short", mi, sv);
        char cv = 'A';
        face("widen-local-char", mi, cv);
        face("reject-long", mi, Long.valueOf(5L));
        face("reject-float", mi, Float.valueOf(2.5f));
    }
}
