import java.util.ArrayList;
import java.util.HashSet;
import java.util.LinkedList;
import java.util.List;
import java.util.Set;
import java.util.TreeSet;

/**
 * 循环体内 if/else 两臂分别以接口工厂（Set.of）与具体类（new HashSet）给同一局部赋值，
 * 循环内的抛出路径在循环后读取该局部（javac 为两臂各生成一条同名同槽的 LocalVariableTable 项）。
 * 局部的声明类型应取 LVT 的接口类型，而非两臂值类型的公共超类 Object。
 */
public class TestLoopJoinedLocal {
    static String requires(int[] flags, String[] names) {
        StringBuilder sb = new StringBuilder();
        for (int i = 0; i < flags.length; i++) {
            String dn = names[i];
            Set<String> mods;
            if (flags[i] == 0) {
                mods = Set.of();
            } else {
                mods = new HashSet<>();
                if ((flags[i] & 1) != 0) mods.add("transitive");
                if ((flags[i] & 2) != 0) mods.add("static");
            }
            int version = flags[i] >> 4;
            if (version == 0) {
                record(sb, mods, dn);
            } else {
                String vs = "@" + version;
                record(sb, mods, dn + vs);
            }
            if (dn.equals("java.base")) {
                if (mods.contains("transitive") || mods.contains("static")) {
                    String flagName;
                    if (mods.contains("transitive")) {
                        flagName = "ACC_TRANSITIVE";
                    } else {
                        flagName = "ACC_STATIC_PHASE";
                    }
                    throw new IllegalArgumentException("java.base has " + flagName + " set");
                }
            }
        }
        if (sb.length() == 0) {
            throw new IllegalStateException("empty");
        }
        return sb.toString();
    }

    static void record(StringBuilder sb, Set<String> mods, String dn) {
        sb.append(dn).append(new TreeSet<>(mods)).append(';');
    }

    /** 两臂均为具体类（ArrayList / LinkedList），声明类型为接口 List。 */
    static int lists(int n) {
        int total = 0;
        for (int i = 0; i < n; i++) {
            List<Integer> xs;
            if (i % 2 == 0) {
                xs = new ArrayList<>();
            } else {
                xs = new LinkedList<>();
            }
            xs.add(i);
            xs.add(i * 10);
            if (xs.size() > 5) {
                throw new IllegalStateException("too many " + xs);
            }
            total += xs.get(1);
        }
        return total;
    }

    public static void main(String[] args) {
        System.out.println(requires(new int[] {0, 1, 0x23, 0}, new String[] {"a", "b", "c", "java.base"}));
        for (int f : new int[] {1, 2}) {
            try {
                requires(new int[] {0, f}, new String[] {"a", "java.base"});
            } catch (IllegalArgumentException e) {
                System.out.println("rejected: " + e.getMessage());
            }
        }
        System.out.println("lists = " + lists(4));
    }
}
