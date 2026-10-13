package com.example.asm;

import java.lang.reflect.Method;

import org.objectweb.asm.ClassWriter;
import org.objectweb.asm.MethodVisitor;
import org.objectweb.asm.Opcodes;

/**
 * ASM 实操三（进阶）：不用 JDK Proxy、不用 CGLib，
 * 手写一个基于 ASM 的代理类生成器——生成目标类的子类并重写指定方法，
 * 在重写方法中前后插入增强逻辑后调用 super。
 */
public class AsmProxyDemo {

    public static void main(String[] args) throws Exception {
        Hello proxy = (Hello) createProxy(Hello.class, "sayHello");
        proxy.sayHello();       // 会输出【ASM代理前置】/【后置】
        proxy.calculate();      // 未重写，直接走父类原逻辑
    }

    /** 生成 targetClass 的子类，重写 methodName（无参 void 方法），前后插入打印。 */
    public static Object createProxy(Class<?> targetClass, String methodName) throws Exception {
        String targetInternal = targetClass.getName().replace('.', '/');
        String proxyName = targetInternal + "$$AsmProxy";

        ClassWriter cw = new ClassWriter(ClassWriter.COMPUTE_FRAMES | ClassWriter.COMPUTE_MAXS);
        cw.visit(Opcodes.V21, Opcodes.ACC_PUBLIC | Opcodes.ACC_SUPER,
                proxyName, null, targetInternal, null);

        // 构造器：public Proxy() { super(); }
        MethodVisitor ctor = cw.visitMethod(Opcodes.ACC_PUBLIC, "<init>", "()V", null, null);
        ctor.visitCode();
        ctor.visitVarInsn(Opcodes.ALOAD, 0);
        ctor.visitMethodInsn(Opcodes.INVOKESPECIAL, targetInternal, "<init>", "()V", false);
        ctor.visitInsn(Opcodes.RETURN);
        ctor.visitMaxs(1, 1);
        ctor.visitEnd();

        // 重写目标方法：前置打印 → super.method() → 后置打印
        MethodVisitor mv = cw.visitMethod(Opcodes.ACC_PUBLIC, methodName, "()V", null, null);
        mv.visitCode();
        println(mv, "【ASM代理前置】" + methodName + " 开始");
        mv.visitVarInsn(Opcodes.ALOAD, 0);
        mv.visitMethodInsn(Opcodes.INVOKESPECIAL, targetInternal, methodName, "()V", false);
        println(mv, "【ASM代理后置】" + methodName + " 结束");
        mv.visitInsn(Opcodes.RETURN);
        mv.visitMaxs(2, 1);
        mv.visitEnd();

        cw.visitEnd();
        byte[] code = cw.toByteArray();

        Class<?> proxyClass = new ChildClassLoader(targetClass.getClassLoader())
                .defineClass(proxyName.replace('/', '.'), code);
        return proxyClass.getDeclaredConstructor().newInstance();
    }

    /** 生成 System.out.println(msg) 指令序列。 */
    private static void println(MethodVisitor mv, String msg) {
        mv.visitFieldInsn(Opcodes.GETSTATIC, "java/lang/System",
                "out", "Ljava/io/PrintStream;");
        mv.visitLdcInsn(msg);
        mv.visitMethodInsn(Opcodes.INVOKEVIRTUAL, "java/io/PrintStream",
                "println", "(Ljava/lang/String;)V", false);
    }
}
