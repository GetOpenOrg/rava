import java.util.AbstractList;
import java.util.List;

/**
 * 缺口 E2：继承自 JDK 抽象类的「具体方法」是否被子类的 vtable 正确承载。
 *
 * MyList 只实现 get(int) / size()，其余行为（toString / contains / indexOf /
 * iterator）全部继承自 AbstractList 与 AbstractCollection。
 * 调用这些方法时，RTA 需要把虚调用分派到继承来的实现并把方法体注入子类，
 * 否则会落到 AbstractCollection.iterator() 这类 abstract 声明（运行时 panic）。
 */
public class TestInheritedMethod {
    static class MyList extends AbstractList<String> {
        private final String[] data;

        MyList(String[] data) {
            this.data = data;
        }

        @Override
        public String get(int index) {
            return data[index];
        }

        @Override
        public int size() {
            return data.length;
        }
    }

    public static void main(String[] args) {
        List<String> list = new MyList(new String[]{"a", "b", "c"});

        // 继承自 AbstractCollection 的具体方法（内部走 iterator() 虚调用）
        System.out.println(list);
        System.out.println(list.contains("b"));
        System.out.println(list.contains("z"));
        System.out.println(list.isEmpty());

        // 继承自 AbstractList 的具体方法
        System.out.println(list.indexOf("c"));
        System.out.println(list.indexOf("nope"));
        System.out.println(list.lastIndexOf("a"));

        // 继承自 AbstractCollection 的 toArray
        Object[] arr = list.toArray();
        System.out.println(arr.length);
        System.out.println(arr[0]);
    }
}
