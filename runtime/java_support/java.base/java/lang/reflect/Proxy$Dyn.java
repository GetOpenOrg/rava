/*
 * VM 支持类（java_rta）：动态代理的通用载体（FS-R R4a）。
 *
 * JDK 的 Proxy.newProxyInstance 经 ProxyBuilder.defineProxyClass → ProxyGenerator 在运行期
 * 生成并定义 $ProxyN 类（每个接口方法体为 h.invoke(this, m, args) + 异常 / 返回值转换）。
 * 原生二进制不能在运行期定义类，全部代理实例共用本类：实例携带接口列表 intfs，
 * 接口方法调用由 VM 在接口载体分派的回退点转入 dispatch（runtime/java_runtime/src/proxy_dyn.rs），
 * instanceof / checkcast 按 intfs 应答。代理语义（转发、声明异常透传、
 * UndeclaredThrowableException、Object 三方法转发）全部在本类以 Java 表达、经字节码翻译。
 * 方案：docs/plans/2026-09-27-reflection-metadata-table.md §2.6
 *
 * 编译：转译时以当前 JDK 的 javac --patch-module java.base 编入 java.lang.reflect 包
 * （codegen/jdk_resolver.py vm_support_classes）。
 */
package java.lang.reflect;

import java.util.Objects;

final class Proxy$Dyn extends Proxy {
    private static final long serialVersionUID = 1L;

    /** 代理实现的接口（newProxyInstance 实参的副本，声明序）。 */
    final Class<?>[] intfs;

    private Proxy$Dyn(InvocationHandler h, Class<?>[] intfs) {
        super(h);
        this.intfs = intfs;
    }

    /** Proxy.newProxyInstance 的 VM 承载（intrinsics.txt「运行期类定义点」）。 */
    static Object create(Class<?>[] interfaces, InvocationHandler h) {
        Objects.requireNonNull(h);
        Class<?>[] intfs = interfaces.clone();
        for (Class<?> intf : intfs) {
            if (!intf.isInterface()) {
                throw new IllegalArgumentException(intf.getName() + " is not an interface");
            }
        }
        return new Proxy$Dyn(h, intfs);
    }

    /** 代理是否实现 type（含超接口，instanceof / checkcast 的 VM 应答）。 */
    boolean implementsType(Class<?> type) {
        for (Class<?> intf : intfs) {
            if (type.isAssignableFrom(intf)) {
                return true;
            }
        }
        return false;
    }

    /**
     * 代理方法体（ProxyGenerator 生成体的等价物）：转发 InvocationHandler；RuntimeException /
     * Error 与方法声明的 checked 异常原样透传，其余包装为 UndeclaredThrowableException。
     */
    Object dispatch(Method m, Object[] args) throws Throwable {
        try {
            return h.invoke(this, m, args);
        } catch (RuntimeException | Error e) {
            throw e;
        } catch (Throwable t) {
            for (Class<?> ex : m.getExceptionTypes()) {
                if (ex.isInstance(t)) {
                    throw t;
                }
            }
            throw new UndeclaredThrowableException(t);
        }
    }

    private Object dispatchObject(String name, Class<?>[] params, Object[] args) {
        try {
            return dispatch(Object.class.getMethod(name, params), args);
        } catch (RuntimeException | Error e) {
            throw e;
        } catch (Throwable t) {
            throw new UndeclaredThrowableException(t);
        }
    }

    @Override
    public int hashCode() {
        return (Integer) dispatchObject("hashCode", new Class<?>[0], null);
    }

    @Override
    public boolean equals(Object obj) {
        return (Boolean) dispatchObject("equals", new Class<?>[] {Object.class}, new Object[] {obj});
    }

    @Override
    public String toString() {
        return (String) dispatchObject("toString", new Class<?>[0], null);
    }
}
