import java.util.ArrayList;
import java.util.List;
import java.util.ListIterator;

/**
 * ListIterator 游标族（方法级实测：nextIndex 7 jar，此前零覆盖）：
 * nextIndex/previousIndex 与游标位置、add/set/remove 的位置语义、双向遍历。
 */
public class TestListIteratorIndexes {

    public static void main(String[] args) {
        List<String> list = new ArrayList<>(List.of("a", "b", "c"));
        ListIterator<String> it = list.listIterator();

        System.out.println("start-next-idx=" + it.nextIndex() + " prev-idx=" + it.previousIndex());
        System.out.println("has-prev=" + it.hasPrevious());
        it.next();
        System.out.println("after-next-idx=" + it.nextIndex() + " prev-idx=" + it.previousIndex());
        it.next();
        System.out.println("end-idx=" + it.nextIndex() + " has-next=" + it.hasNext());

        // previous 回退与索引对称
        String back = it.previous();
        System.out.println("prev-val=" + back + " idx-after-prev=" + it.nextIndex());

        // set：替换最近经过的元素
        it.set("B");
        System.out.println("after-set=" + list);

        // add：插入在游标处，新元素不可被 previous 触达（游标在新元素前）
        ListIterator<String> it2 = list.listIterator(1);
        it2.add("X");
        System.out.println("after-add=" + list + " idx=" + it2.nextIndex());
        System.out.println("add-prev-skip=" + (it2.previous().equals("b")));

        // remove：删除最近经过的元素
        ListIterator<String> it3 = list.listIterator();
        it3.next();
        it3.remove();
        System.out.println("after-remove=" + list + " idx=" + it3.nextIndex());

        // 从中间位置开始的索引对
        ListIterator<String> it4 = list.listIterator(2);
        System.out.println("mid-idx=" + it4.nextIndex() + " prev-idx=" + it4.previousIndex());
        System.out.println("mid-val=" + it4.next());
    }
}
