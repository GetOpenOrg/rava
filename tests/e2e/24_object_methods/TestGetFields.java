import java.lang.reflect.Field;
import java.util.ArrayList;
import java.util.Collections;
import java.util.List;

/**
 * Class.getFields（public 字段，含继承与接口常量）与 getDeclaredFields 对照（ListFields 揭出：
 * getFields 走 JDK ReflectionData 缓存链落存根）。覆盖：本类 public / 非 public、父类 public
 * 继承、多层继承、接口常量（直接实现与父类实现的接口）、无字段类、Field.get 取值。
 * 输出按名排序（接口常量枚举顺序为已登记偏差，见 compatibility.md）。
 */
public class TestGetFields {
    interface Consts { int LIMIT = 10; String TAG = "t"; }
    interface More extends Consts { long BIG = 1L << 40; }
    static class Base implements Consts {
        public int baseVisible = 1;
        protected int baseProt = 2;
        private int basePriv = 3;
    }
    static class Mid extends Base { public String midName = "mid"; }
    static class Leaf extends Mid implements More {
        public double leafValue = 2.5;
        int leafPkg = 4;
    }
    static class Empty {}

    static List<String> names(Field[] fs, Object target) throws Exception {
        List<String> out = new ArrayList<>();
        for (Field f : fs) out.add(f.getName() + "=" + f.get(target));
        Collections.sort(out);
        return out;
    }

    public static void main(String[] args) throws Exception {
        Leaf leaf = new Leaf();
        System.out.println("Leaf.getFields " + names(Leaf.class.getFields(), leaf));
        System.out.println("Leaf.getDeclaredFields " + names(Leaf.class.getDeclaredFields(), leaf));
        System.out.println("Base.getFields " + names(Base.class.getFields(), new Base()));
        System.out.println("Empty.getFields " + Empty.class.getFields().length);
        System.out.println("Consts.getFields " + names(Consts.class.getFields(), null));
    }
}
