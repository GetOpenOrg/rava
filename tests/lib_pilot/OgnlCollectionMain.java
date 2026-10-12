
import java.util.List;

import ognl.Ognl;
import ognl.OgnlContext;

/**
 * OGNL pilot A21：集合投影 / 选择表达式（OGNL 独有的集合 DSL）。
 */
public class OgnlCollectionMain {

    public static class Item {
        public final String name;
        public final int qty;

        public Item(String name, int qty) {
            this.name = name;
            this.qty = qty;
        }

        public String getName() {
            return name;
        }

        public int getQty() {
            return qty;
        }
    }

    static void check(String name, Object expected, Object actual) {
        boolean ok = String.valueOf(expected).equals(String.valueOf(actual));
        System.out.println((ok ? "PASS " : "FAIL ") + name
                + (ok ? "" : " :: expected=<" + expected + "> actual=<" + actual + ">"));
    }

    public static void main(String[] args) throws Exception {
        List<Item> items = List.of(
                new Item("apple", 3), new Item("banana", 1), new Item("cherry", 5));
        Object root = new Object();
        OgnlContext ctx = (OgnlContext) Ognl.createDefaultContext(root);
        ctx.put("items", items);

        // 投影：.{表达式}
        check("projection", "[apple, banana, cherry]",
                Ognl.getValue("#items.{name}", ctx, root).toString());
        check("projection-qty", "[3, 1, 5]",
                Ognl.getValue("#items.{qty}", ctx, root).toString());

        // 选择：.{? 条件}（过滤后逐个取名）
        List<?> selected = (List<?>) Ognl.getValue("#items.{? qty > 2}", ctx, root);
        List<String> selectedNames = selected.stream()
                .map(i -> ((Item) i).name).sorted().toList();
        check("select", "[apple, cherry]", selectedNames.toString());

        // 选择第一个/最后一个：.{^ } .{$ }
        Object first = Ognl.getValue("#items.{^ qty > 2}", ctx, root);
        check("select-first", "apple", ((List<?>) first).isEmpty() ? "empty"
                : ((Item) ((List<?>) first).get(0)).name);
        Object last = Ognl.getValue("#items.{$ qty > 2}", ctx, root);
        check("select-last", "cherry", ((List<?>) last).isEmpty() ? "empty"
                : ((Item) ((List<?>) last).get(((List<?>) last).size() - 1)).name);

        // 嵌套：先选择再投影（一行完成过滤+取字段）
        Object filtered = Ognl.getValue("#items.{? qty > 2}.{name}", ctx, root);
        check("select-then-project", "[apple, cherry]", filtered.toString());

        // 聚合（投影后 sum 的等价写法：投影到 qty 再手写求和）
        List<?> qtys = (List<?>) Ognl.getValue("#items.{qty}", ctx, root);
        int sum = qtys.stream().mapToInt(o -> (Integer) o).sum();
        check("project-sum", 9, sum);
        System.out.println("DONE OgnlCollectionMain");
    }
}
