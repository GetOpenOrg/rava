import java.util.ArrayList;
import java.util.HashMap;
import java.util.HashSet;

/**
 * Java 版 ArrayList/HashMap/HashSet 测试——与 Rust 生成 API 对比
 * 目标：证明 Rust ergonomic API 使用方式与此 Java 代码高度相似
 *
 * 注：当前 Rust 生成代码已通过 tests/test_array_list_rust.rs 验证输出一致
 * 待实现（已记录为 T42）：for-each 增强循环的正确代码生成
 */
public class TestArrayList {
    public static void main(String[] args) {
        // === 1. ArrayList<String> 基本操作 ===
        ArrayList<String> names = new ArrayList<>();
        names.add("Alice");
        names.add("Bob");
        names.add("Charlie");
        System.out.println(names.size());           // 3
        System.out.println(names.get(0));           // Alice
        System.out.println(names.get(1));           // Bob
        System.out.println(names.get(2));           // Charlie

        // === 2. ArrayList<Integer> 整数集合（autoboxing）===
        ArrayList<Integer> scores = new ArrayList<>();
        scores.add(100);
        scores.add(95);
        scores.add(87);
        System.out.println(scores.size());          // 3
        int first = scores.get(0);                  // unboxing
        System.out.println(first);                  // 100
        System.out.println(scores.get(1));          // 95

        // === 3. 遍历 ArrayList（增强 for 循环）===
        for (String name : names) {
            System.out.println(name);
        }

        // === 4. HashMap<String, Integer> ===
        HashMap<String, Integer> ages = new HashMap<>();
        ages.put("Alice", 30);
        ages.put("Bob", 25);
        ages.put("Charlie", 35);
        System.out.println(ages.size());            // 3
        System.out.println(ages.get("Alice"));      // 30
        System.out.println(ages.get("Bob"));        // 25
        System.out.println(ages.containsKey("Charlie"));  // true
        System.out.println(ages.containsKey("Dave"));     // false

        // === 5. HashSet<String> 去重 ===
        HashSet<String> unique = new HashSet<>();
        unique.add("apple");
        unique.add("banana");
        unique.add("apple");   // 重复，不加入
        System.out.println(unique.size());          // 2
        System.out.println(unique.contains("apple"));    // true
        System.out.println(unique.contains("grape"));    // false
    }
}
