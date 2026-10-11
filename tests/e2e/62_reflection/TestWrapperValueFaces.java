import java.lang.reflect.Method;

// 包装类取值的分层见证（fix-widen，b1012-widen5 存根根因的对照组）：同一翻译包装实例
// 经三条路径取值——(a) 静态类型为包装类的直呼（调用点入闭包 → 真翻译体）；
// (b) 擦除 Object 接收者的虚分派（ObjectVTable 桥 → 本类 vtable 槽；槽体存在时桥
// 自动生效）；(c) Method.invoke 实参拆箱（reflect_dispatch::unbox_* 的承载字段直读，
// 不依赖方法链）。三层应答一致且等于 JVM 语义；hashCode 按值契约
//（Integer/Short/Byte/Character 恒等、Float=floatToIntBits、Long/Double 折叠、Boolean 1231/1237）。
// 注意：本测例自身的直呼使 hashCode/toString 入链——「程序不触达这些槽时落到存根
//（Ok(0)/二进制名）」的失败面由 TestWidenFacesInvoke / TestUnboxWrapperFaces 承载。
public class TestWrapperValueFaces {
    static int takeInt(int x) { return x; }
    static float takeFloat(float f) { return f; }
    static char takeChar(char c) { return c; }

    public static void main(String[] args) throws Exception {
        Short sv = Short.valueOf((short) -300);
        Object so = sv;
        System.out.println("short " + sv.hashCode() + " " + so.hashCode() + " " + sv.toString().length());
        Character cv = Character.valueOf('A');
        Object co = cv;
        System.out.println("char " + cv.hashCode() + " " + co.hashCode() + " " + cv.toString().length());
        Float fv = Float.valueOf(3.0f);
        Object fo = fv;
        System.out.println("float " + fv.hashCode() + " " + fo.hashCode()
                + " " + Float.valueOf(-0.0f).hashCode() + " " + Float.valueOf(Float.NaN).hashCode());
        Byte bv = Byte.valueOf((byte) 100);
        Object bo = bv;
        System.out.println("byte " + bv.hashCode() + " " + bo.hashCode());
        Integer iv = Integer.valueOf(7);
        Object io = iv;
        System.out.println("int " + iv.hashCode() + " " + io.hashCode());
        Long lv = Long.valueOf(5L);
        Object lo = lv;
        System.out.println("long " + lv.hashCode() + " " + lo.hashCode());
        Double dv = Double.valueOf(1.5);
        Object dobj = dv;
        System.out.println("double " + dv.hashCode() + " " + dobj.hashCode());
        Boolean zv = Boolean.valueOf(true);
        Object zo = zv;
        System.out.println("bool " + zv.hashCode() + " " + zo.hashCode());

        Method mi = TestWrapperValueFaces.class.getDeclaredMethod("takeInt", int.class);
        Method mf = TestWrapperValueFaces.class.getDeclaredMethod("takeFloat", float.class);
        Method mc = TestWrapperValueFaces.class.getDeclaredMethod("takeChar", char.class);
        for (Method m : new Method[] { mi, mf, mc }) m.setAccessible(true);
        System.out.println("refl " + (Integer) mi.invoke(null, sv)
                + " " + (Integer) mi.invoke(null, cv)
                + " " + (Integer) mi.invoke(null, bv)
                + " " + (Integer) mi.invoke(null, iv));
        System.out.println("refl-float " + (Float) mf.invoke(null, fv));
        System.out.println("refl-char " + (int) (char) (Character) mc.invoke(null, cv));
    }
}
