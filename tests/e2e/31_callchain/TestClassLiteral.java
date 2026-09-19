import java.util.ArrayList;
import java.util.HashSet;
import java.util.LinkedList;
import java.util.List;
import java.util.Map;
import java.util.Set;

/**
 * 调用链缺口 A1：类字面量（ldc / ldc_w 加载 CONSTANT_Class）。
 *
 * `X.class` 编译为 `ldc Class java/util/X` 指令，是类引用进入调用链的独立通道。
 * 本用例刻意只用类字面量引用 LinkedList / TreeMap / HashSet，全程不 new 它们，
 * 使 ldc 成为这些类进入闭包的唯一入口：ldc 通道缺失时它们不会被转译。
 */
public class TestClassLiteral {
    public static void main(String[] args) {
        // 仅通过 ldc 引用的类：不实例化、不调用其任何方法
        Class<?> linkedListClass = LinkedList.class;
        Class<?> treeMapClass = java.util.TreeMap.class;
        Class<?> hashSetClass = HashSet.class;

        System.out.println(linkedListClass.getName());
        System.out.println(treeMapClass.getName());
        System.out.println(hashSetClass.getName());

        // 同一字面量的两次 ldc 应拿到同一个 Class 对象
        System.out.println(linkedListClass == LinkedList.class);

        // 类字面量作为实参传给 JDK 方法（isAssignableFrom）
        System.out.println(List.class.isAssignableFrom(ArrayList.class));
        System.out.println(Map.class.isAssignableFrom(java.util.TreeMap.class));
        System.out.println(Set.class.isAssignableFrom(HashSet.class));

        // 基本类型与数组的类字面量
        System.out.println(int.class.getName());
        System.out.println(String[].class.getName());
        System.out.println(int[].class.getName());
    }
}
