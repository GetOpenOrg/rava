import java.util.ArrayList;
import java.util.Arrays;
import java.util.Iterator;
import java.util.List;

public class TestForEach {

    public static void main(String[] args) {
        // 遍历数组
        int[] nums = {1, 2, 3, 4};
        int total = 0;
        for (int n : nums) {
            total += n;
        }
        System.out.println("array total=" + total);

        StringBuilder sb = new StringBuilder();
        for (String s : new String[]{"a", "b", "c"}) {
            sb.append(s).append("-");
        }
        System.out.println("array join=" + sb);

        // 遍历 Iterable
        List<String> names = new ArrayList<>(Arrays.asList("x", "y", "z"));
        for (String name : names) {
            System.out.println("name=" + name);
        }

        // 嵌套 for-each
        for (int a : nums) {
            for (String s : names) {
                System.out.print(a + s + " ");
            }
        }
        System.out.println();

        // for-each 对 int[] 是只读拷贝语义（修改循环变量不影响数组）
        for (int n : nums) {
            n = n * 10;
        }
        System.out.println("array unchanged=" + Arrays.toString(nums));

        // 但修改对象内容会反映出来
        List<StringBuilder> builders = new ArrayList<>();
        builders.add(new StringBuilder("v"));
        for (StringBuilder b : builders) {
            b.append("!");
        }
        System.out.println("object mutated=" + builders.get(0));

        // 遍历中结构性修改 → ConcurrentModificationException
        try {
            for (String name : names) {
                names.add("new");
            }
        } catch (Exception e) {
            System.out.println("structure modify -> " + e.getClass().getSimpleName());
        }

        // 用 Iterator.remove 是合法的
        List<String> removable = new ArrayList<>(Arrays.asList("p", "q", "r"));
        Iterator<String> it = removable.iterator();
        while (it.hasNext()) {
            String cur = it.next();
            if (cur.equals("q")) {
                it.remove();
            }
        }
        System.out.println("after iterator remove=" + removable);

        // 遍历多维数组
        int[][] matrix = {{1, 2}, {3, 4}};
        StringBuilder flat = new StringBuilder();
        for (int[] row : matrix) {
            for (int cell : row) {
                flat.append(cell).append(",");
            }
        }
        System.out.println("flatten=" + flat);

        // for-each + break/continue
        for (int n : nums) {
            if (n == 3) {
                break;
            }
            if (n == 1) {
                continue;
            }
            System.out.println("selected=" + n);
        }

        // for-each 遍历 char[]
        StringBuilder upper = new StringBuilder();
        for (char ch : "abc".toCharArray()) {
            upper.append(Character.toUpperCase(ch));
        }
        System.out.println("chars=" + upper);

        System.out.println("done");
    }
}
