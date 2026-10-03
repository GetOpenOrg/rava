import java.util.ArrayList;
import java.util.List;

/**
 * 边界用例：子类以具体类型实参读取继承来的泛型字段（javac 在 getfield 后插 checkcast），
 * 覆盖字符串拼接、调用 String 方法、赋给 String 局部变量、作实参传递、经另一实例读取、
 * 多层继承与多个类型形参。
 */
public class TestGenericFieldInheritCast {
    abstract static class Shape<T> {
        final T tag;
        protected T extra;

        Shape(T tag) {
            this.tag = tag;
        }

        abstract String label();
    }

    static final class Square extends Shape<String> {
        final int side;

        Square(String tag, int side) {
            super(tag);
            this.side = side;
            this.extra = tag.toUpperCase();
        }

        String concat() {
            return "Square(" + tag + ", " + side + ")";
        }

        int tagLength() {
            return tag.length() + extra.indexOf('E');
        }

        String local() {
            String s = tag;
            String e = extra;
            return s + "/" + e;
        }

        String passed() {
            return describe(tag) + " " + describe(other(this).extra);
        }

        static Square other(Square sq) {
            return sq;
        }

        static String describe(String s) {
            return "<" + s + ":" + s.length() + ">";
        }

        @Override
        String label() {
            String s = tag;
            extra = s + "!";
            return describe(tag.toUpperCase()) + extra;
        }

        @Override
        public String toString() {
            return "Square(" + tag + ", " + side + ")";
        }

        boolean sameTag(Square o) {
            return tag.equals(o.tag) && o.tag.startsWith(tag.substring(0, 1));
        }
    }

    /** 非泛型中间类：可被覆盖的虚方法读继承的泛型字段（子类经 super 调用） */
    static class Mid extends Shape<String> {
        Mid(String tag) {
            super(tag);
        }

        String info() {
            String e = extra;
            return tag + "!" + tag.length() + "/" + e;
        }

        @Override
        String label() {
            extra = tag.concat("-x");
            return "mid:" + tag;
        }
    }

    static final class Leaf extends Mid {
        Leaf(String tag) {
            super(tag);
        }

        @Override
        String info() {
            return super.info() + " leaf " + tag.charAt(0);
        }
    }

    static class Pair<A, B> {
        A first;
        B second;
    }

    static class NumPair<N extends Number> extends Pair<String, N> {
        String render() {
            first = "n";
            String f = first;
            return f + "=" + second + " int=" + second.intValue();
        }
    }

    static final class IntPair extends NumPair<Integer> {
        int doubled() {
            second = 21;
            first = "xy";
            Integer v = second;
            return v * 2 + first.length();
        }
    }

    static class Holder<T> {
        T value;
    }

    static final class ListHolder extends Holder<List<String>> {
        String joined() {
            value = new ArrayList<>();
            value.add("a");
            value.add("b");
            List<String> l = value;
            return String.join("+", l) + " size=" + value.size();
        }
    }

    public static void main(String[] args) {
        Square sq = new Square("red", 3);
        System.out.println(sq.concat());
        System.out.println(sq.tagLength());
        System.out.println(sq.local());
        System.out.println(sq.passed());
        System.out.println(sq.sameTag(new Square("red", 5)) + " " + sq.sameTag(new Square("blue", 3)));
        String outside = sq.tag;
        System.out.println("outside " + outside.toUpperCase() + " " + sq.extra.toLowerCase());

        System.out.println(sq.label() + " " + sq + " " + sq.extra);
        Shape<String> sh = sq;
        System.out.println(sh.label() + " " + sh.tag.length());

        Mid mid = new Mid("mm");
        System.out.println(mid.label() + " " + mid.info());
        Mid leaf = new Leaf("leafy");
        System.out.println(leaf.label() + " " + leaf.info());

        IntPair ip = new IntPair();
        System.out.println(ip.doubled());
        System.out.println(ip.render());

        System.out.println(new ListHolder().joined());
    }
}
