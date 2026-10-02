// 桥方法并入继承槽：Spliterators$EmptySpliterator$OfRef 的 tryAdvance(Consumer) 是 synthetic 桥
// （invokespecial EmptySpliterator.tryAdvance(Object)），发射层省略桥、把 Spliterator 槽并入继承的
// 真实实现；派发到桥即派发到该实现，槽条目不得发存根。
import java.util.Spliterator;
import java.util.Spliterators;

public class BridgeMergedSlot {
    public static void main(String[] args) {
        Spliterator<String> s = Spliterators.emptySpliterator();
        System.out.println(s.tryAdvance(x -> System.out.println(x)));
    }
}
