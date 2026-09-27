/*
 * VM 支持类（FS-R R3，docs/plans/2026-09-27-reflection-metadata-table.md §2.4）：
 * 序列化构造器访问器的原生承载。
 *
 * JDK 的 MethodAccessorGenerator.generateSerializationConstructor 现场生成字节码类——分配
 * declaringClass 的实例、只运行 targetConstructorClass（首个不可序列化超类）的无参构造体。
 * 原生二进制无运行期类定义：全部序列化构造器共用本类，实例字段携带两端类，分配与构造体
 * 由 VM native 按 L3 分派的 `<alloc>` / `<init_on>` 协议执行（N2 同一协议）。
 */
package jdk.internal.reflect;

import java.lang.reflect.InvocationTargetException;

final class SerializationConstructorAccessorDyn extends SerializationConstructorAccessorImpl {
    private final Class<?> target;
    private final Class<?> initCl;

    SerializationConstructorAccessorDyn(Class<?> target, Class<?> initCl) {
        this.target = target;
        this.initCl = initCl;
    }

    @Override
    public Object newInstance(Object[] args) throws InstantiationException, InvocationTargetException {
        try {
            return allocateAndInit(target, initCl);
        } catch (Throwable t) {
            throw new InvocationTargetException(t);
        }
    }

    /** 分配 target 实例（不运行其构造器），在其上运行 initCl 的无参构造体。 */
    private static native Object allocateAndInit(Class<?> target, Class<?> initCl);
}
