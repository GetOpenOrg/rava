
import org.objectweb.asm.ClassReader;
import org.objectweb.asm.ClassVisitor;
import org.objectweb.asm.ClassWriter;
import org.objectweb.asm.FieldVisitor;
import org.objectweb.asm.MethodVisitor;
import org.objectweb.asm.Opcodes;

/**
 * ASM pilot F1：ClassWriter 从零生成类 → ClassReader 回读断言。
 * 生成面不 defineClass——纯字节码计算，C-SYN 无关。
 */
public class AsmGenerateMain {

    static void check(String name, Object expected, Object actual) {
        boolean ok = String.valueOf(expected).equals(String.valueOf(actual));
        System.out.println((ok ? "PASS " : "FAIL ") + name
                + (ok ? "" : " :: expected=<" + expected + "> actual=<" + actual + ">"));
    }

    public static void main(String[] args) {
        String internal = "pilot/Generated";

        // ── 生成：public class Generated extends Object { int answer(); static String tag; } ──
        ClassWriter cw = new ClassWriter(0);
        cw.visit(Opcodes.V21, Opcodes.ACC_PUBLIC | Opcodes.ACC_SUPER,
                internal, null, "java/lang/Object", null);

        FieldVisitor fv = cw.visitField(Opcodes.ACC_PUBLIC | Opcodes.ACC_STATIC,
                "tag", "Ljava/lang/String;", null, "GEN");
        fv.visitEnd();

        MethodVisitor ctor = cw.visitMethod(Opcodes.ACC_PUBLIC, "<init>", "()V", null, null);
        ctor.visitCode();
        ctor.visitVarInsn(Opcodes.ALOAD, 0);
        ctor.visitMethodInsn(Opcodes.INVOKESPECIAL, "java/lang/Object", "<init>", "()V", false);
        ctor.visitInsn(Opcodes.RETURN);
        ctor.visitMaxs(1, 1);
        ctor.visitEnd();

        MethodVisitor answer = cw.visitMethod(Opcodes.ACC_PUBLIC, "answer", "()I", null, null);
        answer.visitCode();
        answer.visitLdcInsn(42);
        answer.visitInsn(Opcodes.IRETURN);
        answer.visitMaxs(1, 1);
        answer.visitEnd();
        cw.visitEnd();
        byte[] bytes = cw.toByteArray();

        check("bytes-length>100", true, bytes.length > 100);
        check("magic-cafebabe", "cafebabe",
                java.util.HexFormat.of().formatHex(
                        java.util.Arrays.copyOfRange(bytes, 0, 4)));

        // ── 回读：用 ClassReader 断言结构 ──
        var facts = new java.util.LinkedHashMap<String, String>();
        new ClassReader(bytes).accept(new ClassVisitor(Opcodes.ASM9) {
            @Override
            public void visit(int version, int access, String name, String sig,
                              String superName, String[] interfaces) {
                facts.put("class", name + " extends " + superName
                        + " version=" + (version & 0xFFFF));
            }

            @Override
            public FieldVisitor visitField(int access, String name, String desc,
                                           String sig, Object value) {
                facts.put("field", name + ":" + desc + " value=" + value);
                return null;
            }

            @Override
            public MethodVisitor visitMethod(int access, String name, String desc,
                                             String sig, String[] exceptions) {
                facts.put("method:" + name, desc + " acc=" + access);
                return null;
            }
        }, 0);

        check("class-header", "pilot/Generated extends java/lang/Object version=65",
                facts.get("class"));
        check("field", "tag:Ljava/lang/String; value=GEN", facts.get("field"));
        check("method-ctor", "()V acc=1", facts.get("method:<init>"));
        check("method-answer", "()I acc=1", facts.get("method:answer"));
        System.out.println("DONE AsmGenerateMain");
    }
}
