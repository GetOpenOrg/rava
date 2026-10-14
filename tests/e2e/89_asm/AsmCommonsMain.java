
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

    /** 待改写的目标源：测试内生成的最小 User POJO（等价于上游 javac 产物；单文件自足，不读外部 class）。 */
    static final String TARGET = "com.example.cglib.User";

    /** 生成 com/example/cglib/User 字节码：默认构造器 + name(String)/age(int) 字段与
     * getter/setter 的普通 POJO，无 serialVersionUID（对齐上游 cglib 工程的 User 形态）。 */
    static byte[] generateUser() {
        String internal = TARGET.replace('.', '/');
        ClassWriter cw = new ClassWriter(ClassWriter.COMPUTE_FRAMES | ClassWriter.COMPUTE_MAXS);
        cw.visit(Opcodes.V21, Opcodes.ACC_PUBLIC, internal, null, "java/lang/Object", null);
        var init = cw.visitMethod(Opcodes.ACC_PUBLIC, "<init>", "()V", null, null);
        init.visitCode();
        init.visitVarInsn(Opcodes.ALOAD, 0);
        init.visitMethodInsn(Opcodes.INVOKESPECIAL, "java/lang/Object", "<init>", "()V", false);
        init.visitInsn(Opcodes.RETURN);
        init.visitMaxs(1, 1);
        init.visitEnd();
        cw.visitField(Opcodes.ACC_PRIVATE, "name", "Ljava/lang/String;", null, null).visitEnd();
        var getName = cw.visitMethod(Opcodes.ACC_PUBLIC, "getName", "()Ljava/lang/String;", null, null);
        getName.visitCode();
        getName.visitVarInsn(Opcodes.ALOAD, 0);
        getName.visitFieldInsn(Opcodes.GETFIELD, internal, "name", "Ljava/lang/String;");
        getName.visitInsn(Opcodes.ARETURN);
        getName.visitMaxs(1, 1);
        getName.visitEnd();
        var setName = cw.visitMethod(Opcodes.ACC_PUBLIC, "setName", "(Ljava/lang/String;)V", null, null);
        setName.visitCode();
        setName.visitVarInsn(Opcodes.ALOAD, 0);
        setName.visitVarInsn(Opcodes.ALOAD, 1);
        setName.visitFieldInsn(Opcodes.PUTFIELD, internal, "name", "Ljava/lang/String;");
        setName.visitInsn(Opcodes.RETURN);
        setName.visitMaxs(2, 2);
        setName.visitEnd();
        cw.visitField(Opcodes.ACC_PRIVATE, "age", "I", null, null).visitEnd();
        var getAge = cw.visitMethod(Opcodes.ACC_PUBLIC, "getAge", "()I", null, null);
        getAge.visitCode();
        getAge.visitVarInsn(Opcodes.ALOAD, 0);
        getAge.visitFieldInsn(Opcodes.GETFIELD, internal, "age", "I");
        getAge.visitInsn(Opcodes.IRETURN);
        getAge.visitMaxs(1, 1);
        getAge.visitEnd();
        var setAge = cw.visitMethod(Opcodes.ACC_PUBLIC, "setAge", "(I)V", null, null);
        setAge.visitCode();
        setAge.visitVarInsn(Opcodes.ALOAD, 0);
        setAge.visitVarInsn(Opcodes.ILOAD, 1);
        setAge.visitFieldInsn(Opcodes.PUTFIELD, internal, "age", "I");
        setAge.visitInsn(Opcodes.RETURN);
        setAge.visitMaxs(2, 2);
        setAge.visitEnd();
        cw.visitEnd();
        return cw.toByteArray();
    }

    static void check(String name, Object expected, Object actual) {
        boolean ok = String.valueOf(expected).equals(String.valueOf(actual));
        System.out.println((ok ? "PASS " : "FAIL ") + name
                + (ok ? "" : " :: expected=<" + expected + "> actual=<" + actual + ">"));
    }

    public static void main(String[] args) throws Exception {
        // 目标：测试内生成的 User POJO 字节码（无 SVUID 的普通 POJO）
        byte[] original = generateUser();

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
