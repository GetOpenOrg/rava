// 祖先类经接口 A 注入的 default 槽位，被子类经更具体接口 B（B extends A）的 default 覆盖：
// 子类展开的 default 必须落在祖先 vtable 槽位上（JVM 最具体 default 规则）
public class DefaultSlot {
    interface Coll {
        default String split() { return "coll"; }
        default String stream() { return "stream:" + split(); }
    }

    interface Lst extends Coll {
        default String split() { return "list"; }
    }

    static abstract class AbsColl implements Coll {}

    static abstract class AbsList extends AbsColl implements Lst {}

    static final class ListN extends AbsList {}

    public static void main(String[] args) {
        Coll c = new ListN();
        System.out.println(c.stream());
    }
}
