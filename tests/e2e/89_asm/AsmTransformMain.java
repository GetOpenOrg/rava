
import org.objectweb.asm.ClassReader;
import org.objectweb.asm.ClassVisitor;
import org.objectweb.asm.ClassWriter;
import org.objectweb.asm.MethodVisitor;
import org.objectweb.asm.Opcodes;
import org.objectweb.asm.commons.AdviceAdapter;

/**
 * ASM pilot F2：ClassReader.accept + AdviceAdapter 方法改写 → 回读验证指令变化。
 * 转换面同样不 defineClass——输入字节码、输出字节码，纯计算。
 */
public class AsmTransformMain {

    /** 待改写的目标源：测试内生成的最小类（等价于上游 javac 产物；单文件自足，不读外部 class）。 */
    static final String TARGET = "pilot.AsmHello";

    /** 生成含 sayHello()（一条 ldc "Hello World"）与 calculate()（无 ldc）的最小类字节码。 */
    static byte[] generateHello() {
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
        var say = cw.visitMethod(Opcodes.ACC_PUBLIC, "sayHello", "()V", null, null);
        say.visitCode();
        say.visitFieldInsn(Opcodes.GETSTATIC, "java/lang/System", "out", "Ljava/io/PrintStream;");
        say.visitLdcInsn("Hello World");
        say.visitMethodInsn(Opcodes.INVOKEVIRTUAL, "java/io/PrintStream", "println", "(Ljava/lang/String;)V", false);
        say.visitInsn(Opcodes.RETURN);
        say.visitMaxs(2, 1);
        say.visitEnd();
        var calc = cw.visitMethod(Opcodes.ACC_PUBLIC, "calculate", "(II)I", null, null);
        calc.visitCode();
        calc.visitVarInsn(Opcodes.ILOAD, 1);
        calc.visitVarInsn(Opcodes.ILOAD, 2);
        calc.visitInsn(Opcodes.IADD);
        calc.visitInsn(Opcodes.IRETURN);
        calc.visitMaxs(2, 3);
        calc.visitEnd();
        cw.visitEnd();
        return cw.toByteArray();
    }

    static void check(String name, Object expected, Object actual) {
        boolean ok = String.valueOf(expected).equals(String.valueOf(actual));
        System.out.println((ok ? "PASS " : "FAIL ") + name
                + (ok ? "" : " :: expected=<" + expected + "> actual=<" + actual + ">"));
    }

    public static void main(String[] args) throws Exception {
        byte[] original = generateHello();
        String internal = TARGET.replace('.', '/');

        // ── 改写：给 sayHello() 首尾插入 println 指令（AdviceAdapter 精确出入点） ──
        ClassWriter cw = new ClassWriter(ClassWriter.COMPUTE_FRAMES | ClassWriter.COMPUTE_MAXS);
        int[] injected = {0};
        new ClassReader(original).accept(new ClassVisitor(Opcodes.ASM9, cw) {
            @Override
            public MethodVisitor visitMethod(int access, String name, String desc,
                                             String sig, String[] exceptions) {
                MethodVisitor mv = super.visitMethod(access, name, desc, sig, exceptions);
                if (name.equals("sayHello")) {
                    return new AdviceAdapter(Opcodes.ASM9, mv, access, name, desc) {
                        @Override
                        protected void onMethodEnter() {
                            injected[0]++;
                            mv.visitFieldInsn(Opcodes.GETSTATIC, "java/lang/System",
                                    "out", "Ljava/io/PrintStream;");
                            mv.visitLdcInsn("→ enter sayHello");
                            mv.visitMethodInsn(Opcodes.INVOKEVIRTUAL, "java/io/PrintStream",
                                    "println", "(Ljava/lang/String;)V", false);
                        }

                        @Override
                        protected void onMethodExit(int opcode) {
                            if (opcode != Opcodes.ATHROW) {
                                mv.visitFieldInsn(Opcodes.GETSTATIC, "java/lang/System",
                                        "out", "Ljava/io/PrintStream;");
                                mv.visitLdcInsn("← exit sayHello");
                                mv.visitMethodInsn(Opcodes.INVOKEVIRTUAL, "java/io/PrintStream",
                                        "println", "(Ljava/lang/String;)V", false);
                            }
                        }
                    };
                }
                return mv;
            }
        }, ClassReader.EXPAND_FRAMES);
        byte[] transformed = cw.toByteArray();

        check("inject-points", 1, injected[0]);
        check("size-changed", true, transformed.length != original.length);

        // ── 回读：验证指令序列包含插入的两条 ldc ──
        var ldcs = new java.util.ArrayList<String>();
        new ClassReader(transformed).accept(new ClassVisitor(Opcodes.ASM9) {
            String current;

            @Override
            public MethodVisitor visitMethod(int access, String name, String desc,
                                             String sig, String[] exceptions) {
                current = name;
                return new MethodVisitor(Opcodes.ASM9) {
                    @Override
                    public void visitLdcInsn(Object value) {
                        if ("sayHello".equals(current) && value instanceof String s) {
                            ldcs.add(s);
                        }
                    }
                };
            }
        }, 0);

        check("ldc-strings", "[→ enter sayHello, Hello World, ← exit sayHello]", ldcs.toString());

        // 未插桩方法的指令不受影响（calculate 的 ldc "Result: " 仍在）
        var otherLdc = new java.util.ArrayList<String>();
        new ClassReader(transformed).accept(new ClassVisitor(Opcodes.ASM9) {
            String current;

            @Override
            public MethodVisitor visitMethod(int access, String name, String desc,
                                             String sig, String[] exceptions) {
                current = name;
                return new MethodVisitor(Opcodes.ASM9) {
                    @Override
                    public void visitLdcInsn(Object value) {
                        if ("calculate".equals(current) && value instanceof String s) {
                            otherLdc.add(s);
                        }
                    }
                };
            }
        }, 0);
        check("calculate-untouched-no-ldc", "[]", otherLdc.toString()); // javac 拼接走 indy，无普通 ldc
        System.out.println("DONE AsmTransformMain");
    }
}
