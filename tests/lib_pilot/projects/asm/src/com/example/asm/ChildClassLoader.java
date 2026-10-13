package com.example.asm;

/**
 * AsmProxyDemo 生成字节码的 defineClass 载体（上游迁移遗漏件，最小补齐：
 * 上游 java-examples 的 AsmTransformDemo 内嵌同名类随宿主类一起未迁入，
 * 但 AsmProxyDemo 仍引用它——本文件按原用途独立成同包顶层类，不改 demo 输出语义）。
 */
final class ChildClassLoader extends ClassLoader {

    ChildClassLoader(ClassLoader parent) {
        super(parent);
    }

    /** 把 ASM 生成的字节码定义为 Class（ClassLoader.defineClass 的公开化包装）。 */
    Class<?> defineClass(String name, byte[] code) {
        return defineClass(name, code, 0, code.length);
    }
}
