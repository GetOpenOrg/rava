
import org.objectweb.asm.ClassReader;
import org.objectweb.asm.ClassVisitor;
import org.objectweb.asm.ClassWriter;
import org.objectweb.asm.MethodVisitor;
import org.objectweb.asm.Opcodes;
import org.objectweb.asm.commons.SerialVersionUIDAdder;

/**
 * ASM pilot F3：asm-commons 组件——SerialVersionUIDAdder。
 * 给无 serialVersionUID 的类自动计算并注入 SVUID 字段（序列化工具链的经典用法）。
 */
public class AsmCommonsMain {

    static void check(String name, Object expected, Object actual) {
        boolean ok = String.valueOf(expected).equals(String.valueOf(actual));
        System.out.println((ok ? "PASS " : "FAIL ") + name
                + (ok ? "" : " :: expected=<" + expected + "> actual=<" + actual + ">"));
    }

    public static void main(String[] args) throws Exception {
        // 目标：CglibUser.class（本项目无 SVUID 的普通 POJO）
        byte[] original = java.nio.file.Files.readAllBytes(java.nio.file.Path.of(
                "target/classes/com/example/cglib/User.class"));

        // 改写前：无 serialVersionUID 字段
        check("no-svuid-before", false, hasSvuid(original));

        // SerialVersionUIDAdder：计算并注入 SVUID
        ClassWriter cw = new ClassWriter(0);
        ClassVisitor adder = new SerialVersionUIDAdder(cw);
        new ClassReader(original).accept(adder, 0);
        byte[] transformed = cw.toByteArray();

        check("svuid-added", true, hasSvuid(transformed));

        // SVUID 值：读取字段值（同输入恒定——算法对类结构确定性）
        Long svuid = readSvuid(transformed);
        check("svuid-value-fixed", true, svuid != null);
        System.out.println("  svuid = " + svuid);

        // 幂等性：二次改写不重复添加（已有 SVUID 时 Adder 直接透传）
        ClassWriter cw2 = new ClassWriter(0);
        new ClassReader(transformed).accept(new SerialVersionUIDAdder(cw2), 0);
        byte[] twice = cw2.toByteArray();
        check("idempotent", true, readSvuid(twice) != null && countSvuidFields(twice) == 1);

        // 同一输入再次全流程：SVUID 值一致（确定性）
        ClassWriter cw3 = new ClassWriter(0);
        new ClassReader(original).accept(new SerialVersionUIDAdder(cw3), 0);
        check("deterministic", true, readSvuid(cw3.toByteArray()).equals(svuid));
        System.out.println("DONE AsmCommonsMain");
    }

    static boolean hasSvuid(byte[] bytes) {
        return countSvuidFields(bytes) > 0;
    }

    static int countSvuidFields(byte[] bytes) {
        int[] count = {0};
        new ClassReader(bytes).accept(new ClassVisitor(Opcodes.ASM9) {
            @Override
            public org.objectweb.asm.FieldVisitor visitField(int access, String name,
                    String desc, String sig, Object value) {
                if ("serialVersionUID".equals(name) && "J".equals(desc)) {
                    count[0]++;
                }
                return null;
            }
        }, 0);
        return count[0];
    }

    static Long readSvuid(byte[] bytes) {
        Long[] found = {null};
        new ClassReader(bytes).accept(new ClassVisitor(Opcodes.ASM9) {
            @Override
            public org.objectweb.asm.FieldVisitor visitField(int access, String name,
                    String desc, String sig, Object value) {
                if ("serialVersionUID".equals(name) && value instanceof Long l) {
                    found[0] = l;
                }
                return null;
            }
        }, 0);
        return found[0];
    }
}
