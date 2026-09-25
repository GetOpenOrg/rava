import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;

/**
 * equiv 探针④：Object.clone（浅拷贝语义、Cloneable 契约、协变覆盖、数组 clone）。
 */
public class TestObjectClone {
    static class Point implements Cloneable {
        int x, y;
        int[] tags;
        List<String> names;
        Point(int x, int y) { this.x = x; this.y = y; this.tags = new int[]{1, 2}; this.names = new ArrayList<>(List.of("p")); }
        @Override public Point clone() {              // 协变返回 + 吞掉受检异常
            try { return (Point) super.clone(); }
            catch (CloneNotSupportedException e) { throw new AssertionError(e); }
        }
    }

    static class Deep extends Point {
        String label = "deep";
        Deep(int x, int y) { super(x, y); }
        @Override public Deep clone() {               // 深拷贝：在浅拷贝基础上复制可变字段
            Deep d = (Deep) super.clone();
            d.tags = tags.clone();
            d.names = new ArrayList<>(names);
            return d;
        }
    }

    static class NotCloneable {
        Object tryClone() throws CloneNotSupportedException { return super.clone(); }
    }

    public static void main(String[] args) throws Exception {
        // 浅拷贝：新对象、基本字段复制、引用字段共享
        Point p = new Point(3, 4);
        Point q = p.clone();
        System.out.println("shallow.distinct=" + (p != q) + ",class=" + q.getClass().getSimpleName());
        System.out.println("shallow.fields=" + q.x + "," + q.y);
        q.x = 99;
        System.out.println("shallow.primIndependent=" + p.x);
        System.out.println("shallow.refShared=" + (p.tags == q.tags) + "," + (p.names == q.names));
        q.tags[0] = 42; q.names.add("q");
        System.out.println("shallow.mutationVisible=" + p.tags[0] + "," + p.names);

        // 子类 clone：运行时类保留、子类字段一并复制；深拷贝的可变字段独立
        Deep d = new Deep(1, 2);
        Deep e = d.clone();
        System.out.println("deep.class=" + e.getClass().getSimpleName() + ",label=" + e.label);
        e.tags[0] = 7; e.names.add("e");
        System.out.println("deep.independent=" + d.tags[0] + "," + d.names + "," + e.names);
        Point viaBase = d;
        System.out.println("deep.viaBase=" + viaBase.clone().getClass().getSimpleName());

        // 未实现 Cloneable → CloneNotSupportedException
        try {
            new NotCloneable().tryClone();
            System.out.println("notCloneable=unexpected");
        } catch (CloneNotSupportedException ex) {
            System.out.println("notCloneable=" + ex.getClass().getSimpleName());
        }

        // 数组 clone：一维基本类型独立、引用数组浅拷贝、二维只复制外层
        int[] ia = {1, 2, 3};
        int[] ib = ia.clone();
        ib[0] = 100;
        System.out.println("arr.int=" + Arrays.toString(ia) + "," + Arrays.toString(ib) + "," + (ia != ib));
        Point[] pa = {new Point(1, 1), null};
        Point[] pb = pa.clone();
        System.out.println("arr.ref=" + (pa != pb) + "," + (pa[0] == pb[0]) + "," + (pb[1] == null));
        int[][] m = {{1, 2}, {3, 4}};
        int[][] n = m.clone();
        n[0][0] = 9; n[1] = new int[]{8};
        System.out.println("arr.2d=" + m[0][0] + "," + m[1].length + "," + (m[0] == n[0]));
        String[] empty = new String[0];
        System.out.println("arr.empty=" + empty.clone().length + "," + (empty.clone() != empty));
        char[] cs = "hi".toCharArray();
        System.out.println("arr.char=" + new String(cs.clone()));
    }
}
