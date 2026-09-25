import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;
import java.util.Set;
import java.util.stream.Collectors;
import java.util.stream.Stream;

/**
 * N5 槽位形态 2：继承槽位的分派（最近声明者填槽）。
 *
 * 用户类层次覆盖四种形态，JDK 类层次覆盖对应的真实调用面：
 *   A. 抽象声明 → 中间类实现 → 叶子静默继承，基类体内 this.m() 回调；
 *   B. 具体声明 → 中间类覆盖 → 叶子继承（叶子必须走中间覆盖，不是声明体）；
 *   C. 泛型抽象槽 + 子类具体类型覆盖（javac 桥填擦除槽位）；
 *   D. 接口 default → 中间类覆盖 → 叶子继承。
 * JDK 侧：不可变集合 size/isEmpty（AbstractCollection 抽象 size）、流水线
 * opIsStateful（无状态/有状态阶段混合）、Set/List 的 spliterator、skip/limit 切片。
 */
public class TestInheritedSlots {
    // ── A：抽象声明 + 中间实现 + 叶子继承 ────────────────────────────
    static abstract class Shape {
        abstract String kind();
        String describe() { return "shape:" + this.kind(); }
    }
    static class Polygon extends Shape {
        String kind() { return "polygon"; }
    }
    static class Square extends Polygon { }

    // ── B：具体声明 + 中间覆盖 + 叶子继承 ────────────────────────────
    static class Animal {
        String sound() { return "..."; }
        String speak() { return "says " + this.sound(); }
    }
    static class Dog extends Animal {
        String sound() { return "woof"; }
    }
    static class Puppy extends Dog { }

    // ── C：泛型抽象槽 + 具体类型覆盖（桥）────────────────────────────
    static abstract class Box<T> {
        abstract int weigh(T item);
        int total(List<T> items) {
            int s = 0;
            for (T t : items) s += this.weigh(t);
            return s;
        }
    }
    static class StringBox extends Box<String> {
        int weigh(String item) { return item.length(); }
    }
    static class LabelBox extends StringBox { }

    // ── D：接口 default + 中间覆盖 + 叶子继承 ────────────────────────
    interface Greeter {
        default String greet() { return "hello"; }
        default String twice() { return this.greet() + " " + this.greet(); }
    }
    static class Formal implements Greeter {
        public String greet() { return "good day"; }
    }
    static class VeryFormal extends Formal { }
    static class Plain implements Greeter { }

    public static void main(String[] args) {
        System.out.println("A.square=" + new Square().describe());
        System.out.println("A.polygon=" + new Polygon().describe());
        Shape s = new Square();
        System.out.println("A.virtual=" + s.kind());

        System.out.println("B.animal=" + new Animal().speak());
        System.out.println("B.dog=" + new Dog().speak());
        System.out.println("B.puppy=" + new Puppy().speak());

        List<String> words = Arrays.asList("ab", "cde", "f");
        System.out.println("C.stringBox=" + new StringBox().total(words));
        System.out.println("C.labelBox=" + new LabelBox().total(words));
        Box<String> b = new LabelBox();
        System.out.println("C.viaBase=" + b.weigh("wxyz"));

        System.out.println("D.plain=" + new Plain().twice());
        System.out.println("D.formal=" + new Formal().twice());
        System.out.println("D.veryFormal=" + new VeryFormal().twice());

        // ── JDK：不可变集合 size / isEmpty（AbstractCollection.size 抽象）─
        List<Integer> l0 = List.of();
        List<Integer> l2 = List.of(1, 2);
        List<Integer> ln = List.of(1, 2, 3, 4, 5);
        Set<String> s3 = Set.of("x", "y", "z");
        System.out.println("imm.size=" + l0.size() + "," + l2.size() + "," + ln.size() + "," + s3.size());
        System.out.println("imm.isEmpty=" + l0.isEmpty() + "," + l2.isEmpty() + "," + s3.isEmpty());
        System.out.println("imm.contains=" + ln.contains(4) + "," + s3.containsAll(List.of("x", "z")));

        // ── JDK：spliterator（AbstractSet / AbstractCollection 声明）──────
        System.out.println("spl.setCount=" + s3.stream().count());
        System.out.println("spl.listSum=" + ln.stream().mapToInt(Integer::intValue).sum());
        List<String> al = new ArrayList<>(List.of("q", "w", "e"));
        System.out.println("spl.subList=" + al.subList(1, 3).stream().collect(Collectors.joining()));

        // ── JDK：opIsStateful（无状态 / 有状态阶段混合）──────────────────
        String chain = Stream.of(5, 3, 5, 1, 3, 9)
                .filter(v -> v > 1)          // 无状态
                .map(v -> v * 10)            // 无状态
                .distinct()                  // 有状态
                .sorted()                    // 有状态
                .map(String::valueOf)
                .collect(Collectors.joining(","));
        System.out.println("pipe.mixed=" + chain);
        // ── JDK：skip / limit 切片 ───────────────────────────────────────
        System.out.println("slice=" + Stream.iterate(1, v -> v + 1).skip(3).limit(4)
                .map(String::valueOf).collect(Collectors.joining(",")));
        System.out.println("slice.sorted=" + Stream.of(9, 8, 7, 6, 5).sorted().skip(1).limit(2)
                .map(String::valueOf).collect(Collectors.joining(",")));
    }
}
