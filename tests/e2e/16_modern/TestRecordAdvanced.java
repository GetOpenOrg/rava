import java.util.ArrayList;
import java.util.List;

interface Identifiable {
    long id();
}

record Point(int x, int y) {
    // 紧凑构造器：可规范化参数
    Point {
        if (x < 0) x = -x;
        if (y < 0) y = -y;
    }

    Point(int x, int y, boolean unused) {
        this(x, y);
        if (unused) {
            System.out.println("3-arg ctor used");
        }
    }

    static Point origin() {
        return new Point(0, 0);
    }

    static final String UNIT_LABEL = "point";

    Point translate(int dx, int dy) {
        return new Point(x + dx, y + dy);
    }

    double distance() {
        return Math.sqrt(x * x + y * y);
    }
}

record PersonRecord(String name, int age) implements Identifiable, Comparable<PersonRecord> {
    PersonRecord {
        if (age < 0) {
            throw new IllegalArgumentException("negative age");
        }
    }

    @Override
    public long id() {
        return name.length() * 100L + age;
    }

    @Override
    public int compareTo(PersonRecord other) {
        return Integer.compare(age, other.age);
    }
}

record Box<T>(String label, T value) {
    T doubled(java.util.function.BinaryOperator<T> op) {
        return op.apply(value, value);
    }
}

public class TestRecordAdvanced {

    public static void main(String[] args) {
        Point p = new Point(3, 4);
        System.out.println("point=" + p);
        System.out.println("accessors=" + p.x() + "," + p.y());
        System.out.println("distance=" + p.distance());
        System.out.println("translated=" + p.translate(1, 1));

        Point neg = new Point(-5, -6);
        System.out.println("normalized=" + neg.x() + "," + neg.y());

        System.out.println("origin=" + Point.origin());
        System.out.println("static field=" + Point.UNIT_LABEL);

        // equals / hashCode / toString 自动生成
        Point a = new Point(1, 2);
        Point b = new Point(1, 2);
        System.out.println("equals=" + a.equals(b));
        System.out.println("hashCode equal=" + (a.hashCode() == b.hashCode()));
        System.out.println("toString=" + a.toString());

        Point c = new Point(7, 8, true);
        System.out.println("3-arg=" + c);

        // record 实现接口
        PersonRecord pr = new PersonRecord("alice", 30);
        System.out.println("id=" + pr.id());
        System.out.println("record=" + pr);
        Identifiable idf = pr;
        System.out.println("via interface id=" + idf.id());

        PersonRecord pr2 = new PersonRecord("bob", 25);
        System.out.println("compare=" + pr.compareTo(pr2));
        List<PersonRecord> people = new ArrayList<>();
        people.add(pr);
        people.add(pr2);
        java.util.Collections.sort(people);
        System.out.println("sorted=" + people);

        // 紧凑构造器校验
        try {
            new PersonRecord("bad", -1);
            System.out.println("validation skipped");
        } catch (IllegalArgumentException e) {
            System.out.println("validation -> " + e.getMessage());
        }

        // 泛型 record
        Box<Integer> ib = new Box<>("int", 21);
        Box<String> sb = new Box<>("str", "ab");
        System.out.println("int box=" + ib + " doubled=" + ib.doubled((x, y) -> x + y));
        System.out.println("str box=" + sb + " doubled=" + sb.doubled((x, y) -> x + y));

        System.out.println("done");
    }
}
